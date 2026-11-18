//! Race-resistant bounded reads of ordinary OFFLINE files.
//!
//! Compares verified path identity with a handle opened once, and rechecks
//! after reading. This closes common check/open file swaps, but is NOT a
//! directory-handle API or a proof against adversarial parent-path TOCTOU.
use std::fs::{self, Metadata, OpenOptions};
use std::io::Read;
use std::path::Path;

pub fn is_indirection(meta:&Metadata)->bool {
    if meta.file_type().is_symlink(){return true;}
    #[cfg(windows)] {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT:u32=0x400;
        return meta.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0;
    }
    #[cfg(not(windows))]
    { false }
}
#[cfg(unix)]
fn same_identity(a:&Metadata,b:&Metadata)->bool {
    use std::os::unix::fs::MetadataExt;
    a.dev()==b.dev() && a.ino()==b.ino()
}
#[cfg(windows)]
fn same_identity(a:&Metadata,b:&Metadata)->bool {
    // MetadataExt::file_index()/volume_serial_number() are still nightly-only.
    // These metadata comparisons are secondary checks, not a file identity.
    a.len()==b.len() && a.modified().ok()==b.modified().ok()
}

#[cfg(windows)]
fn win_open_no_reparse(path:&Path)->Result<std::fs::File,String> {
    use std::os::windows::fs::OpenOptionsExt;
    // A direct reparse point must be opened AS the point, not followed.
    const FILE_FLAG_OPEN_REPARSE_POINT:u32=0x0020_0000;
    let file=OpenOptions::new().read(true)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT).open(path)
        .map_err(|e|format!("Ошибка Windows OPEN_REPARSE_POINT: {e}"))?;
    let metadata=file.metadata().map_err(|e|format!("Ошибка Windows metadata: {e}"))?;
    if is_indirection(&metadata)||!metadata.is_file() {
        return Err("Windows reparse point или нестандартный файл запрещён".into());
    }
    Ok(file)
}
#[cfg(windows)]
fn win_file_id(file:&std::fs::File)->Result<(u32,u64),String>{
    use std::ffi::c_void;
    use std::mem::MaybeUninit;
    use std::os::windows::io::AsRawHandle;
    #[repr(C)]
    struct WinFileTime { low:u32, high:u32 }
    #[repr(C)]
    struct WinFileInfo {
        attributes:u32,
        created:WinFileTime,
        last_access:WinFileTime,
        last_write:WinFileTime,
        volume:u32,
        size_high:u32,
        size_low:u32,
        links:u32,
        index_high:u32,
        index_low:u32,
    }
    #[link(name="kernel32")]
    unsafe extern "system" {
        fn GetFileInformationByHandle(
            file:*mut c_void, info:*mut WinFileInfo
        )->i32;
    }
    let mut result=MaybeUninit::<WinFileInfo>::uninit();
    // SAFETY: The opened std::fs::File owns a valid Windows HANDLE.
    // WinFileInfo layout exactly mirrors BY_HANDLE_FILE_INFORMATION (52 bytes).
    // The OS writes all fields iff the call returns nonzero.
    let ok=unsafe {
        GetFileInformationByHandle(file.as_raw_handle(),result.as_mut_ptr())
    };
    if ok==0 {
        return Err(format!("Windows File ID недоступен: {}",
            std::io::Error::last_os_error()));
    }
    let info=unsafe{result.assume_init()};
    Ok((info.volume,(u64::from(info.index_high)<<32)|u64::from(info.index_low)))
}
#[cfg(not(any(windows,unix)))]
fn same_identity(_a:&Metadata,_b:&Metadata)->bool {false}

/// Limit allocation even if the file grows after a metadata inspection.
/// Reject changed path identity/size/mtime and symlinks, including Windows
/// reparse points. Filesystems lacking reliable identifiers fail closed.
pub fn read_regular(path:&Path,max:usize)->Result<Vec<u8>,String>{
    read_regular_inner(path,max,||{})
}

fn read_regular_inner(path:&Path,max:usize,before_open:impl FnOnce())->Result<Vec<u8>,String>{
    crate::draft_io::verify_parent_directory(path)?;
    let before=fs::symlink_metadata(path)
        .map_err(|e|format!("Ошибка проверки файла: {e}"))?;
    if is_indirection(&before)||!before.is_file(){
        return Err("Ожидается обычный файл без ссылок/reparse".into());
    }
    if before.len()>max as u64 {
        return Err("Размер файла превышает разрешённый предел".into());
    }
    before_open();
    #[cfg(windows)]
    let before_win=win_open_no_reparse(path)?;
    #[cfg(windows)]
    let handle=win_open_no_reparse(path)?;
    #[cfg(not(windows))]
    let handle=OpenOptions::new().read(true).open(path)
        .map_err(|e|format!("Ошибка открытия файла: {e}"))?;
    let opened=handle.metadata()
        .map_err(|e|format!("Ошибка проверки открытого файла: {e}"))?;
    if is_indirection(&opened)||!opened.is_file()||
       !same_identity(&before,&opened) {
        return Err("Файл изменился между проверкой и открытием".into());
    }
    #[cfg(windows)]
    if win_file_id(&before_win)? != win_file_id(&handle)? {
        return Err("Windows File ID изменился между двумя открытиями".into());
    }
    let mut bytes=Vec::new();
    (&handle).take((max+1) as u64).read_to_end(&mut bytes)
        .map_err(|e|format!("Ошибка чтения файла: {e}"))?;
    if bytes.len()>max {
        return Err("Файл вырос за предел допустимого размера".into());
    }
    let after_handle=handle.metadata()
        .map_err(|e|format!("Ошибка повторной проверки дескриптора: {e}"))?;
    crate::draft_io::verify_parent_directory(path)?;
    let after_path=fs::symlink_metadata(path)
        .map_err(|e|format!("Файл исчез при чтении: {e}"))?;
    if is_indirection(&after_path)||!after_path.is_file()||
       !same_identity(&before,&after_path)||
       !same_identity(&opened,&after_handle)||
       opened.len()!=after_handle.len()||
       opened.modified().ok()!=after_handle.modified().ok() {
        return Err("Файл заменён или изменён при чтении".into());
    }
    #[cfg(windows)] {
        let after_win=win_open_no_reparse(path)?;
        if win_file_id(&after_win)? != win_file_id(&handle)? {
            return Err("Windows File ID изменился после чтения".into());
        }
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    fn dir()->PathBuf {
        let p=std::env::temp_dir().join(format!("mlad-handle-{}",uuid::Uuid::new_v4()));
        fs::create_dir_all(&p).unwrap();p
    }
    #[test]
    fn regular_read_preserves_exact_bytes(){
        let root=dir();let p=root.join("test.graph.json");
        let data=b"synthetic\r\n\x00\xff";
        fs::write(&p,data).unwrap();
        assert_eq!(read_regular(&p,100).unwrap(),data);
        assert!(read_regular(&p,2).is_err());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn substituted_file_between_metadata_and_open_fails_closed(){
        let root=dir();let path=root.join("draft.graph.json");
        let displaced=root.join("original.snapshot");
        fs::write(&path,b"first").unwrap();
        let result=read_regular_inner(&path,100,||{
            fs::rename(&path,&displaced).unwrap();
            fs::write(&path,b"second").unwrap();
        });
        assert!(result.is_err(),"a substituted file must not pass identity check");
        assert_eq!(fs::read(&path).unwrap(),b"second");
        assert_eq!(fs::read(&displaced).unwrap(),b"first");
        fs::remove_dir_all(root).unwrap();
    }
    #[cfg(unix)]
    #[test]
    fn direct_and_parent_symlinks_are_rejected_without_modification(){
        use std::os::unix::fs::symlink;
        let root=dir();let folder=root.join("real");
        fs::create_dir(&folder).unwrap();
        let real=folder.join("CIOPRG.LST");
        fs::write(&real,b"robot bytes synthetic").unwrap();
        symlink(&real,folder.join("alias.LST")).unwrap();
        symlink(&folder,root.join("linked")).unwrap();
        assert!(read_regular(&folder.join("alias.LST"),1024).is_err());
        assert!(read_regular(&root.join("linked").join("CIOPRG.LST"),1024).is_err());
        assert_eq!(fs::read(&real).unwrap(),b"robot bytes synthetic");
        fs::remove_dir_all(root).unwrap();
    }
    #[cfg(windows)]
    #[test]
    fn windows_junction_parent_rejects_native_read_and_preserves_bytes(){
        use std::process::Command;
        let root=dir();
        let real=root.join("normal");
        fs::create_dir(&real).unwrap();
        let original=real.join("CIOPRG.LST");
        let data=b"synthetic offline sample\r\nEND\r\n";
        fs::write(&original,data).unwrap();
        let junction=root.join("junction");
        let output=Command::new("cmd").args(["/C","mklink","/J"])
            .arg(&junction).arg(&real).output().unwrap();
        assert!(output.status.success(),"mklink /J failed: {}",
            String::from_utf8_lossy(&output.stderr));
        assert!(read_regular(&junction.join("CIOPRG.LST"),4096).is_err());
        assert_eq!(fs::read(&original).unwrap(),data);
        assert!(read_regular(&original,4096).is_ok());
        fs::remove_dir(&junction).unwrap();
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(windows)]
    #[test]
    fn windows_regular_file_has_identifiers_and_no_reparse_flag(){
        let root=dir();let path=root.join("test.graph.json");
        fs::write(&path,b"synthetic").unwrap();
        let before=fs::symlink_metadata(&path).unwrap();
        let opened=fs::File::open(&path).unwrap().metadata().unwrap();
        assert!(same_identity(&before,&opened));
        assert!(!is_indirection(&before));
        assert_eq!(win_file_id(&win_open_no_reparse(&path).unwrap()).unwrap(),
                   win_file_id(&fs::File::open(&path).unwrap()).unwrap());
        assert_eq!(read_regular(&path,1024).unwrap(),b"synthetic");
        fs::remove_dir_all(root).unwrap();
    }
}
