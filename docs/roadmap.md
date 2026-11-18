# MLAD — дорожная карта v2: Native CIO Workspace → Engineering IDE → Verified Controller Integration

**Актуализация:** 18.11.2026; `main` `2092db1825880d251c87efcb7c331d78cb5e4d28`, PR **#1–#26 merged**, [CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44228204776); Windows [rc.9 опубликован](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference) и не содержит PR #24–#26. [План следующих PR](implementation_plan.md).
**Статус:** базовый Native Workspace и read-only parser работают; пять команд типизированы и снабжены автономными OEM-reference векторами, но scan/dataflow, System/User, стенд и native writer **не подтверждены/заблокированы**. [Текущий статус](project_status.md).

## 1. Продуктовая цель и ограничения

**Цель:** единая инженерная IDE для Yaskawa Concurrent I/O (DX100, DX200, YRC1000), где можно: открыть *настоящий* `CIOPRG.LST` в основном рабочем поле; увидеть полный исходный текст, цепи и функциональные блоки; найти сигналы, перекрёстные ссылки и диагностику; отдельно создать и проверить редактируемый офлайн-проект; в дальнейшем — после доказанной совместимости — выполнять подтверждённые операции чтения и, только при отдельном разрешении, изменения на оборудовании.

**Инварианты:**
- `CIOPRG.LST` **immutable read-only source**; UTF/BOM, окончания строк, неизвестные секции, байты и SHA-256 не изменять. Ни один GUI workflow не перезаписывает оригинал.
- Полностью распознанное, частично распознанное и **opaque/unknown** отображаются **раздельно**. Графика не должна выдумывать структуру управления.
- `*.graph.json` и `*.advanced.json` пока самостоятельные изменяемые модели, **не эквивалентные** native CIOPRG.
- TRUE/FALSE/UNKNOWN в текущем UI — локальные ручные входы/модели, **не HSES телеметрия** и не доказательство состояния физического выхода.
- Реальный импорт для просмотра, read-only сетевые операции, native writer и deployment — **разные** разрешения и разные safety gates.
- Приоритет — **реальные исходники и воспроизводимость**, а не снятие предупреждений ради «зелёного» отчёта.

## 2. Зафиксированный baseline

| PR | Реализовано и слито в `main` | Не следует считать доказанным |
|---|---|---|
| #1–#5 | Профили памяти, lossless, Boolean AST, JSON Graph Draft и TMR/CNT/GSTR/GOUT | Нет native writer и OEM-измерений |
| #6–#9 | Windows preview, multi-rung, inline JSON, аудит документации | CI ≠ hardware validation |
| #10–#12 | Roadmap v2, GPL-3.0-only, npm lockfile/`npm ci` | Юридический и security аудит открыты |
| #13–#14 | Удалён неподтверждённый HSES/FTP runtime, source paging/search | Безопасность filesystem ещё требует аудита |
| #15–#17 | Native Workspace, IONAME overlay, системные Open dialogs, компактный UI | OEM IONAME mapping и writer не подтверждены |
| #18 | Post-END ALARM/MESSAGE как данные, группировка diagnostics | Не универсальная OEM-грамматика |
| #19 | TypedCandidate MOV/ADD/SUB/SHL/SHR | Не исполняемые операции |
| #20 | Статические гипотезы M-ссылок | Не реальный поток данных/scan |
| #21 | Справочник EN/Carry/Zero/Error и изолированные reference векторы | Нет стендового подтверждения |

**Историческое наблюдение по ранней Windows-сборке (17.11.2026):** пользователь открыл экспорт `CIOPRG.LST` в Inspector. На предоставленных скриншотах показано **1396 строк, 13189 байт, 215 unknown/opaque, 64 булевы цепи**, предупреждения по MOV/ADD и визуальные блоки CNT/GSTR/GOUT. Это **диагностическое наблюдение по одному файлу**, а не одобренный открытый golden fixture. Количество/применимость инструкций для других controller/firmware не обобщать. До работы с байтами нужен разрешённый оригинальный файл и provenance, см. [controller_fixture_intake.md](controller_fixture_intake.md).

## 3. Этапы разработки и зависимости

Обозначения: **P0** — блокирующий фундамент; **P1** — ключевая функция офлайн IDE; **P2** — углубление проверок/чтения; **P3** — потенциальные операции записи только после отдельных допусков. В статусах ниже учитываются слитые PR #1–#26; `DONE` означает выполненный программный этап, **не** аппаратную верификацию.

| ID | Этап | Приоритет | Зависит от | Проверяемый результат | Статус |
|---|---|---|---|---|---|
| M0 | Baseline PR #1–#26 | DONE | — | В main объединены #1–#26, основной CI успешен | DONE: main `2092db18`, docs sync — отдельный PR #27 |
| M1 | Корпус реальных `CIOPRG.LST` | P0 | Разрешение на получение/хранение | Hash+metadata+OEM evidence, закрытые golden fixtures DX100/DX200/YRC1000, synthetic public regression | PARTIAL: пять закрытых исходников изучены; нет полной controller/firmware/OEM evidence |
| M2 | Безопасность и воспроизводимость | P0 | M0 | HSES/FTP runtime удалён; lockfiles; #24 staged JSON, #25 CSP/path checks, #26 file identity/conflict guard | PARTIAL: Windows CI успешен; user smoke, TOCTOU/directory handles, ACL/backup transaction открыты |
| M3 | Native Offline Workspace | P0 | M0, интерфейс чтения M1 может расширяться параллельно | Открытие `CIOPRG.LST` в **главном рабочем окне**, полностью read-only, без потерь байтов | DONE (read-only): PR #15–#17 |
| M4 | Навигация и диагностика native файла | P0 | M3 | Полный исходник, фильтры/поиск/переход к строке, cross-links, компактные блоки/ошибки | PARTIAL: paging/search/diagnostics; расширенные cross-ref/UX открыты |
| M5 | Typed Syntax расширение | P1 | M1, M3/M4 | MOV/ADD/SUB/SHL + иные встреченные команды: typed operands, source spans, неизвестное opaque | PARTIAL: пять typed opcodes; иные команды unknown |
| M6 | Semantic / Dataflow Analysis v2 | P1 | M5, M1 | READ/WRITE refs, конфликтные записи, таймеры/счётчики, неполные ветви, trace source→graph | PARTIAL: Boolean AST + static M-links + reference vectors; scan/dataflow unknown |
| M7 | Unified Offline Engineering Project | P1 | M3, M6 | Сессия `native snapshot + sidecar + separate editable drafts`; поиск, заметки, сравнение изменений | OPEN: native snapshot и local annotations есть, portable sidecar не создан |
| M8 | Редактор и статическая проверка инженерного проекта | P1 | M7, M6 | Общие компоненты Ladder/advanced, история изменений, правила адресации, диагностика до сохранения | OPEN: отдельные JSON drafts есть; единого native engineering project нет |
| M9 | Воспроизводимая offline-модель | P2 | M6, M1, OEM evidence | Тест-кейсы логики и ограниченные stateful сценарии с явно описанным scan/time контрактом | OPEN: offline reference vectors ≠ OEM scan simulation |
| M10 | OEM bench validation и сравнительная матрица | P1–P2 | M1, M5/M6, разрешённый стенд | Сопоставление MLAD↔OEM Ladder Editor по каждому controller/firmware, измеренные результаты | NOT DONE: доступа к стенду нет |
| M11 | Read-only HSES диагностика | P2, независимо от writer | M2, OEM HSES specification, M10 для I/O mapping | Проверяемое read-only чтение, timestamps/quality/staleness, безопасный UNKNOWN | BLOCKED: HSES read-only не разрешён |
| M12 | Native CIOPRG writer + verified round-trip | P3 | M1, M5–M10, System/User boundaries | Доказанная точность native формата и независимая OEM проверка; отдельное решение о доступе | BLOCKED: native CIO writer не разрешён |
| M13 | Контролируемый deployment и recovery | P3 | M12, risk assessment, OEM approvals | Отдельно разрешённая процедура загрузки, backup, rollback, право записи, трассировка | BLOCKED: deployment не разрешён |
| MX | UX/Quality/Release engineering (сквозной поток) | P0–P2 | В каждом этапе | Versioned releases, CI, Windows smoke, логирование, performance, подписывание, docs | ONGOING: CI / docs / Windows rc.9 |

**Новые источники (закрытая обработка, 17.11.2026):** локально изучены ещё пять пользовательских `.LST`-выгрузок без публикации файлов, хешей, адресов и конфигурации. Отмечены варианты заголовков, `//$M` и отдельные разделы после `END`; точные контроллеры/firmware и результаты OEM проверки не предоставлены. Для parser v2 подготовить синтетические тесты различающихся заголовков и недопустимости интерпретации ALARM/MESSAGE как Ladder.

**Не считать M1 и M2 препятствием для разработки read-only M3/M4:** разработка интерфейса может идти по безопасным синтетическим фикстурам и приведённым пользователем наблюдениям. Но для объявления «полноценного разбора» M1 обязателен.

### Фаза A — фундамент доверия (M1, M2; параллельно с M3)

**M1 — реальные данные:** принимать копии native exports с model, firmware, application, source hash, encoding и OEM outcomes; хранить в приватном корпусе при наличии прав. Не коммитить клиентские конфигурации, технологические IP/пароли/названия линий. Проверка неизменности `copy_original == source_bytes`. При анонимизации сохранять отдельные original и sanitized hashes. Полная грамматика не выводится из одной выгрузки.

**M2 — безопасность приложения и supply-chain:** удалить/изолировать прежний неподтверждённый код `network/mod.rs` из исполняемой поверхности; добавить тесты отказа FTP/HSES, CSP review, file access и символические ссылки/TOCTOU, атомарную запись с rollback для JSON, фиксированный npm lockfile+`npm ci`, правила версии/подписи Windows. Сеть и native writer остаются отключёнными.

**Acceptance:** ни один вызов, доступный UI, не инициирует сетевую запись; все входные файлы ограничены по размеру и корректно обрабатывают ошибку; CI и Windows smoke reproducible; нормальные/повреждённые LST не меняются.

### Фаза B — основное рабочее окно для настоящего файла (M3, M4)

**M3 — Native Offline Workspace:**
- Команда `Открыть проект → CIOPRG.LST (только чтение)` открывает **основную** рабочую область, не модальное окно инспектора.
- Unified view-model: `NativeSnapshot{source_hash, controller, source_lines, spans, recognized/opaque, diagnostics}`. Источник лежит отдельно от любых редактируемых данных.
- Слева дерево по `PART`, цепям, регистрам/переменным и типам блоков; **PART 1/2 — метки анализа, не проверенная схема прав System/User**.
- Центр — все достоверно распознанные цепи и блоки с компактным/расширенным видом; неполные и unknown сегменты явно помечены. Справа — свойства, подробный source fragment и предупреждения.
- Возвращение к файлу по хешу, прозрачное сообщение при изменении источника, без трансляции обратно в `.LST` и без edit callbacks.

**M4 — инспектор уровня инженерного инструмента:**
- Полный источник, а не лимит первых 600 строк UI: virtualized lines, поиск по мнемонике, `Mxxx`, `#XXXXX`, переход к физической строке/offset.
- Двунаправленная навигация source-line ↔ rung/AST/advanced block с точным диапазоном, показ raw/typed классификации.
- Группировка повторов диагностики (`MOV 101` и т.п.) с раскрытием конкретных строк; severity/confidence/support status.
- Табличный/компактный вид десятков GSTR/GOUT и CNT/TMR, раскрытие одного элемента; фильтры/закладки.
- Отчёт по покрытию **по категориям**: `fully supported / partially analyzed / opaque / malformed`; не считать «распознанные строки» доказательством полной семантики.
- Плавная работа на реальном файле порядка 1400 строк и на существенно более крупных фикстурах; отсутствие ложного состояния LIVE.

**Acceptance M3/M4:** открывается исходный YRC1000 файл в главном окне, все строки доступны; можно найти адрес/инструкцию и перейти к Ladder-цепи и обратно; неизвестные конструкции видны и не дорисовываются; повторное чтение сохраняет исходный SHA-256. Для DX100/DX200 до фикстур — только безопасная деградация.

### Фаза C — корректность интерпретации (M5, M6)

**M5 — Typed Syntax по реальным потребностям:** приоритет реальных неизвестных инструкций из наблюдения: `MOV`, `ADD`, `SUB`, `SHL`. Разделить **распознать оператор и типы аргументов** от **доказать эффект выполнения**. Подтвердить формат операндов и все образы по OEM manual и конкретным native fixtures. Другие opcodes добавлять по частоте/риску; unknown всегда остаётся opaque без отбрасывания байтов.

**M6 — Semantic IR / dataflow:** source span каждой операции, чтение/запись регистров и битов, циклы/переприсвоения, ветви, последовательность инструкций, связи TMR/CNT/GSTR/GOUT, предупреждения о конфликтах. Не считать сложную native конструкцию аналогом одной катушки. Разница между числом native `OUT` и числом построенных rungs должна быть объясняема и трассируема. Инструмент показывает confidence/coverage и отдельно `unsupported`.

**Acceptance:** golden tests на реальных разрешённых exports, отсутствие потери/подмены неизвестного, корректное построение только подтверждённого подмножества, найденные неверные/неопределённые writes блокируют дальнейшие семантические утверждения.

### Фаза D — единый офлайн-проект и инженерная разработка (M7, M8)

**M7 — sidecar session:** неэквивалентная native-файлу оболочка проекта, например `*.mlad.json`, которая **ссылается** на источник по hash и хранит UI layout, notes/bookmarks, controller profile, выбранные разделы и ручные test signals. Она **не содержит изменённый CIOPRG.LST**. Редактируемые черновики сохраняются отдельно. Сравнение двух оригинальных snapshot по сигналам/изменениям — read-only отчёт.

**M8 — создание и проверка логики:** единый canvas для булевых/advanced блоков; редактирование по двойному щелчку; Undo/Redo и корректная история advanced; каталог адресов, cross-ref, конфликты output, тесты диапазонов, запрет записи в защищённые участки. Возможность «скопировать подтверждённую цепь в JSON-черновик» — **отдельная явная операция** только для полностью поддерживаемого подмножества; нет массового автоматического конвертера native файла.

**Acceptance:** импортированный оригинал остаётся неизменным; notes сохраняются отдельно; JSON можно воспроизвести независимо; ошибки и unsupported конструкции не замалчиваются.

### Фаза E — независимые проверки и техника Yaskawa (M9, M10)

**M9 — offline simulation:** детерминированные тестовые сценарии для булевой логики, timer/counter/byte copy и дополненных инструкций; явно заданные ticks, front edges, reset priority, memory semantics. Отдельные уровни `synthetic verified` vs `OEM measured`. При неизвестной семантике не моделировать поведение по догадке.

**M10 — hardware acceptance:** точная версия controller/firmware, исходник OEM, утверждённый тестовый участок, safe I/O map, trace и измерения. В тестируемом оборудовании применять **штатные средства Yaskawa**, не MLAD native writer. Сверять синтаксис, граф, значения M-регистров, таймеры, счётчики, групповые передачи, границы доступа, ошибки и рестарт. Никаких safety/motion сигналов в тестовой карте.

**Acceptance:** протокол по DX100/DX200/YRC1000 с результатами, датами, checksum, pass/fail и rollback; CI **не** закрывает аппаратные gates.

### Фаза F — ограниченное аппаратное взаимодействие (M11–M13)

**M11 — только чтение HSES:** независимо подтверждённый vendor protocol, exact request/response, временные метки и качество, таймауты/отключения/конфликты с существующими данными; UI чётко отличает `OFFLINE`, `LIVE`, `STALE`, `UNKNOWN`. Физическую запись, FTP, servo/motion не включать.

**M12 — native serializer (отдельное управленческое решение):** только при полной подтверждённой грамматике, защищённых System/User boundaries, побайтовых golden round-trips, сравнении OEM validator и отсутствии потери unsupported. Сначала локальный *export to an explicitly named new file*, не overwrite и не network upload. Пока prerequisites не выполнены — **BLOCKED**.

**M13 — controlled deployment:** возможен только после M12, внешней оценки рисков, спецификации официального метода, backup, независимого сравнения, dry-run, rollback и аппаратного разрешения. Работа с System Ladder, входами безопасности, servo permissives и motion исключается из обычных пользовательских сценариев.

## 4. Предлагаемые пути после обсуждения

| Вариант | Первые этапы | Главный выигрыш | Ограничение |
|---|---|---|---|
| **A — Workspace-first** | M3 → M4; M1/M2 параллельно | Быстро получить удобный read-only инженерный просмотр настоящих LST | Начальное число opaque остаётся большим |
| **B — Parser-first** | M1 → M5 → M6; M2 параллельно | Быстрее повышается достоверность typed разборов и dataflow | Пользователь продолжает работать в неудобном Inspector |
| **C — Reliability-first** | M2 → M1 → M3/M4 | Меньше риска security/сборки перед расширением функций | Позднее появится заметный новый UX |
| **D — Баланс (рекомендуется к обсуждению)** | Короткий M2 safety baseline + M3/M4; M1 параллельно; M5/M6 по оригиналам | Ранний полезный workspace без заявлений о полной грамматике; к parser этапу накапливаются fixtures | Требует дисциплины в статусах поддержки и едином source-span контракте |

**Рекомендация для обсуждения:** D. Открывать native `CIOPRG.LST` в главном окне и делать поиск/связи, **пока** собираются файлы и фиксируются тесты; но не объединять это с необоснованным разрешением редактирования native файла.

## 5. Решения, которые нужно принять до кодового PR

| ID | Решение | Варианты / предлагаемое значение | Влияние |
|---|---|---|---|
| D1 | Главная цель ближайшего релиза | Просмотр native (A) / полнота parser (B) / безопасность (C) / гибрид (D) | Порядок PR |
| D2 | Работа с исходным `CIOPRG.LST` | **Только чтение, редактируемый sidecar отдельно** | Граница доверия |
| D3 | Как размещать блоки в native workspace | **Компактный список + раскрытие**, выбор цепи в центре | Производительность/UX |
| D4 | Как показывать unknown | **Текст source + span + unsupported**, без графической реконструкции | Честность анализа |
| D5 | Набор корпусов | Закрытые реальные fixtures, публичные только синтетические/разрешённые | Конфиденциальность |
| D6 | Достоверность инструкции | Сначала grammar, затем verified effects; никакого «поддерживается» только по имени | Риски MOV/ADD/SUB/SHL |
| D7 | Аппаратная связь | **Read-only HSES отдельно и позднее**; writer/deploy без одобрения не делать | Безопасность |
| D8 | Что считать offline MVP | Полный read-only source, navigation ↔ diagrams, filtered diagnostics, exact hash | Критерий завершения |
| D9 | Технический долг до новых UX | Минимальный safety barrier сейчас, остальное постепенно | Объём первого PR |

## 6. Что НЕ является выполненным и не входит в первый UI-MVP

- Полная native грамматика `CIOPRG.LST` и валидированный `CIOPRG.LST → editable project → CIOPRG.LST` круг.
- Достоверный System/User ownership по номеру `PART`.
- Живое чтение I/O или изображение actual coil states.
- Проверенная на роботах реализация MOV/ADD/SUB/SHL или реальных timer/counter scan semantics.
- Реальная компиляция OEM native, загрузка по FTP или HSES и использование любых защищённых System Ladder областей.
- Доказательство аппаратной безопасности/сертификация по результатам CI.

## 7. Управление работой и проверками

Каждый кодовый этап — отдельная ветка/PR после согласования, без слияния автоматически. PR содержит источник требований, изменённые контракты, позитивные/негативные Rust+TS тесты, Windows smoke checklist и ограничения. `npm run docs:check` проверяет внутренние Markdown-ссылки; первичные руководства и аппаратные замеры проверяются отдельно.

Документы: [architecture.md](architecture.md), [controller_fixture_intake.md](controller_fixture_intake.md), [cioprg_lst_spec.md](cioprg_lst_spec.md), [development_workflow.md](development_workflow.md), [advanced_hardware_test_matrix.md](advanced_hardware_test_matrix.md).

**Настоящий документ — карта решений, а не согласие на запуск M11–M13.**

### PR #19 — Typed Syntax (merged 17.11.2026)

- Предложено: `MOV`, `ADD`, `SUB`, `SHL`, `SHR` как типизированные, но **не исполняемые** формы; сохранение неизменных raw bytes/source spans и отчёт по opcode в Native Workspace.
- Не входит: семантическая эмуляция, flags, влияние на выходные цепи, изменение System/User, OEM writer, FTP/HSES.
- Валидация: синтетические Rust fixtures + документация + Windows rc7 preview. Проверка на настоящих файлах — только в локальном приватном корпусе, без публикации.

### PR #20 — Static evidence (merged 17.11.2026)

- Read-only, bounded inventory of syntactically typed source/destination operand positions; exact M-register match may propose *source-order-only* predecessor links.
- Any unsupported/malformed, known boolean/advanced, PART, END or post-END boundary breaks tentative continuity.
- No runtime values, arithmetic flags, scan-cycle semantics, control-flow simulation, hardware I/O or controller upload.
- OEM semantic verification остаётся отдельным gate: PR #20 слит, документальные формулы не заменяют измерений.

### PR #21 — OEM reference for five opcodes (merged 17.11.2026)

- Проанализированы MOV/ADD/SUB/SHL/SHR параллельно, по разделам YRC1000 R12 и DX100 Rev.0; DX200 всё ещё требует отдельного source verification.
- Отдельная ручная 16-bit reference-модель Rust с тестами пределов 0/16 и 255/256 и явно неопределёнными флагами.
- Справка по OEM в Native Workspace; никакой интеграции reference-ядра с реальным CIO, HSES/FTP, writer или scan engine.
- Windows rc.9 опубликован, PR #21 слит; испытания на контроллере не проводились.

## 8. Исторический маршрут после PR #21 и фактические PR #24–#26 (18.11.2026)

**Пока недоступны роботы:** документально сверить ревизии DX200, расширить negative/parser tests, разработать переносимый проверяемый sidecar для локальных комментариев, усилить cross-ref/diff и отдельно провести security review CSP/filesystem/JSON, third-party notices/SBOM. **Не** включать native writer или сеть.

**При появлении стенда:** пройти [матрицу испытаний](advanced_hardware_test_matrix.md), включая MOV byte=256, SHL/SHR n=0/16, EN/scan и флаги #51400/#51401/#51402 по каждому поколению. Только воспроизводимые OEM измерения повышают уровень confidence реальной семантики.

**Организация:** каждый будущий этап в отдельном Draft PR, с тестами и согласованием владельца до слияния. Разделы с вариантами приоритетов выше сохранены как история обсуждения, а не текущая задача по первому запуску UI.


### PR #24 — Atomic Offline JSON Draft Persistence (merged, 18.11.2026)

- Сохраняются только отдельные `*.graph.json`, `*.advanced.json`, legacy `*.json`; оригиналы `CIOPRG.LST` и `IONAME.DAT` всегда read-only.
- Перед записью ограничение размера 2 МиБ; запись во вновь создаваемый файл в той же директории, `sync_all`, переименование/замена; предыдущая версия при перезаписи переносится в `*.bak` через отдельный staging/rename.
- Защита от **прямых** symlink у JSON и backup; Rust-тесты на первоначальную запись, последовательные сохранения, отказ, oversized и symlink. Ошибка backup/rename не должна обрезать старый JSON.
- **Остаётся M2 PARTIAL:** backup и target не образуют единую транзакцию; Windows replacement зависит от правил файловой системы/ACL, нет межпроцессной блокировки, не устранены гонки parent path, hardlinks и TOCTOU; CSP `null` без изменений. Следующий security PR должен рассматривать доступ по file handle, directory trust/locks и CSP отдельно.
- Влияние на release: Windows binary `rc.9` не изменялся; нужен отдельный Windows acceptance smoke перед объявлением устойчивости поведения на Windows.


### PR #25 — Tauri CSP и parent-path safety (merged, 18.11.2026)

- CSP production: запрет удалённых scripts/connect, `unsafe-eval`, объектов, iframes, worker; разрешён только локальный `'self'` и `ipc: http://ipc.localhost`. `style-src 'unsafe-inline'` временно необходим существующему React/inline CSS. Vite dev CSP отдельно ограничен localhost:5173 + WebSocket для HMR.
- Файловые операции: отклонение явно обнаруженных symlink в родительских каталогах, `..` и direct symlink у native CIOPRG/IONAME. JSON read/write продолжают staging из PR #24, но проверяют всех текущих предков.
- Rust-тесты: обычные вложенные пути, symlink-parent, symlink-native, traversal, побайтовая сохранность исходника; Node security tests валидируют CSP/код. Windows MSVC и сборка Tauri проверены отдельным успешным CI для PR #25; пользовательский smoke остаётся.
- **Не заявлять полноту M2:** путь может быть заменён между проверкой и open/rename; недоверенные Windows junction/reparse points и hardlinks, concurrent writers, permission ACL и CSP runtime smoke проверяются отдельно. Без аппаратного интерфейса.


### PR #26 — Guarded File Snapshots and Conflict Detection (merged, 18.11.2026)

- Дескрипторные чтения CIOPRG/IONAME/JSON с 2/4/16 МиБ пределами, `symlink_metadata → File::metadata` сравнение ID (Unix dev/inode, Windows volume/index), проверка ID после чтения. Reparse point detection включая Windows junction.
- JSON staged/backup из PR #24 сохраняется; перед финальным `rename` байты ещё раз сопоставляются с исходными; при конфликте — отказ без перезаписи изменённого JSON. Новое создание тоже запрещает silent overwrite при появлении файла во время staging.
- Синтетические Rust-тесты: подмена между check/open, внешняя запись между backup/commit, конкурирующее создание, чтение/отказ linked parent, стабильные тесты оригинальных байтов.
- **Остаётся:** race между последней сверкой и финальным rename, небезопасные изменения каталогов/ACL, Windows junction, hardlinks, interprocess serialization и atomic backup transaction. Ручной Windows smoke после CI.

## 9. Актуальный рабочий backlog после PR #26

**Зафиксированный порядок:** документация PR #27 → Windows runtime/negative checks (предложение #28) → targeted filesystem hardening (#29) → synthetic parser corpus (#30) → Native navigation/diagnostics (#31) → portable sidecar (#32) → evidence-based typed analysis (#33) → unified offline IDE (#34) → кандидат `0.6.x` (#35). Номера #28–#35 предварительные; этапы создаются по одному PR, после согласования области работ.

Подробные зависимости, проверяемые результаты и ограничения — [implementation_plan.md](implementation_plan.md). Независимые тестовые gates — [verification_gates.md](verification_gates.md). Принятые решения D1–D9 — [engineering_decisions.md](engineering_decisions.md). **M11–M13 остаются BLOCKED** независимо от завершения этих PR.
