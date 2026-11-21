/**
 * Scanner regression coverage: all GitHub/root/docs Markdown, exact delimiter
 * closure, and refusal of missing links outside fenced source examples.
 * Offline only; does not call external resources or ship native controller data.
 */
import test from "node:test";
import assert from "node:assert/strict";
import {mkdtempSync,mkdirSync,writeFileSync,copyFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {dirname} from "node:path";
const base=dirname(fileURLToPath(import.meta.url));
function withRepo(files,fn){
  const root=mkdtempSync(join(tmpdir(),"mlad-md-links-"));
  try {
    mkdirSync(join(root,"scripts"));
    copyFileSync(join(base,"check-docs.mjs"),join(root,"scripts","check-docs.mjs"));
    writeFileSync(join(root,"package.json"),JSON.stringify({type:"module"}));
    for(const [name,body] of Object.entries(files)){
      const path=join(root,name);
      mkdirSync(dirname(path),{recursive:true});
      writeFileSync(path,body);
    }
    fn(root);
  }finally{rmSync(root,{recursive:true,force:true});}
}
function run(root){
  return spawnSync(process.execPath,["scripts/check-docs.mjs"],{
    cwd:root,encoding:"utf8",timeout:12000,
  });
}
test("checks every root RU/EN and .github Markdown, not only original language",()=>{
  withRepo({
    "README.md":"# Root\n",
    "README.ru.md":"[missing](not-here.md)\n",
    ".github/workflow-archive/README.md":"# Historic workflow\n",
  },root=>{
    const out=run(root);
    assert.equal(out.status,1,out.stderr+out.stdout);
    assert.match(out.stderr,/README\.ru\.md: missing local target/);
  });
});
test("a missing link inside both valid fenced styles is exempt",()=>{
  withRepo({
    "SECURITY.md":"# Private policy\n",
    "docs/example.md":"# Example\n\n~~~markdown\n[unavailable](missing.md)\n~~~\n\n```text\n[also unavailable](missing2.md)\n```\n\n[valid](../SECURITY.md)\n",
  },root=>{
    const out=run(root);
    assert.equal(out.status,0,out.stderr+out.stdout);
    assert.match(out.stdout,/Docs check OK/);
  });
});
test("closing a tilde fence must not conceal broken links after the fence",()=>{
  withRepo({
    "docs/a.md":"~~~yaml\n[inside](unavailable.md)\n~~~\n\n[outside](missing.md)\n",
  },root=>{
    const out=run(root);
    assert.equal(out.status,1,out.stderr+out.stdout);
    assert.match(out.stderr,/missing local target: missing\.md/);
  });
});
test("local paths must not traverse outside the repository",()=>{
  withRepo({"README.md":"[escape](../outside.md)\n"},root=>{
    const out=run(root);
    assert.equal(out.status,1);
    assert.match(out.stderr,/reference escapes repository/);
  });
});
