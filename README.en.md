<!-- MLAD-I18N:EDITORIAL:reviewed 2026-11-19 against README.md -->
> **Language / Язык:** [English](README.en.md) · [Русский](README.ru.md) · [Source / Оригинал](README.md)
>
> **Scope:** This English version explains the project's repository baseline. The original and manufacturer documentation take precedence where there is any discrepancy. Controller compatibility, scan semantics, and authorization are **not** established by translation.

> **Roadmap v5 (26 November 2026, owner-approved):** strategy C, M0–M21, Semantic Core v2, specialized workspace layouts, Signal Intelligence, Compare, Virtual Lab, Trace & Test, Documentation, Fleet and AI. [Current roadmap](docs/roadmap.en.md) · [V5-01…15 plan](docs/implementation_plan.en.md) · [Decisions D10–D15](docs/engineering_decisions.en.md). This is a plan, not a claim that new features are implemented. The v3/v4 snapshots below are historical.

> **Current audit (21 November 2026):** verified `main` `9fcd11d61856c8a559d9acf3c77bd662ad814dfe`, published Windows pre-release `v0.5.0-rc.10-unified-offline`. `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`; `HSES`/`FTP`, native writer and deployment `BLOCKED`. [Current Git audit](docs/repository_audit_2026-11-21.en.md).
> **Historical context:** old release/PR references below are dated snapshots, not a claim that rc.9 or PR #29 is latest.

# MLAD — Motoman Ladder Analysis & Draft IDE

**Historical repository snapshot (18 November 2026):** [main](https://github.com/uzbatsu-beep/MLAD) at `f85ddfcce3063dd087425be981a9d2b7a16d6309`; pull requests **#1–#29 merged**. The corresponding [Linux CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454) and [Windows CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621) succeeded. The most recent **published** Windows release is **rc.9**; it **does not contain PR #24–#29**. See the [v3 roadmap](docs/roadmap.en.md) and [project status](docs/project_status.en.md). Independent Windows/WebView2 GUI acceptance and OEM controller bench testing are **NOT RUN**.

**Windows release as of 18 November 2026 (historical):** [v0.5.0-rc.9-oem-reference](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference) (portable ZIP, NSIS installer, GPL source archive, SHA-256). Its own [main validation](https://github.com/uzbatsu-beep/MLAD/actions/runs/44166354563) and [RC9 Windows build](https://github.com/uzbatsu-beep/MLAD/actions/runs/44162078155) succeeded. **Do not confuse a successful current-branch CI artifact with a new published release.**

> **Safety boundary:** MLAD is neither an OEM-certified Yaskawa engineering tool nor a verified native `CIOPRG.LST` compiler. Original `CIOPRG.LST` and `IONAME.DAT` files are strictly **read-only**. Native writing, FTP/HSES, robot deployment and live telemetry are disabled. For actual equipment, use OEM-approved tools and a separately authorized safety procedure.

## Implemented functionality

| Feature | Verified implementation and limitation |
| --- | --- |
| DX100 / DX200 / YRC1000 profiles | Typed address syntax and range validation; exact firmware-specific memory maps and ownership remain subject to verification |
| Native Workspace | Main application workspace with a compact header, native system file-open dialog, navigation tree, paginated source viewer, search and diagnostics |
| `CIOPRG.LST` inspection | Exact byte preservation, SHA-256, post-`END` sections treated as data, and explicit supported/unknown classification |
| Ladder | **All recognized rungs returned by the bounded SemanticPreview**; zoom and source references. Unknown material is not fabricated into a diagram |
| TMR/CNT/GSTR/GOUT | Read-only inspection of recognized native patterns, plus separate editable offline `*.advanced.json` drafts |
| `IONAME.DAT` and annotations | Read-only preview, local rung/address notes and manually created associations. Native files remain unchanged |
| `MOV`, `ADD`, `SUB`, `SHL`, `SHR` | Typed operands and exact source spans, with **tentative static M-register references** rather than demonstrated runtime dataflow |
| OEM-derived reference material | Documented EN/16-bit/Carry/Zero/Error rules and isolated reference tests; **not execution of a source program on the controller** |
| JSON Graph Draft | SVG ladder editor, AND/OR/NO/NC/OUT, Undo/Redo, double-click/F2 address editing, manually supplied offline TRUE/FALSE/UNKNOWN states |
| Legacy JSON | Compatibility with the earlier `project.json` format through a separate menu item |

**JSON persistence and security (merged PR #24–#29):** staged file writes and `*.bak` backups; bounded reads; conflict detection comparing **both original bytes and File ID**; rejection of multiply hard-linked JSON targets/backups; staged-file SHA-256 verification; rejection of Windows ADS/UNC/device-style paths; and production CSP. These mechanisms **do not** guarantee an atomic backup-and-target transaction, complete TOCTOU protection, ACL preservation or crash recovery. See the [filesystem security review](docs/filesystem_security_pr29.en.md) and [verification gates](docs/verification_gates.en.md). Manual Windows UserSmoke remains **NOT RUN**.

**Format separation:** `CIOPRG.LST` and `IONAME.DAT` are immutable inputs, whereas `*.graph.json` and `*.advanced.json` are separate editable drafts. Never save JSON under the guise of a native CIOPRG export. Native Workspace annotations are stored in localStorage; they are **not** OEM annotations embedded in a robot file.

## Developer quick start

Stack: Rust, Tauri v2, React 18, TypeScript and Vite. CI uses Node.js 22; the Windows application requires the Windows MSVC toolchain and WebView2. Both `package-lock.json` and `src-tauri/Cargo.lock` are committed.

```bash
npm ci --no-audit --no-fund
npm run tauri dev
```

Full local validation, with Rust/Tauri prerequisites installed:

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

GitHub Actions includes [MLAD validation](.github/workflows/validation.yml) and the permanent [Windows Acceptance & Security Regression workflow](.github/workflows/windows-security-acceptance.yml). The [WIN-01–WIN-12 manual Windows acceptance protocol](docs/windows_acceptance_pr28.en.md) has **not** been independently completed. Release rc.9 used its own [historical Windows workflow](.github/workflow-archive/windows-oem-reference-rc9.yml). A passing CI job is **not** equivalent to a GUI test on the end user's Windows installation or to verified OEM behavior.

## First steps

1. In the main **Robot Program** tab, select DX100, DX200 or YRC1000. Using **Browse…**, open a **copy** of `CIOPRG.LST` and record its SHA-256.
2. Inspect the paginated source, supported Ladder rungs, blocks and diagnostics. The `MOV/ADD/SUB/SHL/SHR` reference material does **not** calculate real I/O states.
3. Optionally open a separate copy of `IONAME.DAT`. Do not assume that the name-to-address mapping is OEM-verified; use only explicit local manual associations.
4. To edit a logic model, work in a **separate Graph/Advanced JSON draft**, never in the controller's native program file.

## Roadmap, evidence and limitations

- [Current project status](docs/project_status.en.md), [roadmap v3](docs/roadmap.en.md), [proposed PR backlog](docs/implementation_plan.en.md), [architecture](docs/architecture.en.md).
- [Verification/release gates](docs/verification_gates.en.md), [approved engineering decisions D1–D9](docs/engineering_decisions.en.md).
- [Native syntax evidence](docs/cioprg_lst_spec.en.md), [typed instruction forms](docs/typed_opcode_grammar.en.md), [five OEM reference instructions](docs/oem_5_opcode_analysis.en.md), [semantic verification matrix](docs/semantic_verification_matrix.en.md).
- [Private fixture intake procedure](docs/controller_fixture_intake.en.md), [hardware validation matrix](docs/advanced_hardware_test_matrix.en.md).
- [Documentation index](docs/README.en.md), [development and release workflow](docs/development_workflow.en.md).

## Community, bug reports and security

- [Open an issue, feature request or compatibility report](https://github.com/uzbatsu-beep/MLAD/issues/new/choose) using **synthetic**, reproducible examples.
- [SECURITY](SECURITY.en.md): report sensitive vulnerabilities privately. Never publish exploits or production robot files in public issues.
- [CODE_OF_CONDUCT](CODE_OF_CONDUCT.en.md): professional community conduct.
- [CONTRIBUTING](CONTRIBUTING.en.md) and the [pull-request template](.github/PULL_REQUEST_TEMPLATE.en.md): required evidence, tests and review.
- [ACCESSIBILITY](ACCESSIBILITY.en.md): current limitations; **no WCAG conformance claim**.

Community documentation is **not** a security certification. Public issues and pull requests are not suitable for actual `CIOPRG.LST` or `IONAME.DAT` data, network settings, confidential logs or robot programs.

## License

**GNU GPL-3.0-only** — [LICENSE](LICENSE). Copyright in MLAD's original materials belongs to **Фёдор Сергеевич Кочедыков** (an individual), © 2026. The license does **not** transfer ownership of third-party libraries, Yaskawa manuals or users' industrial data. Third-party notices and chain-of-title review are still required. See the [licensing policy](docs/licensing.en.md).

MLAD is independent and **is not developed or endorsed by Yaskawa**.

## Next development cycle

The proposed order after the documentation baseline is: synthetic CIOPRG parser fixtures, improved Native Workspace navigation, versioned `*.mlad.json` sidecar, evidence-based typed grammar, source-linked static analysis, a unified offline engineering IDE and a conditional Windows 0.6.x preview. Each requires independent tests, review and owner approval for merge. See the [v3 roadmap](docs/roadmap.en.md) and [implementation plan](docs/implementation_plan.en.md).
