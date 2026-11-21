<!-- MLAD-I18N:NAV:START -->
> 🌐 **Язык / Language:** [Русский](windows_offline_preview_pr48.ru.md) · [English](windows_offline_preview_pr48.en.md) · [Original / Оригинал](windows_offline_preview_pr48.md). **Machine translation / Машинный перевод: editorial review required / необходима редакторская проверка.**
<!-- MLAD-I18N:NAV:END -->

# MLAD PR #48 — Windows Offline Preview and user acceptance

**Status:** preparation of an unpublished preview artifact, NOT an OEM release or acceptance. **Baseline:** merged [PR #47](https://github.com/uzbatsu-beep/MLAD/pull/47), `main` SHA `e72a51f8ea69466e716fb268a62b1cf1bd38d6f2`. **Platform:** Windows 10/11 x64, Tauri 2, Microsoft Edge WebView2 Runtime and NTFS. This guide defines manual UserSmoke after automated CI on the exact PR #48 SHA.

## 1. Preview contents

The artifact `MLAD-PR48-Windows-x64-OfflinePreview-Unsigned-NonRelease` is produced by the [Windows Preview workflow](../.github/workflow-archive/windows-offline-preview-pr48.yml). Download it from a SUCCESSFUL GitHub Actions run **Artifacts**, NOT Releases; no Git tag is created. The inner ZIP contains:

- `app/yaskawa-ladder-ide.exe`: unsigned Windows x64 production build, not a signed installer.
- `sample/cell.mladproj.json`, `sample/CIOPRG.LST`, `sample/IONAME.DAT`, `sample/cell.mlad.json`: a completely **SYNTHETIC** unified project from the [fixture generator](../scripts/offline-preview-fixture.py). These bytes do not claim OEM-compatible robot code and must never be uploaded.
- `sample/SYNTHETIC-SHA256.json`: independent inventory of four synthetic files, while `SHA256SUMS.txt` covers every staged member.
- `build-evidence.json`: exact Git SHA, EXE SHA256, Authenticode signature status and `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`.
- `corresponding-source.zip`: source for the exact executable commit and GPL-3.0-only `LICENSE`; `tools/windows-offline-preview.ps1`: offline integrity verification; this protocol in `docs/WINDOWS-OFFLINE-USER-SMOKE.md`.

CI **automates** locked `npm ci`, frontend contracts, Windows/NTFS Rust tests, synthetic fixture generation/tamper-negative tests, Tauri PE production build via `tauri build --no-bundle`, SHA verification and artifact upload. **CI does not launch or interact with the WebView2 GUI**.

## 2. Preparing Windows safely

1. On an isolated Windows 10/11 x64 machine/VM with WebView2 Runtime, extract BOTH the GitHub Actions artifact archive and its inner ZIP into a normal NTFS directory. Disconnect from production robot networks during the review. Do not use real production `CIOPRG.LST` or `IONAME.DAT`.
2. Record `winver`, WebView2 version, exact SHA from `build-evidence.json`, Actions run URL, tester and date. Verify provenance and checksums before launch. This EXE is unsigned; an organizational security exception must be separately authorized, not bypassed by these instructions.
3. In PowerShell from the extracted inner ZIP root, run **file-integrity only** (stock Windows PowerShell 5.1 is supported):

~~~powershell
powershell -NoProfile -File .\tools\windows-offline-preview.ps1 -Mode Verify -StageRoot .
Get-FileHash .\app\yaskawa-ladder-ide.exe -Algorithm SHA256
Get-Content .\build-evidence.json
~~~

4. Copy the WHOLE `sample` directory to a separate writable test folder, e.g. `C:\MLAD-PR48-UserSmoke\cell`. Perform **all** edits, missing-file tests and tamper tests only on independent copies. Do not share production secrets, source exports or controller addresses.
5. Start the EXE in `app`, navigate to Native Workspace, provide the full working-copy path to `cell.mladproj.json` in the offline project field and select Open project. Standalone `CIOPRG.LST` opening remains available.

## 3. Manual UserSmoke matrix

All scenarios default to `NOT RUN`. Only the tester may assign **PASS** after observing actual behavior, **FAIL** with redacted evidence, or **BLOCKED** when a safe procedure is unavailable.

| ID | Action (on copies only) | Expected outcome |
| --- | --- | --- |
| P48-01 | Launch EXE in Windows | WebView2 window opens, no blank page, crash or CSP/IPC failure |
| P48-02 | Open `cell.mladproj.json` | Source, IONAME, sidecar, bookmark and analysis load together |
| P48-03 | Inspect 3-line `CIOPRG.LST` | STR, OUT and END appear; original native bytes remain unchanged |
| P48-04 | Navigate L1/L2/bookmark | Source ↔ Ladder ↔ diagnostics links work without fabricated execution |
| P48-05 | Filter `READ`/`WRITE`/`UNKNOWN` | Only source-linked potential roles, no live I/O |
| P48-06 | Inspect `IONAME.DAT` labels | Synthetic names appear; no inferred physical relay mapping |
| P48-07 | Edit note/bookmark and save sidecar | Independent `*.mlad.json` saved, native files untouched |
| P48-08 | Save manifest after sidecar | `*.mladproj.json` saved; second save checks revision/backup |
| P48-09 | Move entire project folder | Portable relative references reopen without previous absolute path |
| P48-10 | Change `CIOPRG.LST` bytes in a copy | SHA mismatch aborts reopen without replacing prior session |
| P48-11 | Change `IONAME.DAT` in a separate copy | Hash mismatch aborts open without partial session state |
| P48-12 | Change `cell.mlad.json` in a separate copy | SHA/revision rejection, no silent unrelated cached notes |
| P48-13 | Remove one component in a copy | Clear missing-file error; previous session retained |
| P48-14 | Edit source path with project attached | Explicit Detach required; source cannot silently change |
| P48-15 | Review parser-version/stale-analysis indicators | `analysis_changed` marks recomputation; saved counts not executed |
| P48-16 | Check no live/FTP/HSES/deploy/native writer | Passive UI/IPC inspection only, never connect to any robot |
| P48-17 | Re-run `Verify` on the **untouched** package | SHA values still match; not based on edited working copies |

Use a new independent copy for each negative scenario P48-10…P48-13. Verify only the untouched original extracted sample, not the intentionally modified working copies. File integrity verification is **not** a GUI test.

## 4. Report and decision rule

Copy the P48-01…P48-17 matrix into your report and give each case `PASS`, `FAIL`, `BLOCKED` or `NOT RUN`. Record Windows build, WebView2 version, source SHA, Actions link, EXE hash, date, redacted problems and offline launch result. Do not attach private robot exports, credentials or sensitive addresses.

Only the human tester can report `UserSmoke=PASS`, and only when **all mandatory scenarios** P48-01…P48-17 have been completed and marked PASS and the evidence is reviewed. Any FAIL, BLOCKED or NOT RUN means `UserSmoke=NOT PASSED` or `NOT RUN`. Merging the PR, publishing a Release, or enabling hardware interfaces is a separate owner decision.

## 5. Safety limits

- No OEM controller/firmware, System/User Ladder partition, EN/scan/Carry/Zero/Error acceptance has been tested: `OEMBench=NOT RUN`.
- Native `CIOPRG.LST` and `IONAME.DAT` are strictly read-only; `FTP`, `HSES`, native writer, live I/O and deployment remain `BLOCKED`.
- Unsigned preview EXE only, no publisher certificate or setup installer. Do not bypass organizational security policies.
- Windows DACL/ownership inheritance, parent check-to-open/rename races, true multi-file atomicity and concurrent writer transaction semantics remain unverified.
- A successful Windows CI means reproducible build, tests and packaging; it NEVER certifies interactive `UserSmoke` or hardware `OEMBench`.

[Windows security baseline](windows_acceptance_pr28.md) · [Project manifest plan](implementation_plan.md) · [Verification gates](verification_gates.md).
