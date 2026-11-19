# Historical GitHub Actions workflows — archived, not executable

These **15 exact historical workflow YAML files** were moved from `.github/workflows/` to `.github/workflow-archive/` in a repository-maintenance PR after PR #37. Their contents were preserved **byte-for-byte**; only their path changed. GitHub Actions registers workflows only at the supported `.github/workflows/` location, so these archived copies **do not run** on pushes, PRs or manual dispatches. Existing release tags, Git history, older workflow-run logs and GitHub Releases are not deleted by this move.

This archive is **historical evidence**, not permission to rebuild a legacy workflow on the default branch without review. Some old jobs had `contents: write` or pushed releases, and several refer to deleted or obsolete branch names. Do not move a YAML back under `.github/workflows/` without an explicit owner decision, permissions review, pinned-dependency verification and fresh CI.

## Snapshots

- [`bilingual-docs-generation.yml`](bilingual-docs-generation.yml)
- [`bilingual-docs-polish.yml`](bilingual-docs-polish.yml)
- [`windows-bench-rc.yml`](windows-bench-rc.yml)
- [`windows-cioprg-rc2.yml`](windows-cioprg-rc2.yml)
- [`windows-native-workspace-rc3.yml`](windows-native-workspace-rc3.yml)
- [`windows-cioprg-annotations-rc4.yml`](windows-cioprg-annotations-rc4.yml)
- [`windows-compact-workspace-rc5.yml`](windows-compact-workspace-rc5.yml)
- [`windows-postend-rc6.yml`](windows-postend-rc6.yml)
- [`windows-typed-cio-rc7.yml`](windows-typed-cio-rc7.yml)
- [`windows-static-evidence-rc8.yml`](windows-static-evidence-rc8.yml)
- [`windows-oem-reference-rc9.yml`](windows-oem-reference-rc9.yml)
- [`windows-ui-preview.yml`](windows-ui-preview.yml)
- [`windows-json-persistence-pr24.yml`](windows-json-persistence-pr24.yml)
- [`windows-csp-path-pr25.yml`](windows-csp-path-pr25.yml)
- [`windows-guarded-io-pr26.yml`](windows-guarded-io-pr26.yml)

## Active workflows (not moved)

- [`MLAD validation`](../workflows/validation.yml) — PR/main tests and unified `ci-gate`, calls Windows when needed
- [`MLAD Windows Acceptance & Security Regression`](../workflows/windows-security-acceptance.yml) — reusable Windows NTFS/Tauri and optional manual run
- [`MLAD — Prune merged PR branches (manual)`](../workflows/merged-branches-prune.yml) — manual-only dry-run/delete with explicit confirmation

**Safety:** `CIOPRG.LST` and `IONAME.DAT` remain read-only; HSES/FTP, live I/O, native robot writer and deployment remain blocked. Historical CI success is **not** manual `UserSmoke=PASS` or OEM bench validation.
