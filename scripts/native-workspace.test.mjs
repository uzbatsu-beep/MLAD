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
