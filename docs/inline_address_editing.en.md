<!-- MLAD-I18N:EDITORIAL:reviewed 2026-11-19 against inline_address_editing.md -->
> **Language / Язык:** [English](inline_address_editing.en.md) · [Русский](inline_address_editing.ru.md) · [Original](inline_address_editing.md)
>
> **Safety:** Only independent `*.graph.json`/`*.advanced.json` addresses can be edited. Native `CIOPRG.LST`/`IONAME.DAT` are read-only; HSES/FTP/native writer/deployment **BLOCKED**. Manual Windows WebView2 UserSmoke and OEMBench remain **NOT RUN** absent independent evidence.

# Inline editing of operand addresses — merged PR #8

> **Status and applicability:** this describes the functionality and evidence available up to PR #21; **it does not imply all project development ended with PR #21**. See [project_status.md](project_status.en.md) for the post-[PR #29](https://github.com/uzbatsu-beep/MLAD/pull/29) baseline and separate CI/UserSmoke/OEM evidence levels. This specification authorizes **no** native writer, HSES or actual controller scan.

**Status (18 November 2026):** PR #8 merged. Changing the **operand address itself** is allowed **only in JSON drafts**. In Native Workspace (PR #16), a double-click may edit **a local annotation only**, **not** an address in the original CIOPRG.LST or IONAME.DAT.

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

In **Просмотр CIOPRG.LST** (read-only native inspector), all diagrams remain **read-only**: no editing callback is provided to either the Ladder or Advanced SVG diagram. The original backup's source bytes and SHA-256 are not modified.

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


## Known limitations and deferred improvements

- **First-stage frontend validation checks syntax only.** For example, `#XXXXX` may be well formed but outside the allowable class or range for the selected generation. The subsequent Rust check produces a diagnostic, and the draft **may retain an invalid address for later correction**. Do not equate a frontend pass with an accepted robot address.
- **Complex** TMR/CNT input conditions consisting of multiple contacts must be edited in the expression-tree panel. Silently replacing the entire tree with one contact via inline editing is prohibited.
- `Undo/Redo` is implemented for Graph Workspace. Full independent undo/redo history for `*.advanced.json` is **not yet implemented**.
- Automated tests cover model/input formats, **not actual focus or double-click behavior in WebView2**. The latter needs manual Windows smoke testing.
- Controller family and firmware/configuration must be checked separately; frontend validation never authorizes physical robot writes.

See [architecture.md](architecture.en.md) and [development_workflow.md](development_workflow.en.md).
