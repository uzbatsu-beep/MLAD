/** Source-linked IPC DTOs. Rust is the canonical analysis authority. */
export type Controller = "DX100" | "DX200" | "YRC1000";
export type ByteSpan = {start:number;end:number};
export type SourceIdentity = {source_sha256:string;controller:Controller;byte_length:number;line_count:number};
export type SourceSpan = {first_line:number;last_line:number;bytes:ByteSpan};
export type RungReference = {id:string;number:number;source:SourceSpan};
export type EngineeringCore = {
  schema_version:1;
  identity:SourceIdentity;
  rung_count:number;
  rung_refs:RungReference[];
  rung_refs_truncated:boolean;
  evidence:{
    boolean_subset_complete:boolean;
    unsupported_segments:number;
    execution_verified:false;
    scan_order_verified:false;
    native_write_allowed:false;
  };
};
export type SemanticNode = {
  op: { Contact: { relay: number; inverted: boolean } }
    | { And: { left: number; right: number } }
    | { Or: { left: number; right: number } };
  line: number;
  span: ByteSpan;
};

export type SemanticDiagnostic = {
  code: string;
  level: "Warning" | "Error";
  line: number;
  message: string;
};
export type SemanticRung = {
  number: number;
  first_line: number;
  output_line: number;
  output: number;
  inputs: number[];
  expression_preview: string;
  expression_root: number;
  section: "Unspecified" | "System" | "User";
};
export type SemanticPreview = {
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

