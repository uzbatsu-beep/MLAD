<!-- Generated en companion for ACCESSIBILITY.md; review required. -->
> **English** · [Русский](ACCESSIBILITY.ru.md) · [English](ACCESSIBILITY.en.md) · [Original / Оригинал](ACCESSIBILITY.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.
> **MLAD safety invariants:** `CIOPRG.LST` and `IONAME.DAT` are strictly **read-only**. FTP/HSES, live I/O, native writer and robot deployment remain **BLOCKED**. Passing `WindowsCI` does **not** imply `UserSmoke=PASS` or `OEMBenchVerified`: both remain **NOT RUN** unless independently documented. Machine translation requires source and Yaskawa manual review.

# Availability of the MLAD interface

**State as of 18.11.2026:** engineering prototype on Tauri/WebView2 (Windows x64), React and SVG. An independent availability audit ** has not been conducted **; compliance with WCAG 2.2 AA ** is not claimed **.

## Supported environment

The main Windows preview uses WebView2. Builds for other OSes may require separate dependencies and testing. The interface is designed primarily for engineering desktop workflow, but should remain clear when zooming in and working with the keyboard.

## What's done

- Signatured form elements and navigation sections in Native Workspace, independent scale visual Ladder and Advanced panels.
- Access to source code even for unsupported graphic designs; information about uncertainty should not depend solely on color.
- Systemic dialogues of choice CIOPRG.LST/IONAME.DATText diagnostic messages and transitions to lines.
- A separate JSON draft provides double-click/F2 operations; this **not* means that all scripts can only be executed by the keyboard.

## What is untested and needs development

- Full mouse-free navigation, focus and recovery, Tab order, SVG/diagram handling via keyboard.
- Screen reader/ARIA signatures of each graphical instruction, zooming and contrasting all states in a dark theme.
- Behavior on 1280×800 screens and with increasing system scale, high-contrast/reduced-motion support.
- Separate scenarios for editing local comments and declaring them with assistive technologies.

If an accessibility barrier is found, open [Feature Request](https://github.com/uzbatsu-beep/MLAD/issues/new/choose) and describe the **synthetic** scenario, OS/scale/input method. Do not attach original robot programs, private identifiers, or enterprise data. Use for vulnerabilities [SECURITY.md](SECURITY.en.md).

A document is an honest description of current availability, **not a certificate of conformity**.

## The nearest accessibility roadmap (proposal)

[Roadmap v3](docs/roadmap.en.md) preliminary PR #32 Keyboard navigation/focus for Native Workspace, large file search/diagnostics, and zoom verification. This is a **not* implemented warranty on the baseline CI date after #29. Independent manual Windows/WebView2 acceptance [WIN-01–WIN-12](docs/windows_acceptance_pr28.en.md) remains `NOT RUN`WCAG 2.2 AA is not claimed.
