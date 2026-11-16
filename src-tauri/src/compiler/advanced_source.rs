//! Read-only recognition of selected advanced instruction patterns from an
//! immutable CIOPRG.LST snapshot. No source transformation or native export.
//! Exact single-contact examples only; complex vendor ladder graphs are not
//! silently approximated.

use std::collections::{HashMap, HashSet};
use serde::Serialize;
use super::lossless::{LineKind, LosslessDocument, AstInstruction};
use super::memory::{parse_relay, parse_group, RelayGroupWidth};
use super::advanced::{MAX_BLOCKS};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SourcePattern { Timer, Counter, GroupCopy }

#[derive(Debug, Clone, Serialize)]
pub struct SourceBlock {
    pub kind: SourcePattern,
    pub line: usize,
    pub end_line: usize,
    pub operands: Vec<String>,
    pub count_or_enable: Option<String>,
    pub reset: Option<String>,
    pub count_or_enable_inverted: bool,
    pub reset_inverted: bool,
    pub output: Option<String>,
    /// Recognized as a narrow manual example, not an editable native format.
    pub example_pattern_recognized: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct SourceWarning {
    pub code: &'static str,
    pub line: usize,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AdvancedSourceReport {
    pub source_sha256: String,
    pub blocks: Vec<SourceBlock>,
    pub warnings: Vec<SourceWarning>,
    pub blocks_truncated: bool,
    pub original_only: bool,
    pub native_export_allowed: bool,
    pub hardware_test_authorized: bool,
}

fn is_simple_str(i: &AstInstruction) -> bool {
    i.syntax_valid &&
    matches!(i.mnemonic.as_str(),"STR"|"STR-NOT") &&
    i.operands.len() == 1
}

fn typed<'a>(lines: &'a [Option<&'a AstInstruction>], i: usize) -> Option<&'a AstInstruction> {
    lines.get(i).and_then(|v| *v)
}
fn op(inst: &AstInstruction) -> Vec<String> {
    inst.operands.iter().map(|x|x.spelling.clone()).collect()
}

pub fn inspect(doc: &LosslessDocument) -> AdvancedSourceReport {
    let mut blocks = vec![];
    let mut warnings = vec![];
    let mut used_current: HashMap<String,usize> = HashMap::new();
    let mut typed_insts: Vec<Option<&AstInstruction>> = vec![];
    for line in &doc.lines {
        match &line.kind {
            LineKind::Instruction(inst) => typed_insts.push(Some(inst)),
            LineKind::Empty | LineKind::Comment => {},
            LineKind::Directive | LineKind::MHeaderRow {..} =>
                typed_insts.push(None),
            LineKind::UnsupportedInstruction {..} | LineKind::Opaque =>
                typed_insts.push(None),
        }
    }
    let mut i=0;
    let mut recognized_group_out = HashSet::new();
    let mut group_destinations=HashMap::<u32,usize>::new();
    let mut known_coils=HashMap::<u32,usize>::new();

    while i<typed_insts.len() {
        let Some(inst)=typed(&typed_insts,i) else {i+=1;continue};
        if inst.mnemonic=="OUT" && inst.syntax_valid {
            if let Some(relay)=inst.operands.first().and_then(|x|parse_relay(doc.controller,&x.spelling).ok()){
                known_coils.insert(relay.number,inst.line);
            }
        }
        match inst.mnemonic.as_str() {
            "TMR" | "CNT" => {
                let timer=inst.mnemonic=="TMR";
                let start=if timer { i.checked_sub(1) } else { i.checked_sub(2) };
                let first=start.and_then(|s|typed(&typed_insts,s));
                let second=if timer {None} else {i.checked_sub(1).and_then(|s|typed(&typed_insts,s))};
                let next=typed(&typed_insts,i+1);
                let valid=inst.syntax_valid &&
                    first.is_some_and(is_simple_str) &&
                    (timer || second.is_some_and(is_simple_str)) &&
                    next.is_some_and(|n|n.mnemonic=="OUT" && n.syntax_valid);
                if !valid {
                    warnings.push(SourceWarning {
                        code:"UnverifiedAdvancedTopology",line:inst.line,
                        message:format!("{}: распознан opcode, но последовательность не соответствует простому примеру из руководства; сложная логика не реконструирована",
                            inst.mnemonic),
                    });
                }
                if let Some(register)=inst.operands.first().map(|o|o.spelling.clone()) {
                    if let Some(prev)=used_current.insert(register.clone(),inst.line) {
                        warnings.push(SourceWarning {code:"DuplicateCurrentRegister",
                            line:inst.line,message:format!("{} уже использован в строке {}",register,prev)});
                    }
                }
                blocks.push(SourceBlock {
                    kind:if timer{SourcePattern::Timer}else{SourcePattern::Counter},
                    line:inst.line,end_line:next.map(|n|n.line).unwrap_or(inst.line),
                    operands:op(inst),
                    count_or_enable: first.and_then(|n|n.operands.first().map(|o|o.spelling.clone())),
                    reset:second.and_then(|n|n.operands.first().map(|o|o.spelling.clone())),
                    count_or_enable_inverted:first.is_some_and(|n|n.mnemonic=="STR-NOT"),
                    reset_inverted:second.is_some_and(|n|n.mnemonic=="STR-NOT"),
                    output:next.and_then(|n|if n.mnemonic=="OUT"{
                        n.operands.first().map(|o|o.spelling.clone())
                    }else{None}),
                    example_pattern_recognized:valid,
                });
            }
            "GSTR" => {
                let next=typed(&typed_insts,i+1);
                let valid=inst.syntax_valid &&
                    next.is_some_and(|x|x.mnemonic=="GOUT" && x.syntax_valid);
                if !valid {
                    warnings.push(SourceWarning {code:"UnpairedGSTR",line:inst.line,
                        message:"GSTR не имеет немедленно следующего корректного GOUT".into()});
                }
                if let Some(out)=next.filter(|x|x.mnemonic=="GOUT" && x.syntax_valid) {
                    recognized_group_out.insert(out.line);
                    if let Some(relay)=out.operands.first().and_then(|o|
                        parse_group(doc.controller,&o.spelling,RelayGroupWidth::Byte8).ok()) {
                        if let Some(prev)=group_destinations.insert(relay.first_bit.number,out.line) {
                            warnings.push(SourceWarning{code:"RepeatedGOUT",line:out.line,
                                message:format!("Повторное GOUT в байт, ранее использованный в строке {}",prev)});
                        }
                    }
                }
                let mut operands=op(inst);
                if valid {
                    if let Some(other)=next {operands.extend(op(other));}
                }
                blocks.push(SourceBlock {
                    kind:SourcePattern::GroupCopy,line:inst.line,
                    end_line:if valid {next.map(|o|o.line).unwrap_or(inst.line)}else{inst.line},
                    operands,count_or_enable:None,reset:None,
                    count_or_enable_inverted:false,reset_inverted:false,output:None,
                    example_pattern_recognized:valid,
                });
            }
            "GOUT" => {
                if !recognized_group_out.contains(&inst.line) {
                    warnings.push(SourceWarning {code:"UnpairedGOUT",line:inst.line,
                        message:"GOUT не входит в подтверждённую пару с GSTR".into()});
                }
            }
            _ => {}
        }
        if blocks.len()>MAX_BLOCKS {
            warnings.push(SourceWarning{code:"PreviewLimit",line:inst.line,
                message:"Превышен лимит числа блоков предпросмотра".into()});
            blocks.truncate(MAX_BLOCKS);
            break;
        }
        i+=1;
    }
    // Check the full source as well; some OUT lines may follow a GSTR/GOUT pair.
    for (first, group_line) in group_destinations {
        for bit in 0..8 {
            if let Some(line)=known_coils.get(&(first+bit)){
                warnings.push(SourceWarning {code:"GroupOutputOverlap",line:group_line,
                    message:format!("GOUT #{first:05} пересекается с OUT в строке {line}")});
            }
        }
    }
    AdvancedSourceReport{
        source_sha256:doc.source_sha256.clone(),
        blocks_truncated:blocks.len()>=MAX_BLOCKS,
        blocks,warnings,original_only:true,native_export_allowed:false,
        hardware_test_authorized:false,
    }
}

#[cfg(test)]
mod tests;
