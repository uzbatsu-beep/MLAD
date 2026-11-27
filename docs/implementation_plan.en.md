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
| V5-03 | #53 | Controller Profiles v2 | Dialects, firmware, address spaces, evidence. |
| V5-04 | #54 | Signal Index | Address keys, READ/WRITE/UNKNOWN, source references, snapshot identity. |
| V5-05 | #55 | UI Foundation | Design tokens, themes, palettes and persisted settings. |
| V5-06 | #56 | Engineering Workspace | IDE shell, layout registry, panels, tabs, layout switching and persistence. |
| V5-07 | #57 | Ladder Viewer v3 | Virtualized multi-rung canvas, zoom, blocks, UNKNOWN, navigation. |
| V5-08 | #58 | Signal Inspector | Shared-index inspector, G2 Windows preview subject to separate approval. |
| V5-09 | #59 | Cross Reference v2 | Role filters and bidirectional navigation. |
| V5-10 | #60 | Dependency Graph | Potential dependencies with provenance and unknown barriers. |
| V5-11 | #61 | Diagnostics Engine | Extensible rules, severity, explainable results. |
| V5-12 | #62 | Logic Tracing | Forward/backward, cycles, unknowns, explanations, G3. |
| V5-13 | #63 | Project Compare v1 | Byte/text and structural diff, ambiguous matches. |
| V5-14 | #64 | Project Snapshots | History, baselines, integrity, portability. |
| V5-15 | #65 | Engineering Preview | Integration tests, G4, conditional Windows preview and UserSmoke. |

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

## 6. UI delivery plan (approved concept 26 November 2026)

UI strategy B: design in parallel, implement in existing sequence. PR #50 and #51 merged; UI documentation refinement is PR #52. Subsequent PR numbers in the backlog are adjusted and remain indicative; stable V5 IDs and dependencies are unchanged.

| Stage | UI deliverable | Acceptance |
| --- | --- | --- |
| V5-03 | Selected family/firmware context, profile and verified support boundaries. | No silent fallback; profile change invalidates derived data, explicit unknown. |
| V5-04 | Shared signal keys, search/filters and source navigation; minimal integration into existing UI. | READ/WRITE/UNKNOWN; exact links, distinguish bit/byte/word/register, reject stale snapshot. |
| G1 | Semantic Core / Controller Profiles / Signal Index. | Retain features and original bytes, consistent contracts. |
| V5-05 | UI Foundation: CSS tokens, Dark/Light, six accents, Compact/Standard/Comfortable, preferences and reset. | Persistence after restart; handle invalid/old settings; contrast, focus, independent logical colors. |
| V5-06 | Engineering Workspace: shell, Activity Bar, Explorer, tabs, Inspector host, Bottom Panel, Status Bar, layout registry, resize/hide, Custom Workspace, Zen Mode; Navigation History and command registry. | Switch without losing session/draft, persist presets, reset, keyboard, missing modules do not simulate functionality. |
| V5-07 | Ladder Viewer v3: all recognized rungs, virtualization, zoom, navigation, supported blocks, UNKNOWN; Minimap as separate substage after core Viewer. | Measure open, navigation and scrolling on 10k/50k/100k lines, no truncation of accessible rungs; explicit unavailable semantics. |
| V5-08 | Signal Inspector: card, Pin Inspector, source and role navigation. | Signal and snapshot binding, pin does not substitute another project's data, clear empty states. |
| G2 | Windows Engineering Workspace Preview. | Separately approved Preview, manual UserSmoke; themes, layouts, Ladder, card, focus, persistence/restart. |
| V5-09…15 | Cross Reference, Diagnostics, Logic Tracing, Compare, snapshots; expand Command Palette and Navigation History. | Real functionality in matching layouts; G3/G4. |
| After G4 | Virtual Laboratory, Trace & Test, Documentation, Robot Fleet and AI within Roadmap. | Layouts receive real modules as available, no fabricated telemetry. |

### 6.1. Proposed UI QA criteria

- Check 1280×800, 1920×1080, Windows scaling 100/125/150%; these are not claimed tested.
- Check keyboard, visible focus, accessible names, meaning independent of color, text and element contrast.
- Check restart, theme/accent/density switching, corrupt preferences, layout restore, unsaved draft, source snapshot update.
- Proposed bindings: Ctrl+P — navigate, Ctrl+Shift+P — Command Palette, Ctrl+F / Ctrl+Shift+F — search, F12 / Shift+F12 — write/references (choose among multiple writers), Alt+Left / Alt+Right — history, Ctrl+Tab — tabs, Ctrl+B / Ctrl+J — panels, F2 — permitted draft only, F11 — fullscreen. Check WebView2/Windows conflicts before finalizing.
- Automated model, IPC and UI tests; separate visual Windows protocol with real user workflows. Performance measured on synthetic and authorized private data, not a claim of achieved FPS/latency.

This is a delivery plan, not a claim of implementation. New dependencies, exact persistence settings and substages are justified in their respective PRs. All merges and releases require separate consent; native write/online remain blocked.
