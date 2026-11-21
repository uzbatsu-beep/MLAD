// PR48 supply-chain contracts: never launches a GUI or robot.
import test from "node:test";
import assert from "node:assert/strict";
import {readFileSync,mkdtempSync,rmSync,existsSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";

const workflow=readFileSync(".github/workflow-archive/windows-offline-preview-pr48.yml","utf8");
const packaging=readFileSync("scripts/windows-offline-preview.ps1","utf8");
const guide=readFileSync("docs/windows_offline_preview_pr48.md","utf8");
const rust=readFileSync("src-tauri/src/main.rs","utf8");
const python=process.platform==="win32"?"python":"python3";
function fixture(args,expectedStatus=0){
  const result=spawnSync(python,["scripts/offline-preview-fixture.py",...args],
    {encoding:"utf8",timeout:20000});
  assert.equal(result.status,expectedStatus,
    args.join(" ")+":\n"+result.stderr+"\n"+result.stdout);
  return result;
}
test("preview checks out exact PR head and never publishes or signs installers",()=>{
  assert.match(workflow,/github\.event\.pull_request\.head\.sha \|\| github\.sha/);
  assert.ok(workflow.includes("ref: ${{ github.event.pull_request.head.sha || github.sha }}"));
  assert.ok(workflow.includes("contents: read"));
  assert.match(workflow,/npm ci --no-audit --no-fund/);
  assert.match(workflow,/cargo test --manifest-path src-tauri\/Cargo\.toml --locked/);
  assert.match(workflow,/npm run tauri -- build --no-bundle/);
  assert.match(workflow,/python scripts\/offline-preview-fixture\.py --mode self-test/);
  assert.match(workflow,/windows-offline-preview\.ps1 -Mode Verify/);
  assert.match(workflow,/Compress-Archive -Path/);
  assert.match(workflow,/MLAD-PR48-Windows-x64-OfflinePreview-Unsigned-NonRelease/);
  assert.match(workflow,/retention-days: 21/);
  assert.doesNotMatch(workflow,/(?:gh release|gh api|create-release|releases\/upload|--bundles nsis|contents: write)/i);
  assert.doesNotMatch(workflow,/cmd_deploy_to_robot|ftp:\/\//i);
});
test("package includes corresponding GPL source, verifies SHA and preserves untested statuses",()=>{
  assert.match(packaging,/corresponding-source\.zip/);
  assert.match(packaging,/git -C \$repo archive --format=zip/);
  assert.match(packaging,/SHA256SUMS\.txt/);
  assert.match(packaging,/Get-FileHash -LiteralPath \$path -Algorithm SHA256/);
  assert.match(packaging,/MLAD_PR48_WINDOWS_OFFLINE_PREVIEW_V1/);
  assert.match(packaging,/MLAD_PR48_SYNTHETIC_FIXTURE_V1/);
  assert.match(packaging,/executable_authenticode/);
  assert.match(packaging,/user_smoke="NOT RUN"/);
  assert.match(packaging,/oem_bench="NOT RUN"/);
  assert.match(packaging,/release="NOT PUBLISHED"/);
  assert.match(packaging,/robot_access="BLOCKED"/);
  assert.match(packaging,/ExpectedGitSha/);
  assert.match(packaging,/Refusing to overwrite a staged preview/);
  assert.match(packaging,/MLAD_OFFLINE_PROJECT/);
  assert.doesNotMatch(packaging,/Start-Process.*yaskawa|Invoke-WebRequest.*robot|gh release/);
});
test("human acceptance guide contains 17 manual cases and evidence limits",()=>{
  for(let n=1;n<=17;n++){
    assert.match(guide,new RegExp("P48-"+String(n).padStart(2,"0")));
  }
  assert.match(guide,/UserSmoke=NOT RUN/);
  assert.match(guide,/OEMBench=NOT RUN/);
  assert.match(guide,/Get-FileHash/);
  assert.match(guide,/WebView2/);
  assert.match(guide,/CIOPRG\.LST/);
  assert.match(guide,/IONAME\.DAT/);
  assert.match(guide,/SHA256SUMS\.txt/);
  assert.match(guide,/СИНТЕТИЧЕСКИЙ/);
  assert.match(guide,/соединение с производственной сетью роботов/);
  assert.match(rust,/cmd_open_offline_project/);
  assert.match(rust,/cmd_save_offline_project/);
  assert.match(rust,/Загрузка на робот отключена/);
});
test("synthetic project is byte-exact with manifest/sidecar and fails on tamper",()=>{
  fixture(["--mode","self-test"]);
  const root=mkdtempSync(join(tmpdir(),"mlad-pr48-node-"));
  const sample=join(root,"new-project");
  try{
    fixture(["--mode","prepare","--root",sample]);
    fixture(["--mode","verify","--root",sample]);
    const raw=readFileSync(join(sample,"CIOPRG.LST"));
    assert.deepEqual(raw,Buffer.from("STR #70010\r\nOUT #70020\nEND\n"));
    assert.ok(existsSync(join(sample,"IONAME.DAT")));
    const meta=JSON.parse(readFileSync(join(sample,"cell.mladproj.json")));
    const notes=JSON.parse(readFileSync(join(sample,"cell.mlad.json")));
    assert.equal(meta.format,"MLAD_OFFLINE_PROJECT");
    assert.equal(meta.schema_version,1);
    assert.equal(meta.source.controller,"YRC1000");
    assert.equal(meta.source.line_count,3);
    assert.equal(meta.analysis.execution_verified,false);
    assert.equal(meta.analysis.read_roles,1);
    assert.equal(meta.analysis.write_roles,1);
    assert.equal(notes.source.sha256,meta.source.sha256);
    assert.equal(notes.bookmarks[0].line,1);
    assert.equal(meta.source.relative_path,"CIOPRG.LST");
    assert.equal(meta.ioname.relative_path,"IONAME.DAT");
    assert.equal(meta.sidecar.relative_path,"cell.mlad.json");
    fixture(["--mode","prepare","--root",sample],1);
    writeFileSync(join(sample,"CIOPRG.LST"),Buffer.concat([raw,Buffer.from("x")]));
    fixture(["--mode","verify","--root",sample],1);
  }finally{rmSync(root,{recursive:true,force:true});}
});
