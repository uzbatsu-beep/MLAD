<!-- Generated en companion for SECURITY.md; review required. -->
> **English** · [Русский](SECURITY.ru.md) · [English](SECURITY.en.md) · [Original / Оригинал](SECURITY.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.
> **MLAD safety invariants:** `CIOPRG.LST` and `IONAME.DAT` are strictly **read-only**. FTP/HSES, live I/O, native writer and robot deployment remain **BLOCKED**. Passing `WindowsCI` does **not** imply `UserSmoke=PASS` or `OEMBenchVerified`: both remain **NOT RUN** unless independently documented. Machine translation requires source and Yaskawa manual review.

# Security Policy MLAD (Security Policy)

**Date of introduction: ** 18.11.2026. **Region:** MLAD open repository and its published engineering preview builds.

> MLAD is an experimental OFFLINE/BENCH analysis tool for Yaskawa Concurrent I/O. **Do not use as a proven compiler, security device, robot operation tool or live I/O source.** No mode allows downloading programs to the robot or modifying the original CIOPRG.LST/IONAME.DAT.

## Supported versions and corrections

| Component | Policy |
| --- | --- |
| Current `main` | Communications and corrections are accepted; timing of response and issuance of corrections is not guaranteed |
| Last Windows pre-release (as of policy date): `v0.5.0-rc.9-oem-reference`) | Messages accepted; experimental version, **not certified as safe for equipment** |
| Old RCs and individual archive branches | Not supported as separate lines of correction; it is recommended to check the defect with the current one. `main` |

This table **is not a service promise by SLA**. Check the status of a specific version [GitHub Releases](https://github.com/uzbatsu-beep/MLAD/releases) и [project-matrix](docs/project_status.en.md).

## How to report vulnerability

**Do not create a public Issue with technical details of an unresolved vulnerability. **

1. Open the **Security** repository tab and check if the **Report a vulnerability* button is available. If available, please send a description via **GitHub private vulnerability reporting**. Possibility depends on the settings of the repository and is not guaranteed by the presence of one only. `SECURITY.md`.
2. If the feature is unavailable, create a regular public Issue without the details of the vulnerability**, only with a request to the attendant to determine a secure private channel for coordination. **Do not attach PoCs, paths, tokens, file content, network configurations, or a playback exploit. Then use the agreed closed channel.
3. If the breach relates to abuse on the GitHub platform itself, use the **Report** GitHub mechanisms; they do not replace the technical channel for the MLAD vulnerabilities.

**The email address of the escort has not been invented or published; the presence of a private channel is checked separately.** Instructions for enabling private vulnerability reporting: [GitHub Docs](https://docs.github.com/en/code-security/how-tos/report-and-fix-vulnerabilities/configure-vulnerability-reporting/configure-for-a-repository).

With a closed message, it is useful to describe: the MLAD/commit version, operating system, expected and actual security boundary, minimum **synthetic** playback, degree of influence and affected files/components. Do not attach real robot files.

## Confidentiality of industrial data

**Not allowed on GitHub Issues/PR/Actions/Committees:**

- real `CIOPRG.LST`, `IONAME.DAT`Backups of system software, I/O cards and parameters of production robots without the right to disclose;
- real IP addresses, network topologies, credentials, access keys, session tokens and logs with them;
- personal/commercial data of the organization and information that allows you to restore the operational chain without the permission of the owner.

Before publishing, replace any data with self-created synthetic examples**. Simply deleting a business name does not guarantee the depersonalization of logic and addresses. The real source hash, file path, node name, and firmware version can also reveal context; publish them only with permission.

## Limits of trust of the project

- `CIOPRG.LST` и `IONAME.DAT` JSON drafts are a separate format. Original files cannot be modified or reassembled as native programs.
- FTP/HSES, monitoring, live I/O, deploy and recording commands to the robot ** disabled**.
- `TRUE/FALSE/UNKNOWN` In local models - live controller signals.
- `MOV/ADD/SUB/SHL/SHR`: `TypedCandidate`, static references and `ManualDerived` **Not equal to performance on a particular system.
- In the current `main` PR #25#26 added CSP/path guard/identity-checked reads PR #28 — Windows NTFS security CI, PR #29 Comparison of File ID+bytes, rejection of hardlinks/ADS/UNC/device paths and staged SHA-256 checks. **Open:** Final. `check→rename` TOCTOU, ancestor races, Windows ACL, transaction `backup+target`Power-loss durability and manual WindowsWebView2 smoke. CI**not** is an independent pentest. [threat model](docs/filesystem_security_pr29.en.md).

Check the experimental software only autonomously on copies, **without connecting to existing industrial equipment**. Do not investigate vulnerabilities on someone else’s controller, enterprise, or network without explicit permission.

## Work on correction

The accompanying person evaluates the message, determines the impact and agrees on the correct disclosure. The publication of the patch, Advisory/CVE, the timing and the decision to release the version are taken separately. **Automatic publication of parts before correction is not expected.** Rust/TypeScript/CI checks and hardware verification are different levels of confirmation.

[CONTRIBUTING.md](CONTRIBUTING.en.md) · [Architecture](docs/architecture.en.md) · [Current status](docs/project_status.en.md)

## Technical control point (18.11.2026)

Proven. `main` @ `f85ddfcce3063dd087425be981a9d2b7a16d6309` merged [PR #29](https://github.com/uzbatsu-beep/MLAD/pull/29). [Linux CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454) и [Windows security CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621) on this SHA. That ******** confirms an independent WebView2 Smoke, file racing protection, OEM bench or native writer admission. [Verification gates](docs/verification_gates.en.md) · [Roadmap v3](docs/roadmap.en.md).
