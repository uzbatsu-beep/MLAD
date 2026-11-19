<!-- Generated en companion for docs/references.md; review required. -->
> **English** · [Русский](references.ru.md) · [English](references.en.md) · [Original / Оригинал](references.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.
> **MLAD safety invariants:** `CIOPRG.LST` and `IONAME.DAT` are strictly **read-only**. FTP/HSES, live I/O, native writer and robot deployment remain **BLOCKED**. Passing `WindowsCI` does **not** imply `UserSmoke=PASS` or `OEMBenchVerified`: both remain **NOT RUN** unless independently documented. Machine translation requires source and Yaskawa manual review.

# References – Archive of Preliminary Source List

> **Archived index (reconciliation 18.11.2026):** This file is saved for references; do not use it as a valid specification. PR #1-#21 merged, the latest Windows release is rc.9. [Current status](project_status.en.md).

The previously maintained table included a number of unverified assumptions, including incorrect read-only/write classes for several CIO relay banks. It is retained as a stable navigation path.

**Authoritative project index:** [source_registry.md](source_registry.en.md)

**Verified controller-specific ranges:** [typed_memory_model.md](typed_memory_model.en.md)

**File syntax and safe round-trip:** [cioprg_lst_spec.md](cioprg_lst_spec.en.md)

Do not derive writing privileges or complete file headers from an isolated manual example. Controller-specific manufacturer manuals, application options and genuine exported test files must be verified before enabling compile/deploy.


**Updated 17.11.2026:** Main [source_registry.md](source_registry.en.md) It distinguishes between documented instructions, implemented tests, real files and measurements on the stand. So far, there are no hardware results and original Golden fixtures in the public repository. For new external links, indicate an accurate manual review and access restrictions.

The path to up-to-date documentation: [docs/README.md](README.en.md)requirements for obtaining sources: [controller_fixture_intake.md](controller_fixture_intake.en.md).
