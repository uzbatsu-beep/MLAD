<!-- Generated en companion for docs/semantic_verification_matrix.md; review required. -->
> **English** · [Русский](semantic_verification_matrix.ru.md) · [English](semantic_verification_matrix.en.md) · [Original / Оригинал](semantic_verification_matrix.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.

# MLAD PR #20 Semantic verification and audit of references

**Actual on 18.11.2026:** PR #20 и PR #21 **merged. Static auditing and autonomous OEM manual-derived vectors are implemented, but **real EN/scan/flags were not measured on the controller**. The table below is the historical baseline to PR #21; the current documentary level is supplemented by [five-team](oem_5_opcode_analysis.en.md). [Readiness matrix](project_status.en.md).

## Sources and level of evidence

1. Official index [YRC1000 Concurrent I/O R12](https://knowledge.motoman.com/hc/en-us/articles/40670912825362-YRC1000-OPTIONS-INSTRUCTIONS-FOR-CONCURRENT-I-O-R12) §12.2.17 ADD, §12.2.18 SUB, §12.2.24 MOV, §12.2.29 SHL, §12.2.30 SHR and §12.3 Arithmetic Flag. ** The index itself does not prove the conduct of the command**.
2. [YRC1000 Ladder Editor manual HW1483388](https://www.motoman.com/getmedia/7DCAB851-59B3-4698-97FB-AE39E45FD106/179482-1CD) confirm the presence of these instructions in the editor's menu. Does not prove scan features, relay status or generational equivalence.
3. [docs/typed_opcode_grammar.md](typed_opcode_grammar.en.md) It contains typed **forms** added PR #19. It is the result of syntactic analysis, not measurement of performance.
4. Five true true. CIOPRG.LST Previously used for private inventory forms. Available synthetic tests confirm OEM semantics. Do not publish real files/addresses.

## Historical division of evidence (before) PR #21)

| Team. | The form of operands in MLAD | Positions S/D | Importance of outcome | Condition of execution/scanning | Carry/Zero/Error flags | Influence on the relay and other circuits | OEM test |
| --- | --- | --- | --- | --- | --- | --- | --- |
| MOV | PR #19 syntax-only | The S,D hypothesis | **Not confirmed* * | **Not confirmed* * | **Not confirmed* * | **Not confirmed* * | Expected. |
| ADD | PR #19 syntax-only | Hypothesis S1,S2 . D. | **Not confirmed* * | **Not confirmed* * | **Not confirmed* * | **Not confirmed* * | Expected. |
| SUB | PR #19 syntax-only | Hypothesis S1,S2 . D. | **Not confirmed* * | **Not confirmed* * | **Not confirmed* * | **Not confirmed* * | Expected. |
| SHL | PR #19 syntax-only | The S,n,D hypothesis | **Not confirmed* * | **Not confirmed* * | **Not confirmed* * | **Not confirmed* * | Expected. |
| SHR | PR #19 syntax-only | The S,n,D hypothesis | **Not confirmed* * | **Not confirmed* * | **Not confirmed* * | **Not confirmed* * | Expected. |

**Do not consider computations in “training” JavaScript or the general rules of IEC 61131-3 as proof of Yaskawa Concurrent I/O.** In particular, transfer, overflow, sign, flag and execution by EN must be separately confirmed by the controller and the software version.

## Current classification after PR #21

| Opcode | Lexical form | Formula single operation in the manual | Flags in the manual | The Real Controller |
| --- | --- | --- | --- | --- |
| MOV | TypedCandidate | Transmission S→D (for 16-bit) | MOV byte Carry - boundary uncertainty 256 | **Unchecked** |
| ADD | TypedCandidate | `(S1+S2) mod 65536` | Carry/Zero on 16-bit results | **Unchecked** |
| SUB | TypedCandidate | `(S1−S2) mod 65536` | Borrow/Zero | **Unchecked** |
| SHL | TypedCandidate | Logical 16-bit left shift | Carry by repressed bit; n = 0/16 indefinite | **Unchecked** |
| SHR | TypedCandidate | Logical 16-bit right shift | Carry by repressed bit; n = 0/16 indefinite | **Unchecked** |

**ManualDerived does not mean BenchVerified.** Reference kernel isolated from CIO/IPC. Future independent verification remains responsible for EN conditions when multiple instructions, scan flags and a particular firmware interact.

## Realized in PR #20Limited static auditing

\`src-tauri/src/compiler/typed_dataflow.rs\`:

- Only the original \`TypedCandidate\` from the byte analyzer. No calculations.`MOV\`, \`ADD\`, \`SUB\`, \`SHL\`, \`SHR\`.
- \`CandidateRole\`: **Source**, **ShiftAmount**, **Destination**—Hypotheses *positions of operand* on the basis of form, not the fact of reference to memory.
- \`possible_previous_definition\` **Strictly matches the text name of the M-register with the previous typed destination**. It never means a guaranteed transfer of value or execution of a command.
- Coincidences`#XXXX0\`, \`W#XXXX0\` They do not create links because overlapping bits/bytes/words and the address card are not confirmed.
- The context is reset by`PART\`, \`END\`Any other known Boolean/extended instruction, unknown/erroneous instruction, and unconfirmed section. Scanning after END is stopped.
- The report contains the number of all candidates and all alleged matches, but the detailed list is limited to the first 1000 entries.`events_truncated\` It clearly means abbreviation.
- \`semantics_verified=false\`, \`scan_order_verified=false\`, \`arithmetic_flags_verified=false\`, \`native_write_allowed=false\`. There is no way to download/edit native CIO.

## The next mandatory OEM verification protocol

Conduct **only on a separate test bench and with the permission of the equipment owner**, without automatically loading the CIO or changing the system Ladder:

1. Lock the controller (DX100/DX200/YRC1000), system software version, options, OEM manual revision, register configuration and SHA-256 discharge.
2. Verify acceptable forms through the regular Ladder Editor; all discrepancies are issued as`unverified\`.
3. Set for each command: exact operation at EN=0/1, limit values, overflow, order of updating the destination address, Carry/Zero/Error, influence on subsequent instructions.
4. Perform controlled tests only in an isolated environment with no actuators connected and with its own safety regulations. PR #20 **Does not offer or contain a robot download script**.
5. Confirm observations with OEM IDE/controller images, repeated tests and references to the correct version of documentation; if evidence is insufficient, the condition remains`Unknown\`.
6. Only after a separate code review is it permissible to add the execution of a **private verified subset** to an isolated simulator. Authorization of native writer / HSES / FTP An independent ban and a separate phase.

## Acceptance criteria PR #20

- Synthetic Rust tests: exact M-names, missing/unconfirmed operands, PART/END, blocking on unknown/boolean, output limitation, absence of alias-outputs, byte immutability.
- The UI shows only the supposed text links and allows you to go to the source line; the absence of execution confirmation is visible without opening the documentation.
- CI Rust/TypeScript and Windows MSVC are successful; PR do not merge until the owner's permission.

## PR #21 Documentary semantics, without poster equivalence

The table of unknowns above reflects the state of **do** study YRC1000 R12 §12.2–12.3 и DX100 Rev.0. The EN=ON conditions, individual flag formulas and tables are now documented for all five teams in the world. [study](oem_5_opcode_analysis.en.md). This **not* translates a real CIOPRG into an executable subset and does not change `semantics_verified=false`, `scan_order_verified=false`.

## Further work after PR #29

[Roadmap v3](roadmap.en.md) offers parser fixtures, typed grammar PR #34) and source-linked static analysis (conditional analysis) PR #35). Current `ManualDerived` and M-link observations **do not confirm *real values of EN/Carry/Zero/Error during scan, System/User ownership or read-write hierarchy in hardware CIO. Up to independent OEM bench data results `Unresolved` и `NotExecuted`, а HSES/FTP/native writer — `BLOCKED`. [Current status](project_status.en.md).
