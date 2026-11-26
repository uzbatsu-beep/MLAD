<!-- MLAD-I18N:EDITORIAL:AI-assisted English draft, technical editorial review still required -->
> **English** · [Русский](roadmap.ru.md) · [English](roadmap.en.md) · [Original / Оригинал](roadmap.md)
> **Translation review:** AI-assisted; compare with Russian source and OEM evidence.

# MLAD — Roadmap v5.0: Engineering Workbench

Approved by the owner on 26 November 2026: all 11 directions, strategy C (balanced), shared signal model, functional previews, specialized layouts, independent offline 1.0. This approves direction, not PR merge.

Baseline: PR #49 merged, `main` `e291d414f2440e2e407c1e0fdadc248de3d9dc80`, [CI PR #49](https://github.com/uzbatsu-beep/MLAD/actions/runs/44906615882) SUCCESS. Last documented Windows pre-release `v0.5.0-rc.10-unified-offline`. Historical v4: [roadmap](https://github.com/uzbatsu-beep/MLAD/blob/e291d414f2440e2e407c1e0fdadc248de3d9dc80/docs/roadmap.md), [plan](https://github.com/uzbatsu-beep/MLAD/blob/e291d414f2440e2e407c1e0fdadc248de3d9dc80/docs/implementation_plan.md).

## 1. Product and boundaries

Independent offline environment for understanding, analyzing, diagnosing, comparing, modeling and maintaining Yaskawa Motoman Concurrent I/O. Audience: industrial automation, robotics, service, integration and education. DX100, DX200, YRC1000; retain Rust/Tauri/React/TypeScript. No expansion to generic PLC IDE, other brands, 3D simulation, mandatory server or cloud.

`CIOPRG.LST` and `IONAME.DAT` remain read-only. `HSES`/`FTP`, native writer and deployment remain `BLOCKED`; `UserSmoke=NOT RUN`, `OEMBench=NOT RUN` until independent evidence. JSON drafts are not controller programs. Public fixtures are synthetic or explicitly authorized; no private production files or hashes.

## 2. Milestones M0–M21

| ID | Milestone | Priority | Scope / acceptance target |
| --- | --- | --- | --- |
| M0 | Governance & Build | Continuous | CI, lockfiles, licenses; required ci-gate needs separate approval. |
| M1 | OEM Knowledge & Evidence | P0 | Private real fixtures, documentation, firmware, evidence per claim. |
| M2 | Filesystem & Project Reliability | Continuous | Immutable sources, backup/recovery; ACL, TOCTOU and multi-file atomicity open. |
| M3 | Engineering Workspace | P0 | IDE, panels, tabs, multi-rung Ladder, specialized layouts. |
| M4 | Controller Dialect Engine | P0 | DX100/DX200/YRC1000, lossless parser, per-profile coverage, explicit unknowns. |
| M5 | Navigation & Signal Index | P0 | Shared index, addresses, source spans, bidirectional navigation. |
| M6 | Static Analysis & Diagnostics | P0 | Typed operands, READ/WRITE/UNKNOWN, explainable rules. |
| M7 | Portable Sidecar v2 | P1 | Symbols, groups, comments and bookmarks, explicit rebinding on SHA change. |
| M8 | Unified Project Management | P1 | Projects, snapshots, relative paths, baselines and history, portability. |
| M9 | Windows User Acceptance | Continuous | Independent manual UserSmoke of each functional preview. |
| M10 | OEM Bench Verification | Parallel | Instruction, model, firmware, scenario and result; no unsupported generalization. |
| M11 | Read-only Online Monitoring | BLOCKED | Separate OEM/owner HSES/FTP authorization and protocol verification. |
| M12 | Native Writer & Compiler | BLOCKED | Applicable grammar, OEM round-trip, semantics and write permissions. |
| M13 | Robot Deployment | BLOCKED | Separate hardware and procedural authorization; M12 does not authorize deployment. |
| M14 | Signal Intelligence | P0 | Inspector, cross-reference, browser, groups, dictionary, multiple writers without false diagnoses. |
| M15 | Project Compare | P0 | Byte/text → structural → bounded semantic diff → impact/fleet; explicit ambiguity. |
| M16 | Logic Tracing | P0 | Forward/backward slicing, condition tree, cycles, Why TRUE/FALSE only in a defined model. |
| M17 | Virtual Laboratory | P1 | Tri-state, virtual memory, steps, time, TMR/CNT, registers and scenarios of supported subset. |
| M18 | Trace & Test | P1 | Watch table, traces, cursors, triggers, events, PASS/FAIL/INCONCLUSIVE. |
| M19 | Documentation Engine | P1 | Signal, Ladder, diagnostics, diff and test reports; CSV/HTML/PDF/SVG/JSON as appropriate. |
| M20 | Robot Fleet | P2 | Local plant–area–cell–robot–controller–project catalog, snapshots and knowledge. |
| M21 | Engineering AI | P2 | Optional local LLMs, explanations based on structures and OEM evidence, separate from deterministic outputs. |

M0–M13 retain history; M14–M21 expand functionality. Baseline implementation is partial, new functions planned; priority does not indicate readiness.

## 3. Balanced phases

| Phase | Milestones | Gate / target |
| --- | --- | --- |
| 00 | M0–M13 | Existing Offline Preview |
| 01 | M1/M4/M5/M6 | G1 Semantic Core |
| 02 | M3/M5/M14 | G2 Workspace Preview |
| 03 | M6/M14/M16 | G3 Diagnostics Preview |
| 04 | M7/M8/M15 | G4 Engineering Preview |
| 05 | M17 | Simulation Preview |
| 06 | M18 | Engineering Test Suite |
| 07 | M19/M20 | Offline Engineering Suite |
| 08 | M21 | AI-assisted Workbench |
| 09 | M11/M12/M13 | BLOCKED |

M0/M1/M2/M9/M10 in parallel. Release and OEM gates are independent. 1.0 does not depend on online engineering.

## 4. Specialized workspace layouts

| Workspace | Layout |
| --- | --- |
| Ladder Engineering | Tree, multi-rung canvas, inspector, source. |
| Signal Intelligence | Table, filters, inspector, cross-reference. |
| Diagnostics | Diagnostics, graph, condition tree, references. |
| Project Compare | Two snapshots, diff and change inspector. |
| Virtual Laboratory | Ladder, virtual I/O, scenarios, model controls. |
| Trace & Test | Watch, traces, events and test results. |
| Robot Fleet | Catalog, robot profile, snapshots. |
| Documentation | Report selection, preview, export. |

Shared project, snapshot and selection context. Switch without reopening sources or losing unsaved changes; separate size, visibility, tab and preset settings, restore defaults. Unimplemented modules explicitly unavailable. Layout is not a permission boundary.

| ID | Requirement |
| --- | --- |
| UI-01–03 | Light/dark themes, palettes, persisted settings. |
| UI-04–09 | Narrow navigation, left tree, right inspector, tabs, bottom diagnostics, resize/hide. |
| UI-10–12 | Virtualized multi-rung canvas, keyboard, RU/EN. |
| UI-13–18 | Saved layouts, split view, multiple projects, command palette, help, fullscreen. |
| UI-19–20 | Custom shortcuts, separate windows and monitors. |

TRUE green, FALSE red, UNKNOWN neutral, UNSUPPORTED amber; SELECTED independent accent, text/shape supplements color. Offline without defined scenario is UNKNOWN. All recognized rungs accessible on one canvas; TMR/CNT/GSTR/GOUT supported forms only; F2/double-click edits separate drafts.

## 5. Quality and evidence

Maturity L0 Planned, L1 Prototype, L2 Implemented, L3 Automated Tested, L4 User Accepted, L5 OEM Verified (applicable controller claims only). Independent evidence flags can combine: `Unknown`, `TypedCandidate`, `ManualDerived`, `RealFixtureObserved`, `SyntheticTested`, `OEMBenchVerified`. Real fixture observation does not prove execution.

Provisional targets: 10k/50k/100k physical lines; 100% classified lines including unknown and 100% preserved bytes; indexed search/navigation ≤150 ms; target 60 FPS. Measure baseline on representative PC first. Deterministic scenarios, no annotation loss on reopening.

## 6. Research and versions

R-01…R-11: real CIOPRG, IONAME, memory/firmware, OEM semantics, engineering IDEs, static analysis, diff, deterministic modeling, trace/test, HSES/FTP (research only), local LLMs. Convert findings into specs, fixtures and tasks.

Tentative versions: 0.5.x foundation; 0.6.x core/workspace; 0.7.x intelligence; 0.8.x compare/history; 0.9.x lab/test; 0.10.x documentation/fleet; 0.11.x AI; 1.0.0 stable offline. Separate consent for each release and merge.

[Implementation plan](implementation_plan.en.md) · [Architecture](architecture.en.md) · [Decisions](engineering_decisions.en.md) · [Verification gates](verification_gates.en.md).
