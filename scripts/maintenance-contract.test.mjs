import test from "node:test";
import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
const read=p=>readFileSync(new URL("../"+p,import.meta.url),"utf8");
test("proposed main rule requires aggregate gate and preserves PR and ref protection",()=>{
 const r=JSON.parse(read(".github/rulesets/main-proposed.json"));
 assert.equal(r.enforcement,"active"); assert.deepEqual(r.conditions.ref_name.include,["refs/heads/main"]);
 for(const type of ["deletion","non_fast_forward","pull_request"])assert.ok(r.rules.some(x=>x.type===type));
 const s=r.rules.find(x=>x.type==="required_status_checks").parameters;
 assert.deepEqual(s.required_status_checks.map(x=>x.context),["ci-gate"]);assert.equal(s.strict_required_status_checks_policy,true);
 assert.equal(r.rules.find(x=>x.type==="pull_request").parameters.required_approving_review_count,0);
});
test("cleanup is manual, defaults dry-run, protects releases and checks head",()=>{
 const w=read(".github/workflows/merged-branches-prune.yml");
 assert.match(w,/workflow_dispatch:/);assert.doesNotMatch(w,/^  (push|schedule|pull_request):/m);
 assert.match(w,/default: dry-run/);assert.match(w,/DELETE_MERGED_PR_BRANCHES/);
 assert.ok(w.includes('!b.name.startsWith("release/")'));assert.ok(w.includes('p.head.sha === b.commit.sha'));
 assert.ok(w.includes('latest.data.commit.sha !== b.commit.sha'));assert.ok(w.includes('!activeHeads.has(b.name)'));assert.ok(w.includes('!activeBases.has(b.name)'));
});
test("dated cleanup snapshot cannot include release, main or duplicated branches",()=>{
 const a=JSON.parse(read(".github/maintenance/merged-branches-2026-11-27.json")).candidates;
 assert.equal(new Set(a.map(x=>x.branch)).size,a.length);
 for(const b of a){assert.notEqual(b.branch,"main");assert.ok(!b.branch.startsWith("release/"));assert.match(b.sha,/^[0-9a-f]{40}$/);assert.ok(Number.isInteger(b.pr));}
});
