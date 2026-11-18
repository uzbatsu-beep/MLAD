<!-- Generated en companion for docs/licensing.md; review required. -->
> **English** · [Русский](licensing.ru.md) · [English](licensing.en.md) · [Original / Оригинал](licensing.md)
> **Translation review / Проверка:** machine-generated draft — verify engineering claims against the original and OEM manuals.

# Legal policy of MLAD - GNU GPL-3.0-only

** Decision date:** 17.11.2026. ** Status:** selected `GPL-3.0-only`; the copyright holder of his own original materials is an individual ** Fedor Sergeyevich Kochedykov**. Verification of rights to previously included third-party materials and compliance with the terms of distribution ** is not yet complete**.

## 1. What kind of license?

- ** SPDX ID:** `GPL-3.0-only`.
- **Exact text:** [../LICENSE](../LICENSE) — GNU General Public License, Version 3, 29 June 2007.
- ** Important difference:** `GPL-3.0-only` You can use version 3. **Or any later version is not available. For the future transition to `GPL-3.0-or-later` A separate decision of the authors/right holders with such authority will be required.
- **Sources of full text and rules of use:** [GNU GPLv3](https://www.gnu.org/licenses/gpl-3.0.html), [GNU: How to use GNU licenses](https://www.gnu.org/licenses/gpl-howto.html), [SPDX: GPL-3.0-only](https://spdx.org/licenses/GPL-3.0-only.html).
- **Metadata:** `package.json` / `src-tauri/Cargo.toml` must contain `GPL-3.0-only`.

In particular, when distributing executable MLAD or modified binary assemblies, there are obligations to provide the relevant source code and to retain the necessary notifications under GPLv3. The placement of the source on GitHub cannot be considered an automatic fulfillment of all the conditions of each mode of delivery.

## 2. What does and does not apply to GPL MLAD

| Object | Legal regime |
|---|---|
| MLAD proprietary source code to which authors are entitled | GPL-3.0-only |
| Own changes made to the repository | Must be compatible with GPL-3.0-only |
| Rust/npm libraries, WebView2 and system components | ** Keep your own ** licenses and notices |
| Manufacturer's manuals, standard drawings and diagrams | Do not become a GPL just because of citation in MLAD |
| Original/depersonalized `CIOPRG.LST` client | File owner rights; not automatically opened |
| Yaskawa/Motoman trademarks, logos and trademarks | Not licensed by GPL MLAD |
| User JSON projects/saved data | They are not automatically licensed to the GPL just because they are created in MLAD. |

**MLAD is not an official Yaskawa product, is not approved by the manufacturer and is not certified for robot control.** Prohibitions by native writer, HSES и FTP They remain irrespective of their legal status.

## 3. Rightholder and verification of origin

**Rightholder of his own original parts MLAD:** ** Fedor Sergeyevich Kochedykov**, an individual. Notice: **© 2026 Fedor Sergeyevich Kochedykov**. The copyright holder is determined by the decision of the project owner; the name is indicated in full in Russian. The notice relates only to the material to which the said person actually has rights.

Previous `authors = ["Antigravity Developer"]` в `Cargo.toml` It was a technical filler and replaced. Latin transliteration **not intended **: the full name in Cyrillic is used for international distribution before writing is confirmed.

**Open checks before the next binary release:**

1. Make sure that the original parts were created by the copyright holder with the necessary rights, taking into account possible contracts with employers / contractors.
2. Check the availability of contributions from other rightholders and the legality of their inclusion under the GPL-3.0-only.
3. Record the origin and licenses of embedded icons, images, templates, fonts, examples, borrowed and generated code.
4. Specify individual copyright notices for other people's materials - the name of the main copyright holder ** does not replace the authorship of third parties **.
5. Make a separate decision on Tauri `identifier` and migration of the Windows application; changing the license/author does not mean changing the ID of the installed program.
6. Verify the rights to any demo/source files of robots and not publish confidential `CIOPRG.LST`.

**The GPL does not transfer rights to third-party materials or waive the obligation to comply with their licenses.** Approval of the copyright holder of the original parts does not replace chain of rights verification for the entire repository.

## 4. Third-party dependencies and Windows builds

- **Do not change the licenses of third-party modules** in their source directories. Comply with their terms and conditions and necessary copyright/notice files.
- For future releases, form a machine-testable SBOM/register of npm and Rust packets with versions, SPDX license ID, source and binary distribution indication. **RC9 is already published; the availability of the source archive and LICENSE does not prove the completeness of the legal notices. **
- **PR #12 merged:** `package-lock.json` и `npm ci` Act in CI; also in Git stored `Cargo.lock`. This is a dependency accounting database, but not a ready-made third-party NOTICE/SBOM.
- For Windows NSIS/portable ZIP add text `LICENSE`, suitable third-party notifications and a clear reference to the origins of the issued commit/tag**. Consider the relevant GPLv3 obligations when transferring binary files.
- Previously released `v0.5.0-rc.1-hardware` created before this change; its ZIP cannot be assumed to have been automatically updated with a license or notification. Do not rewrite the tag/release retroactively.
- Check the Tauri license, WebView2 Runtime and other mandatory means of distribution separately; do not rely on the assumption of compatibility without a list of dependencies.

## 5. Supporting process

- The PR on the new code indicates the origin of the material and the license of the components to be included.
- In the PR template to provide verification “no client” `CIOPRG.LST` / secrets / author's manuals”.
- When a vulnerability request is received, use the future `SECURITY.md`Licensing and safety are different issues.
- When changing the license or external delivery principles, a separate legal/organizational review is required.

**State as of 18.11.2026:** GPL-3.0-only, details of the copyright holder, PR #11#12 and the ZIP+GPL source pipeline is available. **Remain:** Independent rights verification, SBOM/third-party notices, installation notification accuracy, and Windows code signing. This document is not an individual legal opinion.

## 6. The current release gate after PR #29

Readiness `main` (PR #1-#29 merged, CI SUCCESS) **not* equals the complete distribution license. Last published Windows rc.9 collected before PR #2429; the new temporary unsigned CI artifacts are not releases. The upcoming version of 0.6.x separately requires a GPL source archive of exact tag, third-party notices/license/SBOM review, Windows code-signing status, source asset/hash integrity and the owner's decision to publish. There are no contractual rights to Yaskawa/client materials under the MLAD license. [Current status](project_status.en.md) · [Release gates](verification_gates.en.md).
