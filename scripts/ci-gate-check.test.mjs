import test from "node:test";
import assert from "node:assert/strict";
import { evaluateGate } from "./ci-gate-check.mjs";

const base = {
  scope: "success",
  frontend: "success",
  rust: "success",
  windows: "success",
  runtimeRequired: "true",
};

const docOnly = {
  ...base,
  rust: "skipped",
  windows: "skipped",
  runtimeRequired: "false",
};

test("runtime changes require complete Linux and Windows SUCCESS", () => {
  assert.deepEqual(evaluateGate(base), { ok: true, problems: [] });
});

test("doc-only changes pass with fast checks and skipped heavy builds", () => {
  assert.deepEqual(evaluateGate(docOnly), { ok: true, problems: [] });
});

for (const [name, mutation] of [
  ["missing scope result", {scope: undefined}],
  ["failed scope", {scope: "failure"}],
  ["skipped scope", {scope: "skipped"}],
  ["failed frontend", {frontend: "failure"}],
  ["skipped frontend", {frontend: "skipped"}],
  ["Rust failure", {rust: "failure"}],
  ["Rust skipped unexpectedly", {rust: "skipped"}],
  ["Windows failed", {windows: "failure"}],
  ["Windows cancelled", {windows: "cancelled"}],
  ["Windows unexpectedly skipped", {windows: "skipped"}],
  ["missing runtime classification", {runtimeRequired: undefined}],
  ["malformed runtime classification", {runtimeRequired: "yes"}],
]) {
  test("fail closed: " + name, () => {
    assert.equal(evaluateGate({...base, ...mutation}).ok, false);
  });
}

for (const [name, mutation] of [
  ["Rust unexpectedly ran", {rust: "success"}],
  ["Rust failed", {rust: "failure"}],
  ["Windows unexpectedly ran", {windows: "success"}],
  ["Windows failed", {windows: "failure"}],
  ["missing classification", {runtimeRequired: ""}],
  ["docs frontend failed", {frontend: "failure"}],
]) {
  test("docs-only fail closed: " + name, () => {
    assert.equal(evaluateGate({...docOnly, ...mutation}).ok, false);
  });
}
