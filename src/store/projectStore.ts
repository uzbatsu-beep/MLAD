import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";

// Описание интерфейса инструкции релейной логики
export interface LadderInstruction {
  id: string;
  type: "STR" | "STR-NOT" | "AND" | "AND-NOT" | "OR" | "OR-NOT" | "OUT" | "TMR" | "CNT";
  operand: string; // Пятизначный адрес (например, "70010") или параметры ФБ
}

// Описание структуры одной цепи (Rung)
export interface Rung {
  id: number;
  comment?: string;
  instructions: LadderInstruction[];
  isActive: boolean; // Используется для онлайн-подсветки при мониторинге
}

interface ProjectState {
  projectName: string;
  rungs: Rung[];
  selectedRungId: number | null;
  isMonitoring: boolean;
  
  // Действия управления проектом
  loadProject: (path: string) => Promise<void>;
  saveProject: (path: string) => Promise<void>;
  addRung: () => void;
  deleteRung: (id: number) => void;
  updateInstruction: (rungId: number, instructionId: string, updated: Partial<LadderInstruction>) => void;
  
  // Действия для работы с сетью и мониторинга
  setMonitoring: (status: boolean) => void;
  updateRuntimeStates: (activeAddresses: Record<string, boolean>) => void;
}

export const useProjectStore = create<ProjectState>((set, get) => ({
  projectName: "Новый проект",
  rungs: [
    { id: 1, comment: "Стартовая цепь по умолчанию", instructions: [], isActive: false }
  ],
  selectedRungId: null,
  isMonitoring: false,

  loadProject: async (path) => {
    try {
      // Вызов бэкенд-команды Rust для чтения и парсинга проекта
      const loadedRungs: Rung[] = await invoke("cmd_load_project", { filePath: path });
      set({ rungs: loadedRungs, projectName: path.split("/").pop() || "Проект" });
    } catch (error) {
      console.error("Ошибка при загрузке проекта:", error);
    }
  },

  saveProject: async (path) => {
    try {
      const { rungs } = get();
      // Вызов бэкенд-компилятора Rust для сохранения структуры проекта в JSON/LST
      await invoke("cmd_save_project", { filePath: path, data: rungs });
    } catch (error) {
      console.error("Ошибка при сохранении проекта:", error);
    }
  },

  addRung: () => set((state) => {
    const newId = state.rungs.length > 0 ? Math.max(...state.rungs.map(r => r.id)) + 1 : 1;
    return {
      rungs: [...state.rungs, { id: newId, instructions: [], isActive: false }]
    };
  }),

  deleteRung: (id) => set((state) => ({
    rungs: state.rungs.filter((rung) => rung.id !== id),
    selectedRungId: state.selectedRungId === id ? null : state.selectedRungId
  })),

  updateInstruction: (rungId, instructionId, updated) => set((state) => ({
    rungs: state.rungs.map((rung) => {
      if (rung.id !== rungId) return rung;
      return {
        ...rung,
        instructions: rung.instructions.map((inst) => 
          inst.id === instructionId ? { ...inst, ...updated } : inst
        )
      };
    })
  })),

  setMonitoring: (status) => set({ isMonitoring: status }),

  updateRuntimeStates: (activeAddresses) => set((state) => ({
    rungs: state.rungs.map((rung) => {
      // Логика проверки: если в инструкции цепи есть активный адрес, подсвечиваем Rung
      const hasActiveElement = rung.instructions.some(inst => activeAddresses[inst.operand]);
      return { ...rung, isActive: hasActiveElement };
    })
  }))
}));
