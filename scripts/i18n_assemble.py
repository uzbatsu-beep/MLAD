#!/usr/bin/env python3
"""Verify translation artifacts and add a language switcher to source Markdown."""
from __future__ import annotations
import json, shutil
from pathlib import Path
from i18n_translate import sources, localized, strip_nav, NAV_START, NAV_END, ROOT

def main():
    stage=ROOT/"_i18n_staging"
    originals=sources()
    manifests=[]
    for path in sorted(stage.glob("_manifest_shard_*.json")):
        manifests += json.loads(path.read_text(encoding="utf-8"))
    expected={p.as_posix() for p in originals}
    found={item["source"] for item in manifests}
    if found!=expected or len(manifests)!=len(originals):
        raise SystemExit(f"Incomplete bilingual coverage, expected={len(expected)}, found={len(found)}, missing={expected-found}")
    for original in originals:
        for lang in ("ru","en"):
            variant=localized(original,lang)
            generated=stage/variant
            if not generated.is_file() or generated.stat().st_size<100:
                raise SystemExit(f"Missing companion {generated}")
            dest=ROOT/variant
            dest.parent.mkdir(parents=True,exist_ok=True)
            shutil.copyfile(generated,dest)
        path=ROOT/original
        text=strip_nav(path.read_text(encoding="utf-8"))
        nav=(f"{NAV_START}\n"
             f"> 🌐 **Язык / Language:** "
             f"[Русский]({localized(original,'ru').name}) · "
             f"[English]({localized(original,'en').name}) · "
             f"[Original / Исходный текст]({original.name}). "
             f"**Machine translation / Машинный перевод: editorial review required / необходима редакторская проверка.**\n"
             f"{NAV_END}\n\n")
        path.write_text(nav+text,encoding="utf-8")
    report=ROOT/"docs"/"translation_coverage.md"
    rows=[]
    for src in originals:
        def relative(p):
            if src.parts[0]=="docs":
                return str(p.relative_to("docs"))
            return "../"+p.as_posix()
        rows.append(f"| [{src.as_posix()}]({relative(src)}) | "
                    f"[Русский]({relative(localized(src,'ru'))}) | "
                    f"[English]({relative(localized(src,'en'))}) |")
    report.write_text(
        "# MLAD — translation coverage / Покрытие переводом\n\n"
        "> **Status:** generated with local/offline open-source translation models. "
        "All results require an engineer's editorial review; original source is authoritative.\n\n"
        "> **Статус:** автоматический локальный перевод. "
        "Требуется редакторская проверка профильным инженером; исходный файл приоритетен.\n\n"
        f"**Markdown sources:** {len(originals)}. **RU/EN companions:** {len(originals)*2}. "
        f"**Total Markdown including this register:** {len(originals)*3+1}.\n\n"
        "| Original / Оригинал | Русский | English |\n"
        "| --- | --- | --- |\n" + "\n".join(rows) + "\n\n"
        "## Review gates / Критерии проверки\n\n"
        "1. Match each localized file to its source and preserve all fenced code, URLs, "
        "command invocations, paths and exact technical identifiers. / "
        "Сверить каждый перевод с исходником, сохранить код, адреса, команды и ссылки.\n"
        "2. Verify safety clauses, CIOPRG/IONAME read-only, HSES/FTP/native writer BLOCKED. / "
        "Проверить предупреждения о безопасности и запреты на запись/подключение.\n"
        "3. Independently review terminology and examples for DX100/DX200/YRC1000. / "
        "Проверить терминологию и примеры по руководствам Yaskawa.\n"
        "4. Until human review, translated files are drafts, not an independently validated OEM source. / "
        "До рецензирования переводы являются черновиками, а не подтверждённой OEM документацией.\n",
        encoding="utf-8")
    print(f"Completed {len(originals)} originals, {2*len(originals)} translations.",flush=True)

if __name__=="__main__":
    main()
