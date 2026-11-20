// Предотвращает появление дополнительного окна консоли на Windows в релизной сборке
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod compiler;
mod draft_io;
mod guarded_read;
mod native_filter;
mod sidecar;
mod offline_project;
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
        check_legacy_json_path(&file_path)?;
        let bytes = draft_io::read_bounded(std::path::Path::new(&file_path))?;
        let rungs: Vec<Rung> = serde_json::from_slice(&bytes)
            .map_err(|e| format!("Ошибка разбора JSON проекта: {e}"))?;
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
    static_access: compiler::static_access::AccessOverview,
}

/// Bound native input before allocation-intensive parsing. Read-only, no mutation.
async fn read_cioprg_bytes(file_path: &str) -> Result<Vec<u8>, String> {
    const MAX_CIO_FILE_BYTES: u64 = 16 * 1024 * 1024;

    let path = std::path::Path::new(&file_path);
    if !path.extension().is_some_and(|ext| ext.to_string_lossy().eq_ignore_ascii_case("lst")) {
        return Err("Выберите файл .LST для просмотра".into());
    }
    let selected=path.to_path_buf();
    let bytes=tokio::task::spawn_blocking(move ||
        guarded_read::read_regular(&selected,MAX_CIO_FILE_BYTES as usize))
        .await.map_err(|e|format!("Ошибка безопасного чтения CIO: {e}"))??;
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


#[tauri::command]
async fn cmd_filter_cioprg_lines(
    file_path: String,
    controller: compiler::memory::ControllerFamily,
    expected_sha256: String,
    syntax_class: Option<String>,
    opcode: Option<String>,
    address: Option<String>,
    offset: usize,
) -> Result<native_filter::NativeFilterPage, String> {
    let bytes = read_cioprg_bytes(&file_path).await?;
    let doc = compiler::lossless::parse_document(bytes, controller);
    if doc.source_sha256 != expected_sha256 {
        return Err("Исходный CIOPRG.LST изменился: фильтрация старого снимка запрещена".into());
    }
    native_filter::filter_source(
        &doc, syntax_class.as_deref(), opcode.as_deref(), address.as_deref(), offset
    )
}



/// Query source-linked manual operand READ/WRITE/UNKNOWN with bounded pages.
/// Full-source totals are scanned from a fresh read-only SHA-pinned snapshot.
/// Does not expose a controller writer, live I/O, scan or simulation.
#[tauri::command]
async fn cmd_query_static_access(
    file_path:String,
    controller:compiler::memory::ControllerFamily,
    expected_sha256:String,
    access_kind:Option<String>,
    symbol:Option<String>,
    offset:usize,
)->Result<compiler::static_access::AccessPage,String>{
    let bytes=read_cioprg_bytes(&file_path).await?;
    let doc=compiler::lossless::parse_document(bytes,controller);
    if doc.source_sha256!=expected_sha256 {
        return Err("CIOPRG изменился: данные статического аудита устарели".into());
    }
    compiler::static_access::query(
        &doc,access_kind.as_deref(),symbol.as_deref(),offset
    )
}


#[derive(Serialize)]
struct UnifiedProjectOpened {
    project:offline_project::OfflineProject,
    revision:String,
    source_path:String,
    report:FullCioprgInspection,
    ioname_path:Option<String>,
    ioname:Option<compiler::ioname::IonameReport>,
    sidecar_path:Option<String>,
    sidecar:Option<sidecar::OfflineSidecar>,
    sidecar_revision:Option<String>,
    analysis_changed:bool,
}
#[derive(Serialize)]
struct UnifiedProjectSaved {
    project:offline_project::OfflineProject,
    revision:String,
}
/// One atomic UI-level project opening: load a STRICT portable manifest,
/// re-read all referenced native files with existing protections, check every
/// referenced SHA, and only then deliver an internally consistent snapshot.
/// Files are never modified. Snapshot analysis is RECOMPUTED, not trusted.
#[tauri::command]
async fn cmd_open_offline_project(
    manifest_path:String
)->Result<UnifiedProjectOpened,String>{
    let path=std::path::PathBuf::from(manifest_path);
    offline_project::check_project_path(&path)?;
    let (bytes,revision)=draft_io::read_bounded_with_revision(&path)?;
    let project:offline_project::OfflineProject=serde_json::from_slice(&bytes)
        .map_err(|e|format!("Некорректный манифест проекта: {e}"))?;
    let native_path=offline_project::resolve(&path,&project.source.relative_path)?;
    let source=read_cioprg_bytes(native_path.to_str().ok_or("Неверный путь CIOPRG")?).await?;
    let doc=compiler::lossless::parse_document(source,project.source.controller);
    project.validate(&doc)?;
    let (ioname_path,ioname)=if let Some(binding)=&project.ioname{
        let target=offline_project::resolve(&path,&binding.relative_path)?;
        let bytes=read_ioname_bytes(target.to_str().ok_or("Неверный путь IONAME")?).await?;
        if !offline_project::matches_attachment(binding,&bytes){
            return Err("IONAME.DAT изменён: SHA-256/размер не совпадают с проектом".into());
        }
        (Some(target.to_string_lossy().into_owned()),
            Some(compiler::ioname::inspect(&bytes)))
    }else{(None,None)};
    let (sidecar_path,notes,sidecar_revision)=if let Some(binding)=&project.sidecar{
        let target=offline_project::resolve(&path,&binding.relative_path)?;
        let (bytes,revision)=draft_io::read_bounded_with_revision(&target)?;
        if !offline_project::matches_attachment(binding,&bytes){
            return Err("Sidecar изменён: SHA-256/размер не совпадают с проектом".into());
        }
        let notes:sidecar::OfflineSidecar=serde_json::from_slice(&bytes)
            .map_err(|e|format!("Некорректный JSON sidecar: {e}"))?;
        notes.validate(&doc)?;
        (Some(target.to_string_lossy().into_owned()),Some(notes),Some(revision))
    }else{(None,None,None)};
    // A parser upgrade can legitimately change derived counts. The snapshot
    // is labeled stale; all presented results are recomputed from original bytes.
    let analysis_changed=project.analysis!=offline_project::snapshot(&doc);
    let report=build_full_inspection(&doc);
    Ok(UnifiedProjectOpened {
        project,revision,
        source_path:native_path.to_string_lossy().into_owned(),
        report,ioname_path,ioname,sidecar_path,sidecar:notes,
        sidecar_revision,analysis_changed,
    })
}

/// Save only a separate *.mladproj.json manifest. All referenced files must
/// ALREADY live inside its directory tree. There is NO copy/move of OEM bytes.
/// A sidecar must be saved first and revision-matched, preventing stale
/// unsaved UI annotations from being implicitly included.
#[tauri::command]
async fn cmd_save_offline_project(
    manifest_path:String,
    source_path:String,
    controller:compiler::memory::ControllerFamily,
    expected_source_sha256:String,
    ioname_path:Option<String>,
    expected_ioname_sha256:Option<String>,
    sidecar_path:Option<String>,
    expected_sidecar_revision:Option<String>,
    expected_manifest_revision:Option<String>,
)->Result<UnifiedProjectSaved,String>{
    let path=std::path::PathBuf::from(manifest_path);
    offline_project::check_project_path(&path)?;
    let source_abs=std::path::PathBuf::from(&source_path);
    let source_rel=offline_project::relative_file(&path,&source_abs)?;
    let bytes=read_cioprg_bytes(&source_path).await?;
    let doc=compiler::lossless::parse_document(bytes,controller);
    if doc.source_sha256!=expected_source_sha256 {
        return Err("Исходный CIOPRG изменился: проект не сохранён".into());
    }
    let ioname=if let Some(name)=ioname_path {
        let rel=offline_project::relative_file(&path,std::path::Path::new(&name))?;
        let bytes=read_ioname_bytes(&name).await?;
        if expected_ioname_sha256.as_deref()!=Some(offline_project::sha256(&bytes).as_str()){
            return Err("IONAME изменён: проект не сохранён".into());
        }
        Some(offline_project::attachment(rel,&bytes))
    } else {
        if expected_ioname_sha256.is_some(){
            return Err("IONAME SHA без пути не допускается".into());
        }
        None
    };
    let sidecar_binding=if let Some(name)=sidecar_path {
        let rel=offline_project::relative_file(&path,std::path::Path::new(&name))?;
        let selected=sidecar::check_sidecar_path(&name)?;
        let expected=expected_sidecar_revision.as_deref()
            .ok_or("Sidecar должен быть загружен или сохранён до фиксации проекта")?;
        let (data,revision)=draft_io::read_bounded_with_revision(selected)?;
        if expected!=revision {
            return Err("Sidecar был изменён: сохраните или загрузите его повторно".into());
        }
        let notes:sidecar::OfflineSidecar=serde_json::from_slice(&data)
            .map_err(|e|format!("Неверный JSON sidecar: {e}"))?;
        notes.validate(&doc)?;
        Some(offline_project::attachment(rel,&data))
    }else{
        if expected_sidecar_revision.is_some() {
            return Err("Ревизия sidecar без пути не допускается".into());
        }
        None
    };
    let analysis=offline_project::snapshot(&doc);
    let project=offline_project::OfflineProject::new(
        &doc,source_rel,ioname,sidecar_binding,analysis);
    project.validate(&doc)?;
    let mut data=serde_json::to_vec_pretty(&project)
        .map_err(|e|format!("Ошибка сериализации проекта: {e}"))?;
    data.push(b'\n');
    if data.len()>draft_io::MAX_JSON_BYTES {
        return Err("Манифест превышает 2 МиБ".into());
    }
    reject_existing_symlink(&path).await?;
    let revision=draft_io::write_atomic_with_revision(
        &path,&data,expected_manifest_revision.as_deref())?;
    Ok(UnifiedProjectSaved{project,revision})
}

#[derive(Serialize)]
struct OfflineSidecarRead {
    sidecar: sidecar::OfflineSidecar,
    revision: String,
}
#[derive(Serialize)]
struct OfflineSidecarSaved {
    revision: String,
}

/// Load only a portable user-authored JSON sidecar. Recheck the immutable
/// native source SHA/profile before applying any saved annotations.
#[tauri::command]
async fn cmd_load_offline_sidecar(
    file_path: String,
    source_path: String,
    controller: compiler::memory::ControllerFamily,
    expected_source_sha256: String,
) -> Result<OfflineSidecarRead,String> {
    let selected=sidecar::check_sidecar_path(&file_path)?.to_path_buf();
    let source=read_cioprg_bytes(&source_path).await?;
    let doc=compiler::lossless::parse_document(source,controller);
    if doc.source_sha256!=expected_source_sha256 {
        return Err("CIOPRG изменился: нельзя применять sidecar к другому снимку".into());
    }
    let (bytes,revision)=tokio::task::spawn_blocking(move ||
        draft_io::read_bounded_with_revision(&selected))
        .await.map_err(|e|format!("Ошибка чтения sidecar: {e}"))??;
    let sidecar:sidecar::OfflineSidecar=serde_json::from_slice(&bytes)
        .map_err(|e|format!("Некорректный JSON sidecar: {e}"))?;
    sidecar.validate(&doc)?;
    Ok(OfflineSidecarRead{sidecar,revision})
}

/// Save a portable sidecar only, after validating the CURRENT native source.
/// A first save needs an absent target; overwriting an existing sidecar
/// requires an opaque revision from a previous load/save in this session.
#[tauri::command]
async fn cmd_save_offline_sidecar(
    file_path: String,
    source_path: String,
    controller: compiler::memory::ControllerFamily,
    expected_source_sha256: String,
    sidecar: sidecar::OfflineSidecar,
    expected_revision: Option<String>,
) -> Result<OfflineSidecarSaved,String> {
    let selected=sidecar::check_sidecar_path(&file_path)?.to_path_buf();
    let source=read_cioprg_bytes(&source_path).await?;
    let doc=compiler::lossless::parse_document(source,controller);
    if doc.source_sha256!=expected_source_sha256 {
        return Err("CIOPRG изменился: сохранение sidecar запрещено".into());
    }
    sidecar.validate(&doc)?;
    let mut content=serde_json::to_vec_pretty(&sidecar)
        .map_err(|e|format!("Ошибка сериализации sidecar: {e}"))?;
    content.push(b'\n');
    if content.len()>draft_io::MAX_JSON_BYTES {
        return Err("Sidecar превышает предел 2 МиБ".into());
    }
    reject_existing_symlink(&selected).await?;
    let revision=tokio::task::spawn_blocking(move ||
        draft_io::write_atomic_with_revision(&selected,&content,expected_revision.as_deref()))
        .await.map_err(|e|format!("Ошибка сохранения sidecar: {e}"))??;
    Ok(OfflineSidecarSaved{revision})
}

/// Import only a *display snapshot* of IONAME.DAT, without editing source or
/// assuming its sequential IN/OUT indices are CIO #XXXXX physical addresses.
#[tauri::command]
async fn cmd_inspect_ioname(file_path: String)
    -> Result<compiler::ioname::IonameReport, String> {
    let bytes=read_ioname_bytes(&file_path).await?;
    Ok(compiler::ioname::inspect(&bytes))
}

/// Read-only bounded IONAME snapshot shared by native inspection and project rehydration.
async fn read_ioname_bytes(file_path:&str)->Result<Vec<u8>,String>{
    const MAX_IONAME_BYTES: u64 = 4 * 1024 * 1024;
    let path=std::path::Path::new(&file_path);
    if !path.file_name().and_then(|n|n.to_str())
        .is_some_and(|n|n.eq_ignore_ascii_case("IONAME.DAT")) {
        return Err("Для просмотра выберите IONAME.DAT (копию файла)".into());
    }
    let selected=path.to_path_buf();
    let bytes=tokio::task::spawn_blocking(move ||
        guarded_read::read_regular(&selected,MAX_IONAME_BYTES as usize))
        .await.map_err(|e|format!("Ошибка безопасного чтения IONAME: {e}"))??;
    Ok(bytes)
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
            draft_io::verify_parent_directory(&path)?;
            let file = tokio::fs::symlink_metadata(&path).await
                .map_err(|e| format!("Не удалось проверить выбранный файл: {e}"))?;
            if guarded_read::is_indirection(&file) || !file.is_file() {
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
    Ok(build_full_inspection(&doc))
}
fn build_full_inspection(doc:&compiler::lossless::LosslessDocument)->FullCioprgInspection {
    let semantic = compiler::semantic::analyze(doc);
    let advanced = compiler::advanced_source::inspect(doc);
    let sections = compiler::native_sections::scan(doc);
    let dataflow = compiler::typed_dataflow::inspect(doc, 1000);
    let static_access = compiler::static_access::inspect(doc,160);
    FullCioprgInspection {
        base: doc.inspection_report(600, 500),
        semantic: semantic.preview(200, 200, 500),
        advanced,
        sections,
        dataflow,
        static_access,
    }
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
    draft_io::verify_parent_directory(path)?;
    match tokio::fs::symlink_metadata(path).await {
        Ok(meta) if guarded_read::is_indirection(&meta) =>
            Err("Сохранение через символическую ссылку или reparse point запрещено".into()),
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
    let bytes = draft_io::read_bounded(path)?;
    let text = std::str::from_utf8(&bytes).map_err(|e| format!("Ошибка UTF-8: {e}"))?;
    let (draft, _) = compiler::graph_draft::read_draft_json(text)?;
    Ok(draft)
}

/// Graph drafts are JSON-only, with an atomic per-file replacement + backup.
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
    let target=path.to_path_buf();
    tokio::task::spawn_blocking(move || draft_io::write_atomic(&target, content.as_bytes()))
        .await.map_err(|e| format!("Ошибка потока сохранения JSON: {e}"))??;
    Ok(result)
}

fn check_legacy_json_path(file_path: &str) -> Result<(), String> {
    let name = std::path::Path::new(file_path).file_name()
        .and_then(|name| name.to_str())
        .ok_or("Не указан путь к JSON-проекту")?;
    if !name.ends_with(".json") || name.ends_with(".graph.json") || name.ends_with(".advanced.json") || name.ends_with(".mlad.json") || name.ends_with(".mladproj.json") {
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
    let bytes=draft_io::read_bounded(path)?;
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
    let target=path.to_path_buf();
    tokio::task::spawn_blocking(move || draft_io::write_atomic(&target,json.as_bytes()))
        .await.map_err(|e| format!("Ошибка потока сохранения advanced JSON: {e}"))??;
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
    let target=std::path::PathBuf::from(file_path);
    tokio::task::spawn_blocking(move || draft_io::write_atomic(&target,json_content.as_bytes()))
        .await.map_err(|e| format!("Ошибка потока сохранения legacy JSON: {e}"))??;
        
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
            cmd_filter_cioprg_lines,
            cmd_query_static_access,
            cmd_open_offline_project,
            cmd_save_offline_project,
            cmd_load_offline_sidecar,
            cmd_save_offline_sidecar,
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
mod source_linked_access_ipc_tests {
    use super::*;
    #[tokio::test]
    async fn source_linked_access_rejects_changed_original_and_untrusted_filter(){
        let folder=std::env::temp_dir().join(format!(
            "mlad-static-access-{}",uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&folder).unwrap();
        let path=folder.join("CIOPRG.LST");
        let raw=b"STR #70010\r\nMOV M010,M011\nEND\n//ALARM1\nOUT #70020\n";
        std::fs::write(&path,raw).unwrap();
        let file_path=path.to_string_lossy().into_owned();
        let controller=compiler::memory::ControllerFamily::YRC1000;
        let doc=compiler::lossless::parse_document(raw.to_vec(),controller);
        let expected_sha256=doc.source_sha256.clone();
        let first=cmd_query_static_access(
            file_path.clone(),controller,expected_sha256.clone(),
            Some("Read".into()),None,0
        ).await.unwrap();
        assert_eq!(first.total_matches,2);
        assert_eq!(first.hits[0].line,1);
        assert_eq!(first.hits[1].line,2);
        assert_eq!(&raw[first.hits[0].span.start..first.hits[0].span.end],b"#70010");
        assert!(!first.execution_verified);
        assert!(!first.write_permission_verified);
        assert!(cmd_query_static_access(
            file_path.clone(),controller,expected_sha256.clone(),
            Some("Read".into()),Some("M01".into()),0
        ).await.is_err());
        assert!(cmd_query_static_access(
            file_path.clone(),controller,"0".repeat(64),
            Some("Write".into()),None,0
        ).await.is_err());
        std::fs::write(&path,b"END\n").unwrap();
        assert!(cmd_query_static_access(
            file_path.clone(),controller,expected_sha256,
            Some("Read".into()),None,0
        ).await.is_err());
        assert_eq!(std::fs::read(&path).unwrap(),b"END\n");
        std::fs::remove_dir_all(folder).unwrap();
    }
}


#[cfg(test)]
mod unified_offline_project_ipc_tests {
    use super::*;
    fn temporary()->std::path::PathBuf {
        let folder=std::env::temp_dir().join(format!(
            "mlad-unified-{}",uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&folder).unwrap();
        folder
    }

    #[tokio::test]
    async fn complete_project_roundtrip_recomputes_and_verifies_all_sources() {
        let root=temporary();
        let source=root.join("CIOPRG.LST");
        let ioname=root.join("IONAME.DAT");
        let notes=root.join("cell.mlad.json");
        let manifest=root.join("cell.mladproj.json");
        let source_bytes=b"STR #70010\r\nOUT #70020\nEND\n";
        let ioname_bytes=b"/IONAME\n//IN\nSafety,,Motor,\n";
        std::fs::write(&source,source_bytes).unwrap();
        std::fs::write(&ioname,ioname_bytes).unwrap();
        let source_path=source.to_string_lossy().into_owned();
        let sidecar_path=notes.to_string_lossy().into_owned();
        let manifest_path=manifest.to_string_lossy().into_owned();
        let ioname_path=ioname.to_string_lossy().into_owned();
        let family=compiler::memory::ControllerFamily::YRC1000;
        let doc=compiler::lossless::parse_document(source_bytes.to_vec(),family);
        let mut sidecar=sidecar::OfflineSidecar::new(&doc);
        sidecar.annotations.addresses.insert("#70010".into(),"Safety".into());
        sidecar.bookmarks.push(sidecar::Bookmark{line:1,label:"Entry".into()});
        let saved_notes=cmd_save_offline_sidecar(
            sidecar_path.clone(),source_path.clone(),family,
            doc.source_sha256.clone(),sidecar.clone(),None
        ).await.unwrap();
        let saved=cmd_save_offline_project(
            manifest_path.clone(),source_path.clone(),family,doc.source_sha256.clone(),
            Some(ioname_path.clone()),Some(offline_project::sha256(ioname_bytes)),
            Some(sidecar_path.clone()),Some(saved_notes.revision.clone()),None
        ).await.unwrap();
        assert_eq!(saved.project.source.relative_path,"CIOPRG.LST");
        assert_eq!(saved.project.ioname.as_ref().unwrap().relative_path,"IONAME.DAT");
        assert_eq!(saved.project.sidecar.as_ref().unwrap().relative_path,"cell.mlad.json");
        assert!(!saved.project.analysis.execution_verified);
        let manifest_bytes=std::fs::read(&manifest).unwrap();
        assert!(!manifest_bytes.windows(source_bytes.len()).any(|s|s==source_bytes));
        assert!(!String::from_utf8_lossy(&manifest_bytes).contains(source_path.as_str()),
            "portable manifest cannot store original absolute paths");
        let opened=cmd_open_offline_project(manifest_path.clone()).await.unwrap();
        assert!(!opened.analysis_changed);
        assert_eq!(opened.project,saved.project);
        assert_eq!(opened.revision,saved.revision);
        assert_eq!(opened.report.base.source_sha256,doc.source_sha256);
        assert_eq!(opened.report.static_access.read_sites,1);
        assert_eq!(opened.report.static_access.write_sites,1);
        assert!(!opened.report.edited_export_allowed());
        assert!(opened.ioname.as_ref().unwrap().entries.len()>=4);
        assert_eq!(opened.sidecar.unwrap(),sidecar);
        assert_eq!(opened.sidecar_revision,Some(saved_notes.revision));
        assert_eq!(std::fs::read(&source).unwrap(),source_bytes);
        assert_eq!(std::fs::read(&ioname).unwrap(),ioname_bytes);
        assert!(cmd_save_offline_project(
            manifest_path.clone(),source_path.clone(),family,doc.source_sha256.clone(),
            None,None,None,None,None
        ).await.is_err(),"existing manifest requires revision");
        assert!(cmd_save_offline_project(
            manifest_path.clone(),source_path.clone(),family,doc.source_sha256.clone(),
            None,None,None,None,Some("0".repeat(64))
        ).await.is_err(),"stale revision must not replace");
        assert!(cmd_save_offline_project(
            manifest_path.clone(),source_path.clone(),family,doc.source_sha256.clone(),
            None,None,None,None,Some(saved.revision)
        ).await.is_ok(),"explicitly removing optional components creates a new revision");
        assert_eq!(std::fs::read(&source).unwrap(),source_bytes);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[tokio::test]
    async fn project_refuses_renamed_components_unknown_fields_and_changed_digests(){
        let root=temporary();
        let source=root.join("CIOPRG.LST");
        let ioname=root.join("IONAME.DAT");
        let manifest=root.join("cell.mladproj.json");
        let raw=b"STR #70010\nOUT #70020\nEND\n";
        std::fs::write(&source,raw).unwrap();
        std::fs::write(&ioname,b"/IONAME\n//IN\none,,,\n").unwrap();
        let path=manifest.to_string_lossy().into_owned();
        let src=source.to_string_lossy().into_owned();
        let family=compiler::memory::ControllerFamily::YRC1000;
        let doc=compiler::lossless::parse_document(raw.to_vec(),family);
        assert!(cmd_save_offline_project(
            path.clone(),src.clone(),family,"0".repeat(64),None,None,None,None,None
        ).await.is_err());
        let saved=cmd_save_offline_project(
            path.clone(),src.clone(),family,doc.source_sha256.clone(),
            Some(ioname.to_string_lossy().into_owned()),
            Some(offline_project::sha256(b"/IONAME\n//IN\none,,,\n")),
            None,None,None
        ).await.unwrap();
        std::fs::write(&ioname,b"/IONAME\n//IN\ntwo,,,\n").unwrap();
        assert!(cmd_open_offline_project(path.clone()).await.is_err(),
            "modified IONAME cannot be silently matched by order");
        std::fs::write(&ioname,b"/IONAME\n//IN\none,,,\n").unwrap();
        let outside=root.parent().unwrap().join("outside").join("CIOPRG.LST");
        assert!(cmd_save_offline_project(
            path.clone(),outside.to_string_lossy().into_owned(),family,
            doc.source_sha256.clone(),None,None,None,None,
            Some(saved.revision.clone())
        ).await.is_err(),"source outside project root must fail");
        let mut text=serde_json::to_value(&saved.project).unwrap();
        text["controller_ip"]=serde_json::json!("192.0.2.1");
        std::fs::write(&manifest,serde_json::to_vec(&text).unwrap()).unwrap();
        assert!(cmd_open_offline_project(path.clone()).await.is_err());
        std::fs::write(&manifest,serde_json::to_vec(&saved.project).unwrap()).unwrap();
        assert!(cmd_open_offline_project(path.clone()).await.is_ok());
        std::fs::write(&source,b"END\n").unwrap();
        assert!(cmd_open_offline_project(path).await.is_err(),
            "native source drift must fail closed even with same filename");
        std::fs::remove_dir_all(root).unwrap();
    }
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
        assert!(check_legacy_json_path("robot.mlad.json").is_err());
        assert!(check_legacy_json_path("cell.mladproj.json").is_err());
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


    #[cfg(unix)]
    #[tokio::test]
    async fn read_only_native_file_through_symlinked_parent_is_rejected(){
        let root=std::env::temp_dir().join(format!(
            "mlad-native-parent-{}", uuid::Uuid::new_v4()));
        let real=root.join("regular");
        std::fs::create_dir_all(&real).unwrap();
        let cioprg=real.join("CIOPRG.LST");
        let ioname=real.join("IONAME.DAT");
        std::fs::write(&cioprg,b"/CIOPRG DX\r\nEND\r\n").unwrap();
        std::fs::write(&ioname,b"synthetic").unwrap();
        let link=root.join("link");
        std::os::unix::fs::symlink(&real,&link).unwrap();
        assert!(read_cioprg_bytes(link.join("CIOPRG.LST")
            .to_string_lossy().as_ref()).await.is_err());
        assert!(cmd_inspect_ioname(link.join("IONAME.DAT")
            .to_string_lossy().to_string()).await.is_err());
        assert_eq!(std::fs::read(&cioprg).unwrap(),b"/CIOPRG DX\r\nEND\r\n");
        assert_eq!(std::fs::read(&ioname).unwrap(),b"synthetic");
        std::fs::remove_dir_all(&root).unwrap();
    }
    #[cfg(unix)]
    #[tokio::test]
    async fn read_only_native_direct_symlink_is_rejected(){
        let root=std::env::temp_dir().join(format!(
            "mlad-native-file-{}",uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&root).unwrap();
        let real=root.join("source.LST");
        let alias=root.join("CIOPRG.LST");
        std::fs::write(&real,b"/CIOPRG DX\r\nEND\r\n").unwrap();
        std::os::unix::fs::symlink(&real,&alias).unwrap();
        assert!(read_cioprg_bytes(alias.to_string_lossy().as_ref()).await.is_err());
        assert_eq!(std::fs::read(&real).unwrap(),b"/CIOPRG DX\r\nEND\r\n");
        std::fs::remove_dir_all(&root).unwrap();
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


#[cfg(test)]
mod offline_sidecar_ipc_tests {
    use super::*;
    fn temporary()->std::path::PathBuf {
        let root=std::env::temp_dir().join(format!("mlad-sidecar-{}",uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&root).unwrap();
        root
    }
    #[tokio::test]
    async fn export_import_revision_conflict_and_source_binding_are_enforced() {
        let root=temporary();
        let native=root.join("CIOPRG.LST");
        let saved=root.join("session.mlad.json");
        let raw=b"STR #70010\r\nOUT #70020\r\nEND\n";
        std::fs::write(&native,raw).unwrap();
        let source_path=native.to_string_lossy().into_owned();
        let file_path=saved.to_string_lossy().into_owned();
        let controller=compiler::memory::ControllerFamily::YRC1000;
        let doc=compiler::lossless::parse_document(raw.to_vec(),controller);
        let mut notes=sidecar::OfflineSidecar::new(&doc);
        notes.bookmarks.push(sidecar::Bookmark{line:2,label:"first".into()});
        notes.annotations.addresses.insert("#70010".into(),"Entry".into());
        let first=cmd_save_offline_sidecar(
            file_path.clone(),source_path.clone(),controller,doc.source_sha256.clone(),
            notes.clone(),None
        ).await.unwrap();
        assert_eq!(first.revision.len(),64);
        assert!(cmd_save_offline_sidecar(
            file_path.clone(),source_path.clone(),controller,doc.source_sha256.clone(),
            notes.clone(),None
        ).await.is_err(),"explicit revision required for existing JSON");
        let loaded=cmd_load_offline_sidecar(
            file_path.clone(),source_path.clone(),controller,doc.source_sha256.clone()
        ).await.unwrap();
        assert_eq!(loaded.sidecar,notes);
        assert_eq!(loaded.revision,first.revision);
        let old_bytes=std::fs::read(&saved).unwrap();
        notes.bookmarks[0].label="second".into();
        let second=cmd_save_offline_sidecar(
            file_path.clone(),source_path.clone(),controller,doc.source_sha256.clone(),
            notes.clone(),Some(first.revision.clone())
        ).await.unwrap();
        assert_ne!(first.revision,second.revision);
        assert_eq!(std::fs::read(saved.with_file_name("session.mlad.json.bak")).unwrap(),old_bytes);
        assert!(cmd_save_offline_sidecar(
            file_path.clone(),source_path.clone(),controller,doc.source_sha256.clone(),
            notes,Some(first.revision)
        ).await.is_err(),"stale sidecar revisions must never overwrite");
        assert_eq!(std::fs::read(&native).unwrap(),raw);
        std::fs::write(&native,b"END\n").unwrap();
        assert!(cmd_load_offline_sidecar(
            file_path.clone(),source_path.clone(),controller,doc.source_sha256.clone()
        ).await.is_err(),"changed source cannot use stale sidecar");
        assert!(cmd_save_offline_sidecar(
            file_path,source_path,controller,doc.source_sha256.clone(),
            sidecar::OfflineSidecar::new(&doc),Some(second.revision)
        ).await.is_err(),"changed source must block sidecar write");
        std::fs::remove_dir_all(root).unwrap();
    }
    #[tokio::test]
    async fn mismatched_sidecar_and_native_target_are_rejected_without_write() {
        let root=temporary();let source=root.join("CIOPRG.LST");
        let source_path=source.to_string_lossy().into_owned();
        let raw=b"STR #70010\nEND\n";
        std::fs::write(&source,raw).unwrap();
        let family=compiler::memory::ControllerFamily::YRC1000;
        let doc=compiler::lossless::parse_document(raw.to_vec(),family);
        let notes=sidecar::OfflineSidecar::new(&doc);
        for invalid in ["CIOPRG.LST","IONAME.DAT","old.graph.json","old.advanced.json","legacy.json"] {
            let target=root.join(invalid).to_string_lossy().into_owned();
            assert!(cmd_save_offline_sidecar(
                target,source_path.clone(),family,doc.source_sha256.clone(),
                notes.clone(),None
            ).await.is_err());
        }
        let mut wrong=notes.clone();wrong.source.sha256="0".repeat(64);
        let target=root.join("safe.mlad.json").to_string_lossy().into_owned();
        assert!(cmd_save_offline_sidecar(
            target.clone(),source_path.clone(),family,doc.source_sha256.clone(),
            wrong,None
        ).await.is_err());
        assert!(!root.join("safe.mlad.json").exists());
        assert!(cmd_save_offline_sidecar(
            target,source_path,family,"0".repeat(64),notes,None
        ).await.is_err());
        assert_eq!(std::fs::read(source).unwrap(),raw);
        std::fs::remove_dir_all(root).unwrap();
    }
}
