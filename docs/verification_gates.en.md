<!-- Generated en companion for docs/verification_gates.md; review required. -->
> **English** · [Русский](verification_gates.ru.md) · [English](verification_gates.en.md) · [Original / Оригинал](verification_gates.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.
> **MLAD safety invariants:** `CIOPRG.LST` and `IONAME.DAT` are strictly **read-only**. FTP/HSES, live I/O, native writer and robot deployment remain **BLOCKED**. Passing `WindowsCI` does **not** imply `UserSmoke=PASS` or `OEMBenchVerified`: both remain **NOT RUN** unless independently documented. Machine translation requires source and Yaskawa manual review.

# MLAD – Uniform Verification, CI and Release Criteria

**Update:** 18.11.2026. `main` @ `f85ddfcce3063dd087425be981a9d2b7a16d6309`: PR #1–#29 merged, [Linux CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454) и [Windows CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621). Automatic Windows Verification does not include independent manual verification. WebView2OEM stand.

This document is an acceptance contract for new PR and offline preview. No substitute [matrix](advanced_hardware_test_matrix.en.md) or official safety regulations.

## 1. Independent levels of evidence

| Level. | What does it mean? | Which doesn't mean |
| --- | --- | --- |
| `Documented` | The fact is confirmed by a specific OEM manual, version / section is fixed | Performance on this robot/firmware |
| `SyntheticTested` | Test successfully performed on artificial data | Compatibility with the real program |
| `ManualDerived` | Formula and reference result reconstructed from documentation | Real scan/dataflow и Live I/O |
| `WindowsCI` | Build/autotest on Windows runner is successful | Manual smoke in the user application |
| `UserSmoke` | Manual passage of a specific scenario and OS | OEM equivalence |
| `OEMBenchVerified` | Repeated verification on specified controller, firmware and OEM tool | Permission to export/load from MLAD |
| `Authorized` | The owner separately allowed a strictly limited operation. | Automatic permission for other devices/functions |

Record the result: `PASS / FAIL / NOT RUN / BLOCKED`, date, commit SHA, controller/firmware (where applicable), evidence link and limitations.

## 2. Primary CI – mandatory for every PR

Start from pure navigation tree, by lockfiles:

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

CI `.github/workflows/validation.yml` launcher `frontend-build` и `rust-tests`. For Rust/Tauri/filesystem/security changes, a separate Windows MSVC + Tauri check is required unless equivalent up-to-date verification is covered by workflow. For docs-only PR manual Windows/hardware test is not required, but status - `NOT RUN (docs-only)`.

## 3. Change audits

| The Domain | Positive tests | Negative/borderline tests |
| --- | --- | --- |
| `CIOPRG.LST` reader | Search, paging, exact source spans and SHA-256 | BOM/newlines, malformed/oversized, unsupported/opaque, read-only, file swap/reparse |
| `IONAME.DAT` | Bounded preview, manual associations | Unknown map, oversized, incorrect index, immutable bytes |
| Native Workspace | All available strings; sourceиrung; compact blocks | Unsupported does not turn into Ladder, offscreen/large source, false LIVE |
| Typed/semantic | Types, ranges and links with provenance | Unknown opcode, ambiguous alias/operand, EN/scan unknown, `PART/END` boundaries |
| JSON draft/sidecar | Save/reopen, backup, schema version, source hash match | Concurrent edit, read-only path, oversize, symlink/junction, damaged JSON, wrong hash, no native overwrite |
| CSP/Tauri IPC | Local UI and Authorized Commands Work | No remote scripts/connect, `unsafe-eval`, HSES/FTP/write/robot deployment |
| Windows preview | Build, installer/portable checksums, source archive | Actual checks WebView2Refuse/restore, blocked file, ACL; unsigned status explicitly stated |

## 4. Mandatory manual Windows smoke (after technical PR and before release)

- [ ] Lock Windows version, WebView2, application version/commit, date and performer.
- [ ] Open the **copy** `CIOPRG.LST` through system open, write down the original SHA-256 reconfirm SHA-256 after watching.
- [ ] Check full source/paging, search and line transitions, recognized chains, partial and opaque sections; there is no false “LIVE”.
- [ ] Download the authorized copy `IONAME.DAT`Check for local comments, no DAT change and incorrect automatic OEM binding.
- [ ] Create, open and save twice `*.graph.json` и `*.advanced.json`check `*.bak`Failure to record in an unacceptable place and external editing/conflict scenario.
- [ ] Check packaged Tauri CSP: there is no lock on legitimate local IPC, external resources are not loaded; consider this a functional smoke check, not penetration test.
- [ ] Check for large/erroneous/damaged synthetic files and no UI freeze.
- [ ] Ensure that download/monitoring/writer buttons cannot initiate actions on the robot; **do not connect existing industrial equipment**.
- [ ] Save the PASS/FAIL/NOT RUN result and evidence without publishing these programs/addresses/passwords.

## 5. Conditions for Release Candidate Admission

1. All applicable CIs are green on the **same SHA* used for the build/tag.
2. No regression. `original_source_bytes` and safety deny-only borders.
3. Windows Smoke is executed and documented. `NOT RUN`The release does not have to approve manual acceptance.
4. Release assets: portable ZIP, NSIS, where applicable, source archive specific tag, `SHA256SUMS.txt`, GPL-3.0-onlyThird-party license review/notifications and explicit status of unsigned executable file.
5. Updated `README.md`, `docs/project_status.md`, `docs/roadmap.md` and related engineering documents.
6. There are no real robots or secrets in commits/logs/artifacts.
7. A separate decision by the owner to merge and, later, a *separate* decision to publish the release.

**No False Confirmation: * No CI, screenshot, or synthetic test translates M10-M13 into a `DONE`. Recording in the controller and native writer is prohibited until self-admission.

## 6. Evidence template for PR

| Field. | Meaning. |
| --- | --- |
| PR / base SHA / head SHA | Specify exactly. |
| Modified contracts | Format/IPC/UX and Controller Applicability |
| Actual Linux CI / Windows CI | URL + status |
| Positive/negative tests | Names and results |
| Windows UserSmoke | PASS/FAIL/NOT RUN |
| OEM Bench | PASS/FAIL/NOT RUN/BLOCKED, model and firmware |
| Security | immutable original; HSES/FTP/writer disabled |
| Residual risk | Specific limitation |
| Merger | Separate approval of the owner |

Supplement: [development_workflow.md](development_workflow.en.md), [implementation_plan.md](implementation_plan.en.md), [source_registry.md](source_registry.en.md).

## 7. Repository rules and powers

**Observation 18.11.2026:** `main` GitHub is returning as `protected=false`. Even successful workflow does not prevent manual bypassing the process without branch protection. Separately, docs PR is recommended to configure GitHub Ruleset/Branch Protection with mandatory checks. `frontend-build` и `rust-tests`Prohibition of direct push and review procedure available for the current account type. The technical rule does not replace the owner's explicit decision to merge. Documentation PR #27 does not change the settings of the repository.

## 8. PR #28 — Windows Acceptance & Security Regression

[Windows regression workflow](../.github/workflows/windows-security-acceptance.yml) It works on the relevant PR/push in `main` and can be manually started after appearing in the default branch. Checks Node security contracts, Rust file integrity/NTFS exclusive lock/junction, locked dependencies, Tauri production EXE, binary hash/PE evidence. Temporary unsigned artifact is stored for 14 days and is not a release. [Detailed protocol](windows_acceptance_pr28.en.md).

On baseline. `f85ddfc` WindowsCI **PASS** ([run](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621)); for every **new PR** this level is again `NOT RUN` before running Windows workflow on its HEAD SHA. After a successful CI, only WindowsCI can be changed `PASS`. manually WebView2 `UserSmoke` WIN-01...WIN-12 and OEM Bench remain `NOT RUN` before the independent protocol. UI verification, control-system connectivity, absolute TOCTOU protection and full backup+target atomicity do not follow from `cargo test` и `tauri build`.

## 9. PR #29 — File ID/Hardlink/Staging (merged)

[Threat model and acceptance](filesystem_security_pr29.en.md): be sure to check the comparison of JSON bytes ** and ** open file identity (Windows volume/file index, Unix dev/inode), ban hardlinks on JSON and backup, re-check staged temporary file on File ID +SHA-256 and return fail-closed when detected substitution. CI: Linux Rust and Windows NTFS/Rust on the same SHA PR, Node security, Tauri EXE. **Do not pass for full TOCTOU closure or ACL/backup+target assurance. UserSmoke and OEM Bench remain independent.

## 10. Gates of the next cycle PR #30–#37

| Phase | Mandatory evidence | Unacceptable false PASS |
| --- | --- | --- |
| #30 Docs | `docs:check`Relevance of README/status/roadmap, linking archives | Docs CI ≠ Windows UserSmoke |
| #31 Parser fixtures | Synthetic byte spans, SHA-256, malformed/opaque negative tests, metadata manifest | Do not call synthetic OEM files golden |
| #32 Native UX | 1400+ lines, full paging/sourceикаrung, diagnosis unknown, keyboard/navigation smoke | Do not declare full semantics on the graph |
| #33 Sidecar | Schema/version, source SHA bind, stale/missing ref rejection, JSON write safety | Sidecar isn't an OEM writer |
| #34 Typed grammar | OEM manual/version evidence, explicit family, typed/partial/opaque, positive+negative tests | TypedCandidate ≠ scan-executed |
| #35 Static analysis | Source-linked READ/WRITE refs, ambiguous cases, false positive guards | Do not confirm physical scans/flags |
| #36 Unified IDE | UI end-to-end open→annotate→JSON save/reopen, original bytes unchanged | Native export/deploy banned |
| #37 0.6.x RC | Exact tag SHA, Linux/Windows CI, **manual Windows UserSmoke**, legal/source/SHA assets, owner approval | CI   OEM bench; release is not automatic |

**Open filesystem area:** Protection PR #29 from known hardlinks/ADS does not make an atomic operation check→rename, does not carry existing ACLs guaranteed and does not solve a two-file transaction. For new user formats check [threat review](filesystem_security_pr29.en.md) To merge. Separate [Windows protocol](windows_acceptance_pr28.en.md) Mandatory until preview.
