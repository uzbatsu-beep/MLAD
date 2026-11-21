<!-- MLAD-I18N:EDITORIAL:reviewed 2026-11-19 against cioprg_lst_spec.md -->
> **Language / Язык:** [English](cioprg_lst_spec.en.md) · [Русский](cioprg_lst_spec.ru.md) · [Original](cioprg_lst_spec.md)
>
> **Evidence:** Language review against the MLAD source, **not** independent OEM controller verification. Native `CIOPRG.LST`/`IONAME.DAT` stay read-only; HSES/FTP/native writer/deployment remain **BLOCKED**; OEMBench **NOT RUN**.

> **Current evidence boundary (21 November 2026):** `main` `9fcd11d61856c8a559d9acf3c77bd662ad814dfe` and Windows pre-release `v0.5.0-rc.10-unified-offline`. This technical specification must be read with the [current repository audit](repository_audit_2026-11-21.en.md); historical parser/PR statuses below are time-scoped. Native file write and `HSES`/`FTP` remain `BLOCKED`; `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`. No new OEM semantic or firmware guarantee is asserted.

# CIOPRG.LST specification baseline (DX100 / DX200 / YRC1000)

> **Status and applicability:** this document covers the constrained implementation subset and evidence available by PR #21; it does **not** imply that all project development stopped at PR #21. See the current `main` [project status](project_status.en.md) for the post-[PR #29](https://github.com/uzbatsu-beep/MLAD/pull/29) baseline and independent CI/UserSmoke/OEM evidence levels. This specification authorizes **neither** native writing, HSES nor real scan execution.

**Status (18 November 2026, after merged PR #21):** a **partial** documented specification; read-only lossless/semantic/advanced source inspection is implemented. This is **NOT** a complete verified native-file grammar, a native writer or a certified controller compiler.
**Scope:** the offline CIO program file and mnemonic syntax; **not** INFORM `.JBI`, HSES packets or YRC1000micro.
**Safety:** inspect/view of user-supplied `.LST` is available in a bounded read-only workflow; modified regeneration and controller deployment remain **disabled** until real-controller fixtures, protected boundaries, instruction semantics and vendor round-trip verification are established.

## 1. Evidence levels

- **CONFIRMED:** explicit in the referenced vendor documents.
- **PARTIAL:** a documented form exists but its file-level delimiters/variant rules have not been verified against full exported CIO files.
- **UNKNOWN:** retain raw data; do not invent syntax or silently normalize.

Official source registry and exact documents: [source registry](source_registry.en.md).

## 2. File identity, ownership and safety

**CONFIRMED:** Concurrent I/O executes separately from robot INFORM motion jobs. Its export is named **`CIOPRG.LST`**, saved from `EX.MEMORY → SAVE → I/O DATA → C.IO PRGM`. The file represents a program containing **System Ladder** (controller/application-specific, only Yaskawa-authorized modification) and **User Ladder** (editable with proper permission). Never copy a whole system ladder between robots merely because their models match. [Yaskawa Concurrent I/O Overview](https://knowledge.motoman.com/hc/en-us/articles/1500012236161-Concurrent-I-O-Overview)

**HISTORICAL / BLOCKED:** In early code, a legacy exporter attempted `/JOB`, `//POS`, `//INST` and `IOCPT STRSTART` and used an FTP name `CIOLAD.LST`. Those templates are **not native CIOPRG.LST**. The current Tauri `cmd_deploy_to_robot` always refuses requests; `CIOPRG.LST` edited export is not exposed. No such historical template may be labelled controller-ready.

**UNKNOWN:** complete text-level system/user section separators, exact mandatory header order, encoding variations, comments, checksums, application metadata, END placement and per-generation headers. All must be established from real exports plus vendor operator/compiler evidence. In particular, do **not** assume that `/JOB` is valid here.

### Preservation contract (mandatory for implementation)

1. Import a `CIOPRG.LST` **as bytes** and record the source SHA-256, length, encoding decision and line-endings. Do not change the byte stream during viewing.
2. Keep an ordered, lossless list of `RawLine { raw_bytes, line_ending, span, classification }` and immutable file-level sections. Preserve unknown headers, comments, whitespace, leading zeroes, ordering and unrecognized mnemonic/operand forms.
3. A best-effort AST is an **overlay**, not a substitute for original bytes. Report unsupported constructs with line/byte offsets, never drop them.
4. Viewing and diagnostics may continue with unknown constructs. **Recompiling or exporting an edited file must fail closed** unless every modified semantic region is supported and the original protected regions are byte-identical.
5. Never rewrite System Ladder, protected metadata, register initialization, or undocumented sections. Patch only an explicitly identified User Ladder region. If ownership boundaries cannot be positively identified, export **must be disabled**.
6. An untouched document's round-trip `import → export` must be byte-identical; an edited document must have a documented diff confined to permitted regions.
7. All output requires a selected controller family, version/configuration, validated semantics, clean diagnostics, explicit operator confirmation and offline backup. No network write is authorized by this spec.

## 3. Confirmed example mnemonic syntax

**CONFIRMED:** Yaskawa concurrent-I/O manuals describe relay bit addresses as **`#XXXXX`**, exactly five decimal digits, last digit 0–7. Thus `#70010` is valid, `#70018` is not. One channel holds eight relays, and channel number is `address / 10`. `0xxxx` must preserve leading zeroes. Bare addresses stored by the legacy MLAD JSON editor are **a compatibility input only**, not an asserted canonical CIOPRG text syntax.

**CONFIRMED** instructions include:
- `STR #70010`, `STR-NOT #70010`: start chain, normal/inverted.
- `AND #70010`, `AND-NOT #70010`, `OR #70010`, `OR-NOT #70010`: boolean composition.
- `AND-STR`, `OR-STR`: group/branch operations; **not** just a flat sequence of contacts.
- `OUT #70010`: coil output (duplicate OUT destination is an error).
- `TMR M010,25`, `CNT M256,M466`: register/current-value and preset syntax examples. **Do not** model the first argument as a 1..256 timer/counter index. Instruction reference tables must be used to validate writable destinations and source types.
- `GSTR #70010`, `GOUT #70100`: grouped/byte relay operands.
- `MOV`, `BMOV`, `CMP`, arithmetic, shift, BCD, pulse instructions are documented; this is **not** a complete instruction semantics implementation.

The controller manuals list instruction-specific types: relay bit `#XXXXX`, relay byte `#XXXX0`, relay word `W#XXXX0`, 16-bit M registers `M000`–`M999`, and decimal immediates. Operand position and permission depend on mnemonic. For examples see DX200 `RE-CKI-A465` §12.1–12.2 and YRC1000 `HW1483388` §1.3.3.

### Lexical subset (PARTIAL, for recognizers **only**)

```ebnf
relay_bit      = "#", digit, digit, digit, digit, bit_digit ;
relay_byte     = "#", digit, digit, digit, digit, "0" ;
relay_word     = "W#", digit, digit, digit, digit, "0" ;
m_register     = "M", digit, digit, digit ;
immediate      = digit, { digit } ;
bit_mnemonic   = "STR" | "STR-NOT" | "AND" | "AND-NOT"
                 | "OR" | "OR-NOT" | "OUT" ;
bit_statement  = bit_mnemonic, whitespace, relay_bit ;
group_operator = "AND-STR" | "OR-STR" ;
```

This grammar is **only a recognizer for known individual instructions**; it is intentionally not represented as the grammar for the entire file. Byte/word operands are **contextual**, e.g. `#70010` may denote a bit or byte according to the instruction. `W#70010` must never be coerced into an ordinary bit.

No assumption is made about whether a newline alone separates every instruction, about inline comments, or about system/user section markers. Preserve what was actually exported.

## 4. The documented `//$M` header

**CONFIRMED (Yaskawa article, 2025-06-30):** an exported `CIOPRG.LST` may contain a section named `//$M` with **eight lines of ten comma-separated values**, relating to **M550 down to M480** in descending groups of ten:

| Header row | Corresponding registers |
|---|---|
| 1 | M550–M559 |
| 2 | M540–M549 |
| 3 | M530–M539 |
| 4 | M520–M529 |
| 5 | M510–M519 |
| 6 | M500–M509 |
| 7 | M490–M499 |
| 8 | M480–M489 |

**Do not** silently replace its values, re-index or interpret the application-specific meanings universally. The values differ for welding/painting and potentially other applications.

This section is *not* interchangeable with the separate Ladder Editor dialog for system-register header parameters `M920–M999` documented by `HW1483388` §1.3.6.1. Both require faithful preservation until exact file-level mapping is validated.

Source: [Yaskawa: Specification of ‘$M’ header in CIOPRG.LST](https://knowledge.motoman.com/hc/en-us/articles/27636724261138-Specification-of-M-header-in-CIOPRG-LST).

## 5. Internal representation / compilation design

```text
ControllerProfile(DX100 | DX200 | YRC1000, firmware, application, I/O options)
  └─ FileSnapshot(raw bytes, source digest, encoding, line endings)
      ├─ RawSections (immutable unless explicitly confirmed User Ladder)
      ├─ RawLines (offsets, original tokens, whitespace and endings)
      ├─ ParsedSyntax (instruction kind + typed operands + source spans)
      ├─ SemanticGraph (series/parallel branches, rung boundaries)
      └─ Diagnostics (severity, controller, evidence, line, column)
```

Two-stage parsing is required: lossless lexer (may recognize but **never delete** unknown syntax), then a controller-aware semantic analyzer. A plain `Vec<Rung> { instructions: Vec<Instruction> }` cannot losslessly represent all branches, instructions and metadata.

**Compilation gates:**

- G0: controller & exported-file provenance known.
- G1: every relevant mnemonic/operand recognized and source spans tracked.
- G2: protected/unknown sections unchanged; User Ladder boundaries verified.
- G3: typed memory ranges and instruction-specific read/write/type constraints pass.
- G4: logical branch tree and execution order validated, duplicate coils checked.
- G5: build golden fixtures pass, independent Ladder Editor confirms intended results.
- G6: only then may a separately designed, explicit offline output be enabled; robot deployment remains a later PR with independent safety review.

## 6. Validation matrix / required test fixtures

| Test | Expected outcome |
|---|---|
| Imported untouched file (DX100/DX200/YRC1000) | Byte-for-byte identity, including CRLF and unknown sections |
| Unknown instruction `VENDOR_X` | Report unsupported; preserve original bytes; no edited export |
| Missing/ambiguous system–user boundary | Read-only; fail closed on write |
| `STR #70010`; `OUT #70020` | Recognize two relay bit operands |
| `STR #70018` | Invalid bit index |
| `STR #00010` | Preserve leading zeros |
| `CNT M256,M466` | Two register operands, not timer ID + integer |
| `TMR M010,25` | Register destination + 16-bit numeric setpoint (verify per family) |
| `AND-STR` / `OR-STR` | Branch graph, not discarded by lexer |
| `//$M` with nonzero values | Preserve eight rows and raw bytes |
| Same output repeated in two rungs | Duplicate coil diagnostic |
| User edit plus unknown System Ladder content | System Ladder remains byte-identical |
| Same relay on different generations | Family-specific classification (see memory model) |

**Data needed for a *fully verified* file grammar**: anonymized controller-exported `CIOPRG.LST` for each of DX100, DX200 and YRC1000, with original controller model, firmware, application type and Ladder Editor syntax-check outcome. Do not invent examples of complete headers or assert controller-ready compilation until these fixtures exist.

## 7. Implemented subset after PR #8

| Component | Current implemented subset | Evidence/limitation |
|---|---|---|
| `src-tauri/src/compiler/lossless.rs` | Byte-preserving import, original SHA-256, source spans and typed lexical overlay | **Read-only inspection works**; unknown material remains unchanged |
| `src-tauri/src/compiler/semantic.rs` | STR/AND/OR, NOT, AND-STR/OR-STR, OUT Boolean branches, diagnostics and graph | Partial AST; stateful instructions are barriers |
| `src-tauri/src/compiler/advanced_source.rs` | Simple TMR/CNT samples and GSTR/GOUT pairs | Read-only inventory, **not** complete logic reconstruction |
| `src-tauri/src/compiler/graph_draft.rs` | `*.graph.json` and controller-aware address validation | Separate JSON, not native controller program |
| `src-tauri/src/compiler/advanced.rs` | `*.advanced.json`, local timer/counter/group models and diagnostics | Local models, not a hardware scan |
| `src/components/SvgOperandInput.tsx` | Double-click/F2 operand editing in **JSON** diagrams | Does not edit Inspector `CIOPRG.LST` |
| `src-tauri/src/main.rs` | `.LST` inspection, JSON editing; HSES/FTP commands refuse requests | **No controller modification** |

Opening an original `CIOPRG.LST` **does not** mean its full contents can be imported into the editable graphical model. These are separate representations with different coverage.

## 8. Unverified features and technical debt

- No complete OEM-verified native file covering all separators and System/User ownership boundaries across firmware/application configurations. `PART` is an analysis label **not** proof that System Ladder can be edited.
- `src-tauri/src/compiler/parser.rs` is the **legacy parser** for an older JSON model. It must **not** replace `lossless.rs` or be used as the new native importer.
- PR #13 **removed** `src-tauri/src/network/mod.rs`; the unverified network runtime is no longer compiled into Tauri. HSES/FTP/deployment remain deny-only. The `suppaftp` dependency still appears in `src-tauri/Cargo.toml` for separate dependency review, but **does not** prove that robot upload is available. See [HSES](hse_protocol.en.md) and [Security](../SECURITY.en.md).
- The newer UI tabs have DX100/DX200/YRC1000 profile selection, but this is **not** an exact firmware/options/workcell-topology binding. The legacy JSON interface and validator cannot be treated as hardware certification.
- Combined semantics of stateful instructions, scan sequence, M-register effects, complete native-file validation and OEM acceptance remain unverified on actual controllers.
- Closing these gaps requires [authorized native file intake](controller_fixture_intake.en.md) and the [roadmap](roadmap.en.md), **not** inventing additional unverified templates.

**Native export, protected-area editing and connecting to a production controller remain prohibited.**

## Read-only post-END data classification (PR #18 merged)

The lossless parser now distinguishes syntactically valid \`END\` from subsequent **observed** \`//ALARMN\` and \`//MESSAGEN\` data markers. Subsequent text is preserved as non-executable, display-only \`PostEndSection\`, \`PostEndData\` or \`PostEndOpaque\` rather than inventing extra Ladder instructions. See [post-END classification and test limitations](post_end_data.en.md). This is a structural observation and **not** confirmation of a universal OEM grammar.

## PR #19 — TypedCandidate forms for MOV/ADD/SUB/SHL/SHR

The `TypedCandidate` layer is distinct from executable `Instruction` and `UnsupportedInstruction`. It validates lexical operand roles and the constraints in documented DX200/YRC1000 tables, **not** full execution semantics. Flags, register writes, scan behavior and physical relay changes remain unverified. At this stage DX100-specific forms of these five instructions have not been established independently. See the [forms, sources and limitations](typed_opcode_grammar.en.md).

## Next native-grammar verification phase (after PR #29)

[Roadmap v3](roadmap.en.md) and the [proposed parser corpus PR #31](implementation_plan.en.md) require synthetic edge cases for `//$M`, `PART/END`, post-END data, unknown/opaque spans and exact source ranges. Actual private exports must only be handled through authorized [fixture intake](controller_fixture_intake.en.md) and must not be published. None of this enables a native writer.
