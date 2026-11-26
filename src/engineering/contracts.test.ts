import test from "node:test";
import assert from "node:assert/strict";
import { assertEngineeringContract } from "./contracts.ts";
import type { EngineeringCore, SemanticPreview } from "./types.ts";

function fixture(){
  const sha="a".repeat(64);
  const semantic:SemanticPreview={
    nodes:[],nodes_truncated:false,node_count:0,rung_count:1,reference_count:0,diagnostic_count:0,
    unsupported_segments:0,subset_analysis_complete:true,system_user_boundaries_verified:false,
    edited_export_allowed:false,end_seen:true,
    rungs:[{number:1,first_line:1,output_line:2,output:70020,inputs:[70010],
      expression_preview:"",expression_root:0,section:"Unspecified"}],
    references:[],part_markers:[],diagnostics:[],rungs_truncated:false,
    references_truncated:false,diagnostics_truncated:false
  };
  const core:EngineeringCore={schema_version:1,
    identity:{source_sha256:sha,controller:"YRC1000",line_count:3,byte_length:30},
    rung_count:1,rung_refs_truncated:false,
    rung_refs:[{id:"YRC1000:"+sha+":rung:0:23",number:1,source:{first_line:1,last_line:2,bytes:{start:0,end:23}}}],
    evidence:{boolean_subset_complete:true,unsupported_segments:0,execution_verified:false,scan_order_verified:false,native_write_allowed:false}
  };
  return {...core.identity,core,semantic};
}

test("accepts matching versioned source-linked engineering report",()=>{
  assert.doesNotThrow(()=>assertEngineeringContract(fixture()));
});
test("rejects mismatched snapshot, family, size and schema",()=>{
  for(const change of [
    (f:ReturnType<typeof fixture>)=>{f.source_sha256="b".repeat(64);},
    (f:ReturnType<typeof fixture>)=>{f.controller="DX200";},
    (f:ReturnType<typeof fixture>)=>{f.byte_length=31;},
    (f:ReturnType<typeof fixture>)=>{Object.assign(f.core,{schema_version:2});},
  ]){const f=fixture();change(f);assert.throws(()=>assertEngineeringContract(f));}
});
test("rejects unsafe claims and inconsistent unknowns",()=>{
  for(const key of ["execution_verified","scan_order_verified","native_write_allowed"]){
    const f=fixture();Object.assign(f.core.evidence,{[key]:true});assert.throws(()=>assertEngineeringContract(f));
  }
  const f=fixture();f.core.evidence.unsupported_segments=3;assert.throws(()=>assertEngineeringContract(f));
});
test("rejects invalid byte ranges, stale rung keys and hidden truncation",()=>{
  const invalid=fixture();invalid.core.rung_refs[0].source.bytes.end=31;
  assert.throws(()=>assertEngineeringContract(invalid));
  const stale=fixture();stale.core.rung_refs[0].id="old";assert.throws(()=>assertEngineeringContract(stale));
  const truncated=fixture();truncated.core.rung_count=2;truncated.semantic.rung_count=2;
  assert.throws(()=>assertEngineeringContract(truncated));
  truncated.core.rung_refs_truncated=true;truncated.semantic.rungs_truncated=true;
  assert.doesNotThrow(()=>assertEngineeringContract(truncated));
});
test("rejects wrong line, negative length and mismatched semantic rung",()=>{
  const a=fixture();a.core.rung_refs[0].source.last_line=4;assert.throws(()=>assertEngineeringContract(a));
  const b=fixture();b.core.identity.byte_length=-1;b.byte_length=-1;assert.throws(()=>assertEngineeringContract(b));
  const c=fixture();c.semantic.rungs[0].number=2;assert.throws(()=>assertEngineeringContract(c));
});
