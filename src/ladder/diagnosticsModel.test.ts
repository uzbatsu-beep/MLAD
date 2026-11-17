import assert from "node:assert/strict";
import test from "node:test";
import {groupDiagnostics} from "./diagnosticsModel.ts";

test("groups warnings by severity and code without losing source lines",()=>{
  const values=[
    {severity:"Warning",code:"WriteNeedsReview",line:21,message:"OUT #40045"},
    {severity:"Warning",code:"WriteNeedsReview",line:24,message:"OUT #40046"},
    {severity:"Error",code:"OperandCount",line:40,message:"bad"},
    {severity:"Warning",code:"WriteNeedsReview",line:42,message:"OUT #40047"},
  ] as const;
  const original=JSON.stringify(values);
  const groups=groupDiagnostics(values);
  assert.equal(groups.length,2);
  assert.equal(groups[0].severity,"Error");
  assert.deepEqual(groups[1].items.map(i=>i.line),[21,24,42]);
  assert.equal(groups[1].firstLine,21);
  assert.equal(original,JSON.stringify(values));
});
test("different severities of same diagnostic remain separate",()=>{
  const groups=groupDiagnostics([
    {severity:"Warning",code:"Same",line:1,message:"x"},
    {severity:"Error",code:"Same",line:2,message:"y"},
  ]);
  assert.equal(groups.length,2);
});
test("grouped view retains all 525 diagnostics",()=>{
  const input=Array.from({length:525},(_,n)=>({
    severity:"Warning" as const,code:"Repeated",line:n+1,message:"item "+n,
  }));
  const groups=groupDiagnostics(input);
  assert.equal(groups.length,1);
  assert.equal(groups[0].items.length,525);
  assert.equal(groups[0].items.at(-1)?.line,525);
});
