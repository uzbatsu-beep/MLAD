import test from "node:test";
import assert from "node:assert/strict";
import {
  addBranch, appendDraftRung, changeAt, deleteContact, evaluateDraft,
  findContact, layoutCircuit, makeContact, newDraft,
  semanticToExpression,
} from "./graphModel.ts";

test("series and parallel geometry has realistic branching and independent signal paths", () => {
  const a=makeContact("#70010"); const b=makeContact("#70011");
  const c=makeContact("#70012");
  const expression={kind:"and" as const,id:"g1",
    left:a, right:{kind:"or" as const,id:"g2",left:b,right:c}};
  const result=layoutCircuit(expression);
  assert.equal(result.contacts.length,3);
  assert.ok(result.wires.length>=7);
  assert.ok(result.height>170);
  assert.ok(result.width>350);
  assert.equal(evaluateDraft(expression,{"#70010":true,"#70011":false,"#70012":true}),true);
  assert.equal(evaluateDraft(expression,{"#70010":false,"#70011":true,"#70012":true}),false);
  assert.equal(evaluateDraft(expression,{"#70010":true,"#70011":false}),undefined);
});

test("NO and NC inversion is functional",()=>{
  const nc=makeContact("#70010",true);
  assert.equal(evaluateDraft(nc,{"#70010":true}),false);
  assert.equal(evaluateDraft(nc,{"#70010":false}),true);
  assert.equal(evaluateDraft(nc,{}),undefined);
});

test("click-target edits preserve other branch IDs and are immutable",()=>{
  const start=makeContact("#70010");
  const junction=addBranch(start,start.id,"or");
  assert.equal(junction.kind,"or");
  if(junction.kind==="contact") throw Error("expected junction");
  const siblingId=junction.right.id;
  const revised=changeAt(junction,siblingId, old=>({...old,relay:"#70011"}));
  assert.notEqual(revised,junction);
  assert.equal(findContact(revised,siblingId)?.relay,"#70011");
  assert.equal(findContact(junction,siblingId)?.relay,"#70010");
  const trimmed=deleteContact(revised,siblingId);
  assert.equal(trimmed.id,start.id);
});

test("wrapping whole expression in series preserves entire subtree",()=>{
  const first=makeContact("#70010");
  const graph=addBranch(addBranch(first,null,"or"),null,"and");
  const layout=layoutCircuit(graph);
  assert.equal(layout.contacts.length,3);
  assert.equal(findContact(graph,first.id)?.relay,"#70010");
});

test("semantic Rust AST vector becomes the same boolean network",()=>{
  const nodes=[
    {op:{Contact:{relay:70010,inverted:false}},line:1},
    {op:{Contact:{relay:70011,inverted:true}},line:2},
    {op:{Or:{left:0,right:1}},line:3},
    {op:{Contact:{relay:70020,inverted:false}},line:4},
    {op:{And:{left:2,right:3}},line:5},
  ];
  const expr=semanticToExpression(nodes,4);
  assert.equal(expr.kind,"and");
  assert.equal(layoutCircuit(expr).contacts.length,3);
  assert.equal(evaluateDraft(expr,{"#70010":false,"#70011":false,"#70020":true}),true);
  assert.equal(evaluateDraft(expr,{"#70010":false,"#70011":true,"#70020":true}),false);
});

test("invalid AST references and node caps fail explicitly",()=>{
  assert.throws(()=>semanticToExpression([],0));
  assert.throws(()=>semanticToExpression([{op:{And:{left:0,right:0}},line:1}],0));
  const nodes=Array.from({length:270},(_,i)=>
    i===0?{op:{Contact:{relay:70010,inverted:false}},line:1}:
    {op:{And:{left:i-1,right:0}},line:i+1});
  assert.throws(()=>semanticToExpression(nodes,269,100));
});

test("layout rejects structurally invalid repeated IDs",()=>{
  const node=makeContact("#70010");
  const broken={kind:"or" as const,id:"join",left:node,right:node};
  assert.throws(()=>layoutCircuit(broken));
});

test("draft creation keeps compatible schema version and bounded rung count",()=>{
  let p=newDraft();
  assert.equal(p.version,1);
  assert.equal(p.controller,"YRC1000");
  p=appendDraftRung(p);
  assert.equal(p.rungs.length,2);
  assert.notEqual(p.rungs[0].id,p.rungs[1].id);
  assert.ok(p.rungs[1].output.startsWith("#"));
});
