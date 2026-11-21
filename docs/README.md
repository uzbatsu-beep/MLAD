<!-- MLAD-I18N:NAV:START -->
> 🌐 **Язык / Language:** [Русский](README.ru.md) · [English](README.en.md) · [Original / Исходный текст](README.md). **Machine translation / Машинный перевод: editorial review required / необходима редакторская проверка.**
<!-- MLAD-I18N:NAV:END -->

> **Актуальная сверка (21.11.2026):** подтверждён `main` `9fcd11d61856c8a559d9acf3c77bd662ad814dfe`, опубликован Windows pre-release `v0.5.0-rc.10-unified-offline`. `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`; `HSES`/`FTP`, native writer и deployment — `BLOCKED`. [Актуальный аудит Git](repository_audit_2026-11-21.md).
> **Хронология:** прежние ссылки на rc.9 и PR #29 ниже описывают старые срезы, а не сегодняшнее состояние.

# MLAD — каталог технической документации

**Исторический статус каталога на 18.11.2026:** `main` @ `f85ddfcce3063dd087425be981a9d2b7a16d6309` (PR #1–#29 merged), [Linux CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454), [Windows CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621). Последний опубликованный [rc.9](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference) не содержит PR #24–#29. Публикация документационного PR #30 не означает merge; [текущий статус](project_status.md).

## С чего начать

| Что нужно | Документ |
| --- | --- |
| Назначение и быстрый старт | [README репозитория](../README.md) |
| Полная сверенная функциональность, риски, прогресс | [project_status.md](project_status.md) |
| **Roadmap v3 на 18.11.2026 (история):** M0–M13, приоритеты, зависимости | [roadmap.md](roadmap.md) |
| **План PR #30–#37 на 18.11.2026 (история):** объём, проверки, последовательность | [implementation_plan.md](implementation_plan.md) |
| Матрица CI, Windows smoke, проверок и релизов | [verification_gates.md](verification_gates.md) |
| Windows Acceptance & Security Regression (PR #28 merged) | [windows_acceptance_pr28.md](windows_acceptance_pr28.md) |
| Filesystem Security Hardening и остаточные угрозы (PR #29 merged) | [filesystem_security_pr29.md](filesystem_security_pr29.md) |
| Согласованные инженерные решения D1–D9 | [engineering_decisions.md](engineering_decisions.md) |
| Компоненты и разрешения доступа | [architecture.md](architecture.md) |
| Процесс разработки, CI, Windows Preview | [development_workflow.md](development_workflow.md) |
| Безопасное сохранение JSON (PR #24 merged) | [json_persistence_safety.md](json_persistence_safety.md) |
| CSP / родительские файловые пути (PR #25 merged) | [csp_path_hardening.md](csp_path_hardening.md) |
| Идентичность открытого файла и конфликты сохранения (PR #26 merged) | [guarded_io_review.md](guarded_io_review.md) |
| Правила вкладов | [CONTRIBUTING.md](../CONTRIBUTING.md) |
| Сообщество и шаблоны Issues | [CONTRIBUTING.md](../CONTRIBUTING.md) / [Issue templates](../.github/ISSUE_TEMPLATE) |
| Ответственное сообщение об уязвимостях | [SECURITY.md](../SECURITY.md) |
| Кодекс взаимодействия | [CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.md) |
| Доступность интерфейса и ограничения | [ACCESSIBILITY.md](../ACCESSIBILITY.md) |
| Шаблон PR с ограничениями MLAD | [PULL_REQUEST_TEMPLATE.md](../.github/PULL_REQUEST_TEMPLATE.md) |
| Лицензия и сторонние компоненты | [licensing.md](licensing.md) |
| Реестр ревизии всех Markdown после PR #29 | [documentation_inventory_2026-11-18.md](documentation_inventory_2026-11-18.md) |
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
| **rc.9** | **OEM reference для пяти команд — исторический выпуск от 17.11.2026** | [rc.9](releases/v0.5.0-rc.9-oem-reference.md) |

Текущий [GitHub Release](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference); опубликованный архив исходников соответствует релизному commit, а не будущим изменениям `main`.

## Исторические материалы и перенаправления

- [roadmap_v2_archive.md](roadmap_v2_archive.md) — **архив** прежней детальной дорожной карты, в ней есть устаревшие на сегодня Draft-статусы.
- [implementation_plan_v2_archive.md](implementation_plan_v2_archive.md) — **архив** прежней очереди PR #27–#35; текущие номера переназначены после документального PR #30.
- [documentation_audit_2026-11-17.md](documentation_audit_2026-11-17.md) — исторический аудит на дату создания.


- [ladder_spec.md](ladder_spec.md) → [cioprg_lst_spec.md](cioprg_lst_spec.md).
- [memory_map.md](memory_map.md) → [typed_memory_model.md](typed_memory_model.md).
- [references.md](references.md) → [source_registry.md](source_registry.md).

## Политика актуализации Markdown

- **Действующие источники истины:** этот индекс, `project_status.md`, `roadmap.md`, `implementation_plan.md`, `verification_gates.md`, `engineering_decisions.md`, `SECURITY.md`, `architecture.md`.
- **Спецификации функций** (CIOPRG, memory, parser, graph, advanced, HSES) остаются действующими **только в описанных ими границах**; старое `PR #21` в заголовке означает provenance реализации, а не последний слитый PR проекта.
- **Исторические записи** (rc.1–rc.9, архивные решения, датированные audits) не переписывать задним числом и не принимать их SHA/CI за сегодняшнее состояние.
- Отчёт по проверке покрытия всех Markdown — [documentation_inventory_2026-11-18.md](documentation_inventory_2026-11-18.md). После нового merge повторять ревизию только изменённых/статусных файлов, сохраняя архивы.

## Дисциплина доказательности

Точные различия: `Documented/ManualDerived` ≠ `SyntheticTested` ≠ `OEMBenchVerified` ≠ `Authorized`. Read-only GUI ≠ возможность native export или загрузки в робот. Каждая новая функция требует исходного OEM-источника, тестов отрицательных случаев, привязки к ControllerFamily, сценария безопасности и актуализации статуса, не стирая историю.
