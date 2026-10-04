//! Definition installation and current-CAS continuation on the SAME ConductHost.
//! Configuration validation is pure; only the private native host stages and
//! commits after actual Scene/source/consumer pre/post qualification.
use super::*;
use crate::procedural_consumers::NativeConsumerContract;
use crate::procedural_source::NativeSourceBootstrap;

pub const PREPARED_INSTALL: &str = "ql.native-procedural-prepared-install/v1";
pub const SOURCE_CONTINUATION: &str = "ql.native-procedural-source-continuation/v1";
pub const DEFINITION_RECEIPT: &str = "ql.native-procedural-definition-receipt/v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativePreparedInstall {
    pub schema: String,
    pub definition: ConductInstall,
    pub original_preparation: PreparedProcedure,
    pub source_bootstrap: NativeSourceBootstrap,
}
/// Root supplies current owner readings and identities only. Rule state,
/// pending preparation, prior native position and original definition are read
/// from the installed owner, never from this transported input.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeSourceContinuation {
    pub schema: String,
    pub procedure_ref: String,
    pub expected_procedure_revision: String,
    pub source_bootstrap: NativeSourceBootstrap,
    pub current_readings: Vec<TargetReading>,
    pub materialization: Option<lifecycle::NativeLifecycleMaterialization>,
}

/// Borrowed only from the actual installed ConductHost. There is no public
/// constructor, deserializer or copied Value-to-origin conversion. This keeps
/// original semantic authoring separate from a newly observed material body.
pub(crate) struct NativeDefinitionSourceOrigin<'a> {
    original: &'a NativeSourceBootstrap,
    definition: &'a ConductInstall,
}
impl NativeDefinitionSourceOrigin<'_> {
    pub(crate) fn original(&self) -> &NativeSourceBootstrap {
        self.original
    }
    pub(crate) fn definition(&self) -> &ConductInstall {
        self.definition
    }
}
pub(crate) struct StagedDefinition {
    procedure_ref: String,
    original_checkpoint_fingerprint: Option<String>,
    next: InstalledConduct,
    receipt: Value,
    current_source: NativeSourceBootstrap,
}
impl StagedDefinition {
    pub(crate) fn source_bootstrap(&self) -> &NativeSourceBootstrap {
        &self.current_source
    }
}

fn source_context(source: &Value) -> Result<(Value, OperativeScopeCurrentnessRequest, ThreadPlan)> {
    if source["schema"] != crate::procedural_source::SOURCE_BOOTSTRAP {
        return Err("definition requires the actual freshly compiled native Source".into());
    }
    Ok((
        source["source_composition"].clone(),
        serde_json::from_value(source["currentness"].clone()).map_err(|e| e.to_string())?,
        serde_json::from_value(source["thread_plan"].clone()).map_err(|e| e.to_string())?,
    ))
}
fn source_matches_definition(
    source: &Value,
    definition: &ConductInstall,
    bootstrap: &NativeSourceBootstrap,
) -> Result<()> {
    let (composition, currentness, plan) = source_context(source)?;
    if source["expression_ref"] != definition.expression_ref
        || source["scene_ref"] != bootstrap.scene.scene_ref
        || source["document_revision"].as_u64() != Some(definition.document_revision)
        || source["authored_cprime"]
            != serde_json::to_value(&definition.procedure.composition).map_err(|e| e.to_string())?
        || composition != definition.source_composition
        || serde_json::to_value(&currentness).map_err(|e| e.to_string())?
            != serde_json::to_value(&definition.currentness).map_err(|e| e.to_string())?
        || serde_json::to_value(&plan).map_err(|e| e.to_string())?
            != serde_json::to_value(&definition.thread_plan).map_err(|e| e.to_string())?
    {
        return Err("definition differs from the private current native Source compilation".into());
    }
    Ok(())
}
/// Native batch configuration validation only. This does not install a rule,
/// construct a timing/Scene capability, acknowledge material, or seed a journal.
/// The private host adds original Source custody only after the real C/R checks.
pub fn prepare_native_definition_configuration(
    definition: &ConductInstall,
    prepared: &PreparedProcedure,
    position: &NativePosition,
    admitted_cursor: u64,
) -> Result<ConductCheckpoint> {
    prepared_batch_configuration(definition, prepared, position, admitted_cursor)
        .map(|installed| installed.checkpoint)
}
fn prepared_batch_configuration(
    d: &ConductInstall,
    p: &PreparedProcedure,
    position: &NativePosition,
    admitted_cursor: u64,
) -> Result<InstalledConduct> {
    bounded(&(d, p))?;
    validate_definition(d, position)?;
    if serde_json::to_value(&d.program).map_err(|e| e.to_string())?
        != d.procedure
            .recipe_parameters
            .get("native_program")
            .cloned()
            .unwrap_or(Value::Null)
    {
        return Err("first native program differs from its full original Procedure".into());
    }
    let (_, graph) = crate::vak_composition_wire::compile_request(&d.source_composition)?;
    qualify_source(&graph, d, &d.currentness)?;
    let membership = resolve_procedure_membership(
        native_current_m_registry(),
        &d.procedure,
        &d.expression_ref,
        &d.current_readings,
        None,
    )?;
    if p.original_procedure != d.procedure
        || p.expected_document_revision != d.document_revision
        || p.native_edit["expression_ref"] != d.expression_ref
        || p.native_edit["expected_revision"].as_u64() != Some(d.document_revision)
        || p.required_consumers != d.required_consumers
        || p.membership != membership
        || p.timing != d.procedure.timing
        || p.lifecycle_intent.is_some()
        || p.output_readings
            .iter()
            .any(|r| r.procedure_ref != d.procedure.procedure_ref)
    {
        return Err(
            "first prepared batch changed its original definition/CAS/selector/roles".into(),
        );
    }
    let mut qualified = p.clone();
    qualified.qualify_native_cprime(
        native_current_m_registry(),
        &d.procedure,
        &graph,
        d.currentness.clone(),
        d.thread_plan.clone(),
    )?;
    if qualified != *p || p.native_cprime.is_none() {
        return Err("first batch lacks its exact original native C-prime seal".into());
    }
    let operations = p.native_edit["changes"]
        .as_array()
        .ok_or("first native Edit changes absent")?
        .len();
    if operations > d.procedure.budgets.max_operations {
        return Err("first generated batch exceeds original operation budget".into());
    }
    let mut rule = RuleExecution::new(&d.procedure, &d.interval_ref)?;
    rule.cursor = admitted_cursor;
    rule.operations = operations;
    let checkpoint = ConductCheckpoint {
        schema: CONDUCT_CONTRACT.into(),
        definition: d.clone(),
        original_source: None,
        original_consumer_contract: None,
        rule,
        membership,
        last_generated: p.contributions.clone(),
        pending_operation_ref: Some(p.operation_ref.clone()),
        pending_preparation: Some(p.clone()),
        native_position: position.clone(),
        events: BTreeMap::new(),
        lifecycle: lifecycle::LifecycleCheckpoint::default(),
    };
    bounded(&checkpoint)?;
    Ok(InstalledConduct { checkpoint, graph })
}
/// Exact first batch seed, not an Applied operation or material receipt.
fn prepared_install_configuration(
    input: &NativePreparedInstall,
    position: &NativePosition,
    admitted_cursor: u64,
    source: &Value,
) -> Result<InstalledConduct> {
    bounded(input)?;
    input.source_bootstrap.scene.validate()?;
    if input.schema != PREPARED_INSTALL {
        return Err("unsupported private prepared installation".into());
    }
    source_matches_definition(source, &input.definition, &input.source_bootstrap)?;
    let mut installed = prepared_batch_configuration(
        &input.definition,
        &input.original_preparation,
        position,
        admitted_cursor,
    )?;
    let contract: NativeConsumerContract =
        serde_json::from_value(source["consumer_contract"].clone()).map_err(|e| e.to_string())?;
    installed.checkpoint.original_source = Some(input.source_bootstrap.clone());
    installed.checkpoint.original_consumer_contract = Some(contract);
    bounded(&installed.checkpoint)?;
    Ok(installed)
}

/// Recompile a configured native graph at a new Document CAS. This returns
/// definition configuration only, never a live owner/source capability.
pub fn reobserve_native_definition_configuration(
    original: &ConductInstall,
    composition: &Value,
    currentness: &OperativeScopeCurrentnessRequest,
    plan: &ThreadPlan,
    current_readings: &[TargetReading],
    document_revision: u64,
) -> Result<ConductInstall> {
    reobserve_definition_configuration(
        original,
        composition,
        currentness,
        plan,
        current_readings,
        document_revision,
    )
    .map(|(definition, _)| definition)
}
fn reobserve_definition_configuration(
    original: &ConductInstall,
    composition: &Value,
    currentness: &OperativeScopeCurrentnessRequest,
    plan: &ThreadPlan,
    current_readings: &[TargetReading],
    document_revision: u64,
) -> Result<(ConductInstall, VakComposition)> {
    bounded(&(original, composition, currentness, plan, current_readings))?;
    if document_revision < original.document_revision
        || currentness.expected != original.currentness.expected
        || currentness.correlation != original.currentness.correlation
        || serde_json::to_value(plan).map_err(|e| e.to_string())?
            != serde_json::to_value(&original.thread_plan).map_err(|e| e.to_string())?
    {
        return Err(
            "current native definition changed original semantic binding/thread or rewound CAS"
                .into(),
        );
    }
    let (_, graph) = crate::vak_composition_wire::compile_request(composition)?;
    prepare_native_cprime(
        native_current_m_registry(),
        &original.procedure,
        &graph,
        currentness.clone(),
        plan.clone(),
    )?;
    let mut next = original.clone();
    next.document_revision = document_revision;
    next.current_readings = current_readings.to_vec();
    next.source_composition = composition.clone();
    next.currentness = currentness.clone();
    next.thread_plan = plan.clone();
    // An old material reading does not become current by copying it.
    next.materialization = None;
    Ok((next, graph))
}
/// Validate original/current source configuration identity only. Current
/// material is permitted to differ after actual owned native or human edits;
/// each complete presentation must retain its own exact fingerprint. This
/// grants no Source capability and the private host still qualifies the fresh
/// full Document/Scene read before and after the staged continuation.
pub fn validate_continuation_scene_configuration(
    original: &crate::procedural_source::NativeBootstrapSceneRead,
    current: &crate::procedural_source::NativeBootstrapSceneRead,
) -> Result<()> {
    bounded(&(original, current))?;
    original.validate()?;
    current.validate()?;
    if current.native_owner != original.native_owner
        || current.expression_ref != original.expression_ref
        || current.scene_ref != original.scene_ref
        || current.document_revision < original.document_revision
        || current.source_basis != original.source_basis
        || current.locus != original.locus
    {
        return Err(
            "continuation changed original source namespace/profile/locus or rewound CAS".into(),
        );
    }
    Ok(())
}
fn continuation_configuration(
    installed: &InstalledConduct,
    input: &NativeSourceContinuation,
    position: &NativePosition,
    source: &Value,
    contract: &NativeConsumerContract,
) -> Result<InstalledConduct> {
    bounded(input)?;
    input.source_bootstrap.scene.validate()?;
    let old = &installed.checkpoint;
    let origin = old.original_source.as_ref().ok_or("legacy install has no accepted full source authoring; explicit qualified replacement required")?;
    let scene = &input.source_bootstrap.scene;
    validate_continuation_scene_configuration(&origin.scene, scene)?;
    if input.schema != SOURCE_CONTINUATION
        || input.procedure_ref != old.definition.procedure.procedure_ref
        || input.expected_procedure_revision != old.definition.procedure.revision
        || scene.expression_ref != old.definition.expression_ref
        || scene.scene_ref != origin.scene.scene_ref
        || scene.document_revision < old.definition.document_revision
        || scene.source_basis != origin.scene.source_basis
        || scene.locus != origin.scene.locus
        || serde_json::to_value(&input.source_bootstrap.authorship).map_err(|e| e.to_string())?
            != serde_json::to_value(&origin.authorship).map_err(|e| e.to_string())?
    {
        return Err(
            "continuation changed original source authoring/profile/locus or rewound CAS".into(),
        );
    }
    let original_contract = old
        .original_consumer_contract
        .as_ref()
        .ok_or("original native constructor roster absent")?;
    if original_contract.requirements != contract.requirements {
        return Err("continuation changed original registered constructor lifetimes".into());
    }
    check_position(old, position)?;
    let (composition, currentness, plan) = source_context(source)?;
    if source["document_revision"].as_u64() != Some(scene.document_revision)
        || source["authored_cprime"]
            != serde_json::to_value(&old.definition.procedure.composition)
                .map_err(|e| e.to_string())?
        || currentness.expected != old.definition.currentness.expected
        || currentness.correlation != old.definition.currentness.correlation
        || serde_json::to_value(&plan).map_err(|e| e.to_string())?
            != serde_json::to_value(&old.definition.thread_plan).map_err(|e| e.to_string())?
        || contract.required_consumers != old.definition.required_consumers
        || !same_timing_anchor(&contract.original_timing, &old.definition.procedure.timing)
    {
        return Err(
            "fresh Source differs from original semantic definition/thread/consumer/timing anchor"
                .into(),
        );
    }
    let (current_definition, graph) = reobserve_definition_configuration(
        &old.definition,
        &composition,
        &currentness,
        &plan,
        &input.current_readings,
        scene.document_revision,
    )?;
    let membership = resolve_procedure_membership(
        native_current_m_registry(),
        &old.definition.procedure,
        &old.definition.expression_ref,
        &input.current_readings,
        Some(&old.membership),
    )?;
    if membership.selector != old.membership.selector
        || membership.mode != old.membership.mode
        || membership.addresses != old.membership.addresses
        || membership.targets.keys().ne(old.membership.targets.keys())
    {
        return Err("source continuation cannot change original selector membership; use native event/replacement".into());
    }
    let mut next = old.clone();
    next.definition = current_definition;
    if let Some(read) = &input.materialization {
        if read.schema != lifecycle::LIFECYCLE_MATERIALIZATION
            || read.document_revision != scene.document_revision
            || read
                .scenes
                .iter()
                .any(|s| s.document_revision != scene.document_revision)
        {
            return Err(
                "continuation materialization has another current native Document CAS".into(),
            );
        }
        next.definition.materialization = Some(NativeMaterialization {
            schema: MATERIALIZATION_CONTRACT.into(),
            lifecycles: vec![],
            document_revision: scene.document_revision,
            rule_cursor: old.rule.cursor,
            state: if old.rule.position == RulePosition::Running {
                "running"
            } else {
                "held"
            }
            .into(),
            scenes: read.scenes.clone(),
        });
    } else {
        // Never leave an old-CAS material reading presented as current.
        next.definition.materialization = None;
    }
    next.native_position = position.clone();
    bounded(&next)?;
    Ok(InstalledConduct {
        checkpoint: next,
        graph,
    })
}
pub(crate) fn same_timing_anchor(a: &TimingBinding, b: &TimingBinding) -> bool {
    a.owner_ref == b.owner_ref
        && a.domain == b.domain
        && a.epoch_ref == b.epoch_ref
        && a.time_mapping_ref == b.time_mapping_ref
}
impl ConductHost {
    pub(crate) fn source_continuation_origin(
        &self,
        request: &ConductRequest,
    ) -> Result<Option<NativeDefinitionSourceOrigin<'_>>> {
        match request {
            ConductRequest::InstallPrepared { .. } => Ok(None),
            ConductRequest::SourceContinue { input } => {
                let current = self
                    .rules
                    .get(&input.procedure_ref)
                    .ok_or("unknown native installed procedure")?;
                if input.expected_procedure_revision
                    != current.checkpoint.definition.procedure.revision
                {
                    return Err("continuation changed original installed Procedure revision".into());
                }
                Ok(Some(NativeDefinitionSourceOrigin {
                    original: current
                        .checkpoint
                        .original_source
                        .as_ref()
                        .ok_or("installed native original Source custody absent")?,
                    definition: &current.checkpoint.definition,
                }))
            }
            _ => Err("semantic original source is unavailable to another public action".into()),
        }
    }
    pub(crate) fn stage_prepared_install(
        &self,
        input: NativePreparedInstall,
        position: NativePosition,
        witness: &NativeTimingWitness,
        source: &Value,
        contract: &NativeConsumerContract,
    ) -> Result<StagedDefinition> {
        if witness.original_binding() != &input.definition.procedure.timing
            || witness.native_position() != &position
            || !same_timing_anchor(&contract.original_timing, witness.original_binding())
            || contract.required_consumers != input.definition.required_consumers
            || self
                .rules
                .contains_key(&input.definition.procedure.procedure_ref)
            || self.rules.len() >= MAX_RULES
        {
            return Err(
                "private first installation has another original timing/roster or colliding rule"
                    .into(),
            );
        }
        let next =
            prepared_install_configuration(&input, &position, witness.admitted_cursor(), source)?;
        let mut result = receipt(
            &next.checkpoint,
            "installed_pending_material",
            None,
            None,
            None,
        )?;
        result["definition_receipt"] = json!({"schema":DEFINITION_RECEIPT,"kind":"install_prepared",
            "definition":next.checkpoint.definition,"original_preparation":input.original_preparation,
            "source_bootstrap":input.source_bootstrap,"current_source":source,"consumer_contract":contract,
            "native_timing_evidence":witness.evidence(),"material_status":"pending_reception"});
        Ok(StagedDefinition {
            procedure_ref: next.checkpoint.definition.procedure.procedure_ref.clone(),
            original_checkpoint_fingerprint: None,
            next,
            receipt: result,
            current_source: input.source_bootstrap,
        })
    }
    pub(crate) fn stage_source_continuation(
        &self,
        input: NativeSourceContinuation,
        position: NativePosition,
        witness: &NativeTimingWitness,
        source: &Value,
        contract: &NativeConsumerContract,
    ) -> Result<StagedDefinition> {
        let installed = self
            .rules
            .get(&input.procedure_ref)
            .ok_or("unknown native installed procedure")?;
        if witness.original_binding() != &installed.checkpoint.definition.procedure.timing
            || witness.native_position() != &position
        {
            return Err("continuation lacks SAME native original timing/current boundary".into());
        }
        let next = continuation_configuration(installed, &input, &position, source, contract)?;
        let mut result = receipt(&next.checkpoint, "source_continued", None, None, None)?;
        result["definition_receipt"] = json!({"schema":DEFINITION_RECEIPT,"kind":"source_continue",
            "original_definition":installed.checkpoint.definition,"definition":next.checkpoint.definition,
            "source_bootstrap":input.source_bootstrap,"current_source":source,"consumer_contract":contract,
            "native_timing_evidence":witness.evidence(),"material_status":"unchanged"});
        Ok(StagedDefinition {
            procedure_ref: input.procedure_ref,
            original_checkpoint_fingerprint: Some(fingerprint(&installed.checkpoint)?),
            next,
            receipt: result,
            current_source: input.source_bootstrap,
        })
    }
    pub(crate) fn commit_definition(&mut self, staged: StagedDefinition) -> Result<Value> {
        match (
            &staged.original_checkpoint_fingerprint,
            self.rules.get(&staged.procedure_ref),
        ) {
            (None, None) if self.rules.len() < MAX_RULES => {}
            (Some(original), Some(current)) if fingerprint(&current.checkpoint)? == *original => {}
            _ => {
                return Err(
                    "native definition/checkpoint changed before actual source postvalidation"
                        .into(),
                );
            }
        }
        self.rules.insert(staged.procedure_ref, staged.next);
        Ok(staged.receipt)
    }
}
