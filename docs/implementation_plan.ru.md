<!-- MLAD-I18N:EDITORIAL:restored 2026-11-19 from predominantly Russian original -->
> **Язык / Language:** [Русский](implementation_plan.ru.md) · [English](implementation_plan.en.md) · [Исходный текст / Original](implementation_plan.md)
>
> **Примечание:** русскоязычный исходник сохранён без машинного пересказа; технические имена, ограничения и команды не переводились произвольно. / Source fidelity is preserved; OEM validation is independent.

# MLAD — исполнимый план разработки после PR #29

**Базовая точка:** 18.11.2026, `main` SHA `f85ddfcce3063dd087425be981a9d2b7a16d6309`. PR #1–#29 merged. [Linux CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454), [Windows CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621). Последний [Windows Release — rc.9](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference) выпущен **до** PR #24–#29. Ручной `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`. После следующего merge обновлять конкретный baseline в [project_status.md](project_status.ru.md).

**Статус плана:** PR #30 — только документационная синхронизация; PR #31–#37 — **предложенные номера**, а не созданные PR/согласованные сроки. Каждый этап требует собственного Draft PR, CI и **явного решения владельца о merge**. Согласованные D1–D9 сохраняются без изменений. [Roadmap v3](roadmap.ru.md) · [Реестр решений](engineering_decisions.ru.md) · [Архив плана v2](implementation_plan_v2_archive.ru.md).

## 1. Приоритеты нового цикла

| Порядок | Цель | Почему сейчас |
| --- | --- | --- |
| 0 | Точность документации после #29 | Устаревшие статусы и SHA в MD затрудняют планирование |
| 1 | Синтетический parser fixture corpus | Зафиксировать observed edge cases, partial/opaque и неизменность байтов |
| 2 | Native Workspace: поиск и cross-navigation | Пользовательские проблемы с обзором большого CIOPRG и неизвестных конструкций |
| 3 | Versioned portable sidecar | Привязка заметок/закладок к SHA без изменения native файла |
| 4 | Evidence-driven typed syntax + static analysis | Разделить синтаксис, проверку ссылок и непроверенное исполнение |
| 5 | Unified Offline Engineering Project | Единый инженерный сценарий без native writer |
| 6 | Условный Windows 0.6.x preview | Только после успешных CI, ручной приёмки и правового контроля |

**Параллельные задачи:** приватный OEM fixture intake, независимый UserSmoke WIN-01–WIN-12, Windows ACL/crash/TOCTOU review, GitHub branch protection/ruleset и third-party license review. Модернизация интерфейса не должна автоматически включать HSES/FTP и native writer.

## 2. Proposed PR backlog и зависимости

| PR | Результат | Зависимости | Gate |
| --- | --- | --- | --- |
| **#30 — Docs Baseline / Roadmap v3** | Актуальные MD, единственная каноническая дорожная карта, исторические архивы и risk matrix | merged #29 | `docs:check` + стандартный CI; docs-only |
| **#31 — Synthetic CIOPRG Parser Corpus** | Синтетические варианты заголовка, `//$M`, `PART/END`, ALARM/MESSAGE, BOM/newlines, malformed/opaque, oversized | #30 | source spans/SHA-256 byte identity, Rust negative tests |
| **#32 — Native Workspace Navigation & Diagnostics** | Source↔rung↔block/diagnostic, search/filter/keyboard, large-file usability | #31 частично; возможно параллельно | Все физические строки доступны, unknown явно виден |
| **#33 — Portable `*.mlad.json` Sidecar v1** | Versioned schema, source hash/profile, bookmarks/annotations, ссылки на drafts, migrations/conflict | #30, filesystem safety #29 | Roundtrip/relocation/stale-hash/invalid JSON fail-closed |
| **#34 — Typed Opcode Grammar Extension** | Дополнить только OEM-evidence-validated native forms, exact spans и typed/opaque states | #31, подтверждённые источники | Partial ≠ executed; negative syntax tests |
| **#35 — Source-linked Static Analysis v2** | READ/WRITE index, cross-references, duplicate/conflict diagnostics и confidence | #31/#32/#34 | Ни ложных runtime/scan утверждений, ни потерянных unsupported |
| **#36 — Unified Offline Engineering Project** | Native Snapshot + Sidecar + независимые Graph/Advanced JSON drafts в общем workflow | #32/#33/#35 | Source неизменен, user data восстанавливается, safe save/undo |
| **#37 — Windows OFFLINE Preview 0.6.x RC** | Новая версия с source/portable/NSIS/SHA и release notes | только принятые функции + Windows UserSmoke + legal gate | Сборка на tag SHA и **отдельное согласование release** |

Номера #31–#37 условные, GitHub их не резервировал. Bugfix/security PR могут быть вставлены между этапами по реальным дефектам и явному согласованию.

## 3. PR #31 — Synthetic Native Parser Fixtures (P0)

**Вход:** [source registry](source_registry.ru.md), [CIOPRG baseline](cioprg_lst_spec.ru.md), [lossless contract](cioprg_lossless_parser.ru.md), [post-END](post_end_data.ru.md) и [fixture intake](controller_fixture_intake.ru.md). Приватные источники не коммитить; для публичных тестов заново составлять synthetic минимальные примеры без копирования уникальных клиентских фрагментов.

**Выход:** версионированный набор фикстур с манифестом `synthetic origin / controller family assumption / encoding / newlines / expected raw spans / supported status`. Сценарии: обычные и нестандартные заголовки, `//$M`, порядок/повторы `PART`, `END`, `ALARM/MESSAGE` после END, неопределённые mnemonics, malformed operand/адрес, BOM, CRLF/LF, большие файлы.

**Acceptance:** оригинальные байты и SHA-256 не меняются после parse/page/search; source span/physical line неизменны; unsupported не превращается в ложный rung, `PART 1/2` не трактуется как подтверждённая System/User policy; тесты Linux и нужные Windows регрессии.

## 4. PR #32 — Native Workspace UX (P0)

**Выход:** связка `source ↔ rung ↔ advanced block ↔ diagnostics` с точной physical line/offset, фильтры по opcode и address, раскрытие групп ошибок, компактные TMR/CNT/GSTR/GOUT списки, cross-link-to-source, keyboard focus и zoom. Инспектор не ограничивать только первыми 600 строками; использовать bounded pages.

**Acceptance:** синтетический файл порядка 1400+ строк полностью доступен по навигации; нет скрытых unknown; отфильтрованные элементы не исчезают из модели; не допускаются UI edits исходного `CIOPRG.LST`; данные после `END` не визуализируются как Ladder. Производительность измерить и приложить среду теста, без выдуманных метрик.

## 5. PR #33 — Portable Sidecar (P1)

**Контракт:** `*.mlad.json` — отдельный пользовательский manifest, **не** изменённый нативный код. Предложенная структура: `schema_version`, `source_sha256`, `controller_family`, optional source provenance (без секретов), bookmarks/notes/manual IONAME associations и ссылки на самостоятельные `*.graph.json`/`*.advanced.json`. Решение по schema — отдельный дизайн внутри PR с утверждением владельца; эта таблица не считается уже согласованной схемой.

**Acceptance:** UTF-8/кириллица roundtrip; несовпадение source SHA → явный `detached/stale`, не silent rebind; unknown version/invalid JSON/broken references → диагностируемый отказ; migration и backup recovery, обычные локальные Windows пути и filesystem guards #29; `CIOPRG.LST`/`IONAME.DAT` ни разу не записываются.

## 6. PR #34 — Typed Grammar (P1)

От каждого opcode требуется trace: OEM manual revision/section, семейство контроллера, точная форма операндов, синтетический positive/negative fixture, raw/typed span, `unknown` для неподдерживаемого. Исследовать по частоте ошибок реального Native Workspace, но приватные выгрузки не разглашать.

**Acceptance:** корректные typed candidates для **подтверждённых** форм, точный source mapping, неизвестная команда остаётся opaque; нет повышения `ManualDerived` до `OEMBenchVerified`, нет исполнения PLC scan и native export.

## 7. PR #35 — Static Cross-references (P1)

READ/WRITE index для `#XXXXX` и `Mxxx`, ссылки на source, анализ неоднозначных/повторных назначений, ветвей и TMR/CNT/GSTR/GOUT *с пределами доказательности*. Результат включает source span, severity, support/confidence и отдельные статусы `Unresolved`.

**Acceptance:** гарантированная неизменность исходных данных, тесты негативных и ambiguous cases, отсутствие ложного dataflow/coil для unsupported opcodes; scan semantics остаётся `NotExecuted`.

## 8. PR #36 — Unified Offline Project (P1)

Единая UI-сессия `Native Snapshot (read-only) + sidecar (user data) + editable Graph/Advanced JSON`. Переходы между адресами, цепями, блоками, исходником и заметками. Копирование verified native subset в JSON draft — только **явным** действием, не автоматический native→controller compiler.

**Acceptance:** открыть → найти → отметить → сохранить sidecar → создать/изменить draft → повторно открыть; все native SHA прежние, повреждённый/старый sidecar не маскируется, конфликты записи сообщаются, кнопок загрузки в робот нет.

## 9. PR #37 — Windows Preview 0.6.x (условный)

**Минимальные gates:** необходимые feature PR слиты с CI на tag SHA; [Windows manual acceptance WIN-01…WIN-12](windows_acceptance_pr28.ru.md) выполнена и приложена; [filesystem threat model](filesystem_security_pr29.ru.md) пересмотрена; source zip, portable zip/NSIS, GPL-3.0-only, third-party notices/SBOM review, SHA256SUMS, unsigned binary status; release note перечисляет unsupported grammar и `OEMBench NOT RUN`. Даты и release-name согласовать отдельно.

Официальное OEM bench не требуется для честно названного **OFFLINE** preview, но реальная совместимость/эквивалентность с контроллером **не** заявляется. Публиковать только после отдельного решения владельца.

## 10. Security/governance поток (открыт)

PR #29 сделал проверку File ID/inode + bytes при JSON сохранении, hardlink rejection, SHA-256 staged и запрет Windows ADS/UNC/device paths. Не решил целиком:

- `check→rename` / parent directory handle TOCTOU и adversarial multi-writer races;
- ACL/owner при staged replacement и read-only permissions;
- atomic transaction для пары `target + backup` и crash/power-loss durability;
- независимую приёмку Windows WebView2.

Эти задачи оставлять **OPEN**, а для новых sidecar/edit flows проводить доработки через отдельные P0 bugfix/security PR по необходимости. См. [filesystem_security_pr29.md](filesystem_security_pr29.ru.md). `main` при проверке GitHub имел `protected=false`; включение required checks / Ruleset — отдельное действие владельца, docs PR не меняет настройки.

## 11. Параллельный private OEM evidence intake

Проверять каждый разрешённый пример `CIOPRG.LST` по controller model/firmware, method of export, encoding, original hash, OEM Ladder Editor/version и источнику прав. Никаких настоящих файлов, паролей, адресов предприятий, production IP или fingerprint hashes в публичных PR/CI. В open-source fixtures использовать только оригинальные синтетические формы. Не переносить выводы между DX100, DX200 и YRC1000 автоматически.

## 12. Definition of Done (все будущие PR)

1. Перед началом: прочитать актуальные `main` SHA, открытые PR и [D1–D9](engineering_decisions.ru.md); описать изменённый контракт.
2. В коммитах: синтетические позитивные и негативные тесты, проверка byte fidelity, документация, обоснование unknown/opcode.
3. CI: `npm ci --no-audit --no-fund`, `npm run test:security`, `npm run test:native`, другие модельные тесты, `npm run docs:check`, `npm run build`, `cargo test --manifest-path src-tauri/Cargo.toml --locked`; Windows MSVC/Tauri при IPC/filesystem изменениях.
4. Статус `WindowsCI` отдельно от `UserSmoke` и `OEMBench`; если тест не проведён — `NOT RUN`, если направление запрещено — `BLOCKED`.
5. Проверить запреты HSES/FTP/live I/O/native writer/deployment, GDPR/производственные секреты, правовой статус исходников.
6. Не сливать PR и не публиковать release без **отдельного согласования** владельца; после merge создать новый актуальный baseline и не переписывать исторические релизные файлы.

[Единые verification gates](verification_gates.ru.md) · [Workflow](development_workflow.ru.md) · [История прежнего PR #27–#35 плана](implementation_plan_v2_archive.ru.md).
