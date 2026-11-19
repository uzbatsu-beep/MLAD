<!-- MLAD-I18N:EDITORIAL:source-aligned Russian; v4 updated 2026-11-19 -->
> **Язык / Language:** [Русский](project_status.ru.md) · [English](project_status.en.md) · [Исходный текст / Original](project_status.md)
>
> **Примечание:** русскоязычный исходник сохранён без машинного пересказа; OEM validation independent.

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

[Roadmap v4](roadmap.ru.md) · [План PR #40+](implementation_plan.ru.md) · [Verification gates](verification_gates.ru.md) · [Security risk review](filesystem_security_pr29.ru.md) · [Протокол ручной приёмки](windows_acceptance_pr28.ru.md).
