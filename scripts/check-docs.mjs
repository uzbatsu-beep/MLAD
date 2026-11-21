/**
 * Validate *local* Markdown targets in MLAD. No web requests, no dependency.
 * Run with Node 22: npm run docs:check
 * Does not claim that external manufacturer URLs or cited manual facts are verified.
 */
import fs from "node:fs";
import path from "node:path";
import process from "node:process";

const root=process.cwd();
function walk(directory){
  if(!fs.existsSync(directory))return [];
  return fs.readdirSync(directory,{withFileTypes:true}).flatMap(item=>{
    const p=path.join(directory,item.name);
    return item.isDirectory()?walk(p):item.isFile()&&p.endsWith(".md")?[p]:[];
  });
}
// Scan all maintained Markdown surfaces including language variants and
// archived GitHub workflow READMEs, not only one language of root documents.
const markdownFiles=[
  ...fs.readdirSync(root,{withFileTypes:true})
    .filter(entry=>entry.isFile()&&entry.name.endsWith(".md"))
    .map(entry=>path.join(root,entry.name)),
  ...walk(path.join(root,"docs")),
  ...walk(path.join(root,".github")),
].filter((p,i,paths)=>fs.existsSync(p)&&paths.indexOf(p)===i);
const issues=[];
let checked=0;
for(const filename of markdownFiles){
  const relative=path.relative(root,filename).split(path.sep).join("/");
  const text=fs.readFileSync(filename,"utf8");
  // Do not interpret Markdown examples embedded in fenced code as links.
  const prose=text.replace(/^(?:`{3,}|~{3,})[^\n]*\n[\s\S]*?^(?:`{3,}|~{3,})\s*$/gm,"");
  const links=prose.matchAll(/!?\[[^\]]*\]\(([^)]+)\)/g);
  for(const match of links){
    const target=match[1].trim().replace(/^<|>$/g,"");
    const base=target.split("#")[0].split("?")[0];
    if(!base||/^(?:[a-z][a-z0-9+.-]*:|\/\/)/i.test(base))continue;
    const resolved=path.resolve(path.dirname(filename),decodeURIComponent(base));
    checked++;
    const relativeTarget=path.relative(root,resolved);
    if(relativeTarget.startsWith("..")||path.isAbsolute(relativeTarget)){
      issues.push(`${relative}: reference escapes repository: ${target}`);
    }else if(!fs.existsSync(resolved)){
      issues.push(`${relative}: missing local target: ${target}`);
    }
  }
  // Catch a historical paste artifact: visible literal \n\n outside code spans.
  // Normal Markdown/code examples may intentionally mention a single escaped \n.
  const noInlineCode=prose.replace(/`[^`\n]*`/g,"");
  if(noInlineCode.includes("\\n\\n")){
    issues.push(`${relative}: literal escaped newline sequence appears in prose`);
  }
}
if(issues.length){
  for(const issue of issues)console.error("DOCS ERROR:",issue);
  console.error(`Docs check FAILED: ${issues.length} problems across ${markdownFiles.length} Markdown files`);
  process.exitCode=1;
}else{
  console.log(`Docs check OK: ${markdownFiles.length} Markdown files, ${checked} local links`);
}
