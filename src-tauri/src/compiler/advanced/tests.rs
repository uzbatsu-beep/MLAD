use super::*;

fn leaf(id: &str, relay: &str) -> GraphDraftExpr {
    GraphDraftExpr::Contact { id:id.into(), relay:relay.into(), normally_closed:false }
}
fn tmr(id:&str, curr:&str, p:Preset, output:&str) -> AdvancedBlock {
    AdvancedBlock::Tmr {
        id:id.into(),enable:leaf(&format!("{id}-en"),"#70010"),
        current:curr.into(),preset:p,output:output.into()
    }
}
fn cnt(id:&str, curr:&str, output:&str) -> AdvancedBlock {
    AdvancedBlock::Cnt {
        id:id.into(),count:leaf(&format!("{id}-count"),"#70011"),
        reset:leaf(&format!("{id}-reset"),"#70012"),
        current:curr.into(),preset:Preset::Literal{value:3},output:output.into()
    }
}
fn group(id:&str,source:&str,destination:&str)->AdvancedBlock {
    AdvancedBlock::Group{id:id.into(),source:source.into(),destination:destination.into()}
}
fn draft(blocks:Vec<AdvancedBlock>,controller:ControllerFamily)->AdvancedDraft {
    AdvancedDraft{version:ADVANCED_DRAFT_VERSION,controller,blocks}
}
fn codes(v:&Validation)->Vec<&'static str>{v.diagnostics.iter().map(|d|d.code).collect()}

#[test]
fn valid_timer_and_counter_and_group_are_typed() {
    let d=draft(vec![
        tmr("tmr","M010",Preset::Literal{value:100},"#70100"),
        cnt("cnt","M020","#70110"),
        group("gr","#20010","#00010"),
    ],ControllerFamily::YRC1000);
    let v=validate(&d);
    assert!(!v.has_errors,"{:?}",codes(&v));
    assert!(v.structure_valid);
    assert!(!v.native_export_allowed);
    assert!(!v.hardware_test_authorized);
    let (j,_) = serialize_checked(&d).unwrap();
    let (r,_) = deserialize_checked(&j).unwrap();
    assert_eq!(r,d);
    assert!(j.contains("\"TMR\""));
    assert!(j.contains("\"CNT\""));
    assert!(j.contains("\"GROUP\""));
}
#[test]
fn timer_100ms_ticks_and_reset_are_deterministic() {
    let mut t=TimerState::new(3);
    assert_eq!(t.current,3); assert!(!t.contact);
    t.tick(true,3); assert_eq!(t.current,2); assert!(!t.contact);
    t.tick(true,3); assert_eq!(t.current,1); assert!(!t.contact);
    t.tick(true,3); assert_eq!(t.current,0); assert!(t.contact);
    t.tick(true,3); assert_eq!(t.current,0); assert!(t.contact);
    t.tick(false,3); assert_eq!(t.current,3); assert!(!t.contact);
    t.tick(true,3); assert_eq!(t.current,2);
}
#[test]
fn timer_10_second_example_requires_100_explicit_ticks() {
    let mut t=TimerState::new(100);
    for _ in 0..99 {t.tick(true,100);}
    assert_eq!(t.current,1);assert!(!t.contact);
    t.tick(true,100);
    assert_eq!(t.current,0);assert!(t.contact);
}
#[test]
fn counter_counts_rising_edges_not_held_level() {
    let mut c=CounterState::new(3);
    c.scan(true,false,3); assert_eq!(c.current,2);
    c.scan(true,false,3); assert_eq!(c.current,2);
    c.scan(false,false,3); assert_eq!(c.current,2);
    c.scan(true,false,3); assert_eq!(c.current,1);
    c.scan(false,false,3);c.scan(true,false,3);
    assert_eq!(c.current,0); assert!(c.contact);
    c.scan(false,false,3);c.scan(true,false,3);
    assert_eq!(c.current,0);assert!(c.contact);
}
#[test]
fn counter_reset_priority_and_held_count_during_reset() {
    let mut c=CounterState::new(3);
    c.scan(true,true,3);assert_eq!(c.current,3);assert!(!c.contact);
    c.scan(true,false,3);assert_eq!(c.current,3); // no false edge
    c.scan(false,false,3);c.scan(true,false,3);assert_eq!(c.current,2);
    c.scan(false,true,3);assert_eq!(c.current,3);assert!(!c.contact);
}
#[test]
fn group_transfer_exactly_one_byte() {
    for x in [0,1,128,255] {assert_eq!(group_copy(x),x);}
}
#[test]
fn timer_and_counter_current_are_restricted_to_m000_m559() {
    let x=draft(vec![tmr("a","M559",Preset::Literal{value:65535},"#70100")],ControllerFamily::DX100);
    assert!(!validate(&x).has_errors);
    let x=draft(vec![tmr("a","M560",Preset::Literal{value:1},"#70100")],ControllerFamily::DX100);
    assert!(codes(&validate(&x)).contains(&"InvalidCurrentRegister"));
    let x=draft(vec![cnt("a","M999","#70100")],ControllerFamily::DX100);
    assert!(codes(&validate(&x)).contains(&"InvalidCurrentRegister"));
}
#[test]
fn setpoint_can_be_m999_but_not_m1000() {
    let x=draft(vec![tmr("a","M010",Preset::Register{address:"M999".into()},"#70100")],ControllerFamily::DX200);
    assert!(!validate(&x).has_errors);
    let x=draft(vec![tmr("a","M010",Preset::Register{address:"M1000".into()},"#70100")],ControllerFamily::DX200);
    assert!(codes(&validate(&x)).contains(&"InvalidPresetRegister"));
}
#[test]
fn current_register_double_ownership_is_an_error() {
    let x=draft(vec![tmr("a","M010",Preset::Literal{value:1},"#70100"),
        cnt("b","M010","#70110")],ControllerFamily::YRC1000);
    assert!(codes(&validate(&x)).contains(&"DuplicateCurrentRegister"));
}
#[test]
fn duplicate_coils_and_group_overlap_are_blocked() {
    let x=draft(vec![tmr("a","M010",Preset::Literal{value:1},"#70012"),
        cnt("b","M020","#70012"),group("c","#20010","#70010")],ControllerFamily::DX200);
    let v=validate(&x);
    assert!(codes(&v).contains(&"DoubleCoil"));
    assert!(codes(&v).contains(&"GroupCoilOverlap"));
}
#[test]
fn unaligned_group_address_is_invalid() {
    let x=draft(vec![group("gr","#20011","#70010")],ControllerFamily::YRC1000);
    assert!(codes(&validate(&x)).contains(&"InvalidGroupSource"));
    let x=draft(vec![group("gr","#20010","#70011")],ControllerFamily::YRC1000);
    assert!(codes(&validate(&x)).contains(&"InvalidGroupDestination"));
}
#[test]
fn controller_specific_memory_bounds_are_respected() {
    let a=draft(vec![group("gr","#04010","#70010")],ControllerFamily::DX100);
    assert!(codes(&validate(&a)).contains(&"InvalidGroupSource"));
    let b=draft(vec![group("gr","#04010","#70010")],ControllerFamily::YRC1000);
    assert!(!codes(&validate(&b)).contains(&"InvalidGroupSource"));
    // Different pseudo-input mapping for DX100/DX200 and YRC1000.
    let a=draft(vec![group("gr","#87010","#70010")],ControllerFamily::YRC1000);
    assert!(!codes(&validate(&a)).contains(&"InvalidGroupSource"));
    let b=draft(vec![group("gr","#87010","#70010")],ControllerFamily::DX200);
    assert!(codes(&validate(&b)).contains(&"InvalidGroupSource"));
}
#[test]
fn invalid_destination_policy_rejects_control_status_and_external_input() {
    let a=draft(vec![group("gr","#20010","#80010")],ControllerFamily::DX200);
    assert!(codes(&validate(&a)).contains(&"GroupDestinationDenied"));
    let a=draft(vec![group("gr","#20010","#20010")],ControllerFamily::DX200);
    assert!(codes(&validate(&a)).contains(&"GroupDestinationDenied"));
}
#[test]
fn unknown_input_propagates_and_false_short_circuits() {
    let expr=GraphDraftExpr::And{id:"branch".into(),
        left:Box::new(leaf("a","#70010")),right:Box::new(leaf("b","#70011"))};
    let mut s=HashMap::new();
    assert_eq!(evaluate_condition(&expr,&s,ControllerFamily::DX100),None);
    s.insert(70010,false);
    assert_eq!(evaluate_condition(&expr,&s,ControllerFamily::DX100),Some(false));
    s.insert(70010,true);s.insert(70011,true);
    assert_eq!(evaluate_condition(&expr,&s,ControllerFamily::DX100),Some(true));
}
#[test]
fn invalid_version_and_duplicate_ids_are_structural_failures() {
    let mut d=draft(vec![tmr("x","M010",Preset::Literal{value:1},"#70100")],ControllerFamily::DX100);
    d.version=2;assert!(!validate(&d).structure_valid);
    assert!(serialize_checked(&d).is_err());
    d.version=1;
    d.blocks.push(tmr("x","M020",Preset::Literal{value:1},"#70110"));
    assert!(codes(&validate(&d)).contains(&"DuplicateBlockId"));
}
#[test]
fn zero_presets_are_warnings_and_no_hardware_authorization() {
    let d=draft(vec![tmr("zero","M010",Preset::Literal{value:0},"#70100")],ControllerFamily::YRC1000);
    let v=validate(&d);
    assert!(!v.has_errors);assert!(codes(&v).contains(&"ZeroPreset"));
    assert!(!v.hardware_test_authorized);
}

#[test]
fn yaskawa_error_4220_budget_is_reflected_in_draft_diagnostics() {
    let blocks:Vec<_>=(0..101).map(|i|
        tmr(&format!("t{i}"),&format!("M{i:03}"),
            Preset::Literal{value:1},&format!("#{:05}",70100+i*10))).collect();
    let d=draft(blocks,ControllerFamily::DX200);
    let validation=validate(&d);
    assert!(codes(&validation).contains(&"YaskawaInstructionBudget"));
    assert!(validation.has_errors);
}
