<!-- MLAD-I18N:NAV:START -->
> 🌐 **Язык / Language:** [Русский](graph_editor.ru.md) · [English](graph_editor.en.md) · [Original / Исходный текст](graph_editor.md). **Machine translation / Машинный перевод: editorial review required / необходима редакторская проверка.**
<!-- MLAD-I18N:NAV:END -->

# Ladder Graph Editor & Visualization — PR #4 + интерфейсные изменения PR #7/#8

**Status:** interactive **JSON graph-draft editor** + independent **read-only CIOPRG.LST semantic visualization**.
Target families: DX100, DX200, YRC1000. **This is not a robot-compatible editor/exporter yet.**

## 1. Design and safety separation

The project has four distinct surfaces. **They must not be conflated**:

| Surface | Source format | Mutates file? | Native CIOPRG.LST export? | Sends commands to robot? |
|---|---|---|---|---|
| Graph Workspace (отдельная вкладка JSON draft; не default) | `*.graph.json` (version 1) | Yes, JSON drafts only | **No** | **No** |
| Advanced Workspace | `*.advanced.json` (version 1) | Yes, JSON drafts only | **No** | **No** |
| Native Workspace (основная вкладка, read-only) | Original `CIOPRG.LST` | **Never** | **No** | **No** |
| Legacy JSON interface | `project.json` (`Vec<Rung>`) | Yes, legacy JSON only | **No** | **No** |

The unverified HSES monitor is disabled **in both the visible interface and the backend Tauri command**, because a single relay being true cannot establish the powered state of a complete rung; proper scan semantics and HSES protocol validation remain a separate workstream.

Neither `PART 1` / `PART 2` markers nor a successful semantic graph imply that *System Ladder* is safely editable. No existing CIOPRG import data is ever copied into the editable workspace.

## 2. Editable model: `GraphDraft` v1

```json
{
  "version": 1,
  "controller": "YRC1000",
  "rungs": [{
    "id": "example-rung",
    "output": "#70020",
    "comment": "Demonstration only",
    "expression": {
      "kind": "or",
      "id": "join-1",
      "left": {"kind":"contact","id":"a","relay":"#20010","normallyClosed":false},
      "right":{"kind":"contact","id":"b","relay":"#70010","normallyClosed":true}
    }
  }]
}
```

- `kind=and` means series contacts; `kind=or` means parallel branches. A contact is NO or NC.
- Each expression is a *binary tree*, not a raw flat `Vec<Instruction>`.
- Stable unique IDs support selecting a contact and editing the expression without accidental changes elsewhere.
- Controls: add rung, remove rung (confirmation), select contact, insert AND/OR branch, toggle NC, edit address/coil/comment, delete contact (promote sibling), undo and redo. Zoom and scroll support large diagrams.
- The frontend `src/ladder/graphModel.ts` contains pure immutable updates, AST conversion and a bounded, deterministic SVG layout. The renderer `src/components/LadderCircuit.tsx` displays electrical-style series and parallel wiring and a coil. Since PR #7 it shows all rungs together and colors contacts/coils by explicit `TRUE/FALSE/UNKNOWN` offline test values, **without live power-state claims**. Since PR #8 editable JSON diagrams support **double-click/F2** on contact/OUT addresses, Enter to save and Esc to cancel. The original `CIOPRG.LST` Inspector never receives edit callbacks.
- An optional, clearly labelled **offline three-valued condition calculator** tests user-entered 0/1/unknown contacts; it does not simulate controller scan state, motion, robot safety or timers.
- A Graph Workspace stays mounted when switching to the legacy tab, so unsaved changes are not lost by tab navigation.

## 3. Rust validation and draft file handling

Module: `src-tauri/src/compiler/graph_draft.rs`.

Tauri commands:
- `cmd_validate_graph_draft(project)`: classify the configured controller's relays, validate structure and detect duplicate coils and self-feedback.
- `cmd_save_graph_draft(filePath, project)`: save only as `*.graph.json`, make best-effort `.bak` copy when replacing an existing file; **never** write `.LST`. Invalid *addresses* may be retained in drafts with diagnostics; structurally corrupt or oversized drafts are rejected.
- `cmd_load_graph_draft(filePath)`: read only `*.graph.json`; reject oversized documents, malformed JSON, unsupported schema versions and broken structure.

Safety bounds: version 1, up to 128 rungs, 4,096 total AST nodes, depth ≤64, file size ≤2 MiB. The UI layout additionally caps expression rendering to 256 elements and depth 32. **These are application resource limits, not robot/controller limits.**

The legacy save command rejects `CIOPRG.LST` and the separate `*.graph.json` format; it is restricted to normal `*.json` files. Backend regression tests cover format separation even for direct IPC calls. Existing symlink targets are rejected before JSON writes (defense-in-depth; OS filesystem access controls remain necessary).

The address validator distinguishes `DX100`, `DX200`, `YRC1000` according to `docs/typed_memory_model.md`. It warns on self-feedback or `SpecificInput` coils and reports errors for invalid/duplicate coil destinations.

Every backend response contains `edited_cioprg_export_allowed: false`. A successful JSON draft validation is **not** a safety or execution authorization.

## 4. Read-only semantic visualization

PR #3's `SemanticPreview` now also returns the first **4,096 indexed expression nodes** and a truncation flag. The app's `CioprgInspector` converts recognized AST contact/AND/OR nodes (up to 256 per displayed rung) into the *same* electrical circuit renderer.

- The user chooses a recognized rung; contacts and coil are drawn in order, including true parallel wiring.
- If the semantic graph is missing nodes, too deeply nested, unsupported or malformed, the UI shows an explicit refusal, **never an invented or partial circuit**.
- Syntax diagnostics, cross-references, source SHA-256 and original text remain visible in the Inspector.
- No graph element in a native CIOPRG inspection has edit affordances; no modified file is exported.

## 5. Verification

- Rust graph draft model tests: JSON round-trip, controller-specific classes, duplicate coils, stable IDs, invalid addresses, structural/node/depth limits, self-feedback.
- TypeScript graph-model tests: AND/OR circuit geometry, NC inversion, immutable insert/delete, nested branching, semantic indexed-node conversion, invalid AST and layout guardrails.
- Existing lossless-parser byte-for-byte and semantic-logic tests remain unchanged.
- Current `MLAD validation` on PR/main runs `cargo test --locked`, `npm run test:graph`, `npm run test:advanced`, `npm run test:visual`, `npm run test:inline` and `npm run build`. This does **not** execute a manual Windows GUI test.

## 6. Operator quick-start

1. Launch MLAD and open the **Графический редактор** tab. This is an independent JSON project, not an editable controller program.
2. Select controller family **DX100 / DX200 / YRC1000** and a rung in the left column. Click a contact in the diagram to select it.
3. Use **+ Последовательно (AND)** for series contacts or **+ Параллельно (OR)** for parallel branches. If no contact is selected, these commands wrap the entire current expression.
4. Edit relay addresses and OUT either in property fields or by **double-click/F2** on a label inside the editable SVG; Enter saves, Esc cancels. The Rust validator checks selected controller ranges and diagnoses invalid or duplicate addresses. For complex advanced input conditions, use the expression editor instead of replacing them with one contact.
5. Use **↶ Отмена / ↷ Повтор** to navigate graph changes. Above the **combined view of all rungs**, set manual `? / 0 / 1` inputs to see gray/red/green contacts and coils; it **never** reads the robot. Advanced blocks have their own independent local state tests.
6. Save to a file named **`ladder.graph.json`** or any `*.graph.json`; the backend copies the previous draft to `*.graph.json.bak` before overwriting.
7. To inspect a real robot backup, choose **Просмотр CIOPRG.LST** and select the family and source file path. Select a recognized rung and view its **read-only** electrical-style diagram. If the AST is unsupported or truncated, an explicit diagnostic is shown rather than a guessed diagram.
8. **No operation in this workflow creates controller-ready CIOPRG.LST or uploads anything to the robot.**

## 7. Deferred engineering work

- Real examples and golden fixtures exported by DX100, DX200 and YRC1000 to confirm file-level ownership and executable semantics.
- A typed offline **subset** for `TMR`, `CNT`, `GSTR` and `GOUT` is implemented in PR #5, but native scan-time equivalence, remaining `BMOV`, arithmetic and functional semantics are still unverified.
- Native CIOPRG compiler, verified System/User boundaries, approved connection and deployment workflow.
- Correct Yaskawa HSES and live scan-level diagnostics; current graph editor does not attempt monitoring.
- Independent Ladder Editor 32 round-trip and safe laboratory controller tests.

**No industrial controller, system ladder, or robot file was modified by this PR.**


## 8. Дополнительные UI-контракты

- Общая компоновка и значения подсветки: [ui_multi_rung_state.md](ui_multi_rung_state.md).
- Inline-редактирование, ограничения F2/Enter/Esc и read-only Inspector: [inline_address_editing.md](inline_address_editing.md).
- Схемы TMR, CNT с раздельными C/R и групповой передачи: [advanced_blocks.md](advanced_blocks.md).
- У JSON-черновиков нет подтверждённой трансляции в native файл робота. Результаты `cmd_validate_graph_draft` не дают права на экспорт.

## 9. JSON persistence после PR #29

Сохранение Graph/Advanced/legacy JSON теперь использует staged-file, backup, конфликтные проверки **байтов + File ID**, отказ для hardlinks и проверку SHA-256 временного файла; Windows ADS/UNC/device paths запрещены. Это применяется к backend writer, не делает графический JSON файлом контроллера. [Filesystem threat review](filesystem_security_pr29.md): last check→rename, ACL, atomic backup+target и power-loss остаются **OPEN**. Отдельный план portable sidecar — [PR #33 (предложение)](implementation_plan.md), не реализован.
