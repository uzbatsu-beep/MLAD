<!-- machine-generated draft; editorial review required. -->
> **English** · [Русский](CHANGELOG.ru.md) · [English](CHANGELOG.en.md) · [Original / Оригинал](CHANGELOG.md)

# MLAD — changelog

Release entries refer to a specific tag, not current main. Detailed historical release notes remain unchanged.

## Unreleased

- PR #50: approved Roadmap v5.
- PR #51: Semantic Core v2, source identity, versioned IPC, engineering summary.
- PR #52: approved UI concept, D16–D20; new UI not implemented.

## v0.5.0-rc.10-unified-offline — 2026-11-20

Unified portable offline project, sidecar, static READ/WRITE/UNKNOWN, synthetic Windows Preview. [Release and SHA-256](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.10-unified-offline). UserSmoke=NOT RUN; OEMBench=NOT RUN.

## Earlier

rc.1–rc.9: [release notes archive](docs/releases/), [GitHub Releases](https://github.com/uzbatsu-beep/MLAD/releases). rc.9 does not include PR #24–#29.

## Version policy

Proposed implementation: SemVer for published versions; pre-release suffix explicitly indicates stage. Plan 0.6.x core/workspace, 0.7.x intelligence, 0.8.x compare, 0.9.x lab/test, 0.10.x docs/fleet, 0.11.x AI, 1.0.0 stable offline. Before release reconcile tag, package.json, Cargo.toml, lockfiles and tauri.conf.json by one rule; do not rewrite historical rc. Separate schema versions are not product version. Publication only after separate approval; CI artifact is not Release. Hashes, corresponding source and evidence required.

`CIOPRG.LST` and `IONAME.DAT` read-only; HSES/FTP/native writer/deployment BLOCKED.
