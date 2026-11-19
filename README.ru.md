<!-- MLAD-I18N:EDITORIAL:reviewed 2026-11-19 against README.md -->
> **Язык / Language:** [Русский](README.ru.md) · [English](README.en.md) · [Исходный текст / Original](README.md)
>
> **Область применения:** перевод сохраняет исходные технические обозначения и ограничения. При расхождении приоритет имеет исходная спецификация и официальная документация производителя. Наличие перевода не подтверждает OEM-совместимость или право записи в контроллер.

# MLAD — Motoman Ladder Analysis & Draft IDE

**Сверка 18.11.2026:** [main](https://github.com/uzbatsu-beep/MLAD) @ `f85ddfcce3063dd087425be981a9d2b7a16d6309`; **PR #1–#29 merged**. [Linux CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454), [Windows CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621). Последний опубликованный Windows релиз — **rc.9**, он **не включает PR #24–#29**. Документационная актуализация — [roadmap v3](docs/roadmap.md), [полный статус](docs/project_status.md); ручной WebView2/OEM bench **NOT RUN**.

**Последний опубликованный Windows x64 Preview:** [v0.5.0-rc.9-oem-reference](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.9-oem-reference) (portable ZIP, NSIS setup, GPL source archive, SHA-256). [Основной CI — SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44166354563), [Windows RC9 — SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44162078155).

> **Безопасность:** MLAD не является сертифицированной средой Yaskawa или подтверждённым компилятором `CIOPRG.LST`. Оригинальные `CIOPRG.LST` и `IONAME.DAT` открываются **только для чтения**; native writer, FTP/HSES, загрузка в робот и реальная телеметрия выключены. Для работы с оборудованием применять официальные средства производителя и отдельную процедуру допуска.

## Фактически доступно

| Функция | Состояние |
| --- | --- |
| Профили DX100/DX200/YRC1000 | Типизированные форматы адресов и проверка диапазонов; контроллерную карту/firmware ещё требуется подтвердить |
| Native Workspace | Главная рабочая область: компактная шапка, системный диалог открытия, дерево, постраничный исходник, поиск и диагностика |
| CIOPRG.LST | Побайтовая неизменность, SHA-256, разделы после `END` как данные, безопасная классификация поддерживаемого/неизвестного |
| Ladder | **Все распознанные** цепи текущего SemanticPreview, масштаб, ссылки на исходный текст; неизвестные фрагменты не дорисовываются |
| TMR/CNT/GSTR/GOUT | Просмотр native-паттернов и отдельные **офлайн-черновики** `*.advanced.json` |
| IONAME.DAT / комментарии | Read-only preview, локальные комментарии цепей/адресов, ручные ассоциации; оригиналы не меняются |
| `MOV`, `ADD`, `SUB`, `SHL`, `SHR` | Типизированные операнды и исходные spans, статические **предположительные** M-зависимости |
| OEM-справка | Документальные формулы EN/16-bit/Carry/Zero/Error и автономные reference-тесты; **не** исполнение исходника на контроллере |
| JSON Graph Draft | SVG, AND/OR/NO/NC/OUT, Undo/Redo, двойной щелчок/F2 по адресам и офлайн TRUE/FALSE/UNKNOWN |
| Legacy JSON | Совместимость со старым `project.json` через отдельный пункт меню |

**Сохранение и защита JSON (PR #24–#29 merged):** staged write + `*.bak`, ограниченное чтение, конфликт по байтам **и** File ID, отказ для hardlinks, проверка временного файла по SHA-256, отказ Windows ADS/UNC/device, production CSP. Это **не** транзакция backup+target и **не** полное решение TOCTOU/ACL/crash recovery. [Security review](docs/filesystem_security_pr29.md), [verification gates](docs/verification_gates.md). Ручной Windows UserSmoke — **NOT RUN**.

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

GitHub Actions: [MLAD validation](.github/workflows/validation.yml) и постоянно действующий [Windows Acceptance & Security Regression](.github/workflows/windows-security-acceptance.yml). [Ручной протокол WIN-01…WIN-12](docs/windows_acceptance_pr28.md) ещё не закрыт. Windows rc.9 проверен [историческим release workflow](.github/workflow-archive/windows-oem-reference-rc9.yml); после PR #29 опубликован только временный unsigned non-release EXE. CI **не** означает прохождение пользовательского GUI/WebView2 smoke или OEM-эквивалентность.

## Начать работу

1. В основной вкладке «Программа робота» выберите семейство DX100/DX200/YRC1000, откройте **копию** `CIOPRG.LST` через кнопку «Обзор…» и проверьте SHA-256.
2. Просмотрите постраничный исходник, поддерживаемые цепи Ladder, блоки и диагностику. Для `MOV/ADD/SUB/SHL/SHR` доступна справка OEM — **без расчёта состояний реальных I/O**.
3. При необходимости отдельно откройте `IONAME.DAT`. Корректность соответствия имён адресам не предполагается; используйте только ручные локальные привязки.
4. Для редактируемой логики используйте **отдельный JSON-черновик** Graph/Advanced, а не оригинальную программу робота.

## Roadmap, доказательства и ограничения

- [Текущий статус](docs/project_status.md), [дорожная карта](docs/roadmap.md), [план следующих PR](docs/implementation_plan.md), [архитектура](docs/architecture.md).
- [Критерии проверок и выпуска](docs/verification_gates.md), [реестр решений D1–D9](docs/engineering_decisions.md).
- [Синтаксис native](docs/cioprg_lst_spec.md), [типизированные команды](docs/typed_opcode_grammar.md), [пять OEM команд](docs/oem_5_opcode_analysis.md), [матрица семантической верификации](docs/semantic_verification_matrix.md).
- [Корпус оригинальных файлов](docs/controller_fixture_intake.md), [программа стендовых испытаний](docs/advanced_hardware_test_matrix.md).
- [Каталог всех Markdown](docs/README.md), [процесс PR и выпусков](docs/development_workflow.md).

## Сообщество, сообщения об ошибках и безопасность

- [Открыть ошибку, предложение функции или проверку совместимости](https://github.com/uzbatsu-beep/MLAD/issues/new/choose) — через структурированные формы с **синтетическими** воспроизводимыми примерами.
- [SECURITY.md](SECURITY.md) — порядок **непубличного** сообщения об уязвимостях; не публикуйте эксплойты/закрытые данные оборудования.
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) — правила профессионального взаимодействия.
- [CONTRIBUTING.md](CONTRIBUTING.md) и [шаблон PR](.github/PULL_REQUEST_TEMPLATE.md) — тесты, источники и ревью.
- [ACCESSIBILITY.md](ACCESSIBILITY.md) — доступность интерфейса и известные ограничения, **без заявления соответствия WCAG**.

**GitHub Community Standards — организационная документация, не сертификат безопасности.** Публичные Issues и PR не предназначены для оригинальных CIOPRG.LST, IONAME.DAT, сетевых параметров, закрытых журналов и программ роботов.

## Лицензия

**GNU GPL-3.0-only** — [LICENSE](LICENSE). Правообладатель собственных оригинальных материалов MLAD: **Фёдор Сергеевич Кочедыков** (физическое лицо), © 2026. Право собственности на чужие библиотеки, материалы Yaskawa и пользовательские данные не переходит к MLAD; third-party notices и chain-of-title ещё требуют отдельной проверки. Подробнее: [docs/licensing.md](docs/licensing.md).

MLAD — независимый проект, **не разработан и не одобрен Yaskawa**.


## Дальнейшая разработка

**Следующий цикл:** [PR #30 — документация](docs/implementation_plan.md), далее предложены синтетические parser fixtures, улучшение Native Workspace, versioned sidecar `*.mlad.json`, typed syntax, source-linked static analysis, Unified Offline IDE и условный Windows 0.6.x preview. Отдельные PR с тестами и **без автоматического merge**. [Дорожная карта v3](docs/roadmap.md) · [подробный backlog](docs/implementation_plan.md) · [принятые инженерные решения](docs/engineering_decisions.md).
