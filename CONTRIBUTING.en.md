<!-- Generated en companion for CONTRIBUTING.md; review required. -->
> **English** · [Русский](CONTRIBUTING.ru.md) · [English](CONTRIBUTING.en.md) · [Original / Оригинал](CONTRIBUTING.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.
> **MLAD safety invariants:** `CIOPRG.LST` and `IONAME.DAT` are strictly **read-only**. FTP/HSES, live I/O, native writer and robot deployment remain **BLOCKED**. Passing `WindowsCI` does **not** imply `UserSmoke=PASS` or `OEMBenchVerified`: both remain **NOT RUN** unless independently documented. Machine translation requires source and Yaskawa manual review.

# Participation in the development of MLAD

> **State of 18.11.2026:** PR #1–#29 merged в `main` @ `f85ddfcce3063dd087425be981a9d2b7a16d6309`; [Linux CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454) и [Windows CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621) SUCCESS. The latest public Windows release is rc.9 (up to #24-#29). WebView2 UserSmoke/OEMBench — `NOT RUN`. [Project status](docs/project_status.en.md).

Working with MLAD source code requires you to separate **Yaskawa-described instructions ** and **unverified native file/protocol details **. Any PR must maintain a record ban on the controller without an independent authorized workflow.

## Before we start.

1. Read [README](README.en.md), [architecture](docs/architecture.en.md) и [Yaskawa source register](docs/source_registry.en.md).
2. Confirm current `main`The latest CI and Open PR. Do not consider current status from old correspondence or release note.
3. Select one PR goal; prepare code, tests, and update the relevant document simultaneously. For planning: [implementation_plan.md](docs/implementation_plan.en.md)for evidence: [verification_gates.md](docs/verification_gates.en.md).
4. Do not merge without direct approval of the project owner.

## PR rules

- Control-dependent logic necessarily contains an explicitly selected `ControllerFamily` differentiation `Candidate` vs. actual rights of record.
- True registers/addresses and values are modeled by types. `TMR` и `CNT` have the condition of `CNT` individual C and R, `GSTR/GOUT` - Pair byte.
- **Don't* add native `CIOPRG.LST` writer, connection to a robot or team FTP/HSES based on a synthetic test.
- The original file must not change bytes, encoding, headers, or partitions, even if the UI cannot interpret them.
- Any new addressing rule should be specified with a revision of Yaskawa instructions and positive/negative tests.
- For UI to work with three states `TRUE/FALSE/UNKNOWN`If the data is not received from the robot through a verified protocol, write **OFFLINE**.
- Commit Markdown, test cases, description of restrictions; do not commit client files, passwords, IP addresses from production networks or secrets.

## License of deposits

MLAD source code is offered on terms [GNU GPL-3.0-only](LICENSE). The program changes included in the MLAD must allow distribution under these conditions; the author of the contributions must have the necessary rights to publish them. Do not import code, diagrams, company documentation and `CIOPRG.LST` third parties without permission. Original dependency licenses should be accounted for separately; the GPL should not be considered an automatic change to the license of third-party libraries. See. [docs/licensing.md](docs/licensing.en.md).

## Community Standards and Safe Submission

- Read it. [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.en.md), [SECURITY.md](SECURITY.en.md) и [ACCESSIBILITY.md](ACCESSIBILITY.en.md).
- New Errors, Function Requests and Compatibility DX100/DX200/YRC1000 arrange through [structured issues](https://github.com/uzbatsu-beep/MLAD/issues/new/choose) **Only with synthetic examples. If a vulnerability is detected, **don't* disclose details publicly, follow SECURITY.md.
- Use it for PR. [checkout](.github/PULL_REQUEST_TEMPLATE.en.md)Security boundaries, level of evidence, negative tests, unverified scenarios, and explicit merger authorization.
- The repository is open: you can not attach real CIOPRG.LST, IONAME.DATClosed circuits, production IPs or secrets.

## Local inspection

```bash
npm ci --no-audit --no-fund
npm run test:community
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

`npm run tauri dev` Windows portable preview is a separate artifact and is not required for each Markdown PR.

## Reception.

In PR, you must specify: the initial SHA `main`the affected controllers, references to Yaskawa or `UNKNOWN`Automated tests, manual checks and what exactly was not checked.

Matrix of hardware tests: [advanced_hardware_test_matrix.md](docs/advanced_hardware_test_matrix.en.md).  
Process: [development_workflow.md](docs/development_workflow.en.md).  
Primary files: [controller_fixture_intake.md](docs/controller_fixture_intake.en.md).

## Applicability of tests and PR status

For each PR, indicate the actual CI, SHA and `NOT RUN`It is not intended to run Windows UserSmoke or OEM Bench. Statuses `ManualDerived`, `SyntheticTested`, `WindowsCI`, `OEMBenchVerified` и `Authorized` independent; [matrix](docs/verification_gates.en.md). Adopted limitations D1–D9: [engineering_decisions.md](docs/engineering_decisions.en.md). Modifying these decisions and publishing the release require separate consent from the owner.

## Post-development PR #29

[Roadmap v3](docs/roadmap.en.md) и [backlog PR #30–#37](docs/implementation_plan.en.md) Synthetic parser fixtures, Native Workspace navigation, portable annotations/sidecar, typed grammar and static analysis. Each PR must specify physical source span, level of evidence, negative cases, not modify `CIOPRG.LST`/`IONAME.DAT` Do not add network/native writer permissions. When working with JSON, consider [File ID/hardlink/staging guards](docs/filesystem_security_pr29.en.md)The TOCTOU/ACL/backup transaction is closed. For Rust/Tauri changes, Windows workflow is required; for docs-only PR, a basic CI is sufficient. The owner separately approves merge and release.

## Current privacy and governance addendum (21 November 2026)

The current Windows pre-release is `v0.5.0-rc.10-unified-offline`, not rc.9. Any earlier release/baseline statements above are scoped to their stated dates. `UserSmoke=NOT RUN` and `OEMBench=NOT RUN` remain independently unverified. Consult the [current repository audit](docs/repository_audit_2026-11-21.en.md).

Treat local `*.mlad.json` sidecars and `*.mladproj.json` manifests as **potentially sensitive**, even without raw `CIOPRG.LST` or `IONAME.DAT` bytes: comments, relative paths and SHA-256 inventories can reveal plant details. The repository's `.gitignore` is an accidental-addition guard, not a leak prevention policy for previously tracked files. `HSES`/`FTP`, native writer and robot deployment stay `BLOCKED`. GitHub `main` branch protection currently requires `frontend-build` but not the aggregate `ci-gate`; only the repository owner may approve a stricter Ruleset.
