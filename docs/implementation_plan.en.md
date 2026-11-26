<!-- Generated en companion; machine-generated draft; editorial review required. -->
> **English** · [Русский](implementation_plan.ru.md) · [English](implementation_plan.en.md) · [Original / Оригинал](implementation_plan.md)
> **Translation review:** AI-assisted; compare with Russian source and OEM evidence.

# MLAD — implementation plan v5

Strategy C approved 26 November 2026. Stable IDs V5-01…15; indicative PR numbers, not reserved. Historical v4: [baseline](https://github.com/uzbatsu-beep/MLAD/blob/e291d414f2440e2e407c1e0fdadc248de3d9dc80/docs/implementation_plan.md).

`CIOPRG.LST` and `IONAME.DAT` remain read-only. `HSES`/`FTP`, native writer and deployment remain `BLOCKED`; `UserSmoke=NOT RUN`, `OEMBench=NOT RUN` until independent evidence. JSON drafts are not controller programs. Public fixtures are synthetic or explicitly authorized; no private production files or hashes.

## 1. Backlog

| ID | PR | Task | Deliverable |
| --- | --- | --- | --- |
| V5-01 | #50 | Roadmap v5 & Architecture Decisions | Approved plan, ADRs, docs; no runtime. |
| V5-02 | #51 | Semantic Core v2 | Inventory, AST/IR, source spans, contracts, no duplicate parser. |
| V5-03 | #52 | Controller Profiles v2 | Dialects, firmware, address spaces, evidence. |
| V5-04 | #53 | Signal Index | Address keys, READ/WRITE/UNKNOWN, source references, snapshot identity. |
| V5-05 | #54 | UI Foundation | Design tokens, themes, palettes and persisted settings. |
| V5-06 | #55 | Engineering Workspace | IDE shell, layout registry, panels, tabs, layout switching and persistence. |
| V5-07 | #56 | Ladder Viewer v3 | Virtualized multi-rung canvas, zoom, blocks, UNKNOWN, navigation. |
| V5-08 | #57 | Signal Inspector | Shared-index inspector, G2 Windows preview subject to separate approval. |
| V5-09 | #58 | Cross Reference v2 | Role filters and bidirectional navigation. |
| V5-10 | #59 | Dependency Graph | Potential dependencies with provenance and unknown barriers. |
| V5-11 | #60 | Diagnostics Engine | Extensible rules, severity, explainable results. |
| V5-12 | #61 | Logic Tracing | Forward/backward, cycles, unknowns, explanations, G3. |
| V5-13 | #62 | Project Compare v1 | Byte/text and structural diff, ambiguous matches. |
| V5-14 | #63 | Project Snapshots | History, baselines, integrity, portability. |
| V5-15 | #64 | Engineering Preview | Integration tests, G4, conditional Windows preview and UserSmoke. |

## 2. Acceptance gates

| Gate | Acceptance |
| --- | --- |
| G1 | Existing functions retained; shared source-linked core/profile/index, no second parser. |
| G2 | Working Windows UI, restored themes/layouts, multi-rung navigation, inspector and unknown; independent smoke. |
| G3 | Cross-reference and trace show source links, potential dependencies, unknown barriers, tests for cycles and multiple writers. |
| G4 | Compare/snapshots do not mutate sources; integration tests and manual acceptance of separately approved preview. |


## 3. Workspace contract

V5-05 introduces design system, V5-06 typed layout registry and shared project session. UI preferences separate from project data. V5-07/08 provide working Ladder/Inspector. Other layouts appear with real modules. Test switching/restart/reset, unsaved draft, focus, absent module, stale snapshot, invalid settings. UI does not change permissions.

## 4. Later phases

After G4 refine PRs using feedback: M17 tri-state → memory/step → time/TMR/CNT → registers → scenarios; M18 watch/trace/events/test with INCONCLUSIVE; M19 reports; M20 local fleet; M21 optional AI. Research and parser work in parallel. This does not authorize online features.

## 5. Definition of Done

Every PR: scope, positive/negative tests, regression, source preservation, explicit unknown, applicable CI on final SHA, RU/EN docs, limitations, separate merge consent. Existing docs-only CI; skipped runtime is not PASS. GUI smoke and OEM bench independent. Release, Ruleset and controller access require separate decisions.

[Roadmap](roadmap.en.md) · [Architecture](architecture.en.md) · [Verification gates](verification_gates.en.md).
