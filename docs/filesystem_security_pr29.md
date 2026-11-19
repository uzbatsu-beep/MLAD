<!-- MLAD-I18N:NAV:START -->
> 🌐 **Язык / Language:** [Русский](filesystem_security_pr29.ru.md) · [English](filesystem_security_pr29.en.md) · [Original / Исходный текст](filesystem_security_pr29.md). **Machine translation / Машинный перевод: editorial review required / необходима редакторская проверка.**
<!-- MLAD-I18N:NAV:END -->

# MLAD PR #29 — Filesystem Security Hardening: threat review and evidence

**История:** PR #29 разработан от `main` SHA `3c184cd36e076dc2e5c9aed7a663b601b4899dea` и [merged](https://github.com/uzbatsu-beep/MLAD/pull/29) 18.11.2026 в `main` SHA `f85ddfcce3063dd087425be981a9d2b7a16d6309`. Финальные проверки [Linux CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454) и [Windows CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621) на этом SHA. Исторические [Linux PR CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44369892563) и [Windows PR CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44369892566) также успешны. **UserSmoke/OEMBench = NOT RUN**; это не release.

## 1. Границы доверия и атакующий

Рабочий контекст — **OFFLINE/BENCH IDE**, которая читает `CIOPRG.LST` и `IONAME.DAT` исключительно для просмотра и сохраняет самостоятельные `*.graph.json`, `*.advanced.json` и legacy `*.json`. Тесты не используют настоящие программы/сигналы роботов.

Потенциальный нарушитель: другой локальный процесс с возможностью изменять выбранный каталог, файл/backup, hardlinks, junction/reparse либо менять JSON между чтением и сохранением. **Не** подразумевается защита от локального администратора, вредоносного ядра, недоверенного файлового сервера, атак на supply chain или компрометации Tauri runtime. Доступ к папке и права ACL задаются ОС и остаются основным барьером.

Критические инварианты: (a) исходные native-байты неизменны, (b) writer не получает путь native export, (c) допустимые JSON изменения не уничтожают конкурентную версию, когда подмена выявлена, (d) отказ не должен оставлять повреждённый файл, (e) ни один пункт не включает robot network, HSES/FTP или deployment.

## 2. Реальные изменения относительно PR #28

| Вектор | Ранее | PR #29 | Доказательство / статус |
| --- | --- | --- | --- |
| Target с заменой на другие **те же байты** | `read_bounded` сверял только содержимое до rename | `read_regular_with_identity` возвращает bytes и **FileIdentity одного открытого handle**: Windows volume+file index, Unix dev+inode; JSON конфликт сверяет оба поля | `same_content_target_replacement_is_a_save_conflict`, проверяется Rust CI |
| Скрытый JSON hard link на исходный `CIOPRG.LST` или другой файл | Обычный `metadata.is_file` не различал alias | `verify_single_link` требует link count ровно 1 для существующих JSON target и `.bak`, иначе fail-closed | `hardlink_alias_to_native_source_or_json_backup_is_rejected` |
| Замена/модификация staged `.tmp` до rename | Создавался через `create_new`, выполнялся `sync_all`, но не повторная сверка | `StagedFile` хранит FileIdentity открытого staged handle и SHA-256 из ожидаемого контента; перед rename повторно читает staged файл и сравнивает identity + checksum | `staged_file_modified_after_flush_cannot_be_committed` |
| Windows предварительная IPC-проверка | `reject_existing_symlink` проверяла только `is_symlink` | Использует `is_indirection` (все обнаруженные reparse attributes), далее основной writer снова проверяет и ancestor и target | Windows CI + security source contract |
| Windows ADS/UNC/device path обходы | `*.graph.json` мог быть частью NTFS alternate stream после двоеточия или Windows path попадал на UNC/`\\.\\`/verbatim namespace | `verify_parent_directory` отвергает `:` в нормальных компонентах пути, UNC/device/verbatim и drive-relative префиксы; разрешён обычный локальный абсолютный `C:\\...` или относительный путь | `windows_ads_unc_and_device_paths_are_rejected_before_filesystem_io` + Windows CI |
| Проверка path с junction и блокировкой | PR #28 | Сохраняются тесты PR #28 на NTFS с реальным `mklink /J` и эксклюзивной блокировкой | Linux/Windows CI, без претензии на полную atomicity |

**Техническая форма:** `FileIdentity` формируется из `File::metadata` на Unix, из WinAPI `GetFileInformationByHandle` на Windows. Для Windows кроме volume/file index читается `nNumberOfLinks` из того же `BY_HANDLE_FILE_INFORMATION`. Любая ошибка получения надёжной идентичности/числа ссылок приводит к отказу, а не к продолжению записи. Новых `Cargo`/npm зависимостей не добавлено, lockfiles не менялись. Windows UNC, extended-length/verbatim и device/ADS пути намеренно не поддерживаются: для работы выбирать **локальную обычную копию файла**.

## 3. Остаточные риски: статус OPEN

| Угроза | Почему не закрыта PR #29 | Следующий возможный этап / подход |
| --- | --- | --- |
| `check → rename` TOCTOU и подмена parent ancestor | Последняя проверка File ID/хеша отделена от ОС `rename`; pathname может изменить цель в тот момент. Чтение текущим `guarded_read` не является `openat`/relative-handle traversal | Windows: открыть/удерживать directory handles с контролем `FILE_SHARE_DELETE`, identity и reparse; Unix: `openat2` (`RESOLVE_BENEATH/NO_SYMLINKS`) или проверяемая openat цепь; **отдельный security design** |
| Windows hardlinks в *момент* после link count check | Link count=1 — моментальный snapshot, не блокировка дополнительных `CreateHardLink` | Directory handle/ACL и привязанные к дескриптору rename semantics; отдельное испытание |
| ACL/owner/attributes при замене JSON | Staged file наследует ACL каталога; `rename` может изменить DACL/owner/атрибуты по сравнению с исходным target, даже при успешной записи. ACL не копируются | Windows ACL inheritance/security descriptor policy, explicit DACL review and privilege model; проверить на реальной тестовой машине |
| Одновременные writer-процессы | Сравнение snapshot предотвращает выявленные изменения, но два писателя могут пройти проверку одновременно | Version token из UI, process lock, ОС-специфичный conditional replacement; не считать поддержку multi-writer закрытой |
| Backup + target | Два отдельных rename, `*.bak` может обновиться при ошибке target; power loss может затронуть только часть операции | Журналируемый commit protocol с recovery/FS sync + crash-test; исключить заявления «полностью атомарно» |
| Краш, отключение питания, SMB / ReFS / UNC | Тесты не подтверждают crash durability и remote FS identity; прямой UNC отклоняется новым guard, но иные remote/mounted случаи требуют отдельной оценки | Зафиксировать условия local NTFS, fault injection, recovery protocol |
| Реальные клиентские файлы | В Git/CI только синтетические данные | Приватный corpus и независимая OEM проверка, см. [правила приёма исходников](controller_fixture_intake.md) |

## 4. Acceptance matrix

| Gate | Что выполнять | Ожидаемое доказательство |
| --- | --- | --- |
| Linux compiler/Rust | `cargo test --manifest-path src-tauri/Cargo.toml --locked` | Все существующие и новые тесты проходят |
| Windows MSVC + NTFS | [windows-security-acceptance.yml](../.github/workflows/windows-security-acceptance.yml) на SHA ветки | В том числе три новых PR #29 Rust теста и четыре Windows теста PR #28 |
| Offline frontend/security | `npm ci`; `npm run test:security`; `npm run test:native`; `npm run docs:check`; `npm run build` | No network module; monitored/deploy deny-only; unchanged lockfile |
| Windows Tauri artifact | Production `npm run tauri -- build --no-bundle` | Temporary unsigned non-release artifact; PE + SHA-256. **Не** WebView2 manual test |
| Byte immutability | Rust fixtures + ручной [WIN-01–WIN-12](windows_acceptance_pr28.md) | Синтетический оригинал не меняется; UserSmoke `NOT RUN`, пока нет отчёта |
| Hardware | **Не выполнять** | `OEMBench=NOT RUN`; HSES/FTP/writer `BLOCKED` |

## 5. Результаты CI и открытые задачи после merge

- [x] PR HEAD `0e1eae035309a1bc8a43594323b1197deb57a47f`: Linux/Windows CI SUCCESS; после merge `main` SHA `f85ddfc` — Linux/Windows CI SUCCESS.
- [x] Windows `cargo test --locked` и Windows-specific regression tests выполнены в указанном GitHub Actions.
- [x] Имеющиеся Rust unit/regression тесты JSON staging/backup/oversize/lock прошли на Windows runner; **ручной UI тест по-прежнему NOT RUN**.
- [x] Rust same-content File ID swap test возвращает конфликт до rename; последнее race window остаётся OPEN.
- [x] Synthetic Rust test подтверждает отсутствие native write в протестированных сценариях; HSES/FTP/native writer остаются BLOCKED.
- [x] Не заявляется `TOCTOU fixed` / `atomic backup+target`; отдельный архитектурный review directory handles/ACL и crash recovery остаётся OPEN.
- [x] PR #29 merged; независимая Windows UserSmoke/проверка WebView2 всё ещё **NOT RUN** и должна быть отдельно зафиксирована.

См. [PR #26 review](guarded_io_review.md), [verification gates](verification_gates.md), [Windows protocol](windows_acceptance_pr28.md), [implementation plan](implementation_plan.md).

## 6. После PR #29 — следующий risk review

**Реализованные проверки — только обнаруживаемые случаи.** Для нового versioned sidecar и любых последующих edit-paths отдельно оценить атомарность target+backup, parent-handle traversal, ACL/owner inheritance, одновременный UI session save и crash recovery. §main§ включает PR #29, но **M2 остаётся PARTIAL**. [Текущая roadmap v3](roadmap.md) · [план](implementation_plan.md) · [Windows manual acceptance](windows_acceptance_pr28.md).
