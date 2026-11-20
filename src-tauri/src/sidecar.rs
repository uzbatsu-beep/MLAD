//! MLAD Portable Offline Sidecar v1: structured notes, NEVER an OEM program.
//! No native CIOPRG.LST/IONAME.DAT text, source path or operational controls
//! are serialized. All annotations refer to a cryptographically pinned source.
use std::collections::{BTreeMap, BTreeSet};
use serde::{Deserialize, Serialize};
use crate::compiler::lossless::LosslessDocument;
use crate::compiler::memory::ControllerFamily;

pub const SIDECAR_FORMAT: &str = "MLAD_OFFLINE_SIDECAR";
pub const SIDECAR_VERSION: u32 = 1;
pub const MAX_NOTES_PER_CATEGORY: usize = 1000;
pub const MAX_BOOKMARKS: usize = 500;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceBinding {
    pub kind: String,
    pub controller: ControllerFamily,
    pub sha256: String,
    pub byte_length: usize,
    pub line_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(deny_unknown_fields)]
pub struct Annotations {
    pub rungs: BTreeMap<String,String>,
    pub addresses: BTreeMap<String,String>,
    pub ioname: BTreeMap<String,String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Bookmark {
    pub line: usize,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct OfflineSidecar {
    pub format: String,
    pub schema_version: u32,
    pub source: SourceBinding,
    pub annotations: Annotations,
    pub bookmarks: Vec<Bookmark>,
}

fn is_hex64(value:&str)->bool {
    value.len()==64 && value.bytes().all(|b|b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}
fn valid_relay(value:&str)->bool {
    let bytes=value.as_bytes();
    (bytes.len()==6 && bytes[0]==b'#' && bytes[1..].iter().all(u8::is_ascii_digit)) ||
    (bytes.len()==4 && bytes[0]==b'M' && bytes[1..].iter().all(u8::is_ascii_digit))
}
fn validate_annotation_text(value:&str)->bool {
    value.chars().count()<=200 && !value.chars().any(|ch|ch.is_control() && ch!='\n' && ch!='\t')
}
fn validate_rung_key(key:&str, max_line:usize)->bool {
    let Some(rest)=key.strip_prefix('L') else {return false;};
    let Some((line,output))=rest.split_once(":OUT") else {return false;};
    let Ok(n)=line.parse::<usize>() else {return false;};
    n>0 && n<=max_line && !output.is_empty() && output.len()<=8 &&
        output.bytes().all(|b|b.is_ascii_digit())
}
fn validate_ioname_key(key:&str)->bool {
    let mut parts=key.split(':');
    let (Some(hash),Some(line),Some(slot),None)=
        (parts.next(),parts.next(),parts.next(),parts.next()) else {return false};
    is_hex64(hash) &&
        line.parse::<usize>().is_ok_and(|n|n>0 && n<=200_000) &&
        slot.parse::<usize>().is_ok_and(|n|n<=200_000)
}

impl OfflineSidecar {
    pub fn new(doc:&LosslessDocument) -> Self {
        Self {
            format: SIDECAR_FORMAT.to_owned(), schema_version: SIDECAR_VERSION,
            source: SourceBinding {
                kind:"CIOPRG.LST".to_owned(),
                controller:doc.controller,
                sha256:doc.source_sha256.clone(),
                byte_length:doc.original_bytes().len(),
                line_count:doc.lines.len(),
            },
            annotations:Annotations::default(),
            bookmarks:Vec::new(),
        }
    }
    pub fn validate(&self,doc:&LosslessDocument)->Result<(),String> {
        if self.format!=SIDECAR_FORMAT || self.schema_version!=SIDECAR_VERSION {
            return Err("Неизвестный формат/версия sidecar; автоматическая миграция запрещена".into());
        }
        if self.source.kind!="CIOPRG.LST" || !is_hex64(&self.source.sha256) ||
           self.source.sha256!=doc.source_sha256 ||
           self.source.controller!=doc.controller ||
           self.source.byte_length!=doc.original_bytes().len() ||
           self.source.line_count!=doc.lines.len() {
            return Err("Sidecar относится к другому исходному CIOPRG (SHA-256, профиль или размер)".into());
        }
        for (category,map) in [
            ("rungs",&self.annotations.rungs),
            ("addresses",&self.annotations.addresses),
            ("ioname",&self.annotations.ioname),
        ] {
            if map.len()>MAX_NOTES_PER_CATEGORY {
                return Err(format!("Превышен лимит комментариев ({category})"));
            }
            for (key,value) in map {
                if key.is_empty() || key.len()>160 || !validate_annotation_text(value) {
                    return Err(format!("Недопустимый комментарий ({category})"));
                }
                let valid=match category {
                    "rungs"=>validate_rung_key(key,doc.lines.len()),
                    "addresses"=>valid_relay(key),
                    "ioname"=>validate_ioname_key(key),
                    _=>false,
                };
                if !valid{return Err(format!("Недопустимый ключ комментария ({category})"));}
            }
        }
        if self.bookmarks.len()>MAX_BOOKMARKS {
            return Err("Sidecar: слишком много закладок".into());
        }
        let mut lines=BTreeSet::new();
        for bookmark in &self.bookmarks {
            if bookmark.line==0 || bookmark.line>doc.lines.len() ||
               bookmark.label.trim().is_empty() || bookmark.label.chars().count()>80 ||
               bookmark.label.chars().any(|ch|ch.is_control()) ||
               !lines.insert(bookmark.line) {
                return Err("Sidecar: повреждённая, дублированная или лишняя закладка".into());
            }
        }
        Ok(())
    }
}

pub fn check_sidecar_path(file_path:&str)->Result<&std::path::Path,String> {
    let path=std::path::Path::new(file_path);
    let filename=path.file_name().and_then(|s|s.to_str()).unwrap_or("");
    if file_path.len()>1024 || filename.len()<11 || !filename.ends_with(".mlad.json") {
        return Err("Sidecar сохраняется только в отдельный файл *.mlad.json".into());
    }
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::lossless::parse_document;
    fn doc(bytes:&[u8]) -> LosslessDocument {
        parse_document(bytes.to_vec(),ControllerFamily::YRC1000)
    }
    fn annotated(d:&LosslessDocument)->OfflineSidecar {
        let mut s=OfflineSidecar::new(d);
        s.annotations.rungs.insert("L2:OUT70020".into(),"Привод 1".into());
        s.annotations.addresses.insert("#70010".into(),"Разрешение".into());
        s.annotations.ioname.insert(
            format!("{}:1:0","a".repeat(64)), "Ручная метка".into()
        );
        s.bookmarks.push(Bookmark{line:2,label:"Цепь управления".into()});
        s
    }
    #[test]
    fn portable_json_roundtrip_preserves_notes_not_original_or_paths() {
        let d=doc(b"STR #70010\r\nOUT #70020\nEND\r");
        let s=annotated(&d);
        s.validate(&d).unwrap();
        let encoded=serde_json::to_vec_pretty(&s).unwrap();
        let back:OfflineSidecar=serde_json::from_slice(&encoded).unwrap();
        assert_eq!(back,s);
        assert!(encoded.windows(b"CIOPRG.LST".len()).any(|x|x==b"CIOPRG.LST"));
        assert!(!encoded.windows(b"STR #70010".len()).any(|x|x==b"STR #70010"));
        assert_eq!(d.copy_original(),b"STR #70010\r\nOUT #70020\nEND\r");
        assert_eq!(back.source.sha256,d.source_sha256);
    }

    #[test]
    fn cyrillic_and_multibyte_notes_use_character_limits_not_utf8_bytes(){
        let d=doc(b"STR #70010\nOUT #70020\nEND\n");
        let mut s=annotated(&d);
        s.annotations.addresses.insert("#70010".into(),"Ю".repeat(200));
        s.bookmarks[0].label="Закладка".repeat(10); // 80 Unicode scalars
        s.validate(&d).unwrap();
        let copy:OfflineSidecar=serde_json::from_slice(
            &serde_json::to_vec(&s).unwrap()
        ).unwrap();
        assert_eq!(copy.annotations.addresses["#70010"].chars().count(),200);
        assert_eq!(copy.bookmarks[0].label.chars().count(),80);
        s.bookmarks[0].label.push('ж');
        assert!(s.validate(&d).is_err());
        s.bookmarks[0].label="OK".into();
        s.annotations.addresses.insert("#70010".into(),"Я".repeat(201));
        assert!(s.validate(&d).is_err());
    }

    #[test]
    fn stale_source_controller_and_size_are_rejected() {
        let d=doc(b"STR #70010\nOUT #70020\nEND\n");
        let mut s=annotated(&d);
        s.source.sha256="0".repeat(64);
        assert!(s.validate(&d).is_err());
        s=annotated(&d);s.source.byte_length+=1;
        assert!(s.validate(&d).is_err());
        s=annotated(&d);s.source.controller=ControllerFamily::DX200;
        assert!(s.validate(&d).is_err());
        s=annotated(&d);s.schema_version=99;
        assert!(s.validate(&d).is_err());
        s=annotated(&d);s.source.kind="IONAME.DAT".into();
        assert!(s.validate(&d).is_err());
    }
    #[test]
    fn invalid_notes_bookmarks_and_unknown_fields_fail_closed() {
        let d=doc(b"STR #70010\nOUT #70020\nEND\n");
        let mut s=annotated(&d);
        s.annotations.addresses.insert("CIOPRG.LST".into(),"bad".into());
        assert!(s.validate(&d).is_err());
        s=annotated(&d);s.annotations.rungs.insert("L999:OUT70020".into(),"bad".into());
        assert!(s.validate(&d).is_err());
        s=annotated(&d);s.annotations.ioname.insert("garbage".into(),"bad".into());
        assert!(s.validate(&d).is_err());
        s=annotated(&d);s.bookmarks.push(Bookmark{line:2,label:"duplicate".into()});
        assert!(s.validate(&d).is_err());
        s=annotated(&d);s.bookmarks.push(Bookmark{line:999,label:"bad".into()});
        assert!(s.validate(&d).is_err());
        s=annotated(&d);s.annotations.addresses.insert("M010".into(),"X".repeat(201));
        assert!(s.validate(&d).is_err());
        let mut json=serde_json::to_value(annotated(&d)).unwrap();
        json["native_writer_enabled"]=serde_json::json!(true);
        assert!(serde_json::from_value::<OfflineSidecar>(json).is_err());
    }
    #[test]
    fn paths_are_isolated_from_native_and_other_json_editors() {
        for bad in ["CIOPRG.LST","IONAME.DAT","source.graph.json",
                    "draft.advanced.json","project.json",".mlad.json",
                    "nested/robot.mlad.json.tmp"] {
            assert!(check_sidecar_path(bad).is_err(),"{bad}");
        }
        assert!(check_sidecar_path("case.mlad.json").is_ok());
        assert!(check_sidecar_path("folder/CIOPRG.mlad.json").is_ok());
    }
}
