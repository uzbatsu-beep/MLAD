#!/usr/bin/env python3
"""Offline RU/EN Markdown companion generator. Originals remain authoritative."""
from __future__ import annotations
import argparse, hashlib, json, os, re
from functools import lru_cache
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
ROOT_DOCS=("README.md","ACCESSIBILITY.md","CONTRIBUTING.md","SECURITY.md","CODE_OF_CONDUCT.md",".github/PULL_REQUEST_TEMPLATE.md")
NAV_START="<!-- MLAD-I18N:NAV:START -->"
NAV_END="<!-- MLAD-I18N:NAV:END -->"
NAV_RE=re.compile(re.escape(NAV_START)+r"[\s\S]*?"+re.escape(NAV_END)+r"\n*",re.M)
INLINES=re.compile(r"(`+[^`\n]+`+|!?\[[^\]\n]+\]\([^\n)]*\)|https?://[^\s)]+|<!--.*?-->|</?[A-Za-z][^>]*>)")
FENCE=re.compile(r"^\s*(`{3,}|~{3,})")
IDENT=re.compile(r"^[A-Za-z0-9_./@:\-+]+$")
def sources():
    docs=[Path(p) for p in ROOT_DOCS if (ROOT/p).is_file()]
    docs += [p.relative_to(ROOT) for p in (ROOT/"docs").rglob("*.md")
             if p.is_file() and "_i18n_staging" not in p.parts]
    return sorted({p for p in docs if not p.name.endswith((".ru.md",".en.md"))
                   and p.name!="translation_coverage.md"},key=str)
def localized(p,lang):
    return p.with_name(p.stem+f".{lang}.md")
def strip_nav(text):
    return NAV_RE.sub("",text,count=1)
def source_language(text,target):
    c=len(re.findall("[А-Яа-яЁё]",text))
    e=len(re.findall("[A-Za-z]",text))
    if target=="en": return "ru" if c>=2 else None
    return "en" if c==0 and e>=3 else None
class Engine:
    def __init__(self):
        import argostranslate.package as package
        import argostranslate.translate as translate
        package.update_package_index()
        avail=package.get_available_packages()
        installed={(p.from_code,p.to_code) for p in package.get_installed_packages()}
        for fr,to in (("ru","en"),("en","ru")):
            if (fr,to) not in installed:
                options=[p for p in avail if p.from_code==fr and p.to_code==to]
                if not options: raise RuntimeError(f"Argos model missing {fr}->{to}")
                package.install_from_path(options[0].download())
        self.models={}
        for fr,to in (("ru","en"),("en","ru")):
            src,dst=translate.get_language_from_code(fr),translate.get_language_from_code(to)
            if src is None or dst is None: raise RuntimeError(f"Uninstalled model: {fr}->{to}")
            self.models[(fr,to)]=src.get_translation(dst)
    @lru_cache(maxsize=50000)
    def natural(self,text,target):
        if not text.strip() or IDENT.fullmatch(text.strip()): return text
        fr=source_language(text,target)
        if fr is None or len(text.strip())<3: return text
        lead=text[:len(text)-len(text.lstrip())]
        tail=text[len(text.rstrip()):] if text.rstrip() else ""
        value=text.strip()
        chunks=[]
        if len(value)<=1200: chunks=[value]
        else:
            for sentence in re.split(r"(?<=[.!?;:])\s+(?=\S)",value):
                if len(sentence)<=1200: chunks.append(sentence)
                else:
                    buf=""
                    for word in sentence.split():
                        if len(buf)+len(word)>1000 and buf:
                            chunks.append(buf);buf=word
                        else: buf=(buf+" "+word).strip()
                    if buf: chunks.append(buf)
        return lead+" ".join(self.models[(fr,target)].translate(c) for c in chunks)+tail
    def inline(self,text,target,source,docs):
        def translate_link(token):
            m=re.fullmatch(r"(!?)\[([^\]\n]+)\]\(([^\n)]*)\)",token)
            if not m: return token
            prefix,label,uri=m.groups()
            label=self.inline(label,target,source,docs)
            if uri and not re.match(r"(?:[a-z][a-z0-9+.-]*:|//|#)",uri,re.I):
                rel=uri.split("#")[0].split("?")[0]
                # Deep links keep their original anchor because headings may change.
                if "#" not in uri and rel.endswith(".md"):
                    dest=Path(os.path.normpath((source.parent/rel).as_posix()))
                    if dest in docs: uri=rel[:-3]+f".{target}.md"
            return f"{prefix}[{label}]({uri})"
        out=[];last=0
        for m in INLINES.finditer(text):
            out.append(self.natural(text[last:m.start()],target))
            token=m.group(0)
            out.append(translate_link(token) if token.startswith(("[","![")) else token)
            last=m.end()
        out.append(self.natural(text[last:],target))
        return "".join(out)
    def line(self,text,target,source,docs):
        if not text.strip() or text.lstrip().startswith(("<!--","```","~~~")):return text
        if re.match(r"^\s*(?:\|?\s*:?-{3,}:?\s*\|?)+\s*$",text):return text
        if any(ch in text for ch in ("│","├","└","╰","═")):return text
        if re.match(r"^\s*(?:<[/!]?[A-Za-z]|(?:import|export|const|let|fn|pub|def|class)\s)",text):return text
        if text.lstrip().startswith("|"):
            return "|".join(self.inline(c,target,source,docs)
                            for c in re.split(r"(?<!\\)\|",text))
        p=re.match(r"^(\s*(?:(?:#{1,6}\s+)|(?:>\s*)+|(?:[-+*]|\d+\.)\s+(?:\[[ xX]\]\s+)?))",text)
        return (p.group(0)+self.inline(text[p.end():],target,source,docs) if p
                else self.inline(text,target,source,docs))
    def document(self,raw,target,source,docs):
        out=[];fence=None;size=0
        for line in strip_nav(raw).splitlines(keepends=True):
            body=line.rstrip("\r\n");end=line[len(body):]
            m=FENCE.match(body)
            if m:
                seq=m.group(1)
                if fence is None: fence,size=seq[0],len(seq)
                elif seq[0]==fence and len(seq)>=size: fence,size=None,0
                out.append(line)
            elif fence is not None or (body.startswith("    ") and
                not body.lstrip().startswith(("-","*","1."))):
                out.append(line)
            else:out.append(self.line(body,target,source,docs)+end)
        notice=("автоматический черновик — сверяйте инженерные утверждения с оригиналом и OEM документацией."
                if target=="ru" else
                "machine-generated draft — verify engineering claims against the original and OEM manuals.")
        header=(f"<!-- Generated {target} companion for {source}; review required. -->\n"
                f"> **{('Русский' if target=='ru' else 'English')}** · "
                f"[Русский]({localized(source,'ru').name}) · "
                f"[English]({localized(source,'en').name}) · "
                f"[Original / Оригинал]({source.name})\n"
                f"> **Translation review / Проверка:** {notice}\n\n")
        return header+"".join(out)
def main():
    a=argparse.ArgumentParser()
    a.add_argument("--shard",type=int,default=0)
    a.add_argument("--total",type=int,default=1)
    a.add_argument("--output",type=Path,required=True)
    opt=a.parse_args()
    if not 0<=opt.shard<opt.total:raise SystemExit("Invalid shard")
    docs=sources(); subset=[p for i,p in enumerate(docs) if i%opt.total==opt.shard]
    print(f"Inputs={len(docs)}, shard={opt.shard}/{opt.total}, selected={len(subset)}",flush=True)
    engine=Engine(); output=[]
    for path in subset:
        raw=strip_nav((ROOT/path).read_text(encoding="utf-8"))
        for language in ("ru","en"):
            dest=opt.output/localized(path,language)
            dest.parent.mkdir(parents=True,exist_ok=True)
            dest.write_text(engine.document(raw,language,path,set(docs)),encoding="utf-8")
        output.append({"source":path.as_posix(),"sha256":hashlib.sha256(raw.encode()).hexdigest()})
        print(f"Translated {path} ({len(raw)} source chars)",flush=True)
    (opt.output/f"_manifest_shard_{opt.shard}.json").write_text(
        json.dumps(output,ensure_ascii=False,indent=2),encoding="utf-8")
if __name__=="__main__":main()
