# MLAD PR #29 — Filesystem Security Hardening: threat review and evidence

**Baseline:** `main` SHA `3c184cd36e076dc2e5c9aed7a663b601b4899dea`, [PR #28 merged](https://github.com/uzbatsu-beep/MLAD/pull/28), [Linux CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44365314264) and [Windows CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44365314173) SUCCESS. PR #29 — отдельная ветка/Draft, не часть `main` до согласованного merge. Дата: 18.11.2026.

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
| Проверка path с junction и блокировкой | PR #28 | Сохраняются тесты PR #28 на NTFS с реальным `mklink /J` и эксклюзивной блокировкой | Linux/Windows CI, без претензии на полную atomicity |

**Техническая форма:** `FileIdentity` формируется из `File::metadata` на Unix, из WinAPI `GetFileInformationByHandle` на Windows. Для Windows кроме volume/file index читается `nNumberOfLinks` из того же `BY_HANDLE_FILE_INFORMATION`. Любая ошибка получения надёжной идентичности/числа ссылок приводит к отказу, а не к продолжению записи. Новых `Cargo`/npm зависимостей не добавлено, lockfiles не менялись.

## 3. Остаточные риски: статус OPEN

| Угроза | Почему не закрыта PR #29 | Следующий возможный этап / подход |
| --- | --- | --- |
| `check → rename` TOCTOU и подмена parent ancestor | Последняя проверка File ID/хеша отделена от ОС `rename`; pathname может изменить цель в тот момент. Чтение текущим `guarded_read` не является `openat`/relative-handle traversal | Windows: открыть/удерживать directory handles с контролем `FILE_SHARE_DELETE`, identity и reparse; Unix: `openat2` (`RESOLVE_BENEATH/NO_SYMLINKS`) или проверяемая openat цепь; **отдельный security design** |
| Windows hardlinks в *момент* после link count check | Link count=1 — моментальный snapshot, не блокировка дополнительных `CreateHardLink` | Directory handle/ACL и привязанные к дескриптору rename semantics; отдельное испытание |
| ACL/owner/attributes при замене JSON | Staged file наследует ACL каталога; `rename` может изменить DACL/owner/атрибуты по сравнению с исходным target, даже при успешной записи. ACL не копируются | Windows ACL inheritance/security descriptor policy, explicit DACL review and privilege model; проверить на реальной тестовой машине |
| Одновременные writer-процессы | Сравнение snapshot предотвращает выявленные изменения, но два писателя могут пройти проверку одновременно | Version token из UI, process lock, ОС-специфичный conditional replacement; не считать поддержку multi-writer закрытой |
| Backup + target | Два отдельных rename, `*.bak` может обновиться при ошибке target; power loss может затронуть только часть операции | Журналируемый commit protocol с recovery/FS sync + crash-test; исключить заявления «полностью атомарно» |
| Краш, отключение питания, SMB / ReFS / UNC | Тесты на обычной файловой системе не подтверждают crash durability, remote FS identity и семантику `rename` | Условия FS support и fault injection на отдельном стенде; возможно ограничение на local NTFS |
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

## 5. Необходимо отдельно проверить перед merge

- [ ] CI on actual PR HEAD SHA, Linux frontend/Rust и Windows acceptance: только реальный SUCCESS.
- [ ] В Windows полный `cargo test --locked`: hardlink snapshot и staging protection без flaky failures.
- [ ] Сохранение первого/второго JSON, поведение `.bak`, cleanup `.tmp`, oversized JSON и ошибки Windows lock не регрессировали.
- [ ] При одинаковых байтах подменённого JSON тест возвращает конфликт вместо тихого overwrite.
- [ ] Отдельно отметить неизменность `CIOPRG.LST` / `IONAME.DAT` и границы native writer/FTP/HSES.
- [ ] Проанализировать открытые угрозы выше, не писать `TOCTOU fixed` / `atomic backup+target`.
- [ ] **Только владелец** решает merge; даже после merge требуется отдельная Windows UserSmoke до претензий на user acceptance.

См. [PR #26 review](guarded_io_review.md), [verification gates](verification_gates.md), [Windows protocol](windows_acceptance_pr28.md), [implementation plan](implementation_plan.md).
