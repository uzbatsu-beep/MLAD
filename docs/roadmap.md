<!-- MLAD-I18N:NAV:START -->
> 🌐 **Язык / Language:** [Русский](roadmap.ru.md) · [English](roadmap.en.md) · [Original / Исходный текст](roadmap.md). **Machine translation / Машинный перевод: editorial review required / необходима редакторская проверка.**
<!-- MLAD-I18N:NAV:END -->

# MLAD — Roadmap v5.0: Engineering Workbench

Утверждено владельцем 26.11.2026: все 11 направлений, стратегия C (сбалансированная), единая модель сигналов, функциональные Preview, специализированные компоновки, независимый офлайн 1.0. Это решение о направлении, не о слиянии PR.

База: PR #49 слит, `main` `e291d414f2440e2e407c1e0fdadc248de3d9dc80`, [CI PR #49](https://github.com/uzbatsu-beep/MLAD/actions/runs/44906615882) SUCCESS. Последний документированный Windows pre-release `v0.5.0-rc.10-unified-offline`. История v4: [roadmap](https://github.com/uzbatsu-beep/MLAD/blob/e291d414f2440e2e407c1e0fdadc248de3d9dc80/docs/roadmap.md), [plan](https://github.com/uzbatsu-beep/MLAD/blob/e291d414f2440e2e407c1e0fdadc248de3d9dc80/docs/implementation_plan.md).

## 1. Продукт и границы

Независимая офлайн-среда понимания, анализа, диагностики, сравнения, моделирования и сопровождения Yaskawa Motoman Concurrent I/O. Аудитория: АСУ ТП, робототехника, сервис, интеграция и обучение. DX100, DX200, YRC1000; сохраняем Rust/Tauri/React/TypeScript. Не расширяем область до универсальной PLC IDE, других марок, 3D-симуляции, обязательного сервера или облака.

`CIOPRG.LST` и `IONAME.DAT` остаются read-only. `HSES`/`FTP`, native writer и deployment остаются `BLOCKED`; `UserSmoke=NOT RUN`, `OEMBench=NOT RUN` до независимых доказательств. JSON-черновики не программы контроллера. Публичные fixtures синтетические или явно разрешённые; без закрытых производственных файлов и хешей.

## 2. Этапы M0–M21

| ID | Milestone | Priority | Содержание / целевой результат |
| --- | --- | --- | --- |
| M0 | Governance & Build | Continuous | CI, lockfiles, лицензии; обязательный ci-gate требует отдельного согласования. |
| M1 | OEM Knowledge & Evidence | P0 | Реальные приватные fixtures, документация, firmware, доказательства по утверждениям. |
| M2 | Filesystem & Project Reliability | Continuous | Неизменность источников, backup/recovery; ACL, TOCTOU, многофайловая атомарность открыты. |
| M3 | Engineering Workspace | P0 | IDE, панели, вкладки, многоцепное Ladder, специализированные компоновки. |
| M4 | Controller Dialect Engine | P0 | DX100/DX200/YRC1000, lossless parser, покрытие по профилям, unknown явно. |
| M5 | Navigation & Signal Index | P0 | Единый индекс, адреса, source spans, двусторонняя навигация. |
| M6 | Static Analysis & Diagnostics | P0 | Типизированные операнды, READ/WRITE/UNKNOWN, объяснимые правила. |
| M7 | Portable Sidecar v2 | P1 | Символы, группы, комментарии и закладки, явная перепривязка при смене SHA. |
| M8 | Unified Project Management | P1 | Проекты, снимки, относительные пути, эталоны и история, переносимость. |
| M9 | Windows User Acceptance | Continuous | Независимый ручной UserSmoke каждого функционального Preview. |
| M10 | OEM Bench Verification | Parallel | Команда, модель, firmware, сценарий и результат; без необоснованного обобщения. |
| M11 | Read-only Online Monitoring | BLOCKED | Отдельный OEM/owner допуск HSES/FTP и проверка протокола. |
| M12 | Native Writer & Compiler | BLOCKED | Применимая грамматика, OEM round-trip, семантика и права записи. |
| M13 | Robot Deployment | BLOCKED | Отдельный аппаратный и процедурный допуск; M12 не разрешает загрузку. |
| M14 | Signal Intelligence | P0 | Карточка, Cross Reference, обозреватель, группы, словарь, множественные записи без ложных диагнозов. |
| M15 | Project Compare | P0 | Byte/text → structural → ограниченный semantic diff → impact/fleet; неоднозначность явно. |
| M16 | Logic Tracing | P0 | Forward/backward slicing, дерево условий, циклы, Why TRUE/FALSE только в заданной модели. |
| M17 | Virtual Laboratory | P1 | Tri-state, виртуальная память, шаги, время, TMR/CNT, регистры и сценарии поддерживаемого подмножества. |
| M18 | Trace & Test | P1 | Watch Table, осциллограммы, курсоры, триггеры, события, PASS/FAIL/INCONCLUSIVE. |
| M19 | Documentation Engine | P1 | Отчёты сигналов, Ladder, диагностики, diff и тестов; CSV/HTML/PDF/SVG/JSON по назначению. |
| M20 | Robot Fleet | P2 | Локальный каталог предприятие–участок–комплекс–робот–контроллер–проект, снимки и знания. |
| M21 | Engineering AI | P2 | Опциональные локальные LLM, объяснения по структурам и OEM evidence, отдельно от детерминированных выводов. |

M0–M13 сохраняют историю; M14–M21 расширяют функции. Реализация baseline частичная, новые функции запланированы; приоритет не означает готовность.

## 3. Сбалансированные фазы

| Phase | Milestones | Gate / target |
| --- | --- | --- |
| 00 | M0–M13 | Existing Offline Preview |
| 01 | M1/M4/M5/M6 | G1 Semantic Core |
| 02 | M3/M5/M14 | G2 Workspace Preview |
| 03 | M6/M14/M16 | G3 Diagnostics Preview |
| 04 | M7/M8/M15 | G4 Engineering Preview |
| 05 | M17 | Simulation Preview |
| 06 | M18 | Engineering Test Suite |
| 07 | M19/M20 | Offline Engineering Suite |
| 08 | M21 | AI-assisted Workbench |
| 09 | M11/M12/M13 | BLOCKED |

M0/M1/M2/M9/M10 параллельно. Выпуск и OEM-допуски независимы. 1.0 не зависит от online engineering.

## 4. Специализированные рабочие компоновки

| Workspace | Компоновка |
| --- | --- |
| Ladder Engineering | Дерево, многоцепное поле, инспектор, исходник. |
| Signal Intelligence | Таблица, фильтры, карточка, Cross Reference. |
| Diagnostics | Диагностика, граф, дерево условий, ссылки. |
| Project Compare | Два снимка, diff и инспектор изменений. |
| Virtual Laboratory | Ladder, виртуальные I/O, сценарии, управление моделью. |
| Trace & Test | Watch, трассы, события и результаты тестов. |
| Robot Fleet | Каталог, паспорт робота, снимки. |
| Documentation | Выбор отчёта, просмотр, экспорт. |

Общий контекст проекта, снимка и выбора. Переключение без переоткрытия источника и потери несохранённых изменений; отдельные настройки размеров, видимости, вкладок и пресетов, восстановление умолчаний. Нереализованные модули явно недоступны. Компоновка не граница полномочий.

| ID | Требование |
| --- | --- |
| UI-01–03 | Светлая/тёмная темы, палитры, сохранение настроек. |
| UI-04–09 | Узкая навигация, дерево слева, инспектор справа, вкладки, диагностика снизу, resize/hide. |
| UI-10–12 | Виртуализированное многоцепное поле, клавиатура, RU/EN. |
| UI-13–18 | Сохранённые компоновки, split view, несколько проектов, Command Palette, справка, fullscreen. |
| UI-19–20 | Настраиваемые клавиши, отдельные окна и мониторы. |

TRUE зелёный, FALSE красный, UNKNOWN нейтральный, UNSUPPORTED янтарный; SELECTED независимый акцент, текст/форма дублируют цвет. Офлайн без заданного сценария UNKNOWN. Все распознанные цепи доступны в одном поле; TMR/CNT/GSTR/GOUT только поддерживаемые формы; F2/двойной щелчок редактирует отдельный черновик.

## 5. Качество и доказательность

Зрелость L0 Planned, L1 Prototype, L2 Implemented, L3 Automated Tested, L4 User Accepted, L5 OEM Verified (только применимые утверждения о контроллере). Независимые признаки evidence могут сочетаться: `Unknown`, `TypedCandidate`, `ManualDerived`, `RealFixtureObserved`, `SyntheticTested`, `OEMBenchVerified`. Наблюдение реального файла не доказывает исполнение.

Предварительные цели: 10/50/100 тыс. физических строк; 100% классифицированных строк включая unknown и 100% сохранённых байтов; индексный поиск/переход ≤150 мс; целевые 60 FPS. Сначала измерить baseline на типовом ПК. Детерминированные сценарии, без потери аннотаций при переоткрытии.

## 6. Исследования и версии

R-01…R-11: реальные CIOPRG, IONAME, память/firmware, OEM-семантика, инженерные IDE, статический анализ, diff, детерминированное моделирование, trace/test, HSES/FTP (только исследование), локальные LLM. Результаты переводить в спецификации, fixtures и задачи.

Версии ориентировочно: 0.5.x foundation; 0.6.x core/workspace; 0.7.x intelligence; 0.8.x compare/history; 0.9.x lab/test; 0.10.x documentation/fleet; 0.11.x AI; 1.0.0 stable offline. Отдельное согласование каждого выпуска и merge.

[Implementation plan](implementation_plan.md) · [Architecture](architecture.md) · [Decisions](engineering_decisions.md) · [Verification gates](verification_gates.md).
