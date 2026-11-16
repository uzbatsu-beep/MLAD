import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev
export default defineConfig({
  plugins: [react()],

  // Предотвращаем очистку экрана Vite, чтобы логи ошибок Tauri не пропадали
  clearScreen: false,

  // Настройки сервера разработки для Tauri
  server: {
    port: 5173,
    strictPort: true,
    watch: {
      // Игнорируем изменения в директории src-tauri, чтобы избежать двойного перезапуска
      ignored: ["**/src-tauri/**"],
    },
  },

  // Опции сборки фронтенда приложения
  build: {
    // Tauri использует Chromium на Windows и WebKit на macOS/Linux
    target: process.env.TAURI_ENV_PLATFORM == "windows" ? "chrome105" : "safari13",
    // Не минифицировать сборку при дебаге для детального анализа стека ошибок
    minify: !process.env.TAURI_ENV_DEBUG ? "esbuild" : false,
    // Включаем генерацию sourcemaps для удобной отладки в IDE
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
  },
});
