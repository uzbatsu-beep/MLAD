<!-- Generated en companion for docs/memory_map.md; review required. -->
> **English** · [Русский](memory_map.ru.md) · [English](memory_map.en.md) · [Original / Оригинал](memory_map.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.

# Memory Map – Archive of Preliminary Address Map

> **Archived index (reconciliation 18.11.2026):** This file is saved for references; do not use it as a valid specification. PR #1-#21 merged, the latest Windows release is rc.9. [Current status](project_status.en.md).

**Do not use earlier versions of this file as a source of address validity or write permission.**

The original MLAD prototype had shifted CIO classes (for example, `1xxxx` was labelled External Input and `2xxxx` External Output). For the verified controller-specific tables, use **[Typed memory model — DX100, DX200, YRC1000](typed_memory_model.en.md)**.

The implementation of these ranges, typed bit/byte/word/register addresses and tests lives at `src-tauri/src/compiler/memory.rs`.

## Key rules

- Set an explicit `ControllerFamily` for production validation: `DX100`, `DX200`, `YRC1000`.
- Relay bit address notation: `#XXXXX` with final digit `0..7`; channel `number/10`.
- Network I/O and pseudo input ranges differ materially between generations.
- The address-class table does **not** grant permission to write to an actual robot.
- M-registers are 16-bit `M000..M999`; role, ownership and instruction restrictions apply.
- Read the [CIOPRG.LST specification baseline](cioprg_lst_spec.en.md) before attempting a compiler.

Source index: [source registry](source_registry.en.md).

## UI status after PR #8

In the new MLAD tabs, the operator selects `DX100`, `DX200` or `YRC1000`; Rust uses a profile in `memory.rs` JSON/read-only inspector validators. The inline input front only checks the **format**, after which Rust checks the types and ranges. The Legacy JSON workflow maintains compatible limitations.

No successful range check means a permitted hardware record. See. [typed_memory_model.md](typed_memory_model.en.md), [architecture.md](architecture.en.md) и [documentation](README.en.md).
