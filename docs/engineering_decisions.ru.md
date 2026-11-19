<!-- MLAD-I18N:EDITORIAL:restored 2026-11-19 from predominantly Russian original -->
> **Язык / Language:** [Русский](engineering_decisions.ru.md) · [English](engineering_decisions.en.md) · [Исходный текст / Original](engineering_decisions.md)
>
> **Примечание:** перевод не заменяет исходную спецификацию; русскоязычный оригинал сохранён без машинного искажения. / Technical source fidelity retained; OEM verification remains independent.

# MLAD — реестр утверждённых инженерных решений

**Версия реестра:** 18.11.2026. Этот документ фиксирует согласованные ограничения и разделяет их с ещё не принятыми предложениями. Основание: [дорожная карта v3](roadmap.ru.md) и [архив v2](roadmap_v2_archive.ru.md), [текущий статус](project_status.ru.md), [план PR](implementation_plan.ru.md). Решение о merge конкретного PR всегда принимается отдельно.

## 1. Стабильные решения D1–D9

| ID | Согласованное решение | Следствие для разработки |
| --- | --- | --- |
| D1 | **Сбалансированный путь D:** короткий safety baseline → Native Workspace и навигация, реальные fixtures параллельно → parser/semantics → Unified IDE | Выбор архитектурной последовательности; UX не ждать полной OEM-грамматики, но не заявлять ложную совместимость |
| D2 | `CIOPRG.LST` — **только read-only**, редактируемые данные и sidecar отдельно | Не перезаписывать/нормализовать оригинал; неизменные байты и SHA-256, любые изменения только в отдельных JSON |
| D3 | В Native Workspace блоки представлены **компактным списком с раскрытием**, выбранная цепь — в центре | Все распознанные цепи доступны, исходный текст — постранично; не перегружать полотно сотнями тяжёлых диаграмм |
| D4 | Неизвестное показывать как исходный текст, без догадок о семантике | Состояния `recognized`, `partially supported` и `opaque`, исходные spans и явные warnings |
| D5 | Реальные выгрузки — закрытый корпус, публично только разрешённые/синтетические фикстуры | Нет реального `CIOPRG.LST`/`IONAME.DAT`, hashes и производственных IP в PR/CI |
| D6 | Синтаксис и семантика подтверждаются **раздельно** | `TypedCandidate` не означает исполнимый оператор; `ManualDerived` не означает OEM scan |
| D7 | Сначала полностью автономный read-only инструмент; HSES — отдельное будущее решение | В `main` FTP/HSES runtime отключён, нет телеметрии и разрешений на запись |
| D8 | Offline MVP `0.6.x`: Native Workspace, навигация, диагностика, сохранность SHA-256 | Acceptance по функционалу и smoke, не по наличию OEM writer |
| D9 | Приоритет: воспроизводимые npm/CI и безопасность, затем workspace/парсер/unified IDE | Изменения только через маленькие PR и негативные тесты, без автоматического слияния |

**Лицензия:** `GPL-3.0-only`; правообладатель **собственных оригинальных материалов MLAD** — Фёдор Сергеевич Кочедыков (физическое лицо), © 2026. Это **не** заявление о владении OEM/сторонними материалами. [licensing.md](licensing.ru.md).

## 2. Непересматриваемые без отдельного решения ограничения

- Native export/write `CIOPRG.LST` запрещён; JSON draft не маскируется под OEM программу.
- HSES/FTP, robot upload, live I/O и изменение System/User запрещены.
- `PART 1/2` нельзя объявлять установленными правами System/User.
- Для каждого controller/firmware подтверждение формы и результата требуется отдельно.
- Запрос на подключение к реальному роботу **не подразумевается** из разработки offline UI.
- Все PR тестируются; **merge исключительно по согласию владельца**.
- Исторические release notes и опубликованные артефакты не переписываются задним числом.

## 3. Открытые решения (не считать утверждёнными)

| Вопрос | Нужные данные | Текущий статус |
| --- | --- | --- |
| Format `*.mlad.json` sidecar, schema version, миграции | Схема+prototype+negative tests, связь по SHA | Предложение в [плане](implementation_plan.ru.md), не реализовано |
| Проверка/коррекция native grammar DX100/DX200/YRC1000 | Права на fixture, firmware и OEM manual | Исследование M1/M5, без разрешения на native writer |
| Формальное правило cross-references и static dataflow | Source spans, ambiguities, evidence levels | Не проверенный scan model |
| Windows file trust: handles/ACL/races | Threat model, reproducible Windows tests | M2 PARTIAL; PR #24–#29 снизили часть рисков, но last-check→rename/ACL/recovery открыты |
| Release `0.6.x` | Windows UserSmoke, legal/source/assets, approval | Кандидат, не объявлен; условно PR #37 |
| Read-only HSES | Separate owner approval, OEM read spec, isolated rig | BLOCKED |
| Native writer и deployment | OEM verified fidelity, safety review, отдельное согласие | BLOCKED |

**Новый план PR #31–#37 — предложение**, он не создаёт D10 и не утверждает автоматически sidecar schema, набор новых opcodes или дату release. Прежде чем менять архитектуру или ограничения, требуется отдельный ADR/согласование.

Любая смена D1–D9 оформляется через отдельный ADR/PR с мотивировкой, рисками и тестами. Новое решение не действует, пока владелец не утвердил его.

[Матрица проверок](verification_gates.ru.md) · [Процесс PR](development_workflow.ru.md) · [Текущая модель угроз](filesystem_security_pr29.ru.md)
