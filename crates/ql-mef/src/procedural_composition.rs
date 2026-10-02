//! Bounded native procedure compilation over the existing Expression/C′ owners.
//!
//! The retained record is carried by Scene.procedural at its native owner.
//! This compiler has no store, clock, model invocation or live graph query.
//! A prepared Edit is not evidence that its body/audio consumers applied it.
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::m_tree::MRegistry;
use crate::procedural_manifestation::{
    NativeSubject, RequiredRelation, SourceBasis, fingerprint, nonempty, validate_material,
    validate_native_subject_basis, validate_relation,
};
use crate::vak_profile::ThreadForm;
use crate::vak_workflow_types::AuthoredCPrime;

pub const PROCEDURE_CONTRACT: &str = "ql.procedural-composition/v1";
pub const SEED_ALGORITHM: &str = "sha256-counter-v1";
pub type Result<T> = std::result::Result<T, String>;
const MAX_OPERATIONS: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TagOrigin {
    NativeSource,
    Authored,
    Generated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualifiedTag {
    pub value: String,
    pub origin: TagOrigin,
    pub scope_ref: String,
    pub basis: SourceBasis,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetReading {
    pub occurrence_ref: String,
    pub address: OwnedAddress,
    pub subject: NativeSubject,
    pub revision: u64,
    pub tags: Vec<QualifiedTag>,
    pub properties: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "selector", rename_all = "snake_case", deny_unknown_fields)]
pub enum Selector {
    Occurrences {
        refs: Vec<String>,
    },
    Subject {
        subject_ref: String,
    },
    Tag {
        value: String,
        origin: TagOrigin,
        scope_ref: String,
    },
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MembershipMode {
    Frozen,
    Sustained,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MembershipChangePolicy {
    AdmitAndRecord,
    RetainExisting,
    RejectChange,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResolvedMembership {
    pub selector: Selector,
    pub mode: MembershipMode,
    pub containing_expression_ref: String,
    pub targets: BTreeMap<String, u64>,
    /// Concrete native addresses frozen with the exact owner readings. A
    /// declared contribution write cannot widen this selector membership.
    pub addresses: BTreeMap<String, OwnedAddress>,
    pub joined: Vec<String>,
    pub left: Vec<String>,
}

pub fn resolve_membership(
    selector: &Selector,
    expression_ref: &str,
    readings: &[TargetReading],
    mode: MembershipMode,
    previous: Option<&ResolvedMembership>,
    policy: MembershipChangePolicy,
) -> Result<ResolvedMembership> {
    nonempty(expression_ref, "selector containing Expression")?;
    if readings.len() > 2048 {
        return Err("selector input exceeds native membership budget".into());
    }
    if let Some(previous) = previous {
        if previous.containing_expression_ref != expression_ref || previous.mode != mode {
            return Err("selector membership scope or mode cannot be silently changed".into());
        }
        if &previous.selector != selector {
            return Err(
                "retained selector definition changed; explicit source re-evaluation required"
                    .into(),
            );
        }
        if mode == MembershipMode::Frozen {
            return Ok(previous.clone());
        }
    }
    let mut seen = BTreeSet::new();
    let mut targets = BTreeMap::new();
    let mut addresses = BTreeMap::new();
    for reading in readings {
        nonempty(&reading.occurrence_ref, "target occurrence")?;
        reading.subject.validate()?;
        if !seen.insert(&reading.occurrence_ref) {
            return Err("duplicate native occurrence reading".into());
        }
        reading.address.validate()?;
        if reading.address.expression_ref != expression_ref {
            continue;
        }
        let matched = match selector {
            Selector::All => true,
            Selector::Occurrences { refs } => refs.contains(&reading.occurrence_ref),
            Selector::Subject { subject_ref } => subject_ref == &reading.subject.subject_ref,
            Selector::Tag {
                value,
                origin,
                scope_ref,
            } => reading.tags.iter().any(|tag| {
                &tag.value == value && &tag.origin == origin && &tag.scope_ref == scope_ref
            }),
        };
        if matched {
            targets.insert(reading.occurrence_ref.clone(), reading.revision);
            addresses.insert(reading.occurrence_ref.clone(), reading.address.clone());
        }
    }
    if let Selector::Occurrences { refs } = selector {
        if refs.len() > 2048 || refs.iter().collect::<BTreeSet<_>>().len() != refs.len() {
            return Err("occurrence selector is duplicate or excessive".into());
        }
        if refs
            .iter()
            .any(|reference| !targets.contains_key(reference))
        {
            return Err("unknown or out-of-scope explicit occurrence".into());
        }
    }
    let old = previous.map(|p| &p.targets);
    let joined = targets
        .keys()
        .filter(|r| old.is_none_or(|old| !old.contains_key(*r)))
        .cloned()
        .collect::<Vec<_>>();
    let left = old
        .into_iter()
        .flat_map(|old| old.keys())
        .filter(|r| !targets.contains_key(*r))
        .cloned()
        .collect::<Vec<_>>();
    if previous.is_some() && (!joined.is_empty() || !left.is_empty()) {
        match policy {
            MembershipChangePolicy::RejectChange => {
                return Err("sustained selector membership changed".into());
            }
            MembershipChangePolicy::RetainExisting => return Ok(previous.unwrap().clone()),
            MembershipChangePolicy::AdmitAndRecord => {}
        }
    }
    Ok(ResolvedMembership {
        selector: selector.clone(),
        mode,
        containing_expression_ref: expression_ref.into(),
        targets,
        addresses,
        joined,
        left,
    })
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "condition", rename_all = "snake_case", deny_unknown_fields)]
pub enum Condition {
    Equals {
        property: String,
        value: Value,
    },
    ScalarRange {
        property: String,
        minimum: f64,
        maximum: f64,
    },
    NativeRelation {
        relation: RequiredRelation,
    },
}

impl Condition {
    pub fn accepts(&self, registry: &MRegistry, reading: &TargetReading) -> Result<bool> {
        match self {
            Self::Equals { property, value } => Ok(reading.properties.get(property) == Some(value)),
            Self::ScalarRange {
                property,
                minimum,
                maximum,
            } => {
                if !minimum.is_finite() || !maximum.is_finite() || minimum > maximum {
                    return Err("invalid typed scalar condition".into());
                }
                Ok(reading
                    .properties
                    .get(property)
                    .and_then(Value::as_f64)
                    .is_some_and(|v| v.is_finite() && v >= *minimum && v <= *maximum))
            }
            Self::NativeRelation { relation } => {
                validate_relation(registry, relation)?;
                let subject = &reading.subject.subject_ref;
                Ok(subject == &relation.from_ref
                    || subject == &relation.to_ref
                    || registry.resolve(subject).is_some_and(|node| {
                        node.source_ref == relation.from_ref || node.source_ref == relation.to_ref
                    }))
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerKind {
    StateChanged,
    MusicalGesture,
    PhysicalGesture,
    SourceChanged,
    SceneEntered,
    SceneLeft,
    SequenceTransition,
    Explicit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerMode {
    Edge,
    Level,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Trigger {
    pub kind: TriggerKind,
    pub mode: TriggerMode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionBudget {
    pub max_evaluations: usize,
    pub max_operations: usize,
    pub max_expansion_depth: usize,
    pub max_active_instances: usize,
    pub max_queue: usize,
}

impl ExecutionBudget {
    pub fn validate(&self) -> Result<()> {
        if self.max_evaluations == 0
            || self.max_evaluations > 4096
            || self.max_operations == 0
            || self.max_operations > MAX_OPERATIONS
            || self.max_expansion_depth == 0
            || self.max_expansion_depth > 64
            || self.max_active_instances == 0
            || self.max_active_instances > 2048
            || self.max_queue == 0
            || self.max_queue > 4096
        {
            return Err("procedure budget is outside native bounded execution".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TimingBinding {
    pub owner_ref: String,
    pub domain: String,
    pub epoch_ref: String,
    /// Exact cursor in the owner domain; never a renderer-derived audio time.
    pub requested_cursor: u64,
    pub time_mapping_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Procedure {
    pub schema: String,
    pub procedure_ref: String,
    pub revision: String,
    pub recipe: SourceBasis,
    pub profile: SourceBasis,
    pub registry_revision: String,
    pub principal_subject_ref: String,
    pub locus_ref: String,
    pub occurrence_ref: String,
    pub composition: AuthoredCPrime,
    pub trigger: Trigger,
    pub selector: Selector,
    pub conditions: Vec<Condition>,
    pub recipe_parameters: BTreeMap<String, Value>,
    pub membership_mode: MembershipMode,
    pub membership_change_policy: MembershipChangePolicy,
    pub timing: TimingBinding,
    pub budgets: ExecutionBudget,
    pub seed: String,
    pub seed_algorithm: String,
    pub removal_policy: RemovalPolicy,
    pub failure_policy: FailurePolicy,
    pub continuation_policy: ProcedureContinuation,
    /// Actual resolved native operations admitted by the owning SDK.
    pub admitted_changes: BTreeSet<String>,
}

impl Procedure {
    pub fn validate(&self, registry: &MRegistry) -> Result<()> {
        if self.schema != PROCEDURE_CONTRACT
            || self.registry_revision != registry.manifest().registry_revision
        {
            return Err("unsupported procedure or stale native source revision".into());
        }
        for (value, name) in [
            (&self.procedure_ref, "procedure ref"),
            (&self.revision, "procedure revision"),
            (&self.principal_subject_ref, "principal subject"),
            (&self.locus_ref, "locus ref"),
            (&self.occurrence_ref, "occurrence ref"),
            (&self.seed, "seed"),
            (&self.timing.owner_ref, "timing owner"),
            (&self.timing.domain, "timing domain"),
            (&self.timing.epoch_ref, "timing epoch"),
        ] {
            nonempty(value, name)?;
        }
        self.recipe.validate()?;
        self.profile.validate()?;
        self.budgets.validate()?;
        if self.conditions.len() > 64 || self.recipe_parameters.len() > 256 {
            return Err("recipe conditions or parameter bound exceeded".into());
        }
        for condition in &self.conditions {
            match condition {
                Condition::Equals { property, value } => {
                    nonempty(property, "condition property")?;
                    validate_material(value, 0)?;
                }
                Condition::ScalarRange {
                    property,
                    minimum,
                    maximum,
                } => {
                    nonempty(property, "condition property")?;
                    if !minimum.is_finite() || !maximum.is_finite() || minimum > maximum {
                        return Err("invalid typed scalar condition".into());
                    }
                }
                Condition::NativeRelation { relation } => validate_relation(registry, relation)?,
            }
        }
        validate_material(
            &serde_json::to_value(&self.recipe_parameters).map_err(|e| e.to_string())?,
            0,
        )?;
        self.composition.validate().map_err(|e| e.to_string())?;
        if self.seed_algorithm != SEED_ALGORITHM {
            return Err("unsupported replay seed algorithm".into());
        }
        if self.composition.whole != self.occurrence_ref {
            return Err("C-prime whole differs from procedure occurrence".into());
        }
        if self.admitted_changes.is_empty()
            || self
                .admitted_changes
                .iter()
                .any(|name| !NATIVE_CHANGES.contains(&name.as_str()))
        {
            return Err("procedure names an unavailable native change capability".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailurePolicy {
    StopAffectedAndCheckpoint,
    HoldLastAdmittedAndReport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProcedureContinuation {
    Continue,
    Hold,
    CheckpointAndRelease,
}

/// Evaluate expensive source-qualified conditions before preparing a live
/// operation, then freeze the actual owner addresses/revisions it can affect.
pub fn resolve_procedure_membership(
    registry: &MRegistry,
    procedure: &Procedure,
    expression_ref: &str,
    readings: &[TargetReading],
    previous: Option<&ResolvedMembership>,
) -> Result<ResolvedMembership> {
    procedure.validate(registry)?;
    let mut accepted = Vec::new();
    for reading in readings {
        let mut matches = true;
        for condition in &procedure.conditions {
            if !condition.accepts(registry, reading)? {
                matches = false;
                break;
            }
        }
        if matches {
            accepted.push(reading.clone());
        }
    }
    resolve_membership(
        &procedure.selector,
        expression_ref,
        &accepted,
        procedure.membership_mode,
        previous,
        procedure.membership_change_policy.clone(),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemovalPolicy {
    RetireUneditedDetachEdited,
    ConflictOnEdited,
}

const NATIVE_CHANGES: &[&str] = &[
    "scene_create",
    "scene_remove",
    "entity_add",
    "entity_remove",
    "subject_bind",
    "scene_compose",
    "scene_material_set",
    "parameter_set",
    "focus",
    "scene_reorder",
];

/// No private imperative script: every recipe output is an existing native
/// change. The final owner repeats schema, units/range, authority and CAS.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "change", rename_all = "snake_case", deny_unknown_fields)]
pub enum NativeChange {
    SceneCreate {
        scene_ref: String,
        title: String,
    },
    SceneRemove {
        scene_ref: String,
    },
    EntityAdd {
        scene_ref: String,
        entity_ref: String,
        title: String,
    },
    EntityRemove {
        entity_ref: String,
    },
    SubjectBind {
        entity_ref: String,
        binding: NativeSubject,
    },
    SceneCompose {
        scene_ref: String,
        entity_refs: Vec<String>,
    },
    SceneMaterialSet {
        scene_ref: String,
        presentation: Value,
    },
    ParameterSet {
        entity_ref: String,
        parameter: String,
        value: Value,
    },
    Focus {
        scene_ref: String,
        entity_ref: Option<String>,
    },
    SceneReorder {
        scene_refs: Vec<String>,
    },
}

impl NativeChange {
    pub fn name(&self) -> &'static str {
        match self {
            Self::SceneCreate { .. } => "scene_create",
            Self::SceneRemove { .. } => "scene_remove",
            Self::EntityAdd { .. } => "entity_add",
            Self::EntityRemove { .. } => "entity_remove",
            Self::SubjectBind { .. } => "subject_bind",
            Self::SceneCompose { .. } => "scene_compose",
            Self::SceneMaterialSet { .. } => "scene_material_set",
            Self::ParameterSet { .. } => "parameter_set",
            Self::Focus { .. } => "focus",
            Self::SceneReorder { .. } => "scene_reorder",
        }
    }
    pub fn validate(&self) -> Result<()> {
        let wire = serde_json::to_value(self).map_err(|e| e.to_string())?;
        validate_material(&wire, 0)?;
        for key in ["scene_ref", "entity_ref", "parameter", "title"] {
            if let Some(Value::String(value)) = wire.get(key) {
                nonempty(value, key)?;
            }
        }
        match self {
            Self::SubjectBind { binding, .. } => binding.validate(),
            Self::SceneMaterialSet {
                scene_ref,
                presentation,
            } => {
                if presentation["schema"] != "oi.journey-scene/v1"
                    || presentation["scene"]["id"] != *scene_ref
                {
                    return Err(
                        "material change is not the addressed native scene presentation".into(),
                    );
                }
                if let Some(retained) = presentation["scene"].get("procedural") {
                    if retained["operations"]
                        .as_array()
                        .is_none_or(|rows| !rows.is_empty())
                    {
                        return Err(
                            "native operation intent must omit retained journal rows".into()
                        );
                    }
                }
                Ok(())
            }
            Self::SceneCompose { entity_refs, .. }
            | Self::SceneReorder {
                scene_refs: entity_refs,
            } => {
                if entity_refs.len() > 2048
                    || entity_refs.iter().collect::<BTreeSet<_>>().len() != entity_refs.len()
                {
                    return Err("native ordered membership is excessive or duplicate".into());
                }
                for reference in entity_refs {
                    nonempty(reference, "native membership ref")?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OwnedAddress {
    pub expression_ref: String,
    pub scene_ref: Option<String>,
    pub entity_ref: Option<String>,
    pub component: String,
    pub constituent_ref: Option<String>,
    pub property: Option<String>,
}

impl OwnedAddress {
    pub fn validate(&self) -> Result<()> {
        nonempty(&self.expression_ref, "native addressed Expression")?;
        if !self.expression_ref.starts_with("expression:")
            || ![
                "expression",
                "scene",
                "field",
                "entity",
                "force",
                "layer",
                "sequence",
                "sequence_link",
                "property",
                "driver",
            ]
            .contains(&self.component.as_str())
        {
            return Err("unavailable native address component".into());
        }
        if let Some(scene) = &self.scene_ref {
            if !scene.starts_with(&format!("{}:scene:", self.expression_ref)) {
                return Err("native scene address is outside its Expression".into());
            }
        }
        if let Some(entity) = &self.entity_ref {
            if !entity.starts_with(&format!("{}:entity:", self.expression_ref)) {
                return Err("native entity address is outside its Expression".into());
            }
        }
        if (self.component == "expression"
            && (self.scene_ref.is_some() || self.entity_ref.is_some()))
            || (self.component != "expression" && self.scene_ref.is_none())
            || (["entity", "force", "layer", "sequence", "sequence_link"]
                .contains(&self.component.as_str())
                && self.entity_ref.is_none())
            || (["scene", "field"].contains(&self.component.as_str()) && self.entity_ref.is_some())
            || (["layer", "sequence_link", "driver"].contains(&self.component.as_str())
                != self.constituent_ref.is_some())
        {
            return Err("native address lacks its exact containing occurrence".into());
        }
        for value in [&self.constituent_ref, &self.property]
            .into_iter()
            .flatten()
        {
            nonempty(value, "native constituent/property")?;
        }
        if self.property.as_ref().is_some_and(|property| {
            property.split('.').any(|part| {
                part.is_empty()
                    || ["__proto__", "prototype", "constructor"].contains(&part)
                    || !part.as_bytes()[0].is_ascii_alphabetic() && part.as_bytes()[0] != b'_'
                    || !part
                        .bytes()
                        .all(|ch| ch.is_ascii_alphanumeric() || ch == b'_')
            })
        }) {
            return Err("unsafe native property address".into());
        }
        Ok(())
    }

    /// Containment is an owner address relation, never string-prefix matching
    /// of arbitrary entity/constituent IDs. Distinct exact properties are disjoint.
    pub fn covers(&self, other: &Self) -> bool {
        if self.expression_ref != other.expression_ref {
            return false;
        }
        if self.component == "expression" && self.property.is_none() {
            return true;
        }
        if self.scene_ref != other.scene_ref {
            return false;
        }
        if self.component == "scene" && self.entity_ref.is_none() && self.property.is_none() {
            return true;
        }
        if self.entity_ref != other.entity_ref {
            return false;
        }
        if self.component == "entity" && self.entity_ref.is_some() && self.property.is_none() {
            return true;
        }
        self.component == other.component
            && self.constituent_ref == other.constituent_ref
            && (self.property.is_none()
                || self.property == other.property
                || self.property.as_ref().is_some_and(|property| {
                    other
                        .property
                        .as_ref()
                        .is_some_and(|target| target.starts_with(&format!("{property}.")))
                }))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GeneratedContribution {
    pub contribution_ref: String,
    pub procedure_ref: String,
    pub output_slot: String,
    pub subjects: Vec<String>,
    pub occurrence_ref: String,
    pub recipe: SourceBasis,
    pub procedure_revision: String,
    pub owned_addresses: Vec<OwnedAddress>,
    pub native_changes: Vec<NativeChange>,
    pub generated_basis: Value,
}

/// Identity omits rule revision and output payload; a revised rule updates the
/// same contribution. A deliberate clone changes the explicit instance ref.
pub fn contribution_identity(
    procedure: &str,
    slot: &str,
    subjects: &[String],
    occurrence: &str,
) -> Result<String> {
    nonempty(procedure, "procedure ref")?;
    nonempty(slot, "output slot")?;
    nonempty(occurrence, "instance ref")?;
    if subjects.is_empty() || subjects.len() > 256 {
        return Err("contribution requires bounded subjects".into());
    }
    let mut subjects = subjects.to_vec();
    for subject in &subjects {
        nonempty(subject, "bound subject")?;
    }
    subjects.sort();
    subjects.dedup();
    Ok(format!(
        "contribution:{}",
        fingerprint(&(PROCEDURE_CONTRACT, procedure, slot, subjects, occurrence))?
    ))
}

/// Deterministic, versioned selection independent of target arrival order.
pub fn seeded_index(seed: &str, occurrence: &str, draw: u64, count: usize) -> Result<usize> {
    nonempty(seed, "seed")?;
    nonempty(occurrence, "seed occurrence")?;
    if count == 0 || count > 2048 {
        return Err("seed selection count outside native budget".into());
    }
    let hash = fingerprint(&(SEED_ALGORITHM, seed, occurrence, draw))?;
    let number = u64::from_str_radix(&hash[..16], 16).map_err(|e| e.to_string())?;
    Ok((number % count as u64) as usize)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthoredOverlay {
    pub contribution_ref: String,
    /// Object segments use RFC6901 escaping. Array segments MUST be `@<id>`
    /// (also escaped); positional indices cannot retarget a human intervention.
    pub pointer: String,
    pub value: Value,
    pub actor_ref: String,
    pub persistent: bool,
}

fn overlay_target<'a>(material: &'a mut Value, pointer: &str) -> Result<&'a mut Value> {
    if pointer.is_empty() {
        return Ok(material);
    }
    if !pointer.starts_with('/') || pointer.len() > 4096 {
        return Err("invalid bounded authored override path".into());
    }
    let mut target = material;
    for raw in pointer[1..].split('/') {
        let mut chars = raw.chars();
        let mut part = String::new();
        while let Some(ch) = chars.next() {
            if ch == '~' {
                part.push(match chars.next() {
                    Some('0') => '~',
                    Some('1') => '/',
                    _ => return Err("invalid authored override path escaping".into()),
                });
            } else {
                part.push(ch);
            }
        }
        target = match target {
            Value::Object(object) => object
                .get_mut(&part)
                .ok_or("authored override target disappeared; detachment required")?,
            Value::Array(values) => {
                let id = part
                    .strip_prefix('@')
                    .filter(|id| !id.is_empty())
                    .ok_or("positional authored override refused; use stable constituent ID")?;
                if values
                    .iter()
                    .filter(|value| value["id"].as_str() == Some(id))
                    .count()
                    != 1
                {
                    return Err("authored override constituent disappeared or became ambiguous; detachment required".into());
                }
                values
                    .iter_mut()
                    .find(|value| value["id"].as_str() == Some(id))
                    .unwrap()
            }
            _ => return Err("authored override path traverses a scalar".into()),
        };
    }
    Ok(target)
}

/// Three-way reconciliation retains human changes and unknown authored keys.
/// Arrays of constituents are paired by stable `id`, never positional index.
pub fn reconcile_material(previous: &Value, current: &Value, generated: &Value) -> Result<Value> {
    merge(Some(previous), Some(current), Some(generated), "")?
        .ok_or("material unexpectedly retired".into())
}

fn merge(
    old: Option<&Value>,
    current: Option<&Value>,
    next: Option<&Value>,
    path: &str,
) -> Result<Option<Value>> {
    if current == old {
        return Ok(next.cloned());
    }
    if next == old || current == next {
        return Ok(current.cloned());
    }
    match (old, current, next) {
        (Some(Value::Object(old)), Some(Value::Object(current)), Some(Value::Object(next))) => {
            let keys = old
                .keys()
                .chain(current.keys())
                .chain(next.keys())
                .collect::<BTreeSet<_>>();
            let mut result = serde_json::Map::new();
            for key in keys {
                if let Some(value) = merge(
                    old.get(key),
                    current.get(key),
                    next.get(key),
                    &format!("{path}/{key}"),
                )? {
                    result.insert(key.clone(), value);
                }
            }
            Ok(Some(Value::Object(result)))
        }
        (Some(Value::Array(old)), Some(Value::Array(current)), Some(Value::Array(next)))
            if old
                .iter()
                .chain(current)
                .chain(next)
                .all(|value| value["id"].as_str().is_some()) =>
        {
            let old_map = keyed_material(old, path)?;
            let current_map = keyed_material(current, path)?;
            let next_map = keyed_material(next, path)?;
            let mut ids = next
                .iter()
                .map(|v| v["id"].as_str().unwrap().to_owned())
                .collect::<Vec<_>>();
            ids.extend(current.iter().filter_map(|v| {
                let id = v["id"].as_str().unwrap();
                (!next_map.contains_key(id)).then(|| id.to_owned())
            }));
            let mut result = Vec::new();
            for id in ids {
                if let Some(value) = merge(
                    old_map.get(&id).copied(),
                    current_map.get(&id).copied(),
                    next_map.get(&id).copied(),
                    &format!("{path}/{id}"),
                )? {
                    result.push(value);
                }
            }
            Ok(Some(Value::Array(result)))
        }
        // A changed authored value has precedence over its revised generated
        // basis. A generated deletion therefore preserves the edited material.
        (Some(_), Some(current), _) => Ok(Some(current.clone())),
        (None, Some(current), None) => Ok(Some(current.clone())),
        (Some(_), None, _) => Ok(None), // deliberate human deletion
        (None, Some(_), Some(_)) => Err(format!("authored/generated creation conflict at {path}")),
        (None, None, next) => Ok(next.cloned()),
    }
}

fn keyed_material<'a>(values: &'a [Value], path: &str) -> Result<BTreeMap<String, &'a Value>> {
    let mut map = BTreeMap::new();
    for value in values {
        let id = value["id"]
            .as_str()
            .ok_or("material constituent has no stable identity")?
            .to_owned();
        if map.insert(id, value).is_some() {
            return Err(format!("duplicate material identity at {path}"));
        }
    }
    Ok(map)
}

// Native authoring occurrence references are structural fields, never arbitrary
// source/provenance strings. Keep the adapter paired with nativeParameters,
// nativeBridge, semanticTypes, blueprintGeometry and researchMaterial.
fn encoded_occurrence(value: &str) -> String {
    let mut out = String::new();
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || b"-_.!~*'()".contains(&byte) {
            out.push(char::from(byte));
        } else {
            out.push('%');
            out.push(char::from(HEX[usize::from(byte >> 4)]));
            out.push(char::from(HEX[usize::from(byte & 15)]));
        }
    }
    out
}

fn decoded_occurrence(value: &str) -> Result<String> {
    let mut bytes = Vec::new();
    let mut source = value.as_bytes().iter().copied();
    while let Some(byte) = source.next() {
        if byte == b'%' {
            let high = source.next().ok_or("invalid encoded occurrence")?;
            let low = source.next().ok_or("invalid encoded occurrence")?;
            let high = char::from(high)
                .to_digit(16)
                .ok_or("invalid encoded occurrence")?;
            let low = char::from(low)
                .to_digit(16)
                .ok_or("invalid encoded occurrence")?;
            bytes.push((high * 16 + low) as u8);
        } else {
            bytes.push(byte);
        }
    }
    String::from_utf8(bytes).map_err(|_| "invalid occurrence UTF-8".into())
}

fn remap_reference(value: &mut Value, ids: &BTreeMap<String, String>) -> Result<()> {
    let old = value
        .as_str()
        .ok_or("native occurrence reference must be a string")?;
    let new = ids
        .get(old)
        .ok_or("native template reference addresses a missing constituent")?;
    *value = Value::String(new.clone());
    Ok(())
}

fn remap_optional_reference(
    value: &mut Value,
    key: &str,
    ids: &BTreeMap<String, String>,
) -> Result<()> {
    if let Some(reference) = value.get_mut(key).filter(|r| !r.is_null()) {
        remap_reference(reference, ids)?;
    }
    Ok(())
}

fn remap_semantic_carriers(value: &mut Value, ids: &BTreeMap<String, String>) -> Result<()> {
    if let Some(bindings) = value.get_mut("bindings").and_then(Value::as_array_mut) {
        for binding in bindings {
            if let Some(carriers) = binding.get_mut("carriers").and_then(Value::as_array_mut) {
                for carrier in carriers {
                    match carrier["kind"].as_str() {
                        Some("entity") => remap_reference(&mut carrier["id"], ids)?,
                        Some("forceEmitter") => {
                            let id = carrier["id"]
                                .as_str()
                                .ok_or("native force carrier has no id")?;
                            if let Some(new) = ids.get(id) {
                                carrier["id"] = Value::String(new.clone());
                            } else if let Some(entity) = id.strip_prefix("entity:") {
                                let new = ids.get(entity).ok_or(
                                    "native force carrier addresses a missing constituent",
                                )?;
                                carrier["id"] = Value::String(format!("entity:{new}"));
                            }
                            // Independent relational/pointer emitters retain
                            // their native identity, not an entity projection.
                        }
                        _ => return Err("unknown native semantic carrier".into()),
                    }
                }
            }
        }
    }
    Ok(())
}

fn remap_occurrence_material(
    scene: &mut serde_json::Map<String, Value>,
    old_scene_ref: &str,
    scene_ref: &str,
    ids: &BTreeMap<String, String>,
) -> Result<()> {
    if let Some(lanes) = scene.get_mut("automation").and_then(Value::as_array_mut) {
        for lane in lanes {
            remap_optional_reference(lane, "entityId", ids)?;
            let target = lane["target"]
                .as_str()
                .ok_or("native automation has no target")?;
            let mut parts = target.splitn(3, ':');
            let kind = parts.next().unwrap_or_default();
            if kind == "entity" || kind == "link" {
                let old = decoded_occurrence(
                    parts
                        .next()
                        .ok_or("native automation occurrence is missing")?,
                )?;
                let new = ids
                    .get(&old)
                    .ok_or("native automation addresses a missing constituent")?;
                let suffix = parts
                    .next()
                    .ok_or("native automation property is missing")?;
                lane["target"] =
                    Value::String(format!("{kind}:{}:{suffix}", encoded_occurrence(new)));
            }
            if let Some(clock) = lane.get_mut("clockId") {
                if let Some(new) = clock
                    .as_str()
                    .and_then(|s| s.strip_prefix("pin:"))
                    .and_then(|id| ids.get(id))
                {
                    *clock = Value::String(format!("pin:{new}"));
                }
            }
        }
    }
    for collection in ["propertyTracks", "toolbelt"] {
        if let Some(values) = scene.get_mut(collection).and_then(Value::as_array_mut) {
            for value in values {
                remap_optional_reference(value, "entityId", ids)?;
                if value["sceneId"].as_str() == Some(old_scene_ref) {
                    value["sceneId"] = Value::String(scene_ref.into());
                }
            }
        }
    }
    if let Some(field) = scene.get_mut("semanticField") {
        remap_semantic_carriers(field, ids)?;
    }
    if let Some(members) = scene
        .get_mut("composition")
        .and_then(|v| v.get_mut("blueprint"))
        .and_then(|v| v.get_mut("members"))
        .and_then(Value::as_array_mut)
    {
        for member in members {
            remap_reference(&mut member["entity_ref"], ids)?;
        }
    }
    if let Some(native) = scene.get_mut("native") {
        for projection in ["config", "projection"] {
            if let Some(config) = native.get_mut(projection) {
                if let Some(entities) = config.get_mut("entities").and_then(Value::as_array_mut) {
                    for entity in entities {
                        remap_reference(&mut entity["id"], ids)?;
                    }
                }
                if let Some(field) = config.get_mut("semanticField") {
                    remap_semantic_carriers(field, ids)?;
                }
                // Native lane paths address array positions; ordering is
                // unchanged. Original imported source is retained verbatim.
            }
        }
    }
    if let Some(research) = scene.get_mut("research") {
        for collection in ["cards", "timeline"] {
            if let Some(values) = research.get_mut(collection).and_then(Value::as_object_mut) {
                let old = std::mem::take(values);
                for (id, value) in old {
                    let new = ids
                        .get(&id)
                        .ok_or("native research material addresses a missing constituent")?;
                    values.insert(new.clone(), value);
                }
            }
        }
        for (collection, references) in [("frames", "memberRefs"), ("namedViews", "selectedRefs")] {
            if let Some(values) = research.get_mut(collection).and_then(Value::as_object_mut) {
                for value in values.values_mut() {
                    if let Some(refs) = value.get_mut(references).and_then(Value::as_array_mut) {
                        for reference in refs {
                            remap_reference(reference, ids)?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

/// Instantiate a complete, admitted native Scene template. All formation,
/// force, sequence, layer, field and composition material survives from that
/// source. Only explicit new occurrence IDs are remapped deterministically.
/// The same procedure/slot/instance regenerates the same native references.
pub fn instantiate_scene(
    procedure: &Procedure,
    expression_ref: &str,
    output_slot: &str,
    scene_ref: &str,
    subjects: &[NativeSubject],
    source_presentation: &Value,
) -> Result<GeneratedContribution> {
    nonempty(expression_ref, "Expression ref")?;
    nonempty(scene_ref, "scene occurrence ref")?;
    if subjects.is_empty() || subjects.len() > 256 {
        return Err("scene recipe requires native participants".into());
    }
    for subject in subjects {
        subject.validate()?;
    }
    if subjects[0].subject_ref != procedure.principal_subject_ref {
        return Err("scene recipe principal differs from procedure".into());
    }
    let subject_refs = subjects
        .iter()
        .map(|s| s.subject_ref.clone())
        .collect::<Vec<_>>();
    let identity = contribution_identity(
        &procedure.procedure_ref,
        output_slot,
        &subject_refs,
        scene_ref,
    )?;
    if source_presentation["schema"] != "oi.journey-scene/v1" {
        return Err("recipe input is not a native Scene presentation".into());
    }
    validate_material(source_presentation, 0)?;
    let mut presentation = source_presentation.clone();
    presentation
        .as_object_mut()
        .ok_or("native presentation is not an object")?
        .remove("saved");
    let scene = presentation["scene"]
        .as_object_mut()
        .ok_or("scene recipe needs complete native material")?;
    if scene.get("procedural").is_some_and(|v| !v.is_null()) {
        return Err(
            "retained procedural runtime requires an explicit native clone/rebind operation".into(),
        );
    }
    let old_scene_ref = scene
        .get("id")
        .and_then(Value::as_str)
        .ok_or("native template has no scene identity")?
        .to_owned();
    let title = scene
        .get("name")
        .and_then(Value::as_str)
        .ok_or("scene template has no name")?
        .to_owned();
    let local_expression = expression_ref
        .strip_prefix("expression:")
        .ok_or("native Expression ref expected")?;
    nonempty(local_expression, "Expression local ref")?;
    if !scene_ref.starts_with(&format!("{expression_ref}:scene:")) {
        return Err("generated scene belongs to a different native Expression".into());
    }
    scene.insert("id".into(), Value::String(scene_ref.into()));
    let entities = scene
        .get_mut("entities")
        .and_then(Value::as_array_mut)
        .ok_or("scene template has no constituents")?;
    if entities.len() > 256 {
        return Err("scene template exceeds native constituent budget".into());
    }
    let mut native_changes = vec![NativeChange::SceneCreate {
        scene_ref: scene_ref.into(),
        title,
    }];
    let mut native_refs = Vec::new();
    let mut slots = BTreeSet::new();
    let mut remapped_ids = BTreeMap::new();
    for entity in entities {
        let old_id = entity["id"]
            .as_str()
            .ok_or("template constituent has no stable output slot")?
            .to_owned();
        if !slots.insert(old_id.clone()) {
            return Err("duplicate template constituent slot".into());
        }
        let entity_ref = format!(
            "{expression_ref}:entity:p-{}",
            &fingerprint(&(&identity, &old_id))?[..32]
        );
        let title = entity["name"]
            .as_str()
            .ok_or("template constituent has no title")?
            .to_owned();
        entity["id"] = Value::String(entity_ref.clone());
        if let Some(native) = entity.get_mut("native").filter(|v| !v.is_null()) {
            if native["id"].as_str() != Some(old_id.as_str()) {
                return Err(
                    "native constituent projection differs from the template occurrence".into(),
                );
            }
            native["id"] = Value::String(entity_ref.clone());
        }
        remapped_ids.insert(old_id, entity_ref.clone());
        native_changes.push(NativeChange::EntityAdd {
            scene_ref: scene_ref.into(),
            entity_ref: entity_ref.clone(),
            title,
        });
        native_changes.push(NativeChange::SubjectBind {
            entity_ref: entity_ref.clone(),
            binding: subjects[0].clone(),
        });
        native_refs.push(entity_ref);
    }
    remap_occurrence_material(scene, &old_scene_ref, scene_ref, &remapped_ids)?;
    native_changes.push(NativeChange::SceneCompose {
        scene_ref: scene_ref.into(),
        entity_refs: native_refs,
    });
    native_changes.push(NativeChange::SceneMaterialSet {
        scene_ref: scene_ref.into(),
        presentation: presentation.clone(),
    });
    Ok(GeneratedContribution {
        contribution_ref: identity,
        procedure_ref: procedure.procedure_ref.clone(),
        output_slot: output_slot.into(),
        subjects: subject_refs,
        occurrence_ref: scene_ref.into(),
        recipe: procedure.recipe.clone(),
        procedure_revision: procedure.revision.clone(),
        owned_addresses: vec![OwnedAddress {
            expression_ref: expression_ref.into(),
            scene_ref: Some(scene_ref.into()),
            entity_ref: None,
            component: "scene".into(),
            constituent_ref: None,
            property: None,
        }],
        native_changes,
        generated_basis: presentation,
    })
}

/// Turn a retained three-way diff into ordinary native operations. Existing
/// scenes are revised, never re-created. Edited retired scenes remain detached
/// authored material; their subjects are never removed by this procedure.
pub fn regeneration_native_changes(
    delta: &Regeneration,
    previous: &[GeneratedContribution],
    current: &[CurrentContribution],
) -> Result<Vec<NativeChange>> {
    let previous = previous
        .iter()
        .map(|c| (c.contribution_ref.clone(), c))
        .collect::<BTreeMap<_, _>>();
    let current = current
        .iter()
        .map(|c| (c.contribution_ref.clone(), c))
        .collect::<BTreeMap<_, _>>();
    let mut changes = Vec::new();
    for contribution in &delta.contributions {
        let effective = delta
            .effective_basis
            .get(&contribution.contribution_ref)
            .ok_or("generated effective material absent")?;
        if !previous.contains_key(&contribution.contribution_ref) {
            for change in &contribution.native_changes {
                changes.push(match change {
                    NativeChange::SceneMaterialSet { scene_ref, .. } => {
                        NativeChange::SceneMaterialSet {
                            scene_ref: scene_ref.clone(),
                            presentation: procedural_intent_projection(effective)?,
                        }
                    }
                    _ => change.clone(),
                });
            }
            continue;
        }
        let actual = current
            .get(&contribution.contribution_ref)
            .ok_or("native regeneration read missing")?;
        if effective == &actual.material {
            continue;
        }
        if effective["schema"] == "oi.journey-scene/v1" {
            let scene_ref = effective["scene"]["id"]
                .as_str()
                .ok_or("generated scene lost native identity")?;
            let old_entities = actual.material["scene"]["entities"]
                .as_array()
                .ok_or("current scene has no constituents")?;
            let new_entities = effective["scene"]["entities"]
                .as_array()
                .ok_or("effective scene has no constituents")?;
            let old_ids = keyed_material(old_entities, "current/entities")?;
            let new_ids = keyed_material(new_entities, "effective/entities")?;
            for (id, entity) in &new_ids {
                if !old_ids.contains_key(id) {
                    changes.push(NativeChange::EntityAdd {
                        scene_ref: scene_ref.into(),
                        entity_ref: id.clone(),
                        title: entity["name"]
                            .as_str()
                            .ok_or("new constituent has no title")?
                            .into(),
                    });
                    if let Some(binding) =
                        contribution
                            .native_changes
                            .iter()
                            .find_map(|change| match change {
                                NativeChange::SubjectBind {
                                    entity_ref,
                                    binding,
                                } if entity_ref == id => Some(binding.clone()),
                                _ => None,
                            })
                    {
                        changes.push(NativeChange::SubjectBind {
                            entity_ref: id.clone(),
                            binding,
                        });
                    }
                }
            }
            for id in old_ids.keys().filter(|id| !new_ids.contains_key(*id)) {
                changes.push(NativeChange::EntityRemove {
                    entity_ref: id.clone(),
                });
            }
            let entity_refs = new_entities
                .iter()
                .map(|entity| entity["id"].as_str().unwrap().to_owned())
                .collect();
            changes.push(NativeChange::SceneCompose {
                scene_ref: scene_ref.into(),
                entity_refs,
            });
            changes.push(NativeChange::SceneMaterialSet {
                scene_ref: scene_ref.into(),
                presentation: procedural_intent_projection(effective)?,
            });
        } else {
            return Err(
                "non-scene generated diff requires its typed native property operation".into(),
            );
        }
    }
    for reference in &delta.retired {
        let old = previous
            .get(reference)
            .ok_or("retired contribution original basis absent")?;
        if old.generated_basis["schema"] != "oi.journey-scene/v1" {
            return Err("retirement requires the contribution's native owner operation".into());
        }
        changes.push(NativeChange::SceneRemove {
            scene_ref: old.occurrence_ref.clone(),
        });
    }
    if changes.len() > MAX_OPERATIONS {
        return Err("regeneration operation budget exceeded".into());
    }
    Ok(changes)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CurrentContribution {
    pub contribution_ref: String,
    pub material: Value,
    pub overlays: Vec<AuthoredOverlay>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Regeneration {
    pub contributions: Vec<GeneratedContribution>,
    pub effective_basis: BTreeMap<String, Value>,
    pub created: Vec<String>,
    pub updated: Vec<String>,
    pub retired: Vec<String>,
    pub detached: Vec<String>,
    pub retained_overlays: Vec<AuthoredOverlay>,
}

pub fn regenerate(
    previous: &[GeneratedContribution],
    current: &[CurrentContribution],
    next: &[GeneratedContribution],
    policy: RemovalPolicy,
) -> Result<Regeneration> {
    if previous.len() > 2048 || current.len() > 2048 || next.len() > 2048 {
        return Err("regeneration exceeds native contribution bound".into());
    }
    let index =
        |values: &[GeneratedContribution]| -> Result<BTreeMap<String, GeneratedContribution>> {
            let mut result = BTreeMap::new();
            for value in values {
                if value.contribution_ref
                    != contribution_identity(
                        &value.procedure_ref,
                        &value.output_slot,
                        &value.subjects,
                        &value.occurrence_ref,
                    )?
                {
                    return Err(
                        "generated contribution identity does not match its exact basis".into(),
                    );
                }
                if result
                    .insert(value.contribution_ref.clone(), value.clone())
                    .is_some()
                {
                    return Err("duplicate generated contribution".into());
                }
            }
            Ok(result)
        };
    let previous = index(previous)?;
    let next_index = index(next)?;
    let mut current_index = BTreeMap::new();
    for value in current {
        if current_index
            .insert(value.contribution_ref.clone(), value)
            .is_some()
        {
            return Err("duplicate current contribution".into());
        }
    }
    let mut output = Regeneration {
        contributions: next.to_vec(),
        effective_basis: BTreeMap::new(),
        created: Vec::new(),
        updated: Vec::new(),
        retired: Vec::new(),
        detached: Vec::new(),
        retained_overlays: Vec::new(),
    };
    for (reference, generated) in &next_index {
        let old = previous.get(reference);
        let actual = current_index.get(reference).copied();
        let mut effective = match (old, actual) {
            (Some(old), Some(actual)) => reconcile_material(
                &old.generated_basis,
                &actual.material,
                &generated.generated_basis,
            )?,
            (Some(_), None) => {
                return Err("regeneration lacks the current native contribution reading".into());
            }
            (None, Some(_)) => {
                return Err(
                    "generated creation collides with an existing unowned contribution".into(),
                );
            }
            (None, None) => generated.generated_basis.clone(),
        };
        if let Some(actual) = actual {
            for overlay in &actual.overlays {
                if overlay.contribution_ref != *reference {
                    return Err("authored overlay targets a different contribution".into());
                }
                nonempty(&overlay.actor_ref, "overlay actor")?;
                validate_material(&overlay.value, 0)?;
                if overlay.persistent {
                    *overlay_target(&mut effective, &overlay.pointer)? = overlay.value.clone();
                    output.retained_overlays.push(overlay.clone());
                }
            }
        }
        if old.is_none() {
            output.created.push(reference.clone());
        } else if old.is_some_and(|old| {
            old.generated_basis != generated.generated_basis
                || old.procedure_revision != generated.procedure_revision
        }) {
            output.updated.push(reference.clone());
        }
        output.effective_basis.insert(reference.clone(), effective);
    }
    for (reference, old) in &previous {
        if next_index.contains_key(reference) {
            continue;
        }
        let actual = current_index
            .get(reference)
            .ok_or("retirement lacks the actual native contribution")?;
        let edited =
            actual.material != old.generated_basis || actual.overlays.iter().any(|o| o.persistent);
        if edited {
            if policy == RemovalPolicy::ConflictOnEdited {
                return Err(format!(
                    "edited contribution {reference} requires a removal decision"
                ));
            }
            output.detached.push(reference.clone());
            output
                .effective_basis
                .insert(reference.clone(), actual.material.clone());
            output
                .retained_overlays
                .extend(actual.overlays.iter().filter(|o| o.persistent).cloned());
        } else {
            output.retired.push(reference.clone());
        }
    }
    Ok(output)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetainedOutputReading {
    pub schema: String,
    pub native_owner: String,
    pub expression_ref: String,
    pub document_revision: u64,
    pub procedure_ref: String,
    pub source_basis: Vec<SourceBasis>,
    pub contribution_ref: String,
    pub output_slot: String,
    pub subject_refs: Vec<String>,
    pub occurrence_ref: String,
    pub recipe_revision: String,
    pub owned_addresses: Vec<OwnedAddress>,
    pub generated_basis: Value,
    /// Exact native material projection at document_revision: omit only
    /// scene.procedural to avoid recursive journal embedding. Human material
    /// and every other Presentation field, including saved, are preserved.
    pub current_basis: Value,
    pub status: String,
    /// Opaque FULL ORIGINAL CREATION Operation for this contribution, stable
    /// across regeneration. It contains no prior output readings. Native S
    /// checks its digest against the typed journal; it is no new ACK.
    pub applied_operation: Value,
}

pub const RETAINED_OUTPUT_READING: &str = "oi.expression-procedural-output-reading/v1";

pub fn procedural_material_projection(presentation: &Value) -> Result<Value> {
    if presentation["schema"] != "oi.journey-scene/v1" || !presentation["scene"].is_object() {
        return Err("native material projection requires an actual Scene Presentation".into());
    }
    validate_material(presentation, 0)?;
    let mut value = presentation.clone();
    if let Some(scene) = value.get_mut("scene").and_then(Value::as_object_mut) {
        scene.remove("procedural");
    }
    Ok(value)
}

/// Canonical operation intent preserves authored procedural definitions,
/// controls and contributions, but does not recursively carry journal rows.
/// The native owner inherits its retained rows and adds the new receipt at CAS.
pub fn procedural_intent_projection(presentation: &Value) -> Result<Value> {
    if presentation["schema"] != "oi.journey-scene/v1" || !presentation["scene"].is_object() {
        return Err("native intent projection requires an actual Scene Presentation".into());
    }
    validate_material(presentation, 0)?;
    let mut value = presentation.clone();
    if let Some(retention) = value["scene"].get_mut("procedural") {
        retention
            .as_object_mut()
            .ok_or("native procedural metadata must be an object")?
            .insert("operations".into(), json!([]));
    }
    Ok(value)
}

fn validate_output_readings(
    procedure: &Procedure,
    expression_ref: &str,
    document_revision: u64,
    previous: &[GeneratedContribution],
    current: &[CurrentContribution],
    readings: &[RetainedOutputReading],
) -> Result<BTreeMap<String, Vec<OwnedAddress>>> {
    if readings.len() > procedure.budgets.max_active_instances {
        return Err("retained output reading budget exceeded".into());
    }
    let mut result = BTreeMap::new();
    for reading in readings {
        if reading.schema != RETAINED_OUTPUT_READING
            || reading.native_owner != "oi.expression"
            || reading.expression_ref != expression_ref
            || reading.document_revision != document_revision
            || reading.procedure_ref != procedure.procedure_ref
            || reading.status != "active"
            || reading.source_basis.is_empty()
            || reading.source_basis.len() > 256
            || !reading.source_basis.contains(&procedure.profile)
        {
            return Err(
                "retained output has a wrong native owner, subject, source or document revision"
                    .into(),
            );
        }
        for basis in &reading.source_basis {
            basis.validate()?;
        }
        if reading.current_basis["scene"].get("procedural").is_some()
            || reading.generated_basis["scene"].get("procedural").is_some()
        {
            return Err(
                "retained output material must use the nontransitive native projection".into(),
            );
        }
        let old = previous
            .iter()
            .find(|c| c.contribution_ref == reading.contribution_ref)
            .ok_or("retained output is not an original contribution")?;
        let actual = current
            .iter()
            .find(|c| c.contribution_ref == reading.contribution_ref)
            .ok_or("retained output has no actual current contribution")?;
        if old.procedure_ref != reading.procedure_ref
            || old.output_slot != reading.output_slot
            || old.subjects != reading.subject_refs
            || old.occurrence_ref != reading.occurrence_ref
            || old.recipe.revision != reading.recipe_revision
            || old.owned_addresses != reading.owned_addresses
            || old.generated_basis != reading.generated_basis
            || actual.material != reading.current_basis
            || !reading.source_basis.contains(&old.recipe)
            || reading.contribution_ref
                != contribution_identity(
                    &reading.procedure_ref,
                    &reading.output_slot,
                    &reading.subject_refs,
                    &reading.occurrence_ref,
                )?
        {
            return Err(
                "retained output identity, original source basis or actual material differs".into(),
            );
        }
        validate_material(&reading.applied_operation, 0)?;
        validate_material(&reading.current_basis, 0)?;
        let operation = &reading.applied_operation;
        let envelope = &operation["envelope"];
        if envelope
            .get("output_readings")
            .is_some_and(|readings| readings.as_array().is_none_or(|values| !values.is_empty()))
        {
            return Err(
                "retained output requires its nontransitive original creation operation".into(),
            );
        }
        if !operation["observations"].is_array()
            || !operation["failure"].is_null()
            || !envelope["participants"].is_array()
            || !envelope["scope"].is_object()
            || !envelope["timing"].is_object()
        {
            return Err("retained output lacks the full native operation and envelope".into());
        }
        let original_targets: Vec<OwnedAddress> =
            serde_json::from_value(operation["targets"].clone())
                .map_err(|_| "retained output operation lacks its original resolved targets")?;
        for target in &original_targets {
            target.validate()?;
        }
        if reading
            .owned_addresses
            .iter()
            .any(|owned| !original_targets.iter().any(|target| target.covers(owned)))
        {
            return Err(
                "retained output ownership was not in the original native operation".into(),
            );
        }
        let committed = operation["applied_revision"]
            .as_u64()
            .ok_or("retained output lacks an actual committed revision")?;
        let accepted = operation["accepted_revision"]
            .as_u64()
            .ok_or("retained output lacks an accepted native operation")?;
        let requested = envelope["expected_revision"]
            .as_u64()
            .ok_or("retained output lacks original native CAS")?;
        let digest = operation["fingerprint"]
            .as_str()
            .ok_or("retained output lacks its native operation digest")?;
        if operation["status"] != "applied"
            || envelope["expression_ref"] != expression_ref
            || committed > document_revision
            || committed <= accepted
            || accepted <= requested
            || digest.len() != 64
            || !digest.bytes().all(|c| c.is_ascii_hexdigit())
            || envelope["operation_ref"].as_str().is_none_or(str::is_empty)
        {
            return Err(
                "retained output operation is foreign, uncommitted or has a stale native CAS basis"
                    .into(),
            );
        }
        let changes = envelope["changes"]
            .as_array()
            .ok_or("retained output operation has no full original changes")?;
        if !changes.iter().any(|change| {
            change["change"] == "scene_create" && change["scene_ref"] == reading.occurrence_ref
        }) || !changes.iter().any(|change| {
            change["change"] == "scene_material_set"
                && change["scene_ref"] == reading.occurrence_ref
                && change["presentation"]["schema"] == "oi.journey-scene/v1"
                && change["presentation"]["scene"]["id"] == reading.occurrence_ref
        }) {
            return Err(
                "retained output does not match its original applied native scene material".into(),
            );
        }
        let sources = envelope["sources"]
            .as_array()
            .ok_or("retained output operation has no original qualified sources")?;
        if sources.is_empty()
            || sources.iter().any(|s| {
                s["availability"] != "available"
                    || s["ref"].as_str().is_none_or(str::is_empty)
                    || s["revision"].as_str().is_none_or(str::is_empty)
            })
        {
            return Err("retained output creation has unqualified original sources".into());
        }
        // Later generations retain their new recipe basis separately. The
        // immutable creation receipt proves native identity/initial subject;
        // the native owner attests latest generated/current material and CAS.
        if !changes.iter().any(|change| {
            change["change"] == "subject_bind"
                && change["binding"]["subject_ref"] == procedure.principal_subject_ref
                && change["entity_ref"].as_str().is_some_and(|entity| {
                    changes.iter().any(|created| {
                        created["change"] == "entity_add"
                            && created["scene_ref"] == reading.occurrence_ref
                            && created["entity_ref"] == entity
                    })
                })
        }) {
            return Err("retained output creation does not bind the native principal".into());
        }
        if reading.current_basis["schema"] != "oi.journey-scene/v1"
            || reading.current_basis["scene"]["id"] != reading.occurrence_ref
            || reading.owned_addresses.is_empty()
        {
            return Err("retained output lacks an actual native scene occurrence".into());
        }
        let mut addresses = reading.owned_addresses.clone();
        for address in &addresses {
            address.validate()?;
            if address.expression_ref != expression_ref
                || address.scene_ref.as_deref() != Some(reading.occurrence_ref.as_str())
            {
                return Err("retained output ownership escapes its actual occurrence".into());
            }
        }
        let entities = reading.current_basis["scene"]["entities"]
            .as_array()
            .ok_or("retained output lacks its current constituents")?;
        for entity in entities {
            let address = OwnedAddress {
                expression_ref: expression_ref.into(),
                scene_ref: Some(reading.occurrence_ref.clone()),
                entity_ref: Some(
                    entity["id"]
                        .as_str()
                        .ok_or("retained native entity lacks identity")?
                        .into(),
                ),
                component: "entity".into(),
                constituent_ref: None,
                property: None,
            };
            address.validate()?;
            if reading
                .owned_addresses
                .iter()
                .any(|owned| owned.covers(&address))
            {
                addresses.push(address);
            }
        }
        if result
            .insert(reading.contribution_ref.clone(), addresses)
            .is_some()
        {
            return Err("duplicate retained output capability".into());
        }
    }
    Ok(result)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreparedProcedure {
    pub schema: String,
    pub operation_ref: String,
    pub fingerprint: String,
    pub procedure_ref: String,
    pub recipe_revision: String,
    pub source_revision: String,
    pub expected_document_revision: u64,
    pub membership: ResolvedMembership,
    /// Original membership is unchanged. Native S re-attests these separate
    /// retained output readings against its current Document and journal.
    pub output_readings: Vec<RetainedOutputReading>,
    pub contributions: Vec<GeneratedContribution>,
    pub timing: TimingBinding,
    pub required_consumers: BTreeSet<String>,
    pub native_edit: Value,
}

/// Compile the admitted recipe into the *same* native Edit used by humans.
/// Concurrent writers require disjoint owned addresses; shared modifiers must
/// already be composed by the state owner and supplied as one admitted write.
#[allow(clippy::too_many_arguments)]
pub fn compile_native_batch(
    registry: &MRegistry,
    procedure: &Procedure,
    operation_ref: &str,
    expression_ref: &str,
    document_revision: u64,
    membership: ResolvedMembership,
    contributions: Vec<GeneratedContribution>,
    required_consumers: BTreeSet<String>,
) -> Result<PreparedProcedure> {
    compile_native_batch_inner(
        registry,
        procedure,
        operation_ref,
        expression_ref,
        document_revision,
        membership,
        contributions,
        required_consumers,
        BTreeMap::new(),
        vec![],
    )
}

/// Continue only outputs actually retained by the SAME native owner. The
/// returned bare Edit is applied only after S independently rechecks current
/// CAS and its typed operation journal; this producer grants no authority.
#[allow(clippy::too_many_arguments)]
pub fn compile_native_regeneration_batch(
    registry: &MRegistry,
    procedure: &Procedure,
    operation_ref: &str,
    expression_ref: &str,
    document_revision: u64,
    membership: ResolvedMembership,
    contributions: Vec<GeneratedContribution>,
    required_consumers: BTreeSet<String>,
    previous: &[GeneratedContribution],
    current: &[CurrentContribution],
    output_readings: Vec<RetainedOutputReading>,
) -> Result<PreparedProcedure> {
    let retained = validate_output_readings(
        procedure,
        expression_ref,
        document_revision,
        previous,
        current,
        &output_readings,
    )?;
    compile_native_batch_inner(
        registry,
        procedure,
        operation_ref,
        expression_ref,
        document_revision,
        membership,
        contributions,
        required_consumers,
        retained,
        output_readings,
    )
}

#[allow(clippy::too_many_arguments)]
fn compile_native_batch_inner(
    registry: &MRegistry,
    procedure: &Procedure,
    operation_ref: &str,
    expression_ref: &str,
    document_revision: u64,
    membership: ResolvedMembership,
    contributions: Vec<GeneratedContribution>,
    required_consumers: BTreeSet<String>,
    retained: BTreeMap<String, Vec<OwnedAddress>>,
    output_readings: Vec<RetainedOutputReading>,
) -> Result<PreparedProcedure> {
    procedure.validate(registry)?;
    nonempty(operation_ref, "operation ref")?;
    nonempty(expression_ref, "Expression ref")?;
    if membership.selector != procedure.selector
        || membership.containing_expression_ref != expression_ref
        || membership.mode != procedure.membership_mode
    {
        return Err("prepared selector scope or membership mode differs".into());
    }
    if membership.targets.len() != membership.addresses.len()
        || membership
            .targets
            .keys()
            .any(|reference| !membership.addresses.contains_key(reference))
    {
        return Err("selector has no exact native address for each retained reading".into());
    }
    for address in membership.addresses.values() {
        address.validate()?;
        if address.expression_ref != expression_ref {
            return Err("selected native address escapes containing Expression".into());
        }
    }
    if contributions.len() > procedure.budgets.max_active_instances {
        return Err("active procedure contribution budget exceeded".into());
    }
    if required_consumers.is_empty()
        || required_consumers.len() > 16
        || !required_consumers.contains("scene")
    {
        return Err("prepared operation requires its actual scene consumer".into());
    }
    let mut writes: Vec<&OwnedAddress> = Vec::new();
    let mut references = BTreeSet::new();
    let mut changes = Vec::new();
    let mut created_scenes = BTreeSet::new();
    let mut created_entities = BTreeMap::new();
    for contribution in &contributions {
        let retained_addresses = retained
            .get(&contribution.contribution_ref)
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        if contribution.procedure_ref != procedure.procedure_ref
            || contribution.recipe != procedure.recipe
            || contribution.procedure_revision != procedure.revision
            || !contribution
                .subjects
                .contains(&procedure.principal_subject_ref)
            || contribution.contribution_ref
                != contribution_identity(
                    &contribution.procedure_ref,
                    &contribution.output_slot,
                    &contribution.subjects,
                    &contribution.occurrence_ref,
                )?
        {
            return Err(
                "contribution does not belong to the prepared procedure/subject/source".into(),
            );
        }
        if !references.insert(&contribution.contribution_ref)
            || contribution.owned_addresses.is_empty()
        {
            return Err("duplicate contribution or absent owned write set".into());
        }
        validate_material(&contribution.generated_basis, 0)?;
        for address in &contribution.owned_addresses {
            address.validate()?;
            if address.expression_ref != expression_ref
                || writes
                    .iter()
                    .any(|other| other.covers(address) || address.covers(other))
            {
                return Err("conflicting procedural writers or out-of-scope ownership".into());
            }
            writes.push(address);
        }
        for change in &contribution.native_changes {
            change.validate()?;
            if let NativeChange::SubjectBind { binding, .. } = change {
                validate_native_subject_basis(registry, binding)?;
            }
            if !procedure.admitted_changes.contains(change.name()) {
                return Err("recipe operation was not admitted by the native SDK".into());
            }
            let entity_scene = |entity: &str| -> Result<String> {
                membership
                    .addresses
                    .values()
                    .chain(retained_addresses.iter())
                    .find(|address| address.entity_ref.as_deref() == Some(entity))
                    .and_then(|address| address.scene_ref.clone())
                    .or_else(|| created_entities.get(entity).cloned())
                    .or_else(|| {
                        contribution
                            .owned_addresses
                            .iter()
                            .find(|address| address.entity_ref.as_deref() == Some(entity))
                            .and_then(|address| address.scene_ref.clone())
                    })
                    .ok_or_else(|| {
                        "native entity write ownership has no resolved containing scene".into()
                    })
            };
            let (scene, entity, component, property) = match change {
                NativeChange::SceneCreate { scene_ref, .. }
                | NativeChange::SceneRemove { scene_ref }
                | NativeChange::SceneCompose { scene_ref, .. }
                | NativeChange::SceneMaterialSet { scene_ref, .. } => {
                    (Some(scene_ref.clone()), None, "scene", None)
                }
                NativeChange::EntityAdd {
                    scene_ref,
                    entity_ref,
                    ..
                } => (
                    Some(scene_ref.clone()),
                    Some(entity_ref.clone()),
                    "entity",
                    None,
                ),
                NativeChange::EntityRemove { entity_ref }
                | NativeChange::SubjectBind { entity_ref, .. } => (
                    Some(entity_scene(entity_ref)?),
                    Some(entity_ref.clone()),
                    "entity",
                    None,
                ),
                NativeChange::ParameterSet {
                    entity_ref,
                    parameter,
                    ..
                } => (
                    Some(entity_scene(entity_ref)?),
                    Some(entity_ref.clone()),
                    "property",
                    Some(parameter.clone()),
                ),
                NativeChange::Focus {
                    scene_ref,
                    entity_ref,
                } => (
                    Some(scene_ref.clone()),
                    entity_ref.clone(),
                    if entity_ref.is_some() {
                        "entity"
                    } else {
                        "scene"
                    },
                    None,
                ),
                NativeChange::SceneReorder { .. } => (None, None, "expression", None),
            };
            let target = OwnedAddress {
                expression_ref: expression_ref.into(),
                scene_ref: scene.clone(),
                entity_ref: entity.clone(),
                component: component.into(),
                constituent_ref: None,
                property,
            };
            target.validate()?;
            if let NativeChange::SceneCompose {
                scene_ref,
                entity_refs,
            } = change
            {
                if entity_refs.iter().any(|entity_ref| {
                    created_entities.get(entity_ref) != Some(scene_ref)
                        && !membership
                            .addresses
                            .values()
                            .chain(retained_addresses.iter())
                            .any(|address| {
                                address.scene_ref.as_deref() == Some(scene_ref)
                                    && address.entity_ref.as_deref() == Some(entity_ref)
                            })
                }) {
                    return Err(
                        "scene composition includes an entity outside resolved native membership"
                            .into(),
                    );
                }
            }
            if !contribution
                .owned_addresses
                .iter()
                .any(|address| address.covers(&target))
            {
                return Err("native recipe write exceeds its exact declared ownership".into());
            }
            let selected = membership
                .addresses
                .values()
                .chain(retained_addresses.iter())
                .any(|address| {
                    // For an existing entity, its scene association must come from
                    // the owner reading, never a producer's declared owned set.
                    (target.entity_ref.is_none()
                        || address.entity_ref == target.entity_ref
                        || address.component == "expression")
                        && address.covers(&target)
                });
            match change {
                NativeChange::SceneCreate { scene_ref, .. } => {
                    if membership
                        .addresses
                        .values()
                        .chain(retained_addresses.iter())
                        .any(|address| address.scene_ref.as_deref() == Some(scene_ref))
                        || !created_scenes.insert(scene_ref.clone())
                    {
                        return Err(
                            "recipe creation collides with an existing selected scene".into()
                        );
                    }
                }
                NativeChange::EntityAdd {
                    scene_ref,
                    entity_ref,
                    ..
                } => {
                    let selected_scene = OwnedAddress {
                        expression_ref: expression_ref.into(),
                        scene_ref: Some(scene_ref.clone()),
                        entity_ref: None,
                        component: "scene".into(),
                        constituent_ref: None,
                        property: None,
                    };
                    if (!created_scenes.contains(scene_ref)
                        && !membership
                            .addresses
                            .values()
                            .chain(retained_addresses.iter())
                            .any(|address| address.covers(&selected_scene)))
                        || membership
                            .addresses
                            .values()
                            .chain(retained_addresses.iter())
                            .any(|address| address.entity_ref.as_deref() == Some(entity_ref))
                        || created_entities
                            .insert(entity_ref.clone(), scene_ref.clone())
                            .is_some()
                    {
                        return Err("entity construction exceeds resolved scope or collides with existing identity".into());
                    }
                }
                _ => {
                    let constructed = entity
                        .as_ref()
                        .is_some_and(|entity| created_entities.get(entity) == scene.as_ref())
                        || (entity.is_none()
                            && scene
                                .as_ref()
                                .is_some_and(|scene| created_scenes.contains(scene)));
                    if !constructed && !selected {
                        return Err(
                            "native existing-target write exceeds resolved selector membership"
                                .into(),
                        );
                    }
                }
            }
            changes.push(serde_json::to_value(change).map_err(|e| e.to_string())?);
        }
    }
    if changes.is_empty() || changes.len() > procedure.budgets.max_operations {
        return Err("native operation batch is empty or exceeds its admitted budget".into());
    }
    let native_edit = json!({"operation":"edit","expression_ref":expression_ref,"expected_revision":document_revision,
        "actor":procedure.composition.actor,"changes":changes});
    let mut prepared = PreparedProcedure {
        schema: PROCEDURE_CONTRACT.into(),
        operation_ref: operation_ref.into(),
        fingerprint: String::new(),
        procedure_ref: procedure.procedure_ref.clone(),
        recipe_revision: procedure.recipe.revision.clone(),
        source_revision: procedure.registry_revision.clone(),
        expected_document_revision: document_revision,
        membership,
        output_readings,
        contributions,
        timing: procedure.timing.clone(),
        required_consumers,
        native_edit,
    };
    prepared.fingerprint = fingerprint(&prepared)?;
    Ok(prepared)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cause {
    pub event_ref: String,
    pub ancestors: Vec<String>,
    pub progress_revision: String,
    pub depth: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuleEvent {
    pub kind: TriggerKind,
    pub active: bool,
    pub cause: Cause,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RulePosition {
    Running,
    Paused,
    Cancelled,
}

/// Retained bounded procedure position at an *owner-supplied* interval/cursor.
/// The hosting owner persists this body; this does not create a second clock.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuleExecution {
    /// Exact authoring/source/locus/profile/C′/registry/trigger/seed basis.
    /// Compatible refresh is an explicit new authored operation, not resume.
    pub original_procedure: Procedure,
    pub procedure_ref: String,
    pub position: RulePosition,
    pub cursor: u64,
    pub epoch_ref: String,
    pub interval_ref: String,
    pub evaluations: usize,
    pub operations: usize,
    pub queue_high_water: usize,
    queue: VecDeque<RuleEvent>,
    last_edge: BTreeMap<String, bool>,
    visited: BTreeSet<(String, String)>,
    seen_intervals: BTreeSet<String>,
}

impl RuleExecution {
    pub fn new(procedure: &Procedure, interval_ref: &str) -> Result<Self> {
        nonempty(interval_ref, "native interval")?;
        procedure.budgets.validate()?;
        Ok(Self {
            original_procedure: procedure.clone(),
            procedure_ref: procedure.procedure_ref.clone(),
            position: RulePosition::Running,
            cursor: procedure.timing.requested_cursor,
            epoch_ref: procedure.timing.epoch_ref.clone(),
            interval_ref: interval_ref.into(),
            evaluations: 0,
            operations: 0,
            queue_high_water: 0,
            queue: VecDeque::new(),
            last_edge: BTreeMap::new(),
            visited: BTreeSet::new(),
            seen_intervals: BTreeSet::from([interval_ref.into()]),
        })
    }
    pub fn verify_procedure(&self, procedure: &Procedure) -> Result<()> {
        if &self.original_procedure != procedure
            || self.procedure_ref != procedure.procedure_ref
            || self.epoch_ref != procedure.timing.epoch_ref
        {
            return Err("retained rule checkpoint has a different original procedure/source/profile/locus/registry basis".into());
        }
        Ok(())
    }
    pub fn enqueue(&mut self, procedure: &Procedure, event: RuleEvent) -> Result<()> {
        self.verify_procedure(procedure)?;
        if self.position == RulePosition::Cancelled {
            return Err("procedure cancelled".into());
        }
        if self.procedure_ref != procedure.procedure_ref {
            return Err("event belongs to a different procedure".into());
        }
        if self.queue.len() >= procedure.budgets.max_queue
            || event.cause.depth > procedure.budgets.max_expansion_depth
            || event.cause.ancestors.len() > procedure.budgets.max_expansion_depth
        {
            return Err("bounded procedure queue or expansion depth exceeded".into());
        }
        nonempty(&event.cause.event_ref, "cause event")?;
        nonempty(&event.cause.progress_revision, "cause progress")?;
        self.queue.push_back(event);
        self.queue_high_water = self.queue_high_water.max(self.queue.len());
        Ok(())
    }
    pub fn next(
        &mut self,
        procedure: &Procedure,
        generated_operations: usize,
    ) -> Result<Option<RuleEvent>> {
        self.verify_procedure(procedure)?;
        if self.position != RulePosition::Running {
            return Ok(None);
        }
        if self.procedure_ref != procedure.procedure_ref
            || self.epoch_ref != procedure.timing.epoch_ref
        {
            return Err("procedure or timing epoch changed".into());
        }
        let Some(event) = self.queue.front() else {
            return Ok(None);
        };
        if self.evaluations >= procedure.budgets.max_evaluations
            || self
                .operations
                .checked_add(generated_operations)
                .is_none_or(|n| n > procedure.budgets.max_operations)
        {
            return Err("bounded procedure evaluations or operations exceeded".into());
        }
        let event = event.clone();
        self.queue.pop_front();
        self.evaluations += 1;
        if event.kind != procedure.trigger.kind {
            return Ok(None);
        }
        let last = self
            .last_edge
            .get(&format!("{:?}", event.kind))
            .copied()
            .unwrap_or(false);
        self.last_edge
            .insert(format!("{:?}", event.kind), event.active);
        if !event.active || (procedure.trigger.mode == TriggerMode::Edge && last) {
            return Ok(None);
        }
        let traversal = (
            self.procedure_ref.clone(),
            event.cause.progress_revision.clone(),
        );
        if (event.cause.ancestors.contains(&self.procedure_ref)
            || event.cause.ancestors.contains(&event.cause.event_ref))
            && self.visited.contains(&traversal)
        {
            return Err("non-progress procedural feedback cycle refused".into());
        }
        self.visited.insert(traversal);
        self.operations += generated_operations;
        if self.visited.len() > procedure.budgets.max_evaluations {
            return Err(
                "retained causal progress budget exceeded; checkpoint/restart required".into(),
            );
        }
        Ok(Some(event))
    }
    pub fn pause(&mut self, observed_cursor: u64) {
        self.cursor = observed_cursor;
        if self.position != RulePosition::Cancelled {
            self.position = RulePosition::Paused;
        }
    }
    pub fn resume(&mut self, epoch_ref: &str) -> Result<()> {
        if self.position == RulePosition::Cancelled || self.epoch_ref != epoch_ref {
            return Err("cancelled procedure or incompatible timing epoch".into());
        }
        self.position = RulePosition::Running;
        Ok(())
    }
    pub fn seek(&mut self, observed_cursor: u64, epoch_ref: &str) -> Result<()> {
        if self.epoch_ref != epoch_ref || self.position == RulePosition::Cancelled {
            return Err("seek needs the admitted owner epoch and continuing procedure".into());
        }
        self.cursor = observed_cursor;
        self.queue.clear();
        self.last_edge.clear();
        self.visited.clear();
        Ok(())
    }
    pub fn cancel(&mut self, observed_cursor: u64) {
        self.cursor = observed_cursor;
        self.position = RulePosition::Cancelled;
        self.queue.clear();
    }
    pub fn begin_interval(&mut self, interval_ref: &str, observed_cursor: u64) -> Result<()> {
        nonempty(interval_ref, "native interval")?;
        if self.position == RulePosition::Cancelled {
            return Err("cancelled procedure cannot begin another interval".into());
        }
        if self.seen_intervals.contains(interval_ref) {
            return Err(
                "interval budgets cannot be reset by repeating the same owner interval".into(),
            );
        }
        if self.seen_intervals.len() >= 4096 {
            return Err(
                "retained owner interval budget exhausted; checkpoint/restart required".into(),
            );
        }
        self.seen_intervals.insert(interval_ref.into());
        self.interval_ref = interval_ref.into();
        self.cursor = observed_cursor;
        self.evaluations = 0;
        self.operations = 0;
        Ok(())
    }
}

/// Use source-native C′ topology for which recipe legs may be prepared
/// together. Chain/Fusion consume actual predecessor returns at their owner.
pub fn cprime_dependencies(
    procedure: &Procedure,
    slots: &[String],
    actual_return_refs: &BTreeMap<String, String>,
) -> Result<BTreeMap<String, Vec<String>>> {
    if slots.is_empty()
        || slots.len() > procedure.budgets.max_active_instances
        || slots.iter().collect::<BTreeSet<_>>().len() != slots.len()
    {
        return Err("invalid C-prime recipe legs".into());
    }
    let mut result = BTreeMap::new();
    for (index, slot) in slots.iter().enumerate() {
        nonempty(slot, "C-prime leg")?;
        let prerequisites = match procedure.composition.thread {
            ThreadForm::Single if slots.len() != 1 => {
                return Err("single-voice C-prime requires one recipe leg".into());
            }
            ThreadForm::Chain if index > 0 => vec![
                actual_return_refs
                    .get(&slots[index - 1])
                    .ok_or("chained recipe needs the actual preceding Return")?
                    .clone(),
            ],
            ThreadForm::Fusion if index + 1 == slots.len() && slots.len() > 1 => slots[..index]
                .iter()
                .map(|slot| {
                    actual_return_refs
                        .get(slot)
                        .cloned()
                        .ok_or_else(|| "fusion recipe requires all actual contributor Returns".to_owned())
                })
                .collect::<Result<Vec<_>>>()?,
            ThreadForm::Sustained if procedure.membership_mode != MembershipMode::Sustained => {
                return Err("sustained C-prime requires explicit sustained membership".into());
            }
            ThreadForm::Nested if procedure.timing.time_mapping_ref.is_none() => {
                return Err("nested C-prime needs its admitted parent time mapping".into());
            }
            _ => Vec::new(),
        };
        result.insert(slot.clone(), prerequisites);
    }
    Ok(result)
}
