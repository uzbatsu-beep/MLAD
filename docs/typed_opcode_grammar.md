# MLAD — Typed Candidate forms for native CIOPRG.LST (PR #19)

> **Сверка статуса 18.11.2026:** в `main` включены PR #1–#21, Windows `rc.9` опубликован и CI успешен. Данный материал относится к ограниченному техническому подмножеству; аппаратная эквивалентность не подтверждена. [Актуальный статус проекта](project_status.md).

**Status (18.11.2026):** PR #19 merged; partial read-only instruction grammar; **not** a verified controller compiler, executor or writer.

## Primary sources and evidence status

1. Official [Yaskawa YRC1000 Concurrent I/O instructions R12 (2026-11-11)](https://knowledge.motoman.com/hc/en-us/articles/40670912825362-YRC1000-OPTIONS-INSTRUCTIONS-FOR-CONCURRENT-I-O-R12): index to instruction descriptions, §12.2.17 ADD, §12.2.18 SUB, §12.2.24 MOV, §12.2.29 SHL, §12.2.30 SHR.
2. Official [YRC1000 Ladder Editor operator manual HW1483388](https://www.motoman.com/getmedia/7DCAB851-59B3-4698-97FB-AE39E45FD106/179482-1CD): §1.3.3 lists Calc ADD/SUB, Logic MOV and Shift SHL/SHR; this does **not** itself establish source-file grammar.
3. DX200 Concurrent I/O manual RE-CKI-A465 §12.1–12.2: operand tables are described in the repository [source registry](source_registry.md), and should be independently cross-checked on OEM exports. No assertion of cross-firmware runtime equivalence is made.
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
