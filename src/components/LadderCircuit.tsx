import { useMemo } from "react";
import { layoutCircuit, type GraphExpr, type ContactLocation } from "../ladder/graphModel.ts";

type Props={
  expression: GraphExpr;
  output: string;
  zoom?: number;
  selectedContactId?: string | null;
  onSelectContact?: (id:string)=>void;
  readOnly?: boolean;
};
const WIRE="#93a3b8";
const ACTIVE="#c7e09b";
const TEXT="#e4ecf3";

function ContactSymbol({ item, ox, oy, selected, onClick, readOnly }:{
  item:ContactLocation;ox:number;oy:number;selected:boolean;
  onClick?:()=>void;readOnly:boolean;
}) {
  const x=ox+item.x, y=oy+item.y+item.height/2;
  const stroke=selected?ACTIVE:WIRE;
  return <g role={readOnly?"img":"button"} tabIndex={readOnly?undefined:0}
    aria-label={`${item.normallyClosed?"NC":"NO"} ${item.relay}`}
    onClick={onClick}
    onKeyDown={event=>{
      if(!readOnly && (event.key==="Enter"||event.key===" ")) {
        event.preventDefault();onClick?.();
      }
    }}
    style={{cursor:readOnly?"default":"pointer",outline:"none"}}>
    <rect x={x} y={y-39} width={item.width} height={78} rx={6}
      fill={selected?"#354a3b":"transparent"} stroke={selected?ACTIVE:"transparent"} />
    <line x1={x} y1={y} x2={x+51} y2={y} stroke={stroke} strokeWidth={2}/>
    <line x1={x+93} y1={y} x2={x+item.width} y2={y} stroke={stroke} strokeWidth={2}/>
    <line x1={x+55} y1={y-18} x2={x+55} y2={y+18} stroke={stroke} strokeWidth={2.5}/>
    <line x1={x+89} y1={y-18} x2={x+89} y2={y+18} stroke={stroke} strokeWidth={2.5}/>
    {item.normallyClosed&&<line x1={x+50} y1={y+20} x2={x+94} y2={y-20}
      stroke={stroke} strokeWidth={2.2}/>}
    <text x={x+72} y={y+33} fontSize={13} fontFamily="monospace"
      fill={selected?ACTIVE:TEXT} textAnchor="middle">{item.relay}</text>
    <text x={x+72} y={y-26} fontSize={10} fill="#8fa0b1" textAnchor="middle">
      {item.normallyClosed?"NC":"NO"}
    </text>
  </g>;
}

/** Graphical ladder preview: AND=series, OR=parallel, with visible electrical rails.
 * It never claims that live I/O has been measured or that edited CIOPRG is valid.
 */
export function LadderCircuit({expression,output,zoom=1,selectedContactId=null,
  onSelectContact,readOnly=true}:Props) {
  const result=useMemo(()=>{
    try{return {layout:layoutCircuit(expression),error:""};}
    catch(e){return {layout:null,error:String(e)};}
  },[expression]);
  if (!result.layout) return <div role="alert" style={{color:"#ffb1b8"}}>
    Невозможно построить граф: {result.error}. Исходные данные не изменены.
  </div>;
  const {layout}=result;
  const ox=55,oy=45;
  const cy=oy+layout.height/2;
  const outX=ox+layout.width+86;
  const totalWidth=outX+115;
  const totalHeight=layout.height+oy*2;
  return <div style={{overflow:"auto",maxWidth:"100%",border:"1px solid #49505d",
    borderRadius:7,background:"#20252b"}}>
    <svg role="img" aria-label={`Схема цепи Ladder: ${output}`}
      viewBox={`0 0 ${totalWidth} ${totalHeight}`}
      width={totalWidth*zoom} height={totalHeight*zoom}
      style={{display:"block",minWidth:totalWidth*zoom,minHeight:totalHeight*zoom}}>
      <line x1={25} y1={24} x2={25} y2={totalHeight-24}
        stroke="#77919d" strokeWidth={3}/>
      <line x1={totalWidth-25} y1={24} x2={totalWidth-25} y2={totalHeight-24}
        stroke="#77919d" strokeWidth={3}/>
      <line x1={25} y1={cy} x2={ox} y2={cy} stroke={WIRE} strokeWidth={2}/>
      {layout.wires.map((w,i)=><line key={i}
        x1={ox+w.x1} y1={oy+w.y1}
        x2={ox+w.x2} y2={oy+w.y2}
        stroke={WIRE} strokeWidth={2}/>)}
      {layout.contacts.map(contact=><ContactSymbol
        key={contact.id} item={contact} ox={ox} oy={oy}
        selected={contact.id===selectedContactId}
        onClick={!readOnly?()=>onSelectContact?.(contact.id):undefined}
        readOnly={readOnly}/>)}
      <line x1={ox+layout.width} y1={cy} x2={outX-22} y2={cy}
        stroke={WIRE} strokeWidth={2}/>
      <path d={`M ${outX-20},${cy-19} Q ${outX-39},${cy} ${outX-20},${cy+19}`}
        fill="none" stroke={WIRE} strokeWidth={2.3}/>
      <path d={`M ${outX+20},${cy-19} Q ${outX+39},${cy} ${outX+20},${cy+19}`}
        fill="none" stroke={WIRE} strokeWidth={2.3}/>
      <line x1={outX+22} y1={cy} x2={totalWidth-25} y2={cy}
        stroke={WIRE} strokeWidth={2}/>
      <text x={outX} y={cy-27} fill="#e1c68c" fontSize={11} textAnchor="middle">OUT</text>
      <text x={outX} y={cy+37} fill={TEXT} fontSize={13}
        fontFamily="monospace" textAnchor="middle">{output}</text>
    </svg>
  </div>;
}
