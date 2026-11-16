import test from "node:test";
import assert from "node:assert/strict";
import {newBlock,freshAdvancedDraft,blockReviewLines,isValidPreset,isGroupStart,describeCondition} from "./advancedModel.ts";
import type {GraphExpr} from "./graphModel.ts";

test("timer UI model has independent enable and typed 100ms preset",()=>{
  const b=newBlock("TMR");
  assert.equal(b.kind,"TMR");
  if(b.kind!=="TMR")throw Error();
  assert.equal(b.preset.kind,"literal");
  if(b.preset.kind==="literal")assert.equal(b.preset.value,100);
  assert.equal(b.enable.kind,"contact");
  const lines=blockReviewLines(b);
  assert.ok(lines.some(x=>x.startsWith("TMR M")));
  assert.ok(lines.some(x=>x.startsWith("OUT #")));
});
test("counter exposes separate count and reset condition ports",()=>{
  const b=newBlock("CNT");
  assert.equal(b.kind,"CNT");
  if(b.kind!=="CNT")throw Error();
  assert.notEqual(b.count.id,b.reset.id);
  const lines=blockReviewLines(b);
  assert.ok(lines.some(x=>x.startsWith("COUNT CONDITION")));
  assert.ok(lines.some(x=>x.startsWith("RESET CONDITION")));
  assert.ok(lines.some(x=>x.startsWith("CNT M")));
});
test("group is paired and byte aligned",()=>{
  const b=newBlock("GROUP");
  if(b.kind!=="GROUP")throw Error();
  assert.deepEqual(blockReviewLines(b),["GSTR #20010","GOUT #00010"]);
  assert.ok(isGroupStart(b.source));
  assert.equal(isGroupStart("#20011"),false);
  assert.equal(isGroupStart("#87010"),true);
});
test("manual numeric bounds and M register spelling are enforced in editor helper",()=>{
  assert.ok(isValidPreset({kind:"literal",value:65535}));
  assert.ok(isValidPreset({kind:"register",address:"M999"}));
  assert.equal(isValidPreset({kind:"literal",value:65536}),false);
  assert.equal(isValidPreset({kind:"literal",value:-1}),false);
  assert.equal(isValidPreset({kind:"register",address:"M1000"}),false);
});
test("review condition preserves boolean expression without a native file export",()=>{
  const tree:GraphExpr={kind:"and",id:"root",
    left:{kind:"contact",id:"a",relay:"#70010",normallyClosed:false},
    right:{kind:"contact",id:"b",relay:"#70011",normallyClosed:true}};
  assert.equal(describeCondition(tree),"(#70010 AND NOT #70011)");
});
test("advanced project is independent versioned JSON model",()=>{
  const d=freshAdvancedDraft();
  assert.equal(d.version,1);
  assert.equal(d.controller,"YRC1000");
  assert.deepEqual(d.blocks,[]);
  assert.notEqual("native_export_allowed" in d,true);
});
