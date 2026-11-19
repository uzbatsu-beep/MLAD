<!-- MLAD-I18N:EDITORIAL:reviewed 2026-11-19 against docs/roadmap.md -->
> **Language / Язык:** [English](roadmap.en.md) · [Русский](roadmap.ru.md) · [Original](roadmap.md)
>
> **Historical scope:** This roadmap captures the planning baseline dated 18 November 2026. Some suggested PR numbers were subsequently assigned to documentation and translation work; verify current GitHub PRs before planning implementation. Translation is not an approval to modify controller software.

# MLAD — Roadmap v3: Native CIO Workspace → Offline Engineering IDE

**Baseline:** 18 November 2026, `main` at `f85ddfcce3063dd087425be981a9d2b7a16d6309`. PR #1–#29 were merged. [Linux validation SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454) and [Windows Acceptance & Security Regression SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621) apply to this SHA. The latest Windows release at this baseline, [`v0.5.0-rc.9-oem-reference`](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference), is an older artifact and **does not contain PR #24–#29**. Independent testing on the user's Windows PC and on a real robot was **not confirmed**.

**Document status:** Technical roadmap. Following PR #29, subsequent PRs were **proposals rather than existing pull requests**, except the separately published documentation PR #30. [Implementation backlog](implementation_plan.en.md) · [Verified status](project_status.en.md) · [Approved decisions D1–D9](engineering_decisions.en.md) · [Archived roadmap v2](roadmap_v2_archive.en.md).

## 1. Purpose and product contract

**MLAD (Motoman Ladder Analysis & Draft IDE)** is an independent **OFFLINE/BENCH** engineering environment for Yaskawa Concurrent I/O, initially supporting controller-family profiles DX100, DX200 and YRC1000. Engineers can open an exported `CIOPRG.LST` in the main workspace; inspect original text, rungs and blocks; find addresses and source references; classify unknown material; and maintain **separate editable JSON drafts**, annotations and future offline projects.

**Non-negotiable principles:**

1. `CIOPRG.LST` and `IONAME.DAT` are **read-only** inputs. Original bytes and SHA-256 must remain unchanged. There is no operation to repair a native LST in place or convert JSON back into a robot-native program.
2. `supported / partially supported / opaque / malformed` are distinct classes. The interface must not invent executable logic for an unrecognized source fragment. `PART`/`END` and following data do not, by themselves, establish System/User ownership or rights.
3. `*.graph.json`, `*.advanced.json` and legacy `*.json` are independent user files. A potential `*.mlad.json` sidecar would **not** be a controller-native format.
4. MOV/ADD/SUB/SHL/SHR have typed, manual-derived **reference-only** subsets; actual scan/EN/dataflow execution is not verified. OFFLINE TRUE/FALSE/UNKNOWN values are **not** physical I/O states.
5. HSES/FTP, live monitoring, native writing, robot deployment and automatic controller control remain **BLOCKED**. Changing this boundary requires separate owner authorization and OEM evidence; passing CI confers neither.
6. Confidential controller exports, configurations and personal/production data must not be posted to GitHub, issues or CI. Public fixtures must be independently constructed synthetic examples or explicitly authorized.
7. Each technical milestone requires its own PR, positive and negative tests, evidence, CI, and a **separate owner decision to merge**.

## 2. Verified baseline after PR #29

| PR group | Implemented baseline | Remaining limitation |
| --- | --- | --- |
| #1–#12 | Address classes, lossless read-only parsing, Boolean AST, separate Graph/Advanced JSON, Windows preview, GPL-3.0-only, npm lockfile / `npm ci` | Neither a scan simulator nor a native writer |
| #13–#18 | Compiled HSES/FTP runtime removed; source paging/search; Native Workspace, IONAME and local annotations; data after `END` | Incomplete UX/cross-references and unverified complete OEM grammar |
| #19–#21 | Five `TypedCandidate` operations, tentative static M-register references, isolated manual-derived OEM reference vectors | No verified scan/runtime execution or firmware-specific OEM bench testing |
| #22–#23 | Source/architecture organization and Community Standards | Not a hardware safety certification |
| #24–#26 | Staged JSON writes with `*.bak`, CSP, bounded guarded native/JSON reads, File ID checks and JSON conflict detection | Directory races, ACL handling and crash recovery remain |
| #27 | Roadmap, verification gate and decision-register synchronization | Documentation does not close functional gaps |
| #28 | Permanent Windows regression workflow, NTFS junction/locking tests, temporary unsigned EXE, WIN-01–WIN-12 checklist | Manual WebView2 UserSmoke was not performed |
| #29 | JSON snapshots comparing File ID **and bytes**, hardlink rejection, staged File ID+SHA-256 checks, Windows ADS/UNC/device path rejection | No complete TOCTOU/ACL/atomic `backup+target` guarantee |

`GPL-3.0-only` and ownership of MLAD's original materials are unchanged. Publishing MLAD source code does not grant permission to publish OEM manuals or customer programs; see [licensing](licensing.en.md).

## 3. Milestones M0–M13: actual state

Status meanings: **DONE** = verified software capability; **PARTIAL** = a constrained subset; **OPEN** = required capability missing; **BLOCKED** = additional independent authorization needed. `SyntheticTested` ≠ `UserSmoke` ≠ `OEMBenchVerified`.

| Milestone | Priority | Status at `f85ddfc` | Required gate |
| --- | --- | --- | --- |
| M0 — Git/CI/legal baseline | P0 | **DONE:** PR #1–#29, successful Linux/Windows CI; `main` unprotected | Optional GitHub Branch Protection and explicit merge rules |
| M1 — OEM source corpus | P0 | **PARTIAL:** preliminary private inspection; firmware/OEM provenance incomplete | Authorized private fixtures, precise controller/firmware and OEM results; public synthetic corpus |
| M2 — filesystem/security | P0 | **PARTIAL:** #24–#29 mitigated known risks | Windows UserSmoke, residual ACL/race/crash assessment and further security reviews |
| M3 — Native Workspace | P0 | **DONE (basic read-only):** #15–#17 | Stable read-only source viewing; no native save |
| M4 — source navigation/diagnostics | P0 | **PARTIAL:** paging, source search, rung lists and grouped warnings | Reliable source↔rung/block/diagnostic navigation, filtering and large-file UX |
| M5 — typed native syntax | P1 | **PARTIAL:** five opcodes; other forms opaque | Synthetic edge cases and evidence-verified grammar extensions |
| M6 — static semantic/dataflow | P1 | **PARTIAL:** Boolean AST, static M-links and manual-derived reference cases | Source-linked evidence, explicit unknowns and no invented scan |
| M7 — portable offline session | P1 | **OPEN:** local annotations exist, `*.mlad.json` does not | Versioned sidecar, hash binding, migration/conflict/error handling |
| M8 — unified engineering IDE | P1 | **OPEN:** JSON editors are separate | Shared navigation across source/notes/drafts, with the native original unchanged |
| M9 — scan simulator | P2 | **OPEN / NOT VERIFIED** | Formal OEM scan/time contract and independent bench validation |
| M10 — OEM bench | P1–P2 | **NOT RUN** | Separately authorized isolated rig, model and firmware evidence |
| M11 — read-only HSES | P2 | **BLOCKED** | Owner authorization, verified protocol and security review |
| M12 — native CIO writer | P3 | **BLOCKED** | Complete OEM grammar/System/User boundaries, golden roundtrip and explicit permission |
| M13 — robot deployment | P3 | **BLOCKED** | M12 plus independent safety/recovery process and documented rights |
| MX — quality/UX/release engineering | P0–P2 | **ONGOING:** CI and rc.9; no new public release | Manual Windows acceptance, legal artifacts and reproducible hashes |

## 4. Roadmap v3: workstreams and order

**Top priority: make genuine files reliable to open and understand without treating unknown syntax as verified.** Early feedback from viewing an actual exported program revealed substantial `unknown/opaque` content and navigation problems. A synthetic parser corpus, traceable diagnostics and better navigation provide more immediate engineering value than a premature scan simulator or native writer.

**Workstream A — source confidence and parser fidelity (P0).** Synchronize documentation; build synthetic tests covering headers, `//$M`, `PART`/`END`, ALARM/MESSAGE tails, malformed lines, BOM/CRLF, opaque spans and file-size limits. Keep actual robot programs private under separately authorized provenance intake. Do not disclose private hashes, unique programs or production data.

**Workstream B — usable Native Workspace (P0, parallel with A).** Improve paginated source access, address/name search, two-way `source↔rung↔block` jumps, expandable diagnostics linked to exact physical lines, explicit partial-coverage indicators and compact displays for large numbers of blocks. Build a useful read-only instrument before claiming deeper semantics.

**Workstream C — user-owned offline project (P1).** Design a versioned `*.mlad.json` sidecar holding controller profile, source SHA, bookmarks, local annotations and links to independent JSON drafts. Define stale-source detection, conflict handling, schema upgrade, backup and portability **before** integrating the format into the UI. Do not store modified native CIO code in the sidecar.

**Workstream D — evidence-driven typing and static analysis (P1).** Extend typed opcode grammar through: problem → OEM evidence → parser rule → negative test. Add conservative READ/WRITE cross-references without inferring actual scan order or physical output status. Unknown stays unknown.

**Workstream E — Unified Offline IDE (P1).** Combine Native Snapshot, annotations, sidecar and the independent Graph/Advanced JSON editors in one coherent user workflow. Transferring a verified native subset to a draft must be an **explicit action**; native source is read-only by default.

**Workstream F — Windows acceptance and release engineering (cross-cutting).** Perform WIN-01–WIN-12 manually on a test Windows PC outside production networks and record `UserSmoke PASS/FAIL/NOT RUN`. Before publishing 0.6.x, review GPL/source notices, third-party licenses/SBOM, unsigned status, source archives/checksums and the manual test report. **Windows CI alone does not authorize a release.**

## 5. Phased implementation sequence and decisions

**Phase 0 — proposed PR #30:** align all active Markdown with `main` after #29 and agree on the backlog. **Docs-only**; adds no runtime and does not complete Windows UserSmoke.

**Phase 1 — proposed PR #31 and #32 (parallel while retaining the safety baseline):** synthetic parser regression corpus and Native Workspace UX/diagnostics. Evidence: exact original bytes, complete line access, accurate navigation and consistently visible opaque data.

**Phase 2 — proposed PR #33 and #34:** versioned sidecar schema and typed grammar for selected unrecognized forms, grounded in evidence, **without** inventing an execution model.

**Phase 3 — proposed PR #35 and #36:** source-linked static analysis and Unified Offline Engineering Project, with synthetic QA and separately authorized private/manual checks.

**Phase 4 — conditional PR #37:** Windows OFFLINE preview 0.6.x after successful CI, manual smoke, legal review and explicit owner permission. Failed gates require a blocking bugfix, not a nominal release.

**The source roadmap used #31–#37 as provisional planning labels, not a claim that the PRs existed or their scope was permanently reserved.** For current assignments verify GitHub and consult the [implementation plan](implementation_plan.en.md).

## 6. Facts and open gates that must remain visible

- **Semantics:** `typed valid`, `manual-derived`, and `OEM-verified` are different evidence classes. Five arithmetic reference operations do **not** constitute a scan simulator.
- **Files:** Windows hardlink/ADS/UNC restrictions and File ID checks after #29 are defenses, but `check→rename` races, directory handles, ACL inheritance, concurrent writers and crash/power-loss recovery remain **OPEN**. See [threat review](filesystem_security_pr29.en.md).
- **Windows:** [PR #28 manual acceptance protocol](windows_acceptance_pr28.en.md) is a separate mandatory gate. Automated builds without a real GUI test do **not** make UserSmoke PASS.
- **Yaskawa evidence:** `PART 1/2` does not by itself establish System/User access rights, and one file cannot prove a universal IONAME→CIO address map.
- **Authorization:** read-only HSES, native writer and robot deployment require separate explicit approvals. Developing an offline UI cannot enable them.
- **Governance:** GitHub reported `main` as `protected=false` at this baseline. A Ruleset with required CI checks is recommended but must be configured by the owner and is not itself merge permission.

Approved decisions D1–D9 remain in force. New formats or permission changes require an independent ADR. Historical wording and the prior milestone table are preserved in the [roadmap v2 archive](roadmap_v2_archive.en.md).
