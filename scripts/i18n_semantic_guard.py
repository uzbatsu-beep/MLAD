#!/usr/bin/env python3
"""Verify RU/EN document coverage and source/meaning safety boundaries.

This check does NOT certify the remaining machine-translated documents.
A passing status is not OEM validation, a human translation review,
a controller safety authorization or a Windows UserSmoke result.
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

from i18n_translate import ROOT, localized, sources, strip_nav

# Russian originals copied without machine paraphrase; only local .md links
# were pointed at corresponding .ru.md variants.
SOURCE_ALIGNED_RU = {
    "README.md",
    "docs/README.md",
    "SECURITY.md",
    "CONTRIBUTING.md",
    "CODE_OF_CONDUCT.md",
    "ACCESSIBILITY.md",
    "docs/roadmap.md",
    "docs/implementation_plan.md",
    "docs/project_status.md",
    "docs/verification_gates.md",
    "docs/engineering_decisions.md",
    "docs/development_workflow.md",
    "docs/architecture.md",
    "docs/filesystem_security_pr29.md",
    "docs/windows_acceptance_pr28.md",
    "docs/licensing.md",
    "docs/json_persistence_safety.md",
    "docs/guarded_io_review.md",
    "docs/csp_path_hardening.md",
    "docs/hse_protocol.md",
    "docs/documentation_audit_2026-11-17.md",
    "docs/documentation_inventory_2026-11-18.md",
    "docs/oem_5_opcode_analysis.md",
    "docs/semantic_verification_matrix.md",
    "docs/roadmap_v2_archive.md",
    "docs/implementation_plan_v2_archive.md",
    "docs/controller_fixture_intake.md",
}

EDITORIAL_EN = {"README.md", "docs/README.md", "SECURITY.md", "docs/engineering_decisions.md", "docs/roadmap.md"}

EN_RELEASE_NEGATION = re.compile(
    r"(?:does\s+not\s+(?:include|contain)|(?:not|never)\s+included)"
    r".{0,35}PR\s*#24[–-]#29",
    flags=re.IGNORECASE | re.DOTALL,
)
RU_RELEASE_NEGATION = re.compile(
    r"(?:не\s+(?:содержит|включает)|без)\s+PR\s*#24[–-]#29",
    flags=re.IGNORECASE,
)
UNSAFE_EN = (
    "** Preservation and protection of JSONPR",
    "Parental File Paths",
    "unconducted ** robot tests",
    "the last leaked PR project",
)
DRAFT_MARKERS = (
    "machine-generated draft",
    "автоматический черновик",
    "editorial review required",
)


def canonical_body(localized_text: str) -> str:
    # The header is navigation/provenance; the actual localized document
    # starts at its first ATX H1 heading.
    match = re.search(r"(?m)^#\s+", localized_text)
    if not match:
        raise ValueError("missing document H1")
    prose = localized_text[match.start():]
    # Undo RU companion link routing, while preserving all other bytes.
    return re.sub(
        r"(\]\((?!https?:|//|#)[^)\s]+?)\.ru\.md(\))",
        r"\1.md\2",
        prose,
    )


def main() -> None:
    failures = []
    count_ru = 0
    count_en = 0
    drafts = 0
    found = sources()

    if len(found) != 53:
        failures.append(f"source inventory changed: expected 53, found {len(found)}; update review register")

    for src in found:
        original = strip_nav((ROOT / src).read_text(encoding="utf-8"))
        for lang in ("ru", "en"):
            target = localized(src, lang)
            full_path = ROOT / target
            if not full_path.is_file():
                failures.append(f"{target}: missing companion")
                continue
            text = full_path.read_text(encoding="utf-8")
            if src.as_posix() in SOURCE_ALIGNED_RU and lang == "ru":
                count_ru += 1
                if "MLAD-I18N:EDITORIAL" not in text:
                    failures.append(f"{target}: missing source-alignment provenance")
                try:
                    if canonical_body(text).rstrip("\n") != original.rstrip("\n"):
                        failures.append(f"{target}: source-aligned Russian content diverged")
                except ValueError as err:
                    failures.append(f"{target}: {err}")
            elif src.as_posix() in EDITORIAL_EN and lang == "en":
                count_en += 1
                if "MLAD-I18N:EDITORIAL" not in text:
                    failures.append(f"{target}: missing English editorial provenance")
                if src.as_posix() in {"README.md", "docs/README.md"} and not EN_RELEASE_NEGATION.search(text):
                    failures.append(f"{target}: rc.9 PR24–29 NOT-IN-RELEASE claim absent/reversed")
                if src.as_posix() in {"README.md", "docs/README.md", "SECURITY.md"} and "NOT RUN" not in text:
                    failures.append(f"{target}: independent Windows/OEM test status missing")
                if "CIOPRG.LST" not in text or "IONAME.DAT" not in text:
                    failures.append(f"{target}: immutable source identifiers missing")
                if any(bad in text for bad in UNSAFE_EN):
                    failures.append(f"{target}: known damaged English technical phrase present")
            else:
                drafts += 1
                if not any(marker in text for marker in DRAFT_MARKERS):
                    failures.append(f"{target}: unreviewed machine draft lacks warning")

    if SOURCE_ALIGNED_RU - {p.as_posix() for p in found}:
        failures.append("review register lists missing Russian sources")
    if (count_ru, count_en, drafts) != (27, 5, 74):
        failures.append(f"incorrect review totals: ru={count_ru}, en={count_en}, drafts={drafts}")

    if failures:
        for problem in failures:
            print("I18N SEMANTIC ERROR:", problem, file=sys.stderr)
        raise SystemExit(f"I18N semantic guard FAILED ({len(failures)} issues)")

    print(
        f"I18N semantic guard OK: {len(found)} source docs / {len(found)*2} companions; "
        f"{count_ru} Russian source-aligned, {count_en} English editorial, "
        f"{drafts} explicitly unreviewed machine drafts."
    )
    print("Structural/semantic guard != bilingual engineering validation or OEM certification.")


if __name__ == "__main__":
    main()
