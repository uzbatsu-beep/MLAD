pub mod parser;
pub mod lossless;
pub mod semantic;
pub mod graph_draft;
pub mod memory;

use serde::{Deserialize, Serialize};
use crate::Rung;
use memory::{parse_relay, relay_write_policy, AddressError, ControllerFamily, LadderWritePolicy};

/// Instructions recognised by the *planned* compiler. Do not treat this enum as
/// proof that the current legacy .LST parser supports all of these instructions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Token {
    Str(String),
    StrNot(String),
    And(String),
    AndNot(String),
    Or(String),
    OrNot(String),
    Out(String),
    Tmr { timer_id: u32, setpoint: u32 },
    Cnt { counter_id: u32, setpoint: u32 },
    Mov { src: String, dest: String },
    Unknown(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationError {
    pub rung_id: usize,
    pub error_type: String,
    pub message: String,
}

/// Compatibility wrapper only. Do NOT use the implicit YRC1000 profile for
/// production/export: the controller profile must be selected explicitly.
pub fn validate_rungs(rungs: &[Rung]) -> Vec<ValidationError> {
    validate_rungs_for_controller(rungs, ControllerFamily::YRC1000)
}

/// Static checks for bit operands supported by the legacy editor.
/// They are NOT a full concurrent-I/O compiler, firmware-dependent access
/// validator or permission to download a program to a robot.
pub fn validate_rungs_for_controller(rungs: &[Rung], controller: ControllerFamily) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let mut written_coils = std::collections::HashSet::new();

    for rung in rungs {
        for inst in &rung.instructions {
            let is_bit_instruction = matches!(inst.inst_type.as_str(),
                "STR" | "STR-NOT" | "AND" | "AND-NOT" |
                "OR" | "OR-NOT" | "OUT");
            if !is_bit_instruction {
                continue; // other instruction operand schemas not yet implemented
            }

            let relay = match parse_relay(controller, &inst.operand) {
                Ok(address) => address,
                Err(reason) => {
                    let error_type = match reason {
                        AddressError::InvalidBitIndex => "InvalidBitIndex",
                        AddressError::UnknownRelayRange => "UnknownAddress",
                        _ => "InvalidOperand",
                    };
                    errors.push(ValidationError {
                        rung_id: rung.id,
                        error_type: error_type.into(),
                        message: format!("{} {}: {:?} ({:?})", inst.inst_type, inst.operand, reason, controller),
                    });
                    continue;
                }
            };

            if inst.inst_type == "OUT" {
                if !written_coils.insert(relay.number) {
                    errors.push(ValidationError {
                        rung_id: rung.id,
                        error_type: "DoubleCoil".into(),
                        message: format!("Double-coil: OUT {} assigned more than once", relay.display()),
                    });
                }
                match relay_write_policy(relay.class) {
                    LadderWritePolicy::Denied => errors.push(ValidationError {
                        rung_id: rung.id,
                        error_type: "ReadOnlyAddress".into(),
                        message: format!("OUT {} targets non-writable class {:?} ({:?})",
                            relay.display(), relay.class, controller),
                    }),
                    LadderWritePolicy::RequiresReview => errors.push(ValidationError {
                        rung_id: rung.id,
                        error_type: "ReviewRequired".into(),
                        message: format!("OUT {} requires configuration-specific review ({:?})",
                            relay.display(), relay.class),
                    }),
                    LadderWritePolicy::Candidate => (),
                }
            }
        }
    }
    errors
}
#[cfg(test)]
mod tests;
