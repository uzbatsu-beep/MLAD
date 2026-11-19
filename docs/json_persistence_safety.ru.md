<!-- MLAD-I18N:EDITORIAL:restored 2026-11-19 from predominantly Russian original -->
> **Язык / Language:** [Русский](json_persistence_safety.ru.md) · [English](json_persistence_safety.en.md) · [Исходный текст / Original](json_persistence_safety.md)
>
> **Примечание:** сохранены исходные русские определения безопасности, команды и технические обозначения; OEM-проверки независимы. / No native writer, HSES/FTP or controller permission is granted by localization.

# MLAD — модель безопасности JSON сохранения (PR #24)

**Дата:** 18.11.2026. **Статус:** PR #24 merged в main 18.11.2026; Windows MSVC regression CI SUCCESS, но отдельный Windows binary с изменением ещё не опубликован. Расширение M2, не изменение CIOPRG grammar.

## Задача и причина

В исходном `main` после PR #23 все три вида пользовательского JSON сохранялись через прямое обрезание/запись целевого пути (`tokio::fs::write` / `std::fs::write`), а резервные копии создавались простым copy. Это создавало риск потери предыдущей версии при ошибке или прерывании операции записи.

## Реализуемый контракт

- Только **отдельные** файлы `*.graph.json`, `*.advanced.json` и legacy `*.json` проходят независимую валидацию имени перед вызовом единого `draft_io::write_atomic`. Writer дополнительно проверяет расширение `.json`. `CIOPRG.LST` и `IONAME.DAT` не получают write command.
- Чтение всех трёх форматов: предел 2 МиБ, отказ при прямой symlink и при неверном типе файла; невозможно неограниченно прочитать подменённый большой файл без проверки размера результата.
- Запись: проверка размера до 2 МиБ, создание нового *уникального* временного файла в той же директории с `create_new(true)`, запись и `sync_all`, затем замена исходного пути через `rename`. До последнего шага оригинал не обрезается.
- Для существующего JSON предварительно читаются его предыдущие байты и аналогично готовится `*.bak`; копирование через перезапись в существующую symlink запрещено. Если staging/backup завершился ошибкой, старый JSON остаётся на месте; случай ошибки финального rename также не должен обрезать его.
- При сбое временные файлы удаляются в `Drop`; сохранение запускается в `tokio::task::spawn_blocking`, чтобы не блокировать async runtime.
- Положительные/отрицательные Rust-тесты не содержат данных реальных роботов. `npm run test:security` проверяет, что Tauri handlers не вернулись к прямому `fs::write`.

## Ограничения: что НЕ гарантировано

1. **Нет общей атомарной транзакции** между обновлением `*.bak` и основного JSON. Прерывание после установки backup, но до замены target оставит старый target и новый backup.
2. **TOCTOU и перехват родительских каталогов не устранены**: стандартные path-based `metadata/open/rename` не являются secure directory-handle API; злонамеренно изменяемые symlink ancestors, hardlinks и межпроцессные гонки требуют отдельного threat review.
3. На Windows `rename` при занятом/защищённом целевом файле может вернуть ошибку; задача — сохранить старый файл и сообщить об отказе, а не обещать бесшовную замену на любом томе.
4. Нет шифрования JSON и отдельных гарантированных прав на каталоги, профили или резервные копии. Пользователь отвечает за права доступа к своей папке. Без Windows smoke, моделирования отключения питания и проверки NTFS нельзя обещать crash-consistent durable commit.
5. Содержание legacy проекта проходит существующую модель валидации и ограничения формата, не модель native CIO.
6. PR #24 не менял CSP, но последующие merged PR #25 и #26 добавили CSP/path checks и guarded JSON conflict detection: [CSP review](csp_path_hardening.ru.md), [guarded I/O](guarded_io_review.ru.md). Это не обещает устранения TOCTOU и не включено в выпущенный rc.9.

## Приёмка

- Cargo tests, включая синтетические файлы, symlink negative cases и проверку `copy_original == source_bytes` для нативного LST, должны пройти.
- TypeScript/frontend и `npm run test:security` должны пройти без регрессий; контроль direct write targets обязателен.
- Windows user acceptance проверит два сохранения каждого JSON, содержимое `*.bak`, занятый файл, no overwrites of robot files, и возможность повторного открытия.
- Отдельный security PR для true file-handle/dir-handle и CSP, межпроцессной блокировки/версий, atomic backup protocol, permission audit.

**Статус M2 остаётся PARTIAL** даже после merged PR #24 и успешного Rust CI. Для PR #24 добавлен отдельный [Windows MSVC regression workflow](../.github/workflow-archive/windows-json-persistence-pr24.yml), без публикации релиза. Он проверяет поведение файлового сохранения на Windows runner; пользовательский smoke на конкретной машине остаётся самостоятельным gate.


## PR #26 (merged): конкурентное редактирование

Перед финальным обновлением JSON сравнивается с snapshot старой версии; появление другого файла при первичном сохранении также вызывает отказ. Стадирование/backup из PR #24 сохраняется. **Не гарантируется** исключение races после последней проверки и транзакция backup+target. [Исследование](guarded_io_review.ru.md).

**Проверка актуального baseline:** [main CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44228204776), [PR #26 Windows CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44227145031). Ручной Windows WebView2 smoke и проверка сбоя питания — NOT RUN. [Unified gates](verification_gates.ru.md).

## Дополнение после PR #28–#29 (merged, 18.11.2026)

[PR #28](https://github.com/uzbatsu-beep/MLAD/pull/28) добавил Windows NTFS regression (junction, exclusive lock) и формальный UserSmoke checklist. [PR #29](https://github.com/uzbatsu-beep/MLAD/pull/29) добавил сравнение snapshot по **File ID + байтам**, отказ для hardlinked JSON target/backup, проверку staged File ID+SHA-256 и запрет Windows ADS/UNC/device paths. `main` SHA `f85ddfcce3063dd087425be981a9d2b7a16d6309` прошёл [Linux](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454) и [Windows](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621) CI. Иными словами, контракт PR #24 дополнен **новыми регрессиями**, но полной atomic backup+target, ACL-preservation или final check→rename TOCTOU guarantee по-прежнему нет. [Актуальная модель угроз](filesystem_security_pr29.ru.md). Ручной WebView2 smoke = `NOT RUN`.
