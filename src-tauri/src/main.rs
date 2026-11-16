// Предотвращает появление дополнительного окна консоли на Windows в релизной сборке
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod compiler;
pub mod network;

use serde::{Deserialize, Serialize};
use tracing::{info, error, warn};
use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::{State, Manager};

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
        // Чтение и парсинг оригинального текстового листинга Yaskawa с помощью nom
        let content = std::fs::read_to_string(&file_path)
            .map_err(|e| format!("Не удалось прочитать .LST файл: {}", e))?;
        let parsed_rungs = compiler::parser::parse_lst_to_rungs(&content);
        info!("Успешно распарсен нативный файл .LST. Найдено цепей: {}", parsed_rungs.len());
        Ok(parsed_rungs)
    } else {
        // Чтение стандартного Git-friendly JSON проекта нашей среды
        let content = std::fs::read_to_string(&file_path)
            .map_err(|e| format!("Не удалось прочитать файл проекта: {}", e))?;
        let rungs: Vec<Rung> = serde_json::from_str(&content)
            .map_err(|e| format!("Ошибка разбора JSON проекта: {}", e))?;
        Ok(rungs)
    }
}

/// Команда Tauri: Валидация и сохранение проекта (в JSON и экспорт в .LST)
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
        
    // 3. Автоматический экспорт сопутствующего нативного файла листинга .LST для робота
    let lst_path = file_path.replace(".json", ".lst");
    let mut lst_content = String::from("/JOB\n//NAME CONCURRENT\n//POS\n///NPOS 0,0,0,0,0,0\n//INST\nIOCPT STRSTART\n");
    
    for rung in &data {
        if let Some(ref comment) = rung.comment {
            lst_content.push_str(&format!("; {}\n", comment));
        }
        for inst in &rung.instructions {
            if inst.inst_type == "TMR" {
                lst_content.push_str(&format!("TMR {}\n", inst.operand));
            } else {
                lst_content.push_str(&format!("{} {}\n", inst.inst_type, inst.operand));
            }
        }
    }
    lst_content.push_str("END\n");
    
    let _ = std::fs::write(lst_path, lst_content);
    info!("Проект успешно сохранен и экспортирован в .LST");
    
    // Возвращаем список ошибок/предупреждений на фронтенд для подсветки проблемных мест
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

/// Команда Tauri: Деплой скомпилированного листинга логики на робота по FTP
#[tauri::command]
async fn cmd_deploy_to_robot(ip: String, local_path: String) -> Result<(), String> {
    info!("Запуск процедуры FTP деплоя на робота: {}", ip);
    let deployer = network::RobotFtpDeployer::new(&ip);
    
    deployer.upload_ladder_file(&local_path, "CIOLAD.LST")
        .await
        .map_err(|e| format!("Ошибка FTP загрузки: {}", e))?;
        
    Ok(())
}

fn main() {
    // Инициализация подсистемы логирования и трассировки для диагностики в Antigravity
    tracing_subscriber::fmt::init();
    info!("Запуск бэкенда среды разработки Next-Gen Yaskawa Motoman Ladder IDE...");

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_network::init())
        .manage(AppNetworkState {
            hse_client: Arc::new(Mutex::new(None)),
            monitor_handle: Arc::new(Mutex::new(None)),
        })
        .invoke_handler(tauri::generate_handler![
            cmd_load_project,
            cmd_save_project,
            cmd_toggle_monitoring,
            cmd_deploy_to_robot
        ])
        .run(tauri::generate_context!())
        .expect("Критическая ошибка при запуске runtime-окружения Tauri");
}
