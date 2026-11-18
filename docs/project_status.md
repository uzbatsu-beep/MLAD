# MLAD — актуальное состояние проекта

**Дата сверки:** 18 ноября 2026 года.  
**Проверенный baseline:** GitHub `main` @ `bdb443eb65dc134edc2e11ffc3230c9dfa6257df`.  
**Последний слитый PR:** [#27](https://github.com/uzbatsu-beep/MLAD/pull/27); PR #1–#27 merged. PR #28 — Draft в отдельной ветке, **не в main**.
**Основной CI:** [MLAD validation — SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44237125310) на указанном `main`. [Windows PR #26 — SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44227145031), но user WebView2 smoke **NOT RUN**.
**Последний опубликованный Windows pre-release:** [v0.5.0-rc.9-oem-reference](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference); соответствующий [Windows workflow — SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44162078155).  
**Версия приложения в манифестах:** `0.5.0`; суффиксы `rc.9` применяются к release tag / Windows preview, а не означают релиз 1.0 или промышленную готовность.

> **Область действия этого документа:** проверенное состояние `main` на указанном SHA. Новые PR или релизы после даты сверки не считаются отражёнными здесь автоматически. Исторические release notes описывают **именно свои артефакты**, а не текущую ветку.

## 1. Продукт и границы

MLAD (Motoman Ladder Analysis & Draft IDE) — **инженерный OFFLINE/BENCH-прототип** для анализа Yaskawa Concurrent I/O (профили DX100, DX200, YRC1000), просмотра оригинальных файлов и редактирования **отдельных JSON-черновиков**.

**Не является:** сертифицированным Yaskawa Ladder Editor, нативным компилятором, подтверждённым симулятором сканирования, средством программирования роботов или аппаратным монитором.

**Инварианты:** исходный `CIOPRG.LST` неизменяем; `IONAME.DAT` неизменяем; native writer/export и FTP/HSES запрещены; System/User Ladder права не подтверждены; TRUE/FALSE/UNKNOWN в локальных моделях не являются живыми I/O. Успех CI не означает успешных испытаний на контроллере.

## 2. Что реально включено в main

| Подсистема | Реализовано и проверяется тестами | Принципиальные ограничения | PR |
| --- | --- | --- | --- |
| Профили адресации | Типизированные классы, диапазоны и `Mxxx` по семейству контроллера | Нет firmware-specific authority/проверки физической карты | #1 |
| Lossless `CIOPRG.LST` | Read-only import, SHA-256, byte spans, CRLF/LF/CR, постраничное чтение и поиск | Нет полного round-trip изменённого native файла | #2, #14 |
| Semantic AST / Ladder | Подмножество булевой логики, распознанные цепи, переходы к строкам | Unsupported/неоднозначная топология создаёт барьер; лимит preview | #3 |
| JSON Graph Draft | SVG, AND/OR/NO/NC/OUT, Undo/Redo, проверка адресов | `*.graph.json` не является CIOPRG | #4, #7, #8 |
| Advanced Draft | TMR/CNT/GSTR/GOUT, локальные test steps, offline diagrams | Не настоящий scan cycle / физическое устройство | #5, #7, #8 |
| Native Workspace | Главная область с деревом, исходником, Ladder, блоками, диагностикой | Native файл остаётся read-only | #15 |
| UI и импорт | Все **доступные в SemanticPreview** цепи, масштаб, системные Open dialog, компактная шапка | Не весь текст автоматически имеет достоверную Ladder-диаграмму | #16, #17 |
| `IONAME.DAT` | Ограниченный структурный просмотр, локальные комментарии и ручные ассоциации | Нет OEM-проверенной автоматической привязки IONAME→#XXXXX; исходный DAT не меняется | #16 |
| Post-END | После точного `END` секции `ALARM/MESSAGE` рассматриваются как данные; группировка диагностики | Наблюдаемый формат, не универсальный vendor контракт | #18 |
| Typed opcodes | `MOV/ADD/SUB/SHL/SHR`, категории операндов, проверки форм и исходные spans | Не допущены к исполняемому Boolean AST | #19 |
| Статические ссылки | Предположительные соседние M-register site links | Не доказывают фактическую передачу значения или порядок сканов | #20 |
| OEM-справка/эталон | Документальная таблица пяти команд, 16-bit manual-derived reference vectors и edge cases | Изолированное Rust-ядро **не подключено** к реальному CIOPRG и Tauri IPC; стенд не проверен | #21 |
| Сборка и правовая база | GNU GPL-3.0-only, `package-lock.json`, `Cargo.lock`, `npm ci`, Rust/TS CI, Windows releases | Юридический аудит third-party/SBOM и подписывание EXE не завершены | #11–#13, release workflow |
| JSON persistence | Staged write/backup, bounded JSON reads/writes | Backup+target не атомарная транзакция; ручной Windows smoke отсутствует | #24 |
| CSP и контроль путей | Production/dev CSP, symlink ancestor/traversal rejection | CSP runtime не подтверждён ручным smoke; не полная защита от гонок | #25 |
| Guarded native/JSON reads | File identity/reparse checks, размеры 2/4/16 МиБ; JSON конфликт при внешнем изменении | Не исключены последние TOCTOU-окна, hardlinks, ACL, power loss | #26 |

Разделы `PART 1/2` — метки структуры, **не** подтверждённая политика доступа к System/User. Весь native исходник доступен постранично; визуальное представление ограничено поддерживаемой семантикой и лимитами безопасного preview.

## 3. Релизы и источники проверок

- **Последний:** [rc.9 OEM Reference](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference) — Windows x64 portable ZIP, NSIS EXE, GPL source archive, `SHA256SUMS.txt`. Pipeline [SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44162078155).
- **Предыдущие:** [rc.8 Static Evidence](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.8-evidence), [rc.7 Typed CIO](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.7-typed), [rc.6 Post-END](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.6-postend), [rc.5 Compact Workspace](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.5-layout). История не переписывается при изменениях `main`.
- Версия `0.5.0-rc.9` — **инженерная предварительная**, не аппаратная сертификация.
- `npm ci` проверяет точность npm lockfile; `cargo test --locked` — Rust lockfile. Windows runner проверяет MSVC и формирует артефакты; **ручной smoke test на пользовательской Windows и проверка на роботе остаются отдельными gates**.

## 4. Открытые ограничения / риски

1. **Аппаратная проверка:** нет подтверждённых измерений для указанных реальных контроллеров/firmware; приватные файлы нескольких роботов анализировались, но не опубликованы. Статус `ManualDerived` ≠ `BenchVerified`.
2. **Семантика пяти команд:** документированы основные формулы и правила Carry/Zero/Error; MOV→byte при значении 256, Carry при SHL/SHR n=0/16, конфликты регистров, EN/скан и перенос правил между DX100/DX200/YRC1000 требуют независимых тестов.
3. **Native CIO grammar:** неизвестные инструкции/секции сохраняются без потерь, но полный OEM-синтаксис, System/User ownership и безопасная сериализация **не подтверждены**.
4. **IONAME:** парсер основан на наблюдаемой структуре 4-slot, а карта индексов на CIO-адреса и OEM-совместимый writer **не подтверждены**; комментарии хранятся локально и могут пропасть при очистке профиля.
5. **Безопасность Tauri:** HSES/FTP отключены; CSP добавлена PR #25, guarded reads и JSON lost-update checks добавлены PR #26. Остаются window `check→rename`, directory handle/ACL, hardlinks/reparse edge cases, общая атомарность backup+target и реальный пользовательский Windows/WebView2 smoke. Отдельный [Windows CI PR #26 успешен](https://github.com/uzbatsu-beep/MLAD/actions/runs/44227145031), но это не security-аудит.
6. **Лицензирование:** GPL-3.0-only установлен; third-party notices/SBOM и происхождение всех материалов требуют отдельного аудита.
7. **UX:** source paging реализован; не каждая нативная команда имеет диаграмму, долгосрочный portable sidecar для комментариев не завершён.

## 5. Дорожная карта — приоритеты

| Веха | Прогресс на дату сверки |
| --- | --- |
| M0: PR-база и CI | Выполнено: #1–#26 merged, main CI SUCCESS на SHA `2092db18` |
| M1: приватные OEM golden fixtures | Частично: 5 исходников исследованы, нет полной модели controller/firmware/OEM outcome |
| M2: hardening/воспроизводимость | Частично: PR #24–#26 merged (JSON staging, CSP, guarded reads/conflict checks); Windows CI успешен, но ручной Windows smoke, полная TOCTOU и backup transaction открыты |
| M3: Native Workspace | Основной read-only workflow реализован (#15–#17) |
| M4: навигация и диагностика | Реализована база; расширенные фильтры/cross-ref/performance остаются |
| M5: Typed Syntax | Частично реализовано для 5 команд, другой синтаксис остаётся opaque |
| M6: Semantic/Dataflow | Частично: Boolean AST, static M-ссылки и отдельные reference vectors; scan/value flow не проверен |
| M7–M8: общий инженерный проект | Не завершено; JSON drafts существуют отдельно, comments local-only |
| M9: симулятор по проверенной модели | Не готов; изолированные offline-тесты ≠ сканирующий симулятор |
| M10: OEM стенд | Не выполнено; нет доступа к роботам |
| M11: HSES read-only | Не разрешено |
| M12: native writer | Заблокировано |
| M13: deployment | Заблокировано |

Последующие шаги должны сохранять **отдельные статусы**: implementation, source evidence, CI, Windows smoke, bench verification и authorization. Подробности: [roadmap.md](roadmap.md) и [матрица семантики](semantic_verification_matrix.md).

## 6. Навигация

- [README репозитория](../README.md) — назначение, быстрый старт, ограничения.
- [Каталог документации](README.md) — действующие и исторические документы.
- [Архитектура и доверие](architecture.md), [исследование пяти команд](oem_5_opcode_analysis.md), [исходники и доказательность](source_registry.md).
- [Релиз rc.9](releases/v0.5.0-rc.9-oem-reference.md) — исторические сведения именно об этом релизе.
- [Процесс CI / Release](development_workflow.md), [лицензия](licensing.md).


## 7. GitHub Community Standards (PR #23)

Для участия и безопасного сообщения об ошибках уже слиты в `main` (PR #23) [CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.md), [SECURITY.md](../SECURITY.md), [ACCESSIBILITY.md](../ACCESSIBILITY.md), три [Issue forms](../.github/ISSUE_TEMPLATE) и [шаблон PR](../.github/PULL_REQUEST_TEMPLATE.md). Это **правила подачи и ревью**, не изменение parser/runtime, границ OFFLINE или статуса OEM-верификации.

Публичные Issues не предназначены для реальных `CIOPRG.LST`, `IONAME.DAT`, закрытых карт памяти/сети и деталей уязвимостей. Включение GitHub Private Vulnerability Reporting зависит от настроек репозитория и проверяется владельцем отдельно. Публикация SECURITY.md **не включает** этот механизм автоматически.


## 8. Статус PR #24 — сохранение JSON (merged, ещё не в отдельном Windows-релизе)

Merged PR #24 реализовал единый модуль `draft_io.rs` для staged JSON write/backup, проверки ограничений размера 2 МиБ при чтении/записи и отказов при прямых symlink. Функция уже находится в `main`, но **не** выпущена отдельным Windows binary rc.9 и не закрывает M2. После merge надо отдельно проверить Windows и продолжить CSP/TOCTOU; нативные роботовые файлы и сетевое управление остаются вне области изменений. [Контракт и пределы](json_persistence_safety.md).


## 9. PR #25 — CSP и проверки путей (merged)

CSP настроена для production/dev WebView, входные пути проверяются на известные symlink/traversal. Результат включён в `main`, но **не** в опубликованный rc.9. Ручной WebView2 smoke и security penetration-test не проведены. [Контракт](csp_path_hardening.md).

## 10. PR #26 — Guarded File Snapshots / JSON Conflict Guard (merged)

Native CIOPRG/IONAME и JSON открываются с проверкой identity/reparse и ограничением длины; при внешнем редактировании JSON используется fail-closed conflict detection. [Linux CI для main](https://github.com/uzbatsu-beep/MLAD/actions/runs/44228204776) и [Windows MSVC/Tauri проверки PR #26](https://github.com/uzbatsu-beep/MLAD/actions/runs/44227145031) успешны. **Не доказаны** абсолютная атомарность backup+target, отсутствие всех file races, power-loss durability или user smoke. [Threat review](guarded_io_review.md).

## 11. Управление следующими этапами

[Исполнимый план PR #27+](implementation_plan.md) · [матрица доказательности/приёмки](verification_gates.md) · [реестр решений D1–D9](engineering_decisions.md). Без отдельного согласия владельца не сливать будущие PR, не публиковать новые релизы и не включать HSES/native writer/deployment.

## 12. PR #28 — Windows Acceptance & Security Regression (Draft, не в main)

Отдельно разрабатываются [Windows workflow](../.github/workflows/windows-security-acceptance.yml), NTFS exclusive-lock/junction Rust tests, синтетический SHA-256 harness и [протокол Windows UserSmoke](windows_acceptance_pr28.md). На данный момент **не считать их частью текущего main**, пока PR не слит. После выполнения GitHub Actions отдельно фиксировать WindowsCI PASS/FAIL, а WebView2 UserSmoke — только по ручному тесту. Native writer, HSES/FTP, live I/O и аппаратный upload по-прежнему заблокированы.
