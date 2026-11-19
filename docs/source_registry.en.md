<!-- MLAD-I18N:EDITORIAL:reviewed 2026-11-19 against source_registry.md -->
> **Language / Язык:** [English](source_registry.en.md) · [Русский](source_registry.ru.md) · [Original](source_registry.md)
>
> **Evidence:** Translation reviewed against MLAD's source registry, **not** an independent audit of every OEM PDF revision or controller firmware. Native `CIOPRG.LST`/`IONAME.DAT` read-only; HSES/FTP/native writer/deployment **BLOCKED**; `OEMBench=NOT RUN`.

# Source register — Yaskawa Concurrent I/O / Ladder Editor

> **Status and applicability:** this records the documented subset and evidence available by PR #21, **not** the final stage of the whole project. See the [current `main` status](project_status.en.md) after [PR #29](https://github.com/uzbatsu-beep/MLAD/pull/29) and independent CI/UserSmoke/OEM evidence levels. Nothing in this registry authorizes a native writer, HSES or actual controller scan execution.

Updated 2026-11-18 (after merged PR #21). References are for **DX100, DX200, YRC1000** (not automatically YRC1000micro). Manufacturer instructions take precedence over MLAD prose and third-party mirrors.

| ID | Controller | Relevant material | Confidence / access |
|---|---|---|---|
| [RE-CKI-A453](https://gab.wallawalla.edu/~ralph.stirling/classes/engr480/docs/Motoman/dx100_concurrent_io_155491-1CD.0.pdf) | DX100 | Concurrent I/O §§1–2, instruction list | Vendor-authored PDF hosted by university; classification corroborated by [text mirror](https://www.manualzz.com/doc/84663173/yaskawa-motoman-dx100-owner-s-manual) |
| [RE-CKI-A465](https://www.motoman.com/getmedia/77390944-14FE-4715-9444-C708FE752DED/165294-1CD) | DX200 | Concurrent I/O §§1–2, §12 | Manufacturer link from [Yaskawa overview](https://knowledge.motoman.com/hc/en-us/articles/1500012236161-Concurrent-I-O-Overview); direct PDF retrieval sometimes restricted; [text mirror](https://manualzz.com/doc/84662547/yaskawa-motoman-dx200-concurrent-i-o-owner-s-manual) |
| [RE-CKI-A467](https://www.motoman.com/getmedia/E9027AFA-478B-4F88-BC14-245CDD6D51BE/178648-1CD) | YRC1000 | Concurrent I/O §§1–2, §12 | Manufacturer document linked by [Yaskawa overview](https://knowledge.motoman.com/hc/en-us/articles/1500012236161-Concurrent-I-O-Overview); direct fetch sometimes restricted; [text mirror](https://manualzz.com/doc/84663174/yaskawa-motoman-yrc1000-owner-s-manual) |
| [HW1483388](https://www.motoman.com/getmedia/7DCAB851-59B3-4698-97FB-AE39E45FD106/179482-1CD) | YRC1000 | On-pendant Ladder Editor, §1.3.3 instruction families; §1.3.6 header UI | Manufacturer PDF inspected |
| [HW1483921](https://www.scribd.com/document/772248336/LadderEditor32YRC1000-Us) | YRC1000 PC | Ladder Editor 32 for YRC1000 | Third-party document host; use as supplementary reference |
| [HW1485509](https://www.ckp.edu.pl/images/YASKAWA_Manual_SmartPendant.pdf) | YRC1000 / micro | Smart Pendant §§7, 13 | Vendor-authored PDF at educational mirror; distinct from CIO options manuals |
| [MRS55130](https://www.yumpu.com/en/document/view/3622119/users-manual-ladder-editor-32-version-12-motoman) | legacy XRC/NX100 | Ladder Editor 32 | Not used to assert DX100/DX200/YRC1000 file grammar without corroboration |
| [Yaskawa CIO Overview](https://knowledge.motoman.com/hc/en-us/articles/1500012236161-Concurrent-I-O-Overview) | All | CIOPRG.LST identity, System/User Ladder, model-specific manuals | Manufacturer web article |
| [Yaskawa CIO editing](https://knowledge.motoman.com/hc/en-us/articles/4408446257175-Options-for-Editing-the-Concurrent-I-O-Ladder) | All | Editing and hazard warning | Manufacturer web article |
| [Yaskawa `//$M` note](https://knowledge.motoman.com/hc/en-us/articles/27636724261138-Specification-of-M-header-in-CIOPRG-LST) | CIOPRG.LST | Eight header rows, M550..M489 | Manufacturer web article dated 2025-06-30 |

## What is actually verified

1. **Controller-specific address classifications**: primary-manual chapter 2 tables (DX100/DX200/YRC1000) cross-checked with secondary text sources.
2. **Instruction samples**: YRC1000 Ladder Editor and Concurrent I/O manuals, DX200 §12; an individual mnemonic example is **not** a full-file format example.
3. **CIOPRG.LST filename and system/user ownership**: manufacturer CIO overview.
4. **`//$M` section**: manufacturer article shows a concrete eight-row data mapping.

## Advanced instructions — documented behavior and implementation evidence

- **DX100 RE-CKI-A453 §§12.2.12–14 (manual pp. 12-11–12-13):**
  `TMR` 0.1 s clock, start OFF resets; `CNT` has C and independent R,
  OFF→ON counting, reset priority; `GSTR/GOUT` are paired 8-bit
  operations. Current M000..559, preset M000..999 or 0..65535.
- **DX200 RE-CKI-A465 §§12.2.12–14 (manual pp. 12-12–12-14)**:
  corresponding formats and operating principle independently checked
  against the DX200 manual text.
- **YRC1000 RE-CKI-A467 §§12.2.12–14 (manual pp. 12-12–12-15)**:
  corresponding formats and operating principle independently checked
  against the YRC1000 manual text.
- **Error 4040/4100/4150/4220/4230** cross-checked against the official
  manufacturer error-list pages for DX100/DX200/YRC1000. Error 4220
  concerns the *combined* TMR/CNT/arithmetic budget (more than 100).
- **Not verified:** entire CIOPRG file grammar, machine scan-cycle
  equivalence, hardware I/O activation, firmware-specific optional behavior,
  actual parameter refresh timing and real-controller acceptance of any
  MLAD-written file. No editable `CIOPRG.LST` serializer was added.
- Implementation: [advanced instruction contract](advanced_blocks.en.md);
  [unexecuted laboratory matrix](advanced_hardware_test_matrix.en.md).

## Not yet verified: explicit evidence requests

- A complete unmodified `CIOPRG.LST` export from each of DX100, DX200, YRC1000 (system type/option-specific) to establish all section delimiters, header bytes, comments and encoding.
- Validation of exported files using manufacturer Ladder Editor 32, preserving the system section.
- Instruction-level destination-type matrix for all mnemonics and model/firmware combinations.
- Protocol-specific HSES and controller transfer procedures. **No deploy permission** follows from this document.

### Handling the fixtures

Do not commit proprietary customer ladder content or network credentials. Use private, access-controlled golden fixtures when available; publish only minimal synthetic snippets where permission allows. Record manufacturer manual/revision, software version, application type and source file SHA-256 for reproducibility.


## Evidence levels and required verification after PR #8

- **MANUAL:** a specific manual, section and controller family document an operator and operand form **in manufacturer literature**. This does **not** establish 100% compatibility with an executable `CIOPRG.LST`.
- **SOURCE / TEST:** Rust/TypeScript implementation and CI confirm tested behavior and negative cases **on synthetic inputs**. Tests do not replace inspections of actual controller exports or hardware scan measurements.
- **REAL FILE:** a hashed original export, exact model/firmware/application and OEM Ladder Editor acceptance. **No such golden fixtures are available in the public repository.**
- **BENCH:** repeatable DX100/DX200/YRC1000 measurements, approved protocol, specific test case and OEM result. **No such results are available in the repository.**
- **UNVERIFIED:** HSES framing/command map, complete CIOPRG grammar, System/User boundaries, firmware-specific effects and native deployment. Hypotheses must **not** become robot-writing code.

See [architecture.md](architecture.en.md), [controller_fixture_intake.md](controller_fixture_intake.en.md) and [development_workflow.md](development_workflow.en.md).
