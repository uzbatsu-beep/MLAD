//! Standalone JSON-only graph drafts for MLAD's offline editor.
//! Never serializes a native CIOPRG.LST or changes protected controller logic.
//! The draft schema is versioned to avoid mixing it with the legacy Vec<Rung>.

use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use super::memory::{
    parse_relay, relay_write_policy, ControllerFamily, LadderWritePolicy,
};

pub const GRAPH_DRAFT_VERSION: u32 = 1;
pub const MAX_DRAFT_RUNGS: usize = 128;
pub const MAX_DRAFT_NODES: usize = 4096;
pub const MAX_DRAFT_DEPTH: usize = 64;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphDraft {
    pub version: u32,
    pub controller: ControllerFamily,
    pub rungs: Vec<GraphDraftRung>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphDraftRung {
    pub id: String,
    pub output: String,
    pub comment: String,
    pub expression: GraphDraftExpr,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum GraphDraftExpr {
    Contact {
        id: String,
        relay: String,
        #[serde(rename = "normallyClosed")]
        normally_closed: bool,
    },
    And {
        id: String,
        left: Box<GraphDraftExpr>,
        right: Box<GraphDraftExpr>,
    },
    Or {
        id: String,
        left: Box<GraphDraftExpr>,
        right: Box<GraphDraftExpr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DraftLevel { Error, Warning }

#[derive(Debug, Clone, Serialize)]
pub struct DraftDiagnostic {
    pub rung_id: String,
    pub level: DraftLevel,
    pub code: &'static str,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DraftValidation {
    pub diagnostics: Vec<DraftDiagnostic>,
    /// Draft validity is NOT approval for controller export/deployment.
    pub structure_valid: bool,
    pub has_errors: bool,
    pub edited_cioprg_export_allowed: bool,
}

fn diag(diags: &mut Vec<DraftDiagnostic>, rung_id: &str, level: DraftLevel,
        code: &'static str, message: impl Into<String>) {
    if diags.len() < 500 {
        diags.push(DraftDiagnostic {
            rung_id: rung_id.to_owned(), level, code, message: message.into(),
        });
    }
}

/// Validates only offline editing rules and memory classifications.
/// Does not prove vendor instruction semantics or permission to write PLC I/O.
pub fn validate_graph_draft(project: &GraphDraft) -> DraftValidation {
    let mut diags = Vec::new();
    let mut structure_valid = true;
    if project.version != GRAPH_DRAFT_VERSION {
        diag(&mut diags, "", DraftLevel::Error, "UnsupportedVersion",
             format!("Неизвестная версия проекта: {}", project.version));
        structure_valid = false;
    }
    if project.rungs.len() > MAX_DRAFT_RUNGS {
        diag(&mut diags, "", DraftLevel::Error, "TooManyRungs", "Превышено число цепей черновика");
        structure_valid = false;
    }

    let mut rung_ids = HashSet::new();
    let mut component_ids = HashSet::new();
    let mut coil_writers: HashMap<u32, String> = HashMap::new();
    let mut nodes_total = 0;

    for rung in &project.rungs {
        let id = rung.id.as_str();
        if id.is_empty() || !rung_ids.insert(id.to_owned()) {
            structure_valid = false;
            diag(&mut diags, id, DraftLevel::Error, "DuplicateRungId",
                 "Идентификатор цепи пуст или повторяется");
        }
        if rung.comment.len() > 2048 {
            structure_valid = false;
            diag(&mut diags, id, DraftLevel::Error, "CommentTooLong", "Комментарий длиннее 2048 байт");
        }

        let coil = if !rung.output.starts_with('#') {
            Err(super::memory::AddressError::MalformedRelay)
        } else { parse_relay(project.controller, &rung.output) };

        if let Ok(output) = coil {
            if let Some(earlier) = coil_writers.insert(output.number, id.to_owned()) {
                diag(&mut diags, id, DraftLevel::Error, "DoubleCoil",
                     format!("OUT {} уже назначен цепи {}", rung.output, earlier));
            }
            match relay_write_policy(output.class) {
                LadderWritePolicy::Denied =>
                    diag(&mut diags, id, DraftLevel::Error, "InvalidCoilClass",
                         format!("OUT {} относится к недопустимому классу {:?}", rung.output, output.class)),
                LadderWritePolicy::RequiresReview =>
                    diag(&mut diags, id, DraftLevel::Warning, "CoilNeedsReview",
                         format!("OUT {} требует проверки конфигурации робота", rung.output)),
                LadderWritePolicy::Candidate => {},
            }
        } else {
            diag(&mut diags, id, DraftLevel::Error, "InvalidOutputAddress",
                 format!("Неверный адрес OUT {} для {:?}", rung.output, project.controller));
        }

        let mut inputs = HashSet::new();
        let mut stack: Vec<(&GraphDraftExpr, usize)> = vec![(&rung.expression, 1)];
        while let Some((node, depth)) = stack.pop() {
            nodes_total += 1;
            if nodes_total > MAX_DRAFT_NODES || depth > MAX_DRAFT_DEPTH {
                structure_valid = false;
                diag(&mut diags, id, DraftLevel::Error, "ComplexityLimit",
                     "Превышены лимиты размера или вложенности черновика");
                break;
            }
            let node_id = match node {
                GraphDraftExpr::Contact { id, .. } |
                GraphDraftExpr::And { id, .. } |
                GraphDraftExpr::Or { id, .. } => id,
            };
            if node_id.is_empty() || !component_ids.insert(node_id.to_owned()) {
                structure_valid = false;
                diag(&mut diags, id, DraftLevel::Error, "DuplicateNodeId",
                     "Идентификатор элемента отсутствует или повторяется");
            }
            match node {
                GraphDraftExpr::Contact { relay, .. } => {
                    let address = if relay.starts_with('#') {
                        parse_relay(project.controller, relay)
                    } else {
                        Err(super::memory::AddressError::MalformedRelay)
                    };
                    match address {
                        Ok(a) => { inputs.insert(a.number); },
                        Err(_) => diag(&mut diags, id, DraftLevel::Error, "InvalidContactAddress",
                            format!("Недопустимый контакт {} для {:?}", relay, project.controller)),
                    }
                }
                GraphDraftExpr::And { left, right, .. } |
                GraphDraftExpr::Or { left, right, .. } => {
                    stack.push((right, depth+1));
                    stack.push((left, depth+1));
                }
            }
        }
        if let Ok(output) = coil {
            if inputs.contains(&output.number) {
                diag(&mut diags, id, DraftLevel::Warning, "SelfFeedback",
                     "Цепь считывает собственную выходную катушку; возможно, это защёлка");
            }
        }
        if nodes_total > MAX_DRAFT_NODES { break; }
    }
    let has_errors = diags.iter().any(|d| d.level == DraftLevel::Error);
    DraftValidation {
        diagnostics: diags,
        structure_valid,
        has_errors,
        edited_cioprg_export_allowed: false,
    }
}

pub fn checked_draft_json(draft: &GraphDraft) -> Result<(String, DraftValidation), String> {
    let validation = validate_graph_draft(draft);
    if !validation.structure_valid {
        return Err("Структура черновика нарушена или превышает установленные ограничения".into());
    }
    let serialized = serde_json::to_string_pretty(draft)
        .map_err(|e| format!("Ошибка сериализации черновика: {e}"))?;
    if serialized.len() > 2 * 1024 * 1024 {
        return Err("Черновик превышает предел 2 МиБ".into());
    }
    Ok((serialized, validation))
}

pub fn read_draft_json(contents: &str) -> Result<(GraphDraft, DraftValidation), String> {
    if contents.len() > 2 * 1024 * 1024 { return Err("Черновик превышает предел 2 МиБ".into()); }
    let draft: GraphDraft = serde_json::from_str(contents)
        .map_err(|e| format!("Недопустимый формат графического черновика: {e}"))?;
    let validation = validate_graph_draft(&draft);
    if !validation.structure_valid {
        return Err("Повреждённая структура графического черновика".into());
    }
    Ok((draft, validation))
}

#[cfg(test)]
mod tests;
