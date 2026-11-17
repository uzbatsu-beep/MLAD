import test from "node:test";
import assert from "node:assert/strict";
import {inlineOperandError} from "./inlineOperand.ts";

test("relay syntax accepts exactly five decimal digits and bit 0..7",()=>{
  assert.equal(inlineOperandError("#70010","relay"),null);
  assert.equal(inlineOperandError("#00007","relay"),null);
  assert.equal(inlineOperandError("  #70020  ","relay"),null);
  assert.ok(inlineOperandError("#70018","relay"));
  assert.ok(inlineOperandError("#70019","relay"));
  assert.ok(inlineOperandError("70010","relay"));
  assert.ok(inlineOperandError("#700100","relay"));
  assert.ok(inlineOperandError("#7001A","relay"));
});
test("8-bit GSTR/GOUT requires bit-zero byte address",()=>{
  assert.equal(inlineOperandError("#20010","group"),null);
  assert.equal(inlineOperandError("#00010","group"),null);
  assert.ok(inlineOperandError("#20011","group"));
  assert.ok(inlineOperandError("#20018","group"));
});
test("TMR/CNT current must be M000..M559",()=>{
  assert.equal(inlineOperandError("M000","current"),null);
  assert.equal(inlineOperandError("M559","current"),null);
  assert.ok(inlineOperandError("M560","current"));
  assert.ok(inlineOperandError("M999","current"));
  assert.ok(inlineOperandError("M1000","current"));
  assert.ok(inlineOperandError("m010","current"));
});
test("timer/counter presets permit unsigned u16 and M000..999",()=>{
  for(const value of ["0","100","65535","M000","M999"]){
    assert.equal(inlineOperandError(value,"preset"),null);
  }
  for(const value of ["65536","-1","1.5","M1000","X100"]){
    assert.ok(inlineOperandError(value,"preset"));
  }
});
test("empty, overlong or malformed input never silently mutates the draft",()=>{
  for(const value of ["","A".repeat(30),"$40010","str #70010"]){
    assert.ok(inlineOperandError(value,"relay"));
  }
});
