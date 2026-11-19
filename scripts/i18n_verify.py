#!/usr/bin/env python3
"""Fail-closed structure checks for all generated RU/EN Markdown companions."""
from __future__ import annotations
import re, sys
from i18n_translate import ROOT, sources, localized, strip_nav
INLINE=re.compile(r"(?<!\\)`+[^`\n]+(?<!\\)`+")
TITLE=re.compile(r"(?m)^[ \t]*#{1,6}[ \t]+")
def code_fences(content):
    lines=content.splitlines(keepends=True)
    output=[];current=[];symbol=None;length=0
    for line in lines:
        m=re.match(r"^\s*(`{3,}|~{3,})",line)
        if m:
            mark=m.group(1)
            if symbol is None: symbol,length,current=mark[0],len(mark),[line]
            elif symbol==mark[0] and len(mark)>=length:
                current.append(line);output.append("".join(current))
                symbol,length,current=None,0,[]
            else:current.append(line)
        elif symbol is not None:current.append(line)
    if symbol is not None:raise ValueError("Unclosed code fence")
    return output
def main():
    failures=[];originals=sources()
    for path in originals:
        src=strip_nav((ROOT/path).read_text(encoding="utf-8"))
        for lang in ("ru","en"):
            target=ROOT/localized(path,lang)
            if not target.is_file():
                failures.append(f"{path}: missing {lang} file");continue
            out=target.read_text(encoding="utf-8")
            try:
                if code_fences(src)!=code_fences(out):
                    failures.append(f"{path}: code-fence content changed in {lang}")
            except ValueError as e:failures.append(f"{path}: {e}")
            for token in set(INLINE.findall(src)):
                if token not in out:
                    # In a reviewed English document, an inline code span
                    # containing natural-language Cyrillic may be translated;
                    # executable identifiers, paths and numeric literals
                    # must still be retained exactly.
                    if lang == "en" and "MLAD-I18N:EDITORIAL" in out and re.search(r"[А-Яа-яЁё]",token):
                        continue
                    failures.append(f"{path}: missing literal {token[:60]} in {lang}")
            if len(TITLE.findall(src))!=len(TITLE.findall(out)):
                failures.append(f"{path}: Markdown heading count differs in {lang}")
            if not any(marker in out for marker in (
                f"[Original / Оригинал]({path.name})",
                f"[Source / Оригинал]({path.name})",
                f"[Исходный текст / Original]({path.name})",
                f"[Original]({path.name})",
                f"[Исходный текст]({path.name})",
            )):
                failures.append(f"{path}: original navigation missing in {lang}")
        if "MLAD-I18N:NAV:START" not in (ROOT/path).read_text(encoding="utf-8"):
            failures.append(f"{path}: original language switcher missing")
    if not (ROOT/"docs/translation_coverage.md").is_file():
        failures.append("translation coverage register absent")
    if failures:
        for err in failures[:100]:print("I18N ERROR:",err,file=sys.stderr)
        raise SystemExit(f"Translation verification FAILED ({len(failures)} problems)")
    print(f"Translation verification OK: {len(originals)} sources + {len(originals)*2} translated docs")
if __name__=="__main__":main()
