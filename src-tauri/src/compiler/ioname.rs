//! Read-only, byte-preserving preview of Yaskawa IONAME.DAT.
 //! This is a structural parser, NOT a verified mapping to CIO #XXXXX relays.
 //! The vendor file is NEVER rewritten or exported from this module.
 use serde::Serialize;
 use sha2::{Digest,Sha256};

 const MAX_LABELS: usize = 16_000;
 const MAX_WARNINGS: usize = 100;

 #[derive(Debug, Clone, Serialize)]
 pub struct IonameEntry {
     pub section: String,
     pub ordinal: usize,
     pub line: usize,
     pub slot: usize,
     pub label: String,
 }
 #[derive(Debug, Clone, Serialize)]
 pub struct IonameReport {
     pub source_sha256: String,
     pub byte_length: usize,
     pub encoding: &'static str,
     pub header_valid: bool,
     pub entries: Vec<IonameEntry>,
     pub entries_truncated: bool,
     pub warnings: Vec<String>,
     pub original_only: bool,
     pub native_write_allowed: bool,
     pub relay_mapping_verified: bool,
 }
 fn strip_cr(data:&[u8])->&[u8] { data.strip_suffix(b"\r").unwrap_or(data) }
 fn safe_display(data:&[u8])->String {
     // Display is allowed to be lossy. Never re-serialize it to IONAME.DAT.
     String::from_utf8_lossy(data).trim().to_string()
 }
 pub fn inspect(bytes:&[u8])->IonameReport {
     let sha = hex::encode(Sha256::digest(bytes));
     let encoding=if bytes.starts_with(&[0xef,0xbb,0xbf]){"Utf8Bom"}
         else if std::str::from_utf8(bytes).is_ok(){"Utf8"} else {"OpaqueBytes"};
     let mut sections=String::new();
     let mut entries=Vec::new();
     let mut warnings=Vec::new();
     let mut seen_header=false;
     let mut seen_section=false;
     let mut section_ordinal=0usize;
     let mut truncated=false;
     // This is a structural 4-columns-per-row preview, not an address parser.
     for (index,raw_line) in bytes.split(|b|*b==b'\n').enumerate() {
         let raw=strip_cr(raw_line);
         if index==0 && raw.starts_with(&[0xef,0xbb,0xbf]) {
             if &raw[3..]==b"/IONAME" {seen_header=true;continue}
         }
         if raw==b"/IONAME" {seen_header=true;continue}
         if raw.starts_with(b"//") {
             sections=safe_display(raw);
             seen_section=true;
             section_ordinal=0;
             continue;
         }
         if raw.is_empty() {continue}
         if !seen_header || !seen_section {
             if warnings.len()<MAX_WARNINGS {
                 warnings.push(format!("Строка {}: нет проверенного заголовка /IONAME и секции",index+1));
             }
             continue;
         }
         let columns=raw.split(|b|*b==b',').collect::<Vec<_>>();
         if columns.len()!=4 {
             if warnings.len()<MAX_WARNINGS {
                 warnings.push(format!("Строка {}: {} поля вместо четырёх; исходные байты не изменены",index+1,columns.len()));
             }
             // Unsupported row is deliberately not mapped to I/O slots.
             continue;
         }
         if entries.len()+4>MAX_LABELS { truncated=true;break }
         for (idx,value) in columns.iter().enumerate() {
             let ordinal=section_ordinal+idx+1;
             entries.push(IonameEntry {
                 section:sections.clone(),
                 ordinal,
                 line:index+1,
                 slot:idx+1,
                 label:safe_display(value),
             });
         }
         section_ordinal+=4;
     }
     if !seen_header {warnings.push("Не найден точный заголовок /IONAME".into())}
     if !seen_section {warnings.push("Не обнаружены секции //...".into())}
     IonameReport {
         source_sha256:sha,byte_length:bytes.len(),encoding,
         header_valid:seen_header && seen_section,entries,entries_truncated:truncated,warnings,
         original_only:true,native_write_allowed:false,relay_mapping_verified:false,
     }
 }

 #[cfg(test)]
 mod tests {
     use super::*;
     #[test]
     fn parses_four_names_per_row_without_assuming_relay_numbers() {
         let src=b"/IONAME\r\n//IN\r\nIn One,,In Three,\r\n//OUT\r\nOut One,Out Two,,Out Four\r\n";
         let copy=src.to_vec();
         let p=inspect(src);
         assert_eq!(p.entries.len(),8);
         assert_eq!(p.entries[0].section,"//IN");
         assert_eq!(p.entries[0].ordinal,1);
         assert_eq!(p.entries[2].label,"In Three");
         assert_eq!(p.entries[4].section,"//OUT");
         assert_eq!(p.entries[7].ordinal,4);
         assert!(!p.relay_mapping_verified);
         assert!(!p.native_write_allowed);
         assert_eq!(src.as_slice(),copy.as_slice());
     }
     #[test]
     fn rejects_unverified_csv_format_and_keeps_unknown_source() {
         let src=b"IN0023,CONVEYOR_ESTOP_OK\r\n";
         let r=inspect(src);
         assert!(!r.header_valid);
         assert!(r.entries.is_empty());
         assert!(!r.warnings.is_empty());
     }
     #[test]
     fn malformed_rows_are_not_forced_into_address_mapping() {
         let src=b"/IONAME\n//IN\none,two,three\nok,,,\n";
         let r=inspect(src);
         assert_eq!(r.entries.len(),4);
         assert!(r.warnings.iter().any(|w|w.contains("вместо четырёх")));
     }
     #[test]
     fn non_utf8_is_display_only_and_hash_is_of_exact_original() {
         let src=b"/IONAME\r\n//IN\r\n\xff,,,\r\n";
         let r=inspect(src);
         assert_eq!(r.encoding,"OpaqueBytes");
         assert_eq!(r.source_sha256,hex::encode(Sha256::digest(src)));
         assert!(r.entries[0].label.contains('\u{fffd}'));
     }
 }
