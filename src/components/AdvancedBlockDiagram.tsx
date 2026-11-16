import type {AdvancedBlock, Preset} from "../ladder/advancedModel.ts";
import {describeCondition} from "../ladder/advancedModel.ts";

const W="#a4b8cb", FG="#e6edf3",ACC="#ddb775";
function presetLabel(p:Preset):string {
  return p.kind==="literal"?String(p.value):p.address;
}

/** Advanced block is never drawn as a mere contact-to-coil wire.
 * TMR has timing state, CNT has C and R ports, GRP copies exactly one byte. */
export function AdvancedBlockDiagram({block}:{block:AdvancedBlock}) {
  const isCnt=block.kind==="CNT";
  const isGrp=block.kind==="GROUP";
  const h=isCnt?250:175;
  const source=isGrp?block.source:
    block.kind==="TMR"?describeCondition(block.enable):describeCondition(block.count);
  const reset=isCnt?describeCondition(block.reset):"";
  const output=isGrp?block.destination:block.output;
  const title=isGrp?"GRP (GSTR + GOUT)":block.kind;
  const current=isGrp?block.source:block.current;
  const second=isGrp?block.destination:presetLabel(block.preset);
  return <div style={{overflowX:"auto",border:"1px solid #4d5d6e",background:"#1e2832",borderRadius:8}}>
    <svg viewBox={`0 0 740 ${h}`} width="740" height={h}
      role="img" aria-label={`Схема блока ${title}. Только offline`}>
      <line x1={30} y1={20} x2={30} y2={h-20} stroke={W} strokeWidth={3}/>
      <line x1={710} y1={20} x2={710} y2={h-20} stroke={W} strokeWidth={3}/>
      <rect x={290} y={isCnt?50:36} width={182} height={isCnt?165:110}
        rx={8} stroke="#a9c0d0" strokeWidth={2} fill="#293a46"/>
      <text x={381} y={isCnt?82:66} fontSize={18} fontWeight={700}
        textAnchor="middle" fill={ACC}>{title}</text>
      <text x={381} y={isCnt?112:98} fontSize={15} fontFamily="monospace"
        textAnchor="middle" fill={FG}>{current}</text>
      <text x={381} y={isCnt?142:124} fontSize={14} fontFamily="monospace"
        textAnchor="middle" fill="#9bc7d8">{isGrp?"8 бит":`PRESET ${second}`}</text>
      {isCnt&&<text x={381} y={184} fontSize={11} fill="#cbdeea" textAnchor="middle">
        Сброс имеет приоритет
      </text>}
      <line x1={30} y1={isCnt?104:90} x2={290} y2={isCnt?104:90} stroke={W} strokeWidth={2}/>
      <line x1={472} y1={isCnt?130:90} x2={710} y2={isCnt?130:90} stroke={W} strokeWidth={2}/>
      <text x={165} y={isCnt?80:65} fill={FG} fontSize={12} fontFamily="monospace"
        textAnchor="middle">{source.length>28?source.slice(0,28)+"…":source}</text>
      <text x={277} y={isCnt?97:83} fill={ACC} fontSize={12} textAnchor="end">{isCnt?"C":isGrp?"GSTR":"IN"}</text>
      <text x={492} y={isCnt?123:83} fill={ACC} fontSize={12}>{isGrp?"GOUT":"Q"}</text>
      <text x={590} y={isCnt?109:72} fontFamily="monospace" fontSize={14}
        textAnchor="middle" fill={FG}>{output}</text>
      {isCnt&&<>
        <line x1={30} y1={190} x2={290} y2={190} stroke={W} strokeWidth={2}/>
        <text x={165} y={174} fill={FG} fontFamily="monospace"
          fontSize={12} textAnchor="middle">{reset.length>28?reset.slice(0,28)+"…":reset}</text>
        <text x={277} y={183} fill={ACC} textAnchor="end" fontSize={12}>R</text>
      </>}
      <text x={32} y={h-8} fill="#90a4b2" fontSize={10}>
        Схематическое представление. Не проверено на реальном контроллере.
      </text>
    </svg>
  </div>;
}
