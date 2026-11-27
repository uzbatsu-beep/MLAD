<!-- machine-generated draft; editorial review required. -->
> **English** · [Русский](project_status.ru.md) · [English](project_status.en.md) · [Original / Оригинал](project_status.md)

# MLAD — current project status

Verified 27 November 2026 against main `39137a60f68041ed1b4e65d2c3fcdf3518dff3d8`, PR #52 merged. Runtime baseline: PR #51, [merge commit CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/45813397360) SUCCESS. Latest published build: [rc.10](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.10-unified-offline) — separate release, does not contain subsequent Semantic Core v2. Distinguish main, CI artifact and release.

## Implemented

| Component | Status and limits |
| --- | --- |
| Lossless native parser | Immutable bytes, SHA-256, physical lines, unknown/post-END |
| Controller memory | DX100/DX200/YRC1000; incomplete OEM/firmware coverage |
| Semantic Core v2 | Unified facade, IPC version 1, SourceIdentity and spans; 200 rung refs preview |
| Static analysis | READ/WRITE/UNKNOWN; dataflow hypotheses, not execution |
| Native Workspace | Source, Ladder, blocks, IONAME, search and navigation |
| JSON drafts | Graph/Advanced, tri-state and supported forms |
| Offline project | `*.mladproj.json`, `*.mlad.json` v1; SHA and relative paths |
| Filesystem | Staged write, backup, File ID, path restrictions; residual ACL/TOCTOU/multifile recovery open |

## Planned

PR #52 approved UI; new layouts not implemented. Next stable tasks V5-03 Controller Profiles, V5-04 Signal Index, V5-05 UI Foundation, V5-06 Workspace, V5-07 Ladder Viewer, V5-08 Inspector. PR numbers indicative. [Roadmap](roadmap.en.md), [plan](implementation_plan.en.md).

## Verification and limits

`CIOPRG.LST` and `IONAME.DAT` read-only. HSES/FTP, native writer, live I/O and deployment BLOCKED. UserSmoke=NOT RUN; OEMBench=NOT RUN. OEM scan, System/User permissions and full semantics unverified. Never load synthetic fixtures onto a robot.

## History and sources

[Architecture](architecture.en.md) · [Gates](verification_gates.en.md) · [21 November audit](repository_audit_2026-11-21.en.md) · [CHANGELOG](../CHANGELOG.en.md). All previous detailed records PR #1–#51 preserved in an [immutable snapshot](https://github.com/uzbatsu-beep/MLAD/blob/0754dea0d37f5fbc45c795008b60fe69e6c8b08f/docs/project_status.md). Do not duplicate them here; update current status only.
