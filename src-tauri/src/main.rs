// Предотвращает появление дополнительного окна консоли на Windows в релизной сборке
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod compiler;
// Offline-only build: no compiled robot networking implementation.

use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use tauri_plugin_dialog::DialogExt;
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
    advanced: compiler::advanced_source::AdvancedSourceReport,
    sections: Vec<compiler::native_sections::SourceSectionMarker>,
    dataflow: compiler::typed_dataflow::StaticAudit,
}

/// Bound native input before allocation-intensive parsing. Read-only, no mutation.
async fn read_cioprg_bytes(file_path: &str) -> Result<Vec<u8>, String> {
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

    Ok(bytes)
}

#[derive(Serialize)]
struct CioprgReadPage {
    source_sha256: String,
    start_line: usize,
    line_count: usize,
    lines: Vec<compiler::lossless::LinePreview>,
}

/// Read a small display page of the *same* unchanged snapshot that was inspected.
/// Changes on disk invalidate the old digest rather than silently showing mixed files.
#[tauri::command]
async fn cmd_read_cioprg_page(
    file_path: String,
    controller: compiler::memory::ControllerFamily,
    expected_sha256: String,
    start_line: usize,
) -> Result<CioprgReadPage, String> {
    const PAGE_SIZE: usize = 250;
    if start_line == 0 || start_line > 200_000 {
        return Err("Недопустимая начальная строка (1..200000)".into());
    }
    let bytes = read_cioprg_bytes(&file_path).await?;
    let doc = compiler::lossless::parse_document(bytes, controller);
    if doc.source_sha256 != expected_sha256 {
        return Err("Файл изменился после первичного просмотра. Откройте его повторно.".into());
    }
    if start_line > doc.lines.len() && !(start_line == 1 && doc.lines.is_empty()) {
        return Err("Начальная строка находится за концом файла".into());
    }
    Ok(CioprgReadPage {
        source_sha256: doc.source_sha256.clone(),
        start_line,
        line_count: doc.lines.len(),
        lines: doc.line_preview_range(start_line, PAGE_SIZE),
    })
}


#[derive(Serialize)]
struct SourceSearchHit {
    line: usize,
    preview: String,
}

#[derive(Serialize)]
struct CioprgSearchResult {
    source_sha256: String,
    hits: Vec<SourceSearchHit>,
    truncated: bool,
}

/// ASCII case-insensitive bounded read-only search. Every request checks the
/// original digest, so results cannot silently combine multiple revisions.
#[tauri::command]
async fn cmd_search_cioprg(
    file_path: String,
    controller: compiler::memory::ControllerFamily,
    expected_sha256: String,
    query: String,
) -> Result<CioprgSearchResult, String> {
    let needle = query.trim().as_bytes().to_vec();
    if needle.len() < 2 || needle.len() > 64 ||
        needle.iter().any(|b| !b.is_ascii_graphic() && *b != b' ') {
        return Err("Поиск: от 2 до 64 печатных ASCII-символов".into());
    }
    let bytes = read_cioprg_bytes(&file_path).await?;
    let doc = compiler::lossless::parse_document(bytes, controller);
    if doc.source_sha256 != expected_sha256 {
        return Err("Файл изменился после открытия. Откройте его повторно.".into());
    }
    let mut hits = Vec::new();
    let mut truncated = false;
    for line in &doc.lines {
        let body = &doc.original_bytes()[line.body_span.start..line.body_span.end];
        if body.windows(needle.len()).any(|slice| slice.eq_ignore_ascii_case(&needle)) {
            if hits.len() >= 100 {
                truncated = true;
                break;
            }
            hits.push(SourceSearchHit {
                line: line.number,
                preview: String::from_utf8_lossy(&body[..body.len().min(180)]).into_owned(),
            });
        }
    }
    Ok(CioprgSearchResult { source_sha256: doc.source_sha256, hits, truncated })
}

/// Import only a *display snapshot* of IONAME.DAT, without editing source or
/// assuming its sequential IN/OUT indices are CIO #XXXXX physical addresses.
#[tauri::command]
async fn cmd_inspect_ioname(file_path: String)
    -> Result<compiler::ioname::IonameReport, String> {
    const MAX_IONAME_BYTES: u64 = 4 * 1024 * 1024;
    let path=std::path::Path::new(&file_path);
    if !path.file_name().and_then(|n|n.to_str())
        .is_some_and(|n|n.eq_ignore_ascii_case("IONAME.DAT")) {
        return Err("Для просмотра выберите IONAME.DAT (копию файла)".into());
    }
    let meta=tokio::fs::symlink_metadata(path).await
        .map_err(|e|format!("Нет доступа к IONAME.DAT: {e}"))?;
    if meta.file_type().is_symlink() || !meta.is_file() || meta.len()>MAX_IONAME_BYTES {
        return Err("IONAME.DAT: обычный файл без symlink размером до 4 МиБ".into());
    }
    let bytes=tokio::fs::read(path).await
        .map_err(|e|format!("Не удалось прочитать IONAME.DAT: {e}"))?;
    if bytes.len() as u64>MAX_IONAME_BYTES {
        return Err("IONAME.DAT превышает 4 МиБ".into());
    }
    Ok(compiler::ioname::inspect(&bytes))
}

/// Allow only two explicitly supported native *read-only* input types.
fn validate_native_pick(kind: &str, path: &std::path::Path) -> Result<(), String> {
    if !path.is_absolute() {
        return Err("Диалог вернул не абсолютный путь".into());
    }
    if kind == "cioprg" {
        if !path.extension().is_some_and(|e| e.to_string_lossy().eq_ignore_ascii_case("lst")) {
            return Err("Выберите файл .LST".into());
        }
    } else if kind == "ioname" {
        if !path.file_name().and_then(|n| n.to_str())
            .is_some_and(|n| n.eq_ignore_ascii_case("IONAME.DAT")) {
            return Err("Выберите IONAME.DAT".into());
        }
    } else {
        return Err("Недопустимый тип входного файла".into());
    }
    Ok(())
}

/// Desktop native OPEN dialog. The command does not read or mutate files.
/// Existing cmd_inspect_cioprg/cmd_inspect_ioname still enforce size/type bounds.
/// Rust-only integration avoids granting WebView any file save dialog capability.
#[tauri::command]
async fn cmd_pick_native_file(
    app: tauri::AppHandle,
    kind: String,
) -> Result<Option<String>, String> {
    let (title, extension): (&str, &str) = match kind.as_str() {
        "cioprg" => ("Yaskawa CIOPRG.LST (только чтение)", "lst"),
        "ioname" => ("Yaskawa IONAME.DAT (только чтение)", "dat"),
        _ => return Err("Недопустимый тип входного файла".into()),
    };
    let (tx, rx) = tokio::sync::oneshot::channel();
    app.dialog().file().set_title(title).add_filter(title, &[extension])
        .pick_file(move |picked| { let _ = tx.send(picked); });
    let selected = rx.await.map_err(|_| "Не удалось получить результат диалога".to_string())?;
    match selected {
        None => Ok(None), // User cancelled, keep both previous file states.
        Some(value) => {
            let path = value.into_path().map_err(|_| "Выбранный путь не является локальным файлом".to_string())?;
            validate_native_pick(&kind, &path)?;
            let file = tokio::fs::symlink_metadata(&path).await
                .map_err(|e| format!("Не удалось проверить выбранный файл: {e}"))?;
            if file.file_type().is_symlink() || !file.is_file() {
                return Err("Можно открыть только обычный файл, не ссылку".into());
            }
            let max_size = if kind == "cioprg" { 16 * 1024 * 1024 } else { 4 * 1024 * 1024 };
            if file.len() > max_size {
                return Err("Размер выбранного файла превышает допустимое значение".into());
            }
            path.to_str().map(|s| Some(s.to_owned()))
                .ok_or_else(|| "Путь содержит неподдерживаемые символы".into())
        }
    }
}

#[cfg(test)]
mod native_file_picker_guard_tests {
    use super::*;
    #[test]
    fn dialog_input_type_cannot_be_extended_to_save_or_upload() {
        use std::path::Path;
        assert!(validate_native_pick("cioprg", Path::new("CIOPRG.LST")).is_err());
        #[cfg(windows)]
        {
            assert!(validate_native_pick("cioprg", Path::new("C:\\robot\\CIOPRG.LST")).is_ok());
            assert!(validate_native_pick("ioname", Path::new("C:\\robot\\IONAME.DAT")).is_ok());
            assert!(validate_native_pick("ioname", Path::new("C:\\robot\\OTHER.DAT")).is_err());
        }
        #[cfg(not(windows))]
        {
            assert!(validate_native_pick("cioprg", Path::new("/tmp/CIOPRG.LST")).is_ok());
            assert!(validate_native_pick("ioname", Path::new("/tmp/IONAME.DAT")).is_ok());
            assert!(validate_native_pick("ioname", Path::new("/tmp/other.dat")).is_err());
        }
        assert!(validate_native_pick("save", Path::new("/tmp/CIOPRG.LST")).is_err());
    }
}

/// Read-only inspection of a native exported CIOPRG.LST file.
/// This command neither changes the source file nor creates a controller-ready
/// program. The original byte stream stays in Rust memory during parsing only.
#[tauri::command]
async fn cmd_inspect_cioprg(
    file_path: String,
    controller: compiler::memory::ControllerFamily,
) -> Result<FullCioprgInspection, String> {
    let bytes = read_cioprg_bytes(&file_path).await?;
    let doc = compiler::lossless::parse_document(bytes, controller);
    let semantic = compiler::semantic::analyze(&doc);
    let advanced = compiler::advanced_source::inspect(&doc);
    let sections = compiler::native_sections::scan(&doc);
    let dataflow = compiler::typed_dataflow::inspect(&doc, 1000);
    Ok(FullCioprgInspection {
        base: doc.inspection_report(600, 500),
        semantic: semantic.preview(200, 200, 500),
        advanced,
        sections,
        dataflow,
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

// Reject direct symlink targets before any JSON write. This is a
// defense-in-depth check, not a substitute for filesystem ACLs or locking.
async fn reject_existing_symlink(path: &std::path::Path) -> Result<(), String> {
    match tokio::fs::symlink_metadata(path).await {
        Ok(meta) if meta.file_type().is_symlink() =>
            Err("Сохранение через символическую ссылку запрещено".into()),
        Ok(meta) if !meta.is_file() =>
            Err("Сохранение допускается только в обычный файл".into()),
        Ok(_) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(format!("Ошибка проверки целевого файла: {e}")),
    }
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
    reject_existing_symlink(path).await?;
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

fn check_legacy_json_path(file_path: &str) -> Result<(), String> {
    let name = std::path::Path::new(file_path).file_name()
        .and_then(|name| name.to_str())
        .ok_or("Не указан путь к JSON-проекту")?;
    if !name.ends_with(".json") || name.ends_with(".graph.json") || name.ends_with(".advanced.json") {
        return Err("Старый редактор может сохранять только обычный *.json, но не *.graph.json или *.LST".into());
    }
    Ok(())
}

/// Separate advanced-block JSON workspace. This file must NEVER be a
/// controller export or overwrite a legacy graph / CIOPRG program.
fn check_advanced_path(file_path: &str) -> Result<&std::path::Path, String> {
    let path=std::path::Path::new(file_path);
    let name=path.file_name().and_then(|s|s.to_str())
        .ok_or("Неверный путь advanced-черновика")?;
    if file_path.len()>1024 || !name.ends_with(".advanced.json") {
        return Err("Для TMR/CNT/GRP разрешены только файлы *.advanced.json".into());
    }
    Ok(path)
}

#[tauri::command]
fn cmd_validate_advanced_draft(
    project: compiler::advanced::AdvancedDraft,
) -> compiler::advanced::Validation {
    compiler::advanced::validate(&project)
}

#[tauri::command]
async fn cmd_load_advanced_draft(
    file_path: String,
) -> Result<compiler::advanced::AdvancedDraft,String> {
    let path=check_advanced_path(&file_path)?;
    let meta=tokio::fs::metadata(path).await.map_err(|e|e.to_string())?;
    if !meta.is_file()||meta.len()>2*1024*1024 {
        return Err("Недопустимый файл или превышение 2 МиБ".into());
    }
    let bytes=tokio::fs::read(path).await.map_err(|e|e.to_string())?;
    let s=std::str::from_utf8(&bytes).map_err(|e|e.to_string())?;
    Ok(compiler::advanced::deserialize_checked(s)?.0)
}

#[tauri::command]
async fn cmd_save_advanced_draft(
    file_path: String,
    project: compiler::advanced::AdvancedDraft,
) -> Result<compiler::advanced::Validation,String> {
    let path=check_advanced_path(&file_path)?;
    reject_existing_symlink(path).await?;
    let (json,validation)=compiler::advanced::serialize_checked(&project)?;
    if path.exists() {
        let filename=path.file_name().unwrap().to_string_lossy();
        let backup=path.with_file_name(format!("{filename}.bak"));
        reject_existing_symlink(&backup).await?;
        tokio::fs::copy(path,backup).await.map_err(|e|format!("Backup: {e}"))?;
    }
    tokio::fs::write(path,json).await.map_err(|e|e.to_string())?;
    Ok(validation)
}

/// Pure offline timer/counter test helpers. No connected I/O, time source,
/// HSES, or robot execution. Explicit sample each time.
#[tauri::command]
fn cmd_test_tmr_step(
    mut state: compiler::advanced::TimerState, start:bool, preset:u16,
) -> compiler::advanced::TimerState {
    state.tick(start,preset);state
}
#[tauri::command]
fn cmd_test_cnt_step(
    mut state: compiler::advanced::CounterState,
    count:bool,reset:bool,preset:u16,
) -> compiler::advanced::CounterState {
    state.scan(count,reset,preset);state
}
#[tauri::command]
fn cmd_test_group_copy(source:u8)->u8 {
    compiler::advanced::group_copy(source)
}

/// Команда Tauri: Валидация и сохранение чернового проекта (только JSON)
#[tauri::command]
async fn cmd_save_project(file_path: String, data: Vec<Rung>) -> Result<Vec<compiler::ValidationError>, String> {
    check_legacy_json_path(&file_path)?;
    reject_existing_symlink(std::path::Path::new(&file_path)).await?;
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

/// Offline-only deny stub; argument names preserved for existing Tauri callers.
#[tauri::command]
async fn cmd_toggle_monitoring(ip: String, active: bool) -> Result<(), String> {
    let _ = ip;
    if active {
        Err("Мониторинг HSES отключён: сетевой протокол и семантика данных не верифицированы".into())
    } else {
        // Stop remains idempotent; there is no network client or task to stop.
        Ok(())
    }
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
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            cmd_load_project,
            cmd_inspect_cioprg,
            cmd_pick_native_file,
            cmd_read_cioprg_page,
            cmd_search_cioprg,
            cmd_inspect_ioname,
            cmd_validate_advanced_draft,
            cmd_load_advanced_draft,
            cmd_save_advanced_draft,
            cmd_test_tmr_step,
            cmd_test_cnt_step,
            cmd_test_group_copy,
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

#[cfg(test)]
mod graph_write_safety_tests {
    use super::*;

    #[test]
    fn legacy_writer_rejects_native_and_new_graph_formats() {
        assert!(check_legacy_json_path("CIOPRG.LST").is_err());
        assert!(check_legacy_json_path("robot.lst").is_err());
        assert!(check_legacy_json_path("ladder.graph.json").is_err());
        assert!(check_legacy_json_path("ladder.advanced.json").is_err());
        assert!(check_legacy_json_path("project.json").is_ok());
    }


    #[cfg(unix)]
    #[tokio::test]
    async fn existing_symlink_to_native_file_is_not_a_writable_json_target() {
        let root = std::env::temp_dir().join(format!("mlad-guard-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&root).unwrap();
        let real = root.join("CIOPRG.LST");
        let alias = root.join("draft.graph.json");
        std::fs::write(&real, b"original robot data").unwrap();
        std::os::unix::fs::symlink(&real, &alias).unwrap();
        assert!(reject_existing_symlink(&alias).await.is_err());
        assert_eq!(std::fs::read(&real).unwrap(), b"original robot data");
        std::fs::remove_file(&alias).unwrap();
        std::fs::remove_file(&real).unwrap();
        std::fs::remove_dir(&root).unwrap();
    }

    #[test]
    fn graph_draft_path_never_targets_native_files() {
        assert!(check_graph_draft_path("CIOPRG.LST").is_err());
        assert!(check_graph_draft_path("project.json").is_err());
        assert!(check_graph_draft_path("ladder.graph.json").is_ok());
        assert!(check_graph_draft_path("example.graph.json").is_ok());
    }
    #[test]
    fn advanced_path_guard_separates_three_file_formats() {
        assert!(check_advanced_path("CIOPRG.LST").is_err());
        assert!(check_advanced_path("ladder.graph.json").is_err());
        assert!(check_advanced_path("project.json").is_err());
        assert!(check_advanced_path("ladder.advanced.json").is_ok());
    }

}

#[cfg(test)]
mod offline_controller_safety_tests {
    use super::*;

    #[tokio::test]
    async fn monitoring_enable_is_denied_for_every_address() {
        for ip in ["127.0.0.1", "192.0.2.4", "not-an-ip", ""] {
            assert!(cmd_toggle_monitoring(ip.into(), true).await.is_err());
        }
    }

    #[tokio::test]
    async fn monitoring_stop_has_no_live_state_to_touch() {
        assert!(cmd_toggle_monitoring("".into(), false).await.is_ok());
    }

    #[tokio::test]
    async fn deployment_is_always_rejected_even_for_native_filename() {
        for (ip, path) in [
            ("192.0.2.4", "CIOPRG.LST"),
            ("127.0.0.1", "draft.graph.json"),
            ("", "../CIOPRG.LST"),
        ] {
            assert!(cmd_deploy_to_robot(ip.into(), path.into()).await.is_err());
        }
    }
}

#[cfg(test)]
mod cioprg_page_safety_tests {
    use super::*;

    #[tokio::test]
    async fn original_remains_byte_identical_across_pages_and_rejects_stale_digest() {
        let path = std::env::temp_dir().join(format!("mlad-page-{}.LST", uuid::Uuid::new_v4()));
        let content = (1..=710).map(|n| format!("* test-{n}\r\n"))
            .collect::<String>();
        std::fs::write(&path, content.as_bytes()).unwrap();
        let file_path = path.to_string_lossy().into_owned();
        let initial = cmd_inspect_cioprg(file_path.clone(), compiler::memory::ControllerFamily::YRC1000)
            .await.unwrap();
        let digest = initial.base.source_sha256;
        let page = cmd_read_cioprg_page(
            file_path.clone(), compiler::memory::ControllerFamily::YRC1000, digest.clone(), 501,
        ).await.unwrap();
        assert_eq!(page.start_line, 501);
        assert_eq!(page.lines.first().unwrap().line, 501);
        assert_eq!(page.lines.last().unwrap().line, 710);
        assert_eq!(std::fs::read(&path).unwrap(), content.as_bytes());
        assert!(cmd_read_cioprg_page(
            file_path.clone(), compiler::memory::ControllerFamily::YRC1000, digest.clone(), 0,
        ).await.is_err());
        std::fs::write(&path, b"* modified\r\n").unwrap();
        assert!(cmd_read_cioprg_page(
            file_path.clone(), compiler::memory::ControllerFamily::YRC1000, digest, 1,
        ).await.is_err());
        std::fs::remove_file(path).unwrap();
    }
}

#[cfg(test)]
mod cioprg_search_tests {
    use super::*;
    #[tokio::test]
    async fn search_reads_source_only_and_checks_snapshot_digest() {
        let path = std::env::temp_dir().join(format!("mlad-search-{}.LST", uuid::Uuid::new_v4()));
        let contents = b"/CIOPRG DX\r\nPART 1\r\nMOV M010,M011\r\nEND\r\n//ALARM1\r\n";
        std::fs::write(&path, contents).unwrap();
        let p = path.to_string_lossy().into_owned();
        let initial = cmd_inspect_cioprg(p.clone(), compiler::memory::ControllerFamily::DX200)
            .await.unwrap();
        let hash = initial.base.source_sha256;
        assert!(initial.sections.iter().any(|s| s.category == "Alarm" && s.line == 5));
        let result = cmd_search_cioprg(
            p.clone(), compiler::memory::ControllerFamily::DX200, hash.clone(), "mov".into(),
        ).await.unwrap();
        assert_eq!(result.hits.len(), 1);
        assert_eq!(result.hits[0].line, 3);
        assert_eq!(std::fs::read(&path).unwrap(), contents);
        assert!(cmd_search_cioprg(
            p.clone(), compiler::memory::ControllerFamily::DX200, hash.clone(), "".into(),
        ).await.is_err());
        std::fs::write(&path, b"changed\r\n").unwrap();
        assert!(cmd_search_cioprg(
            p.clone(), compiler::memory::ControllerFamily::DX200, hash, "MOV".into(),
        ).await.is_err());
        std::fs::remove_file(path).unwrap();
    }
}
