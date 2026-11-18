<!-- Generated en companion for docs/post_end_data.md; review required. -->
> **English** · [Русский](post_end_data.ru.md) · [English](post_end_data.en.md) · [Original / Оригинал](post_end_data.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.

# MLAD — read-only post-END data classification and diagnostics

**Scope on 18.11.2026:** PR #18 **merged**, [Windows rc.6 published](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.6-postend); support is included in the current `main`. Describes the observed post-END format, **not** full OEM grammar/editor. [Current status](project_status.en.md).

## Observed exports, not assumed OEM specification

In private controller backups, a syntactically valid standalone `END` is followed by human-readable alarm/message data. Some observed section markers are `//ALARM1`, `//ALARM2`, `//MESSAGE1` and `//MESSAGE2`.

This observation does **not** establish a universal grammar, firmware compatibility, write access, or an executable semantics contract. No genuine controller exports have been committed as public test fixtures.

## Parsing rules

1. The complete source is first scanned **byte-for-byte** with offsets and exact CRLF/LF/CR terminators.
2. Before a syntactically valid `END`, the existing partial instruction parser remains unchanged, including detection of unsupported mnemonics.
3. After `END`, lines are no longer treated as executable Ladder:
   - **PostEndSection**: case-sensitive `//ALARM` or `//MESSAGE` followed by 1–8 ASCII decimal digits; keep the original marker text.
   - **PostEndData**: uninterpreted bytes inside an observed section (and blank lines).
   - **PostEndOpaque**: text outside an observed section, unknown directives, or bytes containing NUL. Retain the exact bytes, emit one representative warning `PostEndUnknownStructure` (rather than one per data record).
4. All source lines remain pageable, searchable, and byte-identical. These classifications are **display overlays only**.
5. The semantic analyzer skips recognized post-END section/data lines, but an unknown post-END region generates a limited `AfterEnd` warning and keeps `subset_analysis_complete=false`. It never fabricates extra rungs.
6. A malformed `END operand` does not start a data region. Missing or ambiguous END is not silently repaired.

## UI and acceptance

The status strip shows first valid END line, the number of post-END lines, recognized marker count and unverified lines. The source pane uses separate categories, and the diagnostic panel groups by severity and code; opening a group reveals **every returned** individual source-line warning. The current backend still limits diagnostic response counts and the UI must state this explicitly.

- Synthetic Rust fixtures test mixed line endings, embedded non-UTF8 bytes, unknown/malformed markers, malformed END, and metadata rows whose text resembles Ladder instructions.
- TypeScript tests check that diagnostic grouping never drops line references, including 525 repeated warnings.
- **Safety invariant:** native CIOPRG/IONAME edits, FTP/HSES, controller upload and OEM-equivalence claims remain prohibited.

## Known limits

This change prevents a documented class of false positives; it does not fix other unsupported opcodes (`MOV`, `ADD`, `SUB`, `SHL`, etc.), establish firmware-specific memory mapping, grant System/User modification rights or guarantee diagnostic completeness. Each supported controller family still requires private source/correctness comparison with OEM Ladder Editor.

## Next verification cycle (roadmap v3)

[Proposed PR #31 synthetic parser corpus](implementation_plan.en.md) must include `PART/END` combinations and synthetic `//ALARM`/`//MESSAGE` tail records with exact source spans and no fake Ladder interpretation. Findings from private exports **do not establish universal OEM grammar**. Main after PR #29 uses unchanged read-only parser boundaries; no native export or robot deployment is authorized. [Status](project_status.en.md).
