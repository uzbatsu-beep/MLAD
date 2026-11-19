<!-- Generated en companion for docs/filesystem_security_pr29.md; review required. -->
> **English** · [Русский](filesystem_security_pr29.ru.md) · [English](filesystem_security_pr29.en.md) · [Original / Оригинал](filesystem_security_pr29.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.
> **MLAD safety invariants:** `CIOPRG.LST` and `IONAME.DAT` are strictly **read-only**. FTP/HSES, live I/O, native writer and robot deployment remain **BLOCKED**. Passing `WindowsCI` does **not** imply `UserSmoke=PASS` or `OEMBenchVerified`: both remain **NOT RUN** unless independently documented. Machine translation requires source and Yaskawa manual review.

# MLAD PR #29 — Filesystem Security Hardening: threat review and evidence

** History:** PR #29 designed `main` SHA `3c184cd36e076dc2e5c9aed7a663b601b4899dea` и [merged](https://github.com/uzbatsu-beep/MLAD/pull/29) 18.11.2026 в `main` SHA `f85ddfcce3063dd087425be981a9d2b7a16d6309`. Final inspections [Linux CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454) и [Windows CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621) on this SHA. Historical [Linux PR CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44369892563) и [Windows PR CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44369892566) They're also successful. **UserSmoke/OEMBench = NOT RUN; this is not a release.

## 1. Boundaries of trust and attacking

Working context - **OFFLINE/BENCH IDE** which reads `CIOPRG.LST` и `IONAME.DAT` exclusively for viewing and maintains independent `*.graph.json`, `*.advanced.json` и legacy `*.json`. Tests do not use real robot programs/signals.

Potential Intruder: Another local process with the ability to change the selected directory, file/backup, hardlinks, junction/reparse, or change JSON between reading and saving. **Not meant protection from local administrator, malicious kernel, untrusted file server, supply chain attacks, or Tauri runtime compromise. Folder access and ACL rights are set by the OS and remain a major barrier.

Critical invariants: (a) the original native bytes are unchanged, (b) the writer does not receive the native export path, (c) the JSON allowable changes do not destroy the competitive version when the substitution is detected, (d) the failure must not leave a corrupt file, (e) no item includes the robot network, HSES/FTP or deployment.

## 2. Real changes regarding PR #28

| vector | Previously | PR #29 | Evidence/status |
| --- | --- | --- | --- |
| Target replaced with other **same bytes* * | `read_bounded` I only checked the contents to rename. | `read_regular_with_identity` returns bytes and **FileIdentity of one open handle**: Windows volume+file index, Unix dev+inode; JSON conflict reconciles both fields | `same_content_target_replacement_is_a_save_conflict`checked by Rust CI |
| Hidden JSON hard link to source `CIOPRG.LST` file | Normal. `metadata.is_file` did not distinguish alias | `verify_single_link` Requires a link count of exactly 1 for existing JSON target `.bak`Otherwise, fail-closed | `hardlink_alias_to_native_source_or_json_backup_is_rejected` |
| Replacement/modification of staged `.tmp` rename | Created through `create_new`carried out `sync_all`but not reconciling | `StagedFile` FileIdentity is an open staged handle and SHA-256 from expected content; before rename rereads the staged file and compares identity + checksum | `staged_file_modified_after_flush_cannot_be_committed` |
| Windows Preliminary IPC Verification | `reject_existing_symlink` checked `is_symlink` | Use it. `is_indirection` (all reparse attributes found), then the main writer checks both the ancestor and the target again | Windows CI + security source contract |
| Windows ADS/UNC/device path bypasses | `*.graph.json` Could be part of NTFS alternate stream after colon or Windows path hit UNC`\\.\\`/verbatim namespace | `verify_parent_directory` reject `:` in normal path components, UNC/device/verbatim and drive-relative prefixes; normal local absolute `C:\\...` relative | `windows_ads_unc_and_device_paths_are_rejected_before_filesystem_io` + Windows CI |
| Check path with junction and lock | PR #28 | Tests are kept. PR #28 on NTFS with real `mklink /J` and exclusive lockdown | Linux/Windows CI with no claim to complete atomicity |

** Technical form:** `FileIdentity` form `File::metadata` on Unix, from WinAPI. `GetFileInformationByHandle` Windows. For Windows, in addition to volume/file index `nNumberOfLinks` same `BY_HANDLE_FILE_INFORMATION`. Any error in obtaining a reliable identity/link count results in a failure rather than a continuation of the record. New. `Cargo`/npm dependencies are not added, lockfiles have not changed. Windows UNC, extended-length/verbatim and device/ADS paths are intentionally not supported: select a **local, regular copy of the file** to work with.

## 3. Residual risks: the status of OPEN

| Threat. | Why isn't it closed? PR #29 | Next possible stage/approach |
| --- | --- | --- |
| `check → rename` TOCTOU and replacement of parent ancestor | Last File ID/Hash Verification Separated from OS `rename`The pathname can change the target at that moment. Reading current `guarded_read` is not `openat`/relative-handle traversal | Windows: Open/hold directory handles with control `FILE_SHARE_DELETE`, identity и reparse; Unix: `openat2` (`RESOLVE_BENEATH/NO_SYMLINKS`) or verifiable openat circuit; ** separate security design** |
| Windows hardlinks at *moment* after link count check | Link count=1 – instant snapshot, no more blocking `CreateHardLink` | Directory handle/ACL and descriptor-bound rename semantics; separate test |
| ACL/owner/attributes when replacing JSON | Staged file inherits the ACL directory. `rename` It can change the DACL/owner/attributes from the original target, even if the record is successful. ACLs are not copied | Windows ACL inheritance/security descriptor policy, explicit DACL review and privilege model |
| Simultaneous Writers Processes | Snapshot comparison prevents identified changes, but two writers can get checked at the same time | Version token from UI, process lock, OS-specific conditional replacement; do not consider support for multi-writer closed |
| Backup + target | Two separate renames, `*.bak` It can be updated when a target error occurs; power loss can affect only part of the operation. | Journaled commit protocol with recovery/FS sync + crash-test; delete "completely atomic" statements |
| Crash, power outage, SMB/ReFS/UNC | Tests do not confirm crash durability and remote FS identity; direct UNC is rejected by the new guard, but other remote/mounted cases require separate evaluation. | Local NTFS, fault injection, recovery protocol |
| Real client files | Git/CI only uses synthetic data | Private corpus and independent OEM verification, see [source-reception](controller_fixture_intake.en.md) |

## 4. Acceptance matrix

| Gate | What to do | Expected evidence |
| --- | --- | --- |
| Linux compiler/Rust | `cargo test --manifest-path src-tauri/Cargo.toml --locked` | All existing and new tests are completed |
| Windows MSVC + NTFS | [windows-security-acceptance.yml](../.github/workflows/windows-security-acceptance.yml) branch | Including three new PR #29 Rust tests and four Windows tests PR #28 |
| Offline frontend/security | `npm ci`; `npm run test:security`; `npm run test:native`; `npm run docs:check`; `npm run build` | No network module; monitored/deploy deny-only; unchanged lockfile |
| Windows Tauri artifact | Production `npm run tauri -- build --no-bundle` | Temporary unsigned non-release artifact; PE + SHA-256. *Not* WebView2 manual test |
| Byte immutability | Rust fixtures + manual [WIN-01–WIN-12](windows_acceptance_pr28.en.md) | Synthetic originals are unchanged; UserSmoke `NOT RUN`No report yet. |
| Hardware | **Do not comply** | `OEMBench=NOT RUN`; HSES/FTP/writer `BLOCKED` |

## 5. CI results and open tasks after merge

- [x] PR HEAD `0e1eae035309a1bc8a43594323b1197deb57a47f`: Linux/Windows CI SUCCESS; after merge `main` SHA `f85ddfc` — Linux/Windows CI SUCCESS.
- [x] Windows `cargo test --locked` Windows-specific regression tests are performed in GitHub Actions.
- [x] Available Rust unit/regression tests JSON staging/backup/oversize/lock passed on Windows runner; ** manual UI test is still NOT RUN**.
- [x] The Rust same-content file ID swap test returns the conflict to rename; the last race window remains OPEN.
- [x] Synthetic Rust test confirms the absence of native writing in the tested scenarios. HSES/FTP/native writer remains BLOCKED.
- [x] No statement. `TOCTOU fixed` / `atomic backup+target`A separate architectural review directory handles/ACL and crash recovery remains OPEN.
- [x] PR #29 merged; independent Windows UserSmoke/Verification WebView2 It's still **NOT RUN* and should be separately recorded.

See. [PR #26 review](guarded_io_review.en.md), [verification gates](verification_gates.en.md), [Windows protocol](windows_acceptance_pr28.en.md), [implementation plan](implementation_plan.en.md).

## 6. After PR #29 The next risk review

**Executed inspections are only detectable cases. ** For the new versioned sidecar and any subsequent edit-paths, separately evaluate the atomicity of target+backup, parent-handle traversal, ACL/owner inheritance, simultaneous UI session save and crash recovery. §main§ includes PR #29But **M2 remains PARTIAL. [Current roadmap v3](roadmap.en.md) · [plan](implementation_plan.en.md) · [Windows manual acceptance](windows_acceptance_pr28.en.md).
