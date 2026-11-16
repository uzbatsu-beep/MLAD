import type {AdvancedBlock, Preset} from "../ladder/advancedModel.ts";
import {describeCondition} from "../ladder/advancedModel.ts";
import {STATE_BACKGROUNDS,STATE_COLORS,type LogicState} from "../ladder/visualState.ts";

const W="#8799a8", FG="#e6edf3", ACC="#ddb775";
function presetLabel(p:Preset):string {
  return p.kind==="literal"?String(p.value):p.address;
}
const short=(text:string,max=30)=>text.length>max?text.slice(0,max-1)+"…":text;
type Props={
  block:AdvancedBlock;
  outputState?:LogicState;
  inputState?:LogicState;
  resetState?:LogicState;
  currentValue?:number|null;
  groupByte?:number|null;
};
function stateMarker(state:LogicState,x:number,y:number,name:string){
  const color=STATE_COLORS[state];
  return <g>
    <circle cx={x} cy={y} r={8} fill={STATE_BACKGROUNDS[state]}
      stroke={color} strokeWidth={2}/>
    <text x={x+16} y={y+4} fill={color} fontSize={12}>
      {name}={state==="UNKNOWN"?"?":state==="TRUE"?"1":"0"}
    </text>
  </g>;
}

/** TMR, CNT, GRP are stateful functional blocks, NOT plain relay contacts.
 * Green/red/grey status is EXCLUSIVELY from explicit offline single-step models.
 * A missing sample never implies zero, inactive outputs, or hardware status.
 * Layout uses full SVG height and hides overflow-Y to prevent CNT clipping.
 */
export function AdvancedBlockDiagram({block,outputState="UNKNOWN",inputState="UNKNOWN",
  resetState="UNKNOWN",currentValue=null,groupByte=null}:Props) {
  const isCnt=block.kind==="CNT";
  const isGrp=block.kind==="GROUP";
  const h=isCnt?246:isGrp?211:185;
  const centerY=isCnt?115:84;
  const source=isGrp?block.source:
    block.kind==="TMR"?describeCondition(block.enable):describeCondition(block.count);
  const reset=isCnt?describeCondition(block.reset):"";
  const output=isGrp?block.destination:block.output;
  const title=isGrp?"GSTR → GOUT":block.kind;
  const current=isGrp?block.source:block.current;
  const second=isGrp?"8 бит":presetLabel(block.preset);
  const Q=STATE_COLORS[outputState];
  return <div style={{overflowX:"auto",overflowY:"hidden",
    border:"1px solid #526476",background:"#17232e",borderRadius:8,
    minHeight:0,width:"100%"}}>
    <svg viewBox={`0 0 740 ${h}`} width="740" height={h}
      preserveAspectRatio="xMinYMid meet" role="img"
      aria-label={`Офлайн-схема ${title}, выход ${outputState}`}
      style={{display:"block",width:"100%",maxWidth:900,minWidth:610,height:"auto"}}>
      <line x1={22} y1={18} x2={22} y2={h-18} stroke="#70899b" strokeWidth={3}/>
      <line x1={718} y1={18} x2={718} y2={h-18} stroke="#70899b" strokeWidth={3}/>
      <rect x={292} y={isCnt?48:29} width={180} height={isCnt?165:isGrp?132:122}
        rx={8} fill={STATE_BACKGROUNDS[outputState]} stroke={Q} strokeWidth={2.7}/>
      <text x={382} y={isCnt?77:54} fontSize={18} fontWeight={700}
        textAnchor="middle" fill={ACC}>{title}</text>
      <text x={382} y={isCnt?107:83} fontSize={15} fontFamily="monospace"
        textAnchor="middle" fill={FG}>{current}</text>
      <text x={382} y={isCnt?136:112} fontSize={13} fontFamily="monospace"
        textAnchor="middle" fill="#bfdbea">
        {isGrp?"BYTE TRANSFER":`PRESET ${second}`}
      </text>
      {!isGrp&&<text x={382} y={isCnt?164:138} fontSize={12}
        fontFamily="monospace" textAnchor="middle" fill={Q}>
        Current={currentValue===null?"?":currentValue} · Q={outputState==="UNKNOWN"?"?":outputState==="TRUE"?"1":"0"}
      </text>}
      {isCnt&&<text x={382} y={197} fontSize={12} fill="#d0ddec" textAnchor="middle">
        RESET приоритетнее COUNT
      </text>}
      <line x1={22} y1={centerY} x2={292} y2={centerY} stroke={W} strokeWidth={2}/>
      <line x1={472} y1={centerY} x2={718} y2={centerY} stroke={W} strokeWidth={2}/>
      <text x={155} y={centerY-18} fill={FG} fontSize={13} fontFamily="monospace"
        textAnchor="middle">{short(source,29)}</text>
      <text x={282} y={centerY-6} fill={STATE_COLORS[inputState]}
        fontWeight={700} fontSize={13} textAnchor="end">
        {isCnt?"C":isGrp?"GSTR":"IN"}
      </text>
      <text x={483} y={centerY-6} fill={Q} fontWeight={700} fontSize={13}>
        {isGrp?"GOUT":"Q"}
      </text>
      <text x={586} y={centerY-18} fontFamily="monospace" fontSize={14}
        textAnchor="middle" fill={FG}>{output}</text>
      {stateMarker(inputState,115,centerY+20,isCnt?"C":isGrp?"IN":"IN")}
      {!isGrp&&stateMarker(outputState,550,centerY+22,"Q")}
      {isCnt&&<>
        <line x1={22} y1={185} x2={292} y2={185} stroke={W} strokeWidth={2}/>
        <text x={155} y={167} fill={FG} fontFamily="monospace"
          fontSize={13} textAnchor="middle">{short(reset,28)}</text>
        <text x={282} y={177} fill={STATE_COLORS[resetState]}
          fontWeight={700} textAnchor="end" fontSize={13}>R</text>
        {stateMarker(resetState,115,205,"R")}
      </>}
      {isGrp&&<>
        <text x={382} y={139} fill={Q} fontSize={12} textAnchor="middle">
          {groupByte===null?"Байт назначения: неизвестен":
            `0x${groupByte.toString(16).padStart(2,"0").toUpperCase()}`}
        </text>
        {Array.from({length:8},(_,i)=>{
          const bit=7-i,x=183+i*54,y=172;
          const state:LogicState=groupByte===null?"UNKNOWN":
            (groupByte & (1<<bit))!==0?"TRUE":"FALSE";
          return <g key={bit}>
            <circle cx={x} cy={y} r={9} fill={STATE_BACKGROUNDS[state]}
              stroke={STATE_COLORS[state]} strokeWidth={2}/>
            <text x={x} y={y+25} textAnchor="middle" fill={FG} fontSize={11}>b{bit}</text>
          </g>;
        })}
      </>}
      <text x={24} y={h-7} fill="#98aabb" fontSize={10}>
        OFFLINE · Данные введены вручную · Не подтверждено на роботе
      </text>
    </svg>
  </div>;
}
