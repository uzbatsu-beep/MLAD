//! Read-only, SHA-pinned native source filtering. Only source-text matches,
 //! never a controller scan/semantic evaluation or native writer.
use serde::Serialize;
use crate::compiler::lossless::{LineKind, LosslessDocument, SyntaxClass};

const FILTER_PAGE_SIZE: usize = 100;
const MAX_SOURCE_LINES: usize = 200_000;

#[derive(Debug, Serialize)]
pub struct NativeFilterHit {
    pub line: usize,
    pub byte_offset: usize,
    pub syntax_class: SyntaxClass,
    pub preview: String,
}

#[derive(Debug, Serialize)]
pub struct NativeFilterPage {
    pub source_sha256: String,
    pub line_count: usize,
    pub offset: usize,
    pub total_matches: usize,
    pub next_offset: Option<usize>,
    pub hits: Vec<NativeFilterHit>,
}

fn class_filter(raw: Option<&str>) -> Result<Option<SyntaxClass>, String> {
    match raw.unwrap_or("").trim() {
        "" => Ok(None),
        "Recognized" => Ok(Some(SyntaxClass::Recognized)),
        "TypedCandidate" => Ok(Some(SyntaxClass::TypedCandidate)),
        "Unsupported" => Ok(Some(SyntaxClass::Unsupported)),
        "Malformed" => Ok(Some(SyntaxClass::Malformed)),
        "Opaque" => Ok(Some(SyntaxClass::Opaque)),
        "PostEndData" => Ok(Some(SyntaxClass::PostEndData)),
        "Structural" => Ok(Some(SyntaxClass::Structural)),
        _ => Err("Неизвестная категория фильтра CIOPRG".into()),
    }
}
fn opcode_filter(raw: Option<&str>) -> Result<String, String> {
    let text = raw.unwrap_or("").trim().to_ascii_uppercase();
    if !text.is_empty() && (text.len() > 20 || !text.as_bytes()[0].is_ascii_uppercase() ||
        !text.bytes().all(|b| b.is_ascii_uppercase() || b == b'-')) {
        return Err("Opcode: только ASCII A-Z и дефис, не более 20 символов".into());
    }
    Ok(text)
}
fn address_filter(raw: Option<&str>) -> Result<String, String> {
    let text = raw.unwrap_or("").trim().to_ascii_uppercase();
    let bytes = text.as_bytes();
    let valid = text.is_empty() ||
        (bytes.len() == 6 && bytes[0] == b'#' && bytes[1..].iter().all(u8::is_ascii_digit)) ||
        (bytes.len() == 4 && bytes[0] == b'M' && bytes[1..].iter().all(u8::is_ascii_digit));
    if !valid {
        return Err("Адрес: ожидается точный текстовый токен #XXXXX либо Mxxx".into());
    }
    Ok(text)
}
fn opcode_of(kind: &LineKind) -> Option<&str> {
    match kind {
        LineKind::Instruction(inst) | LineKind::TypedCandidate { instruction: inst, .. } =>
            Some(inst.mnemonic.as_str()),
        LineKind::UnsupportedInstruction { mnemonic } => Some(mnemonic.as_str()),
        _ => None,
    }
}
fn token_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'#' || b == b'_'
}
fn contains_token(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.len() > haystack.len() { return false; }
    haystack.windows(needle.len()).enumerate().any(|(i, window)| {
        window.eq_ignore_ascii_case(needle) &&
        (i == 0 || !token_char(haystack[i-1])) &&
        (i+needle.len() == haystack.len() || !token_char(haystack[i+needle.len()]))
    })
}

/// Filters ALL source lines, but returns at most 100 hits per page. No source
/// mutation or hypothetical semantic interpretation is performed.
pub fn filter_source(
    doc: &LosslessDocument,
    category: Option<&str>,
    opcode: Option<&str>,
    address: Option<&str>,
    offset: usize,
) -> Result<NativeFilterPage, String> {
    if offset > MAX_SOURCE_LINES {
        return Err("Фильтр: смещение страницы превышает лимит строк".into());
    }
    let category = class_filter(category)?;
    let opcode = opcode_filter(opcode)?;
    let address = address_filter(address)?;
    if category.is_none() && opcode.is_empty() && address.is_empty() {
        return Err("Укажите категорию, opcode или адрес для фильтрации".into());
    }
    let mut total_matches = 0usize;
    let mut hits = Vec::with_capacity(FILTER_PAGE_SIZE);
    for line in &doc.lines {
        let class = SyntaxClass::of(&line.kind);
        if category.is_some_and(|chosen| chosen != class) { continue; }
        if !opcode.is_empty() && !opcode_of(&line.kind).is_some_and(
            |value| value.eq_ignore_ascii_case(&opcode)
        ) { continue; }
        let body = &doc.original_bytes()[line.body_span.start..line.body_span.end];
        // This is a *textual* address filter, including comments/data if present.
        // It is NOT evidence of a live read/write reference.
        if !address.is_empty() && !contains_token(body, address.as_bytes()) { continue; }
        if total_matches >= offset && hits.len() < FILTER_PAGE_SIZE {
            hits.push(NativeFilterHit {
                line: line.number,
                byte_offset: line.body_span.start,
                syntax_class: class,
                preview: String::from_utf8_lossy(&body[..body.len().min(180)]).into_owned(),
            });
        }
        total_matches += 1;
    }
    let next = offset.saturating_add(hits.len());
    Ok(NativeFilterPage {
        source_sha256: doc.source_sha256.clone(),
        line_count: doc.lines.len(),
        offset,
        total_matches,
        next_offset: if next < total_matches { Some(next) } else { None },
        hits,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::lossless::parse_document;
    use crate::compiler::memory::ControllerFamily;

    fn parse(source: &[u8]) -> LosslessDocument {
        parse_document(source.to_vec(), ControllerFamily::YRC1000)
    }
    #[test]
    fn long_source_filter_pages_preserve_physical_lines_and_source_hash() {
        let mut source = b"// SYNTHETIC\r\n".repeat(300);
        source.extend_from_slice(&b"STR #70010\r\nOUT #70020\n".repeat(700));
        source.extend_from_slice(b"END\n//MESSAGE1\ntext\n");
        let doc = parse(&source);
        let sha = doc.source_sha256.clone();
        let first = filter_source(&doc, Some("Recognized"), Some("STR"), Some("#70010"), 0).unwrap();
        assert_eq!(first.line_count, 1703);
        assert_eq!(first.total_matches, 700);
        assert_eq!(first.hits.len(), 100);
        assert_eq!(first.hits[0].line, 301);
        assert_eq!(first.next_offset, Some(100));
        assert_eq!(first.hits[0].byte_offset, doc.lines[300].body_span.start);
        let last = filter_source(&doc, Some("Recognized"), Some("STR"), Some("#70010"), 600).unwrap();
        assert_eq!(last.hits.len(), 100);
        assert_eq!(last.next_offset, None);
        assert_eq!(last.hits[99].line, 1699);
        assert_eq!(doc.original_bytes(), source);
        assert_eq!(doc.source_sha256, sha);
        assert!(!doc.inspection_report(1,1).edited_export_allowed);
    }
    #[test]
    fn ambiguous_and_unsupported_syntax_remains_textual() {
        let doc = parse(b"STR #70010\nFOO #70010\n// #70010\nSTR #700100\nEND\n");
        let unsupported = filter_source(&doc, Some("Unsupported"), Some("FOO"), Some("#70010"), 0).unwrap();
        assert_eq!(unsupported.hits.len(), 1);
        assert_eq!(unsupported.hits[0].line, 2);
        let textual = filter_source(&doc, None, None, Some("#70010"), 0).unwrap();
        assert_eq!(textual.hits.iter().map(|h|h.line).collect::<Vec<_>>(),vec![1,2,3]);
        assert!(filter_source(&doc, Some("unsupported"), None, None, 0).is_err());
        assert!(filter_source(&doc, None, Some("BAD_OP"), None, 0).is_err());
        assert!(filter_source(&doc, None, None, Some("#7001"), 0).is_err());
        assert!(filter_source(&doc, None, None, None, 0).is_err());
        assert!(filter_source(&doc, None, Some("STR"), None, 200_001).is_err());
    }
    #[test]
    fn malformed_and_post_end_categories_are_not_hidden() {
        let doc = parse(b"STR #70018\nEND\n//ALARM1\npayload\n");
        let malformed = filter_source(&doc, Some("Malformed"), None, None, 0).unwrap();
        assert_eq!(malformed.hits[0].line, 1);
        let tail = filter_source(&doc, Some("PostEndData"), None, None, 0).unwrap();
        assert_eq!(tail.hits.iter().map(|x|x.line).collect::<Vec<_>>(),vec![3,4]);
        assert_eq!(filter_source(&doc, Some("PostEndData"), None, None, 1).unwrap().hits[0].line,4);
        assert!(filter_source(&doc, Some("PostEndData"), None, None, 200).unwrap().hits.is_empty());
    }
}
