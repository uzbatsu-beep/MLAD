<!-- Generated en companion for docs/windows_acceptance_pr28.md; review required. -->
> **English** · [Русский](windows_acceptance_pr28.ru.md) · [English](windows_acceptance_pr28.en.md) · [Original / Оригинал](windows_acceptance_pr28.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.
> **MLAD safety invariants:** `CIOPRG.LST` and `IONAME.DAT` are strictly **read-only**. FTP/HSES, live I/O, native writer and robot deployment remain **BLOCKED**. Passing `WindowsCI` does **not** imply `UserSmoke=PASS` or `OEMBenchVerified`: both remain **NOT RUN** unless independently documented. Machine translation requires source and Yaskawa manual review.

# MLAD PR #28 — Windows Acceptance & Security Regression

**Scope:** Windows 10/11 x64 . WebView2, NTFS, Rust/Tauri v2, OFFLINE/BENCH. **Base:** `main` later [PR #27](https://github.com/uzbatsu-beep/MLAD/pull/27), SHA `bdb443eb65dc134edc2e11ffc3230c9dfa6257df`. Status: [PR #28 merged](https://github.com/uzbatsu-beep/MLAD/pull/28) в `main` SHA `3c184cd36e076dc2e5c9aed7a663b601b4899dea`; [Linux CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44365314264), [Windows CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44365314173). **No* release and no hardware acceptance; UserSmoke still `NOT RUN`.

## 1. What is truly automated

New [Windows workflow](../.github/workflows/windows-security-acceptance.yml) на `windows-2022` launches:

1. `npm ci` lockfile, frontend/security/native/docs verification and `npm run build`.
2. Node-contract [windows-security-contract.test.mjs](../scripts/windows-security-contract.test.mjs)Offline IPC deny-only, production CSP, prohibition of external resources, mandatory Windows tests.
3. Complete. `cargo test --manifest-path src-tauri/Cargo.toml --locked` *On Windows*, plus checking for compiled Windows-specific tests.
4. Negative Rust tests with **NTFS exclusive sharing lock**`share_mode(0)`) and this Windows **junction** (`mklink /J`). The original bytes are saved when JSON writes/native read fails. External JSON Change, Competitive Creation and File Identity Tests.
5. `npm run tauri -- build --no-bundle`Checking the existence of EXE, PE `MZ` header, SHA-256 Authenticode status (including honest status) `NotSigned`).
6. **Exclusively synthetic** CIOPRG/IONAME sample; no real robot programs or production addresses.
7. GitHub Actions artifact **unsigned EXE**, checksum and report with storage of 14 days; GitHub Release and tag are not created.

**Not automatically tested:** visible GUI WebView2True robot, penetration-test, aggressive directory races, absolute atomicity backup+target, OEM editor compatibility. GitHub Actions **no** puts out `UserSmoke=PASS` or `OEMBenchVerified=PASS`.

## 2. Test artifact

Open workflow. PR #28 Successful launch `MLAD Windows Acceptance & Security Regression` → `MLAD-PR28-Windows-Unsigned-Acceptance-NonRelease`. In it:

- `yaskawa-ladder-ide.exe` Unsigned test EXE with the requirement of WebView2 Runtime, not production release.
- `SHA256SUMS.txt` Checksum is exactly this assembly.
- `binary-evidence.json` SHA, size, Authenticode and `NOT RUN` for manual/OEM tests.
- `LICENSE`This protocol and `windows-acceptance.ps1`.

Do not run EXE on a production network or on a computer with unintended access to controllers. Verify only your own synthetic data or permitted local copies. A Windows SmartScreen warning about an unsigned EXE does not confirm its danger or security; follow the organization's rules.

## 3. Preparation of manual inspection

Set up the Windows build (S)`winver`), WebView2 Runtime, build SHA from `binary-evidence.json`Executor, date and results. Check EXE:

~~~powershell
Get-FileHash .\yaskawa-ladder-ide.exe -Algorithm SHA256
Get-Content .\SHA256SUMS.txt
~~~

Create a new test folder (re-starting Prepare in an existing folder is deliberately rejected):

~~~powershell
$fixtures = Join-Path $env:TEMP ("MLAD-PR28-" + [guid]::NewGuid().ToString("N"))
pwsh -NoProfile -File .\windows-acceptance.ps1 -Mode Prepare -FixtureRoot $fixtures
~~~

Subfolder `native` contains **artificial ** `CIOPRG.LST` и `IONAME.DAT`: they are NOT OEM-compatible and are intended to demonstrate read-only, unknown/error handling and SHA-256. folder `drafts` - for **created through UI** JSON projects. Displaying unsupported/opaque or rejecting the demo format is normal; it does not confirm the completeness of the parser.

## 4. Manual matrix UserSmoke

For each ID, write down `PASS`, `FAIL` or `NOT RUN` and observation. All scripts were originally `NOT RUN`.

| ID | Script | Criteria |
| --- | --- | --- |
| WIN-01 | Starting EXE / creating a window | Tauri/WebView2 UI shows no crash/blank page |
| WIN-02 | Native Workspace | System Open selects synthetic `CIOPRG.LST`, shows the source/reports a distinct error |
| WIN-03 | Read-only / diagnostics | Source/opaque/messages are visible correctly, there is no operation to save native `.LST` |
| WIN-04 | SHA fidelity | Repeated `-Mode Verify` confirm SHA-256 both source files |
| WIN-05 | `IONAME.DAT` | Read-only preview or correct opt-out for artificial format; original unchanged |
| WIN-06 | Graph JSON | Create/Twice Keep Separate `*.graph.json`check out `*.bak` reopening |
| WIN-07 | Advanced JSON | Create/twice save `*.advanced.json` check backup |
| WIN-08 | Locked JSON | Check the failure of the blocked file without truncation; if not reproduced - `NOT RUN` |
| WIN-09 | Invalid/oversize | Incorrect synthetic data does not hang UI, reported error |
| WIN-10 | CSP/local IPC | Search, zoom, Open/dialog and IPC work; the presence of this UI does not prove the absence of external queries |
| WIN-11 | Deny-only | No active LIVE, HSES/FTP upload, native export/writer; no robot connection |
| WIN-12 | Exit/reopen | Restart and reopen JSON, source files unchanged |

After work:

~~~powershell
pwsh -NoProfile -File .\windows-acceptance.ps1 -Mode Verify -FixtureRoot $fixtures
~~~

SHA mismatch means **FAIL* and stop testing. Verify confirms **only the immutability of artificial sources**, not the correctness of the Ladder or the operation of the GUI.

## 5. Boundaries of evidence

- **Concurrent JSON:** Rust test simulates the change between staging and commit. Changing the file before starting Save from the interface and race after the last check is still not guaranteed to be detected.
- **Windows lock:** Behavior depends on sharing mode and OS. The new Rust-test applies a strict `share_mode(0)`The manual test may remain `NOT RUN`.
- **Reparse:** The test checks a *stable* junction, a non-interprocess race with a substitution ancestry. Hardlinks/ACL/power loss is an open risk.
- **CSP:** Configuration and production build checked automatically WebView2IPC runtime is not confirmed until actual testing.
- **Robot/network:** Prohibitions are checked autonomously and do not involve attempts to work on production equipment.

## 6. Acceptance protocol (complete after verification)

| Field. | Status |
| --- | --- |
| Artist/Date/Windows build/WebView2 | NOT FULLED |
| Artifact URL/Git SHA/SHA-256 | NOT FULLED |
| WIN-01…WIN-12 | `NOT RUN` every scenario |
| Synthetic SHA-256 Verify | `NOT RUN` |
| Windows automated CI | Only the actual run URL + status |
| UserSmoke Results | `NOT RUN` |
| OEM bench | `NOT RUN` |
| HSES/FTP/native writer/deployment | `BLOCKED` |
| Defects/evidence | Do not attach real files/addresses/passwords |
| Decision on acceptance/release | Separately agree with the owner |

[Verification gates](verification_gates.en.md) · [Implementation plan](implementation_plan.en.md) · [Threat review](guarded_io_review.en.md).

## 7. Addition after merged PR #29

[Filesystem review PR #29](filesystem_security_pr29.en.md) already included in `main` SHA `f85ddfcce3063dd087425be981a9d2b7a16d6309`; [Windows CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621) Check the same-content target swap, hardlink alias, staged-file integrity and Windows ADS/UNC/device rejection. These are automatic checks, they do not replace any WIN-01...WIN-12. Windows UserSmoke still needs a manual protocol on a specific EXE and SHA.

## 8. Receiving Windows after PR #29

The next test build for UserSmoke should refer to a specific main/tag SHA and SHA256SUMS**, not rc.9. For the updated filesystem code, additionally check the correct failure in case of corrupted JSON, NTFS reparse/junction, catalog availability problems and alternative paths; **not** experiment with the files of the real controller. Successful Windows CI confirmed, but separate manual `UserSmoke` remains `NOT RUN` before the completed protocol.
