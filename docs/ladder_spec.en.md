<!-- Generated en companion for docs/ladder_spec.md; review required. -->
> **English** · [Русский](ladder_spec.ru.md) · [English](ladder_spec.en.md) · [Original / Оригинал](ladder_spec.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.

# Ladder / CIOPRG.LST Archive of Preliminary Specification

> **Historical document (navigation mark 18.11.2026):** The following conclusions preserve the date of the initial verification, and do not replace the current implementation. PR #1#21 merged, Windows rc.9 published. [Current status of MLAD](project_status.en.md).

The original prototype contained a fabricated `/JOB`-style output template and incomplete instruction examples. These are **not validated as the native Yaskawa Concurrent I/O file** and **must not be exported or deployed as CIOPRG.LST**.

Current source of truth: **[CIOPRG.LST specification baseline](cioprg_lst_spec.en.md)**.

Current requirements include lossless raw-file preservation, typed operands, correct `#XXXXX` and `W#XXXX0` syntax, controller-specific semantic validation, system/user boundary protection and fail-closed modified export.

The baseline documents what manufacturer sources establish and marks unknown file-level grammar explicitly. A complete verified grammar still requires untouched real-controller exports for each generation.

See also [typed memory model](typed_memory_model.en.md) and [source registry](source_registry.en.md).

## Status at 17.11.2026

Since the creation of this file in `main` combined read-only `lossless.rs`, `semantic.rs`A limited native inventory of TMR/CNT/GSTR/GOUT and two independent JSON editors. It doesn't change the old pattern. `/JOB` grammar `CIOPRG.LST`. For entering addresses on the graph is used [inline editing](inline_address_editing.en.md) - Only in JSON.

Sources and status: [complete map](README.en.md), [basic specification](cioprg_lst_spec.en.md) и [architecture](architecture.en.md).
