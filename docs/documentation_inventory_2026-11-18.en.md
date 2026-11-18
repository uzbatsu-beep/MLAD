<!-- Generated en companion for docs/documentation_inventory_2026-11-18.md; review required. -->
> **English** · [Русский](documentation_inventory_2026-11-18.ru.md) · [English](documentation_inventory_2026-11-18.en.md) · [Original / Оригинал](documentation_inventory_2026-11-18.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.
> **MLAD safety invariants:** `CIOPRG.LST` and `IONAME.DAT` are strictly **read-only**. FTP/HSES, live I/O, native writer and robot deployment remain **BLOCKED**. Passing `WindowsCI` does **not** imply `UserSmoke=PASS` or `OEMBenchVerified`: both remain **NOT RUN** unless independently documented. Machine translation requires source and Yaskawa manual review.

# MLAD – an audit of the entire Markdown directory after PR #29

**Date:** 18.11.2026. **Verified baseline:** `main` `f85ddfcce3063dd087425be981a9d2b7a16d6309` (PR #1-#29 merged. All tracks checked. `*.md` из [github](https://github.com/uzbatsu-beep/MLAD/tree/f85ddfcce3063dd087425be981a9d2b7a16d6309). In the original `main` - **50 Markdown. The proposed branch of this document is **53 Markdown**: two new historical archives and this registry. 38 Markdown files have proposed changes (including two archives), 14 source files are deliberately left unchanged. The links below refer to the branch PR #30 To merge.

**Technical control fact:** [Linux CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454), [Windows CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621) for `main`; the new documentation branch requires its own ** `docs:check` и `MLAD validation` on the actual Head SHA. `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`, `HSES/native writer/deployment=BLOCKED`.

## 1. Key audit findings

1. Removed confusion with **PR #29 Draft**: in fact PR #29 merged, а `main` Windows File ID+byte snapshots, hardlink/staged SHA-256 guards и Windows ADS/UNC/device path deny.
2. `docs/roadmap.md` Updated to **v3** with actual milestone M0–M13, standalone evidence gates and parser fixtures/Native Workspace priority; the old v2 is saved entirely in the archive.
3. `docs/implementation_plan.md` Now leads the preliminary queue **#30-#37** and separates the runtime PR from the documentation; the former table is saved as v2 archive.
4. `docs/project_status.md` Reconfirmed with GitHub sources: `main` SHA, Linux/Windows CI, latest official release `rc.9` and independent hand smoke. ** New CI artifacts not named release.**
5. Technical anachronisms have been corrected: `cioprg_lst_spec.md` old `network/mod.rs` is no longer described as existing; `hse_protocol.md` obsolete `csp: null`. It is noted that Cargo contains `suppaftp` dependency, although runtime HSES/FTP blocked.
6. Active feature-specs retain historical evidence/contracts but no longer use banner `PR #1–#21 merged` The overall level of readiness of the project.
7. Archival release notes `rc.1–rc.9`Original Dated Audits and Legacy Pointers are saved without rewriting the facts of past assemblies. These CIOPRG/IONAME and sensitive sources have not been added.

## 2. Current documents: 37 MD

| File. | Audit result |
| --- | --- |
| [.github/PULL_REQUEST_TEMPLATE.md](../.github/PULL_REQUEST_TEMPLATE.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [ACCESSIBILITY.md](../ACCESSIBILITY.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [CONTRIBUTING.md](../CONTRIBUTING.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [README.md](../README.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [SECURITY.md](../SECURITY.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/README.md](README.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/advanced_blocks.md](advanced_blocks.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/advanced_hardware_test_matrix.md](advanced_hardware_test_matrix.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/architecture.md](architecture.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/cioprg_lossless_parser.md](cioprg_lossless_parser.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/cioprg_lst_spec.md](cioprg_lst_spec.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/controller_fixture_intake.md](controller_fixture_intake.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/csp_path_hardening.md](csp_path_hardening.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/development_workflow.md](development_workflow.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/documentation_inventory_2026-11-18.md](documentation_inventory_2026-11-18.en.md) | **Created** - Updating Registry |
| [docs/engineering_decisions.md](engineering_decisions.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/filesystem_security_pr29.md](filesystem_security_pr29.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/graph_editor.md](graph_editor.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/guarded_io_review.md](guarded_io_review.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/hse_protocol.md](hse_protocol.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/implementation_plan.md](implementation_plan.en.md) | **Updated** - Rewritten baseline, roadmap/backlog |
| [docs/inline_address_editing.md](inline_address_editing.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/ioname_annotations.md](ioname_annotations.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/json_persistence_safety.md](json_persistence_safety.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/licensing.md](licensing.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/oem_5_opcode_analysis.md](oem_5_opcode_analysis.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/post_end_data.md](post_end_data.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/project_status.md](project_status.en.md) | **Updated** - Rewritten baseline, roadmap/backlog |
| [docs/roadmap.md](roadmap.en.md) | **Updated** - Rewritten baseline, roadmap/backlog |
| [docs/semantic_ast.md](semantic_ast.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/semantic_verification_matrix.md](semantic_verification_matrix.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/source_registry.md](source_registry.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/typed_memory_model.md](typed_memory_model.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/typed_opcode_grammar.md](typed_opcode_grammar.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/ui_multi_rung_state.md](ui_multi_rung_state.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/verification_gates.md](verification_gates.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |
| [docs/windows_acceptance_pr28.md](windows_acceptance_pr28.en.md) | **Updated** Status, references, risk/boundary or inspection requirements |

## 3. Historical specifications and release records: 15 MD

| File. | Audit result |
| --- | --- |
| [docs/documentation_audit_2026-11-17.md](documentation_audit_2026-11-17.en.md) | **Without change, historical meaning is preserved. |
| [docs/implementation_plan_v2_archive.md](implementation_plan_v2_archive.en.md) | **Created ** Archive of the previous version |
| [docs/ladder_spec.md](ladder_spec.en.md) | **Without change, historical meaning is preserved. |
| [docs/memory_map.md](memory_map.en.md) | **Without change, historical meaning is preserved. |
| [docs/references.md](references.en.md) | **Without change, historical meaning is preserved. |
| [docs/releases/v0.5.0-rc.1-hardware.md](releases/v0.5.0-rc.1-hardware.en.md) | **Without change, historical meaning is preserved. |
| [docs/releases/v0.5.0-rc.2-cioprg.md](releases/v0.5.0-rc.2-cioprg.en.md) | **Without change, historical meaning is preserved. |
| [docs/releases/v0.5.0-rc.3-workspace.md](releases/v0.5.0-rc.3-workspace.en.md) | **Without change, historical meaning is preserved. |
| [docs/releases/v0.5.0-rc.4-annotations.md](releases/v0.5.0-rc.4-annotations.en.md) | **Without change, historical meaning is preserved. |
| [docs/releases/v0.5.0-rc.5-layout.md](releases/v0.5.0-rc.5-layout.en.md) | **Without change, historical meaning is preserved. |
| [docs/releases/v0.5.0-rc.6-postend.md](releases/v0.5.0-rc.6-postend.en.md) | **Without change, historical meaning is preserved. |
| [docs/releases/v0.5.0-rc.7-typed.md](releases/v0.5.0-rc.7-typed.en.md) | **Without change, historical meaning is preserved. |
| [docs/releases/v0.5.0-rc.8-evidence.md](releases/v0.5.0-rc.8-evidence.en.md) | **Without change, historical meaning is preserved. |
| [docs/releases/v0.5.0-rc.9-oem-reference.md](releases/v0.5.0-rc.9-oem-reference.en.md) | **Without change, historical meaning is preserved. |
| [docs/roadmap_v2_archive.md](roadmap_v2_archive.en.md) | **Created ** Archive of the previous version |

## 4. Stable regulatory document: 1 MD

| File. | Audit result |
| --- | --- |
| [CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.en.md) | **Without change, historical meaning is preserved. |

## 5. Rules for the next updates

- One point of truth: [project_status](project_status.en.md) (Real SHA/CI and release) [roadmap](roadmap.en.md) (milestones) [implementation_plan](implementation_plan.en.md) (PR scope), [verification_gates](verification_gates.en.md) (reception) [engineering_decisions](engineering_decisions.en.md) (approved).
- Any feature spec in Section 2 only describes its support area and cannot automatically claim full OEM grammar, real scan or write rights.
- Release notes/historical audit should not be rewritten retroactively. Their Navigational References to the Old `main` dated to the relevant historical check; for a new state, open a link to `project_status`.
- After merge next PR fix the new main SHA and CI; after manual Windows test separately update `UserSmoke`without replacing it automatically. `WindowsCI`.
- No PR is merged and no release is published without a separate decision by the owner.

[Catalog](README.en.md) · [Decisions](engineering_decisions.en.md) · [Further development plan](implementation_plan.en.md).
