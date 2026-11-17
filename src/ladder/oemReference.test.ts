import test from "node:test";
import assert from "node:assert/strict";
import {oemManualEntry} from "./oemReference.ts";
test("all five typed operations have independent manual evidence",()=>{
  for(const name of ["MOV","ADD","SUB","SHL","SHR"]){
    const item=oemManualEntry(name);
    assert.ok(item,name);
    assert.match(item.format,new RegExp("^"+name+" "));
    assert.ok(item.section.startsWith("§12.2."));
    assert.ok(item.carry.length>15);
    assert.ok(item.unresolved.length>15);
  }
  assert.equal(oemManualEntry("MOVJ"),null);
  assert.equal(oemManualEntry("BMOV"),null);
});
test("unsafe edge cases are not disguised as confirmed runtime results",()=>{
  assert.match(oemManualEntry("MOV")!.unresolved,/256/);
  assert.match(oemManualEntry("SHL")!.unresolved,/n=0/);
  assert.match(oemManualEntry("SHR")!.unresolved,/n=16/);
});
