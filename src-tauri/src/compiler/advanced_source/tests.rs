use super::*;
use crate::compiler::lossless::parse_document;
use crate::compiler::memory::ControllerFamily;

fn parse(code: &[u8], controller: ControllerFamily) -> AdvancedSourceReport {
    inspect(&parse_document(code.to_vec(),controller))
}
fn codes(a: &AdvancedSourceReport) -> Vec<&'static str> {
    a.warnings.iter().map(|w| w.code).collect()
}
#[test]
fn manufacturer_examples_tmr_cnt_and_group_are_recognized_without_edit_permission() {
    let src=b"PART 2\r\nSTR #70010\r\nTMR M010,100\r\nOUT #70100\r\nSTR #70010\r\nSTR #70011\r\nCNT M020,3\r\nOUT #70110\r\nGSTR #20010\r\nGOUT #00010\r\nEND\r\n";
    for family in [ControllerFamily::DX100,ControllerFamily::DX200,ControllerFamily::YRC1000] {
        let doc=parse_document(src.to_vec(),family);
        let r=inspect(&doc);
        assert_eq!(r.blocks.len(),3);
        assert!(r.blocks.iter().all(|b|b.example_pattern_recognized));
        assert_eq!(r.blocks[0].count_or_enable.as_deref(),Some("#70010"));
        assert_eq!(r.blocks[0].output.as_deref(),Some("#70100"));
        assert_eq!(r.blocks[1].reset.as_deref(),Some("#70011"));
        assert_eq!(r.blocks[2].operands,vec!["#20010","#00010"]);
        assert_eq!(r.source_sha256,doc.source_sha256);
        assert!(!r.native_export_allowed);
        assert!(!r.hardware_test_authorized);
        assert_eq!(doc.copy_original(),src);
    }
}
#[test]
fn unpaired_group_is_flagged_without_guessing_missing_side() {
    let r=parse(b"GSTR #20010\nEND\n",ControllerFamily::DX100);
    assert!(codes(&r).contains(&"UnpairedGSTR"));
    assert!(!r.blocks[0].example_pattern_recognized);
    let r=parse(b"GOUT #00010\nEND\n",ControllerFamily::DX100);
    assert!(codes(&r).contains(&"UnpairedGOUT"));
    assert!(r.blocks.is_empty());
}
#[test]
fn repeated_gout_and_coil_overlap_warn() {
    let r=parse(b"GSTR #20010\nGOUT #70010\nGSTR #20020\nGOUT #70010\nSTR #70020\nOUT #70013\nEND\n",ControllerFamily::DX200);
    assert!(codes(&r).contains(&"RepeatedGOUT"));
    assert!(codes(&r).contains(&"GroupOutputOverlap"));
}
#[test]
fn complex_timer_expression_does_not_fabricate_a_supported_pattern() {
    let r=parse(b"STR #70010\nAND #70011\nTMR M010,100\nOUT #70100\nEND\n",ControllerFamily::YRC1000);
    assert_eq!(r.blocks.len(),1);
    assert!(!r.blocks[0].example_pattern_recognized);
    assert!(codes(&r).contains(&"UnverifiedAdvancedTopology"));
}
#[test]
fn invalid_register_is_detected_in_lossless_and_advanced_pattern_is_refused() {
    let r=parse(b"STR #70010\nTMR M560,100\nOUT #70100\nEND\n",ControllerFamily::DX100);
    assert_eq!(r.blocks.len(),1);
    assert!(!r.blocks[0].example_pattern_recognized);
}
#[test]
fn duplicate_current_registers_are_detected_in_a_native_snapshot() {
    let r=parse(b"STR #70010\nTMR M010,100\nOUT #70100\nSTR #70011\nSTR #70012\nCNT M010,3\nOUT #70110\nEND\n",ControllerFamily::DX200);
    assert!(codes(&r).contains(&"DuplicateCurrentRegister"));
}
