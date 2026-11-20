import assert from "node:assert/strict";
import {readFileSync,existsSync} from "node:fs";
import test from "node:test";
import {createHash} from "node:crypto";
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

test("Ladder cards are read-only and selected from the bounded semantic preview",()=>{
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

test("extended arithmetic and shift opcodes are typed read-only and separately counted",()=>{
  assert.match(native,/getTypedCandidate/);
  assert.match(native,/TypedCandidatePreview/);
  assert.match(native,/typed_candidate_count/);
  assert.match(native,/typed_valid_forms/);
  assert.match(native,/typed_invalid_forms/);
  assert.match(native,/typed_mnemonic_counts/);
  assert.match(native,/Типизировано\*/);
  assert.match(native,/Не участвует в реконструкции Ladder/);
  const rust=readFileSync("src-tauri/src/compiler/lossless.rs","utf8");
  assert.match(rust,/semantics_verified:false/);
  assert.match(rust,/controller_form_verified/);
  assert.match(rust,/"MOV"/);
  assert.match(rust,/"ADD" \| "SUB"/);
  assert.match(rust,/"SHL" \| "SHR"/);
});

test("dataflow evidence is source-order-only and always unverified",()=>{
  assert.match(native,/dataflow:\s*StaticDependencyAudit/);
  assert.match(native,/report\.dataflow\.source_sha256!==report\.source_sha256/);
  assert.match(native,/aria-label="Статический аудит регистров"/);
  assert.match(native,/possible_previous_definition/);
  assert.match(native,/Только гипотезы|только гипотезы/);
  assert.match(native,/порядок сканирования и арифметические флаги не подтверждены/);
  assert.match(native,/report\.dataflow\.events_truncated/);
  assert.match(native,/showLine\(site\.possible_previous_definition!/);
  const rust=readFileSync("src-tauri/src/compiler/typed_dataflow.rs","utf8");
  assert.match(rust,/semantics_verified:false/);
  assert.match(rust,/scan_order_verified:false/);
  assert.match(rust,/arithmetic_flags_verified:false/);
  assert.match(rust,/native_write_allowed:false/);
  assert.match(rust,/definitions\.clear\(\)/);
  assert.match(rust,/MAX_STATIC_EVENTS_PREVIEW: usize = 1000/);
  assert.match(rust,/PostEndOpaque/);
  const main=readFileSync("src-tauri/src/main.rs","utf8");
  assert.match(main,/typed_dataflow::inspect\(&doc, 1000\)/);
});

test("OEM manual source panel never claims runtime verification",()=>{
  assert.match(native,/oemManualEntry\(selectedTyped\.instruction\.mnemonic\)/);
  assert.match(native,/Справка OEM/);
  assert.match(native,/не результат выполнения/);
  assert.match(native,/EN, выходные данные и переходы флагов между командами не моделируются/);
  assert.match(native,/Для выбранного семейства форма отдельно не подтверждена документацией/);
  const rust=readFileSync("src-tauri/src/compiler/reference_semantics.rs","utf8");
  assert.match(rust,/fn evaluate_reference/);
  assert.match(rust,/controller_execution_verified:false/);
  assert.match(rust,/native_write_allowed:false/);
  assert.doesNotMatch(rust,/tauri::command|cmd_deploy_to_robot|TcpStream|UdpSocket/);
});

test("synthetic CIOPRG manifest hashes and provenance are consistent",()=>{
  const corpus=JSON.parse(readFileSync(
    "src-tauri/src/compiler/lossless/fixtures/synthetic_cases.json","utf8"));
  assert.equal(corpus.schema_version,1);
  assert.equal(corpus.origin,"SYNTHETIC");
  assert.ok(corpus.cases.length>=12);
  assert.equal(new Set(corpus.cases.map(c=>c.id)).size,corpus.cases.length);
  for(const f of corpus.cases){
    assert.match(f.id,/^F\d+$/);
    assert.match(f.source_hex,/^(?:[0-9a-f]{2})*$/);
    const bytes=Buffer.from(f.source_hex,"hex");
    assert.equal(bytes.toString("hex"),f.source_hex);
    assert.equal(createHash("sha256").update(bytes).digest("hex"),f.sha256,f.id);
    assert.ok(["DX100","DX200","YRC1000"].includes(f.controller));
    assert.ok(Array.isArray(f.expected_classes));
  }
});
test("Native Workspace displays syntax-only counts and exact source byte offsets",()=>{
  const rust=readFileSync("src-tauri/src/compiler/lossless.rs","utf8");
  assert.match(rust,/pub struct SyntaxCoverage/);
  assert.match(rust,/all_source_lines_counted: true/);
  assert.match(rust,/semantics_verified: false/);
  assert.match(rust,/syntax_class: SyntaxClass::of\(&l.kind\)/);
  assert.match(native,/syntax_coverage: SyntaxCoverage/);
  assert.match(native,/Покрытие синтаксиса/);
  assert.match(native,/l\.body_span\.start/);
  assert.match(native,/l\.body_span\.end/);
  assert.doesNotMatch(native,/title=\{"Byte offset: "\+l\.line\}/);
  assert.match(native,/Только синтаксис; исполнение OEM не проверено/);
});

test("Native Workspace shows exactly one selected rung or block",()=>{
  assert.match(native,/semantic\.rungs\.filter\(sourceRung=>sourceRung\.number===rung\)/);
  assert.match(native,/\(activeBlock\?\[activeBlock\]:\[\]\)\.map/);
  assert.match(native,/setSelectedBlockLine/);
  assert.match(native,/aria-current=\{rung===r\.number/);
  assert.match(native,/aria-current=\{selectedBlockLine===b\.line/);
  assert.match(native,/sections\.slice\(0,160\)/);
  assert.match(native,/К исходной строке/);
});
test("Native source cross-navigation and keyboard remain physical-line based",()=>{
  assert.match(native,/selectedRung=useMemo/);
  assert.match(native,/selectedRelatedBlock=useMemo/);
  assert.match(native,/К цепи R/);
  assert.match(native,/К блоку L/);
  assert.match(native,/onKeyDown=\{event=>\{/);
  for(const key of ["ArrowDown","ArrowUp","PageDown","PageUp","Home","End"]){
    assert.ok(native.includes('event.key==="'+key+'"'),key);
  }
  assert.match(native,/activeSourceLineRef\.current\?\.focus\(\)/);
  assert.match(native,/body_span\.start/);
});
test("Full-source opcode/address/class filter uses SHA-pinned bounded read-only pages",()=>{
  const main=readFileSync("src-tauri/src/main.rs","utf8");
  const filter=readFileSync("src-tauri/src/native_filter.rs","utf8");
  assert.match(main,/async fn cmd_filter_cioprg_lines/);
  assert.match(main,/expected_sha256/);
  assert.match(main,/read_cioprg_bytes\(&file_path\)\.await/);
  assert.match(main,/cmd_filter_cioprg_lines,/);
  assert.match(native,/invoke<NativeFilterPage>\("cmd_filter_cioprg_lines"/);
  assert.match(native,/expectedSha256:report\.source_sha256/);
  assert.match(native,/controller:report\.controller/);
  assert.match(native,/sourceFilterResult\.next_offset/);
  assert.match(native,/Фильтрация всего CIOPRG/);
  assert.match(native,/Фильтр класса синтаксиса/);
  assert.match(native,/Фильтр opcode/);
  assert.match(native,/Фильтр адреса/);
  assert.match(filter,/const FILTER_PAGE_SIZE: usize = 100/);
  assert.match(filter,/MAX_SOURCE_LINES: usize = 200_000/);
  assert.match(filter,/fn contains_token/);
  assert.match(filter,/pub fn filter_source/);
  assert.doesNotMatch(filter,/TcpStream|UdpSocket|tauri::command|write\(|rename\(/);
});

test("portable MLAD sidecar v1 is explicit, source-bound and never a native export",()=>{
  const main=readFileSync("src-tauri/src/main.rs","utf8");
  const schema=readFileSync("src-tauri/src/sidecar.rs","utf8");
  assert.match(main,/cmd_load_offline_sidecar,/);
  assert.match(main,/cmd_save_offline_sidecar,/);
  assert.match(main,/sidecar::check_sidecar_path\(&file_path\)/);
  assert.match(main,/sidecar\.validate\(&doc\)\?/);
  assert.match(main,/read_cioprg_bytes\(&source_path\)\.await\?/);
  assert.match(main,/expected_source_sha256/);
  assert.match(main,/name\.ends_with\("\.mlad\.json"\)/);
  assert.match(schema,/MLAD_OFFLINE_SIDECAR/);
  assert.match(schema,/pub schema_version: u32/);
  assert.match(schema,/pub sha256: String/);
  assert.match(schema,/pub line_count: usize/);
  assert.match(schema,/pub bookmarks: Vec<Bookmark>/);
  assert.match(schema,/deny_unknown_fields/);
  assert.match(schema,/fn invalid_notes_bookmarks_and_unknown_fields_fail_closed/);
  assert.doesNotMatch(schema,/write\(|rename\(|TcpStream|UdpSocket|tauri::command/);
});
test("Native Workspace explicitly imports and exports portable notes and bookmarks",()=>{
  assert.match(native,/invoke<OfflineSidecarRead>\("cmd_load_offline_sidecar"/);
  assert.match(native,/invoke<OfflineSidecarSaved>\("cmd_save_offline_sidecar"/);
  assert.match(native,/expectedSourceSha256:report\.source_sha256/);
  assert.match(native,/expectedRevision:sidecarRevision/);
  assert.match(native,/setSidecarDirty\(true\)/);
  assert.match(native,/setSidecarRevision\(null\)/);
  assert.match(native,/nativeRequestEpoch\.current/);
  assert.match(native,/Загрузить и заменить несохранённые заметки/);
  assert.match(native,/Переносимый проект/);
  assert.match(native,/Путь к MLAD sidecar/);
  assert.match(native,/Добавить закладку/);
  assert.match(native,/Удалить закладку L/);
  assert.match(native,/\.replace\(\/\\\.lst\$\/i,"\.mlad\.json"\)/);
  assert.doesNotMatch(native,/invoke.*cmd_deploy_to_robot/);
});

test("sidecar UX never loses unsaved notes during source switching or concurrent saves",()=>{
  assert.match(native,/if\(report&&sidecarDirty\)/);
  assert.match(native,/if\(sidecarBusy\)\s*\{\s*setIssue\("Сначала дождитесь/);
  assert.match(native,/setSavedSidecarNotes\(\{annotations:cached,bookmarks:\[\]\}\)/);
  assert.match(native,/setSavedSidecarNotes\(\{annotations:payload\.annotations,bookmarks:payload\.bookmarks\}\)/);
  assert.match(native,/function discardUnsavedSidecarNotes\(/);
  assert.match(native,/setAnnotations\(savedSidecarNotes\.annotations\)/);
  assert.match(native,/Отменить несохранённые изменения sidecar/);
  assert.match(native,/if\(!report \|\| !editing\|\|sidecarBusy\)return/);
  assert.match(native,/if\(sidecarBusy\)return;/);
});

test("PR46 source-linked READ/WRITE/UNKNOWN UX has no runtime access claims",()=>{
  assert.match(native,/static_access: StaticAccessOverview/);
  assert.match(native,/type StaticAccessSite =/);
  assert.match(native,/aria-label="READ WRITE UNKNOWN — статический аудит"/);
  assert.match(native,/cmd_query_static_access/);
  assert.match(native,/expectedSha256:report\.source_sha256/);
  assert.match(native,/page\.source_sha256!==report\.source_sha256/);
  assert.match(native,/page\.hits\.some\(hit=>hit\.line<1/);
  assert.match(native,/accessRequestSeq\.current\+\+/);
  assert.match(native,/READ и WRITE — предполагаемые роли адресов по руководству OEM/);
  assert.match(native,/execution_verified/);
  assert.match(native,/write_permission_verified/);
  assert.match(native,/span\.start/);
  assert.match(native,/showLine\(site\.line\)/);
  assert.match(native,/UnknownOpcodeOrStructure/);
  assert.match(native,/skipped_literal_operands/);
  assert.match(native,/post_end_lines/);
  assert.match(native,/next_offset/);
  assert.doesNotMatch(native,/invoke.*cmd_deploy_to_robot/);
});
