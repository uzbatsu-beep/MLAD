<!-- MLAD-I18N:NAV:START -->
> 🌐 **Язык / Language:** [Русский](project_status.ru.md) · [English](project_status.en.md) · [Original / Исходный текст](project_status.md). **Machine translation / Машинный перевод: editorial review required / необходима редакторская проверка.**
<!-- MLAD-I18N:NAV:END -->

# MLAD — фактическое состояние после PR #39

**GitHub baseline:** `main` @ `066db5f8c01a9da8f5f0218f12539b305ab021e4`, состояние после слияния [PR #39](https://github.com/uzbatsu-beep/MLAD/pull/39) (19.11.2026); PR #1–#39 слиты. [CI PR #39](https://github.com/uzbatsu-beep/MLAD/actions/runs/44571515321): `scope`, `frontend-build`, `rust-tests`, `windows-security`, `ci-gate` = **SUCCESS** на HEAD PR, не отдельная аттестация merge commit.

**Последний опубликованный Windows release:** [`v0.5.0-rc.9-oem-reference`](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference) (17.11.2026); не содержит PR #24–#39. `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`.

## 1. Продукт и безопасность

MLAD — локальная OFFLINE/BENCH среда `DX100`/`DX200`/`YRC1000` для анализа `CIOPRG.LST` и `IONAME.DAT`. Оба исходных файла immutable read-only; JSON-черновики не являются контроллерными программами. `HSES`/`FTP`, native writer, live monitoring, robot deployment — **BLOCKED**. `PART 1/2` не устанавливает права System/User. `OEMBenchVerified` отсутствует.

## 2. Функции, присутствующие в main

| Компонент | Подтверждённый функционал | Ограничение |
| --- | --- | --- |
| Native source inspector | Работает: байты, SHA-256, постраничный просмотр и поиск | Не полная OEM grammar |
| Native Workspace | Встроен в главное окно, раздельный source/diagrams/blocks/IONAME | Cross-navigation/large-file UX частично |
| Boolean AST | Распознанные булевые цепи, diagnostics и локальная tri-state оценка | Не контроллерный scan |
| Typed data candidates | MOV/ADD/SUB/SHL/SHR и source-order M links | EN/flags/dataflow не подтверждены |
| JSON draft editor | Graph/Advanced/legacy JSON отдельно от native | Portable sidecar отсутствует |
| Local annotations | localStorage, IONAME read-only с ручными associations | Не гарантирован перенос/backup |
| Filesystem safety | Bounded reads, CSP, File ID/hardlinks, staged JSON+backup | ACL/TOCTOU/crash risks |
| GitHub Actions | PR39: 5 заданий SUCCESS на его HEAD | Не UserSmoke/bench; Ruleset требует только frontend-build |

## 3. Независимые статусы проверки

| Gate | Статус | Основание |
| --- | --- | --- |
| `SyntheticTested` | **PASS (subset)** | Rust/TS/Node tests на PR #39 HEAD; не OEM fixtures |
| `WindowsCI` | **PASS (PR #39 HEAD)** | [Actions run](https://github.com/uzbatsu-beep/MLAD/actions/runs/44571515321) |
| `UserSmoke` | **NOT RUN** | Нет независимого WebView2 Windows 10/11 протокола |
| `OEMBenchVerified` | **NOT RUN** | Нет протокола точной модели/firmware/результатов |
| `Authorized native writer` | **BLOCKED** | Не подтверждены grammar/System/User/round-trip |
| `Authorized HSES/FTP/deployment` | **BLOCKED** | Отдельного допуска нет |

## 4. Открытые технические задачи

1. **Parser fidelity:** `SYNTHETIC` корпус и coverage отсутствуют как самостоятельный проверяемый артефакт; `unknown`/`opaque` не считать распознанной логикой.
2. **Native UX:** `source↔rung↔block↔diagnostic` переходы и удобство больших файлов частичны; подсказка `Byte offset` отображает номер строки вместо byte offset.
3. **Data safety:** `check→rename` TOCTOU, Windows ACL, `*.bak` + target atomicity, concurrent writer/crash risks — **OPEN**.
4. **Dependencies:** `suppaftp` остаётся в Cargo.toml без используемого runtime; нужна отдельная ревизия.
5. **Workflow governance:** активный Ruleset `MLAD — main branch protection` требует `frontend-build`, но не `ci-gate`; изменение настройки требует явного решения владельца.
6. **Product roadmap:** `*.mlad.json`, расширенная OEM grammar и единая IDE пока не реализованы.

## 5. Следующая очередь

**PR #40:** Native CIOPRG Parser Fidelity / Coverage, начало с документационного baseline в Draft PR. **PR #41–#47:** UX, security, portable sidecar, typed grammar, static analysis, Unified IDE и условный offline release. Готовность PR #40 не определяется одним изменением Markdown.

[Roadmap v4](roadmap.md) · [План PR #40+](implementation_plan.md) · [Verification gates](verification_gates.md) · [Security risk review](filesystem_security_pr29.md) · [Протокол ручной приёмки](windows_acceptance_pr28.md).

## Проверяемое изменение статуса — PR #41 (20.11.2026)

PR #40 уже слит как документация, без внедрения парсера. В Draft PR #41 добавлены синтетический корпус CIOPRG, полная покомпонентная классификация физических строк и отображение исходных byte spans в Native Workspace. Это состояние ветки разработки, а не внедрение в main и не проверка реального контроллера. Фактическое принятие PR #41 требует завершения Linux/Windows CI и согласия владельца на merge.

Отдельные статусы: `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`; CIOPRG.LST/IONAME.DAT read-only; HSES/FTP/native writer/deployment **BLOCKED**. Старый опубликованный rc.9 не включает PR #40/#41. Настройки required checks Ruleset не менялись.

## Текущая ветка разработки: PR #42 — Native Workspace UX v2

PR #41 Parser Fidelity слит в `main` `0c1b233354564d9fd3c242675a78a45417feacc1`. Новый PR #42 создан как Draft и пока не входит в main. В ветке разработаны переходы source↔rung/block/diagnostic, выбор одной схемы, клавиатурная навигация, серверный постраничный фильтр по `SyntaxClass`, opcode и адресу с контролем SHA-256.

Доказательство CI должно относиться к финальному HEAD PR #42. Любой успешный CI не равен ручной Windows проверке WebView2 и OEM bench. `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`; CIOPRG.LST/IONAME.DAT read-only; HSES/FTP, native writer и deployment **BLOCKED**. Публичный Windows Release для данного изменения не выпускался.

## Текущая разработка: PR #43 (20.11.2026)

PR #42 Native Workspace UX v2 слит в `main` SHA `73e675f7df33c5751a35e29752736c3470b11b31`; PR #43 — отдельный Draft, пока не находится в `main`. Реализованы owner-only `0600` для новых Unix staged JSON/backup, `verify_single_link` перед переименованием временного файла, проверка File ID при уборке staged-файла и удаление неиспользуемого `suppaftp` с пятью эксклюзивными транзитивными пакетами из `Cargo.lock`, а также исключение прямых неиспользуемых `anyhow` / `futures-util` из манифеста и root lock dependencies. Добавлены синтетические положительные/отрицательные тесты и offline-security проверки.

**Критерии доказанности:** окончательные результаты Linux Rust/frontend, Windows NTFS/Tauri и `ci-gate` подтверждать строго по SHA финального HEAD; пока отчёт не принят, PR остаётся Draft. Отдельного аудита всех npm/Rust CVE и лицензий этот PR не заменяет. Риски родительского `check→rename` TOCTOU, Windows ACL/owner, согласованной записи `*.bak` и target, конкурентной записи и power-loss recovery — OPEN.

`CIOPRG.LST`/`IONAME.DAT` строго read-only; `HSES/FTP`, native writer и deployment **BLOCKED**. `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`; Windows Release не выпускался.

## Фактическое состояние разработки — PR #44 (20.11.2026)

PR #43 слит в `main` `c36dc4bc4f84b3dfdb2eac52625c3c0389226320`. PR #44 — новый Draft, **не слит**. Создан отдельный формат `*.mlad.json` / `MLAD_OFFLINE_SIDECAR` версии `schema_version=1`: привязка к SHA-256 и метаданным оригинала, комментарии по цепям/адресам/IONAME, закладки физических строк. Поддержаны load/save в Native Workspace и отдельные backend IPC `cmd_load_offline_sidecar` / `cmd_save_offline_sidecar`. Старый JSON editor запрещает запись `*.mlad.json`; нативные файлы не копируются и не изменяются.

Сохранение использует `revision` и существующий staged JSON/`*.bak`; открытый источник перечитывается только для проверки SHA-256/профиля. Тесты фиксируют валидную схему, отказы при изменении исходника и sidecar, защиту источника и корректное сохранение комментариев/закладок. Результаты CI публиковать строго по окончательному HEAD; разработка в Draft сама по себе не означает release.

Остаточные риски файловой системы из PR #43 остаются OPEN. `CIOPRG.LST`/`IONAME.DAT` read-only, native writer, FTP/HSES, live I/O, robot deployment **BLOCKED**. `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`; Windows-релиз для PR #44 не выпускался.
