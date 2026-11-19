<!-- Generated en companion for docs/ui_multi_rung_state.md; review required. -->
> **English** · [Русский](ui_multi_rung_state.ru.md) · [English](ui_multi_rung_state.en.md) · [Original / Оригинал](ui_multi_rung_state.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.
> **MLAD safety invariants:** `CIOPRG.LST` and `IONAME.DAT` are strictly **read-only**. FTP/HSES, live I/O, native writer and robot deployment remain **BLOCKED**. Passing `WindowsCI` does **not** imply `UserSmoke=PASS` or `OEMBenchVerified`: both remain **NOT RUN** unless independently documented. Machine translation requires source and Yaskawa manual review.

# Multiple visible rungs and offline tri-state visualization — merged PR #7

> **Status and applicability:** document captures OFFLINE tri-state and UI behavior PR #7/#8/#16/#17. Common. `main` later [PR #29](https://github.com/uzbatsu-beep/MLAD/pull/29) and the levels of inspection are reflected in [project_status.md](project_status.en.md). This UI is a live I/O or physical safety indicator.

**Status (18.11.2026):** PR #7/#8 merged; PR #16/#17 added native multi-rung and independent scale. The document describes primarily **OFFLINE** visual states, not live I/O.

## User-reported issues (Windows Bench v0.5.0-rc.1)

Review of five screenshots:
- The graph editor originally displayed only the active rung, even when two rungs existed.
- Relay contact selection changed a green selection border, but **did not** indicate a measured or simulated TRUE/FALSE state.
- TMR appeared clipped by an inner scroll area, and CNT was not legible as a whole function block.
- GSTR/GOUT was shown as one symbolic block but lacked eight distinct status bits.

These are UI presentation and ergonomics problems, not evidence of controller communication.

## Implemented

### Multi-rung work area
- All `*.graph.json` rungs are rendered in one vertically scrollable work area, in file order.
- Each rung keeps its identifier, OUT coil address, comment, selectable contacts and individual expression result badge.
- Click a contact on any rung to make that rung active and open its editing controls. Adding a rung does not hide existing rungs.
- Shared manual input values are **not** reset when selecting another rung. A relay address shared by multiple rungs receives the same manual value across those rungs.
- Main canvas scales to the available width; complex graphs can scroll horizontally *inside the rung* without clipping contacts.

### Tri-state contact and coil colors (OFFLINE only)

| State | Presentation | Meaning |
|---|---|---|
| TRUE | Green | Boolean expression/contact is true under **manually supplied** inputs |
| FALSE | Red | Boolean expression/contact is false under **manually supplied** inputs |
| UNKNOWN | Grey | Not enough input data to determine expression result |

- Normal-open (NO) contacts follow the manually assigned relay bit.
- Normally-closed (NC) contacts **invert** the input bit. Coloring shows contact *conduction*, not the raw bit.
- Series AND and parallel OR use three-valued logic. Example: `FALSE AND UNKNOWN = FALSE`, `TRUE OR UNKNOWN = TRUE`.
- Coils are colored by the evaluated **expression condition**, not by an actual output feedback sample.
- Source-only `CIOPRG.LST` inspection without test inputs remains neutral; it does not invent live states.
- Wiring remains neutral gray because a full scan-time powered path isn't modeled; colorizing every wire segment would imply unverified flow information.
- All states are labeled with both text and color; selection uses a separate yellow outline and never modifies the Boolean state.

### Advanced block overview and readability
- The Advanced panel renders **every configured TMR / CNT / GSTR+GOUT** as an independent card on one shared scrollable main canvas.
- Diagram SVG height is not shrunk by a viewport-height grid row; vertical inner scroll is disabled and the diagram scales to the available width.
- CNT explicitly displays **C (count)**, **R (reset)** and **Q**; TMR displays its start, current/preset and Q.
- Group transfer displays distinct bits **b7...b0** with green/red/gray sample indicators and a hexadecimal byte.
- Every block starts with **UNKNOWN** output; only a user-initiated offline step can make a block's output known.
- The last offline sample is stored **per block ID**, so switching between TMR and CNT does not discard previous results. Editing a block invalidates its old sample.
- A group byte is not misrepresented as one Boolean Q. Bit states are shown only after the manual byte-copy test.

## Safety and fidelity

**Critical:** This is a display of manually supplied offline signals and deterministic test-model outputs only. It is not telemetry from DX100/DX200/YRC1000. The remote connection, HSES monitoring, native CIOPRG writer and FTP deployment remain disabled.

The colors must never be interpreted as a safety-status indication or as permission for real motion. No changed `CIOPRG.LST` is generated.

## Tests

Run:
```bash
npm run test:graph
npm run test:advanced
npm run test:visual
npm run build
cargo test --manifest-path src-tauri/Cargo.toml --locked
```

`visualState.test.ts` checks NO/NC inversion, contact vs coil logic, mixed UNKNOWN inputs under AND/OR, manual input cycle and deterministic bounds. Existing Rust and TypeScript model tests run unchanged.

## Windows user acceptance

1. Create at least 3 rungs; verify all are simultaneously present and selecting one never hides other rungs.
2. Set a shared input `#70010` to `?`, `0`, `1`. Verify matching contacts in multiple rungs change **gray / red / green** together (for NO), and NC contacts invert red/green.
3. Verify the coil state follows the rung condition; switch between rungs and verify manual bits persist.
4. Add TMR, CNT and GSTR/GOUT. Verify **all three** are visible at once and CNT C/R/Q labels are not clipped.
5. Step TMR, toggle C/R and step CNT; verify each block preserves its own last sampled state during block selection.
6. Copy 0x55 to a group; verify b7..b0 are displayed as alternating false/true values. Change source byte, rerun step and compare.
7. Update a TMR preset or CNT current address; verify the edited block's previous output returns to unknown.
8. Verify HSES/FTP stays disabled and no option to export native `CIOPRG.LST` appears.

Windows GUI smoke tests are **not** part of static CI. PR #7/PR #8 preview artifacts are historical GitHub Actions artifacts with limited retention; use a current Windows build to repeat manual acceptance. UI screenshots and user feedback establish visibility of chains and blocks but do **not** establish controller behavior.


## After PR #8How to choose and edit

- **One click selects the contact/block; the color of the state is determined separately, not by highlighting.
- **Double click / F2** to the address in the JSON draft opens inline input; Enter saves, Esc cancels. You cannot edit the original scheme. `CIOPRG.LST`.
- In Advanced blocks, a change in address/specification removes the previous manual sample; it returns to UNKNOWN rather than showing in outdated green.
- Detailed scenarios: [inline_address_editing.md](inline_address_editing.en.md).
