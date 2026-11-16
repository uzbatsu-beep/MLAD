use super::*;
use crate::compiler::memory::ControllerFamily;

fn parse(bytes: &[u8]) -> LosslessDocument {
    parse_document(bytes.to_vec(), ControllerFamily::YRC1000)
}

fn assert_round_trip(input: &[u8]) {
    let doc = parse(input);
    assert_eq!(doc.copy_original(), input);
    assert_eq!(doc.original_bytes(), input);
    let mut pos = 0;
    for l in &doc.lines {
        assert_eq!(l.full_span.start, pos);
        assert_eq!(l.full_span.end - l.full_span.start,
            l.body_span.end - l.body_span.start + match l.ending {
                LineEnding::None => 0, LineEnding::CrLf => 2, _ => 1,
            });
        pos = l.full_span.end;
    }
    assert_eq!(pos, input.len(), "all input bytes must appear in exactly one line");
}

#[test]
fn byte_identical_mixed_endings_and_leading_zeroes() {
    let fixture = b"//MODE,UNKNOWN\r\nSTR #00010\nAND-NOT #70017\rOUT #70020";
    assert_round_trip(fixture);
    let doc = parse(fixture);
    assert_eq!(doc.lines.len(), 4);
    assert_eq!(doc.lines[0].ending, LineEnding::CrLf);
    assert_eq!(doc.lines[1].ending, LineEnding::Lf);
    assert_eq!(doc.lines[2].ending, LineEnding::Cr);
    assert_eq!(doc.lines[3].ending, LineEnding::None);
    assert!(matches!(doc.lines[1].kind, LineKind::Instruction(_)));
    if let LineKind::Instruction(ref inst) = doc.lines[1].kind {
        assert_eq!(inst.mnemonic, "STR");
        assert_eq!(inst.operands[0].spelling, "#00010");
        assert_eq!(inst.operands[0].kind, OperandKind::RelayBit);
    } else { panic!("expected instruction"); }
    assert!(doc.diagnostics.is_empty());
}

#[test]
fn bom_and_non_utf8_are_preserved_without_panic() {
    let a = b"\xef\xbb\xbfSTR #20010\r\nOUT #70010\r\n";
    let doc = parse(a);
    assert_round_trip(a);
    assert_eq!(doc.encoding, EncodingObservation::Utf8Bom);
    assert_eq!(doc.lines.len(), 2);
    assert!(matches!(doc.lines[0].kind, LineKind::Instruction(_)));

    let opaque = b"//META\r\n*binary \xff\xfe\r\nSTR #70010\r\n";
    let b = parse(opaque);
    assert_round_trip(opaque);
    assert_eq!(b.encoding, EncodingObservation::OpaqueBytes);
    assert!(matches!(b.lines[1].kind, LineKind::Comment));
    assert!(b.inspection_report(30,30).lines[1].display_text.contains('\u{fffd}'));
}

#[test]
fn unknown_mnemonics_and_genuine_directives_are_not_lost() {
    let b = b"//$UNKNOWN\nGSTR #70010\nBMOV #20010,#30010\nFOO BAR\n* note \n";
    let doc = parse(b);
    assert_round_trip(b);
    assert!(matches!(doc.lines[0].kind, LineKind::Directive));
    assert!(matches!(doc.lines[1].kind, LineKind::Instruction(_)));
    assert!(matches!(doc.lines[2].kind, LineKind::UnsupportedInstruction{..}));
    assert!(matches!(doc.lines[3].kind, LineKind::UnsupportedInstruction{..}));
    assert!(matches!(doc.lines[4].kind, LineKind::Comment));
    assert_eq!(doc.diagnostics.iter().filter(|d| d.code == "UnsupportedMnemonic").count(), 2);
    assert!(!doc.inspection_report(100,100).edited_export_allowed);
}

#[test]
fn valid_m_header_preserves_all_data_rows() {
    let data = b"//$M\r\n40,10,2,3,5,1,1,3,0,0\r\n40,10,2,3,5,1,1,3,0,0\r\n40,10,2,3,5,1,1,3,0,0\r\n40,10,2,3,5,1,1,3,0,0\r\n0,0,0,0,0,0,0,0,0,0\r\n0,0,0,0,0,0,0,0,0,0\r\n0,0,0,0,0,0,0,0,0,0\r\n-1,+2,0,0,0,0,0,0,0,0\r\nSTR #70010\r\n";
    let doc = parse(data);
    assert_round_trip(data);
    assert_eq!(doc.m_headers.len(), 1);
    assert_eq!(doc.m_headers[0].data_lines, vec![2,3,4,5,6,7,8,9]);
    assert!(doc.m_headers[0].complete);
    assert_eq!(doc.lines[8].kind_is_m_row(), true);
    assert!(matches!(doc.lines[9].kind, LineKind::Instruction(_)));
    assert!(doc.diagnostics.is_empty());
}

trait IsMRow {
    fn kind_is_m_row(&self) -> bool;
}
impl IsMRow for SourceLine {
    fn kind_is_m_row(&self) -> bool {
        matches!(self.kind, LineKind::MHeaderRow{..})
    }
}

#[test]
fn short_or_malformed_m_header_is_diagnosed_and_preserved() {
    let incomplete = b"//$M\r\n1,2,3\r\n";
    let doc = parse(incomplete);
    assert_round_trip(incomplete);
    assert_eq!(doc.m_headers.len(), 1);
    assert!(!doc.m_headers[0].complete);
    assert!(doc.diagnostics.iter().any(|d| d.code == "MHeaderInvalidRow"));
    assert!(doc.diagnostics.iter().any(|d| d.code == "MHeaderIncomplete"));
}

#[test]
fn typed_operand_overlay_enforces_address_syntax() {
    let doc = parse(b"STR #70018\nCNT M256,M466\nTMR M010,25\nOR-STR\nOUT #30010\n");
    assert_round_trip(doc.original_bytes());
    assert!(doc.diagnostics.iter().any(|d| d.code == "InvalidOperand"));
    let inst = |i: usize| match &doc.lines[i].kind {
        LineKind::Instruction(i) => i,
        _ => panic!("expected instruction"),
    };
    assert!(!inst(0).syntax_valid);
    assert!(inst(1).syntax_valid);
    assert_eq!(inst(1).operands[0].kind, OperandKind::MRegister);
    assert_eq!(inst(1).operands[1].kind, OperandKind::MRegister);
    assert_eq!(inst(2).operands[1].kind, OperandKind::DecimalLiteral);
    assert!(inst(3).operands.is_empty());
    assert!(inst(4).syntax_valid);
}

#[test]
fn controllers_do_not_share_unverified_address_classes() {
    let sample = b"STR #87010\nOUT #70010\n";
    let dx200 = parse_document(sample.to_vec(), ControllerFamily::DX200);
    let yrc = parse_document(sample.to_vec(), ControllerFamily::YRC1000);
    assert!(dx200.diagnostics.iter().any(|d| d.code == "InvalidOperand"));
    assert!(yrc.diagnostics.is_empty());
    assert_eq!(dx200.copy_original(), yrc.copy_original());
}

#[test]
fn no_implicit_write_or_user_boundary_claim_in_report() {
    let doc = parse(b"//SYSTEM\nOUT #70010\n//USER\n");
    let r = doc.inspection_report(2, 3);
    assert_eq!(r.line_count, 3);
    assert_eq!(r.lines.len(), 2);
    assert!(r.preview_truncated);
    assert!(!r.edited_export_allowed);
    assert!(r.original_round_trip_only);
    assert!(!r.system_user_boundaries_verified);
}

#[test]
fn empty_and_bare_terminator_files_are_safe() {
    for b in [b"".as_slice(), b"\n".as_slice(), b"\r\n".as_slice(), b"\r".as_slice()] {
        assert_round_trip(b);
    }
    assert_eq!(parse(b"").lines.len(), 0);
    assert_eq!(parse(b"\n").lines.len(), 1);
}

#[test]
fn invalid_operand_count_and_trailing_comment_are_not_accepted() {
    let doc = parse(b"OR-STR X\nSTR #70010 extra\nOUT #70010 ;not-verified\n");
    assert!(doc.diagnostics.iter().any(|d| d.code == "OperandCount"));
    assert!(doc.diagnostics.iter().any(|d| d.code == "InvalidOperand"));
    assert!(doc.lines.iter().filter_map(|l| match &l.kind {
        LineKind::Instruction(i) => Some(i.syntax_valid), _ => None
    }).all(|valid| !valid));
}

#[test]
fn byte_spans_reference_exact_source_tokens() {
    let data = b"   STR  #20010  \r\n";
    let doc = parse(data);
    let LineKind::Instruction(inst) = &doc.lines[0].kind else { panic!("instruction missing"); };
    assert_eq!(&data[inst.operands[0].span.start..inst.operands[0].span.end], b"#20010");
}

#[test]
fn deterministic_binary_fuzz_preserves_every_byte() {
    let mut x: u32 = 0x8123abcd;
    for length in 0..256 {
        let mut input = Vec::with_capacity(length);
        for _ in 0..length {
            x = x.wrapping_mul(1664525).wrapping_add(1013904223);
            input.push((x >> 24) as u8);
        }
        assert_round_trip(&input);
    }
}

#[test]
fn unsupported_lines_never_fake_editability() {
    let sample = b"//SYSX\n*Name\nGOUT #70020\nUNKNOWN_COMMAND\r\n";
    let report = parse(sample).inspection_report(100, 100);
    assert!(!report.edited_export_allowed);
    assert_eq!(report.byte_length, sample.len());
    assert_eq!(report.unsupported_lines, 1);
}
