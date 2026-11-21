<!-- MLAD-I18N:EDITORIAL:restored 2026-11-19 from predominantly Russian original -->
> **Язык / Language:** [Русский](architecture.ru.md) · [English](architecture.en.md) · [Исходный текст / Original](architecture.md)
>
> **Примечание:** перевод не заменяет исходную спецификацию; русскоязычный оригинал сохранён без машинного искажения. / Technical source fidelity retained; OEM verification remains independent.

> **Действующий архитектурный/процессный контекст (21.11.2026):** `main` `9fcd11d61856c8a559d9acf3c77bd662ad814dfe`, Windows pre-release `v0.5.0-rc.10-unified-offline`. Native `CIOPRG.LST` и `IONAME.DAT` неизменяемы; `*.mlad.json` и `*.mladproj.json` — отдельные метаданные. `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`; `HSES`/`FTP`/native writer/deploy `BLOCKED`. [Аудит Git](repository_audit_2026-11-21.ru.md); прежние этапы ниже исторические.

# Архитектура MLAD и границы доверия

**Фактический baseline 18.11.2026:** `main` `f85ddfcce3063dd087425be981a9d2b7a16d6309` после [merge PR #29](https://github.com/uzbatsu-beep/MLAD/pull/29). [Linux CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454) и [Windows CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621) успешны. [Project status](project_status.ru.md), [Roadmap v3](roadmap.ru.md), [Backlog](implementation_plan.ru.md). Native source строго read-only; FTP/HSES/deploy отключены.

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
    ├── guarded_read.rs — bounded snapshot и identity/reparse проверки
    ├── draft_io.rs — staged JSON write, backup, conflict guard
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

Backend не предоставляет рабочей команды сохранения изменённого CIOPRG и не включает программу робота. PR #24 уже слит (JSON staged/rename). **PR #25 merged** реализует production CSP и проверки предков путей; CSP **в опубликованном rc.9 ещё отсутствует**, а PR #29 добавил File ID/byte snapshots, hardlink rejection и Windows ADS/UNC/device path deny. Остаются race windows/TOCTOU, ACL/power-loss и Windows user smoke. [Обзор](filesystem_security_pr29.ru.md).

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

1. PR #25 merged: CSP/ancestor checks; PR #26 merged: guarded reads/conflicts; PR #28 merged: реальный Windows NTFS test workflow; PR #29 merged: File ID + bytes, hardlinks, staged SHA-256, Windows path deny. Остаются last check→rename TOCTOU, directory handles, ACL/Windows edge cases, concurrent writers и отсутствие транзакции backup+target.
2. Реальные golden fixtures с provenance по DX100/DX200/YRC1000 и firmware.
3. Полная native grammar, byte/word aliasing, System/User ownership.
4. Взаимодействие флагов и условий EN при полном скане; крайние MOV byte=256, SHL/SHR n=0/16.
5. Portable sidecar с сохранением комментариев и контроль хэшей внешних файлов.
6. Third-party SBOM/notices, подпись Windows EXE и пользовательский smoke test.

Открытые темы — в [roadmap](roadmap.ru.md); OEM испытания — в [матрице](semantic_verification_matrix.ru.md). Прохождение Rust/TypeScript/Windows CI само по себе не закрывает эти риски.


## 6. PR #24 — offline JSON persistence (merged, не в выпущенном rc.9)

Три IPC-метода записи JSON используют один Rust `draft_io::write_atomic` для временного файла с уникальным именем, `sync_all` и замены файла в той же папке. Читатели Graph/Advanced/legacy JSON ограничены 2 МиБ и отвергают прямые symlink. Резервная копия `*.bak` формируется отдельно и **не атомарна одновременно** с целевым JSON. Это **не** capability grant для записи CIOPRG.LST или IONAME.DAT, no FTP/HSES. См. [модель безопасности записи](json_persistence_safety.ru.md).


## 7. PR #25 — CSP and linked path rejection (merged)

Настройка `app.security.csp` ограничивает external loads в packaged WebView; `devCsp` отдельно позволяет только Vite localhost HMR. Inline CSS/React styles требуют исключения `style-src 'unsafe-inline'`; JS остаётся без `unsafe-inline` и `unsafe-eval`. Для файлов `draft_io::verify_parent_directory` предварительно выявляет symlink ancestors/ParentDir, но не удерживает дескриптор директории от изменения. [Подробные ограничения](csp_path_hardening.ru.md).


## 8. PR #26 — bounded identity-checked read and JSON conflict guard (merged)

`guarded_read::read_regular` использует единый дескриптор для содержимого (на Windows — дополнительные контрольные дескрипторы), предварительно и после чтения сверяет Unix dev/inode или Windows volume/file index. Проверяются symlink/reparse, размер и mtime. Native CIOPRG и IONAME используют `spawn_blocking` и не предоставляют writer. `draft_io::write_atomic` сравнивает байты target перед финальным rename; возвращает конфликт вместо silent lost update. Подробно: [guarded_io_review.md](guarded_io_review.ru.md).

## 9. PR #28/#29 — Windows regression и filesystem hardening (merged)

[Windows workflow](../.github/workflows/windows-security-acceptance.yml) после PR #28 исполняет тесты на Windows runner, компилирует Tauri EXE с production CSP и формирует временный unsigned artifact. В PR #29 JSON read/save расширены `FileIdentity` snapshots и запретом hardlinked target/backup; staged JSON сверяет File ID и SHA-256 перед переименованием. Прямые Windows alternate streams, UNC/device/verbatim и drive-relative пути отклоняются; для работы выбирать обычные локальные копии. [Threat review](filesystem_security_pr29.ru.md). **Не гарантируются** directory-handle atomicity, сохранение ACL, crash durability и транзакция пары `backup+target`. В `Cargo.toml` всё ещё указан `suppaftp` как зависимость — отдельная проверка зависимостей не означает, что сетевой runtime активен.

## 10. Архитектурный маршрут после PR #29

**Следующие отдельные изменения:** synthetic native parser corpus → source↔rung navigation → versioned sidecar `*.mlad.json` → evidence-based typed syntax → source-linked static analysis → unified Offline Engineering IDE → условный Windows 0.6.x preview. [Implementation plan](implementation_plan.ru.md) · [Verification gates](verification_gates.ru.md) · [Decisions D1–D9](engineering_decisions.ru.md). Ни один документ не санкционирует запись native или подключение контроллера.
