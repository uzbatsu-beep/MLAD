<!-- MLAD-I18N:EDITORIAL:reviewed 2026-11-19 against oem_5_opcode_analysis.md -->
> **Language / Язык:** [English](oem_5_opcode_analysis.en.md) · [Русский](oem_5_opcode_analysis.ru.md) · [Original](oem_5_opcode_analysis.md)
>
> **Evidence:** A translation of MLAD's documented reference analysis. `ManualDerived` is **not** controller-executed behavior. `CIOPRG.LST`/`IONAME.DAT` read-only; HSES/FTP/native writer/deployment **BLOCKED**; OEMBench **NOT RUN**.

> **Current semantic confidence (21 November 2026):** `main` `9fcd11d61856c8a559d9acf3c77bd662ad814dfe`; released Windows pre-release `v0.5.0-rc.10-unified-offline`. See the [dated Git audit](repository_audit_2026-11-21.en.md) for currently supported parser and source-linked roles. These references do **not** validate robot scan, flag effects or write permissions: `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`; native writer, `HSES`/`FTP` and deployment `BLOCKED`.

# PR #21 — MOV, ADD, SUB, SHL, SHR: Yaskawa Concurrent I/O reference analysis

> **Status and scope:** this document records the constrained forms and evidence available by PR #21; it does **not** imply the project stopped at PR #21. The current `main` status after [PR #29](https://github.com/uzbatsu-beep/MLAD/pull/29) and separate CI/UserSmoke/OEM levels are in [project status](project_status.en.md). No permission for a native writer, HSES or actual scan execution follows from this analysis.

**Status on 18 November 2026:** PR #21 merged and Windows rc.9 published. This is a manual-derived reference model. **None of the relevant DX100, DX200 or YRC1000 firmware/controller combinations were independently bench tested.** It must not be described as a full simulator or native compiler.

## Primary sources and access limitations

1. [YRC1000 Concurrent I/O R12, official OEM index](https://knowledge.motoman.com/hc/en-us/articles/40670912825362-YRC1000-OPTIONS-INSTRUCTIONS-FOR-CONCURRENT-I-O-R12), document 178648-1CD. Direct OEM PDF retrieval was unsuccessful (HTTP 403); full-text tables were compared against a [reproduction of the YRC1000 manual](https://manualzz.com/doc/84663174/yaskawa-motoman-yrc1000-owner-s-manual): §12.2.17 ADD (p.12-18), §12.2.18 SUB (p.12-19), §12.2.24 MOV (p.12-25), §12.2.29 SHL (p.12-30), §12.2.30 SHR (p.12-31), §12.3 (p.12-36).
2. [DX100 Concurrent I/O, RE-CKI-A453, Rev.0 / 155491-1CD, PDF](https://gab.wallawalla.edu/~ralph.stirling/classes/engr480/docs/Motoman/dx100_concurrent_io_155491-1CD.0.pdf): corresponding sections §12.2/§12.3 describe a similar general model. Specific controllers and firmware have not been tested.
3. DX200 RE-CKI-A465 remains a **separate documentary verification gate**; never silently assume equivalence across generations. See the [preceding operand-form analysis](typed_opcode_grammar.en.md).

Evidence labels: **Manual** = directly stated in a manual; **Derived** = mathematical result derived from a documented formula; **Unknown** = insufficient or ambiguous evidence; **Bench** = actual controller verification (**none available yet**).

## Five-instruction reference matrix

| Opcode | Enabled operation / result (Manual) | Carry / Zero / Error (Manual) | Unknown or unresolved |
| --- | --- | --- | --- |
| MOV S,D | With EN=ON, S is transferred to D. D: M000–M599 / byte / word; S: M000–M999 / 0–65535 / byte / word | Carry changes when moving to a byte; the manual wording is "more than 256 (BCD)". Zero/Error are unused (0) | Exact behavior **at 256** and byte truncation |
| ADD S1,S2,D | `D=(S1+S2) mod 65536`; S1/S2 are unsigned 16-bit M values or 0..65535; D: M000..M599 | Carry=1 when the sum is **≥65536**; Zero=1 if D=0; Error unused (0) | Interacting EN conditions and flag flow during the complete scan |
| SUB S1,S2,D | `D=(S1−S2) mod 65536`; corresponding operand roles | Carry=1 when **S1<S2**; Zero=1 when D=0; Error unused (0) | Multiple destinations and scan ordering |
| SHL S,n,D | Shift 16-bit S left by n=0..16, inserting zeros at the lower end | Carry is the shifted-out **MSB**; Zero/Error unused (0) | Carry for n=0/16; byte destination details |
| SHR S,n,D | Shift 16-bit S right by n=0..16, inserting zeros at the upper end | Carry is the shifted-out **LSB**; Zero/Error unused (0) | Carry for n=0/16; byte destination details |

### Execution and flags during a controller scan

According to YRC1000 §12.3, flag addresses are **#51400 Carry, #51401 Zero, #51402 Error**. The manual describes clearing flags before the Ladder scan; a flag-affecting instruction updates its corresponding flag for a true EN and clears that flag for a false EN. An instruction that does **not** affect a given flag leaves it unchanged.

With EN=ON, SHL/SHR execute **on every scan**; for a single-shot effect, the manual recommends PLS/PLF in the enable condition. All five instructions require their ON condition for execution. MLAD does **not** yet verify the real rung conditions or ordering of multiple instructions.

A blanket ban on repeated M-register destinations would be incorrect: §12.2 allows repeated assignments to registers by arithmetic instructions, but restricts repeated use of the current-value registers of TMR/CNT. Individual relay destinations also have restrictions.

### Critical edge cases

- **MOV→byte with S=256.** The phrase "more than 256 (BCD)" does **not** justify silently applying generic PLC behavior at `>255`. The reference model returns **Unknown for byte values from 256**; actual truncation and Carry behavior remain bench work.
- **SHL/SHR at n=0:** mathematically the result is S, but Carry remains unresolved since no bit is displaced. **At n=16:** the 16-bit result is zero, while Carry and byte-specific behavior remain Unknown. Calling Rust `wrapping_shl(16)` without a separate branch would incorrectly reduce the shift to zero; the reference code handles 16 explicitly.
- **EN=OFF:** the result is `NotExecuted`, and isolated reference flags remain `Unresolved`. Section §12.3 describes per-scan flag clearing, but a complete model of instruction interactions does not exist.
- Byte/word aliasing, firmware-specific behavior and multiple M-register destination conflicts require independent resolution.

## Offline reference implementation

`src-tauri/src/compiler/reference_semantics.rs` contains the isolated, pure `evaluate_reference(ReferenceInput)` function. It does **not** read `CIOPRG.LST`, execute Ladder logic or connect to Tauri IPC. Outputs are labelled `ManualDerived`, `Unresolved` or `NotExecuted`. Regardless of input, `controller_execution_verified=false`, `scan_chain_verified=false`, `native_write_allowed=false`.

Tests cover six carry/borrow/zero arithmetic classes, an exhaustive 65,536-input ADD/SUB sweep with the other operand set to 1, MOV byte at 255/256/257, SHL/SHR at 0/1/16/17, EN On/Off/Unknown and invalid destinations. **No actual robot programs** are included in the tests.

Native Workspace presents the reference forms, documented rules, limitations and unknown areas for a selected instruction. It does **not** display a supposedly "actually computed" value for a native CIO source line.

## Future OEM bench plan

| Test ID | Scenario | Evidence to establish |
| --- | --- | --- |
| EN-01 | EN=0/1 for all five instructions | No D write with EN=0 and correct operation with EN=1 |
| SCAN-01 | Consecutive flag-affecting instructions | Clearing/overwriting #51400/#51401/#51402 within a scan |
| ADD-01 | 0+0; 65535+0; 65535+1; 65535+65535 | Carry/Zero and modulo wrap |
| SUB-01 | 0−0; 1−1; 0−1; 100−101 | Borrow and modulo wrap |
| MOV-01 | 255, 256, 257, 65535 in byte and word | Exact Carry threshold and truncation |
| SHIFT-01 | n=0,1,8,15,16 in both directions | Last shifted-out bit and byte/word behavior |
| GEN-01 | DX100, DX200, YRC1000 firmware/options/manual | Generation differences and independent OEM evidence |

These tests require an **owner-authorized, isolated rig**. MLAD contains no robot deployment, FTP/HSES control or native writer. **No controller bench result is available at this stage.**
