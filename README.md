<!-- MLAD-I18N:NAV:START -->
> 🌐 **Язык / Language:** [Русский](README.ru.md) · [English](README.en.md) · [Original / Исходный текст](README.md). **Machine translation / Машинный перевод: editorial review required / необходима редакторская проверка.**
<!-- MLAD-I18N:NAV:END -->

# MLAD — Motoman Ladder Analysis & Draft IDE

**Офлайн-среда инженера для анализа Yaskawa Motoman Concurrent I/O.** Независимый проект, не официальный продукт Yaskawa.

[Скачать Windows Preview](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.10-unified-offline) · [Документация](docs/README.md) · [Roadmap v5](docs/roadmap.md) · [CHANGELOG](CHANGELOG.md)

| Поставка | Состояние |
| --- | --- |
| Windows x64 pre-release | v0.5.0-rc.10-unified-offline |
| Разработка | Semantic Core v2, PR #51 merged; концепция UI, PR #52 merged |
| Лицензия | GPL-3.0-only |
| Ручная приёмка | UserSmoke=NOT RUN; OEMBench=NOT RUN |

## Что уже работает

| Возможность | Граница |
| --- | --- |
| DX100 / DX200 / YRC1000 | Адресные модели; firmware и OEM-семантика не полностью подтверждены |
| Native Workspace | Read-only `CIOPRG.LST`, SHA-256, исходный текст, поиск, диагностика |
| Ladder | Распознанные цепи, ссылки на исходник; предпросмотр ограничен 200 цепями |
| IONAME.DAT | Просмотр и ручные ассоциации; карта имён не подтверждена |
| Static analysis | READ/WRITE/UNKNOWN с исходными ссылками; не scan model |
| Offline project | `*.mladproj.json`, `*.mlad.json` v1; отдельные комментарии и закладки |
| JSON drafts | Graph/Advanced; редактор, tri-state, ограниченные формы TMR/CNT/GSTR/GOUT |

Состояние main и опубликованной сборки различается. [Матрица возможностей](docs/project_status.md).

## Быстрый старт пользователя

1. Выберите portable ZIP или NSIS setup на [странице релиза](https://github.com/uzbatsu-beep/MLAD/releases/tag/v0.5.0-rc.10-unified-offline). Сверьте SHA-256 по приложенному SHA256SUMS.txt; сборка не подписана.
2. Откройте копию исходного файла в Native Workspace, выберите семейство контроллера.
3. Исследуйте текст, цепи, блоки и диагностику. При необходимости откройте `IONAME.DAT`.
4. Сохраняйте аннотации/манифест отдельно. Редактирование только в JSON-черновиках, не в оригинальной программе.

## Разработка

Стек: Rust, Tauri 2, React, TypeScript, Vite; Node.js 22 в CI. Требуются Rust и системные зависимости Tauri (Windows MSVC/WebView2).

```bash
npm ci --no-audit --no-fund
npm run tauri dev
npm run docs:check
npm run build
cargo test --manifest-path src-tauri/Cargo.toml --locked
```

[Полный процесс разработки](docs/development_workflow.md) · [Критерии приёмки](docs/verification_gates.md) · [CI проверенного runtime main PR #51](https://github.com/uzbatsu-beep/MLAD/actions/runs/45813397360).

## Дорожная карта

Engineering Workbench: восемь специализированных компоновок, темы, Signal Intelligence, Compare, Virtual Laboratory, Trace & Test, Documentation, Robot Fleet и AI. Это **утверждённый план**, не описание выпущенного интерфейса. Реальные скриншоты публикуются после проверки соответствующей сборки.

## Ограничения и обратная связь

`CIOPRG.LST` и `IONAME.DAT` — read-only. HSES/FTP, native writer, live I/O и robot deployment — BLOCKED. Неизвестное не считается ложным или истинным; моделирование не является телеметрией. Не публикуйте производственные выгрузки, реальные хеши и IP.

[Issues](https://github.com/uzbatsu-beep/MLAD/issues) · [SECURITY](SECURITY.md) · [CONTRIBUTING](CONTRIBUTING.md) · [ACCESSIBILITY](ACCESSIBILITY.md) · [LICENSE](LICENSE) · [Лицензирование](docs/licensing.md).

История: rc.9 не содержит PR #24–#29; [предыдущий README](https://github.com/uzbatsu-beep/MLAD/blob/0754dea0d37f5fbc45c795008b60fe69e6c8b08f/README.md). Не использовать исторический срез как текущую инструкцию.
