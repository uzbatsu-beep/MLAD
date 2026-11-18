# MLAD PR #28 — Windows Acceptance & Security Regression

**Область:** Windows 10/11 x64, WebView2, NTFS, Rust/Tauri v2, OFFLINE/BENCH. **Base:** `main` после [PR #27](https://github.com/uzbatsu-beep/MLAD/pull/27), SHA `bdb443eb65dc134edc2e11ffc3230c9dfa6257df`. Статус: [PR #28 merged](https://github.com/uzbatsu-beep/MLAD/pull/28) в `main` SHA `3c184cd36e076dc2e5c9aed7a663b601b4899dea`; [Linux CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44365314264), [Windows CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44365314173). **Не** релиз и не аппаратная приёмка; UserSmoke ещё `NOT RUN`.

## 1. Что действительно автоматизировано

Новый [Windows workflow](../.github/workflows/windows-security-acceptance.yml) на `windows-2022` запускает:

1. `npm ci` с проверкой неизменности lockfile, frontend/security/native/docs проверки и `npm run build`.
2. Node-контракт [windows-security-contract.test.mjs](../scripts/windows-security-contract.test.mjs): offline IPC deny-only, production CSP, запрет внешних ресурсов, обязательные Windows-тесты.
3. Полный `cargo test --manifest-path src-tauri/Cargo.toml --locked` **на Windows**, плюс проверку наличия скомпилированных Windows-specific тестов.
4. Отрицательные Rust-тесты с **NTFS exclusive sharing lock** (`share_mode(0)`) и настоящим Windows **junction** (`mklink /J`). Исходные байты сохраняются при отказе JSON write/native read. Имеющиеся тесты проверяют external JSON change, competing creation и file identity.
5. `npm run tauri -- build --no-bundle`, проверку существования EXE, PE `MZ` header, SHA-256 и Authenticode status (включая честный `NotSigned`).
6. Создание/проверку **исключительно синтетического** CIOPRG/IONAME sample; реальных программ роботов и производственных адресов нет.
7. GitHub Actions artifact **unsigned EXE**, контрольной суммы и отчёта с хранением 14 дней; GitHub Release и tag не создаются.

**Не тестируется автоматически:** видимый GUI в WebView2, настоящий робот, penetration-test, агрессивные directory races, абсолютная атомарность backup+target, OEM editor compatibility. GitHub Actions **не** выставляет `UserSmoke=PASS` или `OEMBenchVerified=PASS`.

## 2. Тестовый артефакт

Откройте workflow PR #28 → успешный запуск `MLAD Windows Acceptance & Security Regression` → `MLAD-PR28-Windows-Unsigned-Acceptance-NonRelease`. В нём:

- `yaskawa-ladder-ide.exe` — неподписанный тестовый EXE с требованием установленного WebView2 Runtime, не production release.
- `SHA256SUMS.txt` — checksum именно этой сборки.
- `binary-evidence.json` — SHA, размер, Authenticode и `NOT RUN` для ручного/OEM тестов.
- `LICENSE`, этот протокол и `windows-acceptance.ps1`.

Не запускать EXE в производственной сети или на компьютере с непредусмотренным доступом к контроллерам. Проверять только собственные синтетические данные или разрешённые локальные **копии**. Предупреждение Windows SmartScreen о неподписанном EXE не подтверждает его опасность или безопасность; соблюдайте правила организации.

## 3. Подготовка ручной проверки

Зафиксировать Windows build (`winver`), WebView2 Runtime, build SHA из `binary-evidence.json`, исполнителя, дату и результаты. Сверить EXE:

~~~powershell
Get-FileHash .\yaskawa-ladder-ide.exe -Algorithm SHA256
Get-Content .\SHA256SUMS.txt
~~~

Создать новую тестовую папку (повторный запуск Prepare в существующей папке намеренно отвергается):

~~~powershell
$fixtures = Join-Path $env:TEMP ("MLAD-PR28-" + [guid]::NewGuid().ToString("N"))
pwsh -NoProfile -File .\windows-acceptance.ps1 -Mode Prepare -FixtureRoot $fixtures
~~~

Подпапка `native` содержит **искусственные** `CIOPRG.LST` и `IONAME.DAT`: они НЕ OEM-совместимы и предназначены для демонстрации read-only, unknown/error handling и SHA-256. Папка `drafts` — для **созданных через UI** JSON-проектов. Отображение unsupported/opaque или отклонение демонстрационного формата нормально; оно не подтверждает полноту парсера.

## 4. Ручная матрица UserSmoke

Для каждого ID записать `PASS`, `FAIL` или `NOT RUN` и наблюдение. Все сценарии изначально `NOT RUN`.

| ID | Сценарий | Критерий |
| --- | --- | --- |
| WIN-01 | Запуск EXE / создание окна | Tauri/WebView2 показывает UI, нет crash/blank page |
| WIN-02 | Native Workspace | Системный Open выбирает синтетический `CIOPRG.LST`, показывает source/сообщает внятную ошибку |
| WIN-03 | Read-only / diagnostics | Источник/opaque/сообщения видны корректно, нет операции сохранения нативного `.LST` |
| WIN-04 | SHA fidelity | Повторный `-Mode Verify` подтверждает SHA-256 обоих исходных файлов |
| WIN-05 | `IONAME.DAT` | Read-only preview либо корректный отказ для искусственного формата; original не изменён |
| WIN-06 | Graph JSON | Создать/дважды сохранить отдельный `*.graph.json`, проверить `*.bak` и повторное открытие |
| WIN-07 | Advanced JSON | Создать/дважды сохранить `*.advanced.json` и проверить backup |
| WIN-08 | Locked JSON | Проверить отказ при заблокированном файле без truncation; если не воспроизведено — `NOT RUN` |
| WIN-09 | Invalid/oversize | Некорректные синтетические данные не вешают UI, сообщается ошибка |
| WIN-10 | CSP/local IPC | Работают поиск, zoom, допустимые Open/dialog и IPC; наличие этого UI не доказывает отсутствие внешних запросов |
| WIN-11 | Deny-only | Нет активного LIVE, HSES/FTP upload, native export/writer; без подключения к роботу |
| WIN-12 | Exit/reopen | Перезапуск и повторное открытие JSON, исходные файлы неизменны |

После работы:

~~~powershell
pwsh -NoProfile -File .\windows-acceptance.ps1 -Mode Verify -FixtureRoot $fixtures
~~~

Несовпадение SHA означает **FAIL** и остановку тестирования. Verify подтверждает **только неизменность искусственных исходников**, не правильность Ladder или работу GUI.

## 5. Границы доказательности

- **Concurrent JSON:** Rust-тест моделирует изменение между staging и commit. Изменение файла до начала Save из интерфейса и race после последней проверки всё ещё не гарантированно обнаруживаются.
- **Windows lock:** поведение зависит от sharing mode и ОС. Новая Rust-проверка применяет жёсткий `share_mode(0)`; ручной тест может оставаться `NOT RUN`.
- **Reparse:** тест проверяет *стабильную* junction, не межпроцессную гонку с подменой ancestry. Hardlinks/ACL/power loss — открытые риски.
- **CSP:** конфигурация и production build проверены автоматически, ручной WebView2/IPC runtime не подтверждён до фактического тестирования.
- **Robot/network:** запреты проверяются автономно и не предполагают попыток работы на производственном оборудовании.

## 6. Протокол acceptance (заполнить после проверки)

| Поле | Статус |
| --- | --- |
| Исполнитель/дата/Windows build/WebView2 | НЕ ЗАПОЛНЕНО |
| Artifact URL/Git SHA/SHA-256 | НЕ ЗАПОЛНЕНО |
| WIN-01…WIN-12 | `NOT RUN` до каждого сценария |
| Synthetic SHA-256 Verify | `NOT RUN` |
| Windows automated CI | Только фактический run URL + status |
| UserSmoke итог | `NOT RUN` |
| OEM bench | `NOT RUN` |
| HSES/FTP/native writer/deployment | `BLOCKED` |
| Дефекты/evidence | Не прикладывать реальные файлы/адреса/пароли |
| Решение об acceptance/release | Отдельно согласовать с владельцем |

[Verification gates](verification_gates.md) · [Implementation plan](implementation_plan.md) · [Threat review](guarded_io_review.md).

## 7. Дополнение PR #29 (Draft)

Новый [filesystem review](filesystem_security_pr29.md) предусматривает отдельные тесты same-content target swap, hardlink alias и staged-file corruption. Это будущая проверка Rust/Windows CI, но она не заменяет ни один WIN-01…WIN-12. Для Windows UserSmoke по-прежнему нужен ручной протокол на конкретном EXE и SHA.
