import {useEffect,useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {addBranch,changeAt,deleteContact,findContact,type GraphExpr} from "../ladder/graphModel.ts";
import {AdvancedBlockDiagram} from "./AdvancedBlockDiagram.tsx";
import {
  STATE_BACKGROUNDS,STATE_COLORS,STATE_LABELS,type LogicState,
} from "../ladder/visualState.ts";
import {
  newBlock,freshAdvancedDraft,blockReviewLines,describeCondition,
  type AdvancedBlock,type AdvancedDraft,type AdvancedValidation,type Preset,
} from "../ladder/advancedModel.ts";

const button:React.CSSProperties={color:"#eef3f7",background:"#35465a",border:"1px solid #687989",
  borderRadius:6,padding:"7px 10px",cursor:"pointer",fontSize:12};
const input:React.CSSProperties={color:"#eef3f7",background:"#1e2730",border:"1px solid #637486",
  borderRadius:5,padding:7,fontFamily:"monospace",fontSize:12,minWidth:0};
const card:React.CSSProperties={background:"#263039",border:"1px solid #465563",
  padding:14,borderRadius:9};
type Tim={current:number;contact:boolean};
type Cnt={current:number;contact:boolean;previous_count:boolean};
type BlockSample =
  | {kind:"TMR";start:boolean;data:Tim}
  | {kind:"CNT";count:boolean;reset:boolean;data:Cnt}
  | {kind:"GROUP";source:number;destination:number};

function bitState(bit:boolean | undefined):LogicState{
  return bit===undefined?"UNKNOWN":bit?"TRUE":"FALSE";
}
function outputState(sample:BlockSample | undefined):LogicState{
  if(!sample)return "UNKNOWN";
  if(sample.kind==="GROUP")return "UNKNOWN"; // a byte is not a BOOL
  return bitState(sample.data.contact);
}


function contactsIn(expr:GraphExpr) {
  const result:{id:string;relay:string;normallyClosed:boolean}[]=[];
  const stack=[expr];
  while(stack.length) {
    const x=stack.pop()!;
    if(x.kind==="contact")result.push(x);
    else stack.push(x.right,x.left);
  }
  return result;
}
function ConditionEditor({label,expression,onUpdate}:{label:string;
  expression:GraphExpr;onUpdate:(x:GraphExpr)=>void}) {
  const [selected,setSelected]=useState<string|null>(null);
  const contacts=contactsIn(expression);
  const active=selected?findContact(expression,selected):null;
  function wrap(kind:"and"|"or"){
    onUpdate(addBranch(expression,active?.id??null,kind));
    setSelected(null);
  }
  return <div style={{...card,marginTop:9}}>
    <strong>{label}</strong>
    <div style={{fontFamily:"monospace",fontSize:12,margin:"8px 0",
      color:"#c6d8e6",overflowWrap:"anywhere"}}>{describeCondition(expression)}</div>
    <div style={{display:"flex",flexWrap:"wrap",gap:6}}>
      {contacts.map(c=><button type="button" style={{...button,
        background:active?.id===c.id?"#5e7f72":"#35465a"}}
        key={c.id} onClick={()=>setSelected(c.id)}>{c.normallyClosed?"NC":"NO"} {c.relay}</button>)}
    </div>
    <div style={{display:"flex",flexWrap:"wrap",gap:7,alignItems:"center",marginTop:9}}>
      <button type="button" style={button} onClick={()=>{setSelected(null);}}>Выбрать всё</button>
      <button type="button" style={button} onClick={()=>wrap("and")}>+ AND</button>
      <button type="button" style={button} onClick={()=>wrap("or")}>+ OR</button>
      <button type="button" style={button} disabled={!active} onClick={()=>{
        if(active){onUpdate(deleteContact(expression,active.id));setSelected(null);}
      }}>Удалить контакт</button>
    </div>
    {active&&<div style={{display:"flex",flexWrap:"wrap",gap:14,marginTop:9}}>
      <label>Контакт{" "}<input style={{...input,width:110}} value={active.relay}
        onChange={e=>onUpdate(changeAt(expression,active.id,old=>
          old.kind==="contact"?{...old,relay:e.target.value}:old))}/></label>
      <label><input type="checkbox" checked={active.normallyClosed}
        onChange={e=>onUpdate(changeAt(expression,active.id,old=>
          old.kind==="contact"?{...old,normallyClosed:e.target.checked}:old))}/>
        {" "}NC (инверсия)</label>
    </div>}
  </div>;
}

export function AdvancedBlocksPanel(){
  const [project,setProject]=useState<AdvancedDraft>(()=>freshAdvancedDraft());
  const [selectedId,setSelectedId]=useState<string|null>(null);
  const [filePath,setFilePath]=useState("ladder.advanced.json");
  const [status,setStatus]=useState("Advanced-блоки сохраняются отдельно от графических черновиков.");
  const [validation,setValidation]=useState<AdvancedValidation|null>(null);
  const [testPreset,setTestPreset]=useState(100);
  const [testEnable,setTestEnable]=useState(false);
  const [testCount,setTestCount]=useState(false);
  const [testReset,setTestReset]=useState(false);
  // Persist each block's last EXPLICIT offline sample independently of selection.
  // Editing a block invalidates only that block's sample; no hidden live state.
  const [samples,setSamples]=useState<Record<string,BlockSample>>({});
  const [sourceByte,setSourceByte]=useState(170);
  const [busy,setBusy]=useState(false);
  const selected=project.blocks.find(b=>b.id===selectedId)??project.blocks[0]??null;
  const selectedSample=selected?samples[selected.id]:undefined;
  const testTimer=selectedSample?.kind==="TMR"?selectedSample.data:null;
  const testCounter=selectedSample?.kind==="CNT"?selectedSample.data:null;
  const copiedByte=selectedSample?.kind==="GROUP"?selectedSample.destination:null;

  useEffect(()=>{
    let alive=true;
    const timer=setTimeout(()=>{
      invoke<AdvancedValidation>("cmd_validate_advanced_draft",{project})
        .then(x=>{if(alive)setValidation(x);})
        .catch(x=>{if(alive)setStatus("Ошибка проверки: "+String(x));});
    },240);
    return()=>{alive=false;clearTimeout(timer);};
  },[project]);

  useEffect(()=>{
    const current=project.blocks.find(b=>b.id===selectedId)??project.blocks[0];
    const prior=current?samples[current.id]:undefined;
    setTestCount(prior?.kind==="CNT"?prior.count:false);
    setTestReset(prior?.kind==="CNT"?prior.reset:false);
    setTestEnable(prior?.kind==="TMR"?prior.start:false);
    setSourceByte(prior?.kind==="GROUP"?prior.source:170);
    if(current&&current.kind!=="GROUP") {
      setTestPreset(current.preset.kind==="literal"?current.preset.value:100);
    }
  // Only switch controls when selecting another block, not when changing a test sample.
  // eslint-disable-next-line react-hooks/exhaustive-deps
  },[selectedId]);

  function update(id:string,modify:(b:AdvancedBlock)=>AdvancedBlock) {
    setProject(d=>({...d,blocks:d.blocks.map(b=>b.id===id?modify(b):b)}));
    setSamples(old=>{
      if(!(id in old))return old;
      const copy={...old};delete copy[id];return copy;
    });
  }
  function add(kind:AdvancedBlock["kind"]){
    if(project.blocks.length>=128){setStatus("Достигнут лимит 128 блоков.");return;}
    const b=newBlock(kind);
    setProject(d=>({...d,blocks:[...d.blocks,b]}));
    setSelectedId(b.id);
    setStatus(kind+" добавлен в JSON-черновик. Для оборудования нужен отдельный этап проверки.");
  }
  function remove(){
    if(!selected)return;
    if(!window.confirm("Удалить функциональный блок из черновика?"))return;
    const id=selected.id;
    setProject(d=>({...d,blocks:d.blocks.filter(b=>b.id!==id)}));
    setSamples(old=>{const next={...old};delete next[id];return next;});
    setSelectedId(null);
  }
  async function save(){
    setBusy(true);
    try{
      const v=await invoke<AdvancedValidation>("cmd_save_advanced_draft",{
        filePath:filePath.trim(),project,
      });
      setValidation(v);
      setStatus("Сохранён JSON-черновик. Нативный CIOPRG.LST не создавался. "+
        (v.has_errors?"Остались ошибки диагностики.":"Структурная проверка завершена."));
    }catch(e){setStatus("Ошибка сохранения: "+String(e));}
    finally{setBusy(false);}
  }
  async function load(){
    setBusy(true);
    try{
      const v=await invoke<AdvancedDraft>("cmd_load_advanced_draft",{filePath:filePath.trim()});
      setProject(v);setSamples({});setSelectedId(v.blocks[0]?.id??null);
      setStatus("Открыт advanced-черновик JSON. Изменения C/IO контроллера не выполнялись.");
    }catch(e){setStatus("Ошибка открытия: "+String(e));}
    finally{setBusy(false);}
  }
  async function offlineStep(){
    if(!selected)return;
    if (selected.kind!=="GROUP" &&
        (!Number.isInteger(testPreset)||testPreset<0||testPreset>65535)) {
      setStatus("Невозможно выполнить offline-шаг: уставка должна быть целым числом 0–65535.");
      return;
    }
    const preset=testPreset;
    try{
      if(selected.kind==="TMR"){
        const state=testTimer??{current:preset,contact:false};
        const result=await invoke<Tim>("cmd_test_tmr_step",{state,start:testEnable,preset});
        setSamples(old=>({...old,[selected.id]:{kind:"TMR",start:testEnable,data:result}}));
      }else if(selected.kind==="CNT"){
        const state=testCounter??{current:preset,contact:false,previous_count:false};
        const result=await invoke<Cnt>("cmd_test_cnt_step",{
          state,count:testCount,reset:testReset,preset,
        });
        setSamples(old=>({...old,[selected.id]:{kind:"CNT",count:testCount,reset:testReset,data:result}}));
      }else {
        const result=await invoke<number>("cmd_test_group_copy",{source:sourceByte});
        setSamples(old=>({...old,[selected.id]:{kind:"GROUP",source:sourceByte,destination:result}}));
      }
    }catch(e){setStatus("Ошибка offline-теста: "+String(e));}
  }
  function updatePreset(preset:Preset) {
    if(!selected||selected.kind==="GROUP")return;
    update(selected.id, b=>b.kind==="GROUP"?b:{...b,preset});
  }
  function editOnDiagram(id:string,
    field:"current"|"output"|"source"|"destination"|"preset"|
      "enable"|"count"|"reset",
    value:string) {
    update(id,b=>{
      if(b.kind==="GROUP") {
        if(field==="source")return {...b,source:value};
        if(field==="destination")return {...b,destination:value};
        return b;
      }
      if(field==="current")return {...b,current:value};
      if(field==="output")return {...b,output:value};
      if(field==="preset")return {...b,preset:value.startsWith("M")?
        {kind:"register",address:value}:{kind:"literal",value:Number(value)}};
      if(b.kind==="TMR" && field==="enable" && b.enable.kind==="contact")
        return {...b,enable:{...b.enable,relay:value}};
      if(b.kind==="CNT" && field==="count" && b.count.kind==="contact")
        return {...b,count:{...b.count,relay:value}};
      if(b.kind==="CNT" && field==="reset" && b.reset.kind==="contact")
        return {...b,reset:{...b.reset,relay:value}};
      return b;
    });
    setSelectedId(id);
    setStatus(`Адрес ${field} изменён в *.advanced.json: ${value}; проводится проверка Rust. Экспорт CIOPRG.LST запрещён.`);
  }

  return <div style={{width:"100%",height:"100%",minHeight:0,display:"flex",
    background:"#1c252e",color:"#e4edf5",overflow:"hidden",fontSize:13}}>
    <aside style={{width:250,flexShrink:0,overflow:"auto",padding:12,
      borderRight:"1px solid #42525f",display:"grid",gap:12,alignContent:"start"}}>
      <div style={card}>
        <h3 style={{margin:"0 0 8px"}}>Advanced CIO</h3>
        <div style={{fontSize:12,color:"#ebc685"}}>
          Экспериментальный JSON-редактор. Нативный экспорт и запись в робот запрещены.
        </div>
        <label style={{display:"grid",marginTop:8,gap:4}}>Контроллер
          <select style={input} value={project.controller} onChange={e=>setProject(d=>({
            ...d,controller:e.target.value as AdvancedDraft["controller"],
          }))}><option>YRC1000</option><option>DX200</option><option>DX100</option></select>
        </label>
      </div>
      <div style={card}>
        <strong>Добавить инструкцию</strong>
        <div style={{display:"flex",flexWrap:"wrap",gap:6,marginTop:9}}>
          <button type="button" style={button} onClick={()=>add("TMR")}>+ TMR</button>
          <button type="button" style={button} onClick={()=>add("CNT")}>+ CNT</button>
          <button type="button" style={button} onClick={()=>add("GROUP")}>+ GSTR/GOUT</button>
        </div>
      </div>
      <div style={card}>
        <strong>Блоки ({project.blocks.length})</strong>
        <div style={{display:"grid",gap:6,marginTop:9}}>
          {project.blocks.map((b,i)=><button type="button" style={{...button,textAlign:"left",
            background:b.id===selected?.id?"#486b72":"#35465a"}}
            key={b.id} onClick={()=>setSelectedId(b.id)}>
            {i+1}. {b.kind==="GROUP"?"GSTR / GOUT":b.kind} · {b.id.slice(-5)}
          </button>)}
        </div>
        <button type="button" style={{...button,marginTop:8}} onClick={remove}
          disabled={!selected}>Удалить блок</button>
      </div>
      <div style={card}>
        <strong>Файл проекта</strong>
        <div style={{fontSize:11,color:"#aebfcf",margin:"7px 0"}}>Только *.advanced.json</div>
        <input style={{...input,width:"100%",boxSizing:"border-box"}}
          value={filePath} onChange={e=>setFilePath(e.target.value)}/>
        <div style={{display:"flex",gap:6,marginTop:9}}>
          <button type="button" style={button} disabled={busy} onClick={save}>Сохранить</button>
          <button type="button" style={button} disabled={busy} onClick={load}>Открыть</button>
        </div>
      </div>
    </aside>
    <main style={{flex:1,minWidth:0,overflowY:"auto",padding:14,
      display:"grid",gap:13,gridAutoRows:"max-content",alignContent:"start"}}>
      <div style={card}>
        <h2 style={{margin:"0 0 6px"}}>TMR · CNT · GSTR/GOUT</h2>
        <span style={{color:"#e6c789",fontSize:12}}>
          Стандарты: DX100 RE-CKI-A453, DX200 RE-CKI-A465, YRC1000 RE-CKI-A467.
          Проверка на оборудовании и экспорт действующей Ladder-программы отдельно.
        </span>
      </div>
      <section style={{display:"grid",gap:11,gridAutoRows:"max-content"}}>
        <div style={{display:"flex",justifyContent:"space-between",gap:10,
          alignItems:"center",flexWrap:"wrap"}}>
          <strong style={{fontSize:16}}>Рабочее поле · все функциональные блоки ({project.blocks.length})</strong>
          <span style={{color:"#d2c18e",fontSize:11}}>Двойной щелчок по M-регистру, уставке, адресу C/R, OUT, GSTR/GOUT — изменить · Enter/Esc</span>
          <span style={{fontSize:12,color:"#d4bd82"}}>
            ◉ OFFLINE · только результаты выполненных вручную шагов
          </span>
        </div>
        <div style={{display:"flex",gap:10,flexWrap:"wrap",fontSize:12}}>
          {(["TRUE","FALSE","UNKNOWN"] as const).map(state=>
            <span key={state} style={{color:STATE_COLORS[state]}}>
              ● {STATE_LABELS[state]}</span>)}
          <span style={{color:"#91aabd"}}>GOUT: восемь отдельных индикаторов b7…b0</span>
        </div>
        {project.blocks.map((b,i)=>{
          const sample=samples[b.id];
          const result=outputState(sample);
          const selectedBlock=selected?.id===b.id;
          const input:LogicState=sample?.kind==="TMR"?bitState(sample.start):
            sample?.kind==="CNT"?bitState(sample.count):"UNKNOWN";
          const reset:LogicState=sample?.kind==="CNT"?bitState(sample.reset):"UNKNOWN";
          const current=sample?.kind==="TMR"||sample?.kind==="CNT"?sample.data.current:null;
          const group=sample?.kind==="GROUP"?sample.destination:null;
          return <article key={b.id} style={{
            ...card,padding:10,background:selectedBlock?"#253540":"#202a34",
            border:selectedBlock?"2px solid #91b3c3":"1px solid #465563"}}>
            <div style={{display:"flex",gap:10,flexWrap:"wrap",alignItems:"center",
              justifyContent:"space-between",marginBottom:8}}>
              <button type="button" style={{
                ...button,background:selectedBlock?"#496d76":"#344457"}}
                onClick={()=>setSelectedId(b.id)}>
                {i+1}. {b.kind==="GROUP"?"GSTR / GOUT":b.kind}
                {selectedBlock?" · редактируется":""}
              </button>
              <span style={{color:STATE_COLORS[result],fontSize:12,
                background:STATE_BACKGROUNDS[result],borderRadius:5,padding:"5px 9px"}}>
                {b.kind==="GROUP"?(group===null?"8 бит: НЕИЗВЕСТНО":
                  `Байт: 0x${group.toString(16).padStart(2,"0").toUpperCase()}`):
                  `Q: ${STATE_LABELS[result]}`}
              </span>
            </div>
            <div onClick={()=>setSelectedId(b.id)} role="button" tabIndex={0}
              aria-label={`Выбрать блок ${i+1}`}
              onKeyDown={e=>{if(e.key==="Enter"||e.key===" "){
                e.preventDefault();setSelectedId(b.id);
              }}} style={{cursor:"pointer"}}>
              <AdvancedBlockDiagram block={b}
                outputState={result} inputState={input} resetState={reset}
                currentValue={current} groupByte={group}
                onEditField={(field,value)=>editOnDiagram(b.id,field,value)}/>
            </div>
          </article>;
        })}
        {!project.blocks.length&&<div style={card}>Добавьте TMR, CNT или GSTR/GOUT.</div>}
      </section>
      <div style={{fontWeight:700,fontSize:15}}>Свойства выбранного блока</div>
      {!selected?<div style={card}>Выберите TMR, CNT или GSTR/GOUT в левой панели.</div>:<>
        <div style={card}>
          <strong>Параметры блока</strong>
          {selected.kind==="GROUP"?
            <div style={{display:"flex",gap:13,marginTop:9,flexWrap:"wrap"}}>
              <label>GSTR · источник (byte){" "}
                <input style={{...input,width:115}} value={selected.source}
                  onChange={e=>update(selected.id,b=>b.kind==="GROUP"?{...b,source:e.target.value}:b)}/>
              </label>
              <label>GOUT · назначение (byte){" "}
                <input style={{...input,width:115}} value={selected.destination}
                  onChange={e=>update(selected.id,b=>b.kind==="GROUP"?{...b,destination:e.target.value}:b)}/>
              </label>
              <div style={{color:"#e5c083",fontSize:12}}>
                Пара GSTR/GOUT переносит восемь бит целиком; адрес начинается на бит 0.
              </div>
            </div>:
            <div style={{display:"grid",gap:11,marginTop:9}}>
              <div style={{display:"flex",gap:14,flexWrap:"wrap",alignItems:"center"}}>
                <label>Текущий регистр{" "}
                  <input style={{...input,width:90}} value={selected.current}
                    onChange={e=>update(selected.id,b=>b.kind==="GROUP"?b:{...b,current:e.target.value})}/>
                </label>
                <label>Катушка OUT{" "}
                  <input style={{...input,width:100}} value={selected.output}
                    onChange={e=>update(selected.id,b=>b.kind==="GROUP"?b:{...b,output:e.target.value})}/>
                </label>
                <label>Тип уставки{" "}
                  <select style={input} value={selected.preset.kind}
                    onChange={e=>updatePreset(e.target.value==="literal"?
                      {kind:"literal",value:100}:{kind:"register",address:"M011"})}>
                    <option value="literal">Число</option><option value="register">M-регистр</option>
                  </select>
                </label>
                {selected.preset.kind==="literal"?
                  <label>Уставка (0–65535){" "}
                    <input type="number" min={0} max={65535} style={{...input,width:90}}
                      value={selected.preset.value}
                      onChange={e=>updatePreset({kind:"literal",value:Number(e.target.value)})}/>
                  </label>:
                  <label>Уставка Mxxx{" "}
                    <input style={{...input,width:90}} value={selected.preset.address}
                      onChange={e=>updatePreset({kind:"register",address:e.target.value})}/>
                  </label>}
              </div>
              <div style={{fontSize:12,color:"#c7d4df"}}>
                {selected.kind==="TMR"?
                  "TMR: 1 дискретный шаг = 0,1 с; контакт устанавливается при достижении нуля.":
                  "CNT: C — счётный вход по фронту OFF→ON, R — отдельный вход сброса с приоритетом."}
              </div>
            </div>}
        </div>
        {selected.kind==="TMR"&&<ConditionEditor
          key={selected.id+"-en"} label="Вход запуска TMR"
          expression={selected.enable}
          onUpdate={value=>update(selected.id,b=>b.kind==="TMR"?{...b,enable:value}:b)}/>}
        {selected.kind==="CNT"&&<>
          <ConditionEditor key={selected.id+"-count"} label="Вход C: счёт"
            expression={selected.count}
            onUpdate={value=>update(selected.id,b=>b.kind==="CNT"?{...b,count:value}:b)}/>
          <ConditionEditor key={selected.id+"-reset"} label="Вход R: сброс"
            expression={selected.reset}
            onUpdate={value=>update(selected.id,b=>b.kind==="CNT"?{...b,reset:value}:b)}/>
        </>}
        <div style={card}>
          <strong>Mnemonic — только справочная запись (НЕ CIOPRG.LST)</strong>
          <pre style={{background:"#19222b",color:"#d1e4ed",padding:10,
            fontSize:12,overflowX:"auto"}}>{blockReviewLines(selected).join("\n")}</pre>
          <p style={{fontSize:12,color:"#d7bb7d"}}>
            Для сложных условий запись обозначает структуру выражения, но не является
            исполняемым файлом Yaskawa. Нет команды экспорта или загрузки.
          </p>
        </div>
        <div style={card}>
          <strong>Offline-тест модели — без соединения с роботом</strong>
          <p style={{fontSize:12,color:"#e7c68e"}}>
            Шаг таймера моделирует ровно 100 мс, шаг счётчика — один вызов с
            заданным C/R. Результат не является подтверждением реального scan-cycle.
          </p>
          <div style={{display:"flex",gap:10,flexWrap:"wrap",alignItems:"center"}}>
            {selected.kind!=="GROUP"?<>
              <label>Уставка для теста{" "}
                <input type="number" min={0} max={65535} style={{...input,width:95}}
                  value={testPreset} onChange={e=>{
                    setTestPreset(Number(e.target.value));
                    if(selected)setSamples(old=>{const next={...old};delete next[selected.id];return next;});
                  }}/>
              </label>
              {selected.kind==="TMR"?
                <label><input type="checkbox" checked={testEnable}
                  onChange={e=>setTestEnable(e.target.checked)}/>{" "}Пуск</label>:
                <>
                  <label><input type="checkbox" checked={testCount}
                    onChange={e=>setTestCount(e.target.checked)}/>{" "}C</label>
                  <label><input type="checkbox" checked={testReset}
                    onChange={e=>setTestReset(e.target.checked)}/>{" "}R</label>
                </>}
            </>:<label>Байт источника (0–255){" "}
              <input type="number" min={0} max={255} style={{...input,width:80}}
                value={sourceByte} onChange={e=>setSourceByte(Number(e.target.value))}/>
            </label>}
            <button type="button" style={button} onClick={offlineStep}
              disabled={selected.kind==="GROUP"&&(!Number.isInteger(sourceByte)||sourceByte<0||sourceByte>255)}>
              {selected.kind==="TMR"?"+100 мс":selected.kind==="CNT"?"Шаг CNT":"Копировать 8 бит"}
            </button>
            <button type="button" style={button} onClick={()=>{
              if(selected)setSamples(old=>{const next={...old};delete next[selected.id];return next;});
              setTestEnable(false);setTestCount(false);setTestReset(false);
            }}>Сброс теста</button>
          </div>
          <div style={{fontFamily:"monospace",marginTop:10}}>
            {selected.kind==="TMR"?testTimer?
              `Current=${testTimer.current}, Q=${testTimer.contact?1:0}`:"TMR: тест не запущен":
            selected.kind==="CNT"?testCounter?
              `Current=${testCounter.current}, Q=${testCounter.contact?1:0}, last C=${testCounter.previous_count?1:0}`:"CNT: тест не запущен":
              copiedByte===null?"GSTR/GOUT: тест не запущен":
              `GOUT byte=${copiedByte} (0x${copiedByte.toString(16).padStart(2,"0").toUpperCase()})`}
          </div>
        </div>
      </>}
      <div style={card} role="status">{status}</div>
      {validation&&<div style={card}>
        <strong>Диагностика Rust · {validation.diagnostics.length} записей</strong>
        <p style={{fontSize:12,color:validation.has_errors?"#f9aeb3":"#b2d2b7"}}>
          {validation.has_errors?"Обнаружены ошибки параметров.":"Ошибок модели не найдено."}
          {" "}Экспорт CIOPRG.LST и испытания на оборудовании этим отчётом не разрешаются.
        </p>
        {validation.diagnostics.map((d,i)=><div key={i}
          style={{fontFamily:"monospace",fontSize:12,padding:4,
          color:d.level==="Error"?"#ffa9b0":"#e9c483"}}>
          {d.level} [{d.code}] {d.block_id.slice(-8)}: {d.message}
        </div>)}
      </div>}
    </main>
  </div>;
}
