# PR #4 — Ladder Graph Editor & Visualization

**Status:** interactive **JSON graph-draft editor** + independent **read-only CIOPRG.LST semantic visualization**.
Target families: DX100, DX200, YRC1000. **This is not a robot-compatible editor/exporter yet.**

## 1. Design and safety separation

The project has three distinct surfaces. **They must not be conflated**:

| Surface | Source format | Mutates file? | Native CIOPRG.LST export? | Sends commands to robot? |
|---|---|---|---|---|
| Graph Workspace (new default) | `*.graph.json` (version 1) | Yes, JSON drafts only | **No** | **No** |
| Read-only Inspector | Original `CIOPRG.LST` | **Never** | **No** | **No** |
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
- The frontend `src/ladder/graphModel.ts` contains pure immutable updates, AST conversion and a bounded, deterministic SVG layout. The renderer `src/components/LadderCircuit.tsx` displays electrical-style series and parallel wiring and a coil, without live power-state claims.
- An optional, clearly labelled **offline three-valued condition calculator** tests user-entered 0/1/unknown contacts; it does not simulate controller scan state, motion, robot safety or timers.
- A Graph Workspace stays mounted when switching to the legacy tab, so unsaved changes are not lost by tab navigation.

## 3. Rust validation and draft file handling

Module: `src-tauri/src/compiler/graph_draft.rs`.

Tauri commands:
- `cmd_validate_graph_draft(project)`: classify the configured controller's relays, validate structure and detect duplicate coils and self-feedback.
- `cmd_save_graph_draft(filePath, project)`: save only as `*.graph.json`, make best-effort `.bak` copy when replacing an existing file; **never** write `.LST`. Invalid *addresses* may be retained in drafts with diagnostics; structurally corrupt or oversized drafts are rejected.
- `cmd_load_graph_draft(filePath)`: read only `*.graph.json`; reject oversized documents, malformed JSON, unsupported schema versions and broken structure.

Safety bounds: version 1, up to 128 rungs, 4,096 total AST nodes, depth ≤64, file size ≤2 MiB. The UI layout additionally caps expression rendering to 256 elements and depth 32. **These are application resource limits, not robot/controller limits.**

The legacy save command rejects `CIOPRG.LST` and the separate `*.graph.json` format; it is restricted to normal `*.json` files. Backend regression tests cover format separation even for direct IPC calls.

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
- GitHub Actions executes `cargo test --locked`, `npm run test:graph` and `npm run build` on PR.

## 6. Operator quick-start

1. Launch MLAD and open the **Графический редактор** tab. This is an independent JSON project, not an editable controller program.
2. Select controller family **DX100 / DX200 / YRC1000** and a rung in the left column. Click a contact in the diagram to select it.
3. Use **+ Последовательно (AND)** for series contacts or **+ Параллельно (OR)** for parallel branches. If no contact is selected, these commands wrap the entire current expression.
4. Edit relay addresses, choose NO/NC or change the OUT address. The Rust validator reports malformed, controller-incompatible or duplicate addresses.
5. Use **↶ Отмена / ↷ Повтор** to navigate changes. Optional **Локальная проверка логики** lets you assign synthetic 0/1/? values to contacts; it **never** reads the robot.
6. Save to a file named **`ladder.graph.json`** or any `*.graph.json`; the backend copies the previous draft to `*.graph.json.bak` before overwriting.
7. To inspect a real robot backup, choose **Просмотр CIOPRG.LST** and select the family and source file path. Select a recognized rung and view its **read-only** electrical-style diagram. If the AST is unsupported or truncated, an explicit diagnostic is shown rather than a guessed diagram.
8. **No operation in this workflow creates controller-ready CIOPRG.LST or uploads anything to the robot.**

## 7. Deferred engineering work

- Real examples and golden fixtures exported by DX100, DX200 and YRC1000 to confirm file-level ownership and executable semantics.
- Authoritative instruction-specific semantics of `TMR`, `CNT`, `GSTR`, `GOUT`, `BMOV`, arithmetic and function blocks.
- Native CIOPRG compiler, verified System/User boundaries, approved connection and deployment workflow.
- Correct Yaskawa HSES and live scan-level diagnostics; current graph editor does not attempt monitoring.
- Independent Ladder Editor 32 round-trip and safe laboratory controller tests.

**No industrial controller, system ladder, or robot file was modified by this PR.**
