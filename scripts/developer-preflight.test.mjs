import test from "node:test";
import assert from "node:assert/strict";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {readFileSync} from "node:fs";
const script=fileURLToPath(new URL("./developer-preflight.mjs",import.meta.url));
const invoke=(...args)=>spawnSync(process.execPath,[script,...args],{encoding:"utf8"});
const plan=m=>{const r=invoke(m,"--plan");assert.equal(r.status,0,r.stderr);return JSON.parse(r.stdout);};
test("plan mode enumerates all current model and documentation checks without executing",()=>{
 const p=plan("ci"),s=p.map(x=>x.join(" ")).join("\n");
 for(const token of ["ci --no-audit --no-fund","package-lock.json","docs:check","i18n_verify.py","i18n_semantic_guard.py","ci-gate-check.test.mjs","test:community","test:security","test:native","offline-preview-contract.test.mjs","test:diagnostics","test:oem-reference","test:graph","test:advanced","test:visual","test:inline","test:engineering","run build","cargo test --manifest-path src-tauri/Cargo.toml --locked"]) assert.ok(s.includes(token),token);
 assert.ok(p.findIndex(x=>x.includes("ci"))<p.findIndex(x=>x.includes("build")));
});
test("quality is separate from existing acceptance; local check does not install or release",()=>{
 const c=plan("check").flat(),q=plan("quality").flat();assert.ok(!c.includes("ci"));assert.ok(!c.includes("release"));assert.ok(q.includes("fmt"));assert.ok(q.includes("clippy"));assert.ok(!plan("test").flat().includes("test"));
});
test("invalid options fail closed",()=>{assert.equal(invoke("wrong","--plan").status,2);assert.equal(invoke("docs","--unexpected").status,2);});
test("CI exercises preflight contract through existing community command",()=>{const p=JSON.parse(readFileSync(new URL("../package.json",import.meta.url),"utf8"));assert.match(p.scripts["test:community"],/developer-preflight\.test\.mjs/);});
