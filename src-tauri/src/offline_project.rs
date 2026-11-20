//! Unified *offline* project manifest. Never stores OEM source bytes,
//! absolute paths or cached analysis as if it were controller truth.
//! Linked CIOPRG/IONAME/sidecar remain independent read-only source files.
//! Only the dedicated .mladproj.json metadata file may be written here.
use serde::{Deserialize,Serialize};
use sha2::{Digest,Sha256};
use std::path::{Component,Path,PathBuf};
use crate::compiler::{lossless::LosslessDocument,memory::ControllerFamily};

pub const PROJECT_FORMAT:&str="MLAD_OFFLINE_PROJECT";
pub const PROJECT_VERSION:u32=1;
pub const ANALYSIS_MODEL:&str="SOURCE_ROLE_PREVIEW_V1";
pub const MAX_RELATIVE_BYTES:usize=512;

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub struct AttachedFile {
    pub relative_path:String,
    pub sha256:String,
    pub byte_length:usize,
}
#[derive(Debug,Clone,Serialize,Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub struct NativeSource {
    pub relative_path:String,
    pub sha256:String,
    pub byte_length:usize,
    pub line_count:usize,
    pub controller:ControllerFamily,
}
#[derive(Debug,Clone,Serialize,Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub struct AnalysisSnapshot {
    pub model:String,
    pub source_sha256:String,
    pub typed_candidates:usize,
    pub semantic_rungs:usize,
    pub read_roles:usize,
    pub write_roles:usize,
    pub unknown_roles:usize,
    pub execution_verified:bool,
}
#[derive(Debug,Clone,Serialize,Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub struct OfflineProject {
    pub format:String,
    pub schema_version:u32,
    pub source:NativeSource,
    pub ioname:Option<AttachedFile>,
    pub sidecar:Option<AttachedFile>,
    pub analysis:AnalysisSnapshot,
}
pub fn sha256(bytes:&[u8])->String {hex::encode(Sha256::digest(bytes))}
fn is_sha256(value:&str)->bool {
    value.len()==64 && value.bytes().all(|b|b.is_ascii_hexdigit()&&!b.is_ascii_uppercase())
}
pub fn check_project_path(path:&Path)->Result<(),String>{
    let filename=path.file_name().and_then(|x|x.to_str()).unwrap_or("");
    if !path.is_absolute() || filename.len()<15 || !filename.ends_with(".mladproj.json") ||
        path.to_str().is_none_or(|s|s.len()>1024){
        return Err("Проект должен иметь абсолютный путь *.mladproj.json".into());
    }
    Ok(())
}
pub fn checked_relative(input:&str)->Result<PathBuf,String>{
    if input.is_empty() || input.len()>MAX_RELATIVE_BYTES ||
        input.bytes().any(|x|x==b'\\'||x==b':'||x==0) ||
        input.starts_with("//") {
        return Err("Не переносимый относительный путь компонента MLAD".into());
    }
    let relative=Path::new(input);
    if relative.is_absolute() || !relative.components().all(|c|matches!(c,Component::Normal(_))){
        return Err("Путь компонента должен находиться внутри папки проекта без ../ или ссылок".into());
    }
    Ok(relative.to_path_buf())
}
pub fn resolve(project_path:&Path,relative:&str)->Result<PathBuf,String>{
    check_project_path(project_path)?;
    let rel=checked_relative(relative)?;
    let parent=project_path.parent().ok_or("Нет каталога проекта")?;
    Ok(parent.join(rel))
}
pub fn relative_file(project_path:&Path,target:&Path)->Result<String,String>{
    check_project_path(project_path)?;
    if !target.is_absolute(){
        return Err("Компоненты проекта должны иметь абсолютные пути".into());
    }
    let relative=target.strip_prefix(project_path.parent().ok_or("Нет каталога проекта")?)
        .map_err(|_|"Файл находится вне переносимой папки проекта")?;
    let text=relative.to_str().ok_or("Путь компонента не Unicode")?;
    let normalized=text.replace('\\',"/");
    checked_relative(&normalized)?;
    Ok(normalized)
}
fn is_lst_file(file:&str)->bool {
    Path::new(file).extension().is_some_and(|e|e.to_string_lossy().eq_ignore_ascii_case("lst"))
}
fn is_ioname(file:&str)->bool {
    Path::new(file).file_name().and_then(|n|n.to_str())
        .is_some_and(|n|n.eq_ignore_ascii_case("IONAME.DAT"))
}
fn is_sidecar(file:&str)->bool {
    Path::new(file).file_name().and_then(|n|n.to_str())
        .is_some_and(|n|n.ends_with(".mlad.json"))
}
impl OfflineProject {
    pub fn new(doc:&LosslessDocument,source_path:String,
        ioname:Option<AttachedFile>,sidecar:Option<AttachedFile>,
        analysis:AnalysisSnapshot)->Self {
        Self {
            format:PROJECT_FORMAT.to_owned(),schema_version:PROJECT_VERSION,
            source:NativeSource {
                relative_path:source_path,sha256:doc.source_sha256.clone(),
                byte_length:doc.original_bytes().len(),line_count:doc.lines.len(),
                controller:doc.controller,
            },
            ioname,sidecar,analysis,
        }
    }
    pub fn validate(&self,doc:&LosslessDocument)->Result<(),String>{
        if self.format!=PROJECT_FORMAT || self.schema_version!=PROJECT_VERSION {
            return Err("Неподдерживаемая версия offline-проекта; не выполнять автоматическую миграцию".into());
        }
        checked_relative(&self.source.relative_path)?;
        if !is_lst_file(&self.source.relative_path) ||
           !is_sha256(&self.source.sha256) ||
           self.source.sha256!=doc.source_sha256 ||
           self.source.byte_length!=doc.original_bytes().len() ||
           self.source.line_count!=doc.lines.len() ||
           self.source.controller!=doc.controller {
            return Err("Файл CIOPRG не соответствует манифесту (SHA-256/размер/семейство)".into());
        }
        if self.analysis.model!=ANALYSIS_MODEL ||
           self.analysis.source_sha256!=doc.source_sha256 ||
           self.analysis.execution_verified {
            return Err("Некорректная метка источника/достоверности анализа".into());
        }
        for (kind,item) in [("IONAME",self.ioname.as_ref()),("sidecar",self.sidecar.as_ref())]{
            if let Some(item)=item{
                checked_relative(&item.relative_path)?;
                let correct=if kind=="IONAME" {is_ioname(&item.relative_path)}
                    else {is_sidecar(&item.relative_path)};
                if !correct || !is_sha256(&item.sha256) || item.byte_length==0 ||
                   item.byte_length>if kind=="IONAME"{4*1024*1024}else{2*1024*1024}{
                    return Err(format!("Некорректное вложение {kind}"));
                }
                if item.relative_path==self.source.relative_path ||
                   item.relative_path==self.ioname.as_ref().map(|x|x.relative_path.as_str()).unwrap_or("")
                        && kind=="sidecar"{
                    return Err("Компоненты проекта должны ссылаться на разные файлы".into());
                }
            }
        }
        Ok(())
    }
}
pub fn snapshot(doc:&LosslessDocument)->AnalysisSnapshot {
    let roles=crate::compiler::static_access::inspect(doc,0);
    let sem=crate::compiler::semantic::analyze(doc);
    AnalysisSnapshot {
        model:ANALYSIS_MODEL.to_owned(),source_sha256:doc.source_sha256.clone(),
        typed_candidates:doc.inspection_report(0,0).typed_candidate_count,
        semantic_rungs:sem.rungs.len(),
        read_roles:roles.read_sites,write_roles:roles.write_sites,
        unknown_roles:roles.unknown_sites,execution_verified:false,
    }
}
pub fn attachment(relative_path:String,bytes:&[u8])->AttachedFile{
    AttachedFile{relative_path,sha256:sha256(bytes),byte_length:bytes.len()}
}
pub fn matches_attachment(item:&AttachedFile,bytes:&[u8])->bool{
    item.byte_length==bytes.len() && item.sha256==sha256(bytes)
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::compiler::lossless::parse_document;
    fn sample()->LosslessDocument{
        parse_document(b"STR #70010\r\nOUT #70020\r\nEND\n".to_vec(),ControllerFamily::YRC1000)
    }
    #[test]
    fn project_manifest_roundtrip_excludes_original_source_and_absolute_paths(){
        let doc=sample();
        let mut project=OfflineProject::new(&doc,"CIOPRG.LST".into(),
            Some(attachment("names/IONAME.DAT".into(),b"/IONAME\n//IN\na,,,\n")),
            Some(attachment("notes.mlad.json".into(),b"{\"notes\":true}")),snapshot(&doc));
        project.validate(&doc).unwrap();
        let bytes=serde_json::to_vec_pretty(&project).unwrap();
        assert!(!bytes.windows(b"STR #70010".len()).any(|b|b==b"STR #70010"));
        assert!(!bytes.windows(b"\\\\robot".len()).any(|b|b==b"\\\\robot"));
        let back:OfflineProject=serde_json::from_slice(&bytes).unwrap();
        assert_eq!(back,project);
        assert!(matches_attachment(back.ioname.as_ref().unwrap(),b"/IONAME\n//IN\na,,,\n"));
        assert!(!matches_attachment(back.sidecar.as_ref().unwrap(),b"{\"notes\":false}"));
        project.analysis.read_roles+=1;
        assert_ne!(project.analysis,snapshot(&doc));
    }
    #[test]
    fn corrupted_source_and_unverified_execution_are_rejected(){
        let doc=sample();
        let original=OfflineProject::new(&doc,"CIOPRG.LST".into(),None,None,snapshot(&doc));
        let mut bad=original.clone();bad.source.sha256="0".repeat(64);
        assert!(bad.validate(&doc).is_err());
        bad=original.clone();bad.source.controller=ControllerFamily::DX200;
        assert!(bad.validate(&doc).is_err());
        bad=original.clone();bad.source.line_count+=1;
        assert!(bad.validate(&doc).is_err());
        bad=original.clone();bad.schema_version=99;
        assert!(bad.validate(&doc).is_err());
        bad=original.clone();bad.analysis.execution_verified=true;
        assert!(bad.validate(&doc).is_err());
        bad=original.clone();bad.format="YASKAWA_PROGRAM".into();
        assert!(bad.validate(&doc).is_err());
        let mut json=serde_json::to_value(original).unwrap();
        json["robot_ip"]=serde_json::json!("192.0.2.1");
        assert!(serde_json::from_value::<OfflineProject>(json).is_err());
    }
    #[test]
    fn path_rules_refuse_escape_native_alias_and_nonportable_windows_paths(){
        let root=std::env::temp_dir().join("mlad-project-fixture");
        let manifest=root.join("cell.mladproj.json");
        assert!(check_project_path(&manifest).is_ok());
        assert!(check_project_path(&root.join("CIOPRG.LST")).is_err());
        assert!(check_project_path(&root.join("cell.mlad.json")).is_err());
        for path in ["../CIOPRG.LST","/etc/passwd","C:\\robot\\CIOPRG.LST",
            "a/../IONAME.DAT","./IONAME.DAT","names\\IONAME.DAT","foo//../bar",
            "","CIOPRG.LST:stream"]{
            assert!(checked_relative(path).is_err(),"{path}");
        }
        assert!(checked_relative("cell/CIOPRG.LST").is_ok());
        assert!(relative_file(&manifest,&root.join("cell/CIOPRG.LST")).is_ok());
        assert!(relative_file(&manifest,&root.join("../CIOPRG.LST")).is_err());
        assert!(resolve(&manifest,"names/IONAME.DAT").unwrap().starts_with(&root));
    }
    #[test]
    fn attachments_must_be_distinct_and_kind_checked(){
        let doc=sample();
        let mut m=OfflineProject::new(&doc,"CIOPRG.LST".into(),None,None,snapshot(&doc));
        m.ioname=Some(attachment("CIOPRG.LST".into(),b"x"));
        assert!(m.validate(&doc).is_err());
        m.ioname=Some(attachment("IONAME.DAT".into(),b"x"));
        m.sidecar=Some(attachment("IONAME.DAT".into(),b"x"));
        assert!(m.validate(&doc).is_err());
        m.sidecar=Some(attachment("notes.mlad.json".into(),b"{}"));
        assert!(m.validate(&doc).is_ok());
    }
}
