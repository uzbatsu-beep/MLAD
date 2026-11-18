#!/usr/bin/env python3
"""Editorial safety terminology pass for generated RU/EN Markdown drafts.

This is intentionally conservative: it does not rewrite any source .md or
assert that machine translation has undergone full human-language review.
"""
from __future__ import annotations
from pathlib import Path
from i18n_translate import ROOT, sources, localized

RU_FIXES={
    "PR #1#29 Слияние":"PR #1–#29 объединены",
    "** Не бегать**":"**NOT RUN (не проводилось)**",
    "**Не бегать**":"**NOT RUN (не проводилось)**",
    "*Не бегать*":"**NOT RUN (не проводилось)**",
    "Не бегать":"NOT RUN (не проводилось)",
    "Не бегали":"NOT RUN (не проводилось)",
    "не бегать":"не проводилось (NOT RUN)",
    "OEM скамейка":"OEM-стенд",
    "Скамейка OEM":"OEM-стенд",
    "OEM скамейку":"OEM-стенд",
    "ОЕМ скамейка":"OEM-стенд",
    "скамейка OEM":"OEM-стенд",
    "** Заблокировано**":"**BLOCKED (заблокировано)**",
    "*Заблокировано*":"**BLOCKED (заблокировано)**",
    "Родное рабочее пространство":"Рабочая область Native Workspace",
    "родное рабочее пространство":"рабочая область Native Workspace",
}
EN_FIXES={
    "Bypass immutability":"Byte-for-byte immutability",
    "BYPASS immutability":"Byte-for-byte immutability",
    "HSES/FTPLive":"HSES/FTP, live",
    "FTP/HSESRobot":"FTP/HSES, robot",
    "wood, page source":"navigation tree, paginated source text",
    "wood,":"navigation tree,",
    "The latest release released by Windows":"The latest published Windows release",
    "documentary updating":"documentation update",
    "Documentary updating":"Documentation update",
    "hand-held WebView2/OEM bench":"manual WebView2 smoke test / OEM bench",
    "HSES/FTP runtime remote":"HSES/FTP runtime removed",
    "Remote compiled HSES/FTP runtime":"Compiled HSES/FTP runtime removed",
    "Policy | Politics":"Policy | Policy",
    "| Component | Politics |":"| Component | Policy |",
    "| Actual `main` |":"| Current `main` |",
    "It means that the whole project stopped at PR #21.":
        "This does **not** mean the entire project stopped at PR #21.",
    "It means that the whole project stopped at PR #21":
        "This does **not** mean the entire project stopped at PR #21",
}
SAFETY={
"ru":("> **Контрольные ограничения MLAD:** `CIOPRG.LST` и `IONAME.DAT` — "
      "строго **только чтение**. FTP/HSES, live I/O, native writer и загрузка "
      "в контроллер — **BLOCKED (запрещены)**. `WindowsCI` не заменяет "
      "`UserSmoke=NOT RUN` и `OEMBench=NOT RUN`; машинный перевод должен быть "
      "сверен с исходным документом и документацией Yaskawa."),
"en":("> **MLAD safety invariants:** `CIOPRG.LST` and `IONAME.DAT` are strictly "
      "**read-only**. FTP/HSES, live I/O, native writer and robot deployment "
      "remain **BLOCKED**. Passing `WindowsCI` does **not** imply "
      "`UserSmoke=PASS` or `OEMBenchVerified`: both remain **NOT RUN** "
      "unless independently documented. Machine translation requires source "
      "and Yaskawa manual review.")
}
def apply(path,language):
    text=path.read_text(encoding="utf-8")
    fixes=RU_FIXES if language=="ru" else EN_FIXES
    for old,new in fixes.items():
        text=text.replace(old,new)
    if path.name=="README.en.md":
        start=text.find("**Reconciliation 18.11.2026:")
        end=text.find("\n\n",start)
        if start>=0 and end>start:
            text=text[:start]+(
                "**Verified on 18 November 2026:** [main](https://github.com/uzbatsu-beep/MLAD) "
                "at `f85ddfcce3063dd087425be981a9d2b7a16d6309`; "
                "**PR #1–#29 merged**. "
                "[Linux CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454) "
                "and [Windows CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621). "
                "The latest published Windows release is **rc.9** and does **not** include PR #24–#29. "
                "See [roadmap v3](docs/roadmap.en.md) and [project status](docs/project_status.en.md). "
                "Manual WebView2 smoke testing and OEM bench verification are **NOT RUN**."
            )+text[end:]
        start=text.find("> **Security:**")
        end=text.find("\n\n",start)
        if start>=0 and end>start:
            text=text[:start]+(
                "> **Safety:** MLAD is neither a Yaskawa-certified engineering environment "
                "nor a verified native `CIOPRG.LST` compiler. Original `CIOPRG.LST` "
                "and `IONAME.DAT` are **read-only**. Native writing, FTP/HSES, "
                "robot deployment and live telemetry are **disabled**. "
                "Use manufacturer-approved tools and independent authorization "
                "for any work involving physical equipment."
            )+text[end:]
        text=text.replace("## Virtually available","## Implemented functionality")
        text=text.replace("| Function | Status. |","| Function | Status |")
    if path.name=="README.ru.md":
        start=text.find("**Сверка 18.11.2026:")
        end=text.find("\n\n",start)
        if start>=0 and end>start:
            text=text[:start]+(
                "**Сверка 18.11.2026:** [main](https://github.com/uzbatsu-beep/MLAD) "
                "`f85ddfcce3063dd087425be981a9d2b7a16d6309`, "
                "**PR #1–#29 объединены**; "
                "[Linux CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759454) "
                "и [Windows CI SUCCESS](https://github.com/uzbatsu-beep/MLAD/actions/runs/44373759621). "
                "Последний опубликованный Windows-релиз — **rc.9**, без PR #24–#29. "
                "См. [roadmap v3](docs/roadmap.ru.md) и [статус](docs/project_status.ru.md). "
                "Ручная проверка WebView2 и OEM-испытания — **NOT RUN**."
            )+text[end:]
    lines=text.splitlines()
    if not any("MLAD safety invariants" in s or "Контрольные ограничения MLAD" in s for s in lines[:12]):
        index=next((i for i,line in enumerate(lines)
                    if line.startswith("> **Translation review / Проверка:**")),None)
        if index is not None:
            lines.insert(index+1, SAFETY[language])
            text="\n".join(lines)+("\n" if text.endswith("\n") else "")
    path.write_text(text.replace("`",chr(96)),encoding="utf-8")
def main():
    count=0
    for src in sources():
        for lang in ("ru","en"):
            apply(ROOT/localized(src,lang),lang)
            count+=1
    print(f"Applied consistent engineering safety and terminology to {count} translated Markdown companions.")
if __name__=="__main__":main()
