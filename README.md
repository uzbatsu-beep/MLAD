# MLAD — Motoman Ladder Analysis & Draft IDE

**Сверка 18.11.2026:** проверенный `main` SHA `3c184cd36e076dc2e5c9aed7a663b601b4899dea`, PR #1–#28 merged; [Linux validation SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44365314264), [Windows security CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44365314173). PR #29 — отдельный **Draft** (не в `main`). Последний опубликованный Windows pre-release — rc.9, **не** включает PR #24–#29. [Матрица](docs/project_status.md).

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

**Сохранение JSON:** PR #24 слит в `main`; используются staged file/rename и `.bak`, отдельные Windows MSVC tests прошли, но ручной Windows smoke пока не проводился. Нативные файлы роботов остаются read-only. [Детали и ограничения](docs/json_persistence_safety.md).

**Новый PR #29 (Draft):** сравнение File ID/inode наряду с JSON-байтами, отказ для hardlink target/backup, проверка идентичности и SHA-256 staged JSON перед rename; сохраняются окна TOCTOU, ACL/permissions и отсутствие атомарной транзакции backup+target. [Подробный аудит](docs/filesystem_security_pr29.md).\n\n**Усиление защиты (PR #25 и #26 merged):** CSP ограничивает внешние загрузки/JS eval, а файловые проверки выявляют часть symlink/reparse-подмен и конфликтов изменения JSON. [CSP](docs/csp_path_hardening.md) · [guarded reads и остаточные гонки](docs/guarded_io_review.md). [Windows MSVC/Tauri CI для PR #26 — SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44227145031), **ручной Windows smoke не подтверждён**; полная защита от TOCTOU не заявляется.

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

GitHub Actions — [MLAD validation](.github/workflows/validation.yml). PR #28 предлагает [постоянный Windows regression workflow](.github/workflows/windows-security-acceptance.yml) и [ручной протокол Windows](docs/windows_acceptance_pr28.md), пока только в Draft. Windows rc.9 проверен [отдельным историческим workflow](.github/workflows/windows-oem-reference-rc9.yml). CI **не** означает проверку GUI на пользовательском Windows или OEM-эквивалентность.

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


### PR #26 — устойчивость чтения и защита от конкурирующего сохранения (merged)

Новые чтения `CIOPRG.LST`, `IONAME.DAT` и JSON используют один дескриптор и сверяют идентичность файла до/после открытия и после чтения. Прямая reparse-ссылка в Windows отклоняется. Для JSON перед заменой проверяется, что исходное содержимое не изменилось с момента подготовки. **Полный TOCTOU, межпроцессная блокировка и атомарность пары backup+target остаются открытыми.** [Модель угроз и тесты](docs/guarded_io_review.md).
