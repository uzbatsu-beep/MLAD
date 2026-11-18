<!-- Generated en companion for docs/hse_protocol.md; review required. -->
> **English** · [Русский](hse_protocol.ru.md) · [English](hse_protocol.en.md) · [Original / Оригинал](hse_protocol.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.
> **MLAD safety invariants:** `CIOPRG.LST` and `IONAME.DAT` are strictly **read-only**. FTP/HSES, live I/O, native writer and robot deployment remain **BLOCKED**. Passing `WindowsCI` does **not** imply `UserSmoke=PASS` or `OEMBenchVerified`: both remain **NOT RUN** unless independently documented. Machine translation requires source and Yaskawa manual review.

# Yaskawa High-Speed Ethernet (HSESstatus and verification requirements

**Verified in `main` at 18.11.2026:** HSES/FTP Runtime removed from compiled backend PR #13. Monitoring and loading to the controller are disabled; the protocol is not verified. [Current status](project_status.en.md).

> **Do not use this file as a packet instruction for a real controller.** The old contents of the document included a simplified YERC schema and arbitrary matching of I/O command/class codes. It has not been confirmed by the required guidance and testing. This data ** should not be considered a reliable implementation HSES for DX100, DX200 or YRC1000.

## 1. Actual state of source code

- В PR #13 past-file `src-tauri/src/network/mod.rs` ** removed from the source, and `pub mod network` absent `src-tauri/src/main.rs`. Unconfirmed UDP/FTP The client does not compile to MLAD.
- `cmd_toggle_monitoring` returns the refusal of inclusion; `cmd_deploy_to_robot` It always comes back. The existence of Tauri IPC for the sake of compatibility is not protocol support.
- `TRUE/FALSE/UNKNOWN` and local TMR/CNT/GSTR states originate only from manual/autonomous models, **not live HSES**.
- Subsequently PR #25#29 added production CSP, guarded reads, File ID, hardlink/staged safeguards and Windows regression. Last check→rename TOCTOU, ACL, backup+target and manual are still open WebView2 acceptance. Exclusion of the network client **not ** means a full security audit; [model](filesystem_security_pr29.en.md).

## 2. What to confirm by Yaskawa manuals

For each generation and version of firmware, install:

1. **Transport requirements:** UDP/TCP, port and availability options, modes/read rights, connection restrictions; do not assume the same behavior for all systems.
2. ** Complete YERC header:** Sizes, endian, reserved values, identifiers, service/ack/block, request/response correspondence, error fields and return rules.
3. **Exact command and addressing model:** allowed *read-only* commands, instance/class/attribute, addressing units and bit packing for a specific memory range.
4. **Controller-dependent addresses:** CIO comparison `#XXXXX`, logical groups and factual answers through the verified family table (see para. [typed_memory_model.md](typed_memory_model.en.md)).
5. **Reliability: ** Timeouts, repeat requests, response deduplication, sequence IDs, incomplete/error packet processing, survey limits, UNKNOWN status when unavailable signal.
6. **Write isolation:** Separate, code-limited read-only interface; no read operation should have implicit commands to write or control servo drives.
7. **Operational security:** Network segments, controlled request frequencies, no impact on standard automation, manufacturer/enterprise permission.

The fact that fields meet `YERC` port `8040` in examples of third-party sources, **does not confirm the correctness of a particular MLAD message. First, you need an original manual of the corresponding option with a full command table and confirmed query/response dumps.

## 3. Contract for future implementation (project, not current function)

```text
Explicit ControllerProfile + verified HSES option/firmware
    → ReadOnlyTransport
       → Validated request encoder (known command + allowed input operands)
       → Response parser (length, IDs, reserved/status, offsets, CRC if applicable)
       → Bounded scheduler / timeout / retry
       → SignalSnapshot(value | UNKNOWN, timestamp, quality, provenance)
       → Separate UI label: LIVE / STALE / DISCONNECTED
```

**No LIVE indicator until the source, freshness and connection of the response to the request are verified. **FALSE cannot be substituted for UNKNOWN** by timeout. The logical state of the coil requires a separate computation model and is not identical to its actual output feedback.

The encoder and decoder are first tested for **offline golden packets**: positive cases, negative status/error, truncated packets, reordered responses, disabling/repeating. Only then - limited read-only tests on the approved stand.

## 4. Read-only Monitoring Permission Criteria

Gate H0: Yaskawa + firmware/option profile and authorised list of read commands.  
Gate H1: test parser/encoder on saved packages and strict format errors  
Gate H2: Confirmation of I/O cards and correct display of bits from an external benchmark.  
Gate H3: Laboratory stress test timeouts, discount, reconnection and delays with fault-safe UNKNOWN.  
Gate H4: Independent code, security, and project owner approval.

**Even completed read-only HSES forbid FTP upload, register recording, robot control or System Ladder editing.**

Sources and gaps: [source_registry.md](source_registry.en.md), [architecture.md](architecture.en.md), [roadmap.md](roadmap.en.md).

## 5. Post-merged status PR #29

В `main` `f85ddfcce3063dd087425be981a9d2b7a16d6309` Monitoring/deploying teams retain **deny-only** behavior. [Linux](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454) и [Windows CI](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621) accomplished. In `Cargo.toml` yet still present `suppaftp` as dependency, which requires a separate supply-chain review/removal of unused components, **without the inclusion of network runtime. Possible read-only HSES BLOCKED M11 requires independent approval and OEM evidence. [Decisions](engineering_decisions.en.md).
