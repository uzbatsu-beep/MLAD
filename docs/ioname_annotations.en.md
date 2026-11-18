<!-- Generated en companion for docs/ioname_annotations.md; review required. -->
> **English** · [Русский](ioname_annotations.ru.md) · [English](ioname_annotations.en.md) · [Original / Оригинал](ioname_annotations.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.

# MLAD — IONAME.DAT and per-address/per-rung comments (experimental)

**Status on 18.11.2026:** functionality PR #16 slug in `main`**; read-only import `IONAME.DAT`Local comments and manual associations are available on Native Workspace. **No native writer and IONAME verified automatic card #XXXXX. [Certified status](project_status.en.md).

## Input evidence and unknowns

Yaskawa identifies `IONAME.DAT` as I/O name data, distinct from `CIOPRG.LST`. Published examples use a `/IONAME` header followed by named sections such as `//IN`, `//OUT` and data lines with **four comma-separated names**. This implementation reads these records into a bounded display-only table with original SHA-256. Other layouts are diagnosed, not guessed.

References:
- Yaskawa: https://knowledge.motoman.com/hc/en-us/articles/4405166620567-Alias-Function-for-Variables-and-I-O
- Yaskawa: https://knowledge.motoman.com/hc/en-us/articles/25864549058327-Robot-Pendant-Concurrent-I-O-Graphical-Ladder-with-Comments
- Example layout: https://dramat.eu/doc/yaskawa_basic.html

**Unverified:** exact YRC1000 controller/firmware layout and mapping of IONAME section positions to Concurrent I/O `#XXXXX` addresses. A four-column positional name table must not be represented as a proven relay address table. A genuine sample is required before any automatic mapping is attempted.

## Separation of data

- `CIOPRG.LST`: immutable native program, read-only, byte-exact SHA-256.
- `IONAME.DAT`: immutable native name data, read-only, byte-exact SHA-256. The exposed labels are for human reference, not native export.
- MLAD comments: local user annotations (`rungs`, `addresses`, `ioname`) keyed to `CIOPRG.LST` SHA-256 and, for individual IONAME entries, the IONAME SHA-256 and physical source line/slot. LocalStorage is retained across ordinary tab switches and app restarts, but **not** a guaranteed backup.
- Double-click edits only the local annotation. Clear or change app profile may remove data. Sidecar JSON import/export and verified native IONAME writer require separate design/testing and explicit approval.

## Safety checks

1. Files limited to 4 MiB for IONAME and 16 MiB for CIOPRG, rejecting symlinks for IONAME.
2. A view-only snapshot never writes an original program or name file.
3. Native label mapping is always `relay_mapping_verified = false`, and `native_write_allowed = false`.
4. Manual association of IONAME label to `#XXXXX` or `Mxxx` affects only local display.
5. Full Ladder list is limited to all SemanticPreview rungs, with explicit warning if results were truncated.
6. No HSES/FTP, device I/O, source translation, or System/User authorization is activated.

## Next step: Portable annotations (offer, not implementation)

В [roadmap v3](roadmap.en.md) merged PR #29 Versioned sidecar offered `*.mlad.json` conditional PR #33), linked to source SHA, with migrations and rejection to stale source. At the moment, localStorage remains a local mechanism, not a portable project and not a guaranteed backup. Native. `IONAME.DAT` Read-only remains; automatic name mapping to **n* addresses is approved.
