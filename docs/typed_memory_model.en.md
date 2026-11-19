<!-- MLAD-I18N:EDITORIAL:reviewed 2026-11-19 against typed_memory_model.md -->
> **Language / Язык:** [English](typed_memory_model.en.md) · [Русский](typed_memory_model.ru.md) · [Original](typed_memory_model.md)
>
> **Scope:** Source-aligned engineering explanation, **not** an independent OEM firmware audit. Source `CIOPRG.LST`/`IONAME.DAT` remain read-only. HSES/FTP, native writer and deployment are **BLOCKED**; OEMBench is **NOT RUN**.

# Typed CIO memory model — DX100 / DX200 / YRC1000

> **Status and scope:** this document describes the constrained feature subset and evidence available by PR #21; this **does not** imply the whole project stopped at PR #21. See the [current project status](project_status.en.en.md) for merged PR #29 and independent CI/UserSmoke/OEM evidence levels. This specification does **not** authorize a native writer, HSES or execution of a controller scan.

**Implementation:** [`src-tauri/src/compiler/memory.rs`](../src-tauri/src/compiler/memory.rs).
**Version:** 0.1 address baseline. The implementation in main was checked as of 18 November 2026, but **no new OEM approval for specific firmware** was obtained. Do not use these tables to authorize HSES writes or robot deployment.
**Evidence:** DX100 `RE-CKI-A453` §2 table 2-1; DX200 `RE-CKI-A465` §2 table 2-1; YRC1000 Concurrent I/O §2 table 2-1. See [source_registry.md](source_registry.en.md).

## 1. Addressing model

An individual CIO bit relay has exactly five decimal digits (canonical file notation `#XXXXX`), with the last decimal digit restricted to `0..7`. The least significant digit is the bit index in an eight-bit channel. Bits `8` and `9` never exist. For example `#70017` is the eighth bit of channel 7001; `#70018` is invalid.

A prefix such as `#0...` must retain **all five digits**. The integer representation is only an internal key and is never used to reformat protected raw files.

## 2. Exact controller-family relay ranges

Both endpoints are **inclusive**. The final digit of all range limits is 0–7.

| CIO class | DX100 | DX200 | YRC1000 |
|---|---:|---:|---:|
| General Input (CIO → job) | 00010–02567 | 00010–05127 | 00010–05127 |
| General Output (job → CIO) | 10010–12567 | 10010–15127 | 10010–15127 |
| External Input | 20010–22567 | 20010–25127 | 20010–25127 |
| External Output | 30010–32567 | 30010–35127 | 30010–35127 |
| Specific Input (command toward robot) | 40010–41607 | 40010–41607 | 40010–42567 |
| Specific Output (robot status) | 50010–52007 | 50010–53007 | 50010–55127 |
| Interface Panel Input | 60010–60647 | 60010–60647 | 60010–60647 |
| Auxiliary Relay | 70010–79997 | 70010–79997 | 70010–79997 |
| Control Status | 80010–80647 | 80010–82007 | 80010–85127 |
| Pseudo Input | 82010–82207 | 82010–82207 | 87010–87207 |
| Network Input | 25010–27567 | 27010–29567 | 27010–29567 |
| Network Output | 35010–37567 | 37010–39567 | 37010–39567 |

**Non-interchangeable cases:**
- `#82010`: DX100/DX200 `PseudoInput`, YRC1000 `ControlStatus`.
- `#87010`: invalid under DX100/DX200 profile; `PseudoInput` under YRC1000.
- `#25010`: `NetworkInput` in DX100 but `ExternalInput` in DX200/YRC1000.
- `#37010`: `NetworkOutput` in all three; it is *not* an ordinary `ExternalOutput`.

Range membership is **not** evidence of installed I/O modules or permitted system command. Multiple network cards/ladder allocations must be checked against the actual robot configuration.

## 3. M-registers: 16-bit values, class ≠ write permission

The documented CIO register space for these three families is `M000–M999` (1,000 registers). **Do not assume M1000–M3999 exists in CIO** based on unrelated robot variable facilities or other controller products.

| Register class | Range | Semantics / restrictions |
|---|---|---|
| General | M000–M559 | General-purpose 16-bit CIO register; writes are instruction- and ownership-dependent |
| Analog Output | M560–M599 | Analog-output backing register; modifications require configuration review |
| Analog Input | M600–M639 | Analog-input measurement; disallow ordinary destination writes |
| System | M640–M999 | Controller/system-held values; do not grant generic write access |

Yaskawa gives individual instructions different source/destination constraints; for example timer and counter current-value destinations may be limited to general registers even though their set values can reference a wider source range.

The header `//$M` uses **M480–M559** in eight descending groups. A register that is generally writable may still be application-reserved; `//$M` values and system/user ownership must be preserved.

## 4. Rust types and validation responsibilities

```text
ControllerFamily = DX100 | DX200 | YRC1000
RelayBitAddress  = number + channel + bit + RelayClass
RelayGroupAddress = first_bit + (Byte8 | Word16)
RegisterAddress = number + RegisterClass
LadderWritePolicy = Denied | Candidate | RequiresReview
```

- `parse_relay(controller, "#70010")` checks five ASCII decimal digits, bit 0–7, and the exact controller table.
- `parse_register("M256")` checks `M` and exactly three ASCII digits, then classifies it.
- `parse_group(controller, "#70010", Byte8)` / `parse_group(controller, "W#70010", Word16)` distinguishes group widths. It checks bit-zero start but **does not yet validate** per-instruction word alignment, bank crossings or instruction-specific acceptable operand classes.
- `relay_write_policy` is deliberately conservative: certain classes are denied as `OUT` targets, and specific control inputs require review. **Candidate is not permission to write to hardware**, only that the generic category may be the destination of some ladder instructions.
- System Ladder ownership is not inferred from the address. A user program may reference system signals without owning their definitions. **System vs User** is a file/program region property, not a relay address property.

The legacy validator `validate_rungs` assumes YRC1000 for backward compatibility; callers handling DX100/DX200 **must** use `validate_rungs_for_controller(rungs, family)`. **The current UI already allows DX100/DX200/YRC1000 selection** in Graph Workspace, Advanced CIO and Inspector. The older JSON interface retains its legacy limitations. Selecting a controller family **does not** automatically specify the firmware, installed options, physical configuration or permission to write to a robot; export and deployment remain prohibited.

## Current controller-profile integration after PR #8

- `graph_draft.rs` validates JSON addresses and output classes against the selected controller family.
- `advanced.rs` additionally restricts TMR/CNT current-value registers to M000–M559, presets to M000–M999 or 0–65535, and `GSTR/GOUT` operands to byte groups starting at bit 0.
- `inlineOperand.ts` performs **preliminary syntax validation only** in the frontend; Rust validates controller-specific ranges. Successful inline entry is **not** hardware verification.
- `lossless.rs` and `semantic.rs` use the explicitly selected controller profile while reading the original `CIOPRG.LST`, leaving unrecognized fragments unchanged.
- Exact model/firmware/application properties and physical I/O constraints still require verification with authorized controller exports and manufacturer documentation.

## 5. Validation checklist

1. Select controller family before examining ranges.
2. Check syntax and bit index; `#70018` / `#70019` are invalid, even if within a broad numeric range.
3. Check exact controller-specific range boundaries; gaps are not valid relays.
4. Classify bit/byte/word/register separately; `W#70010` must not be read as a bit or an M-register.
5. Apply instruction-specific operand schema, source/destination and stateful behavior.
6. Apply project-level ownership and robot configuration constraints.
7. Diagnose duplicate coils, unsafe destinations and unresolved instructions.
8. For any modified CIOPRG file, block export/deployment until independent golden-file verification succeeds.

Unit tests for this baseline reside directly in `memory.rs`. They cover boundaries, generation differences, malformed addresses, pseudo inputs and network I/O.

## 6. Unresolved until controller-backed fixtures

- Instruction-specific write permissions for particular `SpecificInput` points; no blanket authorization.
- Register reserved use by technology (welding/handling/etc.).
- `W#` alignment and composite operand range crossing.
- Firmware/option-dependent addresses and absent I/O hardware.
- Extended or nonstandard CIO address spaces, if separately documented for particular revisions.
- Full instructions/semantics and port permissions for HSES (separate networking workstream).

See [CIOPRG.LST baseline](cioprg_lst_spec.en.md) for the import/export contract.
