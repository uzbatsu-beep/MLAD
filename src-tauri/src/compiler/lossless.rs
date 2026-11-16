//! Lossless, read-only CIOPRG.LST parser.
//!
//! This module scans raw bytes into an ordered sequence of source lines and
//! overlays a *limited* typed syntax tree. No unknown text is discarded.
//! It intentionally does not infer SYSTEM/USER ownership or produce editable
//! controller programs. See docs/cioprg_lossless_parser.md.

use serde::Serialize;
use sha2::{Digest, Sha256};
use super::memory::{
    parse_group, parse_register, parse_relay, ControllerFamily, RelayGroupWidth,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ByteSpan {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum LineEnding { Lf, CrLf, Cr, None }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum EncodingObservation { Utf8, Utf8Bom, OpaqueBytes }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Severity { Warning, Error }

#[derive(Debug, Clone, Serialize)]
pub struct ParseDiagnostic {
    pub code: &'static str,
    pub severity: Severity,
    pub line: usize,
    pub byte_offset: usize,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum OperandKind {
    RelayBit,
    RelayByte,
    RelayWord,
    MRegister,
    DecimalLiteral,
    Unrecognized,
}

/// A syntax-only operand: token byte locations are kept for later AST work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AstOperand {
    pub spelling: String,
    pub kind: OperandKind,
    pub span: ByteSpan,
}

/// An overlay on the source, never a re-serialized replacement.
#[derive(Debug, Clone, Serialize)]
pub struct AstInstruction {
    pub mnemonic: String,
    pub operands: Vec<AstOperand>,
    pub line: usize,
    pub span: ByteSpan,
    pub syntax_valid: bool,
}

/// Unknown lines, unknown instructions and directives remain in raw storage.
#[derive(Debug, Clone, Serialize)]
pub enum LineKind {
    Empty,
    Comment,
    Directive,
    MHeaderRow { row: usize, valid: bool },
    Instruction(AstInstruction),
    UnsupportedInstruction { mnemonic: String },
    Opaque,
}

#[derive(Debug, Clone, Serialize)]
pub struct SourceLine {
    /// One-based line number, stable within this snapshot.
    pub number: usize,
    /// Entire line including original terminator bytes.
    pub full_span: ByteSpan,
    /// Entire line excluding its terminator.
    pub body_span: ByteSpan,
    pub ending: LineEnding,
    pub kind: LineKind,
}

#[derive(Debug, Clone, Serialize)]
pub struct MHeaderInfo {
    pub marker_line: usize,
    pub data_lines: Vec<usize>,
    pub complete: bool,
}

/// Exact immutable snapshot. Raw content is deliberately not Serializable.
#[derive(Debug, Clone)]
pub struct LosslessDocument {
    raw: Vec<u8>,
    pub source_sha256: String,
    pub controller: ControllerFamily,
    pub encoding: EncodingObservation,
    pub lines: Vec<SourceLine>,
    pub m_headers: Vec<MHeaderInfo>,
    pub diagnostics: Vec<ParseDiagnostic>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LinePreview {
    pub line: usize,
    pub ending: LineEnding,
    pub body_span: ByteSpan,
    pub kind: LineKind,
    /// Display text is lossy when source is not UTF-8. NEVER use for export.
    pub display_text: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct InspectionReport {
    pub controller: ControllerFamily,
    pub encoding: EncodingObservation,
    pub byte_length: usize,
    pub source_sha256: String,
    pub line_count: usize,
    pub recognized_instructions: usize,
    pub unsupported_lines: usize,
    pub m_headers: Vec<MHeaderInfo>,
    pub diagnostics: Vec<ParseDiagnostic>,
    pub lines: Vec<LinePreview>,
    pub preview_truncated: bool,
    pub original_round_trip_only: bool,
    pub edited_export_allowed: bool,
    pub system_user_boundaries_verified: bool,
}

impl LosslessDocument {
    pub fn original_bytes(&self) -> &[u8] { &self.raw }

    /// The *only* supported export: an unmodified, byte-identical copy.
    /// No API for edited CIOPRG export is provided in PR #2.
    pub fn copy_original(&self) -> Vec<u8> { self.raw.clone() }

    pub fn inspection_report(&self, max_preview_lines: usize, max_diagnostics: usize) -> InspectionReport {
        let recognized_instructions = self.lines.iter().filter(|l|
            matches!(l.kind, LineKind::Instruction(_))).count();
        let unsupported_lines = self.lines.iter().filter(|l|
            matches!(l.kind, LineKind::UnsupportedInstruction { .. } | LineKind::Opaque)).count();

        let lines = self.lines.iter().take(max_preview_lines).map(|l| LinePreview {
            line: l.number,
            ending: l.ending,
            body_span: l.body_span,
            kind: l.kind.clone(),
            display_text: String::from_utf8_lossy(&self.raw[l.body_span.start..l.body_span.end])
                .to_string(),
        }).collect();

        InspectionReport {
            controller: self.controller,
            encoding: self.encoding,
            byte_length: self.raw.len(),
            source_sha256: self.source_sha256.clone(),
            line_count: self.lines.len(),
            recognized_instructions,
            unsupported_lines,
            m_headers: self.m_headers.clone(),
            diagnostics: self.diagnostics.iter().take(max_diagnostics).cloned().collect(),
            lines,
            preview_truncated: self.lines.len() > max_preview_lines,
            original_round_trip_only: true,
            edited_export_allowed: false,
            system_user_boundaries_verified: false,
        }
    }
}

/// Split at ASCII CRLF, LF or CR. No Unicode conversions occur.
fn scan_lines(raw: &[u8]) -> Vec<SourceLine> {
    let mut lines = Vec::new();
    let mut start = 0usize;
    while start < raw.len() {
        let mut end = start;
        while end < raw.len() && raw[end] != b'\n' && raw[end] != b'\r' {
            end += 1;
        }
        let (ending, full_end) = if end == raw.len() {
            (LineEnding::None, end)
        } else if raw[end] == b'\r' && raw.get(end + 1) == Some(&b'\n') {
            (LineEnding::CrLf, end + 2)
        } else if raw[end] == b'\r' {
            (LineEnding::Cr, end + 1)
        } else {
            (LineEnding::Lf, end + 1)
        };
        lines.push(SourceLine {
            number: lines.len() + 1,
            full_span: ByteSpan { start, end: full_end },
            body_span: ByteSpan { start, end },
            ending,
            kind: LineKind::Empty,
        });
        start = full_end;
    }
    lines
}

fn push_diag(
    diagnostics: &mut Vec<ParseDiagnostic>,
    code: &'static str,
    severity: Severity,
    line: &SourceLine,
    message: impl Into<String>,
) {
    diagnostics.push(ParseDiagnostic {
        code,
        severity,
        line: line.number,
        byte_offset: line.body_span.start,
        message: message.into(),
    });
}

fn is_space(b: u8) -> bool { matches!(b, b' ' | b'\t') }

fn ascii_trim(mut bytes: &[u8]) -> &[u8] {
    while bytes.first().copied().is_some_and(is_space) { bytes = &bytes[1..]; }
    while bytes.last().copied().is_some_and(is_space) { bytes = &bytes[..bytes.len() - 1]; }
    bytes
}

/// A text subset recognizer only. No semantic/branch evaluation.
fn parse_instruction(
    source: &[u8],
    line: &SourceLine,
    source_offset: usize,
    controller: ControllerFamily,
    diagnostics: &mut Vec<ParseDiagnostic>,
) -> LineKind {
    let mut start = 0usize;
    while start < source.len() && is_space(source[start]) { start += 1; }
    let mut mnemonic_end = start;
    while mnemonic_end < source.len() && !is_space(source[mnemonic_end]) {
        mnemonic_end += 1;
    }
    let mnemonic_bytes = &source[start..mnemonic_end];
    let mnemonic = match std::str::from_utf8(mnemonic_bytes) {
        Ok(t) if t.bytes().all(|b| b.is_ascii_uppercase() || b == b'-') => t,
        _ => {
            push_diag(diagnostics, "OpaqueLine", Severity::Warning, line,
                "Не удалось определить ASCII-мнемонику; байты сохранены");
            return LineKind::Opaque;
        }
    };
    let schema: &[OperandKind] = match mnemonic {
        "STR" | "STR-NOT" | "AND" | "AND-NOT" |
        "OR" | "OR-NOT" | "OUT" => &[OperandKind::RelayBit],
        "AND-STR" | "OR-STR" => &[],
        "TMR" | "CNT" => &[OperandKind::MRegister, OperandKind::Unrecognized],
        "GSTR" | "GOUT" => &[OperandKind::RelayByte],
        _ => {
            push_diag(diagnostics, "UnsupportedMnemonic", Severity::Warning, line,
                format!("Мнемоника {mnemonic} пока не имеет подтверждённой грамматики"));
            return LineKind::UnsupportedInstruction { mnemonic: mnemonic.to_string() };
        }
    };
    let mut args_start = mnemonic_end;
    while args_start < source.len() && is_space(source[args_start]) { args_start += 1; }
    let mut fields: Vec<(usize, usize)> = Vec::new();
    if args_start < source.len() {
        let mut cursor = args_start;
        for i in args_start..=source.len() {
            if i == source.len() || source[i] == b',' {
                let mut a = cursor;
                let mut b = i;
                while a < b && is_space(source[a]) { a += 1; }
                while b > a && is_space(source[b - 1]) { b -= 1; }
                fields.push((a, b));
                cursor = i + 1;
            }
        }
    }

    // No assumptions about comments trailing a source instruction. Anything
    // unrecognised is preserved and causes a diagnostic rather than truncation.
    let mut syntax_valid = fields.len() == schema.len();
    if fields.len() != schema.len() {
        push_diag(diagnostics, "OperandCount", Severity::Error, line,
            format!("{mnemonic}: ожидалось {} аргументов, получено {}", schema.len(), fields.len()));
    }
    let mut operands = Vec::new();
    for (index, (start, end)) in fields.into_iter().enumerate() {
        let spelling = String::from_utf8_lossy(&source[start..end]).into_owned();
        let absolute = ByteSpan {
            start: line.body_span.start + source_offset + start,
            end: line.body_span.start + source_offset + end,
        };
        let expectation = schema.get(index).copied().unwrap_or(OperandKind::Unrecognized);
        let kind = match expectation {
            OperandKind::RelayBit if parse_relay(controller, &spelling).is_ok()
                && spelling.starts_with('#') => OperandKind::RelayBit,
            OperandKind::RelayByte if parse_group(controller, &spelling, RelayGroupWidth::Byte8).is_ok()
                => OperandKind::RelayByte,
            OperandKind::MRegister if parse_register(&spelling).is_ok() => OperandKind::MRegister,
            OperandKind::Unrecognized if index == 1 && matches!(mnemonic, "CNT" | "TMR") => {
                if parse_register(&spelling).is_ok() { OperandKind::MRegister }
                else if !spelling.is_empty() && spelling.bytes().all(|b| b.is_ascii_digit()) {
                    OperandKind::DecimalLiteral
                } else { OperandKind::Unrecognized }
            }
            _ => OperandKind::Unrecognized,
        };
        let good = kind != OperandKind::Unrecognized
            || (schema.is_empty() && false);
        if !good {
            syntax_valid = false;
            push_diag(diagnostics, "InvalidOperand", Severity::Error, line,
                format!("{mnemonic}: неподдерживаемый или недопустимый операнд '{}'", spelling));
        }
        operands.push(AstOperand { spelling, kind, span: absolute });
    }
    LineKind::Instruction(AstInstruction {
        mnemonic: mnemonic.to_string(),
        operands,
        line: line.number,
        span: line.body_span,
        syntax_valid,
    })
}

fn parse_m_row(text: &[u8]) -> bool {
    // Preserve values exactly. Only verify eight rows of ten comma-separated
    // decimal cells; signed decimals are accepted for 16-bit register values.
    let Ok(text) = std::str::from_utf8(text) else { return false };
    let fields: Vec<_> = text.split(',').collect();
    fields.len() == 10 && fields.iter().all(|part| {
        let p = part.trim();
        let digits = p.strip_prefix('+').or_else(|| p.strip_prefix('-')).unwrap_or(p);
        !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit())
            && p.parse::<i32>().is_ok()
    })
}

pub fn parse_document(raw: Vec<u8>, controller: ControllerFamily) -> LosslessDocument {
    let source_sha256 = hex::encode(Sha256::digest(&raw));
    let encoding = if raw.starts_with(&[0xef, 0xbb, 0xbf]) {
        EncodingObservation::Utf8Bom
    } else if std::str::from_utf8(&raw).is_ok() {
        EncodingObservation::Utf8
    } else {
        EncodingObservation::OpaqueBytes
    };
    let mut lines = scan_lines(&raw);
    let mut diagnostics = Vec::new();
    let mut m_headers = Vec::new();
    let mut idx = 0usize;
    while idx < lines.len() {
        let mut body = &raw[lines[idx].body_span.start..lines[idx].body_span.end];
        let mut source_offset = 0usize;
        if idx == 0 && body.starts_with(&[0xef, 0xbb, 0xbf]) {
            body = &body[3..];
            source_offset = 3;
        }
        let trimmed = ascii_trim(body);
        if trimmed.is_empty() {
            lines[idx].kind = LineKind::Empty;
            idx += 1;
            continue;
        }
        if trimmed == b"//$M" {
            lines[idx].kind = LineKind::Directive;
            let marker = lines[idx].number;
            let mut data_lines = Vec::new();
            let mut complete = true;
            let mut consumed = 0usize;
            for row in 0..8 {
                let next = idx + 1 + consumed;
                if next >= lines.len() {
                    complete = false;
                    break;
                }
                let data = ascii_trim(&raw[lines[next].body_span.start..lines[next].body_span.end]);
                if !parse_m_row(data) {
                    complete = false;
                    push_diag(&mut diagnostics, "MHeaderInvalidRow", Severity::Error,
                        &lines[next], format!("Строка {} заголовка //$M: ожидается 10 целых чисел", row + 1));
                    // Do not consume the unexpected source line as an M row.
                    // It might be a real instruction following a truncated header.
                    break;
                }
                lines[next].kind = LineKind::MHeaderRow { row: row + 1, valid: true };
                data_lines.push(lines[next].number);
                consumed += 1;
            }
            if !complete {
                push_diag(&mut diagnostics, "MHeaderIncomplete", Severity::Error, &lines[idx],
                    "Секция //$M не содержит восемь корректных строк");
            }
            m_headers.push(MHeaderInfo { marker_line: marker, data_lines, complete });
            idx += 1 + consumed;
            continue;
        }
        if trimmed.starts_with(b"//") || trimmed.starts_with(b"/") {
            lines[idx].kind = LineKind::Directive;
        } else if trimmed.starts_with(b"*") || trimmed.starts_with(b";") {
            lines[idx].kind = LineKind::Comment;
        } else if trimmed.contains(&0) {
            lines[idx].kind = LineKind::Opaque;
            push_diag(&mut diagnostics, "EmbeddedNul", Severity::Warning, &lines[idx],
                "Строка содержит NUL; исходные байты сохранены");
        } else {
            lines[idx].kind = parse_instruction(body, &lines[idx], source_offset, controller, &mut diagnostics);
        }
        idx += 1;
    }
    LosslessDocument { raw, source_sha256, controller, encoding, lines, m_headers, diagnostics }
}

#[cfg(test)]
mod tests;
