import {useEffect,useRef,useState} from "react";
import {inlineOperandError,type InlineOperandKind} from "../ladder/inlineOperand.ts";

/** Ephemeral in-SVG editor: Enter commits, Escape/blur cancel. Non-native drafts only.
 * Reads no device state and does not touch source CIOPRG.LST.
 */
export function SvgOperandInput({x,y,value,kind,onCommit,onCancel,width=140}:{
  x:number;y:number;value:string;kind:InlineOperandKind;
  onCommit:(next:string)=>void;onCancel:()=>void;width?:number;
}) {
  const [draft,setDraft]=useState(value);
  const [error,setError]=useState<string|null>(null);
  const ref=useRef<HTMLInputElement>(null);
  useEffect(()=>{
    ref.current?.focus();
    ref.current?.select();
  },[]);
  function commit(){
    const normalized=draft.trim();
    const issue=inlineOperandError(normalized,kind);
    if(issue){setError(issue);return;}
    onCommit(normalized);
  }
  return <foreignObject x={x-width/2} y={y-15} width={width} height={error?69:43}
    style={{overflow:"visible"}}>
    <div
      style={{background:"#18232d",border:"2px solid #e4c582",borderRadius:6,
        padding:3,boxSizing:"border-box",fontFamily:"sans-serif"}}
      onMouseDown={e=>e.stopPropagation()}
      onDoubleClick={e=>e.stopPropagation()}>
      <input ref={ref} value={draft} maxLength={24}
        aria-label="Редактирование адреса на схеме, Enter — подтвердить, Esc — отменить"
        onChange={e=>{setDraft(e.target.value);setError(null);}}
        onKeyDown={e=>{
          e.stopPropagation();
          if(e.key==="Enter"){e.preventDefault();commit();}
          if(e.key==="Escape"){e.preventDefault();onCancel();}
        }}
        onBlur={onCancel}
        style={{width:"100%",boxSizing:"border-box",height:26,
          fontSize:14,fontFamily:"monospace",fontWeight:700,
          background:"#f3f7fa",color:"#18212a",border:"none",borderRadius:3,
          textAlign:"center",outline:"none"}}/>
      {error&&<div role="alert" style={{fontSize:11,color:"#ffb1ba",maxWidth:250,
        background:"#18232d",padding:"3px 1px"}}>{error}</div>}
    </div>
  </foreignObject>;
}
