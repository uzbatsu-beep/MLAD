<!-- Generated en companion for docs/advanced_hardware_test_matrix.md; review required. -->
> **English** · [Русский](advanced_hardware_test_matrix.ru.md) · [English](advanced_hardware_test_matrix.en.md) · [Original / Оригинал](advanced_hardware_test_matrix.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.

# MLAD — Hardware Bench Validation Matrix (DX100 / DX200 / YRC1000)

Applies separately to **DX100**, **DX200**, and **YRC1000**. Each result must record exact model/firmware/application, Ladder Editor revision, `CIOPRG.LST` backup SHA-256, electrical I/O mapping, scan timing and the engineer who authorized the test.

**Safety / setup gate (update 18.11.2026, after PR #21):** No MLAD CIOPRG export, deployment or live I/O control exists in the current `main`. Use manufacturer-supported editing/backup, an isolated nonhazardous test rig, independent functional safety, physical motion inhibition and the site's test procedure. Never substitute this matrix for an OEM commissioning checklist or use production safety-related signals as test points.

## Reference instructions under test

```text
; documentation examples ONLY, not a complete CIOPRG.LST
; TMR: one start condition, current M010, 100 x 100 ms = 10 s
STR #70010
TMR M010,100
OUT #70100

; CNT: two independent conditions, C then R, current M020
STR #70010
STR #70011
CNT M020,3
OUT #70110

; GRP: a pair of byte operations
GSTR #20010
GOUT #00010
```

Comments and snippets above are explanatory, **not** a validated controller-native file header/section layout. Only the OEM Ladder Editor can be used to prepare controller test programs until the full native format and System/User boundaries are verified.

## Preconditions (must pass before functional testing)

| Gate | Required evidence | Status |
|---|---|---|
| G-01 | Written engineering authorization, protected cell, risk assessment, operator trained | Not verified |
| G-02 | Robot model and firmware/application IDs recorded | Not verified |
| G-03 | Full original controller backup with hash and restoration procedure | Not verified |
| G-04 | Signals reserved for tests are not motion/servo/guard/permissive safety signals | Not verified |
| G-05 | Manual ladder edited using official authorized tool; OEM parser accepts program | Not verified |
| G-06 | Output wiring and monitoring independently checked; no unexpected motion possible | Not verified |
| G-07 | Test and restoration operators assigned; rollback verified | Not verified |

## TMR verification cases

| ID | Inputs/setup | Expected (manual model) | Evidence to collect |
|---|---|---|---|
| T-01 | Preset=1, start OFF | Current=1, Q=OFF | Monitor M, Q |
| T-02 | Preset=1, start ON ≥100ms | Current reaches 0, Q=ON | Scope timing |
| T-03 | Preset=100, start ON <10s | Current>0, Q=OFF | Timestamps, tick count |
| T-04 | Preset=100, start ON ≥10s | Current=0, Q=ON | Output edge timing |
| T-05 | During a running countdown, turn start OFF | Current reloads preset, Q=OFF | Read register |
| T-06 | After elapsed timer, turn start OFF | Q turns OFF; current reloads preset | Monitor Q |
| T-07 | Preset via M-register | Current and Q follow OEM documentation | Both M values, firmware |
| T-08 | Change preset M-register during active timing | **To be established** (no MLAD assertion) | Raw sequence, scans |
| T-09 | Preset=0 | **Controller behavior must be measured**; offline model not authoritative | M/Q trace |
| T-10 | Power-cycle with start active | OEM timer reset semantics verified | Boot state, timeline |

## CNT verification cases

| ID | Inputs/setup | Expected (manual model) | Evidence |
|---|---|---|---|
| C-01 | Preset=3, R=ON | Current=3, Q=OFF | M/Q |
| C-02 | R=OFF, first C OFF→ON | Current=2, Q=OFF | C edge/M |
| C-03 | Hold C=ON through multiple scans | No repeated decrements | M trace |
| C-04 | C OFF→ON twice more with falling edges between | Current=0, Q=ON | Edge count, Q |
| C-05 | Additional C pulses after zero | Current remains 0, Q remains ON | M/Q |
| C-06 | C and R assert together | R wins: Current=3, Q=OFF | C,R,M,Q |
| C-07 | R=ON while C held ON; then R OFF while C remains ON | No synthetic extra C edge | Detailed trace |
| C-08 | Reset after Q=ON | Current reloads, Q=OFF | M/Q |
| C-09 | Preset from M-register | Test full declared range, including 65535 | Preset/M/Q |
| C-10 | Power-cycle with C active | OEM reset behavior verified | Boot trace |

## GSTR/GOUT verification cases

| ID | Source byte | Expected destination byte | Evidence |
|---|---|---|---|
| B-01 | 0x00 | 0x00 | Raw byte and all bits |
| B-02 | 0x01 | 0x01 | Bit0 ordering |
| B-03 | 0x55 | 0x55 | Alternating bit pattern |
| B-04 | 0x80 | 0x80 | Bit7 ordering |
| B-05 | 0xFF | 0xFF | All eight bits |
| B-06 | Rapidly change source byte | **Controller propagation/scan behavior to measure** | Both byte streams |
| B-07 | Unpaired GSTR or GOUT in OEM test parser | Expect ladder syntax/pair error according to manual 4150 | OEM diagnostic |
| B-08 | GOUT byte overlaps individual OUT destination | Expect rejection of duplicate output/overlap | OEM diagnostic 4040 |
| B-09 | Address not on bit0, e.g. `#20011` | Expected operand validation failure | OEM diagnostic |
| B-10 | Destination address read-only per controller map | Expected refusal by validation/configuration | Controller/profile log |

## Resource/ownership/format negative tests

| ID | Test | Expected |
|---|---|---|
| N-01 | TMR current register M560 | Rejected (M000–M559 required) |
| N-02 | CNT preset M999 | Syntactically accepted in manual range, actual register role must be reviewed |
| N-03 | Literal 65536 | Rejected (unsigned 16-bit maximum 65535) |
| N-04 | Two TMR/CNT blocks share current M010 | Duplicate current register error (4100) |
| N-05 | TMR/CNT/arithmetic total exceeds 100 | OEM resource diagnostic 4220; note total includes existing System Ladder |
| N-06 | Native file round-trip | Original bytes and SHA-256 exactly unchanged after MLAD read-only inspection |
| N-07 | Attempt `*.advanced.json` export as `CIOPRG.LST` | Must remain blocked |
| N-08 | Attempt HSES monitoring or FTP deployment | Must remain blocked |

## Acceptance evidence format

For each completed case, capture controller family, exact firmware, application/installed options, case ID, approved test configuration, source and destination addresses, initial register state, input edge timestamps, actual monitored register/output values, expected result, pass/fail, anomalies and restoration evidence. Store any OEM program exports privately until sanitization and approvals are complete.

**The repository does not publish the results of these hardware tests.** Cases are considered **unconfirmed** unless the bench protocol, exact model/firmware and measured values are attached. Local MLAD/CI tests are not counted for hardware execution.


## Additional UI and source immutability (after PR #7/#8)

| ID | Action. | Expected output |
|---|---|---|
| U-01 | Create several chains in `*.graph.json` | All chains are displayed simultaneously |
| U-02 | Manually switch `? / 0 / 1` в Graph Workspace | The color NO/NC/OUT is calculated locally; **not** LIVE |
| U-03 | Double click on NO/NC/OUT in JSON | Changes to JSON, Enter/Esc and Undo/Redo |
| U-04 | Double-click on M-register/TMR or CNT C/R | Editing only individual `*.advanced.json`reset of an old sample |
| U-05 | Modify GSTR/GOUT address `#20010` на `#20011` | UI rejects incorrect alignment of byte |
| U-06 | Double-click on Native Workspace `CIOPRG.LST` | Allow **only local comment**, not change address or original |
| U-07 | Compare. SHA-256 Original copy before/after Inspector | Values are identical; modified LST is not created |

UI test results are documented separately from T/C/B/N hardware cases. Procedure for obtaining and anonymizing source files: [controller_fixture_intake.md](controller_fixture_intake.en.md).

## Additional tests after PR #21 Not executed on equipment

| ID | Isolated bench | Confirmation |
| --- | --- | --- |
| EN-01 | MOV/ADD/SUB/SHL/SHR at EN=0/1 | Record D and flags in scan context |
| MOV-01 | 255/256/257 in byte and word | Carry truncation at border 256 |
| SHIFT-01 | n=0/1/8/15/16 | Carry and the Last Repressed Bit |
| SCAN-01 | Two Influencers on #51400/#51401/#51402 | Procedure for cleaning/updating scans |
| GEN-01 | DX100/DX200/YRC1000individual firmware | Generational differences |

Lack of access to robots means **NOT RUN* for all of these tests, regardless of the CI and manual reference model. [Details.](oem_5_opcode_analysis.en.md).

## Status of hardware verification after PR #29

As of the reconciliation date `main` SHA `f85ddfcce3063dd087425be981a9d2b7a16d6309` passed [Linux CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454) и [Windows CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621)**No test of this hardware matrix is confirmed to have been performed on DX100, DX200 or YRC1000**: `OEMBench=NOT RUN`. Verification on Windows runner is not a bench test. The future will be held only with a separate permitted insulated stand, OEM tool and a separate security procedure. [Verification gates](verification_gates.en.md).
