import { useMemo } from "react";
import { layoutCircuit, type GraphExpr, type ContactLocation } from "../ladder/graphModel.ts";
import {
  contactLogicState, expressionLogicState, STATE_BACKGROUNDS, STATE_COLORS,
  type LogicState, type SignalSnapshot,
} from "../ladder/visualState.ts";

type Props={
  expression: GraphExpr;
  output: string;
  zoom?: number;
  selectedContactId?: string | null;
  onSelectContact?: (id:string)=>void;
  readOnly?: boolean;
  /** Present only in manually driven offline diagnostics; never live HSES. */
  signalValues?: SignalSnapshot;
  /** Responsive SVG with a single shared vertical workspace scroll. */
  fitToWidth?: boolean;
};
const WIRE="#8293a3";
const TEXT="#e4ecf3";
const SELECTED="#efc978";

function ContactSymbol({ item, ox, oy, selected, onClick, readOnly, state }:{
  item:ContactLocation;ox:number;oy:number;selected:boolean;
  onClick?:()=>void;readOnly:boolean;state:LogicState|null;
}) {
  const x=ox+item.x, y=oy+item.y+item.height/2;
  const stroke=state?STATE_COLORS[state]:WIRE;
  const border=selected?SELECTED:state?stroke:"transparent";
  const fill=state?STATE_BACKGROUNDS[state]:"transparent";
  const stateText=state===null?"без диагностики":state==="UNKNOWN"?"?":state==="TRUE"?"1":"0";
  return <g role={readOnly?"img":"button"} tabIndex={readOnly?undefined:0}
    aria-label={`${item.normallyClosed?"NC":"NO"} ${item.relay}, ${stateText}`}
    onClick={onClick}
    onKeyDown={event=>{
      if(!readOnly && (event.key==="Enter"||event.key===" ")) {
        event.preventDefault();onClick?.();
      }
    }}
    style={{cursor:readOnly?"default":"pointer",outline:"none"}}>
    <rect x={x+3} y={y-37} width={item.width-6} height={75} rx={7}
      fill={fill} stroke={border} strokeWidth={selected?2.5:1.2}/>
    <line x1={x} y1={y} x2={x+51} y2={y} stroke={stroke} strokeWidth={2.4}/>
    <line x1={x+93} y1={y} x2={x+item.width} y2={y} stroke={stroke} strokeWidth={2.4}/>
    <line x1={x+55} y1={y-18} x2={x+55} y2={y+18} stroke={stroke} strokeWidth={3}/>
    <line x1={x+89} y1={y-18} x2={x+89} y2={y+18} stroke={stroke} strokeWidth={3}/>
    {item.normallyClosed&&<line x1={x+50} y1={y+20} x2={x+94} y2={y-20}
      stroke={stroke} strokeWidth={2.5}/>}
    <text x={x+72} y={y+33} fontSize={13} fontFamily="monospace"
      fill={TEXT} textAnchor="middle">{item.relay}</text>
    <text x={x+72} y={y-25} fontSize={11} fill={stroke} textAnchor="middle">
      {item.normallyClosed?"NC":"NO"}{state!==null?` · ${stateText}`:""}
    </text>
  </g>;
}

/** AND = series, OR = parallel. Colored contacts indicate Boolean conduction
 * computed from explicit offline test bits; coil indicates expression result.
 * Source-only inspector without signals stays neutral, not deceptively TRUE.
 */
export function LadderCircuit({expression,output,zoom=1,selectedContactId=null,
  onSelectContact,readOnly=true,signalValues,fitToWidth=false}:Props) {
  const result=useMemo(()=>{
    try{return {layout:layoutCircuit(expression),error:""};}
    catch(e){return {layout:null,error:String(e)};}
  },[expression]);
  if (!result.layout) return <div role="alert" style={{color:"#ffb1b8"}}>
    Невозможно построить граф: {result.error}. Исходные данные не изменены.
  </div>;
  const {layout}=result;
  const ox=55,oy=43;
  const cy=oy+layout.height/2;
  const outX=ox+layout.width+86;
  const totalWidth=outX+115;
  const totalHeight=layout.height+oy*2;
  const coilState=signalValues?expressionLogicState(expression,signalValues):null;
  const coilStroke=coilState?STATE_COLORS[coilState]:WIRE;
  const coilFill=coilState?STATE_BACKGROUNDS[coilState]:"transparent";
  return <div style={{overflowX:"auto",overflowY:"hidden",maxWidth:"100%",
    border:"1px solid #465465",borderRadius:7,background:"#172028"}}>
    <svg role="img" aria-label={`Схема цепи Ladder ${output}. ${coilState ?? "Без диагностики"}`}
      viewBox={`0 0 ${totalWidth} ${totalHeight}`}
      width={totalWidth*zoom} height={totalHeight*zoom}
      preserveAspectRatio="xMinYMid meet"
      style={{
        display:"block",
        width:fitToWidth?"100%":totalWidth*zoom,
        height:fitToWidth?"auto":totalHeight*zoom,
        minWidth:fitToWidth?Math.min(totalWidth*zoom,540):totalWidth*zoom,
      }}>
      <line x1={25} y1={20} x2={25} y2={totalHeight-20}
        stroke="#708999" strokeWidth={3}/>
      <line x1={totalWidth-25} y1={20} x2={totalWidth-25} y2={totalHeight-20}
        stroke="#708999" strokeWidth={3}/>
      <line x1={25} y1={cy} x2={ox} y2={cy} stroke={WIRE} strokeWidth={2}/>
      {layout.wires.map((w,i)=><line key={i}
        x1={ox+w.x1} y1={oy+w.y1}
        x2={ox+w.x2} y2={oy+w.y2}
        stroke={WIRE} strokeWidth={2}/>)}
      {layout.contacts.map(contact=><ContactSymbol
        key={contact.id} item={contact} ox={ox} oy={oy}
        selected={contact.id===selectedContactId}
        state={signalValues?contactLogicState(contact,signalValues):null}
        onClick={!readOnly?()=>onSelectContact?.(contact.id):undefined}
        readOnly={readOnly}/>)}
      <line x1={ox+layout.width} y1={cy} x2={outX-22} y2={cy}
        stroke={WIRE} strokeWidth={2}/>
      <rect x={outX-35} y={cy-41} width={70} height={84}
        rx={8} fill={coilFill} stroke={coilState?coilStroke:"transparent"} strokeWidth={1.5}/>
      <path d={`M ${outX-20},${cy-19} Q ${outX-39},${cy} ${outX-20},${cy+19}`}
        fill="none" stroke={coilStroke} strokeWidth={3}/>
      <path d={`M ${outX+20},${cy-19} Q ${outX+39},${cy} ${outX+20},${cy+19}`}
        fill="none" stroke={coilStroke} strokeWidth={3}/>
      <line x1={outX+22} y1={cy} x2={totalWidth-25} y2={cy}
        stroke={WIRE} strokeWidth={2}/>
      <text x={outX} y={cy-28} fill={coilStroke} fontSize={11} textAnchor="middle">
        OUT{coilState?` · ${coilState==="UNKNOWN"?"?":coilState==="TRUE"?"1":"0"}`:""}
      </text>
      <text x={outX} y={cy+37} fill={TEXT} fontSize={13}
        fontFamily="monospace" textAnchor="middle">{output}</text>
    </svg>
  </div>;
}
