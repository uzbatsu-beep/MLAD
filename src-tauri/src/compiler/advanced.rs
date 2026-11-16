//! Yaskawa advanced CIO instructions, offline-only.
//! Manual sources: DX100 RE-CKI-A453 §§12.2.12–14; DX200 RE-CKI-A465
//! §§12.2.12–14; YRC1000 RE-CKI-A467 §§12.2.12–14.
//! The model is a typed, **independent JSON draft**, not CIOPRG.LST output.
//! Simulators are deterministic test aids, not controller scan emulators.

use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use super::graph_draft::GraphDraftExpr;
use super::memory::{
    parse_group, parse_register, parse_relay, relay_write_policy, ControllerFamily,
    LadderWritePolicy, RelayGroupWidth,
};

pub const ADVANCED_DRAFT_VERSION: u32 = 1;
pub const MAX_BLOCKS: usize = 128;
pub const MAX_EXPR_NODES: usize = 4096;
pub const MAX_EXPR_DEPTH: usize = 64;

/// An integer 0..65535 or a register M000..M999. Kept as spelled in JSON;
/// the controller's actual register content is never inferred.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Preset {
    Literal { value: u16 },
    Register { address: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "UPPERCASE")]
pub enum AdvancedBlock {
    /// Start input is a Boolean condition; the current register is decremented
    /// every 100 ms while start remains ON. Output represents TMR contact.
    Tmr {
        id: String,
        enable: GraphDraftExpr,
        current: String,
        preset: Preset,
        output: String,
    },
    /// Counter input and RESET input are *independent Boolean conditions*.
    /// Reset takes precedence over a rising counter edge.
    Cnt {
        id: String,
        count: GraphDraftExpr,
        reset: GraphDraftExpr,
        current: String,
        preset: Preset,
        output: String,
    },
    /// GSTR + GOUT paired 8-bit transfer. Neither has implicit analog values.
    /// Source and destination must begin at bit zero of a relay byte.
    Group {
        id: String,
        source: String,
        destination: String,
    },
}

impl AdvancedBlock {
    pub fn id(&self) -> &str {
        match self {
            Self::Tmr { id, .. } | Self::Cnt { id, .. } |
            Self::Group { id, .. } => id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdvancedDraft {
    pub version: u32,
    pub controller: ControllerFamily,
    pub blocks: Vec<AdvancedBlock>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Level { Error, Warning }

#[derive(Debug, Clone, Serialize)]
pub struct Diagnostic {
    pub block_id: String,
    pub code: &'static str,
    pub level: Level,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Validation {
    pub diagnostics: Vec<Diagnostic>,
    pub has_errors: bool,
    pub structure_valid: bool,
    pub native_export_allowed: bool,
    pub hardware_test_authorized: bool,
}

struct Checker {
    diagnostics: Vec<Diagnostic>,
    structural: bool,
    node_count: usize,
    used_node_ids: HashSet<String>,
    seen_blocks: HashSet<String>,
    used_coils: HashMap<u32, String>,
    used_current_registers: HashMap<u16, String>,
    used_group_writes: HashMap<u32, String>,
}

impl Checker {
    fn new() -> Self {
        Self {
            diagnostics: vec![], structural: true, node_count: 0,
            used_node_ids: HashSet::new(), seen_blocks: HashSet::new(),
            used_coils: HashMap::new(), used_current_registers: HashMap::new(),
            used_group_writes: HashMap::new(),
        }
    }
    fn diagnostic(&mut self, id: &str, code: &'static str, level: Level, msg: impl Into<String>) {
        if self.diagnostics.len() < 1000 {
            self.diagnostics.push(Diagnostic {
                block_id: id.into(), code, level, message: msg.into(),
            });
        }
    }
    fn check_expr(&mut self, id: &str, expression: &GraphDraftExpr, controller: ControllerFamily) {
        let mut stack = vec![(expression, 1usize)];
        while let Some((node, depth)) = stack.pop() {
            self.node_count += 1;
            if depth > MAX_EXPR_DEPTH || self.node_count > MAX_EXPR_NODES {
                self.structural = false;
                self.diagnostic(id, "ExpressionLimit", Level::Error,
                    "Размер/глубина дерева выражений превышает предел приложения");
                break;
            }
            let key = match node {
                GraphDraftExpr::Contact { id, .. } | GraphDraftExpr::And { id, .. } |
                GraphDraftExpr::Or { id, .. } => id,
            };
            if key.is_empty() || !self.used_node_ids.insert(key.clone()) {
                self.structural = false;
                self.diagnostic(id, "DuplicateNodeId", Level::Error,
                    "Идентификатор контакта/узла пуст или повторяется");
            }
            match node {
                GraphDraftExpr::Contact { relay, .. } => {
                    let addr = parse_relay(controller, relay);
                    if !relay.starts_with('#') || addr.is_err() {
                        self.diagnostic(id, "InvalidContact", Level::Error,
                            format!("Неверный контакт {relay} для {controller:?}"));
                    }
                }
                GraphDraftExpr::And {left, right, ..} |
                GraphDraftExpr::Or {left, right, ..} => {
                    stack.push((right, depth+1));
                    stack.push((left, depth+1));
                }
            }
        }
    }
    fn check_registers(&mut self, id: &str, current: &str, preset: &Preset) {
        match parse_register(current) {
            Ok(address) if address.number <= 559 => {
                if let Some(previous) = self.used_current_registers.insert(address.number, id.to_string()) {
                    self.diagnostic(id, "DuplicateCurrentRegister", Level::Error,
                        format!("{current} уже является текущим значением блока {previous}"));
                }
            }
            _ => self.diagnostic(id, "InvalidCurrentRegister", Level::Error,
                format!("{current}: текущее значение TMR/CNT допускается только M000..M559")),
        }
        match preset {
            Preset::Register {address} => {
                if parse_register(address).is_err() {
                    self.diagnostic(id, "InvalidPresetRegister", Level::Error,
                        format!("Неверный регистр уставки {address}; требуется M000..M999"));
                }
            }
            Preset::Literal {value} => {
                if *value == 0 {
                    self.diagnostic(id, "ZeroPreset", Level::Warning,
                        "Нулевая уставка допустима синтаксически, но функциональное поведение проверяйте на оборудовании");
                }
            }
        }
    }
    fn check_coil(&mut self, id: &str, coil: &str, controller: ControllerFamily) {
        match parse_relay(controller, coil) {
            Ok(relay) if coil.starts_with('#') => {
                if let Some(previous) = self.used_coils.insert(relay.number, id.to_string()) {
                    self.diagnostic(id, "DoubleCoil", Level::Error,
                        format!("{coil} уже используется выходом блока {previous}"));
                }
                match relay_write_policy(relay.class) {
                    LadderWritePolicy::Denied =>
                        self.diagnostic(id, "CoilDenied", Level::Error,
                            format!("Адрес {coil} имеет недопустимый класс назначения")),
                    LadderWritePolicy::RequiresReview =>
                        self.diagnostic(id, "CoilReview", Level::Warning,
                            format!("Адрес {coil}: требуется проверка конфигурации контроллера")),
                    LadderWritePolicy::Candidate => {}
                }
            }
            _ => self.diagnostic(id, "InvalidCoil", Level::Error,
                format!("Недопустимый OUT {coil} для {controller:?}")),
        }
    }
    fn check_group(&mut self, id: &str, source: &str, destination: &str, controller: ControllerFamily) {
        if parse_group(controller, source, RelayGroupWidth::Byte8).is_err() {
            self.diagnostic(id, "InvalidGroupSource", Level::Error,
                format!("GSTR {source}: требуется существующий байтовый адрес #XXXX0"));
        }
        match parse_group(controller, destination, RelayGroupWidth::Byte8) {
            Ok(dest) => {
                if let Some(previous) = self.used_group_writes.insert(dest.first_bit.number, id.into()) {
                    self.diagnostic(id, "DuplicateGroupDestination", Level::Error,
                        format!("GOUT {destination} уже назначен блоку {previous}"));
                }
                match relay_write_policy(dest.first_bit.class) {
                    LadderWritePolicy::Denied => self.diagnostic(id, "GroupDestinationDenied", Level::Error,
                        format!("GOUT {destination}: запись в этот класс сигналов запрещена")),
                    LadderWritePolicy::RequiresReview =>
                        self.diagnostic(id, "GroupDestinationReview", Level::Warning,
                            format!("GOUT {destination} требует проверки конфигурации")),
                    LadderWritePolicy::Candidate => {}
                }
            }
            Err(_) => self.diagnostic(id, "InvalidGroupDestination", Level::Error,
                format!("GOUT {destination}: требуется байтовый адрес #XXXX0")),
        }
    }
}

pub fn validate(draft: &AdvancedDraft) -> Validation {
    let mut c = Checker::new();
    if draft.version != ADVANCED_DRAFT_VERSION {
        c.structural = false;
        c.diagnostic("", "InvalidVersion", Level::Error,
            format!("Версия {} не поддерживается", draft.version));
    }
    if draft.blocks.len() > MAX_BLOCKS {
        c.structural = false;
        c.diagnostic("", "BlockLimit", Level::Error, "Превышен лимит 128 функциональных блоков");
    }
    // Yaskawa error 4220 describes >100 TMR/CNT/arithmetic instructions.
    // This editor does not see separate system-ladder arithmetic instructions,
    // so warn early; it is never permission to consume the remaining budget.
    let stateful_count = draft.blocks.iter().filter(|b| matches!(b,
        AdvancedBlock::Tmr{..} | AdvancedBlock::Cnt{..})).count();
    if stateful_count > 100 {
        c.diagnostic("", "YaskawaInstructionBudget", Level::Error,
            "Более 100 TMR/CNT: ошибка 4220; общий лимит включает также арифметические инструкции");
    } else if stateful_count >= 90 {
        c.diagnostic("", "YaskawaInstructionBudget", Level::Warning,
            "TMR/CNT близко к лимиту: общее число включает и арифметику других секций");
    }
    for block in &draft.blocks {
        let id = block.id();
        if id.is_empty() || !c.seen_blocks.insert(id.to_string()) {
            c.structural = false;
            c.diagnostic(id, "DuplicateBlockId", Level::Error,
                "Пустой или повторяющийся ID функционального блока");
        }
        match block {
            AdvancedBlock::Tmr { enable, current, preset, output, .. } => {
                c.check_expr(id, enable, draft.controller);
                c.check_registers(id, current, preset);
                c.check_coil(id, output, draft.controller);
            }
            AdvancedBlock::Cnt { count, reset, current, preset, output, .. } => {
                c.check_expr(id, count, draft.controller);
                c.check_expr(id, reset, draft.controller);
                c.check_registers(id, current, preset);
                c.check_coil(id, output, draft.controller);
            }
            AdvancedBlock::Group {source, destination, ..} =>
                c.check_group(id, source, destination, draft.controller),
        }
        if c.node_count > MAX_EXPR_NODES { break; }
    }
    for (coil, writer) in c.used_coils.clone() {
        if let Some(group) = c.used_group_writes.get(&(coil/10*10)) {
            c.diagnostic(&writer, "GroupCoilOverlap", Level::Error,
                format!("OUT #{coil:05} пересекается с GOUT блока {group}"));
        }
    }
    let has_errors = c.diagnostics.iter().any(|d| d.level == Level::Error);
    Validation {
        diagnostics: c.diagnostics, has_errors, structure_valid: c.structural,
        native_export_allowed: false, hardware_test_authorized: false,
    }
}

/// A JSON draft is *not* a valid native controller file. Structurally valid
/// drafts can be saved with operand diagnostics for later correction.
pub fn serialize_checked(d: &AdvancedDraft) -> Result<(String, Validation), String> {
    let v = validate(d);
    if !v.structure_valid { return Err("Недопустимая структура advanced-черновика".into()); }
    let json = serde_json::to_string_pretty(d).map_err(|e| e.to_string())?;
    if json.len() > 2*1024*1024 { return Err("Черновик слишком велик".into()); }
    Ok((json, v))
}
pub fn deserialize_checked(data: &str) -> Result<(AdvancedDraft, Validation), String> {
    if data.len() > 2*1024*1024 { return Err("Черновик слишком велик".into()); }
    let d: AdvancedDraft = serde_json::from_str(data).map_err(|e| e.to_string())?;
    let v = validate(&d);
    if !v.structure_valid { return Err("Недопустимая структура advanced-черновика".into()); }
    Ok((d, v))
}

/// A discrete clock tick in this test model represents exactly 100 ms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimerState {
    pub current: u16,
    pub contact: bool,
}
impl TimerState {
    pub fn new(preset: u16) -> Self { Self { current: preset, contact: false } }
    /// A single explicit 100ms tick; not a real-time wall-clock timer.
    pub fn tick(&mut self, start: bool, preset: u16) {
        if !start {
            self.current = preset;
            self.contact = false;
        } else {
            self.current = self.current.saturating_sub(1);
            self.contact = self.current == 0;
        }
    }
}

/// Reset takes priority, and one decrement occurs on each OFF->ON edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CounterState {
    pub current: u16,
    pub contact: bool,
    pub previous_count: bool,
}
impl CounterState {
    pub fn new(preset: u16) -> Self {
        Self { current: preset, contact: false, previous_count: false }
    }
    pub fn scan(&mut self, count: bool, reset: bool, preset: u16) {
        if reset {
            self.current = preset;
            self.contact = false;
        } else if count && !self.previous_count {
            self.current = self.current.saturating_sub(1);
            self.contact = self.current == 0;
        }
        self.previous_count = count;
    }
}

/// Byte copy only, not an HSES/Modbus write or permission to change I/O.
pub fn group_copy(source: u8) -> u8 { source }

/// Evaluate GraphDraftExpr using an explicit offline relay snapshot. Missing
/// inputs propagate an UNKNOWN value with three-valued short-circuit logic.
pub fn evaluate_condition(expr: &GraphDraftExpr, bits: &HashMap<u32, bool>,
    controller: ControllerFamily) -> Option<bool> {
    let mut stack = vec![(expr, false)];
    let mut values: Vec<Option<bool>> = vec![];
    let mut counter = 0;
    while let Some((node, visited)) = stack.pop() {
        counter += 1;
        if counter > MAX_EXPR_NODES*3 { return None; }
        if !visited {
            stack.push((node, true));
            match node {
                GraphDraftExpr::Contact { .. } => {}
                GraphDraftExpr::And {left,right,..} |
                GraphDraftExpr::Or {left,right,..} => {
                    stack.push((right, false));
                    stack.push((left, false));
                }
            }
        } else {
            match node {
                GraphDraftExpr::Contact {relay,normally_closed,..} => {
                    let v = parse_relay(controller,relay).ok()
                        .and_then(|a| bits.get(&a.number).copied())
                        .map(|b| if *normally_closed {!b} else {b});
                    values.push(v);
                }
                GraphDraftExpr::And { .. } | GraphDraftExpr::Or { .. } => {
                    let right = values.pop()?;
                    let left = values.pop()?;
                    let result = if matches!(node,GraphDraftExpr::And{..}) {
                        match (left,right) {
                            (Some(false),_)|(_,Some(false))=>Some(false),
                            (Some(true),Some(true))=>Some(true),_=>None
                        }
                    } else {
                        match (left,right) {
                            (Some(true),_)|(_,Some(true))=>Some(true),
                            (Some(false),Some(false))=>Some(false),_=>None
                        }
                    };
                    values.push(result);
                }
            }
        }
    }
    if values.len() == 1 {values.pop().flatten()} else {None}
}

#[cfg(test)]
mod tests;
