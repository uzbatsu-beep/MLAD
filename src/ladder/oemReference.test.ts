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
  assert.equal(oemManualEntry("MOVL"),null);
});
test("unsafe edge cases are not disguised as confirmed runtime results",()=>{
  assert.match(oemManualEntry("MOV")!.unresolved,/256/);
  assert.match(oemManualEntry("SHL")!.unresolved,/n=0/);
  assert.match(oemManualEntry("SHR")!.unresolved,/n=16/);
});

test("YRC1000 R12 expanded grammar has evidence for 15 new shapes, not runtime permission",()=>{
  const cases=[
    ["MUL","§12.2.19"],["DIV","§12.2.20"],["MOD","§12.2.21"],
    ["BIN","§12.2.22"],["BCD","§12.2.23"],
    ["WAND","§12.2.25"],["WOR","§12.2.26"],["WXOR","§12.2.27"],
    ["WNOT","§12.2.28"],["ROL","§12.2.31"],["ROR","§12.2.32"],
    ["BMOV","§12.2.33"],["CMP","§12.2.34"],["PLS","§12.2.15"],["PLF","§12.2.16"],
  ];
  for(const [mnemonic,section] of cases) {
    const item=oemManualEntry(mnemonic);
    assert.ok(item,mnemonic);
    assert.equal(item.section,section);
    assert.match(item.format,new RegExp("^"+mnemonic+" "));
    assert.match(item.carry,/не вычисляет/);
    assert.match(item.unresolved,/прошивка не проверены/);
  }
  assert.equal(oemManualEntry("UNKNOWN-OP"),null);
  assert.match(oemManualEntry("MUL")!.limits,/M598/);
  assert.match(oemManualEntry("BMOV")!.limits,/1–999 байт/);
  assert.match(oemManualEntry("CMP")!.format,/S1,S2$/);
});
