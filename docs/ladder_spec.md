<!-- MLAD-I18N:NAV:START -->
> 🌐 **Язык / Language:** [Русский](ladder_spec.ru.md) · [English](ladder_spec.en.md) · [Original / Исходный текст](ladder_spec.md). **Machine translation / Машинный перевод: editorial review required / необходима редакторская проверка.**
<!-- MLAD-I18N:NAV:END -->

# Ladder / CIOPRG.LST — архив предварительной спецификации

> **Исторический документ (навигационная отметка 18.11.2026):** нижеследующие выводы сохраняют дату первоначальной проверки, а не подменяют текущую реализацию. PR #1–#21 merged, Windows rc.9 опубликован. [Текущее состояние MLAD](project_status.md).

The original prototype contained a fabricated `/JOB`-style output template and incomplete instruction examples. These are **not validated as the native Yaskawa Concurrent I/O file** and **must not be exported or deployed as CIOPRG.LST**.

Current source of truth: **[CIOPRG.LST specification baseline](cioprg_lst_spec.md)**.

Current requirements include lossless raw-file preservation, typed operands, correct `#XXXXX` and `W#XXXX0` syntax, controller-specific semantic validation, system/user boundary protection and fail-closed modified export.

The baseline documents what manufacturer sources establish and marks unknown file-level grammar explicitly. A complete verified grammar still requires untouched real-controller exports for each generation.

See also [typed memory model](typed_memory_model.md) and [source registry](source_registry.md).

## Состояние на 17.11.2026

С момента создания этого файла в `main` объединены read-only `lossless.rs`, `semantic.rs`, ограниченная native-инвентаризация TMR/CNT/GSTR/GOUT и два независимых JSON-редактора. Это не превращает прежний шаблон `/JOB` в правильную грамматику `CIOPRG.LST`. Для ввода адресов на графе используется [inline-редактирование](inline_address_editing.md) **только в JSON**.

Источники и статус: [полная карта документации](README.md), [базовая спецификация](cioprg_lst_spec.md) и [архитектура](architecture.md).
