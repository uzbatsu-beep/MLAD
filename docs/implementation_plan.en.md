<!-- Generated en companion; machine-generated draft; editorial review required. -->
> **English** · [Русский](implementation_plan.ru.md) · [English](implementation_plan.en.md) · [Original / Оригинал](implementation_plan.md)
> **Translation review:** AI-assisted translation; check engineering claims against the Russian original and OEM evidence. `CIOPRG.LST` and `IONAME.DAT` are read-only. `HSES`/`FTP`, native writer and robot deployment remain **BLOCKED**; `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`.

# MLAD — implementation plan v4 after PR #39

**Baseline:** `main` @ `066db5f8c01a9da8f5f0218f12539b305ab021e4`, PR #1–#39 merged. Numbers #40–#47 are planning identifiers, not automatically created or completed PRs. Markdown updates alone do not establish functional completion of PR #40. [Roadmap v4](roadmap.en.md) · [Project status](project_status.en.md).

## 1. Priorities and scope boundaries

- **P0:** verified parser corpus, immutable native snapshot, honest diagnostics; then reliable navigation.
- **P0:** independent security/Windows tests before adding any user writer.
- **P1:** portable sidecar and Unified Offline IDE; expand OEM-evidenced syntax incrementally.
- `CIOPRG.LST`/`IONAME.DAT` **read-only**; `HSES`/`FTP`/native writer/deployment **BLOCKED**; `PART`/`END` do not establish System/User permissions.
- `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`; public GitHub must use synthetic fixtures only.

## 2. PR #40 — Native Parser Fidelity & Coverage (draft scope)

**Goal:** do not simulate complete OEM grammar; instead account for recognized, partial and unsupported source data reproducibly without losing bytes.

**Deliverables:**
1. `SYNTHETIC` fixture corpus v1 with at least 12 independent scenarios and a machine-readable manifest: provenance, profile, encoding/EOL, source hash, expected line spans and diagnostics.
2. Classification: `recognized`, `typed candidate`, `unsupported`, `malformed`, `opaque`, `post-END data` — syntax coverage only, not execution verification.
3. Coverage summary for the entire source; separate parser/semantic diagnostics and explicit truncation state.
4. `NativeWorkspace.tsx`: correct the incorrect tooltip `Byte offset`, which currently shows the line number rather than `body_span.start`; display the actual byte range.
5. Synchronize `docs/roadmap.md`, `docs/implementation_plan.md`, `docs/project_status.md` and their RU/EN companions; validate docs/i18n.

## 3. Synthetic fixture matrix v1

| ID | Test case |
| --- | --- |
| F01 | Minimal boolean rung, exact spans |
| F02 | BOM/UTF-8 and non-UTF8 variant |
| F03 | LF/CRLF/CR and absent final EOL |
| F04 | Empty and EOL-only file |
| F05 | Valid `//$M`, eight rows with ten cells |
| F06 | Truncated and malformed `//$M` |
| F07 | `PART` markers and a valid `END` |
| F08 | Malformed `END`: later lines stay visible |
| F09 | `//ALARM`/`//MESSAGE` after `END` |
| F10 | Unsupported opcode between boolean instructions |
| F11 | Invalid operands and typed candidates |
| F12 | MOV/ADD/SUB/SHL/SHR and coverage accounting |
| F13 | Generate oversized/boundary inputs in tests, never commit giant fixture data |

## 4. Classification and preservation requirements

- One primary class per physical line, with independent diagnostic flags; total coverage cannot exceed physical line count.
- `recognized` ≠ `OEMBenchVerified`; `TypedCandidate` ≠ `semantics_verified`.
- `opaque`/`unsupported`/`malformed` remain accessible in raw source and must not produce an invented diagram.
- `full_span`, `body_span`, `SHA-256`, `BOM`, `CRLF` are tested against original bytes, not normalized display text.
- Data after valid `END` remain data; invalid END must not hide following lines.
- Retain limits: `16 MiB`, `200000` lines, bounded pages and `SHA-256`-pinned reopen.

## 5. Explicit exclusions from PR #40

No controller-native writer, deployment, network runtime, automatic IONAME→relay mapping, new sidecar writer, full scan/EN/timing emulator, unsupported OEM verification claims, or private customer programs in CI.

## 6. PR #40 exact-head gates and evidence

- [ ] `cargo test --manifest-path src-tauri/Cargo.toml --locked`, `npm ci --no-audit --no-fund`, `npm run test:native`, `npm run test:security`, `npm run docs:check`, `npm run build`.
- [ ] `scope`, `frontend-build`, `rust-tests`, `windows-security`, `ci-gate` **SUCCESS** at the exact HEAD after implementation (where applicable).
- [ ] Positive/negative fixtures verify raw byte equality and absence of fabricated rungs.
- [ ] `cmd_deploy_to_robot` always rejects; native/IONAME never written, HSES/FTP not reintroduced.
- [ ] No secrets or real OEM exports in diff; all fixtures carry provenance `SYNTHETIC`.
- [ ] `UserSmoke=NOT RUN`, `OEMBench=NOT RUN` until independently documented.
- [ ] **Merge requires separate explicit owner approval**.

## 7. Planned work after PR #40

| PR | Area | Acceptance gate |
| --- | --- | --- |
| #41 | Native UX v2 | source↔rung↔block↔diagnostic, verified jumps and large-file UX |
| #42 | Filesystem/supply chain | Review ACL/TOCTOU, crash/backup and unnecessary FTP dependency; no network functionality |
| #43 | Portable sidecar | Versioned `*.mlad.json`, source SHA, stale/conflict/backup, no native overwrite |
| #44 | Typed grammar | Only OEM-evidenced grammar, no implicit scan semantics |
| #45 | Static analysis | source-linked READ/WRITE/UNKNOWN, provenance and conservative barriers |
| #46 | Unified Offline Project | Native snapshot + sidecar + JSON drafts, no native export |
| #47 | 0.6.x Windows preview | Exact SHA/tag, manual UserSmoke, licenses, checksum, separate release decision |

## 8. Evidence levels and references

`Documented` ≠ `SyntheticTested` ≠ `WindowsCI` ≠ `UserSmoke` ≠ `OEMBenchVerified` ≠ `Authorized`. Each step records SHA, positive/negative tests, failure boundaries, and lawful OEM material use independently.

[Roadmap v4](roadmap.en.md) · [Verification gates](verification_gates.en.md) · [Controller fixture intake](controller_fixture_intake.en.md) · [Filesystem review](filesystem_security_pr29.en.md) · [Engineering decisions](engineering_decisions.en.md).

## Implementation update after PR #40 (20 November 2026)

Documentation PR #40 is already merged; parser fidelity and corpus implementation are now in PR #41. Deliverables include fully synthetic bytes, SHA-256, expected physical-line classes, complete non-lossy classification, unknown/opaque retention, malformed diagnostics and original byte spans in Native Workspace. Successful CI does not replace UserSmoke or OEMBench and cannot authorize HSES/FTP, native writer or deployment. `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`, controller actions **BLOCKED**.

The former PR #41 UX task moves to PR #42 or later; further numbers are tentative. The current PR remains Draft until explicit owner approval.

## PR #42 — Native CIOPRG Offline Workspace UX v2: acceptance criteria

Baseline is merged PR #41, `main` `0c1b233354564d9fd3c242675a78a45417feacc1`. PR #42 is a separate Draft; the earlier proposed PR #42 security task moves to PR #43 or later.

Contract: render no more than one selected Ladder rung and one selected functional block; list other preview items in the navigation tree with explicit truncation notices. Go from physical source line to recognized rung/block, back to source, and from diagnostics to physical line. ArrowUp/ArrowDown, PageUp/PageDown, Home/End keys, source pages of 250 lines. Full-source filter by `SyntaxClass`, exact ASCII opcode, textual address token `#XXXXX` or `Mxxx`; 100 results per page with a complete match count within bounded input.

Acceptance: preserve raw bytes, SHA-256 and selected controller family; reject changed source; test at least 1400 lines and pagination beyond the first 100 matches; negative cases for malformed opcode/address, opaque/malformed and post-END. Unknown text is retained; no execution simulation. Linux frontend/Rust/Windows CI evidence remains separate from UserSmoke/OEMBench.

CIOPRG.LST and IONAME.DAT are read-only; HSES/FTP, native writer, live I/O and deployment **BLOCKED**. `UserSmoke=NOT RUN`, `OEMBench=NOT RUN` until independently tested; separate owner approval required before merge.

## PR #43 — practical security gates and evidence boundaries

Baseline: merged PR #42, `main` `73e675f7df33c5751a35e29752736c3470b11b31`. PR #43 is limited to independent JSON draft hardening, Rust dependency review, tests and documented failure modes. No controller writer.

**Implementation in PR #43:**
1. Remove `suppaftp` from `Cargo.toml` and `Cargo.lock`, pruning only unreachable transitive crates. Do not alter `package-lock.json`; keep `npm ci` and immutable dependency locks. Also remove the unused direct `anyhow` and `futures-util` requirements from the manifest/root lockfile; retain `nom`, which is referenced by `compiler/parser.rs`.
2. On Unix, create staged JSON and `*.bak` with owner-only `0600` regardless of umask; test privacy after repeated saves.
3. Call `verify_single_link` before staged rename and reject replacement/additional hardlinks. During `Drop`, unlink the temporary pathname only if File ID still matches the identity captured at creation.
4. Test positive and negative cases: first save, backup, staged hardlink, temporary pathname substitution, `CIOPRG.LST`/`IONAME.DAT` protection, Windows NTFS/local permissions, and existing conflict guards.
5. CI: `cargo test --locked` on Linux/Windows, `npm run test:security`, applicable final-HEAD `frontend-build`, `rust-tests`, `windows-security`, `ci-gate`.

**Not implemented:** OS-relative directory-handle traversal, Windows DACL/owner preservation, full multiprocess CAS, crash-safe journal for target+`*.bak`. These remain OPEN, not remediated. `HSES/FTP`, native writer and deployment **BLOCKED**; `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`. Merge requires separate owner approval.

## PR #44 — portable sidecar v1 contract and acceptance criteria

**Baseline:** merged PR #43, `main` `c36dc4bc4f84b3dfdb2eac52625c3c0389226320`. Draft PR #44 is not part of `main`. Earlier v4 proposed PR numbers are historical.

**`*.mlad.json` schema:** `format=MLAD_OFFLINE_SIDECAR` and `schema_version=1` required. `source.kind=CIOPRG.LST` provides `source.controller`, `source.sha256`, `source.byte_length`, `source.line_count`; `annotations.rungs`, `annotations.addresses`, `annotations.ioname`, and `bookmarks` are allowed. No native bytes, original path, executable script, robot IP, FTP/HSES or OEM source overwrite. Unknown fields, unsupported versions, bad keys and size limits are rejected.

**I/O contract:** `cmd_load_offline_sidecar` and `cmd_save_offline_sidecar` accept only `*.mlad.json` paths. Every operation read-only revalidates `CIOPRG.LST` and checks `SHA-256`, controller family, bytes and physical lines. First save requires a missing target. Existing files require a `revision` derived from actual bytes and file identity. Stale/missing revisions and external modifications fail closed before replacement. `*.bak` uses existing staged JSON guarantees but is not atomically committed together with the target.

**Acceptance:** Rust round-trip/conflict tests, invalid version/metadata, empty/invalid bookmarks, size limits, unknown fields, native file protection, hardlink/symlink and backup preservation; frontend tests for explicit import/export and offline-only operation. Exact-HEAD CI includes `frontend-build`, `rust-tests`, `windows-security`, `ci-gate`. `UserSmoke=NOT RUN`, `OEMBench=NOT RUN` remain separate; FTP/HSES, native writer and deployment **BLOCKED**.

**Risks:** `revision` is not an atomic filesystem CAS. TOCTOU in `check→rename`, Windows ACL, concurrent writers, crash recovery and backup+target remain OPEN. Owner approval is required before merging.

## PR #45 — OEM-evidenced typed grammar implementation plan

**Base:** merged PR #44, `main` `48522c1e2d0ee41560ce61ba0d2aa8af14e1c6d8`. PR #45 is a separate Draft. Source: YRC1000 Concurrent I/O R12 178648-1CD (RE-CKI-A467) §§12.2.15–16/19–23/25–28/31–34, [official OEM index](https://knowledge.motoman.com/hc/en-us/articles/40670912825362-YRC1000-OPTIONS-INSTRUCTIONS-FOR-CONCURRENT-I-O-R12), accessible [manualzz §12.2 transcription](https://www.manualzz.com/doc/84663174/yaskawa-motoman-yrc1000-owner-s-manual).

**Scope:** syntactic `TypedCandidate` forms for `MUL/DIV/MOD`, `BIN/BCD`, `WAND/WOR/WXOR/WNOT`, `ROL/ROR`, `BMOV`, `CMP`, `PLS/PLF`. Distinct S/D/n roles: no immediate BIN/BCD source; BMOV byte count, not words; MUL Dn+1 register pair; PLS/PLF one bit relay; CMP two sources without D. None may become an executable AST `Instruction` or authorize native source generation.

**Evidence / negative tests:** 15 valid and 17 invalid synthetic examples, DX100/DX200 `controller_form_verified=false` for expanded forms, byte-exact BOM/EOL/spans, opaque and post-END preservation. Update prior fixture classifications ONLY when supported by evidence: BMOV is no longer unsupported; the synthetic unknown becomes `BMOX` with recomputed SHA-256. `MOVJ/MOVL` are distinct JOB instructions, never MOV/BMOV variants.

**UI:** each new opcode links to its manual section; no computed Carry/Zero/Error, EN, actual read/write or live scan result. New DX100/DX200 forms have no family manual evidence; do not show a DX100-only disclaimer.

**CI gate:** exact-final-HEAD `scope`, `frontend-build`, `rust-tests` (`cargo test --locked`), actual Windows NTFS/Tauri `windows-security`, and `ci-gate` SUCCESS before review. Owner approval required before merging. `UserSmoke=NOT RUN`, `OEMBench=NOT RUN` separately. Native `CIOPRG.LST`/`IONAME.DAT` read-only; HSES/FTP/live I/O/native writer/deployment **BLOCKED**.
The pre-existing typed candidate `MOV` remains distinct from `MOVJ` and `MOVL`; adding `BMOV` does not change that boundary.

## PR #46 — source-linked READ / WRITE / UNKNOWN contract

**Baseline:** PR #45 merged, `main` `12f00f4b4fb6879dc0e3ae478aaf8b6d33553139`; PR #46 is an independent Draft. `compiler/static_access.rs`, `inspect` and read-only Tauri `cmd_query_static_access` process only the immutable `CIOPRG.LST` source snapshot.

**Model:** `Read`/`Write` are potential address operand roles, not observed scan events. Contacts, GSTR and documented arithmetic/bitwise S operands indicate candidate reads; OUT, GOUT, D indicate candidate writes. `CMP` has no D; `PLS`/`PLF` indicate pulse output roles, not ordinary register writes. `MUL` creates no fabricated Dn+1 write, `BMOV` expands no block ranges. Immediate constants and shift counts do not read memory; TMR/CNT remain `Unknown` (internal memory semantics unresolved). Invalid, family-unverified and opaque forms also remain `Unknown`. Post-END is not executable source.

**Evidence:** `source_sha256`, one-based `line`, original absolute `ByteSpan`; 160-site `inspect` preview with full-file counts; `query` by kind/exact text token, 100-site offset pages and fresh SHA verification on each request. Rust/IPC/UI tests cover bounds, Unknown, post-END, invalid tokens, 352-site pagination, changed source, byte-exact preservation. No byte/word aliasing, EN/flags, actual hardware memory READ/WRITE or System/User editing permission is inferred.

**Acceptance:** exact final HEAD `scope`, `frontend-build`, `rust-tests`, `windows-security`, `ci-gate`; separately `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`. `execution_verified=false`, `scan_order_verified=false`, `address_aliases_verified=false`, `write_permission_verified=false`, `native_write_allowed=false`. `IONAME.DAT` read-only; HSES/FTP/native writer/deployment **BLOCKED**. No merge before owner approval.
