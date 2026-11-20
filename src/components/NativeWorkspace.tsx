import { useEffect, useMemo, useRef, useState } from "react";
import { LadderCircuit } from "./LadderCircuit.tsx";
import { AdvancedBlockDiagram } from "./AdvancedBlockDiagram.tsx";
import type { AdvancedBlock, Preset } from "../ladder/advancedModel.ts";
import { semanticToExpression, relayText, type SemanticNode } from "../ladder/graphModel.ts";
import { invoke } from "@tauri-apps/api/core";
import {groupDiagnostics} from "../ladder/diagnosticsModel.ts";
import {oemManualEntry} from "../ladder/oemReference.ts";
import "./native-workspace.css";

type Controller = "DX100" | "DX200" | "YRC1000";
type Encoding = "Utf8" | "Utf8Bom" | "OpaqueBytes";
type SyntaxClass = "Recognized" | "TypedCandidate" | "Unsupported" |
  "Malformed" | "Opaque" | "PostEndData" | "Structural";
type SyntaxCoverage = {
  total_lines: number;
  recognized: number;
  typed_candidate: number;
  unsupported: number;
  malformed: number;
  opaque: number;
  post_end_data: number;
  structural: number;
  all_source_lines_counted: boolean;
  semantics_verified: boolean;
};
type Diagnostic = {
  code: string;
  severity: "Warning" | "Error";
  line: number;
  byte_offset: number;
  message: string;
};
type TypedCandidatePreview={
  instruction:{
    mnemonic:string;syntax_valid:boolean;line:number;
    operands:{spelling:string;kind:string;span:{start:number;end:number}}[];
  };
  semantics_verified:boolean;
  controller_form_verified:boolean;
};
function getTypedCandidate(kind:unknown):TypedCandidatePreview|null {
  if(!kind||typeof kind!=="object"||!("TypedCandidate" in kind))return null;
  const value=(kind as {TypedCandidate:unknown}).TypedCandidate;
  if(!value||typeof value!=="object"||!("instruction" in value))return null;
  const t=value as TypedCandidatePreview;
  return t.instruction&&typeof t.instruction.mnemonic==="string"&&
    Array.isArray(t.instruction.operands)?t:null;
}
type PreviewLine = {
  line: number;
  body_span: {start:number;end:number};
  syntax_class: SyntaxClass;
  ending: string;
  kind: unknown;
  display_text: string;
};
type SemanticDiagnostic = {
  code: string;
  level: "Warning" | "Error";
  line: number;
  message: string;
};
type SemanticRung = {
  number: number;
  first_line: number;
  output_line: number;
  output: number;
  inputs: number[];
  expression_preview: string;
  expression_root: number;
  section: "Unspecified" | "System" | "User";
};
type SemanticPreview = {
  nodes: SemanticNode[];
  nodes_truncated: boolean;
  node_count: number;
  rung_count: number;
  reference_count: number;
  diagnostic_count: number;
  unsupported_segments: number;
  subset_analysis_complete: boolean;
  system_user_boundaries_verified: boolean;
  edited_export_allowed: boolean;
  end_seen: boolean;
  rungs: SemanticRung[];
  references: { symbol: string; sites: { line: number; operation: string }[] }[];
  part_markers: { line: number; section: string }[];
  diagnostics: SemanticDiagnostic[];
  rungs_truncated: boolean;
  references_truncated: boolean;
  diagnostics_truncated: boolean;
};
type AdvancedSourceBlock = {
  kind:"Timer"|"Counter"|"GroupCopy";
  line:number;end_line:number;
  operands:string[];
  count_or_enable:string|null;
  reset:string|null;
  count_or_enable_inverted:boolean;
  reset_inverted:boolean;
  output:string|null;
  example_pattern_recognized:boolean;
};
type AdvancedSourceReport = {
  source_sha256:string;
  blocks:AdvancedSourceBlock[];
  warnings:{code:string;line:number;message:string}[];
  blocks_truncated:boolean;
  original_only:boolean;
  native_export_allowed:boolean;
  hardware_test_authorized:boolean;
};
function parseManualPreset(text:string):Preset|null{
  if(/^M[0-9]{3}$/.test(text))return {kind:"register",address:text};
  if(/^[0-9]+$/.test(text)){
    const n=Number(text);
    if(Number.isInteger(n)&&n>=0&&n<=65535)return {kind:"literal",value:n};
  }
  return null;
}
function toReadOnlyAdvanced(b:AdvancedSourceBlock):AdvancedBlock|null{
  if(!b.example_pattern_recognized)return null;
  if(b.kind==="GroupCopy" && b.operands.length===2)return {
    kind:"GROUP",id:"source-"+b.line,source:b.operands[0],
    destination:b.operands[1],
  };
  if(b.operands.length!==2||!b.count_or_enable||!b.output)return null;
  const preset=parseManualPreset(b.operands[1]);
  if(!preset)return null;
  const first={
    kind:"contact" as const,id:"src-c-"+b.line,relay:b.count_or_enable,
    normallyClosed:b.count_or_enable_inverted,
  };
  if(b.kind==="Timer")return {
    kind:"TMR",id:"src-"+b.line,current:b.operands[0],preset,
    output:b.output,enable:first,
  };
  if(b.kind==="Counter" && b.reset)return {
    kind:"CNT",id:"src-"+b.line,current:b.operands[0],preset,
    output:b.output,count:first,
    reset:{kind:"contact",id:"src-r-"+b.line,relay:b.reset,
      normallyClosed:b.reset_inverted},
  };
  return null;
}

type NativeSection = {line:number;category:string;label:string};
type SearchHit = {line:number;preview:string};
type SearchResponse = {source_sha256:string;hits:SearchHit[];truncated:boolean};
type NativeFilterHit = {
  line:number;byte_offset:number;syntax_class:SyntaxClass;preview:string;
};
type NativeFilterPage = {
  source_sha256:string;line_count:number;offset:number;total_matches:number;
  next_offset:number|null;hits:NativeFilterHit[];
};
type StaticCandidateSite={
  spelling:string;kind:string;span:{start:number;end:number};
  role:"Source"|"ShiftAmount"|"Destination";
  possible_previous_definition:number|null;
};
type StaticCandidateEvent={
  line:number;mnemonic:string;syntax_shape_valid:boolean;controller_form_verified:boolean;
  semantics_verified:boolean;execution_condition_verified:boolean;arithmetic_flags_verified:boolean;
  sites:StaticCandidateSite[];possible_prior_lines:number[];
};
type StaticDependencyAudit={
  source_sha256:string;total_candidates:number;valid_shape_candidates:number;
  invalid_shape_candidates:number;possible_source_order_links:number;continuity_barriers:number;
  events_truncated:boolean;events:StaticCandidateEvent[];
  semantics_verified:boolean;scan_order_verified:boolean;arithmetic_flags_verified:boolean;
  native_write_allowed:boolean;
};
type StaticAccessKind = "Read"|"Write"|"Unknown";
type StaticAccessSite = {
  line:number;mnemonic:string;operand_index:number|null;symbol:string;
  span:{start:number;end:number};access:StaticAccessKind;
  basis:"DocumentedOperandRole"|"InvalidForm"|"UnverifiedFamily"|
    "UnknownOpcodeOrStructure"|"StatefulOpcodeUnresolved";
  controller_form_verified:boolean;
};
type StaticAccessOverview = {
  source_sha256:string;line_count:number;
  total_sites:number;read_sites:number;write_sites:number;unknown_sites:number;
  unmodeled_lines:number;skipped_literal_operands:number;post_end_lines:number;
  sites_truncated:boolean;sites:StaticAccessSite[];
  execution_verified:boolean;scan_order_verified:boolean;
  address_aliases_verified:boolean;write_permission_verified:boolean;
  native_write_allowed:boolean;
};
type StaticAccessPage = {
  source_sha256:string;line_count:number;offset:number;total_matches:number;
  next_offset:number|null;hits:StaticAccessSite[];
  execution_verified:boolean;write_permission_verified:boolean;
};
type Report = {
  static_access: StaticAccessOverview;
  advanced: AdvancedSourceReport;
  sections: NativeSection[];
  semantic: SemanticPreview;
  dataflow: StaticDependencyAudit;
  controller: Controller;
  encoding: Encoding;
  byte_length: number;
  source_sha256: string;
  line_count: number;
  syntax_coverage: SyntaxCoverage;
  recognized_instructions: number;
  typed_candidate_count: number;
  typed_valid_forms: number;
  typed_invalid_forms: number;
  typed_mnemonic_counts: Record<string,number>;
  unsupported_lines: number;
  end_line: number|null;
  post_end_line_count: number;
  post_end_sections: number;
  post_end_unverified_lines: number;
  diagnostics: Diagnostic[];
  lines: PreviewLine[];
  preview_truncated: boolean;
  m_headers: { marker_line: number; data_lines: number[]; complete: boolean }[];
  edited_export_allowed: boolean;
  system_user_boundaries_verified: boolean;
  original_round_trip_only: boolean;
};

type IonameEntry = {section:string;ordinal:number;line:number;slot:number;label:string};
type IonameReport = {
  source_sha256:string; byte_length:number; encoding:string; header_valid:boolean;
  entries:IonameEntry[]; entries_truncated:boolean; warnings:string[];
  original_only:boolean;native_write_allowed:boolean;relay_mapping_verified:boolean;
};
type Annotations = {
  rungs:Record<string,string>;addresses:Record<string,string>;ioname:Record<string,string>;
};
type SidecarBookmark = {line:number;label:string};
type SavedSidecarNotes = {annotations:Annotations;bookmarks:SidecarBookmark[]};
type OfflineSidecar = {
  format:"MLAD_OFFLINE_SIDECAR";
  schema_version:1;
  source:{
    kind:"CIOPRG.LST";controller:Controller;sha256:string;
    byte_length:number;line_count:number;
  };
  annotations:Annotations;
  bookmarks:SidecarBookmark[];
};
type OfflineSidecarRead = {sidecar:OfflineSidecar;revision:string};
type OfflineSidecarSaved = {revision:string};
type EditKey = "rungs"|"addresses"|"ioname";
type EditTarget = {type:EditKey;key:string;caption:string};
const blankAnnotations=():Annotations=>({rungs:{},addresses:{},ioname:{}});
function loadLocalAnnotations(hash:string):Annotations {
  try {
    const data=window.localStorage.getItem("mlad-annotations:"+hash);
    if (!data || data.length>400_000) return blankAnnotations();
    const source:unknown=JSON.parse(data);
    if(!source || typeof source!=="object")return blankAnnotations();
    const parts=source as Partial<Annotations>;
    const safe=(v:unknown):Record<string,string> => {
      if(!v || typeof v!=="object" || Array.isArray(v))return {};
      return Object.fromEntries(Object.entries(v).filter(([key,value])=>
        key.length<160 && typeof value==="string" && value.length<=200));
    };
    return {rungs:safe(parts.rungs),addresses:safe(parts.addresses),ioname:safe(parts.ioname)};
  } catch{return blankAnnotations();}
}
type ReadPage = {
  source_sha256: string;
  start_line: number;
  line_count: number;
  lines: PreviewLine[];
};
const PAGE_SIZE = 250;

const panel: React.CSSProperties = {
  background: "#252526",
  border: "1px solid #454952",
  padding: "16px",
  borderRadius: "8px",
};
const field: React.CSSProperties = {
  color: "#e0e4eb",
  background: "#171b20",
  border: "1px solid #606875",
  borderRadius: "4px",
  padding: "8px",
};
const btn: React.CSSProperties = {
  color: "#edf4fa", background: "#354b66", border: "1px solid #61758e",
  borderRadius: "4px", padding: "8px 12px", cursor: "pointer",
};

function lineCategory(kind: unknown): string {
  if (typeof kind === "string") return kind;
  if (typeof kind === "object" && kind !== null) {
    return Object.keys(kind)[0] || "Unknown";
  }
  return "Unknown";
}


function syntaxClassLabel(line: PreviewLine): string {
  if(lineCategory(line.kind)==="PostEndSection") return "Раздел данных";
  if(lineCategory(line.kind)==="PostEndData") return "Данные после END";
  if(lineCategory(line.kind)==="PostEndOpaque") return "Неизвестно после END";
  switch(line.syntax_class) {
    case "Recognized": return "Распознано*";
    case "TypedCandidate": return "Типизировано*";
    case "Unsupported": return "Не поддерживается";
    case "Malformed": return "Ошибка формы";
    case "Opaque": return "Неизвестно";
    case "PostEndData": return "Данные после END";
    case "Structural": return "Структура/комм.";
  }
}

/** Primary document workspace, mounted in the main app window — never a modal. */
export function NativeWorkspace() {
  const [path,setPath]=useState("");
  const [controller,setController]=useState<Controller>("YRC1000");
  const [report,setReport]=useState<Report|null>(null);
  const [busy,setBusy]=useState(false);
  const [filePickerBusy,setFilePickerBusy]=useState<"cioprg"|"ioname"|null>(null);
  const [issue,setIssue]=useState("");
  const [tab,setTab]=useState<"source"|"diagram"|"blocks"|"ioname">("source");
  const [pageStart,setPageStart]=useState(1);
  const [pageLines,setPageLines]=useState<PreviewLine[]>([]);
  const [pageBusy,setPageBusy]=useState(false);
  const [selectedLine,setSelectedLine]=useState<number|null>(null);
  const [goto,setGoto]=useState("");
  const [rung,setRung]=useState(1);
  const [selectedBlockLine,setSelectedBlockLine]=useState<number|null>(null);
  const [zoom,setZoom]=useState(1);
  const [blockZoom,setBlockZoom]=useState(1);
  const [ionamePath,setIonamePath]=useState("");
  const [ioname,setIoname]=useState<IonameReport|null>(null);
  const [ionameQuery,setIonameQuery]=useState("");
  const [ionameBusy,setIonameBusy]=useState(false);
  const [annotations,setAnnotations]=useState<Annotations>(blankAnnotations);
  const [openedSourcePath,setOpenedSourcePath]=useState("");
  const [sidecarPath,setSidecarPath]=useState("");
  const [sidecarRevision,setSidecarRevision]=useState<string|null>(null);
  const [sidecarBusy,setSidecarBusy]=useState(false);
  const [sidecarDirty,setSidecarDirty]=useState(false);
  const [bookmarks,setBookmarks]=useState<SidecarBookmark[]>([]);
  const [savedSidecarNotes,setSavedSidecarNotes]=useState<SavedSidecarNotes|null>(null);
  const [bookmarkLabel,setBookmarkLabel]=useState("");
  const [editing,setEditing]=useState<EditTarget|null>(null);
  const [commentText,setCommentText]=useState("");
  const [linkEntry,setLinkEntry]=useState<IonameEntry|null>(null);
  const [linkAddress,setLinkAddress]=useState("");
  const [search,setSearch]=useState("");
  const [hits,setHits]=useState<SearchHit[]>([]);
  const [searchTruncated,setSearchTruncated]=useState(false);
  const [searchBusy,setSearchBusy]=useState(false);
  const [sourceClass,setSourceClass]=useState<""|SyntaxClass>("");
  const [sourceOpcode,setSourceOpcode]=useState("");
  const [sourceAddress,setSourceAddress]=useState("");
  const [sourceFilterResult,setSourceFilterResult]=useState<NativeFilterPage|null>(null);
  const [sourceFilterBusy,setSourceFilterBusy]=useState(false);
  const [accessKind,setAccessKind]=useState<""|StaticAccessKind>("");
  const [accessSymbol,setAccessSymbol]=useState("");
  const [accessResult,setAccessResult]=useState<StaticAccessPage|null>(null);
  const [accessBusy,setAccessBusy]=useState(false);
  const accessRequestSeq=useRef(0);
  const nativeRequestEpoch=useRef(0);
  const activeSourceLineRef=useRef<HTMLButtonElement|null>(null);
  const selectedDiagnosticsRef=useRef<HTMLDivElement|null>(null);
  const [filter,setFilter]=useState<"all"|"Error"|"Warning">("all");
  const [groupedDiagnostics,setGroupedDiagnostics]=useState(true);
  const [json,setJson]=useState("");

  const visibleRungs=useMemo(()=>{
    if(!report)return [];
    return report.semantic.rungs.filter(sourceRung=>sourceRung.number===rung).map(sourceRung=>{
      try{return {rung:sourceRung,expr:semanticToExpression(report.semantic.nodes,sourceRung.expression_root),error:""};}
      catch(error){return {rung:sourceRung,expr:null,error:String(error)};}
    });
  },[report,rung]);
  const activeBlock=useMemo(()=>
    report?.advanced.blocks.find(block=>block.line===selectedBlockLine)??null,
  [report,selectedBlockLine]);
  const selectedRung=useMemo(()=>{
    if(!report||selectedLine===null)return null;
    return report.semantic.rungs.find(candidate=>
      selectedLine>=candidate.first_line&&selectedLine<=candidate.output_line)??null;
  },[report,selectedLine]);
  const selectedRelatedBlock=useMemo(()=>{
    if(!report||selectedLine===null)return null;
    return report.advanced.blocks.find(candidate=>
      selectedLine>=candidate.line&&selectedLine<=candidate.end_line)??null;
  },[report,selectedLine]);
  useEffect(()=>{
    if(tab==="source"&&selectedLine!==null)activeSourceLineRef.current?.focus();
  },[tab,selectedLine,pageStart,pageLines]);
  const selectedTyped=useMemo(()=>{
    if(selectedLine===null)return null;
    return getTypedCandidate(pageLines.find(l=>l.line===selectedLine)?.kind);
  },[selectedLine,pageLines]);
  const selectedStatic=useMemo(()=>{
    if(!report||selectedLine===null)return null;
    // Same inspected immutable source snapshot as the paginated source.
    if(report.dataflow.source_sha256!==report.source_sha256)return null;
    return report.dataflow.events.find(e=>e.line===selectedLine)??null;
  },[report,selectedLine]);
  const filteredIoname=useMemo(()=>{
    if(!ioname)return [];
    const query=ionameQuery.trim().toLowerCase();
    return ioname.entries.filter(entry=>{
      const key=ioname.source_sha256+":"+entry.line+":"+entry.slot;
      const label=annotations.ioname[key]??entry.label;
      return !query||label.toLowerCase().includes(query)||
        entry.section.toLowerCase().includes(query)||String(entry.ordinal).includes(query);
    }).slice(0,300);
  },[ioname,ionameQuery,annotations.ioname]);
  const diags=useMemo(()=>{
    if(!report)return [];
    return [
      ...report.diagnostics.map(d=>({severity:d.severity,line:d.line,code:d.code,message:d.message})),
      ...report.semantic.diagnostics.map(d=>({severity:d.level,line:d.line,code:d.code,message:d.message})),
    ].filter(x=>filter==="all"||x.severity===filter).sort((a,b)=>a.line-b.line);
  },[report,filter]);
  const diagnosticGroups=useMemo(()=>groupDiagnostics(diags),[diags]);
  const selectedAccessSites=useMemo(()=>{
    if(!report||selectedLine===null)return [];
    return report.static_access.sites.filter(x=>x.line===selectedLine).slice(0,16);
  },[report,selectedLine]);
  const selectedLineDiagnostics=useMemo(()=>{
    if(!report||selectedLine===null)return [];
    return [
      ...report.diagnostics.map(d=>({severity:d.severity,line:d.line,code:d.code,message:d.message})),
      ...report.semantic.diagnostics.map(d=>({severity:d.level,line:d.line,code:d.code,message:d.message})),
    ].filter(d=>d.line===selectedLine).slice(0,20);
  },[report,selectedLine]);

  async function openSource(selectedPath:string=path){
    if(sidecarBusy) {
      setIssue("Сначала дождитесь завершения чтения/записи sidecar.");
      return;
    }
    if(report&&sidecarDirty) {
      setIssue("Есть несохранённые изменения sidecar: сохраните или явно отмените их перед открытием другого исходника.");
      return;
    }
    const epoch=++nativeRequestEpoch.current;
    setBusy(true);setIssue("");setReport(null);setHits([]);setJson("");setEditing(null);
    setSourceFilterResult(null);setSourceFilterBusy(false);
    accessRequestSeq.current++;
    setAccessResult(null);setAccessBusy(false);
    setAccessKind("");setAccessSymbol("");
    setSearchBusy(false);setPageBusy(false);
    setSidecarBusy(false);setSidecarRevision(null);
    setSidecarDirty(false);setBookmarks([]);setBookmarkLabel("");
    setSavedSidecarNotes(null);
    setOpenedSourcePath("");
    try{
      const value=await invoke<Report>("cmd_inspect_cioprg",{filePath:selectedPath.trim(),controller});
      if(epoch!==nativeRequestEpoch.current)return;
      if(value.syntax_coverage.total_lines!==value.line_count ||
          !value.syntax_coverage.all_source_lines_counted ||
          value.syntax_coverage.semantics_verified){
        throw Error("Несовместимый отчёт покрытия или неполный снимок");
      }
      if(value.static_access.source_sha256!==value.source_sha256 ||
         value.static_access.line_count!==value.line_count ||
         value.static_access.execution_verified ||
         value.static_access.scan_order_verified ||
         value.static_access.write_permission_verified ||
         value.static_access.native_write_allowed) {
        throw Error("Недопустимый или неполный отчёт READ/WRITE/UNKNOWN");
      }
      setPath(selectedPath);
      setOpenedSourcePath(selectedPath.trim());
      setSidecarPath(selectedPath.trim().replace(/\.lst$/i,".mlad.json"));
      setReport(value);
      setPageLines(value.lines.slice(0,PAGE_SIZE));
      setPageStart(1);setSelectedLine(null);
      setRung(value.semantic.rungs[0]?.number??1);
      setSelectedBlockLine(value.advanced.blocks[0]?.line??null);setTab("source");
      const cached=loadLocalAnnotations(value.source_sha256);
      setAnnotations(cached);
      setSavedSidecarNotes({annotations:cached,bookmarks:[]});
      setIoname(null);setLinkEntry(null);
    }catch(error){if(epoch===nativeRequestEpoch.current)setIssue(String(error));}
    finally{if(epoch===nativeRequestEpoch.current)setBusy(false);}
  }

  /** Native desktop OPEN dialog; no save dialog, native writer, FTP or HSES. */
  async function chooseFile(kind:"cioprg"|"ioname"){
    if(filePickerBusy||busy||ionameBusy||sidecarBusy)return;
    if(kind==="cioprg"&&sidecarDirty){
      setIssue("Сохраните или отмените изменения sidecar перед выбором другого CIOPRG.");
      return;
    }
    setFilePickerBusy(kind);setIssue("");
    try{
      const chosen=await invoke<string|null>("cmd_pick_native_file",{kind});
      if(!chosen)return; // User cancelled; preserve previous file and annotations.
      if(kind==="cioprg")await openSource(chosen);
      else await importIoname(chosen);
    }catch(error){setIssue("Не удалось выбрать файл: "+String(error));}
    finally{setFilePickerBusy(null);}
  }
  const shortName=(filePath:string):string=>filePath.split(/[\\/]/).filter(Boolean).at(-1)??"Файл не выбран";

  async function showLine(line:number){
    if(!report||pageBusy||!Number.isInteger(line)||line<1||line>report.line_count)return;
    const start=Math.floor((line-1)/PAGE_SIZE)*PAGE_SIZE+1;
    const epoch=nativeRequestEpoch.current;
    setIssue("");setTab("source");
    if(start+PAGE_SIZE-1<=report.lines.length){
      setPageLines(report.lines.slice(start-1,start-1+PAGE_SIZE));
      setPageStart(start);setSelectedLine(line);return;
    }
    setPageBusy(true);
    try{
      const value=await invoke<ReadPage>("cmd_read_cioprg_page",{
        filePath:path.trim(),controller:report.controller,expectedSha256:report.source_sha256,startLine:start,
      });
      if(epoch!==nativeRequestEpoch.current)return;
      if(value.source_sha256!==report.source_sha256||value.line_count!==report.line_count||value.start_line!==start){
        throw Error("Контрольная сумма исходного файла изменилась");
      }
      setPageLines(value.lines);setPageStart(start);setSelectedLine(line);
    }catch(error){if(epoch===nativeRequestEpoch.current)setIssue(String(error));}
    finally{if(epoch===nativeRequestEpoch.current)setPageBusy(false);}
  }
  async function searchAll(){
    if(!report||searchBusy||search.trim().length<2)return;
    const epoch=nativeRequestEpoch.current;
    setSearchBusy(true);setIssue("");
    try{
      const value=await invoke<SearchResponse>("cmd_search_cioprg",{
        filePath:path.trim(),controller:report.controller,expectedSha256:report.source_sha256,query:search,
      });
      if(epoch!==nativeRequestEpoch.current)return;
      if(value.source_sha256!==report.source_sha256)throw Error("Снимок файла изменился");
      setHits(value.hits);setSearchTruncated(value.truncated);
    }catch(error){if(epoch===nativeRequestEpoch.current){setIssue(String(error));setHits([]);}}
    finally{if(epoch===nativeRequestEpoch.current)setSearchBusy(false);}
  }
  async function filterFullSource(offset:number=0){
    if(!report||sourceFilterBusy)return;
    if(!sourceClass&&!sourceOpcode.trim()&&!sourceAddress.trim()){
      setIssue("Укажите класс синтаксиса, opcode или адрес");return;
    }
    const epoch=nativeRequestEpoch.current;
    setIssue("");setSourceFilterBusy(true);
    try{
      const value=await invoke<NativeFilterPage>("cmd_filter_cioprg_lines",{
        filePath:path.trim(),controller:report.controller,
        expectedSha256:report.source_sha256,
        syntaxClass:sourceClass||null,opcode:sourceOpcode.trim()||null,
        address:sourceAddress.trim()||null,offset,
      });
      if(epoch!==nativeRequestEpoch.current)return;
      if(value.source_sha256!==report.source_sha256||
        value.line_count!==report.line_count||value.offset!==offset ||
        value.hits.some(hit=>hit.line<1||hit.line>report.line_count)){
        throw Error("Фильтр не соответствует открытому снимку CIOPRG");
      }
      setSourceFilterResult(value);
    }catch(error){
      if(epoch===nativeRequestEpoch.current){setIssue(String(error));setSourceFilterResult(null);}
    }finally{if(epoch===nativeRequestEpoch.current)setSourceFilterBusy(false);}
  }
  async function queryStaticAccess(offset:number=0){
    if(!report||accessBusy)return;
    const symbol=accessSymbol.trim();
    if(!accessKind&&!symbol){
      setIssue("Выберите READ/WRITE/UNKNOWN или точный адрес.");return;
    }
    const seq=++accessRequestSeq.current;
    const epoch=nativeRequestEpoch.current;
    setIssue("");setAccessBusy(true);
    try{
      const page=await invoke<StaticAccessPage>("cmd_query_static_access",{
        filePath:openedSourcePath,controller:report.controller,
        expectedSha256:report.source_sha256,
        accessKind:accessKind||null,symbol:symbol||null,offset,
      });
      if(epoch!==nativeRequestEpoch.current||seq!==accessRequestSeq.current)return;
      if(page.source_sha256!==report.source_sha256||
         page.line_count!==report.line_count||page.offset!==offset||
         page.execution_verified||page.write_permission_verified||
         page.hits.some(hit=>hit.line<1||hit.line>report.line_count||
           hit.span.start<0||hit.span.end>report.byte_length||
           hit.span.start>=hit.span.end)) {
        throw Error("Результат аудита не относится к открытому снимку CIOPRG");
      }
      setAccessResult(page);
    }catch(error){
      if(epoch===nativeRequestEpoch.current&&seq===accessRequestSeq.current) {
        setAccessResult(null);setIssue(String(error));
      }
    }finally{
      if(epoch===nativeRequestEpoch.current&&seq===accessRequestSeq.current){
        setAccessBusy(false);
      }
    }
  }

  function persistAnnotations(next:Annotations) {
    if(!report||sidecarBusy)return;
    setAnnotations(next);
    setSidecarDirty(true);
    try {
      window.localStorage.setItem("mlad-annotations:"+report.source_sha256,JSON.stringify(next));
    } catch(error) {
      setIssue("Локальное сохранение комментариев недоступно: "+String(error));
    }
  }
  async function loadPortableSidecar(replaceUnsaved:boolean=false){
    if(!report||!openedSourcePath||sidecarBusy||!sidecarPath.trim())return;
    if(sidecarDirty&&!replaceUnsaved){
      setIssue("Есть несохранённые изменения sidecar. Сохраните их перед загрузкой другого файла.");
      return;
    }
    const epoch=nativeRequestEpoch.current;
    setIssue("");setSidecarBusy(true);
    try{
      const loaded=await invoke<OfflineSidecarRead>("cmd_load_offline_sidecar",{
        filePath:sidecarPath.trim(),sourcePath:openedSourcePath,
        controller:report.controller,expectedSourceSha256:report.source_sha256,
      });
      if(epoch!==nativeRequestEpoch.current)return;
      if(loaded.sidecar.format!=="MLAD_OFFLINE_SIDECAR" ||
        loaded.sidecar.schema_version!==1 ||
        loaded.sidecar.source.sha256!==report.source_sha256 ||
        loaded.sidecar.source.controller!==report.controller ||
        loaded.sidecar.source.line_count!==report.line_count ||
        loaded.sidecar.source.byte_length!==report.byte_length) {
        throw Error("Sidecar не соответствует открытому CIOPRG");
      }
      setAnnotations(loaded.sidecar.annotations);
      setBookmarks(loaded.sidecar.bookmarks);
      setSavedSidecarNotes({
        annotations:loaded.sidecar.annotations,
        bookmarks:loaded.sidecar.bookmarks,
      });
      setSidecarRevision(loaded.revision);
      setSidecarDirty(false);setBookmarkLabel("");
      try {
        window.localStorage.setItem("mlad-annotations:"+report.source_sha256,
          JSON.stringify(loaded.sidecar.annotations));
      } catch {
        // Sidecar remains authoritative even when localStorage is unavailable.
      }
    }catch(error){if(epoch===nativeRequestEpoch.current)setIssue(String(error));}
    finally{if(epoch===nativeRequestEpoch.current)setSidecarBusy(false);}
  }
  async function savePortableSidecar(){
    if(!report||!openedSourcePath||sidecarBusy||!sidecarPath.trim())return;
    if(editing){setIssue("Сначала завершите редактирование комментария.");return;}
    const epoch=nativeRequestEpoch.current;
    const payload:OfflineSidecar={
      format:"MLAD_OFFLINE_SIDECAR",schema_version:1,
      source:{
        kind:"CIOPRG.LST",controller:report.controller,sha256:report.source_sha256,
        byte_length:report.byte_length,line_count:report.line_count,
      },annotations,bookmarks,
    };
    setIssue("");setSidecarBusy(true);
    try{
      const saved=await invoke<OfflineSidecarSaved>("cmd_save_offline_sidecar",{
        filePath:sidecarPath.trim(),sourcePath:openedSourcePath,
        controller:report.controller,expectedSourceSha256:report.source_sha256,
        sidecar:payload,expectedRevision:sidecarRevision,
      });
      if(epoch!==nativeRequestEpoch.current)return;
      setSidecarRevision(saved.revision);
      setSavedSidecarNotes({annotations:payload.annotations,bookmarks:payload.bookmarks});
      setSidecarDirty(false);
    }catch(error){if(epoch===nativeRequestEpoch.current)setIssue(String(error));}
    finally{if(epoch===nativeRequestEpoch.current)setSidecarBusy(false);}
  }
  function addBookmark(){
    if(!report||selectedLine===null||sidecarBusy)return;
    if(bookmarks.some(bookmark=>bookmark.line===selectedLine)){
      setIssue("Для этой строки уже есть закладка.");return;
    }
    if(bookmarks.length>=500){setIssue("Достигнут предел 500 закладок.");return;}
    const label=bookmarkLabel.trim()||"Строка L"+selectedLine;
    if(label.length>80){setIssue("Подпись закладки: не более 80 символов.");return;}
    setBookmarks([...bookmarks,{line:selectedLine,label}].sort((a,b)=>a.line-b.line));
    setBookmarkLabel("");setSidecarDirty(true);
  }
  function removeBookmark(line:number){
    if(sidecarBusy)return;
    setBookmarks(bookmarks.filter(x=>x.line!==line));
    setSidecarDirty(true);
  }
  function discardUnsavedSidecarNotes(){
    if(sidecarBusy||!report||!savedSidecarNotes)return;
    setAnnotations(savedSidecarNotes.annotations);
    setBookmarks(savedSidecarNotes.bookmarks);
    setSidecarDirty(false);setBookmarkLabel("");setIssue("");cancelEdit();
    try {
      window.localStorage.setItem("mlad-annotations:"+report.source_sha256,
        JSON.stringify(savedSidecarNotes.annotations));
    } catch {
      // Local cache is optional and may be disabled by the host.
    }
  }
  const rungKey=(outputLine:number,output:number)=>"L"+outputLine+":OUT"+output;
  const ionameKey=(entry:IonameEntry)=>ioname!.source_sha256+":"+entry.line+":"+entry.slot;

  function beginEdit(type:EditKey,key:string,caption:string,current:string) {
    if(sidecarBusy)return;
    setEditing({type,key,caption});
    setCommentText(current);
  }
  function cancelEdit(){setEditing(null);setCommentText("");}
  function saveEdit(){
    if(!report || !editing||sidecarBusy)return;
    if(commentText.length>200){setIssue("Максимальная длина комментария — 200 символов");return;}
    const next={...annotations,[editing.type]:{
      ...annotations[editing.type],[editing.key]:commentText.trim(),
    }};
    persistAnnotations(next);
    cancelEdit();
  }
  async function importIoname(selectedPath:string=ionamePath){
    if(!selectedPath.trim()||ionameBusy||sidecarBusy)return;
    setIonameBusy(true);setIssue("");setIoname(null);setLinkEntry(null);
    try{
      const value=await invoke<IonameReport>("cmd_inspect_ioname",{filePath:selectedPath.trim()});
      setIonamePath(selectedPath);
      setIoname(value);setTab("ioname");
      if(!value.header_valid)setIssue("IONAME.DAT: заголовок/секции не подтверждены; показаны только безопасно распознанные позиции.");
    }catch(error){setIssue(String(error));}finally{setIonameBusy(false);}
  }
  function bindIonameLabel(){
    if(!linkEntry||!ioname||!report||sidecarBusy)return;
    const addr=linkAddress.trim().toUpperCase();
    // Explicit manual binding only. Never infer #XXXXX from IN/OUT row order.
    if(!/^#[0-9]{5}$/.test(addr)&&!/^M[0-9]{3}$/.test(addr)){
      setIssue("Для ручной привязки укажите #XXXXX или Mxxx. Никакой автоматической адресации нет.");return;
    }
    const key=ionameKey(linkEntry);
    const label=annotations.ioname[key]??linkEntry.label;
    persistAnnotations({...annotations,addresses:{...annotations.addresses,[addr]:label}});
    setLinkEntry(null);setLinkAddress("");
  }

  function reportAsJson(){
    if(!report)return;
    setJson(JSON.stringify({
      format:"MLAD Native Workspace read-only diagnostic report",
      controller:report.controller,source_sha256:report.source_sha256,
      byte_length:report.byte_length,line_count:report.line_count,encoding:report.encoding,
      syntax_coverage:report.syntax_coverage,
      coverage_warning:"Syntax-only full-source line classification; not OEM execution coverage.",
      recognized_instructions:report.recognized_instructions,unsupported_lines:report.unsupported_lines,
      typed_candidate_count:report.typed_candidate_count,
      typed_valid_forms:report.typed_valid_forms,
      typed_invalid_forms:report.typed_invalid_forms,
      typed_mnemonic_counts:report.typed_mnemonic_counts,
      static_dataflow_summary:{
        total_candidates:report.dataflow.total_candidates,
        possible_source_order_links:report.dataflow.possible_source_order_links,
        continuity_barriers:report.dataflow.continuity_barriers,
        preview_truncated:report.dataflow.events_truncated,
        semantics_verified:report.dataflow.semantics_verified,
        scan_order_verified:report.dataflow.scan_order_verified,
        arithmetic_flags_verified:report.dataflow.arithmetic_flags_verified,
      },
      end_line:report.end_line,post_end_line_count:report.post_end_line_count,
      post_end_sections:report.post_end_sections,post_end_unverified_lines:report.post_end_unverified_lines,
      sections:report.sections,parser_diagnostics:report.diagnostics,
      semantic_diagnostics:report.semantic.diagnostics,
      semantic_diagnostics_truncated:report.semantic.diagnostics_truncated,
      semantic_rung_count:report.semantic.rung_count,
      annotation_count:{
        rungs:Object.keys(annotations.rungs).length,
        addresses:Object.keys(annotations.addresses).length,
      },
      ioname_source_sha256:ioname?.source_sha256??null,
      annotation_policy:"LocalStorage cache + opt-in standalone *.mlad.json sidecar; originals untouched",
      original_round_trip_only:report.original_round_trip_only,
      edited_export_allowed:report.edited_export_allowed,
      system_user_boundaries_verified:report.system_user_boundaries_verified,
      warning:"Report may contain sensitive controller addresses. Review before sharing.",
    },null,2));
  }
  const sideStyle:React.CSSProperties={
    background:"#182530",borderRight:"1px solid #405263",minWidth:0,
    overflowY:"auto",padding:10,fontSize:12,
  };
  const itemStyle:React.CSSProperties={
    display:"block",background:"transparent",border:0,color:"#ced9e5",
    cursor:"pointer",padding:"5px 3px",textAlign:"left",width:"100%",
    overflow:"hidden",textOverflow:"ellipsis",whiteSpace:"nowrap",fontSize:12,
  };
  const tabButton=(view:typeof tab,label:string)=>(
    <button type="button" style={{...btn,background:tab===view?"#3b695e":"#2e4152"}}
      aria-pressed={tab===view} onClick={()=>setTab(view)}>{label}</button>
  );
  return <main aria-label="Native CIO Workspace" style={{
    display:"flex",flex:1,minHeight:0,minWidth:0,flexDirection:"column",
    background:"#101820",color:"#dce5ef",
  }}>
    <header className="native-compact-header">
      <div className="native-header-title">
        <strong>Программа робота · CIOPRG.LST</strong>
        <small>Native Source · Только чтение · System/User не подтверждены</small>
      </div>
      <form className="native-header-file" aria-label="Открыть CIOPRG.LST"
        onSubmit={event=>{event.preventDefault();void openSource();}}>
        <div className="native-file-caption">
          <b>CIOPRG.LST</b>
          <span title={path}>{shortName(path)}</span>
        </div>
        <div className="native-file-inputs">
          <input aria-label="Путь к файлу CIOPRG.LST" title={path}
            value={path} onChange={event=>setPath(event.target.value)}
            placeholder="Выберите файл программы…" style={field}/>
          <button type="button" className="native-pick-file" title="Стандартный диалог открытия .LST"
            aria-label="Выбрать файл CIOPRG.LST" disabled={!!filePickerBusy||busy}
            onClick={()=>void chooseFile("cioprg")}>
            {filePickerBusy==="cioprg"?"Выбор…":"Обзор…"}
          </button>
          <button type="submit" className="native-open-file" disabled={!path.trim()||busy||!!filePickerBusy}>
            {busy?"Чтение…":"Открыть"}
          </button>
        </div>
        <div className="native-file-controller">
          <label htmlFor="native-controller">Контроллер</label>
          <select id="native-controller" aria-label="Семейство контроллера" value={controller}
            onChange={event=>setController(event.target.value as Controller)} style={field}>
            <option value="DX100">DX100</option><option value="DX200">DX200</option>
            <option value="YRC1000">YRC1000</option>
          </select>
        </div>
      </form>
      <form className="native-header-file" aria-label="Открыть IONAME.DAT"
        onSubmit={event=>{event.preventDefault();void importIoname();}}>
        <div className="native-file-caption">
          <b>IONAME.DAT</b>
          <span title={ionamePath}>{shortName(ionamePath)}</span>
        </div>
        <div className="native-file-inputs">
          <input aria-label="Путь к IONAME.DAT" title={ionamePath}
            value={ionamePath} onChange={event=>setIonamePath(event.target.value)}
            placeholder="Выберите файл имён I/O…" style={field}/>
          <button type="button" className="native-pick-file" title="Стандартный диалог открытия IONAME.DAT"
            aria-label="Выбрать файл IONAME.DAT" disabled={!!filePickerBusy||ionameBusy}
            onClick={()=>void chooseFile("ioname")}>
            {filePickerBusy==="ioname"?"Выбор…":"Обзор…"}
          </button>
          <button type="submit" className="native-open-file"
            disabled={!ionamePath.trim()||ionameBusy||!!filePickerBusy}>
            {ionameBusy?"Чтение…":"Открыть"}
          </button>
        </div>
        <small className="native-file-status">
          {ioname?"Загружено "+ioname.entries.length+" позиций · оригинал неизменён":"Файл не загружен"}
        </small>
      </form>
      <div className="native-header-offline">● OFFLINE</div>
    </header>
    {issue&&<div role="alert" style={{background:"#56363c",color:"#ffd5d6",padding:"8px 14px"}}>
      {issue}
    </div>}
    {!report?<section style={{padding:24,maxWidth:800,lineHeight:1.7}}>
      <h2 style={{marginTop:0}}>Native CIOPRG Workspace</h2>
      <p>Откройте копию выгрузки робота. Структура, исходный текст, распознанные цепи и
        диагностика будут расположены в основном рабочем окне. Это не JSON-черновик.</p>
      <p style={{color:"#e2c18b"}}>Изменение файла, экспорт CIOPRG.LST и сетевой доступ заблокированы.</p>
    </section>:<>
      <div style={{display:"flex",gap:15,flexWrap:"wrap",padding:"7px 14px",fontSize:12,
        background:"#192735",borderBottom:"1px solid #35485b"}}>
        <span>{report.controller} · <b>{report.line_count}</b> строк · {report.byte_length} байт</span>
        <span style={{color:"#a6d5b5"}}>{report.recognized_instructions} булево/блоки</span>
        <span style={{color:"#a9c8e6"}} title="Форма операндов определена; исполнение и флаги не подтверждены">
          {report.typed_candidate_count} типизировано · {report.typed_valid_forms} формы соответствуют
        </span>
        {report.typed_invalid_forms>0&&<span style={{color:"#eead85"}}>
          {report.typed_invalid_forms} неподтверждённых форм
        </span>}
        <span style={{color:"#edc38b"}}>{report.unsupported_lines} opaque/unknown</span>
        {report.end_line!==null&&<span title="Текстовая классификация, не подтверждение OEM-грамматики">
          END L{report.end_line} · после END {report.post_end_line_count} строк · {report.post_end_sections} разделов
          {report.post_end_unverified_lines>0?" · неизвестных "+report.post_end_unverified_lines:""}
        </span>}
        <span title={report.source_sha256}>SHA-256: {report.source_sha256.slice(0,16)}…</span>
      </div>
      <div aria-label="Покрытие синтаксиса" style={{
        display:"flex",gap:12,flexWrap:"wrap",padding:"5px 14px",fontSize:11,
        background:"#18242d",borderBottom:"1px solid #35485b",
      }}>
        <b>Классификация {report.syntax_coverage.total_lines} физических строк:</b>
        <span>Распознано* {report.syntax_coverage.recognized}</span>
        <span>Типизировано* {report.syntax_coverage.typed_candidate}</span>
        <span>Unsupported {report.syntax_coverage.unsupported}</span>
        <span>Malformed {report.syntax_coverage.malformed}</span>
        <span>Opaque {report.syntax_coverage.opaque}</span>
        <span>Post-END data {report.syntax_coverage.post_end_data}</span>
        <span>Структура {report.syntax_coverage.structural}</span>
        <span style={{color:"#e9c489"}}>* Только синтаксис; исполнение OEM не проверено.</span>
      </div>
      <div style={{display:"grid",gridTemplateColumns:"220px minmax(300px,1fr) 290px",flex:1,minHeight:0}}>
        <aside style={sideStyle} aria-label="Дерево программы">
          <strong>Закладки sidecar ({bookmarks.length})</strong>
          {bookmarks.slice(0,150).map(bookmark=><div key={bookmark.line}
            style={{display:"flex",gap:3,alignItems:"center"}}>
            <button type="button" style={{...itemStyle,flex:1}}
              title={bookmark.label} onClick={()=>void showLine(bookmark.line)}>
              L{bookmark.line} · {bookmark.label}
            </button>
            <button type="button" style={{...btn,padding:"3px 6px",fontSize:10}}
              aria-label={"Удалить закладку L"+bookmark.line}
              onClick={()=>removeBookmark(bookmark.line)}>×</button>
          </div>)}
          {bookmarks.length>150&&<small>Показаны первые 150 закладок из {bookmarks.length}.</small>}
          <hr style={{borderColor:"#3a4d5e"}}/>
          <strong>Структура файла</strong>
          <p style={{fontSize:11,color:"#d2b484"}}>
            PART — только текстовая метка, не право записи в System/User
          </p>
          {report.sections.slice(0,160).map((s,i)=><button key={i} type="button" style={itemStyle}
            title={s.label+" · строка "+s.line} onClick={()=>void showLine(s.line)}>
            {s.category==="Part"?"▸ ":"· "}{s.label} <small>L{s.line}</small>
          </button>)}
          {report.sections.length>160&&<small style={{color:"#e4bd80"}}>
            Показано 160 меток из {report.sections.length}; полный исходник доступен по номеру строки.
          </small>}
          <hr style={{borderColor:"#3a4d5e"}}/>
          <strong>Логические цепи ({report.semantic.rung_count})</strong>
          {report.semantic.rungs.map(r=><button type="button" key={r.number}
            style={{...itemStyle,background:rung===r.number?"#294c4c":"transparent"}}
            aria-current={rung===r.number?"true":undefined}
            onClick={()=>{setRung(r.number);setSelectedLine(r.first_line);setTab("diagram");}}>
            R{r.number} · OUT {relayText(r.output)}
          </button>)}
          {report.semantic.rungs_truncated&&<small>Список цепей ограничен</small>}
          <hr style={{borderColor:"#3a4d5e"}}/>
          <strong>Функциональные блоки</strong>
          {report.advanced.blocks.map(b=><button type="button"
            style={{...itemStyle,background:selectedBlockLine===b.line?"#294c4c":"transparent"}}
            aria-current={selectedBlockLine===b.line?"true":undefined} key={b.line}
            onClick={()=>{setSelectedBlockLine(b.line);setTab("blocks");setSelectedLine(b.line);}}>
            {b.kind} · L{b.line}
          </button>)}
          {report.advanced.blocks_truncated&&<small style={{color:"#e4bd80"}}>
            Список блоков ограничен предпросмотром; остальные доступны в исходном тексте.
          </small>}
        </aside>
        <section aria-label="Рабочее поле CIOPRG" style={{
          display:"flex",flexDirection:"column",minWidth:0,minHeight:0,background:"#0f1923",
        }}>
          <nav className="native-context-toolbar" aria-label="Вкладки и инструменты схем">
            <div className="native-context-tabs">
              {tabButton("source","Исходный текст")}
              {tabButton("diagram","Ladder ("+report.semantic.rung_count+")")}
              {tabButton("blocks","TMR / CNT / GSTR ("+report.advanced.blocks.length+")")}
              {tabButton("ioname","Имена I/O"+(ioname?" ("+ioname.entries.length+")":""))}
            </div>
            {tab==="diagram"&&<div role="group" aria-label="Масштаб Ladder" className="native-context-actions">
              <strong>Все цепи ({report.semantic.rung_count}) · выбрана R{rung}</strong>
              <button type="button" style={btn} aria-label="Уменьшить масштаб Ladder"
                onClick={()=>setZoom(z=>Math.max(.5,+(z-.15).toFixed(2)))}>−</button>
              <span aria-live="polite">{Math.round(zoom*100)}%</span>
              <button type="button" style={btn} aria-label="Увеличить масштаб Ladder"
                onClick={()=>setZoom(z=>Math.min(1.8,+(z+.15).toFixed(2)))}>+</button>
              <button type="button" style={btn} onClick={()=>setZoom(1)}>100%</button>
            </div>}
            {tab==="blocks"&&<div role="group" aria-label="Масштаб TMR CNT GSTR" className="native-context-actions">
              <strong>Все блоки ({report.advanced.blocks.length}) · выбран L{selectedBlockLine??"—"}</strong>
              <button type="button" style={btn} aria-label="Уменьшить масштаб блоков"
                onClick={()=>setBlockZoom(z=>Math.max(.5,+(z-.15).toFixed(2)))}>−</button>
              <span aria-live="polite">{Math.round(blockZoom*100)}%</span>
              <button type="button" style={btn} aria-label="Увеличить масштаб блоков"
                onClick={()=>setBlockZoom(z=>Math.min(1.8,+(z+.15).toFixed(2)))}>+</button>
              <button type="button" style={btn} onClick={()=>setBlockZoom(1)}>100%</button>
            </div>}
          </nav>
          {tab==="source"&&<>
            <div style={{display:"flex",gap:8,alignItems:"center",flexWrap:"wrap",
              padding:9,borderBottom:"1px solid #35485b",fontSize:12}}>
              <button type="button" style={btn} disabled={pageBusy||pageStart===1}
                onClick={()=>void showLine(Math.max(1,pageStart-PAGE_SIZE))}>←</button>
              <span>{pageStart}–{pageLines[pageLines.length-1]?.line??0} / {report.line_count}</span>
              <button type="button" style={btn}
                disabled={pageBusy||pageStart+PAGE_SIZE>report.line_count}
                onClick={()=>void showLine(pageStart+PAGE_SIZE)}>→</button>
              <input type="number" min={1} max={report.line_count} aria-label="Номер строки"
                value={goto} onChange={e=>setGoto(e.target.value)}
                style={{...field,width:80}} placeholder="Строка"/>
              <button type="button" style={btn} onClick={()=>void showLine(Number(goto))}
                disabled={pageBusy}>Перейти</button>
              {selectedRung&&<button type="button" style={btn}
                onClick={()=>{setRung(selectedRung.number);setTab("diagram");}}>
                К цепи R{selectedRung.number} ↗
              </button>}
              {selectedRelatedBlock&&<button type="button" style={btn}
                onClick={()=>{setSelectedBlockLine(selectedRelatedBlock.line);setTab("blocks");}}>
                К блоку L{selectedRelatedBlock.line} ↗
              </button>}
              {selectedLine!==null&&<button type="button" style={btn}
                onClick={()=>selectedDiagnosticsRef.current?.scrollIntoView({block:"nearest",behavior:"smooth"})}>
                Диагностика L{selectedLine} ({selectedLineDiagnostics.length}) ↗
              </button>}
              {pageBusy&&<span>Чтение страницы…</span>}
            </div>
            <div style={{flex:1,minHeight:0,overflow:"auto",fontFamily:"Consolas,monospace",
              fontSize:12,lineHeight:1.7,padding:"8px 12px"}}>
              {pageLines.map(l=><button type="button" key={l.line}
                ref={element=>{if(selectedLine===l.line)activeSourceLineRef.current=element;}}
                aria-current={selectedLine===l.line?"true":undefined}
                onClick={()=>setSelectedLine(l.line)}
                onKeyDown={event=>{
                  const next=event.key==="ArrowDown"?l.line+1:
                    event.key==="ArrowUp"?l.line-1:
                    event.key==="PageDown"?l.line+PAGE_SIZE:
                    event.key==="PageUp"?l.line-PAGE_SIZE:
                    event.key==="Home"?1:event.key==="End"?report.line_count:null;
                  if(next!==null){
                    event.preventDefault();
                    void showLine(Math.max(1,Math.min(report.line_count,next)));
                  }
                }}
                title={"Byte offset: "+l.body_span.start+"–"+l.body_span.end}
                style={{display:"flex",gap:10,whiteSpace:"pre",width:"max-content",minWidth:"100%",
                  textAlign:"left",font:"inherit",padding:"0 4px",border:0,cursor:"pointer",
                  color:"#dce5ef",background:selectedLine===l.line?"#2b4d5b":"transparent"}}>
                <span style={{width:51,flexShrink:0,textAlign:"right",color:"#849eaf"}}>{l.line}</span>
                <span style={{width:142,flexShrink:0,color:
                  l.syntax_class==="Malformed"?"#f4a8ad":
                  l.syntax_class==="Unsupported"||l.syntax_class==="Opaque"?"#e4b97e":
                  l.syntax_class==="TypedCandidate"?"#b8ccec":"#85abb9"}}>
                  {syntaxClassLabel(l)}</span>
                <span>{l.display_text||" "}</span>
              </button>)}
            </div>
            <div style={{fontSize:11,color:"#a9c0cb",padding:"5px 12px",borderTop:"1px solid #35485b"}}>
              Стрелки ↑/↓ · PageUp/PageDown · Home/End · полный исходник по страницам · только чтение
            </div>
          </>}
          {tab==="diagram"&&<div style={{overflow:"auto",flex:1,padding:15}}>
            <p style={{color:"#e4bd80",fontSize:12}}>
              Только выбранная цепь из ограниченного Semantic AST — крупные документы не рендерятся целиком.
              Неизвестные команды не преобразуются в предполагаемую схему.
              Двойной щелчок по заголовку цепи или адресам изменяет только локальный комментарий.
            </p>
            {visibleRungs.map(({rung:current,expr,error})=>{
              const key=rungKey(current.output_line,current.output);
              const comment=annotations.rungs[key]??"";
              return <article key={current.number} style={{
                ...panel,marginBottom:12,background:"#142230",padding:12,
                borderColor:rung===current.number?"#578275":"#3e5367",
              }}>
                <div style={{display:"flex",gap:9,alignItems:"center",justifyContent:"space-between",
                  flexWrap:"wrap",marginBottom:8}}>
                  <button type="button" onDoubleClick={()=>beginEdit("rungs",key,
                      "Комментарий цепи R"+current.number,comment)}
                    title="Двойной щелчок — изменить комментарий цепи" style={{
                      ...btn,background:"#244052",textAlign:"left",
                    }} onClick={()=>setRung(current.number)}>
                    R{current.number} · OUT {relayText(current.output)}
                    {comment?" · "+comment:" · двойной щелчок: комментарий"}
                  </button>
                  <span style={{fontSize:11,color:"#afc6d7"}}>
                    L{current.first_line}–L{current.output_line}
                  </span>
                  <button type="button" style={btn}
                    onClick={()=>void showLine(current.first_line)}>К исходной строке ↗</button>
                </div>
                {expr?<LadderCircuit readOnly zoom={zoom} expression={expr}
                  output={relayText(current.output)}
                  onEditComment={address=>beginEdit("addresses",address,"Комментарий адреса "+address,
                    annotations.addresses[address]??"")}
                  commentForAddress={address=>annotations.addresses[address]??""}/>:
                  <div role="alert" style={{color:"#e0bd8b"}}>Граф недоступен: {error||"неполное AST"}</div>}
              </article>;
            })}
            {!visibleRungs.length&&<div>Поддерживаемых графических цепей не обнаружено.</div>}
            {report.semantic.rungs_truncated&&<p style={{color:"#e5b986"}}>
              Внимание: источник содержит больше распознанных цепей, чем возвращает текущий лимит preview.
              Остальные доступны в исходном тексте, но не на схеме.
            </p>}
            {report.semantic.nodes_truncated&&<p style={{color:"#e5b986"}}>Узлы AST ограничены; некоторые схемы могут отсутствовать.</p>}
          </div>}
          {tab==="blocks"&&<div style={{flex:1,overflow:"auto",padding:15}}>
            <p style={{color:"#e4bd80",fontSize:12}}>
              Визуализируются только подтверждённые простые топологии. Остальные доступны в исходнике.
            </p>
            {(activeBlock?[activeBlock]:[]).map(b=>{
              const overlay=toReadOnlyAdvanced(b);
              return <div style={{...panel,marginBottom:12}} key={b.line}>
                <div style={{display:"flex",gap:10,alignItems:"center",
                  justifyContent:"space-between",flexWrap:"wrap",marginBottom:10}}>
                  <strong>{b.kind} · L{b.line}–{b.end_line} · {b.operands.join(", ")}</strong>
                  <button type="button" style={btn} onClick={()=>void showLine(b.line)}>К исходнику ↗</button>
                </div>
                {overlay?<AdvancedBlockDiagram block={overlay} zoom={blockZoom}
                  onEditComment={address=>beginEdit("addresses",address,
                    "Комментарий адреса "+address,annotations.addresses[address]??"")}
                  commentForAddress={address=>annotations.addresses[address]??""}/>:
                  <div style={{color:"#e4bd80"}}>Граф не построен: топология не подтверждена.</div>}
              </div>;
            })}
            {!report.advanced.blocks.length&&<p>Поддерживаемые блоки не найдены.</p>}
            {report.advanced.blocks.length>0&&!activeBlock&&<p>Выберите блок в дереве слева.</p>}
            {report.advanced.blocks_truncated&&<p>Предпросмотр блоков ограничен.</p>}
          </div>}
          {tab==="ioname"&&<div style={{flex:1,overflow:"auto",padding:15}}>
            <h3 style={{margin:"0 0 8px"}}>IONAME.DAT · просмотр и локальные комментарии</h3>
            <p style={{fontSize:12,color:"#e4bd80"}}>
              IONAME хранит имена I/O по секциям и порядковым позициям. Соответствие
              //IN или //OUT конкретному адресу #XXXXX здесь не подтверждено.
              Двойной щелчок меняет только локальную заметку MLAD, не файл робота.
            </p>
            {!ioname?<p>Укажите путь к копии IONAME.DAT в верхней панели и нажмите «Загрузить имена I/O».</p>:
            <>
              <div style={{fontSize:12,marginBottom:10}}>
                {ioname.header_valid?"Заголовок /IONAME подтверждён":"Структура не подтверждена"} ·
                {ioname.byte_length} байт · SHA-256 {ioname.source_sha256.slice(0,16)}… ·
                {ioname.encoding} · Автоматическая карта I/O: <b>НЕ ВЕРИФИЦИРОВАНА</b>
              </div>
              {!!ioname.warnings.length&&<div style={{color:"#e9be88",fontSize:12,marginBottom:10}}>
                {ioname.warnings.slice(0,10).map((w,i)=><div key={i}>{w}</div>)}
              </div>}
              <input aria-label="Фильтр IONAME" value={ionameQuery} onChange={e=>setIonameQuery(e.target.value)}
                placeholder="Поиск по секции, позиции или имени"
                style={{...field,width:"100%",boxSizing:"border-box",marginBottom:8}}/>
              <div style={{fontSize:12,color:"#b4c9d9",marginBottom:7}}>
                Показано {filteredIoname.length} записей (макс. 300 на экране).
                {ioname.entries_truncated&&" Объём файла превышает лимит безопасного предпросмотра."}
              </div>
              {filteredIoname.map(entry=>{
                const key=ionameKey(entry);
                const label=annotations.ioname[key]??entry.label;
                return <div key={key} style={{display:"flex",gap:8,alignItems:"center",
                  borderBottom:"1px solid #304557",padding:"6px 4px",fontSize:12}}>
                  <code style={{flex:"0 0 120px",color:"#91b8cd"}}>
                    {entry.section} · поз. {entry.ordinal}
                  </code>
                  <button type="button" style={{...btn,flex:1,textAlign:"left",
                      color:"#cfe6da",minWidth:100}}
                    onDoubleClick={()=>beginEdit("ioname",key,
                      "Локальное имя "+entry.section+" / "+entry.ordinal,label)}
                    title="Двойной щелчок — редактировать локальное имя">
                    {label||"— без имени —"}
                  </button>
                  <button type="button" style={btn}
                    title="Назначить значение из этой позиции выбранному CIO адресу вручную"
                    onClick={()=>{setLinkEntry(entry);setLinkAddress("");}}>Связать с адресом</button>
                </div>;
              })}
              {linkEntry&&<div style={{...panel,marginTop:10}}>
                <strong>Ручная привязка позиции {linkEntry.section} {linkEntry.ordinal}</strong>
                <p style={{fontSize:12,color:"#e4bd80"}}>
                  Укажите точный адрес из CIOPRG.LST вручную; порядок в IONAME
                  НЕ подтверждает соответствие Relay-адресам.
                </p>
                <input aria-label="Адрес CIO для ручной привязки" value={linkAddress}
                  onChange={e=>setLinkAddress(e.target.value)} placeholder="#20010 или M021"
                  style={field}/>
                <button type="button" style={{...btn,marginLeft:8}} onClick={bindIonameLabel}>Привязать локально</button>
                <button type="button" style={{...btn,marginLeft:8}} onClick={()=>setLinkEntry(null)}>Отмена</button>
              </div>}
            </>}
          </div>}
        </section>
        <aside style={{...sideStyle,borderRight:0,borderLeft:"1px solid #405263"}}
          aria-label="Диагностика и поиск">
          {selectedLine!==null&&<div ref={selectedDiagnosticsRef}
            aria-label="Диагностика выбранной строки"
            style={{marginBottom:12,padding:9,border:"1px solid #577276",borderRadius:5,background:"#19323a"}}>
            <strong>Диагностика L{selectedLine} ({selectedLineDiagnostics.length})</strong>
            {selectedLineDiagnostics.map((d,i)=><div key={i} style={{fontSize:11,marginTop:5}}>
              <b style={{color:d.severity==="Error"?"#f4a8ad":"#e4bd80"}}>{d.code}</b>
              <div>{d.message}</div>
            </div>)}
            {!selectedLineDiagnostics.length&&<small>
              В доступном preview диагностики для строки нет; полная фильтрация неизвестных конструкций доступна ниже.
            </small>}
            {(report.diagnostics.length>=500||report.semantic.diagnostics_truncated)&&<small>
              Список диагностики ограничен, отсутствие сообщения не означает подтверждение синтаксиса.
            </small>}
          </div>}
          <section aria-label="Фильтр исходного CIOPRG" style={{
            marginBottom:12,padding:9,border:"1px solid #456078",borderRadius:5,
            background:"#1c2d3b",fontSize:12,
          }}>
            <strong>Фильтрация всего CIOPRG · до 200 000 строк</strong>
            <p style={{fontSize:11,color:"#dabc88",lineHeight:1.5}}>
              По физическому исходнику, не по выполняемой логике. Результаты по 100 строк,
              снимок привязан к SHA-256; неизвестное содержимое не скрывается.
            </p>
            <form onSubmit={event=>{event.preventDefault();void filterFullSource(0);}}
              style={{display:"grid",gap:6}}>
              <label style={{display:"grid",gap:3}}>
                Класс синтаксиса
                <select aria-label="Фильтр класса синтаксиса" value={sourceClass} disabled={sourceFilterBusy}
                  onChange={event=>{
                    setSourceClass(event.target.value as ""|SyntaxClass);
                    setSourceFilterResult(null);
                  }} style={{...field,width:"100%",boxSizing:"border-box"}}>
                  <option value="">Все классы</option>
                  <option value="Recognized">Recognized</option>
                  <option value="TypedCandidate">TypedCandidate</option>
                  <option value="Unsupported">Unsupported</option>
                  <option value="Malformed">Malformed</option>
                  <option value="Opaque">Opaque</option>
                  <option value="PostEndData">PostEndData</option>
                  <option value="Structural">Structural</option>
                </select>
              </label>
              <label style={{display:"grid",gap:3}}>
                Opcode (точное ASCII-имя)
                <input aria-label="Фильтр opcode" value={sourceOpcode} maxLength={20} disabled={sourceFilterBusy}
                  onChange={event=>{setSourceOpcode(event.target.value);setSourceFilterResult(null);}}
                  placeholder="STR / OUT / MOV / TMR" style={{...field,width:"100%",boxSizing:"border-box"}}/>
              </label>
              <label style={{display:"grid",gap:3}}>
                Адрес (текстовый токен)
                <input aria-label="Фильтр адреса" value={sourceAddress} maxLength={6} disabled={sourceFilterBusy}
                  onChange={event=>{setSourceAddress(event.target.value);setSourceFilterResult(null);}}
                  placeholder="#70010 / M010" style={{...field,width:"100%",boxSizing:"border-box"}}/>
              </label>
              <button type="submit" style={btn} disabled={sourceFilterBusy||pageBusy}>
                {sourceFilterBusy?"Фильтрация…":"Применить фильтр"}
              </button>
            </form>
            {sourceFilterResult&&<div style={{marginTop:8}} aria-live="polite">
              <div style={{fontSize:11,color:"#acc9d6"}}>
                Совпадений {sourceFilterResult.total_matches}; показано
                {" "}{sourceFilterResult.hits.length===0?0:sourceFilterResult.offset+1}–
                {sourceFilterResult.offset+sourceFilterResult.hits.length}
              </div>
              <div style={{display:"flex",gap:7,marginTop:5,marginBottom:5}}>
                <button type="button" style={btn}
                  disabled={sourceFilterBusy||sourceFilterResult.offset===0}
                  onClick={()=>void filterFullSource(Math.max(0,sourceFilterResult.offset-100))}>
                  ← 100
                </button>
                <button type="button" style={btn}
                  disabled={sourceFilterBusy||sourceFilterResult.next_offset===null}
                  onClick={()=>void filterFullSource(sourceFilterResult.next_offset!)}>
                  100 →
                </button>
              </div>
              <div style={{maxHeight:190,overflowY:"auto"}}>
                {sourceFilterResult.hits.map(hit=><button type="button" key={hit.line}
                  style={itemStyle} title={"Byte offset: "+hit.byte_offset}
                  onClick={()=>void showLine(hit.line)}>
                  L{hit.line} · {hit.syntax_class} · {hit.preview}
                </button>)}
              </div>
              {!sourceFilterResult.hits.length&&<small>Совпадения отсутствуют на этой странице.</small>}
            </div>}
          </section>
          <section aria-label="Типизированные инструкции" style={{
            marginBottom:12,padding:9,border:"1px solid #46617d",borderRadius:5,
            background:"#1b2a3a",fontSize:12,
          }}>
            <strong>Типизированные формы ({report.typed_candidate_count})</strong>
            <p style={{fontSize:11,color:"#e1bd83",lineHeight:1.4}}>
              MOV / ADD / SUB / SHL / SHR · Только синтаксис. Семантика, флаги,
              System/User и разрешение записи не подтверждены.
            </p>
            <div style={{display:"flex",gap:6,flexWrap:"wrap",fontSize:11}}>
              {Object.entries(report.typed_mnemonic_counts).map(([code,count])=>
                <span key={code} title="Только инвентаризация форм"
                  style={{border:"1px solid #405a72",padding:"3px 6px",borderRadius:4}}>
                  {code}: {count}
                </span>)}
            </div>
            {selectedTyped&&<div style={{marginTop:10,paddingTop:8,borderTop:"1px solid #385269"}}>
              <strong>{selectedTyped.instruction.mnemonic} · L{selectedTyped.instruction.line}</strong>
              <div style={{fontSize:11,color:selectedTyped.instruction.syntax_valid?"#b7d8c5":"#f1b38e"}}>
                {selectedTyped.instruction.syntax_valid?
                  "Типы/количество операндов соответствуют проверяемой форме":
                  "Форма не соответствует проверяемому подмножеству"}
              </div>
              {!selectedTyped.controller_form_verified&&<div style={{fontSize:11,color:"#f1b38e"}}>
                Для выбранного семейства форма отдельно не подтверждена документацией; отображается только синтаксис.
              </div>}
              {selectedTyped.instruction.operands.map((operand,index)=>
                <div key={index} style={{display:"flex",gap:6,fontFamily:"Consolas,monospace",
                  marginTop:5,fontSize:11,overflowWrap:"anywhere"}}>
                  <b>{index+1}.</b><span>{operand.spelling}</span>
                  <span style={{color:"#9cb7d0"}}>{operand.kind}</span>
                </div>)}
              <div style={{fontSize:11,color:"#e0b985",marginTop:7}}>
                Не участвует в реконструкции Ladder. Изменение файла запрещено.
              </div>
              {oemManualEntry(selectedTyped.instruction.mnemonic)&&<details
                style={{marginTop:9,padding:"6px 7px",border:"1px solid #566d61",borderRadius:5}}>
                <summary style={{cursor:"pointer",color:"#b8d6c4"}}>
                  Справка OEM · {oemManualEntry(selectedTyped.instruction.mnemonic)?.section}
                  {" · не результат выполнения"}
                </summary>
                {(()=>{
                  const rule=oemManualEntry(selectedTyped.instruction.mnemonic);
                  if(!rule)return null;
                  return <div style={{display:"grid",gap:5,marginTop:7,fontSize:11}}>
                    <b>{rule.format}</b>
                    <span>{rule.whenOn}</span>
                    <span>Carry: {rule.carry}</span>
                    <span>Zero: {rule.zero}</span>
                    <span>Error: {rule.error}</span>
                    <span>Операнды: {rule.limits}</span>
                    <span style={{color:"#e6bd81"}}>Не подтверждено: {rule.unresolved}</span>
                    <span style={{color:"#e6bd81"}}>
                      EN, выходные данные и переходы флагов между командами не моделируются.
                      Испытания на конкретном контроллере отсутствуют.
                    </span>
                  </div>;
                })()}
              </details>}
            </div>}
            {!selectedTyped&&<p style={{fontSize:11,color:"#aec6d2"}}>
              Выберите типизированную строку в исходном тексте, чтобы увидеть операнды.
            </p>}
          </section>
          <section aria-label="Статический аудит регистров" style={{
            marginBottom:12,padding:9,border:"1px solid #5f6654",borderRadius:5,
            background:"#252b2d",fontSize:12,
          }}>
            <strong>Статические связи · только гипотезы</strong>
            <p style={{fontSize:11,color:"#e7c58e",lineHeight:1.5}}>
              Здесь показаны только совпадения имён M-регистров в соседних
              типизированных командах. Это НЕ граф фактического исполнения:
              условия, порядок сканирования и арифметические флаги не подтверждены.
            </p>
            <div style={{fontSize:11,color:"#bbceda",display:"grid",gap:3}}>
              <span>Форм в документе: {report.dataflow.total_candidates}</span>
              <span>Потенциальных текстовых связей: {report.dataflow.possible_source_order_links}</span>
              <span>Разрывов непрерывности: {report.dataflow.continuity_barriers}</span>
              {report.dataflow.events_truncated&&<span style={{color:"#e7bd85"}}>
                Подробные записи ограничены первыми {report.dataflow.events.length} строками-кандидатами.
              </span>}
            </div>
            {selectedStatic&&<div style={{borderTop:"1px solid #58635e",marginTop:9,paddingTop:7}}>
              <strong>Строка L{selectedStatic.line} · {selectedStatic.mnemonic}</strong>
              {!selectedStatic.controller_form_verified&&<p style={{fontSize:11,color:"#e4b886"}}>
                Семейство контроллера: форма не подтверждена.
              </p>}
              <div style={{display:"grid",gap:5,marginTop:6}}>
                {selectedStatic.sites.map((site,i)=><div key={i} style={{fontSize:11,
                  display:"flex",flexWrap:"wrap",alignItems:"center",gap:6}}>
                  <code>{site.role==="Destination"?"D?":
                    site.role==="ShiftAmount"?"n?":"S?"} {site.spelling}</code>
                  <span style={{color:"#8fadb9"}}>{site.kind}</span>
                  {site.possible_previous_definition!==null&&
                    <button type="button" style={{...btn,padding:"3px 6px",fontSize:10}}
                      title="Только возможная текстовая связь, не подтверждённое значение"
                      onClick={()=>void showLine(site.possible_previous_definition!)}>
                      Возможный источник: L{site.possible_previous_definition} ↗
                    </button>}
                </div>)}
              </div>
              <small style={{display:"block",color:"#e7c58e",marginTop:7}}>
                Роли S?/D? обозначают только позиции операндов. Ни одна запись не верифицирована как действие робота.
              </small>
            </div>}
            {!selectedStatic&&<p style={{fontSize:11,color:"#a7c1c7"}}>
              Выберите типизированную строку исходника для анализа соседних ссылок.
            </p>}
          </section>
          <section aria-label="READ WRITE UNKNOWN — статический аудит" style={{
            marginBottom:12,padding:9,border:"1px solid #52637c",borderRadius:5,
            background:"#202b3a",fontSize:12,
          }}>
            <strong>READ / WRITE / UNKNOWN · только роли операндов</strong>
            <p style={{fontSize:11,color:"#e6c28b",lineHeight:1.5}}>
              READ и WRITE — предполагаемые роли адресов по руководству OEM,
              НЕ выполненные чтения/записи. Нет расчёта EN, scan, alias,
              переходов флагов или разрешения записи на робот.
            </p>
            <div style={{display:"flex",gap:8,flexWrap:"wrap",fontSize:11}}>
              <span>READ: {report.static_access.read_sites}</span>
              <span>WRITE: {report.static_access.write_sites}</span>
              <span>UNKNOWN: {report.static_access.unknown_sites}</span>
              <span>Неполная модель: {report.static_access.unmodeled_lines} строк</span>
            </div>
            <small style={{display:"block",color:"#9cb7ca",marginTop:5}}>
              Всего адресных/неизвестных мест {report.static_access.total_sites}.
              Числовых констант без доступа {report.static_access.skipped_literal_operands};
              post-END строк данных {report.static_access.post_end_lines}.
              {report.static_access.sites_truncated&&
                " Preview ограничен первыми "+report.static_access.sites.length+
                " местами; ниже доступен поиск по всему файлу."}
            </small>
            {selectedLine!==null&&<div style={{marginTop:9,paddingTop:7,borderTop:"1px solid #52637c"}}>
              <strong>L{selectedLine} · роли исходных операндов</strong>
              {selectedAccessSites.map((site,i)=><div key={i} style={{
                display:"flex",flexWrap:"wrap",gap:6,alignItems:"center",marginTop:4,fontSize:11,
              }}>
                <b style={{color:site.access==="Unknown"?"#e3bc88":"#b6d3c8"}}>
                  {site.access.toUpperCase()}</b>
                <code>{site.symbol}</code>
                <span>{site.mnemonic} · байты {site.span.start}–{site.span.end}</span>
                <small style={{color:"#b4bdce"}}>{site.basis}</small>
              </div>)}
              {!selectedAccessSites.length&&<small style={{display:"block",color:"#b5c7d2"}}>
                Для строки нет сведений в ограниченном preview; полная выборка ниже.
                Отсутствие записи не подтверждает отсутствие обращения.
              </small>}
            </div>}
            <form onSubmit={event=>{event.preventDefault();void queryStaticAccess(0);}}
              style={{display:"grid",gap:6,marginTop:10}}>
              <label style={{display:"grid",gap:3}}>
                Вид предполагаемого доступа
                <select aria-label="Фильтр вида READ WRITE UNKNOWN" value={accessKind}
                  disabled={accessBusy} style={{...field,width:"100%"}}
                  onChange={event=>{
                    accessRequestSeq.current++;
                    setAccessKind(event.target.value as ""|StaticAccessKind);
                    setAccessResult(null);
                  }}>
                  <option value="">Любой (укажите адрес)</option>
                  <option value="Read">READ — роль источника</option>
                  <option value="Write">WRITE — роль назначения</option>
                  <option value="Unknown">UNKNOWN — требуется проверка</option>
                </select>
              </label>
              <label style={{display:"grid",gap:3}}>
                Точный текстовый адрес
                <input aria-label="Адрес для статического аудита" value={accessSymbol}
                  disabled={accessBusy} maxLength={7}
                  onChange={event=>{
                    accessRequestSeq.current++;
                    setAccessSymbol(event.target.value);
                    setAccessResult(null);
                  }} placeholder="#70010 / M010 / W#70010"
                  style={{...field,width:"100%",boxSizing:"border-box"}}/>
              </label>
              <button type="submit" style={btn} disabled={accessBusy||pageBusy}>
                {accessBusy?"Проверка…":"Показать адресные роли"}
              </button>
            </form>
            {accessResult&&<div style={{marginTop:8}} aria-live="polite">
              <small>Совпадений: {accessResult.total_matches}; показано
                {" "}{accessResult.hits.length?accessResult.offset+1:0}–
                {accessResult.offset+accessResult.hits.length}</small>
              <div style={{display:"flex",gap:6,marginTop:6}}>
                <button type="button" style={btn} disabled={accessBusy||accessResult.offset===0}
                  onClick={()=>void queryStaticAccess(Math.max(0,accessResult.offset-100))}>
                  ← 100</button>
                <button type="button" style={btn}
                  disabled={accessBusy||accessResult.next_offset===null}
                  onClick={()=>void queryStaticAccess(accessResult.next_offset!)}>
                  100 →</button>
              </div>
              <div style={{maxHeight:230,overflowY:"auto",marginTop:5}}>
                {accessResult.hits.map((site,i)=><button type="button"
                  key={site.line+":"+site.span.start+":"+i} style={itemStyle}
                  title={"Оригинальный byte offset: "+site.span.start+
                    "; только синтаксическая роль"}
                  onClick={()=>void showLine(site.line)}>
                  L{site.line} · {site.access.toUpperCase()} · {site.mnemonic}
                  {" "}{site.symbol} · {site.basis}
                </button>)}
              </div>
              {!accessResult.hits.length&&<small>
                В этой выборке нет совпадений. Это не доказательство отсутствия
                аппаратного чтения/записи.
              </small>}
            </div>}
          </section>
          <section aria-label="Переносимый offline sidecar" style={{
            marginBottom:14,padding:9,border:"1px solid #47708d",borderRadius:5,background:"#172d40",
            fontSize:12,
          }}>
            <strong>Переносимый проект · *.mlad.json</strong>
            <p style={{fontSize:11,color:"#d9bc82",lineHeight:1.5}}>
              Только заметки и закладки, привязанные к SHA-256 CIOPRG.
              Не содержит исходный Ladder и не создаёт файл для контроллера.
            </p>
            <label style={{display:"grid",gap:4,marginBottom:7}}>
              Путь к отдельному sidecar JSON
              <input aria-label="Путь к MLAD sidecar" value={sidecarPath}
                disabled={sidecarBusy} style={{...field,width:"100%",boxSizing:"border-box"}}
                onChange={event=>{
                  setSidecarPath(event.target.value);setSidecarRevision(null);
                }} placeholder="CIOPRG.mlad.json"/>
            </label>
            <div style={{display:"flex",gap:5,flexWrap:"wrap"}}>
              <button type="button" style={btn}
                disabled={sidecarBusy||busy||!sidecarPath.trim()}
                onClick={()=>void loadPortableSidecar()}>
                {sidecarBusy?"Обработка…":"Загрузить sidecar"}
              </button>
              <button type="button" style={btn}
                disabled={sidecarBusy||busy||!sidecarPath.trim()||!!editing}
                onClick={()=>void savePortableSidecar()}>
                {sidecarBusy?"Обработка…":"Сохранить sidecar"}
              </button>
            </div>
            {sidecarDirty&&<div style={{marginTop:7,fontSize:11,color:"#f2c686"}}>
              Несохранённые в sidecar изменения.
              <button type="button" style={{...btn,padding:"3px 6px",marginTop:5,width:"100%"}}
                disabled={sidecarBusy||busy}
                onClick={()=>void loadPortableSidecar(true)}>
                Загрузить и заменить несохранённые заметки
              </button>
              <button type="button" style={{...btn,padding:"3px 6px",marginTop:5,width:"100%"}}
                disabled={sidecarBusy} onClick={discardUnsavedSidecarNotes}>
                Отменить несохранённые изменения sidecar
              </button>
            </div>}
            <div aria-live="polite" style={{marginTop:7,fontSize:11,color:"#b9d0e2"}}>
              {sidecarRevision?"Sidecar открыт/сохранён; ревизия "+sidecarRevision.slice(0,12)+"…":
                "Новый sidecar: если файл уже существует, сначала загрузите его."}
            </div>
            <div style={{borderTop:"1px solid #45627c",marginTop:9,paddingTop:8}}>
              <strong>Закладки ({bookmarks.length})</strong>
              <small style={{display:"block",color:"#c9d4e0",marginTop:4}}>
                Выберите физическую строку исходника и добавьте закладку.
              </small>
              <input aria-label="Подпись закладки" value={bookmarkLabel}
                maxLength={80} onChange={event=>setBookmarkLabel(event.target.value)}
                placeholder={selectedLine!==null?"Строка L"+selectedLine:"Сначала выберите строку"}
                style={{...field,width:"100%",boxSizing:"border-box",marginTop:5}}/>
              <button type="button" style={{...btn,width:"100%",marginTop:6}}
                disabled={selectedLine===null||bookmarks.length>=500}
                onClick={addBookmark}>
                Добавить закладку L{selectedLine??"—"}
              </button>
            </div>
          </section>
          <section aria-label="Редактор локальных комментариев" style={{
            marginBottom:14,padding:9,border:"1px solid #527369",borderRadius:5,
            background:"#1e3536",
          }}>
            <strong>Комментарии · локально и в sidecar по запросу</strong>
            <p style={{fontSize:11,color:"#d3c298",lineHeight:1.4}}>
              Двойной щелчок по заголовку цепи, адресу на схеме или имени IONAME.
              CIOPRG.LST и IONAME.DAT не изменяются. Переносной sidecar сохраняется только отдельной командой.
            </p>
            {editing?<div>
              <label style={{display:"grid",gap:5,fontSize:12}}>
                {editing.caption}
                <input autoFocus aria-label="Введите комментарий" value={commentText}
                  maxLength={200} onChange={e=>setCommentText(e.target.value)}
                  onKeyDown={e=>{
                    if(e.key==="Enter"){e.preventDefault();saveEdit();}
                    if(e.key==="Escape"){e.preventDefault();cancelEdit();}
                  }} style={{...field,width:"100%",boxSizing:"border-box"}}/>
              </label>
              <div style={{display:"flex",gap:7,marginTop:9}}>
                <button type="button" style={btn} onClick={saveEdit}>Сохранить</button>
                <button type="button" style={btn} onClick={cancelEdit}>Отмена</button>
              </div>
              <small>Enter — сохранить · Esc — отменить</small>
            </div>:<small style={{color:"#aec9c5"}}>Выберите элемент двойным щелчком.</small>}
          </section>
          <strong>Поиск по всему файлу</strong>
          <div style={{display:"flex",gap:6,marginTop:8}}>
            <input aria-label="Строка поиска" value={search} onChange={e=>setSearch(e.target.value)}
              onKeyDown={e=>{if(e.key==="Enter")void searchAll();}}
              placeholder="MOV, M010, #70010" style={{...field,minWidth:0,flex:1}}/>
            <button type="button" style={btn} onClick={()=>void searchAll()}
              disabled={searchBusy||search.trim().length<2}>Найти</button>
          </div>
          {!!hits.length&&<div style={{maxHeight:190,overflowY:"auto",marginTop:8}}>
            <small>{hits.length} совпадений {searchTruncated?"(лимит 100)":""}</small>
            {hits.map(h=><button type="button" key={h.line} style={itemStyle}
              title={h.preview} onClick={()=>void showLine(h.line)}>
              L{h.line}: {h.preview}
            </button>)}
          </div>}
          <hr style={{borderColor:"#3a4d5e"}}/>
          <div style={{display:"flex",justifyContent:"space-between",alignItems:"center",gap:5,flexWrap:"wrap"}}>
            <strong>Диагностика ({diags.length})</strong>
            <select aria-label="Фильтр сообщений" style={{...field,padding:4}} value={filter}
              onChange={e=>setFilter(e.target.value as typeof filter)}>
              <option value="all">Все</option><option value="Error">Ошибки</option>
              <option value="Warning">Предупр.</option>
            </select>
          </div>
          <label style={{display:"flex",gap:6,alignItems:"center",fontSize:11,marginTop:7}}>
            <input aria-label="Группировать диагностику" type="checkbox" checked={groupedDiagnostics}
              onChange={e=>setGroupedDiagnostics(e.target.checked)}/>
            Группировать по коду ({diagnosticGroups.length} групп)
          </label>
          <p style={{fontSize:11,color:"#ddb781"}}>
            Частичная семантика, не OEM-верификация. Группировка не исключает исходные строки.
          </p>
          <div style={{maxHeight:"45%",overflowY:"auto"}}>
            {groupedDiagnostics?
              diagnosticGroups.map(g=><details key={g.severity+":"+g.code}
                style={{border:"1px solid #33495a",borderRadius:4,marginBottom:5,padding:"4px 5px"}}>
                <summary style={{fontSize:11,cursor:"pointer",color:g.severity==="Error"?"#f4adb3":"#edc28a"}}>
                  {g.severity==="Error"?"ERROR":"WARN"} · {g.code} · {g.items.length} · первая L{g.firstLine}
                </summary>
                <div style={{maxHeight:240,overflowY:"auto",marginTop:6}}>
                  {g.items.map((d,i)=><button type="button" key={i}
                    onClick={()=>void showLine(d.line)}
                    style={{...itemStyle,whiteSpace:"normal",borderBottom:"1px solid #30465a"}}>
                    L{d.line}: {d.message}
                  </button>)}
                </div>
              </details>):
              diags.map((d,i)=><button type="button" key={i}
                onClick={()=>void showLine(d.line)}
                style={{...itemStyle,whiteSpace:"normal",borderBottom:"1px solid #314557"}}>
                <b style={{color:d.severity==="Error"?"#f7a8aa":"#e6be83"}}>
                  {d.severity==="Error"?"ERROR":"WARN"} · L{d.line}</b>
                <div>{d.code}</div><div style={{fontSize:11,color:"#b5c8d5"}}>{d.message}</div>
              </button>)}
            {!diags.length&&<small>В доступном подмножестве сообщений нет.</small>}
          </div>
          {(report.diagnostics.length>=500||report.semantic.diagnostics_truncated)&&
            <p style={{fontSize:11,color:"#e7bc83"}}>
              Backend ограничил число сообщений: часть диагностик может отсутствовать.
            </p>}
          <hr style={{borderColor:"#3a4d5e"}}/>
          <div>Выбрана строка: {selectedLine??"—"}</div>
          <div style={{marginTop:8,fontSize:11}}>
            Локальных комментариев: цепей {Object.values(annotations.rungs).filter(Boolean).length} ·
            адресов {Object.values(annotations.addresses).filter(Boolean).length}
          </div>
          <button type="button" onClick={reportAsJson} style={{...btn,marginTop:9,width:"100%"}}>
            JSON-отчёт диагностики
          </button>
          {json&&<textarea aria-label="JSON отчёт" readOnly value={json} rows={11}
            style={{...field,marginTop:8,width:"100%",boxSizing:"border-box",fontSize:11}}/>}
          <p style={{fontSize:11,color:"#d9b881"}}>
            В отчёте могут быть технологические адреса. Проверьте его перед отправкой.
          </p>
        </aside>
      </div>
      <footer style={{padding:"5px 12px",fontSize:11,color:"#b0c3d0",
        borderTop:"1px solid #364d5f",background:"#1b2b39"}}>
        SHA-256 исходника неизменен · Запись только отдельного *.mlad.json · Native CIOPRG/IONAME, FTP, HSES — только чтение/запрет
      </footer>
    </>}
  </main>;
}
