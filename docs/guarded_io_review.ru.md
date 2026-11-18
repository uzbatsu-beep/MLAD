<!-- Generated ru companion for docs/guarded_io_review.md; review required. -->
> **Русский** · [Русский](guarded_io_review.ru.md) · [English](guarded_io_review.en.md) · [Original / Оригинал](guarded_io_review.md)
> **Translation review / Проверка:** автоматический черновик — сверяйте инженерные утверждения с оригиналом и OEM документацией.

# MLAD PR #26 Охраняемые чтения и предотвращение конфликтов JSON

**Дата:** 18.11.2026. **Статус:** [PR #26 merged](https://github.com/uzbatsu-beep/MLAD/pull/26) в `main` `2092db1825880d251c87efcb7c331d78cb5e4d28`; [Linux CI Успех](https://github.com/uzbatsu-beep/MLAD/actions/runs/44228204776), [Windows MSVC/Tauri CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44227145031). Опубликованный rc.9 не изменён; M2 остаётся PARTIAL.

## Мотивировка

PR #25 проверял symlink компонентов пути до файлового доступа, но открытие через независимый `tokio::fs::read` могло попасть на файл, подменённый между предварительной проверкой и вызовом open. PR #24 использовал staged write, но не проверял конкурирующее редактирование target после создания резервной копии и до финального rename.

## 1. Идентичность открытого файла

Отдельный модуль `src-tauri/src/guarded_read.rs`:

- Сначала отвергает обнаруженные родительские symlink/reparse и прямую ссылку.
- Считывает метаданные и использует **один основной дескриптор для чтения**. На Windows дополнительно открывает контрольные дескрипторы с `FILE_FLAG_OPEN_REPARSE_POINT` до и после чтения; сравнивает их WinAPI File ID. Это уменьшает окно подмены, но не гарантирует полностью атомарный путь. Unix — `dev+ino`Windows — WinAPI `GetFileInformationByHandle` (volume serial + 64-bit file index) для нескольких `HANDLE`; без nightly Rust. Если WinAPI не возвращает идентификаторы, операция завершается отказом. В PR используется Win32 `FILE_FLAG_OPEN_REPARSE_POINT` и проверка флагов reparse на открытых handles.
- Ограничивает чтение ещё до выделения всего содержимого: JSON — 2 МиБ, `IONAME.DAT` — 4 МиБ, `CIOPRG.LST` — 16 МиБ.
- После чтения повторно сверяет pathname и метаданные открытого дескриптора, а также длину и время последнего изменения.
- Отклоняет все Windows `FILE_ATTRIBUTE_REPARSE_POINT`, включая junction (а не только `is_symlink()`).
- Не изменяет оригинальные байты. CIOPRG и IONAME вызывают его через `spawn_blocking`; все JSON-потребители используют ту же функцию.

Это **fail-closed checks** вокруг открытия и чтения, не полноценная атомарная операция с каталогом.

## 2. Конкурентное сохранение JSON

`draft_io::write_atomic_inner` после подготовки нового файла и `*.bak` проверяет, что старые байты не изменились; непосредственно перед заменой делает повторное сравнение. Если другой процесс поменял файл или создал target во время первого сохранения, MLAD возвращает ошибку и **не** перезаписывает более новую версию.

В тестах вводится синтетическая подмена ровно в окне проверки/открытия и синтетическая модификация между созданием backup и финальным обновлением. Проверяются неизменность внешних данных и сохранность нативного файла.

## 3. Что остаётся открытым

1. Последняя повторная проверка и `rename` всё ещё разделены. Это **не** транзакция и **не** безопасный `openat2`/`NtCreateFile` с дескриптором каталога и запретом следования ссылкам.
2. Изменение файла **на том же inode** с искусственно восстановленным размером/mtime может остаться незамеченным; проверка не заменяет файловые блокировки и гарантированный неизменяемый snapshot.
3. Windows volume/index не гарантирован для всех сетевых файловых систем. По соображениям безопасности операция отклоняется, если ID недоступен; сеть роботов по-прежнему отключена.
4. Backup и target не образуют единой атомарной транзакции. Возможен устаревший backup после конфликта; целевой JSON не должен быть перезаписан.
5. Не проведены испытания с неконтролируемыми Windows reparse/junction, сторонними процессами с агрессивной гонкой, hardlinks, ACL, power loss и атомарной директории.
6. Ручной Windows WebView2 smoke (Native Workspace, JSON draft, IONAME, ошибки конфликта) **NOT RUN**.

## 4. Приёмка

| Gate | Проверка |
| --- | --- |
| Linux Rust | `cargo test --manifest-path src-tauri/Cargo.toml --locked` |
| Frontend/security | `npm run test:security`, `npm run docs:check`, `npm run build` |
| Windows | [MSVC Rust + Tauri build - Успешное строительство](https://github.com/uzbatsu-beep/MLAD/actions/runs/44227145031), без публикации релиза |
| Negative | Подмена до open, direct symlink, directory symlink, конкурентная замена JSON/создание target |
| Hardware | Не требуется и не выполняется; HSES/FTP/writer выключены |
| Принятие пользователем | Требуется ручная проверка; НЕ заменяется GitHub CI |

[Статус всего MLAD](project_status.ru.md) · [Ранее закрытые ограничения PR #25](csp_path_hardening.ru.md).


**Результаты CI не означают ручной Windows smoke или full TOCTOU closure.** Следующий план: [implementation_plan.md](implementation_plan.ru.md), [verification_gates.md](verification_gates.ru.md).

** Источники Windows API: ** [GetFileInformationByHandle](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfileinformationbyhandle), [BY_HANDLE_FILE_INFORMATION](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/ns-fileapi-by_handle_file_information), [FILE_FLAG_OPEN_REPARSE_POINT](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew). FFI-код требует независимого security review (ABI, reparse и NTFS/ReFS различия), а не только прохождения компиляции.

## Последующие доказательства PR #28–#29 (18.11.2026)

Этот документ является **историческим исследованием PR #26**, а не описанием исчерпывающей текущей защиты. После него [PR #28](https://github.com/uzbatsu-beep/MLAD/pull/28) ввёл регулярный Windows NTFS/Win32 build/regression; [PR #29](https://github.com/uzbatsu-beep/MLAD/pull/29) JSON Bytes+ Снимки идентификатора файла, отказ от жесткой ссылки, инсценированная идентификация +SHA-256 и Windows ADS/UNC/device path deny. [Фактический main CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621) — SUCCESS, но **не** ручной WebView2 smoke и не гарантия полного TOCTOU closure. Канонический продолженный review: [filesystem_security_pr29.md](filesystem_security_pr29.ru.md).
