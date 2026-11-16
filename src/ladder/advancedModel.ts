/** MLAD advanced instruction drafts. All objects are independent JSON only.
 * No emitter for native CIOPRG.LST exists here. */
import {makeContact, type Controller, type GraphExpr} from "./graphModel.ts";

export type Preset = {kind:"literal";value:number} | {kind:"register";address:string};
export type TimerBlock = {
  kind:"TMR";id:string;enable:GraphExpr;current:string;preset:Preset;output:string;
};
export type CounterBlock = {
  kind:"CNT";id:string;count:GraphExpr;reset:GraphExpr;current:string;preset:Preset;output:string;
};
export type GroupBlock = {kind:"GROUP";id:string;source:string;destination:string};
export type AdvancedBlock = TimerBlock|CounterBlock|GroupBlock;
export type AdvancedDraft = {version:1;controller:Controller;blocks:AdvancedBlock[]};
export type AdvancedDiagnostic = {block_id:string;code:string;level:"Warning"|"Error";message:string};
export type AdvancedValidation = {
  diagnostics:AdvancedDiagnostic[];has_errors:boolean;structure_valid:boolean;
  native_export_allowed:boolean;hardware_test_authorized:boolean;
};

let counter=0;
export function newBlock(kind:AdvancedBlock["kind"]):AdvancedBlock {
  const id="advanced-"+(++counter).toString(36)+"-"+Date.now().toString(36);
  const n=counter;
  if(kind==="GROUP") return {kind,id,source:"#20010",destination:"#00010"};
  const preset:Preset={kind:"literal",value:kind==="TMR"?100:3};
  const current="M"+String(10+n).padStart(3,"0");
  const output="#"+String(70100+n*10);
  if(kind==="TMR") return {kind,id,current,preset,output,enable:makeContact("#70010")};
  return {kind,id,current,preset,output,
    count:makeContact("#70010"),reset:makeContact("#70011")};
}
export function freshAdvancedDraft():AdvancedDraft {
  return {version:1,controller:"YRC1000",blocks:[]};
}

export function describeCondition(e:GraphExpr, limit=200):string {
  const tokens:(string|GraphExpr)[]=[e];
  let output="";
  let processed=0;
  while(tokens.length&&processed++<limit) {
    const t=tokens.pop()!;
    if(typeof t==="string") {output+=t;continue;}
    if(t.kind==="contact") {
      output+=(t.normallyClosed?"NOT ":"")+t.relay;
    } else {
      tokens.push(")",t.right,t.kind==="and"?" AND ":" OR ",t.left,"(");
    }
    if(output.length>500)return output.slice(0,500)+"…";
  }
  if(tokens.length)return output.slice(0,500)+"…";
  return output;
}

/** Pseudocode for review only, NEVER a native file serializer. */
export function blockReviewLines(b:AdvancedBlock):string[] {
  if(b.kind==="GROUP") return [`GSTR ${b.source}`,`GOUT ${b.destination}`];
  const preset=b.preset.kind==="literal"?String(b.preset.value):b.preset.address;
  if(b.kind==="TMR") return [
    `CONDITION: ${describeCondition(b.enable)}`,
    `TMR ${b.current},${preset}`,
    `OUT ${b.output}`,
  ];
  return [
    `COUNT CONDITION: ${describeCondition(b.count)}`,
    `RESET CONDITION: ${describeCondition(b.reset)}`,
    `CNT ${b.current},${preset}`,
    `OUT ${b.output}`,
  ];
}
export function isValidPreset(p:Preset):boolean {
  if(p.kind==="literal") return Number.isInteger(p.value)&&p.value>=0&&p.value<=65535;
  return /^M[0-9]{3}$/.test(p.address);
}
export function isGroupStart(s:string):boolean {
  return /^#[0-9]{4}0$/.test(s);
}
