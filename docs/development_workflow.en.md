<!-- Generated en companion for docs/development_workflow.md; review required. -->
> **English** · [Русский](development_workflow.ru.md) · [English](development_workflow.en.md) · [Original / Оригинал](development_workflow.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.
> **MLAD safety invariants:** `CIOPRG.LST` and `IONAME.DAT` are strictly **read-only**. FTP/HSES, live I/O, native writer and robot deployment remain **BLOCKED**. Passing `WindowsCI` does **not** imply `UserSmoke=PASS` or `OEMBenchVerified`: both remain **NOT RUN** unless independently documented. Machine translation requires source and Yaskawa manual review.

# MLAD development, testing and release process

**Update:** 18.11.2026; `main` @ `f85ddfcce3063dd087425be981a9d2b7a16d6309`, PR #1–#29 merged, [Linux CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454) и [Windows CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621). Last published rc.9 - before PR #24-#29. New plan. [roadmap v3](roadmap.en.md); docs PR #30 It doesn't extend runtime.

## 1.Git and function modification

1. Basis - verified current `main`Each stage of development and documentation goes in a separate branch** and PR.
2. Do not merge PR without an explicit decision of the project owner; a work branch with CI errors is not considered accepted.
3. Divide by scope: `compiler/` Rust, `src/ladder/` clean frontend models, `src/components/` visualization, `docs/` evidence/contracts.
4. The new operator requires test matrix: controller family, operands, ranges, memory class rights, real vendor evidence, negative tests, failed-closed behavior.
5. Before changing the architecture of the controller, agree on System/User boundaries; enable hardware and native writer – **separate solutions and separate PR**.
6. Prohibit regression: initial `CIOPRG.LST` does not change from viewing; editing JSON never leads to FTP/HSES.

7. For public Issue, use the Bug Report, Feature Request, Controller Compatibility forms. [ISSUE_TEMPLATE](../.github/ISSUE_TEMPLATE)Real robot programs and secrets are prohibited. For vulnerabilities, [SECURITY.md](../SECURITY.en.md) A private channel, if available. For PR -- [PULL_REQUEST_TEMPLATE.md](../.github/PULL_REQUEST_TEMPLATE.en.md).

## 2. Local inspection teams

Recommended CI environment: Node.js 22, npm 10 (lockfile v3), stable Rust, Tauri v2 prerequisites, Windows MSVC for Windows release. Before designing, perform `npm ci`: it uses tracked in Git `package-lock.json`Update dependencies – only conscious change n `package.json` along with the lockfile.

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

For local GUI: `npm run tauri dev`. Check manually save/boot JSON, window size and double click on SVG, UNKNOWN status, ban HSES/FTP and the immutability of the original LST.

Documentation check `npm run docs:check` separately checks relative links and historical escaped newline substitutions; ** it does not verify the actual availability of external links or their reliability**.

`MLAD validation` run-in `main` post-push `main`: `npm ci --no-audit --no-fund`, checks the immutability of lockfile after installation, performs Rust tests (Ubuntu), frontend tests, type check and Vite build. Windows/MSVC runs are performed **individual** workflow, not every PR. The full list of tests depends on the specific workflow – check YAML and commit SHA.

## 3. Quality gates

| Gate | Demand |
|---|---|
| Q0 / Provenance | Main branch and SHA PR recorded |
| Q1 / Build | TypeScript, Vite, Rust Unit Tests Successful |
| Q2 / Semantics | New instructions have Yaskawa sources, border tests and diagnostics toward |
| Q3 / Safety | There is no circumvention of the CIOPRG write ban, FTP/HSESJSON formats are isolated |
| Q4 / Regression | Wiring AND/OR, NC, TMR/CNT C/R, GSTR/GOUT, Undo/Redo checked |
| Q5 / Windows | For Windows build: MSVC/Tauri build, runtime WebView2 and hand-held UI smoke test |
| Q6 / Hardware | **Separate** test program and permit; CI does not close this gate |

If you deviate from Q2/Q3/Q6, you cannot declare the past CI "ready to load on the robot."

## 4. Real status of release workflow on 18.11.2026

- [`validation.yml`](../.github/workflows/validation.yml) Ubuntu pipeline: Rust tests, npm ci with lockfile, security, Native Workspace, diagnostics, OEM help, graphics, TypeScript/Vite and docs-check. Last one. `main` [SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44166354563).
- [`windows-oem-reference-rc9.yml`](../.github/workflow-archive/windows-oem-reference-rc9.yml) Windows x64 NSIS/portable/source/SHA256 pipeline. [RC9 SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44162078155), [release](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference).
- Historic Windows Workflows `windows-bench-rc.yml`, `windows-ui-preview.yml` и `windows-*-rc[2-8].yml` Linked to the original branch/tag and **not* automatically reassemble older versions. A new RC requires a separate tag/branch and a ban on overwriting an existing release.
- `package-lock.json` (v3) и `Cargo.lock` tracked in Git; npm 22/CI `npm ci`; Rust uses `cargo test --locked`. This is not Windows EXE bit reproducibility or hardware verification.
- For release: check Windows MSVC, portable ZIP, NSIS, source archive, SHA256SUMS, GPL/third-party notices, WebView2 And a separate manual Windows smoke. **No GitHub workflow closes the OEM bench gate.**

## 5. Versioning of documentation

- fix `as of` date, commit/PR upon change of status.
- The historical release note does not "rewrite" under the new version; add an external reference to the new version. `main` и changelog.
- Documentation defines features only together with code and tests; in case of conflict, check the original Rust/TypeScript.
- Vendor sources: fix the family and accurate revision of the manual, do not infer from the example of complete grammar and record rights.

See. [CONTRIBUTING.md](../CONTRIBUTING.en.md) и [fixture](controller_fixture_intake.en.md).


## 6. Verification of the safe preservation of JSONPR #24 merged)

New staged writer tested by Rust module `draft_io.rs`, а `test:security` It also prohibits direct recording of JSON from Tauri handlers. Receiving: new/existing draft, previous draft `*.bak`, oversized input, direct symlink, creation/replacement error, and original immutability check `CIOPRG.LST`/`IONAME.DAT`. Rust CI on Linux **doesn't prove* identical replacement behavior on Windows; Windows Smoke is required before release WebView2 and an open JSON file. Current threats to CSP, parent symlink/races, and backup transactions remain outside the scope of this PR.


## 7. PR #25 CSP and path protection (merged)

Before release, check the CSP syntax through `npm run test:security`, `cargo test --locked`, build Windows Tauri, launch WebView2, check system Open dialog, visual Ladder/TMR panels, local editing JSON and no blocked Tauri IPC. Normal symbolic ancestral references are deliberately rejected to reduce risk: the user must choose a direct path to a regular copy of the file. Tests do not eliminate filesystem TOCTOU; a separate review directory-handle/Windows reparse remains.

## 8. PR #26 — guarded reads / JSON conflict guard (merged)

Identity/reparse checks limit substitution errors between the path and the open descriptor; the JSON writer fails when a competitive change is detected. Automatic Linux/Windows CI passed, but does not eliminate the final window `check→rename`Reparse/ACL/power-loss edge cases and are not manual Windows acceptances. [Guarded I/O](guarded_io_review.en.md).

## 9. Unified checklist of new PR

Planning to follow [implementation_plan.md](implementation_plan.en.md)After each functional adjustment, fill in [verification_gates.md](verification_gates.en.md)The agreed prohibitions to check with [engineering_decisions.md](engineering_decisions.en.md). The CI results are tied to the **exact head SHA**. For Windows/hardware Documentary PR - `NOT RUN (docs-only)`Functional Windows PR requires Tauri runner tests, and manual smoke before user release. No stage has lifted the ban on HSES/FTP, native writer or deployment.

## 10. Main Branch Protection – Organizational Gate

At the time of inspection on 18.11.2026 GitHub reports `main.protected=false`. Therefore, the “do not merge without consent” rule is still a process rather than a guaranteed repository barrier. The owner can configure GitHub Ruleset / Branch Protection separately. `frontend-build` и `rust-tests`Relevance of checks on head SHA, prohibition of direct push `main` for conventional development, the requirement to review and disable bypass/auto-merge as far as the tariff and roles allow. **Settings are not changed by this PR.** When enabling the rules, verify the exact names of checks and exclude false blocking of service release workflow.

## 11. PR #28 Repeated Windows regression tests and UserSmoke

[New workflow](../.github/workflows/windows-security-acceptance.yml) Performs automatic Node/Rust Windows tests, Tauri `--no-bundle` **Temporary unsigned non-release** artifact SHA-256 и evidence. [PowerShell harness](../scripts/windows-acceptance.ps1) Prepare/check only synthetic `CIOPRG.LST`/`IONAME.DAT`. Separate [Windows manual acceptance protocol](windows_acceptance_pr28.en.md) WIN-01...WIN-12; GitHub Actions does not replace manual testing WebView2. New Windows negative tests don’t extend Tauri resolutions and don’t include native writingHSES/FTP. [Verification gates](verification_gates.en.md).

## 12. PR #29 Filesystem Security: New Regression Gates

Rust JSON reader/writer uses the ** and ** identity of the open descriptor when writing, as well as a separate hardlink check and checksum staged file. List of limitations and conflict scenarios: [filesystem_security_pr29.md](filesystem_security_pr29.en.md). Windows regression workflow requires listing/executing new tests on real Windows NTFS, then unsigned non-release Tauri EXE. **PR #29 merged** в `main`; [main Windows CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621). This is not a new published release or custom smoke. Directory-handle/TOCTOU/ACL/backup transaction remain separate security-review topics.

## 13. Existing scheme PR #30–#37

1. [Documentation baseline](implementation_plan.en.md) (docs-only), then synthetic native parser fixtures and Native Workspace UX in separate PRs; parallel private OEM intake is not published.
2. Before entering new editable formats (sidecar) check the path/ACL/TOCTOU from [PR #29 threat review](filesystem_security_pr29.en.md)Don’t just rely on unit tests.
3. Every technical PR requires security + functional Rust/Node tests. `npm ci`, `npm run docs:check`Linux CI and separate Windows CI when changing filesystem/Tauri; only accurate PR HEAD SHA is considered verified.
4. Specified in the UI `OFFLINE` does not replace the OEM scan evidence and does not grant network permission; HSES, native writer, deployment blocked.
5. 0.6.x release is possible **only after **self [Windows UserSmoke](windows_acceptance_pr28.en.md)legal/source assets and owner approval. CI is not a substitute for UserSmoke.
6. `main` At the time of the last check, GitHub was returned. `protected=false`A separate inclusion of Ruleset/Branch Protection is recommended. Prior to its implementation, adhere to mandatory manual review and prohibition of merge without consent.

**Reconciliation on 19 November 2026, after merged PR #37 (earlier sections retain their historical dates):** the `main` branch is protected by an active GitHub Ruleset `MLAD — main branch protection`, and automatic deletion of merged head branches is enabled. PR #37 introduced an always-present `ci-gate`: documentation-only changes must pass Markdown, security and model tests while heavy Rust/Windows jobs are expected to be skipped; application changes require successful `frontend-build`, `rust-tests` and Windows NTFS/Tauri checks via `workflow_call`. Failures, cancellations and unexpected skips must make `ci-gate` fail closed. At this reconciliation the Ruleset **still requires `frontend-build`, not `ci-gate`**; the owner must switch the required context separately after both scenarios are verified. GitHub Actions is **not** independent manual `UserSmoke=NOT RUN` or `OEMBench=NOT RUN` evidence. Both historical branch-pruning dispatches used `dry-run`; actual deletion requires a separate explicitly confirmed `delete` run. Historical Windows release and translation workflows should be archived only after checking release reproducibility and documentation links.
