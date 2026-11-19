<!-- MLAD-I18N:EDITORIAL:reviewed 2026-11-19 against docs/engineering_decisions.md -->
> **Language / Язык:** [English](engineering_decisions.en.md) · [Русский](engineering_decisions.ru.md) · [Original](engineering_decisions.md)
>
> **Authority:** Only decisions explicitly approved by the project owner are binding. Translation does not create a new engineering decision or authorize controller operations.

# MLAD — Register of Approved Engineering Decisions

**Register version:** 18 November 2026. This document records agreed restrictions separately from proposals that have **not yet been adopted**. References: [roadmap v3](roadmap.en.md) and the [archived roadmap v2](roadmap_v2_archive.en.md), [current project status](project_status.en.md), and [proposed PR plan](implementation_plan.en.md). **Merging each individual PR requires a separate owner decision.**

## 1. Approved decisions D1–D9

| ID | Approved decision | Consequence for development |
| --- | --- | --- |
| D1 | **Balanced development path D:** establish a short safety baseline → develop Native Workspace and navigation, collecting real fixtures in parallel → refine the parser and semantics → build a Unified IDE | This is the agreed architectural order. UX improvements need not wait for a complete OEM grammar, but must not imply unverified compatibility |
| D2 | `CIOPRG.LST` is strictly **read-only**; edited data and sidecars are maintained separately | Never overwrite or normalize original source bytes; preserve byte identity and SHA-256. User changes are stored only in independent JSON files |
| D3 | Native Workspace displays blocks in a **compact expandable list**, with the selected rung/circuit in the central work area | All recognized circuits remain accessible; source text is paginated. Avoid rendering hundreds of expensive diagrams simultaneously |
| D4 | Show unknown source as source text; **do not guess its semantics** | Distinguish `recognized`, `partially supported` and `opaque`; keep exact source spans and visible warnings |
| D5 | Genuine controller exports belong to a **private corpus**; public fixtures must be explicitly permitted or synthetic | Do not include real `CIOPRG.LST`, `IONAME.DAT`, private hashes or production IP addresses in PRs or CI |
| D6 | Syntax and behavior must be established **independently** | A `TypedCandidate` is not an executable instruction model; `ManualDerived` does not establish an OEM-verified scan |
| D7 | Deliver a fully offline read-only tool first; HSES is a separately authorized future decision | FTP/HSES runtime is disabled in `main`; there is no live telemetry or permission to write to a controller |
| D8 | Offline MVP `0.6.x`: Native Workspace, navigation, diagnostics and SHA-256 source preservation | Acceptance is based on demonstrated offline functionality and smoke testing, **not** on the existence of an OEM-native writer |
| D9 | Prioritize reproducible npm/CI and security, then workspace/parser/unified-IDE improvements | Use small independently tested PRs with negative regressions and **no automatic merging** |

**License:** `GPL-3.0-only`. Copyright in MLAD's **own original materials** is held by **Фёдор Сергеевич Кочедыков** (an individual), © 2026. This does **not** claim ownership of third-party or OEM materials. See [licensing](licensing.en.md).

## 2. Restrictions that must not change without a separate decision

- Native export or writing of `CIOPRG.LST` is **prohibited**. An editable JSON draft must never masquerade as an OEM program.
- HSES/FTP, robot upload, live I/O and modification of System/User Ladder are **prohibited**.
- The `PART 1/2` markers must **not** be treated as a verified System/User access-control policy.
- Syntax, program behavior and any controller-specific effect must be verified separately for each controller family and firmware.
- Developing an offline interface **does not imply permission to connect to a real robot**.
- All PRs require tests; **merge requires the owner's explicit consent**.
- Historical release notes and published artifacts must not be silently rewritten to reflect later code.

## 3. Open questions — not approved decisions

| Question | Evidence needed | Current status |
| --- | --- | --- |
| `*.mlad.json` sidecar format, schema version and migrations | Proposed schema, prototype, negative tests and SHA-based source binding | Proposal in the [implementation plan](implementation_plan.en.md); **not implemented** |
| Verification and correction of native grammar for DX100/DX200/YRC1000 | Rights to use controller exports, firmware metadata and relevant OEM manuals | M1/M5 investigation only; **no native writer permission** |
| Formal cross-reference and static-dataflow rules | Exact source spans, treatment of ambiguity, explicit evidence levels | A controller scan model has **not** been verified |
| Windows filesystem trust: handles, ACLs and races | Threat model and reproducible Windows tests | M2 **PARTIAL**; PR #24–#29 reduced some risks, but final check→rename, ACL and recovery remain open |
| Release `0.6.x` | Windows UserSmoke, licensing/source/assets and explicit approval | Candidate only, not announced; conditionally proposed as PR #37 |
| Read-only HSES | Separate owner approval, verified OEM read specification and isolated bench | **BLOCKED** |
| Native writer and deployment | OEM-verified file fidelity, independent safety review and explicit approval | **BLOCKED** |

**The proposed PR #31–#37 plan does not create decision D10.** It does not automatically approve a sidecar schema, additional opcode set or release date. Any material change to architecture or restrictions requires a separate architecture decision record (ADR) and owner approval.

Every modification to D1–D9 must be documented in a separate ADR/PR with rationale, risks and tests. A new decision does not take effect until the owner explicitly approves it.

[Verification matrix](verification_gates.en.md) · [PR process](development_workflow.en.md) · [Current filesystem threat model](filesystem_security_pr29.en.md)
