<!-- Generated en companion for docs/json_persistence_safety.md; review required. -->
> **English** · [Русский](json_persistence_safety.ru.md) · [English](json_persistence_safety.en.md) · [Original / Оригинал](json_persistence_safety.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.
> **MLAD safety invariants:** `CIOPRG.LST` and `IONAME.DAT` are strictly **read-only**. FTP/HSES, live I/O, native writer and robot deployment remain **BLOCKED**. Passing `WindowsCI` does **not** imply `UserSmoke=PASS` or `OEMBenchVerified`: both remain **NOT RUN** unless independently documented. Machine translation requires source and Yaskawa manual review.

# MLAD is the JSON Conservation Safety Model.PR #24)

**Date:** 18.11.2026. Status:** PR #24 merged in main 18.11.2026; Windows MSVC regression CI SUCCESS, but a separate Windows binary with the change has not yet been published. The M2 extension does not change the CIOPRG grammar.

## Objective and reason

In the original `main` later PR #23 All three types of user JSON were saved through direct circumcision/target path recording.`tokio::fs::write` / `std::fs::write`), and backups were created by a simple copy. This created the risk of losing the previous version if the recording operation was interrupted or mistaken.

## Contract in progress

- Only **individual* files. `*.graph.json`, `*.advanced.json` и legacy `*.json` Independently validate the name before calling a single `draft_io::write_atomic`. Writer is also testing the extension. `.json`. `CIOPRG.LST` и `IONAME.DAT` They don't get written command.
- Reading all three formats: limit 2 MiB, failure with direct symlink and the wrong file type; it is impossible to read the replaced large file without checking the size of the result.
- Recording: check size up to 2 MiB, create a new *unique* temporary file in the same directory with `create_new(true)`recording and `sync_all`Replacement of the initial path through `rename`. Until the last step, the original is not cut.
- For existing JSON, its previous bytes are pre-read and similarly prepared. `*.bak`Copying via overwriting to an existing symlink is prohibited. If the staging/backup ended in an error, the old JSON remains in place; the event of a final rename error should also not trim it.
- In case of failure, temporary files are deleted in `Drop`Conservation is launched in `tokio::task::spawn_blocking`Not to block async runtime.
- Positive/negative Rust tests do not contain data from real robots. `npm run test:security` Checks that Tauri handlers have not returned to direct `fs::write`.

## Limitations: What is NOT guaranteed

1. **No Total Atomic Transaction* Between Updates n `*.bak` and the main JSON. An interruption after installing the backup but before replacing the target will leave the old target and the new backup.
2. **TOCTOU and parental directory interception not eliminated**: standard path-based `metadata/open/rename` Not secure directory-handle APIs; maliciously modifiable symlink ancestors, hardlinks, and cross-process racing require a separate threat review.
3. Windows. `rename` When the target file is occupied/protected, it can return the error; the task is to save the old file and report the failure, rather than promising a seamless replacement on any volume.
4. There is no JSON encryption and individual guaranteed rights to directories, profiles or backups. The user is responsible for access rights to his folder. Without Windows smoke, power outage simulations, and NTFS verification, you can’t promise crash-consistent durable commit.
5. The content of the legacy project passes the existing validation and format limitations model, not the native CIO model.
6. PR #24 did not change the CSP, but later merged PR #25 and #26 added CSP/path checks and guarded JSON conflict detection: [CSP review](csp_path_hardening.en.md), [guarded I/O](guarded_io_review.en.md). This does not promise to eliminate TOCTOU and is not included in the released rc.9.

## Reception.

- Cargo tests, including synthetic files, symlink negative cases and verification `copy_original == source_bytes` For native LST, must pass.
- TypeScript/frontend и `npm run test:security` must pass without regression; control of direct write targets is mandatory.
- Windows user acceptance checks for two saves for each JSON `*.bak`No overwrites of robot files, and the ability to reopen.
- Separate security PR for true file-handle/dir-handle and CSP, interprocess lock/versions, atomic backup protocol, permission audit.

M2 status remains PARTIAL even after merged PR #24 and a successful Rust CI. For PR #24 separate [Windows MSVC regression workflow](../.github/workflows/windows-json-persistence-pr24.yml), without release. It checks file saving behavior on Windows runner; user smoke on a particular machine remains a standalone gate.


## PR #26 (merged): Competitive editing

Before the final update, JSON is compared to the snapshot of the old version; the appearance of another file during the initial saving also causes a failure. Staging/backup from PR #24 It persists. **Not guaranteed** exclusion of races after the last check and backup+target transaction. [Study](guarded_io_review.en.md).

** Check the current baseline:** [main CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44228204776), [PR #26 Windows CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44227145031). Manual Windows WebView2 Smoke and power failure check – NOT RUN [Unified gates](verification_gates.en.md).

## Addition after PR #28–#29 (merged, 18.11.2026)

[PR #28](https://github.com/uzbatsu-beep/MLAD/pull/28) Added Windows NTFS regression (junction, exclusive lock) and a formal UserSmoke checklist. [PR #29](https://github.com/uzbatsu-beep/MLAD/pull/29) added snapshot comparison by **File ID + bytes**, failure for hardlinked JSON target/backup, staged file ID verification +SHA-256 Windows ADS/UNC/device paths. `main` SHA `f85ddfcce3063dd087425be981a9d2b7a16d6309` passed [Linux](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454) и [Windows](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621) CI. In other words, the contract PR #24 * new regressions**, but the full atomic backup+target, ACL-preservation or final check→rename TOCTOU guarantee is still not available. [Current threat model](filesystem_security_pr29.en.md). manually WebView2 smoke = `NOT RUN`.
