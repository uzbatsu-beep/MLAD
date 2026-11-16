# Memory map: superseded preliminary notes

**Do not use earlier versions of this file as a source of address validity or write permission.**

The original MLAD prototype had shifted CIO classes (for example, `1xxxx` was labelled External Input and `2xxxx` External Output). For the verified controller-specific tables, use **[Typed memory model — DX100, DX200, YRC1000](typed_memory_model.md)**.

The implementation of these ranges, typed bit/byte/word/register addresses and tests lives at `src-tauri/src/compiler/memory.rs`.

## Key rules

- Set an explicit `ControllerFamily` for production validation: `DX100`, `DX200`, `YRC1000`.
- Relay bit address notation: `#XXXXX` with final digit `0..7`; channel `number/10`.
- Network I/O and pseudo input ranges differ materially between generations.
- The address-class table does **not** grant permission to write to an actual robot.
- M-registers are 16-bit `M000..M999`; role, ownership and instruction restrictions apply.
- Read the [CIOPRG.LST specification baseline](cioprg_lst_spec.md) before attempting a compiler.

Source index: [source registry](source_registry.md).