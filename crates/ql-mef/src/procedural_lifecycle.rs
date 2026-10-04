//! Source-owned lifecycle intent on the SAME installed native procedure.
//! The protected receiver supplies actual Document readings; Source derives
//! currentness, cause and retained rule position. A lifecycle preparation is
//! material intent, never a physical/body/audio acknowledgement.
use super::*;
use crate::procedural_intervention::NativeInterventionBasis;
use crate::procedural_retention::{
    ContributionLifecycle, NativeContributionLifecycle, NativeRetentionScene,
};
use crate::procedural_source::NativeBootstrapSceneRead;

pub const LIFECYCLE_INTENT: &str = "ql.procedural-lifecycle-intent/v1";
pub const LIFECYCLE_READING: &str = "ql.native-procedural-lifecycle-reading/v1";
pub const LIFECYCLE_MATERIALIZATION: &str = "ql.procedural-lifecycle-materialization-reading/v1";
pub const LIFECYCLE_RECEIPT: &str = "ql.procedural-lifecycle-receipt/v1";
pub const LIFECYCLE_CANCEL: &str = "ql.procedural-lifecycle-cancel/v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum LifecycleAction {
    Retire,
    ContributionDetach {
        contribution_ref: String,
    },
    ScenePolicy {
        from_scene_ref: String,
        to_scene_ref: String,
        policy: ScenePolicy,
    },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScenePolicy {
    Continue,
    Hold,
    CheckpointRelease,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeLifecycleMaterialization {
    pub schema: String,
    pub document_revision: u64,
    pub scenes: Vec<NativeRetentionScene>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeLifecycleReading {
    pub schema: String,
    pub scene_read: NativeBootstrapSceneRead,
    pub current_readings: Vec<TargetReading>,
    pub output_readings: Vec<RetainedOutputReading>,
    pub current_contributions: Vec<CurrentContribution>,
    pub intervention_contexts: Vec<NativeInterventionBasis>,
    pub materialization: NativeLifecycleMaterialization,
    /// Actual native Document.selection and scene_order. Root reads/re-attests
    /// this whole Expression property; it is not derived from a Scene template.
    pub current_flow: Option<Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeLifecycleInput {
    pub schema: String,
    pub expression_ref: String,
    pub document_revision: u64,
    pub scene_ref: String,
    pub operation_ref: String,
    pub actor_ref: String,
    pub procedure_ref: String,
    pub expected_procedure_revision: String,
    pub action: LifecycleAction,
    pub reading: NativeLifecycleReading,
}

/// Protected receiving context for an actual cancelled native S operation.
/// The public caller carries identities only. Native Root substitutes the FULL
/// typed Runtime operation and matching retained Document journal before C31.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeLifecycleCancel {
    pub schema: String,
    pub expression_ref: String,
    pub document_revision: u64,
    pub scene_ref: String,
    pub operation_ref: String,
    pub actor_ref: String,
    pub procedure_ref: String,
    pub expected_procedure_revision: String,
    pub scene_read: NativeBootstrapSceneRead,
    /// Exact original StageSourceAuthorship.contributors in original order.
    pub contributors: Vec<NativeSubject>,
    pub native_record: Value,
}
impl NativeLifecycleCancel {
    pub fn validate_reading(&self, definition: &ConductInstall) -> Result<()> {
        bounded(self)?;
        self.scene_read.validate()?;
        let p = &definition.procedure;
        if self.schema != LIFECYCLE_CANCEL
            || self.expression_ref != definition.expression_ref
            || self.procedure_ref != p.procedure_ref
            || self.expected_procedure_revision != p.revision
            || self.document_revision == 0
            || self.scene_read.expression_ref != self.expression_ref
            || self.scene_read.scene_ref != self.scene_ref
            || self.scene_read.document_revision != self.document_revision
            || self.scene_read.source_basis != p.profile
            || self.scene_read.locus.reference != p.locus_ref
            || self.contributors.len() > 256
        {
            return Err(
                "native lifecycle cancellation source/Document/original definition differs".into(),
            );
        }
        nonempty(&self.actor_ref, "lifecycle cancellation actor")?;
        nonempty(&self.operation_ref, "lifecycle cancellation operation")?;
        for subject in &self.contributors {
            validate_native_subject_basis(native_current_m_registry(), subject)?;
        }
        Ok(())
    }

    /// Pure correspondence validation, NOT receipt authority. SAME private
    /// receiver must first re-attest FULL typed Operation, immutable Envelope,
    /// native opaque fingerprint, scope/sources/participants and retained journal.
    /// In particular a Scheduled operation can reach this only after actual
    /// timing-owner withdrawal and normal native Cancel; JSON cannot withdraw it.
    pub fn validate_cancelled_configuration(
        &self,
        definition: &ConductInstall,
        prepared: &PreparedProcedure,
    ) -> Result<()> {
        self.validate_reading(definition)?;
        let intent = prepared
            .lifecycle_intent
            .as_ref()
            .ok_or("cancelled operation is not an original Source lifecycle preparation")?;
        let record = &self.native_record;
        let envelope = &record["envelope"];
        let expected = prepared.native_edit["expected_revision"]
            .as_u64()
            .ok_or("original lifecycle native CAS absent")?;
        if prepared.original_procedure != definition.procedure
            || self.operation_ref != prepared.operation_ref
            || intent["scene_ref"] != self.scene_ref
            || intent["actor_ref"] != self.actor_ref
            || intent["operation_ref"] != self.operation_ref
            || self.document_revision <= expected
            || record["status"] != "cancelled"
            || record.get("applied_revision") != Some(&Value::Null)
            || record["accepted_revision"]
                .as_u64()
                .is_none_or(|revision| revision <= expected || revision > self.document_revision)
            || record["fingerprint"]
                .as_str()
                .is_none_or(|s| s.len() != 64 || !s.bytes().all(|c| c.is_ascii_hexdigit()))
            || !record["targets"].is_array()
            || !record["observations"].is_array()
            || !envelope["scope"].is_object()
            || !envelope["sources"].is_array()
            || !envelope["participants"].is_array()
            || !envelope["timing"].is_object()
            || envelope["operation_ref"] != prepared.operation_ref
            || envelope["expression_ref"] != prepared.native_edit["expression_ref"]
            || envelope["expected_revision"] != prepared.native_edit["expected_revision"]
            || envelope["actor"] != prepared.native_edit["actor"]
            || envelope["actor"] != self.actor_ref
            || envelope["changes"] != prepared.native_edit["changes"]
            || envelope
                .get("output_readings")
                .cloned()
                .unwrap_or_else(|| json!([]))
                != serde_json::to_value(&prepared.output_readings).map_err(|e| e.to_string())?
        {
            return Err("native cancellation is not the actual cancelled/null-applied original lifecycle operation".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SettledScenePolicy {
    pub from_scene_ref: String,
    pub to_scene_ref: String,
    pub policy: ScenePolicy,
    pub operation_ref: String,
    pub rule_cursor: u64,
    pub native_position: NativePosition,
    pub original_timing: TimingBinding,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PendingLifecycle {
    pub operation_ref: String,
    pub action: LifecycleAction,
    pub policy: Option<SettledScenePolicy>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LifecycleCheckpoint {
    pub detached_contribution_refs: BTreeSet<String>,
    pub scene_policies: BTreeMap<String, SettledScenePolicy>,
    pub pending: Option<PendingLifecycle>,
}

impl NativeLifecycleInput {
    /// This checks the exact paired wire and source qualification. Genuine
    /// native source origin is separately checked by the private C lease before
    /// and after the SAME receiving HostRequest.
    pub fn validate_reading(&self, definition: &ConductInstall) -> Result<()> {
        bounded(self)?;
        let p = &definition.procedure;
        let read = &self.reading.scene_read;
        read.validate()?;
        if self.schema != LIFECYCLE_INTENT
            || self.reading.schema != LIFECYCLE_READING
            || self.expression_ref != definition.expression_ref
            || self.procedure_ref != p.procedure_ref
            || self.expected_procedure_revision != p.revision
            || self.document_revision == 0
            || read.expression_ref != self.expression_ref
            || read.scene_ref != self.scene_ref
            || read.document_revision != self.document_revision
            || read.source_basis != p.profile
            || read.locus.reference != p.locus_ref
            || self.reading.materialization.schema != LIFECYCLE_MATERIALIZATION
            || self.reading.materialization.document_revision != self.document_revision
            || self.reading.materialization.scenes.len() > 2048
            || self.reading.current_readings.len() > 2048
            || self.reading.output_readings.len() > 2048
            || self.reading.current_contributions.len() > 2048
            || self.reading.intervention_contexts.len() > 2048
            || self
                .reading
                .current_contributions
                .iter()
                .any(|c| !c.overlays.is_empty())
        {
            return Err("lifecycle reading differs from the installed original procedure/native source/DocumentCAS".into());
        }
        nonempty(&self.actor_ref, "lifecycle native actor")?;
        nonempty(&self.operation_ref, "lifecycle operation")?;
        let source = self
            .reading
            .materialization
            .scenes
            .iter()
            .find(|s| s.scene_ref == self.scene_ref)
            .ok_or("lifecycle selected native Scene material reading absent")?;
        if source.document_revision != self.document_revision
            || source.principal.subject_ref != p.principal_subject_ref
            || source.locus != read.locus
            || source.current_presentation.as_ref() != Some(&read.presentation)
        {
            return Err("lifecycle Scene source/principal/locus/current material differs".into());
        }
        validate_native_subject_basis(native_current_m_registry(), &source.principal)?;
        for subject in &source.contributors {
            validate_native_subject_basis(native_current_m_registry(), subject)?;
        }
        match &self.action {
            LifecycleAction::ContributionDetach { contribution_ref } => {
                nonempty(contribution_ref, "detached contribution")?;
            }
            LifecycleAction::ScenePolicy {
                from_scene_ref,
                to_scene_ref,
                ..
            } => {
                let flow = self
                    .reading
                    .current_flow
                    .as_ref()
                    .ok_or("Scene policy requires the actual whole native Atlas reading")?;
                if flow["schema"] != "ql.native-atlas-state/v1"
                    || flow["expression_ref"] != self.expression_ref
                    || from_scene_ref != &self.scene_ref
                {
                    return Err("Scene policy has foreign native world/current selection".into());
                }
                let order = flow["scene_order"]
                    .as_array()
                    .ok_or("native Atlas scene order absent")?;
                let mut seen = BTreeSet::new();
                for scene in order {
                    let scene = scene.as_str().ok_or("native Scene ref is not text")?;
                    if !scene.starts_with(&format!("{}:scene:", self.expression_ref))
                        || !seen.insert(scene)
                    {
                        return Err("native Atlas scene order is foreign/duplicated".into());
                    }
                }
                if !seen.contains(from_scene_ref.as_str()) || !seen.contains(to_scene_ref.as_str())
                {
                    return Err(
                        "Scene policy targets are not actual current canonical places".into(),
                    );
                }
                validate_flow_selection(&flow["focus"], &seen, &self.expression_ref)?;
            }
            LifecycleAction::Retire => {}
        }
        Ok(())
    }

    /// Read-only original subjects for the closed native reader. This is not a
    /// lease or scope grant and may not be populated from arbitrary caller rows.
    pub(crate) fn selected_contributors(&self) -> Result<Vec<NativeSubject>> {
        let scene = self
            .reading
            .materialization
            .scenes
            .iter()
            .find(|s| s.scene_ref == self.scene_ref)
            .ok_or("lifecycle selected Scene absent")?;
        Ok(scene.contributors.clone())
    }
}

fn validate_flow_selection(focus: &Value, scenes: &BTreeSet<&str>, expression: &str) -> Result<()> {
    let object = focus.as_object().ok_or("actual native Selection absent")?;
    if object
        .keys()
        .any(|k| !["scene_ref", "entity_ref", "relation_ref"].contains(&k.as_str()))
        || !object.contains_key("scene_ref")
        || !object.contains_key("entity_ref")
        || ["scene_ref", "entity_ref", "relation_ref"]
            .iter()
            .any(|key| {
                object
                    .get(*key)
                    .is_some_and(|v| !v.is_null() && !v.is_string())
            })
        || (!focus["entity_ref"].is_null() && !focus["relation_ref"].is_null())
    {
        return Err("malformed native Selection/entity/relation conflict".into());
    }
    if let Some(scene) = focus["scene_ref"].as_str() {
        if !scenes.contains(scene) {
            return Err("native Selection points outside current world".into());
        }
    } else if !focus["entity_ref"].is_null() || !focus["relation_ref"].is_null() {
        return Err("native Selection constituent has no Scene".into());
    }
    if focus["entity_ref"]
        .as_str()
        .is_some_and(|r| !r.starts_with(&format!("{expression}:entity:")))
    {
        return Err("native Selection Entity belongs to another world".into());
    }
    Ok(())
}

pub fn contribution_is_suppressed(
    lifecycle: &LifecycleCheckpoint,
    procedure: &Procedure,
    contribution: &GeneratedContribution,
) -> Result<bool> {
    let anchor = crate::procedural_retention::source_scene_anchor(procedure, contribution)?;
    Ok(lifecycle
        .detached_contribution_refs
        .contains(&contribution.contribution_ref)
        || contribution
            .owned_addresses
            .iter()
            .chain(anchor.iter())
            .any(|address| {
                address.scene_ref.as_ref().is_some_and(|scene| {
                    lifecycle.scene_policies.get(scene).is_some_and(|p| {
                        matches!(p.policy, ScenePolicy::Hold | ScenePolicy::CheckpointRelease)
                    })
                })
            }))
}

pub fn active_generation(
    lifecycle: &LifecycleCheckpoint,
    procedure: &Procedure,
    contributions: Vec<GeneratedContribution>,
) -> Result<Vec<GeneratedContribution>> {
    contributions
        .into_iter()
        .filter_map(
            |c| match contribution_is_suppressed(lifecycle, procedure, &c) {
                Ok(true) => None,
                Ok(false) => Some(Ok(c)),
                Err(e) => Some(Err(e)),
            },
        )
        .collect()
}

/// Private ephemeral candidate; transport JSON cannot deserialize/commit it.
/// Source checkpoint mutation is deferred until SAME owner's post-C31 checks.
pub(crate) struct StagedLifecycle {
    original_checkpoint_fingerprint: String,
    procedure_ref: String,
    receipt: Value,
    next: Option<ConductCheckpoint>,
}

impl ConductHost {
    pub(crate) fn commit_lifecycle(&mut self, staged: StagedLifecycle) -> Result<Value> {
        let installed = self
            .rules
            .get_mut(&staged.procedure_ref)
            .ok_or("installed lifecycle disappeared before source postvalidation")?;
        if fingerprint(&installed.checkpoint)? != staged.original_checkpoint_fingerprint {
            return Err(
                "installed lifecycle changed before actual source postvalidation/commit".into(),
            );
        }
        if let Some(next) = staged.next {
            installed.checkpoint = next;
        }
        Ok(staged.receipt)
    }

    /// Read-only installed original definition used by the native receiving
    /// owner to gather actual Document readings; never runtime authority.
    pub fn lifecycle_original(&self, procedure_ref: &str) -> Result<&ConductInstall> {
        self.rules
            .get(procedure_ref)
            .map(|i| &i.checkpoint.definition)
            .ok_or_else(|| "unknown native installed procedure".into())
    }

    /// Clock-free settlement of a REAL native cancelled Operation. Nothing
    /// changes until the SAME owner's post-C31 source checks and commit succeed.
    pub(crate) fn stage_lifecycle_cancel(
        &self,
        input: NativeLifecycleCancel,
    ) -> Result<StagedLifecycle> {
        let installed = self
            .rules
            .get(&input.procedure_ref)
            .ok_or("unknown native installed procedure")?;
        input.validate_reading(&installed.checkpoint.definition)?;
        let digest = fingerprint(&input)?;
        let original_checkpoint_fingerprint = fingerprint(&installed.checkpoint)?;
        if installed.checkpoint.pending_operation_ref.is_none() {
            let old = installed
                .checkpoint
                .events
                .get(&input.operation_ref)
                .ok_or("no original pending or cancelled lifecycle operation")?;
            if old.receipt["status"] != "material_abandoned"
                || old.receipt["cancellation_input_fingerprint"] != digest
                || old.receipt["native_cancelled_record"] != input.native_record
            {
                return Err(
                    "cancelled lifecycle retry differs from exact original settlement".into(),
                );
            }
            return Ok(StagedLifecycle {
                original_checkpoint_fingerprint,
                procedure_ref: input.procedure_ref,
                receipt: old.receipt.clone(),
                next: None,
            });
        }
        validate_checkpoint_lifecycle(&installed.checkpoint)?;
        let pending = installed
            .checkpoint
            .pending_preparation
            .as_ref()
            .ok_or("native lifecycle pending preparation absent")?;
        let lifecycle = installed
            .checkpoint
            .lifecycle
            .pending
            .as_ref()
            .ok_or("pending material is not an original lifecycle operation")?;
        if installed.checkpoint.pending_operation_ref.as_deref() != Some(&input.operation_ref)
            || lifecycle.operation_ref != input.operation_ref
        {
            return Err("cancelled operation differs from SAME pending lifecycle identity".into());
        }
        input.validate_cancelled_configuration(&installed.checkpoint.definition, pending)?;
        let original = pending.clone();
        let original_position = installed.checkpoint.native_position.clone();
        let action = lifecycle.action.clone();
        let mut next = installed.checkpoint.clone();
        // Keep rule counters, original admitted position, source definition,
        // generation/basis/interventions and already settled policies unchanged.
        // Native S Cancel did not apply pending material; never settle_lifecycle.
        next.pending_preparation = None;
        next.pending_operation_ref = None;
        next.lifecycle.pending = None;
        let mut value = receipt(&next, "material_abandoned", None, None, None)?;
        value["original_preparation"] =
            serde_json::to_value(&original).map_err(|e| e.to_string())?;
        value["native_cancelled_record"] = input.native_record;
        value["retained_native_position"] =
            serde_json::to_value(&original_position).map_err(|e| e.to_string())?;
        value["cancellation_input_fingerprint"] = json!(digest);
        value["lifecycle"] = json!({"schema":LIFECYCLE_RECEIPT,"action":action,
            "operation_ref":input.operation_ref,"state":"material_abandoned",
            "document_revision":input.document_revision,"actor_ref":input.actor_ref,
            "source_read_receipt_ref":input.scene_read.source_read_receipt_ref,
            "consumer_release":"unconfirmed"});
        let record = next
            .events
            .get_mut(&input.operation_ref)
            .ok_or("original lifecycle operation history absent")?;
        record.receipt = value.clone();
        bounded(&next)?;
        Ok(StagedLifecycle {
            original_checkpoint_fingerprint,
            procedure_ref: input.procedure_ref,
            receipt: value,
            next: Some(next),
        })
    }

    pub(crate) fn stage_lifecycle(
        &self,
        input: NativeLifecycleInput,
        position: NativePosition,
        witness: &NativeTimingWitness,
    ) -> Result<StagedLifecycle> {
        let installed = self
            .rules
            .get(&input.procedure_ref)
            .ok_or("unknown native installed procedure")?;
        input.validate_reading(&installed.checkpoint.definition)?;
        if witness.original_binding() != &installed.checkpoint.definition.procedure.timing
            || witness.native_position() != &position
        {
            return Err("lifecycle witness differs from exact installed original timing/current native position".into());
        }
        check_position(&installed.checkpoint, &position)?;
        let original_checkpoint_fingerprint = fingerprint(&installed.checkpoint)?;
        let digest = fingerprint(&input)?;
        if let Some(old) = installed.checkpoint.events.get(&input.operation_ref) {
            if old.input_fingerprint != digest {
                return Err(
                    "lifecycle operation reused for another original intent/Document basis".into(),
                );
            }
            return Ok(StagedLifecycle {
                original_checkpoint_fingerprint,
                procedure_ref: input.procedure_ref,
                receipt: old.receipt.clone(),
                next: None,
            });
        }
        if installed.checkpoint.events.len() >= MAX_EVENTS {
            return Err("native lifecycle retained operation bound exceeded".into());
        }
        let mut next = installed.checkpoint.clone();
        if next.pending_operation_ref.is_some() || next.lifecycle.pending.is_some() {
            return Err("lifecycle waits for SAME receiver pending material settlement".into());
        }
        let procedure = &next.definition.procedure;
        let currentness = next.definition.currentness.clone();
        // Native graph re-observation derives currentness. Current Scene origin
        // is pre/post qualified by the private same-owner C source reader.
        qualify_source(&installed.graph, &next.definition, &currentness)?;
        let cause = Cause {
            event_ref: input.operation_ref.clone(),
            ancestors: vec![],
            progress_revision: fingerprint(&(
                input.document_revision,
                &position,
                &input.reading.scene_read.material_fingerprint,
            ))?,
            depth: 1,
        };
        let mut event = ConductEvent {
            procedure_ref: input.procedure_ref.clone(),
            event: RuleEvent {
                kind: TriggerKind::Explicit,
                active: true,
                cause: cause.clone(),
            },
            position: position.clone(),
            document_revision: input.document_revision,
            current_readings: input.reading.current_readings.clone(),
            output_readings: input.reading.output_readings.clone(),
            current_contributions: input.reading.current_contributions.clone(),
            intervention_contexts: input.reading.intervention_contexts.clone(),
            materialization: None,
            currentness: currentness.clone(),
        };
        project_event_interventions(&mut event)?;
        next.membership = resolve_procedure_membership(
            native_current_m_registry(),
            &next.definition.procedure,
            &next.definition.expression_ref,
            &event.current_readings,
            Some(&next.membership),
        )?;
        let mut context = NativeMaterialization {
            schema: MATERIALIZATION_CONTRACT.into(),
            lifecycles: vec![],
            document_revision: input.document_revision,
            rule_cursor: next.rule.evaluations as u64,
            state: if next.rule.position == RulePosition::Running {
                "running"
            } else {
                "held"
            }
            .into(),
            scenes: input.reading.materialization.scenes.clone(),
        };
        let selected = next
            .last_generated
            .iter()
            .filter(|c| match &input.action {
                LifecycleAction::Retire => !next
                    .lifecycle
                    .detached_contribution_refs
                    .contains(&c.contribution_ref),
                LifecycleAction::ContributionDetach { contribution_ref } => {
                    &c.contribution_ref == contribution_ref
                }
                LifecycleAction::ScenePolicy { from_scene_ref, .. } => {
                    c.owned_addresses
                        .iter()
                        .any(|a| a.scene_ref.as_ref() == Some(from_scene_ref))
                        || crate::procedural_retention::source_scene_anchor(procedure, c).is_ok_and(
                            |a| a.is_some_and(|a| a.scene_ref.as_ref() == Some(from_scene_ref)),
                        )
                }
            })
            .cloned()
            .collect::<Vec<_>>();
        if selected.is_empty() {
            return Err(
                "lifecycle has no actual retained owned output for the selected original procedure"
                    .into(),
            );
        }
        let references = selected
            .iter()
            .map(|c| c.contribution_ref.as_str())
            .collect::<BTreeSet<_>>();
        let outputs = event
            .output_readings
            .iter()
            .filter(|r| references.contains(r.contribution_ref.as_str()))
            .cloned()
            .collect::<Vec<_>>();
        let current = event
            .current_contributions
            .iter()
            .filter(|r| references.contains(r.contribution_ref.as_str()))
            .cloned()
            .collect::<Vec<_>>();
        if outputs.len() != selected.len()
            || current.len() != selected.len()
            || event.output_readings.len() != outputs.len()
            || event.current_contributions.len() != current.len()
        {
            return Err("lifecycle requires exact selected SAME owner current output readings; unrelated output capabilities refused".into());
        }
        let mut prepared = if input.action == LifecycleAction::Retire {
            context.state = "retired".into();
            crate::procedural_retention::prepare_native_retirement(
                native_current_m_registry(),
                procedure,
                &input.operation_ref,
                &input.expression_ref,
                input.document_revision,
                next.membership.clone(),
                &selected,
                &current,
                outputs,
                next.definition.required_consumers.clone(),
                &context,
            )?
        } else {
            let mut contributions = selected.clone();
            for c in &mut contributions {
                c.native_changes.clear();
            }
            if let LifecycleAction::ContributionDetach { contribution_ref } = &input.action {
                context.lifecycles.push(NativeContributionLifecycle {
                    contribution_ref: contribution_ref.clone(),
                    status: ContributionLifecycle::Detached,
                });
            }
            let mut prepared = compile_retained_metadata_phase(
                native_current_m_registry(),
                procedure,
                &input.operation_ref,
                &input.expression_ref,
                input.document_revision,
                next.membership.clone(),
                contributions,
                next.definition.required_consumers.clone(),
                &selected,
                &current,
                outputs,
            )?;
            materialize_retention(&mut prepared, &context)?;
            prepared
        };
        let policy = if let LifecycleAction::ScenePolicy {
            from_scene_ref,
            to_scene_ref,
            policy,
        } = &input.action
        {
            let row = SettledScenePolicy {
                from_scene_ref: from_scene_ref.clone(),
                to_scene_ref: to_scene_ref.clone(),
                policy: *policy,
                operation_ref: input.operation_ref.clone(),
                rule_cursor: context.rule_cursor,
                native_position: position.clone(),
                original_timing: procedure.timing.clone(),
            };
            append_scene_policy(&mut prepared, &row)?;
            Some(row)
        } else {
            None
        };
        bind_prepared_timing(&mut prepared, witness)?;
        prepared.qualify_native_cprime(
            native_current_m_registry(),
            procedure,
            &installed.graph,
            currentness,
            next.definition.thread_plan.clone(),
        )?;
        finalize_lifecycle_actor(&mut prepared, &input)?;
        let count = prepared.native_edit["changes"]
            .as_array()
            .ok_or("lifecycle native changes absent")?
            .len();
        if next.rule.evaluations >= procedure.budgets.max_evaluations
            || next
                .rule
                .operations
                .checked_add(count)
                .is_none_or(|n| n > procedure.budgets.max_operations)
        {
            return Err("actual lifecycle evaluation/native operation budget exceeded".into());
        }
        next.rule.cursor = witness.admitted_cursor();
        next.rule.evaluations += 1;
        next.rule.operations += count;
        next.pending_operation_ref = Some(input.operation_ref.clone());
        next.pending_preparation = Some(prepared.clone());
        next.lifecycle.pending = Some(PendingLifecycle {
            operation_ref: input.operation_ref.clone(),
            action: input.action.clone(),
            policy,
        });
        next.native_position = position;
        let mut result = receipt(
            &next,
            "prepared",
            Some(
                "lifecycle awaits SAME protected material settlement; consumer release/ACK remains separate",
            ),
            Some(&prepared),
            Some(&cause),
        )?;
        result["lifecycle"] = json!({"schema":LIFECYCLE_RECEIPT,"action":input.action,"operation_ref":input.operation_ref,
            "state":"pending_material","actor_ref":input.actor_ref,"document_revision":input.document_revision,
            "source_read_receipt_ref":input.reading.scene_read.source_read_receipt_ref,"consumer_release":"unconfirmed",
            "operation_kind": if matches!(&input.action,LifecycleAction::ScenePolicy{..}) {"background_policy_configuration"} else {"contribution_lifecycle"}});
        next.events.insert(
            input.operation_ref,
            RecordedConduct {
                input_fingerprint: digest,
                receipt: result.clone(),
            },
        );
        bounded(&next)?;
        Ok(StagedLifecycle {
            original_checkpoint_fingerprint,
            procedure_ref: input.procedure_ref,
            receipt: result,
            next: Some(next),
        })
    }
}

fn sealed_scene_policy_row(procedure_ref: &str, row: &SettledScenePolicy) -> Value {
    json!({"from_scene_ref":row.from_scene_ref,"to_scene_ref":row.to_scene_ref,"policy":row.policy,
        "cursor":row.rule_cursor,"procedure_ref":procedure_ref,"operation_ref":row.operation_ref,
        "original_timing":row.original_timing,"native_position":row.native_position})
}

fn append_scene_policy(prepared: &mut PreparedProcedure, row: &SettledScenePolicy) -> Result<()> {
    let changes = prepared.native_edit["changes"]
        .as_array_mut()
        .ok_or("Scene policy native Edit absent")?;
    let mut changed = false;
    for change in changes {
        if change["change"] != "scene_material_set" || change["scene_ref"] != row.from_scene_ref {
            continue;
        }
        let rows = change["presentation"]["scene"]["procedural"]["scene_flow"]
            .as_array_mut()
            .ok_or("Scene policy retained array absent")?;
        let value = sealed_scene_policy_row(&prepared.procedure_ref, row);
        let indices = rows
            .iter()
            .enumerate()
            .filter(|(_, r)| {
                r["from_scene_ref"] == row.from_scene_ref
                    && r["procedure_ref"] == prepared.procedure_ref
            })
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        if indices.len() > 1 {
            return Err("duplicate original procedure Scene policy rows".into());
        }
        if let Some(i) = indices.first() {
            rows[*i] = value;
        } else {
            if rows.len() >= 64 {
                return Err("native Scene flow policy bound exceeded".into());
            }
            rows.push(value);
        }
        changed = true;
    }
    if !changed {
        return Err("Scene policy requires SAME owned Scene metadata target; canonical source anchor is not a new location grant".into());
    }
    prepared.fingerprint.clear();
    prepared.fingerprint = fingerprint(prepared)?;
    Ok(())
}

/// Qualification is completed with the unchanged original authored actor.
/// This final native lifecycle operation uses the actual requesting actor;
/// receiving S seals both the original definition and this complete final Edit.
fn finalize_lifecycle_actor(
    prepared: &mut PreparedProcedure,
    input: &NativeLifecycleInput,
) -> Result<()> {
    let mut original = prepared.clone();
    original.fingerprint.clear();
    if prepared.native_cprime.is_none()
        || fingerprint(&original)? != prepared.fingerprint
        || prepared.native_edit["actor"] != prepared.original_procedure.composition.actor
        || prepared.operation_ref != input.operation_ref
        || prepared.procedure_ref != input.procedure_ref
        || prepared.original_procedure.revision != input.expected_procedure_revision
        || prepared.native_edit["expression_ref"] != input.expression_ref
        || prepared.expected_document_revision != input.document_revision
        || prepared.native_edit["expected_revision"] != input.document_revision
        || input.reading.scene_read.source_basis != prepared.original_procedure.profile
        || input.reading.scene_read.locus.reference != prepared.original_procedure.locus_ref
    {
        return Err(
            "lifecycle actor finalization requires exact originally qualified native preparation"
                .into(),
        );
    }
    prepared.native_edit["actor"] = json!(input.actor_ref);
    prepared.lifecycle_intent = Some(
        json!({"schema":LIFECYCLE_INTENT,"expression_ref":input.expression_ref,
        "document_revision":input.document_revision,"scene_ref":input.scene_ref,"operation_ref":input.operation_ref,
        "actor_ref":input.actor_ref,"procedure_ref":input.procedure_ref,"expected_procedure_revision":input.expected_procedure_revision,
        "action":input.action,"source_read_receipt_ref":input.reading.scene_read.source_read_receipt_ref}),
    );
    prepared.fingerprint.clear();
    prepared.fingerprint = fingerprint(prepared)?;
    Ok(())
}

/// Validate retained Source lifecycle identity against the exact original
/// procedure and immutable pending preparation. This is a pure configuration
/// check; restore still requires the protected native checkpoint/owner witness.
pub fn validate_checkpoint_lifecycle(checkpoint: &ConductCheckpoint) -> Result<()> {
    let state = &checkpoint.lifecycle;
    let p = &checkpoint.definition.procedure;
    if state.detached_contribution_refs.len() > 2048
        || state.scene_policies.len() > 64
        || state
            .pending
            .as_ref()
            .map(|v| v.operation_ref.as_str())
            .is_some_and(|r| checkpoint.pending_operation_ref.as_deref() != Some(r))
    {
        return Err("retained lifecycle bound/pending operation mismatch".into());
    }
    for (scene, row) in &state.scene_policies {
        if scene != &row.from_scene_ref
            || row.original_timing != p.timing
            || row.native_position.instance_ref != checkpoint.native_position.instance_ref
            || row.native_position.subject_ref != checkpoint.native_position.subject_ref
            || row.native_position.event_ref != checkpoint.native_position.event_ref
            || !row
                .from_scene_ref
                .starts_with(&format!("{}:scene:", checkpoint.definition.expression_ref))
            || !row
                .to_scene_ref
                .starts_with(&format!("{}:scene:", checkpoint.definition.expression_ref))
            || row.native_position.cursor()? > checkpoint.native_position.cursor()?
        {
            return Err(
                "retained Scene policy has foreign original native instance/source/timing/place"
                    .into(),
            );
        }
    }
    if let Some(pending) = &state.pending {
        let prepared = checkpoint
            .pending_preparation
            .as_ref()
            .ok_or("pending lifecycle native preparation absent")?;
        let intent = prepared
            .lifecycle_intent
            .as_ref()
            .ok_or("pending lifecycle original sealed intent absent")?;
        if intent["operation_ref"] != pending.operation_ref
            || intent["action"]
                != serde_json::to_value(&pending.action).map_err(|e| e.to_string())?
            || intent["procedure_ref"] != p.procedure_ref
            || intent["expected_procedure_revision"] != p.revision
            || prepared.original_procedure != *p
        {
            return Err("pending lifecycle differs from exact sealed original preparation".into());
        }
        match (&pending.action, &pending.policy) {
            (
                LifecycleAction::ScenePolicy {
                    from_scene_ref,
                    to_scene_ref,
                    policy,
                },
                Some(row),
            ) => {
                let sealed = sealed_scene_policy_row(&p.procedure_ref, row);
                let matches = prepared.native_edit["changes"]
                    .as_array()
                    .ok_or("pending policy native Edit absent")?
                    .iter()
                    .filter(|c| {
                        c["change"] == "scene_material_set" && c["scene_ref"] == *from_scene_ref
                    })
                    .flat_map(|c| {
                        c["presentation"]["scene"]["procedural"]["scene_flow"]
                            .as_array()
                            .into_iter()
                            .flatten()
                    })
                    .filter(|r| **r == sealed)
                    .count();
                if matches != 1
                    || row.from_scene_ref != *from_scene_ref
                    || row.to_scene_ref != *to_scene_ref
                    || row.policy != *policy
                    || row.operation_ref != pending.operation_ref
                    || row.original_timing != p.timing
                {
                    return Err("pending Scene policy differs from exact sealed native metadata row/original timing".into());
                }
            }
            (LifecycleAction::ScenePolicy { .. }, None) => {
                return Err("pending Scene policy sealed native position absent".into());
            }
            (_, Some(_)) => {
                return Err("non-policy lifecycle carries a foreign pending Scene policy".into());
            }
            (_, None) => {}
        }
    }
    Ok(())
}

pub(super) fn settle_lifecycle(checkpoint: &mut ConductCheckpoint) -> Result<()> {
    validate_checkpoint_lifecycle(checkpoint)?;
    let Some(pending) = checkpoint.lifecycle.pending.clone() else {
        return Ok(());
    };
    if checkpoint.pending_operation_ref.as_deref() != Some(&pending.operation_ref) {
        return Err(
            "lifecycle pending identity differs from actual protected material settlement".into(),
        );
    }
    match pending.action {
        LifecycleAction::Retire => {
            checkpoint.rule.cancel(checkpoint.rule.cursor);
            checkpoint.last_generated.clear();
        }
        LifecycleAction::ContributionDetach { contribution_ref } => {
            checkpoint
                .last_generated
                .retain(|c| c.contribution_ref != contribution_ref);
            checkpoint
                .lifecycle
                .detached_contribution_refs
                .insert(contribution_ref);
        }
        LifecycleAction::ScenePolicy {
            from_scene_ref,
            policy,
            ..
        } => {
            let row = pending
                .policy
                .ok_or("settled Scene policy position absent")?;
            if row.from_scene_ref != from_scene_ref || row.policy != policy {
                return Err("settled Scene policy differs from exact pending intent".into());
            }
            // Continue never resumes a globally paused rule. A checkpoint
            // release preserves Source state and records policy suppression;
            // the existing physical owner separately reports genuine release.
            if policy == ScenePolicy::Continue {
                checkpoint.lifecycle.scene_policies.remove(&from_scene_ref);
            } else {
                checkpoint
                    .lifecycle
                    .scene_policies
                    .insert(from_scene_ref, row);
            }
        }
    }
    checkpoint.lifecycle.pending = None;
    Ok(())
}

#[cfg(test)]
mod source_metadata_tests {
    use super::*;
    use crate::m_tree;
    use crate::procedural_manifestation::{NativeReading, ReadingAvailability, SubjectRole};
    use serde_json::{Value, json};
    use std::collections::{BTreeMap, BTreeSet};
    fn subject() -> NativeSubject {
        let manifest = m_tree::native_current_m_registry().manifest();
        NativeSubject {
            subject_ref: "ql:m-coordinate:bimba:M3".into(),
            native_owner: "ql-mef".into(),
            presentation_role: SubjectRole::Thing,
            sources: vec![NativeReading {
                reference: format!(
                    "{}:{}",
                    manifest.source_repository, manifest.source_dataset_tree
                ),
                revision: manifest.source_snapshot_sha256.clone(),
                availability: ReadingAvailability::Available,
            }],
            readings: vec![],
            actions: vec![],
        }
    }

    fn procedure() -> Procedure {
        let registry = m_tree::native_current_m_registry();
        Procedure { schema: PROCEDURE_CONTRACT.into(), procedure_ref: "procedure:source-native-passage".into(), revision: "1".into(),
        recipe: SourceBasis { source_ref: "docs/integrations/epi-logos/TA-ONTA-PROCEDURAL-EXPRESSION-CONTRACTS.md#2.3".into(),
            revision:fingerprint(&include_str!("../../../docs/integrations/epi-logos/TA-ONTA-PROCEDURAL-EXPRESSION-CONTRACTS.md")).unwrap() },
        profile: SourceBasis { source_ref: "profile:source-native-passage".into(), revision: "1".into() },
        registry_revision: registry.manifest().registry_revision.clone(), principal_subject_ref: subject().subject_ref,
        locus_ref: "ql:m-coordinate:bimba:M3".into(), occurrence_ref: "expression:acceptance".into(),
        composition: serde_json::from_value(json!({"CPF":"dialogical","CT":"CT2","CP":"4.2","CF":"CF3","CFP":"CFP1","CS":"CS0",
            "direction":"forward","actor":"agent:anima","interpretation":{"ref":"ql:interpretation:c-prime","revision":"1"},
            "whole":"expression:acceptance","resolvePath":"aikit:resolve:procedural-stage","contextResolution":"aikit:context:procedural-stage",
            "sources":["source:accepted-procedural-stage"]})).unwrap(),
        trigger: Trigger { kind: TriggerKind::Explicit, mode: TriggerMode::Edge }, selector: Selector::All,
        conditions: vec![], recipe_parameters: BTreeMap::from([("source_treatment".into(),json!("glyph_mask"))]),
        membership_mode: MembershipMode::Frozen, membership_change_policy: MembershipChangePolicy::AdmitAndRecord,
        timing: TimingBinding { owner_ref: "native:expression".into(), domain: "simulation".into(), epoch_ref: "epoch:native-1".into(),
            requested_cursor: 42, time_mapping_ref: None }, budgets: ExecutionBudget { max_evaluations: 32, max_operations: 256,
            max_expansion_depth: 8, max_active_instances: 16, max_queue: 16 }, seed: "source-passage-20261002".into(),
        seed_algorithm: SEED_ALGORITHM.into(), removal_policy: RemovalPolicy::RetireUneditedDetachEdited,
        failure_policy: FailurePolicy::StopAffectedAndCheckpoint, continuation_policy: ProcedureContinuation::Continue,
        admitted_changes: ["scene_create","scene_remove","entity_add","entity_remove","subject_bind","scene_compose","scene_material_set","parameter_set","focus"].into_iter().map(str::to_owned).collect() }
    }

    fn presentation() -> Value {
        serde_json::from_str(include_str!(
            "../../../fixtures/kernel/procedural-scene-template-v1.json"
        ))
        .unwrap()
    }
    fn position() -> NativePosition {
        NativePosition {
            instance_ref: "native:existing-owner".into(),
            event_ref: "native:actual-event".into(),
            subject_ref: "person:existing-owner".into(),
            generation: "7".into(),
            samples_elapsed: "1024".into(),
        }
    }
    fn library(choice: LibraryChoice) -> LibraryBuild {
        let mut p = procedure();
        let pos = position();
        p.timing = TimingBinding {
            owner_ref: pos.instance_ref.clone(),
            epoch_ref: pos.instance_ref,
            domain: "native_samples".into(),
            requested_cursor: 1024,
            time_mapping_ref: None,
        };
        let mut source = presentation();
        source["scene"]["id"] = json!("expression:acceptance:scene:canonical");
        LibraryBuild {
            schema: LIBRARY_CONTRACT.into(),
            source: NativeSceneSource {
                native_owner: "oi.expression".into(),
                expression_ref: "expression:acceptance".into(),
                scene_ref: "expression:acceptance:scene:canonical".into(),
                document_revision: 3,
                source_basis: p.profile.clone(),
                material_fingerprint: fingerprint(&source).unwrap(),
                presentation: source,
                principal: subject(),
                contributors: vec![],
                locus_ref: p.locus_ref.clone(),
                locus_revision: "current-native-profile".into(),
            },
            authored: ProcedureAuthorship {
                procedure_ref: p.procedure_ref,
                revision: p.revision,
                composition: p.composition,
                trigger: Trigger {
                    kind: TriggerKind::Explicit,
                    mode: TriggerMode::Level,
                },
                selector: p.selector,
                conditions: p.conditions,
                recipe_parameters: p.recipe_parameters,
                membership_mode: p.membership_mode,
                membership_change_policy: p.membership_change_policy,
                timing: p.timing,
                budgets: p.budgets,
                seed: p.seed,
                removal_policy: p.removal_policy,
                failure_policy: p.failure_policy,
                continuation_policy: p.continuation_policy,
            },
            choice,
        }
    }
    fn output() -> Vec<OutputSlot> {
        vec![OutputSlot {
            output_slot: "native-passage".into(),
            scene_ref: "expression:acceptance:scene:passage".into(),
        }]
    }
    fn definition() -> ConductInstall {
        use crate::vak_scope::OperativeScopeCorrelation;
        use crate::vak_scope_wire::{
            OPERATIVE_CURRENTNESS_CONTRACT, OperativeScopeCurrentnessRequest,
        };
        let built =
            library_build(library(LibraryChoice::SceneMaterial { outputs: output() })).unwrap();
        let mut p: Procedure = serde_json::from_value(built["procedure"].clone()).unwrap();
        p.composition.thread = crate::vak_profile::ThreadForm::Single;
        p.composition.sources = vec![p.recipe.source_ref.clone(), p.profile.source_ref.clone()];
        let mut whole: Value = serde_json::from_str::<Value>(include_str!(
            "../../../fixtures/kernel/vak-composition-v1.json"
        ))
        .unwrap()["steps"][0]
            .clone();
        whole["useRef"] = json!("whole:procedural-source-ground");
        whole["subjectRef"] = json!(p.principal_subject_ref);
        whole["frame"]["id"] = json!(p.composition.frame.0.code());
        whole["basis"] = json!({"caller":p.composition.actor,"source":p.recipe.source_ref,"revision":p.recipe.revision,"standing":crate::VakStanding::AuthoredArchitecture.as_schema_str(),"evidence":[p.recipe.source_ref]});
        let source = json!({"contract":"ql.vak-composition/v1","steps":[whole,{"op":"reframe","from":"whole:procedural-source-ground","into":p.composition.whole,
      "frame":{"id":p.composition.frame.0.code(),"lens":"L0","basis":"chromatic","face":"direct","positions":"local"},
      "basis":{"caller":p.composition.actor,"source":p.profile.source_ref,"revision":p.profile.revision,"standing":crate::VakStanding::AuthoredArchitecture.as_schema_str(),"evidence":[p.profile.source_ref]}}]});
        let (_, graph) = crate::vak_composition_wire::compile_request(&source).unwrap();
        let correlation = OperativeScopeCorrelation {
            world_ref: "world:acceptance".into(),
            world_generation: "generation:current".into(),
            method_skill_ref: None,
        };
        let binding = graph
            .bind_operative_scope(
                &p.composition.whole,
                p.composition.profile(),
                correlation.clone(),
            )
            .unwrap();
        p.composition.interpretation.reference = binding.binding_ref.clone();
        p.composition.interpretation.revision = binding.binding_revision.clone();
        ConductInstall {schema:CONDUCT_CONTRACT.into(),program:serde_json::from_value(built["program"].clone()).unwrap(),expression_ref:p.occurrence_ref.clone(),document_revision:3,current_readings:vec![],source_composition:source,
      currentness:OperativeScopeCurrentnessRequest{contract:OPERATIVE_CURRENTNESS_CONTRACT.into(),expected:binding,current_whole_ref:p.composition.whole.clone(),correlation},
      thread_plan:serde_json::from_value(json!({"legs":[{"unit_ref":"oi.expression:native-procedure-leg","subject_ref":p.principal_subject_ref,"scope_ref":p.occurrence_ref,"input_refs":[p.recipe.source_ref],"result_ref":"oi.expression:native-procedure-result","after":[],"parent":null}],"aggregation_ref":null,"continuation_ref":null,"stop_condition_ref":null})).unwrap(),procedure:p,required_consumers:BTreeSet::from(["scene".into(),"nativeBody".into(),"audio".into()]),interval_ref:"native:source-interval".into(),materialization:None}
    }

    fn prepared_native_configuration() -> (ConductInstall, PreparedProcedure) {
        let definition = definition();
        let registry = m_tree::native_current_m_registry();
        let membership = resolve_procedure_membership(
            registry,
            &definition.procedure,
            &definition.expression_ref,
            &definition.current_readings,
            None,
        )
        .unwrap();
        let generated = program_contributions(
            registry,
            &definition.procedure,
            &definition.program,
            &definition.expression_ref,
            &definition.current_readings,
            &membership,
        )
        .unwrap();
        let mut prepared = compile_native_batch(
            registry,
            &definition.procedure,
            "operation:source-lifecycle-test",
            &definition.expression_ref,
            3,
            membership,
            generated,
            definition.required_consumers.clone(),
        )
        .unwrap();
        let mut context = program_materialization(&definition.program, 3, 0, "held")
            .unwrap()
            .unwrap();
        let NativeRecipeProgram::SceneMaterial { outputs } = &definition.program else {
            panic!("actual source material recipe expected")
        };
        let source = &outputs[0].source;
        context.scenes.push(NativeRetentionScene {
            scene_ref: source.scene_ref.clone(),
            document_revision: 3,
            existing_retention: Some(crate::procedural_retention::empty_retention()),
            current_presentation: Some(source.presentation.clone()),
            principal: source.principal.clone(),
            contributors: source.contributors.clone(),
            locus: crate::procedural_manifestation::NativeReading {
                reference: source.locus_ref.clone(),
                revision: source.locus_revision.clone(),
                availability: crate::procedural_manifestation::ReadingAvailability::Available,
            },
        });
        materialize_retention(&mut prepared, &context).unwrap();
        (definition, prepared)
    }
    #[test]
    fn source_background_policy_changes_actual_metadata_without_focus_or_material_replacement() {
        let (definition, mut prepared) = prepared_native_configuration();
        let before = prepared.native_edit.clone();
        let NativeRecipeProgram::SceneMaterial { outputs } = &definition.program else {
            panic!("actual source recipe expected")
        };
        let row = SettledScenePolicy {
            from_scene_ref: outputs[0].source.scene_ref.clone(),
            to_scene_ref: outputs[0].scene_ref.clone(),
            policy: ScenePolicy::Hold,
            operation_ref: prepared.operation_ref.clone(),
            rule_cursor: 0,
            native_position: position(),
            original_timing: definition.procedure.timing.clone(),
        };
        append_scene_policy(&mut prepared, &row).unwrap();
        let changes = prepared.native_edit["changes"].as_array().unwrap();
        assert_eq!(changes.len(), before["changes"].as_array().unwrap().len());
        assert!(changes.iter().all(|c| c["change"] != "focus"
            && c["change"] != "relation_focus"
            && c["change"] != "scene_reorder"));
        let mut restored = prepared.native_edit.clone();
        for change in restored["changes"].as_array_mut().unwrap() {
            if change["change"] == "scene_material_set" && change["scene_ref"] == row.from_scene_ref
            {
                change["presentation"]["scene"]["procedural"]["scene_flow"] = json!([]);
            }
        }
        assert_eq!(restored, before);
        assert_eq!(prepared.original_procedure, definition.procedure);
        assert!(prepared.membership.targets.is_empty());
        assert_eq!(prepared.metadata_scope.len(), 1);
        assert_eq!(
            prepared.metadata_scope[0].scene_ref.as_ref(),
            Some(&row.from_scene_ref)
        );
    }
    #[test]
    fn source_lifecycle_actor_is_attributed_after_actual_native_cprime_without_relabelling_original()
     {
        let (definition, mut prepared) = prepared_native_configuration();
        let (_, graph) =
            crate::vak_composition_wire::compile_request(&definition.source_composition).unwrap();
        prepared
            .qualify_native_cprime(
                m_tree::native_current_m_registry(),
                &definition.procedure,
                &graph,
                definition.currentness.clone(),
                definition.thread_plan.clone(),
            )
            .unwrap();
        let original_cprime = prepared.native_cprime.clone();
        let old_fingerprint = prepared.fingerprint.clone();
        let NativeRecipeProgram::SceneMaterial { outputs } = &definition.program else {
            panic!("actual source recipe expected")
        };
        let source = &outputs[0].source;
        let locus = crate::procedural_manifestation::NativeReading {
            reference: source.locus_ref.clone(),
            revision: source.locus_revision.clone(),
            availability: crate::procedural_manifestation::ReadingAvailability::Available,
        };
        let input = NativeLifecycleInput {
            schema: LIFECYCLE_INTENT.into(),
            expression_ref: definition.expression_ref.clone(),
            document_revision: 3,
            scene_ref: source.scene_ref.clone(),
            operation_ref: prepared.operation_ref.clone(),
            actor_ref: "human:stage-author".into(),
            procedure_ref: definition.procedure.procedure_ref.clone(),
            expected_procedure_revision: definition.procedure.revision.clone(),
            action: LifecycleAction::Retire,
            reading: NativeLifecycleReading {
                schema: LIFECYCLE_READING.into(),
                scene_read: NativeBootstrapSceneRead {
                    native_owner: source.native_owner.clone(),
                    expression_ref: source.expression_ref.clone(),
                    scene_ref: source.scene_ref.clone(),
                    document_revision: 3,
                    source_basis: source.source_basis.clone(),
                    material_fingerprint: fingerprint(&source.presentation).unwrap(),
                    presentation: source.presentation.clone(),
                    locus: locus.clone(),
                    source_read_receipt_ref: "test:pure-source-configuration".into(),
                },
                current_readings: vec![],
                output_readings: vec![],
                current_contributions: vec![],
                intervention_contexts: vec![],
                materialization: NativeLifecycleMaterialization {
                    schema: LIFECYCLE_MATERIALIZATION.into(),
                    document_revision: 3,
                    scenes: vec![],
                },
                current_flow: None,
            },
        };
        let mut foreign = input.clone();
        foreign.procedure_ref = "procedure:foreign".into();
        let before = prepared.clone();
        assert!(finalize_lifecycle_actor(&mut prepared, &foreign).is_err());
        assert_eq!(prepared, before);
        finalize_lifecycle_actor(&mut prepared, &input).unwrap();
        assert_eq!(prepared.native_edit["actor"], input.actor_ref);
        assert_eq!(prepared.original_procedure, definition.procedure);
        assert_eq!(prepared.native_cprime, original_cprime);
        assert_eq!(prepared.original_procedure.composition.actor, "agent:anima");
        assert_ne!(prepared.fingerprint, old_fingerprint);
        let mut normalized = prepared.clone();
        normalized.fingerprint.clear();
        assert_eq!(fingerprint(&normalized).unwrap(), prepared.fingerprint);
        assert_eq!(
            prepared.lifecycle_intent.as_ref().unwrap()["operation_ref"],
            input.operation_ref
        );
        // Native C31/timing/receiving/body/audio remain separate genuine gates.
    }
    // Independent additive test for procedural_lifecycle::source_metadata_tests.
    // Uses the real native recipe/material/C-prime helpers in that module.
    // This is retained configuration validation only: no installed ConductHost,
    // private native origin, timing witness, material ACK, body or audio is issued.
    #[test]
    fn retained_pending_background_policy_matches_the_entire_sealed_source_row() {
        let (definition, mut prepared) = prepared_native_configuration();
        let NativeRecipeProgram::SceneMaterial { outputs } = &definition.program else {
            panic!("actual native source material recipe required")
        };
        let source = &outputs[0].source;
        let row = SettledScenePolicy {
            from_scene_ref: source.scene_ref.clone(),
            to_scene_ref: outputs[0].scene_ref.clone(),
            policy: ScenePolicy::Hold,
            operation_ref: prepared.operation_ref.clone(),
            rule_cursor: 0,
            native_position: position(),
            original_timing: definition.procedure.timing.clone(),
        };
        append_scene_policy(&mut prepared, &row).unwrap();
        let (_, graph) =
            crate::vak_composition_wire::compile_request(&definition.source_composition).unwrap();
        prepared
            .qualify_native_cprime(
                m_tree::native_current_m_registry(),
                &definition.procedure,
                &graph,
                definition.currentness.clone(),
                definition.thread_plan.clone(),
            )
            .unwrap();
        let input = NativeLifecycleInput {
            schema: LIFECYCLE_INTENT.into(),
            expression_ref: definition.expression_ref.clone(),
            document_revision: 3,
            scene_ref: source.scene_ref.clone(),
            operation_ref: prepared.operation_ref.clone(),
            actor_ref: "human:stage-author".into(),
            procedure_ref: definition.procedure.procedure_ref.clone(),
            expected_procedure_revision: definition.procedure.revision.clone(),
            action: LifecycleAction::ScenePolicy {
                from_scene_ref: row.from_scene_ref.clone(),
                to_scene_ref: row.to_scene_ref.clone(),
                policy: row.policy,
            },
            reading: NativeLifecycleReading {
                schema: LIFECYCLE_READING.into(),
                scene_read: NativeBootstrapSceneRead {
                    native_owner: source.native_owner.clone(),
                    expression_ref: source.expression_ref.clone(),
                    scene_ref: source.scene_ref.clone(),
                    document_revision: 3,
                    source_basis: source.source_basis.clone(),
                    material_fingerprint: fingerprint(&source.presentation).unwrap(),
                    presentation: source.presentation.clone(),
                    locus: NativeReading {
                        reference: source.locus_ref.clone(),
                        revision: source.locus_revision.clone(),
                        availability: ReadingAvailability::Available,
                    },
                    source_read_receipt_ref: "test:pure-source-configuration".into(),
                },
                current_readings: vec![],
                output_readings: vec![],
                current_contributions: vec![],
                intervention_contexts: vec![],
                materialization: NativeLifecycleMaterialization {
                    schema: LIFECYCLE_MATERIALIZATION.into(),
                    document_revision: 3,
                    scenes: vec![],
                },
                current_flow: None,
            },
        };
        finalize_lifecycle_actor(&mut prepared, &input).unwrap();
        let original_policy_rows = prepared.native_edit["changes"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["change"] == "scene_material_set" && c["scene_ref"] == row.from_scene_ref)
            .unwrap()["presentation"]["scene"]["procedural"]["scene_flow"]
            .clone();
        assert_eq!(original_policy_rows.as_array().unwrap().len(), 1);
        assert_eq!(original_policy_rows[0]["to_scene_ref"], row.to_scene_ref);
        let checkpoint = ConductCheckpoint {
            original_source: None,
            original_consumer_contract: None,
            schema: CONDUCT_CONTRACT.into(),
            rule: RuleExecution::new(&definition.procedure, &definition.interval_ref).unwrap(),
            membership: prepared.membership.clone(),
            last_generated: prepared.contributions.clone(),
            pending_operation_ref: Some(prepared.operation_ref.clone()),
            pending_preparation: Some(prepared),
            native_position: position(),
            events: BTreeMap::new(),
            definition,
            lifecycle: LifecycleCheckpoint {
                detached_contribution_refs: BTreeSet::new(),
                scene_policies: BTreeMap::new(),
                pending: Some(PendingLifecycle {
                    operation_ref: row.operation_ref.clone(),
                    action: input.action,
                    policy: Some(row),
                }),
            },
        };
        validate_checkpoint_lifecycle(&checkpoint).unwrap();
        for mutation in 0..7 {
            let mut changed = checkpoint.clone();
            let pending = changed.lifecycle.pending.as_mut().unwrap();
            if mutation == 6 {
                pending.policy = None;
            } else {
                let policy = pending.policy.as_mut().unwrap();
                match mutation {
                    0 => policy.to_scene_ref = "expression:acceptance:scene:unsealed".into(),
                    1 => policy.operation_ref = "operation:unsealed-policy".into(),
                    2 => policy.rule_cursor += 1,
                    3 => policy.native_position.generation = "8".into(),
                    4 => policy.original_timing.epoch_ref = "epoch:unsealed".into(),
                    _ => policy.native_position.samples_elapsed = "2048".into(),
                }
            }
            let before = serde_json::to_value(&changed).unwrap();
            assert!(
                validate_checkpoint_lifecycle(&changed).is_err(),
                "pending policy mutation {mutation} must refuse before retained settlement"
            );
            assert_eq!(serde_json::to_value(&changed).unwrap(), before);
            assert_eq!(
                changed.pending_preparation.as_ref().unwrap().native_edit["changes"],
                checkpoint.pending_preparation.as_ref().unwrap().native_edit["changes"]
            );
        }
    }
}
