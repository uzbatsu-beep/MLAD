use super::*;
use crate::compiler::lossless::parse_document;
use crate::compiler::memory::ControllerFamily;

fn report(code: &str) -> SemanticReport {
    analyze(&parse_document(code.as_bytes().to_vec(), ControllerFamily::YRC1000))
}
fn signals(pairs: &[(u32, bool)]) -> HashMap<u32, bool> {
    pairs.iter().copied().collect()
}
fn codes(r: &SemanticReport) -> Vec<&'static str> {
    r.diagnostics.iter().map(|d| d.code).collect()
}

#[test]
fn simple_series_and_inversion_evaluates_correctly() {
    let r = report("PART 2\nSTR #20010\nAND-NOT #70011\nOUT #70020\nEND\n");
    assert_eq!(r.rungs.len(), 1);
    assert_eq!(r.rungs[0].inputs, vec![20010, 70011]);
    assert_eq!(r.rungs[0].section, SectionClaim::User);
    assert_eq!(r.evaluate_rung(0, &signals(&[(20010, true),(70011,false)])), Some(true));
    assert_eq!(r.evaluate_rung(0, &signals(&[(20010, true),(70011,true)])), Some(false));
    assert_eq!(r.evaluate_rung(0, &signals(&[(20010, false),(70011,false)])), Some(false));
    assert_eq!(r.evaluate_rung(0, &signals(&[(20010, true)])), None);
    assert!(r.subset_analysis_complete, "{:?}", codes(&r));
    assert!(!r.edited_export_allowed);
    assert!(!r.system_user_boundaries_verified);
}

#[test]
fn and_str_has_precedence_as_combination_of_two_groups() {
    let r = report("PART 2\nSTR #70010\nOR #70011\nSTR #70020\nOR #70021\nAND-STR\nOUT #70100\nEND\n");
    assert_eq!(r.rungs.len(), 1);
    assert!(r.subset_analysis_complete);
    assert_eq!(r.rungs[0].inputs, vec![70010,70011,70020,70021]);
    assert_eq!(r.evaluate_rung(0,&signals(&[(70010,true),(70011,false),(70020,false),(70021,true)])),Some(true));
    assert_eq!(r.evaluate_rung(0,&signals(&[(70010,true),(70011,false),(70020,false),(70021,false)])),Some(false));
    assert_eq!(r.evaluate_rung(0,&signals(&[(70010,false),(70011,false),(70020,false),(70021,true)])),Some(false));
}

#[test]
fn or_str_merges_two_series_groups() {
    let r = report("STR #70010\nAND #70011\nSTR #70020\nAND #70021\nOR-STR\nOUT #70100\nEND\n");
    assert_eq!(r.rungs.len(), 1);
    assert!(r.subset_analysis_complete);
    assert_eq!(r.evaluate_rung(0,&signals(&[(70010,false),(70011,true),(70020,true),(70021,true)])),Some(true));
    assert_eq!(r.evaluate_rung(0,&signals(&[(70010,true),(70011,false),(70020,true),(70021,false)])),Some(false));
}

#[test]
fn nested_operators_use_stack_lifo() {
    // (A OR (B AND C)) AND D
    let r = report("STR #70010\nSTR #70011\nAND #70012\nOR-STR\nSTR #70013\nAND-STR\nOUT #70100\nEND\n");
    assert!(r.subset_analysis_complete, "{:?}", codes(&r));
    assert_eq!(r.evaluate_rung(0,&signals(&[(70010,false),(70011,true),(70012,true),(70013,true)])),Some(true));
    assert_eq!(r.evaluate_rung(0,&signals(&[(70010,true),(70011,false),(70012,false),(70013,false)])),Some(false));
}

#[test]
fn missing_start_is_diagnostic_not_fabricated_rung() {
    let r = report("AND #20010\nOUT #70010\nEND\n");
    assert!(codes(&r).contains(&"MissingStart"));
    assert!(codes(&r).contains(&"UnbalancedOutput"));
    assert!(r.rungs.is_empty());
    assert!(!r.subset_analysis_complete);
}

#[test]
fn unbalanced_branches_do_not_emit_rung() {
    let r = report("STR #70010\nSTR #70020\nOUT #70100\nEND\n");
    assert!(codes(&r).contains(&"UnbalancedOutput"));
    assert!(r.rungs.is_empty());
    assert!(!r.subset_analysis_complete);
    let r = report("STR #70010\nAND-STR\nOUT #70100\nEND\n");
    assert!(codes(&r).contains(&"UnbalancedBranch"));
    assert!(r.rungs.is_empty());
}

#[test]
fn double_coil_detected_across_part_claims() {
    let r = report("PART 1\nSTR #20010\nOUT #70010\nPART 2\nSTR #20011\nOUT #70010\nEND\n");
    assert_eq!(r.rungs.len(), 2);
    assert_eq!(r.rungs[0].section, SectionClaim::System);
    assert_eq!(r.rungs[1].section, SectionClaim::User);
    assert!(codes(&r).contains(&"DoubleCoil"));
    assert!(!r.subset_analysis_complete);
}

#[test]
fn distinct_output_coils_are_valid_candidates() {
    let r = report("PART 2\nSTR #20010\nOUT #30010\nSTR #20011\nOUT #70011\nEND\n");
    assert_eq!(r.rungs.len(), 2);
    assert!(!codes(&r).contains(&"DoubleCoil"));
    assert!(!codes(&r).contains(&"ReadOnlyCoil"));
    assert!(r.subset_analysis_complete);
}

#[test]
fn read_only_output_is_rejected_and_specific_input_requires_review() {
    let r = report("STR #70010\nOUT #20010\nSTR #70011\nOUT #40044\nEND\n");
    assert!(codes(&r).contains(&"ReadOnlyCoil"));
    assert!(codes(&r).contains(&"WriteNeedsReview"));
}

#[test]
fn self_feedback_and_mutual_cycle_are_warnings() {
    let a = report("STR #70010\nOUT #70010\nEND\n");
    assert!(codes(&a).contains(&"SelfFeedback"));
    let b = report("STR #70011\nOUT #70010\nSTR #70010\nOUT #70011\nEND\n");
    assert!(codes(&b).contains(&"FeedbackCycle"));
    assert_eq!(b.rungs.len(),2);
}

#[test]
fn three_value_logic_handles_determining_short_circuit() {
    let a = report("STR #70010\nAND #70011\nOUT #70100\nEND\n");
    assert_eq!(a.evaluate_rung(0,&signals(&[(70010,false)])),Some(false));
    let b = report("STR #70010\nOR #70011\nOUT #70100\nEND\n");
    assert_eq!(b.evaluate_rung(0,&signals(&[(70010,true)])),Some(true));
    assert_eq!(b.evaluate_rung(0,&signals(&[])),None);
}

#[test]
fn cross_references_carry_source_spans_and_roles() {
    let input = "PART 2\n STR #00010\nAND #70011\nOUT #70020\nEND\n";
    let doc = parse_document(input.as_bytes().to_vec(), ControllerFamily::YRC1000);
    let a = analyze(&doc);
    assert_eq!(a.source_sha256, doc.source_sha256);
    let x = a.references.iter().find(|x| x.symbol=="#00010").unwrap();
    assert_eq!(x.sites[0].operation,SymbolOperation::ContactRead);
    assert_eq!(&doc.original_bytes()[x.sites[0].operand_span.start..x.sites[0].operand_span.end],b"#00010");
    let y = a.references.iter().find(|x| x.symbol=="#70020").unwrap();
    assert_eq!(y.sites[0].operation,SymbolOperation::CoilWrite);
}

#[test]
fn timer_is_barrier_but_register_refs_are_recorded() {
    let r = report("STR #70010\nTMR M010,25\nOUT #70020\nSTR #70011\nOUT #70021\nEND\n");
    assert!(codes(&r).contains(&"StatefulOpcode"));
    assert!(codes(&r).contains(&"UnbalancedOutput"));
    assert_eq!(r.rungs.len(),1);
    assert_eq!(r.rungs[0].output,70021);
    let ref_m = r.references.iter().find(|r|r.symbol=="M010").unwrap();
    assert_eq!(ref_m.sites[0].operation,SymbolOperation::RegisterCurrentValue);
    assert!(!r.subset_analysis_complete);
}

#[test]
fn unknown_instruction_breaks_partial_expression_no_false_graph() {
    let r = report("STR #70010\nBMOV M000,M010\nOUT #70020\nSTR #70011\nOUT #70021\nEND\n");
    assert!(codes(&r).contains(&"UnverifiedSource"));
    assert_eq!(r.rungs.len(),1);
    assert_eq!(r.rungs[0].output,70021);
    assert_eq!(r.unsupported_segments > 0,true);
}

#[test]
fn grouped_opcode_potential_collision_is_flagged() {
    let r = report("STR #70010\nOUT #70022\nGOUT #70020\nEND\n");
    assert!(codes(&r).contains(&"PossibleGroupOverlap"));
    assert!(codes(&r).contains(&"GroupedOpcode"));
    assert!(r.references.iter().any(|r|r.symbol=="#70020"&&
        r.sites.iter().any(|s|s.operation==SymbolOperation::PotentialGroupWrite)));
}

#[test]
fn source_part_marker_not_trusted_as_edit_permission() {
    let r = report("PART 1\nPART 2\nEND\n");
    assert!(r.end_seen);
    assert_eq!(r.part_markers.len(),2);
    assert!(r.subset_analysis_complete);
    assert!(!r.system_user_boundaries_verified);
    assert!(!r.edited_export_allowed);
}

#[test]
fn part_inside_incomplete_expression_is_error() {
    let r = report("PART 1\nSTR #70010\nPART 2\nSTR #70011\nOUT #70020\nEND\n");
    assert!(codes(&r).contains(&"PartInsideExpression"));
    assert_eq!(r.rungs.len(),1);
    assert_eq!(r.rungs[0].section,SectionClaim::User);
}

#[test]
fn end_resets_unfinished_state_and_rejects_following_logic() {
    let r = report("STR #70010\nEND\nSTR #70011\nOUT #70020\n");
    assert!(codes(&r).contains(&"UnfinishedAtEnd"));
    assert!(codes(&r).contains(&"AfterEnd"));
    assert!(r.rungs.is_empty());
}

#[test]
fn expression_limit_and_depth_limits_are_fail_closed() {
    let mut code = String::new();
    for _ in 0..(MAX_OPEN_BRANCHES+3) { code.push_str("STR #70010\n"); }
    code.push_str("OUT #70020\nEND\n");
    let r = report(&code);
    assert!(codes(&r).contains(&"BranchDepthLimit"));
    assert!(!r.subset_analysis_complete);
    assert!(r.rungs.is_empty());
}

#[test]
fn report_preview_is_explicitly_truncated() {
    let r = report("STR #70010\nOUT #70020\nSTR #70011\nOUT #70021\nEND\n");
    let p = r.preview(1, 1, 1);
    assert!(p.rungs_truncated);
    assert_eq!(p.rungs.len(),1);
    assert!(p.references_truncated);
    assert!(!p.edited_export_allowed);
}

#[test]
fn file_bytes_are_unchanged_by_semantic_analysis() {
    let raw = b"//header\r\nPART 2\r\nSTR #70010\r\nOUT #70020\r\nEND\r\n".to_vec();
    let doc = parse_document(raw.clone(), ControllerFamily::YRC1000);
    let before = doc.source_sha256.clone();
    let r = analyze(&doc);
    assert_eq!(doc.original_bytes(),raw);
    assert_eq!(r.source_sha256,before);
    assert_eq!(doc.copy_original(),raw);
}
