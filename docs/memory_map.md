<!-- MLAD-I18N:NAV:START -->
> 🌐 **Язык / Language:** [Русский](memory_map.ru.md) · [English](memory_map.en.md) · [Original / Исходный текст](memory_map.md). **Machine translation / Машинный перевод: editorial review required / необходима редакторская проверка.**
<!-- MLAD-I18N:NAV:END -->

> **Актуальный продуктовый контекст (21.11.2026):** `main` `9fcd11d61856c8a559d9acf3c77bd662ad814dfe`, Windows pre-release `v0.5.0-rc.10-unified-offline`. Текущую границу функций фиксирует [аудит Git](repository_audit_2026-11-21.md). Оригинальные `CIOPRG.LST` и `IONAME.DAT` read-only, offline-метаданные отдельны; `UserSmoke=NOT RUN`, `OEMBench=NOT RUN`, native writer и `HSES`/`FTP` — `BLOCKED`.

# Memory map — архив предварительной карты адресов

> **Архивный указатель (сверка 18.11.2026):** этот файл сохраняется для ссылок; не использовать его как действующую спецификацию. PR #1–#21 слиты, последний Windows релиз — rc.9. [Текущий статус](project_status.md).

**Do not use earlier versions of this file as a source of address validity or write permission.**

The original MLAD prototype had shifted CIO classes (for example, `1xxxx` was labelled External Input and `2xxxx` External Output). For the verified controller-specific tables, use **[Typed memory model — DX100, DX200, YRC1000](typed_memory_model.md)**.

The implementation of these ranges, typed bit/byte/word/register addresses and tests lives at `src-tauri/src/compiler/memory.rs`.

## Key rules

- Set an explicit `ControllerFamily` for production validation: `DX100`, `DX200`, `YRC1000`.
- Relay bit address notation: `#XXXXX` with final digit `0..7`; channel `number/10`.
- Network I/O and pseudo input ranges differ materially between generations.
- The address-class table does **not** grant permission to write to an actual robot.
- M-registers are 16-bit `M000..M999`; role, ownership and instruction restrictions apply.
- Read the [CIOPRG.LST specification baseline](cioprg_lst_spec.md) before attempting a compiler.

Source index: [source registry](source_registry.md).

## Статус UI после PR #8

В новых вкладках MLAD оператор выбирает `DX100`, `DX200` или `YRC1000`; Rust использует профиль в `memory.rs` и отдельных валидаторах JSON / read-only Inspector. Фронтенд inline-ввода проверяет только **формат**, после чего Rust проверяет типы и диапазоны. Legacy JSON workflow сохраняет совместимые ограничения.

Никакая успешная проверка диапазона не означает разрешённую аппаратную запись. См. [typed_memory_model.md](typed_memory_model.md), [architecture.md](architecture.md) и [документацию](README.md).
