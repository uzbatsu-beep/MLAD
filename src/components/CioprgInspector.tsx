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

type Report = {
  advanced: AdvancedSourceReport;
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

export function CioprgInspector({ onClose }: { onClose: () => void }) {
  const [path, setPath] = useState("");
  const [controller, setController] = useState<Controller>("YRC1000");
  const [report, setReport] = useState<Report | null>(null);
  const [issue, setIssue] = useState("");
  const [busy, setBusy] = useState(false);
  const [selectedGraphRung, setSelectedGraphRung] = useState(1);
  const [diagramZoom, setDiagramZoom] = useState(1);
  const graphPreview = useMemo(() => {
    if (!report) return null;
    const rung=report.semantic.rungs.find(r=>r.number===selectedGraphRung)??report.semantic.rungs[0];
    if(!rung) return null;
    try{return {rung,expression:semanticToExpression(report.semantic.nodes,rung.expression_root)};}
    catch(err){return {rung,error:String(err)};}
  },[report,selectedGraphRung]);

  async function inspect() {
    setIssue("");
    setReport(null);
    setBusy(true);
    try {
      const result = await invoke<Report>("cmd_inspect_cioprg", { filePath: path.trim(), controller });
      setReport(result);
      setSelectedGraphRung(1);
    } catch (err) {
      setIssue(String(err));
    } finally {
      setBusy(false);
    }
  }

  return (
    <div style={{
      position: "absolute", inset: 0, zIndex: 20, background: "#17191dec",
      overflowY: "auto", padding: "22px", color: "#e5e9ef",
    }}>
      <div style={{ maxWidth: "1250px", margin: "0 auto", display: "grid", gap: "14px" }}>
        <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", gap: "12px" }}>
          <div>
            <h2 style={{ margin: "0 0 6px" }}>CIOPRG.LST — просмотр без изменений</h2>
            <div style={{ color: "#d9bb7a", fontSize: "13px" }}>
              Только чтение. Определение System/User Ladder не подтверждено.
              Компиляция, редактирование и загрузка в робот запрещены.
            </div>
          </div>
          <button type="button" onClick={onClose} style={btn}>Закрыть</button>
        </div>
        <div style={{ ...panel, display: "flex", flexWrap: "wrap", gap: "10px", alignItems: "center" }}>
          <label style={{ flex: "1 1 350px", display: "grid", gap: "6px" }}>
            <span style={{ fontSize: "13px" }}>Путь к локальному файлу CIOPRG.LST</span>
            <input value={path} onChange={e => setPath(e.target.value)} placeholder="C:\\backup\\CIOPRG.LST"
              style={{ ...field, width: "100%", boxSizing: "border-box" }}/>
          </label>
          <label style={{ display: "grid", gap: "6px" }}>
            <span style={{ fontSize: "13px" }}>Контроллер</span>
            <select style={field} value={controller} onChange={e => setController(e.target.value as Controller)}>
              <option value="DX100">DX100</option>
              <option value="DX200">DX200</option>
              <option value="YRC1000">YRC1000</option>
            </select>
          </label>
          <button type="button" style={{ ...btn, alignSelf: "end" }} disabled={busy || !path.trim()} onClick={inspect}>
            {busy ? "Анализ..." : "Открыть в режиме чтения"}
          </button>
        </div>
        {issue && <div style={{ ...panel, borderColor: "#cf6679", color: "#ffb5bc" }}>{issue}</div>}
        {report && (
          <>
            <div style={panel}>
              <strong>{report.controller}</strong> · {report.byte_length.toLocaleString("ru-RU")} байт ·
              {" "}{report.line_count} строк · {report.recognized_instructions} распознано ·
              {" "}{report.unsupported_lines} неизвестных / непрозрачных ·
              кодировка: {report.encoding}
              <div style={{ marginTop: "8px", color: "#d9bb7a" }}>
                Только побайтовое сохранение исходного файла. Экспорт изменённой программы запрещён.
                {" "}Границы System/User Ladder не проверены.
              </div>
              <div style={{ marginTop: "8px", fontSize: "12px", fontFamily: "monospace", overflowWrap: "anywhere" }}>
                SHA-256 исходного файла: {report.source_sha256}
              </div>
              <div style={{ marginTop: "8px", fontSize: "12px" }}>
                Секции //$M: {report.m_headers.length}.{" "}
                {report.m_headers.some(x => !x.complete) && "Есть повреждённые или неполные секции."}
              </div>
            </div>
            <div style={panel}>
              <strong>Semantic AST · анализ логики</strong>
              <div style={{ color: "#d9bb7a", margin: "8px 0", fontSize: "13px" }}>
                Только проверенное булево подмножество. Анализ не моделирует scan-time, состояние TMR/CNT,
                выполнение INFORM или фактические I/O. Метки PART — не подтверждение права редактирования.
              </div>
              <div>
                Цепей: {report.semantic.rung_count} · узлов графа: {report.semantic.node_count}
                {" "}· символов: {report.semantic.reference_count} · неполных участков: {report.semantic.unsupported_segments}
                {" "}· семантических диагностик: {report.semantic.diagnostic_count}
              </div>
              <div style={{ margin: "6px 0", color: report.semantic.subset_analysis_complete ? "#9bcca6" : "#ffb785" }}>
                {report.semantic.subset_analysis_complete
                  ? "Распознанное булево подмножество обработано без ошибок. Проверка контроллера всё ещё необходима."
                  : "Анализ неполный: есть неподдерживаемые инструкции или ошибки."}
              </div>
              <div style={{ maxHeight: "175px", overflow: "auto", fontSize: "12px", fontFamily: "monospace" }}>
                {report.semantic.rungs.map(r => (
                  <div key={r.number} style={{ padding: "4px 0" }}>
                    R{r.number} [{r.section}] L{r.first_line}–{r.output_line}:
                    {" "}{r.expression_preview}
                    {" "}→ OUT #{String(r.output).padStart(5, "0")}
                  </div>
                ))}
                {report.semantic.rungs_truncated && <div>… список цепей ограничен</div>}
              </div>
              <details style={{ marginTop: "8px", fontSize: "12px" }}>
                <summary>Перекрёстные ссылки ({report.semantic.reference_count} символов)</summary>
                <div style={{ maxHeight: "160px", overflow: "auto", fontFamily: "monospace" }}>
                  {report.semantic.references.map(x => (
                    <div key={x.symbol}>{x.symbol} : {x.sites.map(s => "L" + s.line + " " + s.operation).join(" · ")}</div>
                  ))}
                  {report.semantic.references_truncated && <div>… список ограничен</div>}
                </div>
              </details>
              <div style={{ maxHeight: "170px", overflowY: "auto", marginTop: "8px", fontSize: "12px", fontFamily: "monospace" }}>
                {report.semantic.diagnostics.map((d, i) => (
                  <div key={i} style={{ padding: "3px 0", color: d.level === "Error" ? "#ff959f" : "#e9c47a" }}>
                    {d.level} L{d.line} [{d.code}]: {d.message}
                  </div>
                ))}
                {report.semantic.diagnostics_truncated && <div>… диагностика ограничена</div>}
              </div>
            </div>
            <div style={panel}>
              <strong>Графическая схема CIOPRG.LST — только чтение</strong>
              <p style={{color:"#d9bb7a",fontSize:12}}>
                Граф построен по подтверждённой булевой части Semantic AST. Неподдерживаемые
                и пропущенные участки не превращаются в предполагаемую схему.
                Схема не является подтверждённой Ladder-программой для контроллера.
              </p>
              <div style={{display:"flex",gap:10,alignItems:"center",flexWrap:"wrap",marginBottom:10}}>
                <label>Цепь{" "}
                  <select style={field} value={graphPreview?.rung.number??""}
                    onChange={e=>setSelectedGraphRung(Number(e.target.value))}>
                    {report.semantic.rungs.map(r=><option key={r.number} value={r.number}>
                      R{r.number} · OUT {relayText(r.output)} · строки {r.first_line}–{r.output_line}
                    </option>)}
                  </select>
                </label>
                <button type="button" style={btn} onClick={()=>setDiagramZoom(z=>Math.max(.5,z-.15))}>−</button>
                <span>{Math.round(diagramZoom*100)}%</span>
                <button type="button" style={btn} onClick={()=>setDiagramZoom(z=>Math.min(1.8,z+.15))}>+</button>
              </div>
              {graphPreview ? ("expression" in graphPreview && graphPreview.expression ?
                <LadderCircuit readOnly zoom={diagramZoom}
                  expression={graphPreview.expression}
                  output={relayText(graphPreview.rung.output)} /> :
                <div role="alert" style={{color:"#edbd78"}}>
                  Недоступен граф этой цепи: {"error" in graphPreview ? graphPreview.error : "нет данных AST"}.
                  Исходный текст сохранён без изменений.
                </div>) :
                <div>Нет цепей с полностью распознанным булевым выражением.</div>}
              {report.semantic.nodes_truncated && <p style={{color:"#edbd78",fontSize:12}}>
                AST ограничен до 4096 узлов: часть схем может быть недоступна для отображения.
              </p>}
            </div>
            <div style={panel}>
              <strong>Advanced CIO · TMR / CNT / GSTR + GOUT</strong>
              <p style={{fontSize:12,color:"#d9bb7a"}}>
                Предпросмотр только типовых последовательностей из руководств. Неподдерживаемая
                логическая топология не реконструируется; ни один блок нельзя изменить
                или загрузить из этой панели.
              </p>
              <div style={{display:"grid",gap:15}}>
                {report.advanced.blocks.map(b=>{
                  const overlay=toReadOnlyAdvanced(b);
                  return <div key={b.line} style={{background:"#222931",border:"1px solid #475667",
                    borderRadius:7,padding:10}}>
                    <div style={{display:"flex",justifyContent:"space-between",
                      gap:8,flexWrap:"wrap",marginBottom:8}}>
                      <span style={{fontFamily:"monospace"}}>
                        {b.kind==="GroupCopy"?"GSTR/GOUT":b.kind==="Timer"?"TMR":"CNT"} ·
                        строки {b.line}–{b.end_line} · {b.operands.join(", ")}
                      </span>
                      <span style={{color:b.example_pattern_recognized?"#b4d4ac":"#e9b87b"}}>
                        {b.example_pattern_recognized?"Пример распознан":"Топология не подтверждена"}
                      </span>
                    </div>
                    {overlay?<AdvancedBlockDiagram block={overlay}/>:
                      <div role="alert" style={{color:"#e9b87b",fontSize:12}}>
                        Граф не сформирован: неподтверждённая или неполная последовательность.
                      </div>}
                  </div>;
                })}
                {!report.advanced.blocks.length&&<div>Advanced-блоки в распознанном файле не найдены.</div>}
                {report.advanced.blocks_truncated&&<div style={{color:"#e9b87b"}}>
                  Показано только ограниченное число блоков.</div>}
              </div>
              <div style={{fontFamily:"monospace",fontSize:12,marginTop:9}}>
                {report.advanced.warnings.map((w,i)=><div key={i} style={{color:"#e9b87b",
                  padding:"3px 0"}}>L{w.line} [{w.code}] {w.message}</div>)}
              </div>
            </div>
            <div style={{ ...panel, maxHeight: "230px", overflowY: "auto" }}>
              <strong>Диагностика ({report.diagnostics.length} показано)</strong>
              {report.diagnostics.length === 0 && <p>В распознанном подмножестве ошибок не найдено. Это не подтверждает совместимость файла с контроллером.</p>}
              {report.diagnostics.map((d, i) =>
                <div key={i} style={{ fontFamily: "monospace", fontSize: "12px", padding: "4px 0", color: d.severity === "Error" ? "#ff959f" : "#e9c47a" }}>
                  {d.severity === "Error" ? "ERROR" : "WARN"} L{d.line} @byte{d.byte_offset} [{d.code}]: {d.message}
                </div>
              )}
            </div>
            <div style={panel}>
              <strong>Предпросмотр (максимум 600 строк; текст только для чтения)</strong>
              {report.preview_truncated && <div style={{ color: "#e9c47a" }}>Показана только начальная часть файла.</div>}
              <div style={{ marginTop: "10px", maxHeight: "55vh", overflow: "auto", fontFamily: "monospace", fontSize: "12px" }}>
                {report.lines.map(l => <div key={l.line} style={{ display: "flex", gap: "10px", whiteSpace: "pre" }}>
                  <span style={{ color: "#7f93a6", minWidth: "46px", textAlign: "right", userSelect: "none" }}>{l.line}</span>
                  <span style={{ color: "#8daeb9", width: "145px", flexShrink: 0, userSelect: "none" }}>{lineCategory(l.kind)}</span>
                  <span>{l.display_text || " "}</span>
                </div>)}
              </div>
            </div>
          </>
        )}
      </div>
    </div>
  );
}
