<!-- Generated en companion for docs/controller_fixture_intake.md; review required. -->
> **English** · [Русский](controller_fixture_intake.ru.md) · [English](controller_fixture_intake.en.md) · [Original / Оригинал](controller_fixture_intake.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.
> **MLAD safety invariants:** `CIOPRG.LST` and `IONAME.DAT` are strictly **read-only**. FTP/HSES, live I/O, native writer and robot deployment remain **BLOCKED**. Passing `WindowsCI` does **not** imply `UserSmoke=PASS` or `OEMBenchVerified`: both remain **NOT RUN** unless independently documented. Machine translation requires source and Yaskawa manual review.

# CIOPRG.LST Accepting original files from DX100/DX200/YRC1000

> ** Status and applicability:** This document describes the relevant functional subset and evidence collected before/by PR #21This does **not** mean the entire project stopped at PR #21. Actual `main` later [PR #29](https://github.com/uzbatsu-beep/MLAD/pull/29) and independent levels of CI/UserSmoke/OEM: [project_status.md](project_status.en.md). Do not give permission to a native writer, HSES or perform a scan from the specification described herein.

**Status as of 18.11.2026:** Five private CIOPRG files have been previously investigated, but full controller/firmware information and independent OEM validation results are not available. Sources are not published.

**Purpose:** Receive representative vendor-native data to safely expand the parser. **This process does not allow software downloading, System Ladder editing or live robot control.**

## 1. What you need from the engineer from the stand

For each generation of controller, if possible:

| Field. | What to write down |
|---|---|
| Controller | DX100 / DX200 / YRC1000, precise hardware audit |
| Firmware / system software | Version from the panel, assembly date if available |
| Application | Handling / welding / painting / other options installed |
| Means of export | Version of the built-in menu, Ladder Editor 32 and the method of obtaining the file |
| File. | Original. `CIOPRG.LST`, size, exact. SHA-256 |
| Type of data | Source code/presence of BOM, CRLF/LF/CR, features `//$M` |
| OEM | Whether the file was checked by a staff editor/controller, what diagnostics |
| Contents | Are TMR/CNT, GSTR/GOUT, PART, System/User Ladder present? |
| Privacy | Possession, processing permission, distribution restrictions |
| Functional tests | Separate bench protocol: what was really measured, what was not |

### Definition SHA-256 в Windows

In PowerShell (the file must be read from the media without opening it for editing):

```powershell
Get-FileHash -Path "D:\\backup\\CIOPRG.LST" -Algorithm SHA256
```

This is the checksum of **original bytes**, not SHA from normalized text. Compare hashes with each copy, not file names.

## 2. Safe working order

1. Create an unchangeable source copy outside the working directories. The original is not edited or recoded.
2. Record the controller’s metadata, date and source; customer names, IP, passwords and technology secrets **do not publish**.
3. В `Просмотр CIOPRG.LST` pinpoint `ControllerFamily` and open a *copy*, not the original on the media.
4. Write down the hash, number of lines, errors, percentage of coverage, list of **unknown** mnemonics/sections, header presence `//$M`. The fact that there is no error is not a confirmation of native compatibility.
5. Separately compare with the OEM editor / unload without changing the protected part. A synthetic test snippet cannot be issued as a genuine export.
6. For test fixture, agree on the permitted minimum anonymization. ** Any anonymization changes bytes and SHA-256**; keep in touch `original_hash → sanitized_hash` In a closed registry.
7. Use the repository only for permitted **synthetic ** and impersonal fragments with obvious origin. Client System Ladder, network configuration and secrets are not added to public git.

## 3. Golden fixture acceptance

- The content and hash of the file are checked twice independently.
- Verification `parse_document` does not change the original bytes (`copy_original() == input`), including BOM and the end of the lines.
- Real unsupported designs should remain `opaque`Validation should not discard fragments and pass them off as supported logic.
- `system_user_boundaries_verified=false` и `edited_export_allowed=false` They are saved until the full verification of vendor files and the delimitation of rights is completed.
- The test fixture should reproduce exactly the case that it proves: `controller`, `fixture_hash`, `expected_diagnostics`, `parser_version`, `vendor_verification`.
- Complex TMR/CNT/GSTR/GOUT functions and logical branches need to be tested at the *true* source trace level, not just local model JSON tests.

## 4. Priority Questions for First Real Files

1. What are the fields going to the first `PART` And what is the model required?
2. What are the System/User boundaries and the initial length of the protected section?
3. How do codes, lines, directives and `//$M`?
4. What happens between Boolean expressions and stateful instructions?
5. Whether linear paths are maintained `AND-STR / OR-STR`?
6. What memory classes and instructions are different DX100/DX200/YRC1000?
7. How does an independent OEM editor display/verify a file, what messages does it issue?

**As of the current date, the original golden fixtures are not included in the open repository.** The absence of fixtures does not prove that the user does not have a local upload; this is the status of the repository.

## 5. Closed preliminary sample (17.11.2026)

Five new user uploads were studied locally, without transferring content, checksums and configurations to a public repository. There are file start options and separate ALARM/MESSAGE sections for `END`. Based on this, it is permissible to create **synthetic ** test designs that do not contain real values. The identity of the controller, firmware, distribution rights and independent verification by the staff editor have not yet been confirmed; this data cannot be automatically considered a reference for any family.
