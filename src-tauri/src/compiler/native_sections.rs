//! Read-only navigation markers from the original byte snapshot.
//! These describe *textual markers*, NEVER verified System/User ownership.
use serde::Serialize;
use super::lossless::LosslessDocument;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SourceSectionMarker {
    pub line: usize,
    pub category: &'static str,
    pub label: String,
}

fn trim_space(mut line: &[u8]) -> &[u8] {
    while line.first().is_some_and(|b| *b == b' ' || *b == b'\t') { line = &line[1..]; }
    while line.last().is_some_and(|b| *b == b' ' || *b == b'\t') { line = &line[..line.len()-1]; }
    line
}

/// Recognizes only section headers observed in vendor exports, without changing
/// native parser semantics. All bytes remain in the original document.
pub fn scan(doc: &LosslessDocument) -> Vec<SourceSectionMarker> {
    let mut markers = Vec::new();
    for line in &doc.lines {
        let bytes = trim_space(&doc.original_bytes()[line.body_span.start..line.body_span.end]);
        let category = if bytes.starts_with(b"/CIOPRG") { Some("Header") }
            else if bytes == b"//$M" { Some("MHeader") }
            else if bytes.starts_with(b"PART ") && bytes[5..].iter().all(u8::is_ascii_digit)
                && bytes.len() > 5 { Some("Part") }
            else if bytes == b"END" { Some("End") }
            else if bytes.starts_with(b"//ALARM") && bytes.len() <= 32
                && bytes[7..].iter().all(u8::is_ascii_digit) && bytes.len() > 7 { Some("Alarm") }
            else if bytes.starts_with(b"//MESSAGE") && bytes.len() <= 32
                && bytes[9..].iter().all(u8::is_ascii_digit) && bytes.len() > 9 { Some("Message") }
            else { None };
        if let Some(category) = category {
            markers.push(SourceSectionMarker {
                line: line.number,
                category,
                label: String::from_utf8_lossy(&bytes[..bytes.len().min(80)]).into_owned(),
            });
            if markers.len() >= 1000 { break; }
        }
    }
    markers
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::{lossless::parse_document, memory::ControllerFamily};

    #[test]
    fn marks_part_end_and_alarm_message_without_inferring_permissions() {
        let b = b"/CIOPRG DX\r\n//$M\r\nPART 1\r\nSTR #20010\r\nEND\r\n//ALARM1\r\nabc\r\n//MESSAGE2\r\n";
        let doc = parse_document(b.to_vec(), ControllerFamily::YRC1000);
        let actual = scan(&doc);
        assert_eq!(actual.iter().map(|x| (x.line,x.category)).collect::<Vec<_>>(), vec![
            (1,"Header"),(2,"MHeader"),(3,"Part"),(5,"End"),(6,"Alarm"),(8,"Message"),
        ]);
        assert_eq!(doc.copy_original(), b);
        assert!(!doc.inspection_report(100,100).system_user_boundaries_verified);
    }

    #[test]
    fn misleading_similar_prefixes_are_not_navigation_sections() {
        let b = b"PART A\nENDING\n//ALARMred\n//MESSAGEstuff\nEND\nPART 2\n";
        let doc = parse_document(b.to_vec(), ControllerFamily::DX100);
        let markers = scan(&doc);
        assert_eq!(markers.len(), 2);
        assert_eq!(markers[0].category,"End");
        assert_eq!(markers[1].category,"Part");
        assert_eq!(doc.copy_original(), b);
    }
}
