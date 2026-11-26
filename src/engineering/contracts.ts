import type { Controller, EngineeringCore, SemanticPreview } from "./types.ts";

type InspectionEnvelope = {
  controller:Controller;source_sha256:string;byte_length:number;line_count:number;
  core:EngineeringCore;semantic:SemanticPreview;
};
const natural=(n:number)=>Number.isSafeInteger(n)&&n>=0;

/** Reject incompatible/stale IPC projections before committing UI state. */
export function assertEngineeringContract(report:InspectionEnvelope):void {
  const core=report.core;
  if(!core||core.schema_version!==1||!core.identity||!core.evidence||
     !Array.isArray(core.rung_refs))throw Error("Несовместимый контракт Semantic Core");
  const id=core.identity, sem=report.semantic, evidence=core.evidence;
  if(!["DX100","DX200","YRC1000"].includes(id.controller)||
     !/^[0-9a-f]{64}$/.test(id.source_sha256)||
     id.controller!==report.controller||id.source_sha256!==report.source_sha256||
     !natural(id.byte_length)||!natural(id.line_count)||
     id.byte_length!==report.byte_length||id.line_count!==report.line_count||
     evidence.execution_verified!==false||evidence.scan_order_verified!==false||
     evidence.native_write_allowed!==false||
     evidence.boolean_subset_complete!==sem.subset_analysis_complete||
     !natural(evidence.unsupported_segments)||evidence.unsupported_segments!==sem.unsupported_segments||
     !natural(core.rung_count)||core.rung_count!==sem.rung_count||
     core.rung_refs.length!==sem.rungs.length||
     core.rung_refs.length>core.rung_count||
     core.rung_refs_truncated!==(core.rung_refs.length<core.rung_count)||
     core.rung_refs_truncated!==sem.rungs_truncated)
    throw Error("Semantic Core: отчёт не соответствует снимку или границам анализа");
  const seen=new Set<string>();
  for(let i=0;i<core.rung_refs.length;i++){
    const ref=core.rung_refs[i], rung=sem.rungs[i], span=ref.source;
    if(!span||!span.bytes||!natural(ref.number)||ref.number<1||
       ref.number!==rung.number||span.first_line!==rung.first_line||span.last_line!==rung.output_line||
       !natural(span.first_line)||!natural(span.last_line)||span.first_line<1||
       span.first_line>span.last_line||span.last_line>id.line_count||
       !natural(span.bytes.start)||!natural(span.bytes.end)||
       span.bytes.start>=span.bytes.end||span.bytes.end>id.byte_length||
       ref.id!==id.controller+":"+id.source_sha256+":rung:"+span.bytes.start+":"+span.bytes.end||
       seen.has(ref.id))throw Error("Semantic Core: некорректная ссылка на цепь");
    seen.add(ref.id);
  }
}
