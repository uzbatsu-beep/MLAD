<!-- Generated en companion; machine-generated draft; editorial review required. -->
> **English** · [Русский](project_status.ru.md) · [English](project_status.en.md) · [Original / Оригинал](project_status.md)
> **Translation review:** AI-assisted translation; check engineering claims against the Russian original and OEM evidence. `CIOPRG.LST` and `IONAME.DAT` are read-only. `HSES`/`FTP`, native writer and robot deployment remain **BLOCKED**; `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`.

# MLAD — project status after PR #39

**GitHub baseline:** `main` @ `066db5f8c01a9da8f5f0218f12539b305ab021e4`, after merging [PR #39](https://github.com/uzbatsu-beep/MLAD/pull/39) (19.11.2026); PR #1–#39 merged. [PR #39 CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44571515321): `scope`, `frontend-build`, `rust-tests`, `windows-security`, `ci-gate` = **SUCCESS** on the PR head; not independent acceptance of merge commit.

**Last published Windows release:** [`v0.5.0-rc.9-oem-reference`](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference) (17.11.2026); does not contain PR #24–#39. `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`.

## 1. Product and safety boundary

MLAD — local OFFLINE/BENCH environment `DX100`/`DX200`/`YRC1000` for analysis of `CIOPRG.LST` and `IONAME.DAT`. Both native files remain immutable and read-only; JSON drafts are not controller programs. `HSES`/`FTP`, native writer, live monitoring, robot deployment — **BLOCKED**. `PART 1/2` does not establish System/User permissions. `OEMBenchVerified` is absent.

## 2. Functionality present in main

| Component | Supported subset | Limitation |
| --- | --- | --- |
| Native source inspector | Present: bytes, SHA-256, paging and search | Not complete OEM grammar |
| Native Workspace | Integrated main window with source/diagrams/blocks/IONAME | Cross-navigation/large-file UX partial |
| Boolean AST | Recognized boolean rungs, diagnostics and local tri-state evaluation | Not controller scan |
| Typed data candidates | MOV/ADD/SUB/SHL/SHR and source-order M links | EN/flags/dataflow unverified |
| JSON draft editor | Graph/Advanced/legacy JSON separate from native | Portable sidecar absent |
| Local annotations | localStorage, read-only IONAME with manual associations | No guaranteed portability/backup |
| Filesystem safety | Bounded reads, CSP, File ID/hardlinks, staged JSON+backup | ACL/TOCTOU/crash risks |
| GitHub Actions | PR39: 5 jobs SUCCESS on its head | Not UserSmoke/bench; Ruleset requires only frontend-build |

## 3. Independent verification status

| Gate | Status | Evidence |
| --- | --- | --- |
| `SyntheticTested` | **PASS (subset)** | Rust/TS/Node tests on PR #39 head; not OEM fixtures |
| `WindowsCI` | **PASS (PR #39 HEAD)** | [Actions run](https://github.com/uzbatsu-beep/MLAD/actions/runs/44571515321) |
| `UserSmoke` | **NOT RUN** | No independent Windows 10/11 WebView2 report |
| `OEMBenchVerified` | **NOT RUN** | No model/firmware/results protocol |
| `Authorized native writer` | **BLOCKED** | Grammar/System/User/round-trip unverified |
| `Authorized HSES/FTP/deployment` | **BLOCKED** | No separate authorization |

## 4. Open engineering tasks

1. **Parser fidelity:** `SYNTHETIC` corpus and coverage are not a standalone verifiable artifact; `unknown`/`opaque` must not be treated as recognized logic.
2. **Native UX:** `source↔rung↔block↔diagnostic` navigation and large-file UX are partial; the `Byte offset` tooltip reports line number instead of a byte offset.
3. **Data safety:** `check→rename` TOCTOU, Windows ACL, `*.bak` + target atomicity, concurrent writer/crash risks — **OPEN**.
4. **Dependencies:** `suppaftp` remains in Cargo.toml without an active runtime; separate review needed.
5. **Workflow governance:** active Ruleset `MLAD — main branch protection` requires `frontend-build`, not `ci-gate`; settings change needs explicit owner decision.
6. **Product roadmap:** `*.mlad.json`, extended OEM grammar and unified IDE not implemented.

## 5. Next backlog

**PR #40:** Native CIOPRG Parser Fidelity / Coverage, starting with a documentation baseline in Draft PR. **PR #41–#47:** UX, security, portable sidecar, typed grammar, static analysis, Unified IDE and conditional offline release. Updating Markdown alone does not complete PR #40.

[Roadmap v4](roadmap.en.md) · [PR #40+ plan](implementation_plan.en.md) · [Verification gates](verification_gates.en.md) · [Security risk review](filesystem_security_pr29.en.md) · [Manual Windows acceptance](windows_acceptance_pr28.en.md).

## Verifiable implementation status — PR #41 (20 November 2026)

PR #40 is already merged as documentation only, without parser implementation. Draft PR #41 adds a synthetic CIOPRG corpus, full-source physical-line classification and original source byte spans in Native Workspace. This represents a development branch, not functionality merged into main and not a real-controller validation. Acceptance requires completed Linux/Windows CI on the final HEAD and explicit owner approval before merging.

Independent states: `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`; CIOPRG.LST/IONAME.DAT read-only; HSES/FTP/native writer/deployment **BLOCKED**. The previously published rc.9 does not include PR #40/#41. Required-check Ruleset settings are unchanged.

## Current development branch: PR #42 — Native Workspace UX v2

PR #41 Parser Fidelity merged into `main` `0c1b233354564d9fd3c242675a78a45417feacc1`. New PR #42 is a Draft, not in main. The branch implements source↔rung/block/diagnostic navigation, one diagram at a time, keyboard navigation, and server-paged `SyntaxClass`/opcode/address source filtering pinned to SHA-256.

CI evidence must match the final PR #42 HEAD. Passing CI does not imply a manual Windows WebView2 UserSmoke or OEM bench. `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`; CIOPRG.LST/IONAME.DAT read-only; HSES/FTP, native writer, and deployment **BLOCKED**. There is no public Windows Release for this change.
