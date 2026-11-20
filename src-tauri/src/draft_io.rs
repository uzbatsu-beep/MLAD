//! Offline JSON draft storage: staged writes, same-directory rename, bounded reads.
//!
//! Only used after distinct *.graph.json / *.advanced.json / legacy *.json
//! filename checks. No CIOPRG.LST or IONAME.DAT writer exists.
//!
//! This is a *best-effort per-file atomic replacement*, not a complete
//! transaction for target+backup or a guarantee against adversarial changes
//! of parent directories. Symlink/TOCTOU review remains open for future work.
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use sha2::{Digest, Sha256};

pub const MAX_JSON_BYTES: usize = 2 * 1024 * 1024;

/// Windows paths with ADS, UNC, device namespaces or drive-relative syntax
/// are outside the verified local-file contract. A colon in a normal
/// component can select an alternate data stream while bypassing suffix
/// rules. Reject *before* probing the filesystem.
#[cfg(windows)]
fn reject_windows_special_path(path:&Path)->Result<(),String> {
    use std::path::Prefix;
    for component in path.components() {
        match component {
            Component::Prefix(prefix)=>{
                match prefix.kind() {
                    Prefix::Disk(_) if path.has_root() => {},
                    _ => return Err("UNC, device, verbatim и drive-relative пути не поддерживаются".into()),
                }
            }
            Component::Normal(part) if part.to_string_lossy().contains(':') =>
                return Err("Windows alternate data stream (ADS) в пути запрещён".into()),
            _=>{}
        }
    }
    Ok(())
}

/// Best-effort fail-closed check of every pre-existing ancestor, not only
/// the final filename. Avoids accidentally following symlinked parent
/// directories during normal offline IO. Not an atomic no-follow guarantee:
/// an attacker controlling the filesystem can race between check/open/rename.
/// See docs/path_security_review.md for unresolved directory-handle work.
pub fn verify_parent_directory(path: &Path) -> Result<(), String> {
    #[cfg(windows)]
    reject_windows_special_path(path)?;
    let parent=path.parent().filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(||Path::new("."));
    let absolute=if parent.is_absolute() {
        parent.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|e|format!("Не удалось получить текущий каталог: {e}"))?.join(parent)
    };
    let mut cursor=PathBuf::new();
    for component in absolute.components() {
        match component {
            Component::Prefix(_) | Component::RootDir =>
                cursor.push(component.as_os_str()),
            Component::CurDir => {},
            Component::ParentDir =>
                return Err("Пути с .. запрещены при работе с файлами MLAD".into()),
            Component::Normal(name) => {
                cursor.push(name);
                let meta=fs::symlink_metadata(&cursor)
                    .map_err(|e|format!("Не удалось проверить каталог {}: {e}",cursor.display()))?;
                if crate::guarded_read::is_indirection(&meta) {
                    return Err("Ссылка или reparse point в родительском каталоге запрещены".into());
                }
                if !meta.is_dir() {
                    return Err("Родительский путь не является каталогом".into());
                }
            }
        }
    }
    Ok(())
}

fn validate_file(path: &Path, allow_missing: bool) -> Result<bool, String> {
    verify_parent_directory(path)?;
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            if crate::guarded_read::is_indirection(&metadata) {
                return Err("Прямые ссылки/reparse для JSON запрещены".into());
            }
            if !metadata.is_file() {
                return Err("JSON должен быть обычным файлом".into());
            }
            if metadata.len() > MAX_JSON_BYTES as u64 {
                return Err("JSON превышает предел 2 МиБ".into());
            }
            // Multi-link JSON aliases make ownership/backup semantics ambiguous.
            // Reject both final target and *.bak even if they appear regular.
            crate::guarded_read::verify_single_link(path)?;
            Ok(true)
        }
        Err(err) if err.kind()==std::io::ErrorKind::NotFound && allow_missing => Ok(false),
        Err(err) => Err(format!("Не удалось проверить JSON файл: {err}")),
    }
}

/// A bounded reader also rejects direct symlinks. It does not claim a
/// no-follow open on every filesystem or prohibit symlink ancestors.
pub fn read_bounded(path: &Path) -> Result<Vec<u8>, String> {
    read_bounded_snapshot(path).map(|(bytes,_)|bytes)
}

fn read_bounded_snapshot(path:&Path)
    ->Result<(Vec<u8>,crate::guarded_read::FileIdentity),String>{
    validate_file(path,false)?;
    crate::guarded_read::read_regular_with_identity(path,MAX_JSON_BYTES)
}

/// A private staged file is identified by its opened OS handle and SHA-256.
/// This detects replacement/modification *before* rename, not an adversarial
/// swap in the narrow final verify->rename interval.
struct StagedFile {
    path: PathBuf,
    identity: crate::guarded_read::FileIdentity,
    digest: String,
}

impl StagedFile {
    fn create_near(destination:&Path, content:&[u8])->Result<Self,String> {
        verify_parent_directory(destination)?;
        let basename=destination.file_name()
            .and_then(|s|s.to_str()).ok_or("Недопустимое имя JSON")?;
        let scratch=destination.with_file_name(format!(
            ".{basename}.mlad-{}.tmp",uuid::Uuid::new_v4()
        ));
        // Unix JSON drafts and backups are owner-only at creation, regardless
        // of a permissive umask; do not copy a public mode from the old file.
        // Windows ACLs follow parent inheritance and remain a separate review.
        let mut options=OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)] {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut handle=options.open(&scratch)
            .map_err(|e|format!("Не удалось создать временный JSON: {e}"))?;
        let identity=match crate::guarded_read::identity_of_handle(&handle){
            Ok(identity)=>identity,
            Err(error)=>{
                drop(handle);
                let _=fs::remove_file(&scratch);
                return Err(error);
            }
        };
        let staged=Self {
            path:scratch,
            identity,
            digest:hex::encode(Sha256::digest(content)),
        };
        let outcome=handle.write_all(content).and_then(|_|handle.sync_all());
        // Close the Windows handle before a failed write triggers temp cleanup.
        drop(handle);
        outcome.map_err(|e|format!("Не удалось записать временный JSON: {e}"))?;
        Ok(staged)
    }

    fn replace(mut self, destination:&Path)->Result<(),String> {
        verify_parent_directory(destination)?;
        let (bytes,identity)=crate::guarded_read::read_regular_with_identity(&self.path,MAX_JSON_BYTES)?;
        if identity!=self.identity || hex::encode(Sha256::digest(&bytes))!=self.digest {
            return Err("Временный JSON был заменён или изменён перед сохранением".into());
        }
        // A hardlink to a staged file keeps mutable access to the committed
        // target after the rename; reject this detectable exposure.
        crate::guarded_read::verify_single_link(&self.path)?;
        verify_parent_directory(destination)?;
        fs::rename(&self.path,destination)
            .map_err(|e|format!("Не удалось заменить JSON файлом из той же папки: {e}"))?;
        self.path=PathBuf::new();
        Ok(())
    }
}

impl Drop for StagedFile {
    fn drop(&mut self) {
        if !self.path.as_os_str().is_empty() {
            // A different file may have appeared at the random staged path.
            // Do not unlink that file during cleanup; this is not a guarantee
            // against an adversarial swap after the final identity check.
            let ours=fs::symlink_metadata(&self.path).ok()
                .filter(|m|m.is_file()&&!crate::guarded_read::is_indirection(m))
                .and_then(|_|File::open(&self.path).ok())
                .and_then(|file|crate::guarded_read::identity_of_handle(&file).ok())
                .is_some_and(|id|id==self.identity);
            if ours {
                let _=fs::remove_file(&self.path);
            }
        }
    }
}

/// Preserve the previous target bytes in *.bak, then replace the target
/// through a staged file in the *same directory*. Never write through a
/// pre-existing direct symlink or truncate the original in place.
///
/// If backup creation/rename fails, the target is untouched. A final rename
/// failure also leaves the original in place; the previous backup might
/// already be updated. Folder metadata sync is best effort on Unix only.
pub fn write_atomic(path:&Path, content:&[u8])->Result<(),String> {
    write_atomic_inner(path,content,||{})
}

/// Internal seam for deterministic tests of external changes just before commit.
fn write_atomic_inner(path:&Path, content:&[u8], before_commit:impl FnOnce())->Result<(),String> {
    if content.len()>MAX_JSON_BYTES {
        return Err("JSON превышает предел 2 МиБ".into());
    }
    if !path.file_name().and_then(|s|s.to_str())
        .is_some_and(|s|s.ends_with(".json")) {
        return Err("Запись допускается только в JSON черновик".into());
    }
    let existed=validate_file(path,true)?;
    let staged=StagedFile::create_near(path,content)?;
    let original=if existed {Some(read_bounded_snapshot(path)?)} else {None};
    if let Some(ref original)=original {
        let basename=path.file_name().and_then(|s|s.to_str())
            .ok_or("Не удалось определить имя JSON")?;
        let backup=path.with_file_name(format!("{basename}.bak"));
        validate_file(&backup,true)?;
        let previous=StagedFile::create_near(&backup,&original.0)?;
        // Don't replace a newly appeared symlink; parent-dir swaps remain
        // an explicitly documented limitation of std::fs path-based APIs.
        validate_file(&backup,true)?;
        if read_bounded_snapshot(path)? != *original {
            return Err("JSON подменён или изменён при подготовке резервной копии".into());
        }
        previous.replace(&backup)?;
    }
    before_commit();
    match original {
        Some(expected)=>{
            if read_bounded_snapshot(path)? != expected {
                return Err("Конфликт сохранения: JSON изменён другим процессом или подменён".into());
            }
        }
        None=>{
            if validate_file(path,true)? {
                return Err("Конфликт сохранения: другой процесс создал JSON".into());
            }
        }
    }
    staged.replace(path)?;
    #[cfg(unix)]
    if let Some(parent)=path.parent(){
        let folder=if parent.as_os_str().is_empty(){Path::new(".")}else{parent};
        if let Ok(dir)=File::open(folder){let _=dir.sync_all();}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn folder()->PathBuf {
        let dir=std::env::temp_dir().join(format!("mlad-json-{}",uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        dir
    }
    #[test]
    fn graph_first_save_is_complete_and_has_no_backup(){
        let dir=folder();let path=dir.join("draft.graph.json");
        write_atomic(&path,b"{\"version\":1}").unwrap();
        assert_eq!(fs::read(&path).unwrap(),b"{\"version\":1}");
        assert!(!dir.join("draft.graph.json.bak").exists());
        assert_eq!(fs::read_dir(&dir).unwrap().count(),1);
        fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn overwrite_preserves_exact_previous_content_in_backup(){
        let dir=folder();let path=dir.join("session.advanced.json");
        let old=b"{\r\n\"old\":true\r\n}";
        fs::write(&path,old).unwrap();
        write_atomic(&path,b"{\"new\":true}").unwrap();
        assert_eq!(fs::read(&path).unwrap(),b"{\"new\":true}");
        assert_eq!(fs::read(dir.join("session.advanced.json.bak")).unwrap(),old);
        write_atomic(&path,b"{\"third\":true}").unwrap();
        assert_eq!(fs::read(dir.join("session.advanced.json.bak")).unwrap(),b"{\"new\":true}");
        assert_eq!(fs::read_dir(&dir).unwrap().count(),2);
        fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn oversize_write_and_read_are_rejected_without_truncating_original(){
        let dir=folder();let path=dir.join("legacy.json");
        fs::write(&path,b"original draft").unwrap();
        assert!(write_atomic(&path,&vec![b'X';MAX_JSON_BYTES+1]).is_err());
        assert_eq!(fs::read(&path).unwrap(),b"original draft");
        assert!(!dir.join("legacy.json.bak").exists());
        fs::write(&path,vec![b'X';MAX_JSON_BYTES+1]).unwrap();
        assert!(read_bounded(&path).is_err());
        fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn directory_as_target_or_backup_is_rejected_without_mutation(){
        let dir=folder();let path=dir.join("draft.graph.json");
        fs::create_dir(&path).unwrap();
        assert!(write_atomic(&path,b"{}").is_err());
        fs::remove_dir(&path).unwrap();
        fs::write(&path,b"original").unwrap();
        let bak=dir.join("draft.graph.json.bak");
        fs::create_dir(&bak).unwrap();
        assert!(write_atomic(&path,b"updated").is_err());
        assert_eq!(fs::read(&path).unwrap(),b"original");
        fs::remove_dir_all(dir).unwrap();
    }
    #[cfg(unix)]
    #[test]
    fn direct_symlink_target_and_backup_are_rejected(){
        use std::os::unix::fs::symlink;
        let dir=folder();let native=dir.join("CIOPRG.LST");
        fs::write(&native,b"native robot bytes").unwrap();
        let link=dir.join("bad.graph.json");
        symlink(&native,&link).unwrap();
        assert!(write_atomic(&link,b"bad").is_err());
        assert_eq!(fs::read(&native).unwrap(),b"native robot bytes");
        assert!(read_bounded(&link).is_err());
        let draft=dir.join("other.advanced.json");
        fs::write(&draft,b"previous").unwrap();
        symlink(&native,dir.join("other.advanced.json.bak")).unwrap();
        assert!(write_atomic(&draft,b"new").is_err());
        assert_eq!(fs::read(&draft).unwrap(),b"previous");
        assert_eq!(fs::read(&native).unwrap(),b"native robot bytes");
        fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn failed_temp_creation_leaves_old_source_and_backup_untouched(){
        let dir=folder();let path=dir.join("draft.graph.json");
        fs::write(&path,b"original").unwrap();
        let before=fs::read(&path).unwrap();
        let inaccessible=dir.join("nonexistent").join("draft.graph.json");
        assert!(write_atomic(&inaccessible,b"new").is_err());
        assert_eq!(fs::read(&path).unwrap(),before);
        assert!(!dir.join("draft.graph.json.bak").exists());
        fs::remove_dir_all(dir).unwrap();
    }
    #[cfg(windows)]
    #[test]
    fn opened_windows_draft_is_never_truncated_even_if_replace_is_blocked(){
        let dir=folder();let path=dir.join("opened.graph.json");
        fs::write(&path,b"prior complete JSON").unwrap();
        let opened=File::open(&path).unwrap();
        let outcome=write_atomic(&path,b"new complete JSON");
        drop(opened);
        let after=fs::read(&path).unwrap();
        if outcome.is_ok() {
            assert_eq!(after,b"new complete JSON");
            assert_eq!(fs::read(dir.join("opened.graph.json.bak")).unwrap(),
                b"prior complete JSON");
        } else {
            assert_eq!(after,b"prior complete JSON");
        }
        fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn nested_regular_directories_allow_normal_json_roundtrip(){
        let dir=folder();let nested=dir.join("safe").join("project");
        fs::create_dir_all(&nested).unwrap();
        let path=nested.join("draft.graph.json");
        write_atomic(&path,b"first").unwrap();
        write_atomic(&path,b"second").unwrap();
        assert_eq!(read_bounded(&path).unwrap(),b"second");
        assert_eq!(fs::read(nested.join("draft.graph.json.bak")).unwrap(),b"first");
        fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn relative_parent_traversal_rejected_before_any_write(){
        let dir=folder();let parent=dir.join("safe");
        fs::create_dir(&parent).unwrap();
        let tricky=parent.join("..").join("escaped.graph.json");
        assert!(write_atomic(&tricky,b"data").is_err());
        assert!(!dir.join("escaped.graph.json").exists());
        assert!(read_bounded(&tricky).is_err());
        fs::remove_dir_all(dir).unwrap();
    }
    #[cfg(unix)]
    #[test]
    fn symlinked_parent_directory_cannot_redirect_json_into_native_folder(){
        use std::os::unix::fs::symlink;
        let root=folder();let real=root.join("native_files");
        fs::create_dir(&real).unwrap();
        let native=real.join("CIOPRG.LST");
        fs::write(&native,b"original robot data").unwrap();
        let draft=real.join("draft.graph.json");
        fs::write(&draft,b"untouched").unwrap();
        let alias=root.join("linked_dir");
        symlink(&real,&alias).unwrap();
        let forged=alias.join("draft.graph.json");
        assert!(write_atomic(&forged,b"forged").is_err());
        assert!(read_bounded(&forged).is_err());
        assert_eq!(fs::read(&draft).unwrap(),b"untouched");
        assert_eq!(fs::read(&native).unwrap(),b"original robot data");
        assert!(!real.join("draft.graph.json.bak").exists());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn external_change_between_backup_and_commit_cannot_be_overwritten(){
        let root=folder();let path=root.join("client.graph.json");
        fs::write(&path,b"baseline").unwrap();
        let result=write_atomic_inner(&path,b"stale client",||{
            fs::write(&path,b"external update").unwrap();
        });
        assert!(result.is_err());
        assert_eq!(fs::read(&path).unwrap(),b"external update");
        assert_eq!(fs::read(root.join("client.graph.json.bak")).unwrap(),b"baseline");
        assert_eq!(fs::read_dir(&root).unwrap().count(),2);
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn competing_new_file_creation_is_not_overwritten(){
        let root=folder();let path=root.join("new.advanced.json");
        let result=write_atomic_inner(&path,b"my draft",||{
            fs::write(&path,b"other process").unwrap();
        });
        assert!(result.is_err());
        assert_eq!(fs::read(&path).unwrap(),b"other process");
        assert!(!root.join("new.advanced.json.bak").exists());
        fs::remove_dir_all(root).unwrap();
    }
    #[cfg(windows)]
    #[test]
    fn windows_exclusive_lock_refuses_write_without_truncation_or_temp_leak(){
        use std::os::windows::fs::OpenOptionsExt;
        let root=folder();
        let path=root.join("exclusive.graph.json");
        let previous=b"complete unchanged synthetic JSON";
        fs::write(&path,previous).unwrap();
        // share_mode(0): deny any second open while this handle is held.
        let lock=OpenOptions::new().read(true).write(true).share_mode(0)
            .open(&path).unwrap();
        let outcome=write_atomic(&path,b"new JSON");
        drop(lock);
        assert!(outcome.is_err(),"exclusive NTFS handle must reject replacement");
        assert_eq!(fs::read(&path).unwrap(),previous);
        assert!(!root.join("exclusive.graph.json.bak").exists());
        assert_eq!(fs::read_dir(&root).unwrap().count(),1,
            "failed attempt must not leave staging files");
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(windows)]
    #[test]
    fn windows_junction_parent_never_writes_into_target_directory(){
        use std::process::Command;
        let root=folder();
        let real=root.join("real");
        fs::create_dir(&real).unwrap();
        let native=real.join("CIOPRG.LST");
        let draft=real.join("protected.graph.json");
        fs::write(&native,b"synthetic native source bytes").unwrap();
        fs::write(&draft,b"old JSON").unwrap();
        let alias=root.join("junction");
        // Junction creation needs no privilege to create a directory symlink.
        let result=Command::new("cmd").args(["/C","mklink","/J"])
            .arg(&alias).arg(&real).output().unwrap();
        assert!(result.status.success(),"mklink /J failed: {}",
            String::from_utf8_lossy(&result.stderr));
        let through_junction=alias.join("protected.graph.json");
        assert!(verify_parent_directory(&through_junction).is_err());
        assert!(read_bounded(&through_junction).is_err());
        assert!(write_atomic(&through_junction,b"overwritten").is_err());
        assert_eq!(fs::read(&draft).unwrap(),b"old JSON");
        assert_eq!(fs::read(&native).unwrap(),b"synthetic native source bytes");
        assert!(!real.join("protected.graph.json.bak").exists());
        // Remove only the reparse directory, never recursively follow it.
        fs::remove_dir(&alias).unwrap();
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(windows)]
    #[test]
    fn windows_ads_unc_and_device_paths_are_rejected_before_filesystem_io(){
        let ads=Path::new(r"C:\mlad-fixture\CIOPRG.LST:hidden.graph.json");
        assert!(verify_parent_directory(ads).unwrap_err().contains("ADS"));
        let nested_ads=Path::new(r"C:\mlad-fixture:stream\draft.graph.json");
        assert!(verify_parent_directory(nested_ads).unwrap_err().contains("ADS"));
        for unsafe_path in [
            r"\\server\share\draft.graph.json",
            r"\\.\PhysicalDrive0",
            r"\\?\C:\mlad-fixture\draft.graph.json",
            r"C:drive-relative.graph.json",
        ] {
            assert!(verify_parent_directory(Path::new(unsafe_path)).is_err(),
                "unsafe Windows path accepted: {unsafe_path}");
        }
    }

    #[test]
    fn same_content_target_replacement_is_a_save_conflict(){
        let root=folder();
        let path=root.join("same.graph.json");
        let replacement=root.join("replacement.json");
        fs::write(&path,b"identical bytes").unwrap();
        fs::write(&replacement,b"identical bytes").unwrap();
        let result=write_atomic_inner(&path,b"new JSON",||{
            // An attacker swaps in a different inode/File ID with equal content.
            fs::rename(&replacement,&path).unwrap();
        });
        assert!(result.is_err(),"same bytes must not conceal another file identity");
        assert_eq!(fs::read(&path).unwrap(),b"identical bytes");
        assert_eq!(fs::read(root.join("same.graph.json.bak")).unwrap(),b"identical bytes");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn hardlink_alias_to_native_source_or_json_backup_is_rejected(){
        let root=folder();
        let native=root.join("CIOPRG.LST");
        fs::write(&native,b"immutable synthetic original").unwrap();
        let alias=root.join("alias.graph.json");
        fs::hard_link(&native,&alias).unwrap();
        assert!(write_atomic(&alias,b"not allowed").is_err());
        assert!(read_bounded(&alias).is_err());
        assert_eq!(fs::read(&native).unwrap(),b"immutable synthetic original");
        assert!(!root.join("alias.graph.json.bak").exists());
        let draft=root.join("client.advanced.json");
        fs::write(&draft,b"previous JSON").unwrap();
        let backup=root.join("client.advanced.json.bak");
        fs::hard_link(&native,&backup).unwrap();
        assert!(write_atomic(&draft,b"new JSON").is_err());
        assert_eq!(fs::read(&draft).unwrap(),b"previous JSON");
        assert_eq!(fs::read(&native).unwrap(),b"immutable synthetic original");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn staged_file_modified_after_flush_cannot_be_committed(){
        let root=folder();
        let path=root.join("fresh.graph.json");
        let staged=StagedFile::create_near(&path,b"expected JSON").unwrap();
        let scratch=staged.path.clone();
        fs::write(&scratch,b"tampered").unwrap();
        assert!(staged.replace(&path).is_err());
        assert!(!path.exists());
        assert!(!scratch.exists(),"failed stage must be removed by Drop");
        fs::remove_dir_all(root).unwrap();
    }


    #[cfg(unix)]
    #[test]
    fn unix_staged_json_and_backup_are_owner_only_even_with_permissive_umask(){
        use std::os::unix::fs::PermissionsExt;
        let root=folder();let path=root.join("sensitive.graph.json");
        fs::write(&path,b"prior synthetic config").unwrap();
        // The original file deliberately starts world-readable.
        fs::set_permissions(&path,fs::Permissions::from_mode(0o644)).unwrap();
        write_atomic(&path,b"new synthetic config").unwrap();
        let mode=|p:&Path|fs::metadata(p).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode(&path),0o600,"replacement must not widen access");
        assert_eq!(mode(&root.join("sensitive.graph.json.bak")),0o600,
            "backup must not leak pre-existing JSON bytes");
        assert_eq!(fs::read(&path).unwrap(),b"new synthetic config");
        assert_eq!(fs::read(root.join("sensitive.graph.json.bak")).unwrap(),
            b"prior synthetic config");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn staged_hardlink_is_rejected_before_rename(){
        let root=folder();let path=root.join("client.graph.json");
        let stage=StagedFile::create_near(&path,b"confidential synthetic JSON").unwrap();
        let alias=root.join("attacker-alias");
        fs::hard_link(&stage.path,&alias).unwrap();
        assert!(stage.replace(&path).is_err(),
            "staged hardlink would allow edits of committed JSON via alias");
        assert!(!path.exists(),"new target must not be created");
        assert_eq!(fs::read(&alias).unwrap(),b"confidential synthetic JSON");
        fs::remove_file(alias).unwrap();
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn staged_cleanup_preserves_foreign_file_swapped_into_temp_path(){
        let root=folder();let destination=root.join("safe.graph.json");
        let stage=StagedFile::create_near(&destination,b"expected data").unwrap();
        let original_stage=root.join("moved-stage");
        let stage_path=stage.path.clone();
        fs::rename(&stage_path,&original_stage).unwrap();
        fs::write(&stage_path,b"foreign data").unwrap();
        drop(stage);
        assert_eq!(fs::read(&stage_path).unwrap(),b"foreign data",
            "stale stage cleanup must not delete a different file ID");
        assert_eq!(fs::read(&original_stage).unwrap(),b"expected data");
        assert!(!destination.exists());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn staged_cleanup_never_follows_link_to_native_source(){
        let root=folder();let native=root.join("CIOPRG.LST");
        let destination=root.join("safe.graph.json");
        fs::write(&native,b"unaltered synthetic native source").unwrap();
        let stage=StagedFile::create_near(&destination,b"expected data").unwrap();
        let previous=root.join("old-stage");
        let temporary=stage.path.clone();
        fs::rename(&temporary,&previous).unwrap();
        // A hard link at the temporary name must not cause removal of
        // an unrelated native file via cleanup.
        fs::hard_link(&native,&temporary).unwrap();
        drop(stage);
        assert_eq!(fs::read(&native).unwrap(),b"unaltered synthetic native source");
        assert!(temporary.exists(),"foreign alias is not our staged file");
        assert!(!destination.exists());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn neither_native_extension_is_accepted_by_json_writer(){
        let dir=folder();
        for name in ["CIOPRG.LST","IONAME.DAT","project.txt"] {
            let path=dir.join(name);assert!(write_atomic(&path,b"{}").is_err());
            assert!(!path.exists());
        }
        fs::remove_dir_all(dir).unwrap();
    }
}
