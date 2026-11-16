import { useState } from "react";
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
  section: "Unspecified" | "System" | "User";
};
type SemanticPreview = {
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
type Report = {
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

  async function inspect() {
    setIssue("");
    setReport(null);
    setBusy(true);
    try {
      const result = await invoke<Report>("cmd_inspect_cioprg", { filePath: path.trim(), controller });
      setReport(result);
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
