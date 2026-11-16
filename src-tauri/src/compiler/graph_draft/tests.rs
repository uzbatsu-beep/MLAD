use super::*;
fn leaf(id: &str, relay: &str) -> GraphDraftExpr {
    GraphDraftExpr::Contact { id: id.to_string(), relay: relay.to_string(), normally_closed: false }
}
fn sample() -> GraphDraft {
    GraphDraft {
        version: GRAPH_DRAFT_VERSION, controller: ControllerFamily::YRC1000,
        rungs: vec![GraphDraftRung {
            id: "r1".into(), output: "#70020".into(), comment: "test".into(),
            expression: GraphDraftExpr::And {
                id: "and1".into(),
                left: Box::new(leaf("a","#20010")),
                right: Box::new(GraphDraftExpr::Or {
                    id: "or1".into(),
                    left: Box::new(leaf("b","#70010")),
                    right: Box::new(leaf("c","#70011")),
                }),
            },
        }],
    }
}
#[test]
fn json_roundtrip_preserves_branch_topology() {
    let x = sample();
    let (s, check) = checked_draft_json(&x).unwrap();
    assert!(check.structure_valid && !check.has_errors);
    let (y, check2) = read_draft_json(&s).unwrap();
    assert_eq!(x,y);
    assert!(!check.edited_cioprg_export_allowed);
    assert!(!check2.edited_cioprg_export_allowed);
}
#[test]
fn invalid_coil_class_and_invalid_contact_are_errors() {
    let mut p = sample();
    p.rungs[0].output = "#20010".into();
    let v = validate_graph_draft(&p);
    assert!(v.diagnostics.iter().any(|d| d.code=="InvalidCoilClass"));
    assert!(v.has_errors);
    if let GraphDraftExpr::And { left, .. } = &mut p.rungs[0].expression {
        if let GraphDraftExpr::Contact { relay, .. } = left.as_mut() {
            *relay="#70019".into();
        }
    }
    assert!(validate_graph_draft(&p).diagnostics.iter().any(|d| d.code=="InvalidContactAddress"));
}
#[test]
fn generation_is_required_for_relay_range_check() {
    let mut p=sample();
    p.rungs[0].output="#87010".into();
    assert!(validate_graph_draft(&p).diagnostics.iter().any(|d|d.code=="InvalidCoilClass"));
    p.controller=ControllerFamily::DX200;
    assert!(validate_graph_draft(&p).diagnostics.iter().any(|d|d.code=="InvalidOutputAddress"));
}
#[test]
fn duplicate_coils_and_ids_detected() {
    let mut p=sample();
    let mut duplicate=p.rungs[0].clone();
    duplicate.id="r2".into();
    p.rungs.push(duplicate);
    let r=validate_graph_draft(&p);
    assert!(r.diagnostics.iter().any(|d| d.code=="DoubleCoil"));
    assert!(r.diagnostics.iter().any(|d| d.code=="DuplicateNodeId"));
    assert!(!r.structure_valid);
}
#[test]
fn self_feedback_is_a_warning_not_fatal() {
    let mut p=sample();
    p.rungs[0].output="#70010".into();
    let r=validate_graph_draft(&p);
    assert!(r.diagnostics.iter().any(|d|d.code=="SelfFeedback"));
    assert!(!r.has_errors);
}
#[test]
fn invalid_addresses_can_be_saved_in_a_draft_but_not_exported_to_cio() {
    let mut p=sample();
    p.rungs[0].output="#XXXXX".into();
    let (s,v)=checked_draft_json(&p).unwrap();
    assert!(v.has_errors);
    assert!(s.contains("XXXXX"));
    assert!(!v.edited_cioprg_export_allowed);
}
#[test]
fn unsupported_versions_are_rejected() {
    let mut p=sample();
    p.version=42;
    assert!(checked_draft_json(&p).is_err());
    let s=serde_json::to_string(&p).unwrap();
    assert!(read_draft_json(&s).is_err());
}
#[test]
fn max_rungs_guard() {
    let mut p=sample();
    p.rungs=(0..129).map(|i|GraphDraftRung {
        id:format!("r{i}"), output:format!("#{:05}",70010+i),
        comment:"".into(),expression:leaf(&format!("c{i}"),"#70010")
    }).collect();
    assert!(validate_graph_draft(&p).diagnostics.iter().any(|d|d.code=="TooManyRungs"));
    assert!(checked_draft_json(&p).is_err());
}
#[test]
fn deep_nested_json_model_rejected_without_recursion_in_validation() {
    let mut p=sample();
    for i in 0..(MAX_DRAFT_DEPTH + 3) {
        p.rungs[0].expression=GraphDraftExpr::And {
            id:format!("n{i}"),
            left:Box::new(p.rungs[0].expression.clone()),
            right:Box::new(leaf(&format!("extra{i}"),"#70010")),
        };
    }
    assert!(!validate_graph_draft(&p).structure_valid);
}
#[test]
fn malformed_json_and_oversize_are_rejected() {
    assert!(read_draft_json("{broken").is_err());
    assert!(read_draft_json(&" ".repeat(2*1024*1024+1)).is_err());
}
#[test]
fn two_controller_profiles_enforce_different_input_ranges() {
    let mut p=sample();
    p.controller=ControllerFamily::DX100;
    if let GraphDraftExpr::And { left, .. } = &mut p.rungs[0].expression {
        if let GraphDraftExpr::Contact { relay, .. } = left.as_mut() {
            *relay="#02570".into();
        }
    }
    assert!(validate_graph_draft(&p).has_errors);
    p.controller=ControllerFamily::DX200;
    assert!(!validate_graph_draft(&p).has_errors);
}
