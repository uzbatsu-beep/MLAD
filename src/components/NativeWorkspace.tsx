import { useMemo, useState } from "react";
import { LadderCircuit } from "./LadderCircuit.tsx";
import { AdvancedBlockDiagram } from "./AdvancedBlockDiagram.tsx";
import type { AdvancedBlock, Preset } from "../ladder/advancedModel.ts";
import { semanticToExpression, relayText, type SemanticNode } from "../ladder/graphModel.ts";
import { invoke } from "@tauri-apps/api/core";

type Controller = "DX100" | "DX200" | "YRC1000";
type Encoding = "Utf8" | "Utf8Bom" | "OpaqueBytes";
type Diagnostic = {
  code: string;
  severity: "Warning" | "Error";
  line: number;
  byte_offset: number;
  message: string;
};
type PreviewLine = {
  line: number;
  ending: string;
  kind: unknown;
  display_text: string;
};
type SemanticDiagnostic = {
  code: string;
  level: "Warning" | "Error";
  line: number;
  message: string;
};
type SemanticRung = {
  number: number;
  first_line: number;
  output_line: number;
  output: number;
  inputs: number[];
  expression_preview: string;
  expression_root: number;
  section: "Unspecified" | "System" | "User";
};
type SemanticPreview = {
  nodes: SemanticNode[];
  nodes_truncated: boolean;
  node_count: number;
  rung_count: number;
  reference_count: number;
  diagnostic_count: number;
  unsupported_segments: number;
  subset_analysis_complete: boolean;
  system_user_boundaries_verified: boolean;
  edited_export_allowed: boolean;
  end_seen: boolean;
  rungs: SemanticRung[];
  references: { symbol: string; sites: { line: number; operation: string }[] }[];
  part_markers: { line: number; section: string }[];
  diagnostics: SemanticDiagnostic[];
  rungs_truncated: boolean;
  references_truncated: boolean;
  diagnostics_truncated: boolean;
};
type AdvancedSourceBlock = {
  kind:"Timer"|"Counter"|"GroupCopy";
  line:number;end_line:number;
  operands:string[];
  count_or_enable:string|null;
  reset:string|null;
  count_or_enable_inverted:boolean;
  reset_inverted:boolean;
  output:string|null;
  example_pattern_recognized:boolean;
};
type AdvancedSourceReport = {
  source_sha256:string;
  blocks:AdvancedSourceBlock[];
  warnings:{code:string;line:number;message:string}[];
  blocks_truncated:boolean;
  original_only:boolean;
  native_export_allowed:boolean;
  hardware_test_authorized:boolean;
};
function parseManualPreset(text:string):Preset|null{
  if(/^M[0-9]{3}$/.test(text))return {kind:"register",address:text};
  if(/^[0-9]+$/.test(text)){
    const n=Number(text);
    if(Number.isInteger(n)&&n>=0&&n<=65535)return {kind:"literal",value:n};
  }
  return null;
}
function toReadOnlyAdvanced(b:AdvancedSourceBlock):AdvancedBlock|null{
  if(!b.example_pattern_recognized)return null;
  if(b.kind==="GroupCopy" && b.operands.length===2)return {
    kind:"GROUP",id:"source-"+b.line,source:b.operands[0],
    destination:b.operands[1],
  };
  if(b.operands.length!==2||!b.count_or_enable||!b.output)return null;
  const preset=parseManualPreset(b.operands[1]);
  if(!preset)return null;
  const first={
    kind:"contact" as const,id:"src-c-"+b.line,relay:b.count_or_enable,
    normallyClosed:b.count_or_enable_inverted,
  };
  if(b.kind==="Timer")return {
    kind:"TMR",id:"src-"+b.line,current:b.operands[0],preset,
    output:b.output,enable:first,
  };
  if(b.kind==="Counter" && b.reset)return {
    kind:"CNT",id:"src-"+b.line,current:b.operands[0],preset,
    output:b.output,count:first,
    reset:{kind:"contact",id:"src-r-"+b.line,relay:b.reset,
      normallyClosed:b.reset_inverted},
  };
  return null;
}

type NativeSection = {line:number;category:string;label:string};
type SearchHit = {line:number;preview:string};
type SearchResponse = {source_sha256:string;hits:SearchHit[];truncated:boolean};
type Report = {
  advanced: AdvancedSourceReport;
  sections: NativeSection[];
  semantic: SemanticPreview;
  controller: Controller;
  encoding: Encoding;
  byte_length: number;
  source_sha256: string;
  line_count: number;
  recognized_instructions: number;
  unsupported_lines: number;
  diagnostics: Diagnostic[];
  lines: PreviewLine[];
  preview_truncated: boolean;
  m_headers: { marker_line: number; data_lines: number[]; complete: boolean }[];
  edited_export_allowed: boolean;
  system_user_boundaries_verified: boolean;
  original_round_trip_only: boolean;
};

type ReadPage = {
  source_sha256: string;
  start_line: number;
  line_count: number;
  lines: PreviewLine[];
};
const PAGE_SIZE = 250;

const panel: React.CSSProperties = {
  background: "#252526",
  border: "1px solid #454952",
  padding: "16px",
  borderRadius: "8px",
};
const field: React.CSSProperties = {
  color: "#e0e4eb",
  background: "#171b20",
  border: "1px solid #606875",
  borderRadius: "4px",
  padding: "8px",
};
const btn: React.CSSProperties = {
  color: "#edf4fa", background: "#354b66", border: "1px solid #61758e",
  borderRadius: "4px", padding: "8px 12px", cursor: "pointer",
};

function lineCategory(kind: unknown): string {
  if (typeof kind === "string") return kind;
  if (typeof kind === "object" && kind !== null) {
    return Object.keys(kind)[0] || "Unknown";
  }
  return "Unknown";
}


/** Primary document workspace, mounted in the main app window — never a modal. */
export function NativeWorkspace() {
  const [path,setPath]=useState("");
  const [controller,setController]=useState<Controller>("YRC1000");
  const [report,setReport]=useState<Report|null>(null);
  const [busy,setBusy]=useState(false);
  const [issue,setIssue]=useState("");
  const [tab,setTab]=useState<"source"|"diagram"|"blocks">("source");
  const [pageStart,setPageStart]=useState(1);
  const [pageLines,setPageLines]=useState<PreviewLine[]>([]);
  const [pageBusy,setPageBusy]=useState(false);
  const [selectedLine,setSelectedLine]=useState<number|null>(null);
  const [goto,setGoto]=useState("");
  const [rung,setRung]=useState(1);
  const [zoom,setZoom]=useState(1);
  const [search,setSearch]=useState("");
  const [hits,setHits]=useState<SearchHit[]>([]);
  const [searchTruncated,setSearchTruncated]=useState(false);
  const [searchBusy,setSearchBusy]=useState(false);
  const [filter,setFilter]=useState<"all"|"Error"|"Warning">("all");
  const [json,setJson]=useState("");

  const activeRung=report?.semantic.rungs.find(x=>x.number===rung)??report?.semantic.rungs[0];
  const graph=useMemo(()=>{
    if(!report||!activeRung)return null;
    try{return {expression:semanticToExpression(report.semantic.nodes,activeRung.expression_root)};}
    catch(error){return {error:String(error)};}
  },[report,activeRung]);
  const diags=useMemo(()=>{
    if(!report)return [];
    return [
      ...report.diagnostics.map(d=>({severity:d.severity,line:d.line,code:d.code,message:d.message})),
      ...report.semantic.diagnostics.map(d=>({severity:d.level,line:d.line,code:d.code,message:d.message})),
    ].filter(x=>filter==="all"||x.severity===filter).sort((a,b)=>a.line-b.line);
  },[report,filter]);

  async function openSource(){
    setBusy(true);setIssue("");setReport(null);setHits([]);setJson("");
    try{
      const value=await invoke<Report>("cmd_inspect_cioprg",{filePath:path.trim(),controller});
      setReport(value);
      setPageLines(value.lines.slice(0,PAGE_SIZE));
      setPageStart(1);setSelectedLine(null);
      setRung(value.semantic.rungs[0]?.number??1);setTab("source");
    }catch(error){setIssue(String(error));}finally{setBusy(false);}
  }
  async function showLine(line:number){
    if(!report||pageBusy||!Number.isInteger(line)||line<1||line>report.line_count)return;
    const start=Math.floor((line-1)/PAGE_SIZE)*PAGE_SIZE+1;
    setIssue("");setTab("source");
    if(start+PAGE_SIZE-1<=report.lines.length){
      setPageLines(report.lines.slice(start-1,start-1+PAGE_SIZE));
      setPageStart(start);setSelectedLine(line);return;
    }
    setPageBusy(true);
    try{
      const value=await invoke<ReadPage>("cmd_read_cioprg_page",{
        filePath:path.trim(),controller,expectedSha256:report.source_sha256,startLine:start,
      });
      if(value.source_sha256!==report.source_sha256||value.line_count!==report.line_count||value.start_line!==start){
        throw Error("Контрольная сумма исходного файла изменилась");
      }
      setPageLines(value.lines);setPageStart(start);setSelectedLine(line);
    }catch(error){setIssue(String(error));}finally{setPageBusy(false);}
  }
  async function searchAll(){
    if(!report||searchBusy||search.trim().length<2)return;
    setSearchBusy(true);setIssue("");
    try{
      const value=await invoke<SearchResponse>("cmd_search_cioprg",{
        filePath:path.trim(),controller,expectedSha256:report.source_sha256,query:search,
      });
      if(value.source_sha256!==report.source_sha256)throw Error("Снимок файла изменился");
      setHits(value.hits);setSearchTruncated(value.truncated);
    }catch(error){setIssue(String(error));setHits([]);}
    finally{setSearchBusy(false);}
  }
  function reportAsJson(){
    if(!report)return;
    setJson(JSON.stringify({
      format:"MLAD Native Workspace read-only diagnostic report",
      controller:report.controller,source_sha256:report.source_sha256,
      byte_length:report.byte_length,line_count:report.line_count,encoding:report.encoding,
      recognized_instructions:report.recognized_instructions,unsupported_lines:report.unsupported_lines,
      sections:report.sections,parser_diagnostics:report.diagnostics,
      semantic_diagnostics:report.semantic.diagnostics,
      semantic_diagnostics_truncated:report.semantic.diagnostics_truncated,
      semantic_rung_count:report.semantic.rung_count,
      original_round_trip_only:report.original_round_trip_only,
      edited_export_allowed:report.edited_export_allowed,
      system_user_boundaries_verified:report.system_user_boundaries_verified,
      warning:"Report may contain sensitive controller addresses. Review before sharing.",
    },null,2));
  }
  const sideStyle:React.CSSProperties={
    background:"#182530",borderRight:"1px solid #405263",minWidth:0,
    overflowY:"auto",padding:10,fontSize:12,
  };
  const itemStyle:React.CSSProperties={
    display:"block",background:"transparent",border:0,color:"#ced9e5",
    cursor:"pointer",padding:"5px 3px",textAlign:"left",width:"100%",
    overflow:"hidden",textOverflow:"ellipsis",whiteSpace:"nowrap",fontSize:12,
  };
  const tabButton=(view:typeof tab,label:string)=>(
    <button type="button" style={{...btn,background:tab===view?"#3b695e":"#2e4152"}}
      aria-pressed={tab===view} onClick={()=>setTab(view)}>{label}</button>
  );
  return <main aria-label="Native CIO Workspace" style={{
    display:"flex",flex:1,minHeight:0,minWidth:0,flexDirection:"column",
    background:"#101820",color:"#dce5ef",
  }}>
    <header style={{background:"#1c2b39",padding:"10px 14px",borderBottom:"1px solid #42566a",
      display:"flex",alignItems:"center",justifyContent:"space-between",flexWrap:"wrap",gap:8}}>
      <div>
        <strong style={{fontSize:18}}>Программа робота · CIOPRG.LST</strong>
        <div style={{fontSize:12,color:"#dcb97d",marginTop:4}}>
          Native Source · Только чтение · Границы System/User не подтверждены
        </div>
      </div>
      <span style={{fontSize:12,color:"#a5d6b6"}}>● ЛОКАЛЬНО · OFFLINE</span>
    </header>
    <form onSubmit={e=>{e.preventDefault();void openSource();}} style={{
      display:"flex",flexWrap:"wrap",gap:8,alignItems:"end",padding:"10px 14px",
      background:"#1a2733",borderBottom:"1px solid #35485b",
    }}>
      <label style={{display:"grid",gap:5,flex:"1 1 390px",fontSize:12}}>
        Путь к копии CIOPRG.LST
        <input aria-label="Путь к файлу CIOPRG.LST" style={{...field,width:"100%",boxSizing:"border-box"}}
          value={path} onChange={e=>setPath(e.target.value)} placeholder="C:\backup\CIOPRG.LST"/>
      </label>
      <label style={{display:"grid",gap:5,fontSize:12}}>Контроллер
        <select style={field} value={controller} onChange={e=>setController(e.target.value as Controller)}>
          <option value="DX100">DX100</option><option value="DX200">DX200</option>
          <option value="YRC1000">YRC1000</option>
        </select>
      </label>
      <button type="submit" style={{...btn,background:"#396c59"}} disabled={!path.trim()||busy}>
        {busy?"Чтение...":"Открыть исходник"}
      </button>
    </form>
    {issue&&<div role="alert" style={{background:"#56363c",color:"#ffd5d6",padding:"8px 14px"}}>
      {issue}
    </div>}
    {!report?<section style={{padding:24,maxWidth:800,lineHeight:1.7}}>
      <h2 style={{marginTop:0}}>Native CIOPRG Workspace</h2>
      <p>Откройте копию выгрузки робота. Структура, исходный текст, распознанные цепи и
        диагностика будут расположены в основном рабочем окне. Это не JSON-черновик.</p>
      <p style={{color:"#e2c18b"}}>Изменение файла, экспорт CIOPRG.LST и сетевой доступ заблокированы.</p>
    </section>:<>
      <div style={{display:"flex",gap:15,flexWrap:"wrap",padding:"7px 14px",fontSize:12,
        background:"#192735",borderBottom:"1px solid #35485b"}}>
        <span>{report.controller} · <b>{report.line_count}</b> строк · {report.byte_length} байт</span>
        <span style={{color:"#a6d5b5"}}>{report.recognized_instructions} распознано</span>
        <span style={{color:"#edc38b"}}>{report.unsupported_lines} opaque/unknown</span>
        <span title={report.source_sha256}>SHA-256: {report.source_sha256.slice(0,16)}…</span>
      </div>
      <div style={{display:"grid",gridTemplateColumns:"220px minmax(300px,1fr) 290px",flex:1,minHeight:0}}>
        <aside style={sideStyle} aria-label="Дерево программы">
          <strong>Структура файла</strong>
          <p style={{fontSize:11,color:"#d2b484"}}>
            PART — только текстовая метка, не право записи в System/User
          </p>
          {report.sections.map((s,i)=><button key={i} type="button" style={itemStyle}
            title={s.label+" · строка "+s.line} onClick={()=>void showLine(s.line)}>
            {s.category==="Part"?"▸ ":"· "}{s.label} <small>L{s.line}</small>
          </button>)}
          <hr style={{borderColor:"#3a4d5e"}}/>
          <strong>Логические цепи ({report.semantic.rung_count})</strong>
          {report.semantic.rungs.map(r=><button type="button" key={r.number} style={itemStyle}
            onClick={()=>{setRung(r.number);setTab("diagram");}}>
            R{r.number} · OUT {relayText(r.output)}
          </button>)}
          {report.semantic.rungs_truncated&&<small>Список цепей ограничен</small>}
          <hr style={{borderColor:"#3a4d5e"}}/>
          <strong>Функциональные блоки</strong>
          {report.advanced.blocks.map(b=><button type="button" style={itemStyle} key={b.line}
            onClick={()=>{setTab("blocks");setSelectedLine(b.line);}}>
            {b.kind} · L{b.line}
          </button>)}
        </aside>
        <section aria-label="Рабочее поле CIOPRG" style={{
          display:"flex",flexDirection:"column",minWidth:0,minHeight:0,background:"#0f1923",
        }}>
          <nav style={{display:"flex",gap:8,flexWrap:"wrap",padding:9,borderBottom:"1px solid #35485b"}}>
            {tabButton("source","Исходный текст")}
            {tabButton("diagram","Ladder ("+report.semantic.rung_count+")")}
            {tabButton("blocks","TMR / CNT / GSTR ("+report.advanced.blocks.length+")")}
          </nav>
          {tab==="source"&&<>
            <div style={{display:"flex",gap:8,alignItems:"center",flexWrap:"wrap",
              padding:9,borderBottom:"1px solid #35485b",fontSize:12}}>
              <button type="button" style={btn} disabled={pageBusy||pageStart===1}
                onClick={()=>void showLine(Math.max(1,pageStart-PAGE_SIZE))}>←</button>
              <span>{pageStart}–{pageLines[pageLines.length-1]?.line??0} / {report.line_count}</span>
              <button type="button" style={btn}
                disabled={pageBusy||pageStart+PAGE_SIZE>report.line_count}
                onClick={()=>void showLine(pageStart+PAGE_SIZE)}>→</button>
              <input type="number" min={1} max={report.line_count} aria-label="Номер строки"
                value={goto} onChange={e=>setGoto(e.target.value)}
                style={{...field,width:80}} placeholder="Строка"/>
              <button type="button" style={btn} onClick={()=>void showLine(Number(goto))}
                disabled={pageBusy}>Перейти</button>
              {pageBusy&&<span>Чтение страницы…</span>}
            </div>
            <div style={{flex:1,minHeight:0,overflow:"auto",fontFamily:"Consolas,monospace",
              fontSize:12,lineHeight:1.7,padding:"8px 12px"}}>
              {pageLines.map(l=><button type="button" key={l.line} onClick={()=>setSelectedLine(l.line)}
                title={"Byte offset: "+l.line}
                style={{display:"flex",gap:10,whiteSpace:"pre",width:"max-content",minWidth:"100%",
                  textAlign:"left",font:"inherit",padding:"0 4px",border:0,cursor:"pointer",
                  color:"#dce5ef",background:selectedLine===l.line?"#2b4d5b":"transparent"}}>
                <span style={{width:51,flexShrink:0,textAlign:"right",color:"#849eaf"}}>{l.line}</span>
                <span style={{width:142,flexShrink:0,color:
                  lineCategory(l.kind)==="UnsupportedInstruction"?"#e4b97e":"#85abb9"}}>
                  {lineCategory(l.kind)}</span>
                <span>{l.display_text||" "}</span>
              </button>)}
            </div>
            <div style={{fontSize:11,color:"#a9c0cb",padding:"5px 12px",borderTop:"1px solid #35485b"}}>
              Полный исходник по страницам · нет операций редактирования или сохранения
            </div>
          </>}
          {tab==="diagram"&&<div style={{overflow:"auto",flex:1,padding:15}}>
            <div style={{display:"flex",gap:8,alignItems:"center",flexWrap:"wrap"}}>
              <label>Цепь <select style={field} value={activeRung?.number??""}
                onChange={e=>setRung(Number(e.target.value))}>
                {report.semantic.rungs.map(r=><option key={r.number} value={r.number}>
                  R{r.number} · OUT {relayText(r.output)}
                </option>)}
              </select></label>
              <button type="button" style={btn} onClick={()=>setZoom(z=>Math.max(.5,z-.15))}>−</button>
              <span>{Math.round(zoom*100)}%</span>
              <button type="button" style={btn} onClick={()=>setZoom(z=>Math.min(1.8,z+.15))}>+</button>
              {activeRung&&<button type="button" style={btn}
                onClick={()=>void showLine(activeRung.first_line)}>Исходник L{activeRung.first_line} ↗</button>}
            </div>
            <p style={{color:"#e4bd80",fontSize:12}}>
              Частичная реконструкция подтверждённого булевого подмножества. Unknown не преобразуются в цепь.
            </p>
            {activeRung&&graph&&"expression" in graph&&graph.expression?
              <LadderCircuit readOnly zoom={zoom} expression={graph.expression}
                output={relayText(activeRung.output)}/>:
              <div style={{color:"#d9bb83"}}>Нет проверенной схемы для выбранного участка.</div>}
            {report.semantic.nodes_truncated&&<p style={{color:"#e4bd80"}}>AST обрезан по лимиту.</p>}
          </div>}
          {tab==="blocks"&&<div style={{flex:1,overflow:"auto",padding:15}}>
            <p style={{color:"#e4bd80",fontSize:12}}>
              Визуализируются только подтверждённые простые топологии. Остальные доступны в исходнике.
            </p>
            {report.advanced.blocks.map(b=>{
              const overlay=toReadOnlyAdvanced(b);
              return <div style={{...panel,marginBottom:12}} key={b.line}>
                <div style={{display:"flex",gap:10,alignItems:"center",
                  justifyContent:"space-between",flexWrap:"wrap",marginBottom:10}}>
                  <strong>{b.kind} · L{b.line}–{b.end_line} · {b.operands.join(", ")}</strong>
                  <button type="button" style={btn} onClick={()=>void showLine(b.line)}>К исходнику ↗</button>
                </div>
                {overlay?<AdvancedBlockDiagram block={overlay}/>:
                  <div style={{color:"#e4bd80"}}>Граф не построен: топология не подтверждена.</div>}
              </div>;
            })}
            {!report.advanced.blocks.length&&<p>Поддерживаемые блоки не найдены.</p>}
            {report.advanced.blocks_truncated&&<p>Предпросмотр блоков ограничен.</p>}
          </div>}
        </section>
        <aside style={{...sideStyle,borderRight:0,borderLeft:"1px solid #405263"}}
          aria-label="Диагностика и поиск">
          <strong>Поиск по всему файлу</strong>
          <div style={{display:"flex",gap:6,marginTop:8}}>
            <input aria-label="Строка поиска" value={search} onChange={e=>setSearch(e.target.value)}
              onKeyDown={e=>{if(e.key==="Enter")void searchAll();}}
              placeholder="MOV, M010, #70010" style={{...field,minWidth:0,flex:1}}/>
            <button type="button" style={btn} onClick={()=>void searchAll()}
              disabled={searchBusy||search.trim().length<2}>Найти</button>
          </div>
          {!!hits.length&&<div style={{maxHeight:190,overflowY:"auto",marginTop:8}}>
            <small>{hits.length} совпадений {searchTruncated?"(лимит 100)":""}</small>
            {hits.map(h=><button type="button" key={h.line} style={itemStyle}
              title={h.preview} onClick={()=>void showLine(h.line)}>
              L{h.line}: {h.preview}
            </button>)}
          </div>}
          <hr style={{borderColor:"#3a4d5e"}}/>
          <div style={{display:"flex",justifyContent:"space-between",alignItems:"center",gap:5}}>
            <strong>Диагностика ({diags.length})</strong>
            <select aria-label="Фильтр сообщений" style={{...field,padding:4}} value={filter}
              onChange={e=>setFilter(e.target.value as typeof filter)}>
              <option value="all">Все</option><option value="Error">Ошибки</option>
              <option value="Warning">Предупр.</option>
            </select>
          </div>
          <p style={{fontSize:11,color:"#ddb781"}}>
            Частичная семантика, не OEM-верификация.
          </p>
          <div style={{maxHeight:"45%",overflowY:"auto"}}>
            {diags.slice(0,250).map((d,i)=><button type="button" key={i}
              onClick={()=>void showLine(d.line)}
              style={{...itemStyle,whiteSpace:"normal",borderBottom:"1px solid #314557"}}>
              <b style={{color:d.severity==="Error"?"#f7a8aa":"#e6be83"}}>
                {d.severity==="Error"?"ERROR":"WARN"} · L{d.line}</b>
              <div>{d.code}</div><div style={{fontSize:11,color:"#b5c8d5"}}>{d.message}</div>
            </button>)}
            {!diags.length&&<small>Ошибок в распознанной части не найдено.</small>}
          </div>
          {(report.diagnostics.length>=500||report.semantic.diagnostics_truncated)&&
            <p style={{fontSize:11,color:"#e7bc83"}}>Количество диагностик ограничено.</p>}
          <hr style={{borderColor:"#3a4d5e"}}/>
          <div>Выбрана строка: {selectedLine??"—"}</div>
          <button type="button" onClick={reportAsJson} style={{...btn,marginTop:9,width:"100%"}}>
            JSON-отчёт диагностики
          </button>
          {json&&<textarea aria-label="JSON отчёт" readOnly value={json} rows={11}
            style={{...field,marginTop:8,width:"100%",boxSizing:"border-box",fontSize:11}}/>}
          <p style={{fontSize:11,color:"#d9b881"}}>
            В отчёте могут быть технологические адреса. Проверьте его перед отправкой.
          </p>
        </aside>
      </div>
      <footer style={{padding:"5px 12px",fontSize:11,color:"#b0c3d0",
        borderTop:"1px solid #364d5f",background:"#1b2b39"}}>
        SHA-256 исходника неизменен · Полный текст доступен страницами · Редактирование, FTP, HSES запрещены
      </footer>
    </>}
  </main>;
}
