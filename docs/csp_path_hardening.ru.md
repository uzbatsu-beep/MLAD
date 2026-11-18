<!-- Generated ru companion for docs/csp_path_hardening.md; review required. -->
> **Русский** · [Русский](csp_path_hardening.ru.md) · [English](csp_path_hardening.en.md) · [Original / Оригинал](csp_path_hardening.md)
> **Translation review / Проверка:** автоматический черновик — сверяйте инженерные утверждения с оригиналом и OEM документацией.
> **Контрольные ограничения MLAD:** `CIOPRG.LST` и `IONAME.DAT` — строго **только чтение**. FTP/HSES, live I/O, native writer и загрузка в контроллер — **BLOCKED (запрещены)**. `WindowsCI` не заменяет `UserSmoke=NOT RUN` и `OEMBench=NOT RUN`; машинный перевод должен быть сверен с исходным документом и документацией Yaskawa.

# MLAD — CSP и проверки безопасности файловых путей (PR #25)

**Контекст:** проект OFFLINE/BENCH. Дата: 18.11.2026. **Статус:** PR #25 merged в main 18.11.2026; Windows CI SUCCESS, но опубликованный rc.9 не содержит CSP.

## Первичные основания

- [Таури 2 CSP](https://v2.tauri.app/security/csp/) — Tauri при сборке добавляет hash/nonce источники для локальных ресурсов, если CSP явно задана; при `csp:null` эта защита отключена.
- [Конфигурация SecurityConfig](https://v2.tauri.app/reference/config/#securityconfig) - производство `csp` и отдельный `devCsp`, по умолчанию dev наследует production.
- [Оригинальное название: Tauri Vite Guide](https://v2.tauri.app/start/frontend/vite/) — локальный Vite сервер и HMR через WebSocket.

## 1. Реализованная production CSP

В `src-tauri/tauri.conf.json` строго определены `default-src` и `script-src: 'self'`, без `'unsafe-inline'`/`'unsafe-eval'` для JavaScript. `connect-src` допускает только приложение и Tauri IPC (`ipc: http://ipc.localhost`). `object-src`, `frame-src`, `worker-src`, `base-uri`, `form-action` блокируются. `img-src 'self' data:` и `font-src 'self'` не разрешают CDN и внешние изображения.

**Осознанное исключение:** `style-src 'self' 'unsafe-inline'` необходимо существующим inline-style React и стартовому HTML. Это **не** разрешение inline JavaScript и **не** идеальный CSP; следующий UX/security этап может перевести стили в классы и убрать это исключение.

## 2. КСП развития

Отдельный `devCsp` добавляет локальный `http://localhost:5173` и `ws://localhost:5173` только для Vite/HMR; remote HTTP/WSS отсутствуют. Production policy не включает localhost:5173, не допускает создание произвольных сетевых запросов и сохраняет локальный Tauri IPC.

## 3. Файловые пути: в чём усиление

Функция `draft_io::verify_parent_directory(path)` до файловых операций:
- Проверяет синтаксические `..` и отвергает выход за явно указанный путь.
- Проверяет каждый **существующий** компонент каталога через `symlink_metadata`, отвергая symlink и отсутствие обычного каталога.
- Встраивается в JSON `read_bounded/validate_file`, стадирование файлов, final rename, а также read-only CIOPRG и IONAME inspect и Open chooser.
- CIOPRG дополнительно отклоняет **прямой symlink**; ранее `metadata` могла следовать за ним. В исходных файлах робототехники запись не появляется.

Синтетические тесты: вложенные обычные каталоги, `../`, linked parent, linked native file, SHA/byte identity до и после отказа, запрет direct JSON symlink, старые JSON backup/regression tests.

## 4. Что НЕ подтверждено и не гарантировано

1. **TOCTOU всё ещё возможна:** последовательность `symlink_metadata → open/rename` не атомарна. Злоумышленник с доступом к каталогам может подменить компонент после проверки. Надёжное решение требует ОС-специфичных directory handle, `openat2/openat + O_NOFOLLOW` (POSIX), Windows File ID/reparse policy и ACL. Это отдельный PR.
2. Windows junction/reparse points не гарантированно классифицируются как symlink всеми APIs; тесты Windows без повышенных прав не покрывают все варианты.
3. Hardlinks, concurrent writers, атомарная пара backup+target, файловые ACL, crash durability и защита локального профиля **не** решены этим PR.
4. CSP не препятствует злоупотреблениям через разрешённые команды Tauri IPC: нужны отдельные permission reviews и runtime-проверка, включая реальный Windows/WebView2 и все панели.
5. Не заявлять, что \`npm run build\` (обычный Vite) внедряет CSP в web preview: приложение должно быть собрано как Tauri, а CSP нужно проверять в **реальном Tauri WebView**.
6. Некоторые нормальные пользовательские пути с символической ссылкой или `..` теперь намеренно отвергаются; для просмотра используйте прямую копию в обычном каталоге.

## 5. Критерии приёмки

| Проверка | Автоматическая/ручная | Статус |
| --- | --- | --- |
| Node tests на CSP, отсутствие remote/unsafe eval | Автоматическая | PASS - PR #25/#26 CI |
| Rust проверка обычных/небезопасных путей | Автоматическая Linux + Windows MSVC | PASS - основной /PR #26 CI |
| Windows Tauri с компилированным CSP | GitHub Windows workflow, не публикует релиз | [PASS - PR #26 Windows CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44227145031) |
| В WebView2 открываются Native Workspace/IONAME/Ladder/Advanced, работает IPC и zoom | Ручной Windows smoke | NOT RUN (не проводилось) |
| Отсутствие изменения CIOPRG/IONAME на байтовом уровне | Регрессия ржавчины | PASS — автоматические тесты; независимый UserSmoke NOT RUN |
| File-handle/no-follow, symlink race testing, policy для Windows junction | Отдельная задача | OPEN |

**Решение:** PR #25 устраняет `csp:null` и снижает класс рисков symlink-ancestor, но **M2 остаётся PARTIAL**, а robot writer и HSES/FTP запрещены.

**Примечание к истории:** первая реализация — PR #25, в PR #26 добавлен guarded read и Windows reparse check. Это **не** снимает открытые file races и manual WebView2 validation. [Фактический статус](project_status.ru.md) · [единые gates](verification_gates.ru.md).

## Текущее дополнение после PR #28–#29

В `main` после [PR #29](https://github.com/uzbatsu-beep/MLAD/pull/29) Тесты восстановления / соединения Windows, отказ от жесткой ссылки, поэтапный файл SHA-256, File ID snapshot and Windows ADS/UNC/device deny реализованы и [Windows CI PASS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621). Этот раздел **не переписывает исторический объём PR #25**: final check→rename, parent directory races, ACL, crash durability и ручной WebView2 CSP/IPC acceptance всё ещё открыты. [Обновлённый threat review](filesystem_security_pr29.ru.md).
