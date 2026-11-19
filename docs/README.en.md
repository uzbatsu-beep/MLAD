<!-- MLAD-I18N:EDITORIAL:reviewed 2026-11-19 against docs/README.md -->
> **Language / Язык:** [English](README.en.md) · [Русский](README.ru.md) · [Original](README.md)
>
> **Translation policy:** source documents and OEM manuals remain authoritative. Machine-generated companions that have not been editorially reviewed must not be treated as validated engineering or safety specifications.

# MLAD — Technical Documentation Index

**Verified baseline (18 November 2026):** `main` at `f85ddfcce3063dd087425be981a9d2b7a16d6309` (PR #1–#29 merged). [Linux CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454), [Windows CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621). The latest [published Windows preview, rc.9](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference), **does not include PR #24–#29**. Creating PR #30 did not by itself merge it into `main`; consult [project status](project_status.en.md) for an up-to-date baseline.

## Start here

| Purpose | Document |
| --- | --- |
| Project scope and quick start | [Repository README](../README.en.md) |
| Verified features, risks, and maturity | [Project status](project_status.en.md) |
| Roadmap v3: M0–M13, priorities, dependencies | [Roadmap](roadmap.en.md) |
| Proposed implementation sequence PR #30–#37 | [Implementation plan](implementation_plan.en.md) |
| CI, Windows acceptance, testing, and release gates | [Verification gates](verification_gates.en.md) |
| PR #28: Windows Acceptance & Security Regression | [Windows acceptance protocol](windows_acceptance_pr28.en.md) |
| PR #29: Filesystem Security Hardening and residual risks | [Filesystem threat review](filesystem_security_pr29.en.md) |
| Approved decisions D1–D9 | [Engineering decisions](engineering_decisions.en.md) |
| Components, data flow, and permissions | [Architecture](architecture.en.md) |
| Development, CI and release workflow | [Development workflow](development_workflow.en.md) |
| Safe persistence of independent JSON drafts (PR #24) | [JSON persistence safety](json_persistence_safety.en.md) |
| CSP and ancestor-path checks (PR #25) | [CSP/path review](csp_path_hardening.en.md) |
| Open-handle identity and save-conflict guards (PR #26) | [Guarded I/O review](guarded_io_review.en.md) |
| Contributor instructions | [CONTRIBUTING](../CONTRIBUTING.en.md) |
| GitHub community and issue templates | [CONTRIBUTING](../CONTRIBUTING.en.md) / [Issue templates](../.github/ISSUE_TEMPLATE) |
| Sensitive vulnerability reporting | [SECURITY](../SECURITY.en.md) |
| Expected community conduct | [CODE_OF_CONDUCT](../CODE_OF_CONDUCT.en.md) |
| Accessibility limitations | [ACCESSIBILITY](../ACCESSIBILITY.en.md) |
| Pull-request review template | [PR template](../.github/PULL_REQUEST_TEMPLATE.en.md) |
| GPL licensing and third-party material | [Licensing](licensing.en.md) |
| Complete Markdown review inventory after PR #29 | [Documentation inventory](documentation_inventory_2026-11-18.en.md) |
| Historical documentation audit dated 17 November 2026 | [Historical audit](documentation_audit_2026-11-17.en.md) |
| Language availability and editorial-review status | [Translation coverage](translation_coverage.md) |

## Native source files, memory addressing and semantics

| Topic | Document |
| --- | --- |
| Manufacturer manuals and source revisions | [Yaskawa source registry](source_registry.en.md) |
| Known and unknown native CIOPRG grammar | [CIOPRG.LST specification baseline](cioprg_lst_spec.en.md) |
| Exact-byte, lossless file inspection | [Lossless parser](cioprg_lossless_parser.en.md) |
| ALARM/MESSAGE records after `END` | [Post-END data](post_end_data.en.md) |
| Private controller fixture intake and confidentiality | [Fixture intake](controller_fixture_intake.en.md) |
| DX100 / DX200 / YRC1000 memory profiles | [Typed memory model](typed_memory_model.en.md) |
| Boolean source-to-AST analysis | [Semantic AST](semantic_ast.en.md) |
| Typed MOV/ADD/SUB/SHL/SHR candidates | [Typed opcode grammar](typed_opcode_grammar.en.md) |
| OEM-backed reference rules and unresolved cases for five instructions | [Five-opcode analysis](oem_5_opcode_analysis.en.md) |
| Tentative static M-register references and evidence boundaries | [Semantic verification matrix](semantic_verification_matrix.en.md) |
| TMR/CNT/GSTR/GOUT | [Advanced blocks](advanced_blocks.en.md) |
| **Not-yet-executed** controller bench test plan | [Hardware test matrix](advanced_hardware_test_matrix.en.md) |
| HSES: **not enabled** | [HSES protocol verification status](hse_protocol.en.md) |

## Interface and user-owned data

- [Graph editor](graph_editor.en.md) — independent editable JSON drafts; **not** a native CIOPRG exporter.
- [Multi-rung visualization](ui_multi_rung_state.en.md) — offline TRUE/FALSE/UNKNOWN states; **not live telemetry**.
- [Inline address editing](inline_address_editing.en.md) — address changes in JSON drafts; native file remains read-only.
- [IONAME and annotations](ioname_annotations.en.md) — display names and local notes without changing the source file.

## Release notes: historical, release-specific records

The files below describe only the corresponding **tag/commit**. They must not be retroactively rewritten as descriptions of the current `main`.

| Release | Scope | Document |
| --- | --- | --- |
| rc.1 | Initial Windows hardware-bench preview | [rc.1](releases/v0.5.0-rc.1-hardware.en.md) |
| rc.2 | Read-only native CIOPRG inspection | [rc.2](releases/v0.5.0-rc.2-cioprg.en.md) |
| rc.3 | Embedded Native Workspace | [rc.3](releases/v0.5.0-rc.3-workspace.en.md) |
| rc.4 | IONAME and local annotations | [rc.4](releases/v0.5.0-rc.4-annotations.en.md) |
| rc.5 | Compact workspace and system file dialogs | [rc.5](releases/v0.5.0-rc.5-layout.en.md) |
| rc.6 | Post-END classification and grouped diagnostics | [rc.6](releases/v0.5.0-rc.6-postend.en.md) |
| rc.7 | Typed candidates for five opcodes | [rc.7](releases/v0.5.0-rc.7-typed.en.md) |
| rc.8 | Tentative static source dependencies | [rc.8](releases/v0.5.0-rc.8-evidence.en.md) |
| **rc.9** | **OEM reference rules for five opcodes — latest published release** | [rc.9](releases/v0.5.0-rc.9-oem-reference.en.md) |

The [latest published GitHub Release](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference) contains source code corresponding to its own release commit, **not subsequent PRs in `main`**.

## Historical materials and redirects

- [Roadmap v2 archive](roadmap_v2_archive.en.md) — previous roadmap with statuses that have since changed.
- [Previous PR #27–#35 plan](implementation_plan_v2_archive.en.md) — archived planning sequence, superseded by roadmap v3.
- [Documentation audit, 17 November 2026](documentation_audit_2026-11-17.en.md) — historical snapshot.
- [Early ladder specification](ladder_spec.en.md) → [current CIOPRG baseline](cioprg_lst_spec.en.md).
- [Early memory map](memory_map.en.md) → [typed memory model](typed_memory_model.en.md).
- [Early references](references.en.md) → [current source registry](source_registry.en.md).

## Markdown maintenance policy

The authoritative current-status documents are this index, `project_status.md`, `roadmap.md`, `implementation_plan.md`, `verification_gates.md`, `engineering_decisions.md`, `SECURITY.md` and `architecture.md`. Technical specification documents are authoritative **only within their explicitly stated evidence scope**. An old PR number in a title denotes the origin of a feature, not the project's latest state.

Historical release notes and dated audits are preserved, including their original context. Following a new merge, update relevant status-bearing documents and the translation counterparts without altering historical tag claims. See the [Markdown inventory](documentation_inventory_2026-11-18.en.md) and the [translation coverage report](translation_coverage.md).

## Evidence discipline

`Documented` / `ManualDerived` is **not** the same as `SyntheticTested`, `OEMBenchVerified`, or `Authorized`. A read-only GUI does **not** imply permission to export native code or upload it to a robot. Each new capability requires a traceable manufacturer source, negative tests, controller-family scope, safety assessment and a current, explicit evidence status.
