<!-- Generated en companion for docs/oem_5_opcode_analysis.md; review required. -->
> **English** · [Русский](oem_5_opcode_analysis.ru.md) · [English](oem_5_opcode_analysis.en.md) · [Original / Оригинал](oem_5_opcode_analysis.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.

# PR #21 MOV, ADD, SUB, SHL, SHR: Yaskawa Concurrent I/O Study

> ** Status and applicability:** This document describes the relevant functional subset and evidence collected before/by PR #21It means that the whole project stopped at PR #21. Actual `main` later [PR #29](https://github.com/uzbatsu-beep/MLAD/pull/29) and independent levels of CI/UserSmoke/OEM: [project_status.md](project_status.en.md). Do not display permission for a native writer, HSES or performing a scan from the specification described herein.

** Status 18.11.2026:** PR #21 merged, Windows rc.9 published; documentary model, specific YRC1000/DX100/DX200 Not checked at the stand. Not a simulator or a native compiler.

## Primary sources and access restrictions

1. [YRC1000 Concurrent I/O R12, OEM index](https://knowledge.motoman.com/hc/en-us/articles/40670912825362-YRC1000-OPTIONS-INSTRUCTIONS-FOR-CONCURRENT-I-O-R12), document 178648-1CD. Direct PDF OEM download fails (HTTP 403); full-text tables are verified by [reproduction YRC1000 manual](https://manualzz.com/doc/84663174/yaskawa-motoman-yrc1000-owner-s-manual) §12.2.17 ADD (pp. 12-18), §12.2.18 SUB (12-19), §12.2.24 MOV (12-25), §12.2.29 SHL (12-30), §12.2.30 SHR (12-31), §12.3 (12-36).
2. [DX100 Concurrent I/O, RE-CKI-A453, Rev.0 / 155491-1CD, PDF](https://gab.wallawalla.edu/~ralph.stirling/classes/engr480/docs/Motoman/dx100_concurrent_io_155491-1CD.0.pdf): the relevant sections of §12.2/§12.3 confirm a similar general model. Specific controllers and firmware are not checked.
3. DX200 RE-CKI-A465: Remains an independent documentary gate, not to endure the rules of generations silently. For comparison of forms see [previous study](typed_opcode_grammar.en.md).

Marking: **Manual** - direct indication in the manual; **Derived** - calculated consequence of the documented formula; **Unknown** - no unambiguous information; **Bench** - control check (not yet).

## Five-team table

| Opcode | ON condition/result (Manual) | Carry / Zero / Error (Manual) | Unknown |
| --- | --- | --- | --- |
| MOV S,D | At EN=ON, the value of S is transferred to D; D = M000–M599 / byte / word, S = M000–M999 / 0–65535 / byte / word | Carry changes when transmitted to byte; the threshold text is greater than 256 (BCD). Zero/Error is not used (0). | Behavior at exactly 256; precise truncation byte |
| ADD S1,S2,D | D=(S1+S2) mod 65536; S1/S2 is unsigned 16-bit from M or 0..65535; D = M000. . M599 | Carry=1 for ≥65536; Zero=1 for D=0; Error not used (0) | EN and the flow of flags in full scan |
| SUB S1,S2,D | D=(S1−S2) mod 65536; similar roles | Carry=1 if S1<S2; Zero=1 if D=0; Error not used (0) | Appointment/scan conflicts |
| SHL S,n,D | Shift 16-bit S to the left, n=0..16, zero input from the bottom | Carry is ousted by MSB. Zero/Error not used (0) | Carry n=0/16, byte result |
| SHR S,n,D | Shift 16-bit S to the right, n=0..16, insert zeros from top | Carry is a displaced LSB. Zero/Error not used (0) | Carry n=0/16, byte result |

### Performance/flags in scan

§12.3 YRC1000: **#51400 Carry, #51401 Zero, #51402 Error** Before performing the Ladder of each scan, the flags are cleared; the instruction affecting the flag updates the result with a true EN, with a false EN, cleans the corresponding flag; another instruction that does not affect the flag does not change it.

SHL/SHR at EN=ON is executed **every scan**; for single action, management recommends using PLS/PLF in condition. For all five ON is a mandatory condition of execution. The conditions of the real chain and the order of execution of several MLAD commands are not yet confirmed.

It would be a mistake to ban reassignments in M globally: §12.2 allows multiple assignments of arithmetic instruction registers, but prohibits multiple access to current TMR/CNT registers. Duplication of specific relays also has limitations.

### Critical boundary cases

- MOV→byte at S=256. The words “more than 256 (BCD)” do not automatically allow the general PLC heuristic “>255”. The model returns Unknown for byte values from 256; verification of the actual truncation and flag is postponed.
- SHL/SHR at n=0: mathematical result S, but Carry is not determined by the model (no repressed bit). At n=16: 16-bit result is 0, but Carry and byte specificity remain Unknown. Misuse of Rust `wrapping_shl(16)` This would turn the shift into a 0 shift, so the branch 16 is treated separately.
- EN=OFF: result `NotExecuted`flags in a separate model left behind `Unresolved`. §12.3 describes clearing in the scan, but there is no complete model of how the instructions interact.
- Separately, byte/word aliasing, a specific firmware version, and a repeat M-assignment conflict are not solved.

## Autonomous implementation

`src-tauri/src/compiler/reference_semantics.rs` isolated pure function `evaluate_reference(ReferenceInput)`. Doesn't read. `CIOPRG.LST`It does not run Ladder and is not connected to Tauri IPC. All results tagged. `ManualDerived`, `Unresolved` either `NotExecuted`. Flags. `controller_execution_verified=false`, `scan_chain_verified=false`, `native_write_allowed=false` regardless of the source data.

Tests: six classes of arithmetic carry/borrow/zero, a complete search of 65536 ADD/SUB values with second operand 1, MOV byte 255/256/257, SHL/SHR 0/1/16/17, EN On/Off/Unknown, inappropriate purpose. No real robot programs are included in the tests.

In Native Workspace, the handbook of the selected command shows the format, rules, limitations, and unknown areas. **Do not* shows the "actually calculated" result of a real CIO string.

## Poster plan for the future

| Test ID | Verification | Confirmation |
| --- | --- | --- |
| EN-01 | EN = 0/1 for all five instructions | Absence of D at EN=0, behavior at EN=1 |
| SCAN-01 | Several consecutive instructions affecting flags | Reset/rewrite #51400/#51401/#51402 as part of the scan |
| ADD-01 | 0+0; 65535+0; 65535+1; 65535+65535 | Carry/Zero и wrap |
| SUB-01 | 0−0; 1−1; 0−1; 100−101 | Borrow и wrap |
| MOV-01 | 255, 256, 257, 65535 byte and word | Exact Carry boundary / truncation |
| SHIFT-01 | n = 0.1,8,15,16 for both directions | Last repressed bit, byte/word mode |
| GEN-01 | DX100, DX200, YRC1000: firmware/options/manual | Generational Differences and Independent OEM Confirmation |

Tests only on an isolated stand with the permission of the equipment owner. The application does not contain loading into the robot, FTP/HSES and a native writer. The result of the control check is not yet available.
