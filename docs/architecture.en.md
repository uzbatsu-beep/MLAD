<!-- Generated en companion for docs/architecture.md; review required. -->
> **English** · [Русский](architecture.ru.md) · [English](architecture.en.md) · [Original / Оригинал](architecture.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.
> **MLAD safety invariants:** `CIOPRG.LST` and `IONAME.DAT` are strictly **read-only**. FTP/HSES, live I/O, native writer and robot deployment remain **BLOCKED**. Passing `WindowsCI` does **not** imply `UserSmoke=PASS` or `OEMBenchVerified`: both remain **NOT RUN** unless independently documented. Machine translation requires source and Yaskawa manual review.

# MLAD Architecture and Boundaries of Trust

**Actual baseline 18.11.2026:** `main` `f85ddfcce3063dd087425be981a9d2b7a16d6309` later [merge PR #29](https://github.com/uzbatsu-beep/MLAD/pull/29). [Linux CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454) и [Windows CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621) successful. [Project status](project_status.en.md), [Roadmap v3](roadmap.en.md), [Backlog](implementation_plan.en.md). Native source is strictly read-only. FTP/HSES/deploy off.

## 1. Data layers and flow

```text
React / TypeScript / Tauri WebView
├── App.tsx
│   ├── NativeWorkspace.tsx — основная read-only программа робота
│   │   ├── исходник CIOPRG / дерево / поиск / пагинация / диагностика
│   │   ├── LadderCircuit и AdvancedBlockDiagram — только supported preview
│   │   ├── IONAME и локальные комментарии (localStorage)
│   │   └── typed operand / static M-links / OEM reference справка
│   ├── GraphWorkspace.tsx — отдельные *.graph.json drafts
│   ├── AdvancedBlocksPanel.tsx — отдельные *.advanced.json drafts
│   └── Legacy project.json — только совместимость
│
└── Tauri IPC (src-tauri/src/main.rs)
    ├── cmd_inspect_cioprg / cmd_read_cioprg_page / cmd_search_cioprg
    ├── cmd_inspect_ioname / cmd_pick_native_file (Open only)
    ├── compiler/lossless.rs — immutable bytes, SHA-256, spans
    ├── compiler/native_sections.rs — PART/END/tail views
    ├── compiler/semantic.rs — ограниченный Boolean AST
    ├── compiler/advanced_source.rs — source-pattern previews
    ├── compiler/memory.rs — family-specific address classes
    ├── compiler/typed_dataflow.rs — hypothetical register links
    ├── compiler/reference_semantics.rs — isolated manual-derived vectors
    │   (НЕ подключено к CIO исполняющему AST или Tauri command)
    ├── compiler/ioname.rs — ограниченный read-only структурный preview
    ├── guarded_read.rs — bounded snapshot и identity/reparse проверки
    ├── draft_io.rs — staged JSON write, backup, conflict guard
    └── legacy draft/parser helpers — отдельные JSON модели
```

**Network runtime:** the old one `src-tauri/src/network/mod.rs` remotely PR #13; `pub mod network` в `main.rs` missing. Tauri `cmd_toggle_monitoring` refuses inclusion; `cmd_deploy_to_robot` Always refuses. This boundary stands for OFFLINE, but is not a complete security sandbox check.

## 2. Formats, preservation and rights

| Format | Operation | Status |
| --- | --- | --- |
| `CIOPRG.LST` | Native Open + read-only inspect/paging/search/hash | The original is unchanging. No native writer, recompiler, overwrite |
| `IONAME.DAT` | Native Open + bounded structural preview | Read only, indexes on #XXXXX are not confirmed |
| Local native comments | localStorage, keys to SHA-256 | **No* OEM annotations in file; portable sidecar not yet |
| `*.graph.json` | Validation/preservation of own format | Editable separate JSON draft, not native CIO |
| `*.advanced.json` | Validation/preservation of own format | Separate JSON draft, not native CIO |
| `project.json` | Old compatible format | Legacy, not a robot converter |
| Local TMR/CNT/GSTR test steps | Determined **hand** steps | Not a real scan/IO |
| HSES / FTP | Deny-only Tauri IPC | **OFFLINE/banned* * |

Backend does not provide a working command to save the modified CIOPRG and does not include a robot program. PR #24 It's already merged (JSON staged/rename). **PR #25 merged** implements production CSP and ancestral path checks; CSP** is not yet available in published rc.9** PR #29 File ID/byte snapshots, hardlink rejection and Windows ADS/UNC/device path deny. Race windows/TOCTOU, ACL/power-loss and Windows user smoke. [Review](filesystem_security_pr29.en.md).

## 3. Three levels of interpretation

1. **Lexical/structural:** source bytes, BOM, CRLF/LF/CR, spans, PART/END, observed ALARM/MESSAGE sections after END. Unknown remains unknown.
2. **Typed syntax:** `MOV/ADD/SUB/SHL/SHR` how `TypedCandidate`Operands and possible M-links. All new “connections” are only textual hypotheses; boolean AST gets a barrier, not a fulfillment.
3. **Manual-derived reference:** Isolated 16-bit vector check function and OEM help in UI. Do not read or modify the meaning of `CIOPRG.LST`; `controller_execution_verified=false`, `scan_chain_verified=false`.

**No to level four OEM-certified execution. Carry/Zero/Error flags refer to manual and reference vectors, not controller states.

## 4. Diagnostics and interface boundaries

- Native Workspace: Page-by-page source file, separate supported/typed/opaque, all available semantic chains and advanced source preview, ALARM/MESSAGE section, transition-saving diagnostics grouping.
- Scaling is independent for Ladder and Advanced. Selection of sources is only system Open dialog.
- In editable JSON, double-click/F2 changes JSON addresses. In a native view, a double-click changes **only local comments**, not the address itself.
- TRUE/FALSE/UNKNOWN is not a live telemetry. PART 1/2: Observable tags not confirmed by System/User rights.

## 5. Open security/engineering debt

1. PR #25 merged: CSP/ancestor checks; PR #26 merged: guarded reads/conflicts; PR #28 merged: real Windows NTFS test workflow PR #29 merged: File ID + bytes, hardlinks, staged SHA-256, Windows path deny. Last check→rename TOCTOU, directory handles, ACL/Windows edge cases, concurrent writers and no backup+target transaction remain.
2. Real golden fixtures with provenance DX100/DX200/YRC1000 и firmware.
3. Full native grammar, byte/word aliasing, System/User ownership.
4. Interaction of flags and EN conditions at full scan; extreme MOV byte=256, SHL/SHR n=0/16.
5. Portable sidecar with saving comments and controlling hashes of external files.
6. Third-party SBOM/notices, Windows EXE signature and custom smoke test.

Open topics — in [roadmap](roadmap.en.md); OEM testing - in [matrix](semantic_verification_matrix.en.md). Passing Rust/TypeScript/Windows CI alone does not cover these risks.


## 6. PR #24 offline JSON persistence (merged, not in rc.9)

Three JSON IPC Recording Methods Use One Rust `draft_io::write_atomic` For a temporary file with a unique name, `sync_all` and replace the file in the same folder. Graph/Advanced/legacy JSON readers are limited to 2 MiB and reject direct symlink. Backup copy `*.bak` It is formed separately and non-atomically simultaneously with the target JSON. It's a ***** capability grant to record. CIOPRG.LST or IONAME.DAT, no FTP/HSES. See. [security](json_persistence_safety.en.md).


## 7. PR #25 — CSP and linked path rejection (merged)

Setting up `app.security.csp` Limits external loads in WebView packaged `devCsp` Only Vite localhost HMR is allowed. Inline CSS/React Styles Require Exclusion `style-src 'unsafe-inline'`; JS remains without `unsafe-inline` и `unsafe-eval`. For files. `draft_io::verify_parent_directory` Preliminarily detects symlink ancestors/ParentDir, but does not prevent the directory descriptor from changing. [Detailed limitations](csp_path_hardening.en.md).


## 8. PR #26 — bounded identity-checked read and JSON conflict guard (merged)

`guarded_read::read_regular` Uses a single descriptor for content (on Windows - additional control descriptors), first and after reading, compares the Unix dev/inode or Windows volume/file index. Check symlink/reparse, size and mtime. Native CIOPRG and IONAME `spawn_blocking` They don't provide a writer. `draft_io::write_atomic` Compares target bytes before the final rename; returns conflict instead of silent lost update. Details: [guarded_io_review.md](guarded_io_review.en.md).

## 9. PR #28/#29 — Windows regression и filesystem hardening (merged)

[Windows workflow](../.github/workflows/windows-security-acceptance.yml) later PR #28 Performs tests on Windows runner, compiles Tauri EXE with production CSP and forms a temporary unsigned artifact. In PR #29 JSON read/save extended `FileIdentity` snapshots and hardlinked target/backup ban; staged JSON compares File ID and SHA-256 before renaming it. Direct Windows alternate streams, UNC/device/verbatim and drive-relative paths are rejected; to work, choose regular local copies. [Threat review](filesystem_security_pr29.en.md). **Not guaranteed** directory-handle atomicity, ACL retention, crash durability and pair transaction `backup+target`. В `Cargo.toml` still listed `suppaftp` Dependency: A separate dependency check does not mean that the network runtime is active.

## 10. Architectural route after PR #29

**Synthetic native parser corpus → source navigationrung navigation → versioned sidecar `*.mlad.json` → evidence-based typed syntax → source-linked static analysis → unified Offline Engineering IDE → conditional Windows 0.6.x preview. [Implementation plan](implementation_plan.en.md) · [Verification gates](verification_gates.en.md) · [Decisions D1–D9](engineering_decisions.en.md). No document authorizes native recording or controller connection.
