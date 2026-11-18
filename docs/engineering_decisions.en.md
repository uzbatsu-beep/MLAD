<!-- Generated en companion for docs/engineering_decisions.md; review required. -->
> **English** · [Русский](engineering_decisions.ru.md) · [English](engineering_decisions.en.md) · [Original / Оригинал](engineering_decisions.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.

# MLAD – Register of Approved Engineering Solutions

**Register version:** 18.11.2026. This document fixes the agreed limits and shares them with the proposals that have not yet been adopted. Reason: [roadmap v3](roadmap.en.md) и [v2](roadmap_v2_archive.en.md), [status](project_status.en.md), [PR](implementation_plan.en.md). The decision to merge a particular PR is always made separately.

## 1. Stable solutions D1-D9

| ID | Agreed decision | Consequence for development |
| --- | --- | --- |
| D1 | **Balanced path D:** short safety baseline → Native Workspace and navigation, real fixtures parallel → parser/semantics → Unified IDE | Architectural sequence selection; UX does not wait for full OEM grammar, but does not claim false compatibility |
| D2 | `CIOPRG.LST` * read-only** only, editable data and sidecar separately | Do not overwrite/normalize the original; immutable bytes and SHA-256Changes to specific JSONs only |
| D3 | In Native Workspace, blocks are represented by a **compact list with disclosure**, the selected chain is in the center. | All recognized chains are available, the source text is page by page; do not overload the canvas with hundreds of heavy diagrams. |
| D4 | Unknown to show as source text, without guessing about semantics | Conditions `recognized`, `partially supported` и `opaque`, initial spans and explicit warnings |
| D5 | Real unloading - closed case, publicly only allowed/synthetic fixtures | There's no real `CIOPRG.LST`/`IONAME.DAT`hashes and production IP in PR/CI |
| D6 | Syntax and semantics confirmed **separately* * | `TypedCandidate` does not mean an executable operator; `ManualDerived` Does not mean OEM scan |
| D7 | A fully autonomous read-only tool. HSES A separate future solution | В `main` FTP/HSES Runtime disabled, no telemetry or recording permissions |
| D8 | Offline MVP `0.6.x`Native Workspace, navigation, diagnostics, safety SHA-256 | Acceptance on functionality and smoke, not by OEM writer |
| D9 | Priority: reproducible npm/CI and security, then workspace/parser/unified IDE | Changes only through small PR and negative tests, no automatic merger |

License: `GPL-3.0-only`; copyright holder ** own original materials MLAD** — Fedor Sergeyevich Kochedykov (individual), © 2026. This is a ***** statement of ownership of OEM/third party materials. [licensing.md](licensing.en.md).

## 2. Unrevised restrictions without a separate decision

- Native export/write `CIOPRG.LST` JSON draft is not disguised as an OEM program.
- HSES/FTPRobot upload, live I/O and System/User modification are prohibited.
- `PART 1/2` You can not declare the established rights of the System / User.
- For each controller/firmware, form and result confirmation is required separately.
- The request to connect to a real robot ** is not intended** from the offline UI development.
- All PR is tested; **merge is solely with the consent of the owner.
- Historical release notes and published artifacts are not rewritten retroactively.

## 3. Open decisions (not considered approved)

| Question | Relevant data | Current status |
| --- | --- | --- |
| Format `*.mlad.json` sidecar, schema version, migrations | Circuit + prototype + negative tests, communication by SHA | Proposal in [plan](implementation_plan.en.md)not implemented |
| Checking/correcting native grammar DX100/DX200/YRC1000 | Fixture, firmware and OEM manual rights | Study M1/M5 without permission from a native writer |
| Cross-references and static dataflow | Source spans, ambiguities, evidence levels | Untested scan model |
| Windows file trust: handles/ACL/races | Threat model, reproducible Windows tests | M2 PARTIAL; PR #24#29 reduced some risks, but last-check→rename/ACL/recovery is open |
| Release `0.6.x` | Windows UserSmoke, legal/source/assets, approval | Candidate, not announced; conditionally PR #37 |
| Read-only HSES | Separate owner approval, OEM read spec, isolated rig | BLOCKED |
| Native writer и deployment | OEM verified fidelity, safety review, separate consent | BLOCKED |

** New plan. PR #31#37 is a sentence**, it does not create D10 and does not automatically approve sidecar schema, a set of new opcodes or a release date. Separate ADR/consistency is required before changing the architecture or constraints.

Any D1-D9 change is made through a separate ADR/PR with motivation, risks and tests. The new decision is not valid until the owner has approved it.

[Matrix of inspections](verification_gates.en.md) · [PR process](development_workflow.en.md) · [Current threat model](filesystem_security_pr29.en.md)
