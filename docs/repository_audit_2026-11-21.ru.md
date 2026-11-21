<!-- MLAD-I18N:EDITORIAL:source-aligned 2026-11-21 against docs/repository_audit_2026-11-21.md -->
> **Язык / Language:** [Русский](repository_audit_2026-11-21.ru.md) · [English](repository_audit_2026-11-21.en.md) · [Original / Оригинал](repository_audit_2026-11-21.md)
> **Provenance:** Russian source-aligned, relative links preserve language navigation.

# MLAD — аудит Git и Markdown от 21.11.2026

**Основание:** [main](https://github.com/uzbatsu-beep/MLAD) = `9fcd11d61856c8a559d9acf3c77bd662ad814dfe`; [основной CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44743083267) = SUCCESS. Релиз [`v0.5.0-rc.10-unified-offline`](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.10-unified-offline) опубликован из **другого**, релизного коммита `2bba435f2263f2300cb3464b76f919c0da2e4b7f`, [Windows release workflow](https://github.com/uzbatsu-beep/MLAD/actions/runs/44754693972) = SUCCESS. Это два разных Git SHA, а не одна сборка. Аудит не меняет историю старых тегов.

**Метод:** изучены Git tree, активные workflows, Ruleset `main`, release assets, Cargo/npm manifest/lock policy, матрица локализации, корневой и технические Markdown. На момент чтения `main` содержал 294 элемента дерева и **164 Markdown-файла**: 54 оригинала, 108 RU/EN companions, один реестр переводов и отдельный архивный `.github/workflow-archive/README.md`. С настоящим отчётом добавляются новый оригинал + RU/EN (итого **167 Markdown**), но результаты CI для PR с новыми файлами требуют отдельного доказательства на его окончательном SHA.

## 1. Актуальная продуктовая конфигурация

| Область | Подтверждено в коде или GitHub | Не доказано |
| --- | --- | --- |
| Native I/O | `CIOPRG.LST`, `IONAME.DAT`: ограниченный read-only парсер с исходными байтами и SHA-256 | OEM-выполнение программы, право записи |
| Native Workspace | Source, diagrams, диагностика, IONAME, навигация, источник ролей `READ` / `WRITE` / `UNKNOWN` | Live I/O, истинные EN/scan/dataflow |
| Sidecar | Отдельный `*.mlad.json` v1: комментарии, закладки, source SHA, сохранение с ревизией | OEM-комментарии; полноценная транзакция нескольких файлов |
| Unified offline project | `*.mladproj.json` v1: относительные пути к CIO/IONAME/sidecar, SHA-256, пересчёт анализа | Совместимость с контроллерным native writer |
| Typed grammar | Разбор документированных форм YRC1000 R12, в том числе расширенные 15 команд | Права System/User, разрешение output write на реальном роботе |
| Windows | `rc.10` portable ZIP, unsigned NSIS EXE, GPL source ZIP, SHA-256; автоматическая Windows-сборка успешна | **`UserSmoke=NOT RUN`**, **`OEMBench=NOT RUN`** |
| Safety | `FTP`, `HSES`, native writer, live robot commands, deployment **BLOCKED** | Аппаратная приемка и сертификация |

Синтетический набор PR #48 подтверждает только работу fixture pipeline. **Нельзя** загружать тестовый `CIOPRG.LST` на контроллер. Права записи не возникают из `TypedCandidate`, `ManualDerived` или `READ`/`WRITE` ролей.

## 2. Результаты аудита и приоритеты

| Приоритет | Обнаружено | Действие / статус |
| --- | --- | --- |
| **P0 — Governance** | Активный Ruleset `MLAD — main branch protection` требует PR, запрещает удаление/force push, но обязательный status check — **только `frontend-build`**. `ci-gate` не указан; `strict_required_status_checks_policy=false` | **OPEN:** отдельно согласовать обязательный `ci-gate`, включающий Rust+Windows; не изменять настройки без владельца |
| **P1 — Docs drift** | Живые README и центральные документы называли `rc.9` и PR #29/#39 текущим состоянием, хотя слиты PR #40–#48 и есть `rc.10` | **IN PR #49:** добавить явно датированный текущий слой, связать актуальные документы; старую последовательность PR не выдавать за сегодняшнюю |
| **P1 — Personal data hygiene** | В `.gitignore` были исключены нативные `CIOPRG.LST` и `IONAME.DAT`, но не собственные `*.mlad.json` и `*.mladproj.json` | **IN PR #49:** игнорировать пользовательские manifests, sidecars и backup. Файлы, уже добавленные в Git, не удаляются одним `.gitignore` |
| **P1 — Release evidence** | `rc.10` в отдельном Git tag, а сам `main` содержит старый заголовок окна `rc.9`; Windows Preview workflow PR #48 оставался активным после merge | **IN PR #49:** нейтральный заголовок, архивирование завершённого workflow с сохранением источника/контрактов |
| **P1 — Documentation QA** | `docs:check` сканировал не все Markdown: игнорировал RU/EN в корне и `.github/workflow-archive/README.md`; fences `~~~` не исключались | **IN PR #49:** включить все Markdown repo/docs/.github и оба типа fenced code |
| **P2 — Dependency hygiene** | Датированные старые отчёты сообщали о `suppaftp`. В актуальном `src-tauri/Cargo.toml` этой зависимости **нет** (удалена ранее) | **CLOSED:** не дублировать PR #43, старый отчёт оставить историческим. Перепроверка CVE/лицензий всех прямых и транзитивных зависимостей — отдельная задача |
| **P2 — i18n** | Источники имеют RU/EN companions, но значительная часть помечена `MachineDraft`; это не равно проверенному техническому переводу | **OPEN:** редакционная проверка терминов, ограничений безопасности и соответствия источникам; новая работа не повышает качество прежних переводов автоматически |
| **P2 — Filesystem** | Сохранение JSON staged + `*.bak`, SHA, File ID guards; есть ограничения Windows DACL, TOCTOU, многофайловой атомарности | **OPEN:** OS-level threat review, crash recovery, cross-volume и race tests |
| **P2 — Testing** | Linux/Rust/Windows CI и production NSIS успешны; нет записанного интерактивного Windows протокола по P48-01…P48-17 | **OPEN:** провести `UserSmoke`; OEMBench отдельно и без расширения native полномочий |

**Гигиена Git-веток и воспроизводимость CI (дополнительная проверка):** на момент аудита было **39 веток**, включая защищённую `main` и релизную `release/v0.5.0-rc.10-unified-offline`; открыт один PR #49 (Draft). Удаление веток **не выполнялось**: `.github/workflows/merged-branches-prune.yml` имеет отдельный режим `dry-run` и требует явного подтверждения для `delete`. Также часть GitHub Actions использует плавающие `@v4`, `@stable` или теги зависимостей; `npm ci` и Cargo `--locked` не гарантируют побайтово воспроизводимый EXE при изменении OS runner/toolchain. Предлагается отдельный PR на SHA-пиннинг actions и зафиксированный toolchain после анализа совместимости.

## 3. Структура документации и владение содержимым

- **Живые документы:** [README](../README.ru.md), [каталог](README.ru.md), [roadmap](roadmap.ru.md), [implementation plan](implementation_plan.ru.md), [project status](project_status.ru.md), [verification gates](verification_gates.ru.md), [architecture](architecture.ru.md), [development workflow](development_workflow.ru.md), [engineering decisions](engineering_decisions.ru.md). Они отражают текущий status layer и ссылаются на это расследование.
- **Технические спецификации:** `cioprg_lst_spec`, `typed_opcode_grammar`, `semantic_ast`, `json_persistence_safety`, `ioname_annotations`, `post_end_data`, `source_registry` и остальные active docs — договорные границы поведения; отсутствие нового OEM-испытания не превращается в факт OEM compatibility.
- **Исторические материалы:** датированные старые audits/inventory, `roadmap_v2_archive`, `implementation_plan_v2_archive`, release notes `rc.1`–`rc.9` и workflow-archive — неизменяемые свидетельства прошлых проверок. Старые заявления верны **только для указанного baseline**.
- **Локализация:** [реестр переводов](translation_coverage.md) фиксирует оценку каждого source + RU + EN, а `scripts/i18n_verify.py` и `scripts/i18n_semantic_guard.py` блокируют нарушение структуры/критичных оговорок. `MachineDraft` означает не прошедший независимую техническую редактуру материал, не готовую официальную инструкцию.

## 4. Следующие решения владельца

1. Согласовать изменения Ruleset: требовать `ci-gate` на `main` и рассмотреть строгую проверку актуальности ветки перед merge. Это настройка GitHub; текстовый PR её **не** изменяет.
2. Выполнить UserSmoke по [матрице PR #48](windows_offline_preview_pr48.ru.md) на отдельном Windows 10/11 + WebView2, зафиксировать `P48-01`…`P48-17` и версии ПО; не маркировать `UserSmoke=PASS` только за CI.
3. Разделить последующие PR по слоям: security/ACL/TOCTOU и test fixtures; UX; зависимости/лицензии (с `npm audit`/`cargo audit` только при обоснованной настройке), затем расширение verified grammar с OEM evidence.
4. При необходимости провести отдельное пофайловое техническое редактирование 47 неподтверждённых языковых черновиков; не заменять их автоматически и не повышать оценку качества без ревью.

## 5. Evidence и границы аудита

[Main validation SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44743083267) · [RC10 Windows workflow SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44754693972) · [Release RC10](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.10-unified-offline) · [PR #48](https://github.com/uzbatsu-beep/MLAD/pull/48).

**Не выполнено:** независимое OSV/CVE сканирование всей dependency graph, запуск GUI на локальной Windows-машине, OEM bench, проверка backend в производственной сети, широкие файловые race-тесты. Не утверждать обратное. Весь отчёт — аудит исходного состояния и документационных/защитных изменений, **не новая аттестация приложения**. Окончательную успешность PR #49 подтверждать только по его собственному SHA и CI.
