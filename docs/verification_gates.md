# MLAD — единые критерии верификации, CI и выпуска

**Дата:** 18.11.2026. Исходное состояние: `main` @ `2092db1825880d251c87efcb7c331d78cb5e4d28`; [основной CI успешен](https://github.com/uzbatsu-beep/MLAD/actions/runs/44228204776). [PR #26 Windows CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44227145031) проверил Windows MSVC, security tests и сборку unsigned Tauri — **не** ручную эксплуатацию WebView2 или контроллер Yaskawa.

Этот документ — acceptance contract для новых PR и offline preview. Не заменяет [стендовую матрицу](advanced_hardware_test_matrix.md) или официальный регламент безопасности.

## 1. Независимые уровни доказательств

| Уровень | Что означает | Чего не означает |
| --- | --- | --- |
| `Documented` | Факт подтверждён конкретным OEM manual, версия/раздел зафиксированы | Исполнение на данном robot/firmware |
| `SyntheticTested` | Тест успешно выполнен на искусственных данных | Совместимость с реальной программой |
| `ManualDerived` | Формула и эталонный результат реконструированы из документации | Real scan/dataflow и Live I/O |
| `WindowsCI` | Сборка/автотест на Windows runner успешны | Ручной smoke в приложении пользователя |
| `UserSmoke` | Зафиксировано ручное прохождение конкретного сценария и ОС | OEM-эквивалентность |
| `OEMBenchVerified` | Повторяемая проверка на указанном контроллере, firmware и OEM tool | Разрешение на экспорт/загрузку из MLAD |
| `Authorized` | Владелец отдельно разрешил строго ограниченную операцию | Автоматическое разрешение на иные устройства/функции |

Запись результата: `PASS / FAIL / NOT RUN / BLOCKED`, дата, commit SHA, controller/firmware (когда применимо), evidence link и ограничения.

## 2. Основной CI — обязательный на каждый PR

Запускать из чистого дерева, по lockfiles:

```bash
npm ci --no-audit --no-fund
npm run test:community
npm run test:security
npm run test:native
npm run test:diagnostics
npm run test:oem-reference
npm run test:graph
npm run test:advanced
npm run test:visual
npm run test:inline
npm run docs:check
npm run build
cargo test --manifest-path src-tauri/Cargo.toml --locked
```

CI `.github/workflows/validation.yml` запускает `frontend-build` и `rust-tests`. При изменениях Rust/Tauri/filesystem/security требуется отдельный Windows MSVC + Tauri check, если эквивалентная актуальная проверка не охвачена workflow. Для docs-only PR ручной Windows/hardware тест не нужен, но статус — `NOT RUN (docs-only)`.

## 3. Проверки по области изменений

| Область | Позитивные тесты | Отрицательные/пограничные тесты |
| --- | --- | --- |
| `CIOPRG.LST` reader | Поиск, paging, exact source spans и SHA-256 | BOM/newlines, malformed/oversized, unsupported/opaque, read-only, file swap/reparse |
| `IONAME.DAT` | Bounded preview, ручные ассоциации | Неизвестная карта, oversized, неверные index, immutable bytes |
| Native Workspace | Все доступные строки; source↔rung; компактные блоки | Unsupported не превращается в выполняемый Ladder, offscreen/large source, false LIVE |
| Typed/semantic | Проверяемые типы, ranges и links с provenance | Unknown opcode, ambiguous alias/operand, EN/scan unknown, `PART/END` boundaries |
| JSON draft/sidecar | Save/reopen, backup, schema version, source hash match | Concurrent edit, read-only path, oversize, symlink/junction, damaged JSON, wrong hash, no native overwrite |
| CSP/Tauri IPC | Локальный UI и разрешённые команды работают | Нет remote scripts/connect, `unsafe-eval`, HSES/FTP/write/robot deployment |
| Windows preview | Build, installer/portable checksums, source archive | Проверки в фактическом WebView2, отказ/restore, blocked file, ACL; unsigned статус явно указан |

## 4. Обязательный ручной Windows smoke (после технического PR и до релиза)

- [ ] Зафиксировать Windows version, WebView2, версию/commit приложения, дату и исполнителя.
- [ ] Открыть **копию** `CIOPRG.LST` через системный Open, записать исходный SHA-256 и повторно сверить SHA-256 после просмотра.
- [ ] Проверить полный source/paging, поиск и переходы по строкам, распознанные цепи, частичные и opaque участки; нет ложного «LIVE».
- [ ] Загрузить разрешённую копию `IONAME.DAT`; проверить локальные комментарии, отсутствие изменения DAT и неверной автоматической OEM-привязки.
- [ ] Создать, открыть и дважды сохранить `*.graph.json` и `*.advanced.json`; проверить `*.bak`, отказ записи в недопустимом месте и сценарий внешнего редактирования/конфликта.
- [ ] Проверить packaged Tauri CSP: нет блокировки законного локального IPC, внешние ресурсы не загружаются; считать это функциональной smoke-проверкой, не penetration test.
- [ ] Проверить большие/ошибочные/повреждённые синтетические файлы и отсутствие зависания UI.
- [ ] Убедиться, что кнопки загрузки/monitoring/writer не могут инициировать действия на роботе; **не подключать действующее промышленное оборудование**.
- [ ] Сохранить итог PASS/FAIL/NOT RUN и evidence без публикации настоящих программ/адресов/паролей.

## 5. Условия допуска release candidate

1. Все применимые CI — зелёные на **том же SHA**, который используется для сборки/тега.
2. Нет регрессии `original_source_bytes` и safety deny-only границ.
3. Windows smoke выполнен и задокументирован; если `NOT RUN`, релиз не должен утверждать ручную приемку.
4. Release assets: portable ZIP, NSIS при необходимости, source archive конкретного tag, `SHA256SUMS.txt`, GPL-3.0-only, third-party license review/уведомления и явный статус неподписанного исполняемого файла.
5. Обновлены `README.md`, `docs/project_status.md`, `docs/roadmap.md` и относящиеся к функции инженерные документы.
6. Нет исходников реальных роботов или секретов в коммитах/логах/артефактах.
7. Отдельное решение владельца о merge и, позднее, **отдельное** решение о публикации релиза.

**Запрет на ложное подтверждение:** ни один CI, скриншот или синтетический тест не переводит M10–M13 в `DONE`. Запись в контроллер и native writer запрещены до самостоятельного допуска.

## 6. Шаблон evidence для PR

| Поле | Значение |
| --- | --- |
| PR / base SHA / head SHA | Указать точно |
| Изменённые контракты | Формат/IPC/UX и контроллерная применимость |
| Фактические Linux CI / Windows CI | URL + status |
| Положительные / отрицательные тесты | Имена и результаты |
| Windows UserSmoke | PASS/FAIL/NOT RUN |
| OEM Bench | PASS/FAIL/NOT RUN/BLOCKED, модель и firmware |
| Безопасность | immutable original; HSES/FTP/writer disabled |
| Остаточный риск | Конкретное ограничение |
| Право слияния | Отдельное согласование владельца |

Дополнение: [development_workflow.md](development_workflow.md), [implementation_plan.md](implementation_plan.md), [source_registry.md](source_registry.md).

## 7. Правила репозитория и полномочия

**Наблюдение 18.11.2026:** `main` возвращается GitHub как `protected=false`. Даже успешный workflow не препятствует ручному обходу процесса без branch protection. Отдельно от этого docs PR рекомендуется настроить GitHub Ruleset/Branch Protection с обязательными проверками `frontend-build` и `rust-tests`, запретом прямых push и процедурой review, доступной для текущего типа аккаунта. Техническое правило **не заменяет** явного решения владельца о merge. Документационный PR #27 не меняет настройки репозитория.

## 8. PR #28 — Windows Acceptance & Security Regression

[Windows regression workflow](../.github/workflows/windows-security-acceptance.yml) срабатывает на релевантные PR/push в `main` и может запускаться вручную после появления в default branch. Проверяет Node security contracts, Rust file integrity/NTFS exclusive lock/junction, locked dependencies, Tauri production EXE, binary hash/PE evidence. Временный unsigned artifact хранится 14 дней и **не является релизом**. [Подробный протокол](windows_acceptance_pr28.md).

До реального подтверждения запуска CI отмечать WindowsCI `NOT RUN`; после успешного CI менять **только** WindowsCI на `PASS`. Ручной WebView2 `UserSmoke` (WIN-01…WIN-12) и OEM Bench остаются `NOT RUN` до независимого протокола. Проверка пользовательского интерфейса, control-system connectivity, абсолютная защита TOCTOU и полная атомарность backup+target не следуют из `cargo test` и `tauri build`.
