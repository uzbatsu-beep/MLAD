<!-- MLAD-I18N:NAV:START -->
> 🌐 **Язык / Language:** [Русский](references.ru.md) · [English](references.en.md) · [Original / Исходный текст](references.md). **Machine translation / Машинный перевод: editorial review required / необходима редакторская проверка.**
<!-- MLAD-I18N:NAV:END -->

> **Актуальная достоверность семантики (21.11.2026):** `main` `9fcd11d61856c8a559d9acf3c77bd662ad814dfe`, Windows pre-release `v0.5.0-rc.10-unified-offline`. Текущий состав парсера и роли операндов описывает [аудит Git](repository_audit_2026-11-21.md). Источники **не** подтверждают реальный scan/flags или права записи: `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`; native writer, `HSES`/`FTP` и deployment — `BLOCKED`.

# References — архив предварительного списка источников

> **Архивный указатель (сверка 18.11.2026):** этот файл сохраняется для ссылок; не использовать его как действующую спецификацию. PR #1–#21 слиты, последний Windows релиз — rc.9. [Текущий статус](project_status.md).

The previously maintained table included a number of unverified assumptions, including incorrect read-only/write classes for several CIO relay banks. It is retained as a stable navigation path.

**Authoritative project index:** [source_registry.md](source_registry.md)

**Verified controller-specific ranges:** [typed_memory_model.md](typed_memory_model.md)

**File syntax and safe round-trip:** [cioprg_lst_spec.md](cioprg_lst_spec.md)

Do not derive writing privileges or complete file headers from an isolated manual example. Controller-specific manufacturer manuals, application options and genuine exported test files must be verified before enabling compile/deploy.


**Обновлено 17.11.2026:** основной [source_registry.md](source_registry.md) различает документированные инструкции, реализованные тесты, реальные файлы и измерения на стенде. Пока аппаратных результатов и оригинальных golden fixtures в публичном репозитории нет. Для новых внешних ссылок указывать точную ревизию руководства и ограничения доступа.

Путь к актуальной документации: [docs/README.md](README.md); требования к получению источников: [controller_fixture_intake.md](controller_fixture_intake.md).
