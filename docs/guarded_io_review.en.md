<!-- Generated en companion for docs/guarded_io_review.md; review required. -->
> **English** · [Русский](guarded_io_review.ru.md) · [English](guarded_io_review.en.md) · [Original / Оригинал](guarded_io_review.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.
> **MLAD safety invariants:** `CIOPRG.LST` and `IONAME.DAT` are strictly **read-only**. FTP/HSES, live I/O, native writer and robot deployment remain **BLOCKED**. Passing `WindowsCI` does **not** imply `UserSmoke=PASS` or `OEMBenchVerified`: both remain **NOT RUN** unless independently documented. Machine translation requires source and Yaskawa manual review.

# MLAD PR #26 — Guarded Reads & JSON Conflict Prevention

**Date:** 18.11.2026. Status:** [PR #26 merged](https://github.com/uzbatsu-beep/MLAD/pull/26) в `main` `2092db1825880d251c87efcb7c331d78cb5e4d28`; [Linux CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44228204776), [Windows MSVC/Tauri CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44227145031). Published rc.9 is unchanged; M2 remains PARTIAL.

## Motivation

PR #25 checked symlink path components to file access, but opened through an independent `tokio::fs::read` It could have landed on a file swapped between a pre-check and an open call. PR #24 Used staged write, but did not verify competing target editing after backup and before the final rename.

## 1. Identity of an open file

Separate module `src-tauri/src/guarded_read.rs`:

- First rejects the discovered parental symlink/reparse and direct link.
- It reads metadata and uses **one main descriptor for reading **. On Windows, additionally opens control descriptors with `FILE_FLAG_OPEN_REPARSE_POINT` Before and after reading; compare them with WinAPI File ID. This reduces the window of substitution, but does not guarantee a fully atomic pathway. Unix-- `dev+ino`; Windows — WinAPI `GetFileInformationByHandle` Volume serial + 64-bit file index `HANDLE`; no nightly Rust. If WinAPI does not return the identifiers, the operation is terminated. PR uses Win32. `FILE_FLAG_OPEN_REPARSE_POINT` and check reparse flags on open handles.
- Limits reading before all content is highlighted: JSON – 2 MiB `IONAME.DAT` - 4 MiB, `CIOPRG.LST` - 16 MiB.
- After reading, reconfirms the pathname and metadata of the open descriptor, as well as the length and time of the last change.
- Rejects all Windows `FILE_ATTRIBUTE_REPARSE_POINT`including junction (and not only) `is_symlink()`).
- Does not change the original bytes. CIOPRG and IONAME call it through `spawn_blocking`All JSON users use the same function.

It's a **fail-closed checks** around opening and reading, not a full-fledged atomic catalog operation.

## 2. Competitive preservation of JSON

`draft_io::write_atomic_inner` after preparing a new file and `*.bak` Checks that the old bytes have not changed; immediately before the replacement makes a second comparison. If another process has changed a file or created a target during the first save, MLAD returns the error and **not** overwrites the newer version.

The tests introduce synthetic substitution exactly in the check/open window and synthetic modification between the creation of the backup and the final update. The immutability of external data and the safety of the native file are checked.

## 3. What stays open

1. Last re-examination and `rename` Still separated. It's a fucking transaction and a fucking safe one. `openat2`/`NtCreateFile` with a catalog descriptor and prohibition of following links.
2. Changing a file **on the same inode** with an artificially restored size/mtime may go unnoticed; verification does not replace file locks and guaranteed immutable snapshot.
3. Windows volume/index is not guaranteed for all network file systems. For security reasons, the operation is rejected if the ID is not available; the robot network is still disabled.
4. Backup and target do not form a single atomic transaction. Outdated backup is possible after a conflict; the target JSON should not be overwritten.
5. Not tested with uncontrolled Windows reparse/junction, third-party processes with aggressive racing, hardlinks, ACL, power loss and atomic directory.
6. Manual Windows WebView2 smoke (Native Workspace, JSON draft, IONAME, conflict errors) **NOT RUN**

## 4. Reception.

| Gate | Verification |
| --- | --- |
| Linux Rust | `cargo test --manifest-path src-tauri/Cargo.toml --locked` |
| Frontend/security | `npm run test:security`, `npm run docs:check`, `npm run build` |
| Windows | [MSVC Rust + Tauri build — SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44227145031)without release |
| Negative | Switching to open, direct symlink, directory symlink, competitive replacement of JSON / creation of target |
| Hardware | Not required or executed; HSES/FTP/writer off |
| User acceptance | Manual verification required; not replaced by GitHub CI |

[Status of all MLAD](project_status.en.md) · [Previously closed restrictions PR #25](csp_path_hardening.en.md).


**CI results do not mean manual Windows smoke or full TOCTOU closure.** [implementation_plan.md](implementation_plan.en.md), [verification_gates.md](verification_gates.en.md).

**Windows API sources:** [GetFileInformationByHandle](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfileinformationbyhandle), [BY_HANDLE_FILE_INFORMATION](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/ns-fileapi-by_handle_file_information), [FILE_FLAG_OPEN_REPARSE_POINT](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew). The FFI code requires independent security review (ABI, reparse and NTFS/ReFS differences), not just compilation.

## Follow-up evidence PR #28–#29 (18.11.2026)

This document is a **historical study. n PR #26**not a description of the current protection. After him. [PR #28](https://github.com/uzbatsu-beep/MLAD/pull/28) Windows NTFS/Win32 build/regression [PR #29](https://github.com/uzbatsu-beep/MLAD/pull/29) — JSON bytes+File ID snapshots, hardlink rejection, staged identity+SHA-256 и Windows ADS/UNC/device path deny. [Actual main CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621) - SUCCESS, but **not ** handy. WebView2 There is no guarantee of complete TOCTOU closure. Canonical Continued Review: [filesystem_security_pr29.md](filesystem_security_pr29.en.md).
