# MLAD — дорожная карта v3: Native CIO Workspace → Offline Engineering IDE

**Контрольная точка:** 18.11.2026, `main` = `f85ddfcce3063dd087425be981a9d2b7a16d6309`. PR #1–#29 merged. [Linux validation SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454) и [Windows Acceptance & Security Regression SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621) на этом SHA. Windows GitHub Release: [`v0.5.0-rc.9-oem-reference`](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference) — старый артефакт, **не содержит PR #24–#29**. Отдельное прохождение тестов на ПК пользователя и на роботе **не подтверждено**.

**Статус:** техническая дорожная карта; новые PR после #29 пока **предложены**, а не созданы, за исключением отдельного документационного PR #30 при его публикации. [Детальный backlog](implementation_plan.md) · [Фактическое состояние](project_status.md) · [Решения D1–D9](engineering_decisions.md) · [Историческая roadmap v2](roadmap_v2_archive.md).

## 1. Цель и продуктовый контракт

**MLAD (Motoman Ladder Analysis & Draft IDE)** — независимая OFFLINE/BENCH инженерная среда Yaskawa Concurrent I/O для профилей DX100, DX200 и YRC1000. Пользователь открывает экспортированный `CIOPRG.LST` в основном окне, изучает источник, цепи и блоки, ищет адреса/связи, классифицирует неопределённые фрагменты и ведёт **отдельные** редактируемые JSON-черновики, заметки и будущие проекты.

**Принципы неизменны:**

1. `CIOPRG.LST` и `IONAME.DAT` — только **read-only**, исходные байты и SHA-256 неизменны. Нет операции «исправить LST» или конвертации JSON обратно в native.
2. `supported / partially supported / opaque / malformed` различаются, а не дорисовываются как подтверждённая логика. `PART`/`END` и tail-данные не являются автоматически политикой System/User.
3. `*.graph.json`, `*.advanced.json` и legacy `*.json` — отдельные пользовательские проекты. Потенциальный `*.mlad.json` sidecar тоже **не** контроллерный формат.
4. MOV/ADD/SUB/SHL/SHR имеют типизированное и manual-derived **справочное** подмножество, но нет проверенного scan/EN/dataflow исполнения. OFFLINE TRUE/FALSE/UNKNOWN — не физические I/O.
5. HSES/FTP, live monitoring, native writer, robot deployment и автоматическое управление контроллером **BLOCKED**. Любое изменение границы требует независимого допуска владельца и OEM evidence; успешный CI такого допуска не выдаёт.
6. Приватные реальные экспортированные файлы, конфигурация контроллера и персональные/производственные сведения не публикуются в GitHub, Issues или CI. Публичные fixtures — только самостоятельно созданные synthetic или явно разрешённые.
7. Каждый технический этап — отдельный PR с позитивными/негативными тестами, доказательствами, CI и отдельным решением владельца о merge.

## 2. Фактический baseline после PR #29

| Группа PR | Реализованная база | Ограничение |
| --- | --- | --- |
| #1–#12 | Типы адресов, lossless read-only, Boolean AST, отдельные Graph/Advanced JSON, Windows preview, GPL-3.0-only, npm lockfile / `npm ci` | Ни scanner, ни native writer |
| #13–#18 | Удалён компилируемый HSES/FTP runtime; paging/search; единый Native Workspace, IONAME и локальные annotations; post-`END` данные | UX/cross-ref и валидность OEM схемы не полные |
| #19–#21 | TypedCandidate для пяти операций, static M-links, изолированные OEM manual-derived vectors | Нет подтверждённого исполнения/scan и firmware-level OEM bench |
| #22–#23 | Оформление исходников/архитектурного контура и Community Standards | Не аппаратная сертификация |
| #24–#26 | Staged JSON с `*.bak`; CSP; bounded guarded native/JSON reads, File ID checks, JSON conflict detection | Остаются directory races, ACL, crash recovery |
| #27 | Roadmap/verification/engineering decisions sync | Документы не закрывают функциональные пробелы |
| #28 | Постоянный Windows regression workflow, NTFS junction/lock tests, temporary unsigned EXE, WIN-01…WIN-12 checklist | WebView2 UserSmoke не выполнялся |
| #29 | File ID+bytes для JSON snapshots, hardlink rejection, staged File ID+SHA-256, Windows ADS/UNC/device path deny | Не полная TOCTOU/ACL/backup+target гарантия |

`GPL-3.0-only` и авторство оригинальных материалов MLAD остаются без изменений. Публикация кода не означает право включать в репозиторий материалы OEM/клиентов. Правовая база: [licensing.md](licensing.md).

## 3. Этапы M0–M13 — реальное состояние

Обозначения: **DONE** = подтверждённая программная реализация, **PARTIAL** = поддерживаемое подмножество, **OPEN** = ещё нет необходимой функции, **BLOCKED** = нужен независимый допуск. `SyntheticTested` ≠ `UserSmoke` ≠ `OEMBenchVerified`.

| Этап | Приоритет | Статус на `f85ddfc` | Критерий перехода |
| --- | --- | --- | --- |
| M0 — Git/CI/legal baseline | P0 | **DONE:** PR #1–#29, Windows+Linux CI успешны; branch `main` не защищён | Правила merge / опциональная Branch Protection отдельно |
| M1 — OEM source corpus | P0 | **PARTIAL:** приватная предварительная инвентаризация; firmware/OEM provenance не полный | Правомерные private fixtures + controller/firmware + OEM результат; synthetic public corpus |
| M2 — filesystem/security | P0 | **PARTIAL:** #24–#29 снизили выявленные риски | Windows UserSmoke + согласованные остаточные ACL/races/crash risks и дополнительные security reviews |
| M3 — Native Workspace | P0 | **DONE (read-only базовый):** #15–#17 | Стабильное окно чтения, без native save |
| M4 — навигация/диагностика | P0 | **PARTIAL:** paging, source/search, список рунгов/группировка | Надёжный source↔rung/block/diagnostic jump, фильтры и large-file UX |
| M5 — native typed syntax | P1 | **PARTIAL:** 5 opcodes + opaque для прочего | Синтетические edge cases + evidence-validated grammar increments |
| M6 — static semantic/dataflow | P1 | **PARTIAL:** Boolean AST, static M-links, manual-derived reference | Verified references и честное unknown, без fake scan |
| M7 — portable offline session | P1 | **OPEN:** local annotations есть, `*.mlad.json` нет | Versioned sidecar, hash binding, migration/conflict/error handling |
| M8 — unified engineering IDE | P1 | **OPEN:** разные редакторы JSON работают отдельно | Общая навигация, связка source/notes/drafts без изменённого native |
| M9 — scan simulator | P2 | **OPEN / NOT VERIFIED** | Формально проверенный OEM scan/time contract + bench |
| M10 — OEM bench | P1–P2 | **NOT RUN** | Отдельный разрешённый изолированный стенд, точная модель и firmware |
| M11 — read-only HSES | P2 | **BLOCKED** | Независимый owner decision + протокол и risk review |
| M12 — native CIO writer | P3 | **BLOCKED** | Полная OEM grammar/System/User и golden round-trip, отдельный допуск |
| M13 — robot deployment | P3 | **BLOCKED** | M12, независимая safety/recovery процедура, явно согласованные права |
| MX — release/UX/quality | P0–P2 | **ONGOING:** CI и rc.9, но новый релиз не выпускался | Manual Windows acceptance, legal/assets, воспроизводимые SHA |

## 4. Стратегия v3 — что делать в каком порядке

**Приоритет №1 — добиться надёжности открытия и понимания настоящих файлов, не объявляя неизвестный синтаксис подтверждённым.** Из первого пользовательского просмотра был выявлен большой объём `unknown/opaque` и неудобства чтения рабочего пространства. Инвестиция в синтетические parser fixtures, навигацию и трассируемую диагностику даёт больше пользы, чем преждевременное создание сканирующего симулятора или нативного writer.

**Поток A — доверие и качество источника (P0).** Завершить docs-синхронизацию, создать синтетический corpus (заголовки, `//$M`, `PART`/`END`, ALARM/MESSAGE tail, malformed lines, BOM/CRLF, opaque spans, границы размера). Приватные реальные программы только для **отдельного** provenance intake. Не публиковать их hashes, уникальные инструкции или производственные сведения.

**Поток B — удобная Native Workspace (P0; параллелен A).** Source/paging, поиск по адресам и именам, двунаправленные переходы `source↔rung↔block`, раскрываемые диагностики с точной физической строкой, индикатор частичного покрытия и сжатые представления большого числа блоков. Сначала удобный read-only инструмент, затем сложная семантика.

**Поток C — пользовательский offline проект (P1).** Versioned `*.mlad.json` для профиля контроллера, source SHA, bookmarks, local annotations, ссылок на черновики. Продумать проверку источника, conflict detection, schema upgrade, backup и portability до интеграции в UI. Не сохранять native code в sidecar.

**Поток D — доказательная типизация и статический анализ (P1).** Typed opcode grammar расширять по `проблема → OEM evidence → parser rule → negative test`. Делать cross-reference READ/WRITE и безопасные предупреждения без утверждений о физическом состоянии сигнала или фактическом scan order. Unknown остаётся unknown.

**Поток E — Unified Offline IDE (P1).** Свести Native Snapshot, annotations, sidecar и два JSON-редактора в одну последовательную пользовательскую модель. Любой перенос native→draft — только явное действие для независимо подтверждённого подмножества; по умолчанию native read-only.

**Поток F — Windows product acceptance и release engineering (сквозной).** Уже сейчас выполнить WIN-01…WIN-12 на тестовом Windows ПК вне производственной сети; оформить `UserSmoke PASS/FAIL/NOT RUN`. Прежде чем публиковать 0.6.x: обновить GPL/source notices, third-party license/SBOM review, unsigned status, archive/checksums и ручной протокол. Windows CI сам по себе релиз не санкционирует.

## 5. Фазовый порядок и решения

**Фаза 0 — PR #30:** привести все действующие MD в соответствие с `main` после #29 и утвердить backlog. **Docs-only**, не добавляет runtime и не закрывает Windows UserSmoke.

**Фаза 1 — PR #31 и #32 (параллельно при сохранённом safety baseline):** synthetic parser regression corpus и Native Workspace UX/diagnostics. Доказательства: байтовая идентичность source, полный доступ к строкам, точные переходы и гарантированно видимый opaque.

**Фаза 2 — PR #33 и #34:** versioned sidecar schema + типизированная грамматика выбранных нераспознанных паттернов; только на основании evidence, никакой автоматической семантики без подтверждения.

**Фаза 3 — PR #35 и #36:** source-linked static analysis и единый Offline Engineering Project. QA сценарии синтетические плюс private/manual проверки с разрешения владельца данных.

**Фаза 4 — PR #37 (условный):** Windows OFFLINE preview 0.6.x, после успешных CI, ручного smoke, legal review и отдельного решения владельца. При нарушении gate — дефектный/блокирующий PR, а не формальный релиз.

PR #31–#37 — **предварительные номера**, не созданные задачи/коммиты. Действующие объёмы и зависимости — в [implementation_plan.md](implementation_plan.md).

## 6. Точки контроля, которые нельзя скрывать

- **Семантика:** `typed valid`, `manual-derived` и `OEM-verified` — три разные метки. Пять справочных арифметических команд не образуют scanner.
- **Файлы:** Windows hardlink/ADS/UNC restrictions и File ID checks после PR #29 — полезная защита, но `check→rename`, parent directory handles, ACL inheritance, конкурентные writers и power-loss recovery остаются **OPEN**; [threat review](filesystem_security_pr29.md).
- **Windows:** [PR #28 acceptance protocol](windows_acceptance_pr28.md) — обязательный ручной этап; automation без WebView2 не переводит его в PASS.
- **Данные Yaskawa:** `PART 1/2` и System/User, а также IONAME→CIO map не выводятся из одного файла.
- **Правомочия:** HSES/read-only access, native writer и robot deployment — самостоятельные заблокированные решения. Разработка интерфейса их не активирует.
- **Управление:** `main` при последней проверке `protected=false`. Рекомендуется GitHub Ruleset с обязательными CI; это отдельная настройка владельца, не тождественная согласию на merge.

Исторические решения D1–D9 остаются утверждёнными. Новые форматы/изменения разрешений требуют отдельного ADR. Исходные формулировки и предыдущая таблица этапов сохранены в [roadmap v2 archive](roadmap_v2_archive.md).
