<!-- Generated en companion for docs/roadmap_v2_archive.md; review required. -->
> **English** · [Русский](roadmap_v2_archive.ru.md) · [English](roadmap_v2_archive.en.md) · [Original / Оригинал](roadmap_v2_archive.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.

> **Historical copy.** The content below is recorded before updating after merge PR #29 18.11.2026. The “current” SHAs, Draft Statuses, and P.R. numbers listed here reflect the past date. ** Current versions:** [roadmap](roadmap.en.md), [implementation plan](implementation_plan.en.md), [project status](project_status.en.md). Do not use this archive as an actual task or security clearance.

---

# MLAD v2: Native CIO Workspace → Engineering IDE → Verified Controller Integration

**Update:** 18.11.2026; `main` `3c184cd36e076dc2e5c9aed7a663b601b4899dea`, PR #1–#28 merged; [Linux CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44365314264), [Windows CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44365314173). PR #29 — Draft. Windows [rc.9](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference) released PR #24–#29. [Backlog](implementation_plan.en.md).
**Status:** Basic Native Workspace and read-only parser work; five commands are typed and equipped with standalone OEM vectors, but scan/dataflow, System/User, booth and native writer ** are not confirmed/locked**. [Current status](project_status.en.md).

## 1. Product purpose and limitations

**Purpose:** Unified engineering IDE for Yaskawa Concurrent I/ODX100, DX200, YRC1000), where possible: open *real * `CIOPRG.LST` in the main workspace; see the full source code, chains and functional blocks; find signals, cross-references and diagnostics; separately create and check the editable offline project; in the future - after proven compatibility - perform confirmed read operations and, only with a separate resolution, changes to the equipment.

** Invariants:**
- `CIOPRG.LST` **immutable read-only source**; UTF/BOM, line endings, unknown sections, bytes, and SHA-256 Don't cheat. No GUI workflow rewrites the original.
- Fully recognized, partially recognized and **opaque/unknown** are displayed **separately**. Schedules should not invent a management structure.
- `*.graph.json` и `*.advanced.json` For the time being, they are self-modifiable models **not equivalent ** native CIOPRG.
- TRUE/FALSE/UNKNOWN in the current UI – local manual inputs/models HSES Telemetry and not proof of physical exit.
- Real import for viewing, read-only network operations, native writer and deployment – **different** permissions and different safety gates.
- The priority is **real source and reproducibility**, not removing warnings for the sake of a green report.

## 2. Fixed baseline

| PR | Implemented and merged into `main` | Should not be considered proven |
|---|---|---|
| #1–#5 | Memory profiles, lossless, Boolean AST, JSON Graph Draft and TMR/CNT/GSTR/GOUT | No native writer and OEM measurements |
| #6–#9 | Windows preview, multi-rung, inline JSON, documentation audit | CI ≠ hardware validation |
| #10–#12 | Roadmap v2, GPL-3.0-only, npm lockfile/`npm ci` | Legal and security audit is open |
| #13–#14 | Unconfirmed deleted HSES/FTP runtime, source paging/search | Filesystem security still requires audit |
| #15–#17 | Native Workspace, IONAME overlay, System Open dialogs, Compact UI | OEM IONAME mapping and writer not confirmed |
| #18 | Post-END ALARM/MESSAGE as data, diagnostics grouping | Not universal OEM grammar |
| #19 | TypedCandidate MOV/ADD/SUB/SHL/SHR | Non-executable transactions |
| #20 | Static Hypotheses of M-References | Not real data stream/scan |
| #21 | EN/Carry/Zero/Error and isolated reference vectors | No poster confirmation. |

**Historical observation of the early Windows build (17.11.2026):** user opened export `CIOPRG.LST` Inspector. The screenshots provided show **1396 lines, 13189 bytes, 215 unknown/opaque, 64 boolean chains**, MOV/ADD warnings and CNT/GSTR/GOUT visual blocks. This is a **diagnostic observation on a single file*, not an approved open golden fixture. The number/applicability of instructions for other controller/firmware is not generalized. Before working with bytes, you need the authorized original file and provenance. [controller_fixture_intake.md](controller_fixture_intake.en.md).

## 3. Development and Dependency Stages

Designations: **P0** – blocking foundation; **P1** – key function of offline IDE; **P2** – deepening check/reading; **P3** – potential write operations only after individual tolerances. The statuses below take into account merged PR #1–#26; `DONE` means the software step executed, **not* hardware verification.

| ID | Phase | Priority | Depends on | Verified result | Status |
|---|---|---|---|---|---|
| M0 | Baseline PR #1–#28 | DONE | — | Main Combines #1–#28, Main and Windows CI Successful | DONE: main `3c184cd3`; PR #29 - Separate Draft |
| M1 | Corps real. `CIOPRG.LST` | P0 | Permission to receive/hold | Hash+metadata+OEM evidence, closed golden fixtures DX100/DX200/YRC1000, synthetic public regression | PARTIAL: five closed source studies; no full controller/firmware/OEM evidence |
| M2 | Safety and reproducibility | P0 | M0 | HSES/FTP runtime deleted; lockfiles; #24 staged JSON, #25 CSP/path checks, #26 file identity/conflict guard | PARTIAL: Windows CI successful; user smoke, TOCTOU/directory handles, ACL/backup transaction open |
| M3 | Native Offline Workspace | P0 | M0, the M1 read interface can expand in parallel | Opening `CIOPRG.LST` in the **main working window**, fully read-only, without loss of bytes | DONE (read-only): PR #15–#17 |
| M4 | Navigation and diagnosis of native file | P0 | M3 | Full source, filters / search / go to the line, cross-links, compact blocks / errors | PARTIAL: paging/search/diagnostics; extended cross-ref/UX open |
| M5 | Typed Syntax extension | P1 | M1, M3/M4 | MOV/ADD/SUB/SHL + other commands encountered: typed operands, source spans, unknown opaque | PARTIAL: five typed opcodes; other unknown commands |
| M6 | Semantic / Dataflow Analysis v2 | P1 | M5, M1 | READ/WRITE refs, conflict records, timers/counters, incomplete branches, trace source→graph | PARTIAL: Boolean AST + static M-links + reference vectors; scan/dataflow unknown |
| M7 | Unified Offline Engineering Project | P1 | M3, M6 | Session `native snapshot + sidecar + separate editable drafts`; search, notes, comparison of changes | OPEN: native snapshot and local annotations, portable sidecar not created |
| M8 | Editor and static inspection of the engineering project | P1 | M7, M6 | Common Ladder/advanced components, change history, addressing rules, pre-saving diagnostics | Open: separate JSON drafts are available; there is no single native engineering project |
| M9 | Reproduced offline model | P2 | M6, M1, OEM evidence | Logic test cases and limited stateful scenarios with an explicit scan/time contract | OPEN: offline reference vectors ≠ OEM scan simulation |
| M10 | OEM Bench Validation and Comparative Matrix | P1–P2 | M1, M5/M6, permitted stand | Comparison of MLADаOEM Ladder Editor for each controller/firmware, measured results | NOT DONE: No access to the booth |
| M11 | Read-only HSES diagnostics | P2, regardless of writer | M2, OEM HSES specification, M10 for I/O mapping | Verified read-only reading, timestamps/quality/staleness, secure UNKNOWN | BLOCKED: HSES Read-only not allowed |
| M12 | Native CIOPRG writer + verified round-trip | P3 | M1, M5–M10, System/User boundaries | Proven native format accuracy and independent OEM verification; separate access solution | BLOCKED: Native CIO writer not allowed |
| M13 | Controlled deployment and recovery | P3 | M12, risk assessment, OEM approvals | Separately permitted boot procedure, backup, rollback, right to write, trace | BLOCKED: deployment not permitted |
| MX | UX/Quality/Release Engineering | P0–P2 | Every step | Versioned releases, CI, Windows smoke, logging, performance, signing, docs | ONGOING: CI / docs / Windows rc.9 |

**New sources (closed processing, 17.11.2026):** locally studied five more users `.LST`uploads without publishing files, hashes, addresses and configurations. The heading options are noted, `//$M` and separate sections after `END`Exact controllers/firmware and OEM verification results are not provided. For parser v2, prepare synthetic tests of different titles and the inadmissibility of interpreting ALARM/MESSAGE as Ladder.

**Do not consider the M1 and M2 as an obstacle to the development of read-only M3/M4:** interface development can go through secure synthetic fixtures and user-generated observations. But to declare a "full parsing" M1 is a must.

### Phase A – Foundation of Trust (M1, M2; parallel to M3)

**M1 - real data: ** accept copies of native exports with model, firmware, application, source hash, encoding and OEM outcomes; keep in private if you have rights. Do not commit client configurations, technological IP/passwords/line names. Verification of immutability `copy_original == source_bytes`. When anonymizing, keep separate original and sanitized hashes. Full grammar is not derived from one unloading.

**M2 - Application security and supply-chain:** delete/isolate previous unconfirmed code `network/mod.rs` from the executable surface; add failure tests FTP/HSESCSP review, file access and symbolic links/TOCTOU, atomic record with rollback for JSON, fixed npm lockfile +`npm ci`Windows version/signature rules. The network and native writer are shut down.

**Acceptance:** no call available to the UI initiates network recording; all input files are limited in size and correctly handle the error; CI and Windows smoke reproducible; normal/damaged LSTs do not change.

### Phase B is the main operating window for the present file (M3, M4)

**M3 — Native Offline Workspace:**
- Team. `Открыть проект → CIOPRG.LST (только чтение)` Opens the **main* workspace, not the inspector's modal window.
- Unified view-model: `NativeSnapshot{source_hash, controller, source_lines, spans, recognized/opaque, diagnostics}`. The source is separate from any edited data.
- Left tree down `PART`, chains, registers/variables and block types; **PART 1/2 - analysis tags, untested System/User rights scheme**.
- Center – all reliably recognized circuits and blocks with a compact / extended view; incomplete and unknown segments are clearly marked. On the right are properties, detailed source fragment and warnings.
- Return to the file by hash, a transparent message when changing the source, without broadcasting back to `.LST` and no edit callbacks.

**M4 is an engineering tool level inspector:**
- Full source, not the limit of the first 600 lines of UI: virtualized lines `Mxxx`, `#XXXXX`Switch to the physical line/offset.
- Bidirectional source-line navigation ом rung/AST/advanced block with exact range, showing raw/typed classification.
- Grouping of diagnostic repeats (`MOV 101` etc.) with specific lines; severity/confidence/support status.
- Table / compact view of tens of GSTR / GOUT and CNT / TMR, disclosure of one element; filters / tabs.
- Coverage report **by category**: `fully supported / partially analyzed / opaque / malformed`Not to consider the "recognized lines" as proof of complete semantics.
- Smooth work on a real file of about 1,400 lines and on significantly larger fixtures; no false LIVE state.

**Acceptance M3/M4:** Opens source YRC1000 file in the main window, all lines are available; you can find the address/instruction and go to the Ladder chain and back; unknown designs are visible and not drawn; re-reading saves the original SHA-256. For DX100/DX200 To fixtures - only safe degradation.

### Phase C - Correct interpretation (M5, M6)

**M5 - Typed Syntax for Real Needs:** Priority of Real Unknown Observation Instructions `MOV`, `ADD`, `SUB`, `SHL`. Separate **recognize operator and argument types** from **prove execution effect**. Confirm the operand format and all images by OEM manual and specific native fixtures. Other opcodes are added by frequency/risk; unknown always remains opaque without discarding bytes.

**M6 - Semantic IR/dataflow:** source span of each operation, read/write registers and bits, cycles/reassignments, branches, instruction sequence, TMR/CNT/GSTR/GOUT communications, conflict warnings. Not to consider a complex native design analogous to one coil. Difference Between Native Number `OUT` The number of rungs built must be explained and traceable. The tool shows confidence/coverage and separately `unsupported`.

**Acceptance:** Golden tests on real permitted exports, no loss/substitution of the unknown, correct construction of only confirmed subset, found incorrect/indefinite writers block further semantic statements.

### Phase D – single offline project and engineering development (M7, M8)

**M7 - sidecar session:** non-equivalent native file shell of the project, for example `*.mlad.json`**links** to the source by hash and stores UI layout, notes/bookmarks, controller profile, selected sections and manual test signals. It doesn't contain a modified CIOPRG.LST**. Edited drafts are retained separately. Comparison of two original snapshots by signal/change - read-only report

**M8 – creation and verification of logic: ** a single canvas for Boolean / Advanced blocks; editing by double click; Undo / Redo and correct history advanced; address directory, cross-ref, output conflicts, range tests, prohibition of writing to protected areas. The ability to “copy a validated chain into a JSON draft” is a **separate explicit operation** only for a fully supported subset; there is no mass automatic native file converter.

**Acceptance:** The imported original remains unchanged; notes are stored separately; JSON can be reproduced independently; errors and unsupported designs are not silenced.

### Phase E – independent inspections and Yaskawa equipment (M9, M10)

**M9 - offline simulation:** deterministic test scenarios for Boolean logic, timer/counter/byte copy and augmented instructions; explicitly specified ticks, front edges, reset priority, memory semantics. Separate levels `synthetic verified` vs `OEM measured`. With unknown semantics, do not simulate behavior by guess.

**M10 - hardware acceptance:** exact controller/firmware version, OEM source, approved test site, safe I/O map, trace and measurement. In the test equipment, use **regular tools Yaskawa**, not MLAD native writer. Verify syntax, graph, M-register values, timers, counters, group transmissions, access limits, errors and restart. No safety/motion signals on the test card.

**Acceptance:** Protocol to DX100/DX200/YRC1000 With results, dates, checksum, pass/fail and rollback; CI **not** closes hardware gates.

### Phase F: Limited hardware interaction (M11–M13)

**M11 - Reading only HSES:** independently verified vendor protocol, exact request/response, timestamps and quality, timeouts/offs/conflicts with existing data; UI clearly distinguishes between `OFFLINE`, `LIVE`, `STALE`, `UNKNOWN`. Physical recording, FTPDo not include servo/motion.

**M12 - native serializer (separate management solution):** only with full confirmed grammar, protected System/User boundaries, byte golden round-trips, OEM validator comparison and no unsupported loss. *export to an explicitly named new file*, not overwrite or network upload. Until the prerequisites are met - **BLOCKED**

**M13 - controlled deployment:** is only possible after the M12, external risk assessment, official method specification, backup, independent comparison, dry-run, rollback and hardware resolution. Working with System Ladder, security inputs, servo permissives, and motion is excluded from normal user scenarios.

## 4. Proposed ways forward after discussion

| Option | First stages | Main prize | Limitation |
|---|---|---|---|
| **A — Workspace-first** | M3 → M4; M1/M2 parallel | Quickly get a convenient read-only engineering view of these LSTs | The initial opaque number remains large. |
| **B — Parser-first** | M1 → M5 → M6; M2 parallel | The reliability of typed parsing and dataflow increases faster | The user continues to work in an inconvenient Inspector |
| **C — Reliability-first** | M2 → M1 → M3/M4 | Less Security/Assembly Risk Before Expanding Functions | There will be a noticeable new UX later. |
| **D - Balance (recommended for discussion) ** | Short M2 safety baseline + M3/M4; M1 parallel; M5/M6 original n | Early useful workspace without full grammar claims; fixtures accumulate by the parser stage | Requires discipline in support statuses and a single source-span contract |

**Previously discussed recommendation (chosen and adopted path D):** open native `CIOPRG.LST` in the main window and do a search/link, ** while files are collected and tests are recorded; but do not combine this with unreasonable permission to edit the native file.

## 5. Approved D1-D9 solutions (historically recorded in roadmap)

| ID | Decision | Recorded value | Impact |
|---|---|---|---|
| D1 | The main goal of the next release | **Hybrid path D:** short security baseline → Native Workspace + navigation; M1 parallel; parser/semantics by evidence | PR procedure |
| D2 | Working with baseline `CIOPRG.LST` | **Read only, edited by sidecar separately* * | The Boundary of Trust |
| D3 | How to place blocks in native workspace | **Compact list + disclosure**, chain selection in the center | Productivity/UX |
| D4 | How to show the unknown | **Source + span + unsupported** text without graphical reconstruction | Honesty of analysis |
| D5 | Set of hulls | Closed real fixtures, public synthetic/allowed only | Privacy |
| D6 | Reliability of the instruction | First grammar, then verified effects; no "supported" by name only | Risks of MOV/ADD/SUB/SHL |
| D7 | Hardware communications | **Read-only HSES separately and later**; writer/deploy without approval | Security |
| D8 | What is Offline MVP? | Read-only source, navigation ный diagrams, filtered diagnostics, exact hash | Completion criterion |
| D9 | Technical debt to new UX | Minimum safety barrier now, rest gradually | First PR volume |

## 6. What is NOT executed and not included in the first UI-MVP

- Complete native grammar `CIOPRG.LST` validated `CIOPRG.LST → editable project → CIOPRG.LST` circle.
- System/User ownership by number `PART`.
- Live reading of I/O or actual coil states.
- Robot-proven implementation of MOV/ADD/SUB/SHL or real timer/counter scan semantics.
- Real OEM native compilation, download by FTP or HSES and use any protected System Ladder areas.
- Proof of hardware security/CI certification.

## 7. Management of work and inspections

Each code step is a separate branch/PR once agreed, without merging automatically. PR contains source requirements, modified contracts, positive/negative Rust+TS tests, Windows smoke checklist and restrictions. `npm run docs:check` Check internal Markdown links; primary manuals and hardware measurements are checked separately.

Documents: [architecture.md](architecture.en.md), [controller_fixture_intake.md](controller_fixture_intake.en.md), [cioprg_lst_spec.md](cioprg_lst_spec.en.md), [development_workflow.md](development_workflow.en.md), [advanced_hardware_test_matrix.md](advanced_hardware_test_matrix.en.md).

**This document is a decision map, not consent to the launch of M11-M13.**

### PR #19 — Typed Syntax (merged 17.11.2026)

- Proposed: `MOV`, `ADD`, `SUB`, `SHL`, `SHR` as typed but not executable forms; retaining unchanged raw bytes/source spans and reporting opcode in Native Workspace.
- Not included: semantic emulation, flags, impact on output chains, System/User change, OEM writer, FTP/HSES.
- Validation: Synthetic Rust fixtures + documentation + Windows rc7 preview. Verification on real files - only in a local private enclosure, without publication.

### PR #20 — Static evidence (merged 17.11.2026)

- Read-only, bounded inventory of syntactically typed source/destination operand positions; exact M-register match may propose *source-order-only* predecessor links.
- Any unsupported/malformed, known boolean/advanced, PART, END or post-END boundary breaks tentative continuity.
- No runtime values, arithmetic flags, scan-cycle semantics, control-flow simulation, hardware I/O or controller upload.
- OEM semantic verification remains a separate gate: PR #20 Documentary formulas do not replace measurements.

### PR #21 — OEM reference for five opcodes (merged 17.11.2026)

- Analyzed MOV/ADD/SUB/SHL/SHR in parallel, by section YRC1000 R12 и DX100 Rev.0; DX200 It still requires separate source verification.
- A separate manual 16-bit reference model Rust with tests of limits 0/16 and 255/256 and clearly indefinite flags.
- OEM help in Native Workspace; no integration of reference core with real CIO HSES/FTP, writer or scan engine.
- Windows rc.9 published, PR #21 No tests were carried out on the controller.

## 8. Historic route after PR #21 factual PR #24–#26 (18.11.2026)

**While robots are not available:** Documentary review DX200Expand negative/parser tests, develop portable verifiable sidecar for local comments, reinforce cross-ref/diff and separately conduct security review CSP/filesystem/JSON, third-party notices/SBOM. **Do not include a native writer or network.

**When the stand appears:** pass through [test-fill](advanced_hardware_test_matrix.en.md), including MOV byte=256, SHL/SHR n=0/16, EN/scan and flag #51400/#51401/#51402 for each generation. Only reproducible OEM measurements increase the confidence level of real semantics.

**Organization:** Each future stage in a separate Draft PR, with tests and owner approval prior to the merger. The priority options sections above are saved as a discussion history rather than the current task of first running the UI.


### PR #24 — Atomic Offline JSON Draft Persistence (merged, 18.11.2026)

- Only individual individuals are preserved. `*.graph.json`, `*.advanced.json`, legacy `*.json`; originals `CIOPRG.LST` и `IONAME.DAT` Always read-only.
- Before writing, the size limit is 2 MiB; writing to the newly created file in the same directory, `sync_all`renaming/replacement; the previous version is transferred to `*.bak` through a separate staging/rename.
- **Straight* symlink protection for JSON and backup Rust tests for initial recording, sequential saves, failure, oversized and symlink. A backup/rename error should not cut off the old JSON.
- **Remains M2 PARTIAL:** backup and target do not form a single transaction; Windows replacement depends on file system/ACL rules, no interprocess lock, parent path, hardlinks and TOCTOU races are not eliminated; CSP `null` unchanged. The next security PR should treat file handle, directory trust/locks, and CSP separately.
- Impact of release: Windows binary `rc.9` You need a separate Windows acceptance smoke before declaring behavior stability on Windows.


### PR #25 — Tauri CSP и parent-path safety (merged, 18.11.2026)

- CSP production: banning remote scripts/connect `unsafe-eval`Objects, iframes, worker; only local `'self'` и `ipc: http://ipc.localhost`. `style-src 'unsafe-inline'` React/inline CSS is required. Vite dev CSP is separately limited to localhost:5173 + WebSocket for HMR.
- File operations: Deflection of explicitly detected symlink in parent directories `..` and direct symlink from native CIOPRG/IONAME. JSON read/write continues staging PR #24They are testing all current ancestors.
- Rust tests: common nested paths, symlink-parent, symlink-native, traversal, byte source safety; Node security tests validate CSP/code. Windows MSVC and Tauri build tested by individual successful CI PR #25; the user smoke remains.
- **Do not declare completeness M2:** path can be replaced between verification and open/rename; untrusted Windows junction/reparse points and hardlinks, concurrent writers, permission ACL and CSP runtime smoke are checked separately. No hardware interface.


### PR #26 — Guarded File Snapshots and Conflict Detection (merged, 18.11.2026)

- CIOPRG/IONAME/JSON descriptor readings with 2/4/16 MiB limits `symlink_metadata → File::metadata` ID comparison (Unix dev/inode, Windows volume/index), ID verification after reading. Reparse point detection including Windows junction.
- JSON staged/backup PR #24 Preserve; before the final `rename` Bytes are once again compared with the original; in a conflict, a failure without overwriting the changed JSON. The new creation also prohibits silent overwrite when a file appears during staging.
- Synthetic Rust tests: swap between check/open, external record between backup/commit, competing creation, read/opt-out linked parent, stable tests of original bytes.
- **Remains: ** Race between last reconciliation and final rename, unsafe directory changes/ACL, Windows junction, hardlinks, interprocess serialization and atomic backup transaction. Manual Windows smoke after CI.

## 9. Current working backlog after PR #26

**Fixed order:** documentation PR #27 → Windows runtime/negative checks (proposal #28) → targeted filesystem hardening (#29) → synthetic parser corpus (#30) → Native navigation/diagnostics (#31) → portable sidecar (#32) → evidence-based typed analysis (#33) → unified offline IDE (#34) → candidate `0.6.x` (#35). Numbers #28-#35 are preliminary; stages are created by one PR, after agreeing on the scope of work.

Detailed dependencies, verifiable results and limitations [implementation_plan.md](implementation_plan.en.md). Independent test gates - [verification_gates.md](verification_gates.en.md). Decisions D1-D9 adopted [engineering_decisions.md](engineering_decisions.en.md). M11-M13 remains BLOCKED regardless of the completion of these PRs.

## 10. Current PR #28: Windows Acceptance & Security Regression

PR #28 Develops reproducible NTFS exclusive lock, Windows junction/reparse, CSP/IPC contract, unsigned Windows EXE SHA-256 evidence and separate [manual matrix of Windows UserSmoke](windows_acceptance_pr28.en.md). [Limits of CI and Release](verification_gates.en.md). **M2 Remains PARTIAL**: Successful Windows runner does not mean user verification WebView2OEM bench, power-loss durability or elimination of all filesystem races. Merge/release - only with the separate consent of the owner.

## 11. PR #29 — filesystem boundary (Draft)

[New review](filesystem_security_pr29.en.md) Describes enhanced detection of **same-content file swap**, hardlink-target/backup and staged-file corruption. This is a partial progression of M2, not a closing of absolute TOCTOU/ACL/power-loss risks. Windows WebView2 UserSmoke and OEMBench separately `NOT RUN`Control functions are still blocked.
