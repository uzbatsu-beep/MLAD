import { useState, useEffect } from "react";
import { useProjectStore } from "./store/projectStore";
import { LadderCanvas } from "./components/LadderCanvas";
import { CioprgInspector } from "./components/CioprgInspector";
import { GraphWorkspace } from "./components/GraphWorkspace.tsx";
import { AdvancedBlocksPanel } from "./components/AdvancedBlocksPanel.tsx";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { Play, Square, Save, FolderOpen, Plus, Cpu, ArrowUpCircle, AlertTriangle, FileSearch } from "lucide-react";

interface ValidationError {
  rung_id: number;
  error_type: string;
  message: string;
}

export function App() {
  const {
    projectName,
    rungs,
    isMonitoring,
    addRung,
    setMonitoring,
    updateRuntimeStates,
    loadProject
  } = useProjectStore();

  const [ipAddress, setIpAddress] = useState("192.168.1.10");
  const [activeAddresses, setActiveAddresses] = useState<Record<string, boolean>>({});
  const [errors, setErrors] = useState<ValidationError[]>([]);
  const [inspectorOpen, setInspectorOpen] = useState(false);
  const [graphMode, setGraphMode] = useState(true);
  const [advancedMode, setAdvancedMode] = useState(false);

  // 1. Подписка на события рантайма из Rust по протоколу UDP/HSE
  useEffect(() => {
    let unlistenSync: (() => void) | undefined;
    let unlistenDisc: (() => void) | undefined;
    
    async function setupListeners() {
      unlistenSync = await listen<Record<string, boolean>>("ladder-runtime-sync", (event) => {
        setActiveAddresses(event.payload);
        updateRuntimeStates(event.payload);
      });

      unlistenDisc = await listen<void>("robot-disconnected", () => {
        console.warn("Потеряно соединение с роботом");
        setActiveAddresses({});
        updateRuntimeStates({});
      });
    }
    
    setupListeners();
    return () => {
      if (unlistenSync) unlistenSync();
      if (unlistenDisc) unlistenDisc();
    };
  }, [updateRuntimeStates]);

  // 2. Управление онлайн-мониторингом через бэкенд
  const handleToggleMonitoring = async () => {
    const nextState = !isMonitoring;
    try {
      await invoke("cmd_toggle_monitoring", { ip: ipAddress, active: nextState });
      setMonitoring(nextState);
      if (!nextState) {
        setActiveAddresses({});
        updateRuntimeStates({});
      }
    } catch (err) {
      console.error("Ошибка переключения мониторинга:", err);
    }
  };

  // 3. Сохранение JSON-черновика (CIOPRG.LST экспорт отключён до верификации)
  const handleSave = async () => {
    try {
      // Сохраняем проект в JSON, бэкенд возвращает список предупреждений и ошибок
      const validationErrors = await invoke<ValidationError[]>("cmd_save_project", { 
        filePath: "project.json", 
        data: rungs 
      });
      setErrors(validationErrors);
      if (validationErrors.length === 0) {
        alert("JSON-черновик сохранён. Экспорт CIOPRG.LST отключён до верификации формата.");
      }
    } catch (err) {
      console.error("Ошибка сохранения проекта:", err);
    }
  };

  // 4. Деплой скомпилированного нативного файла .LST на контроллер по FTP
  const handleDeploy = async () => {
    try {
      await invoke("cmd_deploy_to_robot", { ip: ipAddress, localPath: "project.lst" });
      alert("Скомпилированный файл CIOLAD.LST успешно загружен на робота по FTP!");
    } catch (err) {
      alert(`Ошибка FTP деплоя: ${err}`);
    }
  };

  return (
    <div style={{ display: "flex", flexDirection: "column", width: "100vw", height: "100vh", backgroundColor: "#1e1e1e" }}>
      <div role="status" style={{background:"#493f25",color:"#ffe0a2",
        borderBottom:"1px solid #a27e39",padding:"7px 16px",
        fontSize:12,fontWeight:600}}>
        MLAD Bench v0.5.0-rc.1 · STAND / OFFLINE · Только подготовка стендовых
        испытаний. Запись CIOPRG.LST, FTP и HSES отключены.
      </div>
      {/* Панель инструментов (Toolbar) */}
      <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", flexWrap: "wrap", gap: "8px", padding: "10px 20px", backgroundColor: "#2d2d2d", borderBottom: "1px solid #3c3c3c" }}>
        <div style={{ display: "flex", alignItems: "center", gap: "10px", flexWrap: "wrap" }}>
          <span style={{ fontWeight: "bold", color: "#61afef" }}>MLAD</span>
          <button type="button" style={{...btnStyle, backgroundColor: graphMode && !advancedMode ? "#506b60" : "#3a3f4b"}}
            onClick={() => {setGraphMode(true);setAdvancedMode(false);}}>Графический редактор</button>
          <button type="button" style={{...btnStyle, backgroundColor:graphMode && advancedMode ? "#506b60" : "#3a3f4b"}}
            onClick={() => {setGraphMode(true);setAdvancedMode(true);}}>TMR · CNT · GSTR/GOUT</button>
          <button type="button" style={{...btnStyle, backgroundColor: !graphMode ? "#506b60" : "#3a3f4b"}}
            onClick={() => setGraphMode(false)}>Старый JSON</button>
          {!graphMode && <span style={{fontSize:12,color:"#a6b4c1"}}>{projectName}</span>}
          {!graphMode && <button onClick={() => loadProject("project.json")} style={btnStyle}><FolderOpen size={16} /> Открыть</button>}
          {!graphMode && <button onClick={handleSave} style={btnStyle}><Save size={16} /> Сохранить старый JSON</button>}
          <button type="button" onClick={() => setInspectorOpen(true)} style={btnStyle}><FileSearch size={16} /> Просмотр CIOPRG.LST</button>
          <button onClick={handleDeploy} disabled title="Небезопасный деплой заблокирован до проверки формата CIOPRG.LST" style={{ ...btnStyle, opacity: 0.45, cursor: "not-allowed" }}><ArrowUpCircle size={16} /> FTP (отключён)</button>
        </div>

        <div style={{ display: "flex", alignItems: "center", gap: "10px" }}>
          <div style={{ display: "flex", alignItems: "center", gap: "5px", backgroundColor: "#1e1e1e", padding: "4px 8px", borderRadius: "4px" }}>
            <Cpu size={14} color="#98c379" />
            <input 
              type="text" 
              value={ipAddress} 
              onChange={(e) => setIpAddress(e.target.value)} 
              disabled={isMonitoring}
              style={{ background: "none", border: "none", color: "#f4f4f4", fontFamily: "monospace", width: "110px", outline: "none" }}
            />
          </div>
          <button 
            onClick={handleToggleMonitoring} 
            style={{ ...btnStyle, backgroundColor: isMonitoring ? "#e06c75" : "#98c379", color: "#1e1e1e" }}
          >
            {isMonitoring ? <Square size={16} /> : <Play size={16} />}
            {isMonitoring ? "Мониторинг" : "HSES (отключён)"}
          </button>
        </div>
      </div>

      {inspectorOpen && <CioprgInspector onClose={() => setInspectorOpen(false)} />}

      {/* Новый графический черновик всегда смонтирован: переключение вкладок не теряет несохранённые правки. */}
      <div style={{display:graphMode && !advancedMode?"flex":"none",flex:1,minHeight:0,overflow:"hidden"}}>
        <GraphWorkspace />
      </div>

      {/* Advanced JSON editor never mixes its blocks with the legacy graph.
          Keep both editors mounted while changing tabs to preserve drafts. */}
      <div style={{display:graphMode && advancedMode?"flex":"none",flex:1,minHeight:0,overflow:"hidden"}}>
        <AdvancedBlocksPanel />
      </div>

      {/* Основное рабочее пространство устаревшей версии (только JSON) */}
      {!graphMode && <div style={{ display: "flex", flex: 1, overflow: "hidden" }}>
        {/* Боковая панель управления (Sidebar) */}
        <div style={{ width: "260px", backgroundColor: "#252526", borderRight: "1px solid #3c3c3c", padding: "15px", display: "flex", flexDirection: "column", gap: "15px" }}>
          <h3 style={{ margin: 0, fontSize: "14px", color: "#abb2bf", textTransform: "uppercase", letterSpacing: "0.5px" }}>Управление</h3>
          <button onClick={addRung} style={{ ...btnStyle, width: "100%", justifyContent: "center", backgroundColor: "#3e4451" }}>
            <Plus size={16} /> Добавить цепь (Rung)
          </button>
          
          {/* Панель вывода ошибок статического анализа (Компилятора) */}
          {errors.length > 0 && (
            <div style={{ marginTop: "10px", display: "flex", flexDirection: "column", gap: "8px" }}>
              <div style={{ fontSize: "12px", color: "#e06c75", fontWeight: "bold", display: "flex", alignItems: "center", gap: "5px" }}>
                <AlertTriangle size={14} /> Проблемы компиляции:
              </div>
              <div style={{ maxHeight: "200px", overflowY: "auto", display: "flex", flexDirection: "column", gap: "5px" }}>
                {errors.map((err, idx) => (
                  <div key={idx} style={{ fontSize: "11px", backgroundColor: "#2c313c", padding: "6px", borderRadius: "4px", borderLeft: "3px solid #e06c75" }}>
                    <span style={{ color: "#e5c07b" }}>Rung {err.rung_id}:</span> {err.message}
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>

        {/* Интерактивный графический холст релейных схем */}
        <div style={{ flex: 1, padding: "20px", overflowY: "auto", backgroundColor: "#1e1e1e" }}>
          <LadderCanvas rungs={rungs} activeAddresses={activeAddresses} />
        </div>
      </div>}
    </div>
  );
}

const btnStyle: React.CSSProperties = {
  display: "flex",
  alignItems: "center",
  gap: "6px",
  backgroundColor: "#3a3f4b",
  border: "none",
  color: "#f4f4f4",
  padding: "6px 12px",
  borderRadius: "4px",
  cursor: "pointer",
  fontSize: "13px",
  fontWeight: 500,
  transition: "background-color 0.2s"
};
