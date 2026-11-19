<!-- MLAD-I18N:EDITORIAL:reviewed 2026-11-19 against semantic_verification_matrix.md -->
> **Language / Язык:** [English](semantic_verification_matrix.en.md) · [Русский](semantic_verification_matrix.ru.md) · [Original](semantic_verification_matrix.md)
>
> **Evidence:** The historical pre-PR21 "not verified" table and the newer manual-derived reference matrix have **different dates and evidence levels**. Neither constitutes a verified controller scan. Native `CIOPRG.LST`/`IONAME.DAT` read-only; HSES/FTP/native writer/deployment **BLOCKED**; OEMBench **NOT RUN**.

# MLAD PR #20 — Semantic Verification and Source-Reference Audit

**Current as of 18 November 2026:** PR #20 and #21 had **merged**. Limited static reference auditing and independent OEM manual-derived vectors were implemented, but **actual EN/scan/flag behavior had not been measured on a controller**. The first table below documents the **historical state before PR #21**. The subsequent documentary status is in the [five-opcode analysis](oem_5_opcode_analysis.en.md). See the [project maturity matrix](project_status.en.md).

## Sources and evidence levels

1. The official [YRC1000 Concurrent I/O R12 index](https://knowledge.motoman.com/hc/en-us/articles/40670912825362-YRC1000-OPTIONS-INSTRUCTIONS-FOR-CONCURRENT-I-O-R12) confirms that the manual contains §12.2.17 ADD, §12.2.18 SUB, §12.2.24 MOV, §12.2.29 SHL, §12.2.30 SHR and §12.3 Arithmetic Flag. **The index alone does not prove instruction behavior.**
2. The [YRC1000 Ladder Editor manual HW1483388](https://www.motoman.com/getmedia/7DCAB851-59B3-4698-97FB-AE39E45FD106/179482-1CD) confirms that the editor lists these instructions. It does **not** establish scan details, actual relay states or equivalence across controller generations.
3. [docs/typed_opcode_grammar.md](typed_opcode_grammar.en.md) describes the typed **operand forms** introduced in PR #19. This is syntactic classification, **not measurement of execution results**.
4. Five genuine exported CIOPRG.LST files had previously been used for **private form inventory**. Synthetic tests available to MLAD do **not** verify OEM semantics. Actual files and production addresses must not be published.

## Historical separation of evidence (before PR #21)

| Instruction | MLAD operand form | S/D position | Result value | Execution condition / scan | Carry/Zero/Error | Impact on relays/other rungs | OEM test |
| --- | --- | --- | --- | --- | --- | --- | --- |
| MOV | PR #19 syntax-only | S,D hypothesis | **Not verified** | **Not verified** | **Not verified** | **Not verified** | Pending |
| ADD | PR #19 syntax-only | S1,S2,D hypothesis | **Not verified** | **Not verified** | **Not verified** | **Not verified** | Pending |
| SUB | PR #19 syntax-only | S1,S2,D hypothesis | **Not verified** | **Not verified** | **Not verified** | **Not verified** | Pending |
| SHL | PR #19 syntax-only | S,n,D hypothesis | **Not verified** | **Not verified** | **Not verified** | **Not verified** | Pending |
| SHR | PR #19 syntax-only | S,n,D hypothesis | **Not verified** | **Not verified** | **Not verified** | **Not verified** | Pending |

**Do not treat calculations from an educational JavaScript model, or generic IEC 61131-3 rules, as evidence of Yaskawa Concurrent I/O execution.** Carry, overflow, signedness, arithmetic flags and EN behavior require separate controller/firmware-specific evidence.

## Updated evidence classification after PR #21

| Opcode | Lexical form | Manual formula for a standalone instruction | Flag behavior described in manual | Real controller |
| --- | --- | --- | --- | --- |
| MOV | TypedCandidate | S→D transfer (16-bit case) | MOV byte Carry has an unresolved boundary at 256 | **Not tested** |
| ADD | TypedCandidate | `(S1+S2) mod 65536` | 16-bit Carry/Zero | **Not tested** |
| SUB | TypedCandidate | `(S1−S2) mod 65536` | Borrow/Zero | **Not tested** |
| SHL | TypedCandidate | Logical 16-bit left shift | Carry is a shifted-out bit; n=0/16 unresolved | **Not tested** |
| SHR | TypedCandidate | Logical 16-bit right shift | Carry is a shifted-out bit; n=0/16 unresolved | **Not tested** |

**ManualDerived does not mean BenchVerified.** The reference core remains isolated from native CIO/IPC. Real EN interactions among instructions, per-scan flag updates and firmware-specific behavior still require independent verification.

## Implemented by PR #20 — conservative static reference audit

`src-tauri/src/compiler/typed_dataflow.rs`:

- Consumes only the byte-preserving reader's `TypedCandidate` records. It does **not** compute `MOV`, `ADD`, `SUB`, `SHL` or `SHR` results.
- `CandidateRole` enumerates **Source**, **ShiftAmount** and **Destination**. These are hypotheses about *operand position*, not proven memory reads or writes.
- `possible_previous_definition` denotes an **exact string match** between an M-register operand and a preceding typed destination. It does **not** establish execution or actual value propagation.
- `#XXXX0` and `W#XXXX0` matches do **not** produce links because byte/bit/word aliasing and the memory map have not been verified.
- The context is cleared at `PART`, `END`, any other known Boolean/advanced instruction, unsupported/malformed instruction or unverified section. Processing stops after END.
- The report includes the number of all candidates and tentative matches. Detailed events are limited to the first **1,000**; `events_truncated` explicitly signals truncation.
- `semantics_verified=false`, `scan_order_verified=false`, `arithmetic_flags_verified=false`, `native_write_allowed=false`. No native CIO modification/upload path exists.

## Mandatory independent OEM verification protocol

Tests may be performed **only on a separately approved test rig with the equipment owner's authorization**, without automatic CIO upload or modification of System Ladder:

1. Record controller family (DX100/DX200/YRC1000), firmware, installed options, OEM manual revision, register configuration and SHA-256 of the authorized export.
2. Use the official Ladder Editor to verify operand forms. Record any discrepancies as `unverified`.
3. For each instruction, determine the exact EN=0/1 behavior, edge cases, overflow, destination update order, Carry/Zero/Error and effects on later instructions.
4. Run controlled tests only in an isolated environment with no connected actuators and an approved site safety procedure. PR #20 **does not include or propose a way to upload files to a robot**.
5. Support observations with OEM IDE/controller evidence, repeatable tests and references to the exact manual revision. Without sufficient evidence, the state remains `Unknown`.
6. Only after a separate code review may execution of a **particular independently verified subset** be considered in an isolated simulator. Authorization of native writer/HSES/FTP remains a **separate, blocked milestone**.

## PR #20 acceptance criteria

- Synthetic Rust tests cover exact M-register text, missing/unsupported operands, PART/END, barriers at unknown/Boolean instructions, output truncation limits, absence of unverified alias inference and immutable source bytes.
- UI shows only *tentative textual links* with source-line navigation and makes unverified semantics apparent.
- Rust/TypeScript CI and Windows MSVC checks pass; merging is subject to the project owner's approval.

## PR #21 — documented reference semantics, not bench equivalence

The historical "not verified" table above describes the state **before** YRC1000 R12 §12.2–12.3 and DX100 Rev.0 were studied. Individual EN=ON rules, formulas and flag tables are now **documented**, for all five instructions, in the [reference study](oem_5_opcode_analysis.en.md). This does **not** make an actual CIOPRG program executable by MLAD and does **not** change `semantics_verified=false` or `scan_order_verified=false`.

## Further work after PR #29

[Roadmap v3](roadmap.en.md) proposes synthetic parser fixtures, typed grammar (provisionally PR #34) and source-linked static analysis (provisionally PR #35). Current `ManualDerived` results and tentative M-links **do not** establish real EN/Carry/Zero/Error values during a scan, System/User ownership or actual hardware read/write order. Until independent OEM bench data exists, results remain `Unresolved` or `NotExecuted`; HSES/FTP/native writer remain `BLOCKED`. See [project status](project_status.en.md).
