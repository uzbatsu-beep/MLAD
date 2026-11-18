<!-- Generated ru companion for docs/README.md; review required. -->
> **Русский** · [Русский](README.ru.md) · [English](README.en.md) · [Original / Оригинал](README.md)
> **Translation review / Проверка:** автоматический черновик — сверяйте инженерные утверждения с оригиналом и OEM документацией.

# MLAD — каталог технической документации

**Сверено 18.11.2026:** `main` @ `f85ddfcce3063dd087425be981a9d2b7a16d6309` (PR #1#29 слияние, [Linux CI Успех](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454), [Windows CI Успех](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621). Последний опубликованный [rc.9](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference) не содержит PR #24–#29. Публикация документационного PR #30 не означает merge; [текущий статус](project_status.ru.md).

## С чего начать

| Что нужно | Документ |
| --- | --- |
| Назначение и быстрый старт | [README репозитория](../README.ru.md) |
| Полная сверенная функциональность, риски, прогресс | [project_status.md](project_status.ru.md) |
| **Актуальная roadmap v3:** M0–M13, приоритеты, зависимости | [roadmap.md](roadmap.ru.md) |
| **План разработки PR #30–#37:** объём, проверки, последовательность | [implementation_plan.md](implementation_plan.ru.md) |
| Матрица CI, Windows smoke, проверок и релизов | [verification_gates.md](verification_gates.ru.md) |
| Восприятие и регрессия безопасности WindowsPR #28 объединенный | [windows_acceptance_pr28.md](windows_acceptance_pr28.ru.md) |
| Filesystem Security Hardening и остаточные угрозы (PR #29 объединенный | [filesystem_security_pr29.md](filesystem_security_pr29.ru.md) |
| Согласованные инженерные решения D1–D9 | [engineering_decisions.md](engineering_decisions.ru.md) |
| Компоненты и разрешения доступа | [architecture.md](architecture.ru.md) |
| Процесс разработки, CI, Windows Preview | [development_workflow.md](development_workflow.ru.md) |
| Безопасное сохранение JSON (PR #24 объединенный | [json_persistence_safety.md](json_persistence_safety.ru.md) |
| CSP / родительские файловые пути (PR #25 объединенный | [csp_path_hardening.md](csp_path_hardening.ru.md) |
| Идентичность открытого файла и конфликты сохранения (PR #26 объединенный | [guarded_io_review.md](guarded_io_review.ru.md) |
| Правила вкладов | [CONTRIBUTING.md](../CONTRIBUTING.ru.md) |
| Сообщество и шаблоны Issues | [CONTRIBUTING.md](../CONTRIBUTING.ru.md) / [Выпуск шаблонов](../.github/ISSUE_TEMPLATE) |
| Ответственное сообщение об уязвимостях | [SECURITY.md](../SECURITY.ru.md) |
| Кодекс взаимодействия | [CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.ru.md) |
| Доступность интерфейса и ограничения | [ACCESSIBILITY.md](../ACCESSIBILITY.ru.md) |
| Шаблон PR с ограничениями MLAD | [PULL_REQUEST_TEMPLATE.md](../.github/PULL_REQUEST_TEMPLATE.ru.md) |
| Лицензия и сторонние компоненты | [licensing.md](licensing.ru.md) |
| Реестр ревизии всех Markdown после PR #29 | [documentation_inventory_2026-11-18.md](documentation_inventory_2026-11-18.ru.md) |
| Исторический аудит документации на 17.11.2026 | [documentation_audit_2026-11-17.md](documentation_audit_2026-11-17.ru.md) |

## Исходные файлы, адреса и семантика

| Направление | Документ |
| --- | --- |
| Первичные источники/ревизии Yaskawa | [source_registry.md](source_registry.ru.md) |
| Native CIOPRG: известное и неизвестное | [cioprg_lst_spec.md](cioprg_lst_spec.ru.md) |
| Байт-сохраняющий импорт | [cioprg_lossless_parser.md](cioprg_lossless_parser.ru.md) |
| Пост-END ALARM/MESSAGE | [post_end_data.md](post_end_data.ru.md) |
| Структура контроллерного корпуса, защита исходников | [controller_fixture_intake.md](controller_fixture_intake.ru.md) |
| DX100/DX200/YRC1000 Профили памяти | [typed_memory_model.md](typed_memory_model.ru.md) |
| Булева Semantic AST | [semantic_ast.md](semantic_ast.ru.md) |
| Типизированные MOV/ADD/SUB/SHL/SHR | [typed_opcode_grammar.md](typed_opcode_grammar.ru.md) |
| Проверенная документация по пяти командам и неизвестные края | [oem_5_opcode_analysis.md](oem_5_opcode_analysis.ru.md) |
| Статические гипотезы M-зависимостей, OEM-допуски | [semantic_verification_matrix.md](semantic_verification_matrix.ru.md) |
| TMR/CNT/GSTR/GOUT | [advanced_blocks.md](advanced_blocks.ru.md) |
| Матрица **не проведённых** испытаний на роботе | [advanced_hardware_test_matrix.md](advanced_hardware_test_matrix.ru.md) |
| HSES **не включён** | [hse_protocol.md](hse_protocol.ru.md) |

## Интерфейс и пользовательские данные

- [graph_editor.md](graph_editor.ru.md) — безопасные JSON черновики; не native export.
- [ui_multi_rung_state.md](ui_multi_rung_state.ru.md) — визуальные цепи и OFFLINE tri-state.
- [inline_address_editing.md](inline_address_editing.ru.md) — редактирование адресов в JSON и локальных заметок отдельно.
- [ioname_annotations.md](ioname_annotations.ru.md) — имена I/O и локальные комментарии без изменения native файлов.

## Release notes (архивные документы конкретных сборок)

Все release notes ниже относятся **только к своему tag/commit**. Они не изменяются задним числом, чтобы не искажать воспроизводимые сведения о ранее опубликованном артефакте.

| Релиз | Особенность | Файл |
| --- | --- | --- |
| rc.1 | Первый Windows hardware bench preview | [rc.1](releases/v0.5.0-rc.1-hardware.ru.md) |
| rc.2 | CIOPRG только для чтения | [rc.2](releases/v0.5.0-rc.2-cioprg.ru.md) |
| rc.3 | Встроенный Native Workspace | [rc.3](releases/v0.5.0-rc.3-workspace.ru.md) |
| rc.4 | IONAME / локальные комментарии | [rc.4](releases/v0.5.0-rc.4-annotations.ru.md) |
| rc.5 | Компактная панель и системные диалоги | [rc.5](releases/v0.5.0-rc.5-layout.ru.md) |
| rc.6 | Post-END и группировка диагностики | [rc.6](releases/v0.5.0-rc.6-postend.ru.md) |
| rc.7 | Типизация пяти команд | [rc.7](releases/v0.5.0-rc.7-typed.ru.md) |
| rc.8 | Статические гипотезы источников | [rc.8](releases/v0.5.0-rc.8-evidence.ru.md) |
| **rc.9** | **OEM reference для пяти команд — последний опубликованный** | [rc.9](releases/v0.5.0-rc.9-oem-reference.ru.md) |

Текущий [Релиз GitHub](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference); опубликованный архив исходников соответствует релизному commit, а не будущим изменениям `main`.

## Исторические материалы и перенаправления

- [roadmap_v2_archive.md](roadmap_v2_archive.ru.md) — **архив** прежней детальной дорожной карты, в ней есть устаревшие на сегодня Draft-статусы.
- [implementation_plan_v2_archive.md](implementation_plan_v2_archive.ru.md) — **архив** прежней очереди PR #27–#35; текущие номера переназначены после документального PR #30.
- [documentation_audit_2026-11-17.md](documentation_audit_2026-11-17.ru.md) — исторический аудит на дату создания.


- [ladder_spec.md](ladder_spec.ru.md) → [cioprg_lst_spec.md](cioprg_lst_spec.ru.md).
- [memory_map.md](memory_map.ru.md) → [typed_memory_model.md](typed_memory_model.ru.md).
- [references.md](references.ru.md) → [source_registry.md](source_registry.ru.md).

## Политика актуализации Markdown

- **Действующие источники истины:** этот индекс, `project_status.md`, `roadmap.md`, `implementation_plan.md`, `verification_gates.md`, `engineering_decisions.md`, `SECURITY.md`, `architecture.md`.
- **Спецификации функций** (CIOPRG, memory, parser, graph, advanced, HSES) остаются действующими **только в описанных ими границах**; старое `PR #21` в заголовке означает provenance реализации, а не последний слитый PR проекта.
- **Исторические записи** (rc.1–rc.9, архивные решения, датированные audits) не переписывать задним числом и не принимать их SHA/CI за сегодняшнее состояние.
- Отчёт по проверке покрытия всех Markdown — [documentation_inventory_2026-11-18.md](documentation_inventory_2026-11-18.ru.md). После нового merge повторять ревизию только изменённых/статусных файлов, сохраняя архивы.

## Дисциплина доказательности

Точные различия: `Documented/ManualDerived` ≠ `SyntheticTested` ≠ `OEMBenchVerified` ≠ `Authorized`. Read-only GUI ≠ возможность native export или загрузки в робот. Каждая новая функция требует исходного OEM-источника, тестов отрицательных случаев, привязки к ControllerFamily, сценария безопасности и актуализации статуса, не стирая историю.
