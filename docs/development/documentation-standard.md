<!-- MLAD-I18N:NAV:START -->
> 🌐 **Язык / Language:** [Русский](documentation-standard.ru.md) · [English](documentation-standard.en.md) · [Original / Исходный текст](documentation-standard.md). **Machine translation / Машинный перевод: editorial review required / необходима редакторская проверка.**
<!-- MLAD-I18N:NAV:END -->

# Стандарт документации и Engineering Excellence

Утверждённое направление 27.11.2026: актуальные документы по назначению, информативный README, постоянный поток качества. Настоящий документ определяет предлагаемые правила реализации; не заявляет выполненность всех задач.

## 1. Информационная архитектура

| Категория | Целевой путь | Назначение |
| --- | --- | --- |
| User Guide | docs/user-guide/ | Установка, сценарии и ограничения |
| Engineering | docs/engineering/ | Форматы, профили, инструкции |
| Architecture | docs/architecture/ | Слои и контракты |
| Development | docs/development/ | Сборка и стандарты |
| Testing | docs/testing/ | CI, ручные протоколы и evidence |
| Decisions | docs/decisions/ | ADR и принятые решения |
| Roadmap | docs/roadmap/ | Стабильные task ID, планы |
| Releases | docs/releases/ | Неизменяемые release notes |
| Audits | docs/audits/ | Датированные отчёты |
| Reference | docs/reference/ | OEM registry и источники |
| Archive | docs/archive/ | Исторические материалы |

Сначала новый каталог [docs/README](../README.md), затем тематические переносы небольшими PR. Существующие пути сохраняются до проверки всех входящих ссылок, companions и тестов; при переносе оставлять понятный redirect/index либо ссылку на immutable commit. Не перемещать опубликованные release notes задним числом. Новые документы размещать в целевой категории.

## 2. Типы документов и актуальность

| Тип | Требование |
| --- | --- |
| README | Назначение, download, реально доступные функции, quick start, roadmap, ограничения, links |
| Live status | Проверенная дата, SHA, evidence, отдельно main/release/planned |
| Specification | Scope, термины, модель, примеры, unknown, источники и тесты |
| ADR | ID, status, context, decision, alternatives, consequences, supersedes |
| Audit | Дата, scope, baseline, evidence, findings, actions, limitations |
| Release note | Tag/SHA, артефакты/хеши, изменения, приёмка, известные ограничения |

Не добавлять новые «актуальные» шапки к устаревшим статусам. Переписывать живой документ, сохранять старый через Git history. История не является текущим регламентом. Примеры синтетические; скриншоты реальной сборки с версией, концепты явно обозначены.

## 3. RU/EN и автоматические проверки

Оригинал авторитетен; RU source-aligned, EN companion с явным статусом. Сохранять команды, ссылки, технические идентификаторы и отрицательные утверждения. Проверять локальные ссылки, heading parity, code fences, review inventory. Новые источники регистрировать в [translation coverage](../translation_coverage.md); редакторский статус не является OEM-верификацией. CI docs-only не означает runtime PASS.

## 4. Engineering Excellence

| ID | Направление | Приёмка |
| --- | --- | --- |
| ENG-01 | Git governance | Required ci-gate, проверенная очистка веток, без удаления release refs |
| ENG-02 | Docs architecture | Актуальный README, каталог, живой статус, миграции с QA |
| ENG-03 | Developer quality | Единый preflight, baseline lint/format, advisories/license audit |
| ENG-04 | Architectural health | Декомпозиция и идентичность профиля/снимка, без второго парсера |
| ENG-05 | UI quality | Поведенческие тесты, доступность, Windows UserSmoke |
| ENG-06 | Performance | Измеряемый baseline 10/50/100 тыс. строк, профиль затрат |
| ENG-07 | Release engineering | Версии, SHA, SBOM, исходный код, отдельное согласование |
| ENG-08 | Continuous audit | Регулярные сверки, evidence и действия |

Каждый PR: scope, тесты, docs, final SHA, ограничения. Еженедельно — Git/CI/backlog, ежемесячно — зависимости/архитектура/docs, перед Preview — UX/performance, перед release — поставка/лицензии. Задачи привязывать к Roadmap V5 и аудиту; функциональную разработку не останавливать. Не вводить автоматическое слияние.

## 5. Границы

`CIOPRG.LST` и `IONAME.DAT` read-only. HSES/FTP, native writer, live I/O, deployment BLOCKED. UserSmoke=NOT RUN; OEMBench=NOT RUN до независимых результатов. Новая документация не расширяет полномочия приложения.
