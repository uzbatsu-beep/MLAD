# Ladder / CIOPRG.LST: superseded preliminary notes

The original prototype contained a fabricated `/JOB`-style output template and incomplete instruction examples. These are **not validated as the native Yaskawa Concurrent I/O file** and **must not be exported or deployed as CIOPRG.LST**.

Current source of truth: **[CIOPRG.LST specification baseline](cioprg_lst_spec.md)**.

Current requirements include lossless raw-file preservation, typed operands, correct `#XXXXX` and `W#XXXX0` syntax, controller-specific semantic validation, system/user boundary protection and fail-closed modified export.

The baseline documents what manufacturer sources establish and marks unknown file-level grammar explicitly. A complete verified grammar still requires untouched real-controller exports for each generation.

See also [typed memory model](typed_memory_model.md) and [source registry](source_registry.md).