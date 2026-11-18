<!-- Generated en companion for docs/semantic_ast.md; review required. -->
> **English** · [Русский](semantic_ast.ru.md) · [English](semantic_ast.en.md) · [Original / Оригинал](semantic_ast.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.

# Semantic AST & Ladder Logic Analyzer — PR #3updating after PR #21

> ** Status and applicability:** This document describes the relevant functional subset and evidence collected before/by PR #21It means that the whole project stopped at PR #21. Actual `main` later [PR #29](https://github.com/uzbatsu-beep/MLAD/pull/29) and independent levels of CI/UserSmoke/OEM: [project_status.md](project_status.en.md). Do not give permission to a native writer, HSES or perform a scan from the specification described herein.

**Scope (18.11.2026):** DX100, DX200, YRC1000 Concurrent I/O, **offline read-only** analysis of documented boolean mnemonics. This is **not** a controller emulator, full CIO compiler, hardware authorization tool, or proof of a valid editable User Ladder program.

## 1. Sources and evidence

- Yaskawa DX200 Concurrent I/O `RE-CKI-A465`, §12.2.1–12.2.11 (STR, STR-NOT, AND/OR variants, AND-STR/OR-STR, OUT, PART, END). [DX200 manual](https://www.scribd.com/document/627793162/165294-1CD-DX200-OPTIONS-CONCURRENT-IO)
- Yaskawa YRC1000 Concurrent I/O `RE-CKI-A467`, §12.2. [YRC1000 Concurrent I/O](https://www.scribd.com/document/573345245/Concurrent).
- Yaskawa [Concurrent I/O Overview](https://knowledge.motoman.com/hc/en-us/articles/1500012236161-Concurrent-I-O-Overview), including duplicate output restrictions and possible BMOV/GOUT overlap.
- Yaskawa [Mapping of Specific I/O](https://knowledge.motoman.com/hc/en-us/articles/13086375717655-Mapping-of-Specific-I-O), giving actual user/system ladder examples.
- DX100 Concurrent I/O `RE-CKI-A453`, [manufacturer PDF at educational mirror](https://gab.wallawalla.edu/~ralph.stirling/classes/engr480/docs/Motoman/dx100_concurrent_io_155491-1CD.0.pdf).

## 2. Architecture

```text
LosslessDocument (PR #2: immutable bytes, original SHA-256, lines, lexical AST)
  -> semantic::analyze(document)
     -> indexed ExprNode array (flat, non-recursive)
        Contact {relay, inverted} | And {left, right} | Or {left, right}
     -> SemanticRung {output, root, first_line, output_line, inputs, section}
     -> Cross-references (#relays and M registers with source byte spans)
     -> Diagnostics + coverage flags
     -> Bounded semantic preview for read-only CIOPRG inspector
```

The original file is never changed or re-serialized. All expression references are **integer indexes into a flat node vector**; analysis, evaluation, feedback-graph detection and serialization avoid unbounded recursive AST traversal.

## 3. Verified boolean subset and stack semantics

- `STR #A`: start a contact expression; if a previous expression is in progress, push that intermediate result on the expression stack.
- `STR-NOT #A`: same, with inverted contact.
- `AND #B`, `AND-NOT #B`: boolean conjunction with the active expression.
- `OR #B`, `OR-NOT #B`: boolean disjunction with the active expression.
- `AND-STR`, `OR-STR`: pop the previous intermediate expression and join it with the current expression using AND/OR. Missing expressions are diagnosed as `UnbalancedBranch`.
- `OUT #C`: emit a rung only if there is an active and **fully balanced** expression. The output is subject to the controller-specific memory model; duplicate writes are flagged across both `PART` sections.
- `PART 1` / `PART 2`: annotate the claimed System/User section; `END`: end the ladder program. **These markers do not prove safe edit ownership, compatible file grammar or correct application/firmware.**

**Manufacturer example — AND-STR:**

```text
STR #70010
OR #70011
STR #70020
OR #70021
AND-STR
OUT #70100
```

AST: `(#70010 OR #70011) AND (#70020 OR #70021)`.

**Manufacturer example — OR-STR:**

```text
STR #70010
AND #70011
STR #70020
AND #70021
OR-STR
OUT #70100
```

AST: `(#70010 AND #70011) OR (#70020 AND #70021)`.

These are **synthetic/illustrative tests reproducing documented manufacturer instruction examples**, not actual exported controller files.

### Intentional analysis barriers

`TMR`, `CNT`, `GSTR`, `GOUT`, and unrecognized instructions **can have state, timing, group or side effects** not represented by a simple boolean expression. The analyzer collects limited register/group references but **discards any in-progress boolean expression** and adds a diagnostic when encountering these instructions. Subsequent independent `STR…OUT` chains can still be analyzed. A rung immediately following a stateful operation is **not** synthesized from the preceding contact condition.

The same applies to unrecognized source directives inside an open expression, invalid operands and unsupported/opaque source lines. Unsupported parts are retained by the PR #2 lossless parser.

## 4. Diagnostics and static checks

| Code | Severity | Interpretation |
|---|---|---|
| MissingStart | Error | AND/OR without active STR |
| UnbalancedBranch | Error | AND-STR / OR-STR without saved expression |
| UnbalancedOutput | Error | OUT without complete stack-balanced expression |
| UnfinishedAtEnd / UnfinishedExpression | Error | END/EOF during an incomplete rung |
| DoubleCoil | Error | Same bit relay has >1 OUT across recognized code, including PART 1/2 |
| ReadOnlyCoil | Error | Output operand category is not allowed by conservative memory model |
| WriteNeedsReview | Warning | SpecificInput or other application-dependent write |
| SelfFeedback | Warning | A rung reads the relay it writes; might be an intentional latch |
| FeedbackCycle | Warning | Cross-rung dependency cycle; might be intentional feedback |
| PossibleGroupOverlap | Warning | GOUT byte may overlap a separately recognized OUT bit |
| StatefulOpcode / GroupedOpcode / UnverifiedSource | Warning | Unknown semantic effect; partial subset result |
| PartInsideExpression / PartOrder | Error/warning | Section marker inconsistent with expression or expected order |
| AfterEnd | Error | Instruction after END |
| NodeLimit / BranchDepthLimit | Warning | Analyzer resource guard exceeded; **not** an asserted Yaskawa hardware limit |

Partial cross references do **not** prove there is no conflicting writer: GOUT/BMOV and unsupported instructions can also affect output bits.

## 5. Reference model

- `ContactRead` and `CoilWrite` are accepted only for recognized bit instruction operands.
- `RegisterCurrentValue` and `RegisterSetpoint` are descriptive for TMR/CNT; runtime effects are not modeled.
- `PotentialGroupRead` and `PotentialGroupWrite` cover recognized GSTR/GOUT. The word “Potential” is intentional.
- All sites retain source line and operand byte span. Every section label is explicitly a **claim** based on PART tokens, not a trust boundary.
- Each rung also provides a bounded, human-readable expression preview; the indexed expression graph remains authoritative.
- The dependency graph links valid rungs by matching output relays to input contacts. Strongly connected components are found by iterative DFS (no recursive call stack); the resulting feedback warning is informative and not a formal deadlock proof.

## 6. Offline evaluation (not hardware simulation)

`SemanticReport::evaluate_rung(i, signal_state_map)` evaluates the indexed expression from known input bits and returns:
- `Some(true)` / `Some(false)` when determinable (including short-circuit outcomes);
- `None` when inputs are insufficient.

This evaluates **the condition alone**, not the robot's scan order, output ownership, I/O latency, SET/RST semantics, timers, counters or execution permission. The function has no network access or write effects.

## 7. Safety gates and limitations

1. The original `LosslessDocument` raw bytes and SHA-256 remain immutable; no CIOPRG writer is added.
2. `SemanticReport.edited_export_allowed` and `system_user_boundaries_verified` are always `false`.
3. A green `subset_analysis_complete` flag means only that the **implemented subset** was analyzable without an error. It **never** grants export or deployment.
4. GOUT/BMOV, M-register writes, execution order, static analysis of entire System/User code, and full controller-specific instruction semantics remain incomplete.
5. Resource caps (`MAX_BOOLEAN_NODES`, `MAX_OPEN_BRANCHES`) are for analyzer stability, not verified controller limits.
6. No real DX100/DX200/YRC1000 fixtures were imported; this PR does not claim identical behavior to genuine Ladder Editor 32. Golden fixtures, hardware compatibility and a complete authorized file format are required for future compiler work.

## 8. Tests and acceptance

The new `src-tauri/src/compiler/semantic/tests.rs` includes representative examples for series/parallel expressions, negated contacts, nested stacks, missing STR, unbalanced branches, double coils across PART boundaries, invalid writes, self/mutual feedback, unknown-opcode barriers, grouped output overlaps, cross-reference spans, three-valued evaluation, resource guards and immutable original bytes.

PR #3 already drained `main`. With further changes in the analyzer, Rust regression tests, frontend checks and separate approval of the new PR are required. Old text on merger authorization PR #3 It's no longer relevant.


## 9. Evolution after PR #3

- **PR #5** added a separate `advanced_source.rs`, which shows limited typical TMR/CNT/GSTR/GOUT sequences. **Boolean AST is not a full-fledged interpreter of stateful instructions.**
- **PR #7** added offline lighting of all chains manually `TRUE/FALSE/UNKNOWN` It's not a telemetry or a real scan calculation.
- **PR #8** Added address editing to **JSON diagrams**, but not in the original `CIOPRG.LST`.
- Continuation requires [original](controller_fixture_intake.en.md)An independent OEM editor and more complete semantics. See. [architecture](architecture.en.md).
