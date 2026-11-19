<!-- Generated en companion for README.md; review required. -->
> **English** · [Русский](README.ru.md) · [English](README.en.md) · [Original / Оригинал](README.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.
> **MLAD safety invariants:** `CIOPRG.LST` and `IONAME.DAT` are strictly **read-only**. FTP/HSES, live I/O, native writer and robot deployment remain **BLOCKED**. Passing `WindowsCI` does **not** imply `UserSmoke=PASS` or `OEMBenchVerified`: both remain **NOT RUN** unless independently documented. Machine translation requires source and Yaskawa manual review.

# MLAD — Motoman Ladder Analysis & Draft IDE

**Verified on 18 November 2026:** [main](https://github.com/uzbatsu-beep/MLAD) at `f85ddfcce3063dd087425be981a9d2b7a16d6309`; **PR #1–#29 merged**. [Linux CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454) and [Windows CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621). The latest published Windows release is **rc.9** and does **not** include PR #24–#29. See [roadmap v3](docs/roadmap.en.md) and [project status](docs/project_status.en.md). Manual WebView2 smoke testing and OEM bench verification are **NOT RUN**.

**Last published Windows x64 Preview:** [v0.5.0-rc.9-oem-reference](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference) (portable ZIP, NSIS setup, GPL source archive, SHA-256). [Main CI - SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44166354563), [Windows RC9 — SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44162078155).

> **Safety:** MLAD is neither a Yaskawa-certified engineering environment nor a verified native `CIOPRG.LST` compiler. Original `CIOPRG.LST` and `IONAME.DAT` are **read-only**. Native writing, FTP/HSES, robot deployment and live telemetry are **disabled**. Use manufacturer-approved tools and independent authorization for any work involving physical equipment.

## Implemented functionality

| Function | Status |
| --- | --- |
| Profiles DX100/DX200/YRC1000 | Typed address formats and range checking; Controller mapping/firmware still needs to be confirmed |
| Native Workspace | Main work area: compact header, system opening dialogue, navigation tree, paginated source text, search and diagnosis |
| CIOPRG.LST | Byte-for-byte immutability, SHA-256, sections after `END` As data, secure classification of supported/unknown |
| Ladder | **All recognized** chains of the current SemanticPreview, scale, references to source code; unknown fragments are not completed |
| TMR/CNT/GSTR/GOUT | View native patterns and individual **offline drafts** `*.advanced.json` |
| IONAME.DAT / Comments / | Read-only preview, local chain/address comments, manual associations; originals do not change |
| `MOV`, `ADD`, `SUB`, `SHL`, `SHR` | Typed operands and source spans, static **presumed ** M-dependencies |
| OEM reference | Documentary formulas EN/16-bit/Carry/Zero/Error and stand-alone reference tests; **not** source execution on controller |
| JSON Graph Draft | SVG, AND/OR/NO/NC/OUT, Undo/Redo, double-click/F2 at TRUE/FALSE/UNKNOWN |
| Legacy JSON | Compatibility with the old `project.json` through a separate menu item |

** Preservation and protection of JSONPR #24–#29 merged):** staged write + `*.bak`, limited reading, conflict over bytes ** and ** File ID, failure for hardlinks, checking the temporary file on SHA-256Windows ADS/UNC/device, production CSP. This is a *****backup+target transaction and a ******** full TOCTOU/ACL/crash recovery solution. [Security review](docs/filesystem_security_pr29.en.md), [verification gates](docs/verification_gates.en.md). Manual Windows UserSmoke - **NOT RUN**

** Format boundaries:** `CIOPRG.LST`/ `IONAME.DAT` - only the source; `*.graph.json` / `*.advanced.json` - individual variable drafts. You cannot save JSON as native CIOPRG. Comments from native Workspace are saved in localStorage and **n* are equal to OEM comments from a robot file.

## Developer's quick start

Current stack: Rust + Tauri v2 + React 18 + TypeScript/Vite; Node.js 22 in CI, Windows MSVC+ WebView2 for Windows build. `package-lock.json` и `src-tauri/Cargo.lock` Tracked in Git.

```bash
npm ci --no-audit --no-fund
npm run tauri dev
```

Full local check (if Rust/Tauri prerequisites are installed):

```bash
npm run test:security
npm run test:native
npm run test:diagnostics
npm run test:oem-reference
npm run test:graph
npm run test:advanced
npm run test:visual
npm run test:inline
npm run docs:check
npm run build
cargo test --manifest-path src-tauri/Cargo.toml --locked
```

GitHub Actions: [MLAD validation](.github/workflows/validation.yml) permanent [Windows Acceptance & Security Regression](.github/workflows/windows-security-acceptance.yml). [Manual protocol WIN-01...WIN-12](docs/windows_acceptance_pr28.en.md) It's not closed yet. Windows rc.9 tested [Workflow release](.github/workflows/windows-oem-reference-rc9.yml)after PR #29 Only a temporary unsigned non-release EXE is published. CI **not** means passing a user GUI/WebView2 Smoke or OEM equivalent.

## Get started.

1. In the main tab "Robot Program" select family DX100/DX200/YRC1000, open the ** copy ** `CIOPRG.LST` through the "Review" button and check SHA-256.
2. View the page-by-page source, supported Ladder chains, blocks, and diagnostics. For `MOV/ADD/SUB/SHL/SHR` OEM help is available - ** without calculating the states of real I/O**.
3. Open separately if necessary `IONAME.DAT`. Correctness of names to addresses is not assumed; use only manual local anchorages.
4. For editable logic, use a **separate JSON draft** Graph/Advanced, not the original robot program.

## Roadmap, evidence and limitations

- [Current status](docs/project_status.en.md), [roadmap](docs/roadmap.en.md), [plan](docs/implementation_plan.en.md), [architecture](docs/architecture.en.md).
- [Inspection and issuance criteria](docs/verification_gates.en.md), [register of decisions D1-D9](docs/engineering_decisions.en.md).
- [Native syntax](docs/cioprg_lst_spec.en.md), [typed](docs/typed_opcode_grammar.en.md), [five OEMs](docs/oem_5_opcode_analysis.en.md), [semantic verification matrix](docs/semantic_verification_matrix.en.md).
- [Body of original files](docs/controller_fixture_intake.en.md), [bench-test](docs/advanced_hardware_test_matrix.en.md).
- [Directory of all Markdown](docs/README.en.md), [PR-issue](docs/development_workflow.en.md).

## Community, Error Reports and Security

- [Open an error, offer a function or check compatibility](https://github.com/uzbatsu-beep/MLAD/issues/new/choose) Structured forms with **synthetic* reproducible examples.
- [SECURITY.md](SECURITY.en.md) **Non-public* reporting of vulnerabilities; do not publish exploits/closed hardware data.
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.en.md) Rules of professional interaction.
- [CONTRIBUTING.md](CONTRIBUTING.en.md) и [PR](.github/PULL_REQUEST_TEMPLATE.en.md) - tests, sources and reviews.
- [ACCESSIBILITY.md](ACCESSIBILITY.en.md) UI availability and known limitations, **without a WCAG declaration of conformity.

**GitHub Community Standards - organizational documentation, not a security certificate.** Public Issues and PR are not intended for the original CIOPRG.LST, IONAME.DATNetwork settings, closed logs and robot programs.

## License.

**GNU GPL-3.0-only** — [LICENSE](LICENSE). Copyright holder of own original materials MLAD: **Fyodor Sergeyevich Kochedykov** (individual), © 2026. Ownership of other people's libraries, Yaskawa materials and user data does not pass to MLAD; third-party notices and chain-of-title still require separate verification. More details: [docs/licensing.md](docs/licensing.en.md).

MLAD is an independent project, not developed or approved by Yaskawa.


## Further development

** Next cycle:** [PR #30 - documentation](docs/implementation_plan.en.md)Synthetic parser fixtures, Native Workspace enhancement, versioned sidecar `*.mlad.json`Typed syntax, source-linked static analysis, Unified Offline IDE and conditional Windows 0.6.x preview. Separate PR with tests and **no automatic merge**. [Roadmap v3](docs/roadmap.en.md) · [backlog](docs/implementation_plan.en.md) · [engineering decisions](docs/engineering_decisions.en.md).
