<!-- MLAD-I18N:NAV:START -->
> 🌐 **Язык / Language:** [Русский](README.ru.md) · [English](README.en.md) · [Original / Исходный текст](README.md). **Machine translation / Машинный перевод: editorial review required / необходима редакторская проверка.**
<!-- MLAD-I18N:NAV:END -->

# MLAD — техническая документация

**Начните здесь.** Каталог организован по назначению; существующие URL сохраняются до проверяемой миграции. [Стандарт документации](development/documentation-standard.md) · [CHANGELOG](../CHANGELOG.md).

## Пользователю

| Раздел | Документ |
| --- | --- |
| Установка и начало работы | [README](../README.md) |
| Доступные функции и ограничения | [Статус](project_status.md) |
| Native Workspace | [Lossless parser](cioprg_lossless_parser.md), [IONAME](ioname_annotations.md) |
| Черновики | [Graph Editor](graph_editor.md), [inline editing](inline_address_editing.md), [tri-state](ui_multi_rung_state.md), [advanced blocks](advanced_blocks.md) |
| Сборка | [Windows Preview](windows_offline_preview_pr48.md), [Releases](https://github.com/uzbatsu-beep/MLAD/releases) |

## Инженерный справочник

| Раздел | Документы |
| --- | --- |
| Формат CIO | [Спецификация](cioprg_lst_spec.md), [Post-END](post_end_data.md), [Ladder](ladder_spec.md) |
| Память | [Memory map](memory_map.md), [Typed model](typed_memory_model.md) |
| Анализ | [Semantic AST](semantic_ast.md), [Typed grammar](typed_opcode_grammar.md), [OEM five opcodes](oem_5_opcode_analysis.md), [Verification matrix](semantic_verification_matrix.md) |
| Источники | [Registry](source_registry.md), [References](references.md), [Fixture intake](controller_fixture_intake.md) |
| Протокол | [HSE research](hse_protocol.md) — BLOCKED |

## Архитектура, разработка и тесты

| Раздел | Документы |
| --- | --- |
| Архитектура | [Architecture](architecture.md), [Decisions](engineering_decisions.md) |
| Сохранение | [JSON persistence](json_persistence_safety.md), [Guarded I/O](guarded_io_review.md), [Path hardening](csp_path_hardening.md), [Filesystem security](filesystem_security_pr29.md) |
| Разработка | [Workflow](development_workflow.md), [Contributing](../CONTRIBUTING.md), [Documentation standard / Engineering Excellence](development/documentation-standard.md) |
| Тесты | [Gates](verification_gates.md), [Windows acceptance](windows_acceptance_pr28.md), [OEM bench matrix](advanced_hardware_test_matrix.md) |
| Политики | [Security](../SECURITY.md), [Accessibility](../ACCESSIBILITY.md), [Licensing](licensing.md), [Code of conduct](../CODE_OF_CONDUCT.md) |

## План, выпуски и архив

| Раздел | Документы |
| --- | --- |
| Актуальный план | [Roadmap v5](roadmap.md), [Implementation plan](implementation_plan.md) |
| Выпуски | [CHANGELOG](../CHANGELOG.md), [Release notes](releases/), [GitHub Releases](https://github.com/uzbatsu-beep/MLAD/releases) |
| Аудит | [21.11.2026](repository_audit_2026-11-21.md), [17.11.2026](documentation_audit_2026-11-17.md), [Inventory](documentation_inventory_2026-11-18.md) |
| История | [Roadmap v2 archive](roadmap_v2_archive.md), [Plan v2 archive](implementation_plan_v2_archive.md), [Прежний каталог](https://github.com/uzbatsu-beep/MLAD/blob/0754dea0d37f5fbc45c795008b60fe69e6c8b08f/docs/README.md) |
| Переводы | [Translation coverage](translation_coverage.md) |

Исторические документы описывают конкретный срез. Например rc.9 не содержит PR #24–#29; текущий статус определяется [project_status](project_status.md).

## Границы доказательности

`CIOPRG.LST` и `IONAME.DAT` read-only. HSES/FTP, native writer и deployment BLOCKED. UserSmoke=NOT RUN, OEMBench=NOT RUN. PASS CI не заменяет OEM-проверку. Источники и руководства имеют приоритет перед переводом.
