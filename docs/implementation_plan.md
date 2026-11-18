# MLAD — план дальнейшей разработки после PR #26

**Контрольная точка:** 18.11.2026, `main` @ `2092db1825880d251c87efcb7c331d78cb5e4d28`. PR #1–#26 merged; [основной CI — SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44228204776). Последняя **опубликованная** Windows-сборка — [v0.5.0-rc.9-oem-reference](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference). Изменения PR #24–#26 присутствуют в `main`, но не в rc.9.

**Назначение:** исполнимый backlog для [roadmap M0–M13](roadmap.md). Номера PR ниже — **проектная последовательность**, а не созданные задачи/обязательство выпуска. Каждый этап — отдельная ветка, Draft PR, обязательные тесты, review и **никакого слияния без отдельного согласия владельца**.

## 1. Текущая граница продукта

MLAD — автономная инженерная IDE для просмотра Yaskawa Concurrent I/O и **отдельных редактируемых JSON-черновиков**. Основной Native Workspace уже работает как read-only поверхность. Разбор `CIOPRG.LST` сохраняет оригинальные байты и различает supported/partially supported/opaque. Справочные 16-bit вычисления MOV/ADD/SUB/SHL/SHR не являются моделью исполнения скана.

**Строгие запреты:** изменение `CIOPRG.LST`/`IONAME.DAT`, их native export, сетевой runtime FTP/HSES, изменение контроллера, misleading LIVE-индикация, автоматическое приравнивание `PART 1/2` к System/User authorization. Применение реальных программ — только в закрытом корпусе при наличии права доступа; публичные тесты синтетические или явно разрешённые.

## 2. Очерёдность выполнения

| Предложение | Вехи | Приоритет | Конкретный результат | Критерий приёмки / запрет |
| --- | --- | --- | --- | --- |
| **PR #27 (merged)** — синхронизация документации | M0/MX | P0 | Настоящий план, [матрица проверок](verification_gates.md), [реестр решений](engineering_decisions.md); устранение устаревших Draft/CI статусов | `docs:check` и полная CI; docs-only, никаких runtime изменений |
| **PR #28 (merged)** — Windows acceptance / security regression | M2/MX | P0 | Скриптуемые негативные проверки Tauri CSP/IPC и JSON на Windows; документированный ручной чек-лист | Windows MSVC+Tauri build, ручные Windows тесты отмечены PASS/FAIL/NOT RUN; исходные файлы неизменны |
| **PR #29 (Draft)** — модель угроз и filesystem hardening | M2 | P0 | Отдельный threat review для directory handles, Windows reparse, hardlinks, ACL, гонок final check→rename, backup/target; исправления в пределах проверенного контракта | Тесты на отказ при подмене, потере доступа, конфликте; нельзя заявлять устранение всех TOCTOU без доказательств |
| **PR #30** — синтетический parser regression corpus | M1/M5 | P0 | Версионированные синтетические кейсы вариантов заголовков, `//$M`/`PART`/`END`, tail ALARM/MESSAGE, границы размера/кодировки и malformed блоков; приватный реестр настоящих файлов вне Git | Byte-preservation и корректные spans, fail-closed unknown, ни одного реального OEM файла в Git/CI |
| **PR #31** — Native Workspace navigation & diagnostics | M4 | P1 | Source↔rung↔advanced block, навигация по адресам/мнёмоникам, фильтры severity/support/confidence, компактный список блоков и раскрытие, закладки | Все строки доступны; точные offsets/spans; выбор не синтезирует неподтверждённую диаграмму; UI performance regression |
| **PR #32** — portable sidecar проекта (спецификация и read-only привязка) | M7 | P1 | Версионированный `*.mlad.json` с source SHA, профилем, bookmarks/annotations и ссылками на отдельные JSON drafts; схема миграции/отказа | Несовпадение source hash — явная ошибка/предупреждение; неизвестные поля не повреждаются молча; native LST не сериализуется |
| **PR #33** — parser/typed syntax по evidence | M5/M6 | P1 | TypedCandidate только для форм из synthetic+подтверждённых OEM данных; категории результата `recognized / partial / opaque`; граф зависимостей с provenance | Отдельные положительные/отрицательные vector tests для DX100/DX200/YRC1000; нет исполнения неподтверждённых команд |
| **PR #34** — Unified Offline Engineering Project | M7/M8 | P1 | Основное окно объединяет native snapshot, sidecar, диагностические переходы и **отдельные** editable Graph/Advanced JSON | Явное отличие source vs draft; отмена/повтор, безопасное сохранение, конфликтные JSON изменения обработаны |
| **PR #35** — кандидат `0.6.x` offline-preview | M3–M8/MX | P1 | Согласованный release candidate с changelog, hash/source/legal notices, Windows checks, пользовательским smoke-отчётом | Публикация **только по решению владельца**; CI ≠ OEM bench; не объявлять industrial-ready |

**Зависимости:** PR #28/#29 — security foundation; #30 — база для #33; #31 зависит от текущего Native Workspace, может идти параллельно #30; #32 — до #34; #33 и #32 — до #34; #35 — после принятых функциональных PR и ручной проверки Windows. Если проверка выявит дефект — выпуск откладывается, создаётся отдельный bugfix PR; номера не являются жёстким календарём.

## 3. Что требуется от инженерного результата каждого этапа

1. **Контракт:** что допустимо делать с source, draft и сторонними файлами; каковы отказ и recovery; поддерживаемое семейство/версия контроллера.
2. **Provenance:** `Documented`, `SyntheticTested`, `ManualDerived`, `OEMBenchVerified`, `Authorized` — раздельные поля, ни одна метка не выводится из другой автоматически.
3. **Сохранность:** `original_bytes_before == original_bytes_after`; SHA-256 до/после открытия/повторного открытия. Для opaque — точный span исходника и источник ошибки.
4. **Тесты:** unit/regression/negative, frontend build, Rust `--locked`, docs check, отдельный Windows Tauri runner при затрагивании IPC/filesystem.
5. **Ограничения:** непроверенные форматы, покрытие контроллеров, Windows smoke, real OEM bench — явно `NOT RUN`, а не `PASS`.
6. **Поставка:** source tag, checksums, GPL source and notices; публичная публикация тестов без клиентской программы/конфигурации.

Подробный checklist: [verification_gates.md](verification_gates.md), [development_workflow.md](development_workflow.md). Текущее фактическое состояние — [project_status.md](project_status.md).

## 4. Отдельная исследовательская линия (параллельно)

**M1: реальные CIOPRG/IONAME (закрытая обработка).** Для каждого разрешённого файла записывать семейство DX100/DX200/YRC1000, firmware, способ экспорта, SHA-256, encoding/newlines, source section map, подтверждение OEM tool, статус распространения. Приватный корпус не является содержимым публичных GitHub Issues/PR/Actions. Публично допускаются только самостоятельные синтетические минимальные кейсы. Следовать [controller_fixture_intake.md](controller_fixture_intake.md).

**M6: семантика.** EN/scan order, flags, byte/word, TMR/CNT/GSTR/GOUT и защищённые разделы разбирать по OEM-ревизиям и разрешённым стендовым наблюдениям. Без этих данных статический анализ остаётся предположительным, а `reference_semantics` — отдельной справкой.

**M10: испытания на стенде.** Только отдельная разрешённая программа для неопасного изолированного оборудования и штатных OEM средств. Результаты для DX100, DX200 и YRC1000 не переносить автоматически между поколениями. [Матрица bench](advanced_hardware_test_matrix.md).

## 5. Заблокированные направления

| Веха | Состояние | Что необходимо, прежде чем даже обсуждать разрешение |
| --- | --- | --- |
| M9 — scan simulation | НЕ ВЫПОЛНЕНО | Подтверждённый scan/time contract и OEM edge cases, отдельный design review |
| M11 — read-only HSES | ЗАБЛОКИРОВАНО | Отдельное решение владельца, верифицированный OEM-протокол, read-only risk assessment, изолированный стенд |
| M12 — native writer | ЗАБЛОКИРОВАНО | Полная доказанная грамматика, подтверждённые System/User boundaries и OEM round-trip, самостоятельное письменное разрешение |
| M13 — robot deployment | ЗАБЛОКИРОВАНО | M12, отдельные права, safety/recovery plan, OEM approval и явное решение владельца |

Ни документирование этих требований, ни успешные CI/Windows tests **не предоставляют** разрешения на HSES, native writer или deployment.

## 6. Контроль прогресса

Обновлять [project_status.md](project_status.md) **после фактического merge**, фиксируя SHA и CI. Отдельно показывать `implemented`, `synthetically tested`, `Windows CI`, `user smoke`, `OEM bench`, `authorized`. Исторические release notes оставлять неизменными. При расхождении документации с кодом — сначала проверять актуальные тесты и исходники, затем исправлять документы в новом PR.

## 7. Текущая работа PR #28

После [merge PR #27](https://github.com/uzbatsu-beep/MLAD/pull/27) baseline `main` — `bdb443eb65dc134edc2e11ffc3230c9dfa6257df`, [main validation SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44237125310). В отдельном **Draft PR #28** добавляется [Windows workflow](../.github/workflows/windows-security-acceptance.yml), NTFS exclusive-lock/junction Rust regressions, [synthetic harness](../scripts/windows-acceptance.ps1) и [ручная матрица проверки](windows_acceptance_pr28.md). Результат WindowsCI фиксируется только после успешного workflow на реальном SHA PR; `UserSmoke`, `OEMBench` и публикация релиза остаются `NOT RUN`/`BLOCKED`, без автоматического перехода в DONE.

## 8. PR #29 — File ID и staged-file guards (только Draft)

Фактический baseline после [merge PR #28](https://github.com/uzbatsu-beep/MLAD/pull/28) — `main` SHA `3c184cd36e076dc2e5c9aed7a663b601b4899dea`; [Linux](https://github.com/uzbatsu-beep/MLAD/actions/runs/44365314264) и [Windows](https://github.com/uzbatsu-beep/MLAD/actions/runs/44365314173) CI успешны. В PR #29 предложено усиление JSON filesystem identity/hardlink/staged-file integrity; документ [модели угроз и остаточных рисков](filesystem_security_pr29.md) отдельно фиксирует недоказанные directory-handle, ACL, TOCTOU и crash durability. Это не разрешение native writer/HSES/FTP. Ручной Windows UserSmoke остаётся `NOT RUN` до пользовательского протокола.
