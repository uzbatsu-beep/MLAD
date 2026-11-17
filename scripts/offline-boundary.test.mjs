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
  assert.match(rust, /inspection_report\(600, 500\)/);
});
