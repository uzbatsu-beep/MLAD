<!-- Generated en companion for docs/roadmap.md; review required. -->
> **English** · [Русский](roadmap.ru.md) · [English](roadmap.en.md) · [Original / Оригинал](roadmap.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.
> **MLAD safety invariants:** `CIOPRG.LST` and `IONAME.DAT` are strictly **read-only**. FTP/HSES, live I/O, native writer and robot deployment remain **BLOCKED**. Passing `WindowsCI` does **not** imply `UserSmoke=PASS` or `OEMBenchVerified`: both remain **NOT RUN** unless independently documented. Machine translation requires source and Yaskawa manual review.

# MLAD - roadmap v3: Native CIO Workspace → Offline Engineering IDE

**Control point:** 18.11.2026, `main` = `f85ddfcce3063dd087425be981a9d2b7a16d6309`. PR #1–#29 merged. [Linux validation SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454) и [Windows Acceptance & Security Regression SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621) on this SHA. Windows GitHub Release: [`v0.5.0-rc.9-oem-reference`](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference) An old artifact, does not contain PR #24-#29**. Separate passing of tests on the user's PC and on the robot **not confirmed **.

**Status:** Technical roadmap; new PR after #29 so far **proposed**, not created, except for separate documentation PR #30 when it was published. [Detailed backlog](implementation_plan.en.md) · [Actual status](project_status.en.md) · [Decisions D1-D9](engineering_decisions.en.md) · [Historic roadmap v2](roadmap_v2_archive.en.md).

## 1. Purpose and Product Contract

**MLAD (Motoman Ladder Analysis & Draft IDE)** - independent OFFLINE/BENCH engineering environment Yaskawa Concurrent I/O for profiles DX100, DX200 и YRC1000. The user opens the exported `CIOPRG.LST` In the main window, examines the source, chains and blocks, searches for addresses/links, classifies undefined fragments, and leads **separate** editable JSON drafts, notes, and future projects.

** Principles unchanged:**

1. `CIOPRG.LST` и `IONAME.DAT` - only **read-only**, source bytes and SHA-256 unchanged. There is no “fix LST” operation or conversion of JSON back to native.
2. `supported / partially supported / opaque / malformed` They are different, not confirmed logic. `PART`/`END` and tail data is not automatically a System/User policy.
3. `*.graph.json`, `*.advanced.json` и legacy `*.json` individual user projects. Potential `*.mlad.json` The sidecar is also a ***** controller format.
4. MOV/ADD/SUB/SHL/SHR have a typed and manual-derived ** reference** subset, but there is no verified scan/EN/dataflow execution. OFFLINE TRUE/FALSE/UNKNOWN is not a physical I/O.
5. HSES/FTP, live monitoring, native writer, robot deployment and automatic control of the **BLOCKED** controller. Any change to the boundary requires an independent clearance of the owner and OEM evidence; a successful CI does not issue such an admission.
6. Private real export files, controller configuration and personal/production information are not published on GitHub, Issues or CI. Public fixtures are only self-created synthetic or explicitly authorized.
7. Each technical stage is a separate PR with positive/negative tests, evidence, CI and a separate decision of the owner about merge.

## 2. The actual baseline after PR #29

| PR Group | Implementation framework | Limitation |
| --- | --- | --- |
| #1–#12 | Address types, lossless read-only, Boolean AST, individual Graph/Advanced JSON, Windows preview GPL-3.0-only, npm lockfile / `npm ci` | No scanner, no native writer. |
| #13–#18 | Compiled HSES/FTP runtime removed; paging/search; unified Native Workspace, IONAME and local annotations;`END` data | UX/cross-ref and OEM validity are not complete |
| #19–#21 | TypedCandidate for five operations, static M-links, isolated OEM manual-derived vectors | No confirmed execution/scan and firmware-level OEM bench |
| #22–#23 | Source/architectural circuit design and Community Standards | Not hardware certification |
| #24–#26 | Staged JSON с `*.bak`; CSP; bounded guarded native/JSON reads, File ID checks, JSON conflict detection | Remain directory races, ACL, crash recovery |
| #27 | Roadmap/verification/engineering decisions sync | Documents do not close functional gaps |
| #28 | Windows regression workflow, NTFS junction/lock tests, temporary unsigned EXE, WIN-01...WIN-12 checklist | WebView2 UserSmoke was not executed. |
| #29 | File ID+bytes for JSON snapshots, hardlink rejection, staged file ID +SHA-256, Windows ADS/UNC/device path deny | Total TOCTOU/ACL/backup+target warranty |

`GPL-3.0-only` The authorship of the original MLAD materials remains unchanged. Publication of the code does not mean the right to include OEM/client materials in the repository. Legal framework: [licensing.md](licensing.en.md).

## 3. Stages M0–M13 – Real state

Designations: **DONE** = validated software implementation, **PARTIAL** = supported subset, **OPEN** = no required function, **BLOCKED** = independent tolerance. `SyntheticTested` ≠ `UserSmoke` ≠ `OEMBenchVerified`.

| Phase | Priority | Status on `f85ddfc` | Transition criterion |
| --- | --- | --- | --- |
| M0 — Git/CI/legal baseline | P0 | **DONE:** PR #1#29, Windows+Linux CI is successful; branch `main` unprotected | Rules of merge / optional Branch Protection separately |
| M1 — OEM source corpus | P0 | **PARTIAL:** Private pre-inventory; firmware/OEM provenance incomplete | Legitimate private fixtures + controller/firmware + OEM result; synthetic public corpus |
| M2 — filesystem/security | P0 | **PARTIAL:** #24-#29 reduced identified risks | Windows UserSmoke + agreed residual ACL/races/crash risks and additional security reviews |
| M3 — Native Workspace | P0 | **DONE (read-only base):** #15-#17 | Stable reading window without native save |
| M4 – navigation/diagnostics | P0 | **PARTIAL:** paging, source/search, rung list/grouping | Reliable sourcerrung/block/diagnostic jump, filters and large-file UX |
| M5 — native typed syntax | P1 | **PARTIAL:** 5 opcodes + opaque | Synthetic edge cases + evidence-validated grammar increments |
| M6 — static semantic/dataflow | P1 | **PARTIAL:** Boolean AST, static M-links, manual-derived reference | Verified references and honest unknown without fake scan |
| M7 — portable offline session | P1 | **OPEN:** local annotations available, `*.mlad.json` no | Versioned sidecar, hash binding, migration/conflict/error handling |
| M8 — unified engineering IDE | P1 | **Open:** Different JSON editors work separately | Source/notes/drafts without modified native navigation |
| M9 — scan simulator | P2 | **OPEN / NOT VERIFIED** | OEM scan/time contract + bench |
| M10 — OEM bench | P1–P2 | **NOT RUN** | Separate allowed isolated stand, exact model and firmware |
| M11 — read-only HSES | P2 | **BLOCKED** | Independent owner decision + protocol and risk review |
| M12 — native CIO writer | P3 | **BLOCKED** | Full OEM grammar/System/User and Golden round-trip, separate admission |
| M13 — robot deployment | P3 | **BLOCKED** | M12, independent safety/recovery procedure, expressly agreed rights |
| MX — release/UX/quality | P0–P2 | **ONGOING:** CI and rc.9, but no new release | Manual Windows acceptance, legal/assets, played by SHA |

## 4. Strategy v3 – what to do in what order

**Priority #1 is to securely open and understand real files without declaring an unknown syntax confirmed. ** From the first user view, a large amount was identified `unknown/opaque` The difficulty of reading the workspace. Investing in synthetic parser fixtures, navigation and traceable diagnostics is more beneficial than prematurely creating a scanning simulator or a native writer.

**Flow A – Trust and Source Quality (P0).** Complete docs synchronization, create synthetic corpus (headers, etc.) `//$M`, `PART`/`END`ALARM/MESSAGE tail, malformed lines, BOM/CRLF, opaque spans, size limits. Private real-world programs only for **separate* provenance intake. Do not publish their hashes, unique instructions or production information.

**Flow B - convenient Native Workspace (P0; parallel A).** Source/paging, address and name search, bidirectional transitions `source↔rung↔block`Disclosed diagnostics with an exact physical string, partial coverage indicator and compressed representations of a large number of blocks. First a handy read-only tool, then complex semantics.

**Flow C is a custom offline project (P1).** Versioned `*.mlad.json` for the controller profile, source SHA, bookmarks, local annotations, links to drafts. Consider source verification, conflict detection, schema upgrade, backup, and portability prior to UI integration. Do not save the native code on the sidecar.

**Flow D - Evidence-Based Typing and Static Analysis (P1).** Typed opcode grammar `проблема → OEM evidence → parser rule → negative test`. Make cross-reference READ/WRITE and secure alerts without assertions about the physical status of the signal or the actual scan order. Unknown remains unknown.

**Stream E - Unified Offline IDE (P1).** Consolidate Native Snapshot, annotations, sidecar and two JSON editors into one consistent user model. Any native→draft transfer is only an explicit action for an independently verified subset; by default native read-only.

**Flow F: Windows product acceptance and release engineering. ** Now run WIN-01...WIN-12 on a test Windows PC outside the production network; `UserSmoke PASS/FAIL/NOT RUN`. Before posting 0.6.x: Update GPL/source notices, third-party license/SBOM review, unsigned status, archive/checksums and manual protocol. Windows CI is not authorized by itself.

## 5. Phase order and solutions

**Phase 0- PR #30:** bring all existing MDs in line with `main` After #29 and approve the backlog. **Docs-only** does not add runtime or close Windows UserSmoke.

**Phase 1 - PR #31 and #32 (parallel with safety baseline saved): ** synthetic parser regression corpus and Native Workspace UX/diagnostics. Evidence: source byte identity, full string access, precise transitions, and guaranteed visible opaque.

**Phase 2-- PR #33 and #34:** versioned sidecar schema + typed grammar of selected unrecognized patterns; evidence only, no automatic semantics without confirmation.

**Phase 3- PR #35 Source-linked static analysis and the Offline Engineering Project. QA scripts are synthetic plus private/manual verification with the permission of the data owner.

**Phase 4-- PR #37 (conditional):** Windows OFFLINE preview 0.6.x, following successful CI, manual smoke, legal review and separate owner decision. In case of violation, the gate is a defective/blocking PR, not a formal release.

PR #31#37 – Preliminary numbers**, uncreated tasks/committees Existing volumes and dependencies - in [implementation_plan.md](implementation_plan.en.md).

## 6. Points of control that cannot be hidden

- Semantics: `typed valid`, `manual-derived` и `OEM-verified` - three different marks. The five reference arithmetic commands do not form a scanner.
- **Files:** Windows hardlink/ADS/UNC restrictions and File ID checks after PR #29 - useful protection, but `check→rename`, parent directory handles, ACL inheritance, competitive writers and power-loss recovery remain **OPEN**; [threat review](filesystem_security_pr29.en.md).
- **Windows:** [PR #28 acceptance protocol](windows_acceptance_pr28.en.md) Mandatory manual step; automation without WebView2 He doesn't transfer it to PASS.
- ** Yaskawa data:** `PART 1/2` Both System/User and IONAME→CIO map are not output from a single file.
- ** Authority:** HSES/read-only access, native writer and robot deployment are self-blocked solutions. Interface development does not activate them.
- ** Management:** `main` last-minute `protected=false`. Recommended GitHub Ruleset with mandatory CI; this is a separate owner setting, not identical to consent to merge.

Historic decisions D1-D9 remain valid. New permission formats/changes require a separate ADR. The initial wording and the previous table of steps are retained in [roadmap v2 archive](roadmap_v2_archive.en.md).
