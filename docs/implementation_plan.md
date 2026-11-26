<!-- MLAD-I18N:NAV:START -->
> 🌐 **Язык / Language:** [Русский](implementation_plan.ru.md) · [English](implementation_plan.en.md) · [Original / Исходный текст](implementation_plan.md). **Machine translation / Машинный перевод: editorial review required / необходима редакторская проверка.**
<!-- MLAD-I18N:NAV:END -->

# MLAD — план разработки v5

Стратегия C утверждена 26.11.2026. Стабильные ID V5-01…15; номера PR ориентировочны, не резервируются. История v4: [baseline](https://github.com/uzbatsu-beep/MLAD/blob/e291d414f2440e2e407c1e0fdadc248de3d9dc80/docs/implementation_plan.md).

`CIOPRG.LST` и `IONAME.DAT` остаются read-only. `HSES`/`FTP`, native writer и deployment остаются `BLOCKED`; `UserSmoke=NOT RUN`, `OEMBench=NOT RUN` до независимых доказательств. JSON-черновики не программы контроллера. Публичные fixtures синтетические или явно разрешённые; без закрытых производственных файлов и хешей.

## 1. Очередь

| ID | PR | Task | Результат |
| --- | --- | --- | --- |
| V5-01 | #50 | Roadmap v5 & Architecture Decisions | Утверждённый план, ADR, документация; без runtime. |
| V5-02 | #51 | Semantic Core v2 | Инвентаризация, AST/IR, source spans, контракты, без дублирования парсера. |
| V5-03 | #52 | Controller Profiles v2 | Диалекты, firmware, адресные пространства, evidence. |
| V5-04 | #53 | Signal Index | Ключи адресов, READ/WRITE/UNKNOWN, исходные ссылки, snapshot identity. |
| V5-05 | #54 | UI Foundation | Токены дизайна, темы, палитры и сохранение настроек. |
| V5-06 | #55 | Engineering Workspace | Оболочка IDE, layout registry, панели, вкладки, переключение и сохранение компоновок. |
| V5-07 | #56 | Ladder Viewer v3 | Виртуализированное многоцепное поле, масштаб, блоки, UNKNOWN, переходы. |
| V5-08 | #57 | Signal Inspector | Карточка из общего индекса, G2 Windows Preview по отдельному согласию. |
| V5-09 | #58 | Cross Reference v2 | Фильтры ролей и двусторонние переходы. |
| V5-10 | #59 | Dependency Graph | Потенциальные связи с provenance и unknown-барьерами. |
| V5-11 | #60 | Diagnostics Engine | Расширяемые правила, severity, объяснимые результаты. |
| V5-12 | #61 | Logic Tracing | Forward/backward, циклы, неизвестность, объяснения, G3. |
| V5-13 | #62 | Project Compare v1 | Byte/text и structural diff, неоднозначные совпадения. |
| V5-14 | #63 | Project Snapshots | История, эталоны, целостность, переносимость. |
| V5-15 | #64 | Engineering Preview | Интеграционные тесты, G4, условный Windows Preview и UserSmoke. |

## 2. Контрольные точки

| Gate | Приёмка |
| --- | --- |
| G1 | Существующие функции сохранены; общий source-linked core/profile/index, без второго парсера. |
| G2 | Рабочий Windows UI, восстановление тем и компоновок, многоцепная навигация, инспектор и unknown; независимый smoke. |
| G3 | Cross Reference и trace показывают исходные ссылки, потенциальные зависимости, unknown-барьеры, тесты циклов и множественных записей. |
| G4 | Compare/снимки не меняют исходники; интеграционные тесты и ручная приёмка отдельно согласованного Preview. |


## 3. Контракт рабочих компоновок

V5-05 вводит дизайн-систему, V5-06 типизированный layout registry и общую сессию проекта. UI-настройки отдельно от данных проекта. V5-07/08 — рабочий Ladder/Inspector. Остальные компоновки появляются с реальными модулями. Тесты: переключение/перезапуск/сброс, несохранённый черновик, фокус, отсутствующий модуль, устаревший снимок, неверные настройки. UI не меняет права доступа.

## 4. Последующие фазы

После G4 уточнить PR по обратной связи: M17 tri-state → память/шаг → время/TMR/CNT → регистры → сценарии; M18 Watch/trace/events/test с INCONCLUSIVE; M19 отчёты; M20 локальный fleet; M21 опциональный AI. Исследования и парсер параллельно. Это не разрешает online-функции.

## 5. Definition of Done

Каждый PR: scope, позитивные/негативные тесты, регрессии, сохранность источника, явное unknown, применимый CI на финальном SHA, документация RU/EN, ограничения, отдельное согласие на merge. Docs-only CI по существующей схеме; пропуск runtime не равен PASS. GUI smoke и OEM bench независимы. Выпуск, Ruleset и доступ к контроллеру требуют отдельных решений.

[Roadmap](roadmap.md) · [Architecture](architecture.md) · [Verification gates](verification_gates.md).
