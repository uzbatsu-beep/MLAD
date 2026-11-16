# MLAD PR #5 — Hardware Bench Validation Matrix (manual procedure, not executed)

Applies separately to **DX100**, **DX200**, and **YRC1000**. Each result must record exact model/firmware/application, Ladder Editor revision, `CIOPRG.LST` backup SHA-256, electrical I/O mapping, scan timing and the engineer who authorized the test.

**Safety / setup gate:** No MLAD CIOPRG export, deployment or live I/O control exists in PR #5. Use manufacturer-supported editing/backup, an isolated nonhazardous test rig, independent functional safety, physical motion inhibition and the site's test procedure. Never substitute this matrix for an OEM commissioning checklist or use production safety-related signals as test points.

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

**All cases in this matrix are currently UNEXECUTED.**
