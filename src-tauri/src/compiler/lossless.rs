//! Lossless, read-only CIOPRG.LST parser.
//!
//! This module scans raw bytes into an ordered sequence of source lines and
//! overlays a *limited* typed syntax tree. No unknown text is discarded.
//! It intentionally does not infer SYSTEM/USER ownership or produce editable
//! controller programs. See docs/cioprg_lossless_parser.md.

use serde::Serialize;
use std::collections::BTreeMap;
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
    PartNumber,
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
    /// Documented operand *shape*, with unverified execution/flag semantics.
    /// This is NOT executable by the existing boolean analyzer.
    TypedCandidate {
        instruction: AstInstruction,
        semantics_verified: bool,
        controller_form_verified: bool,
    },
    UnsupportedInstruction { mnemonic: String },
    Opaque,
    /// Observed ALARM/MESSAGE marker in the data region after exact END.
    /// Label is informational; it does NOT establish a verified OEM grammar.
    PostEndSection { label: String },
    /// Raw data line inside an observed post-END section. Not an executable opcode.
    PostEndData,
    /// Post-END line outside a recognized data section or an unknown directive.
    /// Never guess it is executable, nor silently accept its layout.
    PostEndOpaque,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SyntaxClass {
    // Shape of source text only; NEVER controller execution verification.
    Recognized,
    TypedCandidate,
    Unsupported,
    Malformed,
    Opaque,
    PostEndData,
    Structural,
}
impl SyntaxClass {
    pub fn of(kind: &LineKind) -> Self {
        match kind {
            LineKind::Instruction(inst) if inst.syntax_valid => Self::Recognized,
            LineKind::Instruction(_) => Self::Malformed,
            LineKind::TypedCandidate { instruction, .. } if instruction.syntax_valid => Self::TypedCandidate,
            LineKind::TypedCandidate { .. } => Self::Malformed,
            LineKind::UnsupportedInstruction { .. } => Self::Unsupported,
            LineKind::Opaque | LineKind::PostEndOpaque => Self::Opaque,
            LineKind::PostEndSection { .. } | LineKind::PostEndData => Self::PostEndData,
            LineKind::MHeaderRow { valid: false, .. } => Self::Malformed,
            LineKind::Empty | LineKind::Comment | LineKind::Directive |
                LineKind::MHeaderRow { valid: true, .. } => Self::Structural,
        }
    }
}

/// Exactly one class per physical line; NOT OEM grammar coverage.
#[derive(Debug, Clone, Serialize)]
pub struct SyntaxCoverage {
    pub total_lines: usize,
    pub recognized: usize,
    pub typed_candidate: usize,
    pub unsupported: usize,
    pub malformed: usize,
    pub opaque: usize,
    pub post_end_data: usize,
    pub structural: usize,
    pub all_source_lines_counted: bool,
    pub semantics_verified: bool,
}
impl SyntaxCoverage {
    pub fn from_lines(lines: &[SourceLine]) -> Self {
        let mut result = Self {
            total_lines: lines.len(),
            recognized: 0, typed_candidate: 0, unsupported: 0,
            malformed: 0, opaque: 0, post_end_data: 0, structural: 0,
            all_source_lines_counted: true, semantics_verified: false,
        };
        for line in lines {
            match SyntaxClass::of(&line.kind) {
                SyntaxClass::Recognized => result.recognized += 1,
                SyntaxClass::TypedCandidate => result.typed_candidate += 1,
                SyntaxClass::Unsupported => result.unsupported += 1,
                SyntaxClass::Malformed => result.malformed += 1,
                SyntaxClass::Opaque => result.opaque += 1,
                SyntaxClass::PostEndData => result.post_end_data += 1,
                SyntaxClass::Structural => result.structural += 1,
            }
        }
        debug_assert_eq!(
            result.recognized + result.typed_candidate + result.unsupported +
            result.malformed + result.opaque + result.post_end_data + result.structural,
            result.total_lines
        );
        result
    }
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
    pub syntax_class: SyntaxClass,
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
    pub syntax_coverage: SyntaxCoverage,
    pub recognized_instructions: usize,
    pub typed_candidate_count: usize,
    pub typed_valid_forms: usize,
    pub typed_invalid_forms: usize,
    pub typed_mnemonic_counts: BTreeMap<String,usize>,
    pub unsupported_lines: usize,
    pub m_headers: Vec<MHeaderInfo>,
    pub diagnostics: Vec<ParseDiagnostic>,
    pub lines: Vec<LinePreview>,
    pub preview_truncated: bool,
    pub end_line: Option<usize>,
    pub post_end_line_count: usize,
    pub post_end_sections: usize,
    pub post_end_unverified_lines: usize,
    pub original_round_trip_only: bool,
    pub edited_export_allowed: bool,
    pub system_user_boundaries_verified: bool,
}

impl LosslessDocument {
    pub fn original_bytes(&self) -> &[u8] { &self.raw }

    /// The *only* supported export: an unmodified, byte-identical copy.
    /// No API for edited CIOPRG export is provided in PR #2.
    pub fn copy_original(&self) -> Vec<u8> { self.raw.clone() }

    /// Display-only page of lines, 1-based. Never use lossy display_text for export.
    pub fn line_preview_range(&self, start_line: usize, limit: usize) -> Vec<LinePreview> {
        self.lines.iter().skip(start_line.saturating_sub(1)).take(limit).map(|l| LinePreview {
            line: l.number,
            syntax_class: SyntaxClass::of(&l.kind),
            ending: l.ending,
            body_span: l.body_span,
            kind: l.kind.clone(),
            display_text: String::from_utf8_lossy(&self.raw[l.body_span.start..l.body_span.end])
                .to_string(),
        }).collect()
    }

    pub fn inspection_report(&self, max_preview_lines: usize, max_diagnostics: usize) -> InspectionReport {
        let recognized_instructions = self.lines.iter().filter(|l|
            matches!(l.kind, LineKind::Instruction(_))).count();
        let typed_candidate_count = self.lines.iter().filter(|l|
            matches!(l.kind, LineKind::TypedCandidate{..})).count();
        let typed_valid_forms = self.lines.iter().filter(|l|
            matches!(&l.kind, LineKind::TypedCandidate { instruction, .. }
                if instruction.syntax_valid)).count();
        let typed_invalid_forms = typed_candidate_count - typed_valid_forms;
        let mut typed_mnemonic_counts: BTreeMap<String,usize> = BTreeMap::new();
        for line in &self.lines {
            if let LineKind::TypedCandidate { instruction, .. } = &line.kind {
                *typed_mnemonic_counts.entry(instruction.mnemonic.clone()).or_default() += 1;
            }
        }
        let unsupported_lines = self.lines.iter().filter(|l|
            matches!(l.kind, LineKind::UnsupportedInstruction { .. } | LineKind::Opaque |
                LineKind::PostEndOpaque)).count();
        let end_line = self.lines.iter().find_map(|l| match &l.kind {
            LineKind::Instruction(inst) if inst.mnemonic == "END" && inst.syntax_valid =>
                Some(l.number),
            _ => None,
        });
        let post_end_line_count = self.lines.iter().filter(|l|
            matches!(l.kind, LineKind::PostEndSection {..} | LineKind::PostEndData |
                LineKind::PostEndOpaque)).count();
        let post_end_sections = self.lines.iter().filter(|l|
            matches!(l.kind, LineKind::PostEndSection {..})).count();
        let post_end_unverified_lines = self.lines.iter().filter(|l|
            matches!(l.kind, LineKind::PostEndOpaque)).count();

        let lines = self.line_preview_range(1, max_preview_lines);

        InspectionReport {
            controller: self.controller,
            encoding: self.encoding,
            byte_length: self.raw.len(),
            source_sha256: self.source_sha256.clone(),
            line_count: self.lines.len(),
            syntax_coverage: SyntaxCoverage::from_lines(&self.lines),
            recognized_instructions,
            typed_candidate_count,
            typed_valid_forms,
            typed_invalid_forms,
            typed_mnemonic_counts,
            unsupported_lines,
            m_headers: self.m_headers.clone(),
            diagnostics: self.diagnostics.iter().take(max_diagnostics).cloned().collect(),
            lines,
            preview_truncated: self.lines.len() > max_preview_lines,
            end_line,
            post_end_line_count,
            post_end_sections,
            post_end_unverified_lines,
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

/// Operand roles documented for the YRC1000 Concurrent I/O instruction set,
/// with the same lexical forms present in DX200 RE-CKI-A465. This is NOT a
/// controller-ready validator, program runner, write authorization or a claim
/// that a DX100 firmware supports identical rules.
#[derive(Clone, Copy)]
enum TypedOperandRole {
    WordSource,     // immediate 0..65535, M000..999, byte/word relay group
    WordDest,       // M000..599 or byte/word relay group
    ArithmeticSrc,  // immediate 0..65535 or M000..999
    ArithmeticDest, // M000..599
    MulPairDest,    // M000..598; Dn + 1 must still fit M000..599
    GroupSource,    // M000..999 or relay byte/word; BIN/BCD do NOT allow literal
    BlockSource,    // M000..999 or relay byte
    BlockDest,      // M000..599 or relay byte
    BlockCount,     // decimal 1..999 (bytes, not register/word count)
    PulseRelay,     // #XXXXX relay; NOT a boolean coil AST
    ShiftCount,     // decimal 0..16 (not an M register)
}
fn typed_roles(mnemonic:&str)->Option<&'static [TypedOperandRole]> {
    use TypedOperandRole::*;
    // Forms transcribed from YRC1000 Concurrent I/O R12, §§12.2.15-34.
    // An instruction is only a *typed source candidate*; this function
    // supplies neither controller executable semantics nor write permission.
    match mnemonic {
        "MOV" => Some(&[WordSource,WordDest]),
        "ADD" | "SUB" | "DIV" | "MOD" =>
            Some(&[ArithmeticSrc,ArithmeticSrc,ArithmeticDest]),
        "MUL" => Some(&[ArithmeticSrc,ArithmeticSrc,MulPairDest]),
        "SHL" | "SHR" | "ROL" | "ROR" =>
            Some(&[WordSource,ShiftCount,WordDest]),
        "BIN" | "BCD" => Some(&[GroupSource,WordDest]),
        "WAND" | "WOR" | "WXOR" => Some(&[WordSource,WordSource,WordDest]),
        "WNOT" => Some(&[WordSource,WordDest]),
        "BMOV" => Some(&[BlockSource,BlockCount,BlockDest]),
        "CMP" => Some(&[WordSource,WordSource]),
        "PLS" | "PLF" => Some(&[PulseRelay]),
        _ => None,
    }
}
fn typed_token_kind(token:&str, controller:ControllerFamily)->OperandKind {
    if parse_register(token).is_ok() { return OperandKind::MRegister; }
    if parse_group(controller,token,RelayGroupWidth::Word16).is_ok() {
        return OperandKind::RelayWord;
    }
    if parse_group(controller,token,RelayGroupWidth::Byte8).is_ok() {
        return OperandKind::RelayByte;
    }
    if !token.is_empty() && token.bytes().all(|b| b.is_ascii_digit()) &&
        token.parse::<u16>().is_ok() {
        return OperandKind::DecimalLiteral;
    }
    OperandKind::Unrecognized
}
fn typed_slot_valid(
    role:TypedOperandRole,
    token:&str,
    kind:OperandKind,
)->bool {
    use TypedOperandRole::*;
    match role {
        WordSource => matches!(kind,OperandKind::DecimalLiteral|OperandKind::MRegister|
            OperandKind::RelayByte|OperandKind::RelayWord),
        WordDest => matches!(kind,OperandKind::RelayByte|OperandKind::RelayWord) ||
            (kind==OperandKind::MRegister && parse_register(token).is_ok_and(|m|m.number<=599)),
        ArithmeticSrc => matches!(kind,OperandKind::DecimalLiteral|OperandKind::MRegister),
        ArithmeticDest => kind==OperandKind::MRegister &&
            parse_register(token).is_ok_and(|m|m.number<=599),
        MulPairDest => kind==OperandKind::MRegister &&
            parse_register(token).is_ok_and(|m|m.number<=598),
        GroupSource => matches!(kind,OperandKind::MRegister|
            OperandKind::RelayByte|OperandKind::RelayWord),
        BlockSource => matches!(kind,OperandKind::MRegister|OperandKind::RelayByte),
        BlockDest => kind==OperandKind::RelayByte ||
            (kind==OperandKind::MRegister && parse_register(token).is_ok_and(|m|m.number<=599)),
        BlockCount => kind==OperandKind::DecimalLiteral &&
            token.parse::<u16>().is_ok_and(|n|(1..=999).contains(&n)),
        PulseRelay => kind==OperandKind::RelayBit,
        ShiftCount => kind==OperandKind::DecimalLiteral &&
            token.parse::<u16>().is_ok_and(|n|n<=16),
    }
}
/// Returns a display-only candidate for manual-described instruction *forms*.
/// No output is added to the executable boolean AST, even with valid operands.
/// All new PR45 families are evidence-marked only for YRC1000 Concurrent I/O
/// R12; DX100/DX200 export syntax and firmware remain unverified.
fn parse_typed_candidate(
    source:&[u8], line:&SourceLine, source_offset:usize,
    mnemonic:&str, mnemonic_end:usize, controller:ControllerFamily,
    diagnostics:&mut Vec<ParseDiagnostic>,
)->Option<LineKind> {
    let roles=typed_roles(mnemonic)?;
    let mut start=mnemonic_end;
    while start<source.len() && is_space(source[start]) {start+=1;}
    let mut columns:Vec<(usize,usize)>=Vec::new();
    let mut actual_count=0usize;
    if start<source.len() {
        let mut cell=start;
        for offset in start..=source.len() {
            if offset==source.len() || source[offset]==b',' {
                actual_count+=1;
                if columns.len()<8 {
                    let mut a=cell; let mut b=offset;
                    while a<b && is_space(source[a]) {a+=1;}
                    while b>a && is_space(source[b-1]) {b-=1;}
                    columns.push((a,b));
                }
                cell=offset+1;
            }
        }
    }
    let mut valid=actual_count==roles.len();
    if !valid {
        push_diag(diagnostics,"TypedOperandCount",Severity::Error,line,
            format!("{mnemonic}: ожидается {} операнда(-ов), получено {actual_count}; модель только для просмотра",roles.len()));
    }
    let mut operands=Vec::with_capacity(columns.len());
    for (index,(a,b)) in columns.into_iter().enumerate() {
        // Large malformed tokens are displayed only in bounded preview form,
        // while full source and accurate byte spans stay unchanged.
        let display_end=b.min(a.saturating_add(128));
        let spelling=String::from_utf8_lossy(&source[a..display_end]).into_owned();
        let kind=if b-a>128 {OperandKind::Unrecognized} else if
            matches!(roles.get(index),Some(TypedOperandRole::PulseRelay)) {
            if spelling.starts_with('#') && parse_relay(controller,&spelling).is_ok() {
                OperandKind::RelayBit
            } else {
                OperandKind::Unrecognized
            }
        } else {
            typed_token_kind(&spelling,controller)
        };
        let accepted=roles.get(index).is_some_and(|role|
            typed_slot_valid(*role,&spelling,kind));
        if !accepted {
            valid=false;
            push_diag(diagnostics,"TypedOperandForm",Severity::Warning,line,
                format!("{mnemonic}: позиция {} — неподтверждённый тип/диапазон операнда",index+1));
        }
        operands.push(AstOperand {
            spelling, kind,
            span:ByteSpan{start:line.body_span.start+source_offset+a,
                end:line.body_span.start+source_offset+b},
        });
    }
    let instruction=AstInstruction {
        mnemonic:mnemonic.to_string(),operands,line:line.number,
        span:line.body_span,syntax_valid:valid,
    };
    // These expanded forms have independently inspected *YRC1000* manual
    // evidence. No DX200/DX100 family equivalence is claimed for them here.
    // The old five forms retain their prior DX200 manual-level evidence.
    let prior_five=matches!(mnemonic,"MOV"|"ADD"|"SUB"|"SHL"|"SHR");
    Some(LineKind::TypedCandidate {
        instruction,
        semantics_verified:false,
        controller_form_verified:matches!(controller,ControllerFamily::YRC1000) ||
            (prior_five && matches!(controller,ControllerFamily::DX200)),
    })
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
    if let Some(candidate)=parse_typed_candidate(
        source,line,source_offset,mnemonic,mnemonic_end,controller,diagnostics
    ) { return candidate; }
    let schema: &[OperandKind] = match mnemonic {
        "STR" | "STR-NOT" | "AND" | "AND-NOT" |
        "OR" | "OR-NOT" | "OUT" => &[OperandKind::RelayBit],
        "AND-STR" | "OR-STR" | "END" => &[],
        "PART" => &[OperandKind::PartNumber],
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
            OperandKind::MRegister if parse_register(&spelling).is_ok_and(|a| {
                !matches!(mnemonic, "TMR" | "CNT") || a.number <= 559
            }) => OperandKind::MRegister,
            OperandKind::PartNumber if matches!(spelling.as_str(), "1" | "2") => OperandKind::PartNumber,
            OperandKind::Unrecognized if index == 1 && matches!(mnemonic, "CNT" | "TMR") => {
                if parse_register(&spelling).is_ok() { OperandKind::MRegister }
                else if !spelling.is_empty() && spelling.bytes().all(|b| b.is_ascii_digit())
                    && spelling.parse::<u16>().is_ok() {
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

/// The observed exported ALARM/MESSAGE markers consist of ASCII "//ALARMN"
/// or "//MESSAGEN". Do not infer ownership, encoding, or OEM edit rights.
fn observed_post_end_marker(line: &[u8]) -> Option<String> {
    for prefix in [b"//ALARM".as_slice(), b"//MESSAGE".as_slice()] {
        if let Some(suffix) = line.strip_prefix(prefix) {
            if !suffix.is_empty() && suffix.len() <= 8 &&
                suffix.iter().all(u8::is_ascii_digit) {
                return Some(String::from_utf8_lossy(line).into_owned());
            }
        }
    }
    None
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
    let mut after_end = false;
    let mut in_observed_tail_section = false;
    let mut unknown_tail_reported = false;
    while idx < lines.len() {
        let mut body = &raw[lines[idx].body_span.start..lines[idx].body_span.end];
        let mut source_offset = 0usize;
        if idx == 0 && body.starts_with(&[0xef, 0xbb, 0xbf]) {
            body = &body[3..];
            source_offset = 3;
        }
        let trimmed = ascii_trim(body);
        if after_end {
            // Once a syntactically valid END has been reached, no subsequent
            // source line is interpreted as executable Ladder. Preserve
            // everything verbatim, including unknown proprietary layouts.
            if let Some(label) = observed_post_end_marker(trimmed) {
                lines[idx].kind = LineKind::PostEndSection { label };
                in_observed_tail_section = true;
            } else if trimmed.starts_with(b"//") || trimmed.contains(&0) ||
                (!in_observed_tail_section && !trimmed.is_empty()) {
                lines[idx].kind = LineKind::PostEndOpaque;
                // Avoid hundreds of repeated warnings for text data sections.
                if !unknown_tail_reported {
                    push_diag(&mut diagnostics, "PostEndUnknownStructure", Severity::Warning,
                        &lines[idx], "Неподтверждённая структура после END: байты сохранены, не интерпретированы как инструкции");
                    unknown_tail_reported = true;
                }
            } else {
                lines[idx].kind = LineKind::PostEndData;
            }
            idx += 1;
            continue;
        }
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
            if let LineKind::Instruction(inst) = &lines[idx].kind {
                if inst.mnemonic == "END" && inst.syntax_valid {
                    after_end = true;
                }
            }
        }
        idx += 1;
    }
    LosslessDocument { raw, source_sha256, controller, encoding, lines, m_headers, diagnostics }
}

#[cfg(test)]
mod tests;
