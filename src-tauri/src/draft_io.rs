//! Offline JSON draft storage: staged writes, same-directory rename, bounded reads.
//!
//! Only used after distinct *.graph.json / *.advanced.json / legacy *.json
//! filename checks. No CIOPRG.LST or IONAME.DAT writer exists.
//!
//! This is a *best-effort per-file atomic replacement*, not a complete
//! transaction for target+backup or a guarantee against adversarial changes
//! of parent directories. Symlink/TOCTOU review remains open for future work.
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub const MAX_JSON_BYTES: usize = 2 * 1024 * 1024;

fn validate_file(path: &Path, allow_missing: bool) -> Result<bool, String> {
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() {
                return Err("Прямые символические ссылки для JSON запрещены".into());
            }
            if !metadata.is_file() {
                return Err("JSON должен быть обычным файлом".into());
            }
            if metadata.len() > MAX_JSON_BYTES as u64 {
                return Err("JSON превышает предел 2 МиБ".into());
            }
            Ok(true)
        }
        Err(err) if err.kind()==std::io::ErrorKind::NotFound && allow_missing => Ok(false),
        Err(err) => Err(format!("Не удалось проверить JSON файл: {err}")),
    }
}

/// A bounded reader also rejects direct symlinks. It does not claim a
/// no-follow open on every filesystem or prohibit symlink ancestors.
pub fn read_bounded(path: &Path) -> Result<Vec<u8>, String> {
    validate_file(path,false)?;
    let handle=File::open(path).map_err(|e|format!("Не удалось открыть JSON: {e}"))?;
    let mut result=Vec::new();
    handle.take((MAX_JSON_BYTES+1) as u64).read_to_end(&mut result)
        .map_err(|e|format!("Ошибка чтения JSON: {e}"))?;
    if result.len()>MAX_JSON_BYTES {
        return Err("JSON превышает предел 2 МиБ".into());
    }
    Ok(result)
}

struct StagedFile(PathBuf);

impl StagedFile {
    fn create_near(destination:&Path, content:&[u8])->Result<Self,String> {
        let basename=destination.file_name()
            .and_then(|s|s.to_str()).ok_or("Недопустимое имя JSON")?;
        let scratch=destination.with_file_name(format!(
            ".{basename}.mlad-{}.tmp",uuid::Uuid::new_v4()
        ));
        let mut handle=OpenOptions::new().write(true).create_new(true)
            .open(&scratch).map_err(|e|format!("Не удалось создать временный JSON: {e}"))?;
        let staged=Self(scratch);
        handle.write_all(content).and_then(|_|handle.sync_all())
            .map_err(|e|format!("Не удалось записать временный JSON: {e}"))?;
        drop(handle);
        Ok(staged)
    }

    fn replace(mut self, destination:&Path)->Result<(),String> {
        fs::rename(&self.0,destination)
            .map_err(|e|format!("Не удалось заменить JSON файлом из той же папки: {e}"))?;
        self.0=PathBuf::new();
        Ok(())
    }
}

impl Drop for StagedFile {
    fn drop(&mut self) {
        if !self.0.as_os_str().is_empty() {
            let _=fs::remove_file(&self.0);
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
    if content.len()>MAX_JSON_BYTES {
        return Err("JSON превышает предел 2 МиБ".into());
    }
    if !path.file_name().and_then(|s|s.to_str())
        .is_some_and(|s|s.ends_with(".json")) {
        return Err("Запись допускается только в JSON черновик".into());
    }
    let existed=validate_file(path,true)?;
    let staged=StagedFile::create_near(path,content)?;
    if existed {
        let original=read_bounded(path)?;
        let basename=path.file_name().and_then(|s|s.to_str())
            .ok_or("Не удалось определить имя JSON")?;
        let backup=path.with_file_name(format!("{basename}.bak"));
        validate_file(&backup,true)?;
        let previous=StagedFile::create_near(&backup,&original)?;
        // Don't replace a newly appeared symlink; parent-dir swaps remain
        // an explicitly documented limitation of std::fs path-based APIs.
        validate_file(&backup,true)?;
        previous.replace(&backup)?;
    }
    validate_file(path,true)?;
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
