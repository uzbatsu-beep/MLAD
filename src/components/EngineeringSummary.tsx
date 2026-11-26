import type { EngineeringCore } from "../engineering/types.ts";

type Props={
  core:EngineeringCore;
  selectedRung:number;
  activeTab:"source"|"diagram"|"blocks"|"ioname";
  onSource:()=>void;
  onLadder:()=>void;
  onLocate:(line:number)=>void;
};

/** Compact workbench context; never presents model values as live I/O. */
export function EngineeringSummary({core,selectedRung,activeTab,onSource,onLadder,onLocate}:Props){
  const {identity,evidence}=core;
  const selected=core.rung_refs.find(ref=>ref.number===selectedRung);
  return <section className="engineering-summary" aria-label="Инженерный контекст">
    <div className="engineering-summary-context">
      <strong>{identity.controller} <span className="engineering-summary-badge">OFFLINE · READ ONLY</span></strong>
      <small title={identity.source_sha256}>Снимок {identity.source_sha256.slice(0,12)}… · {identity.line_count.toLocaleString("ru-RU")} строк</small>
    </div>
    <div className="engineering-summary-metrics">
      <span><b>{core.rung_count}</b> цепей</span>
      <span><b>{core.rung_refs.length}</b> в предпросмотре</span>
      <span className={evidence.unsupported_segments?"engineering-summary-warning":""}>
        <b>{evidence.unsupported_segments}</b> неизвестных сегментов
      </span>
    </div>
    <div className="engineering-summary-state">
      <strong className={evidence.boolean_subset_complete?"":"engineering-summary-warning"}>
        {evidence.boolean_subset_complete?"Булево подмножество проанализировано":"Неполный семантический анализ"}
      </strong>
      <small>Исполнение OEM не проверено · не телеметрия</small>
    </div>
    <nav className="engineering-summary-actions" aria-label="Быстрая навигация">
      <button type="button" aria-pressed={activeTab==="source"} onClick={onSource}>Исходник</button>
      <button type="button" aria-pressed={activeTab==="diagram"} onClick={onLadder}>Ladder</button>
      {activeTab==="diagram"&&selected&&<button type="button"
        title={"Байты "+selected.source.bytes.start+"–"+selected.source.bytes.end}
        onClick={()=>onLocate(selected.source.first_line)}>R{selected.number} → L{selected.source.first_line}</button>}
    </nav>
    {core.rung_refs_truncated&&<small className="engineering-summary-limit">
      Показана часть цепей. Полный файл доступен во вкладке «Исходный текст»; это не предел контроллера.
    </small>}
  </section>;
}
