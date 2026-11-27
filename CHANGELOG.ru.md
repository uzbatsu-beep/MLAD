<!-- MLAD-I18N:EDITORIAL:source-aligned Russian; 2026-11-27 -->
> **Язык / Language:** [Русский](CHANGELOG.ru.md) · [English](CHANGELOG.en.md) · [Исходный текст / Original](CHANGELOG.md)

# MLAD — журнал изменений

Записи о выпусках относятся к конкретному tag, а не текущему main. Подробные исторические release notes остаются неизменными.

## Unreleased

- PR #50: утверждена Roadmap v5.
- PR #51: Semantic Core v2, source identity, версионированный IPC, инженерная сводка.
- PR #52: согласована концепция UI, D16–D20; новый UI ещё не реализован.

## v0.5.0-rc.10-unified-offline — 2026-11-20

Единый переносимый офлайн-проект, sidecar, статический READ/WRITE/UNKNOWN, синтетический Windows Preview. [Релиз и SHA-256](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.10-unified-offline). UserSmoke=NOT RUN; OEMBench=NOT RUN.

## Ранее

rc.1–rc.9: [архив release notes](docs/releases/), [GitHub Releases](https://github.com/uzbatsu-beep/MLAD/releases). rc.9 не содержит PR #24–#29.

## Политика версий

Предлагаемая реализация: SemVer для опубликованных версий; pre-release суффикс явно отражает стадию. План 0.6.x core/workspace, 0.7.x intelligence, 0.8.x compare, 0.9.x lab/test, 0.10.x docs/fleet, 0.11.x AI, 1.0.0 stable offline. Перед выпуском сверить tag, package.json, Cargo.toml, lockfiles и tauri.conf.json по единому правилу; исторические rc не переписывать. Отдельные schema versions не равны версии продукта. Публикация только после отдельного решения; CI artifact не Release. Требуются хеши, соответствующий исходный код и evidence.

`CIOPRG.LST` и `IONAME.DAT` read-only; HSES/FTP/native writer/deployment BLOCKED.
