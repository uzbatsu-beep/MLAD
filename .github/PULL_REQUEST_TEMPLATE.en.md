<!-- Generated en companion for .github/PULL_REQUEST_TEMPLATE.md; review required. -->
> **English** · [Русский](PULL_REQUEST_TEMPLATE.ru.md) · [English](PULL_REQUEST_TEMPLATE.en.md) · [Original / Оригинал](PULL_REQUEST_TEMPLATE.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.
> **MLAD safety invariants:** `CIOPRG.LST` and `IONAME.DAT` are strictly **read-only**. FTP/HSES, live I/O, native writer and robot deployment remain **BLOCKED**. Passing `WindowsCI` does **not** imply `UserSmoke=PASS` or `OEMBenchVerified`: both remain **NOT RUN** unless independently documented. Machine translation requires source and Yaskawa manual review.

## What changed?

<!-- Одна техническая цель PR; укажите ссылки на задачи и документы. -->
- Related Issue/roadmap stage:
- Basic commit main (SHA):
- Changes and affected subsystems:
- The families of controllersDX100/DX200/YRC1000 or “not applicable”:

## Evidence and unknown areas

- The source of the rule (guide, revision, section/page) ** or** "no OEM source/not applicable":
- What is confirmed: **Documented/SyntheticTested/ManualDerived/OEM BenchVerified**
- What **not confirmed** (EN, scan, flags, System/User, byte/word, firmware, etc.):
- Why New Limitations Aren't Masqueraded as Supported Functionality

## Checks

- [ ] `npm ci --no-audit --no-fund`Lockfile/manifest is not changed by accident.
- [ ] `npm run test:security`.
- [ ] `npm run test:native` if necessary `test:graph` / `test:advanced` / `test:diagnostics` / `test:oem-reference`.
- [ ] `npm run docs:check` The documentation/roadmap has been updated.
- [ ] `npm run build`; `cargo test --manifest-path src-tauri/Cargo.toml --locked` for changes to Rust.
- [ ] Positive and negative regression tests are added or explained as to why they are not required.
- [ ] Windows smoke/OEM bench executed ** or explicitly labeled NOT RUN** with cause.

> For documentary PR, you can specify **N/A with an explanation** next to the non-applicable items, but general CI checks remain mandatory.

## MLAD security boundaries

- [ ] Original. `CIOPRG.LST` и `IONAME.DAT` They're just for reading.
- [ ] No native writer/export, FTP/HSES, live I/O, download/control robot.
- [ ] No false translation. `TypedCandidate` or `ManualDerived` c really executable semantics.
- [ ] In PR, there are no real programs, closed I/O cards, production network addresses, passwords, and confidential logs.
- [ ] The author has the right to provide the proposed code and materials on conditions `GPL-3.0-only` (Third parties retain their rights).
- [ ] Vulnerabilities are discussed by [SECURITY.md](https://github.com/uzbatsu-beep/MLAD/blob/main/SECURITY.md)**Not through the open details in PR.

## Admission plan

- Results of CI (references):
- Manual checks (what is checked; what is not checked):
- Known risks and method of recoil:

**Merge only after separate explicit approval of the project owner.**

## Relevance of documentation and limits of decision

- [ ] Verified actual `main` SHA, Open PR, Releases and CI Statuses [project status](../docs/project_status.en.md).
- [ ] When changing functions updated [roadmap v3](../docs/roadmap.en.md), [implementation plan](../docs/implementation_plan.en.md), [verification gates](../docs/verification_gates.en.md) and profile MDs; **not** rewritten historical release notes retroactively.
- [ ] If file paths/JSON/Tauri are affected, review [PR #29 filesystem boundaries](../docs/filesystem_security_pr29.en.md) File ID/hardlink/ADS  за closing TOCTOU/ACL/backup transaction.
- [ ] Levels. `SyntheticTested`, `WindowsCI`, `UserSmoke`, `OEMBenchVerified`, `Authorized` recorded independently, missing tests honestly marked `NOT RUN`.
- [ ] Separate explicit consent of the owner to merge is obtained (not considered CI permission).

## Additional current-development checks (after PR #48)

- [ ] Confirm current `main`, GitHub Releases and exact source SHA against the [Git audit](../docs/repository_audit_2026-11-21.en.md); historical PR labels do not indicate the next open task.
- [ ] No local `*.mlad.json`, `*.mladproj.json` or their backups have been committed; `CIOPRG.LST` and `IONAME.DAT` remain read-only.
- [ ] Documentation updates include RU/EN companions and pass `docs:check`, `i18n_verify.py`, `i18n_semantic_guard.py`. A `MachineDraft` is not automatically reviewed.
- [ ] For runtime changes, confirm `frontend-build`, Rust, Windows and aggregate `ci-gate` at the exact SHA. The `main` Ruleset currently requires only `frontend-build`.
- [ ] Preserve `UserSmoke=NOT RUN` and `OEMBench=NOT RUN` without independent evidence. `HSES`/`FTP`, native writer and deployment remain `BLOCKED`.
