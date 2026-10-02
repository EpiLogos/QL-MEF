//! Source-qualified manifestation and Atlas plans over the native registry.
//!
//! Paśu is a bounded entity, not the three current Central participation
//! forms. This module never creates subjects, a graph, a scene store or a
//! clock. The supplied owner readings determine subjects and relations;
//! occurrences and native edit application remain O:I's responsibility.
use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::MFace;
use crate::coordinate_expression::{CoordinateExpressionBinding, resolve_coordinate_expression};
use crate::m_tree::{MRegistry, MTreeId};
use crate::m3_state::M3State;

pub const MANIFESTATION_CONTRACT: &str = "ql.procedural-manifestation/v1";
pub const ATLAS_TRANSITION_CONTRACT: &str = "ql.procedural-atlas-transition/v1";
pub const MAX_MANIFESTATIONS: usize = 256;

pub type Result<T> = std::result::Result<T, String>;

pub(crate) fn nonempty(value: &str, name: &str) -> Result<()> {
    if value.is_empty() || value.len() > 2048 || value.chars().any(char::is_control) {
        return Err(format!("{name} must be bounded, nonempty data"));
    }
    Ok(())
}

/// Producer payload digest. Native owner typed-envelope digests remain opaque
/// and are never replaced by hashing transported generic JSON here.
pub fn fingerprint<T: Serialize>(value: &T) -> Result<String> {
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceBasis {
    pub source_ref: String,
    pub revision: String,
}

impl SourceBasis {
    pub fn validate(&self) -> Result<()> {
        nonempty(&self.source_ref, "source_ref")?;
        nonempty(&self.revision, "source revision")
    }
}

/// Wire-compatible with the existing O:I SubjectBinding. Contributor subjects
/// use separate bindings below, never extra keys in this native carrier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeSubject {
    pub subject_ref: String,
    pub native_owner: String,
    pub presentation_role: SubjectRole,
    pub sources: Vec<NativeReading>,
    pub readings: Vec<NativeReading>,
    pub actions: Vec<NativeAction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubjectRole {
    Being,
    Thing,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeReading {
    #[serde(rename = "ref")]
    pub reference: String,
    pub revision: String,
    pub availability: ReadingAvailability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadingAvailability {
    Available,
    Unavailable,
    Withheld,
    Stale,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeAction {
    pub action_ref: String,
    pub target_ref: String,
    pub authority_requirement: String,
}

impl NativeSubject {
    pub fn validate(&self) -> Result<()> {
        nonempty(&self.subject_ref, "subject_ref")?;
        nonempty(&self.native_owner, "native_owner")?;
        if self.sources.is_empty()
            || self.sources.len() + self.readings.len() > 256
            || self.actions.len() > 256
        {
            return Err("subject requires a bounded native source basis".into());
        }
        for reading in self.sources.iter().chain(&self.readings) {
            nonempty(&reading.reference, "reading ref")?;
            nonempty(&reading.revision, "reading revision")?;
        }
        if self
            .sources
            .iter()
            .any(|r| r.availability != ReadingAvailability::Available)
        {
            return Err("manifestation source is unavailable, withheld or stale".into());
        }
        for action in &self.actions {
            nonempty(&action.action_ref, "action ref")?;
            nonempty(&action.target_ref, "action target")?;
            nonempty(
                &action.authority_requirement,
                "action authority requirement",
            )?;
        }
        Ok(())
    }
}

/// Known QL coordinate subjects are fenced against the actual native source
/// snapshot and its resolved profile inputs, not a caller's availability word.
/// Other bounded subjects remain qualified by their own native owners.
pub fn validate_native_subject_basis(registry: &MRegistry, subject: &NativeSubject) -> Result<()> {
    subject.validate()?;
    if !subject.subject_ref.starts_with("ql:m-coordinate:") {
        return Ok(());
    }
    if subject.native_owner != "ql-mef" {
        return Err("QL coordinate subject has a different native source owner".into());
    }
    let face = if subject
        .subject_ref
        .starts_with("ql:m-coordinate:pratibimba:")
    {
        MFace::Pratibimba
    } else {
        MFace::Bimba
    };
    let binding = resolve_coordinate_expression(registry, &subject.subject_ref, face)?;
    let manifest = registry.manifest();
    let snapshot_ref = format!(
        "{}:{}",
        manifest.source_repository, manifest.source_dataset_tree
    );
    if !subject
        .sources
        .iter()
        .any(|s| s.reference == snapshot_ref && s.revision == manifest.source_snapshot_sha256)
    {
        return Err("native coordinate subject lacks its exact current source snapshot".into());
    }
    for source in &subject.sources {
        let snapshot =
            source.reference == snapshot_ref && source.revision == manifest.source_snapshot_sha256;
        let grammar = binding
            .grammar_sources
            .iter()
            .any(|s| s.source_ref == source.reference && s.content_revision == source.revision);
        let original = binding
            .property_sources
            .iter()
            .any(|s| s.file.path == source.reference && s.file.sha256 == source.revision);
        if !snapshot && !grammar && !original {
            return Err(
                "native coordinate subject source ref or revision differs from its resolved basis"
                    .into(),
            );
        }
    }
    Ok(())
}

/// These are expressive roles, never an enumeration of ontological species.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestationRole {
    Formation,
    Body,
    Field,
    Layer,
    Force,
    Constraint,
    Modulation,
    Movement,
    Sequence,
    SequenceLink,
    Scene,
    Expression,
    SceneCollection,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Contributor {
    pub subject: NativeSubject,
    pub role: String,
    /// Exact native relation ref; interpreted contributions may omit this.
    pub relation_ref: Option<String>,
    pub standing: ChoiceStanding,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChoiceStanding {
    SourceDerived,
    Authored,
    Interpretive,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequiredRelation {
    pub relation_id: MTreeId,
    pub source_kind: String,
    pub from_ref: String,
    pub to_ref: String,
    pub registry_revision: String,
}

pub fn validate_relation(registry: &MRegistry, expected: &RequiredRelation) -> Result<()> {
    if expected.registry_revision != registry.manifest().registry_revision {
        return Err("stale relation registry revision".into());
    }
    let actual = registry
        .relation(expected.relation_id)
        .ok_or("unknown native relation")?;
    if actual.source_kind != expected.source_kind
        || actual.from_ref.as_ref() != Some(&expected.from_ref)
        || actual.to_ref.as_ref() != Some(&expected.to_ref)
    {
        return Err("native relation kind or endpoints differ".into());
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OccurrenceAddress {
    pub expression_ref: String,
    pub scene_ref: Option<String>,
    pub entity_ref: Option<String>,
    pub component: ManifestationRole,
    pub constituent_ref: Option<String>,
}

impl OccurrenceAddress {
    pub fn validate(&self) -> Result<()> {
        nonempty(&self.expression_ref, "expression_ref")?;
        for reference in [&self.scene_ref, &self.entity_ref, &self.constituent_ref]
            .into_iter()
            .flatten()
        {
            nonempty(reference, "occurrence component ref")?;
        }
        match self.component {
            ManifestationRole::Expression | ManifestationRole::SceneCollection => {
                if self.scene_ref.is_some() || self.entity_ref.is_some() {
                    return Err("whole occurrence cannot be an entity occurrence".into());
                }
            }
            ManifestationRole::Scene | ManifestationRole::Field => {
                if self.scene_ref.is_none() || self.entity_ref.is_some() {
                    return Err("scene occurrence requires its scene only".into());
                }
            }
            ManifestationRole::Modulation => {
                if self.scene_ref.is_none() || self.constituent_ref.is_none() {
                    return Err(
                        "modulation occurrence requires its native scene and stable driver".into(),
                    );
                }
            }
            _ => {
                if self.scene_ref.is_none() || self.entity_ref.is_none() {
                    return Err("constituent occurrence requires native scene and entity".into());
                }
            }
        }
        if matches!(
            self.component,
            ManifestationRole::Layer | ManifestationRole::SequenceLink
        ) != self.constituent_ref.is_some()
            && self.component != ManifestationRole::Modulation
        {
            return Err("manifestation constituent ref does not match its native scale".into());
        }
        if !self.expression_ref.starts_with("expression:")
            || self.scene_ref.as_ref().is_some_and(|reference| {
                !reference.starts_with(&format!("{}:scene:", self.expression_ref))
            })
            || self.entity_ref.as_ref().is_some_and(|reference| {
                !reference.starts_with(&format!("{}:entity:", self.expression_ref))
            })
        {
            return Err("manifestation occurrence belongs to another native Expression".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestationRequest {
    pub coordinate_ref: String,
    #[serde(with = "face_wire")]
    pub face: MFace,
    pub expected_registry_revision: String,
    pub expected_profile_revision: String,
    pub occasion_ref: String,
    pub principal: NativeSubject,
    pub contributors: Vec<Contributor>,
    pub required_relations: Vec<RequiredRelation>,
    pub manifestations: Vec<ManifestationInput>,
    pub intended_act: String,
}

mod face_wire {
    use super::MFace;
    use serde::{Deserialize, Deserializer, Serializer};
    pub fn serialize<S: Serializer>(
        face: &MFace,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(face.as_str())
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> std::result::Result<MFace, D::Error> {
        match String::deserialize(deserializer)?.as_str() {
            "bimba" => Ok(MFace::Bimba),
            "pratibimba" => Ok(MFace::Pratibimba),
            _ => Err(serde::de::Error::custom(
                "unknown manifestation coordinate face",
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestationInput {
    pub output_slot: String,
    pub occurrence: OccurrenceAddress,
    pub recipe: SourceBasis,
    pub material: Value,
    pub standing: ChoiceStanding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExpressivePlan {
    pub schema: String,
    pub plan_revision: String,
    pub atlas: CoordinateExpressionBinding,
    pub occasion_ref: String,
    pub principal: NativeSubject,
    pub contributors: Vec<Contributor>,
    pub required_relations: Vec<RequiredRelation>,
    pub manifestations: Vec<ManifestationInput>,
    pub intended_act: String,
    pub native_subject_changes: Vec<Value>,
}

/// Resolve the complete current Atlas basis and validate every contribution.
/// Material is source-qualified prepared data, never executable scene script.
pub fn resolve_manifestation(
    registry: &MRegistry,
    request: ManifestationRequest,
) -> Result<ExpressivePlan> {
    validate_native_subject_basis(registry, &request.principal)?;
    nonempty(&request.occasion_ref, "occasion_ref")?;
    nonempty(&request.intended_act, "intended act")?;
    if request.expected_registry_revision != registry.manifest().registry_revision {
        return Err("stale manifestation registry revision".into());
    }
    let atlas = resolve_coordinate_expression(registry, &request.coordinate_ref, request.face)?;
    if request.expected_profile_revision != atlas.binding_content_revision {
        return Err("stale manifestation profile revision".into());
    }
    // A principal which itself is a coordinate may not be silently moved to a
    // different face or locus. Other bounded subjects can inhabit this locus.
    if request
        .principal
        .subject_ref
        .starts_with("ql:m-coordinate:")
    {
        let principal =
            resolve_coordinate_expression(registry, &request.principal.subject_ref, request.face)?;
        if principal.coordinate_id != atlas.coordinate_id {
            return Err("principal subject and exact Atlas locus differ".into());
        }
    }
    if request.manifestations.is_empty()
        || request.manifestations.len() > MAX_MANIFESTATIONS
        || request.contributors.len() > MAX_MANIFESTATIONS
        || request.required_relations.len() > 1024
    {
        return Err("manifestation cardinality exceeds the admitted plan".into());
    }
    for relation in &request.required_relations {
        validate_relation(registry, relation)?;
    }
    let mut contributor_roles = BTreeSet::new();
    for contributor in &request.contributors {
        validate_native_subject_basis(registry, &contributor.subject)?;
        nonempty(&contributor.role, "contributing role")?;
        if !contributor_roles.insert((&contributor.subject.subject_ref, &contributor.role)) {
            return Err("duplicate contributing subject role".into());
        }
        if contributor.standing == ChoiceStanding::SourceDerived {
            let reference = contributor
                .relation_ref
                .as_ref()
                .ok_or("source-derived contributor needs its native relation")?;
            if !request.required_relations.iter().any(|r| {
                registry
                    .relation(r.relation_id)
                    .is_some_and(|actual| &actual.relation_ref == reference)
                    && (r.from_ref == contributor.subject.subject_ref
                        || r.to_ref == contributor.subject.subject_ref
                        || registry
                            .resolve(&contributor.subject.subject_ref)
                            .is_some_and(|n| {
                                r.from_ref == n.source_ref || r.to_ref == n.source_ref
                            }))
            }) {
                return Err(
                    "contributor is not an endpoint of its qualified native relation".into(),
                );
            }
        }
    }
    let mut slots = BTreeSet::new();
    let mut addresses = BTreeSet::new();
    let mut native_subject_changes = Vec::new();
    let mut bound_entities = BTreeSet::new();
    for manifestation in &request.manifestations {
        nonempty(&manifestation.output_slot, "output slot")?;
        manifestation.occurrence.validate()?;
        manifestation.recipe.validate()?;
        validate_material(&manifestation.material, 0)?;
        if !slots.insert(&manifestation.output_slot)
            || !addresses.insert(fingerprint(&manifestation.occurrence)?)
        {
            return Err("duplicate manifestation output slot or occurrence".into());
        }
        if let Some(entity_ref) = &manifestation.occurrence.entity_ref {
            if bound_entities.insert(entity_ref) {
                native_subject_changes.push(json!({"change":"subject_bind", "entity_ref":entity_ref,"binding":request.principal}));
            }
        }
    }
    let mut plan = ExpressivePlan {
        schema: MANIFESTATION_CONTRACT.into(),
        plan_revision: String::new(),
        atlas,
        occasion_ref: request.occasion_ref,
        principal: request.principal,
        contributors: request.contributors,
        required_relations: request.required_relations,
        manifestations: request.manifestations,
        intended_act: request.intended_act,
        native_subject_changes,
    };
    plan.plan_revision = fingerprint(&plan)?;
    Ok(plan)
}

/// The paired PS-S/PS-C carrier uses existing flattened SubjectBindings for
/// every participant. Rich contributing-role provenance remains in the plan;
/// it is not smuggled into the old native SubjectBinding schema.
pub fn native_procedural_bindings(plan: &ExpressivePlan) -> Result<Vec<Value>> {
    let canonical_locus = match plan.atlas.face {
        crate::aw1_world::RootedFace::Bimba => &plan.atlas.rooted_world.direct.canonical_ref,
        crate::aw1_world::RootedFace::Pratibimba => {
            &plan.atlas.rooted_world.conjugate.canonical_ref
        }
    };
    plan.manifestations.iter().map(|manifestation| {
        let occurrence = &manifestation.occurrence;
        occurrence.validate()?;
        let component = match occurrence.component {
            ManifestationRole::Formation | ManifestationRole::Body => "entity",
            ManifestationRole::Force | ManifestationRole::Constraint => "force",
            ManifestationRole::Modulation => "driver",
            ManifestationRole::Field => "field",
            ManifestationRole::Layer => "layer",
            ManifestationRole::SequenceLink => "sequence_link",
            ManifestationRole::Movement | ManifestationRole::Sequence => "sequence",
            ManifestationRole::Scene => "scene",
            ManifestationRole::Expression | ManifestationRole::SceneCollection => "expression",
        };
        Ok(json!({
            "address":{"expression_ref":occurrence.expression_ref,"scene_ref":occurrence.scene_ref,
                "entity_ref":occurrence.entity_ref,"component":component,"constituent_ref":occurrence.constituent_ref,"property":null},
            "principal":plan.principal,
            "contributors":plan.contributors.iter().map(|contributor| &contributor.subject).collect::<Vec<_>>(),
            "locus":{"ref":canonical_locus,"revision":plan.atlas.binding_content_revision,"availability":"available"},
            "tags":[]
        }))
    }).collect()
}

pub(crate) fn validate_material(value: &Value, depth: usize) -> Result<()> {
    if depth > 32 {
        return Err("prepared material nesting budget exceeded".into());
    }
    match value {
        Value::Object(values) => {
            if values.len() > 256 {
                return Err("prepared material object budget exceeded".into());
            }
            for (key, value) in values {
                if [
                    "__proto__",
                    "prototype",
                    "constructor",
                    "script",
                    "javascript",
                    "eval",
                ]
                .contains(&key.as_str())
                {
                    return Err("executable or unsafe material is inadmissible".into());
                }
                validate_material(value, depth + 1)?;
            }
        }
        Value::Array(values) => {
            if values.len() > 4096 {
                return Err("prepared material array budget exceeded".into());
            }
            for value in values {
                validate_material(value, depth + 1)?;
            }
        }
        Value::String(value) if value.len() > 1024 * 1024 => {
            return Err("prepared material string budget exceeded".into());
        }
        _ => {}
    }
    Ok(())
}

/// Project actual M3 state, not a glyph label or scene-side symbolic table.
pub fn m3_material_binding(
    state: &M3State,
    expected_subject: &str,
    expected_generation: u64,
) -> Result<Value> {
    let snapshot = state.snapshot();
    if snapshot["subject_ref"].as_str() != Some(expected_subject)
        || state.generation() != expected_generation
    {
        return Err("M3 subject or generation differs from the admitted manifestation".into());
    }
    Ok(
        json!({"schema":"ql.procedural-m3-material/v1", "subject_ref":expected_subject,
        "generation":expected_generation,"event_ref":snapshot["identity"]["event_ref"],
        "registry_revision":snapshot["registry_revision"],"source_revision":snapshot["source_revision"],
        "domain_revision":snapshot["domain_revision"],"form":snapshot["form"],
        "transcription":snapshot["transcription"],"iching":snapshot["iching"],
        "tarot":snapshot["tarot"],"clock":snapshot["clock"],"aperture":snapshot["aperture"],
        "source_matrix_cells":snapshot["source_matrix_cells"],
        "standing":"native-M3-state-to-material; physical and scene reception require their consumer acknowledgements"}),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityPolicy {
    Continue,
    Hold,
    CheckpointAndRelease,
}

/// A read of an existing native occurrence; this module never stores it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlaceOccurrence {
    pub occurrence_ref: String,
    pub expression_ref: String,
    pub scene_ref: String,
    pub canonical_locus: String,
    pub registry_revision: String,
    pub profile_revision: String,
    pub occasion_ref: String,
    pub principal_subject_ref: String,
    pub retained_entity_refs: Vec<String>,
    pub runtime_instance_ref: String,
    pub native_cursor: u64,
    pub background_policy: ContinuityPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AtlasTransition {
    pub schema: String,
    pub expression_ref: String,
    pub source_occurrence_ref: String,
    pub destination_occurrence_ref: String,
    pub retained_entity_refs: Vec<String>,
    pub departure_policy: ContinuityPolicy,
    pub destination_policy: ContinuityPolicy,
    pub source_cursor: u64,
    pub destination_cursor: u64,
    pub runtime_instance_ref: String,
    pub native_changes: Vec<Value>,
}

/// Exact locus re-entry selects the caller's retained occurrence. An explicit
/// new instance is represented by a new owner occurrence before this call.
pub fn prepare_atlas_transition(
    registry: &MRegistry,
    source: &PlaceOccurrence,
    destination: &PlaceOccurrence,
    face: MFace,
    expected_destination_locus: &str,
) -> Result<AtlasTransition> {
    for place in [source, destination] {
        for (reference, name) in [
            (&place.occurrence_ref, "occurrence ref"),
            (&place.expression_ref, "expression ref"),
            (&place.scene_ref, "scene ref"),
            (&place.runtime_instance_ref, "runtime instance"),
            (&place.occasion_ref, "occasion"),
            (&place.principal_subject_ref, "principal subject"),
        ] {
            nonempty(reference, name)?;
        }
        let qualified = resolve_coordinate_expression(registry, &place.canonical_locus, face)?;
        if qualified.rooted_world.registry_revision != place.registry_revision
            || qualified.binding_content_revision != place.profile_revision
        {
            return Err(
                "Atlas occurrence source or profile is stale; explicit re-evaluation required"
                    .into(),
            );
        }
        if place.retained_entity_refs.len() > 2048
            || place
                .retained_entity_refs
                .iter()
                .collect::<BTreeSet<_>>()
                .len()
                != place.retained_entity_refs.len()
        {
            return Err("Atlas retained membership is duplicate or excessive".into());
        }
        if !place.expression_ref.starts_with("expression:")
            || !place
                .scene_ref
                .starts_with(&format!("{}:scene:", place.expression_ref))
            || place
                .retained_entity_refs
                .iter()
                .any(|entity| !entity.starts_with(&format!("{}:entity:", place.expression_ref)))
        {
            return Err(
                "Atlas scene or constituent belongs to a different native Expression".into(),
            );
        }
    }
    let expected = resolve_coordinate_expression(registry, expected_destination_locus, face)?;
    let destination_basis =
        resolve_coordinate_expression(registry, &destination.canonical_locus, face)?;
    if expected.coordinate_id != destination_basis.coordinate_id
        || expected.face != destination_basis.face
    {
        return Err("Atlas destination is a different locus or prime".into());
    }
    if source.expression_ref != destination.expression_ref
        || source.runtime_instance_ref != destination.runtime_instance_ref
    {
        return Err(
            "Atlas navigation cannot replace the continuing Expression or native runtime".into(),
        );
    }
    if source.occasion_ref != destination.occasion_ref {
        return Err("Atlas navigation cannot silently change the native occasion".into());
    }
    let source_entities: BTreeSet<_> = source.retained_entity_refs.iter().collect();
    let retained_entity_refs = destination
        .retained_entity_refs
        .iter()
        .filter(|r| source_entities.contains(r))
        .cloned()
        .collect();
    Ok(AtlasTransition {
        schema: ATLAS_TRANSITION_CONTRACT.into(),
        expression_ref: source.expression_ref.clone(),
        source_occurrence_ref: source.occurrence_ref.clone(),
        destination_occurrence_ref: destination.occurrence_ref.clone(),
        retained_entity_refs,
        departure_policy: source.background_policy,
        destination_policy: destination.background_policy,
        source_cursor: source.native_cursor,
        destination_cursor: destination.native_cursor,
        runtime_instance_ref: source.runtime_instance_ref.clone(),
        native_changes: vec![
            json!({"change":"focus","scene_ref":destination.scene_ref,"entity_ref":null}),
        ],
    })
}

/// Capability coverage from the complete native registry; source-only records
/// stay eligible rather than disappearing because no numerical binding exists.
pub fn manifestation_inventory(registry: &MRegistry) -> BTreeMap<String, Vec<String>> {
    registry
        .manifest()
        .nodes
        .iter()
        .filter(|n| n.root_position.is_some())
        .map(|node| {
            let operations = registry
                .manifest()
                .bindings
                .iter()
                .filter(|b| b.coordinate_id == Some(node.id))
                .map(|b| b.identity.clone())
                .collect();
            (node.source_ref.clone(), operations)
        })
        .collect()
}
