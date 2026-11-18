<!-- MLAD-I18N:NAV:START -->
> 🌐 **Язык / Language:** [Русский](inline_address_editing.ru.md) · [English](inline_address_editing.en.md) · [Original / Исходный текст](inline_address_editing.md). **Machine translation / Машинный перевод: editorial review required / необходима редакторская проверка.**
<!-- MLAD-I18N:NAV:END -->

# Inline editing of operand addresses — merged PR #8

> **Статус и применимость:** этот документ описывает соответствующее функциональное подмножество и доказательства, собранные до/по PR #21; он **не** означает, что весь проект остановился на PR #21. Актуальный `main` после [PR #29](https://github.com/uzbatsu-beep/MLAD/pull/29) и независимые уровни CI/UserSmoke/OEM: [project_status.md](project_status.md). Не выводить разрешение на native writer, HSES или исполнение скана из описанной здесь спецификации.

**Status (18.11.2026):** PR #8 merged; изменение **самого адреса** допускается только в JSON-черновиках. В Native Workspace PR #16 двойной щелчок редактирует лишь **локальный комментарий**, не адрес в оригинальном CIOPRG.LST или IONAME.DAT.

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

In **Просмотр CIOPRG.LST**, all diagrams remain **read-only**: no editing callback is provided to either the Ladder or Advanced SVG diagram. The original backup's source bytes and SHA-256 are not modified.

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


## Известные ограничения и отложенные улучшения

- **Только синтаксис в первом шаге:** например `#XXXXX` может быть корректным текстом, но недопустимым классом или диапазоном в выбранном поколении; Rust-проверка после записи возвращает диагностику, но черновик может сохранить некорректный адрес для исправления.
- Для **сложных** условий TMR/CNT из нескольких контактов нужно редактировать дерево выражения в панели; инлайн-замена всего дерева одиночным контактом запрещена.
- `Undo/Redo` в текущей реализации есть у Graph Workspace; отдельная полноценная история изменений `*.advanced.json` пока не реализована.
- Автоматические тесты проверяют форматы, но реальную работу фокуса и double-click в WebView2 следует подтвердить ручным Windows smoke test.
- Контроллерный профиль нужно проверить отдельно: фронтенд-валидация не должна становиться основанием для записи в физический робот.

См. [architecture.md](architecture.md) и [development_workflow.md](development_workflow.md).
