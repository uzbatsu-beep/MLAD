# MLAD — каталог технической документации

**Актуально для `main` от 18.11.2026:** PR #1–#21 merged; текущий Windows pre-release — `v0.5.0-rc.9-oem-reference`. Полная матрица: [project_status.md](project_status.md). Архивные документы намеренно сохраняют своё исходное датированное содержание, а приведённый здесь индекс указывает текущий статус.

## С чего начать

| Что нужно | Документ |
| --- | --- |
| Назначение и быстрый старт | [README репозитория](../README.md) |
| Полная сверенная функциональность, риски, прогресс | [project_status.md](project_status.md) |
| Этапы M0–M13 и критерии завершения | [roadmap.md](roadmap.md) |
| Компоненты и разрешения доступа | [architecture.md](architecture.md) |
| Процесс разработки, CI, Windows Preview | [development_workflow.md](development_workflow.md) |
| Безопасное сохранение JSON (PR #24 merged) | [json_persistence_safety.md](json_persistence_safety.md) |
| CSP / родительские файловые пути (PR #25 Draft) | [csp_path_hardening.md](csp_path_hardening.md) |
| Правила вкладов | [CONTRIBUTING.md](../CONTRIBUTING.md) |
| Сообщество и шаблоны Issues | [CONTRIBUTING.md](../CONTRIBUTING.md) / [Issue templates](../.github/ISSUE_TEMPLATE) |
| Ответственное сообщение об уязвимостях | [SECURITY.md](../SECURITY.md) |
| Кодекс взаимодействия | [CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.md) |
| Доступность интерфейса и ограничения | [ACCESSIBILITY.md](../ACCESSIBILITY.md) |
| Шаблон PR с ограничениями MLAD | [PULL_REQUEST_TEMPLATE.md](../.github/PULL_REQUEST_TEMPLATE.md) |
| Лицензия и сторонние компоненты | [licensing.md](licensing.md) |
| Исторический аудит документации на 17.11.2026 | [documentation_audit_2026-11-17.md](documentation_audit_2026-11-17.md) |

## Исходные файлы, адреса и семантика

| Направление | Документ |
| --- | --- |
| Первичные источники/ревизии Yaskawa | [source_registry.md](source_registry.md) |
| Native CIOPRG: известное и неизвестное | [cioprg_lst_spec.md](cioprg_lst_spec.md) |
| Байт-сохраняющий импорт | [cioprg_lossless_parser.md](cioprg_lossless_parser.md) |
| Post-END ALARM/MESSAGE | [post_end_data.md](post_end_data.md) |
| Структура контроллерного корпуса, защита исходников | [controller_fixture_intake.md](controller_fixture_intake.md) |
| DX100/DX200/YRC1000 memory profiles | [typed_memory_model.md](typed_memory_model.md) |
| Булева Semantic AST | [semantic_ast.md](semantic_ast.md) |
| Типизированные MOV/ADD/SUB/SHL/SHR | [typed_opcode_grammar.md](typed_opcode_grammar.md) |
| Проверенная документация по пяти командам и неизвестные края | [oem_5_opcode_analysis.md](oem_5_opcode_analysis.md) |
| Статические гипотезы M-зависимостей, OEM-допуски | [semantic_verification_matrix.md](semantic_verification_matrix.md) |
| TMR/CNT/GSTR/GOUT | [advanced_blocks.md](advanced_blocks.md) |
| Матрица **не проведённых** испытаний на роботе | [advanced_hardware_test_matrix.md](advanced_hardware_test_matrix.md) |
| HSES **не включён** | [hse_protocol.md](hse_protocol.md) |

## Интерфейс и пользовательские данные

- [graph_editor.md](graph_editor.md) — безопасные JSON черновики; не native export.
- [ui_multi_rung_state.md](ui_multi_rung_state.md) — визуальные цепи и OFFLINE tri-state.
- [inline_address_editing.md](inline_address_editing.md) — редактирование адресов в JSON и локальных заметок отдельно.
- [ioname_annotations.md](ioname_annotations.md) — имена I/O и локальные комментарии без изменения native файлов.

## Release notes (архивные документы конкретных сборок)

Все release notes ниже относятся **только к своему tag/commit**. Они не изменяются задним числом, чтобы не искажать воспроизводимые сведения о ранее опубликованном артефакте.

| Релиз | Особенность | Файл |
| --- | --- | --- |
| rc.1 | Первый Windows hardware bench preview | [rc.1](releases/v0.5.0-rc.1-hardware.md) |
| rc.2 | Native read-only CIOPRG | [rc.2](releases/v0.5.0-rc.2-cioprg.md) |
| rc.3 | Встроенный Native Workspace | [rc.3](releases/v0.5.0-rc.3-workspace.md) |
| rc.4 | IONAME / локальные комментарии | [rc.4](releases/v0.5.0-rc.4-annotations.md) |
| rc.5 | Компактная панель и системные диалоги | [rc.5](releases/v0.5.0-rc.5-layout.md) |
| rc.6 | Post-END и группировка диагностики | [rc.6](releases/v0.5.0-rc.6-postend.md) |
| rc.7 | Типизация пяти команд | [rc.7](releases/v0.5.0-rc.7-typed.md) |
| rc.8 | Статические гипотезы источников | [rc.8](releases/v0.5.0-rc.8-evidence.md) |
| **rc.9** | **OEM reference для пяти команд — текущая** | [rc.9](releases/v0.5.0-rc.9-oem-reference.md) |

Текущий [GitHub Release](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference); опубликованный архив исходников соответствует релизному commit, а не будущим изменениям `main`.

## Архивные перенаправления

- [ladder_spec.md](ladder_spec.md) → [cioprg_lst_spec.md](cioprg_lst_spec.md).
- [memory_map.md](memory_map.md) → [typed_memory_model.md](typed_memory_model.md).
- [references.md](references.md) → [source_registry.md](source_registry.md).

## Дисциплина доказательности

Точные различия: `Documented/ManualDerived` ≠ `SyntheticTested` ≠ `OEMBenchVerified` ≠ `Authorized`. Read-only GUI ≠ возможность native export или загрузки в робот. Каждая новая функция требует исходного OEM-источника, тестов отрицательных случаев, привязки к ControllerFamily, сценария безопасности и актуализации статуса, не стирая историю.
