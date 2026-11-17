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

#[test]
fn bom_token_span_points_to_actual_original_bytes() {
    let bytes = b"\xef\xbb\xbfSTR #70010\r\n";
    let doc = parse(bytes);
    let LineKind::Instruction(inst) = &doc.lines[0].kind else { panic!("not an instruction"); };
    assert_eq!(&bytes[inst.operands[0].span.start..inst.operands[0].span.end], b"#70010");
    assert_eq!(inst.operands[0].span.start, 7);
}

#[test]
fn malformed_m_header_does_not_consume_following_instruction() {
    let bytes = b"//$M\n0,0,0,0,0,0,0,0,0,0\nSTR #20010\nOUT #70010\n";
    let doc = parse(bytes);
    assert_round_trip(bytes);
    assert_eq!(doc.m_headers[0].data_lines, vec![2]);
    assert!(!doc.m_headers[0].complete);
    assert!(matches!(doc.lines[2].kind, LineKind::Instruction(_)));
    assert!(matches!(doc.lines[3].kind, LineKind::Instruction(_)));
}

#[test]
fn empty_operand_and_unknown_sections_remain_byte_identical() {
    let bytes = b"//UNKNOWN\r\nOR-STR,\r\nTMR M010,\r\n";
    let doc = parse(bytes);
    assert_round_trip(bytes);
    assert!(doc.diagnostics.iter().any(|d| d.code == "InvalidOperand" || d.code == "OperandCount"));
    assert!(matches!(doc.lines[0].kind, LineKind::Directive));
}

#[test]
fn original_hash_identifies_unmodified_byte_snapshot() {
    let doc = parse(b"abc");
    assert_eq!(doc.source_sha256,
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    assert_eq!(doc.inspection_report(10, 10).source_sha256, doc.source_sha256);
    let altered = parse(b"abc\n");
    assert_ne!(doc.source_sha256, altered.source_sha256);
}

#[test]
fn tmr_cnt_manual_register_and_decimal_boundaries() {
    let doc=parse(b"TMR M559,65535\nCNT M000,M999\nTMR M560,100\nCNT M010,65536\nTMR M999,M010\n");
    let expected=[true,true,false,false,false];
    for (line, validity) in doc.lines.iter().zip(expected) {
        let LineKind::Instruction(i)=&line.kind else {panic!("typed instruction")};
        assert_eq!(i.syntax_valid,validity,"{}: {:?}",i.mnemonic,i.operands);
    }
    assert_eq!(doc.copy_original(),
        b"TMR M559,65535\nCNT M000,M999\nTMR M560,100\nCNT M010,65536\nTMR M999,M010\n");
}

#[test]
fn read_only_pages_are_exact_and_keep_original_bytes() {
    let source = (1..=701).map(|n| format!("* synthetic line {n}\r\n"))
        .collect::<String>();
    let doc = parse(source.as_bytes());
    assert_eq!(doc.lines.len(), 701);
    let first = doc.line_preview_range(1, 250);
    let middle = doc.line_preview_range(501, 250);
    let last = doc.line_preview_range(701, 250);
    assert_eq!(first.len(), 250);
    assert_eq!(middle[0].line, 501);
    assert_eq!(middle[0].display_text, "* synthetic line 501");
    assert_eq!(last.len(), 1);
    assert_eq!(doc.line_preview_range(702, 250).len(), 0);
    assert_eq!(doc.copy_original(), source.as_bytes());
}


#[test]
fn post_end_alarm_message_rows_are_data_not_false_ladder_instructions() {
    let raw=b"/CIOPRG DX\r\nPART 1\r\nSTR #20010\r\nOUT #70010\r\nEND\r\n//ALARM1\r\nSTR #20011,OUT #70020,XYZ\r\n//ALARM2\r\nA,B,C\r\n//MESSAGE1\r\nMOV M010,M011\r\n//MESSAGE2\r\nOUT #70040\r\n";
    let doc=parse(raw);
    assert_round_trip(raw);
    let report=doc.inspection_report(100,100);
    assert_eq!(report.end_line,Some(5));
    assert_eq!(report.post_end_sections,4);
    assert_eq!(report.post_end_line_count,8);
    assert_eq!(report.post_end_unverified_lines,0);
    assert_eq!(report.recognized_instructions,4);
    assert_eq!(report.unsupported_lines,0);
    assert!(doc.diagnostics.is_empty());
    assert!(matches!(doc.lines[5].kind,LineKind::PostEndSection {..}));
    assert!(matches!(doc.lines[6].kind,LineKind::PostEndData));
    assert_eq!(report.lines[6].display_text,"STR #20011,OUT #70020,XYZ");
    assert_eq!(doc.original_bytes(),raw);
}

#[test]
fn post_end_unknown_markers_are_not_silently_interpreted() {
    let raw=b"STR #70010\nOUT #70020\nEND\n//UNSUPPORTED7\nSTR #70011\n//ALARM1\nalarm text\n//OTHER\nunexpected\n";
    let doc=parse(raw);
    let p=doc.inspection_report(100,100);
    assert_eq!(p.post_end_sections,1);
    assert_eq!(p.post_end_unverified_lines,3);
    assert_eq!(p.post_end_line_count,6);
    assert!(matches!(doc.lines[3].kind,LineKind::PostEndOpaque));
    assert!(matches!(doc.lines[4].kind,LineKind::PostEndOpaque));
    assert!(matches!(doc.lines[5].kind,LineKind::PostEndSection{..}));
    assert!(matches!(doc.lines[6].kind,LineKind::PostEndData));
    assert!(matches!(doc.lines[7].kind,LineKind::PostEndOpaque));
    assert!(matches!(doc.lines[8].kind,LineKind::PostEndData));
    assert_eq!(doc.diagnostics.iter().filter(|d|d.code=="PostEndUnknownStructure").count(),1);
    assert_eq!(doc.copy_original(),raw);
}

#[test]
fn invalid_end_does_not_silence_following_source() {
    let raw=b"END INVALID\nSTR #70010\nOUT #70020\n";
    let doc=parse(raw);
    let p=doc.inspection_report(100,100);
    assert_eq!(p.end_line,None);
    assert_eq!(p.post_end_line_count,0);
    assert!(matches!(doc.lines[1].kind,LineKind::Instruction(_)));
    assert!(doc.diagnostics.iter().any(|x|x.code=="OperandCount"));
}

#[test]
fn mixed_endings_and_opaque_bytes_survive_post_end() {
    let raw=b"END\r\n//MESSAGE1\rpayload \xff\r\n\n//ALARM2\rbinary \x00";
    let doc=parse(raw);
    assert_round_trip(raw);
    assert_eq!(doc.encoding,EncodingObservation::OpaqueBytes);
    assert_eq!(doc.inspection_report(50,50).post_end_line_count,5);
    assert_eq!(doc.original_bytes(),raw);
}

#[test]
fn typed_candidate_formats_hold_exact_tokens_and_byte_spans() {
    let source=b"//META\r\nPART 2\r\n MOV M001,W#70010\r\nADD 1,M010,M020\r\nSUB M020,25,M021\nSHL W#70010,16,W#70020\rSHR M010,0,M011\nEND";
    let doc=parse(source);
    assert_round_trip(source);
    let r=doc.inspection_report(100,100);
    assert_eq!(r.recognized_instructions,2); // PART, END
    assert_eq!(r.typed_candidate_count,5);
    assert_eq!(r.typed_valid_forms,5);
    assert_eq!(r.typed_invalid_forms,0);
    assert_eq!(r.unsupported_lines,0);
    for l in &doc.lines {
        if let LineKind::TypedCandidate {instruction,semantics_verified,controller_form_verified}= &l.kind {
            assert!(!semantics_verified);
            assert!(*controller_form_verified);
            assert!(instruction.syntax_valid);
            for op in &instruction.operands {
                let raw=&doc.original_bytes()[op.span.start..op.span.end];
                assert_eq!(raw,op.spelling.as_bytes());
            }
        }
    }
    let mov=match &doc.lines[2].kind {LineKind::TypedCandidate{instruction,..}=>instruction,_=>panic!("MOV not typed")};
    assert_eq!(mov.mnemonic,"MOV");
    assert_eq!(mov.operands[0].kind,OperandKind::MRegister);
    assert_eq!(mov.operands[1].kind,OperandKind::RelayWord);
    assert_eq!(doc.copy_original(),source);
}
#[test]
fn malformed_extended_operands_fail_shape_only_not_lossless_preservation() {
    let source=b"MOV 99,M640\nADD M010,2,M999\nSUB #70010,3,M010\nSHL M010,M005,M011\nSHL M010,17,M011\nMOV W#70011,#70020\nMOV M010,M011,EXTRA\nEND\n";
    let doc=parse(source);
    assert_round_trip(source);
    let p=doc.inspection_report(100,100);
    assert_eq!(p.typed_candidate_count,7);
    assert_eq!(p.typed_valid_forms,0);
    assert_eq!(p.typed_invalid_forms,7);
    assert!(doc.diagnostics.iter().any(|d|d.code=="TypedOperandCount"));
    assert!(doc.diagnostics.iter().any(|d|d.code=="TypedOperandForm"));
    assert_eq!(doc.copy_original(),source);
}
#[test]
fn typed_forms_are_not_confused_with_output_or_inform_movements() {
    let b=b"BMOV M010,5,M011\nMOVJ P000\nMOVL P001\nCMP M010,M011\nMOV M010,M011\nEND\n//MESSAGE1\nADD 1,2,M020\n";
    let doc=parse(b);
    assert_round_trip(b);
    assert!(matches!(doc.lines[0].kind,LineKind::UnsupportedInstruction{..}));
    assert!(matches!(doc.lines[1].kind,LineKind::UnsupportedInstruction{..}));
    assert!(matches!(doc.lines[2].kind,LineKind::UnsupportedInstruction{..}));
    assert!(matches!(doc.lines[3].kind,LineKind::UnsupportedInstruction{..}));
    assert!(matches!(doc.lines[4].kind,LineKind::TypedCandidate{..}));
    assert!(matches!(doc.lines[7].kind,LineKind::PostEndData));
    let report=doc.inspection_report(100,100);
    assert_eq!(report.typed_candidate_count,1);
    assert_eq!(report.post_end_sections,1);
}
#[test]
fn dx100_typed_form_is_never_labelled_confirmed() {
    let doc=parse_document(b"MOV M010,M011\nEND\n".to_vec(),ControllerFamily::DX100);
    assert!(matches!(doc.lines[0].kind,
        LineKind::TypedCandidate{controller_form_verified:false,semantics_verified:false,..}));
}


#[test]
fn synthetic_four_opcode_distribution_is_documented_shape_only() {
    let mut source=Vec::new();
    for _ in 0..101 {source.extend_from_slice(b"MOV M010,M011\r\n");}
    for _ in 0..102 {source.extend_from_slice(b"ADD M010,M011,M012\r\n");}
    for _ in 0..6 {source.extend_from_slice(b"SUB 3,M010,M011\r\n");}
    for _ in 0..6 {source.extend_from_slice(b"SHL M010,1,M011\r\n");}
    source.extend_from_slice(b"END\r\n//ALARM1\r\nMOV M010,M011\r\n");
    let doc=parse_document(source.clone(),ControllerFamily::YRC1000);
    let p=doc.inspection_report(5,5);
    assert_eq!(p.typed_candidate_count,215);
    assert_eq!(p.typed_valid_forms,215);
    assert_eq!(p.typed_invalid_forms,0);
    assert_eq!(p.typed_mnemonic_counts.get("MOV"),Some(&101));
    assert_eq!(p.typed_mnemonic_counts.get("ADD"),Some(&102));
    assert_eq!(p.typed_mnemonic_counts.get("SUB"),Some(&6));
    assert_eq!(p.typed_mnemonic_counts.get("SHL"),Some(&6));
    assert_eq!(p.post_end_sections,1);
    assert_eq!(doc.original_bytes(),source.as_slice());
    assert_eq!(doc.copy_original(),source);
}
