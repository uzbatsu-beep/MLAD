import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import test from "node:test";

const rust = readFileSync("src-tauri/src/main.rs", "utf8");

// Static defense-in-depth, backed by direct Rust tests for the IPC return paths.
test("offline backend contains no compiled unverified robot network module", () => {
  assert.doesNotMatch(rust, /\b(?:pub\s+)?mod\s+network\s*;/);
  assert.doesNotMatch(rust, /\bAppNetworkState\b/);
  assert.equal(existsSync("src-tauri/src/network/mod.rs"), false);
});
test("monitoring and deployment remain deny-only Tauri IPC commands", () => {
  assert.match(rust, /async fn cmd_toggle_monitoring\s*\(/);
  assert.match(rust, /async fn cmd_deploy_to_robot\s*\(/);
  assert.match(rust, /Мониторинг HSES отключён/);
  assert.match(rust, /Загрузка на робот отключена/);
  assert.match(rust, /cmd_toggle_monitoring\s*,\s*cmd_deploy_to_robot/);
});
test("native CIO inspection remains bounded and read-only", () => {
  assert.match(rust, /MAX_CIO_FILE_BYTES: u64 = 16 \* 1024 \* 1024/);
  assert.match(readFileSync("src-tauri/src/engineering/mod.rs","utf8"), /inspection_report\(600, 500\)/);
});


test("all JSON save entrypoints use staged replacement, not truncating direct writes",()=>{
  const persist=readFileSync("src-tauri/src/draft_io.rs","utf8");
  assert.match(rust,/mod draft_io;/);
  assert.equal((rust.match(/draft_io::write_atomic\(&target,/g)||[]).length,3);
  assert.match(persist,/create_new\(true\)/);
  assert.match(persist,/handle\.write_all\(content\)/);
  assert.match(persist,/handle\.sync_all\(\)/);
  assert.match(persist,/fs::rename\(&self\.path,destination\)/);
  assert.match(persist,/fn direct_symlink_target_and_backup_are_rejected/);
  assert.doesNotMatch(rust,/tokio::fs::write\(path,/);
  assert.doesNotMatch(rust,/std::fs::write\(&file_path,/);
  assert.match(rust,/cmd_toggle_monitoring\s*,\s*cmd_deploy_to_robot/);
});


test("Tauri production CSP is explicit, offline and does not enable inline/eval scripts", () => {
  const config=JSON.parse(readFileSync("src-tauri/tauri.conf.json","utf8"));
  const csp=config.app?.security?.csp;
  assert.ok(csp && typeof csp==="object","production CSP must not be null");
  assert.equal(csp["default-src"],"'self'");
  assert.equal(csp["script-src"],"'self'");
  assert.ok(!csp["script-src"].includes("unsafe-inline"));
  assert.ok(!csp["script-src"].includes("unsafe-eval"));
  assert.equal(csp["object-src"],"'none'");
  assert.equal(csp["frame-src"],"'none'");
  assert.equal(csp["base-uri"],"'none'");
  assert.equal(csp["form-action"],"'none'");
  assert.equal(csp["worker-src"],"'none'");
  assert.match(csp["connect-src"],/\bipc:/);
  assert.match(csp["connect-src"],/http:\/\/ipc\.localhost/);
  assert.doesNotMatch(csp["connect-src"],/\bhttps?:\/\/(?!ipc\.localhost)|\bwss?:\/\//);
  assert.equal(csp["style-src"],"'self' 'unsafe-inline'",
    "Legacy React inline styles require a documented exception");
  assert.ok(!JSON.stringify(csp).includes("'unsafe-eval'"));
  assert.ok(!JSON.stringify(csp).includes("*"));
});

test("Vite development CSP only opens localhost HMR and native IPC", () => {
  const config=JSON.parse(readFileSync("src-tauri/tauri.conf.json","utf8"));
  const dev=config.app.security.devCsp;
  assert.ok(dev);
  assert.match(dev["connect-src"],/http:\/\/localhost:5173/);
  assert.match(dev["connect-src"],/ws:\/\/localhost:5173/);
  assert.match(dev["connect-src"],/http:\/\/ipc\.localhost/);
  assert.equal(dev["object-src"],"'none'");
  assert.doesNotMatch(dev["script-src"],/unsafe-eval|unsafe-inline/);
  assert.doesNotMatch(dev["connect-src"],/\bhttps:\/\/|\bwss:\/\//);
});

test("native file reads and every JSON persistence entry check parent paths", () => {
  const storage=readFileSync("src-tauri/src/draft_io.rs","utf8");
  assert.match(storage,/pub fn verify_parent_directory/);
  assert.match(storage,/Component::ParentDir/);
  assert.match(storage,/crate::guarded_read::is_indirection\(&meta\)/);
  assert.match(storage,/crate::guarded_read::is_indirection\(&metadata\)/);
  assert.match(storage,/fn symlinked_parent_directory_cannot_redirect_json_into_native_folder/);
  assert.match(storage,/fn relative_parent_traversal_rejected_before_any_write/);
  assert.match(rust,/draft_io::verify_parent_directory\(path\)/);
  assert.match(rust,/tokio::fs::symlink_metadata\(path\)/);
  assert.match(rust,/fn read_only_native_file_through_symlinked_parent_is_rejected/);
  assert.match(rust,/fn read_only_native_direct_symlink_is_rejected/);
  assert.doesNotMatch(rust,/pub mod network;/);
});


test("PR29 JSON persistence rejects hardlinks and same-content file swaps", () => {
  const storage=readFileSync("src-tauri/src/draft_io.rs","utf8");
  const guarded=readFileSync("src-tauri/src/guarded_read.rs","utf8");
  assert.match(guarded,/pub\(crate\) fn read_regular_with_identity/);
  assert.match(guarded,/pub\(crate\) fn verify_single_link/);
  assert.match(guarded,/GetFileInformationByHandle/);
  assert.match(guarded,/meta\.nlink\(\) != 1/);
  assert.match(guarded,/links != 1/);
  assert.match(storage,/read_bounded_snapshot\(path\)/);
  assert.match(storage,/fn reject_windows_special_path/);
  assert.match(storage,/reject_windows_special_path\(path\)\?/);
  assert.match(storage,/crate::guarded_read::verify_single_link\(path\)/);
  assert.match(storage,/identity!=self\.identity/);
  assert.match(storage,/Sha256::digest/);
  assert.match(storage,/fn same_content_target_replacement_is_a_save_conflict/);
  assert.match(storage,/fn hardlink_alias_to_native_source_or_json_backup_is_rejected/);
  assert.match(storage,/fn staged_file_modified_after_flush_cannot_be_committed/);
  assert.match(rust,/guarded_read::is_indirection\(&meta\)/);
});

test("PR26 native and JSON reads use one handle with filesystem identity checks",()=>{
  const safe=readFileSync("src-tauri/src/guarded_read.rs","utf8");
  const drafts=readFileSync("src-tauri/src/draft_io.rs","utf8");
  assert.match(rust,/mod guarded_read;/);
  assert.equal((rust.match(/guarded_read::read_regular\(&selected,/g)||[]).length,2);
  assert.match(drafts,/guarded_read::read_regular_with_identity\(path,MAX_JSON_BYTES\)/);
  assert.match(safe,/GetFileInformationByHandle/);
  assert.match(safe,/FILE_FLAG_OPEN_REPARSE_POINT/);
  assert.match(safe,/win_file_id\(&before_win\)/);
  assert.match(safe,/win_file_id\(&after_win\)/);
  assert.match(safe,/same_identity\(&before,&opened\)/);
  assert.match(safe,/is_indirection\(&after_path\)/);
  assert.match(safe,/FILE_ATTRIBUTE_REPARSE_POINT/);
  assert.match(safe,/substituted_file_between_metadata_and_open_fails_closed/);
  assert.match(drafts,/Конфликт сохранения: JSON изменён другим процессом/);
  assert.match(drafts,/Конфликт сохранения: другой процесс создал JSON/);
  assert.match(drafts,/external_change_between_backup_and_commit_cannot_be_overwritten/);
  assert.doesNotMatch(rust,/tokio::fs::read\(path\)/);
});

test("unused FTP dependency and its transitive graph are absent from locked Rust build",()=>{
  const manifest=readFileSync("src-tauri/Cargo.toml","utf8");
  const lockfile=readFileSync("src-tauri/Cargo.lock","utf8");
  assert.doesNotMatch(manifest,/^suppaftp\s*=/m);
  assert.doesNotMatch(lockfile,/^name = "suppaftp"$/m);
  for(const name of ["lazy-regex","lazy-regex-proc_macros"]){
    assert.ok(!lockfile.includes('name = "'+name+'"'),name);
  }
  assert.match(rust,/Мониторинг HSES отключён/);
  assert.match(rust,/Загрузка на робот отключена/);
});
test("JSON stages are private on Unix, single-link before rename and identity-safe on cleanup",()=>{
  const draft=readFileSync("src-tauri/src/draft_io.rs","utf8");
  assert.match(draft,/options\.mode\(0o600\)/);
  assert.match(draft,/verify_single_link\(&self\.path\)/);
  assert.match(draft,/identity_of_handle\(&file\)/);
  assert.match(draft,/unix_staged_json_and_backup_are_owner_only_even_with_permissive_umask/);
  assert.match(draft,/staged_hardlink_is_rejected_before_rename/);
  assert.match(draft,/staged_cleanup_preserves_foreign_file_swapped_into_temp_path/);
});

test("unused direct anyhow and futures-util crates are absent while nom remains required",()=>{
  const manifest=readFileSync("src-tauri/Cargo.toml","utf8");
  const parser=readFileSync("src-tauri/src/compiler/parser.rs","utf8");
  assert.doesNotMatch(manifest,/^anyhow\s*=/m);
  assert.doesNotMatch(manifest,/^futures-util\s*=/m);
  assert.match(manifest,/^nom\s*=/m);
  assert.match(parser,/use nom::/);
});

test("offline sidecar only writes independent JSON with source pinned and conflict checks",()=>{
  const main=readFileSync("src-tauri/src/main.rs","utf8");
  const draft=readFileSync("src-tauri/src/draft_io.rs","utf8");
  const sidecar=readFileSync("src-tauri/src/sidecar.rs","utf8");
  assert.match(main,/fn cmd_load_offline_sidecar\(/);
  assert.match(main,/fn cmd_save_offline_sidecar\(/);
  assert.match(main,/expected_source_sha256/);
  assert.match(main,/cmd_save_offline_sidecar[\s\S]*?draft_io::write_atomic_with_revision/);
  assert.match(main,/name\.ends_with\("\.mlad\.json"\)/);
  assert.match(sidecar,/filename\.ends_with\("\.mlad\.json"\)/);
  assert.match(draft,/read_bounded_with_revision/);
  assert.match(draft,/write_atomic_with_revision/);
  assert.match(draft,/revision_of\(bytes:&\[u8\],id:/);
  assert.match(draft,/RevisionPolicy::Expected\(expected\)/);
  assert.match(draft,/if revision_of\(bytes,id\)!=rev/);
  assert.match(draft,/write_atomic_inner_with_policy\(/);
  assert.match(draft,/fn sidecar_revision_is_checked_before_existing_backup_is_modified/);
  assert.match(draft,/fn changed_sidecar_does_not_get_silently_overwritten_or_backed_up/);
  assert.match(draft,/fn revision_detects_identical_byte_file_identity_swap/);
  assert.doesNotMatch(sidecar,/cmd_deploy_to_robot|TcpStream|UdpSocket/);
});

test("PR46 static access is bounded, SHA-pinned, and distinct from a controller I/O writer",()=>{
  const parser=readFileSync("src-tauri/src/compiler/static_access.rs","utf8");
  const compiler=readFileSync("src-tauri/src/compiler/mod.rs","utf8");
  assert.match(compiler,/pub mod static_access;/);
  assert.match(readFileSync("src-tauri/src/engineering/mod.rs","utf8"),/static_access: static_access::AccessOverview/);
  assert.match(readFileSync("src-tauri/src/engineering/mod.rs","utf8"),/static_access::inspect\(doc,160\)/);
  assert.match(rust,/build_full_inspection\(&doc\)/);
  assert.match(rust,/async fn cmd_query_static_access\(/);
  assert.match(rust,/if doc\.source_sha256!=expected_sha256/);
  assert.match(rust,/compiler::static_access::query\(/);
  assert.match(parser,/pub enum AccessKind \{ Read, Write, Unknown \}/);
  assert.match(parser,/pub enum AccessBasis/);
  assert.match(parser,/MAX_PREVIEW:usize=160/);
  assert.match(parser,/PAGE_SIZE:usize=100/);
  assert.match(parser,/matches!\(kind,OperandKind::RelayBit/);
  assert.match(parser,/LineKind::PostEndData\|LineKind::PostEndOpaque/);
  assert.match(parser,/execution_verified:false/);
  assert.match(parser,/write_permission_verified:false/);
  assert.match(parser,/native_write_allowed:false/);
  assert.match(parser,/fn query_paginates_full_source_and_filters_exact_symbol/);
  assert.doesNotMatch(parser,/TcpStream|UdpSocket|tauri::command|write_atomic|File::create/);
});

test("PR47 portable unified project links only safe relative files, never OEM outputs",()=>{
  const main=readFileSync("src-tauri/src/main.rs","utf8");
  const project=readFileSync("src-tauri/src/offline_project.rs","utf8");
  assert.match(main,/mod offline_project;/);
  assert.match(main,/fn cmd_open_offline_project\(/);
  assert.match(main,/fn cmd_save_offline_project\(/);
  assert.match(main,/cmd_open_offline_project,/);
  assert.match(main,/cmd_save_offline_project,/);
  assert.match(main,/offline_project::check_project_path\(&path\)/);
  assert.match(main,/offline_project::resolve\(&path,/);
  assert.match(main,/offline_project::relative_file\(&path,/);
  assert.match(main,/draft_io::read_bounded_with_revision\(&path\)/);
  assert.match(main,/draft_io::write_atomic_with_revision\(/);
  assert.match(main,/cmd_open_offline_project[\s\S]*?project\.validate\(&doc\)/);
  assert.match(main,/offline_project::matches_attachment\(/);
  assert.match(main,/let analysis_changed=project\.analysis!=offline_project::snapshot\(&doc\)/);
  assert.match(main,/name\.ends_with\("\.mladproj\.json"\)/);
  assert.match(project,/MLAD_OFFLINE_PROJECT/);
  assert.match(project,/SOURCE_ROLE_PREVIEW_V1/);
  assert.match(project,/\#\[serde\(deny_unknown_fields\)\]/);
  assert.match(project,/checked_relative/);
  assert.match(project,/Component::Normal/);
  assert.match(project,/pub fn check_project_path/);
  assert.match(project,/pub fn resolve/);
  assert.match(project,/pub fn snapshot/);
  assert.match(project,/pub fn matches_attachment/);
  assert.match(project,/execution_verified:false/);
  assert.match(project,/fn corrupted_source_and_unverified_execution_are_rejected/);
  assert.match(main,/fn complete_project_roundtrip_recomputes_and_verifies_all_sources/);
  assert.doesNotMatch(project,/TcpStream|UdpSocket|File::create|std::fs::write|cmd_deploy_to_robot|\bHSES\s*\(/);
});
