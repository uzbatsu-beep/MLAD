# Source register — Yaskawa Concurrent I/O / Ladder Editor

Updated 2026-11-17 (after merged PR #8). References are for **DX100, DX200, YRC1000** (not automatically YRC1000micro). Manufacturer instructions take precedence over MLAD prose and third-party mirrors.

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
- Implementation: [advanced instruction contract](advanced_blocks.md);
  [unexecuted laboratory matrix](advanced_hardware_test_matrix.md).

## Not yet verified: explicit evidence requests

- A complete unmodified `CIOPRG.LST` export from each of DX100, DX200, YRC1000 (system type/option-specific) to establish all section delimiters, header bytes, comments and encoding.
- Validation of exported files using manufacturer Ladder Editor 32, preserving the system section.
- Instruction-level destination-type matrix for all mnemonics and model/firmware combinations.
- Protocol-specific HSES and controller transfer procedures. **No deploy permission** follows from this document.

### Handling the fixtures

Do not commit proprietary customer ladder content or network credentials. Use private, access-controlled golden fixtures when available; publish only minimal synthetic snippets where permission allows. Record manufacturer manual/revision, software version, application type and source file SHA-256 for reproducibility.


## Уровни подтверждения и требуемые доказательства после PR #8

- **MANUAL:** номер документа, раздел и семейство подтверждают оператор и формат операнда **в руководстве**. Это не доказательство 100% совместимости исполняемого `CIOPRG.LST`.
- **SOURCE / TEST:** код Rust/TypeScript плюс CI подтверждают реализованную логику и отрицательные случаи на синтетических данных. Тесты не заменяют чтение оригинального файла или проверку аппаратного scan.
- **REAL FILE:** хешированная оригинальная копия, model/firmware/application и результат штатного Ladder Editor. **Таких golden fixtures пока нет в публичном репозитории.**
- **BENCH:** воспроизводимые измерения на DX100/DX200/YRC1000, protocol/тест-кейс/права и результат OEM. **Таких результатов в репозитории пока нет.**
- **UNVERIFIED:** HSES framing/command map, полномасштабная грамматика CIOPRG, System/User boundary, аппаратные семантические нюансы и native deployment. Запрещено превращать предположения в код контроллерной записи.

См. [architecture.md](architecture.md), [controller_fixture_intake.md](controller_fixture_intake.md), [development_workflow.md](development_workflow.md).
