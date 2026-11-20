<!-- MLAD-I18N:EDITORIAL:reviewed 2026-11-19 against typed_opcode_grammar.md -->
> **Language / Язык:** [English](typed_opcode_grammar.en.md) · [Русский](typed_opcode_grammar.ru.md) · [Original](typed_opcode_grammar.md)
>
> **Evidence:** Syntax-only `TypedCandidate` forms; not verified runtime behavior. `CIOPRG.LST`/`IONAME.DAT` read-only; HSES/FTP/native writer/deployment **BLOCKED**; OEMBench **NOT RUN**.

# MLAD — Typed Candidate forms for native CIOPRG.LST (PR #19)

> **Status and applicability:** this describes the constrained feature subset and evidence available by PR #21, **not** a claim that all MLAD development stopped at PR #21. See [current project status](project_status.en.md) for `main` after [PR #29](https://github.com/uzbatsu-beep/MLAD/pull/29) and the distinct CI/UserSmoke/OEM evidence levels. No native writer, HSES or scan execution permission follows from this specification.

**Status (18.11.2026):** PR #19 merged; partial read-only instruction grammar; **not** a verified controller compiler, executor or writer.

## Primary sources and evidence status

1. Official [Yaskawa YRC1000 Concurrent I/O instructions R12 (2026-11-11)](https://knowledge.motoman.com/hc/en-us/articles/40670912825362-YRC1000-OPTIONS-INSTRUCTIONS-FOR-CONCURRENT-I-O-R12): index to instruction descriptions, §12.2.17 ADD, §12.2.18 SUB, §12.2.24 MOV, §12.2.29 SHL, §12.2.30 SHR.
2. Official [YRC1000 Ladder Editor operator manual HW1483388](https://www.motoman.com/getmedia/7DCAB851-59B3-4698-97FB-AE39E45FD106/179482-1CD): §1.3.3 lists Calc ADD/SUB, Logic MOV and Shift SHL/SHR; this does **not** itself establish source-file grammar.
3. DX200 Concurrent I/O manual RE-CKI-A465 §12.1–12.2: operand tables are described in the repository [source registry](source_registry.en.md), and should be independently cross-checked on OEM exports. No assertion of cross-firmware runtime equivalence is made.
4. Five genuine robot exports were reviewed privately for **only the inventory of operand categories**; their filenames, addresses, raw instructions, unique fingerprint values and private metadata are not added to the public project. They match the subset of shapes chosen below, but controller firmware and independent OEM syntax-check outcomes remain unknown.

### Operand shapes (syntactic candidate only)

| Mnemonic | Form | Source roles | Destination | Read-only lexical restrictions |
| --- | --- | --- | --- | --- |
| `MOV` | `MOV S,D` | `M000..M999`, `0..65535`, `#XXXX0` byte, `W#XXXX0` word | `M000..M599`, byte/word | No byte-word coercion; all spans exact |
| `ADD` / `SUB` | `OP S1,S2,D` | two `M000..M999` or `0..65535` | `M000..M599` | 16-bit numeric format only |
| `SHL` / `SHR` | `OP S,n,D` | `S` as MOV; `n` decimal `0..16` | `D` as MOV | `n` cannot be `Mxxx` in this subset |

`#XXXX0` and `W#XXXX0` are **not** bit relays. The existing controller-aware `parse_group` verifies byte/word lexical shape and configured address class boundaries; this is *not* a proof that hardware exists or that the destination can be written in a given application.

### How typed candidates work

- The lossless reader retains original bytes, original line endings, byte spans, leading zeros and checksum. It constructs a `LineKind::TypedCandidate` with typed `AstOperand` records (no regeneration).
- `syntax_valid=true` means only that the limited **operand shape** matches the table. The JSON always sets `semantics_verified=false`. For DX100, `controller_form_verified=false` and the shape is presented as unverified for that family.
- Every candidate is an **explicit barrier** to the boolean/dataflow analyzer (`TypedOpcodeSemanticsPending`). It must never create a fake coil, dependency, flag value, simulator value, or compiled CIO file.
- `unsupported_lines` remains separate from `typed_candidate_count`, `typed_valid_forms`, `typed_invalid_forms`, `typed_mnemonic_counts`, and confirmed `recognized_instructions`.
- The frontend shows aggregate per-opcode counts, and when a source line is selected, typed operands and status of its *shape*.
- Malformed counts, excessive operands, unverified tokens and illegal destination ranges receive diagnostics while retaining raw source. Native data after `END` are never typed.

### Remaining verification gates

1. Compare privately against real exported CIO files from different controller families and firmware. Preserve confidentiality; no real files in public fixtures.
2. Obtain independent OEM syntax-check confirmation for DX100, DX200 and YRC1000 and exact operational semantics of each family.
3. Check scan conditions, arithmetic flags, duplicate destination policies and interactions with TMR/CNT, GSTR/GOUT, interlocks and system ladder.
4. Only after independent validation can executable semantic nodes or diagram representations be discussed. Even then, native writing/upload remains a separate restricted milestone.

**Safety:** original CIOPRG.LST and IONAME.DAT immutable, edited native export disabled, HSES/FTP/robot upload disabled, System/User boundaries unverified.

## PR #45 (20 November 2026) — extended YRC1000 Concurrent I/O R12 typed grammar

**Current base:** PR #44 merged into `main` `48522c1e2d0ee41560ce61ba0d2aa8af14e1c6d8`; PR #45 is a separate Draft. The evidence is Yaskawa YRC1000 Concurrent I/O **RE-CKI-A467 / 178648-1CD R12**, its [official manufacturer index](https://knowledge.motoman.com/hc/en-us/articles/40670912825362-YRC1000-OPTIONS-INSTRUCTIONS-FOR-CONCURRENT-I-O-R12), and an accessible [third-party transcription of §12.2](https://www.manualzz.com/doc/84663174/yaskawa-motoman-yrc1000-owner-s-manual). The transcription permits checking the forms but is not an OEM editor, firmware, or hardware certification. `OEMBench=NOT RUN`.

| Family | New mnemonics | OEM sections | Limited verified operand shape |
| --- | --- | --- | --- |
| Arithmetic | `MUL` | §12.2.19 | `MUL S1,S2,Dn`: S1/S2=M000–M999 or 0–65535; two-register result Dn/Dn+1, conservatively require Dn≤M598 |
| Arithmetic | `DIV`, `MOD` | §§12.2.20–21 | `S1,S2,D`: S1/S2=M000–M999 or 0–65535; D=M000–M599 |
| Conversion | `BIN`, `BCD` | §§12.2.22–23 | `S,D`: S=M000–M999, relay byte or word, **not immediate**; D=M000–M599, byte or word |
| Word bitwise | `WAND`, `WOR`, `WXOR` | §§12.2.25–27 | `S1,S2,D`: register, 0–65535, byte or word sources; register M000–M599, byte or word destination |
| Word bitwise | `WNOT` | §12.2.28 | `S,D` with the same source/destination types |
| Rotate | `ROL`, `ROR` | §§12.2.31–32 | `S,n,D`: register/0–65535/byte/word S; decimal n=0..16; register/byte/word D |
| Byte transfer | `BMOV` | §12.2.33 | `S,n,D`: S=M000–M999 or relay byte; n=**1..999 bytes**; D=M000–M599 or relay byte, no word groups |
| Comparison | `CMP` | §12.2.34 | `S1,S2`: two sources, register/0–65535/relay byte/word, **no D** |
| Pulse output | `PLS`, `PLF` | §§12.2.15–16 | one bit relay `#XXXXX`, not M or byte/word |

**Implementation boundary:** all 15 additions are display-only `LineKind::TypedCandidate`, `semantics_verified=false`. Matching operand spelling is not proof of EN, Carry/Zero/Error, scan order, writeability, BMOV range overflow or system-section privileges. The existing five `MOV/ADD/SUB/SHL/SHR` retain their previous typing. The new `controller_form_verified=true` is **YRC1000 manual-form evidence only**, never physical controller execution; for DX100 and DX200 the new forms remain `false`, even when lexically valid.

**Regressions:** 15 positive and 17 negative original SYNTHETIC instructions, MUL destination pair, BIN/BCD immediate, BMOV count/word exclusion, CMP arity, PLS/PLF bit-only, byte spans/SHA/EOL/BOM/non-UTF8, post-END and static-analysis barriers. Native `CIOPRG.LST` and `IONAME.DAT` remain read-only. `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`; HSES/FTP, native writer, deployment **BLOCKED**.
