// Предотвращает появление дополнительного окна консоли на Windows в релизной сборке
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod compiler;
pub mod network;

use serde::{Deserialize, Serialize};
use tracing::{info, error, warn};
use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::{State, Emitter};

// Глобальное потокобезопасное состояние сетевого клиента
pub struct AppNetworkState {
    pub hse_client: Arc<Mutex<Option<network::YaskawaHseClient>>>,
    pub monitor_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
}

// Структура данных инструкции для десериализации запросов от фронтенда
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LadderInstruction {
    pub id: String,
    #[serde(rename = "type")]
    pub inst_type: String,
    pub operand: String,
}

// Структура данных цепи (Rung) для обмена данными через IPC
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rung {
    pub id: usize,
    pub comment: Option<String>,
    pub instructions: Vec<LadderInstruction>,
    #[serde(rename = "isActive")]
    pub is_active: bool,
}

/// Команда Tauri: Загрузка и парсинг файла проекта (.json или оригинального .LST)
#[tauri::command]
async fn cmd_load_project(file_path: String) -> Result<Vec<Rung>, String> {
    info!("Запрос на загрузку проекта из файла: {}", file_path);
    
    if file_path.ends_with(".lst") || file_path.ends_with(".LST") {
        // The old lossy parser silently discards unknown CIO instructions.
        // Do not present it as a safe importer for a real controller export.
        Err("Импорт CIOPRG.LST временно заблокирован: требуется lossless parser и проверка System/User Ladder".to_string())
    } else {
        // Чтение стандартного Git-friendly JSON проекта нашей среды
        let content = std::fs::read_to_string(&file_path)
            .map_err(|e| format!("Не удалось прочитать файл проекта: {}", e))?;
        let rungs: Vec<Rung> = serde_json::from_str(&content)
            .map_err(|e| format!("Ошибка разбора JSON проекта: {}", e))?;
        Ok(rungs)
    }
}

#[derive(Serialize)]
struct FullCioprgInspection {
    #[serde(flatten)]
    base: compiler::lossless::InspectionReport,
    semantic: compiler::semantic::SemanticPreview,
}

/// Read-only inspection of a native exported CIOPRG.LST file.
/// This command neither changes the source file nor creates a controller-ready
/// program. The original byte stream stays in Rust memory during parsing only.
#[tauri::command]
async fn cmd_inspect_cioprg(
    file_path: String,
    controller: compiler::memory::ControllerFamily,
) -> Result<FullCioprgInspection, String> {
    const MAX_CIO_FILE_BYTES: u64 = 16 * 1024 * 1024;

    let path = std::path::Path::new(&file_path);
    if !path.extension().is_some_and(|ext| ext.to_string_lossy().eq_ignore_ascii_case("lst")) {
        return Err("Выберите файл .LST для просмотра".into());
    }
    let metadata = tokio::fs::metadata(path).await
        .map_err(|e| format!("Не удалось прочитать метаданные файла: {e}"))?;
    if !metadata.is_file() || metadata.len() > MAX_CIO_FILE_BYTES {
        return Err("Файл должен быть обычным .LST размером не более 16 МиБ".into());
    }
    let bytes = tokio::fs::read(path).await
        .map_err(|e| format!("Не удалось прочитать .LST: {e}"))?;
    if bytes.len() as u64 > MAX_CIO_FILE_BYTES {
        return Err("Файл .LST превышает ограничение 16 МиБ".into());
    }
    // Bound line metadata and diagnostic allocation even for hostile text made
    // almost entirely of newline bytes. CRLF is one line terminator.
    let mut line_breaks = 0usize;
    for (i, byte) in bytes.iter().enumerate() {
        if *byte == b'\n' || (*byte == b'\r' && bytes.get(i + 1) != Some(&b'\n')) {
            line_breaks += 1;
            if line_breaks > 200_000 {
                return Err("Слишком много строк в .LST (предел 200000)".into());
            }
        }
    }

    let doc = compiler::lossless::parse_document(bytes, controller);
    let semantic = compiler::semantic::analyze(&doc);
    Ok(FullCioprgInspection {
        base: doc.inspection_report(600, 500),
        semantic: semantic.preview(200, 200, 500),
    })
}

// Only filenames ending in .graph.json are treated as editable graph drafts.
// This prevents accidental mixing with the old project.json or CIOPRG.LST.
fn check_graph_draft_path(file_path: &str) -> Result<&std::path::Path, String> {
    let path = std::path::Path::new(file_path);
    let file_name = path.file_name()
        .and_then(|s| s.to_str())
        .ok_or("Не указан корректный путь к черновику")?;
    if file_path.len() > 1024 || !file_name.ends_with(".graph.json") {
        return Err("Черновик должен иметь имя *.graph.json (не CIOPRG.LST и не project.json)".into());
    }
    Ok(path)
}

/// Offline, non-mutating draft validation. No device/network access.
#[tauri::command]
fn cmd_validate_graph_draft(
    project: compiler::graph_draft::GraphDraft,
) -> compiler::graph_draft::DraftValidation {
    compiler::graph_draft::validate_graph_draft(&project)
}

/// Only loads an independent, versioned graph draft, never a CIOPRG file.
#[tauri::command]
async fn cmd_load_graph_draft(
    file_path: String,
) -> Result<compiler::graph_draft::GraphDraft, String> {
    let path = check_graph_draft_path(&file_path)?;
    let info = tokio::fs::metadata(path).await
        .map_err(|e| format!("Не удалось прочитать черновик: {e}"))?;
    if !info.is_file() || info.len() > 2 * 1024 * 1024 {
        return Err("Размер черновика превышает 2 МиБ или путь не является файлом".into());
    }
    let bytes = tokio::fs::read(path).await
        .map_err(|e| format!("Ошибка чтения черновика: {e}"))?;
    let text = std::str::from_utf8(&bytes).map_err(|e| format!("Ошибка UTF-8: {e}"))?;
    let (draft, _) = compiler::graph_draft::read_draft_json(text)?;
    Ok(draft)
}

/// Graph drafts are JSON-only, with a best-effort backup of the previous JSON.
/// Invalid *relay addresses* can be saved for later editing; invalid structure
/// (duplicate node IDs, wrong version, excessive depth/size) is rejected.
#[tauri::command]
async fn cmd_save_graph_draft(
    file_path: String,
    project: compiler::graph_draft::GraphDraft,
) -> Result<compiler::graph_draft::DraftValidation, String> {
    let path = check_graph_draft_path(&file_path)?;
    let (content, result) = compiler::graph_draft::checked_draft_json(&project)?;
    if path.exists() {
        let info = tokio::fs::metadata(path).await
            .map_err(|e| format!("Не удалось проверить существующий черновик: {e}"))?;
        if !info.is_file() { return Err("Путь не является обычным файлом".into()); }
        let backup = path.with_file_name(format!(
            "{}.bak", path.file_name().unwrap().to_string_lossy()
        ));
        tokio::fs::copy(path, backup).await
            .map_err(|e| format!("Не удалось создать резервную копию: {e}"))?;
    }
    tokio::fs::write(path, content).await
        .map_err(|e| format!("Не удалось сохранить графический черновик: {e}"))?;
    Ok(result)
}

/// Команда Tauri: Валидация и сохранение чернового проекта (только JSON)
#[tauri::command]
async fn cmd_save_project(file_path: String, data: Vec<Rung>) -> Result<Vec<compiler::ValidationError>, String> {
    info!("Запрос на сохранение проекта. Количество цепей: {}", data.len());
    
    // 1. Запуск статического анализа логики на лету (Double Coil, валидация адресов бит)
    let validation_errors = compiler::validate_rungs(&data);
    if !validation_errors.is_empty() {
        warn!("При валидации схемы обнаружено проблем: {}", validation_errors.len());
    }

    // 2. Сериализация в формат JSON для удобного версионирования в Git
    let json_content = serde_json::to_string_pretty(&data)
        .map_err(|e| format!("Ошибка сериализации данных: {}", e))?;
    std::fs::write(&file_path, json_content)
        .map_err(|e| format!("Не удалось записать JSON на диск: {}", e))?;
        
    // STOP: /JOB, //INST and IOCPT are not a verified CIOPRG.LST format.
    // Save only the editable JSON project until a lossless controller-specific
    // compiler and golden-file round-trip tests have been implemented.
    info!("JSON проект сохранен. Экспорт CIOPRG.LST заблокирован до верификации.");

    // Return diagnostics to the UI. The JSON draft is still preserved.
    Ok(validation_errors)
}

/// Команда Tauri: Переключение режима онлайн-мониторинга через UDP/HSE протокол
#[tauri::command]
async fn cmd_toggle_monitoring(
    ip: String, 
    active: bool, 
    state: State<'_, AppNetworkState>, 
    app_handle: tauri::AppHandle
) -> Result<(), String> {
    let mut monitor_lock = state.monitor_handle.lock().await;
    
    // Останавливаем текущий поток мониторинга, если он запущен
    if let Some(handle) = monitor_lock.take() {
        handle.abort();
        info!("Поток циклического опроса робота остановлен.");
    }

    if active {
        info!("Запуск асинхронного онлайн-мониторинга робота: {}:8040", ip);
        
        let hse_client = network::YaskawaHseClient::new(&ip, 8040)
            .await
            .map_err(|e| format!("Ошибка инициализации HSE сокета: {}", e))?;
            
        let client_arc = Arc::new(hse_client);
        
        // Создаем фоновый асинхронный поток опроса памяти с помощью tokio
        let new_handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(100));
            let mut req_id = 0u8;
            
            loop {
                interval.tick().await;
                req_id = req_id.wrapping_add(1);
                
                // Запрашиваем состояние байта пользовательских реле 70010 (класс 0x01, смещение 0)
                // Спецификацию классов и смещений сигналов см. в docs/memory_map.md
                match client_arc.read_relay_byte(0x01, 0, req_id).await {
                    Ok(relay_byte) => {
                        // Формируем карту состояний бит для отправки во фронтенд через события Tauri
                        let mut state_map = std::collections::HashMap::new();
                        state_map.insert("70010".to_string(), (relay_byte & 0x01) != 0);
                        state_map.insert("70011".to_string(), (relay_byte & 0x02) != 0);
                        state_map.insert("70012".to_string(), (relay_byte & 0x04) != 0);
                        
                        let _ = app_handle.emit("ladder-runtime-sync", state_map);
                    },
                    Err(e) => {
                        error!("Ошибка опроса состояния памяти контроллера: {}", e);
                        let _ = app_handle.emit("robot-disconnected", ());
                    }
                }
            }
        });
        
        *monitor_lock = Some(new_handle);
    }
    
    Ok(())
}

/// Команда Tauri: Деплой запрещён до аппаратной и форматной верификации
#[tauri::command]
async fn cmd_deploy_to_robot(_ip: String, _local_path: String) -> Result<(), String> {
    // No verified CIOPRG.LST compiler or safe deployment procedure exists yet.
    // Block all uploads regardless of user-supplied path or credentials.
    Err("Загрузка на робот отключена: отсутствует подтвержденная спецификация CIOPRG.LST и проверенная процедура деплоя".to_string())
}

fn main() {
    // Инициализация подсистемы логирования и трассировки для диагностики в Antigravity
    tracing_subscriber::fmt::init();
    info!("Запуск бэкенда среды разработки Next-Gen Yaskawa Motoman Ladder IDE...");

    tauri::Builder::default()
        .manage(AppNetworkState {
            hse_client: Arc::new(Mutex::new(None)),
            monitor_handle: Arc::new(Mutex::new(None)),
        })
        .invoke_handler(tauri::generate_handler![
            cmd_load_project,
            cmd_inspect_cioprg,
            cmd_validate_graph_draft,
            cmd_load_graph_draft,
            cmd_save_graph_draft,
            cmd_save_project,
            cmd_toggle_monitoring,
            cmd_deploy_to_robot
        ])
        .run(tauri::generate_context!())
        .expect("Критическая ошибка при запуске runtime-окружения Tauri");
}
