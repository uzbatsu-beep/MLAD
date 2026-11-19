<!-- MLAD-I18N:EDITORIAL:reviewed 2026-11-19 against SECURITY.md -->
> **Language / Язык:** [English](SECURITY.en.md) · [Русский](SECURITY.ru.md) · [Original](SECURITY.md)
>
> **Safety:** This policy is not a manufacturer certification, penetration-test report or permission to operate equipment. When the translation and the original disagree, the original and the applicable OEM documents take precedence.

# MLAD Security Policy

**Effective date:** 18 November 2026. **Scope:** the public MLAD repository and its published engineering preview builds.

> MLAD is an experimental **OFFLINE/BENCH** tool for analyzing Yaskawa Concurrent I/O. **Do not use it as a validated compiler, a safety device, a robot operating tool or a source of live I/O signals.** No mode permits uploading a program to a robot or modifying original `CIOPRG.LST` or `IONAME.DAT` files.

## Supported versions and security fixes

| Component | Policy |
| --- | --- |
| Current `main` | Reports and fixes are accepted; response and release times are not guaranteed |
| Latest Windows prerelease at the policy date (`v0.5.0-rc.9-oem-reference`) | Reports are accepted; this is an experimental version and **is not certified safe for industrial equipment** |
| Older release candidates and historical branches | Not maintained as independent security-fix lines; compare defects against current `main` |

This table **does not constitute a service-level agreement (SLA)**. Check version-specific status in [GitHub Releases](https://github.com/uzbatsu-beep/MLAD/releases) and the [current project status](docs/project_status.en.md).

## Reporting a security vulnerability

**Do not create a public issue containing technical details of an unpatched vulnerability.**

1. Open the repository's **Security** tab and check whether **Report a vulnerability** is available. If so, use **GitHub private vulnerability reporting**. Availability depends on repository settings; publishing `SECURITY.md` alone does not enable it.
2. If private reporting is not available, create a **public issue without vulnerability details**, asking the maintainer to establish a safe private communication channel. **Do not attach** proof-of-concept exploit code, sensitive paths, tokens, file contents, network configurations or exploit reproduction steps. Provide those only through an agreed private channel.
3. For abuse that concerns the GitHub platform itself, use GitHub's **Report** facilities. Platform abuse reporting is not a substitute for coordinated disclosure of an MLAD vulnerability.

**No security-contact email address has been invented or published.** The availability of a private reporting route must be checked separately. GitHub documentation: [Configure private vulnerability reporting](https://docs.github.com/en/code-security/how-tos/report-and-fix-vulnerabilities/configure-vulnerability-reporting/configure-for-a-repository).

For a private report, include the MLAD version/commit, operating system, expected and observed security boundary, a minimal **synthetic** reproduction, impact, and affected files/components. **Do not attach real robot programs.**

## Confidentiality of industrial data

**Do not publish the following in GitHub issues, PRs, Actions artifacts/logs or commits without the right to disclose them:**

- Actual `CIOPRG.LST` or `IONAME.DAT` files, controller system backups, I/O maps and production robot parameters.
- Real IP addresses, network topology, credentials, access keys, session tokens or logs containing them.
- Personal or commercial information, or details that could allow a third party to reconstruct production sequences, unless the owner has explicitly authorized disclosure.

Before publishing a reproduction, use **newly created synthetic data**. Merely removing a company name does not necessarily anonymize the logic or addresses. Hashes of real files, paths, hostnames and firmware revisions can also disclose context; publish them only when permitted.

## Project trust boundaries

- `CIOPRG.LST` and `IONAME.DAT` are **read-only** inputs; JSON drafts are a separate format. Original files must not be modified or reassembled into claimed OEM-native programs.
- FTP/HSES, monitoring, live I/O, robot deployment and robot write commands are **disabled (BLOCKED)**.
- Local `TRUE/FALSE/UNKNOWN` test states are **not live controller signals**.
- The `MOV/ADD/SUB/SHL/SHR` `TypedCandidate` forms, static references and `ManualDerived` examples **do not establish how a particular robot executes these operations**.
- PR #25–#26 introduced CSP, path checking and guarded reads; PR #28 added Windows NTFS security CI; PR #29 added byte-and-File-ID comparisons, rejection of hardlinks and Windows ADS/UNC/device paths, and staged-file SHA-256 verification. **Still open:** the final `check→rename` TOCTOU window, parent-directory races, Windows ACL handling, atomicity of the `backup+target` pair, power-loss durability, and independent Windows/WebView2 user testing. Passing CI is **not** an independent penetration test. See the [filesystem threat model](docs/filesystem_security_pr29.en.md).

Use the experimental application **offline, on copies, without connecting it to operational industrial equipment**. Do not test vulnerabilities against somebody else's controller, plant or network without explicit permission.

## Handling and disclosing fixes

The maintainer evaluates reports, assesses impact and coordinates appropriate disclosure. A patch release, security advisory/CVE, timeline and new-version publication require separate decisions. **There is no assumption that vulnerability details will be published automatically before a fix.** Rust, TypeScript and CI tests do not replace independent OEM or hardware validation.

[CONTRIBUTING](CONTRIBUTING.en.md) · [Architecture](docs/architecture.en.md) · [Current status](docs/project_status.en.md)

## Technical baseline (18 November 2026)

Verified `main` at `f85ddfcce3063dd087425be981a9d2b7a16d6309` after [PR #29 merged](https://github.com/uzbatsu-beep/MLAD/pull/29). [Linux CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454) and [Windows security CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621) apply to that SHA. These results **do not** establish successful manual WebView2 testing, elimination of all filesystem races, OEM bench equivalence or permission for native controller writes. [Verification gates](docs/verification_gates.en.md) · [Roadmap v3](docs/roadmap.en.md).
