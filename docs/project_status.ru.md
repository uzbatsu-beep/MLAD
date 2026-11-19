<!-- Generated ru companion for docs/project_status.md; review required. -->
> **Русский** · [Русский](project_status.ru.md) · [English](project_status.en.md) · [Original / Оригинал](project_status.md)
> **Translation review / Проверка:** автоматический черновик — сверяйте инженерные утверждения с оригиналом и OEM документацией.
> **Контрольные ограничения MLAD:** `CIOPRG.LST` и `IONAME.DAT` — строго **только чтение**. FTP/HSES, live I/O, native writer и загрузка в контроллер — **BLOCKED (запрещены)**. `WindowsCI` не заменяет `UserSmoke=NOT RUN` и `OEMBench=NOT RUN`; машинный перевод должен быть сверен с исходным документом и документацией Yaskawa.

# MLAD — фактическое состояние проекта

**Сверка:** 18 ноября 2026. **Baseline:** GitHub `main` @ `f85ddfcce3063dd087425be981a9d2b7a16d6309`, PR **#1–#29 слился**. [Основной успех Linux](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454) (`frontend-build`, `rust-tests`), [Основные достижения Windows Acceptance & Security Regression](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621) (`windows-security`) на указанном SHA. [PR #29](https://github.com/uzbatsu-beep/MLAD/pull/29) merged, его [Linux](https://github.com/uzbatsu-beep/MLAD/actions/runs/44369892563) и [Windows](https://github.com/uzbatsu-beep/MLAD/actions/runs/44369892566) CI также SUCCESS.

**Последний опубликованный Windows Release:** [v0.5.0-rc.9-oem-reference](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference), 17.11.2026. Release содержит опубликованные артефакты только своего tag; **PR #24–#29 в него не входят**. CI build после PR #29 формирует временный **unsigned non-release** EXE, но это не новый GitHub Release. Версия в манифестах приложения пока `0.5.0`; пользовательский Windows/WebView2 `UserSmoke=NOT RUN`OEM-стенд `NOT RUN`.

> **Область:** этот документ описывает доказанный GitHub baseline, а не установленную на компьютере пользователя версию. Статус изменится после новых merge или release; не приписывать новый код старым release assets. Подробности: [дорожная карта v3](roadmap.ru.md), [план PR #30+](implementation_plan.ru.md), [Проверочные ворота](verification_gates.ru.md).

## 1. Продуктовая граница

MLAD — **OFFLINE/BENCH** инженерный прототип для анализа Yaskawa Concurrent I/O (`DX100`, `DX200`, `YRC1000`). Предназначен для просмотра экспортированного `CIOPRG.LST` в основном рабочем окне, source/ladder/advanced диагностики и работы с **отдельными** Graph/Advanced/legacy JSON проектами. Оригинальные `CIOPRG.LST` и `IONAME.DAT` строго read-only; ни native writer, ни проверенного аппаратного scan emulator, ни FTP/HSES runtime нет. Исторические прототипные net-материалы не считаются доступной функциональностью. `PART 1/2` — labels, не подтверждённая модель System/User rights.

## 2. Возможности, реально присутствующие в main

| Направление | Что есть | Важная граница | Основные PR |
| --- | --- | --- | --- |
| Типизация адресов | Контрольно-семейные релейные классы, M-регистрации, типизированные синтаксические проверки | Не верифицированы firmware-specific разрешения на запись | #1, #19 |
| Инспекция без потерь | Точные байты, SHA-256, пролеты источника, BOM / сохранение линии, непрозрачный хвост | Не full OEM grammar/compiler | #2, #14, #18 |
| Рабочая область Native Workspace | Системный Open, дерево, source paging/search, диаграммы распознанных rungs, компактные блоки | Визуально отображается только доказанное поддерживаемое подмножество | #15–#18 |
| `IONAME.DAT` | Таблица / предварительный просмотр + локальные аннотации / ручные ассоциации | Нет доказанной автоматической OEM mapping | #16 |
| Редактор Graph JSON | `*.graph.json`, проверка ввода, диаграмма/Undo/Redo, сохранение/загрузка | Не CIOPRG compiler | #4, #7–#8 |
| Расширенный JSON | Карты TMR/CNT/GSTR/GOUT `*.advanced.json`Изолированные помощники тестовой модели | Не исполнение контроллера | #5, #8 |
| Типовые операции | MOV/ADD/SUB/SHL/SHR candidates и evidence-derived reference vectors | Не полный EN/scan/dataflow emulator | #19–#21 |
| Статический анализ | Boolean AST, сгруппированные diagnostics, static M-links | Гипотезы, не OEM confirmed dataflow | #3, #18–#21 |
| Windows сборка | GitHub Actions, Tauri x64 Unsigned EXE, временные доказательства SHA | CI ≠ ручное приложение WebView2 | #6, #28 |
| упорство JSON | Та же постановка, `*.bak`Защищенные чтения, детектировать байт + Своп с идентификатором файла, отказ от жестких ссылок, постановочная проверка SHA | Final check→rename, ACL и backup transaction остаются открыты | #24–#26, #29 |
| Tauri CSP / файловая система | Производственный CSP, без удаленного скрипта/eval; Windows reparse/ADS/UNC/device paths denied; нативный источник только для чтения | CSP runtime/manual tests и absolute filesystem TOCTOU не доказаны | #25–#26, #29 |
| Git/legal/community | GPL-3.0-only, `package-lock.json` + `npm ci`, `Cargo.lock`Тесты, безопасность/контролирование | SBOM/notices/legal review и Ruleset остаются | #11–#12, #23, #27 |

## 3. Независимые доказательные статусы

| Gate | Доказательство | Статус |
| --- | --- | --- |
| `Implementation` | Код и unit/regression tests в main #1–#29 | **PARTIAL** относительно полной IDE |
| `SyntheticTested` | Rust/Node/TS suites, в том числе Windows junction/lock/File ID/hardlink/ADS | **PASS** для соответствующих проверенных сценариев |
| `WindowsCI` | [Основной рабочий процесс Windows](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621) на `f85ddfc` | **Пасс** |
| `UserSmoke` | Ручной Windows 10/11 WebView2 Протокол WIN-01...WIN-12 | **NOT RUN** (нет подтверждённого отчёта) |
| `ManualDerived` | Подмножество MOV/ADD/SUB/SHL/SHR | **PARTIAL**; независимый scan не реализован |
| `OEMBenchVerified` | DX100/DX200/YRC1000 контроллер/firmware/записи результата | **NOT RUN (не проводилось)** |
| `Authorized native writer` | Нет протокола разрешения или полной OEM грамматики | **BLOCKED (заблокировано)** |
| `Authorized FTP/HSES/deployment` | Отдельного OEM/safety/owner допуска нет | **BLOCKED (заблокировано)** |

Ни одна зелёная GitHub Actions проверка не отменяет `NOT RUN` или `BLOCKED` на других уровнях. Manual Windows test находится в [Принятие Windows](windows_acceptance_pr28.ru.md), а стендовые испытания — в [аппаратная матрица](advanced_hardware_test_matrix.ru.md).

## 4. Открытые вопросы и риски

1. **Native parser:** неизвестные/неподдерживаемые команды, варианты `//$M`, `PART/END`, post-END sections и ряд форматов; необходим synthetic corpus и независимое private fixture provenance. Пока нет проверенной полной Native CIO grammar.
2. **UX:** source navigation реализована на базовом уровне, но cross-links source↔rung/block/diagnostics, большая таблица неизвестных операций и portable project workflow требуют следующего PR.
3. **Семантика:** typed candidates и OEM-reference не означают фактические EN/scan flags, live I/O или timing. Инструкции TMR/CNT/GSTR/GOUT и операции MOV/ADD/SUB/SHL/SHR в связке требуют отдельной OEM проверки.
4. **IONAME:** локальные annotations могут исчезнуть при сбросе профиля; подтверждённой OEM map и portable sidecar нет.
5. **Файловая система:** PR #29 отклоняет распознаваемые hardlinks/ADS/UNC/device, обнаруживает swap по File ID и staged corruption. **Не гарантированы** atomic backup+target, ACL/owner persistence, no-race parent directory, last check→rename и crash durability. [Точный threat review](filesystem_security_pr29.ru.md).
6. **Безопасность supply chain:** `suppaftp` по-прежнему присутствует в `src-tauri/Cargo.toml` как зависимость, хотя контроллерный runtime network module не компилируется и команда deployment deny-only. Нужен самостоятельный dependency cleanup/review, без добавления сетевого функционала.
7. **Windows:** нет подтверждённого ручного smoke на пользовательском WebView2, portable/installer нового `main` релизом не публиковались, бинарник unsigned.
8. **Право/управление:** GPL-3.0-only и авторство оригинальных материалов зафиксированы; проверка third-party notices/SBOM и license provenance не завершена. `main` на момент сверки `protected=false` — Ruleset/Branch Protection отдельно настраивает владелец.

## 5. Ход этапов M0–M13

| Этап | Статус | Что дальше |
| --- | --- | --- |
| M0 — базовая инфраструктура | **DONE: ** #1-#29 | Ревизия документации PR #30Обсуждение GitHub Ruleset |
| M1 — реальные fixtures | ** Частично** | Синтетический корпус, частное происхождение OEM |
| M2 — безопасность | ** Частично** | Windows UserSmoke, остаточный обзор файловой системы / ACL |
| M3 — рабочая область Native Workspace | **Только для чтения** | Перейти к M4 UX |
| M4 - исходная навигация | ** Частично** | Cross-navigation/large-file/filter PR #32 |
| M5 — типизированный парсер | ** Частично** | Доказательная грамматика PR #34 |
| M6 — статическая семантика | ** Частично** | Исходно-индексированный поток данных PR #35 |
| M7–M8 – единый офлайн-проект | ** Открыто** | Sidecar #33 + интегрированный UX #36 |
| M9 — сканирование | **Открыт/не проверен** | Только после OEM scan evidence |
| M10 - OEM-стенд | **NOT RUN (не проводилось)** | Изолированный допущенный стенд |
| M11 - только для чтения HSES | **BLOCKED (заблокировано)** | Отдельное решение владельца, protocol evidence |
| M12 — родной писатель | **BLOCKED (заблокировано)** | Полное одобрение владельцев OEM+ |
| M13 — развертывание | **BLOCKED (заблокировано)** | Дополнительная процедура безопасности и разрешения |

## 6. Текущая очередь разработки

Документация **#30**, synthetic parser corpus **#31**, Native Workspace navigation **#32**, portable sidecar **#33**, evidence-based grammar **#34**, static cross-references **#35**, unified Offline IDE **#36**, условный Windows preview **#37**. Это **проектная последовательность**, а не гарантия PR numbers или сроков. Невыясненный security defect может блокировать следующий этап. [Подробный backlog](implementation_plan.ru.md).

## 7. Ссылки для текущего контроля

[Roadmap](roadmap.ru.md) · [План осуществления](implementation_plan.ru.md) · [Инженерные решения](engineering_decisions.ru.md) · [Проверочные ворота](verification_gates.ru.md) · [Обзор безопасности](filesystem_security_pr29.ru.md) · [Принятие Windows](windows_acceptance_pr28.ru.md) · [Реестр источников ОЕМ](source_registry.ru.md) · [Licensing](licensing.ru.md).

Исторические release notes rc.1–rc.9, [Дорожная карта v2 архив](roadmap_v2_archive.ru.md), [Старый план](implementation_plan_v2_archive.ru.md), старые PR-specific evidence и [аудит от 17.11](documentation_audit_2026-11-17.ru.md) **не переписываются как текущий статус**.
