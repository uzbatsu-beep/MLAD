<!-- MLAD-I18N:EDITORIAL:reviewed 2026-11-19 against json_persistence_safety.md -->
> **Language / Язык:** [English](json_persistence_safety.en.md) · [Русский](json_persistence_safety.ru.md) · [Original](json_persistence_safety.md)
>
> **Evidence and safety:** This is a technical translation of the original PR #24–#29 persistence contract, **not** an independent filesystem-security certification. Native `CIOPRG.LST` and `IONAME.DAT` are **read-only**; HSES/FTP/native writer/deployment remain **BLOCKED**. WindowsCI does not establish manual `UserSmoke=PASS` or OEMBench; both are **NOT RUN** absent separate reports.

# MLAD — JSON Save-Safety Model (PR #24)

**Date:** 18 November 2026. **Historical status:** PR #24 merged into `main` on 18 November; the Windows MSVC regression CI succeeded, but no separate published Windows binary yet included this change at that checkpoint. This extends milestone M2 and **does not change CIOPRG grammar**.

## Problem and motivation

Before PR #24, the `main` following PR #23 saved all three user-owned JSON formats by directly truncating/writing the target path (`tokio::fs::write` / `std::fs::write`). Backups were produced by a simple copy. If writing was interrupted or failed, that approach could destroy the previous version.

## Implemented persistence contract

- Only **independent** `*.graph.json`, `*.advanced.json` and legacy `*.json` files pass their respective filename validation before the common `draft_io::write_atomic` writer. The writer additionally requires the `.json` extension. `CIOPRG.LST` and `IONAME.DAT` are **never** targets of these write commands.
- Reads of all three JSON formats have a **2 MiB limit**, reject a direct symlink and an incorrect filesystem object type, and validate the actual returned data size to avoid an unbounded read after a file swap.
- Writes validate the maximum **2 MiB** payload, create a new **unique staged file** in the destination directory using `create_new(true)`, write the bytes and call `sync_all`, then replace the target path through `rename`. The original target is **not truncated before the final replacement**.
- For an existing JSON target, the old bytes are read first and a separate `*.bak` file is staged. Overwriting an existing symlink as the backup is prohibited. Failure during staging or backup preparation must leave the old JSON target in place; a final rename failure must not truncate it.
- Temporary files are cleaned up in `Drop` when an operation fails. Writes use `tokio::task::spawn_blocking` so filesystem work does not block the async runtime.
- Positive and negative Rust tests use **synthetic**, not production robot data. `npm run test:security` detects any accidental return to direct `fs::write` from Tauri handlers.

## Limitations: what is **NOT** guaranteed

1. **No single atomic transaction** spans both the `*.bak` backup and the main JSON target. An interruption after installing the backup but before replacing the target can leave the original target together with the newly prepared backup.
2. **TOCTOU and parent-directory path substitution were not eliminated** by PR #24. Ordinary path-based `metadata/open/rename` is not a secure directory-handle API. Maliciously changeable symlink ancestors, hardlinks and interprocess races require separate threat assessment. **Later PR #25–#29 reduce specific risks, but do not close all final check→rename races.**
3. On Windows, `rename` can fail if the target is opened with incompatible sharing or protected. The correct response is preserving the old content and reporting failure, **not** promising seamless replacement on every volume.
4. JSON content is **not encrypted**; there are no additional guaranteed ACLs for directories, profiles or backup files. The user remains responsible for access controls. Without actual Windows smoke, simulated power loss and NTFS assessment, no crash-consistent durable-commit guarantee can be claimed.
5. A legacy JSON project is validated under its own existing format and limits, **not** the native CIO format.
6. PR #24 itself did not modify CSP. Later merged PR #25 and #26 introduced CSP/path checks and guarded JSON conflict detection: [CSP review](csp_path_hardening.en.md), [guarded I/O](guarded_io_review.en.md). These do not eliminate TOCTOU and were not part of the older published rc.9.

## Acceptance

- Cargo tests, including synthetic inputs, negative symlink cases and `copy_original == source_bytes` for the native LST, must pass.
- TypeScript/frontend checks and `npm run test:security` must pass with no regression; detection of direct writes to target paths is mandatory.
- Manual Windows user acceptance must cover two successive saves of each JSON format, `*.bak` contents, a locked target file, prevention of overwriting robot-native files and reopening the saved draft.
- Separate security work is still needed for secure file-handle/directory-handle usage and CSP, cross-process locking/versioning, a truly atomic backup protocol and an ACL/permission audit.

**M2 remains PARTIAL** despite merged PR #24 and successful Rust CI. PR #24 added a separate [Windows MSVC regression workflow](../.github/workflows/windows-json-persistence-pr24.yml) **without publishing a new release**. It checks persistence behavior on a Windows runner, while real GUI smoke on an end-user machine remains a separate gate.

## PR #26 (merged): concurrent editing

Before replacing an existing JSON target, the backend compares it with a snapshot of the previously read version. Unexpected appearance of a file during first save also triggers refusal. The staged-writing/backup model from PR #24 remains. **Neither protection against every race after the last check nor an atomic backup+target transaction is guaranteed.** See [guarded I/O review](guarded_io_review.en.md).

**Historical baseline checks:** [main CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44228204776) and [PR #26 Windows CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44227145031). Manual Windows WebView2 smoke and power-loss testing were **NOT RUN**. [Unified verification gates](verification_gates.en.md).

## Update after PR #28–#29 (merged, 18 November 2026)

[PR #28](https://github.com/uzbatsu-beep/MLAD/pull/28) introduced Windows NTFS regressions (junctions, exclusive locks) and the formal manual UserSmoke checklist. [PR #29](https://github.com/uzbatsu-beep/MLAD/pull/29) added target-snapshot comparisons of **File ID + original bytes**, rejection of multiply hard-linked JSON targets/backups, staged File ID + SHA-256 verification, and rejection of Windows ADS/UNC/device-style paths.

The baseline `main` SHA `f85ddfcce3063dd087425be981a9d2b7a16d6309` passed [Linux CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454) and [Windows CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621). Thus PR #24's safeguards have **additional regressions and checks**, but complete atomic backup+target replacement, ACL preservation and elimination of the final check→rename TOCTOU window **are still not guaranteed**. See [the current threat model](filesystem_security_pr29.en.md). Manual WebView2 smoke = `NOT RUN`.
