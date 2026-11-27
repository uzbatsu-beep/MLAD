<!-- MLAD-I18N:EDITORIAL:source-aligned Russian; 2026-11-27 -->
> **Язык / Language:** [Русский](README.ru.md) · [English](README.en.md) · [Исходный текст / Original](README.md)

# MLAD — техническая документация

**Начните здесь.** Каталог организован по назначению; существующие URL сохраняются до проверяемой миграции. [Стандарт документации](development/documentation-standard.ru.md) · [CHANGELOG](../CHANGELOG.ru.md).

## Пользователю

| Раздел | Документ |
| --- | --- |
| Установка и начало работы | [README](../README.ru.md) |
| Доступные функции и ограничения | [Статус](project_status.ru.md) |
| Native Workspace | [Lossless parser](cioprg_lossless_parser.ru.md), [IONAME](ioname_annotations.ru.md) |
| Черновики | [Graph Editor](graph_editor.ru.md), [inline editing](inline_address_editing.ru.md), [tri-state](ui_multi_rung_state.ru.md), [advanced blocks](advanced_blocks.ru.md) |
| Сборка | [Windows Preview](windows_offline_preview_pr48.ru.md), [Releases](https://github.com/uzbatsu-beep/MLAD/releases) |

## Инженерный справочник

| Раздел | Документы |
| --- | --- |
| Формат CIO | [Спецификация](cioprg_lst_spec.ru.md), [Post-END](post_end_data.ru.md), [Ladder](ladder_spec.ru.md) |
| Память | [Memory map](memory_map.ru.md), [Typed model](typed_memory_model.ru.md) |
| Анализ | [Semantic AST](semantic_ast.ru.md), [Typed grammar](typed_opcode_grammar.ru.md), [OEM five opcodes](oem_5_opcode_analysis.ru.md), [Verification matrix](semantic_verification_matrix.ru.md) |
| Источники | [Registry](source_registry.ru.md), [References](references.ru.md), [Fixture intake](controller_fixture_intake.ru.md) |
| Протокол | [HSE research](hse_protocol.ru.md) — BLOCKED |

## Архитектура, разработка и тесты

| Раздел | Документы |
| --- | --- |
| Архитектура | [Architecture](architecture.ru.md), [Decisions](engineering_decisions.ru.md) |
| Сохранение | [JSON persistence](json_persistence_safety.ru.md), [Guarded I/O](guarded_io_review.ru.md), [Path hardening](csp_path_hardening.ru.md), [Filesystem security](filesystem_security_pr29.ru.md) |
| Разработка | [Workflow](development_workflow.ru.md), [Contributing](../CONTRIBUTING.ru.md), [Documentation standard / Engineering Excellence](development/documentation-standard.ru.md) |
| Тесты | [Gates](verification_gates.ru.md), [Windows acceptance](windows_acceptance_pr28.ru.md), [OEM bench matrix](advanced_hardware_test_matrix.ru.md) |
| Политики | [Security](../SECURITY.ru.md), [Accessibility](../ACCESSIBILITY.ru.md), [Licensing](licensing.ru.md), [Code of conduct](../CODE_OF_CONDUCT.ru.md) |

## План, выпуски и архив

| Раздел | Документы |
| --- | --- |
| Актуальный план | [Roadmap v5](roadmap.ru.md), [Implementation plan](implementation_plan.ru.md) |
| Выпуски | [CHANGELOG](../CHANGELOG.ru.md), [Release notes](releases/), [GitHub Releases](https://github.com/uzbatsu-beep/MLAD/releases) |
| Аудит | [21.11.2026](repository_audit_2026-11-21.ru.md), [17.11.2026](documentation_audit_2026-11-17.ru.md), [Inventory](documentation_inventory_2026-11-18.ru.md) |
| История | [Roadmap v2 archive](roadmap_v2_archive.ru.md), [Plan v2 archive](implementation_plan_v2_archive.ru.md), [Прежний каталог](https://github.com/uzbatsu-beep/MLAD/blob/0754dea0d37f5fbc45c795008b60fe69e6c8b08f/docs/README.md) |
| Переводы | [Translation coverage](translation_coverage.md) |

Исторические документы описывают конкретный срез. Например rc.9 не содержит PR #24–#29; текущий статус определяется [project_status](project_status.ru.md).

## Границы доказательности

`CIOPRG.LST` и `IONAME.DAT` read-only. HSES/FTP, native writer и deployment BLOCKED. UserSmoke=NOT RUN, OEMBench=NOT RUN. PASS CI не заменяет OEM-проверку. Источники и руководства имеют приоритет перед переводом.
