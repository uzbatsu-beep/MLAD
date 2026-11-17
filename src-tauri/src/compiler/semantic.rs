//! Offline semantic analysis of the documented CIO boolean instruction subset.
//!
//! This module consumes the immutable PR #2 lossless AST overlay. It never
//! modifies the source, guesses unsupported instruction effects, or authorizes
//! System/User ladder edits. It uses an indexed expression DAG rather than
//! recursive trees, so deeply nested input cannot overflow the Rust stack.
//!
//! Sources: DX200 RE-CKI-A465 §12.2.1-12.2.11; YRC1000 RE-CKI-A467
//! §12.2; https://knowledge.motoman.com/hc/en-us/articles/1500012236161-

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use serde::Serialize;
use super::lossless::{AstInstruction, ByteSpan, LineKind, LosslessDocument, OperandKind};
use super::memory::{parse_relay, relay_write_policy, LadderWritePolicy};

/// Explicit cap is an analyser resource guard, NOT a controller limit.
pub const MAX_BOOLEAN_NODES: usize = 100_000;
pub const MAX_OPEN_BRANCHES: usize = 512;
pub const MAX_SEMANTIC_DIAGNOSTICS: usize = 10_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SectionClaim { Unspecified, System, User }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DiagnosticLevel { Warning, Error }

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SemanticDiagnostic {
    pub code: &'static str,
    pub level: DiagnosticLevel,
    pub line: usize,
    pub byte_offset: usize,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ExprOp {
    Contact { relay: u32, inverted: bool },
    And { left: usize, right: usize },
    Or { left: usize, right: usize },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExprNode {
    pub op: ExprOp,
    pub line: usize,
    pub span: ByteSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SemanticRung {
    pub number: usize,
    pub first_line: usize,
    pub output_line: usize,
    pub output: u32,
    pub expression_root: usize,
    pub inputs: Vec<u32>,
    /// Bounded presentation only. The source of truth is the indexed graph.
    pub expression_preview: String,
    /// This is a document claim based on PART, NOT verified edit ownership.
    pub section: SectionClaim,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SymbolOperation {
    ContactRead,
    CoilWrite,
    PotentialGroupRead,
    PotentialGroupWrite,
    RegisterCurrentValue,
    RegisterSetpoint,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReferenceSite {
    pub line: usize,
    pub operand_span: ByteSpan,
    pub operation: SymbolOperation,
    pub section: SectionClaim,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SymbolReferences {
    pub symbol: String,
    pub sites: Vec<ReferenceSite>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PartMarker {
    pub line: usize,
    pub section: SectionClaim,
}

#[derive(Debug, Clone, Serialize)]
pub struct SemanticReport {
    pub source_sha256: String,
    pub nodes: Vec<ExprNode>,
    pub rungs: Vec<SemanticRung>,
    pub references: Vec<SymbolReferences>,
    pub part_markers: Vec<PartMarker>,
    pub diagnostics: Vec<SemanticDiagnostic>,
    pub unsupported_segments: usize,
    pub end_seen: bool,
    /// TRUE only for the implemented boolean subset, not entire CIO program.
    pub subset_analysis_complete: bool,
    pub system_user_boundaries_verified: bool,
    pub edited_export_allowed: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct SemanticPreview {
    /// Bounded AST node subset for read-only graph visualization.
    pub nodes: Vec<ExprNode>,
    pub nodes_truncated: bool,
    pub node_count: usize,
    pub rung_count: usize,
    pub reference_count: usize,
    pub diagnostic_count: usize,
    pub unsupported_segments: usize,
    pub subset_analysis_complete: bool,
    pub system_user_boundaries_verified: bool,
    pub edited_export_allowed: bool,
    pub end_seen: bool,
    pub rungs: Vec<SemanticRung>,
    pub references: Vec<SymbolReferences>,
    pub part_markers: Vec<PartMarker>,
    pub diagnostics: Vec<SemanticDiagnostic>,
    pub rungs_truncated: bool,
    pub references_truncated: bool,
    pub diagnostics_truncated: bool,
}

impl SemanticReport {
    pub fn preview(&self, max_rungs: usize, max_references: usize, max_diagnostics: usize) -> SemanticPreview {
        SemanticPreview {
            nodes: self.nodes.iter().take(4_096).cloned().collect(),
            nodes_truncated: self.nodes.len() > 4_096,
            node_count: self.nodes.len(),
            rung_count: self.rungs.len(),
            reference_count: self.references.len(),
            diagnostic_count: self.diagnostics.len(),
            unsupported_segments: self.unsupported_segments,
            subset_analysis_complete: self.subset_analysis_complete,
            system_user_boundaries_verified: false,
            edited_export_allowed: false,
            end_seen: self.end_seen,
            rungs: self.rungs.iter().take(max_rungs).cloned().collect(),
            references: self.references.iter().take(max_references).cloned().collect(),
            part_markers: self.part_markers.clone(),
            diagnostics: self.diagnostics.iter().take(max_diagnostics).cloned().collect(),
            rungs_truncated: self.rungs.len() > max_rungs,
            references_truncated: self.references.len() > max_references,
            diagnostics_truncated: self.diagnostics.len() > max_diagnostics,
        }
    }

    /// Three-valued, offline and side-effect-free expression evaluation.
    /// None means the condition is indeterminate because an input is missing.
    /// It never writes to hardware or accounts for controller scan timing.
    pub fn evaluate_rung(&self, rung: usize, signals: &HashMap<u32, bool>) -> Option<bool> {
        let root = self.rungs.get(rung)?.expression_root;
        let mut results = Vec::with_capacity(root + 1);
        for node in &self.nodes[..=root] {
            let value = match &node.op {
                ExprOp::Contact { relay, inverted } => signals.get(relay).map(|x| *x ^ *inverted),
                ExprOp::And { left, right } => {
                    let a: Option<bool> = results.get(*left).copied().flatten();
                    let b: Option<bool> = results.get(*right).copied().flatten();
                    match (a, b) {
                        (Some(false), _) | (_, Some(false)) => Some(false),
                        (Some(true), Some(true)) => Some(true),
                        _ => None,
                    }
                }
                ExprOp::Or { left, right } => {
                    let a: Option<bool> = results.get(*left).copied().flatten();
                    let b: Option<bool> = results.get(*right).copied().flatten();
                    match (a, b) {
                        (Some(true), _) | (_, Some(true)) => Some(true),
                        (Some(false), Some(false)) => Some(false),
                        _ => None,
                    }
                }
            };
            results.push(value);
        }
        results.get(root).copied().flatten()
    }
}

fn relay_name(number: u32) -> String { format!("#{:05}", number) }

struct Analyzer {
    nodes: Vec<ExprNode>,
    rungs: Vec<SemanticRung>,
    references: BTreeMap<String, Vec<ReferenceSite>>,
    part_markers: Vec<PartMarker>,
    diagnostics: Vec<SemanticDiagnostic>,
    current: Option<usize>,
    branch_stack: Vec<usize>,
    first_line: Option<usize>,
    section: SectionClaim,
    seen_part: bool,
    end_seen: bool,
    unsupported_segments: usize,
    tainted_until_out: bool,
    coil_sites: HashMap<u32, (usize, SectionClaim)>,
    group_writes: Vec<(u32, usize)>,
}

impl Analyzer {
    fn new() -> Self {
        Self {
            nodes: Vec::new(), rungs: Vec::new(),
            references: BTreeMap::new(), part_markers: Vec::new(),
            diagnostics: Vec::new(), current: None,
            branch_stack: Vec::new(), first_line: None,
            section: SectionClaim::Unspecified, seen_part: false,
            end_seen: false, unsupported_segments: 0, tainted_until_out: false,
            coil_sites: HashMap::new(), group_writes: Vec::new(),
        }
    }

    fn diag(&mut self, code: &'static str, level: DiagnosticLevel,
            line: usize, byte_offset: usize, msg: impl Into<String>) {
        if self.diagnostics.len() < MAX_SEMANTIC_DIAGNOSTICS {
            self.diagnostics.push(SemanticDiagnostic {
                code, level, line, byte_offset, message: msg.into(),
            });
        }
    }

    fn reference(&mut self, symbol: String, site: &AstInstruction,
            operand: usize, operation: SymbolOperation) {
        if let Some(op) = site.operands.get(operand) {
            self.references.entry(symbol).or_default().push(ReferenceSite {
                line: site.line,
                operand_span: op.span,
                operation,
                section: self.section,
            });
        }
    }

    fn reset(&mut self) {
        self.current = None;
        self.branch_stack.clear();
        self.first_line = None;
    }

    fn barrier(&mut self, code: &'static str, line: usize, offset: usize, message: &str) {
        if self.current.is_some() || !self.branch_stack.is_empty() {
            self.tainted_until_out = true;
            self.diag("AbandonedExpression", DiagnosticLevel::Warning, line, offset,
                "Незавершённый логический блок отброшен из семантического анализа (исходные байты сохранены)");
        }
        self.diag(code, DiagnosticLevel::Warning, line, offset, message);
        self.unsupported_segments += 1;
        self.reset();
    }

    fn node(&mut self, op: ExprOp, inst: &AstInstruction) -> Option<usize> {
        if self.nodes.len() >= MAX_BOOLEAN_NODES {
            self.barrier("NodeLimit", inst.line, inst.span.start,
                "Превышен лимит узлов анализатора; это не предел Yaskawa");
            return None;
        }
        let index = self.nodes.len();
        self.nodes.push(ExprNode { op, line: inst.line, span: inst.span });
        Some(index)
    }

    fn fail_stack(&mut self, code: &'static str, inst: &AstInstruction, message: &str) {
        self.diag(code, DiagnosticLevel::Error, inst.line, inst.span.start, message);
        self.unsupported_segments += 1;
        self.tainted_until_out = false;
        self.reset();
    }

    fn collect_inputs(&self, root: usize) -> Vec<u32> {
        let mut result = BTreeSet::new();
        let mut stack = vec![root];
        while let Some(i) = stack.pop() {
            match &self.nodes[i].op {
                ExprOp::Contact { relay, .. } => { result.insert(*relay); },
                ExprOp::And { left, right } | ExprOp::Or { left, right } => {
                    stack.push(*left);
                    stack.push(*right);
                },
            }
        }
        result.into_iter().collect()
    }

    fn expression_preview(&self, root: usize) -> String {
        enum Frame { Node(usize), Text(&'static str) }
        let mut stack = vec![Frame::Node(root)];
        let mut s = String::new();
        while let Some(frame) = stack.pop() {
            if s.len() >= 512 {
                s.push_str("…");
                break;
            }
            match frame {
                Frame::Text(t) => s.push_str(t),
                Frame::Node(i) => match &self.nodes[i].op {
                    ExprOp::Contact { relay, inverted } => {
                        if *inverted { s.push('!'); }
                        s.push_str(&relay_name(*relay));
                    }
                    ExprOp::And { left, right } | ExprOp::Or { left, right } => {
                        stack.push(Frame::Text(")"));
                        stack.push(Frame::Node(*right));
                        stack.push(Frame::Text(if matches!(&self.nodes[i].op, ExprOp::And { .. }) {
                            " AND "
                        } else { " OR " }));
                        stack.push(Frame::Node(*left));
                        stack.push(Frame::Text("("));
                    }
                }
            }
        }
        s
    }

    fn process(&mut self, inst: &AstInstruction, controller: super::memory::ControllerFamily) {
        if self.end_seen {
            self.diag("AfterEnd", DiagnosticLevel::Error, inst.line, inst.span.start,
                "Инструкция после END; семантический анализ не выполняется");
            self.unsupported_segments += 1;
            return;
        }
        // After an unsupported instruction or analyzer resource overrun in an
        // open expression, an additional STR is NOT proof of a new rung.
        // Ignore that damaged region through OUT or a documented PART/END.
        if self.tainted_until_out {
            match inst.mnemonic.as_str() {
                "OUT" => {
                    self.tainted_until_out = false;
                    if inst.syntax_valid {
                        if let Ok(relay) = parse_relay(controller, &inst.operands[0].spelling) {
                            self.reference(relay_name(relay.number), inst, 0, SymbolOperation::CoilWrite);
                            if let Some((first, _)) = self.coil_sites.insert(relay.number, (inst.line, self.section)) {
                                self.diag("DoubleCoil", DiagnosticLevel::Error, inst.line, inst.span.start,
                                    format!("OUT {} повторяет запись из строки {}", relay_name(relay.number), first));
                            }
                        }
                    }
                    self.fail_stack("UnbalancedOutput", inst,
                        "OUT завершает повреждённый или неподдерживаемый логический участок");
                    return;
                }
                "PART" | "END" => { self.tainted_until_out = false; }
                _ => { return; }
            }
        }
        if !inst.syntax_valid {
            self.barrier("InvalidSyntaxBarrier", inst.line, inst.span.start,
                "Инструкция с синтаксическими ошибками не включена в логический граф");
            return;
        }
        match inst.mnemonic.as_str() {
            "PART" => {
                if self.current.is_some() || !self.branch_stack.is_empty() {
                    self.fail_stack("PartInsideExpression", inst,
                        "PART встретился внутри незавершённого логического выражения");
                }
                let next = match inst.operands[0].spelling.as_str() {
                    "1" => SectionClaim::System, "2" => SectionClaim::User,
                    _ => SectionClaim::Unspecified,
                };
                if self.seen_part && next == SectionClaim::System {
                    self.diag("PartOrder", DiagnosticLevel::Warning, inst.line, inst.span.start,
                        "Повторное PART 1: порядок разделов требует проверки");
                }
                self.seen_part = true;
                self.section = next;
                self.part_markers.push(PartMarker { line: inst.line, section: next });
            }
            "END" => {
                if self.current.is_some() || !self.branch_stack.is_empty() {
                    self.fail_stack("UnfinishedAtEnd", inst,
                        "END встретился до завершения цепи оператором OUT");
                }
                self.end_seen = true;
            }
            "STR" | "STR-NOT" => {
                let relay = parse_relay(controller, &inst.operands[0].spelling)
                    .expect("lossless parser validated relay operand");
                self.reference(relay_name(relay.number), inst, 0, SymbolOperation::ContactRead);
                if let Some(old) = self.current {
                    if self.branch_stack.len() >= MAX_OPEN_BRANCHES {
                        self.barrier("BranchDepthLimit", inst.line, inst.span.start,
                            "Превышен лимит вложенности анализатора (не предел контроллера)");
                        return;
                    }
                    self.branch_stack.push(old);
                } else {
                    self.first_line = Some(inst.line);
                }
                self.current = self.node(ExprOp::Contact {
                    relay: relay.number,
                    inverted: inst.mnemonic == "STR-NOT",
                }, inst);
            }
            "AND" | "AND-NOT" | "OR" | "OR-NOT" => {
                let relay = parse_relay(controller, &inst.operands[0].spelling)
                    .expect("lossless parser validated relay operand");
                self.reference(relay_name(relay.number), inst, 0, SymbolOperation::ContactRead);
                let Some(left) = self.current else {
                    self.fail_stack("MissingStart", inst,
                        "AND/OR без STR либо после незавершённой инструкции");
                    return;
                };
                let Some(right) = self.node(ExprOp::Contact {
                    relay: relay.number,
                    inverted: inst.mnemonic.ends_with("-NOT"),
                }, inst) else { return; };
                let combined = if inst.mnemonic.starts_with("AND") {
                    ExprOp::And { left, right }
                } else {
                    ExprOp::Or { left, right }
                };
                self.current = self.node(combined, inst);
            }
            "AND-STR" | "OR-STR" => {
                let (Some(left), Some(right)) = (self.branch_stack.pop(), self.current) else {
                    self.fail_stack("UnbalancedBranch", inst,
                        "AND-STR/OR-STR без двух промежуточных выражений");
                    return;
                };
                let op = if inst.mnemonic == "AND-STR" {
                    ExprOp::And { left, right }
                } else {
                    ExprOp::Or { left, right }
                };
                self.current = self.node(op, inst);
            }
            "OUT" => {
                let relay = parse_relay(controller, &inst.operands[0].spelling)
                    .expect("lossless parser validated relay operand");
                self.reference(relay_name(relay.number), inst, 0, SymbolOperation::CoilWrite);
                if let Some((first, first_part)) = self.coil_sites.insert(relay.number, (inst.line, self.section)) {
                    self.diag("DoubleCoil", DiagnosticLevel::Error, inst.line, inst.span.start,
                        format!("OUT {} уже задан в строке {} ({:?}); дублирование может затрагивать обе части Ladder",
                            relay_name(relay.number), first, first_part));
                }
                match relay_write_policy(relay.class) {
                    LadderWritePolicy::Denied => self.diag("ReadOnlyCoil", DiagnosticLevel::Error,
                        inst.line, inst.span.start,
                        format!("OUT {} в неподходящем классе {:?}", relay_name(relay.number), relay.class)),
                    LadderWritePolicy::RequiresReview => self.diag("WriteNeedsReview", DiagnosticLevel::Warning,
                        inst.line, inst.span.start,
                        format!("OUT {} требует проверки конфигурации контроллера", relay_name(relay.number))),
                    LadderWritePolicy::Candidate => {}
                }
                let (Some(root), true) = (self.current, self.branch_stack.is_empty()) else {
                    self.fail_stack("UnbalancedOutput", inst,
                        "OUT без завершённого выражения; имеются непарные STR или отсутствует STR");
                    return;
                };
                let inputs = self.collect_inputs(root);
                let expression_preview = self.expression_preview(root);
                if inputs.contains(&relay.number) {
                    self.diag("SelfFeedback", DiagnosticLevel::Warning, inst.line, inst.span.start,
                        format!("OUT {} зависит от собственного состояния (может быть намеренной памятью)",
                            relay_name(relay.number)));
                }
                self.rungs.push(SemanticRung {
                    number: self.rungs.len() + 1,
                    first_line: self.first_line.unwrap_or(inst.line),
                    output_line: inst.line,
                    output: relay.number,
                    expression_root: root,
                    inputs,
                    expression_preview,
                    section: self.section,
                });
                self.reset();
            }
            "TMR" | "CNT" => {
                // Recognized lexical shape only. Timer/counter side effects and
                // scan sequencing are outside the verified boolean subset.
                if inst.operands[0].kind == OperandKind::MRegister {
                    self.reference(inst.operands[0].spelling.clone(), inst, 0,
                        SymbolOperation::RegisterCurrentValue);
                }
                if inst.operands[1].kind == OperandKind::MRegister {
                    self.reference(inst.operands[1].spelling.clone(), inst, 1,
                        SymbolOperation::RegisterSetpoint);
                }
                self.barrier("StatefulOpcode", inst.line, inst.span.start,
                    "TMR/CNT меняют состояние; условие следующего OUT нельзя выводить без семантики функции");
            }
            "GSTR" | "GOUT" => {
                let operand = &inst.operands[0];
                if operand.kind == OperandKind::RelayByte {
                    let relay = parse_relay(controller, &operand.spelling)
                        .expect("group lexical validator validated relay number");
                    let operation = if inst.mnemonic == "GOUT" {
                        self.group_writes.push((relay.number, inst.line));
                        SymbolOperation::PotentialGroupWrite
                    } else { SymbolOperation::PotentialGroupRead };
                    self.reference(relay_name(relay.number), inst, 0, operation);
                }
                self.barrier("GroupedOpcode", inst.line, inst.span.start,
                    "Семантика GSTR/GOUT и влияние на соседние биты не подтверждены для этого анализа");
            }
            _ => self.barrier("UnknownOpcode", inst.line, inst.span.start,
                "Неподдерживаемая операция; анализ не продолжает выражение"),
        }
    }

    fn finish(mut self, doc: &LosslessDocument) -> SemanticReport {
        if self.current.is_some() || !self.branch_stack.is_empty() {
            let line = doc.lines.last().map(|l| l.number).unwrap_or(0);
            self.diag("UnfinishedExpression", DiagnosticLevel::Error, line,
                doc.original_bytes().len(), "В конце файла осталось незавершённое логическое выражение");
            self.unsupported_segments += 1;
        }
        if !self.end_seen {
            self.diag("NoEnd", DiagnosticLevel::Warning, doc.lines.last().map(|l| l.number).unwrap_or(0),
                doc.original_bytes().len(), "Не обнаружена подтверждённая инструкция END");
        }
        // GOUT and BMOV may own bytes that overlap an OUT. We can warn only
        // for recognized GOUT; unknown BMOV is a completeness barrier.
        for (start, line) in self.group_writes.clone() {
            let owner = start / 10;
            let collisions: Vec<_> = self.coil_sites.iter()
                .filter(|(bit, _)| **bit / 10 == owner)
                .map(|(bit, (ln, _))| (*bit, *ln)).collect();
            for (bit, ln) in collisions {
                self.diag("PossibleGroupOverlap", DiagnosticLevel::Warning, line, 0,
                    format!("GOUT {} может конфликтовать с OUT {} в строке {}; проверить данные контроллера",
                        relay_name(start), relay_name(bit), ln));
            }
        }
        detect_feedback_cycles(&self.rungs, &mut self.diagnostics);
        let complete = self.end_seen &&
            self.unsupported_segments == 0 &&
            doc.diagnostics.is_empty() &&
            !self.diagnostics.iter().any(|d| d.level == DiagnosticLevel::Error);
        SemanticReport {
            source_sha256: doc.source_sha256.clone(),
            nodes: self.nodes,
            rungs: self.rungs,
            references: self.references.into_iter().map(|(symbol, sites)| SymbolReferences {
                symbol, sites
            }).collect(),
            part_markers: self.part_markers,
            diagnostics: self.diagnostics,
            unsupported_segments: self.unsupported_segments,
            end_seen: self.end_seen,
            subset_analysis_complete: complete,
            system_user_boundaries_verified: false,
            edited_export_allowed: false,
        }
    }
}

/// Find strongly-connected components of the dependency graph between
/// independently constructed rungs, using iterative DFS (no recursion).
/// Feedback is only a structural flag; intentional latches are possible.
fn detect_feedback_cycles(rungs: &[SemanticRung], diagnostics: &mut Vec<SemanticDiagnostic>) {
    let mut producers: HashMap<u32, Vec<usize>> = HashMap::new();
    for (i, r) in rungs.iter().enumerate() { producers.entry(r.output).or_default().push(i); }
    let n = rungs.len();
    let mut edges = vec![Vec::<usize>::new(); n];
    let mut reverse = vec![Vec::<usize>::new(); n];
    for (i, r) in rungs.iter().enumerate() {
        let mut seen = HashSet::new();
        for input in &r.inputs {
            if let Some(xs) = producers.get(input) {
                for &j in xs {
                    if i != j && seen.insert(j) {
                        edges[i].push(j);
                        reverse[j].push(i);
                    }
                }
            }
        }
    }
    let mut visited = vec![false; n];
    let mut order = Vec::with_capacity(n);
    for i in 0..n {
        if visited[i] { continue; }
        visited[i] = true;
        let mut pending = vec![(i, 0usize)];
        while let Some((u, k)) = pending.last_mut() {
            if *k < edges[*u].len() {
                let v = edges[*u][*k];
                *k += 1;
                if !visited[v] {
                    visited[v] = true;
                    pending.push((v, 0));
                }
            } else {
                order.push(*u);
                pending.pop();
            }
        }
    }
    visited.fill(false);
    for &i in order.iter().rev() {
        if visited[i] { continue; }
        let mut group = Vec::new();
        let mut pending = vec![i];
        visited[i] = true;
        while let Some(u) = pending.pop() {
            group.push(u);
            for &v in &reverse[u] {
                if !visited[v] {
                    visited[v] = true;
                    pending.push(v);
                }
            }
        }
        if group.len() > 1 && diagnostics.len() < MAX_SEMANTIC_DIAGNOSTICS {
            let symbols: Vec<_> = group.iter().map(|i| relay_name(rungs[*i].output)).collect();
            diagnostics.push(SemanticDiagnostic {
                code: "FeedbackCycle",
                level: DiagnosticLevel::Warning,
                line: rungs[i].output_line,
                byte_offset: 0,
                message: format!("Циклическая зависимость цепей (не обязательно ошибка): {}", symbols.join(" ↔ ")),
            });
        }
    }
}

pub fn analyze(doc: &LosslessDocument) -> SemanticReport {
    let mut a = Analyzer::new();
    let mut unverified_tail_reported = false;
    for line in &doc.lines {
        match &line.kind {
            LineKind::Instruction(inst) => a.process(inst, doc.controller),
            LineKind::TypedCandidate { .. } => {
                a.barrier("TypedOpcodeSemanticsPending", line.number, line.body_span.start,
                    "Типизированная форма распознана, но исполнение и флаги не подтверждены; цепь не реконструируется");
            }
            LineKind::UnsupportedInstruction { .. } | LineKind::Opaque => {
                a.barrier("UnverifiedSource", line.number, line.body_span.start,
                    "Неподдерживаемый исходный текст: логические зависимости могут быть неполными");
            }
            LineKind::Directive | LineKind::MHeaderRow { .. } => {
                // Header directives are preserved, not interpreted. If any
                // metadata appears inside an expression, do not assume harmlessness.
                if a.current.is_some() || !a.branch_stack.is_empty() {
                    a.barrier("DirectiveInExpression", line.number, line.body_span.start,
                        "Директива/заголовок внутри логического выражения");
                }
            }
            // Observed post-END ALARM/MESSAGE payload is not executable Ladder.
            LineKind::PostEndSection { .. } | LineKind::PostEndData => {}
            LineKind::PostEndOpaque => {
                // This is unverified material. Do not fabricate an instruction
                // or one warning per payload line; retain an incomplete status.
                if !unverified_tail_reported {
                    a.diag("AfterEnd", DiagnosticLevel::Warning, line.number,
                        line.body_span.start,
                        "После END обнаружена неподтверждённая структура, это не Ladder-инструкция");
                    a.unsupported_segments += 1;
                    unverified_tail_reported = true;
                }
            }
            LineKind::Empty | LineKind::Comment => {}
        }
    }
    a.finish(doc)
}

#[cfg(test)]
mod tests;
