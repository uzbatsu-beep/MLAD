<!-- Generated ru companion for docs/documentation_inventory_2026-11-18.md; review required. -->
> **Русский** · [Русский](documentation_inventory_2026-11-18.ru.md) · [English](documentation_inventory_2026-11-18.en.md) · [Original / Оригинал](documentation_inventory_2026-11-18.md)
> **Translation review / Проверка:** автоматический черновик — сверяйте инженерные утверждения с оригиналом и OEM документацией.
> **Контрольные ограничения MLAD:** `CIOPRG.LST` и `IONAME.DAT` — строго **только чтение**. FTP/HSES, live I/O, native writer и загрузка в контроллер — **BLOCKED (запрещены)**. `WindowsCI` не заменяет `UserSmoke=NOT RUN` и `OEMBench=NOT RUN`; машинный перевод должен быть сверен с исходным документом и документацией Yaskawa.

# MLAD — аудит всего каталога Markdown после PR #29

**Дата:** 18.11.2026. **Проверенный baseline:** `main` `f85ddfcce3063dd087425be981a9d2b7a16d6309` (PR #1–#29 merged). Проверены все пути `*.md` из [дерева GitHub](https://github.com/uzbatsu-beep/MLAD/tree/f85ddfcce3063dd087425be981a9d2b7a16d6309). В исходном `main` — **50 Markdown**. В предлагаемой ветке этого документа — **53 Markdown**: два новых исторических архива и этот реестр. 38 Markdown-файлов имеют предложенные изменения (включая два архива), 14 исходных файлов сознательно оставлены без изменений. Ссылки ниже относятся к ветке PR #30 до merge.

**Технический контрольный факт:** [Linux CI Успех](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454), [Windows CI Успех](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621) для указанного `main`; новая документационная ветка требует **собственного** `docs:check` и `MLAD validation` на фактическом HEAD SHA. `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`, `HSES/native writer/deployment=BLOCKED`.

## 1. Ключевые результаты ревизии

1. Устранена путаница с **PR #29 Draft**: фактически PR #29 merged, а `main` содержит Windows File ID+byte snapshots, hardlink/staged SHA-256 guards и Windows ADS/UNC/device path deny.
2. `docs/roadmap.md` обновлена до **v3** с фактическими milestone M0–M13, самостоятельными evidence gates и приоритетом parser fixtures/Native Workspace; старая v2 сохранена целиком в архиве.
3. `docs/implementation_plan.md` теперь ведёт предварительную очередь **#30–#37** и отделяет runtime PR от документационного; прежняя таблица сохранена как v2 archive.
4. `docs/project_status.md` заново сверена с источниками GitHub: `main` SHA, Linux/Windows CI, последний официальный release `rc.9` и независимый ручной smoke. **Новые CI artifacts не названы релизом.**
5. Исправлены технические анахронизмы: в `cioprg_lst_spec.md` старый `network/mod.rs` больше не описан как существующий; в `hse_protocol.md` убрана устаревшая характеристика `csp: null`. Отмечено, что Cargo содержит `suppaftp` dependency, хотя runtime HSES/FTP заблокирован.
6. Активные feature-specs сохраняют исторические evidence/контракты, но больше не используют banner `PR #1–#21 merged` как актуальную общую степень готовности проекта.
7. Архивные release notes `rc.1–rc.9`, исходные датированные аудиты и legacy pointers **сохранены** без переписывания фактов прошлых сборок. Настоящие CIOPRG/IONAME и чувствительные источники не добавлялись.

## 2. Действующие документы: 37 MD

| Файл | Результат ревизии |
| --- | --- |
| [.github/PULL_REQUEST_TEMPLATE.md](../.github/PULL_REQUEST_TEMPLATE.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [ACCESSIBILITY.md](../ACCESSIBILITY.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [CONTRIBUTING.md](../CONTRIBUTING.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [README.md](../README.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [SECURITY.md](../SECURITY.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/README.md](README.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/advanced_blocks.md](advanced_blocks.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/advanced_hardware_test_matrix.md](advanced_hardware_test_matrix.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/architecture.md](architecture.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/cioprg_lossless_parser.md](cioprg_lossless_parser.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/cioprg_lst_spec.md](cioprg_lst_spec.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/controller_fixture_intake.md](controller_fixture_intake.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/csp_path_hardening.md](csp_path_hardening.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/development_workflow.md](development_workflow.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/documentation_inventory_2026-11-18.md](documentation_inventory_2026-11-18.ru.md) | **Создан** — реестр актуализации |
| [docs/engineering_decisions.md](engineering_decisions.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/filesystem_security_pr29.md](filesystem_security_pr29.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/graph_editor.md](graph_editor.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/guarded_io_review.md](guarded_io_review.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/hse_protocol.md](hse_protocol.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/implementation_plan.md](implementation_plan.ru.md) | **Обновлён** — переписан действующий baseline, roadmap/backlog |
| [docs/inline_address_editing.md](inline_address_editing.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/ioname_annotations.md](ioname_annotations.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/json_persistence_safety.md](json_persistence_safety.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/licensing.md](licensing.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/oem_5_opcode_analysis.md](oem_5_opcode_analysis.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/post_end_data.md](post_end_data.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/project_status.md](project_status.ru.md) | **Обновлён** — переписан действующий baseline, roadmap/backlog |
| [docs/roadmap.md](roadmap.ru.md) | **Обновлён** — переписан действующий baseline, roadmap/backlog |
| [docs/semantic_ast.md](semantic_ast.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/semantic_verification_matrix.md](semantic_verification_matrix.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/source_registry.md](source_registry.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/typed_memory_model.md](typed_memory_model.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/typed_opcode_grammar.md](typed_opcode_grammar.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/ui_multi_rung_state.md](ui_multi_rung_state.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/verification_gates.md](verification_gates.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |
| [docs/windows_acceptance_pr28.md](windows_acceptance_pr28.ru.md) | **Обновлён** — статус, ссылки, риск/границы или требования к проверкам |

## 3. Исторические спецификации и release records: 15 MD

| Файл | Результат ревизии |
| --- | --- |
| [docs/documentation_audit_2026-11-17.md](documentation_audit_2026-11-17.ru.md) | **Без изменений** — сохранён исторический смысл |
| [docs/implementation_plan_v2_archive.md](implementation_plan_v2_archive.ru.md) | **Создан** — архив предыдущей версии |
| [docs/ladder_spec.md](ladder_spec.ru.md) | **Без изменений** — сохранён исторический смысл |
| [docs/memory_map.md](memory_map.ru.md) | **Без изменений** — сохранён исторический смысл |
| [docs/references.md](references.ru.md) | **Без изменений** — сохранён исторический смысл |
| [docs/releases/v0.5.0-rc.1-hardware.md](releases/v0.5.0-rc.1-hardware.ru.md) | **Без изменений** — сохранён исторический смысл |
| [docs/releases/v0.5.0-rc.2-cioprg.md](releases/v0.5.0-rc.2-cioprg.ru.md) | **Без изменений** — сохранён исторический смысл |
| [docs/releases/v0.5.0-rc.3-workspace.md](releases/v0.5.0-rc.3-workspace.ru.md) | **Без изменений** — сохранён исторический смысл |
| [docs/releases/v0.5.0-rc.4-annotations.md](releases/v0.5.0-rc.4-annotations.ru.md) | **Без изменений** — сохранён исторический смысл |
| [docs/releases/v0.5.0-rc.5-layout.md](releases/v0.5.0-rc.5-layout.ru.md) | **Без изменений** — сохранён исторический смысл |
| [docs/releases/v0.5.0-rc.6-postend.md](releases/v0.5.0-rc.6-postend.ru.md) | **Без изменений** — сохранён исторический смысл |
| [docs/releases/v0.5.0-rc.7-typed.md](releases/v0.5.0-rc.7-typed.ru.md) | **Без изменений** — сохранён исторический смысл |
| [docs/releases/v0.5.0-rc.8-evidence.md](releases/v0.5.0-rc.8-evidence.ru.md) | **Без изменений** — сохранён исторический смысл |
| [docs/releases/v0.5.0-rc.9-oem-reference.md](releases/v0.5.0-rc.9-oem-reference.ru.md) | **Без изменений** — сохранён исторический смысл |
| [docs/roadmap_v2_archive.md](roadmap_v2_archive.ru.md) | **Создан** — архив предыдущей версии |

## 4. Стабильный нормативный документ: 1 MD

| Файл | Результат ревизии |
| --- | --- |
| [CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.ru.md) | **Без изменений** — сохранён исторический смысл |

## 5. Правила для следующих обновлений

- **Одна точка истины:** [project_status](project_status.ru.md) (реальные SHA/CI и release), [roadmap](roadmap.ru.md) (вехи), [implementation_plan](implementation_plan.ru.md) (сфера действия PR), [verification_gates](verification_gates.ru.md) (приёмка), [engineering_decisions](engineering_decisions.ru.md) (утверждённое).
- Любая feature spec из раздела 2 описывает только **свою** область поддержки; по ней нельзя автоматически заявить полную OEM грамматику, настоящий scan или права на запись.
- Release notes/historical audit не переписывать задним числом. Их навигационные упоминания старого `main` датированы соответствующей исторической проверкой; для нового состояния открывать ссылку на `project_status`.
- После merge следующего PR фиксировать новый main SHA и CI; после ручного Windows теста отдельно обновить `UserSmoke`, не заменяя его автоматическим `WindowsCI`.
- Ни один PR не сливается и ни один release не публикуется без отдельного решения владельца.

[Каталог](README.ru.md) · [Решения](engineering_decisions.ru.md) · [План дальнейшей разработки](implementation_plan.ru.md).
