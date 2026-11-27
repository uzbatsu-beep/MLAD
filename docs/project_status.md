<!-- MLAD-I18N:NAV:START -->
> 🌐 **Язык / Language:** [Русский](project_status.ru.md) · [English](project_status.en.md) · [Original / Исходный текст](project_status.md). **Machine translation / Машинный перевод: editorial review required / необходима редакторская проверка.**
<!-- MLAD-I18N:NAV:END -->

# MLAD — фактическое состояние

Сверено 27.11.2026 по main `39137a60f68041ed1b4e65d2c3fcdf3518dff3d8`, PR #52 merged. Runtime базис: PR #51, [CI merge commit](https://github.com/uzbatsu-beep/MLAD/actions/runs/45813397360) SUCCESS. Последняя опубликованная сборка: [rc.10](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.10-unified-offline) — отдельный release, не содержит последующий Semantic Core v2. Не путать main, CI artifact и release.

## Реализовано

| Компонент | Статус и пределы |
| --- | --- |
| Lossless native parser | Неизменные байты, SHA-256, физические строки, unknown/post-END |
| Controller memory | DX100/DX200/YRC1000; неполное OEM/firmware покрытие |
| Semantic Core v2 | Единый фасад, версия IPC 1, SourceIdentity и spans; 200 rung refs preview |
| Static analysis | READ/WRITE/UNKNOWN; гипотезы dataflow, не исполнение |
| Native Workspace | Source, Ladder, blocks, IONAME, search и навигация |
| JSON drafts | Graph/Advanced, tri-state и поддерживаемые формы |
| Offline project | `*.mladproj.json`, `*.mlad.json` v1; SHA и относительные пути |
| Filesystem | Staged write, backup, File ID, ограничения путей; остаточные ACL/TOCTOU/multifile recovery открыты |

## Запланировано

PR #52 утвердил UI; реализация новых компоновок ещё не начата. Следующие стабильные задачи V5-03 Controller Profiles, V5-04 Signal Index, V5-05 UI Foundation, V5-06 Workspace, V5-07 Ladder Viewer, V5-08 Inspector. Номера PR ориентировочные. [Roadmap](roadmap.md), [план](implementation_plan.md).

## Верификация и ограничения

`CIOPRG.LST` и `IONAME.DAT` read-only. HSES/FTP, native writer, live I/O и deployment BLOCKED. UserSmoke=NOT RUN; OEMBench=NOT RUN. OEM scan, права System/User и полная семантика не доказаны. Синтетические fixtures не загружать на робота.

## История и источники

[Архитектура](architecture.md) · [Критерии](verification_gates.md) · [Аудит 21.11](repository_audit_2026-11-21.md) · [CHANGELOG](../CHANGELOG.md). Все прежние детальные записи PR #1–#51 сохранены в [неизменяемом срезе](https://github.com/uzbatsu-beep/MLAD/blob/0754dea0d37f5fbc45c795008b60fe69e6c8b08f/docs/project_status.md). Не дублировать их здесь; обновлять только актуальное состояние.
