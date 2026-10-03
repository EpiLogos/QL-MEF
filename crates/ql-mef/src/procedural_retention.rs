//! Producer-owned native Scene retention before immutable intent qualification.
//! Journal rows are inherited by the SAME receiving owner; this never emits ACKs.
use crate::m_tree::native_current_m_registry;
use crate::procedural_composition::{OwnedAddress, PreparedProcedure, Result};
use crate::procedural_manifestation::{
    NativeReading, NativeSubject, ReadingAvailability, fingerprint, nonempty, validate_material,
    validate_native_subject_basis,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
pub const MATERIALIZATION_CONTRACT: &str = "ql.procedural-materialization/v1";
pub const RETENTION_SCHEMA: &str = "oi.expression-procedural/v1";
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeRetentionScene {
    pub scene_ref: String,
    pub document_revision: u64,
    /// Exact current native retention with operations projected to []; None
    /// only for an occurrence actually created by this prepared native Edit.
    pub existing_retention: Option<Value>,
    /// Metadata-only property/Atlas retention consumes SAME current native
    /// Presentation projection without procedural. Receiving S verifies exact
    /// unchanged material at source-sealed CAS before actual parameter op.
    pub current_presentation: Option<Value>,
    pub principal: NativeSubject,
    pub contributors: Vec<NativeSubject>,
    pub locus: NativeReading,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeMaterialization {
    pub schema: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lifecycles: Vec<NativeContributionLifecycle>,
    pub document_revision: u64,
    /// An ordinal of native rule-event conduct, never fractional sim seconds.
    pub rule_cursor: u64,
    pub state: String,
    pub scenes: Vec<NativeRetentionScene>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContributionLifecycle {
    Retired,
    Detached,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeContributionLifecycle {
    pub contribution_ref: String,
    pub status: ContributionLifecycle,
}
pub fn empty_retention() -> Value {
    json!({"schema":RETENTION_SCHEMA,"bindings":[],"procedures":[],"contributions":[],"controls":[],"operations":[],"scene_flow":[],"time_mappings":[],"source_basis":[]})
}
fn rows<'a>(r: &'a mut Value, key: &str) -> Result<&'a mut Vec<Value>> {
    r[key]
        .as_array_mut()
        .ok_or_else(|| format!("native retention lacks {key}"))
}
fn replace_row(rows: &mut Vec<Value>, key: &str, identity: &Value, value: Value) {
    if let Some(i) = rows.iter().position(|r| &r[key] == identity) {
        rows[i] = value;
    } else {
        rows.push(value);
    }
}
fn validate_existing(value: &Value) -> Result<()> {
    validate_material(value, 0)?;
    if value["schema"] != RETENTION_SCHEMA
        || value["operations"].as_array().is_none_or(|a| !a.is_empty())
    {
        return Err("native retention basis must project receipt rows to operations:[]".into());
    }
    for (key, bound) in [
        ("bindings", 2048),
        ("procedures", 64),
        ("contributions", 2048),
        ("controls", 2048),
        ("operations", 256),
        ("scene_flow", 64),
        ("time_mappings", 64),
        ("source_basis", 256),
    ] {
        if value[key].as_array().is_none_or(|a| a.len() > bound) {
            return Err(format!("native retention {key} bound/array missing"));
        }
    }
    Ok(())
}
/// The metadata anchor is the exact canonical source Scene sealed in the
/// original constructor. Its whole Scene address never grants authored writes.
pub fn source_scene_anchor(
    procedure: &crate::procedural_composition::Procedure,
    contribution: &crate::procedural_composition::GeneratedContribution,
) -> Result<Option<OwnedAddress>> {
    source_scene_anchor_identity(
        procedure,
        &contribution.procedure_ref,
        &contribution.output_slot,
        &contribution.occurrence_ref,
    )
}
fn source_scene_anchor_identity(
    procedure: &crate::procedural_composition::Procedure,
    procedure_ref: &str,
    output_slot: &str,
    occurrence_ref: &str,
) -> Result<Option<OwnedAddress>> {
    use crate::procedural_conduct::NativeRecipeProgram;
    let Some(program) = procedure.recipe_parameters.get("native_program") else {
        return Ok(None);
    };
    let program: NativeRecipeProgram =
        serde_json::from_value(program.clone()).map_err(|e| e.to_string())?;
    let NativeRecipeProgram::SceneMaterial { outputs } = program else {
        return Ok(None);
    };
    let output = outputs
        .iter()
        .find(|o| o.output_slot == output_slot && o.scene_ref == occurrence_ref)
        .ok_or("created Scene contribution has no exact original source constructor")?;
    if output.source.expression_ref != procedure.occurrence_ref
        || procedure_ref != procedure.procedure_ref
    {
        return Err("Scene metadata anchor escapes original source Expression/procedure".into());
    }
    let address = OwnedAddress {
        expression_ref: procedure.occurrence_ref.clone(),
        scene_ref: Some(output.source.scene_ref.clone()),
        entity_ref: None,
        component: "scene".into(),
        constituent_ref: None,
        parent_ref: None,
        property: None,
    };
    address.validate()?;
    Ok(Some(address))
}

/// Only material metadata changes inside already-owned SceneMaterialSet.
/// Call this before qualify_native_cprime; the final fingerprint covers exact
/// Procedure/seed/targets/generated basis and retained source-owned material.
pub fn materialize_retention(
    prepared: &mut PreparedProcedure,
    context: &NativeMaterialization,
) -> Result<()> {
    let mut original = prepared.clone();
    original.fingerprint = String::new();
    if prepared.native_cprime.is_some() || fingerprint(&original)? != prepared.fingerprint {
        return Err(
            "retention materialization requires unchanged unqualified native preparation".into(),
        );
    }
    if context.rule_cursor > 9_007_199_254_740_991
        || context.schema != MATERIALIZATION_CONTRACT
        || context.document_revision != prepared.expected_document_revision
        || !["running", "held", "interrupted", "retired"].contains(&context.state.as_str())
        || context.scenes.len() > 2048
    {
        return Err("native retention lacks exact prepared Document/CAS/position".into());
    }
    let mut lifecycle_refs = BTreeSet::new();
    for lifecycle in &context.lifecycles {
        if !lifecycle_refs.insert(&lifecycle.contribution_ref)
            || !prepared
                .output_readings
                .iter()
                .any(|r| r.contribution_ref == lifecycle.contribution_ref)
        {
            return Err(
                "contribution lifecycle lacks original SAME owner retained-output reading".into(),
            );
        }
        let contribution = prepared
            .contributions
            .iter()
            .find(|c| c.contribution_ref == lifecycle.contribution_ref)
            .ok_or("contribution lifecycle has no exact native ownership declaration")?;
        let actual = prepared
            .output_readings
            .iter()
            .find(|r| r.contribution_ref == lifecycle.contribution_ref)
            .unwrap();
        if lifecycle.status==ContributionLifecycle::Detached && contribution.native_changes.iter().any(|change|!
            matches!(change,crate::procedural_composition::NativeChange::SceneMaterialSet{scene_ref,presentation}
                if scene_ref==&contribution.occurrence_ref && presentation==&actual.current_basis)) {
            return Err("detachment changes actual current authored material rather than native retained ownership only".into());
        }
        if lifecycle.status==ContributionLifecycle::Retired && contribution.generated_basis["schema"]=="oi.journey-scene/v1" &&
            !contribution.native_changes.iter().any(|change|matches!(change,crate::procedural_composition::NativeChange::SceneRemove{scene_ref} if scene_ref==&contribution.occurrence_ref)) {
            return Err("retired created Scene contribution lacks actual SceneRemove".into());
        }
    }
    let mut readings = BTreeMap::new();
    for source in &context.scenes {
        nonempty(&source.scene_ref, "native retained Scene")?;
        if source.document_revision != context.document_revision
            || readings.insert(source.scene_ref.clone(), source).is_some()
        {
            return Err("duplicate/stale native retention scene basis".into());
        }
        validate_native_subject_basis(native_current_m_registry(), &source.principal)?;
        if source.principal.subject_ref != prepared.original_procedure.principal_subject_ref
            || source.locus.reference != prepared.original_procedure.locus_ref
            || source.locus.availability != ReadingAvailability::Available
            || source.contributors.len() > 64
        {
            return Err("native retained binding differs from exact source procedure".into());
        }
        nonempty(&source.locus.revision, "native locus revision")?;
        for s in &source.contributors {
            validate_native_subject_basis(native_current_m_registry(), s)?;
        }
        if let Some(r) = &source.existing_retention {
            validate_existing(r)?;
        }
    }
    let mut anchors = BTreeSet::new();
    let mut metadata_scope = Vec::new();
    for contribution in &prepared.contributions {
        if let Some(address) = source_scene_anchor(&prepared.original_procedure, contribution)? {
            let scene = address.scene_ref.as_ref().unwrap();
            if !readings.contains_key(scene) {
                return Err("created Scene requires actual SAME canonical source anchor context before intent sealing".into());
            }
            if anchors.insert(scene.clone()) {
                metadata_scope.push(address);
            }
        }
    }
    if !prepared.metadata_scope.is_empty() && prepared.metadata_scope != metadata_scope {
        return Err("caller supplied metadata scope differs from sealed source anchors".into());
    }
    let created = prepared.native_edit["changes"]
        .as_array()
        .ok_or("native Edit changes absent")?
        .iter()
        .filter(|c| c["change"] == "scene_create")
        .filter_map(|c| c["scene_ref"].as_str())
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    let mut edit = prepared.native_edit.clone();
    let mut touched = BTreeSet::new();
    for change in edit["changes"]
        .as_array_mut()
        .ok_or("native Edit changes absent")?
    {
        if change["change"] != "scene_material_set" {
            continue;
        }
        let scene_ref = change["scene_ref"]
            .as_str()
            .ok_or("native retained Scene ref absent")?
            .to_owned();
        let source = readings
            .get(&scene_ref)
            .ok_or("Scene material requires exact native retention source context")?;
        if source.existing_retention.is_none() && !created.contains(&scene_ref) {
            return Err("existing native Scene cannot lose its original retention".into());
        }
        let address = OwnedAddress {
            expression_ref: prepared.original_procedure.occurrence_ref.clone(),
            scene_ref: Some(scene_ref.clone()),
            entity_ref: None,
            component: "scene".into(),
            constituent_ref: None,
            parent_ref: None,
            property: None,
        };
        let retention = retention_for_scene(prepared, context, source, &[address])?;
        change["presentation"]["scene"]
            .as_object_mut()
            .ok_or("retained material has no native Scene")?
            .insert("procedural".into(), retention);
        touched.insert(scene_ref);
    }
    let mut metadata_changes = Vec::new();
    for (scene_ref, source) in &readings {
        if touched.contains(scene_ref) {
            continue;
        }
        let mut presentation = source
            .current_presentation
            .clone()
            .ok_or("metadata-only native retention requires actual current Scene presentation")?;
        if source.existing_retention.is_none()
            || presentation["schema"] != "oi.journey-scene/v1"
            || presentation["scene"]["id"] != *scene_ref
            || presentation["scene"].get("procedural").is_some()
        {
            return Err(
                "metadata-only retention lacks nontransitive SAME current native Scene/retention"
                    .into(),
            );
        }
        let mut addresses = prepared
            .contributions
            .iter()
            .flat_map(|c| &c.owned_addresses)
            .chain(prepared.membership.addresses.values())
            .filter(|a| {
                a.scene_ref.as_deref() == Some(scene_ref.as_str())
                    || a.component == "expression" && a.property.is_none()
            })
            .cloned()
            .collect::<Vec<_>>();
        if anchors.contains(scene_ref) {
            addresses.extend(
                metadata_scope
                    .iter()
                    .filter(|a| a.scene_ref.as_ref() == Some(scene_ref))
                    .cloned(),
            );
        }
        if addresses.is_empty() {
            let scene_address = OwnedAddress {
                expression_ref: prepared.original_procedure.occurrence_ref.clone(),
                scene_ref: Some(scene_ref.clone()),
                entity_ref: None,
                component: "scene".into(),
                constituent_ref: None,
                parent_ref: None,
                property: None,
            };
            if prepared
                .membership
                .addresses
                .values()
                .any(|owner| owner.covers(&scene_address))
            {
                addresses.push(scene_address);
            } else {
                return Err("metadata-only retention Scene is outside actual source scope".into());
            }
        }
        let retention = retention_for_scene(prepared, context, source, &addresses)?;
        presentation["scene"]
            .as_object_mut()
            .unwrap()
            .insert("procedural".into(), retention);
        // Native ParameterSet MUST follow metadata material, or the saved
        // before-presentation would overwrite its owner-derived scalar effect.
        metadata_changes.push(json!({"change":"scene_material_set","scene_ref":scene_ref,"presentation":presentation}));
        touched.insert(scene_ref.clone());
    }
    let changes = edit["changes"].as_array_mut().unwrap();
    metadata_changes.append(changes);
    *changes = metadata_changes;
    if touched.is_empty() {
        return Err("native procedure has no actual retained Scene target".into());
    }
    validate_material(&edit, 0)?;
    let mut next = prepared.clone();
    next.native_edit = edit;
    next.metadata_scope = metadata_scope;
    next.fingerprint = String::new();
    next.fingerprint = fingerprint(&next)?;
    *prepared = next;
    Ok(())
}

fn retention_for_scene(
    prepared: &PreparedProcedure,
    context: &NativeMaterialization,
    source: &NativeRetentionScene,
    addresses: &[OwnedAddress],
) -> Result<Value> {
    let procedure = &prepared.original_procedure;
    let scene_ref = &source.scene_ref;
    let source_basis = json!([{"ref":procedure.recipe.source_ref,"revision":procedure.recipe.revision,"availability":"available"},
        {"ref":procedure.profile.source_ref,"revision":procedure.profile.revision,"availability":"available"}]);
    let mut retention = source
        .existing_retention
        .clone()
        .unwrap_or_else(empty_retention);
    for address in addresses {
        address.validate()?;
        let binding = json!({"address":address,"principal":source.principal,"contributors":source.contributors,"locus":source.locus,"tags":[]});
        let bindings = rows(&mut retention, "bindings")?;
        // Preserve authored/generated tags on a current binding; subjects and
        // exact source readings remain explicitly provided native owner basis.
        if let Some(i) = bindings
            .iter()
            .position(|b| b["address"] == binding["address"])
        {
            let tags = bindings[i]["tags"].clone();
            bindings[i] = binding;
            bindings[i]["tags"] = tags;
        } else {
            bindings.push(binding);
        }
    }
    let previous = rows(&mut retention, "procedures")?
        .iter()
        .find(|p| p["procedure_ref"] == procedure.procedure_ref)
        .cloned();
    let mut membership_events = previous
        .as_ref()
        .and_then(|p| p["membership_events"].as_array())
        .cloned()
        .unwrap_or_default();
    if !prepared.membership.joined.is_empty() || !prepared.membership.left.is_empty() {
        membership_events.push(json!({"document_revision":context.document_revision,"joined":prepared.membership.joined,"left":prepared.membership.left}));
    }
    let row = json!({"procedure_ref":procedure.procedure_ref,"revision":procedure.revision,"source_basis":source_basis,
        "seed":{"algorithm":procedure.seed_algorithm,"version":"1","value":procedure.seed},"definition":procedure,
        "resolved_targets":prepared.membership.addresses.values().collect::<Vec<_>>(),"cursor":context.rule_cursor,"state":context.state,"membership_events":membership_events});
    replace_row(
        rows(&mut retention, "procedures")?,
        "procedure_ref",
        &json!(procedure.procedure_ref),
        row,
    );
    for contribution in &prepared.contributions {
        let own_scene = contribution.owned_addresses.iter().any(|a| {
            a.scene_ref.as_deref() == Some(scene_ref)
                || a.component == "expression" && a.property.is_none()
        });
        let anchor_scene = source_scene_anchor(procedure, contribution)?
            .is_some_and(|a| a.scene_ref.as_ref() == Some(scene_ref));
        if !own_scene && !anchor_scene {
            continue;
        }
        if contribution.generated_basis["scene"]
            .get("procedural")
            .is_some()
        {
            return Err("generated basis cannot recursively retain procedural metadata".into());
        }
        let old = rows(&mut retention, "contributions")?
            .iter()
            .find(|c| c["contribution_ref"] == contribution.contribution_ref)
            .cloned();
        let authored = old
            .as_ref()
            .and_then(|c| c["authored_overrides"].as_array())
            .cloned()
            .unwrap_or_default();
        let row = json!({"contribution_ref":contribution.contribution_ref,"procedure_ref":contribution.procedure_ref,"output_slot":contribution.output_slot,
            "subject_refs":contribution.subjects,"occurrence_ref":contribution.occurrence_ref,"recipe_revision":contribution.recipe.revision,
            "owned_addresses":contribution.owned_addresses,"generated_basis":contribution.generated_basis,"authored_overrides":authored,
            "status":context.lifecycles.iter().find(|l|l.contribution_ref==contribution.contribution_ref).map(|l|match l.status{ContributionLifecycle::Retired=>"retired",ContributionLifecycle::Detached=>"detached"}).unwrap_or("active")});
        replace_row(
            rows(&mut retention, "contributions")?,
            "contribution_ref",
            &json!(contribution.contribution_ref),
            row,
        );
    }
    for s in source_basis.as_array().unwrap() {
        if !rows(&mut retention, "source_basis")?.contains(s) {
            rows(&mut retention, "source_basis")?.push(s.clone());
        }
    }
    validate_existing(&retention)?;
    Ok(retention)
}

/// Canonical source-owned Scene regeneration/lifecycle compiler. It preserves
/// complete active output identities, and distinguishes a real native removal
/// from metadata detachment of exact current authored material. Qualification
/// follows this final materialization in the existing native C-prime owner.
#[allow(clippy::too_many_arguments)]
pub fn prepare_scene_regeneration(
    registry: &crate::m_tree::MRegistry,
    procedure: &crate::procedural_composition::Procedure,
    operation_ref: &str,
    expression_ref: &str,
    document_revision: u64,
    membership: crate::procedural_composition::ResolvedMembership,
    previous: &[crate::procedural_composition::GeneratedContribution],
    current: &[crate::procedural_composition::CurrentContribution],
    next: &[crate::procedural_composition::GeneratedContribution],
    output_readings: Vec<crate::procedural_composition::RetainedOutputReading>,
    required_consumers: BTreeSet<String>,
    context: &NativeMaterialization,
) -> Result<(
    crate::procedural_composition::Regeneration,
    Option<PreparedProcedure>,
)> {
    use crate::procedural_composition::{
        NativeChange, compile_native_regeneration_batch, regenerate, regeneration_native_changes,
    };
    if !context.lifecycles.is_empty() {
        return Err(
            "lifecycle rows are computed by the native generation diff, not caller material input"
                .into(),
        );
    }
    let delta = regenerate(previous, current, next, procedure.removal_policy)?;
    let changes = regeneration_native_changes(&delta, previous, current)?;
    let mut entity_scenes = BTreeMap::new();
    for contribution in previous.iter().chain(next) {
        if let Some(entities) = contribution.generated_basis["scene"]["entities"].as_array() {
            for entity in entities {
                let id = entity["id"]
                    .as_str()
                    .ok_or("generated constituent lost its exact native identity")?;
                entity_scenes.insert(id.to_owned(), contribution.occurrence_ref.clone());
            }
        }
    }
    let mut by_scene: BTreeMap<String, Vec<NativeChange>> = BTreeMap::new();
    for change in changes {
        let scene = match &change {
            NativeChange::SceneCreate { scene_ref, .. }
            | NativeChange::SceneRemove { scene_ref }
            | NativeChange::SceneCompose { scene_ref, .. }
            | NativeChange::SceneMaterialSet { scene_ref, .. }
            | NativeChange::EntityAdd { scene_ref, .. }
            | NativeChange::Focus { scene_ref, .. }
            | NativeChange::RelationFocus { scene_ref, .. } => scene_ref.clone(),
            NativeChange::EntityRemove { entity_ref }
            | NativeChange::SubjectBind { entity_ref, .. }
            | NativeChange::ParameterSet { entity_ref, .. } => entity_scenes
                .get(entity_ref)
                .cloned()
                .ok_or("native regeneration target lacks its retained containing Scene")?,
            NativeChange::SceneReorder { .. } => {
                return Err("Scene generation cannot change whole Expression flow".into());
            }
        };
        by_scene.entry(scene).or_default().push(change);
    }
    if by_scene.is_empty() && delta.detached.is_empty() {
        return Ok((delta, None));
    }
    let mut compiled = delta.contributions.clone();
    for contribution in &mut compiled {
        contribution.native_changes = by_scene
            .remove(&contribution.occurrence_ref)
            .unwrap_or_default();
    }
    let mut materialization = context.clone();
    for (refs, status) in [
        (&delta.retired, ContributionLifecycle::Retired),
        (&delta.detached, ContributionLifecycle::Detached),
    ] {
        for reference in refs {
            let old = previous
                .iter()
                .find(|c| &c.contribution_ref == reference)
                .ok_or("lifecycle original contribution absent")?;
            let mut retained = old.clone();
            retained.recipe = procedure.recipe.clone();
            retained.procedure_revision = procedure.revision.clone();
            retained.native_changes = if status == ContributionLifecycle::Retired {
                by_scene
                    .remove(&retained.occurrence_ref)
                    .ok_or("native retirement lacks actual removal operation")?
            } else {
                let actual = current
                    .iter()
                    .find(|c| &c.contribution_ref == reference)
                    .ok_or("detachment has no actual current authored material")?;
                let presentation =
                    crate::procedural_composition::procedural_intent_projection(&actual.material)?;
                vec![NativeChange::SceneMaterialSet {
                    scene_ref: retained.occurrence_ref.clone(),
                    presentation,
                }]
            };
            compiled.push(retained);
            materialization
                .lifecycles
                .push(NativeContributionLifecycle {
                    contribution_ref: reference.clone(),
                    status,
                });
        }
    }
    if !by_scene.is_empty() {
        return Err("native changes have no stable generation contribution".into());
    }
    let owned_scenes = compiled
        .iter()
        .flat_map(|c| &c.owned_addresses)
        .filter_map(|a| a.scene_ref.as_ref())
        .cloned()
        .collect::<BTreeSet<_>>();
    materialization.scenes.retain(|s| {
        owned_scenes.contains(&s.scene_ref)
            || compiled.iter().any(|c| {
                source_scene_anchor(procedure, c)
                    .ok()
                    .flatten()
                    .is_some_and(|a| a.scene_ref.as_ref() == Some(&s.scene_ref))
            })
    });
    let mut prepared = compile_native_regeneration_batch(
        registry,
        procedure,
        operation_ref,
        expression_ref,
        document_revision,
        membership,
        compiled,
        required_consumers,
        previous,
        current,
        output_readings,
    )?;
    materialize_retention(&mut prepared, &materialization)?;
    Ok((delta, Some(prepared)))
}

/// Retire source-owned material or detach actual authored interventions through
/// existing native operations. The original retained output capability and
/// selector remain separate. Baselines are genuine first-generation native
/// readbacks preserved inside generated_basis.authored_basis, never scene-factor
/// conversions or a fresh caller reset value.
#[allow(clippy::too_many_arguments)]
pub fn prepare_native_retirement(
    registry: &crate::m_tree::MRegistry,
    procedure: &crate::procedural_composition::Procedure,
    operation_ref: &str,
    expression_ref: &str,
    document_revision: u64,
    membership: crate::procedural_composition::ResolvedMembership,
    previous: &[crate::procedural_composition::GeneratedContribution],
    current: &[crate::procedural_composition::CurrentContribution],
    output_readings: Vec<crate::procedural_composition::RetainedOutputReading>,
    required_consumers: BTreeSet<String>,
    context: &NativeMaterialization,
) -> Result<PreparedProcedure> {
    prepare_native_output_regeneration(
        registry,
        procedure,
        operation_ref,
        expression_ref,
        document_revision,
        membership,
        previous,
        current,
        &[],
        output_readings,
        required_consumers,
        context,
    )
}
/// Sustained output membership compiles leaving retirement together with current
/// selected output operations. Exact old native readings remain separate from
/// original selector; no caller resets, target expansion or second state store.
#[allow(clippy::too_many_arguments)]
pub fn prepare_native_output_regeneration(
    registry: &crate::m_tree::MRegistry,
    procedure: &crate::procedural_composition::Procedure,
    operation_ref: &str,
    expression_ref: &str,
    document_revision: u64,
    membership: crate::procedural_composition::ResolvedMembership,
    previous: &[crate::procedural_composition::GeneratedContribution],
    current: &[crate::procedural_composition::CurrentContribution],
    next: &[crate::procedural_composition::GeneratedContribution],
    output_readings: Vec<crate::procedural_composition::RetainedOutputReading>,
    required_consumers: BTreeSet<String>,
    context: &NativeMaterialization,
) -> Result<PreparedProcedure> {
    use crate::procedural_composition::*;
    if previous.is_empty() || !context.lifecycles.is_empty() {
        return Err(
            "native retirement requires current retained outputs, not caller lifecycle labels"
                .into(),
        );
    }
    if previous
        .iter()
        .all(|c| c.generated_basis["schema"] == "oi.journey-scene/v1")
    {
        return prepare_scene_regeneration(
            registry,
            procedure,
            operation_ref,
            expression_ref,
            document_revision,
            membership,
            previous,
            current,
            next,
            output_readings,
            required_consumers,
            context,
        )?
        .1
        .ok_or_else(|| "native retirement produced no actual lifecycle change".into());
    }
    let mut compiled = next.to_vec();
    let mut materialization = context.clone();
    if next.is_empty() {
        materialization.state = "retired".into();
    }
    for old in previous.iter().filter(|old| {
        !next
            .iter()
            .any(|c| c.contribution_ref == old.contribution_ref)
    }) {
        let actual = current
            .iter()
            .find(|c| c.contribution_ref == old.contribution_ref)
            .ok_or("native retirement has no actual current output state")?;
        let baseline = old.generated_basis.get("authored_basis").ok_or(
            "legacy output lacks original authored native basis; reset cannot be invented",
        )?;
        let mut contribution = old.clone();
        contribution.recipe = procedure.recipe.clone();
        contribution.procedure_revision = procedure.revision.clone();
        let persistent = actual.overlays.iter().any(|o| o.persistent)
            || context
                .scenes
                .iter()
                .filter_map(|s| s.existing_retention.as_ref())
                .filter_map(|r| r["contributions"].as_array())
                .flatten()
                .filter(|c| c["contribution_ref"] == old.contribution_ref)
                .filter_map(|c| c["authored_overrides"].as_array())
                .flatten()
                .any(|r| r["persistent"] != false);
        let (edited, reset) = if let Some(parameter) = old.generated_basis["parameter"].as_str() {
            if baseline["schema"] != "ql.native-parameter-state/v1"
                || baseline["parameter"] != parameter
                || actual.material["schema"] != "ql.native-parameter-state/v1"
                || actual.material["parameter"] != parameter
                || baseline["address"] != actual.material["address"]
            {
                return Err(
                    "native parameter retirement differs from original/current owned native basis"
                        .into(),
                );
            }
            (
                persistent || actual.material["value"] != old.generated_basis["value"],
                vec![NativeChange::ParameterSet {
                    entity_ref: old
                        .owned_addresses
                        .first()
                        .and_then(|a| a.entity_ref.clone())
                        .ok_or("native force retirement lacks entity")?,
                    parameter: parameter.into(),
                    value: baseline["value"].clone(),
                }],
            )
        } else if let Some(flow) = old.generated_basis["native_flow"].as_array() {
            if baseline["schema"] != "ql.native-atlas-state/v1"
                || baseline["expression_ref"] != expression_ref
                || actual.material["schema"] != "ql.native-atlas-state/v1"
                || actual.material["expression_ref"] != expression_ref
            {
                return Err(
                    "Atlas retirement lacks exact original/current continuing world basis".into(),
                );
            }
            let mut edited = persistent;
            let mut reset = Vec::new();
            let mut missing_focus_reset = false;
            for wire in flow {
                let change: NativeChange =
                    serde_json::from_value(wire.clone()).map_err(|e| e.to_string())?;
                match change {
                    NativeChange::Focus {
                        scene_ref,
                        entity_ref,
                    } => {
                        edited |= actual.material["focus"]["scene_ref"] != scene_ref
                            || actual.material["focus"]["entity_ref"] != json!(entity_ref)
                            || !actual.material["focus"]["relation_ref"].is_null();
                        let focus = &baseline["focus"];
                        let Some(scene_ref) = focus["scene_ref"].as_str() else {
                            // An edited flow detaches and needs no reset. Keep
                            // the native missing-clear capability explicit for
                            // unedited retirement instead of inventing a Scene.
                            missing_focus_reset = true;
                            continue;
                        };
                        let scene_ref = scene_ref.to_owned();
                        reset.push(if let Some(relation) = focus["relation_ref"].as_str() {
                            NativeChange::RelationFocus {
                                scene_ref,
                                binding_ref: relation.into(),
                            }
                        } else {
                            NativeChange::Focus {
                                scene_ref,
                                entity_ref: focus["entity_ref"].as_str().map(str::to_owned),
                            }
                        });
                    }
                    NativeChange::RelationFocus {
                        scene_ref,
                        binding_ref,
                    } => {
                        edited |= actual.material["focus"]["scene_ref"] != scene_ref
                            || actual.material["focus"]["relation_ref"] != binding_ref
                            || !actual.material["focus"]["entity_ref"].is_null();
                        let focus = &baseline["focus"];
                        let Some(scene_ref) = focus["scene_ref"].as_str() else {
                            // An edited flow detaches and needs no reset. Keep
                            // the native missing-clear capability explicit for
                            // unedited retirement instead of inventing a Scene.
                            missing_focus_reset = true;
                            continue;
                        };
                        let scene_ref = scene_ref.to_owned();
                        reset.push(if let Some(relation) = focus["relation_ref"].as_str() {
                            NativeChange::RelationFocus {
                                scene_ref,
                                binding_ref: relation.into(),
                            }
                        } else {
                            NativeChange::Focus {
                                scene_ref,
                                entity_ref: focus["entity_ref"].as_str().map(str::to_owned),
                            }
                        });
                    }
                    NativeChange::SceneReorder { scene_refs } => {
                        edited |= actual.material["scene_order"] != json!(scene_refs);
                        reset.push(NativeChange::SceneReorder {
                            scene_refs: serde_json::from_value(baseline["scene_order"].clone())
                                .map_err(|e| e.to_string())?,
                        });
                    }
                    _ => return Err("Atlas retirement contains non-flow generated material".into()),
                }
            }
            if !edited && missing_focus_reset {
                return Err("unedited Atlas retirement requires the native owner's clear-selection operation for original empty Selection".into());
            }
            (edited, reset)
        } else {
            return Err(
                "mixed/unknown native output retirement requires exact owning native operation"
                    .into(),
            );
        };
        if edited && procedure.removal_policy == RemovalPolicy::ConflictOnEdited {
            return Err("edited native contribution requires explicit removal decision".into());
        }
        contribution.native_changes = if edited { vec![] } else { reset };
        materialization
            .lifecycles
            .push(NativeContributionLifecycle {
                contribution_ref: old.contribution_ref.clone(),
                status: if edited {
                    ContributionLifecycle::Detached
                } else {
                    ContributionLifecycle::Retired
                },
            });
        compiled.push(contribution);
    }
    let metadata_only = compiled.iter().all(|c| c.native_changes.is_empty());
    let mut prepared = if metadata_only {
        compile_retained_metadata_phase(
            registry,
            procedure,
            operation_ref,
            expression_ref,
            document_revision,
            membership,
            compiled,
            required_consumers,
            previous,
            current,
            output_readings,
        )?
    } else {
        compile_native_regeneration_batch(
            registry,
            procedure,
            operation_ref,
            expression_ref,
            document_revision,
            membership,
            compiled,
            required_consumers,
            previous,
            current,
            output_readings,
        )?
    };
    materialize_retention(&mut prepared, &materialization)?;
    Ok(prepared)
}

/// Exact stored native row; no compiler-only changes/recipe fields are invented.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetainedContributionDescriptor {
    pub contribution_ref: String,
    pub procedure_ref: String,
    pub output_slot: String,
    pub subject_refs: Vec<String>,
    pub occurrence_ref: String,
    pub recipe_revision: String,
    pub owned_addresses: Vec<OwnedAddress>,
    pub generated_basis: Value,
    pub status: String,
    pub authored_overrides: Vec<crate::procedural_intervention::NativeAuthoredIntervention>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneDeletionAnchor {
    pub procedure: crate::procedural_composition::Procedure,
    pub contribution: RetainedContributionDescriptor,
    pub anchor: NativeRetentionScene,
    pub edit: crate::procedural_intervention::ManualEdit,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneDeletionAnchorEdit {
    pub schema: String,
    pub metadata_scope: Vec<OwnedAddress>,
    pub changes: Vec<crate::procedural_composition::NativeChange>,
    pub attribution: crate::procedural_intervention::ManualInterventions,
}
/// Pure protected ordinary-edit preflight. Root verifies actual before/candidate
/// and CAS then atomically commits metadata BEFORE removing the generated Scene.
pub fn prepare_scene_deletion_anchor(
    input: &SceneDeletionAnchor,
) -> Result<SceneDeletionAnchorEdit> {
    let expected = source_scene_anchor_identity(
        &input.procedure,
        &input.contribution.procedure_ref,
        &input.contribution.output_slot,
        &input.contribution.occurrence_ref,
    )?
    .ok_or("Scene deletion lacks original native constructor anchor")?;
    let scene = expected.scene_ref.as_ref().unwrap().clone();
    if input.anchor.scene_ref != scene
        || input.anchor.document_revision.checked_add(1) != Some(input.edit.document_revision)
        || input.edit.contribution_ref != input.contribution.contribution_ref
        || input.edit.scene_ref != input.contribution.occurrence_ref
        || input.edit.expression_ref != input.procedure.occurrence_ref
        || input.edit.owned_addresses != input.contribution.owned_addresses
        || input.edit.retained_native_records != input.contribution.authored_overrides
        || !input.edit.after.is_null()
    {
        return Err(
            "Scene deletion does not match actual original output/source anchor/CAS".into(),
        );
    }
    let attribution = crate::procedural_intervention::extract_manual_interventions(&input.edit)?;
    let mut retention = input
        .anchor
        .existing_retention
        .clone()
        .ok_or("Scene contribution was not retained on its actual canonical source anchor")?;
    validate_existing(&retention)?;
    let definition = retention["procedures"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["procedure_ref"] == input.procedure.procedure_ref)
        .ok_or("source anchor lacks original procedure")?;
    if definition["definition"] != json!(input.procedure) {
        return Err("source anchor procedure differs from original sealed constructor".into());
    }
    let contribution = rows(&mut retention, "contributions")?
        .iter_mut()
        .find(|c| c["contribution_ref"] == input.contribution.contribution_ref)
        .ok_or("source anchor lacks original stable contribution")?;
    let retained: RetainedContributionDescriptor =
        serde_json::from_value(contribution.clone()).map_err(|e| e.to_string())?;
    if retained != input.contribution
        || input.contribution.procedure_ref != input.procedure.procedure_ref
        || crate::procedural_composition::contribution_identity(
            &input.procedure.procedure_ref,
            &input.contribution.output_slot,
            &input.contribution.subject_refs,
            &input.contribution.occurrence_ref,
        )? != input.contribution.contribution_ref
    {
        return Err("deletion anchor changed original contribution identity/basis".into());
    }
    contribution["status"] = json!("detached");
    contribution["authored_overrides"] = json!(attribution.native_records);
    let mut presentation = input
        .anchor
        .current_presentation
        .clone()
        .ok_or("canonical source anchor material unavailable")?;
    if presentation["schema"] != "oi.journey-scene/v1"
        || presentation["scene"]["id"] != scene
        || presentation["scene"].get("procedural").is_some()
    {
        return Err(
            "deletion anchor material is not actual current nontransitive source Scene".into(),
        );
    }
    presentation["scene"]["procedural"] = retention;
    Ok(SceneDeletionAnchorEdit {
        schema: "ql.procedural-scene-deletion/v1".into(),
        metadata_scope: vec![expected],
        changes: vec![
            crate::procedural_composition::NativeChange::SceneMaterialSet {
                scene_ref: scene.clone(),
                presentation,
            },
            crate::procedural_composition::NativeChange::SceneRemove {
                scene_ref: input.contribution.occurrence_ref.clone(),
            },
        ],
        attribution,
    })
}
/// Explicit accepted release updates retained attribution on SAME source Scene.
/// The next source event may recreate its same stable output identity; release
/// itself never fabricates a current missing-Scene output capability or ACK.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneDeletionRelease {
    pub procedure: crate::procedural_composition::Procedure,
    pub contribution: RetainedContributionDescriptor,
    pub anchor: NativeRetentionScene,
    pub actor_ref: String,
    pub operation_ref: String,
    pub document_revision: u64,
}
pub fn release_scene_deletion_anchor(
    input: &SceneDeletionRelease,
) -> Result<SceneDeletionAnchorEdit> {
    use crate::procedural_intervention::{
        InterventionKind, ManualInterventions, NativeIntervention,
    };
    nonempty(&input.actor_ref, "actual explicit release actor")?;
    nonempty(&input.operation_ref, "actual explicit release operation")?;
    let expected = source_scene_anchor_identity(
        &input.procedure,
        &input.contribution.procedure_ref,
        &input.contribution.output_slot,
        &input.contribution.occurrence_ref,
    )?
    .ok_or("release lacks original source anchor")?;
    let scene = expected.scene_ref.as_ref().unwrap().clone();
    if input.anchor.scene_ref != scene
        || input.anchor.document_revision.checked_add(1) != Some(input.document_revision)
    {
        return Err("release differs from actual source Scene/current CAS".into());
    }
    let mut retention = input
        .anchor
        .existing_retention
        .clone()
        .ok_or("release source anchor unavailable")?;
    validate_existing(&retention)?;
    let definition = retention["procedures"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["procedure_ref"] == input.procedure.procedure_ref)
        .ok_or("release lacks original procedure")?;
    if definition["definition"] != json!(input.procedure) {
        return Err("release changed original sealed definition".into());
    }
    let row = rows(&mut retention, "contributions")?
        .iter_mut()
        .find(|c| c["contribution_ref"] == input.contribution.contribution_ref)
        .ok_or("release retained contribution unavailable")?;
    let actual: RetainedContributionDescriptor =
        serde_json::from_value(row.clone()).map_err(|e| e.to_string())?;
    if actual != input.contribution
        || actual.status != "detached"
        || crate::procedural_composition::contribution_identity(
            &input.procedure.procedure_ref,
            &actual.output_slot,
            &actual.subject_refs,
            &actual.occurrence_ref,
        )? != actual.contribution_ref
    {
        return Err("release changed original actual retained identity/status".into());
    }
    let whole = OwnedAddress {
        expression_ref: input.procedure.occurrence_ref.clone(),
        scene_ref: Some(actual.occurrence_ref.clone()),
        entity_ref: None,
        component: "scene".into(),
        constituent_ref: None,
        parent_ref: None,
        property: None,
    };
    let mut native_records = actual.authored_overrides.clone();
    let mut interventions = Vec::new();
    for record in &mut native_records {
        if record.address != whole
            || record.path != "/scene"
            || record.kind != InterventionKind::Delete
            || !record.persistent
        {
            continue;
        }
        record.persistent = false;
        record.actor = input.actor_ref.clone();
        record.operation_ref = Some(input.operation_ref.clone());
        record.revision = input.document_revision;
        interventions.push(NativeIntervention {
            contribution_ref: actual.contribution_ref.clone(),
            address: whole.clone(),
            pointer: "/scene".into(),
            kind: InterventionKind::Delete,
            before: record.before.clone(),
            value: None,
            actor_ref: input.actor_ref.clone(),
            operation_ref: input.operation_ref.clone(),
            document_revision: input.document_revision,
            persistent: false,
        });
    }
    if interventions.is_empty() {
        return Err("explicit release has no actual persistent whole Scene tombstone".into());
    }
    row["authored_overrides"] = json!(native_records);
    let mut presentation = input
        .anchor
        .current_presentation
        .clone()
        .ok_or("release canonical source material unavailable")?;
    if presentation["schema"] != "oi.journey-scene/v1"
        || presentation["scene"]["id"] != scene
        || presentation["scene"].get("procedural").is_some()
    {
        return Err("release source material differs from actual current source Scene".into());
    }
    presentation["scene"]["procedural"] = retention;
    Ok(SceneDeletionAnchorEdit {
        schema: "ql.procedural-scene-deletion/v1".into(),
        metadata_scope: vec![expected],
        changes: vec![
            crate::procedural_composition::NativeChange::SceneMaterialSet {
                scene_ref: scene,
                presentation,
            },
        ],
        attribution: ManualInterventions {
            schema: "ql.procedural-manual-interventions/v1".into(),
            interventions,
            native_records,
            overlays: vec![],
        },
    })
}
