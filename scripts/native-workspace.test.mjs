import assert from "node:assert/strict";
import {readFileSync,existsSync} from "node:fs";
import test from "node:test";
const app=readFileSync("src/App.tsx","utf8");
const native=readFileSync("src/components/NativeWorkspace.tsx","utf8");
test("Native CIOPRG is the primary and default work area",()=>{
  assert.match(app,/useState<Workspace>\("native"\)/);
  assert.match(app,/Программа робота · CIOPRG\.LST/);
  assert.match(app,/<NativeWorkspace\s*\/>/);
  assert.doesNotMatch(app,/inspectorOpen/);
  assert.doesNotMatch(app,/CioprgInspector/);
});
test("Legacy JSON is an explicitly labelled compatibility workflow",()=>{
  assert.match(app,/Дополнительно/);
  assert.match(app,/Редактор JSON v1 \(совместимость\)/);
  assert.equal(existsSync("src/components/CioprgInspector.tsx"),false);
});
test("Source, diagrams and diagnostics share the main work area",()=>{
  assert.match(native,/aria-label="Дерево программы"/);
  assert.match(native,/aria-label="Рабочее поле CIOPRG"/);
  assert.match(native,/aria-label="Диагностика и поиск"/);
  assert.match(native,/cmd_search_cioprg/);
  assert.match(native,/cmd_read_cioprg_page/);
  assert.match(native,/expectedSha256:report\.source_sha256/);
  assert.doesNotMatch(native,/position:\s*"absolute".*inset:\s*0/);
});

test("every returned Ladder rung is rendered as its own diagram card",()=>{
  assert.match(native,/visibleRungs\.map\(\(\{rung:current,expr,error\}\)/);
  assert.match(native,/semantic\.rungs\.map/);
  assert.match(native,/LadderCircuit readOnly zoom=\{zoom\}/);
  assert.match(native,/semantic\.rungs_truncated/);
  assert.doesNotMatch(native,/activeRung&&graph/);
});
test("Advanced preview has independent zoom buttons and keeps native offline-only",()=>{
  assert.match(native,/setBlockZoom/);
  assert.match(native,/zoom=\{blockZoom\}/);
  assert.match(native,/TMR \/ CNT \/ GSTR/);
  assert.match(native,/onEditComment/);
  const advanced=readFileSync("src/components/AdvancedBlockDiagram.tsx","utf8");
  assert.match(advanced,/onEditComment/);
  assert.match(advanced,/zoom=1/);
});
test("IONAME is imported read-only, mapping is manual, comments use local storage",()=>{
  assert.match(native,/cmd_inspect_ioname/);
  assert.match(native,/ioname\.relay_mapping_verified|Автоматическая карта I\/O/);
  assert.match(native,/localStorage\.setItem/);
  assert.match(native,/onDoubleClick=.*beginEdit/);
  const backend=readFileSync("src-tauri/src/main.rs","utf8");
  assert.match(backend,/async fn cmd_inspect_ioname/);
  assert.match(backend,/MAX_IONAME_BYTES: u64 = 4 \* 1024 \* 1024/);
  const p=readFileSync("src-tauri/src/compiler/ioname.rs","utf8");
  assert.match(p,/relay_mapping_verified:false/);
  assert.match(p,/native_write_allowed:false/);
});
