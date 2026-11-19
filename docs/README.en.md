<!-- Generated en companion for docs/README.md; review required. -->
> **English** · [Русский](README.ru.md) · [English](README.en.md) · [Original / Оригинал](README.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.
> **MLAD safety invariants:** `CIOPRG.LST` and `IONAME.DAT` are strictly **read-only**. FTP/HSES, live I/O, native writer and robot deployment remain **BLOCKED**. Passing `WindowsCI` does **not** imply `UserSmoke=PASS` or `OEMBenchVerified`: both remain **NOT RUN** unless independently documented. Machine translation requires source and Yaskawa manual review.

# MLAD - catalogue of technical documentation

** Verified 18.11.2026:** `main` @ `f85ddfcce3063dd087425be981a9d2b7a16d6309` (PR #1–#29 merged), [Linux CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454), [Windows CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621). Last published [rc.9](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference) contains PR #24-#29. Publication of documentation PR #30 does not mean merge; [status](project_status.en.md).

## Where to start?

| What do you need? | Document |
| --- | --- |
| Appointment and quick start | [README repository](../README.en.md) |
| Full Verified Functionality, Risks, Progress | [project_status.md](project_status.en.md) |
| Current roadmap v3:** M0–M13, priorities, dependencies | [roadmap.md](roadmap.en.md) |
| ** Development plan PR #30-#37:** Volume, checks, sequence | [implementation_plan.md](implementation_plan.en.md) |
| Matrix CI, Windows smoke, checks and releases | [verification_gates.md](verification_gates.en.md) |
| Windows Acceptance & Security Regression (PR #28 merged) | [windows_acceptance_pr28.md](windows_acceptance_pr28.en.md) |
| Filesystem Security Hardening and Residual ThreatsPR #29 merged) | [filesystem_security_pr29.md](filesystem_security_pr29.en.md) |
| Consistent Engineering Solutions D1–D9 | [engineering_decisions.md](engineering_decisions.en.md) |
| Access components and permissions | [architecture.md](architecture.en.md) |
| Development process, CI, Windows Preview | [development_workflow.md](development_workflow.en.md) |
| Safe preservation of JSON (PR #24 merged) | [json_persistence_safety.md](json_persistence_safety.en.md) |
| CSP / Parental File PathsPR #25 merged) | [csp_path_hardening.md](csp_path_hardening.en.md) |
| Open file identity and save conflictsPR #26 merged) | [guarded_io_review.md](guarded_io_review.en.md) |
| Deposit rules | [CONTRIBUTING.md](../CONTRIBUTING.en.md) |
| Community and Issues templates | [CONTRIBUTING.md](../CONTRIBUTING.en.md) / [Issue templates](../.github/ISSUE_TEMPLATE) |
| Responsible reporting of vulnerabilities | [SECURITY.md](../SECURITY.en.md) |
| Code of engagement | [CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.en.md) |
| Interface availability and limitations | [ACCESSIBILITY.md](../ACCESSIBILITY.en.md) |
| PR template with MLAD restrictions | [PULL_REQUEST_TEMPLATE.md](../.github/PULL_REQUEST_TEMPLATE.en.md) |
| License and third-party components | [licensing.md](licensing.en.md) |
| Registry of all Markdown audits after PR #29 | [documentation_inventory_2026-11-18.md](documentation_inventory_2026-11-18.en.md) |
| Historical audit of documentation on 17.11. 2026 | [documentation_audit_2026-11-17.md](documentation_audit_2026-11-17.en.md) |

## Source files, addresses and semantics

| Direction | Document |
| --- | --- |
| Primary sources/revisions of Yaskawa | [source_registry.md](source_registry.en.md) |
| Native CIOPRG: Known and Unknown | [cioprg_lst_spec.md](cioprg_lst_spec.en.md) |
| Byte-saving imports | [cioprg_lossless_parser.md](cioprg_lossless_parser.en.md) |
| Post-END ALARM/MESSAGE | [post_end_data.md](post_end_data.en.md) |
| Control body structure, source protection | [controller_fixture_intake.md](controller_fixture_intake.en.md) |
| DX100/DX200/YRC1000 memory profiles | [typed_memory_model.md](typed_memory_model.en.md) |
| Boolean Semantic AST | [semantic_ast.md](semantic_ast.en.md) |
| Typed MOV/ADD/SUB/SHL/SHR | [typed_opcode_grammar.md](typed_opcode_grammar.en.md) |
| Checked documentation for five teams and unknown edges | [oem_5_opcode_analysis.md](oem_5_opcode_analysis.en.md) |
| Static Hypotheses of M Dependencies, OEM Assumptions | [semantic_verification_matrix.md](semantic_verification_matrix.en.md) |
| TMR/CNT/GSTR/GOUT | [advanced_blocks.md](advanced_blocks.en.md) |
| Matrix of **unconducted ** robot tests | [advanced_hardware_test_matrix.md](advanced_hardware_test_matrix.en.md) |
| HSES **not on** | [hse_protocol.md](hse_protocol.en.md) |

## Interface and user data

- [graph_editor.md](graph_editor.en.md) Safe JSON drafts; not native export.
- [ui_multi_rung_state.md](ui_multi_rung_state.en.md) Visual chains and offline tri-state.
- [inline_address_editing.md](inline_address_editing.en.md) Edit JSON addresses and local notes separately.
- [ioname_annotations.md](ioname_annotations.en.md) I/O names and local comments without changing native files.

## Release notes (archival documents of specific assemblies)

All release notes below refer to your tag/commit** only. They are not modified retroactively so as not to distort the reproducible information about the previously published artifact.

| Release. | Feature | File. |
| --- | --- | --- |
| rc.1 | Windows hardware bench preview | [rc.1](releases/v0.5.0-rc.1-hardware.en.md) |
| rc.2 | Native read-only CIOPRG | [rc.2](releases/v0.5.0-rc.2-cioprg.en.md) |
| rc.3 | Built-in Native Workspace | [rc.3](releases/v0.5.0-rc.3-workspace.en.md) |
| rc.4 | IONAME / Local comments | [rc.4](releases/v0.5.0-rc.4-annotations.en.md) |
| rc.5 | Compact panel and system dialogues | [rc.5](releases/v0.5.0-rc.5-layout.en.md) |
| rc.6 | Post-END and Diagnostic Grouping | [rc.6](releases/v0.5.0-rc.6-postend.en.md) |
| rc.7 | Type five teams | [rc.7](releases/v0.5.0-rc.7-typed.en.md) |
| rc.8 | Static Source Hypotheses | [rc.8](releases/v0.5.0-rc.8-evidence.en.md) |
| **rc.9** | **OEM reference for five teams - most recently published ** | [rc.9](releases/v0.5.0-rc.9-oem-reference.en.md) |

Current [GitHub Release](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference)The published source archive corresponds to the release commit, not future changes n `main`.

## Historical materials and redirections

- [roadmap_v2_archive.md](roadmap_v2_archive.en.md) ** Archive of the previous detailed roadmap, it has outdated draft statuses.
- [implementation_plan_v2_archive.md](implementation_plan_v2_archive.en.md) - **archive** of the previous turn PR #27#35; current numbers reassigned after documentary PR #30.
- [documentation_audit_2026-11-17.md](documentation_audit_2026-11-17.en.md) Historical audit at the date of creation.


- [ladder_spec.md](ladder_spec.en.md) → [cioprg_lst_spec.md](cioprg_lst_spec.en.md).
- [memory_map.md](memory_map.en.md) → [typed_memory_model.md](typed_memory_model.en.md).
- [references.md](references.en.md) → [source_registry.md](source_registry.en.md).

## Markdown Update Policy

- **Acting sources of truth:** This index, `project_status.md`, `roadmap.md`, `implementation_plan.md`, `verification_gates.md`, `engineering_decisions.md`, `SECURITY.md`, `architecture.md`.
- ** Function specifications** (CIOPRG, memory, parser, graph, advanced, HSES) remain valid ** only within the limits described **; old `PR #21` The title stands for provenance of implementation, not the last leaked PR project.
- **Historical records (rc.1–rc.9, archival decisions, dated audits) should not be rewritten retroactively or taken by SHA/CI as current state.
- Review of all Markdown coverage [documentation_inventory_2026-11-18.md](documentation_inventory_2026-11-18.en.md). After the new merge, repeat the revision of only modified / status files, saving the archives.

## Discipline of evidence

Exact differences: `Documented/ManualDerived` ≠ `SyntheticTested` ≠ `OEMBenchVerified` ≠ `Authorized`. Read-only GUI . native export or download to the robot. Each new feature requires an original OEM source, negative case tests, ControllerFamily binding, security script, and status updates without erasing history.
