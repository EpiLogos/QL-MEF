//! Prepared rule conduct inside the existing long-lived native FieldHost.
//! Source compilation occurs at explicit install/restore, never a render/audio
//! callback. The same native receiver commits material and attests actual ACKs.
use crate::m_tree::{MRegistry, native_current_m_registry};
use crate::procedural_composition::*;
use crate::procedural_effective::{
    apply_retained_flow_interventions, apply_retained_force_interventions,
};
use crate::procedural_intervention::{ManualEdit, extract_manual_interventions};
use crate::procedural_manifestation::{
    NativeSubject, SourceBasis, fingerprint, nonempty, validate_material,
    validate_native_subject_basis,
};
use crate::procedural_retention::{
    MATERIALIZATION_CONTRACT, NativeMaterialization, NativeRetentionScene, materialize_retention,
};
use crate::procedural_timing::NativeTimingWitness;
use crate::vak_composition::VakComposition;
use crate::vak_profile::ThreadPlan;
use crate::vak_scope_wire::OperativeScopeCurrentnessRequest;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

#[path = "procedural_lifecycle.rs"]
pub mod lifecycle;

pub const CONDUCT_CONTRACT: &str = "ql.procedural-conduct/v1";
pub const CONDUCT_RECEIPT: &str = "ql.procedural-conduct-receipt/v1";
pub const LIBRARY_CONTRACT: &str = "ql.procedural-library/v1";
const MAX_RULES: usize = 64;
const MAX_EVENTS: usize = 256;
const MAX_CHECKPOINT_BYTES: usize = 8 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativePosition {
    pub instance_ref: String,
    pub event_ref: String,
    pub subject_ref: String,
    pub generation: String,
    pub samples_elapsed: String,
}
impl NativePosition {
    pub fn from_field(instance_ref: &str, field: &Value) -> Result<Self> {
        let text = |key: &str| {
            field[key]
                .as_str()
                .ok_or_else(|| format!("native field has no {key}"))
        };
        let value = Self {
            instance_ref: instance_ref.into(),
            event_ref: text("event_ref")?.into(),
            subject_ref: text("subject_ref")?.into(),
            generation: text("generation")?.into(),
            samples_elapsed: text("samples_elapsed")?.into(),
        };
        value.validate()?;
        Ok(value)
    }
    pub fn cursor(&self) -> Result<u64> {
        exact_cursor(&self.samples_elapsed)
    }
    fn validate(&self) -> Result<()> {
        for (value, name) in [
            (&self.instance_ref, "native instance"),
            (&self.event_ref, "native event"),
            (&self.subject_ref, "native subject"),
        ] {
            nonempty(value, name)?;
        }
        exact_cursor(&self.generation)?;
        self.cursor()?;
        Ok(())
    }
}
fn exact_cursor(value: &str) -> Result<u64> {
    let parsed: u64 = value.parse().map_err(|_| "invalid native cursor")?;
    if parsed.to_string() != value {
        return Err("noncanonical native cursor".into());
    }
    Ok(parsed)
}

/// Actual ordinary material read through the same Expression owner. The native
/// receiving owner re-attests its document revision/hash; this transport is not
/// a source permission or an application acknowledgement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeSceneSource {
    pub native_owner: String,
    pub expression_ref: String,
    pub scene_ref: String,
    pub document_revision: u64,
    pub source_basis: SourceBasis,
    pub material_fingerprint: String,
    pub presentation: Value,
    pub principal: NativeSubject,
    pub contributors: Vec<NativeSubject>,
    pub locus_ref: String,
    pub locus_revision: String,
}
impl NativeSceneSource {
    pub fn validate(&self, registry: &MRegistry) -> Result<()> {
        if self.native_owner != "oi.expression"
            || self.presentation["schema"] != "oi.journey-scene/v1"
            || self.presentation["scene"]["id"] != self.scene_ref
            || !self
                .scene_ref
                .starts_with(&format!("{}:scene:", self.expression_ref))
            || self.document_revision == 0
            || self.presentation["scene"].get("procedural").is_some()
            || fingerprint(&self.presentation)? != self.material_fingerprint
        {
            return Err("library source is not the exact native Scene/document material".into());
        }
        self.source_basis.validate()?;
        nonempty(&self.locus_ref, "native locus")?;
        nonempty(&self.locus_revision, "native locus revision")?;
        validate_native_subject_basis(registry, &self.principal)?;
        if self.contributors.len() > 64 {
            return Err("native contributor bound exceeded".into());
        }
        for subject in &self.contributors {
            validate_native_subject_basis(registry, subject)?;
        }
        validate_material(&self.presentation, 0)
    }
    fn construction_basis(&self) -> Result<Value> {
        let mut basis = self.presentation.clone();
        let scene = basis["scene"]
            .as_object_mut()
            .ok_or("native Scene material is missing")?;
        // A NEW occurrence does not carry the old occurrence's driver targets,
        // output permissions or journal. The full original remains the source.
        scene.remove("procedural");
        Ok(basis)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "value_source", rename_all = "snake_case", deny_unknown_fields)]
pub enum RecipeValue {
    Constant { value: Value },
    NativeProperty { property: String },
}
impl RecipeValue {
    fn resolve(&self, target: &TargetReading) -> Result<Value> {
        let value = match self {
            Self::Constant { value } => value.clone(),
            Self::NativeProperty { property } => {
                nonempty(property, "native recipe input property")?;
                target
                    .properties
                    .get(property)
                    .cloned()
                    .ok_or("native recipe input is unavailable")?
            }
        };
        validate_material(&value, 0)?;
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParameterRecipe {
    pub output_slot: String,
    pub parameter: String,
    pub value: RecipeValue,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneRecipe {
    pub output_slot: String,
    pub scene_ref: String,
    pub source: NativeSceneSource,
    pub sequence_holds: Vec<SequenceHold>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "recipe", rename_all = "snake_case", deny_unknown_fields)]
pub enum NativeRecipeProgram {
    SceneMaterial { outputs: Vec<SceneRecipe> },
    ForceParameters { writes: Vec<ParameterRecipe> },
    AtlasPassage { changes: Vec<NativeChange> },
}
impl NativeRecipeProgram {
    pub fn validate(
        &self,
        registry: &MRegistry,
        procedure: &Procedure,
        expression_ref: &str,
    ) -> Result<()> {
        let wire = serde_json::to_value(self).map_err(|e| e.to_string())?;
        validate_material(&wire, 0)?;
        if procedure.recipe_parameters.get("native_program") != Some(&wire) {
            return Err(
                "native program differs from the exact authored procedure definition".into(),
            );
        }
        match self {
            Self::SceneMaterial { outputs } => {
                if outputs.is_empty() || outputs.len() > procedure.budgets.max_active_instances {
                    return Err("native scene output bound exceeded".into());
                }
                let mut slots = BTreeSet::new();
                let mut scenes = BTreeSet::new();
                for output in outputs {
                    output.source.validate(registry)?;
                    let mut projected = output.source.construction_basis()?;
                    apply_sequence_holds(&mut projected, &output.sequence_holds)?;
                    nonempty(&output.output_slot, "native output slot")?;
                    if output.source.expression_ref != expression_ref
                        || output.source.principal.subject_ref != procedure.principal_subject_ref
                        || output.source.source_basis != procedure.profile
                        || output.source.locus_ref != procedure.locus_ref
                        || !output
                            .scene_ref
                            .starts_with(&format!("{expression_ref}:scene:"))
                        || !slots.insert(&output.output_slot)
                        || !scenes.insert(&output.scene_ref)
                    {
                        return Err("native scene output is foreign or duplicate".into());
                    }
                }
            }
            Self::ForceParameters { writes } => {
                if writes.is_empty() || writes.len() > 64 {
                    return Err("native parameter recipe bound exceeded".into());
                }
                let mut slots = BTreeSet::new();
                for write in writes {
                    nonempty(&write.output_slot, "native output slot")?;
                    if !slots.insert(&write.output_slot)
                        || !["force_strength", "force_spin", "force_radius"]
                            .contains(&write.parameter.as_str())
                    {
                        return Err("unsupported or duplicate native force parameter recipe".into());
                    }
                    if let RecipeValue::Constant { value } = &write.value {
                        force_parameter(&write.parameter, value)?;
                    }
                    if let RecipeValue::NativeProperty { property } = &write.value {
                        nonempty(property, "native source property")?;
                    }
                }
            }
            Self::AtlasPassage { changes } => {
                if changes.is_empty() || changes.len() > 64 {
                    return Err("native Atlas flow bound exceeded".into());
                }
                for change in changes {
                    change.validate()?;
                    if !matches!(
                        change,
                        NativeChange::Focus { .. }
                            | NativeChange::RelationFocus { .. }
                            | NativeChange::SceneReorder { .. }
                    ) {
                        return Err("Atlas procedure may only use the existing native flow/focus operations".into());
                    }
                }
            }
        }
        Ok(())
    }
}
fn force_parameter(parameter: &str, value: &Value) -> Result<()> {
    let scalar = value
        .as_f64()
        .ok_or("native force parameter requires a scalar")?;
    let (minimum, maximum) = if parameter == "force_radius" {
        (1.0, 1600.0)
    } else {
        (-20.0, 20.0)
    };
    if !scalar.is_finite() || scalar < minimum || scalar > maximum {
        return Err("native force parameter exceeds existing Expression parameter domain".into());
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConductInstall {
    pub schema: String,
    pub procedure: Procedure,
    pub program: NativeRecipeProgram,
    pub expression_ref: String,
    pub document_revision: u64,
    pub current_readings: Vec<TargetReading>,
    pub source_composition: Value,
    pub currentness: OperativeScopeCurrentnessRequest,
    pub thread_plan: ThreadPlan,
    pub required_consumers: BTreeSet<String>,
    pub interval_ref: String,
    pub materialization: Option<NativeMaterialization>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConductEvent {
    pub procedure_ref: String,
    pub event: RuleEvent,
    pub position: NativePosition,
    pub document_revision: u64,
    pub current_readings: Vec<TargetReading>,
    pub output_readings: Vec<RetainedOutputReading>,
    pub current_contributions: Vec<CurrentContribution>,
    /// Actual protected native stored rows and current Scene/flow basis; the
    /// same event projects them without another host ordinal or invented actor.
    #[serde(default)]
    pub intervention_contexts: Vec<crate::procedural_intervention::NativeInterventionBasis>,
    pub materialization: Option<NativeMaterialization>,
    pub currentness: OperativeScopeCurrentnessRequest,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub enum ConductRequest {
    SourceBootstrap {
        input: Box<crate::procedural_source::NativeSourceBootstrap>,
    },
    Lifecycle {
        input: Box<lifecycle::NativeLifecycleInput>,
    },
    LifecycleCancel {
        input: Box<lifecycle::NativeLifecycleCancel>,
    },
    LibraryDiscover {},
    LibraryBuild {
        input: Box<LibraryBuild>,
    },
    ControlPreflight {
        input: Box<crate::procedural_control::NativeControlInput>,
    },
    Interventions {
        edit: Box<ManualEdit>,
    },
    InterventionsOwned {
        edit: Box<ManualEdit>,
    },
    InterventionBatch {
        entries: Vec<crate::procedural_intervention::InterventionPreflight>,
    },
    FlowInterventions {
        edit: Box<crate::procedural_intervention::FlowManualEdit>,
    },
    ProjectFlowInterventions {
        edit: Box<crate::procedural_intervention::FlowManualEdit>,
    },
    #[serde(rename = "scene_delete_anchor")]
    SceneDeletionAnchor {
        input: Box<crate::procedural_retention::SceneDeletionAnchor>,
    },
    #[serde(rename = "scene_delete_release")]
    ReleaseSceneDeletion {
        input: Box<crate::procedural_retention::SceneDeletionRelease>,
    },
    MaterialReadback {
        input: Box<MaterialReadback>,
    },
    ProjectInterventions {
        edit: Box<ManualEdit>,
    },
    Retire {
        input: Box<ConductEvent>,
    },
    Replace {
        expected_definition_fingerprint: String,
        definition: Box<ConductInstall>,
    },
    Install {
        definition: Box<ConductInstall>,
    },
    Event {
        input: Box<ConductEvent>,
    },
    Read {
        procedure_ref: String,
    },
    Pause {
        procedure_ref: String,
    },
    Resume {
        procedure_ref: String,
        currentness: OperativeScopeCurrentnessRequest,
    },
    Seek {
        procedure_ref: String,
    },
    Cancel {
        procedure_ref: String,
    },
    Checkpoint {
        procedure_ref: String,
    },
    Restore {
        checkpoint: Box<ConductCheckpoint>,
    },
    BeginInterval {
        procedure_ref: String,
        expected_interval_ref: String,
        currentness: OperativeScopeCurrentnessRequest,
    },
}
/// The SAME protected Expression receiver supplies this actual journal row.
/// Transport itself grants no receipt authority; Root Manager re-attests it.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaterialReadback {
    pub procedure_ref: String,
    pub operation_ref: String,
    pub document_revision: u64,
    pub native_record: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConductCheckpoint {
    pub schema: String,
    pub definition: ConductInstall,
    pub rule: RuleExecution,
    pub membership: ResolvedMembership,
    pub last_generated: Vec<GeneratedContribution>,
    pub pending_operation_ref: Option<String>,
    pub pending_preparation: Option<PreparedProcedure>,
    pub native_position: NativePosition,
    pub events: BTreeMap<String, RecordedConduct>,
    #[serde(default)]
    pub lifecycle: lifecycle::LifecycleCheckpoint,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordedConduct {
    pub input_fingerprint: String,
    pub receipt: Value,
}
struct InstalledConduct {
    checkpoint: ConductCheckpoint,
    graph: VakComposition,
}
#[derive(Default)]
pub struct ConductHost {
    rules: BTreeMap<String, InstalledConduct>,
}

fn validate_definition(definition: &ConductInstall, position: &NativePosition) -> Result<()> {
    if definition.schema != CONDUCT_CONTRACT {
        return Err("unsupported native conduct contract".into());
    }
    let registry = native_current_m_registry();
    definition.procedure.validate(registry)?;
    definition
        .program
        .validate(registry, &definition.procedure, &definition.expression_ref)?;
    if definition.expression_ref != definition.procedure.occurrence_ref {
        return Err("conduct requires the exact containing Expression".into());
    }
    position.validate()?;
    if definition.document_revision == 0 {
        return Err("native document basis is missing".into());
    }
    bounded(definition)
}
fn bounded(value: &impl Serialize) -> Result<()> {
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?;
    if bytes.len() > MAX_CHECKPOINT_BYTES {
        return Err("native conduct/checkpoint retention bound exceeded".into());
    }
    Ok(())
}
fn receipt(
    checkpoint: &ConductCheckpoint,
    status: &str,
    reason: Option<&str>,
    prepared: Option<&PreparedProcedure>,
    cause: Option<&Cause>,
) -> Result<Value> {
    Ok(
        json!({"schema":CONDUCT_RECEIPT,"status":status,"procedure_ref":checkpoint.definition.procedure.procedure_ref,
        "native_position":checkpoint.native_position,"membership":checkpoint.membership,"rule":checkpoint.rule,
        "prepared":prepared,"cause":cause,"generated_operations":prepared.map(|p|p.native_edit["changes"].as_array().map_or(0,Vec::len)).unwrap_or(0),
        "reason":reason,"standing":"native prepared rule conduct; SAME Expression receiver commits material and attests actual scene/body/audio ingress separately"}),
    )
}
impl ConductHost {
    pub fn execute(
        &mut self,
        request: ConductRequest,
        position: NativePosition,
        timing: Option<&NativeTimingWitness>,
    ) -> Result<Value> {
        position.validate()?;
        let authored = self.request_timing(&request)?;
        let witness = if let Some(binding) = authored.as_ref() {
            let actual = timing
                .ok_or("actual current native owner timing/source/Act witness unavailable")?;
            if actual.original_binding() != binding || actual.native_position() != &position {
                return Err("procedure timing witness differs from exact original binding/current native owner position".into());
            }
            Some(actual)
        } else {
            None
        };
        match request {
            ConductRequest::SourceBootstrap { .. } => Err("native source bootstrap requires the private actual selected-Act/Scene read and held-owner route".into()),
            ConductRequest::Lifecycle { .. } | ConductRequest::LifecycleCancel { .. } => Err("native lifecycle requires private staged Scene/source pre/post receiving route".into()),
            ConductRequest::LibraryDiscover {} => Ok(library_discover()),
            ConductRequest::LibraryBuild { input } => library_build(*input),
            ConductRequest::ControlPreflight { input } => {
                crate::procedural_control::prepare_native_control(&input)
            }
            ConductRequest::InterventionsOwned { edit } => serde_json::to_value(
                crate::procedural_intervention::extract_owned_manual_interventions(&edit)?,
            )
            .map_err(|e| e.to_string()),
            ConductRequest::InterventionBatch { entries } => {
                crate::procedural_intervention::intervention_batch(&entries)
            }
            ConductRequest::FlowInterventions { edit } => serde_json::to_value(
                crate::procedural_intervention::extract_flow_interventions(&edit)?,
            )
            .map_err(|e| e.to_string()),
            ConductRequest::ProjectFlowInterventions { edit } => serde_json::to_value(
                crate::procedural_intervention::project_native_flow_interventions(&edit)?,
            )
            .map_err(|e| e.to_string()),
            ConductRequest::SceneDeletionAnchor { input } => serde_json::to_value(
                crate::procedural_retention::prepare_scene_deletion_anchor(&input)?,
            )
            .map_err(|e| e.to_string()),
            ConductRequest::ReleaseSceneDeletion { input } => serde_json::to_value(
                crate::procedural_retention::release_scene_deletion_anchor(&input)?,
            )
            .map_err(|e| e.to_string()),
            ConductRequest::MaterialReadback { input } => self.material_readback(*input, position),
            ConductRequest::Replace {
                expected_definition_fingerprint,
                definition,
            } => self.replace(
                &expected_definition_fingerprint,
                *definition,
                position,
                witness.ok_or("native timing witness absent")?,
            ),
            ConductRequest::ProjectInterventions { edit } => serde_json::to_value(
                crate::procedural_intervention::project_native_authored_interventions(
                    &edit,
                    &edit.retained_native_records,
                )?,
            )
            .map_err(|e| e.to_string()),
            ConductRequest::Retire { input } => self.retire(
                *input,
                position,
                witness.ok_or("native timing witness absent")?,
            ),
            ConductRequest::Interventions { edit } => {
                serde_json::to_value(extract_manual_interventions(&edit)?)
                    .map_err(|e| e.to_string())
            }
            ConductRequest::Install { definition } => self.install(
                *definition,
                position,
                witness.ok_or("native timing witness absent")?,
            ),
            ConductRequest::Restore { checkpoint } => self.restore(
                *checkpoint,
                position,
                witness.ok_or("native timing witness absent")?,
            ),
            ConductRequest::Event { input } => self.event(
                *input,
                position,
                witness.ok_or("native timing witness absent")?,
            ),
            request => {
                let reference = match &request {
                    ConductRequest::Read { procedure_ref }
                    | ConductRequest::Pause { procedure_ref }
                    | ConductRequest::Resume { procedure_ref, .. }
                    | ConductRequest::Seek { procedure_ref }
                    | ConductRequest::Cancel { procedure_ref }
                    | ConductRequest::Checkpoint { procedure_ref }
                    | ConductRequest::BeginInterval { procedure_ref, .. } => procedure_ref,
                    _ => unreachable!("install/event/restore dispatched before control"),
                };
                let installed = self
                    .rules
                    .get_mut(reference)
                    .ok_or("unknown native installed procedure")?;
                check_position(&installed.checkpoint, &position)?;
                let mut next = installed.checkpoint.clone();
                next.native_position = position.clone();
                let status = match request {
                    ConductRequest::Read { .. } => "read",
                    ConductRequest::Pause { .. } => {
                        next.rule.pause(
                            witness
                                .ok_or("native timing witness absent")?
                                .admitted_cursor(),
                        );
                        "held"
                    }
                    ConductRequest::Resume { currentness, .. } => {
                        qualify_source(&installed.graph, &next.definition, &currentness)?;
                        next.rule
                            .resume(&next.definition.procedure.timing.epoch_ref)?;
                        "running"
                    }
                    ConductRequest::Seek { .. } => {
                        next.rule.seek(
                            witness
                                .ok_or("native timing witness absent")?
                                .admitted_cursor(),
                            &next.definition.procedure.timing.epoch_ref,
                        )?;
                        "held"
                    }
                    ConductRequest::Cancel { .. } => {
                        next.rule.cancel(
                            witness
                                .ok_or("native timing witness absent")?
                                .admitted_cursor(),
                        );
                        "cancelled"
                    }
                    ConductRequest::BeginInterval {
                        expected_interval_ref,
                        currentness,
                        ..
                    } => {
                        qualify_source(&installed.graph, &next.definition, &currentness)?;
                        let actual = witness.ok_or("native timing witness absent")?;
                        continue_native_interval(
                            &mut next.rule,
                            &next.definition.procedure,
                            &expected_interval_ref,
                            actual.original_binding(),
                            actual.admitted_cursor(),
                        )?;
                        if next.rule.position == RulePosition::Running {
                            "running"
                        } else {
                            "held"
                        }
                    }
                    ConductRequest::Checkpoint { .. } => {
                        next.rule.pause(
                            witness
                                .ok_or("native timing witness absent")?
                                .admitted_cursor(),
                        );
                        bounded(&next)?;
                        let mut result = receipt(&next, "checkpoint", None, None, None)?;
                        result["checkpoint"] =
                            serde_json::to_value(&next).map_err(|e| e.to_string())?;
                        installed.checkpoint = next;
                        return Ok(result);
                    }
                    _ => unreachable!("non-control request already dispatched"),
                };
                bounded(&next)?;
                let result = receipt(&next, status, None, None, None)?;
                installed.checkpoint = next;
                Ok(result)
            }
        }
    }
    fn install(
        &mut self,
        definition: ConductInstall,
        position: NativePosition,
        witness: &NativeTimingWitness,
    ) -> Result<Value> {
        validate_definition(&definition, &position)?;
        if self.rules.contains_key(&definition.procedure.procedure_ref) {
            return Err("native procedure already installed; preserve/checkpoint/cancel its original definition before explicit replacement".into());
        }
        if self.rules.len() >= MAX_RULES {
            return Err("native installed rule bound exceeded".into());
        }
        let (_, graph) =
            crate::vak_composition_wire::compile_request(&definition.source_composition)?;
        qualify_source(&graph, &definition, &definition.currentness)?;
        let membership = resolve_procedure_membership(
            native_current_m_registry(),
            &definition.procedure,
            &definition.expression_ref,
            &definition.current_readings,
            None,
        )?;
        let mut checkpoint = ConductCheckpoint {
            schema: CONDUCT_CONTRACT.into(),
            rule: RuleExecution::new(&definition.procedure, &definition.interval_ref)?,
            membership,
            definition,
            last_generated: vec![],
            pending_operation_ref: None,
            pending_preparation: None,
            native_position: position,
            events: BTreeMap::new(),
            lifecycle: lifecycle::LifecycleCheckpoint::default(),
        };
        checkpoint.rule.cursor = witness.admitted_cursor();
        bounded(&checkpoint)?;
        let result = receipt(&checkpoint, "installed", None, None, None)?;
        self.rules.insert(
            checkpoint.definition.procedure.procedure_ref.clone(),
            InstalledConduct { checkpoint, graph },
        );
        Ok(result)
    }
    fn restore(
        &mut self,
        mut checkpoint: ConductCheckpoint,
        position: NativePosition,
        witness: &NativeTimingWitness,
    ) -> Result<Value> {
        if checkpoint.schema != CONDUCT_CONTRACT
            || self
                .rules
                .contains_key(&checkpoint.definition.procedure.procedure_ref)
            || self.rules.len() >= MAX_RULES
        {
            return Err("incompatible or colliding native conduct checkpoint".into());
        }
        bounded(&checkpoint)?;
        validate_definition(&checkpoint.definition, &position)?;
        check_position(&checkpoint, &position)?;
        checkpoint
            .rule
            .verify_procedure(&checkpoint.definition.procedure)?;
        if checkpoint
            .pending_preparation
            .as_ref()
            .map(|p| p.operation_ref.as_str())
            != checkpoint.pending_operation_ref.as_deref()
            || checkpoint
                .pending_preparation
                .as_ref()
                .is_some_and(|p| p.original_procedure != checkpoint.definition.procedure)
            || checkpoint.membership.selector != checkpoint.definition.procedure.selector
            || checkpoint.membership.mode != checkpoint.definition.procedure.membership_mode
            || checkpoint.membership.containing_expression_ref
                != checkpoint.definition.expression_ref
            || checkpoint.events.len() > MAX_EVENTS
        {
            return Err("native checkpoint original membership/source differs".into());
        }
        lifecycle::validate_checkpoint_lifecycle(&checkpoint)?;
        if let Some(prepared) = &checkpoint.pending_preparation {
            let mut original = prepared.clone();
            original.fingerprint.clear();
            if fingerprint(&original)? != prepared.fingerprint
                || prepared.procedure_ref != checkpoint.definition.procedure.procedure_ref
                || prepared.membership.selector != checkpoint.membership.selector
            {
                return Err("native checkpoint pending preparation differs from its original immutable intent".into());
            }
        }
        for recorded in checkpoint.events.values() {
            if let Some(value) = recorded.receipt.get("prepared").filter(|v| !v.is_null()) {
                let prepared: PreparedProcedure =
                    serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
                let mut original = prepared.clone();
                original.fingerprint.clear();
                if fingerprint(&original)? != prepared.fingerprint
                    || prepared.original_procedure != checkpoint.definition.procedure
                {
                    return Err(
                        "native checkpoint cached preparation differs from original source intent"
                            .into(),
                    );
                }
            }
        }
        // Restore explicitly compiles the same native source once. It stays
        // held; configuration open never masquerades as checkpoint resume.
        let (_, graph) = crate::vak_composition_wire::compile_request(
            &checkpoint.definition.source_composition,
        )?;
        qualify_source(
            &graph,
            &checkpoint.definition,
            &checkpoint.definition.currentness,
        )?;
        checkpoint.rule.pause(witness.admitted_cursor());
        checkpoint.native_position = position;
        let result = receipt(
            &checkpoint,
            "held",
            Some("explicit native resume required"),
            None,
            None,
        )?;
        self.rules.insert(
            checkpoint.definition.procedure.procedure_ref.clone(),
            InstalledConduct { checkpoint, graph },
        );
        Ok(result)
    }
    fn material_readback(
        &mut self,
        input: MaterialReadback,
        position: NativePosition,
    ) -> Result<Value> {
        let installed = self
            .rules
            .get_mut(&input.procedure_ref)
            .ok_or("unknown native installed procedure")?;
        check_position(&installed.checkpoint, &position)?;
        let pending = installed
            .checkpoint
            .pending_preparation
            .as_ref()
            .ok_or("no pending native preparation")?;
        let record = &input.native_record;
        let envelope = &record["envelope"];
        if input.operation_ref != pending.operation_ref
            || installed.checkpoint.pending_operation_ref.as_deref() != Some(&input.operation_ref)
            || input.document_revision == 0
            || record["status"] != "applied"
            || envelope["operation_ref"] != pending.operation_ref
            || envelope["expression_ref"] != pending.native_edit["expression_ref"]
            || envelope["expected_revision"] != pending.native_edit["expected_revision"]
            || envelope["actor"] != pending.native_edit["actor"]
            || envelope["changes"] != pending.native_edit["changes"]
            || record["accepted_revision"].as_u64().is_none_or(|r| {
                pending.native_edit["expected_revision"]
                    .as_u64()
                    .is_none_or(|expected| r <= expected)
            })
            || record["applied_revision"].as_u64().is_none_or(|r| {
                r > input.document_revision
                    || record["accepted_revision"]
                        .as_u64()
                        .is_none_or(|accepted| r <= accepted)
            })
            || record["fingerprint"]
                .as_str()
                .is_none_or(|s| s.len() != 64 || !s.bytes().all(|c| c.is_ascii_hexdigit()))
        {
            return Err("native material readback is foreign/unapplied or differs from original prepared intent".into());
        }
        // The receiver's opaque native typed fingerprint is retained, never
        // rewritten using a procedural JSON hash. This is material readback,
        // not scene/nativeBody/audio consumer acknowledgement.
        let lifecycle_settlement = installed.checkpoint.lifecycle.pending.clone();
        let original_preparation = pending.clone();
        let original_native_position = installed.checkpoint.native_position.clone();
        let mut next = installed.checkpoint.clone();
        lifecycle::settle_lifecycle(&mut next)?;
        next.pending_preparation = None;
        next.pending_operation_ref = None;
        next.native_position = position;
        let mut value = receipt(&next, "material_received", None, None, None)?;
        value["native_material_record"] = record.clone();
        if let Some(pending) = lifecycle_settlement {
            value["original_preparation"] =
                serde_json::to_value(original_preparation).map_err(|e| e.to_string())?;
            value["retained_original_native_position"] =
                serde_json::to_value(original_native_position).map_err(|e| e.to_string())?;
            value["lifecycle"] = json!({"schema":lifecycle::LIFECYCLE_RECEIPT,"action":pending.action,
                "operation_ref":pending.operation_ref,"state":"material_received","consumer_release":"unconfirmed"});
            let event = next
                .events
                .get_mut(&pending.operation_ref)
                .ok_or("original lifecycle operation history absent at material settlement")?;
            event.receipt = value.clone();
        }
        bounded(&next)?;
        installed.checkpoint = next;
        Ok(value)
    }
    fn replace(
        &mut self,
        expected: &str,
        definition: ConductInstall,
        position: NativePosition,
        witness: &NativeTimingWitness,
    ) -> Result<Value> {
        validate_definition(&definition, &position)?;
        let reference = definition.procedure.procedure_ref.clone();
        let installed = self
            .rules
            .get(&reference)
            .ok_or("unknown native installed procedure")?;
        check_position(&installed.checkpoint, &position)?;
        let old = &installed.checkpoint;
        if fingerprint(&old.definition)? != expected
            || old.rule.position == RulePosition::Running
            || old.pending_operation_ref.is_some()
            || old.definition.procedure.selector != definition.procedure.selector
            || old.definition.procedure.membership_mode != definition.procedure.membership_mode
            || old.definition.procedure.principal_subject_ref
                != definition.procedure.principal_subject_ref
            || old.definition.procedure.locus_ref != definition.procedure.locus_ref
            || old.definition.expression_ref != definition.expression_ref
            || old.definition.procedure.seed != definition.procedure.seed
        {
            return Err("source replacement requires exact held original identity/scope and settled native material".into());
        }
        let (_, graph) =
            crate::vak_composition_wire::compile_request(&definition.source_composition)?;
        qualify_source(&graph, &definition, &definition.currentness)?;
        let mut checkpoint = old.clone();
        // Clone preserves the original private queue, edge/cause history and
        // previously admitted interval identities. Only explicitly authored,
        // newly native-qualified source definition changes here.
        let mut rule = old.rule.clone();
        rule.original_procedure = definition.procedure.clone();
        if rule.evaluations > definition.procedure.budgets.max_evaluations
            || rule.operations > definition.procedure.budgets.max_operations
        {
            return Err("replacement budgets are below retained consumption".into());
        }
        rule.epoch_ref = definition.procedure.timing.epoch_ref.clone();
        rule.pause(witness.admitted_cursor());
        checkpoint.rule = rule;
        checkpoint.definition = definition;
        checkpoint.native_position = position;
        bounded(&checkpoint)?;
        let result = receipt(
            &checkpoint,
            "held",
            Some("authored source replacement qualified; explicit resume required"),
            None,
            None,
        )?;
        self.rules
            .insert(reference.clone(), InstalledConduct { checkpoint, graph });
        Ok(result)
    }
    fn event(
        &mut self,
        mut input: ConductEvent,
        position: NativePosition,
        witness: &NativeTimingWitness,
    ) -> Result<Value> {
        project_event_interventions(&mut input)?;
        if input.position != position {
            return Err("rule event is not the actual native owner boundary".into());
        }
        let installed = self
            .rules
            .get_mut(&input.procedure_ref)
            .ok_or("unknown native installed procedure")?;
        check_position(&installed.checkpoint, &position)?;
        let digest = fingerprint(&input)?;
        if let Some(old) = installed
            .checkpoint
            .events
            .get(&input.event.cause.event_ref)
        {
            if old.input_fingerprint != digest {
                return Err(
                    "native cause/event identity reused for another procedural intent".into(),
                );
            }
            return Ok(old.receipt.clone());
        }
        if installed.checkpoint.events.len() >= MAX_EVENTS {
            return Err("native retained event bound reached; checkpoint/restart required".into());
        }
        let mut next = installed.checkpoint.clone();
        next.native_position = position.clone();
        if let Err(error) = qualify_source(&installed.graph, &next.definition, &input.currentness) {
            next.rule.pause(witness.admitted_cursor());
            let result = receipt(&next, "held", Some(&error), None, Some(&input.event.cause))?;
            installed.checkpoint = next;
            return Ok(result);
        }
        if input.event.kind == TriggerKind::SourceChange {
            next.rule.pause(witness.admitted_cursor());
            let result = receipt(
                &next,
                "held",
                Some("native source changed; explicit source re-evaluation/replacement required"),
                None,
                Some(&input.event.cause),
            )?;
            installed.checkpoint = next;
            return Ok(result);
        }
        if next.rule.position != RulePosition::Running {
            return receipt(
                &next,
                "held",
                Some("procedure is paused or cancelled"),
                None,
                Some(&input.event.cause),
            );
        }
        if next.pending_operation_ref.is_some() {
            return receipt(
                &next,
                "held",
                Some(
                    "pending native preparation requires actual SAME Expression material readback",
                ),
                None,
                Some(&input.event.cause),
            );
        }
        let original = &next.definition.procedure;
        let membership = resolve_procedure_membership(
            native_current_m_registry(),
            original,
            &next.definition.expression_ref,
            &input.current_readings,
            Some(&next.membership),
        )?;
        let operation_ref = format!(
            "operation:procedure:{}",
            fingerprint(&(
                original.procedure_ref.as_str(),
                input.event.cause.event_ref.as_str(),
                position.instance_ref.as_str()
            ))?
        );
        // Native trigger/edge/cause admission happens before recipe work.
        // Only actual produced native changes charge the operation budget.
        next.rule.cursor = witness.admitted_cursor();
        next.rule.enqueue(original, input.event.clone())?;
        let admitted = match next.rule.next(original, 0) {
            Ok(value) => value,
            Err(error) => {
                next.rule.pause(witness.admitted_cursor());
                let result = receipt(&next, "held", Some(&error), None, Some(&input.event.cause))?;
                installed.checkpoint = next;
                return Ok(result);
            }
        };
        let prepared = if admitted.is_some() {
            match produce(
                &next,
                &input,
                &membership,
                &operation_ref,
                &installed.graph,
                witness,
            ) {
                Ok(prepared) => prepared,
                Err(error) => {
                    next.rule.pause(witness.admitted_cursor());
                    let result =
                        receipt(&next, "held", Some(&error), None, Some(&input.event.cause))?;
                    installed.checkpoint = next;
                    return Ok(result);
                }
            }
        } else {
            None
        };
        let operations = prepared
            .as_ref()
            .and_then(|p| p.native_edit["changes"].as_array())
            .map_or(0, Vec::len);
        let Some(total) = next
            .rule
            .operations
            .checked_add(operations)
            .filter(|n| *n <= original.budgets.max_operations)
        else {
            next.rule.pause(witness.admitted_cursor());
            let result = receipt(
                &next,
                "held",
                Some("actual generated native operation budget exceeded"),
                None,
                Some(&input.event.cause),
            )?;
            installed.checkpoint = next;
            return Ok(result);
        };
        next.rule.operations = total;
        next.membership = membership;
        if admitted.is_some() {
            let mut generated = program_contributions(
                native_current_m_registry(),
                &next.definition.procedure,
                &next.definition.program,
                &next.definition.expression_ref,
                &input.current_readings,
                &next.membership,
            )?;
            if let Some(context) = &input.materialization {
                generated.retain(|c| !scene_deletion_tombstone(c, context));
            }
            generated = lifecycle::active_generation(
                &next.lifecycle,
                &next.definition.procedure,
                generated,
            )?;
            preserve_authored_basis(&mut generated, &next.last_generated)?;
            for old in &next.last_generated {
                if !next
                    .lifecycle
                    .detached_contribution_refs
                    .contains(&old.contribution_ref)
                    && lifecycle::contribution_is_suppressed(
                        &next.lifecycle,
                        &next.definition.procedure,
                        old,
                    )?
                {
                    generated.push(old.clone());
                }
            }
            next.last_generated = generated;
        }
        if let Some(prepared) = &prepared {
            // A prepared delta contains only material that actually changes.
            // Keep the full source generation, including unchanged outputs,
            // so the next ReadOutputs cannot silently lose contribution IDs.
            preserve_authored_basis(&mut next.last_generated, &prepared.contributions)?;
            next.pending_operation_ref = Some(prepared.operation_ref.clone());
            next.pending_preparation = Some(prepared.clone());
        }
        let status = if prepared.is_some() {
            "prepared"
        } else {
            "ignored"
        };
        let result = receipt(
            &next,
            status,
            None,
            prepared.as_ref(),
            Some(&input.event.cause),
        )?;
        next.events.insert(
            input.event.cause.event_ref,
            RecordedConduct {
                input_fingerprint: digest,
                receipt: result.clone(),
            },
        );
        bounded(&next)?;
        installed.checkpoint = next;
        Ok(result)
    }
}
/// Pure interval derivation; actual ConductHost admits this only with the
/// private owner witness and full source currentness. Never a transported clock.
pub fn continue_native_interval(
    rule: &mut RuleExecution,
    procedure: &Procedure,
    expected_interval_ref: &str,
    original_binding: &TimingBinding,
    admitted_cursor: u64,
) -> Result<()> {
    rule.verify_procedure(procedure)?;
    if &procedure.timing != original_binding
        || rule.interval_ref != expected_interval_ref
        || admitted_cursor <= rule.cursor
    {
        return Err("native interval requires unchanged original timing/source, expected current interval and advancing admitted native cursor".into());
    }
    let interval = format!(
        "native:procedure-interval:{}",
        fingerprint(&(
            procedure.procedure_ref.as_str(),
            original_binding,
            admitted_cursor
        ))?
    );
    rule.begin_interval(&interval, admitted_cursor)
}

fn check_position(checkpoint: &ConductCheckpoint, position: &NativePosition) -> Result<()> {
    if checkpoint.native_position.instance_ref != position.instance_ref
        || checkpoint.native_position.subject_ref != position.subject_ref
        || checkpoint.native_position.event_ref != position.event_ref
        || position.cursor()? < checkpoint.native_position.cursor()?
    {
        return Err("native rule instance/subject/event/sample epoch changed or rewound without explicit source continuation".into());
    }
    Ok(())
}
fn qualify_source(
    graph: &VakComposition,
    definition: &ConductInstall,
    request: &OperativeScopeCurrentnessRequest,
) -> Result<()> {
    if request.expected != definition.currentness.expected
        || request.correlation != definition.currentness.correlation
    {
        return Err("native rule source/occasion changed from its original installed basis".into());
    }
    prepare_native_cprime(
        native_current_m_registry(),
        &definition.procedure,
        graph,
        request.clone(),
        definition.thread_plan.clone(),
    )
    .map(|_| ())
}
/// Source-owned actual native constructors shared by human/agent CLI prepare
/// and installed event conduct. The receiver re-attests Document/membership.
pub fn program_contributions(
    registry: &MRegistry,
    procedure: &Procedure,
    program: &NativeRecipeProgram,
    expression_ref: &str,
    current_readings: &[TargetReading],
    membership: &ResolvedMembership,
) -> Result<Vec<GeneratedContribution>> {
    procedure.validate(registry)?;
    program.validate(registry, procedure, expression_ref)?;
    let mut contributions = Vec::new();
    match program {
        NativeRecipeProgram::SceneMaterial { outputs } => {
            for output in outputs {
                let mut subjects = vec![output.source.principal.clone()];
                subjects.extend(output.source.contributors.clone());
                let mut basis = output.source.construction_basis()?;
                apply_sequence_holds(&mut basis, &output.sequence_holds)?;
                contributions.push(instantiate_scene(
                    procedure,
                    expression_ref,
                    &output.output_slot,
                    &output.scene_ref,
                    &subjects,
                    &basis,
                )?);
            }
        }
        NativeRecipeProgram::ForceParameters { writes } => {
            let mut groups = BTreeMap::<String, Vec<&TargetReading>>::new();
            for target in current_readings {
                if !membership.targets.contains_key(&target.occurrence_ref) {
                    continue;
                }
                let entity = target
                    .address
                    .entity_ref
                    .as_ref()
                    .ok_or("native force recipe requires selected actual Entity")?;
                groups.entry(entity.clone()).or_default().push(target);
            }
            for (entity, mut targets) in groups {
                targets.sort_by(|a, b| a.address.cmp(&b.address));
                for write in writes {
                    let target = targets[0];
                    let value = write.value.resolve(target)?;
                    force_parameter(&write.parameter, &value)?;
                    let native_value = target
                        .properties
                        .get(&write.parameter)
                        .ok_or("actual native original Parameter value unavailable")?;
                    let mut owned = BTreeSet::new();
                    for location in &targets {
                        if location.revision != target.revision
                            || location.subject != target.subject
                            || write.value.resolve(location)? != value
                            || location.properties.get(&write.parameter) != Some(native_value)
                        {
                            return Err("shared native Entity has conflicting actual subject/parameter/recipe readings".into());
                        }
                        let address = native_parameter_address(
                            expression_ref,
                            location
                                .address
                                .scene_ref
                                .as_deref()
                                .ok_or("native force location lacks Scene")?,
                            &entity,
                            &write.parameter,
                        )?;
                        if !membership.addresses.values().any(|a| a.covers(&address)) {
                            return Err("shared native parameter write omits an affected resolved Scene location".into());
                        }
                        owned.insert(address);
                    }
                    let owned = owned.into_iter().collect::<Vec<_>>();
                    let subjects = BTreeSet::from([
                        procedure.principal_subject_ref.clone(),
                        target.subject.subject_ref.clone(),
                    ])
                    .into_iter()
                    .collect::<Vec<_>>();
                    let output_slot = format!("{}:{}", write.output_slot, entity);
                    let mut authored = json!({"schema":"ql.native-parameter-state/v1","address":owned[0],"parameter":write.parameter,"value":native_value,"target_revision":target.revision});
                    if owned.len() > 1 {
                        authored["addresses"] = json!(owned);
                    }
                    contributions.push(GeneratedContribution{contribution_ref:contribution_identity(&procedure.procedure_ref,&output_slot,&subjects,&entity)?,procedure_ref:procedure.procedure_ref.clone(),procedure_revision:procedure.revision.clone(),recipe:procedure.recipe.clone(),output_slot,subjects,occurrence_ref:entity.clone(),owned_addresses:owned,native_changes:vec![NativeChange::ParameterSet{entity_ref:entity.clone(),parameter:write.parameter.clone(),value:value.clone()}],generated_basis:json!({"parameter":write.parameter,"value":value,"authored_basis":authored})});
                }
            }
        }
        NativeRecipeProgram::AtlasPassage { changes } => {
            let bases = current_readings
                .iter()
                .filter(|r| {
                    membership.targets.contains_key(&r.occurrence_ref)
                        && r.address.component == "expression"
                        && r.address.scene_ref.is_none()
                        && r.address.entity_ref.is_none()
                        && r.address.property.is_none()
                        && r.address.constituent_ref.is_none()
                })
                .collect::<Vec<_>>();
            if bases.len() != 1 {
                return Err("native Atlas authored basis requires one exact selected whole Expression reading".into());
            }
            let authored = bases[0]
                .properties
                .get("native_atlas_state")
                .ok_or("native Atlas original Selection/order is unavailable")?;
            validate_atlas_state(authored, expression_ref)?;
            let addresses = membership.addresses.values().cloned().collect::<Vec<_>>();
            contributions.push(GeneratedContribution {
                contribution_ref: contribution_identity(
                    &procedure.procedure_ref,
                    "native-flow",
                    &[procedure.principal_subject_ref.clone()],
                    &procedure.occurrence_ref,
                )?,
                procedure_ref: procedure.procedure_ref.clone(),
                procedure_revision: procedure.revision.clone(),
                recipe: procedure.recipe.clone(),
                output_slot: "native-flow".into(),
                subjects: vec![procedure.principal_subject_ref.clone()],
                occurrence_ref: procedure.occurrence_ref.clone(),
                owned_addresses: addresses,
                native_changes: changes.clone(),
                generated_basis: json!({"native_flow":changes,"authored_basis":authored}),
            });
        }
    }
    Ok(contributions)
}

/// Context for actual new Scene occurrences. Existing targets must supply
/// native owner-read current retention/presentation at exact current CAS.
pub fn program_materialization(
    program: &NativeRecipeProgram,
    document_revision: u64,
    rule_cursor: u64,
    state: &str,
) -> Result<Option<NativeMaterialization>> {
    match program {
        NativeRecipeProgram::SceneMaterial { outputs } => Ok(Some(NativeMaterialization {
            schema: MATERIALIZATION_CONTRACT.into(),
            lifecycles: vec![],
            document_revision,
            rule_cursor,
            state: state.into(),
            scenes: outputs
                .iter()
                .map(|o| NativeRetentionScene {
                    scene_ref: o.scene_ref.clone(),
                    document_revision,
                    existing_retention: None,
                    current_presentation: None,
                    principal: o.source.principal.clone(),
                    contributors: o.source.contributors.clone(),
                    locus: crate::procedural_manifestation::NativeReading {
                        reference: o.source.locus_ref.clone(),
                        revision: o.source.locus_revision.clone(),
                        availability:
                            crate::procedural_manifestation::ReadingAvailability::Available,
                    },
                })
                .collect(),
        })),
        _ => Ok(None),
    }
}

fn produce(
    checkpoint: &ConductCheckpoint,
    input: &ConductEvent,
    membership: &ResolvedMembership,
    operation_ref: &str,
    graph: &VakComposition,
    witness: &NativeTimingWitness,
) -> Result<Option<PreparedProcedure>> {
    let definition = &checkpoint.definition;
    let procedure = &definition.procedure;
    let active_previous = lifecycle::active_generation(
        &checkpoint.lifecycle,
        procedure,
        checkpoint.last_generated.clone(),
    )?;
    let active_refs = active_previous
        .iter()
        .map(|c| c.contribution_ref.as_str())
        .collect::<BTreeSet<_>>();
    let mut active_input = input.clone();
    active_input
        .output_readings
        .retain(|r| active_refs.contains(r.contribution_ref.as_str()));
    active_input
        .current_contributions
        .retain(|r| active_refs.contains(r.contribution_ref.as_str()));
    let input = &active_input;
    if input.document_revision == 0 {
        return Err("native rule event has no actual document basis".into());
    }
    let mut contributions = program_contributions(
        native_current_m_registry(),
        procedure,
        &definition.program,
        &definition.expression_ref,
        &input.current_readings,
        membership,
    )?;
    if let Some(context) = &input.materialization {
        contributions.retain(|c| !scene_deletion_tombstone(c, context));
    }
    contributions = lifecycle::active_generation(&checkpoint.lifecycle, procedure, contributions)?;
    if contributions.is_empty() && active_previous.is_empty() {
        return Ok(None);
    }
    let mut recreated = BTreeSet::new();
    if let Some(context) = &input.materialization {
        for contribution in &mut contributions {
            if !input
                .output_readings
                .iter()
                .any(|r| r.contribution_ref == contribution.contribution_ref)
                && apply_released_scene_interventions(contribution, procedure, context)?
            {
                recreated.insert(contribution.contribution_ref.clone());
            }
        }
    }
    preserve_authored_basis(&mut contributions, &checkpoint.last_generated)?;
    if matches!(
        &definition.program,
        NativeRecipeProgram::ForceParameters { .. }
    ) {
        let context = input
            .materialization
            .as_ref()
            .or_else(|| {
                definition
                    .materialization
                    .as_ref()
                    .filter(|c| c.document_revision == input.document_revision)
            })
            .ok_or("native force continuation requires SAME current retained interventions")?;
        apply_retained_force_interventions(&mut contributions, &input.current_readings, context)?;
    }
    if matches!(
        &definition.program,
        NativeRecipeProgram::AtlasPassage { .. }
    ) {
        let context = input
            .materialization
            .as_ref()
            .or_else(|| {
                definition
                    .materialization
                    .as_ref()
                    .filter(|c| c.document_revision == input.document_revision)
            })
            .ok_or("native Atlas continuation requires SAME current retained interventions")?;
        apply_retained_flow_interventions(&mut contributions, &input.current_readings, context)?;
    }
    match &definition.program {
        NativeRecipeProgram::SceneMaterial { .. } => {
            let previous = active_previous
                .iter()
                .filter(|c| {
                    !recreated.contains(&c.contribution_ref)
                        && !input
                            .materialization
                            .as_ref()
                            .is_some_and(|ctx| scene_deletion_tombstone(c, ctx))
                })
                .cloned()
                .collect::<Vec<_>>();
            if contributions.is_empty() && previous.is_empty() {
                return Ok(None);
            }
            if !previous.is_empty() {
                if input.output_readings.len() != previous.len() {
                    return Err("outstanding native scene construction requires actual receiving ReadOutputs before continuation".into());
                }
                let current = &input.current_contributions;
                if current.len() != input.output_readings.len()
                    || current.iter().any(|c| {
                        !input.output_readings.iter().any(|reading| {
                            reading.contribution_ref == c.contribution_ref
                                && reading.current_basis == c.material
                        })
                    })
                {
                    return Err("native current material/attributed overlays differ from actual ReadOutputs".into());
                }
                let context = input
                    .materialization
                    .as_ref()
                    .ok_or("native regeneration requires SAME current retention context")?;
                let (_, prepared) = crate::procedural_retention::prepare_scene_regeneration(
                    native_current_m_registry(),
                    procedure,
                    operation_ref,
                    &definition.expression_ref,
                    input.document_revision,
                    membership.clone(),
                    &previous,
                    current,
                    &contributions,
                    input.output_readings.clone(),
                    definition.required_consumers.clone(),
                    context,
                )?;
                let Some(mut prepared) = prepared else {
                    return Ok(None);
                };
                bind_prepared_timing(&mut prepared, witness)?;
                prepared.qualify_native_cprime(
                    native_current_m_registry(),
                    procedure,
                    graph,
                    input.currentness.clone(),
                    definition.thread_plan.clone(),
                )?;
                return Ok(Some(prepared));
            }
        }
        NativeRecipeProgram::ForceParameters { .. } | NativeRecipeProgram::AtlasPassage { .. } => {
            if !active_previous.is_empty() {
                let context = input.materialization.as_ref().ok_or(
                    "native sustained continuation requires actual current output retention",
                )?;
                let mut prepared = crate::procedural_retention::prepare_native_output_regeneration(
                    native_current_m_registry(),
                    procedure,
                    operation_ref,
                    &definition.expression_ref,
                    input.document_revision,
                    membership.clone(),
                    &active_previous,
                    &input.current_contributions,
                    &contributions,
                    input.output_readings.clone(),
                    definition.required_consumers.clone(),
                    context,
                )?;
                bind_prepared_timing(&mut prepared, witness)?;
                prepared.qualify_native_cprime(
                    native_current_m_registry(),
                    procedure,
                    graph,
                    input.currentness.clone(),
                    definition.thread_plan.clone(),
                )?;
                return Ok(Some(prepared));
            }
        }
    }
    if contributions.is_empty() {
        return Ok(None);
    }
    let mut prepared = compile_native_batch(
        native_current_m_registry(),
        procedure,
        operation_ref,
        &definition.expression_ref,
        input.document_revision,
        membership.clone(),
        contributions,
        definition.required_consumers.clone(),
    )?;
    let context = if let Some(context) = input.materialization.as_ref().or_else(|| {
        definition
            .materialization
            .as_ref()
            .filter(|c| c.document_revision == input.document_revision)
    }) {
        context.clone()
    } else {
        program_materialization(
            &definition.program,
            input.document_revision,
            checkpoint.rule.evaluations as u64,
            "held",
        )?
        .ok_or("native force/Atlas procedure requires SAME current material/retention context")?
    };
    materialize_retention(&mut prepared, &context)?;
    bind_prepared_timing(&mut prepared, witness)?;
    prepared.qualify_native_cprime(
        native_current_m_registry(),
        procedure,
        graph,
        input.currentness.clone(),
        definition.thread_plan.clone(),
    )?;
    Ok(Some(prepared))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProcedureAuthorship {
    pub procedure_ref: String,
    pub revision: String,
    pub composition: crate::vak_workflow_types::AuthoredCPrime,
    pub trigger: Trigger,
    pub selector: Selector,
    pub conditions: Vec<Condition>,
    pub recipe_parameters: BTreeMap<String, Value>,
    pub membership_mode: MembershipMode,
    pub membership_change_policy: MembershipChangePolicy,
    pub timing: TimingBinding,
    pub budgets: ExecutionBudget,
    pub seed: String,
    pub removal_policy: RemovalPolicy,
    pub failure_policy: FailurePolicy,
    pub continuation_policy: ProcedureContinuation,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OutputSlot {
    pub output_slot: String,
    pub scene_ref: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SequenceHold {
    pub entity_ref: String,
    pub step_ref: Option<String>,
    pub seconds: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "recipe", rename_all = "snake_case", deny_unknown_fields)]
pub enum LibraryChoice {
    SceneMaterial {
        outputs: Vec<OutputSlot>,
    },
    SequenceMaterial {
        outputs: Vec<OutputSlot>,
        holds: Vec<SequenceHold>,
    },
    ForceParameters {
        writes: Vec<ParameterRecipe>,
    },
    AtlasPassage {
        changes: Vec<NativeChange>,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LibraryBuild {
    pub schema: String,
    pub source: NativeSceneSource,
    pub authored: ProcedureAuthorship,
    pub choice: LibraryChoice,
}
/// Production descriptors reference supported ACTUAL constructors and owners.
/// Native source material is supplied by the application's current Scene, not
/// a test fixture, a fabricated graph or an arbitrary M3 inverse.
pub fn library_discover() -> Value {
    json!({"schema":LIBRARY_CONTRACT,"native_owner":"ql-mef","source":"docs/integrations/epi-logos/TA-ONTA-PROCEDURAL-EXPRESSION-CONTRACTS.md#2.3",
        "recipes":[
            {"recipe":"scene_material","label":"Scene material passage","native_operations":["scene_create","entity_add","subject_bind","scene_compose","scene_material_set"],"required_source":"actual current native Scene material, bindings, profile and document revision"},
            {"recipe":"sequence_material","label":"Entity sequence material","native_operations":["scene_material_set"],"properties":["sequence.hold","sequence.steps/@id/hold+holdOverride"],"required_source":"actual authoring entity/step IDs; existing sampler/source/sequence remains native"},
            {"recipe":"force_parameters","label":"Selected force parameters","native_operations":["parameter_set"],"parameters":[{"key":"force_strength","minimum":-20,"maximum":20},{"key":"force_spin","minimum":-20,"maximum":20},{"key":"force_radius","minimum":1,"maximum":1600}],"domain_owner":"oi.expression existing native ParameterSet; driver/override admission remains receiving owner"},
            {"recipe":"atlas_passage","label":"Existing place passage","native_operations":["focus","relation_focus","scene_reorder"],"required_source":"exact same Expression/current resolved native places; continuity remains native Atlas owner"}],
        "editable":["trigger","selector","conditions","seed","recipe_parameters","timing","budgets","membership_mode","failure_policy","continuation_policy"],
        "dependencies":["current native Scene/source/subject/profile","full actual native Cprime graph/currentness and ThreadPlan","actual native target readings and DocumentCAS","ReadOutputs plus attributed authored interventions on regeneration"],
        "standing":"source-owned native construction/recipe descriptors; receiver applies actual native changes; no fixture/model/graph/physical owner synthesized"})
}
pub fn library_build(input: LibraryBuild) -> Result<Value> {
    if input.schema != LIBRARY_CONTRACT {
        return Err("unsupported production native procedure library contract".into());
    }
    let registry = native_current_m_registry();
    input.source.validate(registry)?;
    let source_fingerprint = input.source.material_fingerprint.clone();
    let label = match &input.choice {
        LibraryChoice::SceneMaterial { .. } => "scene_material",
        LibraryChoice::SequenceMaterial { .. } => "sequence_material",
        LibraryChoice::ForceParameters { .. } => "force_parameters",
        LibraryChoice::AtlasPassage { .. } => "atlas_passage",
    };
    let program = match input.choice {
        LibraryChoice::SceneMaterial { outputs } => NativeRecipeProgram::SceneMaterial {
            outputs: outputs
                .into_iter()
                .map(|output| SceneRecipe {
                    output_slot: output.output_slot,
                    scene_ref: output.scene_ref,
                    source: input.source.clone(),
                    sequence_holds: vec![],
                })
                .collect(),
        },
        LibraryChoice::SequenceMaterial { outputs, holds } => {
            if holds.is_empty() {
                return Err("explicit native sequence edits are required".into());
            }
            NativeRecipeProgram::SceneMaterial {
                outputs: outputs
                    .into_iter()
                    .map(|output| SceneRecipe {
                        output_slot: output.output_slot,
                        scene_ref: output.scene_ref,
                        source: input.source.clone(),
                        sequence_holds: holds.clone(),
                    })
                    .collect(),
            }
        }
        LibraryChoice::ForceParameters { writes } => {
            NativeRecipeProgram::ForceParameters { writes }
        }
        LibraryChoice::AtlasPassage { changes } => NativeRecipeProgram::AtlasPassage { changes },
    };
    let a = input.authored;
    let mut parameters = a.recipe_parameters;
    if parameters.contains_key("native_program") {
        return Err("native_program is constructed by the source owner; edit declared recipe parameters instead".into());
    }
    parameters.insert(
        "native_program".into(),
        serde_json::to_value(&program).map_err(|e| e.to_string())?,
    );
    parameters.insert(
        "source_material_fingerprint".into(),
        json!(source_fingerprint),
    );
    let recipe = SourceBasis {
        source_ref: "docs/integrations/epi-logos/TA-ONTA-PROCEDURAL-EXPRESSION-CONTRACTS.md#2.3"
            .into(),
        revision: fingerprint(&include_str!(
            "../../../docs/integrations/epi-logos/TA-ONTA-PROCEDURAL-EXPRESSION-CONTRACTS.md"
        ))?,
    };
    let procedure = Procedure {
        schema: PROCEDURE_CONTRACT.into(),
        procedure_ref: a.procedure_ref,
        revision: a.revision,
        recipe: recipe.clone(),
        profile: input.source.source_basis.clone(),
        registry_revision: registry.manifest().registry_revision.clone(),
        principal_subject_ref: input.source.principal.subject_ref.clone(),
        locus_ref: input.source.locus_ref.clone(),
        occurrence_ref: input.source.expression_ref.clone(),
        composition: a.composition,
        trigger: a.trigger,
        selector: a.selector,
        conditions: a.conditions,
        recipe_parameters: parameters,
        membership_mode: a.membership_mode,
        membership_change_policy: a.membership_change_policy,
        timing: a.timing,
        budgets: a.budgets,
        seed: a.seed,
        seed_algorithm: SEED_ALGORITHM.into(),
        removal_policy: a.removal_policy,
        failure_policy: a.failure_policy,
        continuation_policy: a.continuation_policy,
        admitted_changes: [
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
        ]
        .into_iter()
        .map(str::to_owned)
        .collect(),
    };
    procedure.validate(registry)?;
    program.validate(registry, &procedure, &input.source.expression_ref)?;
    Ok(
        json!({"schema":LIBRARY_CONTRACT,"recipe":label,"procedure":procedure,"program":program,
        "source_basis":[recipe,input.source.source_basis],"source_material_fingerprint":source_fingerprint,
        "original_source_scene":input.source.scene_ref,"document_revision":input.source.document_revision,
        "source_projection":"exact owner-read source projection excludes scene.procedural; actual source/glyph/layer/sequence/force/automation/body material preserved; full DocumentCAS/source provenance retained separately",
        "prepare_dependencies":{"expression_ref":input.source.expression_ref,"document_revision":input.source.document_revision,"current_readings":"same actual native Document selector target snapshots","native_context":"existing source_composition/currentness/ThreadPlan carrying returned source_basis and full authored profile","regeneration":"same native ReadOutputs current/generated/static original operation and attributed interventions"},
        "standing":"actual source-owned native procedure definition and program; receiving owner must re-attest same Document/material/subject/source before prepare/apply"}),
    )
}

fn apply_sequence_holds(presentation: &mut Value, holds: &[SequenceHold]) -> Result<()> {
    if holds.len() > 2048 {
        return Err("native sequence edit bound exceeded".into());
    }
    for hold in holds {
        if !hold.seconds.is_finite() || !(0.0..=3600.0).contains(&hold.seconds) {
            return Err("native sequence hold requires nonnegative seconds".into());
        }
        let entity = presentation["scene"]["entities"]
            .as_array_mut()
            .ok_or("native source has no entity material")?
            .iter_mut()
            .find(|e| e["id"].as_str() == Some(hold.entity_ref.as_str()))
            .ok_or("native sequence entity is absent")?;
        let sequence = entity["sequence"]
            .as_object_mut()
            .ok_or("native source entity has no authoring sequence")?;
        if let Some(step_ref) = &hold.step_ref {
            let step = sequence
                .get_mut("steps")
                .and_then(Value::as_array_mut)
                .ok_or("native source has no sequence steps")?
                .iter_mut()
                .find(|step| step["id"].as_str() == Some(step_ref.as_str()))
                .ok_or("native sequence step is absent")?;
            step["hold"] = json!(hold.seconds);
            step["holdOverride"] = json!(true);
        } else {
            sequence.insert("hold".into(), json!(hold.seconds));
        }
    }
    Ok(())
}

fn validate_atlas_state(value: &Value, expression_ref: &str) -> Result<()> {
    crate::procedural_intervention::validate_flow_state(value, expression_ref)
}
fn preserve_authored_basis(
    next: &mut [GeneratedContribution],
    previous: &[GeneratedContribution],
) -> Result<()> {
    for generation in next {
        if let Some(old) = previous
            .iter()
            .find(|c| c.contribution_ref == generation.contribution_ref)
        {
            if generation.generated_basis.get("authored_basis").is_some() {
                let original=old.generated_basis.get("authored_basis").ok_or("legacy contribution lacks an original native authored basis; cannot invent it on continuation")?;
                generation.generated_basis["authored_basis"] = original.clone();
            }
        }
    }
    Ok(())
}

pub fn scene_deletion_tombstone(
    contribution: &GeneratedContribution,
    context: &NativeMaterialization,
) -> bool {
    if contribution.generated_basis["schema"] != "oi.journey-scene/v1" {
        return false;
    }
    context
        .scenes
        .iter()
        .filter_map(|s| {
            let retention = s.existing_retention.as_ref()?;
            let procedure: Procedure = serde_json::from_value(
                retention["procedures"]
                    .as_array()?
                    .iter()
                    .find(|p| p["procedure_ref"] == contribution.procedure_ref)?["definition"]
                    .clone(),
            )
            .ok()?;
            let anchor = crate::procedural_retention::source_scene_anchor(&procedure, contribution)
                .ok()
                .flatten()?;
            if anchor.scene_ref.as_ref() != Some(&s.scene_ref) {
                return None;
            }
            retention["contributions"].as_array()
        })
        .flatten()
        .any(|row| {
            row["contribution_ref"] == contribution.contribution_ref
                && row["procedure_ref"] == contribution.procedure_ref
                && row["status"] == "detached"
                && row["owned_addresses"] == json!(contribution.owned_addresses)
                && row["authored_overrides"].as_array().is_some_and(|a| {
                    a.iter().any(|r| {
                        r["kind"] == "delete"
                            && r["path"] == "/scene"
                            && r["persistent"] == true
                            && r["address"]["scene_ref"] == contribution.occurrence_ref
                            && r["address"]["component"] == "scene"
                            && r["address"]["property"].is_null()
                    })
                })
        })
}

/// Explicit release retains earlier constituent edits on the canonical source
/// anchor. Recreate uses the same stable generated IDs and native construction
/// operations; the recipe basis remains distinct from effective authored material.
pub fn apply_released_scene_interventions(
    contribution: &mut GeneratedContribution,
    original_procedure: &Procedure,
    context: &NativeMaterialization,
) -> Result<bool> {
    use crate::procedural_intervention::{
        InterventionKind, SceneInterventionBasis, project_scene_intervention_basis,
    };
    use crate::procedural_retention::RetainedContributionDescriptor;
    if contribution.generated_basis["schema"] != "oi.journey-scene/v1" {
        return Ok(false);
    }
    let mut found = None;
    for source in &context.scenes {
        let Some(retention) = &source.existing_retention else {
            continue;
        };
        let Some(definition) = retention["procedures"].as_array().and_then(|a| {
            a.iter()
                .find(|p| p["procedure_ref"] == contribution.procedure_ref)
        }) else {
            continue;
        };
        let procedure: Procedure =
            serde_json::from_value(definition["definition"].clone()).map_err(|e| e.to_string())?;
        if &procedure != original_procedure {
            return Err(
                "released Scene anchor lost exact original full Procedure definition".into(),
            );
        }
        if contribution.procedure_revision != procedure.revision
            || contribution.recipe != procedure.recipe
        {
            return Err(
                "released Scene output revision/source differs from original full Procedure".into(),
            );
        }
        if crate::procedural_retention::source_scene_anchor(&procedure, contribution)?
            .and_then(|a| a.scene_ref)
            .as_ref()
            != Some(&source.scene_ref)
        {
            continue;
        }
        let Some(row) = retention["contributions"].as_array().and_then(|a| {
            a.iter()
                .find(|c| c["contribution_ref"] == contribution.contribution_ref)
        }) else {
            continue;
        };
        let row: RetainedContributionDescriptor =
            serde_json::from_value(row.clone()).map_err(|e| e.to_string())?;
        if row.procedure_ref != contribution.procedure_ref
            || row.output_slot != contribution.output_slot
            || row.subject_refs != contribution.subjects
            || row.occurrence_ref != contribution.occurrence_ref
            || row.owned_addresses != contribution.owned_addresses
        {
            return Err("released Scene anchor differs from original stable contribution".into());
        }
        if row.status != "detached" {
            continue;
        }
        let tombstones = row
            .authored_overrides
            .iter()
            .filter(|r| r.path == "/scene" && r.kind == InterventionKind::Delete)
            .collect::<Vec<_>>();
        let Some(latest) = tombstones.iter().max_by_key(|r| r.revision) else {
            continue;
        };
        if latest.persistent {
            continue;
        }
        if found.is_some() {
            return Err("duplicate released canonical source Scene anchor".into());
        }
        found = Some(row);
    }
    let Some(row) = found else { return Ok(false) };
    let entity_refs = contribution.generated_basis["scene"]["entities"]
        .as_array()
        .ok_or("released material has no entities")?
        .iter()
        .map(|e| {
            let id = e["id"]
                .as_str()
                .ok_or("released material entity has no stable ID")?;
            Ok((id.to_owned(), id.to_owned()))
        })
        .collect::<Result<BTreeMap<_, _>>>()?;
    let basis = SceneInterventionBasis {
        expression_ref: contribution.owned_addresses[0].expression_ref.clone(),
        scene_ref: contribution.occurrence_ref.clone(),
        contribution_ref: contribution.contribution_ref.clone(),
        owned_addresses: contribution.owned_addresses.clone(),
        entity_refs,
        current_presentation: contribution.generated_basis.clone(),
        document_revision: context.document_revision,
        retained_native_records: row.authored_overrides,
    };
    let overlays = project_scene_intervention_basis(&basis)?;
    let mut effective = contribution.generated_basis.clone();
    apply_retained_material_overlays(&mut effective, &contribution.contribution_ref, &overlays)?;
    let ids = effective["scene"]["entities"]
        .as_array()
        .ok_or("released effective Scene lost entities")?
        .iter()
        .map(|e| {
            e["id"]
                .as_str()
                .map(str::to_owned)
                .ok_or("effective entity has no stable ID")
        })
        .collect::<Result<Vec<_>>>()?;
    contribution.native_changes.retain(|change| match change {
        NativeChange::EntityAdd { entity_ref, .. }
        | NativeChange::SubjectBind { entity_ref, .. } => ids.contains(entity_ref),
        _ => true,
    });
    for change in &mut contribution.native_changes {
        match change {
            NativeChange::SceneCompose { entity_refs, .. } => *entity_refs = ids.clone(),
            NativeChange::SceneMaterialSet { presentation, .. } => {
                *presentation = effective.clone()
            }
            _ => {}
        }
    }
    Ok(true)
}

fn bind_prepared_timing(
    prepared: &mut PreparedProcedure,
    witness: &NativeTimingWitness,
) -> Result<()> {
    let mut original = prepared.clone();
    original.fingerprint.clear();
    if fingerprint(&original)? != prepared.fingerprint
        || &prepared.original_procedure.timing != witness.original_binding()
    {
        return Err(
            "native timing projection requires exact unchanged prepared original binding".into(),
        );
    }
    prepared.timing = witness.event_binding();
    prepared.fingerprint.clear();
    prepared.fingerprint = fingerprint(prepared)?;
    Ok(())
}
impl ConductHost {
    /// Full original timing for SAME held-owner private factory. No transported
    /// evidence or browser timestamp is treated as a current native clock.
    pub fn request_timing(&self, request: &ConductRequest) -> Result<Option<TimingBinding>> {
        let reference = match request {
            ConductRequest::SourceBootstrap { .. }
            | ConductRequest::LifecycleCancel { .. }
            | ConductRequest::LibraryDiscover {}
            | ConductRequest::LibraryBuild { .. }
            | ConductRequest::ControlPreflight { .. }
            | ConductRequest::Interventions { .. }
            | ConductRequest::InterventionsOwned { .. }
            | ConductRequest::InterventionBatch { .. }
            | ConductRequest::FlowInterventions { .. }
            | ConductRequest::ProjectFlowInterventions { .. }
            | ConductRequest::SceneDeletionAnchor { .. }
            | ConductRequest::ReleaseSceneDeletion { .. }
            | ConductRequest::ProjectInterventions { .. } => return Ok(None),
            ConductRequest::Install { definition } | ConductRequest::Replace { definition, .. } => {
                return Ok(Some(definition.procedure.timing.clone()));
            }
            ConductRequest::Restore { checkpoint } => {
                return Ok(Some(checkpoint.definition.procedure.timing.clone()));
            }
            ConductRequest::Lifecycle { input } => &input.procedure_ref,
            ConductRequest::Event { input } | ConductRequest::Retire { input } => {
                &input.procedure_ref
            }
            ConductRequest::MaterialReadback { input } => &input.procedure_ref,
            ConductRequest::Read { procedure_ref }
            | ConductRequest::Pause { procedure_ref }
            | ConductRequest::Resume { procedure_ref, .. }
            | ConductRequest::Seek { procedure_ref }
            | ConductRequest::Cancel { procedure_ref }
            | ConductRequest::Checkpoint { procedure_ref }
            | ConductRequest::BeginInterval { procedure_ref, .. } => procedure_ref,
        };
        Ok(Some(
            self.rules
                .get(reference)
                .ok_or("unknown native installed procedure")?
                .checkpoint
                .definition
                .procedure
                .timing
                .clone(),
        ))
    }
    fn retire(
        &mut self,
        mut input: ConductEvent,
        position: NativePosition,
        witness: &NativeTimingWitness,
    ) -> Result<Value> {
        project_event_interventions(&mut input)?;
        if input.position != position {
            return Err("retirement is not the actual native event boundary".into());
        }
        let installed = self
            .rules
            .get_mut(&input.procedure_ref)
            .ok_or("unknown native installed procedure")?;
        check_position(&installed.checkpoint, &position)?;
        let mut next = installed.checkpoint.clone();
        if next.pending_operation_ref.is_some() {
            return Err("retirement waits for actual native pending material settlement".into());
        }
        qualify_source(&installed.graph, &next.definition, &input.currentness)?;
        let procedure = &next.definition.procedure;
        let context = input
            .materialization
            .as_ref()
            .ok_or("native retirement requires current protected material/owned output reads")?;
        let operation_ref = format!(
            "operation:procedure-retire:{}",
            fingerprint(&(
                procedure.procedure_ref.as_str(),
                input.event.cause.event_ref.as_str(),
                position.instance_ref.as_str()
            ))?
        );
        let mut prepared = crate::procedural_retention::prepare_native_retirement(
            native_current_m_registry(),
            procedure,
            &operation_ref,
            &next.definition.expression_ref,
            input.document_revision,
            next.membership.clone(),
            &next.last_generated,
            &input.current_contributions,
            input.output_readings.clone(),
            next.definition.required_consumers.clone(),
            context,
        )?;
        bind_prepared_timing(&mut prepared, witness)?;
        prepared.qualify_native_cprime(
            native_current_m_registry(),
            procedure,
            &installed.graph,
            input.currentness,
            next.definition.thread_plan.clone(),
        )?;
        let count = prepared.native_edit["changes"]
            .as_array()
            .ok_or("native retirement changes unavailable")?
            .len();
        if next.rule.evaluations >= procedure.budgets.max_evaluations
            || next
                .rule
                .operations
                .checked_add(count)
                .is_none_or(|v| v > procedure.budgets.max_operations)
        {
            return Err("actual retirement operation/evaluation budget exceeded".into());
        }
        next.rule.evaluations += 1;
        next.rule.operations += count;
        next.rule.cancel(witness.admitted_cursor());
        next.pending_operation_ref = Some(prepared.operation_ref.clone());
        next.pending_preparation = Some(prepared.clone());
        next.native_position = position;
        bounded(&next)?;
        let result = receipt(
            &next,
            "prepared",
            Some("native retirement/detachment awaits actual SAME receiver material settlement"),
            Some(&prepared),
            Some(&input.event.cause),
        )?;
        installed.checkpoint = next;
        Ok(result)
    }
}

/// Receiving host reconstructs/re-attests each context from its same actual
/// Document row and never forwards caller overlays. Source performs the single
/// canonical projection before causal fingerprint/C-prime/native material.
pub fn project_event_interventions(input: &mut ConductEvent) -> Result<()> {
    use crate::procedural_intervention::{NativeAuthoredIntervention, NativeInterventionBasis};
    if input.intervention_contexts.len() > 2048
        || input
            .current_contributions
            .iter()
            .any(|c| !c.overlays.is_empty())
    {
        return Err("event refuses caller supplied overlays or excessive native context".into());
    }
    let mut groups = BTreeMap::<String, Vec<&NativeInterventionBasis>>::new();
    for context in &input.intervention_contexts {
        groups
            .entry(context.contribution_ref().into())
            .or_default()
            .push(context);
    }
    for (reference, contexts) in groups {
        let contribution = input
            .current_contributions
            .iter_mut()
            .find(|c| c.contribution_ref == reference)
            .ok_or("native context has no actual current contribution")?;
        let outputs = input
            .output_readings
            .iter()
            .filter(|r| r.contribution_ref == reference)
            .collect::<Vec<_>>();
        if outputs.len() != 1 {
            return Err(
                "native intervention context requires one exact original output reading".into(),
            );
        }
        let output = outputs[0];
        if contribution.material != output.current_basis {
            return Err("native intervention current material differs from actual output".into());
        }
        let parameter = output.generated_basis["parameter"].as_str();
        let mut scenes = BTreeSet::new();
        let mut records = Vec::<&NativeAuthoredIntervention>::new();
        let mut overlays = Vec::new();
        for context in contexts {
            let (expression, revision, owned) = match context {
                NativeInterventionBasis::Scene { basis } => (
                    &basis.expression_ref,
                    basis.document_revision,
                    &basis.owned_addresses,
                ),
                NativeInterventionBasis::NativeFlow { basis } => (
                    &basis.expression_ref,
                    basis.document_revision,
                    &basis.owned_addresses,
                ),
            };
            if expression != &output.expression_ref
                || revision != input.document_revision
                || owned != &output.owned_addresses
            {
                return Err("native event intervention context differs from actual output/current CAS/owned address".into());
            }
            match context {
                NativeInterventionBasis::Scene { basis } => {
                    if !scenes.insert(basis.scene_ref.clone())
                        || basis
                            .retained_native_records
                            .iter()
                            .any(|r| r.address.scene_ref.as_ref() != Some(&basis.scene_ref))
                    {
                        return Err(
                            "duplicate Scene context or foreign Scene intervention row".into()
                        );
                    }
                    if parameter.is_none() && basis.current_presentation != contribution.material {
                        return Err(
                            "Scene intervention basis differs from actual current material".into(),
                        );
                    }
                    let projected = context.project()?;
                    if parameter.is_some() {
                        records.extend(basis.retained_native_records.iter());
                    } else {
                        overlays.extend(projected);
                    }
                }
                NativeInterventionBasis::NativeFlow { basis } => {
                    if parameter.is_some()
                        || !scenes.insert(String::new())
                        || basis.current_state != contribution.material
                    {
                        return Err("native flow intervention context is duplicate or differs from actual state".into());
                    }
                    overlays.extend(context.project()?);
                }
            }
        }
        if let Some(parameter) = parameter {
            let expected = output
                .owned_addresses
                .iter()
                .filter_map(|a| a.scene_ref.clone())
                .collect::<BTreeSet<_>>();
            if scenes != expected
                || contribution.material["schema"] != "ql.native-parameter-state/v1"
                || contribution.material["parameter"] != parameter
            {
                return Err(
                    "native parameter intervention contexts omit an actual owned Scene location"
                        .into(),
                );
            }
            let mut latest: Option<&NativeAuthoredIntervention> = None;
            for record in records {
                if !record.persistent
                    || !output
                        .owned_addresses
                        .iter()
                        .any(|a| record.address.covers(a))
                {
                    continue;
                }
                if let Some(old) = latest {
                    if old.revision == record.revision
                        && (old.actor != record.actor || old.operation_ref != record.operation_ref)
                    {
                        return Err(
                            "conflicting native parameter intervention provenance at one revision"
                                .into(),
                        );
                    }
                    if old.revision > record.revision {
                        continue;
                    }
                }
                latest = Some(record);
            }
            if let Some(row) = latest {
                // Authored Scene radius .3 is not native force_radius 120. The
                // protected current output supplies the existing native value.
                overlays.push(AuthoredOverlay {
                    contribution_ref: reference.clone(),
                    pointer: "/value".into(),
                    value: contribution.material["value"].clone(),
                    actor_ref: row.actor.clone(),
                    persistent: true,
                    operation: OverlayOperation::Set,
                });
            }
        }
        contribution.overlays = overlays;
    }
    Ok(())
}
