<!-- MLAD-I18N:EDITORIAL:reviewed 2026-11-19 against docs/verification_gates.md -->
> **Language / Язык:** [English](verification_gates.en.md) · [Русский](verification_gates.ru.md) · [Original](verification_gates.md)
>
> **Scope:** This is a test-evidence and acceptance contract, not a robot commissioning procedure. Do not equate CI, GUI smoke testing, OEM bench validation and authorization.

# MLAD — Verification, CI and Release Acceptance Gates

**Baseline updated:** 18 November 2026. `main` at `f85ddfcce3063dd087425be981a9d2b7a16d6309`: PR #1–#29 merged, [Linux CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454) and [Windows CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621). Automated Windows checks did **not** include independently performed WebView2 user testing or a real OEM controller bench.

This document defines acceptance evidence for new pull requests and offline previews. It does **not** replace the [hardware test matrix](advanced_hardware_test_matrix.en.md) or a manufacturer-approved safety procedure.

## 1. Independent evidence levels

| Level | What it confirms | What it does **not** confirm |
| --- | --- | --- |
| `Documented` | A claim is supported by a specified manufacturer manual and section/revision | Behavior on the actual robot and firmware |
| `SyntheticTested` | A test passed on deliberately constructed synthetic input | Compatibility with a real controller program |
| `ManualDerived` | A formula and reference outcome were derived from documentation | Real scan order/dataflow or live I/O |
| `WindowsCI` | Automated build/tests passed on a Windows runner | GUI smoke testing on the user's machine |
| `UserSmoke` | A person documented executing a specific UI scenario on a named OS/application build | Equivalence with the OEM controller |
| `OEMBenchVerified` | A repeatable test was performed against a specified controller, firmware and OEM tool | Permission to export or deploy a native file from MLAD |
| `Authorized` | The owner separately permitted a precisely defined operation | Permission for any other device, function or environment |

Record: `PASS / FAIL / NOT RUN / BLOCKED`, test date, commit SHA, model/firmware where applicable, evidence URL and remaining limitations.

## 2. Main CI — required for each PR

Run against a clean checkout with committed dependency lockfiles:

```bash
npm ci --no-audit --no-fund
npm run test:community
npm run test:security
npm run test:native
npm run test:diagnostics
npm run test:oem-reference
npm run test:graph
npm run test:advanced
npm run test:visual
npm run test:inline
npm run docs:check
npm run build
cargo test --manifest-path src-tauri/Cargo.toml --locked
```

The [validation workflow](../.github/workflows/validation.yml) (`.github/workflows/validation.yml`) runs `frontend-build` and `rust-tests`. Changes to Rust, Tauri, filesystem behavior or security require a corresponding Windows MSVC/Tauri check unless an equivalent current workflow already covers them. A documentation-only PR does not require manual Windows or hardware testing, but must report it as **`NOT RUN (docs-only)`**, not PASS.

## 3. Change-specific checks

| Area | Positive tests | Negative and boundary cases |
| --- | --- | --- |
| `CIOPRG.LST` reader | Search, pagination, exact byte/source spans and SHA-256 | BOM/newline variations, malformed or oversized files, unsupported/opaque forms, read-only preservation, file swap and reparse |
| `IONAME.DAT` | Bounded read-only preview and explicit manual associations | Unverified name-to-address maps, oversized input, invalid indices and exact native byte preservation |
| Native Workspace | Every accessible source line, source↔rung navigation, compact block view | Unsupported source must not become executable Ladder; offscreen/large-source issues and false LIVE indications |
| Typed syntax / semantic analysis | Validated operand types, ranges, provenance and references | Unknown opcodes, ambiguous aliases or operands, unknown EN/scan behavior and `PART/END` boundaries |
| JSON drafts / sidecar | Save/reopen, backup, schema version and source-hash binding | Concurrent edits, non-writable path, oversize, symlink/junction, damaged JSON, wrong hash and native overwrite attempts |
| CSP / Tauri IPC | Permitted offline UI and IPC remain functional | Remote script/loading, `unsafe-eval`, HSES/FTP/robot writing/deployment |
| Windows preview | Application build, installer/portable checksums and matching source archive | Actual WebView2 checks, locked-file refusal/recovery, ACL issues; clearly declare unsigned status |

## 4. Required manual Windows smoke — after implementation and before release

- [ ] Record Windows version, WebView2 version, application build/commit, date and tester.
- [ ] Open a **copy** of `CIOPRG.LST` with the native Open dialog, record the original SHA-256, and verify it again after viewing.
- [ ] Inspect complete source pagination, search and line jumps; check recognized rungs, partially supported and opaque segments; there must be no misleading LIVE indication.
- [ ] Open an authorized **copy** of `IONAME.DAT`, check local annotations and that native bytes remain unchanged; never infer an unverified automatic OEM address mapping.
- [ ] Create, open and save `*.graph.json` and `*.advanced.json` twice. Test `*.bak`, write refusal in unsupported locations and external-edit conflict handling.
- [ ] Test the packaged Tauri CSP: legitimate local IPC must work and external resources must remain blocked. Treat this as functional smoke testing, **not** a penetration test.
- [ ] Open large, malformed and damaged **synthetic** files and confirm that the UI does not hang.
- [ ] Verify that upload, monitoring and writer controls cannot operate on a robot; **do not connect to live industrial equipment**.
- [ ] Save PASS/FAIL/NOT RUN and evidence, without publicly exposing actual programs, IP addresses or passwords.

## 5. Conditions for release-candidate approval

1. All applicable CI checks pass on **the exact SHA** used to build/tag the release.
2. No regression in `original_source_bytes` or safety deny-only boundaries.
3. Manual Windows smoke is performed and documented. If it is `NOT RUN`, the release must **not** claim completed manual acceptance.
4. Assets include portable ZIP, NSIS installer where applicable, source archive for the exact tag, `SHA256SUMS.txt`, GPL-3.0-only, third-party notices/license review, and an explicit statement that the executable is unsigned when relevant.
5. Update `README.md`, `docs/project_status.md`, `docs/roadmap.md` and affected engineering documentation.
6. No actual customer/controller source files or secrets appear in commits, logs or uploaded artifacts.
7. The owner separately approves **merge**, and later separately approves **publishing a release**.

**Do not fabricate validation:** CI, screenshots and synthetic tests cannot mark M10–M13 as `DONE`. Writing to a robot or producing native CIO programs remains prohibited without independent approval.

## 6. PR evidence template

| Field | Required value |
| --- | --- |
| PR / base SHA / head SHA | State exact commits |
| Changed contracts | Format, IPC, UX and applicable controller families |
| Actual Linux CI / Windows CI | Workflow URL and status |
| Positive / negative tests | Test names, evidence and results |
| Windows UserSmoke | PASS / FAIL / NOT RUN |
| OEM Bench | PASS / FAIL / NOT RUN / BLOCKED, model and firmware |
| Security | Native source immutable; HSES/FTP/writer disabled |
| Residual risk | Concrete limitation |
| Authority to merge | Separate explicit owner approval |

Related: [development workflow](development_workflow.en.md), [implementation backlog](implementation_plan.en.md), [source registry](source_registry.en.md).

## 7. Repository protections and authority

**Observation dated 18 November 2026:** GitHub reported `main` as `protected=false`. A green workflow does not stop manual bypass in the absence of branch protection. A separate GitHub Ruleset/Branch Protection configuration is recommended, requiring `frontend-build` and `rust-tests`, restricting direct pushes and requiring appropriate review for the account's plan. **Technical branch rules are not a substitute for explicit owner permission to merge.** PR #27 did not modify repository settings.

## 8. PR #28 — Windows Acceptance & Security Regression

The [Windows regression workflow](../.github/workflows/windows-security-acceptance.yml) runs for relevant PRs/pushes to `main` and can be launched manually. It checks Node security contracts, Rust filesystem integrity, NTFS exclusive locks and junctions, lockfiles, production Tauri EXE build and PE/hash evidence. Its **temporary unsigned artifact**, retained for 14 days, **is not a GitHub release**. See the [manual acceptance protocol](windows_acceptance_pr28.en.md).

On baseline `f85ddfc`, `WindowsCI=PASS` ([workflow](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621)). Each **new PR** must establish its own WindowsCI result on the new HEAD SHA. Only WindowsCI changes to `PASS` after CI. Manual `UserSmoke` (WIN-01–WIN-12) and OEMBench remain `NOT RUN` without independent reports. Running `cargo test` or `tauri build` does not establish real GUI behavior, controller connectivity, complete TOCTOU protection or atomic backup+target behavior.

## 9. PR #29 — File ID / Hardlinks / Staging (merged)

[Threat model and acceptance criteria](filesystem_security_pr29.en.md) require JSON content **and OS-handle File ID** comparisons (Windows volume/file index; Unix dev/inode), rejection of hardlinked JSON targets/backups, staged-file File ID+SHA-256 verification, and fail-closed behavior when tampering is detected. Relevant CI includes Linux Rust and Windows NTFS/Rust on the same PR SHA, Node security checks and Tauri EXE build. Passing these checks is **not** proof that all TOCTOU races, ACL issues or atomicity of the backup+target pair are solved. UserSmoke and OEM Bench remain independent.

## 10. Gates for the next proposed PR #30–#37 cycle

| Stage | Required evidence | Invalid inference |
| --- | --- | --- |
| #30 Docs | `docs:check`, current README/status/roadmap and valid historical links | Docs CI ≠ Windows UserSmoke |
| #31 Parser fixtures | Synthetic byte spans, SHA-256, malformed/opaque negative tests and fixture manifest | Synthetic fixtures are **not** OEM golden exports |
| #32 Native UX | 1,400+ lines, complete paging/source↔rung, unknown diagnostics and keyboard navigation | A diagram does **not** establish complete program semantics |
| #33 Sidecar | Schema/version, source SHA binding, missing/stale reference checks, JSON safety | Sidecar is **not** an OEM native writer |
| #34 Typed grammar | Revisioned OEM evidence, explicit controller family, typed/partial/opaque and positive/negative tests | `TypedCandidate` ≠ executed controller scan |
| #35 Static analysis | Source-linked READ/WRITE refs, ambiguous cases, false-positive checks | No confirmed physical scan/flag behavior |
| #36 Unified IDE | End-to-end open → annotate → JSON save/reopen, original bytes unchanged | Native export/deployment **prohibited** |
| #37 0.6.x RC | Exact tag SHA, Linux/Windows CI, **manual Windows UserSmoke**, legal/source/hash assets and owner approval | CI ≠ OEM bench; publication is not automatic |

**Open filesystem concerns:** PR #29 hardlink/ADS guards do not make `check→rename` atomic, do not guarantee inheritance/preservation of Windows ACLs and do not create a single transaction for target plus backup. For new editable formats, review the [filesystem threats](filesystem_security_pr29.en.md) before merging. The separate [manual Windows protocol](windows_acceptance_pr28.en.md) remains required before an offline preview.
