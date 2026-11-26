//! Semantic Core v2: source-linked engineering facade over the existing
//! lossless parser and specialized analyzers. No second parser or native writer.
use crate::compiler::{advanced_source, lossless, memory, native_sections, semantic, static_access, typed_dataflow};
use serde::Serialize;

pub(crate) const SCHEMA_VERSION: u32 = 1;
const RUNG_PREVIEW: usize = 200;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct SourceIdentity {
    pub(crate) source_sha256: String,
    pub(crate) controller: memory::ControllerFamily,
    pub(crate) byte_length: usize,
    pub(crate) line_count: usize,
}
impl SourceIdentity {
    fn from_document(doc: &lossless::LosslessDocument) -> Self {
        Self {
            source_sha256: doc.source_sha256.clone(),
            controller: doc.controller,
            byte_length: doc.original_bytes().len(),
            line_count: doc.lines.len(),
        }
    }
}

/// Half-open ORIGINAL byte range; physical lines are one-based and inclusive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct SourceSpan {
    pub(crate) first_line: usize,
    pub(crate) last_line: usize,
    pub(crate) bytes: lossless::ByteSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct RungReference {
    pub(crate) id: String,
    pub(crate) number: usize,
    pub(crate) source: SourceSpan,
}

/// Claims are scoped to these analyzers, not controller execution or access.
#[derive(Debug, Serialize)]
pub(crate) struct Evidence {
    pub(crate) boolean_subset_complete: bool,
    pub(crate) unsupported_segments: usize,
    pub(crate) execution_verified: bool,
    pub(crate) scan_order_verified: bool,
    pub(crate) native_write_allowed: bool,
}

#[derive(Debug, Serialize)]
pub(crate) struct EngineeringCore {
    pub(crate) schema_version: u32,
    pub(crate) identity: SourceIdentity,
    pub(crate) rung_count: usize,
    pub(crate) rung_refs: Vec<RungReference>,
    pub(crate) rung_refs_truncated: bool,
    pub(crate) evidence: Evidence,
}

impl EngineeringCore {
    fn build(doc: &lossless::LosslessDocument, report: &semantic::SemanticReport) -> Self {
        let identity = SourceIdentity::from_document(doc);
        let rung_refs = report.rungs.iter().take(RUNG_PREVIEW).filter_map(|rung| {
            let first = doc.lines.get(rung.first_line.checked_sub(1)?)?;
            let last = doc.lines.get(rung.output_line.checked_sub(1)?)?;
            if first.number != rung.first_line || last.number != rung.output_line ||
                first.number > last.number || first.body_span.start > last.body_span.end ||
                last.body_span.end > identity.byte_length {
                return None;
            }
            let bytes = lossless::ByteSpan { start: first.body_span.start, end: last.body_span.end };
            // Snapshot-scoped, deterministic; not a cross-version matching key.
            let id = format!("{:?}:{}:rung:{}:{}", identity.controller,
                identity.source_sha256, bytes.start, bytes.end);
            Some(RungReference {
                id, number: rung.number,
                source: SourceSpan { first_line: first.number, last_line: last.number, bytes },
            })
        }).collect::<Vec<_>>();
        EngineeringCore {
            schema_version: SCHEMA_VERSION,
            identity,
            rung_count: report.rungs.len(),
            rung_refs_truncated: rung_refs.len() < report.rungs.len(),
            rung_refs,
            evidence: Evidence {
                boolean_subset_complete: report.subset_analysis_complete,
                unsupported_segments: report.unsupported_segments,
                execution_verified: false,
                scan_order_verified: false,
                native_write_allowed: false,
            },
        }
    }
}

/// Single read-only projection from the existing AST/IR and specialized reports.
/// The legacy presentation fields remain during the staged consumer migration.
/// No raw bytes or additional full IR copy cross IPC.
#[derive(Serialize)]
pub(crate) struct FullCioprgInspection {
    #[serde(flatten)]
    pub(crate) base: lossless::InspectionReport,
    pub(crate) semantic: semantic::SemanticPreview,
    pub(crate) advanced: advanced_source::AdvancedSourceReport,
    pub(crate) sections: Vec<native_sections::SourceSectionMarker>,
    pub(crate) dataflow: typed_dataflow::StaticAudit,
    pub(crate) static_access: static_access::AccessOverview,
    pub(crate) core: EngineeringCore,
}

pub(crate) fn inspect(doc: &lossless::LosslessDocument) -> FullCioprgInspection {
    let semantic = semantic::analyze(doc);
    let core = EngineeringCore::build(doc, &semantic);
    FullCioprgInspection {
        base: doc.inspection_report(600, 500),
        semantic: semantic.preview(RUNG_PREVIEW, 200, 500),
        advanced: advanced_source::inspect(doc),
        sections: native_sections::scan(doc),
        dataflow: typed_dataflow::inspect(doc, 1000),
        static_access: static_access::inspect(doc,160),
        core,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use memory::ControllerFamily;
    fn analyze(raw: &[u8], family: ControllerFamily) -> FullCioprgInspection {
        inspect(&lossless::parse_document(raw.to_vec(), family))
    }

    #[test]
    fn identity_and_ranges_are_exact_and_repeatable() {
        let raw = b"// header\r\nSTR #70010\r\nOUT #70020\nEND\n";
        let result = analyze(raw, ControllerFamily::YRC1000);
        let again = analyze(raw, ControllerFamily::YRC1000);
        assert_eq!(result.core.identity, again.core.identity);
        assert_eq!(result.core.rung_refs, again.core.rung_refs);
        assert_eq!(result.core.rung_count, 1);
        let link = &result.core.rung_refs[0];
        assert_eq!((link.source.first_line, link.source.last_line), (2, 3));
        assert_eq!(&raw[link.source.bytes.start..link.source.bytes.end],
            b"STR #70010\r\nOUT #70020");
        assert_eq!(result.base.source_sha256, result.core.identity.source_sha256);
        assert_eq!(result.static_access.source_sha256, result.core.identity.source_sha256);
        assert_eq!(result.dataflow.source_sha256, result.core.identity.source_sha256);
        assert_eq!(result.advanced.source_sha256, result.core.identity.source_sha256);
        let changed_profile = analyze(raw, ControllerFamily::DX200);
        assert_ne!(result.core.identity, changed_profile.core.identity);
        assert_ne!(link.id, changed_profile.core.rung_refs[0].id);
        let changed_bytes = analyze(b"STR #70011\nOUT #70020\nEND\n", ControllerFamily::YRC1000);
        assert_ne!(link.id, changed_bytes.core.rung_refs[0].id);
    }

    #[test]
    fn unknown_and_post_end_never_gain_executable_semantics() {
        let result = analyze(b"STR #70010\nBMOX M000,M010\nOUT #70020\nSTR #70011\nOUT #70021\nEND\nSTR #70012\nOUT #70022\n", ControllerFamily::YRC1000);
        assert_eq!(result.core.rung_count, 1);
        assert!(result.core.evidence.unsupported_segments > 0);
        assert!(!result.core.evidence.boolean_subset_complete);
        assert!(!result.core.evidence.execution_verified);
        assert!(!result.core.evidence.scan_order_verified);
        assert!(!result.core.evidence.native_write_allowed);
        assert_eq!(result.core.rung_refs[0].source.last_line, 5);
    }

    #[test]
    fn empty_and_large_previews_are_explicit_and_bounded() {
        let empty = analyze(b"", ControllerFamily::DX100);
        assert_eq!(empty.core.rung_count, 0);
        assert!(empty.core.rung_refs.is_empty());
        assert!(!empty.core.rung_refs_truncated);
        let mut raw = "STR #70010\nOUT #70020\n".repeat(201);
        raw.push_str("END\n");
        let large = analyze(raw.as_bytes(), ControllerFamily::YRC1000);
        assert_eq!(large.core.rung_count, 201);
        assert_eq!(large.core.rung_refs.len(), RUNG_PREVIEW);
        assert!(large.core.rung_refs_truncated);
        assert!(large.semantic.rungs_truncated);
        assert_eq!(large.core.rung_refs.len(), large.semantic.rungs.len());
        assert_eq!(large.core.rung_refs.iter().map(|r| &r.id).collect::<std::collections::HashSet<_>>().len(), RUNG_PREVIEW);
    }

    #[test]
    fn versioned_wire_shape_matches_frontend_contract() {
        let result = analyze(b"STR #70010\nOUT #70020\nEND\n", ControllerFamily::YRC1000);
        let wire = serde_json::to_value(&result).unwrap();
        assert_eq!(wire["core"]["schema_version"], 1);
        assert_eq!(wire["core"]["identity"]["controller"], "YRC1000");
        assert_eq!(wire["core"]["identity"]["source_sha256"], wire["source_sha256"]);
        assert_eq!(wire["core"]["rung_count"], wire["semantic"]["rung_count"]);
        assert_eq!(wire["core"]["evidence"]["execution_verified"], false);
        assert_eq!(wire["core"]["rung_refs"][0]["source"]["bytes"]["start"], 0);
        assert_eq!(wire["core"]["rung_refs"][0]["number"], wire["semantic"]["rungs"][0]["number"]);
    }
}
