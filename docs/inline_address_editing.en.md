<!-- Generated en companion for docs/inline_address_editing.md; review required. -->
> **English** · [Русский](inline_address_editing.ru.md) · [English](inline_address_editing.en.md) · [Original / Оригинал](inline_address_editing.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.
> **MLAD safety invariants:** `CIOPRG.LST` and `IONAME.DAT` are strictly **read-only**. FTP/HSES, live I/O, native writer and robot deployment remain **BLOCKED**. Passing `WindowsCI` does **not** imply `UserSmoke=PASS` or `OEMBenchVerified`: both remain **NOT RUN** unless independently documented. Machine translation requires source and Yaskawa manual review.

# Inline editing of operand addresses — merged PR #8

> ** Status and applicability:** This document describes the relevant functional subset and evidence collected before/by PR #21This does **not** mean the entire project stopped at PR #21. Actual `main` later [PR #29](https://github.com/uzbatsu-beep/MLAD/pull/29) and independent levels of CI/UserSmoke/OEM: [project_status.md](project_status.en.md). Do not give permission to a native writer, HSES or perform a scan from the specification described herein.

**Status (18.11.2026):** PR #8 Mixed; change of ** address* is allowed only in JSON drafts. At Native Workspace PR #16 Double-click edits only the **local comment**, not the address in the original CIOPRG.LST or IONAME.DAT.

## User workflow

In the **Graph Editor** or **TMR/CNT/GSTR/GOUT** draft tab, double-click a displayed operand address (or focus a selectable diagram element and press **F2**) to edit **in place**.

- **Enter** — validate syntax and save the new value to the corresponding independent JSON draft.
- **Esc** or click outside — cancel changes; the original value remains.
- **Invalid input** — remains in the editor with an error message; no project mutation.
- **Single click** — preserves previous selection behavior.
- Editing the ordinary graph contact/OUT participates in existing **Undo/Redo** history.
- Editing an advanced block invalidates that block's prior offline sample, returning its display to **UNKNOWN** until a manual test step is performed.

### Editable labels

| Workspace | Editable label | Format |
|---|---|---|
| Graph editor | NO/NC contact relay | `#XXXXX` (five decimal digits, bit 0–7) |
| Graph editor | OUT coil | `#XXXXX` |
| TMR | Start input (only for a simple single-contact condition) | `#XXXXX` |
| TMR | Current register | `M000..M559` |
| TMR | Preset, literal or M register | `0..65535` or `M000..M999` |
| TMR | OUT coil | `#XXXXX` |
| CNT | Count C / Reset R (only for simple single-contact expressions) | `#XXXXX` |
| CNT | Current register / preset / OUT | Same as TMR |
| GSTR/GOUT | Source byte and destination byte | `#XXXX0` (8 bits starting at bit 0) |

More complex AND/OR start/count/reset expressions continue to be edited using the contact-expression editor below the diagram; the displayed expression is **never** replaced with a single relay by a double-click.

## Validation and safety

Frontend performs a small syntax check to reject malformed inputs *before* modifying the draft. The existing **Rust controller-specific validator** is then invoked by the editor to check legal ranges and relay class for DX100, DX200 or YRC1000. An address that passes frontend syntax validation can still fail controller-specific validation; the Rust diagnostic should be used to resolve it.

A double-click has **no access** to network I/O. Edited JSON is not a Yaskawa program, not an `*.LST` file, and not a permit to write or deploy to a robot.

In **View CIOPRG.LST**, all diagrams remain **read-only**: no editing callback is provided to either the Ladder or Advanced SVG diagram. The original backup's source bytes and SHA-256 are not modified.

## Windows preview regression checklist

1. With two or more graph rungs, double-click one NO contact, change `#20010` to `#70011`, press Enter. Confirm only that contact changes and the selected rung remains visible.
2. Press Undo then Redo. Verify both transitions.
3. Double-click the OUT label, enter a valid `#XXXXX` and confirm.
4. Enter an invalid bit index `#70018`; the editor should show an inline error without writing that value.
5. Press Escape after editing a contact; the original value must remain.
6. Add a TMR and double-click `M011`, preset and OUT labels. Verify the corresponding fields change. `M560` must be rejected as a current register.
7. Add a CNT and verify simple C and R contact addresses can be changed independently. Preserve the two-port architecture.
8. Add GSTR/GOUT. Edit source/destination byte addresses. A nonaligned `#20011` must be rejected.
9. Trigger an offline test and then change a relevant block operand. The previous result must return to UNKNOWN; no stale green state should be shown.
10. Open a genuine `CIOPRG.LST` in the inspector. A double click there must **not** create an input field or modify the native file.

For vendor-native backups, **always use a preserved copy**. Record model, firmware, source SHA-256 and source encoding; never edit the System Ladder on the robot as part of this test.


## Known limitations and delayed improvements

- **Only the syntax in the first step:** for example `#XXXXX` It can be a correct text, but an unacceptable class or range in the chosen generation. Rust-check after recording returns the diagnosis, but the draft may save the incorrect address for correction.
- For **complex** TMR/CNT conditions, edit the expression tree in the panel from multiple contacts; online replacement of the entire tree with a single contact is prohibited.
- `Undo/Redo` In the current implementation, Graph Workspace has a separate full history of changes. `*.advanced.json` Not yet implemented.
- Automatic tests check formats, but the real work of focus and double-click WebView2 You should check the manual Windows smoke test.
- The control profile should be checked separately: front-end validation should not become the basis for recording in a physical robot.

See. [architecture.md](architecture.en.md) и [development_workflow.md](development_workflow.en.md).
