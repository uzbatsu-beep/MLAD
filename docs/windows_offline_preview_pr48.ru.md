<!-- MLAD-I18N:NAV:START -->
> 🌐 **Язык / Language:** [Русский](windows_offline_preview_pr48.ru.md) · [English](windows_offline_preview_pr48.en.md) · [Original / Оригинал](windows_offline_preview_pr48.md). **Machine translation / Машинный перевод: editorial review required / необходима редакторская проверка.**
<!-- MLAD-I18N:NAV:END -->

# MLAD PR #48 — Windows Offline Preview и пользовательская приёмка

**Статус:** подготовка непубликуемого preview-артефакта, не GitHub Release и не приёмка OEM. **База:** слитый [PR #47](https://github.com/uzbatsu-beep/MLAD/pull/47), `main` SHA `e72a51f8ea69466e716fb268a62b1cf1bd38d6f2`. **Платформа:** Windows 10/11 x64, Tauri 2, Microsoft Edge WebView2 Runtime, NTFS. Этот документ предназначен для ручного UserSmoke после завершения автоматического CI на точном SHA PR #48.

## 1. Что входит в preview

Артефакт `MLAD-PR48-Windows-x64-OfflinePreview-Unsigned-NonRelease` собирается отдельным [Windows Preview workflow](../.github/workflows/windows-offline-preview.yml). **Загрузка — через Artifacts успешного запуска GitHub Actions**, не через Releases; Git tag не создаётся. Внутри zip:

- `app/yaskawa-ladder-ide.exe` — неподписанная production-сборка Windows x64; это тестовый бинарный файл, не штатное установочное ПО.
- `sample/cell.mladproj.json`, `sample/CIOPRG.LST`, `sample/IONAME.DAT`, `sample/cell.mlad.json` — полностью **СИНТЕТИЧЕСКИЙ** единый проект, изготовленный [генератором](../scripts/offline-preview-fixture.py). Эти байты не являются подтверждённым OEM-форматом/программой робота; в реальный контроллер их не загружать.
- `sample/SYNTHETIC-SHA256.json` — независимая контрольная ведомость четырёх синтетических файлов, а `SHA256SUMS.txt` фиксирует каждый файл пакета.
- `build-evidence.json` — точный Git SHA, SHA256 EXE, статус Authenticode и честные `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`.
- `corresponding-source.zip` — исходный код именно собранного коммита, `LICENSE` — GPL-3.0-only; `tools/windows-offline-preview.ps1` — офлайн-проверка целостности; настоящий протокол в `docs/WINDOWS-OFFLINE-USER-SMOKE.md`.

CI **автоматически** запускает `npm ci`, фронтенд-контракты, Rust-тесты на Windows/NTFS, генерирует и отрицательно тестирует синтетические данные, собирает PE через `tauri build --no-bundle`, проверяет SHA и размещает артефакт без публикации релиза. **CI не запускает и не проверяет интерактивный WebView2 GUI**.

## 2. Безопасная подготовка Windows

1. На отдельном Windows 10/11 x64 ПК/VM с локальным WebView2 Runtime извлеките *оба* уровня архива GitHub Actions и внутреннего ZIP в обычную папку NTFS. На время проверки исключите соединение с производственной сетью роботов. Не используйте оригинальные производственные `CIOPRG.LST` и `IONAME.DAT`.
2. Зафиксируйте `winver`, версию WebView2, SHA коммита из `build-evidence.json`, ссылку на Actions run, дату и исполнителя. До запуска просмотрите исходный код и контрольные суммы. EXE не подписан; Windows SmartScreen/антивирус могут потребовать корпоративного согласования, их обход в рамках этой инструкции не требуется.
3. Откройте PowerShell в корне распакованного внутреннего ZIP. Запустите **только проверку файлов** (стандартный Windows PowerShell 5.1 допустим):

~~~powershell
powershell -NoProfile -File .\tools\windows-offline-preview.ps1 -Mode Verify -StageRoot .
Get-FileHash .\app\yaskawa-ladder-ide.exe -Algorithm SHA256
Get-Content .\build-evidence.json
~~~

4. Скопируйте всю папку `sample` в другую рабочую директорию, например `C:\MLAD-PR48-UserSmoke\cell`. Все изменения, удаление файлов и негативные опыты выполняйте **только в копии**. Записать паспорт рабочего экземпляра, не прикладывать приватные логины, адреса роботов, программы и скриншоты производственных систем.
5. Запустите EXE из `app`. Откройте раздел **Native Workspace**, в поле «Офлайн-проект» укажите абсолютный путь к `cell.mladproj.json` в рабочей копии и нажмите **«Открыть проект»**. Для диагностики открытия также допускается одиночный режим исходного `CIOPRG.LST`.

## 3. Матрица ручной UserSmoke

Все результаты до действия пользователя строго `NOT RUN`. **PASS** ставит только проверяющий после наблюдаемого поведения. **FAIL** — сохраняет шаг воспроизведения и обезличенные признаки ошибки. **BLOCKED** — если шаг нельзя безопасно выполнить.

| ID | Действие (только в копии) | Ожидаемый критерий |
| --- | --- | --- |
| P48-01 | Запустить EXE в Windows | Создаётся окно WebView2; нет blank screen, crash и ошибки CSP/IPC |
| P48-02 | Открыть `cell.mladproj.json` | Одновременно загружены source, IONAME, sidecar, закладка и аналитика |
| P48-03 | Проверить 3 строки `CIOPRG.LST` | Корректно читаются STR, OUT, END; native текст не меняется |
| P48-04 | Перейти к L1/L2 и закладке | Действует навигация source ↔ Ladder ↔ diagnostics, без выдуманного выполнения |
| P48-05 | Проверить `READ`/`WRITE`/`UNKNOWN` | Синтаксические роли находятся и переходят к исходнику; не показаны живые I/O |
| P48-06 | Проверить имена `IONAME.DAT` | Отображаются синтетические метки; автоматического адресного соответствия нет |
| P48-07 | Изменить заметку/закладку и сохранить sidecar | Сохраняется отдельный `*.mlad.json`; нативные файлы неизменны |
| P48-08 | Сохранить манифест после sidecar | Записывается `*.mladproj.json`; второе сохранение проверяет ревизию/backup |
| P48-09 | Перенести *всю* папку проекта в другой каталог | Относительные ссылки и SHA позволяют повторное открытие без исходного пути |
| P48-10 | В рабочей копии изменить байты `CIOPRG.LST` | Повторное открытие отвергнуто; прежняя сессия не заменена |
| P48-11 | В другой копии изменить `IONAME.DAT` | Несоответствие SHA блокирует открытие без частичной подмены |
| P48-12 | В другой копии изменить `cell.mlad.json` | Несоответствие SHA/revision блокирует открытие; нет скрытого импортирования кэша |
| P48-13 | Удалить один компонент из копии | Чёткое сообщение об отсутствующем файле; без потери прежней сессии |
| P48-14 | Редактировать путь source при привязанном проекте | Требуется явное «Отвязать»; нельзя незаметно сменить исходник |
| P48-15 | Проверить боковые замечания и различие версий анализа | `analysis_changed` информирует о пересчёте, сохранённые счётчики не исполняются |
| P48-16 | Проверить отсутствие live/FTP/HSES/deploy/native writer | Только пассивная проверка UI и IPC без контакта с роботами |
| P48-17 | Повторно запустить `Verify` над **нетронутым** образцом | Все SHA совпадают; рабочие копии не используются вместо образца |

Для отрицательных P48-10…P48-13 используйте **независимые копии**, восстанавливайте исходник из ZIP, а не редактируйте файлы в неизменяемом `sample`. Проверка `Verify` подтверждает лишь целостность, **не** работу GUI.

## 4. Бланк результата и критерий решения

Скопируйте таблицу P48-01…P48-17 в отчёт и проставьте каждое `PASS` / `FAIL` / `BLOCKED` / `NOT RUN`. Дополнительно укажите Windows build, WebView2 Runtime, номер версии/точный SHA, Actions URL, SHA256 EXE, дату, обезличенные ошибки и оценку возможности автономного запуска. Не включайте личные файлы или реальные адреса роботов.

`UserSmoke=PASS` допускается только когда **каждый обязательный сценарий** P48-01…P48-17 выполнен и отмечен `PASS`, а результаты переданы владельцем на рассмотрение. Если хотя бы один FAIL, BLOCKED или NOT RUN — итог `UserSmoke=NOT PASSED` либо `NOT RUN`. Публикация Release, слияние PR и снятие блокировок аппаратного интерфейса — отдельные решения владельца.

## 5. Ограничения и безопасность

- Никаких заключений об OEM Controller/firmware acceptance, system/user ladder partition, EN/scan/Carry/Zero/Error: `OEMBench=NOT RUN`.
- Native `CIOPRG.LST` и `IONAME.DAT` строго read-only; `FTP`, `HSES`, native writer, live I/O и deployment `BLOCKED`.
- Preview unsigned, без подписи издателя и без инсталлятора. Не отменять корпоративные политики безопасности ради запуска.
- Windows DACL/owner inheritance, race между проверкой родителя/открытием или rename, точный multi-file atomic transaction и настоящие одновременные писатели не доказаны.
- Успешный CI на Windows доказывает сборку, проход тестов и упаковку; `UserSmoke` и `OEMBench` **не** подтверждаются GitHub Actions.

[Windows security baseline](windows_acceptance_pr28.md) · [Project manifest plan](implementation_plan.md) · [Verification gates](verification_gates.md).
