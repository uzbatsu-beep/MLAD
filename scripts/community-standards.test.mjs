/**
 * Lightweight zero-dependency contract checks for GitHub Community Standards.
 * Validates presence and expected form structure. GitHub itself performs the
 * authoritative issue form rendering/schema check after merge.
 */
import test from "node:test";
import assert from "node:assert/strict";
import {readFileSync,existsSync} from "node:fs";

const text=(p)=>readFileSync(p,"utf8");
const standards=[
  "SECURITY.md","CODE_OF_CONDUCT.md","ACCESSIBILITY.md",
  ".github/ISSUE_TEMPLATE/bug_report.yml",
  ".github/ISSUE_TEMPLATE/feature_request.yml",
  ".github/ISSUE_TEMPLATE/controller_compatibility.yml",
  ".github/ISSUE_TEMPLATE/config.yml",
  ".github/PULL_REQUEST_TEMPLATE.md",
];
const forms=standards.filter(p=>p.endsWith(".yml")&&!p.endsWith("/config.yml"));

test("all community health paths recognized by GitHub are present",()=>{
  for(const file of standards){
    assert.ok(existsSync(file),file);
    assert.ok(text(file).trim().length>100,file+" must be meaningful");
  }
});

test("each issue form has a GitHub name/description/body, unique field ids and required user fields",()=>{
  for(const file of forms){
    const value=text(file);
    assert.match(value,/^name: ".+"/m,file);
    assert.match(value,/^description: ".+"/m,file);
    assert.match(value,/^body:\s*$/m,file);
    assert.match(value,/^  - type: (?:textarea|input|dropdown|checkboxes)\s*$/m,file);
    assert.match(value,/^      required: true\s*$/m,file);
    const ids=[...value.matchAll(/^    id: ([a-z_]+)\s*$/gm)].map(m=>m[1]);
    assert.ok(ids.length>=3,file+" should have multiple input fields");
    assert.equal(new Set(ids).size,ids.length,file+" duplicate field id");
    assert.match(value,/type: checkboxes/,file);
    assert.doesNotMatch(value,/^labels:/m,file+" must not auto-apply unknown labels");
    assert.match(value,/реальные|настоящие/i,file+" public information redaction guidance missing");
  }
});

test("security issue route is non-public by default and does not invent a contact address",()=>{
  const policy=text("SECURITY.md");
  const chooser=text(".github/ISSUE_TEMPLATE/config.yml");
  assert.match(policy,/private vulnerability reporting/i);
  assert.match(policy,/если функция недоступна/i);
  assert.match(policy,/не.*публичн/i);
  assert.match(policy,/CIOPRG\.LST/);
  assert.match(policy,/IONAME\.DAT/);
  assert.match(policy,/FTP\/HSES/);
  assert.doesNotMatch(policy,/[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}/);
  assert.match(chooser,/^blank_issues_enabled: false\s*$/m);
  assert.match(chooser,/\/security\/policy/);
});

test("safety review is mandatory even for documentation-only PR",()=>{
  const p=text(".github/PULL_REQUEST_TEMPLATE.md");
  assert.match(p,/CIOPRG\.LST/);
  assert.match(p,/IONAME\.DAT/);
  assert.match(p,/FTP\/HSES/);
  assert.match(p,/ManualDerived/);
  assert.match(p,/NOT RUN/);
  assert.match(p,/GPL-3\.0-only/);
  assert.match(p,/явного согласования владельца/i);
  assert.match(text("CONTRIBUTING.md"),/test:community/);
  assert.match(text("README.md"),/SECURITY\.md/);
});

test("community conduct/accessibility are truthful, not compliance certifications",()=>{
  const conduct=text("CODE_OF_CONDUCT.md");
  const access=text("ACCESSIBILITY.md");
  assert.match(conduct,/не.*публик/i);
  assert.match(conduct,/SECURITY\.md/);
  assert.match(access,/WCAG 2\.2 AA/);
  assert.match(access,/не заявляется/);
  assert.match(access,/клавиатур/i);
  assert.match(access,/screen reader/i);
});

test("community test is included in standard CI and doc targets are checked",()=>{
  assert.match(text(".github/workflows/validation.yml"),/npm run test:community/);
  assert.match(text("scripts/check-docs.mjs"),/SECURITY\.md/);
  assert.match(text("scripts/check-docs.mjs"),/CODE_OF_CONDUCT\.md/);
  assert.match(text("scripts/check-docs.mjs"),/ACCESSIBILITY\.md/);
});
