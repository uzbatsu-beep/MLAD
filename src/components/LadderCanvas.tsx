import React from "react";
import { LadderInstruction, Rung } from "../store/projectStore";

interface LadderCanvasProps {
  rungs: Rung[];
  activeAddresses: Record<string, boolean>;
}

export const LadderCanvas: React.FC<LadderCanvasProps> = ({ rungs, activeAddresses }) => {
  
  // Функция для отрисовки графического представления конкретного элемента (контакта или катушки)
  const renderElement = (inst: LadderInstruction, isRungActive: boolean) => {
    // Проверяем, активен ли текущий операнд (сигнал) в реальном времени
    const isElementActive = activeAddresses[inst.operand] || false;
    const strokeColor = isElementActive && isRungActive ? "#98c379" : "#5c6370";

    switch (inst.type) {
      case "STR":
      case "AND":
        // Нормально-открытый контакт (NO)
        return (
          <div key={inst.id} style={elementWrapperStyle}>
            <svg width="40" height="60" style={{ overflow: "visible" }}>
              {/* Левая и правая соединительные линии */}
              <line x1="0" y1="30" x2="12" y2="30" stroke={strokeColor} strokeWidth="2" />
              <line x1="28" y1="30" x2="40" y2="30" stroke={strokeColor} strokeWidth="2" />
              {/* Вертикальные обкладки контакта */}
              <line x1="12" y1="15" x2="12" y2="45" stroke={strokeColor} strokeWidth="2" />
              <line x1="28" y1="15" x2="28" y2="45" stroke={strokeColor} strokeWidth="2" />
            </svg>
            <span style={{ ...operandLabelStyle, color: isElementActive ? "#98c379" : "#abb2bf" }}>
              {inst.type} {inst.operand}
            </span>
          </div>
        );

      case "STR-NOT":
      case "AND-NOT":
        // Нормально-закрытый контакт (NC)
        return (
          <div key={inst.id} style={elementWrapperStyle}>
            <svg width="40" height="60" style={{ overflow: "visible" }}>
              {/* Левая и правая линии */}
              <line x1="0" y1="30" x2="12" y2="30" stroke={strokeColor} strokeWidth="2" />
              <line x1="28" y1="30" x2="40" y2="30" stroke={strokeColor} strokeWidth="2" />
              {/* Вертикальные обкладки */}
              <line x1="12" y1="15" x2="12" y2="45" stroke={strokeColor} strokeWidth="2" />
              <line x1="28" y1="15" x2="28" y2="45" stroke={strokeColor} strokeWidth="2" />
              {/* Диагональная линия инверсии */}
              <line x1="10" y1="42" x2="30" y2="18" stroke={strokeColor} strokeWidth="2" />
            </svg>
            <span style={{ ...operandLabelStyle, color: isElementActive ? "#98c379" : "#abb2bf" }}>
              {inst.type} {inst.operand}
            </span>
          </div>
        );

      case "OUT":
        // Выходная катушка (Coil)
        return (
          <div key={inst.id} style={{ ...elementWrapperStyle, marginLeft: "auto" }}>
            <svg width="50" height="60" style={{ overflow: "visible" }}>
              {/* Соединительная линия слева */}
              <line x1="0" y1="30" x2="13" y2="30" stroke={strokeColor} strokeWidth="2" />
              {/* Скобки, формирующие катушку */}
              <path d="M 15,15 A 16,16 0 0,0 15,45" fill="none" stroke={strokeColor} strokeWidth="2" />
              <path d="M 35,15 A 16,16 0 0,1 35,45" fill="none" stroke={strokeColor} strokeWidth="2" />
              {/* Линия до правой шины питания */}
              <line x1="37" y1="30" x2="50" y2="30" stroke={strokeColor} strokeWidth="2" />
            </svg>
            <span style={{ ...operandLabelStyle, color: isElementActive ? "#98c379" : "#abb2bf", textAlign: "right" }}>
              OUT {inst.operand}
            </span>
          </div>
        );

      case "TMR":
        // Функциональный блок Таймера (Box)
        const [timerId, setpoint] = inst.operand.split(",");
        return (
          <div key={inst.id} style={{ ...elementWrapperStyle, marginLeft: "auto" }}>
            <div style={{
              width: "80px",
              height: "50px",
              border: `2px solid ${strokeColor}`,
              backgroundColor: "#1e1e1e",
              borderRadius: "4px",
              display: "flex",
              flexDirection: "column",
              justifyContent: "center",
              alignItems: "center",
              fontSize: "11px",
              fontFamily: "monospace"
            }}>
              <div style={{ color: "#e5c07b", fontWeight: "bold" }}>TMR {timerId}</div>
              <div style={{ color: "#61afef" }}>PV: {setpoint}</div>
            </div>
          </div>
        );

      default:
        return null;
    }
  };

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: "20px", width: "100%" }}>
      {rungs.map((rung) => (
        <div
          key={rung.id}
          style={{
            display: "flex",
            flexDirection: "column",
            backgroundColor: "#252526",
            border: `1px solid ${rung.isActive ? "#98c379" : "#3c3c3c"}`,
            borderRadius: "6px",
            padding: "15px",
            position: "relative"
          }}
        >
          {/* Метаданные и комментарии цепи */}
          <div style={{ display: "flex", justifyContent: "space-between", fontSize: "11px", color: "#5c6370", fontFamily: "monospace", marginBottom: "5px" }}>
            <span>RUNG #{rung.id}</span>
          </div>
          {rung.comment && (
            <div style={{ fontSize: "12px", color: "#98c379", fontStyle: "italic", marginBottom: "10px", fontFamily: "monospace" }}>
              ; {rung.comment}
            </div>
          )}

          {/* Графический контейнер цепи с шинами питания */}
          <div style={{
            display: "flex",
            alignItems: "center",
            height: "70px",
            position: "relative",
            padding: "0 10px",
            // Левая (плюсовая) шина питания
            borderLeft: `3px solid ${rung.isActive ? "#98c379" : "#5c6370"}`,
            // Правая (нулевая) шина питания
            borderRight: `3px solid #5c6370`
          }}>
            {/* Сквозная горизонтальная линия тока (шина) */}
            <div style={{
              position: "absolute",
              left: 0,
              right: 0,
              height: "2px",
              backgroundColor: rung.isActive ? "#98c379" : "#3c3c3c",
              zIndex: 1
            }} />

            {/* Контейнер для логических элементов */}
            <div style={{
              display: "flex",
              alignItems: "center",
              width: "100%",
              zIndex: 2,
              gap: "5px"
            }}>
              {rung.instructions.map((inst) => renderElement(inst, rung.isActive))}
            </div>
          </div>
        </div>
      ))}
    </div>
  );
};

// Стили компонентов элементов
const elementWrapperStyle: React.CSSProperties = {
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
  width: "70px",
  position: "relative"
};

const operandLabelStyle: React.CSSProperties = {
  fontSize: "11px",
  fontFamily: "monospace",
  marginTop: "2px",
  whiteSpace: "nowrap",
  overflow: "hidden",
  textOverflow: "ellipsis",
  width: "100%",
  textAlign: "center"
};
