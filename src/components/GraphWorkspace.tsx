import { useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { LadderCircuit } from "./LadderCircuit.tsx";
import {
  addBranch, appendDraftRung, changeAt, deleteContact, evaluateDraft,
  findContact, newDraft, type GraphDraft, type DraftRung, type ContactExpr,
} from "../ladder/graphModel.ts";
import {
  expressionLogicState, nextManualSignal, STATE_BACKGROUNDS,
  STATE_COLORS, STATE_LABELS,
} from "../ladder/visualState.ts";

type DraftDiagnostic={
  rung_id:string;
  level:"Error"|"Warning";
  code:string;
  message:string;
};
type DraftValidation={
  diagnostics:DraftDiagnostic[];
  structure_valid:boolean;
  has_errors:boolean;
  edited_cioprg_export_allowed:boolean;
};
type History={project:GraphDraft; past:GraphDraft[]; future:GraphDraft[]};
const BTN:React.CSSProperties={border:"1px solid #617282",color:"#e9f2fa",background:"#344457",
  padding:"7px 10px",borderRadius:5,cursor:"pointer",fontSize:12};
const INPUT:React.CSSProperties={background:"#1b252d",color:"#eef3f8",border:"1px solid #5b6b7b",
  padding:"7px",borderRadius:5,fontFamily:"monospace",minWidth:0};
const PANEL:React.CSSProperties={border:"1px solid #414e5d",background:"#252c34",
  padding:13,borderRadius:8};
const MAX_HISTORY=35;

function allContacts(rung:DraftRung):ContactExpr[] {
  const result:ContactExpr[]=[];
  const stack=[rung.expression];
  while(stack.length) {
    const n=stack.pop()!;
    if(n.kind==="contact") result.push(n);
    else stack.push(n.right,n.left);
  }
  return result;
}

export function GraphWorkspace() {
  const [editor,setEditor]=useState<History>(()=>({project:newDraft(),past:[],future:[]}));
  const [activeRungId,setActiveRungId]=useState<string|null>(()=>null);
  const [selectedId,setSelectedId]=useState<string|null>(null);
  const [zoom,setZoom]=useState(1);
  const [filePath,setFilePath]=useState("ladder.graph.json");
  const [status,setStatus]=useState("Создан локальный графический черновик.");
  const [validation,setValidation]=useState<DraftValidation|null>(null);
  const [signals,setSignals]=useState<Record<string,boolean|undefined>>({});
  const project=editor.project;
  const selectedRung=project.rungs.find(r=>r.id===activeRungId)??project.rungs[0]??null;
  const selectedContact=selectedRung&&selectedId?
    findContact(selectedRung.expression,selectedId):null;

  function edit(transform:(old:GraphDraft)=>GraphDraft) {
    setEditor(previous=>{
      const next=transform(previous.project);
      return next===previous.project?previous:{
        project:next,past:[...previous.past.slice(-(MAX_HISTORY-1)),previous.project],future:[],
      };
    });
  }
  function editRung(transform:(old:DraftRung)=>DraftRung) {
    if(!selectedRung)return;
    const id=selectedRung.id;
    edit(old=>({...old,rungs:old.rungs.map(r=>r.id===id?transform(r):r)}));
  }
  function undo() {
    setEditor(s=>{
      if(!s.past.length)return s;
      const last=s.past[s.past.length-1];
      return {project:last,past:s.past.slice(0,-1),future:[s.project,...s.future]};
    });
    setSelectedId(null);
  }
  function redo() {
    setEditor(s=>{
      if(!s.future.length)return s;
      const [next,...future]=s.future;
      return {project:next,past:[...s.past,s.project].slice(-MAX_HISTORY),future};
    });
    setSelectedId(null);
  }

  useEffect(()=>{
    let cancelled=false;
    const timer=setTimeout(()=>{
      invoke<DraftValidation>("cmd_validate_graph_draft",{project})
        .then(r=>{if(!cancelled)setValidation(r);})
        .catch(err=>{if(!cancelled)setStatus("Проверка Rust недоступна: "+String(err));});
    },250);
    return ()=>{cancelled=true;clearTimeout(timer);};
  },[project]);

  const uniqueRelays=useMemo(()=>Array.from(new Set(
    project.rungs.flatMap(r=>allContacts(r).map(contact=>contact.relay))
  )).sort(),[project.rungs]);
  const simulation=selectedRung?evaluateDraft(selectedRung.expression,signals):undefined;

  async function save() {
    try {
      const result=await invoke<DraftValidation>("cmd_save_graph_draft",{
        filePath:filePath.trim(),project,
      });
      setValidation(result);
      setStatus(result.has_errors?
        "JSON-черновик сохранён с диагностикой. Экспорт на робот запрещён.":
        "JSON-черновик сохранён; обновлена резервная копия при перезаписи. CIOPRG.LST не создан.");
    } catch(e) {setStatus("Ошибка сохранения: "+String(e));}
  }
  async function load() {
    try {
      const value=await invoke<GraphDraft>("cmd_load_graph_draft",{filePath:filePath.trim()});
      setEditor({project:value,past:[],future:[]});
      setSelectedId(null);setActiveRungId(value.rungs[0]?.id??null);
      setSignals({});
      setStatus("Загружен самостоятельный графический JSON-черновик.");
    } catch(e) {setStatus("Ошибка открытия: "+String(e));}
  }
  function addRung() {
    if(project.rungs.length>=128) {setStatus("Превышен предел 128 цепей.");return;}
    const next=appendDraftRung(project);
    edit(()=>next);
    setActiveRungId(next.rungs[next.rungs.length-1].id);
    setSelectedId(null);
  }
  function removeRung() {
    if(!selectedRung)return;
    if(!window.confirm("Удалить выбранную цепь из графического черновика?"))return;
    const id=selectedRung.id;
    edit(old=>({...old,rungs:old.rungs.filter(r=>r.id!==id)}));
    setSelectedId(null);
    setActiveRungId(null);
  }
  function addJoin(kind:"and"|"or") {
    if(!selectedRung)return;
    editRung(r=>({...r,expression:addBranch(r.expression,selectedContact?.id??null,kind)}));
    setSelectedId(null);
  }
  function removeSelected() {
    if(!selectedContact||!selectedRung)return;
    editRung(r=>({...r,expression:deleteContact(r.expression,selectedContact.id)}));
    setSelectedId(null);
  }
  function updateContact(changes:Partial<ContactExpr>) {
    if(!selectedContact)return;
    editRung(r=>({...r,expression:changeAt(r.expression,selectedContact.id,old=>
      old.kind==="contact"?{...old,...changes}:old)}));
  }

  return <div style={{display:"flex",flex:1,overflow:"hidden",minHeight:0,
      background:"#1d2329",color:"#e5edf4",fontSize:13}}>
    <aside style={{width:255,flexShrink:0,overflowY:"auto",padding:12,
      borderRight:"1px solid #40505e",display:"grid",gap:11,alignContent:"start"}}>
      <div style={PANEL}>
        <strong style={{fontSize:15}}>Графический черновик</strong>
        <p style={{color:"#e3c587",fontSize:12}}>
          Не является CIOPRG.LST. Нет связи с роботом, компиляция и загрузка отключены.
        </p>
        <label style={{display:"grid",gap:4}}>Контроллер
          <select style={INPUT} value={project.controller} onChange={e=>edit(d=>({...d,
            controller:e.target.value as GraphDraft["controller"]}))}>
            <option>YRC1000</option><option>DX200</option><option>DX100</option>
          </select>
        </label>
      </div>
      <div style={PANEL}>
        <strong>Цепи ({project.rungs.length})</strong>
        <div style={{display:"grid",gap:6,marginTop:10}}>
          {project.rungs.map((r,i)=><button type="button" key={r.id}
            onClick={()=>{setActiveRungId(r.id);setSelectedId(null);}}
            style={{...BTN,textAlign:"left",background:selectedRung?.id===r.id?"#3c5a66":"#293640"}}>
            {i+1}. OUT {r.output}
          </button>)}
        </div>
        <div style={{display:"flex",gap:6,marginTop:10}}>
          <button type="button" style={BTN} onClick={addRung}>+ Цепь</button>
          <button type="button" style={BTN} onClick={removeRung} disabled={!selectedRung}>Удалить</button>
        </div>
      </div>
      <div style={PANEL}>
        <strong>История изменений</strong>
        <div style={{display:"flex",gap:6,marginTop:9}}>
          <button type="button" style={BTN} onClick={undo} disabled={!editor.past.length}>↶ Отмена</button>
          <button type="button" style={BTN} onClick={redo} disabled={!editor.future.length}>↷ Повтор</button>
        </div>
      </div>
      <div style={PANEL}>
        <strong>Сохранение черновика</strong>
        <p style={{fontSize:11,color:"#adbac6"}}>Только имя *.graph.json. Сохранение не создаёт .LST.</p>
        <input aria-label="Путь к графическому черновику"
          style={{...INPUT,width:"100%",boxSizing:"border-box"}}
          value={filePath} onChange={e=>setFilePath(e.target.value)}/>
        <div style={{display:"flex",gap:6,marginTop:9}}>
          <button type="button" style={BTN} onClick={save}>Сохранить</button>
          <button type="button" style={BTN} onClick={load}>Открыть</button>
        </div>
      </div>
    </aside>
    <main style={{flex:1,display:"flex",flexDirection:"column",gap:12,
      overflowY:"auto",padding:14,minWidth:0}}>
      <div style={{...PANEL,display:"flex",justifyContent:"space-between",alignItems:"center",
        gap:10,flexWrap:"wrap"}}>
        <div>
          <h2 style={{margin:0,fontSize:17}}>Ladder Graph Editor · Draft</h2>
          <div style={{color:"#e0bd7b",fontSize:12}}>Редактируется только локальный JSON-черновик.</div>
        </div>
        <div style={{display:"flex",alignItems:"center",gap:7}}>
          <button type="button" style={BTN} onClick={()=>setZoom(z=>Math.max(.5,z-.15))}>−</button>
          <span>{Math.round(zoom*100)}%</span>
          <button type="button" style={BTN} onClick={()=>setZoom(z=>Math.min(1.8,z+.15))}>+</button>
          <button type="button" style={BTN} onClick={()=>setZoom(1)}>100%</button>
        </div>
      </div>
      <section style={{...PANEL,background:"#202c33"}}>
        <div style={{display:"flex",gap:10,flexWrap:"wrap",justifyContent:"space-between",alignItems:"center"}}>
          <div>
            <strong>Диагностика контактов · локальная имитация</strong>
            <div style={{fontSize:11,color:"#ddb87c",marginTop:4}}>
              Ручные значения входов. Не HSES, не состояние робота и не управление выходами.
              Нажмите на адрес для переключения: ? → 0 → 1 → ?.
            </div>
          </div>
          <div style={{display:"flex",gap:9,flexWrap:"wrap",fontSize:12}}>
            {(["TRUE","FALSE","UNKNOWN"] as const).map(state=>
              <span key={state} style={{color:STATE_COLORS[state]}}>● {STATE_LABELS[state]}</span>)}
            <button type="button" style={BTN} onClick={()=>setSignals({})}>Сбросить входы</button>
          </div>
        </div>
        <div style={{display:"flex",gap:7,marginTop:10,flexWrap:"wrap"}}>
          {uniqueRelays.map(relay=>{
            const value=signals[relay];
            const state=value===undefined?"UNKNOWN":value?"TRUE":"FALSE";
            return <button key={relay} type="button"
              aria-label={`Тестовый вход ${relay}: ${STATE_LABELS[state]}`}
              onClick={()=>setSignals(old=>({...old,[relay]:nextManualSignal(old[relay])}))}
              style={{...BTN,color:STATE_COLORS[state],background:STATE_BACKGROUNDS[state],
                borderColor:STATE_COLORS[state],fontFamily:"monospace"}}>
              {relay} = {value===undefined?"?":value?"1":"0"}
            </button>;
          })}
          {!uniqueRelays.length&&<span>Контактов нет.</span>}
        </div>
      </section>
      <section style={{display:"grid",gap:11}}>
        <div style={{display:"flex",justifyContent:"space-between",gap:10,flexWrap:"wrap",alignItems:"center"}}>
          <strong style={{fontSize:15}}>Рабочее поле · все цепи ({project.rungs.length})</strong>
          <span style={{fontSize:11,color:"#9fb1bc"}}>Один вертикальный список, без скрытых цепей · до 128</span>
        </div>
        {project.rungs.map((r,index)=>{
          const selected=selectedRung?.id===r.id;
          const result=expressionLogicState(r.expression,signals);
          return <article key={r.id} style={{...PANEL,padding:9,
            border:selected?"2px solid #9ab0be":"1px solid #475665",
            background:selected?"#25353d":"#202931"}}>
            <header style={{display:"flex",alignItems:"center",justifyContent:"space-between",
              gap:9,flexWrap:"wrap",marginBottom:7}}>
              <button type="button" onClick={()=>{setActiveRungId(r.id);setSelectedId(null);}}
                style={{...BTN,background:selected?"#506b73":"#344457"}}>
                {index+1}. OUT {r.output}{selected?" · редактируется":""}
              </button>
              <span title="Вычислено только по заданным вручную входам" style={{
                borderRadius:5,padding:"4px 9px",fontSize:12,
                background:STATE_BACKGROUNDS[result],color:STATE_COLORS[result],
                border:`1px solid ${STATE_COLORS[result]}`}}>
                ◉ {STATE_LABELS[result]} · OFFLINE
              </span>
            </header>
            <LadderCircuit expression={r.expression} output={r.output}
              zoom={zoom} fitToWidth readOnly={false}
              signalValues={signals}
              selectedContactId={selected?selectedId:null}
              onSelectContact={contactId=>{
                setActiveRungId(r.id);
                setSelectedId(contactId);
              }}/>
            {r.comment&&<div style={{fontSize:11,color:"#abbec9",marginTop:5}}>
              {r.comment}</div>}
          </article>;
        })}
        {!project.rungs.length&&<div style={PANEL}>Добавьте первую цепь кнопкой «+ Цепь».</div>}
      </section>
      <div style={{fontSize:14,fontWeight:700,color:"#dce7f0",marginTop:5}}>
        Свойства выбранной цепи
      </div>
      {!selectedRung?<div style={PANEL}>Добавьте первую цепь кнопкой «+ Цепь».</div>:<>
        <div style={PANEL}>
          <label style={{display:"grid",gap:5}}>Комментарий цепи
            <input style={INPUT} value={selectedRung.comment}
              onChange={e=>editRung(r=>({...r,comment:e.target.value}))}/>
          </label>
          <div style={{display:"flex",flexWrap:"wrap",gap:12,marginTop:9,alignItems:"center"}}>
            <label>Катушка OUT{" "}
              <input aria-label="Адрес выходной катушки" style={{...INPUT,width:110}}
                value={selectedRung.output}
                onChange={e=>editRung(r=>({...r,output:e.target.value}))}/>
            </label>
            <span style={{fontSize:12,color:"#aebdce"}}>Выберите контакт на схеме, затем добавьте последовательный или параллельный контакт.</span>
          </div>
        </div>
        <div style={{...PANEL,display:"flex",gap:8,alignItems:"center",flexWrap:"wrap"}}>
          <strong>{selectedContact?"Выбран контакт "+selectedContact.relay:"Выбрана вся цепь"}</strong>
          <button type="button" style={BTN} onClick={()=>setSelectedId(null)}>Вся цепь</button>
          <button type="button" style={BTN} onClick={()=>addJoin("and")}>+ Последовательно (AND)</button>
          <button type="button" style={BTN} onClick={()=>addJoin("or")}>+ Параллельно (OR)</button>
          <button type="button" style={BTN} disabled={!selectedContact}
            onClick={removeSelected}>Удалить контакт</button>
        </div>
        {selectedContact&&<div style={{...PANEL,display:"flex",gap:14,alignItems:"center",flexWrap:"wrap"}}>
          <label>Адрес контакта{" "}
            <input aria-label="Адрес выбранного контакта" style={{...INPUT,width:105}}
              value={selectedContact.relay}
              onChange={e=>updateContact({relay:e.target.value})}/>
          </label>
          <label style={{display:"flex",gap:6,alignItems:"center"}}>
            <input type="checkbox" checked={selectedContact.normallyClosed}
              onChange={e=>updateContact({normallyClosed:e.target.checked})}/>
            Нормально-закрытый контакт (NC)
          </label>
        </div>}
        <div style={{...PANEL,fontSize:12,color:"#cbd6df"}}>
          Условие выбранной цепи · OFFLINE:
          {" "}<strong style={{color:STATE_COLORS[expressionLogicState(selectedRung.expression,signals)]}}>
            {simulation===undefined?"НЕИЗВЕСТНО":simulation?"ИСТИНА":"ЛОЖЬ"}
          </strong>.
          {" "}Состояние контактов задаётся в панели над всеми схемами.
        </div>
      </>}
      <div role="status" style={{...PANEL,color:"#b8c7d2"}}>{status}</div>
      {validation&&<div style={PANEL}>
        <strong>Диагностика проекта ({validation.diagnostics.length})</strong>
        <p style={{fontSize:12,color:validation.has_errors?"#ffb7b7":"#aaccad"}}>
          {validation.structure_valid?"Структура черновика проверена.":"Нарушена структура черновика."}
          {" "}Экспорт изменённого CIOPRG.LST запрещён при любом результате.
        </p>
        <div style={{maxHeight:190,overflowY:"auto",display:"grid",gap:5}}>
          {validation.diagnostics.map((d,i)=><div key={i}
            style={{color:d.level==="Error"?"#ffabb5":"#e4c58b",fontFamily:"monospace",fontSize:12}}>
            [{d.level}] {d.code} ({d.rung_id.slice(0,8)}): {d.message}
          </div>)}
          {!validation.diagnostics.length&&<div>Замечаний в поддерживаемой модели черновика нет.</div>}
        </div>
      </div>}
    </main>
  </div>;
}
