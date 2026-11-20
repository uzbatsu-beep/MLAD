<!-- MLAD-I18N:EDITORIAL:source-aligned Russian; v4 updated 2026-11-19 -->
> **Язык / Language:** [Русский](implementation_plan.ru.md) · [English](implementation_plan.en.md) · [Исходный текст / Original](implementation_plan.md)
>
> **Примечание:** русскоязычный исходник сохранён без машинного пересказа; OEM validation independent.

# MLAD — план разработки v4 после PR #39

**Baseline:** `main` @ `066db5f8c01a9da8f5f0218f12539b305ab021e4`, PR #1–#39 слиты. Номера #40–#47 — план, не автоматически созданные или завершённые PR. Функциональная реализация PR #40 не подтверждается одним обновлением Markdown. [Roadmap v4](roadmap.ru.md) · [Фактический статус](project_status.ru.md).

## 1. Приоритеты и границы

- **P0:** проверяемый parser corpus, неизменный native snapshot, честная диагностика; затем надёжная навигация.
- **P0:** независимые security/Windows проверки перед добавлением пользовательского writer.
- **P1:** переносимый sidecar и Unified Offline IDE; последовательно расширять только OEM-evidenced syntax.
- `CIOPRG.LST`/`IONAME.DAT` **read-only**; `HSES`/`FTP`/native writer/deployment **BLOCKED**; `PART`/`END` не подтверждают права System/User.
- `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`; только синтетические fixtures в публичном GitHub.

## 2. PR #40 — Native Parser Fidelity & Coverage (Draft scope)

**Цель:** не создавать имитацию полной OEM-грамматики, а воспроизводимо учитывать распознанные, частичные и неподдерживаемые исходные данные без потери байтов.

**Реализация:**
1. `SYNTHETIC` fixture corpus v1 с минимум 12 независимыми сценариями и машиночитаемым manifest: provenance, profile, encoding/EOL, source hash, expected line spans и diagnostics.
2. Классификация: `recognized`, `typed candidate`, `unsupported`, `malformed`, `opaque`, `post-END data` — только синтаксическое покрытие, не подтверждение исполнения.
3. Сводка покрытия по полному источнику; отдельные parser/semantic diagnostics и явное truncation состояние.
4. `NativeWorkspace.tsx`: исправить ошибочную подсказку `Byte offset`, которая показывает номер строки, а не `body_span.start`; показывать достоверный байтовый диапазон.
5. Синхронизировать `docs/roadmap.md`, `docs/implementation_plan.md`, `docs/project_status.md` и их RU/EN версии; проверить docs/i18n.

## 3. Матрица synthetic fixtures v1

| ID | Тестовый случай |
| --- | --- |
| F01 | Минимальная булева цепь, точные spans |
| F02 | BOM/UTF-8 и вариант non-UTF8 |
| F03 | LF/CRLF/CR и отсутствие завершающего EOL |
| F04 | Пустой файл и файл только с EOL |
| F05 | Валидный `//$M`, восемь строк по десять ячеек |
| F06 | Усечённый и неправильный `//$M` |
| F07 | Метки `PART` и корректный `END` |
| F08 | Невалидный `END`: следующие строки не скрыты |
| F09 | `//ALARM`/`//MESSAGE` после `END` |
| F10 | Unsupported opcode между булевыми командами |
| F11 | Неверные операнды и типизированные кандидаты |
| F12 | MOV/ADD/SUB/SHL/SHR и подсчёт покрытий |
| F13 | Большие/лимитные входы синтезировать в тесте, не коммитить гигабайтные fixture |

## 4. Требования к классификации и сохранению

- Одна первичная классификация на физическую строку, независимые диагностические флаги; покрытие в сумме не должно превышать число строк.
- `recognized` ≠ `OEMBenchVerified`; `TypedCandidate` ≠ `semantics_verified`.
- `opaque`/`unsupported`/`malformed` остаются доступными в raw source, а не преобразуются в воображаемую диаграмму.
- `full_span`, `body_span`, `SHA-256`, `BOM`, `CRLF` проверяются по исходным байтам, не по нормализованному тексту.
- Данные после корректного `END` остаются данными; невалидный END не прячет последующие строки.
- Сохраняются ограничения: `16 MiB`, `200000` строк, постраничный просмотр и `SHA-256`-pinned reopen.

## 5. Что не входит в PR #40

Никакого нового контроллерного native writer, deployment, сетевого runtime, автоматического IONAME→relay mapping, нового sidecar-writer, полного scan/EN/timing эмулятора, объявления новых opcode OEM-verified без evidence или реальных клиентских программ в CI.

## 6. Gate и evidence на HEAD PR #40

- [ ] `cargo test --manifest-path src-tauri/Cargo.toml --locked`, `npm ci --no-audit --no-fund`, `npm run test:native`, `npm run test:security`, `npm run docs:check`, `npm run build`.
- [ ] `scope`, `frontend-build`, `rust-tests`, `windows-security`, `ci-gate` **SUCCESS** по конкретному HEAD после всех изменений (когда тесты применимы).
- [ ] Позитивные/негативные fixture тесты проверяют raw byte equality и отсутствие fake rungs.
- [ ] `cmd_deploy_to_robot` всегда отказывает; native/IONAME не записываются, HSES/FTP не возвращаются.
- [ ] В diff нет секретов и реальных OEM выгрузок; все fixtures имеют provenance `SYNTHETIC`.
- [ ] `UserSmoke=NOT RUN`, `OEMBench=NOT RUN` пока нет независимых протоколов.
- [ ] **Только отдельное решение владельца разрешает merge**.

## 7. Предлагаемая очередь после PR #40

| PR | Область | Gate результата |
| --- | --- | --- |
| #41 | Native UX v2 | source↔rung↔block↔diagnostic, verified jump and large-file UX |
| #42 | Filesystem/supply chain | Review ACL/TOCTOU, crash/backup and unnecessary FTP dependency; no network functionality |
| #43 | Portable sidecar | Versioned `*.mlad.json`, source SHA, stale/conflict/backup, no native overwrite |
| #44 | Typed grammar | Only OEM-evidenced grammar, no implicit scan semantics |
| #45 | Static analysis | source-linked READ/WRITE/UNKNOWN, provenance and conservative barriers |
| #46 | Unified Offline Project | Native snapshot + sidecar + JSON drafts, no native export |
| #47 | 0.6.x Windows preview | Exact SHA/tag, manual UserSmoke, licenses, checksum, separate release decision |

## 8. Доказательные уровни и ссылки

`Documented` ≠ `SyntheticTested` ≠ `WindowsCI` ≠ `UserSmoke` ≠ `OEMBenchVerified` ≠ `Authorized`. Каждый шаг фиксирует SHA, positive/negative tests, состояние отказов и право на использование OEM материалов отдельно.

[Roadmap v4](roadmap.ru.md) · [Verification gates](verification_gates.ru.md) · [Исходные реальные данные](controller_fixture_intake.ru.md) · [Filesystem review](filesystem_security_pr29.ru.md) · [Engineering decisions](engineering_decisions.ru.md).

## Уточнение исполнения после PR #40 (20.11.2026)

Документационный PR #40 уже слит; реализация parser fidelity и corpus вынесена в PR #41. Этот этап включает синтетические тестовые байты, SHA-256, ожидаемые классы строк, проверку полной классификации без потерь, сохранение unknown/opaque, диагностику malformed и физические byte spans в Native Workspace. Факт успешного CI не заменяет UserSmoke или OEMBench, а также не разрешает HSES/FTP/native writer/deployment. `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`, controller actions **BLOCKED**.

Ранее запланированный PR #41 по UX переносится на PR #42 или позднее; дальнейшие номера условны. Текущий PR остаётся Draft без слияния до явного согласования.

## PR #42 — Native CIOPRG Offline Workspace UX v2: критерии приёмки

Исходная точка — слитый PR #41, `main` `0c1b233354564d9fd3c242675a78a45417feacc1`. PR #42 реализуется отдельным Draft, старый предложенный PR #42 security переносится на PR #43 или позднее.

Контракт: одновременная отрисовка не более одной Ladder-цепи и одного функционального блока; список остальных в навигационном дереве с указанием границ preview. Переходы от строки исходника к распознанной цепи/блоку, обратные переходы и диагностика→физическая строка. Клавиши ArrowUp/ArrowDown, PageUp/PageDown, Home/End; страницы исходника по 250 строк. Фильтр по `SyntaxClass`, точному ASCII opcode и текстовому токену адреса `#XXXXX` или `Mxxx`; результаты по 100 строк, общее число совпадений по всему ограниченному входному файлу.

Acceptance: неизменность исходных байтов, SHA-256 и профиля контроллера; отказ при подмене файла; тест не менее 1400 строк и переход на страницу дальше первых 100 совпадений; отрицательные случаи для некорректного opcode/адреса, malformed/opaque и post-END. Сохранение неизвестного текста и отсутствие имитации выполнения обязательны. Linux frontend/Rust/Windows CI — независимые от UserSmoke/OEMBench уровни.

CIOPRG.LST и IONAME.DAT read-only; HSES/FTP, native writer, live I/O и deployment **BLOCKED**. `UserSmoke=NOT RUN`, `OEMBench=NOT RUN` до отдельного испытания; merge по отдельному согласованию.

## PR #43 — практический security gate и границы доказательств

Baseline — слитый PR #42 в `main` `73e675f7df33c5751a35e29752736c3470b11b31`. PR #43: защита самостоятельных JSON-черновиков, аудит зависимостей Rust, тесты и документирование отказов. Не добавлять controller writer.

**Разработка в PR #43:**
1. Исключить `suppaftp` из `Cargo.toml` и `Cargo.lock` с удалением только недостижимых транзитивных пакетов. Не менять `package-lock.json`; запускать `npm ci` без автоматической перезаписи lockfile. Также исключить неиспользуемые прямые `anyhow` и `futures-util` из манифеста/root lockfile; `nom` сохранить, потому что он используется в `compiler/parser.rs`.
2. На Unix staged JSON и `*.bak` создавать с правами `0600`, независимо от исходного umask; проверить строгость доступа после повторного сохранения.
3. Перед rename выполнять `verify_single_link` для staged-файла, при его подмене или появлении hardlink отказать. В `Drop` удалять временный путь только если File ID совпал с записанным при создании.
4. Проверить позитивные и негативные сценарии: первая запись, backup, hardlink staged JSON, подмена temporary pathname, защита `CIOPRG.LST`/`IONAME.DAT`, Windows NTFS/локальные права и сохранение существующего контроля конфликтов.
5. CI: `cargo test --locked` для Windows/Linux, `npm run test:security`, все обязательные проверки `frontend-build`, `rust-tests`, `windows-security`, `ci-gate` на финальном HEAD.

**Что НЕ реализуется:** ОС-специфичная атомарная traversal через directory handles, Windows DACL/owner preservation, полномасштабный multi-process CAS, crash-safe журнал target+`*.bak`. Их считать OPEN и не оформлять как устранённые. `HSES/FTP`, native writer и deployment **BLOCKED**, `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`. PR не сливать без разрешения владельца.

## PR #44 — контракт portable sidecar v1 и критерии приёмки

**Baseline:** слитый PR #43, `main` `c36dc4bc4f84b3dfdb2eac52625c3c0389226320`. Новый Draft PR #44 не включён в `main`. Старые номера этапов из раннего плана v4 имеют лишь историческое значение.

**Схема `*.mlad.json`:** `format=MLAD_OFFLINE_SIDECAR` и `schema_version=1` обязательны. `source.kind=CIOPRG.LST` содержит только `source.controller`, `source.sha256`, `source.byte_length`, `source.line_count`; также допускаются `annotations.rungs`, `annotations.addresses`, `annotations.ioname` и `bookmarks`. Никаких нативных байтов, пути исходника, исполняемого скрипта, адреса робота, FTP/HSES или перезаписи OEM источника. Посторонние поля, неподдерживаемые версии, неправильные ключи и объёмы отвергаются.

**Правила I/O:** команды `cmd_load_offline_sidecar` и `cmd_save_offline_sidecar` только для пути `*.mlad.json`. Каждая операция выполняет read-only проверку оригинала `CIOPRG.LST` и сравнивает `SHA-256`, семейство контроллера, размер и число строк. При первом сохранении целевой файл обязан отсутствовать. Для существующего файла нужен `revision`, привязанный к его байтам и файловой идентичности. Старая/отсутствующая ревизия и ручное изменение sidecar приводят к отказу без перезаписи. Выгрузка `*.bak` подчиняется существующей проверке staged JSON, но не транзакционно атомарна с target.

**Acceptance:** Rust-тесты round-trip и конфликтов, испорченной версии/метаданных, пустых и некорректных закладок, лимитов, неизвестных полей, защиты нативного файла, hardlink/symlink и сохранения резервной копии; frontend-тесты явного импорта/экспорта и LOCAL-only режима. CI по конечному SHA включает `frontend-build`, `rust-tests`, `windows-security`, `ci-gate`. Отдельно и независимо остаются `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`; FTP/HSES, native writer и deployment **BLOCKED**.

**Риски:** `revision` не заменяет атомарный файловый CAS; TOCTOU в `check→rename`, Windows ACL, одновременные writers, crash recovery и backup+target остаются OPEN. Решение о слиянии принимает владелец.

## PR #45 — план исполнения OEM-доказательной grammar расширения

**Base:** слитый PR #44, `main` `48522c1e2d0ee41560ce61ba0d2aa8af14e1c6d8`. Технический PR #45 — отдельный Draft. Источник: YRC1000 Concurrent I/O R12 178648-1CD (RE-CKI-A467), §§12.2.15–16/19–23/25–28/31–34; [официальный указатель](https://knowledge.motoman.com/hc/en-us/articles/40670912825362-YRC1000-OPTIONS-INSTRUCTIONS-FOR-CONCURRENT-I-O-R12), доступное воспроизведение §12.2 [manualzz](https://www.manualzz.com/doc/84663174/yaskawa-motoman-yrc1000-owner-s-manual).

**Scope:** синтаксические `TypedCandidate` для `MUL/DIV/MOD`, `BIN/BCD`, `WAND/WOR/WXOR/WNOT`, `ROL/ROR`, `BMOV`, `CMP`, `PLS/PLF`. Для каждой формы — отдельные роли S/D/n; no immediate в BIN/BCD; BMOV байты, не слова; MUL пара регистров Dn+1; PLS/PLF отдельный relay; CMP два источника без D. Нельзя превращать это в `Instruction` для исполняемого AST или генерацию исходника.

**Evidence / negative tests:** синтетическая проверка 15 корректных и 17 ошибочных примеров, контроллеры DX100/DX200 с `controller_form_verified=false` для новинок, проверка сохранения BOM/EOL, byte spans, непарсируемых строк и post-END. Старые фикстуры обновлять только при изменении доказанной классификации: `BMOV` перестал быть unsupported, для synthetic unknown заменён на `BMOX` с новым SHA-256. `MOVJ/MOVL` — отдельные JOB-мнемоники, не `MOV` и не `BMOV`.

**UI:** справочник для каждого нового opcode должен ссылаться на раздел производителя и не показывать вычисленные Carry/Zero/Error, EN, фактическое write/read или результат реального scan. Для новых форм статус DX100/DX200 — manual evidence отсутствует; не писать ложное ограничение только DX100.

**CI Gate:** `scope`, `frontend-build`, `rust-tests` (`cargo test --locked`), реальный Windows NTFS/Tauri `windows-security`, `ci-gate` — SUCCESS на одном окончательном SHA; PR не сливать до согласования. Независимо `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`. Native `CIOPRG.LST`/`IONAME.DAT` read-only; HSES/FTP/live I/O/native writer/deployment **BLOCKED**.

## PR #46 — контракт статических READ / WRITE / UNKNOWN

**Baseline:** PR #45 слит, `main` `12f00f4b4fb6879dc0e3ae478aaf8b6d33553139`; PR #46 — отдельный Draft. Модуль `compiler/static_access.rs`, метод `inspect` и Tauri IPC `cmd_query_static_access` работают только с оригинальным read-only снимком `CIOPRG.LST`.

**Модель:** `Read`/`Write` — потенциальные роли адресных операндов из документированной формы, а не события scan. Контакты, GSTR, S арифметических/логических команд = кандидаты источника; OUT, GOUT, D = кандидаты назначения. `CMP` не имеет D; `PLS`/`PLF` — импульсная выходная роль, не обычная запись регистра. `MUL` не порождает дополнительную выдуманную запись в Dn+1, `BMOV` не разворачивает диапазоны. Иммедиаты и счётчики сдвигов не являются чтениями памяти; TMR/CNT = `Unknown` (не подтверждена внутренняя модель обращения). Невалидные, неподтверждённые для выбранного семейства и непрозрачные конструкции = `Unknown`. Post-END не является кодом.

**Верификация:** `source_sha256`, `line` (с 1), оригинальный `ByteSpan`; 160 первых site в preview при полнофайловых счётчиках; фильтр `query` по виду/точному текстовому адресу с 100 site за запрос, offset и проверкой SHA-256 на каждом обращении. Тесты Rust/IPC/UI: границы, Unknown, post-END, невалидные токены, 352-событийная пагинация, изменение файла между запросами и сохранность байтов. Нет ни aliasing byte/word, ни вычисления EN/флагов, ни фактического Memory READ/WRITE, ни права редактировать System/User.

**Acceptance:** `scope`, `frontend-build`, `rust-tests`, `windows-security`, `ci-gate` на конечном HEAD; отдельно `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`. `execution_verified=false`, `scan_order_verified=false`, `address_aliases_verified=false`, `write_permission_verified=false`, `native_write_allowed=false`. `IONAME.DAT` read-only; HSES/FTP/native writer/deployment **BLOCKED**. До разрешения владельца merge не выполнять.

## PR #47 — формат, операции и процедура работы с единым offline-проектом

**Baseline:** слитый PR #46, `main` `39197458dac01045b30f9ac5a810720c729e2a65`. PR #47 — отдельный Draft. Новый `MLAD_OFFLINE_PROJECT`, `schema_version=1` в `*.mladproj.json`: `source` (относительный путь, `sha256`, `byte_length`, `line_count`, `controller`), `ioname` / `sidecar` (необязательные путь, SHA-256, размер), `analysis` (модель `SOURCE_ROLE_PREVIEW_V1`, SHA исходника, инвентарные счётчики и `execution_verified=false`). Не содержит исходных строк, абсолютных путей или сетевых настроек. Схемы sidecar v1 и native ввода не меняются.

**Порядок создания:** пользователь самостоятельно помещает read-only копию `CIOPRG.LST`, при необходимости `IONAME.DAT` и `*.mlad.json` в одну папку проекта или вложенные папки. Пути в манифесте не выходят из его корня: `../`, абсолютные ссылки, Windows ADS, symlink/reparse компоненты запрещены проверками. В Native Workspace сначала открыть `CIOPRG.LST`, затем `IONAME.DAT` по необходимости, сохранить заметки кнопкой «Сохранить sidecar». В поле проекта указать абсолютный путь к новому `*.mladproj.json` в корне и нажать «Сохранить проект». Исходные файлы никуда не копируются приложением. Чтобы перенести проект, скопировать всю структуру папок без изменения байтов файлов.

**Порядок открытия:** указать `*.mladproj.json` и нажать «Открыть проект». `cmd_open_offline_project` безопасно читает манифест, проверяет строгую схему, все относительные пути, точные хэши и размеры исходника и вложений, sidecar source binding, затем пересчитывает анализ парсера/семантики/READ-WRITE-UNKNOWN и возвращает одну связную сессию. Если какой-то компонент отсутствует или изменён, операция отвергается, прежняя сессия остаётся. `analysis_changed` показывает расхождение исторической сводки с новым парсером, но сохранённые счётчики не заменяют вычисления.

**Конфликты:** только `*.mladproj.json` меняется при сохранении проекта; для перезаписи нужна точная `revision`, целевой backup `*.bak` создаётся существующим staged JSON слоем. Вложенный `*.mlad.json` сохраняется своей отдельной командой/ревизией, до фиксации манифеста. После изменения IONAME/sidecar нужно обновить манифест, иначе reopen fail closed по SHA. Нет транзакционного сохранения сразу нескольких файлов, переноса файлов внутри программы или автоматического сопоставления IONAME с адресами IO.

**Acceptance:** Rust schema/relative-path/security тесты, интеграционный round-trip source+IONAME+sidecar+analysis, неправильные SHA/контроллер, unknown fields, stale manifest revision, source immutability, Windows NTFS, frontend грязные состояния/atomic open и SHA/epoch защита. Финальный `scope`, `frontend-build`, `rust-tests`, `windows-security`, `ci-gate` проверяются по HEAD. `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`; `CIOPRG.LST`/`IONAME.DAT` read-only; native writer, FTP/HSES и deployment **BLOCKED**. Права Windows DACL и parent path TOCTOU остаются OPEN.

## PR #48 — Windows Offline Preview: сборка, доказательства, приёмка

**Base:** PR #47 уже слит, `main` SHA `e72a51f8ea69466e716fb268a62b1cf1bd38d6f2`. PR #48 — отдельный Draft. `.github/workflows/windows-offline-preview.yml` получает точно `github.event.pull_request.head.sha`, а не PR merge ref, устанавливает `npm ci` с неизменным lockfile, выполняет Node offline/Native/preview контракты, `cargo test --locked` на Windows NTFS и `npm run tauri -- build --no-bundle`. Публикация GitHub Release/тега, установка доступа к роботу и подписание EXE отсутствуют.

**SYNTHETIC fixture:** `scripts/offline-preview-fixture.py` строит только искусственный `MLAD_OFFLINE_PROJECT` версии 1: `CIOPRG.LST`, `IONAME.DAT`, `cell.mlad.json`, `cell.mladproj.json`, `SYNTHETIC-SHA256.json`. Тестируются неизменность байтов и четыре независимых случая фальсификации. `scripts/windows-offline-preview.ps1` проверяет PE MZ, SHA256 каждого файла, source/IONAME/sidecar связки, журнал `build-evidence.json`, включает GPL-3.0-only `LICENSE` и `corresponding-source.zip`, после чего CI формирует ZIP с SHA256 и публикует **только временный Actions artifact на 21 день**. Никаких реальных файлов робота в репозиторий не добавляется.

**Human UserSmoke:** для Windows 10/11 x64 и локального WebView2 tester извлекает архив, сравнивает SHA, запускает EXE, открывает тестовый `*.mladproj.json` и последовательно проверяет `P48-01`…`P48-17` по `docs/windows_offline_preview_pr48.md`. Изменение источника, IONAME/sidecar и перенос структуры проводить только в независимых копиях. Пользователь фиксирует Windows build, WebView2, точный SHA/Actions URL, результаты `PASS`/`FAIL`/`BLOCKED`/`NOT RUN` и обезличенные дефекты. Автоматические тесты не могут выставить `UserSmoke=PASS`.

**Acceptance Gate:** обе CI цепочки SUCCESS на финальном SHA; до ручных доказательств `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`. Слияние PR или публичный релиз — отдельное согласование. Нативные `CIOPRG.LST`/`IONAME.DAT` read-only; `FTP`/`HSES`, native writer, deployment `BLOCKED`. TOCTOU, Windows ACL, multi-file transactional save остаются OPEN.
