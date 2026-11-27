<!-- MLAD-I18N:EDITORIAL:source-compared revision 2026-11-27; independent review still required -->
> **English** · [Русский](README.ru.md) · [English](README.en.md) · [Original / Оригинал](README.md)

# MLAD — technical documentation

**Start here.** Catalog organized by purpose; existing URLs remain until a verified migration. [Documentation standard](development/documentation-standard.en.md) · [CHANGELOG](../CHANGELOG.en.md).

## For users

| Section | Document |
| --- | --- |
| Install and quick start | [README](../README.en.md) |
| Available features and limits | [Status](project_status.en.md) |
| Native Workspace | [Lossless parser](cioprg_lossless_parser.en.md), [IONAME](ioname_annotations.en.md) |
| Drafts | [Graph Editor](graph_editor.en.md), [inline editing](inline_address_editing.en.md), [tri-state](ui_multi_rung_state.en.md), [advanced blocks](advanced_blocks.en.md) |
| Build | [Windows Preview](windows_offline_preview_pr48.en.md), [Releases](https://github.com/uzbatsu-beep/MLAD/releases) |

## Engineering reference

| Section | Documents |
| --- | --- |
| CIO format | [Specification](cioprg_lst_spec.en.md), [Post-END](post_end_data.en.md), [Ladder](ladder_spec.en.md) |
| Memory | [Memory map](memory_map.en.md), [Typed model](typed_memory_model.en.md) |
| Analysis | [Semantic AST](semantic_ast.en.md), [Typed grammar](typed_opcode_grammar.en.md), [OEM five opcodes](oem_5_opcode_analysis.en.md), [Verification matrix](semantic_verification_matrix.en.md) |
| Sources | [Registry](source_registry.en.md), [References](references.en.md), [Fixture intake](controller_fixture_intake.en.md) |
| Protocol | [HSE research](hse_protocol.en.md) — BLOCKED |

## Architecture, development and tests

| Section | Documents |
| --- | --- |
| Architecture | [Architecture](architecture.en.md), [Decisions](engineering_decisions.en.md) |
| Persistence | [JSON persistence](json_persistence_safety.en.md), [Guarded I/O](guarded_io_review.en.md), [Path hardening](csp_path_hardening.en.md), [Filesystem security](filesystem_security_pr29.en.md) |
| Development | [Workflow](development_workflow.en.md), [Contributing](../CONTRIBUTING.en.md), [Documentation standard / Engineering Excellence](development/documentation-standard.en.md) |
| Tests | [Gates](verification_gates.en.md), [Windows acceptance](windows_acceptance_pr28.en.md), [OEM bench matrix](advanced_hardware_test_matrix.en.md) |
| Policies | [Security](../SECURITY.en.md), [Accessibility](../ACCESSIBILITY.en.md), [Licensing](licensing.en.md), [Code of conduct](../CODE_OF_CONDUCT.en.md) |

## Plan, releases and archive

| Section | Documents |
| --- | --- |
| Current plan | [Roadmap v5](roadmap.en.md), [Implementation plan](implementation_plan.en.md) |
| Releases | [CHANGELOG](../CHANGELOG.en.md), [Release notes](releases/), [GitHub Releases](https://github.com/uzbatsu-beep/MLAD/releases) |
| Audit | [21 November 2026](repository_audit_2026-11-21.en.md), [17 November 2026](documentation_audit_2026-11-17.en.md), [Inventory](documentation_inventory_2026-11-18.en.md) |
| History | [Roadmap v2 archive](roadmap_v2_archive.en.md), [Plan v2 archive](implementation_plan_v2_archive.en.md), [Previous catalog](https://github.com/uzbatsu-beep/MLAD/blob/0754dea0d37f5fbc45c795008b60fe69e6c8b08f/docs/README.md) |
| Translations | [Translation coverage](translation_coverage.md) |

Historical documents describe a specific snapshot. For example rc.9 does not include PR #24–#29; [project_status](project_status.en.md) defines the current status.

## Evidence boundaries

`CIOPRG.LST` and `IONAME.DAT` are read-only. HSES/FTP, native writer and deployment BLOCKED. UserSmoke=NOT RUN, OEMBench=NOT RUN. CI PASS is not OEM verification. Original sources and manuals take precedence over translations.
