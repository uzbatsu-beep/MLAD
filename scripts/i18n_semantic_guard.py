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

EDITORIAL_RU = {
    "docs/cioprg_lst_spec.md",
    "docs/typed_memory_model.md",
    "docs/advanced_blocks.md",
    "docs/advanced_hardware_test_matrix.md",
    "docs/typed_opcode_grammar.md",
    "docs/cioprg_lossless_parser.md",
    "docs/source_registry.md",
    "docs/post_end_data.md",
    "docs/semantic_ast.md",
    "docs/ioname_annotations.md",
    "docs/graph_editor.md",
    "docs/ui_multi_rung_state.md",
    "docs/inline_address_editing.md",
}
EDITORIAL_EN = {
    "README.md", "docs/README.md", "SECURITY.md",
    "docs/engineering_decisions.md", "docs/roadmap.md",
    "docs/verification_gates.md",
    "docs/cioprg_lst_spec.md", "docs/typed_memory_model.md",
    "docs/advanced_blocks.md", "docs/advanced_hardware_test_matrix.md",
    "docs/typed_opcode_grammar.md", "docs/cioprg_lossless_parser.md",
    "docs/oem_5_opcode_analysis.md", "docs/semantic_verification_matrix.md",
    "docs/source_registry.md", "docs/post_end_data.md",
    "docs/semantic_ast.md", "docs/ioname_annotations.md",
    "docs/graph_editor.md", "docs/ui_multi_rung_state.md",
    "docs/inline_address_editing.md", "docs/json_persistence_safety.md",
}

# Terms that may never be weakened by editorial translations. The set is
# intentionally small and documentation-specific, not an OEM approval.
CRITICAL_REVIEW_MARKERS = {
    "docs/cioprg_lst_spec.md": ("//$M", "System", "User", "PART", "END"),
    "docs/typed_memory_model.md": ("#82010", "#87010", "#25010", "#37010", "M000–M999"),
    "docs/advanced_blocks.md": ("TMR", "CNT", "GSTR", "GOUT", "100"),
    "docs/advanced_hardware_test_matrix.md": ("T-01", "C-01", "B-01", "N-01", "OEMBench"),
    "docs/typed_opcode_grammar.md": ("TypedCandidate", "syntax_valid=true", "semantics_verified=false"),
    "docs/cioprg_lossless_parser.md": ("SHA-256", "//$M", "System", "User", "false"),
    "docs/oem_5_opcode_analysis.md": ("MOV", "ADD", "SUB", "SHL", "SHR", "256"),
    "docs/semantic_verification_matrix.md": ("ManualDerived", "TypedCandidate", "semantics_verified=false"),
    "docs/source_registry.md": ("RE-CKI-A453", "RE-CKI-A465", "RE-CKI-A467", "HW1483388", "MANUAL", "SOURCE / TEST", "REAL FILE", "BENCH"),
    "docs/post_end_data.md": ("PostEndSection", "PostEndData", "PostEndOpaque", "PostEndUnknownStructure", "AfterEnd", "subset_analysis_complete=false"),
    "docs/semantic_ast.md": ("UnbalancedBranch", "PotentialGroupRead", "PotentialGroupWrite", "system_user_boundaries_verified", "MAX_BOOLEAN_NODES", "MAX_OPEN_BRANCHES"),
    "docs/ioname_annotations.md": ("relay_mapping_verified", "native_write_allowed", "4 MiB", "16 MiB", "IONAME.DAT", "localStorage"),
    "docs/graph_editor.md": ("GraphDraft", "cmd_validate_graph_draft", "cmd_save_graph_draft", "edited_cioprg_export_allowed: false", "Undo/Redo", "UNKNOWN", "File ID"),
    "docs/ui_multi_rung_state.md": ("FALSE AND UNKNOWN = FALSE", "TRUE OR UNKNOWN = TRUE", "TRUE", "FALSE", "UNKNOWN", "OFFLINE", "HSES/FTP"),
    "docs/inline_address_editing.md": ("#70018", "#20011", "M560", "F2", "Enter", "Esc", "Undo/Redo", "UNKNOWN"),
    "docs/json_persistence_safety.md": ("draft_io::write_atomic", "create_new(true)", "sync_all", "rename", "File ID", "TOCTOU", "*.bak"),
}

# High-impact negative claims that cannot be recovered by preserving tokens
# alone: each is checked for reviewed language/source pairs.
CRITICAL_PROHIBITIONS = {
    ("docs/graph_editor.md", "en"): ("JSON drafts only", "does not authorize native export"),
    ("docs/graph_editor.md", "ru"): ("только JSON-черновик", "не разрешает"),
    ("docs/ui_multi_rung_state.md", "en"): ("NOT live I/O", "not telemetry"),
    ("docs/ui_multi_rung_state.md", "ru"): ("не является live I/O", "не телеметрия"),
    ("docs/inline_address_editing.md", "en"): ("only in JSON drafts", "no access"),
    ("docs/inline_address_editing.md", "ru"): ("только в JSON-черновиках", "не даёт доступа"),
    ("docs/json_persistence_safety.md", "en"): ("No single atomic transaction", "still not guaranteed"),
}

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


def canonical_body(localized_text: str, original: str) -> str:
    # Find the exact start of the source body. A document can begin with an
    # archival warning BEFORE H1, while a language header may contain several
    # explanatory blockquotes/blank lines before the source begins.
    prefix = original[:min(72, len(original))]
    index = localized_text.find(prefix)
    if index < 0:
        raise ValueError("source body prefix missing")
    prose = localized_text[index:]
    # Undo RU companion link routing, while preserving all other bytes.
    return re.sub(
        r"(\]\((?!https?:|//|#)[^)\s]+?)\.ru\.md(\))",
        r"\1.md\2",
        prose,
    )


def main() -> None:
    failures = []
    count_ru = 0
    count_ru_editorial = 0
    count_en = 0
    drafts = 0
    found = sources()

    if len(found) != 54:
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
            for prohibition in CRITICAL_PROHIBITIONS.get((src.as_posix(), lang), ()):
                if prohibition.casefold() not in text.casefold():
                    failures.append(f"{target}: critical safety negation missing {prohibition!r}")
            if src.as_posix() in SOURCE_ALIGNED_RU and lang == "ru":
                count_ru += 1
                if "MLAD-I18N:EDITORIAL" not in text:
                    failures.append(f"{target}: missing source-alignment provenance")
                try:
                    if canonical_body(text, original).rstrip("\n") != original.rstrip("\n"):
                        failures.append(f"{target}: source-aligned Russian content diverged")
                except ValueError as err:
                    failures.append(f"{target}: {err}")
            elif src.as_posix() in EDITORIAL_RU and lang == "ru":
                count_ru_editorial += 1
                if "MLAD-I18N:EDITORIAL" not in text:
                    failures.append(f"{target}: missing Russian editorial provenance")
                if any(tag not in text for tag in ("CIOPRG.LST", "BLOCKED", "NOT RUN")):
                    failures.append(f"{target}: missing mandatory native/safety boundary")
                for expected in CRITICAL_REVIEW_MARKERS.get(src.as_posix(), ()):
                    if expected not in text:
                        failures.append(f"{target}: missing critical source token {expected!r}")
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
                for expected in CRITICAL_REVIEW_MARKERS.get(src.as_posix(), ()):
                    if expected not in text:
                        failures.append(f"{target}: missing critical source token {expected!r}")
            else:
                drafts += 1
                if not any(marker in text for marker in DRAFT_MARKERS):
                    failures.append(f"{target}: unreviewed machine draft lacks warning")

    # Match the published per-file review matrix, not just aggregate counts.
    coverage = (ROOT / "docs/translation_coverage.md").read_text(encoding="utf-8")
    for src in found:
        name = src.as_posix()
        row = next((line for line in coverage.splitlines()
                    if line.startswith(f"| [{name}](")), None)
        ru_state = "SourceAligned" if name in SOURCE_ALIGNED_RU else (
            "EditorialPass" if name in EDITORIAL_RU else "MachineDraft")
        en_state = "EditorialPass" if name in EDITORIAL_EN else "MachineDraft"
        if row is None or not row.rstrip().endswith(
            f"| {ru_state} | {en_state} |"
        ):
            failures.append(f"{name}: translation_coverage.md quality row is missing or stale")

    if SOURCE_ALIGNED_RU - {p.as_posix() for p in found}:
        failures.append("review register lists missing Russian sources")
    if (count_ru, count_ru_editorial, count_en, drafts) != (27, 13, 22, 46):
        failures.append(f"incorrect review totals: ru_source={count_ru}, ru_editorial={count_ru_editorial}, en={count_en}, drafts={drafts}")

    if failures:
        for problem in failures:
            print("I18N SEMANTIC ERROR:", problem, file=sys.stderr)
        raise SystemExit(f"I18N semantic guard FAILED ({len(failures)} issues)")

    print(
        f"I18N semantic guard OK: {len(found)} source docs / {len(found)*2} companions; "
        f"{count_ru} Russian source-aligned, {count_ru_editorial} Russian editorial, "
        f"{count_en} English editorial, "
        f"{drafts} explicitly unreviewed machine drafts."
    )
    print("Structural/semantic guard != bilingual engineering validation or OEM certification.")


if __name__ == "__main__":
    main()
