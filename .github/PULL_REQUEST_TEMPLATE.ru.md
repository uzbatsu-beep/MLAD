<!-- Generated ru companion for .github/PULL_REQUEST_TEMPLATE.md; review required. -->
> **Русский** · [Русский](PULL_REQUEST_TEMPLATE.ru.md) · [English](PULL_REQUEST_TEMPLATE.en.md) · [Original / Оригинал](PULL_REQUEST_TEMPLATE.md)
> **Translation review / Проверка:** автоматический черновик — сверяйте инженерные утверждения с оригиналом и OEM документацией.

## Что изменено

<!-- Одна техническая цель PR; укажите ссылки на задачи и документы. -->
- Связанный Issue / этап roadmap:
- Базовый commit main (SHA):
- Изменения и затронутые подсистемы:
- Семейства контроллеров (DX100/DX200/YRC1000 или «не применимо»):

## Доказательность и неизвестные области

- Источник правила (руководство, ревизия, раздел/страница) **или** «нет OEM-источника / не применимо»:
- Что подтверждено: **Documented / SyntheticTested / ManualDerived / OEM BenchVerified** (не смешивать уровни):
- Что **не подтверждено** (EN, scan, flags, System/User, byte/word, firmware и т. д.):
- Почему новые ограничения не маскируются как поддерживаемая функциональность:

## Проверки

- [ ] `npm ci --no-audit --no-fund`, lockfile/manifest не изменены случайно.
- [ ] `npm run test:security`.
- [ ] `npm run test:native` и при необходимости `test:graph` / `test:advanced` / `test:diagnostics` / `test:oem-reference`.
- [ ] `npm run docs:check` и документация/roadmap актуализированы.
- [ ] `npm run build`; `cargo test --manifest-path src-tauri/Cargo.toml --locked` для изменений Rust.
- [ ] Регрессионные положительные и отрицательные тесты добавлены или объяснено, почему не требуются.
- [ ] Windows smoke / OEM bench выполнены **или явно помечены NOT RUN** с причиной.

> Для документационного PR можно указать **N/A с объяснением** рядом с неприменимыми пунктами, но общие CI проверки остаются обязательными.

## Границы безопасности MLAD

- [ ] Оригинальные `CIOPRG.LST` и `IONAME.DAT` остаются **только для чтения**.
- [ ] Нет native writer/export, FTP/HSES, live I/O, загрузки/управления роботом.
- [ ] Нет ложного перевода `TypedCandidate` или `ManualDerived` в реально исполняемую семантику.
- [ ] В PR нет реальных программ, закрытых карт I/O, адресов производственных сетей, паролей и конфиденциальных журналов.
- [ ] Автор вправе предоставить предложенный код и материалы на условиях `GPL-3.0-only` (третьи лица сохраняют свои права).
- [ ] Уязвимости обсуждаются по [SECURITY.md](https://github.com/uzbatsu-beep/MLAD/blob/main/SECURITY.md), **не** через открытые детали в PR.

## План приёмки

- Результаты CI (ссылки):
- Ручные проверки (что проверено; что не проверено):
- Известные риски и способ отката:

**Слияние только после отдельного явного согласования владельца проекта.**

## Актуальность документации и границы решения

- [ ] Сверены фактический `main` SHA, открытые PR, релизы и статусы CI с [статус проекта](../docs/project_status.ru.md).
- [ ] При изменении функции обновлены [дорожная карта v3](../docs/roadmap.ru.md), [План осуществления](../docs/implementation_plan.ru.md), [Проверочные ворота](../docs/verification_gates.ru.md) и профильные MD; **не** переписаны исторические release notes задним числом.
- [ ] Если затронуты file paths/JSON/Tauri, проведён review [PR #29 Файловые границы](../docs/filesystem_security_pr29.ru.md) (File ID/hardlink/ADS ≠ закрытие TOCTOU/ACL/backup transaction).
- [ ] Уровни `SyntheticTested`, `WindowsCI`, `UserSmoke`, `OEMBenchVerified`, `Authorized` зафиксированы независимо, отсутствующие тесты честно отмечены `NOT RUN`.
- [ ] Отдельное явное согласие владельца на merge получено (не считать CI разрешением).
