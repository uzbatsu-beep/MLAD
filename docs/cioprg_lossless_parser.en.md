<!-- MLAD-I18N:EDITORIAL:reviewed 2026-11-19 against cioprg_lossless_parser.md -->
> **Language / Язык:** [English](cioprg_lossless_parser.en.md) · [Русский](cioprg_lossless_parser.ru.md) · [Original](cioprg_lossless_parser.md)
>
> **Boundary:** Exact-byte source inspection is **not** a verified native compiler. `CIOPRG.LST`/`IONAME.DAT` read-only, HSES/FTP/native writer/deployment **BLOCKED**, OEMBench **NOT RUN**.

> **Current evidence boundary (21 November 2026):** `main` `9fcd11d61856c8a559d9acf3c77bd662ad814dfe` and Windows pre-release `v0.5.0-rc.10-unified-offline`. This technical specification must be read with the [current repository audit](repository_audit_2026-11-21.en.md); historical parser/PR statuses below are time-scoped. Native file write and `HSES`/`FTP` remain `BLOCKED`; `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`. No new OEM semantic or firmware guarantee is asserted.

# CIOPRG.LST lossless parser — PR #2 implementation contract

> **Status and applicability:** this is the constrained parser implementation and evidence available by PR #21; it **does not** mean that the whole project stopped at PR #21. For `main` after [PR #29](https://github.com/uzbatsu-beep/MLAD/pull/29) and independent CI/UserSmoke/OEM levels see [project status](project_status.en.md). This document authorizes **no** native writer, HSES or real controller scan execution.

**Status after merged PR #21 (18 November 2026):** a read-only inspection parser exists; a manufacturer-compatible complete native-file recompiler is **NOT implemented**. Controller profiles: DX100, DX200, YRC1000.

## Goals achieved

The Rust parser `src-tauri/src/compiler/lossless.rs` operates on **raw bytes**. It does not use UTF-8 decoding as a condition of successful import. It preserves every byte, including BOM, non-UTF-8 character sequences, whitespace and CRLF/LF/CR endings. `copy_original()` is a byte-for-byte copy; the edited/exported/recompiled path does not exist.

The parser calculates a **SHA-256 fingerprint of the immutable raw byte snapshot** for provenance; it is displayed in the inspector and changes if any original byte changes.

The parser maintains a complete sequence of lines with:

- 1-based line numbers;
- byte spans for the entire line and its content without EOL;
- original EOL type;
- a typed *overlay* for a limited set of instructions, including original operand spellings and byte spans;
- separate diagnostics for unsupported mnemonics and invalid operands;
- an inspection summary with bounded UI preview and diagnostics.

The `cmd_inspect_cioprg` Tauri command accepts a local `.LST` path and **explicit controller profile**, reads at most 16 MiB and returns a preview and diagnostics. It has **no write, edit, compile, HSES or FTP action**. The UI exposes the read-only "Inspect CIOPRG.LST" view. The original `cmd_load_project` continues to reject `.LST` files: it returns the old editor-only JSON model, which cannot represent all source constructs.

## Lexical recognition only

| Input | Interpretation | Notes |
|---|---|---|
| Empty line | Empty | all original bytes retained |
| `*` or `;` at start | Comment | `;` treated as legacy commentary; semantics unverified |
| `//...` or `/...` | Directive | exact section semantics unverified |
| `//$M` + 8 rows | Known header marker + row overlays | each row expected 10 integer text fields; never normalize values |
| `STR`, `STR-NOT`, `AND`, `AND-NOT`, `OR`, `OR-NOT`, `OUT` | Typed bit-operand instruction | expects canonical `#XXXXX` with family-specific address validation |
| `AND-STR`, `OR-STR` | Recognized structural opcode | zero operands; **the lexer itself does not construct an execution graph; `semantic.rs` handles only the supported Boolean subset** |
| `TMR`, `CNT` | Typed destination/source overlay | destination `Mxxx`, source `Mxxx` or unsigned decimal; **lexical recognition does not establish stateful behavior; `advanced.rs` contains only local test models** |
| `GSTR`, `GOUT` | Typed byte-group operand | conservative pairing/address checks in `advanced_source.rs`; complete manufacturer behavior still requires fixtures |
| Everything else | Unsupported / opaque | warn, retain bytes and spans; never delete |

The `//$M` header corresponds to eight groups of ten values from M550..M559 down to M480..M489, with application-specific meanings. Parsing a row into ten numbers **does not** authorize modifying that row.

## Safety boundary

**The parser must not imply editable System/User ownership.** A string `//USER` (or a comment containing USER) is *not* accepted as proof of an authorized User Ladder region. `InspectionReport.system_user_boundaries_verified` remains `false` for every input.

**No edited file exports.** `edited_export_allowed` is `false` for all reports regardless of diagnostic count. This remains so even for a syntactically simple file, as file-level grammar and protected areas are not yet verified.

**Do not interpret a clean parser report as successful controller validation.** The subset recognizes some instructions but doesn't prove correct relay ownership, branch graph semantics, file section ordering or controller firmware compatibility. The old non-lossless `parser.rs` remains for historical JSON tests only and is not used to inspect CIO files.

## Tests

Unit tests under `src-tauri/src/compiler/lossless/tests.rs` verify:

1. Exact round-trip for LF, CRLF, CR, final line without terminator.
2. BOM, arbitrary non-UTF-8 and opaque binary preservation.
3. Known mnemonics, `#00010` leading zeroes, source byte spans.
4. `//$M` eight-row header, invalid/truncated headers.
5. Controller-dependent address classification and invalid bit indexes.
6. Unknown instructions, directives and comments are never silently discarded.
7. Preview truncation and never-editable status.
8. Deterministic byte fuzzing for 256 different arbitrary inputs.
9. SHA-256 known-answer verification and change detection.

### Remaining acceptance criteria

The complete **verified** parser requires authorized original `CIOPRG.LST` exports from each of DX100, DX200, YRC1000, with model/firmware/application and file hash recorded. Actual data may be sensitive; use private fixtures or anonymized representative segments with explicit permission.

Acceptance before any edited export: establish System/User section boundaries and header grammar from actual exports, cover the complete instruction schema and branch execution AST, compare with Ladder Editor 32 on real fixture sets, prove byte-preservation for untouched protected sections, and independently review the file transfer procedure. **This PR is not sufficient for safe controller programming.**

Manufacturer evidence:
- [Yaskawa Concurrent I/O Overview](https://knowledge.motoman.com/hc/en-us/articles/1500012236161-Concurrent-I-O-Overview)
- [Yaskawa //$M header specification](https://knowledge.motoman.com/hc/en-us/articles/27636724261138-Specification-of-M-header-in-CIOPRG-LST)
- [Yaskawa editing options and safeguards](https://knowledge.motoman.com/hc/en-us/articles/4408446257175-Options-for-Editing-the-Concurrent-I-O-Ladder)
- [PR #1 source registry](source_registry.en.md)


## Integration with the newer architecture

- The `semantic.rs` layer builds Boolean expressions, partially identifies `PART/END` and warns about stateful instructions. It **does not** replace `copy_original()`.
- `advanced_source.rs` can display **narrow, recognized** sequences of TMR/CNT/GSTR/GOUT; unsupported topologies are not fabricated into valid native rungs.
- `GraphWorkspace` and `AdvancedBlocksPanel` are independent JSON editors; PR #8 double-click behavior **never** makes the original LST editable.
- Golden fixtures and OEM roundtrip: [controller fixture intake](controller_fixture_intake.en.md). System-level file limitations: [CIOPRG.LST specification](cioprg_lst_spec.en.md).
