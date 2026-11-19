<!-- Generated en companion for docs/documentation_audit_2026-11-17.md; review required. -->
> **English** · [Русский](documentation_audit_2026-11-17.ru.md) · [English](documentation_audit_2026-11-17.en.md) · [Original / Оригинал](documentation_audit_2026-11-17.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.
> **MLAD safety invariants:** `CIOPRG.LST` and `IONAME.DAT` are strictly **read-only**. FTP/HSES, live I/O, native writer and robot deployment remain **BLOCKED**. Passing `WindowsCI` does **not** imply `UserSmoke=PASS` or `OEMBenchVerified`: both remain **NOT RUN** unless independently documented. Machine translation requires source and Yaskawa manual review.

# Results of MLAD Documentation Audit – 17 November 2026

> **Historical document (navigation mark 18.11.2026):** The following conclusions preserve the date of the initial verification, and do not replace the current implementation. PR #1#21 merged, Windows rc.9 published. [Current status of MLAD](project_status.en.md).

**Basic:** `main` merge commit `37737b384fe5bbd3f2ef97874357947727f1cdd9`, PR #8 merged 17.11.2026; verification of GitHub Actions `main` - **success. The audit relates to documentation and current source materials, **not** is the conformity assessment of Yaskawa products.

## Volume.

Checked ** All 16 Markdown files available in `main` before the audit:

| Category | Files. |
|---|---|
| Main entrance | `README.md` |
| Format and parser | `docs/cioprg_lst_spec.md`, `docs/cioprg_lossless_parser.md`, `docs/semantic_ast.md` |
| Memory and sources | `docs/typed_memory_model.md`, `docs/source_registry.md` |
| Editors and functional blocks | `docs/graph_editor.md`, `docs/ui_multi_rung_state.md`, `docs/inline_address_editing.md`, `docs/advanced_blocks.md` |
| Stand | `docs/advanced_hardware_test_matrix.md`, `docs/releases/v0.5.0-rc.1-hardware.md` |
| Network | `docs/hse_protocol.md` |
| Archival navigational | `docs/ladder_spec.md`, `docs/memory_map.md`, `docs/references.md` |

## Discovered and corrected in the documentation branch

1. **README:** Future compilation, monitoring, cross-platform and React Flow have been described as operating features. Reworked into a table implemented / deferred; updated stack and test commands.
2. **HSES:** The document presented an unverified packet diagram, commands and addressing as a working protocol. Now explicitly prohibits the use of historical examples and requires a vendor-confirmed read-only contract before enabling monitoring.
3. **Controller profiles:** Old documents claimed that the UI didn't yet allow you to choose DX100/DX200/YRC1000. The new Graph, Advanced and Inspector support family selection; legacy and firmware/option problems remain.
4. **`CIOPRG.LST`**Outdated statements about the current pseudo `/JOB` Exporter, semantics and editing gaps replaced by exact current status `lossless` / `semantic` / `advanced_source`. Prohibition of native-writeFTP It persists.
5. ** Markdown syntax:** Literal sequences eliminated `\\n\\n`, spoiled part of the line transfers.
6. ** Interface:** documents PR #4/#5/#7/#8 synchronized with the joint circuit field `TRUE/FALSE/UNKNOWN`, double-click/F2, separate C/R in CNT, eight-bit group and save only JSON.
7. **History:** confirmed that **PR #6** is a merged release of Windows RC, not a missing milestone. Release `v0.5.0-rc.1-hardware` separated `main` later PR #7/#8.
8. **Results of the test: ** Matrix refined. The repository does not have published measurements on real hardware, and a successful unit-test does not close hardware acceptance.

## Documents created

- [Catalog](README.en.md) и [architecture](architecture.en.md).
- [Development and release rules](development_workflow.en.md) + [CONTRIBUTING](../CONTRIBUTING.en.md).
- [Road map](roadmap.en.md).
- [Acceptance of the original CIOPRG](controller_fixture_intake.en.md).
- This document update report.

## Remaining risks: Documentation does not replace code correction

- In the original `src-tauri/src/network/mod.rs` **There is still an unverified network code** HSES/FTPAlthough current Tauri failure commands prevent usage from the current UI.
- Full native parser/writer, verified System/User Ladder boundaries, OEM golden fixtures and real scan-time execution are missing.
- JSON is not strictly atomic; symlink does not exclude all filesystem races.
- CSP, dependencies/npm lockfile and reproducibility releases require separate technical PR.
- There is no confirmation of complete HSES Read and deliver live states from controllers.
- Manual UI smoke test and real robot files are rated **separate** from CI.

**Recommended next technical step: **Golden fixtures with real life `CIOPRG.LST` and security hardening before creating a native exporter. See. [roadmap.md](roadmap.en.md).

## Consolidated verification

Documentation of changes does not modify `src-tauri/src/compiler/*`UI or `CIOPRG.LST`. CI adds a script to check for **internal Markdown links** and erroneous literal newline escapes; external URLs, manual technical truths, and hardware results are not verified by this script**.
