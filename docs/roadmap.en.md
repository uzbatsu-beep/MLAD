<!-- MLAD-I18N:EDITORIAL:AI-assisted English draft, technical editorial review still required -->
> **English** · [Русский](roadmap.ru.md) · [English](roadmap.en.md) · [Original / Оригинал](roadmap.md)
> **Translation review:** AI-assisted translation; check engineering claims against the Russian original and OEM evidence. `CIOPRG.LST` and `IONAME.DAT` are read-only. `HSES`/`FTP`, native writer and robot deployment remain **BLOCKED**; `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`.

# MLAD — roadmap v4: safe Offline Engineering IDE

**Verified baseline:** `main` = `066db5f8c01a9da8f5f0218f12539b305ab021e4` (merge [PR #39](https://github.com/uzbatsu-beep/MLAD/pull/39), 19.11.2026). **History:** PR #1–#39 merged. [PR #39 CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44571515321) passed on the PR head; this is not manual Windows acceptance, nor independent CI evidence on the new main merge commit.

**Last verified published Windows release:** [`v0.5.0-rc.9-oem-reference`](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference) (17.11.2026), does not include PR #24–#39. `UserSmoke=NOT RUN`; `OEMBench=NOT RUN`. Historical v3 is preserved in Git: [roadmap @ pre-PR40 main](https://github.com/uzbatsu-beep/MLAD/blob/066db5f8c01a9da8f5f0218f12539b305ab021e4/docs/roadmap.md).

## 1. Product contract and unchanged decisions D1–D9

MLAD is an independent OFFLINE/BENCH environment for inspection of exported Yaskawa Concurrent I/O; controller families `DX100`, `DX200`, `YRC1000`. The main workspace can inspect `CIOPRG.LST` and separately `IONAME.DAT`, while editing is restricted to independent user JSON drafts.

- `CIOPRG.LST` / `IONAME.DAT`: **read-only**, original bytes and `SHA-256` remain unchanged; native writer/export is not implemented.
- `PART 1/2`, `//$M`, `END` do not grant System/User Ladder edit permission.
- `*.graph.json`, `*.advanced.json` and legacy JSON are drafts, not controller programs; proposed `*.mlad.json` is a separate sidecar, not a native format.
- `TypedCandidate`, `ManualDerived`, `SyntheticTested` do not mean OEM-verified execution; `unknown` / `opaque` / `malformed` must never become fabricated Ladder logic.
- `HSES`/`FTP`, live I/O, native writer и robot deployment **BLOCKED** until separately authorized.
- Do not commit actual customer files or sensitive information; use authorized private inputs and public `SYNTHETIC` fixtures.
- Each technical stage needs its own PR and tests; merge only on explicit owner approval. [Decisions D1–D9](engineering_decisions.en.md).

## 2. Verified implementation history, PR #1–#39

| PR | Delivered | Limitation |
| --- | --- | --- |
| #1–#3 | Memory typing, lossless source and Boolean AST | Not a complete OEM parser |
| #4–#8 | Graph/Advanced JSON, TMR/CNT/GSTR/GOUT subset, tri-state and inline editing | Independent drafts only |
| #9–#12 | v2 docs, GPL-3.0-only, npm lockfile / npm ci | Third-party license audit open |
| #13–#18 | Compiled FTP/HSES removed; Native Workspace, paging/search, IONAME, post-END data | Partial grammar/UX |
| #19–#21 | MOV/ADD/SUB/SHL/SHR candidates and static hypotheses | EN/flags/scan unverified |
| #22–#29 | Security/docs, staged JSON, guarded reads, CSP, File ID, Windows regression | ACL/TOCTOU/crash risks open |
| #30–#35 | Roadmap v3, RU/EN localization, editorial audit | No parser corpus/sidecar delivered |
| #36–#39 | Housekeeping, unified fail-closed ci-gate, docs-only probe, 15 workflows archived | Ruleset still requires frontend-build |

## 3. Functional stages M0–M13

| Stage | Status | Next acceptance/result |
| --- | --- | --- |
| M0 — Git/CI/legal | **DONE core / PARTIAL governance** | Ruleset requires `frontend-build`, not yet `ci-gate` |
| M1 — OEM evidence | **PARTIAL** | Private real fixtures, firmware, independent OEM review |
| M2 — Filesystem safety | **PARTIAL** | ACL, TOCTOU, crash/backup recovery |
| M3 — Native Workspace | **DONE baseline** | Read-only UI exists; usability needs work |
| M4 — Parser fidelity | **PARTIAL** | Synthetic corpus and honest coverage metrics — PR #40 |
| M5 — Navigation/diagnostics | **PARTIAL** | Cross-navigation, filters and large-file UX — PR #41 |
| M6 — Typed grammar/static analysis | **PARTIAL** | New forms require OEM evidence; PR #44–#45 |
| M7 — `*.mlad.json` | **OPEN** | Sidecar schema/backup/source binding — PR #43 |
| M8 — Unified Offline IDE | **OPEN** | One engineering workflow without native writer — PR #46 |
| M9 — Windows acceptance | **PARTIAL** | CI exists; `UserSmoke=NOT RUN` |
| M10 — OEM bench | **NOT RUN** | No documented model, firmware and authorized bench results |
| M11 — read-only HSES | **BLOCKED** | Separate OEM/security/owner authorization required |
| M12 — native writer | **BLOCKED** | Full grammar and safe OEM round-trip are not proven |
| M13 — deployment | **BLOCKED** | No robot program deployment may be activated |

## 4. Proposed backlog after PR #39

| PR | Priority | Scope |
| --- | --- | --- |
| #40 | P0 | Native Parser Fidelity — synthetic fixture corpus, coverage, spans, diagnostics UI; the present PR starts with documentation |
| #41 | P0 | Native Workspace UX v2 — source↔rung↔block↔diagnostic, filters, large files |
| #42 | P0 | Filesystem/dependency/security: ACL/TOCTOU/backup, unused FTP dependencies, Ruleset |
| #43 | P1 | Portable `*.mlad.json` sidecar v1, versioning, bookmarks, source SHA, conflict/recovery |
| #44 | P1 | Evidence-driven typed grammar, restricted to OEM-supported forms |
| #45 | P1 | Source-linked static READ/WRITE/UNKNOWN; explicitly surface ambiguity |
| #46 | P1 | Unified Offline Project: immutable native snapshot + sidecar + JSON drafts |
| #47 | P1 | Conditional Windows OFFLINE preview 0.6.x after manual smoke/legal/release gate |

PR numbers after #40 are planning labels and not reserved by GitHub. PR #40 may begin with documentation baseline, but remains functionally incomplete until parser corpus/coverage code and CI evidence are delivered.

## 5. Engineering priorities

1. **P0 — source fidelity:** `SYNTHETIC` corpus, exact byte spans and line endings, honest coverage without invented syntax.
2. **P0 — navigation:** `source↔rung↔block↔diagnostic`, unknown filters, large files and verified physical-line jumps.
3. **P0 — security:** fix discovered issues independently of UX; `suppaftp` in Cargo.toml needs separate review.
4. **P1 — engineering session:** `*.mlad.json` with bookmarks/annotations and source binding; JSON editors remain separate.
5. **P1 — semantics:** each opcode extension: OEM manual/version → typed grammar → positive/negative fixture → explicit UNKNOWN.
6. **Cross-cutting gate:** `WindowsCI` ≠ `UserSmoke` ≠ `OEMBenchVerified`. No release is automatically authorized.

## 6. Governance, filesystem and release blockers

- GitHub Ruleset `MLAD — main branch protection` is active, but its required status check is `frontend-build`, not `ci-gate`. Change it only after validation and a separate owner decision.
- Residual local filesystem risks: `check→rename` TOCTOU, Windows ACL/owner, hardlinks после проверки, concurrent writers, atomic `*.bak` + target, crash/power-loss durability.
- A new Windows offline 0.6.x release requires acceptance and a separate decision; do not rewrite published rc.9.
- `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`; `HSES`/`FTP`/native write/deploy **BLOCKED**.

## 7. Documentation and traceability

[Implementation plan](implementation_plan.en.md) · [Project status](project_status.en.md) · [Decisions](engineering_decisions.en.md) · [Verification gates](verification_gates.en.md) · [Private fixture intake](controller_fixture_intake.en.md) · [Filesystem risk review](filesystem_security_pr29.en.md).

## Reconciliation after merged PR #40 — 20 November 2026

PR #40 merged as a documentation update into main (2f28d5c0941c7495c2577fa4839eabc28d52298e). The Parser Fidelity implementation is now in separate PR #41: a synthetic fixture corpus, full-source per-physical-line classification, original byte offsets and unknown-syntax diagnostics. The previously proposed Native Workspace UX PR #41 moves to PR #42 or later. Subsequent Security, Sidecar, Typed Grammar, Static Analysis, Unified IDE and Windows Preview numbers are provisional, not assigned to open PRs.

Exact-final-HEAD checks and explicit owner approval are mandatory before merging. CIOPRG.LST / IONAME.DAT are read-only. HSES/FTP, native writer and deployment are **BLOCKED**. `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`. Synthetic tests do not establish OEM scan behavior or System/User permissions.

## Current queue after merged PR #41 — UX in PR #42

PR #41 Parser Fidelity merged into `main` SHA `0c1b233354564d9fd3c242675a78a45417feacc1`. PR #42 — Native CIOPRG Offline Workspace UX v2 — is a separate Draft. Its scope: source↔rung↔advanced block↔diagnostic, render only the selected rung/block, keyboard navigation, and entire-source paged filtering by `SyntaxClass`, opcode and exact textual address.

Proposed later work (numbers not reserved): PR #43 security/dependencies, PR #44 portable sidecar, PR #45 typed grammar, PR #46 static analysis, PR #47 Unified Offline Project, PR #48 Windows offline preview. Previous v4 numbering is historical planning, not assigned PRs.

CIOPRG.LST/IONAME.DAT remain read-only; HSES/FTP, native writer and deployment **BLOCKED**. Address matches are textual, not proof of a scan. `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`; merging requires explicit owner approval.

## PR #43 — filesystem security and dependency review (Draft, 20 November 2026)

**Current baseline:** PR #42 merged into `main` SHA `73e675f7df33c5751a35e29752736c3470b11b31`. Separate PR #43 remains Draft, not part of `main`. Concrete changes: remove unused FTP crate `suppaftp` and five exclusively reachable transitive dependencies from `Cargo.lock`, plus two unused direct requirements `anyhow` and `futures-util` (their transitive crate packages remain); owner-only `0600` for new Unix staged JSON and backups; reject staged files with additional hardlinks; verify File ID before cleanup of a temporary file.

**M2 remains PARTIAL, not CLOSED.** `check→rename` remains non-atomic; Windows ACL/DACL/owner and replacement permission inheritance are unverified; target and `*.bak` are not an atomic transaction; multi-writer races, power-loss durability and file systems outside the local testing contract remain open. The `0600` mode intentionally makes previously shared Unix JSON drafts private.

The next task after PR #43 (subject to separate approval) is the portable offline sidecar `*.mlad.json` with source SHA-256 binding and a separate write policy review. `CIOPRG.LST`/`IONAME.DAT` stay read-only; `HSES/FTP`, native writer and deployment remain **BLOCKED**. `UserSmoke=NOT RUN`, `OEMBench=NOT RUN` absent separate evidence.

## PR #44 — portable offline sidecar v1 (20 November 2026)

**Current baseline:** PR #43 merged into `main` `c36dc4bc4f84b3dfdb2eac52625c3c0389226320`. PR #44 is a separate Draft. Stage M7 moves from OPEN to **IN PROGRESS**, not DONE until merged and accepted.

**Artifact:** standalone `*.mlad.json` with `format=MLAD_OFFLINE_SIDECAR`, `schema_version=1`, `source.kind=CIOPRG.LST`, `source.controller`, `source.sha256`, `source.byte_length`, `source.line_count`, `annotations`, `bookmarks`. Native `CIOPRG.LST` bytes or path, controller algorithms and HSES/FTP contents are never copied into the sidecar. IONAME note keys reference the IONAME SHA-256, not an inferred automatic relay map.

**Workflow:** open read-only CIOPRG, choose separate `*.mlad.json`, load/save notes and physical-line bookmarks. Backend rereads the original source and validates `SHA-256`, controller profile and sizes. First save requires an absent target; updates need a `revision` based on exact bytes and file ID. Existing staged JSON persistence produces `*.bak`. Source/format mismatches are rejected rather than falling back to a native writer.

**Residual risks:** file ID plus revision is optimistic, not an OS-atomic CAS. `check→rename` TOCTOU, Windows ACL preservation, multi-process writes, atomic target+backup and crash recovery remain OPEN. `CIOPRG.LST`/`IONAME.DAT` remain read-only; FTP/HSES, native writer and robot deployment are **BLOCKED**. `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`. Next proposed phase: PR #45 typed grammar.

## Current sequence: PR #45 — OEM-evidenced extended typed grammar (20 November 2026)

PR #44 portable sidecar merged into `main` SHA `48522c1e2d0ee41560ce61ba0d2aa8af14e1c6d8`. PR #45 is a separate, unmerged **Draft**. Stage M6 expands **syntax-only** classification by 15 YRC1000 R12-documented opcodes in addition to five previous forms. Operand roles and values are checked as `TypedCandidate`; new opcodes remain barriers in the boolean/execution analysis and do not generate native Ladder. The source/role matrices live in `docs/typed_opcode_grammar.md` and `docs/source_registry.md`.

**Acceptance gate:** documented OEM 178648-1CD §§12.2.15–34, 15 positive / 17 negative synthetic forms, exact byte spans/SHA/EOL, faithful unknown lines, final SHA `frontend-build`, `rust-tests`, `windows-security`, `ci-gate`. The new forms have no DX100/DX200 family evidence. `MUL Dn+1` and `BMOV n` require special syntactic constraints with explicit provenance.

Proposed, not-yet-started next tasks: PR #46 source-linked static READ/WRITE/UNKNOWN with conservative barriers, PR #47 Unified Offline Project and conditional PR #48 Windows Preview. Older v4 numbers above are historical. `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`; `CIOPRG.LST`/`IONAME.DAT` read-only; HSES/FTP/native writer/deployment **BLOCKED**. Owner must explicitly approve PR #45 merging.
