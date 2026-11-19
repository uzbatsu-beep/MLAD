#!/usr/bin/env python3
"""Conservative RU/EN docs gate: do not mistake translation coverage for validation.

The script intentionally fails only on mechanical or high-impact semantic
regressions. A PASS is not a substitute for bilingual engineer/OEM review.
"""
from __future__ import annotations

import re
import sys
from pathlib import Path
from i18n_translate import ROOT, localized, sources

# These pairs were directly compared and re-edited against their originals.
EDITORIALLY_REVISED = {
    Path("README.md"),
    Path("docs/README.md"),
}

# Important: keep these claims precise even after future NMT regeneration.
EN_RELEASE_NEGATION = re.compile(
    r"(?:does\s+not\s+(?:include|contain)|(?:not|never)\s+included)"
    r".{0,35}PR\s*#24[–-]#29",
    flags=re.IGNORECASE | re.DOTALL,
)
BAD_EN = (
    "This is a *****",
    "** Preservation and protection of JSONPR",
    "Parental File Paths",
    "unconducted ** robot tests",
    "the last leaked PR project",
)
BAD_RU = (
    "Не бегать",
    "OEM скамейка",
    "Проект JON Graph",
    "мотоманской лестницы",
    "Сохранение и защита JSON (PR #24#29",
)

def check_reviewed(path: Path, lang: str, text: str) -> list[str]:
    failures=[]
    name=localized(path,lang).as_posix()
    if "MLAD-I18N:EDITORIAL" not in text:
        failures.append(f"{name}: reviewed companion missing editorial provenance")
    if "CIOPRG.LST" not in text or "IONAME.DAT" not in text:
        failures.append(f"{name}: mandatory native source names absent")
    if "NOT RUN" not in text or "rc.9" not in text:
        failures.append(f"{name}: release/manual evidence status missing")
    if lang=="en":
        if not EN_RELEASE_NEGATION.search(text):
            failures.append(f"{name}: missing explicit PR24–29 NOT-IN-RC9 negation")
        for fragment in BAD_EN:
            if fragment in text:
                failures.append(f"{name}: known damaged English translation: {fragment!r}")
    else:
        if not re.search(r"(?:не\s+(?:содержит|включает)|без)\s+PR\s*#24[–-]#29",text,re.I):
            failures.append(f"{name}: missing Russian PR24–29 NOT-IN-RC9 statement")
        for fragment in BAD_RU:
            if fragment in text:
                failures.append(f"{name}: known damaged Russian translation: {fragment!r}")
    return failures

def main() -> None:
    failures=[]
    draft_count=0
    reviewed_count=0
    files=sources()
    for source in files:
        for lang in ("ru","en"):
            name=localized(source,lang)
            file=ROOT/name
            if not file.is_file():
                failures.append(f"{name}: companion missing")
                continue
            text=file.read_text(encoding="utf-8")
            if source in EDITORIALLY_REVISED:
                reviewed_count+=1
                failures.extend(check_reviewed(source,lang,text))
            else:
                draft_count+=1
                if not any(
                    tag in text for tag in (
                        "machine-generated draft",
                        "автоматический черновик",
                        "editorial review required",
                        "MLAD-I18N:EDITORIAL",
                    )
                ):
                    failures.append(f"{name}: missing machine-draft/editorial review disclosure")
    if failures:
        for issue in failures:
            print("I18N SEMANTIC ERROR:", issue, file=sys.stderr)
        raise SystemExit(f"I18N semantic guard FAILED: {len(failures)} errors")
    print(f"I18N semantic guard PASS: {len(files)} sources; "
          f"{reviewed_count} editorial companions and {draft_count} "
          "unreviewed draft companions. This is NOT an OEM/linguistic certification.")

if __name__=="__main__":
    main()
