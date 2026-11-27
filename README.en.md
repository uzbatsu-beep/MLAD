<!-- MLAD-I18N:EDITORIAL:source-compared revision 2026-11-27; independent review still required -->
> **English** · [Русский](README.ru.md) · [English](README.en.md) · [Original / Оригинал](README.md)

# MLAD — Motoman Ladder Analysis & Draft IDE

**Offline engineering environment for Yaskawa Motoman Concurrent I/O analysis.** Independent project, not an official Yaskawa product.

[Download Windows Preview](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.10-unified-offline) · [Documentation](docs/README.en.md) · [Roadmap v5](docs/roadmap.en.md) · [CHANGELOG](CHANGELOG.en.md)

| Distribution | Status |
| --- | --- |
| Windows x64 pre-release | v0.5.0-rc.10-unified-offline |
| Development | Semantic Core v2, PR #51 merged; UI concept, PR #52 merged |
| License | GPL-3.0-only |
| Manual acceptance | UserSmoke=NOT RUN; OEMBench=NOT RUN |

## Available functionality

| Capability | Boundary |
| --- | --- |
| DX100 / DX200 / YRC1000 | Address models; firmware and OEM semantics not fully verified |
| Native Workspace | Read-only `CIOPRG.LST`, SHA-256, source text, search, diagnostics |
| Ladder | Recognized rungs, source links; preview limited to 200 rungs |
| IONAME.DAT | Preview and manual associations; name mapping unverified |
| Static analysis | READ/WRITE/UNKNOWN with source links; not a scan model |
| Offline project | `*.mladproj.json`, `*.mlad.json` v1; separate comments and bookmarks |
| JSON drafts | Graph/Advanced; editor, tri-state, limited TMR/CNT/GSTR/GOUT forms |

Main and published build differ. [Capability matrix](docs/project_status.en.md).

## User quick start

1. Choose portable ZIP or NSIS setup on the [release page](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.10-unified-offline). Verify SHA-256 against the supplied SHA256SUMS.txt; build is unsigned.
2. Open a copy of the source file in Native Workspace and select the controller family.
3. Inspect text, rungs, blocks and diagnostics. Open `IONAME.DAT` if needed.
4. Save annotations/manifest separately. Edit only JSON drafts, not the original controller program.

## Development

Stack: Rust, Tauri 2, React, TypeScript, Vite; Node.js 22 in CI. Rust and Tauri system prerequisites required (Windows MSVC/WebView2).

```bash
npm ci --no-audit --no-fund
npm run tauri dev
npm run docs:check
npm run build
cargo test --manifest-path src-tauri/Cargo.toml --locked
```

[Development process](docs/development_workflow.en.md) · [Acceptance gates](docs/verification_gates.en.md) · [Verified PR #51 runtime main CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/45813397360).

## Roadmap

Engineering Workbench: eight specialized layouts, themes, Signal Intelligence, Compare, Virtual Laboratory, Trace & Test, Documentation, Robot Fleet and AI. This is the **approved plan**, not the released UI. Publish actual screenshots after the corresponding build is verified.

## Limits and feedback

`CIOPRG.LST` and `IONAME.DAT` are read-only. HSES/FTP, native writer, live I/O and robot deployment are BLOCKED. Unknown does not imply false or true; modeling is not telemetry. Do not publish production exports, private hashes or IPs.

[Issues](https://github.com/uzbatsu-beep/MLAD/issues) · [SECURITY](SECURITY.en.md) · [CONTRIBUTING](CONTRIBUTING.en.md) · [ACCESSIBILITY](ACCESSIBILITY.en.md) · [LICENSE](LICENSE) · [Licensing](docs/licensing.en.md).

History: rc.9 does not include PR #24–#29; [previous README](https://github.com/uzbatsu-beep/MLAD/blob/0754dea0d37f5fbc45c795008b60fe69e6c8b08f/README.md). Do not treat historical snapshots as current instructions.
