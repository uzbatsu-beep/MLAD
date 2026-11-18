<!-- Generated en companion for docs/implementation_plan_v2_archive.md; review required. -->
> **English** · [Русский](implementation_plan_v2_archive.ru.md) · [English](implementation_plan_v2_archive.en.md) · [Original / Оригинал](implementation_plan_v2_archive.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.

> **Historical copy.** The content below is recorded before updating after merge PR #29 18.11.2026. The “current” SHAs, Draft Statuses and P.R. numbers listed here reflect the past date. ** Current versions:** [roadmap](roadmap.en.md), [implementation plan](implementation_plan.en.md), [project status](project_status.en.md). Do not use this archive as an actual task or security clearance.

---

# MLAD - a plan for further development after PR #26

**Control point:** 18.11.2026, `main` @ `2092db1825880d251c87efcb7c331d78cb5e4d28`. PR #1–#26 merged; [Main CI - Success](https://github.com/uzbatsu-beep/MLAD/actions/runs/44228204776). The latest **published ** Windows build - [v0.5.0-rc.9-oem-reference](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference). Changes in changes PR #24-#26 is present in `main`but not in rc.9.

**Purpose:** Executable backlog for [roadmap M0–M13](roadmap.en.md). The PR numbers below are **project sequence**, not created tasks/issue obligation. Each stage is a separate branch, Draft PR, mandatory tests, review and **no merger without the owner's separate consent**.

## 1. Current product boundary

MLAD is a standalone engineering IDE for viewing Yaskawa Concurrent I/O and **separate editable JSON drafts**. The main Native Workspace already works as a read-only surface. Discussion `CIOPRG.LST` It saves original bytes and distinguishes between supported/partially supported/opaque. Reference 16-bit MOV/ADD/SUB/SHL/SHR is not a scan execution model.

**Strict prohibitions:** modification `CIOPRG.LST`/`IONAME.DAT`their native export, network runtime FTP/HSESController change, misleading LIVE-indication, automatic equalization `PART 1/2` System/User authorization. Use of real programs - only in a closed enclosure, if there is a right of access; public tests are synthetic or explicitly authorized.

## 2. Order of implementation

| Proposal | Milestones | Priority | Specific result | Acceptance criterion/prohibition |
| --- | --- | --- | --- | --- |
| **PR #27 (merged) ** - synchronization of documentation | M0/MX | P0 | The real plan, [check-up](verification_gates.en.md), [register](engineering_decisions.en.md)Removal of outdated Draft/CI statuses | `docs:check` and full CI; docs-only, no runtime changes |
| **PR #28 (merged)** — Windows acceptance / security regression | M2/MX | P0 | Script negative checks Tauri CSP/IPC and JSON on Windows; documented manual checklist | Windows MSVC+Tauri build, manual Windows tests marked with PASS/FAIL/NOT RUN; source files unchanged |
| **PR #29 (Draft) * Threat Model and Filesystem Hardening | M2 | P0 | Separate threat review for directory handles, Windows reparse, hardlinks, ACL, final check→rename, backup/target; patches within a verified contract | Failure tests for substitution, loss of access, conflict; you can not claim elimination of all TOCTOUs without proof |
| **PR #30Synthetic parser regression corpus | M1/M5 | P0 | Versioned synthetic cases of header variants, `//$M`/`PART`/`END`, tail ALARM/MESSAGE, size/encoding boundaries and malformed blocks; private registry of real files outside of Git | Byte-preservation and correct spans, fail-closed unknown, no real OEM files in Git/CI |
| **PR #31** — Native Workspace navigation & diagnostics | M4 | P1 | Sourcerrungамadvanced block, navigation to addresses/mnemonics, severity/support/confidence filters, compact block list and disclosure, bookmarks | All lines are available; precise offsets/spans; selection does not synthesize unconfirmed diagram; UI performance regression |
| **PR #32** - portable sidecar of the project (specification and read-only binding) | M7 | P1 | Versioned `*.mlad.json` with source SHA, profile, bookmarks/annotations and links to individual JSON drafts; migration/opt-out scheme | Source hash mismatches - clear error/warning; unknown fields are not silently damaged; native LST is not serialized |
| **PR #33** - parser/typed syntax by evidence | M5/M6 | P1 | TypedCandidate only for forms from synthetic+ confirmed OEM data; result categories `recognized / partial / opaque`; dependency graph with provenance | Specific positive/negative vector tests DX100/DX200/YRC1000No execution of unconfirmed commands |
| **PR #34** — Unified Offline Engineering Project | M7/M8 | P1 | The main window combines native snapshot, sidecar, diagnostic transitions and **separate** editable Graph/Advanced JSON. | Clear difference source vs draft; cancel/repeat, secure save, conflict JSON changes processed |
| **PR #35** - candidate `0.6.x` offline-preview | M3–M8/MX | P1 | Agreed release candidate with changelog, hash/source/legal notices, Windows checks, custom smoke report | Publication **only by decision of the owner**; CI б OEM bench; do not declare industrial-ready |

** Dependencies:** PR #28/#29 is a security foundation; #30 is a base for #33; #31 depends on the current Native Workspace, can go parallel to #30; #32 to #34; #33 and #32 to #34; #35 after the adopted functional PR and manual Windows check. If the check reveals a defect, the release is postponed, a separate bugfix PR is created; numbers are not a rigid calendar.

## 3. What is required from the engineering results of each stage

1. **Contract:** What is acceptable to do with source, draft and third-party files; what is failure and recovery; supported controller family/version.
2. **Provenance:** `Documented`, `SyntheticTested`, `ManualDerived`, `OEMBenchVerified`, `Authorized` Separate fields, no label is automatically output from the other.
3. **Conservation:** `original_bytes_before == original_bytes_after`; SHA-256 before/after opening/reopening. For opaque, the exact source span and the source of the error.
4. **Tests:** unit/regression/negative, frontend build, Rust `--locked`, docs check, separate Windows Tauri runner when affecting IPC/filesystem.
5. **Limitations:** Untested formats, controller coverage, Windows smoke, real OEM bench - clearly `NOT RUN`not `PASS`.
6. **Supplies:** source tag, checksums, GPL source and notices; public publication of tests without client program/configuration.

Detailed checklist: [verification_gates.md](verification_gates.en.md), [development_workflow.md](development_workflow.en.md). Current actual state of affairs [project_status.md](project_status.en.md).

## 4. Separate research line (in parallel)

**M1: Real CIOPRG/IONAME (closed processing) ** For each permitted file, record a family DX100/DX200/YRC1000firmware, export method, SHA-256, encoding/newlines, source section map, OEM confirmation tool, distribution status. The private enclosure is not the content of public GitHub Issues/PR/Actions. Only independent synthetic minimum cases are allowed in public. Follow [controller_fixture_intake.md](controller_fixture_intake.en.md).

**M6: semantics.** EN/scan order, flags, byte/word, TMR/CNT/GSTR/GOUT and protected sections disassemble by OEM revisions and permitted bench observations. Without these data, the static analysis remains presumptive. `reference_semantics` - a separate certificate.

**M10: bench testing.** Only a separate approved program for non-hazardous insulated equipment and standard OEMs. Results for DX100, DX200 и YRC1000 Do not automatically transfer between generations. [The Bench Matrix](advanced_hardware_test_matrix.en.md).

## 5. Blocked destinations

| milestones | Status. | What is necessary before even discussing permission |
| --- | --- | --- |
| M9 — scan simulation | NOT EXECUTED | Confirmed scan/time contract and OEM edge cases, separate design review |
| M11 — read-only HSES | Blocked | Separate Owner Decision, Verified OEM Protocol, Read-Only Risk Assessment, Isolated Stand |
| M12 — native writer | Blocked | Complete proven grammar, confirmed by System/User boundaries and OEM round-trip, self-written permission |
| M13 — robot deployment | Blocked | M12, individual rights, safety/recovery plan, OEM approval and owner’s explicit decision |

Neither documenting these requirements nor successful CI/Windows tests **provide permissions to HSES, native writer or deployment.

## 6. Monitoring progress

Update [project_status.md](project_status.en.md) **After actual merge**, fixing SHA and CI. Separately show `implemented`, `synthetically tested`, `Windows CI`, `user smoke`, `OEM bench`, `authorized`. Historical release notes should remain unchanged. In case of discrepancy of documentation with the code, first check the current tests and source codes, then correct the documents in the new PR.

## 7. Ongoing work PR #28

After [merge PR #27](https://github.com/uzbatsu-beep/MLAD/pull/27) baseline `main` — `bdb443eb65dc134edc2e11ffc3230c9dfa6257df`, [main validation SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44237125310). In a separate **Draft PR #28** added [Windows workflow](../.github/workflows/windows-security-acceptance.yml), NTFS exclusive-lock/junction Rust regressions, [synthetic harness](../scripts/windows-acceptance.ps1) и [checker](windows_acceptance_pr28.en.md). WindowsCI results are recorded only after successful workflow on real SHA PR. `UserSmoke`, `OEMBench` and publication of the release remains `NOT RUN`/`BLOCKED`There is no automatic transfer to DONE.

## 8. PR #29 File ID and staged-file guards (Draft only)

The actual baseline after [merge PR #28](https://github.com/uzbatsu-beep/MLAD/pull/28) — `main` SHA `3c184cd36e076dc2e5c9aed7a663b601b4899dea`; [Linux](https://github.com/uzbatsu-beep/MLAD/actions/runs/44365314264) и [Windows](https://github.com/uzbatsu-beep/MLAD/actions/runs/44365314173) CI's successful. In PR #29 JSON filesystem identity/hardlink/staged-file integrity strengthened [Threat and Residual Risk Models](filesystem_security_pr29.en.md) Separately records unproven directory-handle, ACL, TOCTOU and crash durability. This is not permission from a native writer.HSES/FTP. Manual Windows UserSmoke remains `NOT RUN` before the user protocol.
