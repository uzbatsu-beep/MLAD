<!-- Generated en companion for CODE_OF_CONDUCT.md; review required. -->
> **English** · [Русский](CODE_OF_CONDUCT.ru.md) · [English](CODE_OF_CONDUCT.en.md) · [Original / Оригинал](CODE_OF_CONDUCT.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.
> **MLAD safety invariants:** `CIOPRG.LST` and `IONAME.DAT` are strictly **read-only**. FTP/HSES, live I/O, native writer and robot deployment remain **BLOCKED**. Passing `WindowsCI` does **not** imply `UserSmoke=PASS` or `OEMBenchVerified`: both remain **NOT RUN** unless independently documented. Machine translation requires source and Yaskawa manual review.

# Code of interaction of MLAD participants

**Date:** 18.11.2026. Applies to GitHub Issues, Pull Requests, Discussions, Code Review and other venues where members submit MLAD.

## Our goal.

MLAD is created for responsible engineering research of industrial automation. Technical criticism, reproducible tests, documentation and proper discussion of limitations are welcome. Disagreeing on the architecture or interpretation of manuals is a normal part of the job; personalization is unacceptable.

## Expected behaviour

- Separate **observation **, **hypothesis **, **approval from documentation ** and **controller test result **. Don’t call untested semantics “reliable.”
- Communicate respectfully and on a case-by-case basis regardless of the participant’s experience, role, language or background; make technical arguments instead of accusations.
- Acknowledge possible errors, correct inaccuracies, link to available permitted sources, and fix check boundaries.
- Respect the rights of owners of programs, documentation, industrial data and third-party libraries.
- Send reports of vulnerabilities to [SECURITY.md](SECURITY.en.md)Do not publish exploits and robotic cell data in public discussions.

## Inappropriate behaviour

Persecution, threats, insults, discrimination, publication of other people's confidential data, attempts to impersonate representatives of the manufacturer, deliberate misleading about safety, imposing actions with equipment without consent. Repeated spam and interference in discussions to block responsible development are also prohibited.

## Reporting of violations and reaction

In case of violation, use the available **private communication channel with the accompanying person via GitHub, if configured **, or the built-in **Report content ** platform. In public Issues, it is permissible to describe only the general problem of moderation without personal, confidential or dangerous details. **There is no public email address for messages here. **

The accompanying person assesses the circumstances and proportionality of the measures: request to correct the message, warning, concealment/deletion of content, restriction of participation or blocking. Where possible, the privacy of the applicant and the parties is maintained; absolute confidentiality is not guaranteed. The issue of reviewing the decision can be raised through the available private channel GitHub; immediate response or installed SLA is not promised.

## Scope and permits

This code does not change [GPL-3.0-only](LICENSE)does not grant the right of access to the controller and does not cancel [CONTRIBUTING.md](CONTRIBUTING.en.md). Decisions to merge PR and include potentially dangerous functions are made by the project owner separately.
