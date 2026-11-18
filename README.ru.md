<!-- Generated ru companion for README.md; review required. -->
> **Русский** · [Русский](README.ru.md) · [English](README.en.md) · [Original / Оригинал](README.md)
> **Translation review / Проверка:** автоматический черновик — сверяйте инженерные утверждения с оригиналом и OEM документацией.

# MLAD - анализ мотоманской лестницы и проект IDE

**Сверка 18.11.2026:** [main](https://github.com/uzbatsu-beep/MLAD) @ `f85ddfcce3063dd087425be981a9d2b7a16d6309`; **PR #1#29 Слияние **. [Linux CI Успех](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454), [Windows CI Успех](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621). Последний опубликованный Windows релиз — **rc.9**, он **не включает PR #24–#29**. Документационная актуализация — [дорожная карта v3](docs/roadmap.ru.md), [полный статус](docs/project_status.ru.md); ручной WebView2/OEM скамейка ** Не бегать**

**Последний опубликованный Windows x64 Preview:** [v0.5.0-rc.9-oem-reference](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference) (портативный ZIP, настройка NSIS, архив источников GPL, SHA-256). [Основной CI — SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44166354563), [Windows RC9 — успех](https://github.com/uzbatsu-beep/MLAD/actions/runs/44162078155).

> **Безопасность:** MLAD не является сертифицированной средой Yaskawa или подтверждённым компилятором `CIOPRG.LST`. Оригинальные `CIOPRG.LST` и `IONAME.DAT` открываются **только для чтения**; native writer, FTP/HSES, загрузка в робот и реальная телеметрия выключены. Для работы с оборудованием применять официальные средства производителя и отдельную процедуру допуска.

## Фактически доступно

| Функция | Состояние |
| --- | --- |
| Профили DX100/DX200/YRC1000 | Типизированные форматы адресов и проверка диапазонов; контроллерную карту/firmware ещё требуется подтвердить |
| Родное рабочее пространство | Главная рабочая область: компактная шапка, системный диалог открытия, дерево, постраничный исходник, поиск и диагностика |
| CIOPRG.LST | Побайтовая неизменность, SHA-256, разделы после `END` как данные, безопасная классификация поддерживаемого/неизвестного |
| Ladder | **Все распознанные** цепи текущего SemanticPreview, масштаб, ссылки на исходный текст; неизвестные фрагменты не дорисовываются |
| TMR/CNT/GSTR/GOUT | Просмотр native-паттернов и отдельные **офлайн-черновики** `*.advanced.json` |
| IONAME.DAT / комментарии | Read-only preview, локальные комментарии цепей/адресов, ручные ассоциации; оригиналы не меняются |
| `MOV`, `ADD`, `SUB`, `SHL`, `SHR` | Типизированные операнды и исходные spans, статические **предположительные** M-зависимости |
| OEM-справка | Документальные формулы EN/16-bit/Carry/Zero/Error и автономные reference-тесты; **не** исполнение исходника на контроллере |
| Проект JON Graph | SVG, AND/OR/NO/NC/OUT, Undo/Redo, двойной щелчок/F2 по адресам и офлайн TRUE/FALSE/UNKNOWN |
| Наследие JSON | Совместимость со старым `project.json` через отдельный пункт меню |

**Сохранение и защита JSON (PR #24#29 слияние: ** постановочная запись + `*.bak`, ограниченное чтение, конфликт по байтам **и** File ID, отказ для hardlinks, проверка временного файла по SHA-256, отказ Windows ADS/UNC/device, production CSP. Это **не** транзакция backup+target и **не** полное решение TOCTOU/ACL/crash recovery. [Обзор безопасности](docs/filesystem_security_pr29.ru.md), [Проверочные ворота](docs/verification_gates.ru.md). Ручной Windows UserSmoke — **NOT RUN**.

**Границы форматов:** `CIOPRG.LST`/ `IONAME.DAT` — только источник; `*.graph.json` / `*.advanced.json` — отдельные изменяемые черновики. Нельзя сохранять JSON под видом native CIOPRG. Комментарии native Workspace сохраняются в localStorage и **не** равны OEM-комментариям файла робота.

## Быстрый старт разработчика

Текущий стек: Rust + Tauri v2 + React 18 + TypeScript/Vite; Node.js 22 в CI, Windows MSVC + WebView2 для Windows сборки. `package-lock.json` и `src-tauri/Cargo.lock` отслеживаются в Git.

```bash
npm ci --no-audit --no-fund
npm run tauri dev
```

Полная локальная проверка (если установлены Rust/Tauri prerequisites):

```bash
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

Действия GitHub: [Проверка MLAD](.github/workflows/validation.yml) и постоянно действующий [Прием Windows и регрессия безопасности](.github/workflows/windows-security-acceptance.yml). [Ручной протокол WIN-01…WIN-12](docs/windows_acceptance_pr28.ru.md) ещё не закрыт. Windows rc.9 проверен [историческим release workflow](.github/workflows/windows-oem-reference-rc9.yml); после PR #29 опубликован только временный unsigned non-release EXE. CI **не** означает прохождение пользовательского GUI/WebView2 smoke или OEM-эквивалентность.

## Начать работу

1. В основной вкладке «Программа робота» выберите семейство DX100/DX200/YRC1000, откройте **копию** `CIOPRG.LST` через кнопку «Обзор…» и проверьте SHA-256.
2. Просмотрите постраничный исходник, поддерживаемые цепи Ladder, блоки и диагностику. Для `MOV/ADD/SUB/SHL/SHR` доступна справка OEM — **без расчёта состояний реальных I/O**.
3. При необходимости отдельно откройте `IONAME.DAT`. Корректность соответствия имён адресам не предполагается; используйте только ручные локальные привязки.
4. Для редактируемой логики используйте **отдельный JSON-черновик** Graph/Advanced, а не оригинальную программу робота.

## Roadmap, доказательства и ограничения

- [Текущий статус](docs/project_status.ru.md), [дорожная карта](docs/roadmap.ru.md), [план следующих PR](docs/implementation_plan.ru.md), [архитектура](docs/architecture.ru.md).
- [Критерии проверок и выпуска](docs/verification_gates.ru.md), [реестр решений D1–D9](docs/engineering_decisions.ru.md).
- [Синтаксис native](docs/cioprg_lst_spec.ru.md), [типизированные команды](docs/typed_opcode_grammar.ru.md), [пять OEM команд](docs/oem_5_opcode_analysis.ru.md), [матрица семантической верификации](docs/semantic_verification_matrix.ru.md).
- [Корпус оригинальных файлов](docs/controller_fixture_intake.ru.md), [программа стендовых испытаний](docs/advanced_hardware_test_matrix.ru.md).
- [Каталог всех Markdown](docs/README.ru.md), [процесс PR и выпусков](docs/development_workflow.ru.md).

## Сообщество, сообщения об ошибках и безопасность

- [Открыть ошибку, предложение функции или проверку совместимости](https://github.com/uzbatsu-beep/MLAD/issues/new/choose) — через структурированные формы с **синтетическими** воспроизводимыми примерами.
- [SECURITY.md](SECURITY.ru.md) — порядок **непубличного** сообщения об уязвимостях; не публикуйте эксплойты/закрытые данные оборудования.
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.ru.md) — правила профессионального взаимодействия.
- [CONTRIBUTING.md](CONTRIBUTING.ru.md) и [шаблон PR](.github/PULL_REQUEST_TEMPLATE.ru.md) — тесты, источники и ревью.
- [ACCESSIBILITY.md](ACCESSIBILITY.ru.md) — доступность интерфейса и известные ограничения, **без заявления соответствия WCAG**.

**GitHub Community Standards — организационная документация, не сертификат безопасности.** Публичные Issues и PR не предназначены для оригинальных CIOPRG.LST, IONAME.DAT, сетевых параметров, закрытых журналов и программ роботов.

## Лицензия

**GNU GPL-3.0-only** — [LICENSE](LICENSE). Правообладатель собственных оригинальных материалов MLAD: **Фёдор Сергеевич Кочедыков** (физическое лицо), © 2026. Право собственности на чужие библиотеки, материалы Yaskawa и пользовательские данные не переходит к MLAD; third-party notices и chain-of-title ещё требуют отдельной проверки. Подробнее: [docs/licensing.md](docs/licensing.ru.md).

MLAD — независимый проект, **не разработан и не одобрен Yaskawa**.


## Дальнейшая разработка

**Следующий цикл:** [PR #30 — документация](docs/implementation_plan.ru.md), далее предложены синтетические parser fixtures, улучшение Native Workspace, versioned sidecar `*.mlad.json`, typed syntax, source-linked static analysis, Unified Offline IDE и условный Windows 0.6.x preview. Отдельные PR с тестами и **без автоматического merge**. [Дорожная карта v3](docs/roadmap.ru.md) · [подробный backlog](docs/implementation_plan.ru.md) · [принятые инженерные решения](docs/engineering_decisions.ru.md).
