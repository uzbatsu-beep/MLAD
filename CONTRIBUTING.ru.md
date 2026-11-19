<!-- Generated ru companion for CONTRIBUTING.md; review required. -->
> **Русский** · [Русский](CONTRIBUTING.ru.md) · [English](CONTRIBUTING.en.md) · [Original / Оригинал](CONTRIBUTING.md)
> **Translation review / Проверка:** автоматический черновик — сверяйте инженерные утверждения с оригиналом и OEM документацией.
> **Контрольные ограничения MLAD:** `CIOPRG.LST` и `IONAME.DAT` — строго **только чтение**. FTP/HSES, live I/O, native writer и загрузка в контроллер — **BLOCKED (запрещены)**. `WindowsCI` не заменяет `UserSmoke=NOT RUN` и `OEMBench=NOT RUN`; машинный перевод должен быть сверен с исходным документом и документацией Yaskawa.

# Участие в разработке MLAD

> **Состояние 18.11.2026:** PR #1–#29 merged в `main` @ `f85ddfcce3063dd087425be981a9d2b7a16d6309`; [Linux CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454) и [Windows CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621) SUCCESS. Последний публичный Windows релиз — rc.9 (до #24–#29), а WebView2 UserSmoke/OEMBench `NOT RUN`. [Статус проекта](docs/project_status.ru.md).

Работа с исходниками MLAD требует разделять **описанные Yaskawa инструкции** и **непроверенные детали нативного файла/протокола**. Любой PR должен сохранять запрет записи в контроллер без независимого разрешённого workflow.

## Прежде чем начинать

1. Прочитать [README](README.ru.md), [архитектуру](docs/architecture.ru.md) и [реестр Yaskawa источников](docs/source_registry.ru.md).
2. Сверить текущий `main`, последний CI и открытые PR. Не считать актуальным статус из старой переписки или release note.
3. Выбрать одну цель PR; готовить код, тесты и обновление соответствующего документа одновременно. Для планирования: [implementation_plan.md](docs/implementation_plan.ru.md); для доказательств: [verification_gates.md](docs/verification_gates.ru.md).
4. Не проводить слияние без прямого согласования владельца проекта.

## Правила PR

- Контроллерозависимая логика обязательно содержит явно выбранный `ControllerFamily` и разграничение `Candidate` vs реальные права записи.
- Истинные регистры/адреса и значения моделируются типами; `TMR` и `CNT` имеют состояние, у `CNT` отдельные C и R, `GSTR/GOUT` — парный байт.
- **Не** добавлять нативный `CIOPRG.LST` писатель, подключение к роботу или команду FTP/HSES на основании синтетического теста.
- Оригинальный файл не должен менять байты, кодировку, заголовки или разделы, даже если UI не умеет интерпретировать их.
- Любое новое правило адресации указывать с ревизией инструкции Yaskawa и с позитивными/негативными тестами.
- Для UI работать с тремя состояниями `TRUE/FALSE/UNKNOWN`; если данные не получены от робота через верифицированный протокол, писать **OFFLINE**.
- Коммитить Markdown, тест-кейсы, описание ограничений; не коммитить клиентские файлы, пароли, IP-адреса из производственных сетей или секреты.

## Лицензия вкладов

Исходный код MLAD предлагается на условиях [GNU GPL-3.0-only](LICENSE). Включаемые в MLAD программные изменения должны допускать распространение на этих условиях; автор вкладов должен обладать необходимыми правами на их публикацию. Не импортировать код, схемы, фирменную документацию и `CIOPRG.LST` третьих лиц без разрешения. Исходные лицензии зависимостей должны учитываться отдельно; не считать GPL автоматическим изменением лицензии сторонних библиотек. См. [docs/licensing.md](docs/licensing.ru.md).

## Community Standards и безопасная подача материалов

- Прочитайте [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.ru.md), [SECURITY.md](SECURITY.ru.md) и [ACCESSIBILITY.md](ACCESSIBILITY.ru.md).
- Новые ошибки, запросы функций и совместимость DX100/DX200/YRC1000 оформляйте через [структурированные Issues](https://github.com/uzbatsu-beep/MLAD/issues/new/choose) **только с синтетическими примерами**. Если обнаружена уязвимость, **не** раскрывайте детали публично, следуйте SECURITY.md.
- Для PR используйте [шаблон проверки](.github/PULL_REQUEST_TEMPLATE.ru.md): границы безопасности, уровень доказательности, негативные тесты, непроверенные сценарии и явное разрешение на слияние.
- Репозиторий открыт: нельзя прикладывать реальные CIOPRG.LST, IONAME.DAT, закрытые схемы, производственные IP или секреты.

## Локальная проверка

```bash
npm ci --no-audit --no-fund
npm run test:community
npm run test:security
npm run test:native
npm run test:diagnostics
npm run test:oem-reference
npm run test:graph
npm run test:advanced
npm run test:visual
npm run test:inline
npm run docs:check
npm run build
cargo test --manifest-path src-tauri/Cargo.toml --locked
```

`npm run tauri dev` для ручной проверки окна; Windows portable preview является отдельным артефактом и не обязателен для каждого Markdown PR.

## Приёмка

В PR обязательно указывать: исходный SHA `main`, затронутые контроллеры, ссылки на Yaskawa или `UNKNOWN`, полученные автоматические тесты, ручные проверки и **что именно не проверялось**.

Матрица аппаратных тестов: [advanced_hardware_test_matrix.md](docs/advanced_hardware_test_matrix.ru.md).  
Процесс: [development_workflow.md](docs/development_workflow.ru.md).  
Первичные файлы: [controller_fixture_intake.md](docs/controller_fixture_intake.ru.md).

## Применимость тестов и статус PR

Для каждого PR указывать фактически пройденные CI, SHA и `NOT RUN`, а не подразумевать выполнение Windows UserSmoke или OEM Bench. Статусы `ManualDerived`, `SyntheticTested`, `WindowsCI`, `OEMBenchVerified` и `Authorized` независимы; [единая матрица](docs/verification_gates.ru.md). Принятые ограничения D1–D9: [engineering_decisions.md](docs/engineering_decisions.ru.md). Изменение этих решений и публикация release требуют отдельного согласия владельца.

## Разработка после PR #29

[Дорожная карта v3](docs/roadmap.ru.md) и [backlog PR #30–#37](docs/implementation_plan.ru.md) разделяют синтетические parser fixtures, Native Workspace navigation, переносимые annotations/sidecar, typed grammar и static analysis. Каждый PR должен указывать физический source span, уровень доказательности, negative cases, не модифицировать `CIOPRG.LST`/`IONAME.DAT` и не добавлять network/native writer разрешений. При работе с JSON учитывать [Идентификатор файла/hardlink/staging guards](docs/filesystem_security_pr29.ru.md), но **не** считать TOCTOU/ACL/backup transaction закрытыми. Для Rust/Tauri изменений обязателен Windows workflow; для docs-only PR достаточно основного CI. Владелец отдельно утверждает merge и release.
