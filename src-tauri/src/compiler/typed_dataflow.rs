//! Static source-order evidence for partially typed CIO instructions.
//!
//! This module makes NO executable semantics or scan-cycle claims. A
//! "possible_previous_definition" is only a lexical proximity observation:
//! same M-register token, preceding valid typed candidate, no intervening
//! unsupported/boolean/advanced instruction. It does not prove a real read,
//! write, enable condition, flag transition, or assignment on hardware.
//! Byte spans and source hashes always refer to the immutable original export.
use std::collections::BTreeMap;
use serde::Serialize;
use super::lossless::{AstInstruction, ByteSpan, LineKind, LosslessDocument, OperandKind};

pub const MAX_STATIC_EVENTS_PREVIEW: usize = 1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CandidateRole {
    Source,
    ShiftAmount,
    Destination,
}

#[derive(Debug, Clone, Serialize)]
pub struct CandidateSite {
    pub spelling: String,
    pub kind: OperandKind,
    pub span: ByteSpan,
    pub role: CandidateRole,
    /// ONLY a possible lexical adjacency in source order, not runtime flow.
    pub possible_previous_definition: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StaticEvent {
    pub line: usize,
    pub mnemonic: String,
    pub syntax_shape_valid: bool,
    pub controller_form_verified: bool,
    pub semantics_verified: bool,
    pub execution_condition_verified: bool,
    pub arithmetic_flags_verified: bool,
    pub sites: Vec<CandidateSite>,
    pub possible_prior_lines: Vec<usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StaticAudit {
    pub source_sha256: String,
    pub total_candidates: usize,
    pub valid_shape_candidates: usize,
    pub invalid_shape_candidates: usize,
    pub possible_source_order_links: usize,
    pub continuity_barriers: usize,
    pub events_truncated: bool,
    pub events: Vec<StaticEvent>,
    pub semantics_verified: bool,
    pub scan_order_verified: bool,
    pub arithmetic_flags_verified: bool,
    pub native_write_allowed: bool,
}

fn destination_index(inst: &AstInstruction) -> Option<usize> {
    match inst.mnemonic.as_str() {
        "MOV" => Some(1),
        "ADD" | "SUB" | "SHL" | "SHR" => Some(2),
        _ => None,
    }
}

/// Strict M-register identity only. No group aliasing or overlap is inferred.
/// Registers have exact three-digit spelling in the accepted grammar.
fn local_register_key(spelling: &str, kind: OperandKind) -> Option<&str> {
    if kind==OperandKind::MRegister && spelling.len()==4 &&
        spelling.starts_with('M') &&
        spelling.as_bytes()[1..].iter().all(u8::is_ascii_digit) {
        Some(spelling)
    } else {
        None
    }
}
/// Creates only bounded *display* evidence. Unsupported syntax or any other
/// instruction clears lexical linkage to avoid pretending execution continuity.
pub fn inspect(doc: &LosslessDocument, max_events: usize) -> StaticAudit {
    let max_events=max_events.min(MAX_STATIC_EVENTS_PREVIEW);
    let mut definitions=BTreeMap::<String,usize>::new();
    let mut events=Vec::new();
    let mut total=0usize;
    let mut valid=0usize;
    let mut invalid=0usize;
    let mut links=0usize;
    let mut barriers=0usize;
    let mut after_end=false;
    for line in &doc.lines {
        if after_end { break; }
        match &line.kind {
            LineKind::Empty | LineKind::Comment => {}
            LineKind::TypedCandidate {
                instruction, controller_form_verified, ..
            } => {
                total+=1;
                // A well-formed operand shape is not the same as an OEM-
                // verified controller form (especially for DX100).
                if instruction.syntax_valid { valid+=1; } else { invalid+=1; }
                let possible=instruction.syntax_valid &&
                    *controller_form_verified && destination_index(instruction).is_some();
                if !possible {
                    definitions.clear();
                    barriers+=1;
                }
                let dest=destination_index(instruction);
                let mut sites=Vec::new();
                let mut prior=Vec::new();
                for (index,op) in instruction.operands.iter().enumerate() {
                    let role=if Some(index)==dest { CandidateRole::Destination }
                        else if matches!(instruction.mnemonic.as_str(),"SHL"|"SHR") &&
                            index==1 { CandidateRole::ShiftAmount }
                        else { CandidateRole::Source };
                    let prior_line=if possible && role==CandidateRole::Source {
                        local_register_key(&op.spelling,op.kind)
                            .and_then(|key|definitions.get(key).copied())
                    } else { None };
                    if let Some(previous)=prior_line {
                        links+=1;
                        if !prior.contains(&previous){prior.push(previous);}
                    }
                    sites.push(CandidateSite {
                        spelling:op.spelling.clone(),kind:op.kind,span:op.span,
                        role,possible_previous_definition:prior_line,
                    });
                }
                if possible {
                    if let Some(index)=dest {
                        if let Some(op)=instruction.operands.get(index) {
                            if let Some(key)=local_register_key(&op.spelling,op.kind) {
                                definitions.insert(key.to_string(),line.number);
                            }
                        }
                    }
                }
                if events.len()<max_events {
                    events.push(StaticEvent {
                        line:line.number,mnemonic:instruction.mnemonic.clone(),
                        syntax_shape_valid:instruction.syntax_valid,
                        controller_form_verified:*controller_form_verified,
                        semantics_verified:false,execution_condition_verified:false,
                        arithmetic_flags_verified:false,sites,
                        possible_prior_lines:prior,
                    });
                }
            }
            LineKind::Instruction(inst) => {
                if inst.mnemonic=="END" && inst.syntax_valid { after_end=true; }
                // Even known boolean opcodes may affect execution conditions.
                // Never assume their interaction with typed opcodes is inert.
                definitions.clear();
                barriers+=1;
            }
            LineKind::PostEndSection{..} | LineKind::PostEndData |
            LineKind::PostEndOpaque => {after_end=true;definitions.clear();}
            _ => {
                definitions.clear();
                barriers+=1;
            }
        }
    }
    StaticAudit {
        source_sha256:doc.source_sha256.clone(),total_candidates:total,
        valid_shape_candidates:valid,invalid_shape_candidates:invalid,
        possible_source_order_links:links,continuity_barriers:barriers,
        events_truncated:total>events.len(),events,
        semantics_verified:false,scan_order_verified:false,
        arithmetic_flags_verified:false,native_write_allowed:false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::memory::ControllerFamily;
    use crate::compiler::lossless::parse_document;

    fn audit(src:&[u8])->(LosslessDocument,StaticAudit) {
        let doc=parse_document(src.to_vec(),ControllerFamily::YRC1000);
        let out=inspect(&doc,100);
        (doc,out)
    }

    #[test]
    fn identical_m_register_in_adjacent_valid_candidates_is_only_possible_link() {
        let raw=b"MOV 1,M010\r\nADD M010,2,M011\r\nSUB M011,M010,M012\r\nEND\r\n";
        let (doc,a)=audit(raw);
        assert_eq!(a.total_candidates,3);
        assert_eq!(a.valid_shape_candidates,3);
        assert_eq!(a.possible_source_order_links,3);
        assert_eq!(a.events[1].sites[0].possible_previous_definition,Some(1));
        assert_eq!(a.events[2].sites[0].possible_previous_definition,Some(2));
        assert_eq!(a.events[2].sites[1].possible_previous_definition,Some(1));
        assert!(!a.semantics_verified);
        assert!(!a.scan_order_verified);
        assert!(!a.arithmetic_flags_verified);
        assert!(!a.native_write_allowed);
        assert_eq!(doc.original_bytes(),raw);
        assert_eq!(doc.copy_original(),raw);
    }

    #[test]
    fn any_boolean_or_unknown_instruction_breaks_lexical_link() {
        let raw=b"MOV 1,M010\nSTR #70010\nADD M010,2,M011\nXOR M011,M010\nSUB M011,1,M012\nEND\n";
        let (_,a)=audit(raw);
        assert_eq!(a.total_candidates,3);
        assert_eq!(a.possible_source_order_links,0);
        assert!(a.continuity_barriers>=3);
    }

    #[test]
    fn missing_form_and_part_markers_do_not_create_false_links() {
        let raw=b"MOV 1,M010\nMOV M010,M999\nADD M010,2,M011\nPART 2\nSUB M011,1,M012\nEND\n";
        let (_,a)=audit(raw);
        assert_eq!(a.total_candidates,4);
        assert_eq!(a.invalid_shape_candidates,1);
        assert_eq!(a.events[2].sites[0].possible_previous_definition,None);
        assert_eq!(a.events[3].sites[0].possible_previous_definition,None);
    }

    #[test]
    fn no_group_overlap_or_after_end_execution_is_inferred() {
        let raw=b"MOV 1,W#70010\nSHL W#70010,3,W#70020\nEND\n//ALARM1\nADD 1,2,M010\n";
        let (doc,a)=audit(raw);
        assert_eq!(a.total_candidates,2);
        assert_eq!(a.possible_source_order_links,0);
        assert_eq!(a.events[1].sites[1].role,CandidateRole::ShiftAmount);
        assert_eq!(a.events[0].sites[1].role,CandidateRole::Destination);
        assert_eq!(doc.copy_original(),raw);
    }

    #[test]
    fn report_is_bounded_but_total_and_hash_cover_all_candidates() {
        let mut raw=Vec::new();
        for _ in 0..105 {raw.extend_from_slice(b"MOV 1,M010\n");}
        raw.extend_from_slice(b"END\n");
        let doc=parse_document(raw.clone(),ControllerFamily::YRC1000);
        let result=inspect(&doc,5);
        assert_eq!(result.total_candidates,105);
        assert_eq!(result.events.len(),5);
        assert!(result.events_truncated);
        assert_eq!(doc.copy_original(),raw);
        let empty=inspect(&doc,0);
        assert!(empty.events_truncated);
        assert!(empty.events.is_empty());
    }

    #[test]
    fn dx100_forms_are_not_elevated_to_evidence() {
        let raw=b"MOV 1,M010\nADD M010,2,M011\nEND\n";
        let doc=parse_document(raw.to_vec(),ControllerFamily::DX100);
        let audit=inspect(&doc,100);
        assert_eq!(audit.total_candidates,2);
        assert_eq!(audit.valid_shape_candidates,2);
        assert_eq!(audit.invalid_shape_candidates,0);
        assert_eq!(audit.possible_source_order_links,0);
        assert!(audit.events.iter().all(|e| !e.controller_form_verified));
    }
}
