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
    use std::os::windows::fs::MetadataExt;
    match (a.volume_serial_number(),a.file_index(),
           b.volume_serial_number(),b.file_index()) {
        (Some(av),Some(ai),Some(bv),Some(bi))=>av==bv && ai==bi,
        _=>false,
    }
}
#[cfg(not(any(windows,unix)))]
fn same_identity(_a:&Metadata,_b:&Metadata)->bool {false}

/// Limit allocation even if the file grows after a metadata inspection.
/// Reject changed path identity/size/mtime and symlinks, including Windows
/// reparse points. Filesystems lacking reliable identifiers fail closed.
pub fn read_regular(path:&Path,max:usize)->Result<Vec<u8>,String>{
    crate::draft_io::verify_parent_directory(path)?;
    let before=fs::symlink_metadata(path)
        .map_err(|e|format!("Ошибка проверки файла: {e}"))?;
    if is_indirection(&before)||!before.is_file(){
        return Err("Ожидается обычный файл без ссылок/reparse".into());
    }
    if before.len()>max as u64 {
        return Err("Размер файла превышает разрешённый предел".into());
    }
    let handle=OpenOptions::new().read(true).open(path)
        .map_err(|e|format!("Ошибка открытия файла: {e}"))?;
    let opened=handle.metadata()
        .map_err(|e|format!("Ошибка проверки открытого файла: {e}"))?;
    if is_indirection(&opened)||!opened.is_file()||
       !same_identity(&before,&opened) {
        return Err("Файл изменился между проверкой и открытием".into());
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
    fn windows_regular_file_has_identifiers_and_no_reparse_flag(){
        let root=dir();let path=root.join("test.graph.json");
        fs::write(&path,b"synthetic").unwrap();
        let before=fs::symlink_metadata(&path).unwrap();
        let opened=fs::File::open(&path).unwrap().metadata().unwrap();
        assert!(same_identity(&before,&opened));
        assert!(!is_indirection(&before));
        assert_eq!(read_regular(&path,1024).unwrap(),b"synthetic");
        fs::remove_dir_all(root).unwrap();
    }
}
