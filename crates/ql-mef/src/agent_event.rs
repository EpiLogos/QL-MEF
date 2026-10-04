//! QL-owned event projection. Providers consume the remaining semantic frame;
//! this module never invokes an inference runtime.

use crate::{
    ContextFrameId, LensId, LensRef, MEF_REGISTRY_VERSION, MUSICAL_HARMONIC_VERSION, MefRotation,
    MusicalBasis, all_lens_definitions, canonical_context_frame_progression, cf_diatonic_cut,
    directed_pitch_delta, lens_anchor, musical_completion_frame, pitch_at_lens,
};
use ql_core::{
    ConjugationDegree, ExpansionSide, QlCoordinate, QlFace, QlPosition, RelationFamily,
    STRUCTURAL_CONTRACT_VERSION, build_d_modulation_frame, classify_relation_pair,
    resolve_shape_ref,
};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

#[path = "agent_event/admission.rs"]
mod admission;
pub use admission::admit_decision;

pub const AGENT_PROJECTION_VERSION: &str = "ql.agent-projection/v1";

fn supplied_option<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    // An omitted optional property differs from an explicitly supplied null.
    T::deserialize(deserializer).map(Some)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRevision {
    pub r#ref: String,
    pub revision: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventFact {
    pub field: String,
    pub value: Value,
    pub origin: String,
    pub basis_refs: Vec<String>,
    #[serde(
        default,
        deserialize_with = "supplied_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub rule_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventMaterial {
    pub r#ref: String,
    pub revision: String,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct EventBindings {
    #[serde(
        default,
        deserialize_with = "supplied_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub body_ref: Option<String>,
    #[serde(
        default,
        deserialize_with = "supplied_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub session_ref: Option<String>,
    #[serde(
        default,
        deserialize_with = "supplied_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub native_event_refs: Option<Vec<String>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ConstellationMember {
    subject_ref: String,
    position: u8,
    face: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ConstellationState {
    anchor_ref: String,
    members: Vec<ConstellationMember>,
}

fn constellation(value: &Value) -> Result<ql_core::StructuralConstellation, String> {
    let state: ConstellationState =
        serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
    let members = state
        .members
        .into_iter()
        .map(|member| {
            let face = parse_face(&json!(member.face))?;
            let position = QlPosition::new(member.position).map_err(|e| e.to_string())?;
            ql_core::StructuralParticipation::new(member.subject_ref, position, face)
                .map_err(|e| e.to_string())
        })
        .collect::<Result<Vec<_>, String>>()?;
    ql_core::StructuralConstellation::new(state.anchor_ref, members, vec![])
        .map_err(|e| e.to_string())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgentEvent {
    pub schema: String,
    pub event_ref: String,
    pub generation: u64,
    pub occasion_refs: Vec<String>,
    pub source_basis: Vec<SourceRevision>,
    pub kind: String,
    pub subject: String,
    pub material: EventMaterial,
    pub observed: Vec<EventFact>,
    pub native_state: Vec<EventFact>,
    #[serde(
        default,
        deserialize_with = "supplied_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bindings: Option<EventBindings>,
    pub provenance_refs: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SemanticHead {
    Lens,
    ContextFrame,
    Faculty,
    Operation,
}

impl SemanticHead {
    pub const fn field(self) -> &'static str {
        match self {
            Self::Lens => "lens",
            Self::ContextFrame => "context-frame",
            Self::Faculty => "faculty",
            Self::Operation => "operation",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectionRequest {
    pub event: AgentEvent,
    /// The body requests only relevant heads. Empty means deterministic-only.
    #[serde(default)]
    pub requested_heads: Vec<SemanticHead>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EventProjection {
    pub schema: &'static str,
    pub event: AgentEvent,
    pub frame: Value,
    pub determination: Value,
    pub harmonic: Value,
    /// Input-required heads are preserved in the frame but never sent to a
    /// discriminator as though missing formal state were semantic evidence.
    pub decision_head_ids: Vec<String>,
    pub missing_inputs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DecisionProvider {
    pub provider_ref: String,
    pub model_ref: String,
    pub model_revision: String,
    pub runtime_revision: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceSpan {
    pub material_ref: String,
    pub revision: String,
    pub start: usize,
    pub end: usize,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LearnedProposal {
    pub head_id: String,
    pub label_ids: Vec<String>,
    #[serde(
        default,
        deserialize_with = "supplied_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub confidence: Option<f64>,
    pub spans: Vec<EvidenceSpan>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DecisionOutcome {
    Answered,
    Unavailable,
}

/// QL-owned response boundary. Provider adapters translate into this contract;
/// they cannot submit their own candidate field or kernel admission receipt.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DecisionResponse {
    pub schema: String,
    pub event_basis_digest: String,
    pub frame_digest: String,
    pub kernel_basis: Value,
    pub outcome: DecisionOutcome,
    #[serde(
        default,
        deserialize_with = "supplied_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub provider: Option<DecisionProvider>,
    pub proposals: Vec<LearnedProposal>,
    #[serde(
        default,
        deserialize_with = "supplied_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DecisionAdmission {
    pub schema: &'static str,
    /// The exact response remains inspectable even when stale or bypassed.
    pub response: DecisionResponse,
    pub admission_status: &'static str,
    pub projection: EventProjection,
}

fn nonempty(value: &str, what: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        Err(format!("empty {what}"))
    } else {
        Ok(())
    }
}

fn validate_event(event: &AgentEvent) -> Result<(), String> {
    if event.schema != "ql.agent-event/v1" {
        return Err("expected ql.agent-event/v1".into());
    }
    for (value, name) in [
        (&event.event_ref, "event ref"),
        (&event.kind, "event kind"),
        (&event.subject, "event subject"),
        (&event.material.r#ref, "material ref"),
        (&event.material.revision, "material revision"),
    ] {
        nonempty(value, name)?;
    }
    if event.source_basis.is_empty() || event.provenance_refs.is_empty() {
        return Err("source and provenance refs are required".into());
    }
    let mut sources = BTreeSet::new();
    for source in &event.source_basis {
        nonempty(&source.r#ref, "source ref")?;
        nonempty(&source.revision, "source revision")?;
        if !sources.insert(&source.r#ref) {
            return Err("duplicate source ref".into());
        }
    }
    for reference in event.occasion_refs.iter().chain(&event.provenance_refs) {
        nonempty(reference, "event reference")?;
    }
    if let Some(bindings) = &event.bindings {
        for value in [&bindings.body_ref, &bindings.session_ref]
            .into_iter()
            .flatten()
        {
            nonempty(value, "body/session binding")?;
        }
        for reference in bindings.native_event_refs.iter().flatten() {
            nonempty(reference, "native event ref")?;
        }
    }
    let mut fields = BTreeSet::new();
    for fact in event.observed.iter().chain(&event.native_state) {
        nonempty(&fact.field, "fact field")?;
        if fact.origin != "observed" {
            return Err("event facts must be observed".into());
        }
        if fact.basis_refs.is_empty() {
            return Err("fact lacks basis refs".into());
        }
        for reference in &fact.basis_refs {
            nonempty(reference, "fact basis ref")?;
        }
        if let Some(rule) = &fact.rule_ref {
            nonempty(rule, "fact rule ref")?;
        }
        if !fields.insert(&fact.field) {
            return Err(format!("duplicate observed field {}", fact.field));
        }
        match fact.field.as_str() {
            "lens" | "lens-conjugate" | "lens-complement" | "lens-mobius-partner" => {
                parse_lens(&fact.value)?;
            }
            "local-position"
            | "absolute-position"
            | "source-position"
            | "target-position"
            | "context-local-position"
            | "lens-position" => {
                position(&fact.value, &fact.field)?;
            }
            "lens-face" => {
                if !matches!(fact.value.as_str(), Some("day" | "night")) {
                    return Err("unknown lens face".into());
                }
            }
            "relation-family" => {
                relation_family(&fact.value)?;
            }
            "pair-index" => {
                pair_index(&fact.value)?;
            }
            "lens-anchor" | "pitch-class" | "context-frame-pitch" | "directed-interval" => {
                if !fact.value.as_u64().is_some_and(|n| n < 12) {
                    return Err(format!("invalid {}", fact.field));
                }
            }
            "coordinate-face" => {
                parse_face(&fact.value)?;
            }
            "musical-basis" => {
                basis(&fact.value)?;
            }
            "completion-degree" => {
                degree(&fact.value)?;
            }
            "d2-expansion" => {
                if !matches!(fact.value.as_str(), Some("source" | "target")) {
                    return Err("D2 expansion must name source or target".into());
                }
            }
            "context-frame" => {
                parse_cf(&fact.value)?;
            }
            "context-unit-face" => {
                if !matches!(fact.value.as_str(), Some("name" | "power")) {
                    return Err("unknown Context Frame unit face".into());
                }
            }
            "context-grain" => {
                if !matches!(fact.value.as_str(), Some("inner-four" | "outer-two")) {
                    return Err("unknown Context Frame grain".into());
                }
            }
            "shape" => {
                if resolve_shape_ref(&text(&fact.value, "shape")?).is_none() {
                    return Err("unknown or stale native shape ref".into());
                }
            }
            "faculty" => {
                faculty_position(&fact.value)?;
            }
            "constellation" => {
                constellation(&fact.value)?;
            }
            "operation" => {
                let operation = text(&fact.value, "operation")?;
                if !(0..6).any(|p| {
                    crate::epi_agent::native_operations(p)
                        .is_ok_and(|ops| ops.contains(&operation.as_str()))
                }) {
                    return Err("unknown native operation".into());
                }
            }
            _ => {}
        }
    }
    Ok(())
}

/// Exact sorted JSON byte basis. Native callers use owner-issued digests rather
/// than recomputing them in Prime or Pi.
pub fn value_digest(value: &Value) -> Result<String, String> {
    fn encode(value: &Value, bytes: &mut Vec<u8>) -> Result<(), String> {
        match value {
            Value::Object(object) => {
                bytes.push(b'{');
                let entries: BTreeMap<_, _> = object.iter().collect();
                for (i, (key, value)) in entries.into_iter().enumerate() {
                    if i != 0 {
                        bytes.push(b',');
                    }
                    bytes.extend(serde_json::to_vec(key).map_err(|e| e.to_string())?);
                    bytes.push(b':');
                    encode(value, bytes)?;
                }
                bytes.push(b'}');
            }
            Value::Array(items) => {
                bytes.push(b'[');
                for (i, item) in items.iter().enumerate() {
                    if i != 0 {
                        bytes.push(b',');
                    }
                    encode(item, bytes)?;
                }
                bytes.push(b']');
            }
            Value::Number(number) => {
                let spelling = number.to_string();
                if spelling.contains(['.', 'e', 'E']) {
                    let number = number
                        .as_f64()
                        .filter(|n| n.is_finite())
                        .ok_or("nonfinite event number")?;
                    let shortest = format!("{number:?}");
                    if let Some((mantissa, exponent)) = shortest.split_once('e') {
                        let exponent: i32 =
                            exponent.parse().map_err(|_| "invalid numeric exponent")?;
                        bytes.extend(
                            format!(
                                "{mantissa}e{}{:02}",
                                if exponent < 0 { "-" } else { "+" },
                                exponent.unsigned_abs()
                            )
                            .as_bytes(),
                        );
                    } else {
                        bytes.extend(shortest.as_bytes());
                    }
                } else {
                    bytes.extend(spelling.as_bytes());
                }
            }
            other => bytes.extend(serde_json::to_vec(other).map_err(|e| e.to_string())?),
        }
        Ok(())
    }
    let mut bytes = Vec::new();
    encode(value, &mut bytes)?;
    Ok(format!("sha256:{:x}", Sha256::digest(bytes)))
}

pub fn event_basis(event: &AgentEvent) -> Result<String, String> {
    validate_event(event)?;
    let mut value = serde_json::to_value(event).map_err(|error| error.to_string())?;
    value
        .as_object_mut()
        .ok_or("event must be an object")?
        .remove("bindings");
    value_digest(&value)
}

/// Fingerprints the compiled owner source; a runtime Git HEAD is not substituted
/// for the bytes that actually perform the determination.
pub fn kernel_basis() -> Value {
    let mut hash = Sha256::new();
    for (name, source) in [
        (
            "ql-core/position",
            include_str!("../../ql-core/src/position.rs"),
        ),
        ("ql-core/face", include_str!("../../ql-core/src/face.rs")),
        (
            "ql-core/structural",
            include_str!("../../ql-core/src/structural.rs"),
        ),
        (
            "ql-core/pairing",
            include_str!("../../ql-core/src/pairing.rs"),
        ),
        (
            "ql-core/relation-classification",
            include_str!("../../ql-core/src/relation_classification.rs"),
        ),
        ("ql-core/shape", include_str!("../../ql-core/src/shape.rs")),
        (
            "ql-core/shape-ref",
            include_str!("../../ql-core/src/shape_ref.rs"),
        ),
        ("ql-mef/lens", include_str!("lens.rs")),
        ("ql-mef/registry", include_str!("registry.rs")),
        (
            "ql-mef/registry/articulation",
            include_str!("registry/articulation.rs"),
        ),
        (
            "ql-mef/registry/encounter",
            include_str!("registry/encounter.rs"),
        ),
        (
            "ql-mef/registry/becoming",
            include_str!("registry/becoming.rs"),
        ),
        ("ql-mef/epi-agent", include_str!("epi_agent.rs")),
        ("ql-mef/coordinate", include_str!("coordinate.rs")),
        ("ql-mef/context-frame", include_str!("context_frame.rs")),
        ("ql-mef/music", include_str!("music.rs")),
        (
            "ql-mef/music-completion",
            include_str!("music_completion.rs"),
        ),
        ("ql-mef/agent-event", include_str!("agent_event.rs")),
        (
            "ql-mef/agent-admission",
            include_str!("agent_event/admission.rs"),
        ),
    ] {
        hash.update(name.as_bytes());
        hash.update([0]);
        hash.update(source.as_bytes());
        hash.update([0]);
    }
    json!({"owner_ref":"ql:owner:ql-mef:agent-event",
        "revision":format!("{AGENT_PROJECTION_VERSION};structural:{STRUCTURAL_CONTRACT_VERSION};mef:{MEF_REGISTRY_VERSION};music:{MUSICAL_HARMONIC_VERSION}"),
        "digest":format!("sha256:{:x}",hash.finalize())})
}

fn fact(field: &str, value: Value, event: &AgentEvent, rule: &str) -> EventFact {
    EventFact {
        field: field.into(),
        value,
        origin: "derived".into(),
        basis_refs: vec![event.event_ref.clone()],
        rule_ref: Some(rule.into()),
    }
}

fn add_fact(
    facts: &mut Vec<EventFact>,
    observed: &[EventFact],
    next: EventFact,
) -> Result<(), String> {
    if let Some(existing) = observed
        .iter()
        .chain(facts.iter())
        .find(|f| f.field == next.field)
    {
        let equivalent = match next.field.as_str() {
            "lens" | "lens-conjugate" | "lens-complement" | "lens-mobius-partner" => {
                parse_lens(&existing.value)? == parse_lens(&next.value)?
            }
            _ => existing.value == next.value,
        };
        if !equivalent {
            return Err(format!(
                "observed {} contradicts kernel derivation",
                next.field
            ));
        }
    } else {
        facts.push(next);
    }
    Ok(())
}

fn text(value: &Value, field: &str) -> Result<String, String> {
    value
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| format!("{field} must be a string"))
}

fn parse_lens(value: &Value) -> Result<LensId, String> {
    let reference = text(value, "lens")?;
    reference
        .parse::<LensId>()
        .or_else(|_| reference.parse::<LensRef>().map(|r| r.lens()))
        .map_err(|e| e.to_string())
}

fn parse_cf(value: &Value) -> Result<ContextFrameId, String> {
    let code = text(value, "context-frame")?;
    ContextFrameId::ALL
        .into_iter()
        .find(|cf| cf.code() == code)
        .ok_or_else(|| "unknown Context Frame".into())
}

fn relation_family(value: &Value) -> Result<RelationFamily, String> {
    [RelationFamily::A, RelationFamily::B, RelationFamily::C]
        .into_iter()
        .find(|family| value.as_str() == Some(family.as_str()))
        .ok_or_else(|| "unknown relation family".into())
}

fn pair_index(value: &Value) -> Result<u8, String> {
    value
        .as_u64()
        .filter(|n| *n < 3)
        .map(|n| n as u8)
        .ok_or_else(|| "invalid native relation pair index".into())
}

fn lens_candidates(values: &BTreeMap<&str, &Value>) -> Result<Vec<LensId>, String> {
    let mut candidates = LensId::ALL.to_vec();
    for (field, value) in values {
        candidates.retain(|lens| match *field {
            "lens" => parse_lens(value).is_ok_and(|v| v == *lens),
            "lens-position" => value.as_u64() == Some(u64::from(lens.index())),
            "lens-face" => {
                value.as_str()
                    == Some(match lens.face() {
                        crate::LensFace::Day => "day",
                        crate::LensFace::Night => "night",
                    })
            }
            "lens-conjugate" => parse_lens(value).is_ok_and(|v| v == lens.conjugate_twin()),
            "lens-complement" => parse_lens(value).is_ok_and(|v| v == lens.same_face_complement()),
            "lens-mobius-partner" => parse_lens(value).is_ok_and(|v| v == lens.mobius_partner()),
            _ => true,
        });
    }
    if let (Some(local), Some(absolute)) = (
        values.get("local-position"),
        values.get("absolute-position"),
    ) {
        let local = position(local, "local-position")?;
        let absolute = position(absolute, "absolute-position")?;
        candidates.retain(|lens| MefRotation::new(*lens, local).absolute_position() == absolute);
    }
    if let Some(musical_basis) = values.get("musical-basis") {
        let musical_basis = basis(musical_basis)?;
        if let Some(anchor) = values.get("lens-anchor").and_then(|v| v.as_u64()) {
            candidates.retain(|lens| u64::from(lens_anchor(musical_basis, *lens).pitch) == anchor);
        }
        if let (Some(pitch), Some(face)) = (
            values.get("pitch-class").and_then(|v| v.as_u64()),
            values.get("coordinate-face"),
        ) {
            let face = parse_face(face)?;
            let local = values
                .get("local-position")
                .map(|v| position(v, "local-position"))
                .transpose()?;
            let absolute = values
                .get("absolute-position")
                .map(|v| position(v, "absolute-position"))
                .transpose()?;
            candidates.retain(|lens| {
                let local = local.or_else(|| {
                    absolute.map(|absolute| {
                        MefRotation::from_absolute(*lens, absolute).local_position()
                    })
                });
                local.is_none_or(|local| {
                    u64::from(pitch_at_lens(
                        musical_basis,
                        *lens,
                        QlCoordinate::new(local, face),
                    )) == pitch
                })
            });
        }
    }
    if candidates.is_empty() {
        return Err("observed lens state contradicts kernel derivation".into());
    }
    Ok(candidates)
}

fn parse_face(value: &Value) -> Result<QlFace, String> {
    match value.as_str() {
        Some("direct") => Ok(QlFace::Direct),
        Some("conjugate") => Ok(QlFace::Conjugate),
        _ => Err("unknown coordinate face".into()),
    }
}

fn faculty_position(value: &Value) -> Result<u8, String> {
    let reference = text(value, "faculty")?;
    let position = reference
        .strip_prefix('#')
        .and_then(|s| s.parse::<u8>().ok())
        .filter(|p| *p < 6)
        .ok_or("unknown native faculty")?;
    if reference != format!("#{position}") {
        return Err("noncanonical native faculty".into());
    }
    Ok(position)
}

fn cf_candidates(
    values: &BTreeMap<&str, &Value>,
    lens: Option<LensId>,
    musical_basis: Option<MusicalBasis>,
) -> Vec<ContextFrameId> {
    canonical_context_frame_progression()
        .into_iter()
        .filter(|selection| {
            values
                .get("context-local-position")
                .is_none_or(|v| v.as_u64() == Some(u64::from(selection.local_position().value())))
                && values.get("context-unit-face").is_none_or(|v| {
                    v.as_str()
                        == Some(match selection.unit_face() {
                            crate::MefUnitFace::Name => "name",
                            crate::MefUnitFace::Power => "power",
                        })
                })
                && values.get("context-grain").is_none_or(|v| {
                    v.as_str()
                        == Some(match selection.grain() {
                            crate::MefGrain::InnerFour => "inner-four",
                            crate::MefGrain::OuterTwo => "outer-two",
                        })
                })
        })
        .map(|selection| selection.frame())
        .filter(|cf| {
            if let (Some(lens), Some(basis), Some(pitch)) = (
                lens,
                musical_basis,
                values.get("context-frame-pitch").and_then(|v| v.as_u64()),
            ) {
                let cut = cf_diatonic_cut(basis, lens);
                cut.frames
                    .iter()
                    .position(|candidate| candidate == cf)
                    .is_some_and(|i| u64::from(cut.pitches[i]) == pitch)
            } else {
                true
            }
        })
        .collect()
}

fn position(value: &Value, field: &str) -> Result<QlPosition, String> {
    let number = value
        .as_u64()
        .ok_or_else(|| format!("{field} must be an integer position"))?;
    let number = u8::try_from(number).map_err(|_| format!("invalid {field}"))?;
    QlPosition::new(number).map_err(|error| error.to_string())
}

fn degree(value: &Value) -> Result<ConjugationDegree, String> {
    match value.as_str() {
        Some("D1") => Ok(ConjugationDegree::D1),
        Some("D2") => Ok(ConjugationDegree::D2),
        Some("D3") => Ok(ConjugationDegree::D3),
        _ => Err("unknown completion degree".into()),
    }
}

fn basis(value: &Value) -> Result<MusicalBasis, String> {
    match value.as_str() {
        Some("chromatic") => Ok(MusicalBasis::Chromatic),
        Some("fifths") => Ok(MusicalBasis::Fifths),
        _ => Err("unknown musical basis".into()),
    }
}

fn coordinate_value(coordinate: QlCoordinate) -> Value {
    json!({"position":coordinate.position.value(),"face":coordinate.face.as_str()})
}

fn completion_value(
    family: RelationFamily,
    index: u8,
    completion: ConjugationDegree,
    side: Option<ExpansionSide>,
) -> Result<Value, String> {
    let field = build_d_modulation_frame(family, index, completion, side)
        .map_err(|error| error.to_string())?;
    Ok(
        json!({"family":family.as_str(),"pair_index":index,"degree":completion.as_str(),
        "expansion_side":side.map(ExpansionSide::as_str),"operator_ref":field.operator_ref(),
        "coordinates":field.coordinates.iter().copied().map(coordinate_value).collect::<Vec<_>>()}),
    )
}

fn owner_label(id: &str, description: &str, owner: &str, revision: &str) -> Value {
    json!({"id":id,"description":description,"owner_ref":owner,"owner_revision":revision})
}

fn input_head(field: &str, labels: Vec<Value>) -> Value {
    json!({"id":format!("input-{field}"),"field":field,"labels":labels,
        "cardinality":{"min":0,"max":1},"ambiguity_policy":"abstain"})
}

fn position_labels() -> Vec<Value> {
    (0..6)
        .map(|n| {
            let position = QlPosition::new(n).expect("the native six positional addresses");
            owner_label(
                &position.value().to_string(),
                &format!("Native QL position {}", position.value()),
                "ql:core:position",
                STRUCTURAL_CONTRACT_VERSION,
            )
        })
        .collect()
}

pub fn project_event(request: ProjectionRequest) -> Result<EventProjection, String> {
    project_fields(request, &[])
}

// Accepted semantic values enter the same deterministic projector, retaining
// their validated standing. They never become observations in the wire result.
fn project_fields(
    request: ProjectionRequest,
    admitted: &[EventFact],
) -> Result<EventProjection, String> {
    let event = request.event;
    validate_event(&event)?;
    let observed: Vec<_> = event
        .observed
        .iter()
        .chain(&event.native_state)
        .cloned()
        .collect();
    let effective: Vec<_> = observed.iter().chain(admitted).cloned().collect();
    let values: BTreeMap<_, _> = effective
        .iter()
        .map(|f| (f.field.as_str(), &f.value))
        .collect();
    let get = |field: &str| values.get(field).copied();
    let mut derived = Vec::new();
    let mut harmonic = Vec::new();
    let mut missing = Vec::new();
    let mut input_heads = Vec::new();
    let lens_candidates = lens_candidates(&values)?;
    let supplied_lens = get("lens").map(parse_lens).transpose()?;
    let lens = supplied_lens.or_else(|| (lens_candidates.len() == 1).then(|| lens_candidates[0]));
    let supplied_local = get("local-position")
        .map(|v| position(v, "local-position"))
        .transpose()?;
    let absolute = get("absolute-position")
        .map(|v| position(v, "absolute-position"))
        .transpose()?;
    let local = supplied_local.or_else(|| {
        lens.zip(absolute)
            .map(|(lens, absolute)| MefRotation::from_absolute(lens, absolute).local_position())
    });
    let musical_basis = get("musical-basis").map(basis).transpose()?;
    let add = |facts: &mut Vec<EventFact>, name: &str, value: Value, rule: &str| {
        add_fact(facts, &effective, fact(name, value, &event, rule))
    };
    let emit =
        |facts: &mut Vec<EventFact>, name: &str, value: Value, rule: &str| -> Result<(), String> {
            if effective
                .iter()
                .any(|f| f.field == name && f.value != value)
            {
                return Err(format!(
                    "observed {name} contradicts kernel harmonic derivation"
                ));
            }
            facts.push(fact(name, value, &event, rule));
            Ok(())
        };
    if let Some(basis) = musical_basis {
        let ratio = basis.generator_ratio();
        emit(
            &mut harmonic,
            "basis-generator-ratio",
            json!({"numerator":ratio.numerator(),"denominator":ratio.denominator()}),
            "ql:music:1.0.0:generator-ratio",
        )?;
    }
    if let Some(lens) = lens {
        let rule = format!("{}:lens", LensRef::canonical(lens));
        if supplied_lens.is_none() {
            add(
                &mut derived,
                "lens",
                json!(lens.code()),
                "ql:mef:lens:1.0.0:candidate-inverse",
            )?;
        }
        add(&mut derived, "lens-position", json!(lens.index()), &rule)?;
        add(
            &mut derived,
            "lens-face",
            json!(match lens.face() {
                crate::LensFace::Day => "day",
                crate::LensFace::Night => "night",
            }),
            &rule,
        )?;
        add(
            &mut derived,
            "lens-conjugate",
            json!(lens.conjugate_twin().code()),
            &rule,
        )?;
        add(
            &mut derived,
            "lens-complement",
            json!(lens.same_face_complement().code()),
            &rule,
        )?;
        add(
            &mut derived,
            "lens-mobius-partner",
            json!(lens.mobius_partner().code()),
            &rule,
        )?;
        if let Some(local) = local {
            if supplied_local.is_none() {
                add(
                    &mut derived,
                    "local-position",
                    json!(local.value()),
                    "ql:mef:rotation:1.0.0",
                )?;
            }
            add(
                &mut derived,
                "absolute-position",
                json!(MefRotation::new(lens, local).absolute_position().value()),
                "ql:mef:rotation:1.0.0",
            )?;
        }
        if let Some(basis) = musical_basis {
            let rule = format!("ql:music:{MUSICAL_HARMONIC_VERSION}:lens-anchor");
            emit(
                &mut harmonic,
                "lens-anchor",
                json!(lens_anchor(basis, lens).pitch),
                &rule,
            )?;
            if let Some(local) = local {
                let face = get("coordinate-face").map(parse_face).transpose()?;
                if let Some(face) = face {
                    emit(
                        &mut harmonic,
                        "pitch-class",
                        json!(pitch_at_lens(basis, lens, QlCoordinate::new(local, face))),
                        &rule,
                    )?;
                }
            }
        }
    } else if musical_basis.is_some() || absolute.is_some() {
        missing.push("lens is required for lens-relative harmonics or rotation".into());
        input_heads.push(input_head(
            "lens",
            all_lens_definitions()
                .iter()
                .filter(|definition| lens_candidates.contains(&definition.id()))
                .map(|definition| {
                    owner_label(
                        definition.id().code(),
                        definition.name(),
                        &definition.reference().to_string(),
                        MEF_REGISTRY_VERSION,
                    )
                })
                .collect(),
        ));
    }

    if musical_basis.is_some()
        && (local.is_some() || absolute.is_some())
        && get("coordinate-face").is_none()
    {
        missing.push("coordinate-face is required for a local pitch".into());
        input_heads.push(input_head(
            "coordinate-face",
            [QlFace::Direct, QlFace::Conjugate]
                .into_iter()
                .map(|face| {
                    owner_label(
                        face.as_str(),
                        face.as_str(),
                        "ql:core:face",
                        STRUCTURAL_CONTRACT_VERSION,
                    )
                })
                .collect(),
        ));
    }

    let context_candidates = cf_candidates(&values, lens, musical_basis);
    if context_candidates.is_empty() {
        return Err("observed Context Frame structure has no native reading".into());
    }
    let supplied_cf = get("context-frame").map(parse_cf).transpose()?;
    let context_frame =
        supplied_cf.or_else(|| (context_candidates.len() == 1).then(|| context_candidates[0]));
    if let Some(cf) = context_frame {
        if !context_candidates.contains(&cf) {
            return Err("observed Context Frame contradicts its native structural state".into());
        }
        if supplied_cf.is_none() {
            add(
                &mut derived,
                "context-frame",
                json!(cf.code()),
                "ql:mef:context-frame:1.0.0:canonical-progression",
            )?;
        }
    }
    if let Some(cf) = context_frame {
        let selection = cf.canonical_selection();
        let rule = format!("ql:mef:context-frame:1.0.0:{}", cf.code());
        add(
            &mut derived,
            "context-local-position",
            json!(selection.local_position().value()),
            &rule,
        )?;
        add(
            &mut derived,
            "context-unit-face",
            json!(format!("{:?}", selection.unit_face()).to_lowercase()),
            &rule,
        )?;
        add(
            &mut derived,
            "context-grain",
            json!(match selection.grain() {
                crate::MefGrain::InnerFour => "inner-four",
                crate::MefGrain::OuterTwo => "outer-two",
            }),
            &rule,
        )?;
        if let (Some(lens), Some(basis)) = (lens, musical_basis) {
            let cut = cf_diatonic_cut(basis, lens);
            let index = cut
                .frames
                .iter()
                .position(|candidate| *candidate == cf)
                .ok_or("CF absent from native diatonic cut")?;
            emit(
                &mut harmonic,
                "context-frame-pitch",
                json!(cut.pitches[index]),
                "ql:music:1.0.0:cf-diatonic-cut",
            )?;
        }
    }
    if let Some(value) = get("constellation") {
        let constellation = constellation(value)?;
        let shape = ql_core::QlShape::Constellation(constellation.grain());
        let rule = format!("ql:structural:{STRUCTURAL_CONTRACT_VERSION}:constellation:grain");
        let mut grain = json!({"kind":constellation.grain().as_str()});
        if let ql_core::ConstellationGrain::Other { direct, conjugate } = constellation.grain() {
            grain["direct"] = json!(direct);
            grain["conjugate"] = json!(conjugate);
        }
        add(&mut derived, "constellation-grain", grain, &rule)?;
        // Anonymous native grains have member structure but no resolvable public
        // shape reference. A derived reference must satisfy the same owner law
        // as a supplied reference.
        if resolve_shape_ref(&shape.shape_ref()).is_some() {
            add(&mut derived, "shape", json!(shape.shape_ref()), &rule)?;
        } else if get("shape").is_some() {
            return Err(
                "supplied shape has no canonical reading of these constellation members".into(),
            );
        }
        if let Some(folds) = shape.fold_count() {
            add(
                &mut derived,
                "shape-fold-count",
                json!(folds),
                &shape.shape_ref(),
            )?;
        }
        if let Some(compression) = ql_core::QlShapeCompression::to_onefold(shape) {
            add(
                &mut derived,
                "shape-compression",
                json!({"presented_shape":compression.presented.shape_ref(),
                "disclosed_shape":compression.disclosed.shape_ref(),"operator_ref":compression.operator_ref,
                "recognition_superset":compression.recognition_superset.map(|shape|shape.shape_ref())}),
                &compression.derivation_ref(),
            )?;
        }
        if let (Some(lens), Some(basis)) = (lens, musical_basis) {
            emit(&mut harmonic,"constellation-pitches",json!(constellation.members.iter().map(|member|
                json!({"subject_ref":member.subject_ref,"coordinate":coordinate_value(member.coordinate),
                    "pitch_class":pitch_at_lens(basis,lens,member.coordinate)})).collect::<Vec<_>>()),"ql:music:1.0.0:pitch-at-lens")?;
        }
    }
    if let Some(reference) = get("shape") {
        let reference = text(reference, "shape")?;
        let shape = resolve_shape_ref(&reference).ok_or("unknown or stale native shape ref")?;
        if let Some(folds) = shape.fold_count() {
            add(
                &mut derived,
                "shape-fold-count",
                json!(folds),
                &shape.shape_ref(),
            )?;
        }
    }

    let mut source = get("source-position")
        .map(|v| position(v, "source-position"))
        .transpose()?;
    let mut target = get("target-position")
        .map(|v| position(v, "target-position"))
        .transpose()?;
    let selected_family = get("relation-family").map(relation_family).transpose()?;
    let selected_index = get("pair-index").map(pair_index).transpose()?;
    let requested_degree = get("completion-degree").map(degree).transpose()?;
    let side = get("d2-expansion").map(|v| v.as_str() == Some("target"));
    if side.is_some() && requested_degree != Some(ConjugationDegree::D2) {
        return Err("D2 side cannot accompany another or absent completion degree".into());
    }
    if let (Some(family), Some(index)) = (selected_family, selected_index) {
        let pair = family.pair(index).map_err(|e| e.to_string())?;
        let other = |given: QlPosition| {
            if given == pair.left {
                Ok(pair.right)
            } else if given == pair.right {
                Ok(pair.left)
            } else {
                Err("observed endpoint is outside supplied native pair")
            }
        };
        if let (Some(given), None) = (source, target) {
            target = Some(other(given)?);
            add(
                &mut derived,
                "target-position",
                json!(target.ok_or("target completion missing")?.value()),
                &pair.operator_ref(),
            )?;
        } else if let (None, Some(given)) = (source, target) {
            source = Some(other(given)?);
            add(
                &mut derived,
                "source-position",
                json!(source.ok_or("source completion missing")?.value()),
                &pair.operator_ref(),
            )?;
        }
    }
    if source.is_some() != target.is_some() {
        missing
            .push("both source-position and target-position are required for a traversal".into());
        input_heads.push(input_head(
            if source.is_none() {
                "source-position"
            } else {
                "target-position"
            },
            position_labels(),
        ));
    }
    let mut selected_relations = Vec::new();
    if let (Some(source), Some(target)) = (source, target) {
        let matches = classify_relation_pair(source, target);
        let relations: Vec<_> = matches.iter().map(|r| json!({"family":r.family.as_str(),"pair_index":r.pair_index,"reversed":r.reversed})).collect();
        add(
            &mut derived,
            "relation-candidates",
            json!(relations),
            "ql:core:relation-classification:1.0.0",
        )?;
        selected_relations = matches
            .into_iter()
            .filter(|r| {
                selected_family.is_none_or(|f| f == r.family)
                    && selected_index.is_none_or(|i| i == r.pair_index)
            })
            .map(|r| (r.family, r.pair_index, Some(r.reversed)))
            .collect();
        if (selected_family.is_some() || selected_index.is_some()) && selected_relations.is_empty()
        {
            return Err(
                "observed relation selection contradicts native traversal candidates".into(),
            );
        }
    } else if let (Some(family), Some(index)) = (selected_family, selected_index) {
        selected_relations.push((family, index, None));
    } else if requested_degree.is_some() || selected_family.is_some() || selected_index.is_some() {
        missing.push(
            "completion requires a positional traversal or exact native relation pair".into(),
        );
        for (field, present) in [
            ("source-position", source.is_some()),
            ("target-position", target.is_some()),
        ] {
            if !present {
                input_heads.push(input_head(field, position_labels()));
            }
        }
    }
    if !selected_relations.is_empty() {
        let mut completions = Vec::new();
        let mut musical_completions = Vec::new();
        for (family, index, reversed) in selected_relations {
            let alternatives: Vec<_> = match requested_degree {
                None => vec![
                    (ConjugationDegree::D1, None),
                    (ConjugationDegree::D2, Some(ExpansionSide::Left)),
                    (ConjugationDegree::D2, Some(ExpansionSide::Right)),
                    (ConjugationDegree::D3, None),
                ],
                Some(ConjugationDegree::D2) => {
                    if let (Some(target_side), Some(reversed)) = (side, reversed) {
                        let canonical_right = target_side != reversed;
                        vec![(
                            ConjugationDegree::D2,
                            Some(if canonical_right {
                                ExpansionSide::Right
                            } else {
                                ExpansionSide::Left
                            }),
                        )]
                    } else {
                        vec![
                            (ConjugationDegree::D2, Some(ExpansionSide::Left)),
                            (ConjugationDegree::D2, Some(ExpansionSide::Right)),
                        ]
                    }
                }
                Some(degree) => vec![(degree, None)],
            };
            for (degree, side) in alternatives {
                let mut formal = completion_value(family, index, degree, side)?;
                if let Some(reversed) = reversed {
                    formal["reversed"] = json!(reversed);
                }
                if let (Some(lens), Some(basis)) = (lens, musical_basis) {
                    let musical =
                        musical_completion_frame(basis, lens, family, index, degree, side)
                            .map_err(|error| error.to_string())?;
                    let mut reading = json!({"operator_ref":musical.structural_operator_ref,"pitches":musical.pitches});
                    if let Some(reversed) = reversed {
                        reading["reversed"] = json!(reversed);
                    }
                    musical_completions.push(reading);
                }
                completions.push(formal);
            }
        }
        if requested_degree == Some(ConjugationDegree::D2)
            && (side.is_none() || source.is_none() || target.is_none())
        {
            missing.push("D2 traversal side is unresolved; both native expansions retained".into());
            if side.is_none() {
                input_heads.push(input_head(
                    "d2-expansion",
                    ["source", "target"]
                        .into_iter()
                        .map(|side| {
                            owner_label(
                                side,
                                side,
                                "ql:music:completion:1.0.0:traversal-expansion-side",
                                "1.0.0",
                            )
                        })
                        .collect(),
                ));
            }
            if source.is_none() || target.is_none() {
                for (field, present) in [
                    ("source-position", source.is_some()),
                    ("target-position", target.is_some()),
                ] {
                    if !present {
                        input_heads.push(input_head(field, position_labels()));
                    }
                }
            }
        }
        add(
            &mut derived,
            "completion-candidates",
            json!(completions),
            "ql:core:pairing:1.0.0",
        )?;
        if !musical_completions.is_empty() {
            emit(
                &mut harmonic,
                "completion-pitches",
                json!(musical_completions),
                "ql:music:completion:1.0.0",
            )?;
        }
    } else if source.is_some() && target.is_some() {
        add(
            &mut derived,
            "completion-candidates",
            json!([]),
            "ql:core:pairing:1.0.0",
        )?;
    }
    if let (Some(source), Some(target), Some(lens), Some(basis), Some(face)) = (
        source,
        target,
        lens,
        musical_basis,
        get("coordinate-face").and_then(Value::as_str),
    ) {
        let face = match face {
            "direct" => QlFace::Direct,
            "conjugate" => QlFace::Conjugate,
            _ => return Err("unknown coordinate face".into()),
        };
        let from = pitch_at_lens(basis, lens, QlCoordinate::new(source, face));
        let to = pitch_at_lens(basis, lens, QlCoordinate::new(target, face));
        emit(
            &mut harmonic,
            "directed-interval",
            json!(directed_pitch_delta(from, to)),
            "ql:music:1.0.0:directed-pitch-delta",
        )?;
    }

    let kernel = kernel_basis();
    let observed_faculty = get("faculty").map(faculty_position).transpose()?;
    let observed_operation = get("operation").map(|v| text(v, "operation")).transpose()?;
    let mut faculty = observed_faculty;
    if let Some(operation) = &observed_operation {
        let owners: Vec<_> = (0..6)
            .filter(|position| {
                crate::epi_agent::native_operations(*position)
                    .is_ok_and(|operations| operations.contains(&operation.as_str()))
            })
            .collect();
        if let Some(position) = observed_faculty {
            if !owners.contains(&position) {
                return Err("observed operation is not admitted for observed faculty".into());
            }
        } else if owners.len() == 1 {
            faculty = Some(owners[0]);
            add(
                &mut derived,
                "faculty",
                json!(format!("#{}", owners[0])),
                "ql:epi-agent:native-operation-membership:v1",
            )?;
        }
    }
    let mut heads = Vec::new();
    let mut constraints = Vec::new();
    let mut requested = BTreeSet::new();
    for kind in request.requested_heads {
        if !requested.insert(kind.field()) {
            return Err("duplicate requested head".into());
        }
        if observed
            .iter()
            .chain(&derived)
            .any(|fact| fact.field == kind.field())
        {
            continue;
        }
        let labels = match kind {
            SemanticHead::Lens => all_lens_definitions().iter().filter(|definition|lens_candidates.contains(&definition.id())).map(|definition| json!({"id":definition.id().code(),"description":definition.name(),"owner_ref":definition.reference().to_string(),"owner_revision":kernel["digest"]})).collect::<Vec<_>>(),
            SemanticHead::ContextFrame => {
                context_candidates.iter().map(|cf| json!({"id":cf.code(),"description":cf.name(),
                    "owner_ref":format!("ql:mef:context-frame:1.0.0:{}",cf.code()),"owner_revision":kernel["digest"]})).collect()
            }
            SemanticHead::Faculty | SemanticHead::Operation => {
                let mut labels = Vec::new();
                for &(position,name) in crate::epi_agent::native_faculties() {
                    if kind == SemanticHead::Faculty {
                        labels.push(json!({"id":format!("#{position}"),"description":name,"owner_ref":"ql:epi-agent:constitution:v1","owner_revision":kernel["digest"]}));
                    } else {
                        if faculty.is_some_and(|selected|selected != position) { continue; }
                        for reference in crate::epi_agent::native_operations(position)? {
                            labels.push(json!({"id":reference,"description":format!("{name}: {reference}"),"owner_ref":"ql:epi-agent:native-operations:v1","owner_revision":kernel["digest"]}));
                        }
                    }
                }
                labels
            }
        };
        if labels.is_empty() {
            return Err(format!("no legal native candidates for {}", kind.field()));
        }
        let multi = matches!(kind, SemanticHead::Lens | SemanticHead::ContextFrame);
        if kind == SemanticHead::ContextFrame && labels.len() < ContextFrameId::ALL.len() {
            constraints.push(json!({"kind":"candidate-restriction","id":"native-context-candidates",
                "rule_ref":"ql:mef:context-frame:1.0.0:canonical-progression",
                "allowed":{"head_id":"semantic-context-frame","label_ids":labels.iter().map(|label|label["id"].clone()).collect::<Vec<_>>(),"match":"all"}}));
        }
        heads.push(json!({"id":format!("semantic-{}",kind.field()),"field":kind.field(),"labels":labels,
            "cardinality":{"min":0,"max":if multi{labels.len()}else{1}},"ambiguity_policy":if multi{"preserve-candidates"}else{"abstain"}}));
    }
    if heads.iter().any(|h| h["field"] == "faculty")
        && heads.iter().any(|h| h["field"] == "operation")
    {
        let selection = |head: &str, labels: Vec<Value>| json!({"head_id":head,"label_ids":labels,"match":"exact"});
        let mut tuples = vec![json!([
            selection("semantic-faculty", vec![]),
            selection("semantic-operation", vec![])
        ])];
        for position in 0..6 {
            let faculty = json!(format!("#{position}"));
            tuples.push(json!([
                selection("semantic-faculty", vec![faculty.clone()]),
                selection("semantic-operation", vec![])
            ]));
            for operation in crate::epi_agent::native_operations(position)? {
                tuples.push(json!([
                    selection("semantic-faculty", vec![faculty.clone()]),
                    selection("semantic-operation", vec![json!(operation)])
                ]));
                tuples.push(json!([
                    selection("semantic-faculty", vec![]),
                    selection("semantic-operation", vec![json!(operation)])
                ]));
            }
        }
        constraints.push(
            json!({"kind":"legal-combination","id":"native-faculty-operation",
            "rule_ref":"ql:epi-agent:native-operation-membership:v1","allowed_tuples":tuples}),
        );
    }
    let decision_head_ids = if event.material.text.trim().is_empty() {
        Vec::new()
    } else {
        heads
            .iter()
            .filter_map(|head| head["id"].as_str().map(str::to_owned))
            .collect()
    };
    for head in input_heads {
        if !heads
            .iter()
            .any(|existing| existing["field"] == head["field"])
        {
            heads.push(head);
        }
    }
    let frame = json!({"schema":"ql.agent-decision-frame/v1","frame_ref":format!("{}:ql-frame:{}",event.event_ref,event.generation),
        "event_ref":event.event_ref,"event_basis_digest":event_basis(&event)?,"source_basis":event.source_basis,
        "kernel_basis":kernel,"determined":{"observed":observed,"derived":derived},"unresolved":heads,
        "constraints":constraints,"requested_evidence":["semantic-span"]});
    let unresolved: Vec<_> = frame["unresolved"].as_array().ok_or("frame has no heads")?.iter().map(|head| json!({"head_id":head["id"],"reason":"semantic evidence has not been determined; no provider invoked"})).collect();
    let determination = json!({"schema":"ql.agent-determination/v1","event_ref":event.event_ref,
        "event_basis_digest":frame["event_basis_digest"],"frame_digest":value_digest(&frame)?,
        "source_basis":event.source_basis,"kernel_basis":kernel,
        "status":if unresolved.is_empty(){"determined"}else{"unresolved"},
        "observed":observed,"derived":derived,"learned":[],"validated":[],"unresolved":unresolved,
        "refused_candidates":[],"constraint_application":constraints.iter().map(|constraint|json!({"constraint_id":constraint["id"],"result":"not-applicable","reason":"no learned result supplied"})).collect::<Vec<_>>()});
    let formal: Vec<_> = observed.iter().chain(&derived).cloned().collect();
    let harmonic = json!({"schema":"ql.harmonic-event/v1","event_ref":event.event_ref,
        "source_basis":event.source_basis,"kernel_basis":kernel,"determination_digest":value_digest(&determination)?,
        "formal":formal,"harmonic":harmonic});
    Ok(EventProjection {
        schema: AGENT_PROJECTION_VERSION,
        event,
        frame,
        determination,
        harmonic,
        decision_head_ids,
        missing_inputs: missing,
    })
}
