import { useState } from "react";
import { useProjectStore } from "./store/projectStore";
import { LadderCanvas } from "./components/LadderCanvas";
import { NativeWorkspace } from "./components/NativeWorkspace";
import { GraphWorkspace } from "./components/GraphWorkspace.tsx";
import { AdvancedBlocksPanel } from "./components/AdvancedBlocksPanel.tsx";
import { invoke } from "@tauri-apps/api/core";
import { Save, FolderOpen, Plus } from "lucide-react";

type Workspace = "native" | "graph" | "advanced" | "compat";
type ValidationError = {rung_id:number;error_type:string;message:string};

export function App() {
  const {projectName,rungs,addRung,loadProject}=useProjectStore();
  const [workspace,setWorkspace]=useState<Workspace>("native");
  const [errors,setErrors]=useState<ValidationError[]>([]);
  const [saving,setSaving]=useState(false);
  const nav=(key:Workspace,label:string)=><button type="button"
    aria-pressed={workspace===key}
    style={{...btnStyle,background:workspace===key?"#3a6c61":"#344454"}}
    onClick={()=>setWorkspace(key)}>{label}</button>;

  async function saveCompatibility() {
    setSaving(true);
    try {
      const warnings=await invoke<ValidationError[]>("cmd_save_project",{
        filePath:"project.json",data:rungs,
      });
      setErrors(warnings);
      if(!warnings.length)alert("JSON-черновик сохранён. CIOPRG.LST не изменён.");
    } catch(error) {
      alert("Ошибка сохранения JSON-черновика: "+String(error));
    } finally {
      setSaving(false);
    }
  }

  return <div style={{
    display:"flex",flexDirection:"column",width:"100vw",height:"100vh",
    overflow:"hidden",background:"#101821",color:"#dce5ed",
  }}>
    <div role="status" style={{background:"#494125",color:"#ffdda3",
      borderBottom:"1px solid #7d7042",padding:"5px 12px",fontSize:12}}>
      MLAD Bench · OFFLINE · Оригинал CIOPRG.LST только для чтения · Экспорт, FTP и HSES отключены
    </div>
    <header style={{display:"flex",alignItems:"center",gap:8,flexWrap:"wrap",
      padding:"9px 12px",borderBottom:"1px solid #465869",background:"#21303c"}}>
      <strong style={{fontSize:16,color:"#a4d4ee",marginRight:10}}>MLAD</strong>
      {nav("native","Программа робота · CIOPRG.LST")}
      {nav("graph","Черновик Ladder")}
      {nav("advanced","TMR · CNT · GSTR/GOUT")}
      <details style={{position:"relative",marginLeft:3}}>
        <summary style={{...btnStyle,listStyle:"none"}}>Дополнительно ▾</summary>
        <div style={{position:"absolute",top:"calc(100% + 6px)",zIndex:10,
          minWidth:280,background:"#273a49",padding:12,border:"1px solid #5b7182",
          boxShadow:"0 8px 20px #000a",borderRadius:5}}>
          <div style={{fontSize:12,color:"#c3d1df",lineHeight:1.5,marginBottom:10}}>
            Совместимость: старый формат project.json. Это самостоятельный JSON-черновик,
            не исходная программа контроллера.
          </div>
          <button type="button" style={btnStyle} onClick={()=>setWorkspace("compat")}>
            Редактор JSON v1 (совместимость)
          </button>
        </div>
      </details>
      <span style={{marginLeft:"auto",fontSize:11,color:"#abd6bd",
        padding:"5px 9px",border:"1px solid #456e5c",borderRadius:4}}>
        ● Локально · Нет связи с контроллером
      </span>
    </header>

    {/* Editors stay mounted: switching tabs retains unsaved offline drafts. */}
    <div style={{display:workspace==="native"?"flex":"none",flex:1,minHeight:0,overflow:"hidden"}}>
      <NativeWorkspace />
    </div>
    <div style={{display:workspace==="graph"?"flex":"none",flex:1,minHeight:0,overflow:"hidden"}}>
      <GraphWorkspace />
    </div>
    <div style={{display:workspace==="advanced"?"flex":"none",flex:1,minHeight:0,overflow:"hidden"}}>
      <AdvancedBlocksPanel />
    </div>

    {workspace==="compat"&&<div style={{display:"flex",flex:1,minHeight:0,overflow:"hidden"}}>
      <aside style={{width:260,padding:14,background:"#21303c",borderRight:"1px solid #3d5266",
        display:"flex",flexDirection:"column",gap:10,overflowY:"auto"}}>
        <strong>Совместимость с JSON v1</strong>
        <p style={{fontSize:12,color:"#dfba84",lineHeight:1.5,margin:0}}>
          Устаревший редактор project.json. Не является просмотром оригинального CIOPRG.LST.
        </p>
        <small>Проект: {projectName}</small>
        <button type="button" style={btnStyle} onClick={()=>addRung()}>
          <Plus size={15}/>Добавить цепь
        </button>
        <button type="button" style={btnStyle} onClick={()=>void loadProject("project.json")}>
          <FolderOpen size={15}/>Открыть project.json
        </button>
        <button type="button" style={btnStyle} onClick={()=>void saveCompatibility()} disabled={saving}>
          <Save size={15}/>Сохранить project.json
        </button>
        {errors.map((e,i)=><div key={i} style={{fontSize:12,color:"#e8be83"}}>
          R{e.rung_id}: {e.message}
        </div>)}
        <button type="button" style={btnStyle} onClick={()=>setWorkspace("native")}>
          Вернуться к CIOPRG.LST
        </button>
      </aside>
      <div style={{minWidth:0,flex:1,overflow:"auto",padding:14}}>
        <LadderCanvas rungs={rungs} activeAddresses={{}}/>
      </div>
    </div>}
  </div>;
}

const btnStyle: React.CSSProperties={
  display:"inline-flex",alignItems:"center",justifyContent:"center",gap:6,
  padding:"8px 12px",border:"1px solid #476071",borderRadius:5,
  background:"#344454",color:"#eaf0f6",cursor:"pointer",fontSize:12,fontWeight:500,
};
