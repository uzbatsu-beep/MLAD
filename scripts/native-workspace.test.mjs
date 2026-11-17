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

test("file choosers are compact, native and read-only",()=>{
  assert.match(native,/className="native-compact-header"/);
  assert.match(native,/aria-label="Выбрать файл CIOPRG\.LST"/);
  assert.match(native,/aria-label="Выбрать файл IONAME\.DAT"/);
  assert.match(native,/invoke<string\|null>\("cmd_pick_native_file",\{kind\}\)/);
  assert.match(native,/if\(!chosen\)return/);
  assert.match(native,/await openSource\(chosen\)/);
  assert.match(native,/await importIoname\(chosen\)/);
  assert.doesNotMatch(native,/<form[^>]+onSubmit[^>]*>\s*<label[^>]+Путь к копии/);
  const css=readFileSync("src/components/native-workspace.css","utf8");
  assert.match(css,/@media \(max-width:1060px\)/);
  assert.match(css,/\.native-compact-header\s*\{/);
  const backend=readFileSync("src-tauri/src/main.rs","utf8");
  assert.match(backend,/async fn cmd_pick_native_file/);
  assert.match(backend,/\.plugin\(tauri_plugin_dialog::init\(\)\)/);
  assert.match(backend,/file\(\)\.set_title\(title\)\.add_filter\(title, &\[extension\]\)/);
  assert.doesNotMatch(backend,/blocking_save_file\(|blocking_pick_file\(/);
});
test("contextual scale controls share the tab toolbar and remain independent",()=>{
  const nav=native.slice(native.indexOf('<nav className="native-context-toolbar"'),
    native.indexOf('</nav>',native.indexOf('<nav className="native-context-toolbar"')));
  assert.match(nav,/native-context-tabs/);
  assert.match(nav,/tab==="diagram"/);
  assert.match(nav,/tab==="blocks"/);
  assert.match(nav,/setZoom\(/);
  assert.match(nav,/setBlockZoom\(/);
  assert.match(nav,/Все цепи/);
  assert.match(nav,/Все блоки/);
  const body=native.slice(native.indexOf('</nav>',native.indexOf('<nav className="native-context-toolbar"'))+6);
  const ladder=body.slice(body.indexOf('{tab==="diagram"&&<div'),
    body.indexOf('{tab==="blocks"&&<div'));
  assert.doesNotMatch(ladder,/setZoom\(/);
  const blocks=body.slice(body.indexOf('{tab==="blocks"&&<div'),
    body.indexOf('{tab==="ioname"&&<div'));
  assert.doesNotMatch(blocks,/setBlockZoom\(/);
});

test("post-END categories are explicit; diagnostic grouping keeps navigation",()=>{
  assert.match(native,/post_end_line_count/);
  assert.match(native,/PostEndSection/);
  assert.match(native,/PostEndData/);
  assert.match(native,/PostEndOpaque/);
  assert.match(native,/groupedDiagnostics/);
  assert.match(native,/groupDiagnostics\(diags\)/);
  assert.match(native,/Группировать по коду/);
  assert.match(native,/g\.items\.map/);
});
