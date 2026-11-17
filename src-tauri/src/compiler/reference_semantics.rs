//! Isolated manual-derived *reference vectors* for five CIO operations.
//!
//! No Tauri IPC, controller interface, real CIOPRG evaluator or scan engine.
//! The only authority used is the stated YRC1000 Concurrent I/O manual
//! R12 §12.2.17/18/24/29/30 and §12.3 (cross-checked with DX100 RE-CKI-A453).
//! This module deliberately returns Unknown for cases not unambiguously
//! specified (e.g. MOV 256 to byte, carry at shift counts 0/16).
//! It NEVER upgrades dataflow/semantic flags or authorizes native export.
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ReferenceOpcode { Mov, Add, Sub, Shl, Shr }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ReferenceDestination { Register16, WordRelay16, ByteRelay8 }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ReferenceCondition { On, Off, Unknown }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ReferenceInput {
    pub opcode: ReferenceOpcode,
    pub condition: ReferenceCondition,
    pub source1: u16,
    /// ADD/SUB second unsigned operand; SHL/SHR literal shift count.
    /// MOV ignores this field.
    pub second: u16,
    pub destination: ReferenceDestination,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag="status",content="value")]
pub enum ReferenceValue<T> {
    /// This value follows the stated *single-instruction* model from the
    /// manual. It has not been measured on a specific controller.
    ManualDerived(T),
    Unresolved(&'static str),
    NotExecuted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReferenceFlagVector {
    pub carry: ReferenceValue<bool>,
    pub zero: ReferenceValue<bool>,
    pub error: ReferenceValue<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReferenceOutcome {
    pub operation: ReferenceOpcode,
    pub destination: ReferenceDestination,
    pub result: ReferenceValue<u16>,
    pub flags: ReferenceFlagVector,
    pub source: &'static str,
    pub manual_rule_only: bool,
    pub controller_execution_verified: bool,
    pub scan_chain_verified: bool,
    pub native_write_allowed: bool,
}
const UNKNOWN_SCAN: &str =
    "Полный контекст флагов/управления в скане неизвестен: эта модель не исполняет Ladder";
const UNKNOWN_BYTE: &str =
    "Точный результат/Carry для байтового назначения и усечения здесь не подтверждён";
const UNKNOWN_EDGE: &str =
    "Сдвиг на 0/16 допустим синтаксически, но крайний бит Carry не установлен однозначно для модели";
const UNKNOWN_CONDITION: &str =
    "Значение EN не подтверждено: результат и флаги не выводятся";
const UNKNOWN_DEST: &str =
    "Эта операция требует destination M000..M599 в рассматриваемом подмножестве";
const UNKNOWN_SHIFT: &str =
    "Счётчик сдвига вне диапазона руководства 0..16";

fn unresolved(reason:&'static str)->ReferenceFlagVector {
    ReferenceFlagVector {
        carry:ReferenceValue::Unresolved(reason),
        zero:ReferenceValue::Unresolved(reason),
        error:ReferenceValue::Unresolved(reason),
    }
}
fn manual_flags(carry: bool,zero:bool,error:bool)->ReferenceFlagVector {
    ReferenceFlagVector {
        carry:ReferenceValue::ManualDerived(carry),
        zero:ReferenceValue::ManualDerived(zero),
        error:ReferenceValue::ManualDerived(error),
    }
}
fn result(
    input:ReferenceInput,
    value:ReferenceValue<u16>,
    flags:ReferenceFlagVector,
    source:&'static str,
)->ReferenceOutcome {
    ReferenceOutcome {
        operation:input.opcode,destination:input.destination,
        result:value,flags,source,manual_rule_only:true,
        controller_execution_verified:false,scan_chain_verified:false,
        native_write_allowed:false,
    }
}

/// For explicit, human-supplied u16 vectors ONLY. No CIOPRG lines/registry,
/// live outputs, timer, flags history or controller values are read or changed.
///
/// EN=OFF: write is suppressed, but this module does not model how a sequence
/// of flag-influencing instructions changes the scan-wide flag state. Hence
/// full flags remain unresolved even though §12.3 describes clearing rules.
pub fn evaluate_reference(input:ReferenceInput)->ReferenceOutcome {
    use ReferenceOpcode::*;
    use ReferenceValue::*;
    let manual=match input.opcode {
        Mov=>"YRC1000 R12 §12.2.24, §12.3",
        Add=>"YRC1000 R12 §12.2.17, §12.3",
        Sub=>"YRC1000 R12 §12.2.18, §12.3",
        Shl=>"YRC1000 R12 §12.2.29, §12.3",
        Shr=>"YRC1000 R12 §12.2.30, §12.3",
    };
    if input.condition==ReferenceCondition::Unknown {
        return result(input,Unresolved(UNKNOWN_CONDITION),
            unresolved(UNKNOWN_CONDITION),manual);
    }
    if input.condition==ReferenceCondition::Off {
        return result(input,NotExecuted,unresolved(UNKNOWN_SCAN),manual);
    }
    if matches!(input.opcode,Add|Sub) &&
       input.destination!=ReferenceDestination::Register16 {
        return result(input,Unresolved(UNKNOWN_DEST),
            unresolved(UNKNOWN_DEST),manual);
    }
    let a=input.source1;
    let b=input.second;
    match input.opcode {
        Add=>{
            let sum=u32::from(a)+u32::from(b);
            let value=(sum&0xffff) as u16;
            result(input,ManualDerived(value),
                manual_flags(sum>65535,value==0,false),manual)
        }
        Sub=>{
            let value=a.wrapping_sub(b);
            result(input,ManualDerived(value),
                manual_flags(a<b,value==0,false),manual)
        }
        Mov=>match input.destination {
            ReferenceDestination::Register16|ReferenceDestination::WordRelay16=>
                result(input,ManualDerived(a),manual_flags(false,false,false),manual),
            ReferenceDestination::ByteRelay8 if a<=255=>
                result(input,ManualDerived(a),manual_flags(false,false,false),manual),
            ReferenceDestination::ByteRelay8=>
                result(input,Unresolved(UNKNOWN_BYTE),unresolved(UNKNOWN_BYTE),manual),
        },
        Shl|Shr=>{
            if b>16 {
                return result(input,Unresolved(UNKNOWN_SHIFT),
                    unresolved(UNKNOWN_SHIFT),manual);
            }
            if input.destination==ReferenceDestination::ByteRelay8 {
                return result(input,Unresolved(UNKNOWN_BYTE),
                    unresolved(UNKNOWN_BYTE),manual);
            }
            // The manual describes shifts of 16-bit S, zero fill, and output
            // of the shifted-out bit to Carry. n=0/n=16 are separated below.
            let value=if input.opcode==Shl {a.wrapping_shl(u32::from(b))}
                      else {a.wrapping_shr(u32::from(b))};
            if b==0||b==16 {
                return result(input,ManualDerived(value),
                    ReferenceFlagVector {
                        carry:Unresolved(UNKNOWN_EDGE),
                        zero:ManualDerived(false),
                        error:ManualDerived(false),
                    },manual);
            }
            let carry=if input.opcode==Shl {
                ((a >> (16-b))&1)!=0
            } else {
                ((a >> (b-1))&1)!=0
            };
            result(input,ManualDerived(value),manual_flags(carry,false,false),manual)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(op:ReferenceOpcode,a:u16,b:u16)->ReferenceOutcome {
        evaluate_reference(ReferenceInput {opcode:op,source1:a,second:b,
            condition:ReferenceCondition::On,
            destination:ReferenceDestination::Register16})
    }
    fn model_value(o:&ReferenceOutcome)->Option<u16>{
        match o.result {ReferenceValue::ManualDerived(v)=>Some(v),_=>None}
    }
    #[test]
    fn add_unsigned_16bit_carry_zero_and_overflow_vectors(){
        let cases=[
            (0,0,0,false,true),
            (1,1,2,false,false),
            (65535,0,65535,false,false),
            (65535,1,0,true,true),
            (65535,2,1,true,false),
            (65535,65535,65534,true,false),
        ];
        for (a,b,expected,carry,zero) in cases {
            let x=run(ReferenceOpcode::Add,a,b);
            assert_eq!(model_value(&x),Some(expected));
            assert_eq!(x.flags,manual_flags(carry,zero,false));
            assert!(!x.controller_execution_verified);
        }
    }
    #[test]
    fn sub_unsigned_wrap_negative_zero_and_borrow_vectors(){
        let cases=[
            (0,0,0,false,true),
            (5,3,2,false,false),
            (1,1,0,false,true),
            (0,1,65535,true,false),
            (100,101,65535,true,false),
            (65535,65535,0,false,true),
        ];
        for (a,b,expected,carry,zero) in cases {
            let x=run(ReferenceOpcode::Sub,a,b);
            assert_eq!(model_value(&x),Some(expected));
            assert_eq!(x.flags,manual_flags(carry,zero,false));
        }
    }
    #[test]
    fn mov_register_and_word_are_reference_only(){
        let m=run(ReferenceOpcode::Mov,65535,0);
        assert_eq!(model_value(&m),Some(65535));
        assert!(m.manual_rule_only);
        assert!(!m.native_write_allowed);
        assert!(!m.scan_chain_verified);
        let word=evaluate_reference(ReferenceInput {
            opcode:ReferenceOpcode::Mov,condition:ReferenceCondition::On,
            source1:42420,second:0,destination:ReferenceDestination::WordRelay16,
        });
        assert_eq!(model_value(&word),Some(42420));
    }
    #[test]
    fn byte_boundary_256_is_not_assumed_from_ambiguous_manual_wording(){
        for byte_value in [256,257,65535] {
            let x=evaluate_reference(ReferenceInput {
                opcode:ReferenceOpcode::Mov,condition:ReferenceCondition::On,
                source1:byte_value,second:0,destination:ReferenceDestination::ByteRelay8,
            });
            assert!(matches!(x.result,ReferenceValue::Unresolved(_)));
            assert!(matches!(x.flags.carry,ReferenceValue::Unresolved(_)));
        }
        let x=evaluate_reference(ReferenceInput {
            opcode:ReferenceOpcode::Mov,condition:ReferenceCondition::On,
            source1:255,second:0,destination:ReferenceDestination::ByteRelay8,
        });
        assert_eq!(model_value(&x),Some(255));
    }
    #[test]
    fn shift_one_matches_manual_exemplar_and_carry_bit(){
        let x=run(ReferenceOpcode::Shl,0x8888,1);
        assert_eq!(model_value(&x),Some(0x1110));
        assert_eq!(x.flags.carry,ReferenceValue::ManualDerived(true));
        let y=run(ReferenceOpcode::Shr,0x8888,1);
        assert_eq!(model_value(&y),Some(0x4444));
        assert_eq!(y.flags.carry,ReferenceValue::ManualDerived(false));
        let z=run(ReferenceOpcode::Shr,1,1);
        assert_eq!(model_value(&z),Some(0));
        assert_eq!(z.flags.carry,ReferenceValue::ManualDerived(true));
    }
    #[test]
    fn shift_zero_and_sixteen_are_explicit_edge_cases(){
        for op in [ReferenceOpcode::Shl,ReferenceOpcode::Shr] {
            let zero=run(op,0x8001,0);
            assert_eq!(model_value(&zero),Some(0x8001));
            assert!(matches!(zero.flags.carry,ReferenceValue::Unresolved(_)));
            let sixteen=run(op,0x8001,16);
            assert_eq!(model_value(&sixteen),Some(0));
            assert!(matches!(sixteen.flags.carry,ReferenceValue::Unresolved(_)));
            let invalid=run(op,0x8001,17);
            assert!(matches!(invalid.result,ReferenceValue::Unresolved(_)));
            let byte=evaluate_reference(ReferenceInput {
                opcode:op,condition:ReferenceCondition::On,
                source1:0xffff,second:1,destination:ReferenceDestination::ByteRelay8,
            });
            assert!(matches!(byte.result,ReferenceValue::Unresolved(_)));
        }
    }
    #[test]
    fn no_execution_when_disabled_or_condition_unknown(){
        for op in [ReferenceOpcode::Mov,ReferenceOpcode::Add,ReferenceOpcode::Sub,
                   ReferenceOpcode::Shl,ReferenceOpcode::Shr] {
            for condition in [ReferenceCondition::Off,ReferenceCondition::Unknown] {
                let x=evaluate_reference(ReferenceInput {
                    opcode:op,source1:65535,second:1,
                    condition,destination:ReferenceDestination::Register16,
                });
                assert!(matches!(x.result,ReferenceValue::NotExecuted|
                    ReferenceValue::Unresolved(_)));
                assert!(matches!(x.flags.carry,ReferenceValue::Unresolved(_)));
                assert!(!x.controller_execution_verified);
            }
        }
    }
    #[test]
    fn arithmetic_destination_must_be_register_form(){
        let x=evaluate_reference(ReferenceInput {
            opcode:ReferenceOpcode::Add,condition:ReferenceCondition::On,
            source1:1,second:2,destination:ReferenceDestination::WordRelay16,
        });
        assert!(matches!(x.result,ReferenceValue::Unresolved(_)));
    }
    #[test]
    fn whole_manual_range_does_not_panic_or_overflow(){
        for n in 0..=65535u16 {
            let x=run(ReferenceOpcode::Add,n,1);
            assert_eq!(model_value(&x),Some(n.wrapping_add(1)));
            let y=run(ReferenceOpcode::Sub,n,1);
            assert_eq!(model_value(&y),Some(n.wrapping_sub(1)));
        }
    }
}
