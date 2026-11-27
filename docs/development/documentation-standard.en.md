<!-- machine-generated draft; editorial review required. -->
> **English** · [Русский](documentation-standard.ru.md) · [English](documentation-standard.en.md) · [Original / Оригинал](documentation-standard.md)

# Documentation standard and Engineering Excellence

Approved direction 27 November 2026: current purpose-oriented documents, informative README and continuous quality stream. This document defines proposed implementation rules, not completion of all tasks.

## 1. Information architecture

| Category | Target path | Purpose |
| --- | --- | --- |
| User Guide | docs/user-guide/ | Installation, workflows and limits |
| Engineering | docs/engineering/ | Formats, profiles, instructions |
| Architecture | docs/architecture/ | Layers and contracts |
| Development | docs/development/ | Build and standards |
| Testing | docs/testing/ | CI, manual protocols and evidence |
| Decisions | docs/decisions/ | ADRs and accepted decisions |
| Roadmap | docs/roadmap/ | Stable task IDs, plans |
| Releases | docs/releases/ | Immutable release notes |
| Audits | docs/audits/ | Dated reports |
| Reference | docs/reference/ | OEM registry and sources |
| Archive | docs/archive/ | Historical materials |

First new [docs/README](../README.en.md), then small thematic migration PRs. Preserve existing paths until inbound links, companions and tests are checked; leave a clear redirect/index or immutable commit link when moving. Do not move published release notes retroactively. Place new documents in target categories.

## 2. Document types and currency

| Type | Requirement |
| --- | --- |
| README | Purpose, download, available features, quick start, roadmap, limits, links |
| Live status | Verified date, SHA, evidence, distinguish main/release/planned |
| Specification | Scope, terms, model, examples, unknown, sources and tests |
| ADR | ID, status, context, decision, alternatives, consequences, supersedes |
| Audit | Date, scope, baseline, evidence, findings, actions, limitations |
| Release note | Tag/SHA, artifacts/hashes, changes, acceptance, known limits |

Do not stack new current banners on obsolete status. Rewrite live documents, retain old versions through Git history. History is not current procedure. Synthetic examples; actual build screenshots with version, clearly labeled concepts.

## 3. RU/EN and automated checks

Original authoritative; RU source-aligned, EN companion with explicit status. Preserve commands, links, identifiers and negative statements. Check local links, heading parity, code fences, review inventory. Register new sources in [translation coverage](../translation_coverage.md); editorial status is not OEM verification. Docs-only CI is not runtime PASS.

## 4. Engineering Excellence

| ID | Stream | Acceptance |
| --- | --- | --- |
| ENG-01 | Git governance | Required ci-gate, verified branch cleanup, no release ref deletion |
| ENG-02 | Docs architecture | Current README, catalog, live status, migrations with QA |
| ENG-03 | Developer quality | Unified preflight, baseline lint/format, advisories/license audit |
| ENG-04 | Architectural health | Decomposition and profile/snapshot identity, no second parser |
| ENG-05 | UI quality | Behavioral tests, accessibility, Windows UserSmoke |
| ENG-06 | Performance | Measured baseline 10k/50k/100k lines, cost profiling |
| ENG-07 | Release engineering | Versions, SHA, SBOM, source, separate approval |
| ENG-08 | Continuous audit | Regular reviews, evidence and actions |

Every PR: scope, tests, docs, final SHA, limits. Weekly — Git/CI/backlog, monthly — dependencies/architecture/docs, before Preview — UX/performance, before release — distribution/licenses. Bind tasks to Roadmap V5 and audit; do not halt functional development. No automatic merges.

## 5. Boundaries

`CIOPRG.LST` and `IONAME.DAT` read-only. HSES/FTP, native writer, live I/O, deployment BLOCKED. UserSmoke=NOT RUN; OEMBench=NOT RUN until independent results. New documentation does not expand application permissions.
