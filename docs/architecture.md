# Архитектура MLAD и границы доверия

**Статус:** PR #13 относительно `main` после PR #12 (17.11.2026); новые меры ещё не слиты. Этот документ описывает реализованные и явно отмеченные предлагаемые изменения.

## 1. Компоненты и потоки данных

```text
React / TypeScript / Tauri frontend
├── GraphWorkspace.tsx / LadderCircuit.tsx
│   ├── graphModel.ts (AND/OR, NO/NC, layout, JSON draft)
│   ├── visualState.ts (TRUE/FALSE/UNKNOWN — ручные сигналы)
│   └── SvgOperandInput.tsx + inlineOperand.ts (ввод адреса на схеме)
├── AdvancedBlocksPanel.tsx / AdvancedBlockDiagram.tsx
│   └── advancedModel.ts (*.advanced.json, TMR/CNT/GSTR+GOUT)
├── CioprgInspector.tsx — read-only Original CIOPRG.LST
└── Legacy LadderCanvas.tsx — project.json (поддержка старого формата)
         │
         ▼ Tauri IPC с разными командами на каждую модель
Rust src-tauri/src/main.rs
├── compiler/memory.rs — ControllerFamily и группы адресов
├── compiler/graph_draft.rs — JSON структура / проверки / сериализация
├── compiler/advanced.rs — функциональные блоки / офлайн-шаги
├── compiler/lossless.rs — исходные байты CIOPRG.LST / SHA-256 / spans
├── compiler/semantic.rs — ограниченный статический AST и diagnostics
├── compiler/advanced_source.rs — распознавание узких native-паттернов
├── compiler/parser.rs — legacy, НЕ нативный безопасный парсер
└── network/mod.rs — удалён в предложенном PR #13 (нет сетевого клиента в backend)
```

**Нет маршрута `*.graph.json` или `*.advanced.json` → контроллеру.** Отображённая мнемоника — справочная, не native компиляция.

## 2. Форматы, права записи и защита

| Вход / выход | Команда или слой | Возможность изменения |
|---|---|---|
| `CIOPRG.LST` исходный | `cmd_inspect_cioprg` → lossless / semantic / advanced_source | **Только чтение**; source SHA-256 |
| `*.graph.json` | `cmd_validate_graph_draft`, `cmd_save_graph_draft`, `cmd_load_graph_draft` | Отдельный JSON-черновик |
| `*.advanced.json` | `cmd_validate_advanced_draft`, `cmd_save_advanced_draft`, `cmd_load_advanced_draft` | Отдельный JSON-черновик |
| `project.json` | legacy `cmd_save_project` / `cmd_load_project` | Устаревшая JSON-модель |
| Ручные логические тесты | `cmd_test_tmr_step`, `cmd_test_cnt_step`, `cmd_test_group_copy` | Локальные функции, без HSES |
| FTP на робот | `cmd_deploy_to_robot` | **Возвращает отказ** |
| HSES мониторинг | `cmd_toggle_monitoring` | **Отказ во включении**, останов старой задачи при вызове |

Backend запрещает записывать JSON под произвольным `.LST` расширением; `*.graph.json` и `*.advanced.json` не смешиваются с legacy JSON. При записи существующих JSON выполняются проверка типа файла и отказ для существующего symlink; старое содержимое копируется в `.bak` там, где это поддержано.

**Граница этих защит:** это не атомарная транзакция, не системный sandbox и не защита от произвольного доступа к каталогам. Между проверкой пути и записью теоретически существует TOCTOU; hardlinks, гонки, ACL и восстановление после прерванной записи требуют отдельного технического решения. Для критических копий применять внешний резервный backup.

## 3. Индикация и семантика

- OFFLINE `TRUE` = зелёный, `FALSE` = красный, `UNKNOWN` = серый. Контакты NC инвертируют тестовый бит, катушка показывает результат логического условия, **не выходную обратную связь** робота.
- TMR — тестовый пошаговый таймер 100 мс; CNT — модель счёта по фронту C и независимого сброса R; GSTR/GOUT — копирование одного 8-битного значения в локальной модели.
- `PART 1/PART 2` в AST лишь метки, не проверенная граница прав доступа System/User Ladder.
- При нераспознанном opcode/топологии выводится диагностика; из неизвестной последовательности не строится выдуманная исполняемая цепь.
- Двойной щелчок по адресу меняет **только JSON-черновик**. Оригинальный файл в Inspector имеет read-only callback.

## 4. Известные инженерные риски и технический долг

1. **Предлагаемый PR #13 удаляет прежний неподтверждённый HSES/FTP код** и сетевое состояние backend. Команды Tauri остаются совместимыми по именам и возвращают отказ. Исторические зависимости в `Cargo.toml` пока не очищены; это отдельная задача с согласованным обновлением `Cargo.lock`.
2. `tauri.conf.json` задаёт `csp: null`; требуется отдельный security review перед подключением сети, плагинов и непроверенного содержимого.
3. PR #12 зафиксировал `package-lock.json` и `npm ci`. Воспроизводимость Windows-бинарников и пересмотр Rust зависимостей остаются задачами релизного процесса.
4. Сохранение JSON не полностью атомарное, file picker и workflow каталогов/backup требуют улучшения.
5. Полная грамматика `CIOPRG.LST`, защищённые секции, истинное время сканирования, правила назначения и firmware-профили не подтверждены оригинальными golden fixtures.
6. Набор тестов покрывает **модель**, но не доказывает эквивалентность Ladder Editor на DX100/DX200/YRC1000.

См. [roadmap.md](roadmap.md), [development_workflow.md](development_workflow.md), [controller_fixture_intake.md](controller_fixture_intake.md).
