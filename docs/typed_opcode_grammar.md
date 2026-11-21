<!-- MLAD-I18N:NAV:START -->
> 🌐 **Язык / Language:** [Русский](typed_opcode_grammar.ru.md) · [English](typed_opcode_grammar.en.md) · [Original / Исходный текст](typed_opcode_grammar.md). **Machine translation / Машинный перевод: editorial review required / необходима редакторская проверка.**
<!-- MLAD-I18N:NAV:END -->

> **Актуальная ограниченная интерпретация (21.11.2026):** `main` `9fcd11d61856c8a559d9acf3c77bd662ad814dfe`, Windows pre-release `v0.5.0-rc.10-unified-offline`. [Аудит Git/MD](repository_audit_2026-11-21.md) фиксирует базу релиза, но не подтверждает реальные права памяти или scan. `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`; native writer, `HSES`/`FTP`, deployment — `BLOCKED`.

# MLAD — Typed Candidate forms for native CIOPRG.LST (PR #19)

> **Статус и применимость:** этот документ описывает соответствующее функциональное подмножество и доказательства, собранные до/по PR #21; он **не** означает, что весь проект остановился на PR #21. Актуальный `main` после [PR #29](https://github.com/uzbatsu-beep/MLAD/pull/29) и независимые уровни CI/UserSmoke/OEM: [project_status.md](project_status.md). Не выводить разрешение на native writer, HSES или исполнение скана из описанной здесь спецификации.

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

## PR #45 (20.11.2026) — расширенная типизация по YRC1000 Concurrent I/O R12

**Актуальная база:** PR #44 слит в `main` `48522c1e2d0ee41560ce61ba0d2aa8af14e1c6d8`. PR #45 открыт Draft. Источником новых форм является документ Yaskawa YRC1000 Concurrent I/O **RE-CKI-A467 / 178648-1CD, R12**, [официальный указатель](https://knowledge.motoman.com/hc/en-us/articles/40670912825362-YRC1000-OPTIONS-INSTRUCTIONS-FOR-CONCURRENT-I-O-R12) и доступное [текстовое воспроизведение раздела 12.2](https://www.manualzz.com/doc/84663174/yaskawa-motoman-yrc1000-owner-s-manual). Воспроизведение третьей стороной — источник проверки таблиц, не сертификат редактора или контроллера; независимый OEMBench **NOT RUN**.

| Семейство | Новые мнемоники | Разделы OEM | Строго подтверждённая форма синтаксиса |
| --- | --- | --- | --- |
| Арифметика | `MUL` | §12.2.19 | `MUL S1,S2,Dn`: S1/S2=M000–M999 или 0–65535; результат Dn/Dn+1 — два M; MLAD консервативно требует Dn≤M598 |
| Арифметика | `DIV`, `MOD` | §§12.2.20–21 | `S1,S2,D`: S1/S2=M000–M999 или 0–65535; D=M000–M599 |
| Преобразование | `BIN`, `BCD` | §§12.2.22–23 | `S,D`: S=M000–M999 / relay byte / word, **не immediate**; D=M000–M599 / relay byte / word |
| Битовая логика слова | `WAND`, `WOR`, `WXOR` | §§12.2.25–27 | `S1,S2,D`: S1/S2=регистр, 0–65535, byte/word; D=регистр M000–M599 либо byte/word |
| Битовая логика слова | `WNOT` | §12.2.28 | `S,D` с теми же типами S/D |
| Циклический сдвиг | `ROL`, `ROR` | §§12.2.31–32 | `S,n,D`: S=регистр/0–65535/byte/word; n — только 0..16; D=регистр/byte/word |
| Копирование байтов | `BMOV` | §12.2.33 | `S,n,D`: S=M000–M999 или relay byte; n — **1..999 байт**; D=M000–M599 или relay byte, без word-групп |
| Сравнение | `CMP` | §12.2.34 | `S1,S2`: два источника, регистр/0–65535/relay byte/word; **нет D** |
| Импульсы | `PLS`, `PLF` | §§12.2.15–16 | ровно один битовый `#XXXXX`, не M и не byte/word |

**Граница реализации:** все 15 новых форм — только `LineKind::TypedCandidate`, `semantics_verified=false`. Валидное расположение операндов не подтверждает EN, флаги Carry/Zero/Error, допустимость записи, scan-order, границы последовательного диапазона BMOV или системные ограничения. Старые пять `MOV/ADD/SUB/SHL/SHR` сохраняют предыдущую типизацию. Новые формы имеют `controller_form_verified=true` **только для YRC1000 в смысле прочитанного руководства**, не реального исполнения. Для DX100/DX200 новые формы остаются `controller_form_verified=false` даже если лексически подходят. Не переносить R12 на другую прошивку без OEM gate.

**Негативные проверки:** 15 положительных и 17 отрицательных оригинальных SYNTHETIC строк, предел `MUL Dn+1`, `BIN/BCD` immediate, `BMOV` word/count, `CMP` неверное число аргументов, `PLS/PLF` адресные формы, прочность byte spans/SHA/EOL/BOM/non-UTF8 и post-END. Статическая семантика обязана видеть новый кандидат как разрыв исполняемого потока; нативные `CIOPRG.LST` и `IONAME.DAT` остаются read-only. `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`; HSES/FTP, native writer и deployment **BLOCKED**.
