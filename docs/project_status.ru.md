<!-- MLAD-I18N:EDITORIAL:restored 2026-11-19 from predominantly Russian original -->
> **Язык / Language:** [Русский](project_status.ru.md) · [English](project_status.en.md) · [Исходный текст / Original](project_status.md)
>
> **Примечание:** перевод не заменяет исходную спецификацию; русскоязычный оригинал сохранён без машинного искажения. / Technical source fidelity retained; OEM verification remains independent.

# MLAD — фактическое состояние проекта

**Сверка:** 18 ноября 2026. **Baseline:** GitHub `main` @ `f85ddfcce3063dd087425be981a9d2b7a16d6309`, PR **#1–#29 merged**. [Main Linux validation SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454) (`frontend-build`, `rust-tests`), [Main Windows Acceptance & Security Regression SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621) (`windows-security`) на указанном SHA. [PR #29](https://github.com/uzbatsu-beep/MLAD/pull/29) merged, его [Linux](https://github.com/uzbatsu-beep/MLAD/actions/runs/44369892563) и [Windows](https://github.com/uzbatsu-beep/MLAD/actions/runs/44369892566) CI также SUCCESS.

**Последний опубликованный Windows Release:** [v0.5.0-rc.9-oem-reference](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference), 17.11.2026. Release содержит опубликованные артефакты только своего tag; **PR #24–#29 в него не входят**. CI build после PR #29 формирует временный **unsigned non-release** EXE, но это не новый GitHub Release. Версия в манифестах приложения пока `0.5.0`; пользовательский Windows/WebView2 `UserSmoke=NOT RUN`, OEM bench `NOT RUN`.

> **Область:** этот документ описывает доказанный GitHub baseline, а не установленную на компьютере пользователя версию. Статус изменится после новых merge или release; не приписывать новый код старым release assets. Подробности: [roadmap v3](roadmap.ru.md), [план PR #30+](implementation_plan.ru.md), [verification gates](verification_gates.ru.md).

## 1. Продуктовая граница

MLAD — **OFFLINE/BENCH** инженерный прототип для анализа Yaskawa Concurrent I/O (`DX100`, `DX200`, `YRC1000`). Предназначен для просмотра экспортированного `CIOPRG.LST` в основном рабочем окне, source/ladder/advanced диагностики и работы с **отдельными** Graph/Advanced/legacy JSON проектами. Оригинальные `CIOPRG.LST` и `IONAME.DAT` строго read-only; ни native writer, ни проверенного аппаратного scan emulator, ни FTP/HSES runtime нет. Исторические прототипные net-материалы не считаются доступной функциональностью. `PART 1/2` — labels, не подтверждённая модель System/User rights.

## 2. Возможности, реально присутствующие в main

| Направление | Что есть | Важная граница | Основные PR |
| --- | --- | --- | --- |
| Типизация адресов | Controller-family relay classes, M-registers, typed syntax checks | Не верифицированы firmware-specific разрешения на запись | #1, #19 |
| Lossless native inspection | Bounded exact bytes, SHA-256, source spans, BOM/newline preservation, opaque tail | Не full OEM grammar/compiler | #2, #14, #18 |
| Native Workspace | Системный Open, дерево, source paging/search, диаграммы распознанных rungs, компактные блоки | Визуально отображается только доказанное поддерживаемое подмножество | #15–#18 |
| `IONAME.DAT` | Read-only table/preview + local annotations/manual associations | Нет доказанной автоматической OEM mapping | #16 |
| Graph JSON editor | `*.graph.json`, input validation, diagram/Undo/Redo, save/load | Не CIOPRG compiler | #4, #7–#8 |
| Advanced JSON | TMR/CNT/GSTR/GOUT cards, `*.advanced.json`, isolated test-model helpers | Не исполнение контроллера | #5, #8 |
| Typed operations | MOV/ADD/SUB/SHL/SHR candidates и evidence-derived reference vectors | Не полный EN/scan/dataflow emulator | #19–#21 |
| Static analysis | Boolean AST, сгруппированные diagnostics, static M-links | Гипотезы, не OEM confirmed dataflow | #3, #18–#21 |
| Windows build | GitHub Actions, Tauri x64 unsigned EXE, temporary SHA evidence | CI ≠ ручное приложение WebView2 | #6, #28 |
| JSON persistence | Same-directory staging, `*.bak`, guarded reads, detect byte+File ID swap, reject hardlinks, staged SHA check | Final check→rename, ACL и backup transaction остаются открыты | #24–#26, #29 |
| Tauri CSP/filesystem | Production CSP, no remote script/eval; Windows reparse/ADS/UNC/device paths denied; native source read-only | CSP runtime/manual tests и absolute filesystem TOCTOU не доказаны | #25–#26, #29 |
| Git/legal/community | GPL-3.0-only, `package-lock.json` + `npm ci`, `Cargo.lock`, tests, SECURITY/CONTRIBUTING | SBOM/notices/legal review и Ruleset остаются | #11–#12, #23, #27 |

## 3. Независимые доказательные статусы

| Gate | Доказательство | Статус |
| --- | --- | --- |
| `Implementation` | Код и unit/regression tests в main #1–#29 | **PARTIAL** относительно полной IDE |
| `SyntheticTested` | Rust/Node/TS suites, в том числе Windows junction/lock/File ID/hardlink/ADS | **PASS** для соответствующих проверенных сценариев |
| `WindowsCI` | [Main Windows workflow](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621) на `f85ddfc` | **PASS** |
| `UserSmoke` | Ручной Windows 10/11 WebView2 protocol WIN-01…WIN-12 | **NOT RUN** (нет подтверждённого отчёта) |
| `ManualDerived` | OEM-reference subset MOV/ADD/SUB/SHL/SHR | **PARTIAL**; независимый scan не реализован |
| `OEMBenchVerified` | DX100/DX200/YRC1000 контроллер/firmware/записи результата | **NOT RUN** |
| `Authorized native writer` | Нет протокола разрешения или полной OEM грамматики | **BLOCKED** |
| `Authorized FTP/HSES/deployment` | Отдельного OEM/safety/owner допуска нет | **BLOCKED** |

Ни одна зелёная GitHub Actions проверка не отменяет `NOT RUN` или `BLOCKED` на других уровнях. Manual Windows test находится в [Windows acceptance](windows_acceptance_pr28.ru.md), а стендовые испытания — в [hardware matrix](advanced_hardware_test_matrix.ru.md).

## 4. Открытые вопросы и риски

1. **Native parser:** неизвестные/неподдерживаемые команды, варианты `//$M`, `PART/END`, post-END sections и ряд форматов; необходим synthetic corpus и независимое private fixture provenance. Пока нет проверенной полной Native CIO grammar.
2. **UX:** source navigation реализована на базовом уровне, но cross-links source↔rung/block/diagnostics, большая таблица неизвестных операций и portable project workflow требуют следующего PR.
3. **Семантика:** typed candidates и OEM-reference не означают фактические EN/scan flags, live I/O или timing. Инструкции TMR/CNT/GSTR/GOUT и операции MOV/ADD/SUB/SHL/SHR в связке требуют отдельной OEM проверки.
4. **IONAME:** локальные annotations могут исчезнуть при сбросе профиля; подтверждённой OEM map и portable sidecar нет.
5. **Filesystem:** PR #29 отклоняет распознаваемые hardlinks/ADS/UNC/device, обнаруживает swap по File ID и staged corruption. **Не гарантированы** atomic backup+target, ACL/owner persistence, no-race parent directory, last check→rename и crash durability. [Точный threat review](filesystem_security_pr29.ru.md).
6. **Безопасность supply chain:** `suppaftp` по-прежнему присутствует в `src-tauri/Cargo.toml` как зависимость, хотя контроллерный runtime network module не компилируется и команда deployment deny-only. Нужен самостоятельный dependency cleanup/review, без добавления сетевого функционала.
7. **Windows:** нет подтверждённого ручного smoke на пользовательском WebView2, portable/installer нового `main` релизом не публиковались, бинарник unsigned.
8. **Право/управление:** GPL-3.0-only и авторство оригинальных материалов зафиксированы; проверка third-party notices/SBOM и license provenance не завершена. `main` на момент сверки `protected=false` — Ruleset/Branch Protection отдельно настраивает владелец.

## 5. Ход этапов M0–M13

| Этап | Статус | Что дальше |
| --- | --- | --- |
| M0 — базовая инфраструктура | **DONE:** #1–#29 | Ревизия документации PR #30, GitHub Ruleset |
| M1 — реальные fixtures | **PARTIAL** | Synthetic corpus, private OEM provenance |
| M2 — безопасность | **PARTIAL** | Windows UserSmoke, residual filesystem review/ACL |
| M3 — native workspace | **DONE read-only base** | Перейти к M4 UX |
| M4 — source navigation | **PARTIAL** | Cross-navigation/large-file/filter PR #32 |
| M5 — typed parser | **PARTIAL** | Evidence-driven grammar PR #34 |
| M6 — static semantics | **PARTIAL** | Source-indexed dataflow PR #35 |
| M7–M8 — unified offline project | **OPEN** | Sidecar #33 + integrated UX #36 |
| M9 — scan simulation | **OPEN / NOT VERIFIED** | Только после OEM scan evidence |
| M10 — OEM bench | **NOT RUN** | Изолированный допущенный стенд |
| M11 — read-only HSES | **BLOCKED** | Отдельное решение владельца, protocol evidence |
| M12 — native writer | **BLOCKED** | Full round-trip, OEM+owner approval |
| M13 — deployment | **BLOCKED** | Дополнительная процедура безопасности и разрешения |

## 6. Текущая очередь разработки

Документация **#30**, synthetic parser corpus **#31**, Native Workspace navigation **#32**, portable sidecar **#33**, evidence-based grammar **#34**, static cross-references **#35**, unified Offline IDE **#36**, условный Windows preview **#37**. Это **проектная последовательность**, а не гарантия PR numbers или сроков. Невыясненный security defect может блокировать следующий этап. [Подробный backlog](implementation_plan.ru.md).

## 7. Ссылки для текущего контроля

[Roadmap](roadmap.ru.md) · [Implementation plan](implementation_plan.ru.md) · [Engineering decisions](engineering_decisions.ru.md) · [Verification gates](verification_gates.ru.md) · [Security review](filesystem_security_pr29.ru.md) · [Windows acceptance](windows_acceptance_pr28.ru.md) · [OEM source registry](source_registry.ru.md) · [Licensing](licensing.ru.md).

Исторические release notes rc.1–rc.9, [roadmap v2 archive](roadmap_v2_archive.ru.md), [old plan](implementation_plan_v2_archive.ru.md), старые PR-specific evidence и [аудит от 17.11](documentation_audit_2026-11-17.ru.md) **не переписываются как текущий статус**.
