# CIOPRG.LST specification baseline (DX100 / DX200 / YRC1000)

**Status:** engineering baseline v0.1, **NOT** a complete verified file grammar or certified controller compiler.
**Scope:** the offline CIO program file and mnemonic syntax; **not** INFORM `.JBI`, HSES packets or YRC1000micro.
**Safety:** display/import of untrusted files is permitted; binary-compatible regeneration and controller deployment must remain **disabled** until real-controller fixtures, instruction semantics and vendor round-trip verification are available.

## 1. Evidence levels

- **CONFIRMED:** explicit in the referenced vendor documents.
- **PARTIAL:** a documented form exists but its file-level delimiters/variant rules have not been verified against full exported CIO files.
- **UNKNOWN:** retain raw data; do not invent syntax or silently normalize.

Official source registry and exact documents: [source registry](source_registry.md).

## 2. File identity, ownership and safety

**CONFIRMED:** Concurrent I/O executes separately from robot INFORM motion jobs. Its export is named **`CIOPRG.LST`**, saved from `EX.MEMORY → SAVE → I/O DATA → C.IO PRGM`. The file represents a program containing **System Ladder** (controller/application-specific, only Yaskawa-authorized modification) and **User Ladder** (editable with proper permission). Never copy a whole system ladder between robots merely because their models match. [Yaskawa Concurrent I/O Overview](https://knowledge.motoman.com/hc/en-us/articles/1500012236161-Concurrent-I-O-Overview)

**BLOCKED:** The legacy MLAD exporter creates `/JOB`, `//POS`, `//INST` and `IOCPT STRSTART` and names an FTP target `CIOLAD.LST`. These are **not verified as CIOPRG.LST file structure**. Do not identify this output as controller-ready.

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

## 7. Status of PR #2 read-only lossless parser\n\nA byte-preserving read-only parser with a typed syntax overlay and SHA-256 provenance is implemented in `src-tauri/src/compiler/lossless.rs`. See [lossless parser contract](cioprg_lossless_parser.md). **It is not a validated full-file compiler.** Original System/User boundaries, all instruction families, semantic branching, edited file export and deployment remain unverified/disabled.\n\n## 8. Known technical debt in current MLAD code (not fixed by this specification)

- `src-tauri/src/compiler/parser.rs` silently skips unknown instructions and only parses limited mnemonics; it does not faithfully parse `#`-prefixed operands or entire CIO files.
- `src-tauri/src/main.rs` creates a `/JOB`-style pseudo-listing, writes it even with validation errors and ignores some I/O errors.
- `src-tauri/src/network/mod.rs` implements neither verified HSES nor a validated CIOPRG transfer workflow.
- The current memory model is exposed as a separate Rust module, and the validator consumes it, but the **UI still does not collect a controller profile**. Implicit YRC1000 validation is legacy-only and not deployment clearance.

References and review history: [source registry](source_registry.md), [typed memory](typed_memory_model.md).
