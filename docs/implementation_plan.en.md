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
