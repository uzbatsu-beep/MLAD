<!-- MLAD-I18N:NAV:START -->
> 🌐 **Язык / Language:** [Русский](roadmap.ru.md) · [English](roadmap.en.md) · [Original / Исходный текст](roadmap.md). **Machine translation / Машинный перевод: editorial review required / необходима редакторская проверка.**
<!-- MLAD-I18N:NAV:END -->

# MLAD — дорожная карта v4: безопасная Offline Engineering IDE

**Проверенный baseline:** `main` = `066db5f8c01a9da8f5f0218f12539b305ab021e4` (слияние [PR #39](https://github.com/uzbatsu-beep/MLAD/pull/39), 19.11.2026). **История:** PR #1–#39 слиты. [PR #39 CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44571515321) успешен на HEAD данного PR; это не ручная Windows приёмка и не отдельное доказательство CI на новом commit main.

**Последний подтверждённый опубликованный Windows release:** [`v0.5.0-rc.9-oem-reference`](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference) (17.11.2026), не включает PR #24–#39. `UserSmoke=NOT RUN`; `OEMBench=NOT RUN`. Историческая v3 сохранена в Git: [roadmap @ pre-PR40 main](https://github.com/uzbatsu-beep/MLAD/blob/066db5f8c01a9da8f5f0218f12539b305ab021e4/docs/roadmap.md).

## 1. Продуктовый контракт и неотменённые ограничения D1–D9

MLAD — независимая OFFLINE/BENCH среда для анализа экспортированных Yaskawa Concurrent I/O; профили `DX100`, `DX200`, `YRC1000`. В основном окне можно просматривать `CIOPRG.LST` и отдельно `IONAME.DAT`, а редактировать только самостоятельные пользовательские JSON-черновики.

- `CIOPRG.LST` / `IONAME.DAT`: **read-only**, исходные байты и `SHA-256` неизменны; native writer/export не реализован.
- `PART 1/2`, `//$M`, `END` не предоставляют автоматически прав на System/User Ladder.
- `*.graph.json`, `*.advanced.json` и legacy JSON — черновики, не программы контроллера; будущий `*.mlad.json` — отдельный sidecar, не native формат.
- `TypedCandidate`, `ManualDerived`, `SyntheticTested` не равны OEM-проверенному выполнению; `unknown` / `opaque` / `malformed` не превращаются в выдуманную Ladder-логику.
- `HSES`/`FTP`, live I/O, native writer и robot deployment **BLOCKED** до отдельных независимых допусков.
- Настоящие клиентские файлы и сведения нельзя коммитить; использовать разрешённые приватные данные и открытые `SYNTHETIC` fixtures.
- Каждый технический этап — отдельный PR, тесты обязательны; merge только после явного согласия владельца. [Решения D1–D9](engineering_decisions.md).

## 2. Что действительно реализовано в PR #1–#39

| PR | Результат | Граница |
| --- | --- | --- |
| #1–#3 | Типы памяти, lossless source и Boolean AST | Не полный OEM parser |
| #4–#8 | Graph/Advanced JSON, TMR/CNT/GSTR/GOUT subset, tri-state и inline editing | Только самостоятельные drafts |
| #9–#12 | Документация v2, GPL-3.0-only, npm lockfile / npm ci | Third-party license audit открыт |
| #13–#18 | Удалён рабочий FTP/HSES код; Native Workspace, paging/search, IONAME, post-END data | Неполная grammar/UX |
| #19–#21 | MOV/ADD/SUB/SHL/SHR candidate и статические гипотезы | Не проверены EN/flags/scan |
| #22–#29 | Security/docs, staged JSON, guarded reads, CSP, File ID, Windows regression | ACL/TOCTOU/crash risks открыты |
| #30–#35 | Roadmap v3, RU/EN локализация, редакционный аудит | Не добавлен parser corpus/sidecar |
| #36–#39 | Housekeeping, единый fail-closed ci-gate, docs-only probe, архивирование 15 workflows | Ruleset всё ещё требует frontend-build |

## 3. Функциональные этапы M0–M13

| Этап | Статус | Следующий допуск/результат |
| --- | --- | --- |
| M0 — Git/CI/legal | **DONE core / PARTIAL governance** | Ruleset обязателен `frontend-build`, но пока не `ci-gate` |
| M1 — OEM evidence | **PARTIAL** | Приватные real fixtures, firmware, независимый OEM review |
| M2 — Filesystem safety | **PARTIAL** | ACL, TOCTOU, crash/backup recovery |
| M3 — Native Workspace | **DONE baseline** | Read-only UI есть, UX требует доработки |
| M4 — Parser fidelity | **PARTIAL** | Синтетический корпус и честные coverage-метрики — PR #40 |
| M5 — Navigation/diagnostics | **PARTIAL** | Полные переходы, фильтры и large-file UX — PR #41 |
| M6 — Typed grammar/static analysis | **PARTIAL** | Новые формы — только с OEM evidence; PR #44–#45 |
| M7 — `*.mlad.json` | **OPEN** | Sidecar schema/backup/source binding — PR #43 |
| M8 — Unified Offline IDE | **OPEN** | Один инженерный сценарий без native writer — PR #46 |
| M9 — Windows acceptance | **PARTIAL** | CI есть; `UserSmoke=NOT RUN` |
| M10 — OEM bench | **NOT RUN** | Модель, firmware и разрешённый стенд не подтверждены |
| M11 — read-only HSES | **BLOCKED** | Требуется отдельный OEM/security/owner допуск |
| M12 — native writer | **BLOCKED** | Полная грамматика и безопасный OEM round-trip не доказаны |
| M13 — deployment | **BLOCKED** | Нельзя активировать загрузку программ в контроллер |

## 4. Очередь разработки после PR #39

| PR | Приоритет | Содержание |
| --- | --- | --- |
| #40 | P0 | Native Parser Fidelity — synthetic fixture corpus, coverage, spans, диагностический UI; текущий PR начинается с обновления документации |
| #41 | P0 | Native Workspace UX v2 — source↔rung↔block↔diagnostic, фильтры, большие файлы |
| #42 | P0 | Filesystem/dependency/security: ACL/TOCTOU/backup, неиспользуемые FTP-зависимости, Ruleset |
| #43 | P1 | Portable `*.mlad.json` sidecar v1, версия, bookmarks, source SHA, conflict/recovery |
| #44 | P1 | Evidence-driven typed grammar, строго по OEM подтверждённым формам |
| #45 | P1 | Static source-linked READ/WRITE/UNKNOWN, неоднозначность явно отображать |
| #46 | P1 | Unified Offline Project: immutable native snapshot + sidecar + JSON drafts |
| #47 | P1 | Условный Windows OFFLINE preview 0.6.x после ручного smoke/legal/release gate |

Номера PR после #40 ориентировочные: GitHub их не резервирует. Текущий PR #40 может начаться с документационного baseline, но не считается функционально завершённым без parser corpus/coverage, реализации и CI.

## 5. Приоритетные инженерные потоки

1. **P0 — доверие к источнику:** `SYNTHETIC` корпус, точные byte spans и line endings, прозрачное покрытие без выдуманного синтаксиса.
2. **P0 — навигация:** `source↔rung↔block↔diagnostic`, unknown-фильтры, крупные файлы и проверенный переход по физической строке.
3. **P0 — безопасность:** закрывать обнаруженные уязвимости независимо от UX; `suppaftp` в Cargo.toml требует отдельной ревизии.
4. **P1 — инженерная сессия:** `*.mlad.json` с bookmark/annotation и source binding; раздельные JSON editors остаются.
5. **P1 — семантика:** каждое расширение opcode: OEM manual/version → typed grammar → positive/negative fixture → честные статусы UNKNOWN.
6. **Сквозной gate:** `WindowsCI` ≠ `UserSmoke` ≠ `OEMBenchVerified`. Никакой релиз автоматически не разрешён.

## 6. Governance, filesystem и релизные блокеры

- GitHub Ruleset `MLAD — main branch protection` включён, но required status check — `frontend-build`, а не `ci-gate`. Изменять только после проверки и отдельного решения владельца.
- Остаточные локальные риски: `check→rename` TOCTOU, Windows ACL/owner, hardlinks после проверки, concurrent writers, atomic `*.bak` + target, crash/power-loss durability.
- Новый Windows offline release 0.6.x возможен лишь после приёмки и отдельного решения; опубликованный rc.9 не обновлять задним числом.
- `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`; `HSES`/`FTP`/native write/deploy **BLOCKED**.

## 7. Документы и traceability

[Исполнимый план](implementation_plan.md) · [Фактический статус](project_status.md) · [Решения](engineering_decisions.md) · [Verification gates](verification_gates.md) · [Приватные fixtures](controller_fixture_intake.md) · [Файловые риски](filesystem_security_pr29.md).
