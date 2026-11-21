<!-- Generated English companion for docs/repository_audit_2026-11-21.md; review required. -->
> **English** · [Русский](repository_audit_2026-11-21.ru.md) · [English](repository_audit_2026-11-21.en.md) · [Original / Оригинал](repository_audit_2026-11-21.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.

# MLAD — Git and Markdown audit, 21 November 2026

**Baseline:** [main](https://github.com/uzbatsu-beep/MLAD) `9fcd11d61856c8a559d9acf3c77bd662ad814dfe`; [main CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44743083267) SUCCESS. Latest Windows [`v0.5.0-rc.10-unified-offline`](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.10-unified-offline) is published from a **distinct release-branch SHA** `2bba435f2263f2300cb3464b76f919c0da2e4b7f`; [Windows release workflow](https://github.com/uzbatsu-beep/MLAD/actions/runs/44754693972) SUCCESS. Never confuse these commits.

**Inventory method:** inspected the tree, current workflows, the `main` Ruleset, published release assets, Cargo/npm manifests and Markdown translation registry. There were 294 Git-tree entries including **164 Markdown files** (54 originals, 108 RU/EN companions, translation register and archived workflow README). Adding this source and RU/EN companions makes **167 Markdown files**; those new files require their own PR CI.

## 1. Current product configuration

| Area | Observed | Not verified |
| --- | --- | --- |
| Native source | Immutable bounded `CIOPRG.LST`, `IONAME.DAT`, SHA-256 | OEM execution or native writing |
| Workspace | Source, diagrams, diagnostics, IONAME, `READ` / `WRITE` / `UNKNOWN` source-linked roles | Live I/O, controller scan and EN semantics |
| Sidecar | Independent `*.mlad.json` v1 with notes, bookmarks, source SHA and revision | OEM-native comments or multi-file transactions |
| Unified project | `*.mladproj.json` v1 portable relative paths, source and attachments SHA, recomputed analysis | Hardware export / writer |
| Typed parser | Manual-derived YRC1000 R12 candidates | Firmware/real controller compatibility |
| Windows | Published unsigned `rc.10`, portable/NSIS/source/hash, automated build | **`UserSmoke=NOT RUN`**, **`OEMBench=NOT RUN`** |
| Safety | `FTP`, `HSES`, native writer, live robot controls, deployment **BLOCKED** | Runtime robot authorization |

The PR48 bundled sample is strictly synthetic. `CIOPRG.LST` / `IONAME.DAT` must never be uploaded from MLAD; `TypedCandidate`, `ManualDerived`, `READ`/`WRITE` operand roles cannot establish write permissions.

## 2. Findings and priorities

| Priority | Finding | Planned action |
| --- | --- | --- |
| **P0 — Governance** | `MLAD — main branch protection` has PR / anti-deletion / non-fast-forward protection, but only `frontend-build` required, not aggregate `ci-gate`; strict up-to-date policy disabled | **OPEN:** owner should review adding required `ci-gate`; document-only PR does not change rules |
| **P1 — Stale documentation** | Root/docs README and roadmap/status text still presents `rc.9`, PR #29/#39, or proposed PR #40–#48 as live | **IN PR49:** add a dated current-state layer; preserve historical PR narratives |
| **P1 — Privacy** | `.gitignore` guarded OEM `CIOPRG.LST` and `IONAME.DAT`, not private `*.mlad.json` or `*.mladproj.json` | **IN PR49:** ignore private sidecars/manifests/backups; already-tracked files require separate review |
| **P1 — Release drift** | `main` window title still said `rc.9`; merged PR48 Windows Preview workflow remained active | **IN PR49:** neutral window title and historical workflow archiving |
| **P1 — Markdown checking** | `docs:check` skipped RU/EN root and archived GitHub Markdown; fenced `~~~` content insufficiently excluded | **IN PR49:** scan all root/docs/.github markdown and both fence styles |
| **P2 — Dependencies** | Previous dated docs mention `suppaftp`, but current `src-tauri/Cargo.toml` no longer includes it | **CLOSED:** PR43 already removed it. Full CVE and dependency license audit remains separate |
| **P2 — Translations** | Every source has RU/EN companions, but a substantial portion marked `MachineDraft` | **OPEN:** human language and technical review; do not claim automatic validation |
| **P2 — Filesystems** | Staged JSON plus `*.bak`, source SHA and File ID safety; Windows DACL, TOCTOU, atomic multi-file recovery unresolved | **OPEN:** dedicated security test evidence |
| **P2 — Acceptance** | CI/Windows build succeeded, but no observed Windows GUI P48-01…P48-17 or real OEM controller test | **OPEN:** separate UserSmoke and OEMBench protocols |

## 3. Documentation ownership and historical records

- **Living documentation:** [README](../README.en.md), [index](README.en.md), [roadmap](roadmap.en.md), [plan](implementation_plan.en.md), [status](project_status.en.md), [verification](verification_gates.en.md), [architecture](architecture.en.md), [development](development_workflow.en.md), [decisions](engineering_decisions.en.md).
- **Technical documents:** CIOPRG source, typed grammar, AST, sidecar, filesystem and IONAME specs describe verified boundaries; historical absence of OEM evidence remains in effect.
- **Historical material:** dated prior audits, `roadmap_v2_archive`, `implementation_plan_v2_archive`, RC1–RC9 release notes and archived workflows are preserved as time-scoped evidence, not silently rewritten.
- **Localization:** [translation register](translation_coverage.md), `scripts/i18n_verify.py`, `scripts/i18n_semantic_guard.py`. A `MachineDraft` is **not** technically reviewed translation or an OEM safety manual.

## 4. Owner decisions

1. Review required `ci-gate` for protected `main` and strict branch currency policy. These are GitHub settings, **not** updated by this PR.
2. Carry out [P48-01…P48-17](windows_offline_preview_pr48.en.md) on an isolated Windows 10/11 + WebView2 machine. CI success is not `UserSmoke=PASS`.
3. Separate subsequent risk-oriented PRs: Windows ACL/TOCTOU and race tests; UI/UX; transitive dependency / licensing review; OEM-evidenced syntax expansion.
4. Commission human editorial review for the remaining `MachineDraft` companions, rather than automatically upgrading provenance.

## 5. References and limits

[main CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44743083267) · [Windows RC10 CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44754693972) · [Release RC10](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.10-unified-offline) · [PR48](https://github.com/uzbatsu-beep/MLAD/pull/48).

**Not performed:** full third-party CVE/OSV/license assessment, interactive Windows GUI UserSmoke, OEM bench/scan, production network test, exhaustive OS-level race and crash tests. The audit is a code/documentation review, **not** application certification. PR49 acceptance requires checks on its own exact final HEAD.
