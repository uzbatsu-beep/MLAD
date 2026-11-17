# Архитектура MLAD и границы доверия

**Статус от 18.11.2026:** отражает `main` `39c5ead8d1b9ac63b49be607312dc8b3976549dd` после merged PR #21. Подробности: [актуальный статус](project_status.md) и [roadmap](roadmap.md). Все нативные исходники только для чтения; сеть и деплой отключены.

## 1. Слои и поток данных

```text
React / TypeScript / Tauri WebView
├── App.tsx
│   ├── NativeWorkspace.tsx — основная read-only программа робота
│   │   ├── исходник CIOPRG / дерево / поиск / пагинация / диагностика
│   │   ├── LadderCircuit и AdvancedBlockDiagram — только supported preview
│   │   ├── IONAME и локальные комментарии (localStorage)
│   │   └── typed operand / static M-links / OEM reference справка
│   ├── GraphWorkspace.tsx — отдельные *.graph.json drafts
│   ├── AdvancedBlocksPanel.tsx — отдельные *.advanced.json drafts
│   └── Legacy project.json — только совместимость
│
└── Tauri IPC (src-tauri/src/main.rs)
    ├── cmd_inspect_cioprg / cmd_read_cioprg_page / cmd_search_cioprg
    ├── cmd_inspect_ioname / cmd_pick_native_file (Open only)
    ├── compiler/lossless.rs — immutable bytes, SHA-256, spans
    ├── compiler/native_sections.rs — PART/END/tail views
    ├── compiler/semantic.rs — ограниченный Boolean AST
    ├── compiler/advanced_source.rs — source-pattern previews
    ├── compiler/memory.rs — family-specific address classes
    ├── compiler/typed_dataflow.rs — hypothetical register links
    ├── compiler/reference_semantics.rs — isolated manual-derived vectors
    │   (НЕ подключено к CIO исполняющему AST или Tauri command)
    ├── compiler/ioname.rs — ограниченный read-only структурный preview
    └── legacy draft/parser helpers — отдельные JSON модели
```

**Network runtime:** старый `src-tauri/src/network/mod.rs` удалён в PR #13; `pub mod network` в `main.rs` отсутствует. Tauri `cmd_toggle_monitoring` отказывает во включении; `cmd_deploy_to_robot` отказывает всегда. Эта граница означает OFFLINE, но не является полной проверкой security sandbox.

## 2. Форматы, сохранение и права

| Формат | Операция | Статус |
| --- | --- | --- |
| `CIOPRG.LST` | Native Open + read-only inspect/paging/search/hash | Оригинал неизменяем. Нет native writer, recompiler, overwrite |
| `IONAME.DAT` | Native Open + bounded structural preview | Только чтение, индексы на #XXXXX не подтверждены |
| Локальные native комментарии | localStorage, ключи по SHA-256 | **Не** OEM-аннотации в файле; portable sidecar ещё нет |
| `*.graph.json` | Валидация/сохранение собственного формата | Редактируемый отдельный JSON draft, не native CIO |
| `*.advanced.json` | Валидация/сохранение собственного формата | Отдельный JSON draft, не native CIO |
| `project.json` | Старый совместимый формат | Legacy, не конвертер на робота |
| Локальные TMR/CNT/GSTR test steps | Детерминированные **ручные** шаги | Не реальный scan/IO |
| HSES / FTP | Deny-only Tauri IPC | **OFFLINE / запрещено** |

Backend не предоставляет рабочей команды сохранения изменённого CIOPRG и не включает программу робота. Действующие проверки файлов не заменяют полноценный sandbox: **CSP сейчас `null`**; filesystem races/symlinks/TOCTOU, hardlinks, ограничения каталогов и атомарность JSON требуют независимого audit.

## 3. Три уровня интерпретации

1. **Lexical/structural:** исходные байты, BOM, CRLF/LF/CR, spans, PART/END, наблюдаемые секции ALARM/MESSAGE после END. Unknown остаётся unknown.
2. **Typed syntax:** `MOV/ADD/SUB/SHL/SHR` как `TypedCandidate`, операнды и возможные M-ссылки. Все новые «связи» — только текстовые гипотезы; boolean AST получает barrier, не исполнения.
3. **Manual-derived reference:** изолированная функция проверки заданных вручную 16-bit векторов и OEM-справка в UI. Не читает и не модифицирует значения из `CIOPRG.LST`; `controller_execution_verified=false`, `scan_chain_verified=false`.

**Нет** четвёртого уровня OEM-certified execution. Флаги Carry/Zero/Error относятся к руководству и reference-векторам, не к состояниям контроллера.

## 4. Диагностика и границы интерфейса

- Native Workspace: исходный файл постранично, раздельные supported/typed/opaque, все доступные семантические цепи и advanced source preview, раздел ALARM/MESSAGE, группировка диагностик с сохранением переходов.
- Установка масштабов независима для Ladder и advanced. Выбор исходников — только системный Open dialog.
- В редактируемом JSON — двойной щелчок/F2 меняет JSON адреса. В native представлении двойной щелчок меняет **только локальные комментарии**, не сам адрес.
- OFFLINE ручное TRUE/FALSE/UNKNOWN — не live telemetry. PART 1/2 — наблюдаемые метки, не подтверждённые System/User rights.

## 5. Открытые security/engineering долги

1. CSP, filesystem symlink/TOCTOU и атомарное JSON сохранение.
2. Реальные golden fixtures с provenance по DX100/DX200/YRC1000 и firmware.
3. Полная native grammar, byte/word aliasing, System/User ownership.
4. Взаимодействие флагов и условий EN при полном скане; крайние MOV byte=256, SHL/SHR n=0/16.
5. Portable sidecar с сохранением комментариев и контроль хэшей внешних файлов.
6. Third-party SBOM/notices, подпись Windows EXE и пользовательский smoke test.

Открытые темы — в [roadmap](roadmap.md); OEM испытания — в [матрице](semantic_verification_matrix.md). Прохождение Rust/TypeScript/Windows CI само по себе не закрывает эти риски.
