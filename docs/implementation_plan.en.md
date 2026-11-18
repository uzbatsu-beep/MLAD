<!-- Generated en companion for docs/implementation_plan.md; review required. -->
> **English** · [Русский](implementation_plan.ru.md) · [English](implementation_plan.en.md) · [Original / Оригинал](implementation_plan.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.

# MLAD - an executable development plan after PR #29

**Basic point:** 18.11.2026, `main` SHA `f85ddfcce3063dd087425be981a9d2b7a16d6309`. PR #1–#29 merged. [Linux CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454), [Windows CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621). Last one. [Windows Release — rc.9](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference) released PR #24-#29. manually `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`. After the next merge update the specific baseline in [project_status.md](project_status.en.md).

** Status of plan:** PR #30 Documentation synchronization only; PR #31-#37 - **proposed numbers**, not created PR/agreed deadlines. Each stage requires its own Draft PR, CI and the owner’s explicit decision about merge. The agreed D1-D9 remains unchanged. [Roadmap v3](roadmap.en.md) · [Register of decisions](engineering_decisions.en.md) · [Plan v2 archive](implementation_plan_v2_archive.en.md).

## 1. Priorities of the new cycle

| Order. | Purpose | Why now? |
| --- | --- | --- |
| 0 | Accuracy of documentation after #29 | Outdated statuses and SHAs in MD make planning difficult |
| 1 | Synthetic parser fixture corpus | Record observed edge cases, partial/opaque and byte immutability |
| 2 | Native Workspace: Search and Cross Navigation | User problems with reviewing a large CIOPRG and unknown designs |
| 3 | Versioned portable sidecar | Link notes/bookmarks to SHA without changing the native file |
| 4 | Evidence-driven typed syntax + static analysis | Separate syntax, link verification and unverified execution |
| 5 | Unified Offline Engineering Project | Unified engineering script without native writer |
| 6 | Conditional Windows 0.6.x preview | Only after successful CI, manual acceptance and legal control |

**Parallel tasks: ** Private OEM fixture intake, independent UserSmoke WIN-01-WIN-12, Windows ACL/crash/TOCTOU review, GitHub branch protection/ruleset and third-party license review. Interface upgrades should not automatically include HSES/FTP и native writer.

## 2. Proposed PR backlog and dependencies

| PR | The result | Dependencies | Gate |
| --- | --- | --- | --- |
| **#30 — Docs Baseline / Roadmap v3** | Current MD, the only canonical roadmap, historical archives and risk matrix | merged #29 | `docs:check` + standard CI; docs-only |
| **#31 — Synthetic CIOPRG Parser Corpus** | Synthetic versions of the title, `//$M`, `PART/END`, ALARM/MESSAGE, BOM/newlines, malformed/opaque, oversized | #30 | source spans/SHA-256 byte identity, Rust negative tests |
| **#32 — Native Workspace Navigation & Diagnostics** | Source↔rung↔block/diagnostic, search/filter/keyboard, large-file usability | #31 partially; possibly parallel | All physical lines available, unknown clearly visible |
| **#33 — Portable `*.mlad.json` Sidecar v1** | Versioned schema, source hash/profile, bookmarks/annotations, links to drafts, migrations/conflict | #30, filesystem safety #29 | Roundtrip/relocation/stale-hash/invalid JSON fail-closed |
| **#34 — Typed Opcode Grammar Extension** | OEM-evidence-validated native forms, exact spans and typed/opaque states | #31, confirmed sources | Partial ≠ executed; negative syntax tests |
| **#35 — Source-linked Static Analysis v2** | READ/WRITE index, cross-references, duplicate/conflict diagnostics и confidence | #31/#32/#34 | No false runtime/scan claims, no lost unsupported claims |
| **#36 — Unified Offline Engineering Project** | Native Snapshot + Sidecar + Independent Graph/Advanced JSON drafts | #32/#33/#35 | Source Unchanged, User Data Restored, Safe Save/undo |
| **#37 — Windows OFFLINE Preview 0.6.x RC** | New version with source/portable/NSIS/SHA and release notes | Windows UserSmoke + Legal Gate | Build on tag SHA and **separate release matching** |

Numbers #31-#37 are conditional, and GitHub did not reserve them. Bugfix/security PR can be inserted between stages for real defects and explicit approval.

## 3. PR #31 — Synthetic Native Parser Fixtures (P0)

** Entrance:** [source registry](source_registry.en.md), [CIOPRG baseline](cioprg_lst_spec.en.md), [lossless contract](cioprg_lossless_parser.en.md), [post-END](post_end_data.en.md) и [fixture intake](controller_fixture_intake.en.md). Private sources do not commit; for public tests, recompose synthetic minimal examples without copying unique client snippets.

**Output:** Versionized set of fixtures with manifesto `synthetic origin / controller family assumption / encoding / newlines / expected raw spans / supported status`. Scenarios: ordinary and non-standard headlines, `//$M`order/repeat `PART`, `END`, `ALARM/MESSAGE` after END, undefined mnemonics, malformed operand/address, BOM, CRLF/LF, large files.

**Acceptance:** Original bytes and SHA-256 do not change after parse/page/search; source span/physical line is unchanged; unsupported does not turn into a false rung `PART 1/2` It is not construed as a validated System/User policy; Linux tests and Windows regressions.

## 4. PR #32 — Native Workspace UX (P0)

** Exit:** bundle `source ↔ rung ↔ advanced block ↔ diagnostics` with accurate physical line/offset, opcode and address filters, error group disclosure, compact TMR/CNT/GSTR/GOUT lists, cross-link-to-source, keyboard focus and zoom. The inspector does not limit only the first 600 lines; use bounded pages.

**Acceptance:** Synthetic file of the order of 1400+ lines is fully navigationable; no hidden unknowns; filtered items do not disappear from the model; UI edits of the original `CIOPRG.LST`data after `END` They are not visualized as Ladder. Performance to measure and apply the test environment, without fictitious metrics.

## 5. PR #33 — Portable Sidecar (P1)

** Contract:** `*.mlad.json` Separate user manifest, **not* modified native code. Proposed structure: `schema_version`, `source_sha256`, `controller_family`, optional source provenance (without secrets), bookmarks/notes/manual IONAME associations and links to stand-alone `*.graph.json`/`*.advanced.json`. The schema solution is a separate design within the PR with the owner’s approval; this table is not considered an already agreed scheme.

**Acceptance:** UTF-8/Cyrillic roundtrip; source SHA mismatch `detached/stale`, not silent rebind; unknown version/invalid JSON/broken references → diagnosable failure; migration and backup recovery, normal local Windows paths and filesystem guards #29; `CIOPRG.LST`/`IONAME.DAT` They're never recorded.

## 6. PR #34 — Typed Grammar (P1)

Each opcode requires trace: OEM manual revision/section, controller family, exact operand shape, synthetic positive/negative fixture, raw/typed span, `unknown` for the unsupported. Investigate the error rate of real Native Workspace, but do not disclose private uploads.

**Acceptance:** Correct typed candidates for **confirmed** forms, exact source mapping, unknown command remains opaque; no increase `ManualDerived` до `OEMBenchVerified`There is no PLC scan or native export.

## 7. PR #35 — Static Cross-references (P1)

READ/WRITE index `#XXXXX` и `Mxxx`, references to source, analysis of ambiguous/reassignments, branches and TMR/CNT/GSTR/GOUT *within the limits of evidence*. The result includes source span, severity, support/confidence, and individual statuses. `Unresolved`.

**Acceptance:** Guaranteed immutability of source data, negative and ambiguous cases tests, no false dataflow/coil for unsupported opcodes; scan semantics remains `NotExecuted`.

## 8. PR #36 — Unified Offline Project (P1)

Unified UI session `Native Snapshot (read-only) + sidecar (user data) + editable Graph/Advanced JSON`. Transitions between addresses, chains, blocks, source and notes. Copy verified native subset in JSON draft - only by explicit action, not automatic native→controller compiler.

**Acceptance:** open → find → note → save sidecar → create / modify draft → reopen; all native SHA former, damaged / old sidecar is not masked, write conflicts are reported, there are no boot buttons to the robot.

## 9. PR #37 Windows Preview 0.6.x (Conditional)

** Minimum gates:** required feature PR merged from CI to tag SHA; [Windows manual acceptance WIN-01…WIN-12](windows_acceptance_pr28.en.md) completed and attached; [filesystem threat model](filesystem_security_pr29.en.md) revised; source zip, portable zip/NSIS, GPL-3.0-only, third-party notices/SBOM review, SHA256SUMS, unsigned binary status; release note lists unsupported grammar and `OEMBench NOT RUN`. Dates and release-names are agreed separately.

An official OEM bench is not required for an honest **OFFLINE** preview, but actual compatibility/equivalence with the **not** controller is claimed. Publication only after a separate decision of the owner.

## 10. Security/governance stream (open)

PR #29 File ID/inode + bytes with JSON saving, hardlink rejection SHA-256 staged and disable Windows ADS/UNC/device paths. I didn't decide entirely:

- `check→rename` / parent directory handle TOCTOU и adversarial multi-writer races;
- ACL/owner with staged replacement and read-only permissions
- Atomic transaction for a couple `target + backup` и crash/power-loss durability;
- independent acceptance of Windows WebView2.

These tasks are left **OPEN**, and for new sidecar/edit flows, refine through individual P0 bugfix/security PR as needed. See. [filesystem_security_pr29.md](filesystem_security_pr29.en.md). `main` In fact, GitHub had `protected=false`Enabling required checks/Ruleset is a separate action of the owner, docs PR does not change the settings.

## 11. Private OEM evidence intake

Check each permitted example `CIOPRG.LST` Controller model/firmware, method of export, encoding, original hash, OEM Ladder Editor/version and source of rights. No real files, passwords, business addresses, production IP or fingerprint hashes in public PR/CI. Open-source fixtures use only original synthetic forms. Do not transfer the conclusions between DX100, DX200 и YRC1000 automatically.

## 12. Definition of Done (all future PR)

1. Before the beginning: read the relevant `main` SHA, Open PR and [D1–D9](engineering_decisions.en.md)Describe the amended contract.
2. In commits: synthetic positive and negative tests, verification byte fidelity, documentation, justification unknown/opcode.
3. CI: `npm ci --no-audit --no-fund`, `npm run test:security`, `npm run test:native`other model tests, `npm run docs:check`, `npm run build`, `cargo test --manifest-path src-tauri/Cargo.toml --locked`Windows MSVC/Tauri with IPC/filesystem changes.
4. Status `WindowsCI` separately `UserSmoke` и `OEMBench`; if the test is not done, `NOT RUN`If direction is prohibited, `BLOCKED`.
5. Check prohibitions HSES/FTP/live I/O/native writer/deployment, GDPR/production secrets, legal status of sources.
6. Do not merge PR or publish release without **separate owner approval**; after merge create a new current baseline and do not rewrite historical release files.

[Unified verification gates](verification_gates.en.md) · [Workflow](development_workflow.en.md) · [History of the past PR #27#35 of the plan.](implementation_plan_v2_archive.en.md).
