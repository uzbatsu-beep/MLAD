# MLAD — актуальное состояние проекта

**Дата сверки:** 18 ноября 2026 года.  
**Проверенный baseline:** GitHub `main` @ `6b8434376affcc02bf6d651cbf8e0edd797ca430`.  
**Последний слитый PR:** [#24](https://github.com/uzbatsu-beep/MLAD/pull/24), все PR #1–#24 merged. PR #25 — Draft.  
**Основной CI:** [MLAD validation — SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44221610901) для указанного `main`.  
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
5. **Безопасность Tauri:** сетевой runtime HSES/FTP исключён из сборки; команды включения/загрузки отказывают. Остаются: review `csp: null`, filesystem symlink/race/TOCTOU, transactional backup, Windows smoke и защита профиля. PR #24 предлагает staging/rename и bounded JSON I/O, но **не** полное устранение гонок.
6. **Лицензирование:** GPL-3.0-only установлен; third-party notices/SBOM и происхождение всех материалов требуют отдельного аудита.
7. **UX:** source paging реализован; не каждая нативная команда имеет диаграмму, долгосрочный portable sidecar для комментариев не завершён.

## 5. Дорожная карта — приоритеты

| Веха | Прогресс на дату сверки |
| --- | --- |
| M0: PR-база и CI | Выполнено: #1–#21 merged, main CI SUCCESS |
| M1: приватные OEM golden fixtures | Частично: 5 исходников исследованы, нет полной модели controller/firmware/OEM outcome |
| M2: hardening/воспроизводимость | Частично: PR #24 merged — staging JSON + `.bak`, Windows MSVC tests; PR #25 Draft — CSP + symlink ancestor guards; TOCTOU и ручной smoke остаются |
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


## 9. PR #25 — CSP и проверки ссылок в пути (проектируемый, не в main)

Для Tauri предлагается строгая production CSP и отдельная devCsp для localhost/Vite. При чтении native данных и чтении/записи JSON выполняется предварительная проверка каждого родительского каталога на известные symlink. Это **defense-in-depth**, не истинный защитный `openat/no-follow` на дескрипторе и не гарантия от TOCTOU. Стендовая верификация и robot writer по-прежнему недоступны. [Контракт PR #25](csp_path_hardening.md).
