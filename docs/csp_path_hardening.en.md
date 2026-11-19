<!-- Generated en companion for docs/csp_path_hardening.md; review required. -->
> **English** · [Русский](csp_path_hardening.ru.md) · [English](csp_path_hardening.en.md) · [Original / Оригинал](csp_path_hardening.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.
> **MLAD safety invariants:** `CIOPRG.LST` and `IONAME.DAT` are strictly **read-only**. FTP/HSES, live I/O, native writer and robot deployment remain **BLOCKED**. Passing `WindowsCI` does **not** imply `UserSmoke=PASS` or `OEMBenchVerified`: both remain **NOT RUN** unless independently documented. Machine translation requires source and Yaskawa manual review.

# MLAD – CSP and file path security checksPR #25)

**Context:** OFFLINE/BENCH project. Date: 18.11.2026. Status:** PR #25 merged in main 18.11.2026; Windows CI SUCCESS, but published rc.9 does not contain CSP.

## Primary grounds

- [Tauri 2 CSP](https://v2.tauri.app/security/csp/) Tauri adds hash/nonce sources for local resources when assembled if the CSP is explicitly specified. `csp:null` This protection is off.
- [Tauri SecurityConfig](https://v2.tauri.app/reference/config/#securityconfig) — production `csp` separate `devCsp`By default, dev inherits production.
- [Tauri Vite Guide](https://v2.tauri.app/start/frontend/vite/) Local Vite server and HMR via WebSocket.

## 1. Implemented production CSP

В `src-tauri/tauri.conf.json` strictly `default-src` и `script-src: 'self'`without `'unsafe-inline'`/`'unsafe-eval'` for JavaScript. `connect-src` Allows only application and Tauri IPC`ipc: http://ipc.localhost`). `object-src`, `frame-src`, `worker-src`, `base-uri`, `form-action` They're blocked. `img-src 'self' data:` и `font-src 'self'` Do not allow CDN and external images.

**Conscious exception:** `style-src 'self' 'unsafe-inline'` You need existing inline-style React and starter HTML. It's a *****inline JavaScript resolution and a *****in' perfect CSP; the next UX/security step can translate styles into classes and remove that exception.

## 2. Development CSP

Separate `devCsp` localize `http://localhost:5173` и `ws://localhost:5173` Vite/HMR only; remote HTTP/WSS is not available. Production policy does not include localhost:5173, does not allow the creation of arbitrary network requests, and retains the local Tauri IPC.

## 3. File paths: what is the strengthening

Function `draft_io::verify_parent_directory(path)` before file operations:
- Checking syntax. `..` He rejects the obvious path.
- Checks every **existing ** component of the directory through `symlink_metadata`Rejecting symlink and the lack of a regular directory.
- Built into JSON. `read_bounded/validate_file`File staging, final rename, and read-only CIOPRG and IONAME inspect and Open chooser.
- CIOPRG further rejects **direct symlink**; previously `metadata` I could have followed him. In the source files of robotics, the record does not appear.

Synthetic tests: nested conventional catalogs, `../`Linked parent, linked native file, SHA/byte identity before and after rejection, direct JSON symlink ban, old JSON backup/regression tests.

## 4. What is NOT confirmed or guaranteed

1. **TOCTOU is still possible:** sequence `symlink_metadata → open/rename` not atomic. An attacker with access to directories can replace the component after checking. Reliable solution requires OS-specific directory handle, `openat2/openat + O_NOFOLLOW` (POSIX), Windows File ID/reparse policy and ACL. It's a separate PR.
2. Windows junction/reparse points are not guaranteed to be classified as symlink by all APIs; Windows tests without enhanced rights do not cover all options.
3. Hardlinks, concurrent writers, atomic pair backup+target, file ACL, crash durability, and local profile protection **not* are all addressed by this PR.
4. CSP does not prevent abuse through authorized Tauri IPC commands: you need separate permission reviews and runtime verification, including real Windows.WebView2 and all the panels.
5. Don't claim that`npm run build\` (Regular Vite) implements CSP in web preview: the application must be assembled as Tauri, and the CSP must be checked in **real Tauri WebView**.
6. Some normal user paths with a symbolic link or `..` now intentionally rejected; use a direct copy in a regular directory to view it.

## 5. Acceptance criteria

| Verification | Automatic/manual | Status |
| --- | --- | --- |
| Node tests on CSP, no remote/unsafe eval | Automatic. | PASS — PR #25/#26 CI |
| RUST checking conventional/unsafe paths | Automatic Linux + Windows MSVC | PASS — main/PR #26 CI |
| Windows Tauri build with compiled CSP | GitHub Windows workflow, does not publish release | [PASS — PR #26 Windows CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44227145031) |
| В WebView2 Native Workspace/IONAME/Ladder/Advanced opens, IPC and zoom work | Manual Windows smoke | NOT RUN |
| No change to CIOPRG/IONAME at the byte level | Rust regression | PASS - Automatic Tests; Independent UserSmoke NOT RUN |
| File-handle/no-follow, symlink race testing, policy for Windows junction | Separate task | OPEN |

** Decision:** PR #25 eliminate `csp:null` and reduces the risk class symlink-ancestor, but **M2 remains PARTIAL** and the robot writer and HSES/FTP forbidden.

**Note to history:** First implementation PR #25, в PR #26 Added guarded read and Windows reparse check. It shoots open file races and manual WebView2 validation. [De facto status](project_status.en.md) · [single gates](verification_gates.en.md).

## Current addition after PR #28–#29

В `main` later [PR #29](https://github.com/uzbatsu-beep/MLAD/pull/29) Windows reparse/junction tests, hardlink rejection, staged-file SHA-256File ID snapshot and Windows ADS/UNC/device deny [Windows CI PASS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621). This section does not rewrite the historical volume. PR #25**: final check→rename, parent directory races, ACL, crash durability and manual WebView2 CSP/IPC acceptance is still open. [Updated threat review](filesystem_security_pr29.en.md).
