# Процесс разработки, тестирования и релизов MLAD

**Актуализация:** 18.11.2026, `main` `2092db1825880d251c87efcb7c331d78cb5e4d28`, PR #1–#26 merged, [validation SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44228204776), [PR #26 Windows CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44227145031); опубликованный [rc.9 Windows](https://github.com/uzbatsu-beep/MLAD/actions/runs/44162078155) собран до PR #24–#26. [Текущий статус](project_status.md).

## 1. Git и изменение функций

1. Основание — проверенный текущий `main`; каждый этап разработки и документации идёт в **отдельной ветке** и PR.
2. Не сливать PR без явного решения владельца проекта; рабочая ветка с ошибками CI не считается принятой.
3. Разделять по scope: `compiler/` Rust, `src/ladder/` чистые frontend модели, `src/components/` визуализация, `docs/` доказательства/контракты.
4. Новый оператор требует test matrix: controller family, operands, диапазоны, права классов памяти, реальные vendor evidence, негативные тесты, fail-closed поведение.
5. Перед изменением архитектуры контроллера согласовывать System/User boundaries; включение аппаратной связи и native writer — **отдельные решения и отдельные PR**.
6. Запрещать регрессии: исходный `CIOPRG.LST` не изменяется от просмотра; редактирование JSON никогда не ведёт к FTP/HSES.

7. Для публичного Issue используйте формы Bug Report, Feature Request, Controller Compatibility в [ISSUE_TEMPLATE](../.github/ISSUE_TEMPLATE); настоящие программы роботов и secrets запрещены. Для уязвимостей — [SECURITY.md](../SECURITY.md) и приватный канал, если он доступен. Для PR — [PULL_REQUEST_TEMPLATE.md](../.github/PULL_REQUEST_TEMPLATE.md).

## 2. Команды локальной проверки

Рекомендуемая среда CI: Node.js 22, npm 10 (lockfile v3), стабильный Rust, Tauri v2 prerequisites, Windows MSVC для выпуска Windows. Перед разработкой выполнить `npm ci`: оно использует отслеживаемый в Git `package-lock.json`; обновлять зависимости — только осознанным изменением `package.json` вместе с lockfile.

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

Для локального GUI: `npm run tauri dev`. Проверять ручным сценарием сохранение/загрузку JSON, размеры окна и двойной щелчок по SVG, состояние UNKNOWN, запрет HSES/FTP и неизменность исходного LST.

Документационная проверка `npm run docs:check` отдельно проверяет относительные ссылки и исторические escaped newline-подстановки; **она не проверяет фактическую доступность внешних ссылок или их достоверность**.

`MLAD validation` запускается на PR в `main` и после push в `main`: использует `npm ci --no-audit --no-fund`, проверяет неизменность lockfile после установки, выполняет Rust-тесты (Ubuntu), frontend тесты, проверку типов и Vite build. Прогон Windows/MSVC выполняется **отдельным** workflow, а не каждым PR. Полный список тестов зависит от конкретного workflow — проверять YAML и commit SHA.

## 3. Quality gates

| Gate | Требование |
|---|---|
| Q0 / Provenance | Основная ветка и SHA PR зафиксированы |
| Q1 / Build | TypeScript, Vite, Rust unit tests успешны на HEAD |
| Q2 / Semantics | Новые инструкции имеют источники Yaskawa, тесты границ и диагностик |
| Q3 / Safety | Отсутствует обход запрета CIOPRG write, FTP/HSES; JSON форматы изолированы |
| Q4 / Regression | Проводка AND/OR, NC, TMR/CNT C/R, GSTR/GOUT, Undo/Redo проверены |
| Q5 / Windows | Для Windows-сборки: MSVC/Tauri build, runtime WebView2 и ручной UI smoke test |
| Q6 / Hardware | **Отдельная** программа испытаний и разрешение; CI не закрывает этот gate |

При отклонении по Q2/Q3/Q6 нельзя объявлять прошедший CI «готовностью к загрузке на робот».

## 4. Реальный статус релизных workflow на 18.11.2026

- [`validation.yml`](../.github/workflows/validation.yml) — действующий Ubuntu pipeline: Rust tests, npm ci с lockfile, безопасность, Native Workspace, diagnostics, OEM справка, графика, TypeScript/Vite и docs-check. Последний `main` [SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44166354563).
- [`windows-oem-reference-rc9.yml`](../.github/workflows/windows-oem-reference-rc9.yml) — последний проверенный Windows x64 NSIS/portable/source/SHA256 pipeline. [RC9 SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44162078155), [релиз опубликован](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference).
- Исторические Windows workflows `windows-bench-rc.yml`, `windows-ui-preview.yml` и `windows-*-rc[2-8].yml` привязаны к исходным branch/tag и **не** автоматически пересобирают старые версии. Для нового RC нужен отдельный tag/branch и запрет перезаписи существующего релиза.
- `package-lock.json` (v3) и `Cargo.lock` отслеживаются в Git; npm 22/CI через `npm ci`; Rust использует `cargo test --locked`. Это не битовая воспроизводимость Windows EXE и не аппаратная верификация.
- Для выпуска: проверить Windows MSVC, portable ZIP, NSIS, source archive, SHA256SUMS, GPL/third-party notices, WebView2 и отдельный ручной Windows smoke. **Ни один GitHub workflow не закрывает OEM bench gate.**

## 5. Версионирование документации

- Фиксировать `as of` дату, commit/PR при изменении статуса.
- Исторический release note не «переписывать» под новую версию; дополнять внешней ссылкой на `main` и changelog.
- Документация определяет возможности только вместе с кодом и тестами; при противоречии проверять исходный Rust/TypeScript.
- Источники vendor: фиксировать семейство и точную ревизию руководства, не выводить из примера полную грамматику и права записи.

См. [CONTRIBUTING.md](../CONTRIBUTING.md) и [контроллерные фикстуры](controller_fixture_intake.md).


## 6. Проверка безопасного сохранения JSON (PR #24 merged)

Новый staged writer тестируется Rust-модулем `draft_io.rs`, а `test:security` дополнительно запрещает прямую запись JSON из Tauri handlers. Для приёмки нужны: новый/существующий черновик, предыдущая `*.bak`, oversized input, прямая symlink, ошибка создания/замены и проверка неизменности оригинальных `CIOPRG.LST`/`IONAME.DAT`. Rust CI на Linux **не доказывает** идентичное поведение замены на Windows; до выпуска нужен Windows smoke с закрытым WebView2 и открытым JSON-файлом. Действующие угрозы CSP, parent symlink/races и backup transaction остались за рамками этого PR.


## 7. PR #25 — CSP и защита пути (merged)

Перед выпуском проверить синтаксис CSP через `npm run test:security`, `cargo test --locked`, сборку Windows Tauri, запуск WebView2, проверку системного Open dialog, визуальных Ladder/TMR панелей, локального редактирования JSON и отсутствие blocked Tauri IPC. Нормальные символические ссылки предков сознательно отвергаются для уменьшения риска: пользователь должен выбирать **прямой путь к обычной копии файла**. Тесты не устраняют filesystem TOCTOU; отдельный review directory-handle/Windows reparse остаётся.

## 8. PR #26 — guarded reads / JSON conflict guard (merged)

Проверки identity/reparse ограничивают ошибки подмены между путём и открытым дескриптором; JSON-писатель отказывает при обнаруженном конкурентном изменении. Автоматические Linux/Windows CI пройдены, но не устраняют финальное окно `check→rename`, reparse/ACL/power-loss edge cases и не являются ручной Windows приёмкой. [Guarded I/O](guarded_io_review.md).

## 9. Единый checklist новых PR

При планировании следовать [implementation_plan.md](implementation_plan.md), после каждой функциональной доработки заполнять [verification_gates.md](verification_gates.md), согласованные запреты сверять с [engineering_decisions.md](engineering_decisions.md). Результаты CI привязывать к **точному head SHA**. Для документационного PR Windows/hardware — `NOT RUN (docs-only)`; для функционального Windows PR требуются тесты на Tauri runner, а перед пользовательским релизом — ручной smoke. Ни один этап не снимает запрет на HSES/FTP, native writer или deployment.
