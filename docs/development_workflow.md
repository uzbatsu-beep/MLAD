# Процесс разработки, тестирования и релизов MLAD

**Применение:** PR после #8; изоляция изменений и запрет непроверенных операций с Yaskawa.

## 1. Git и изменение функций

1. Основание — проверенный текущий `main`; каждый этап разработки и документации идёт в **отдельной ветке** и PR.
2. Не сливать PR без явного решения владельца проекта; рабочая ветка с ошибками CI не считается принятой.
3. Разделять по scope: `compiler/` Rust, `src/ladder/` чистые frontend модели, `src/components/` визуализация, `docs/` доказательства/контракты.
4. Новый оператор требует test matrix: controller family, operands, диапазоны, права классов памяти, реальные vendor evidence, негативные тесты, fail-closed поведение.
5. Перед изменением архитектуры контроллера согласовывать System/User boundaries; включение аппаратной связи и native writer — **отдельные решения и отдельные PR**.
6. Запрещать регрессии: исходный `CIOPRG.LST` не изменяется от просмотра; редактирование JSON никогда не ведёт к FTP/HSES.

## 2. Команды локальной проверки

Рекомендуемая среда CI: Node.js 22, стабильный Rust, Tauri v2 prerequisites, Windows MSVC для выпуска Windows.

```bash
npm install --no-audit --no-fund
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

`MLAD validation` запускается на PR в `main` и после push в `main`, выполняет Rust-тесты (Ubuntu), frontend тесты, проверку типов и Vite build. Прогон Windows/MSVC выполняется **отдельным** workflow, а не каждым PR. Полный список тестов зависит от конкретного workflow — проверять YAML и commit SHA.

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

## 4. Реальный статус релизных workflow на 17.11.2026

- [`.github/workflows/validation.yml`](../.github/workflows/validation.yml): постоянно действующий, только проверки.
- [`.github/workflows/windows-bench-rc.yml`](../.github/workflows/windows-bench-rc.yml): **исторический**, настроен на branch `release/0.5.0-rc1-hardware` и фиксированный tag `v0.5.0-rc.1-hardware`; **не использовать повторно для новой версии без изменения tag и проверки идемпотентности публикации**.
- [`.github/workflows/windows-ui-preview.yml`](../.github/workflows/windows-ui-preview.yml): исторический preview для `feat/pr8-inline-operand-editing`; артефакты Actions ограничены сроком хранения, **не равны GitHub Release**.
- [`v0.5.0-rc.1-hardware`](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.1-hardware) опубликован **до** PR #7 и #8. Нельзя считать его автоматически обновлённым после merge.
- Сборка Windows RC содержит автономный WebView2 в NSIS, unsigned EXE; переносимая версия требует установленный WebView2. Для следующих релизов нужна отдельная версионированная pipeline конфигурация.

**Долг по воспроизводимости:** пока npm lockfile отсутствует в git и CI использует `npm install`, а не `npm ci`. Нельзя заявлять exact reproducible build независимо от конкретного build snapshot.

## 5. Версионирование документации

- Фиксировать `as of` дату, commit/PR при изменении статуса.
- Исторический release note не «переписывать» под новую версию; дополнять внешней ссылкой на `main` и changelog.
- Документация определяет возможности только вместе с кодом и тестами; при противоречии проверять исходный Rust/TypeScript.
- Источники vendor: фиксировать семейство и точную ревизию руководства, не выводить из примера полную грамматику и права записи.

См. [CONTRIBUTING.md](../CONTRIBUTING.md) и [контроллерные фикстуры](controller_fixture_intake.md).
