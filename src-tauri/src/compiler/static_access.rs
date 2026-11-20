//! Read-only, source-linked operand-role audit for exported CIOPRG.LST.
//!
//! READ/WRITE mean **manual-described potential operand roles**, NEVER actual
//! hardware reads/writes, execution, scan order, permissions or provenance of
//! values. Unsupported/malformed/unverified forms are UNKNOWN; post-END data
//! are not interpreted. Every span points into the unchanged source snapshot.
use serde::Serialize;
use super::lossless::{AstInstruction, AstOperand, ByteSpan, LineKind, LosslessDocument, OperandKind};

pub const PAGE_SIZE:usize=100;
pub const MAX_PREVIEW:usize=160;
const MAX_SOURCE_LINES:usize=200_000;

#[derive(Debug,Clone,Copy,PartialEq,Eq,Serialize)]
pub enum AccessKind { Read, Write, Unknown }

#[derive(Debug,Clone,Copy,PartialEq,Eq,Serialize)]
pub enum AccessBasis {
    DocumentedOperandRole,
    InvalidForm,
    UnverifiedFamily,
    UnknownOpcodeOrStructure,
    StatefulOpcodeUnresolved,
}

#[derive(Debug,Clone,Serialize)]
pub struct AccessSite {
    pub line:usize,
    pub mnemonic:String,
    pub operand_index:Option<usize>, // 1-based; None is a whole-line unknown
    pub symbol:String,               // bounded display text, not a parsed alias
    pub span:ByteSpan,                // ORIGINAL absolute byte span, never reconstructed
    pub access:AccessKind,
    pub basis:AccessBasis,
    pub controller_form_verified:bool,
}

#[derive(Debug,Clone,Serialize)]
pub struct AccessOverview {
    pub source_sha256:String,
    pub line_count:usize,
    pub total_sites:usize,
    pub read_sites:usize,
    pub write_sites:usize,
    pub unknown_sites:usize,
    pub unmodeled_lines:usize,
    pub skipped_literal_operands:usize,
    pub post_end_lines:usize,
    pub sites_truncated:bool,
    pub sites:Vec<AccessSite>,
    pub execution_verified:bool,
    pub scan_order_verified:bool,
    pub address_aliases_verified:bool,
    pub write_permission_verified:bool,
    pub native_write_allowed:bool,
}

#[derive(Debug,Clone,Serialize)]
pub struct AccessPage {
    pub source_sha256:String,
    pub line_count:usize,
    pub offset:usize,
    pub total_matches:usize,
    pub next_offset:Option<usize>,
    pub hits:Vec<AccessSite>,
    pub execution_verified:bool,
    pub write_permission_verified:bool,
}

fn is_memory_operand(kind:OperandKind)->bool {
    matches!(kind,OperandKind::RelayBit|OperandKind::RelayByte|
        OperandKind::RelayWord|OperandKind::MRegister)
}
fn source_role(mnemonic:&str, index:usize)->Option<AccessKind> {
    use AccessKind::*;
    match mnemonic {
        "MOV"|"BIN"|"BCD"|"WNOT" =>
            match index{0=>Some(Read),1=>Some(Write),_=>None},
        "ADD"|"SUB"|"MUL"|"DIV"|"MOD"|"WAND"|"WOR"|"WXOR" =>
            match index{0|1=>Some(Read),2=>Some(Write),_=>None},
        "SHL"|"SHR"|"ROL"|"ROR"|"BMOV" =>
            match index{0=>Some(Read),2=>Some(Write),_=>None},
        "CMP" => match index{0|1=>Some(Read),_=>None},
        "PLS"|"PLF" => match index{0=>Some(Write),_=>None},
        "STR"|"STR-NOT"|"AND"|"AND-NOT"|"OR"|"OR-NOT"|"GSTR" =>
            match index{0=>Some(Read),_=>None},
        "OUT"|"GOUT" => match index{0=>Some(Write),_=>None},
        _=>None,
    }
}
fn display(doc:&LosslessDocument,span:ByteSpan)->String{
    let raw=&doc.original_bytes()[span.start..span.end];
    String::from_utf8_lossy(&raw[..raw.len().min(96)]).into_owned()
}
fn bounded_name(name:&str)->String{name.chars().take(32).collect()}
fn emit_operand(
    doc:&LosslessDocument, inst:&AstInstruction, operand:&AstOperand, index:usize,
    role:Option<AccessKind>, valid:bool, family:bool,
    unknown_basis:AccessBasis, push:&mut impl FnMut(AccessSite),
    skipped:&mut usize,
) {
    // Decimal literals and numeric shift/count operands do NOT read CIO memory.
    // Do not create a fictitious memory access for a numeric constant.
    if operand.kind==OperandKind::DecimalLiteral && valid {
        *skipped+=1;return;
    }
    if !is_memory_operand(operand.kind) && valid && role.is_some() {
        // A source token which resembles an unclassified address is kept
        // UNKNOWN rather than promoted to a bogus READ/WRITE.
    }
    let (access,basis)=if valid && family && is_memory_operand(operand.kind) {
        match role {
            Some(kind)=>(kind,AccessBasis::DocumentedOperandRole),
            None=>(AccessKind::Unknown,unknown_basis),
        }
    } else {
        (AccessKind::Unknown,if !valid {AccessBasis::InvalidForm} else if !family {
            AccessBasis::UnverifiedFamily
        } else {unknown_basis})
    };
    push(AccessSite{
        line:inst.line,mnemonic:bounded_name(&inst.mnemonic),
        operand_index:Some(index+1),symbol:display(doc,operand.span),
        span:operand.span,access,basis,controller_form_verified:family,
    });
}
fn unknown_line(doc:&LosslessDocument,line:usize,mnemonic:&str,span:ByteSpan,
    basis:AccessBasis,push:&mut impl FnMut(AccessSite)) {
    push(AccessSite{line,mnemonic:bounded_name(mnemonic),operand_index:None,
        symbol:display(doc,span),span,access:AccessKind::Unknown,basis,
        controller_form_verified:false});
}
fn scan(doc:&LosslessDocument,mut push:impl FnMut(AccessSite)) -> (usize,usize,usize,usize,usize) {
    let mut read=0usize;
    let mut write=0usize;
    let mut unknown=0usize;
    let mut unmodeled=0usize;
    let mut literals=0usize;
    // Wrap the consumer to keep counts independent of preview/query limits.
    let mut counted=|site:AccessSite|{
        match site.access {
            AccessKind::Read=>read+=1,
            AccessKind::Write=>write+=1,
            AccessKind::Unknown=>unknown+=1,
        }
        push(site);
    };
    for line in &doc.lines {
        match &line.kind {
            LineKind::TypedCandidate{instruction,controller_form_verified,..}=>{
                let valid=instruction.syntax_valid;
                if !valid || !*controller_form_verified{unmodeled+=1;}
                if instruction.operands.is_empty() {
                    unmodeled+=1;
                    unknown_line(doc,line.number,&instruction.mnemonic,line.body_span,
                        AccessBasis::InvalidForm,&mut counted);
                }
                for (index,operand) in instruction.operands.iter().enumerate() {
                    emit_operand(doc,instruction,operand,index,
                        source_role(&instruction.mnemonic,index),
                        valid,*controller_form_verified,AccessBasis::UnknownOpcodeOrStructure,
                        &mut counted,&mut literals);
                }
            }
            LineKind::Instruction(inst)=>{
                if matches!(inst.mnemonic.as_str(),"PART"|"END"|"AND-STR"|"OR-STR")
                    && inst.syntax_valid {continue;}
                let simple=matches!(inst.mnemonic.as_str(),
                    "STR"|"STR-NOT"|"AND"|"AND-NOT"|"OR"|"OR-NOT"|
                    "OUT"|"GSTR"|"GOUT");
                let valid=inst.syntax_valid;
                if !simple || !valid {unmodeled+=1;}
                if inst.operands.is_empty() {
                    unmodeled+=1;
                    unknown_line(doc,line.number,&inst.mnemonic,line.body_span,
                        AccessBasis::InvalidForm,&mut counted);
                }
                for (index,operand) in inst.operands.iter().enumerate(){
                    emit_operand(doc,inst,operand,index,
                        if simple{source_role(&inst.mnemonic,index)}else{None},
                        valid,valid && simple,
                        if matches!(inst.mnemonic.as_str(),"TMR"|"CNT"){
                            AccessBasis::StatefulOpcodeUnresolved
                        }else {AccessBasis::UnknownOpcodeOrStructure},
                        &mut counted,&mut literals);
                }
            }
            LineKind::UnsupportedInstruction{mnemonic}=>{
                unmodeled+=1;
                unknown_line(doc,line.number,mnemonic,line.body_span,
                    AccessBasis::UnknownOpcodeOrStructure,&mut counted);
            }
            LineKind::Opaque=>{
                unmodeled+=1;
                unknown_line(doc,line.number,"UNKNOWN",line.body_span,
                    AccessBasis::UnknownOpcodeOrStructure,&mut counted);
            }
            // Structural lines and ALL post-END payloads produce no
            // access claims. Post-END lines are counted separately.
            _=>{}
        }
    }
    (read,write,unknown,unmodeled,literals)
}

pub fn inspect(doc:&LosslessDocument,max_sites:usize)->AccessOverview {
    let max_sites=max_sites.min(MAX_PREVIEW);
    let mut sites=Vec::new();
    let mut total=0;
    let (read_sites,write_sites,unknown_sites,unmodeled_lines,skipped_literal_operands)=
        scan(doc,|site|{
            total+=1;
            if sites.len()<max_sites {sites.push(site);}
        });
    let post_end_lines=doc.lines.iter().filter(|line|matches!(line.kind,
        LineKind::PostEndData|LineKind::PostEndOpaque|LineKind::PostEndSection{..})).count();
    AccessOverview{
        source_sha256:doc.source_sha256.clone(),line_count:doc.lines.len(),
        total_sites:total,read_sites,write_sites,unknown_sites,
        unmodeled_lines,skipped_literal_operands,post_end_lines,
        sites_truncated:total>sites.len(),sites,
        execution_verified:false,scan_order_verified:false,
        address_aliases_verified:false,write_permission_verified:false,
        native_write_allowed:false,
    }
}
fn valid_symbol_filter(value:&str)->bool {
    let b=value.as_bytes();
    (b.len()==6 && b[0]==b'#' && b[1..].iter().all(u8::is_ascii_digit)) ||
    (b.len()==4 && b[0]==b'M' && b[1..].iter().all(u8::is_ascii_digit)) ||
    (b.len()==7 && b[0]==b'W' && b[1]==b'#' &&
        b[2..].iter().all(u8::is_ascii_digit))
}
pub fn query(doc:&LosslessDocument,kind:Option<&str>,symbol:Option<&str>,offset:usize)
    ->Result<AccessPage,String>{
    if offset>MAX_SOURCE_LINES*8 {
        return Err("Недопустимый offset статического аудита".into());
    }
    let filter=match kind {
        None|Some("")=>None,
        Some("Read")=>Some(AccessKind::Read),
        Some("Write")=>Some(AccessKind::Write),
        Some("Unknown")=>Some(AccessKind::Unknown),
        _=>return Err("Неверный фильтр READ/WRITE/UNKNOWN".into()),
    };
    if symbol.is_some_and(|s|!valid_symbol_filter(s)) {
        return Err("Адрес должен быть точным токеном #XXXXX, Mxxx или W#XXXXX".into());
    }
    if filter.is_none() && symbol.is_none() {
        return Err("Задайте вид доступа или адрес для фильтрации".into());
    }
    let mut total=0usize;
    let mut hits=Vec::new();
    scan(doc,|site|{
        if filter.is_some_and(|k|site.access!=k) {return;}
        if symbol.is_some_and(|name|site.symbol!=name) {return;}
        if total>=offset && hits.len()<PAGE_SIZE {hits.push(site);}
        total+=1;
    });
    let next_offset=if offset.saturating_add(hits.len())<total {
        Some(offset.saturating_add(hits.len()))
    }else {None};
    Ok(AccessPage{
        source_sha256:doc.source_sha256.clone(),line_count:doc.lines.len(),
        offset,total_matches:total,next_offset,hits,
        execution_verified:false,write_permission_verified:false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::memory::ControllerFamily;
    use crate::compiler::lossless::parse_document;
    fn parse(raw:&[u8],family:ControllerFamily)->LosslessDocument {
        parse_document(raw.to_vec(),family)
    }
    #[test]
    fn manual_roles_and_byte_spans_are_source_linked_not_hardware_events(){
        let raw=b"STR #70010\r\nAND-NOT #20011\nOUT #70020\r\nMOV M010,M011\nADD M011,2,M012\nMUL M012,3,M598\nCMP M012,M010\nBMOV M010,5,M100\nPLS #70100\nEND\n//MESSAGE1\nOUT #70021\n";
        let doc=parse(raw,ControllerFamily::YRC1000);
        let a=inspect(&doc,160);
        assert_eq!(a.line_count,12);
        assert_eq!(a.post_end_lines,2);
        assert_eq!(a.read_sites,8);
        assert_eq!(a.write_sites,7);
        assert_eq!(a.unknown_sites,0);
        assert_eq!(a.skipped_literal_operands,3);
        assert_eq!(a.total_sites,15);
        assert!(!a.execution_verified);
        assert!(!a.scan_order_verified);
        assert!(!a.address_aliases_verified);
        assert!(!a.write_permission_verified);
        assert!(!a.native_write_allowed);
        for s in &a.sites{
            assert_eq!(&raw[s.span.start..s.span.end],s.symbol.as_bytes());
            assert!(s.line>=1 && s.line<=10);
            assert!(s.controller_form_verified);
        }
        assert_eq!(doc.copy_original(),raw);
    }
    #[test]
    fn unknown_invalid_and_dx_families_do_not_gain_read_write_claims(){
        let raw=b"MUL 1,2,M599\nBMOX M010,5,M011\nTMR M010,10\nCMP M010,M011\nEND\n";
        let dx=parse(raw,ControllerFamily::DX100);
        let a=inspect(&dx,100);
        assert_eq!(a.read_sites,0);
        assert_eq!(a.write_sites,0);
        assert!(a.unknown_sites>=6);
        assert!(a.sites.iter().all(|x|x.access==AccessKind::Unknown));
        assert_eq!(a.sites[0].basis,AccessBasis::InvalidForm);
        assert!(a.sites.iter().any(|x|x.basis==AccessBasis::UnverifiedFamily));
        assert!(a.sites.iter().any(|x|x.basis==AccessBasis::StatefulOpcodeUnresolved));
        assert_eq!(dx.copy_original(),raw);
    }
    #[test]
    fn query_paginates_full_source_and_filters_exact_symbol(){
        let mut raw=Vec::new();
        for _ in 0..352 {raw.extend_from_slice(b"STR #70010\n");}
        raw.extend_from_slice(b"END\n//ALARM1\nSTR #70010\n");
        let doc=parse(&raw,ControllerFamily::YRC1000);
        let sum=inspect(&doc,3);
        assert_eq!(sum.total_sites,352);
        assert_eq!(sum.sites.len(),3);
        assert!(sum.sites_truncated);
        assert_eq!(query(&doc,Some("Read"),None,0).unwrap().hits.len(),100);
        let third=query(&doc,Some("Read"),Some("#70010"),200).unwrap();
        assert_eq!(third.offset,200);
        assert_eq!(third.total_matches,352);
        assert_eq!(third.hits.len(),100);
        assert_eq!(third.hits[0].line,201);
        assert_eq!(third.next_offset,Some(300));
        let last=query(&doc,Some("Read"),None,300).unwrap();
        assert_eq!(last.hits.len(),52);
        assert_eq!(last.next_offset,None);
        assert_eq!(doc.copy_original(),raw);
    }
    #[test]
    fn query_rejects_fuzzy_or_wrong_tokens_and_no_filter(){
        let doc=parse(b"MOV M010,W#70010\nEND\n",ControllerFamily::YRC1000);
        for invalid in ["M01","m010","#7001","W#7001","M010 or 1","\n","#700100"]{
            assert!(query(&doc,None,Some(invalid),0).is_err(),"{invalid}");
        }
        assert!(query(&doc,None,None,0).is_err());
        assert!(query(&doc,Some("EXECUTED"),None,0).is_err());
        assert!(query(&doc,Some("Read"),None,1_600_001).is_err());
        assert_eq!(query(&doc,Some("Read"),Some("M010"),0).unwrap().hits.len(),1);
        assert_eq!(query(&doc,Some("Write"),Some("W#70010"),0).unwrap().hits.len(),1);
    }
    #[test]
    fn post_end_opaque_and_synthetic_bytes_never_become_access_events(){
        let raw=b"STR #70010\r\nEND\r\n//ALARM1\r\nOUT #70020\r\nBAD \xff\x00\n";
        let doc=parse(raw,ControllerFamily::YRC1000);
        let audit=inspect(&doc,100);
        assert_eq!(audit.total_sites,1);
        assert_eq!(audit.post_end_lines,3);
        assert_eq!(doc.copy_original(),raw);
    }
}
