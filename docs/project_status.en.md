<!-- Generated en companion for docs/project_status.md; review required. -->
> **English** · [Русский](project_status.ru.md) · [English](project_status.en.md) · [Original / Оригинал](project_status.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.

# MLAD – the actual status of the project

**Reconciliation:** November 18, 2026. **Baseline:** GitHub `main` @ `f85ddfcce3063dd087425be981a9d2b7a16d6309`, PR **#1–#29 merged**. [Main Linux validation SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454) (`frontend-build`, `rust-tests`), [Main Windows Acceptance & Security Regression SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621) (`windows-security`) on the specified SHA. [PR #29](https://github.com/uzbatsu-beep/MLAD/pull/29) merged, his [Linux](https://github.com/uzbatsu-beep/MLAD/actions/runs/44369892563) и [Windows](https://github.com/uzbatsu-beep/MLAD/actions/runs/44369892566) CI is also SUCCESS.

**Last published Windows Release:** [v0.5.0-rc.9-oem-reference](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference)17.11.2026. Release contains published artifacts only of its tag; **PR #24-#29 doesn't fit in. CI build after PR #29 It forms a temporary **unsigned non-release** EXE, but it's not a new GitHub Release. The version in the application manifests for now `0.5.0`; custom Windows/WebView2 `UserSmoke=NOT RUN`, OEM bench `NOT RUN`.

> **Region:** This document describes the proven GitHub baseline, not the version installed on the user's computer. The status will change after new merge or release; do not assign new code to old release assets. Details: [roadmap v3](roadmap.en.md), [plan PR #30+](implementation_plan.en.md), [verification gates](verification_gates.en.md).

## 1. Product boundary

MLAD - **OFFLINE/BENCH** engineering prototype for Yaskawa Concurrent I/O analysis`DX100`, `DX200`, `YRC1000`). Designed to view exported `CIOPRG.LST` In the main working window, source/ladder/advanced diagnostics and work with **separate** Graph/Advanced/legacy JSON projects. Original. `CIOPRG.LST` и `IONAME.DAT` strictly read-only; no native writer, no verified hardware scan emulator, FTP/HSES No runtime. Historical prototype net materials are not considered available functionality. `PART 1/2` - labels, unconfirmed System/User rights model.

## 2. Possibilities that are present in the main

| Direction | What is | Important border | Primary PR |
| --- | --- | --- | --- |
| Address typing | Controller-family relay classes, M-registers, typed syntax checks | Unverified firmware-specific recording permissions | #1, #19 |
| Lossless native inspection | Bounded exact bytes, SHA-256, source spans, BOM/newline preservation, opaque tail | Not full OEM grammar/compiler | #2, #14, #18 |
| Native Workspace | System Open, wood, source paging/search, recognized rungs diagrams, compact blocks | Only a proven supported subset is visually displayed | #15–#18 |
| `IONAME.DAT` | Read-only table/preview + local annotations/manual associations | No Proven Automatic OEM Mapping | #16 |
| Graph JSON editor | `*.graph.json`, input validation, diagram/Undo/Redo, save/load | Not CIOPRG compiler | #4, #7–#8 |
| Advanced JSON | TMR/CNT/GSTR/GOUT cards, `*.advanced.json`, isolated test-model helpers | Non-execution of the controller | #5, #8 |
| Typed operations | MOV/ADD/SUB/SHL/SHR candidates и evidence-derived reference vectors | Not complete EN/scan/dataflow emulator | #19–#21 |
| Static analysis | Boolean AST, grouped diagnostics, static M-links | Hypotheses, not OEM confirmed dataflow | #3, #18–#21 |
| Windows build | GitHub Actions, Tauri x64 unsigned EXE, temporary SHA evidence | CI ное manual application WebView2 | #6, #28 |
| JSON persistence | Same-directory staging, `*.bak`, guarded reads, detect byte+File ID swap, reject hardlinks, staged SHA check | Final check→rename, ACL and backup transaction remain open | #24–#26, #29 |
| Tauri CSP/filesystem | Production CSP, no remote script/eval; Windows reparse/ADS/UNC/device paths denied; native source read-only | CSP runtime/manual tests and absolute filesystem TOCTOU not proven | #25–#26, #29 |
| Git/legal/community | GPL-3.0-only, `package-lock.json` + `npm ci`, `Cargo.lock`, tests, SECURITY/CONTRIBUTING | SBOM/notices/legal review and Ruleset remain | #11–#12, #23, #27 |

## 3. Independent evidentiary statuses

| Gate | Evidence | Status |
| --- | --- | --- |
| `Implementation` | Unit/regression tests in main #1-#29 | **PARTIAL** with respect to the full IDE |
| `SyntheticTested` | Rust/Node/TS suites, including Windows junction/lock/File ID/hardlink/ADS | **PASS** for relevant verified scenarios |
| `WindowsCI` | [Main Windows workflow](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621) на `f85ddfc` | **PASS** |
| `UserSmoke` | Manual Windows 10/11 WebView2 protocol WIN-01…WIN-12 | **NOT RUN** (no confirmed report) |
| `ManualDerived` | OEM-reference subset MOV/ADD/SUB/SHL/SHR | **PARTIAL**; independent scan not implemented |
| `OEMBenchVerified` | DX100/DX200/YRC1000 Controller/firmware/result records | **NOT RUN** |
| `Authorized native writer` | No permission protocol or full OEM grammar | **BLOCKED** |
| `Authorized FTP/HSES/deployment` | There is no separate OEM/safety/owner tolerance. | **BLOCKED** |

No green GitHub Actions vetting is cancelled `NOT RUN` or `BLOCKED` on other levels. Manual Windows test is available at [Windows acceptance](windows_acceptance_pr28.en.md)and bench tests in [hardware matrix](advanced_hardware_test_matrix.en.md).

## 4. Open issues and risks

1. **Native parser:** Unknown/unsupported commands, options `//$M`, `PART/END`, post-END sections and a range of formats; synthetic corpus and independent private fixture provenance are required. There is no proven full Native CIO grammar yet.
2. **UX:** source navigation is implemented at the basic level, but cross-links sourcerrung/block/diagnostics, large table of unknown operations and portable project workflow require the following PR.
3. Semantics:** typed candidates and OEM reference do not mean actual EN/scan flags, live I/O or timing. The TMR/CNT/GSTR/GOUT instructions and the MOV/ADD/SUB/SHL/SHR operations in conjunction require separate OEM verification.
4. **IONAME:** Local annotations may disappear when a profile is reset; there is no confirmed OEM map or portable sidecar.
5. **Filesystem:** PR #29 Rejects hardlinks/ADS/UNC/device, detects swap by File ID and staged corruption. **Not guaranteed** atomic backup+target, ACL/owner persistence, no-race parent directory, last check→rename and crash durability. [Precise threat review](filesystem_security_pr29.en.md).
6. **Security supply chain:** `suppaftp` still present in `src-tauri/Cargo.toml` As a dependency, although the controller runtime network module is not compiled and the deployment command deny-only. You need an independent dependency cleanup/review, without adding network functionality.
7. **Windows:** There is no confirmed manual smoke WebView2Portable/installer of the new `main` The release was not published, the binary unsigned.
8. **Law/governance:** GPL-3.0-only The authorship of the original materials is fixed; third-party notices/SBOM and license provenance are not completed. `main` reconciliation `protected=false` Ruleset/Branch Protection is set up separately by the owner.

## 5. Progress of stages M0-M13

| Phase | Status | What next? |
| --- | --- | --- |
| M0 – basic infrastructure | **DONE:** #1–#29 | Audit of documentation PR #30, GitHub Ruleset |
| M1 - Real fixtures | **PARTIAL** | Synthetic corpus, private OEM provenance |
| M2 - Safety | **PARTIAL** | Windows UserSmoke, residual filesystem review/ACL |
| M3 — native workspace | **DONE read-only base** | Moving to M4 UX |
| M4 — source navigation | **PARTIAL** | Cross-navigation/large-file/filter PR #32 |
| M5 — typed parser | **PARTIAL** | Evidence-driven grammar PR #34 |
| M6 — static semantics | **PARTIAL** | Source-indexed dataflow PR #35 |
| M7–M8 — unified offline project | **OPEN** | Sidecar #33 + integrated UX #36 |
| M9 — scan simulation | **OPEN / NOT VERIFIED** | Only after OEM scan evidence |
| M10 — OEM bench | **NOT RUN** | Isolated approved stand |
| M11 — read-only HSES | **BLOCKED** | Separate decision of the owner, protocol evidence |
| M12 — native writer | **BLOCKED** | Full round-trip, OEM+owner approval |
| M13 — deployment | **BLOCKED** | Additional security and authorization procedure |

## 6. Current development phase

Documentation **#30**, synthetic parser corpus **#31**, Native Workspace navigation **#32**, portable sidecar **#33**, evidence-based grammar **#34**, static cross-references **#35**, unified Offline IDE **#36**, conditional Windows preview **#37**. This is a project sequence, not a guarantee of PR numbers or deadlines. Undefined security defects can block the next stage. [Detailed backlog](implementation_plan.en.md).

## 7. References for current control

[Roadmap](roadmap.en.md) · [Implementation plan](implementation_plan.en.md) · [Engineering decisions](engineering_decisions.en.md) · [Verification gates](verification_gates.en.md) · [Security review](filesystem_security_pr29.en.md) · [Windows acceptance](windows_acceptance_pr28.en.md) · [OEM source registry](source_registry.en.md) · [Licensing](licensing.en.md).

Historical release notes rc.1-rc.9, [roadmap v2 archive](roadmap_v2_archive.en.md), [old plan](implementation_plan_v2_archive.en.md)old PR-specific evidence and [audit from 17. 11.](documentation_audit_2026-11-17.en.md) **Not rewritten as current status**.
