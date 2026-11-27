/** Local, sequential, fail-fast developer checks. No merge/release actions. */
import {spawnSync} from "node:child_process";
import {existsSync} from "node:fs";
import {fileURLToPath} from "node:url";
const root=fileURLToPath(new URL("../",import.meta.url));
const npm=process.env.npm_execpath;
const py=process.env.MLAD_PYTHON || (process.platform==="win32"?"python":"python3");
const n=(...args)=>[process.execPath,...args];
const run=name=>npm?n(npm,"run",name):[process.platform==="win32"?"npm.cmd":"npm","run",name];
const npmCi=npm?n(npm,"ci","--no-audit","--no-fund"):["npm","ci","--no-audit","--no-fund"];
const docs=[run("docs:check"),[py,"scripts/i18n_verify.py"],[py,"scripts/i18n_semantic_guard.py"]];
const tests=[n("--test","scripts/ci-gate-check.test.mjs"),run("test:community"),run("test:security"),run("test:native"),n("--test","scripts/offline-preview-contract.test.mjs"),run("test:diagnostics"),run("test:oem-reference"),run("test:graph"),run("test:advanced"),run("test:visual"),run("test:inline"),run("test:engineering")];
// Independently delivered ENG-01 tests join the local suite once present.
if(existsSync(new URL("./maintenance-contract.test.mjs",import.meta.url))) tests.push(n("--test","scripts/maintenance-contract.test.mjs"));
const rust=["cargo","test","--manifest-path","src-tauri/Cargo.toml","--locked"];
const modes={docs,test:tests,check:[...docs,...tests,run("build")],ci:[npmCi,["git","diff","--exit-code","--","package.json","package-lock.json"],...docs,...tests,run("build"),rust],quality:[["cargo","fmt","--manifest-path","src-tauri/Cargo.toml","--all","--","--check"],["cargo","clippy","--manifest-path","src-tauri/Cargo.toml","--locked","--all-targets","--","-D","warnings"]]};
const args=process.argv.slice(2);const mode=args[0];
if(!Object.hasOwn(modes,mode)||args.length>2||(args.length===2&&args[1]!=="--plan")){console.error("Usage: npm run check:ci (or node scripts/developer-preflight.mjs docs|test|check|ci|quality --plan)");process.exit(2);}
if(args[1]==="--plan"){console.log(JSON.stringify(modes[mode]));process.exit(0);}
if(!npm&&process.platform==="win32"){console.error("Use npm run <script> on Windows so npm_execpath is available.");process.exit(2);}
for(const [cmd,...a] of modes[mode]){console.log("\n> "+[cmd,...a].join(" "));const result=spawnSync(cmd,a,{cwd:root,stdio:"inherit",shell:false});if(result.error||result.status!==0){console.error("Preflight failed:",result.error?.message||result.status||result.signal);process.exit(result.status&&result.status>0?result.status:1);}}
console.log("Preflight completed; not Windows UserSmoke or OEMBench.");
