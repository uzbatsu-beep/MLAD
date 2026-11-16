import test from "node:test";
import assert from "node:assert/strict";
import {
  toLogicState, contactLogicState, expressionLogicState,
  nextManualSignal, STATE_COLORS,
} from "./visualState.ts";
import type { GraphExpr } from "./graphModel.ts";

const no = (id: string, relay: string): GraphExpr =>
  ({kind:"contact",id,relay,normallyClosed:false});
const nc = (id: string, relay: string): GraphExpr =>
  ({kind:"contact",id,relay,normallyClosed:true});

test("three-state palette is explicit and differs for unknown",()=>{
  assert.notEqual(STATE_COLORS.TRUE,STATE_COLORS.FALSE);
  assert.notEqual(STATE_COLORS.FALSE,STATE_COLORS.UNKNOWN);
  assert.equal(toLogicState(undefined),"UNKNOWN");
  assert.equal(toLogicState(null),"UNKNOWN");
  assert.equal(toLogicState(false),"FALSE");
  assert.equal(toLogicState(true),"TRUE");
});
test("NO and NC contact colors derive from conduction not the raw bit",()=>{
  const a={relay:"#70010",normallyClosed:false};
  const b={relay:"#70010",normallyClosed:true};
  assert.equal(contactLogicState(a,{}),"UNKNOWN");
  assert.equal(contactLogicState(b,{}),"UNKNOWN");
  assert.equal(contactLogicState(a,{"#70010":true}),"TRUE");
  assert.equal(contactLogicState(b,{"#70010":true}),"FALSE");
  assert.equal(contactLogicState(a,{"#70010":false}),"FALSE");
  assert.equal(contactLogicState(b,{"#70010":false}),"TRUE");
});
test("parallel rung evaluates OR including determinable values with unknown input",()=>{
  const expr:GraphExpr={kind:"or",id:"or",left:no("a","#70010"),right:nc("b","#70011")};
  assert.equal(expressionLogicState(expr,{}),"UNKNOWN");
  assert.equal(expressionLogicState(expr,{"#70010":true}),"TRUE");
  assert.equal(expressionLogicState(expr,{"#70010":false}),"UNKNOWN");
  assert.equal(expressionLogicState(expr,{"#70010":false,"#70011":true}),"FALSE");
  assert.equal(expressionLogicState(expr,{"#70010":false,"#70011":false}),"TRUE");
});
test("series rung false dominates unknown; unknown is not displayed as red",()=>{
  const expr:GraphExpr={kind:"and",id:"and",left:no("a","#70010"),right:no("b","#70011")};
  assert.equal(expressionLogicState(expr,{}),"UNKNOWN");
  assert.equal(expressionLogicState(expr,{"#70010":false}),"FALSE");
  assert.equal(expressionLogicState(expr,{"#70010":true}),"UNKNOWN");
  assert.equal(expressionLogicState(expr,{"#70010":true,"#70011":true}),"TRUE");
});
test("single input switch cycles ? 0 1 ? only on explicit operator action",()=>{
  const a=nextManualSignal(undefined),b=nextManualSignal(a),c=nextManualSignal(b);
  assert.equal(a,false);
  assert.equal(b,true);
  assert.equal(c,undefined);
});
test("deep chains terminate as unknown instead of locking UI",()=>{
  let expr:GraphExpr=no("root","#70010");
  for(let i=0;i<600;i++){
    expr={kind:"and",id:"join-"+i,left:expr,right:no("c-"+i,"#70011")};
  }
  assert.equal(expressionLogicState(expr,{}),"UNKNOWN");
});
