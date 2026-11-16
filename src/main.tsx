import React from "react";
import ReactDOM from "react-dom/client";
import { App } from "./App";

// Импортируем базовые стили для корректного отображения интерактивного холста React Flow
import "reactflow/dist/style.css";

// Инициализация и монтирование корневого компонента приложения в DOM-дерево
ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
