import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

const read = (path) => readFileSync(path, "utf8");
const config = JSON.parse(read("src-tauri/tauri.conf.json"));
const backend = read("src-tauri/src/main.rs");
const draft = read("src-tauri/src/draft_io.rs");
const guarded = read("src-tauri/src/guarded_read.rs");
const workflow = read(".github/workflows/windows-security-acceptance.yml");
const caller = read(".github/workflows/validation.yml");
const harness = read("scripts/windows-acceptance.ps1");

test("Windows regression is invoked from the PR/main CI gate, with minimal token permissions", () => {
  assert.match(caller, /^\s*pull_request:/m);
  assert.match(caller, /^\s*push:/m);
  assert.match(caller, /uses:\s*\.\/\.github\/workflows\/windows-security-acceptance\.yml/);
  assert.match(caller, /if:\s*needs\.scope\.outputs\.runtime_required == 'true'/);
  assert.match(caller, /^  ci-gate:/m);
  assert.match(caller, /needs:\s*\[scope, frontend-build, rust-tests, windows-security\]/);
  assert.match(workflow, /^\s*workflow_call:/m);
  assert.match(workflow, /^\s*workflow_dispatch:/m);
  assert.doesNotMatch(workflow, /^\s*(?:pull_request|push):/m);
  assert.match(workflow, /runs-on:\s*windows-2022/);
  assert.match(workflow, /permissions:\s*\n\s*contents:\s*read/);
  assert.doesNotMatch(workflow, /pull_request_target:|contents:\s*write/);
  assert.match(workflow, /cargo test --manifest-path src-tauri\/Cargo.toml --locked/);
  assert.match(workflow, /npm run tauri -- build --no-bundle/);
  assert.match(workflow, /windows-acceptance\.ps1/);
  assert.doesNotMatch(workflow, /gh release create|softprops\/action-gh-release|git push/);
});

test("production CSP cannot request network or arbitrary script execution", () => {
  const prod = config.app.security.csp;
  const dev = config.app.security.devCsp;
  assert.equal(prod["script-src"], "'self'");
  assert.equal(prod["default-src"], "'self'");
  assert.equal(prod["object-src"], "'none'");
  assert.equal(prod["frame-src"], "'none'");
  assert.equal(prod["connect-src"], "'self' ipc: http://ipc.localhost");
  assert.doesNotMatch(JSON.stringify(prod), /unsafe-eval|https:\/\/|wss?:\/\/|\*/);
  assert.match(dev["connect-src"], /http:\/\/localhost:5173/);
  assert.match(dev["connect-src"], /ws:\/\/localhost:5173/);
  assert.doesNotMatch(prod["connect-src"], /localhost:5173/);
});

test("monitoring enable and controller deployment remain deny-only IPC paths", () => {
  const monitor = backend.match(/async fn cmd_toggle_monitoring\([^]*?(?=\n\s*\/\/\/ Команда Tauri: Деплой)/)?.[0];
  const deploy = backend.match(/async fn cmd_deploy_to_robot\([^]*?(?=\nfn main\()/)?.[0];
  assert.ok(monitor, "monitoring command absent");
  assert.ok(deploy, "deployment command absent");
  assert.match(monitor, /if active\s*\{\s*Err\(/);
  assert.match(deploy, /Err\("Загрузка на робот отключена/);
  assert.doesNotMatch(backend, /\bmod\s+network\s*;/);
  assert.doesNotMatch(backend, /std::net::TcpStream|tokio::net::TcpStream/);
});

test("Windows tests must exercise NTFS exclusive lock, junctions and read-only byte identity", () => {
  assert.match(draft, /fn windows_exclusive_lock_refuses_write_without_truncation_or_temp_leak/);
  assert.match(draft, /share_mode\(0\)/);
  assert.match(draft, /fn windows_junction_parent_never_writes_into_target_directory/);
  assert.match(guarded, /fn windows_junction_parent_rejects_native_read_and_preserves_bytes/);
  assert.match(guarded, /FILE_FLAG_OPEN_REPARSE_POINT/);
  assert.match(guarded, /GetFileInformationByHandle/);
  assert.match(draft, /external_change_between_backup_and_commit_cannot_be_overwritten/);
  assert.match(draft, /competing_new_file_creation_is_not_overwritten/);
  assert.match(backend, /cmd_toggle_monitoring\s*,\s*cmd_deploy_to_robot/);
});

test("PR29 Windows regression includes same-byte swap, hardlinks and staged integrity", () => {
  for (const name of [
    "same_content_target_replacement_is_a_save_conflict",
    "hardlink_alias_to_native_source_or_json_backup_is_rejected",
    "staged_file_modified_after_flush_cannot_be_committed",
    "windows_ads_unc_and_device_paths_are_rejected_before_filesystem_io"
  ]) {
    assert.ok(draft.includes("fn " + name), "Rust regression missing: " + name);
    assert.ok(workflow.includes(name), "Windows CI must compile: " + name);
  }
  assert.match(guarded, /pub\(crate\) fn read_regular_with_identity/);
  assert.match(guarded, /pub\(crate\) fn verify_single_link/);
  assert.match(draft, /Sha256::digest/);
});

test("acceptance fixture tool is synthetic-only and reports manual smoke as NOT RUN", () => {
  assert.match(harness, /ValidateSet\("Prepare", "Verify", "ValidateBinary"\)/);
  assert.match(harness, /SYNTHETIC ONLY - NOT AN OEM PROGRAM/);
  assert.match(harness, /Get-FileHash/);
  assert.match(harness, /user_webview2_smoke = "NOT RUN"/);
  assert.match(harness, /oem_bench = "NOT RUN"/);
  assert.match(harness, /release = "NOT PUBLISHED"/);
  assert.doesNotMatch(harness, /Invoke-WebRequest|Invoke-RestMethod|Start-BitsTransfer/);
});
