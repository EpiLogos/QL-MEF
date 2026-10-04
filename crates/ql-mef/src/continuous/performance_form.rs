//! Source-owned Form/material continuation of the existing physical instrument.
//! Preparation is pure. Only the current private Scene/Act lease and the SAME
//! stopped native body transaction can publish the prepared source descendants.
use super::*;
#[path = "performance_physical_return.rs"]
mod scene_return;
use crate::continuous::performance_receiving::{
    NativePerformanceReceivingSource, PreparedCurrentReceiving, ReceivingImplementation,
};
use crate::m3_state::{COMMAND_SCHEMA, M3Command, M3Operation, M3Receipt};
use crate::physical_body::{
    FormTransitionPolicy, PhysicalMaterial, PreparedFormTransition, prepare_form_transition,
    prepare_material_update,
};

pub const PHYSICAL_EDIT: &str = "ql.native-physical-edit/v1";
pub const PHYSICAL_SOURCE_TRANSITION: &str = "ql.native-physical-source-transition/v1";

/// Authored operations/magnitudes only. Event, generation, body revision,
/// physical face, queue ordinal and audio cursor come from the retained owners.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum AuthoredNativePhysicalEdit {
    Form {
        actor_ref: String,
        cause_ref: String,
        occurrence_unix_ms: u64,
        receipt_unix_ms: u64,
        operations: Vec<M3Operation>,
    },
    Material {
        cause_ref: String,
        material: PhysicalMaterial,
    },
    MetricForm {
        cause_ref: String,
        recipe: SourceGeometryRecipe,
    },
}
impl AuthoredNativePhysicalEdit {
    fn kind(&self) -> &'static str {
        match self {
            Self::Material { .. } => "material",
            Self::Form { .. } | Self::MetricForm { .. } => "form",
        }
    }
    fn cause(&self) -> &str {
        match self {
            Self::Form { cause_ref, .. }
            | Self::Material { cause_ref, .. }
            | Self::MetricForm { cause_ref, .. } => cause_ref,
        }
    }
}

/// Original native production plus its actual accepted application. This is
/// evidence, never a deserializable permission or a new body/clock owner.
pub(crate) struct NativePhysicalTransitionRecord {
    source: Value,
    application: Value,
}
impl NativePhysicalTransitionRecord {
    pub(crate) fn snapshot(&self) -> Value {
        json!({"source":self.source,"native_application":self.application})
    }
}

/// Pure source production shared by live preparation and original-owner cold
/// replay. It contains no native readback, installed P or permission token.
struct PreparedPhysicalSourceDescendant {
    after_current: CoupledBasis,
    after_owner: PerformanceOwner,
    current_receiving: PreparedCurrentReceiving,
    transition: PreparedFormTransition,
    source_record: Value,
    before_acoustic: Option<Value>,
    after_acoustic: Option<Value>,
}

/// One concrete candidate over the original owner. No Deserialize/Clone: the
/// original input and binding cannot be selected by an imported JSON record.
pub struct PreparedNativePhysicalEdit {
    before_assets: Value,
    before_boundary: Value,
    before_current: CoupledBasis,
    after_current: CoupledBasis,
    after_owner: PerformanceOwner,
    current_receiving: PreparedCurrentReceiving,
    transition: PreparedFormTransition,
    authored: AuthoredNativePhysicalEdit,
    source_record: Value,
    original_request_id: u64,
    cursor: u64,
    before_acoustic: Option<Value>,
    after_acoustic: Option<Value>,
}
impl PreparedNativePhysicalEdit {
    pub fn before_source_assets(&self) -> &Value {
        &self.before_assets
    }
    pub fn source_assets(&self) -> &Value {
        self.after_owner.source_assets()
    }
    pub fn native_boundary(&self) -> &Value {
        &self.before_boundary
    }
    pub fn original_request_id(&self) -> u64 {
        self.original_request_id
    }
    pub fn native_sample(&self) -> u64 {
        self.cursor
    }
    pub fn source_record(&self) -> &Value {
        &self.source_record
    }
    pub fn after_current(&self) -> &CoupledBasis {
        &self.after_current
    }
    pub fn binding(&self) -> &PreparedPerformanceBinding {
        self.after_owner.binding()
    }
    pub fn transition(&self) -> &PreparedFormTransition {
        &self.transition
    }
    pub fn receiving(&self) -> &PreparedCurrentReceiving {
        &self.current_receiving
    }
}

/// Replay every admitted command; a constructor snapshot/generation alone
/// cannot establish that the current native Form is a legitimate descendant.
fn replay(current: &CoupledBasis) -> Result<M3State, String> {
    if serde_json::to_value(current.input.compose()?).map_err(|e| e.to_string())?
        != serde_json::to_value(current).map_err(|e| e.to_string())?
    {
        return Err("physical source current basis does not replay completely".into());
    }
    let mut state = M3State::new(current.input.m3.clone())?;
    for command in &current.input.m3_commands {
        if state.apply(command.clone())?.status != "applied" {
            return Err("physical source contains an unadmitted M3 descendant".into());
        }
    }
    if state.snapshot() != current.m3 {
        return Err("physical source lost the complete current M3 reading".into());
    }
    Ok(state)
}

/// Complete constructor identity and M1/M2/MEF inputs are immutable in this
/// operation. Only an exact prefix followed by actual typed M3 commands is a
/// Form descendant; material edits cannot rewrite that prefix either.
pub(crate) fn validate_form_lineage(
    origin: &CoupledBasis,
    current: &CoupledBasis,
) -> Result<(), String> {
    let original_commands =
        serde_json::to_value(&origin.input.m3_commands).map_err(|e| e.to_string())?;
    let current_commands =
        serde_json::to_value(&current.input.m3_commands).map_err(|e| e.to_string())?;
    let original_commands = original_commands
        .as_array()
        .ok_or("original M3 lineage absent")?;
    let current_commands = current_commands
        .as_array()
        .ok_or("current M3 lineage absent")?;
    if current_commands.len() < original_commands.len()
        || current_commands[..original_commands.len()] != original_commands[..]
    {
        return Err("physical source lost or rewrote an original M3 descendant".into());
    }
    let mut before = serde_json::to_value(&origin.input).map_err(|e| e.to_string())?;
    let mut after = serde_json::to_value(&current.input).map_err(|e| e.to_string())?;
    before
        .as_object_mut()
        .ok_or("original source object absent")?
        .remove("m3_commands");
    after
        .as_object_mut()
        .ok_or("current source object absent")?
        .remove("m3_commands");
    if before != after {
        return Err("Form/material changed original M1/M2/MEF/identity/source inputs".into());
    }
    replay(origin)?;
    replay(current)?;
    Ok(())
}

fn prepare_source(
    owner: &PerformanceOwner,
    current: &CoupledBasis,
    edit: &AuthoredNativePhysicalEdit,
) -> Result<(CoupledBasis, PerformanceConfig, Option<M3Receipt>), String> {
    owner.validate_current(current)?;
    bounded(edit.cause())?;
    let mut state = replay(current)?;
    let mut input = current.input.clone();
    let mut configuration = owner.config.clone();
    let receipt = match edit {
        AuthoredNativePhysicalEdit::Form {
            actor_ref,
            cause_ref,
            occurrence_unix_ms,
            receipt_unix_ms,
            operations,
        } => {
            bounded(actor_ref)?;
            let command = M3Command {
                schema: COMMAND_SCHEMA.into(),
                event_ref: state.snapshot()["identity"]["event_ref"]
                    .as_str()
                    .ok_or("native M3 event absent")?
                    .into(),
                subject_ref: state.snapshot()["subject_ref"]
                    .as_str()
                    .ok_or("native M3 subject absent")?
                    .into(),
                expected_generation: state.generation(),
                actor_ref: actor_ref.clone(),
                cause_ref: cause_ref.clone(),
                occurrence_unix_ms: *occurrence_unix_ms,
                receipt_unix_ms: *receipt_unix_ms,
                operations: operations.clone(),
            };
            let receipt = state.apply(command.clone())?;
            if receipt.status != "applied" {
                return Err("actual native M3 physical edit refused".into());
            }
            input.m3_commands.push(command);
            Some(receipt)
        }
        AuthoredNativePhysicalEdit::Material { material, .. } => {
            configuration.controls.material = material.clone();
            None
        }
        AuthoredNativePhysicalEdit::MetricForm { recipe, .. } => {
            configuration.recipe = recipe.clone();
            None
        }
    };
    configuration.controls.expected_m3_generation = state.generation();
    configuration.controls.body_revision = configuration
        .controls
        .body_revision
        .checked_add(1)
        .ok_or("native body revision exhausted")?;
    configuration.controls.preparation_ref = format!(
        "{}/body-preparation/{}",
        owner.config.session_ref, configuration.controls.body_revision
    );
    let after = input.compose()?;
    if state.snapshot() != after.m3 {
        return Err("new native M3 receipt differs from complete replay".into());
    }
    validate_form_lineage(&owner.source_origin, &after)?;
    Ok((after, configuration, receipt))
}

impl PerformanceOwner {
    pub(crate) fn immutable_source_origin(&self) -> &CoupledBasis {
        &self.source_origin
    }

    /// Read the full real transition applications for native C recording/cold
    /// qualification. Saved evidence still requires original native replay.
    pub fn native_physical_source_history(&self) -> Vec<Value> {
        self.physical_source_history
            .iter()
            .map(NativePhysicalTransitionRecord::snapshot)
            .collect()
    }

    pub(crate) fn prepare_native_physical_edit(
        &self,
        current: &CoupledBasis,
        source: &NativePerformanceReceivingSource,
        original_request_id: u64,
        authored: AuthoredNativePhysicalEdit,
    ) -> Result<PreparedNativePhysicalEdit, String> {
        self.validate_current(current)?;
        if original_request_id == 0 {
            return Err("actual native request ordinal required".into());
        }
        let reading = self
            .reading()
            .ok_or("actual stopped physical reading absent")?;
        if !matches!(
            reading["device"]["state"].as_str(),
            Some("closed" | "prepared")
        ) {
            return Err("physical source edit requires the actual attached device stopped".into());
        }
        let cursor = decimal(&reading["samples_elapsed"])?;
        if reading["physical"]["samples_elapsed"] != reading["samples_elapsed"] {
            return Err("physical edit lost the actual shared P/audio cursor".into());
        }
        source.prepare_current(self, current, cursor)?;
        let original_cursor = decimal(
            &self.source_assets["current_receiving"]["native_admission"]["operation"]["native_sample"],
        )?;
        if *source
            .prepare_retained(
                self,
                current,
                original_cursor,
                &self.source_assets["current_receiving"],
            )?
            .retained_snapshot()
            != self.source_assets["current_receiving"]
        {
            return Err("physical edit lost the complete actual current receiving source".into());
        }
        let prepared = self.prepare_physical_source_descendant_at(
            current,
            source,
            original_request_id,
            &authored,
            cursor,
        )?;
        Ok(PreparedNativePhysicalEdit {
            before_assets: self.source_assets.clone(),
            before_boundary: reading.clone(),
            before_current: current.clone(),
            after_current: prepared.after_current,
            after_owner: prepared.after_owner,
            current_receiving: prepared.current_receiving,
            transition: prepared.transition,
            authored,
            source_record: prepared.source_record,
            original_request_id,
            cursor,
            before_acoustic: prepared.before_acoustic,
            after_acoustic: prepared.after_acoustic,
        })
    }

    /// Original source replay accepts a historical cursor as evidence only.
    /// It performs all actual producers but cannot install or advance native P.
    fn prepare_physical_source_descendant_at(
        &self,
        current: &CoupledBasis,
        source: &NativePerformanceReceivingSource,
        original_request_id: u64,
        authored: &AuthoredNativePhysicalEdit,
        cursor: u64,
    ) -> Result<PreparedPhysicalSourceDescendant, String> {
        self.prepare_physical_source_descendant_for_replay_at(
            current,
            source,
            original_request_id,
            authored,
            cursor,
            ReceivingImplementation::Current,
        )
    }
    fn prepare_physical_source_descendant_for_replay_at(
        &self,
        current: &CoupledBasis,
        source: &NativePerformanceReceivingSource,
        original_request_id: u64,
        authored: &AuthoredNativePhysicalEdit,
        cursor: u64,
        implementation: ReceivingImplementation,
    ) -> Result<PreparedPhysicalSourceDescendant, String> {
        self.validate_current(current)?;
        if original_request_id == 0 {
            return Err("actual original physical request ordinal required".into());
        }
        source.prepare_current(self, current, cursor)?;
        let original_cursor = decimal(
            &self.source_assets["current_receiving"]["native_admission"]["operation"]["native_sample"],
        )?;
        if *source
            .prepare_retained(
                self,
                current,
                original_cursor,
                &self.source_assets["current_receiving"],
            )?
            .retained_snapshot()
            != self.source_assets["current_receiving"]
        {
            return Err("historical physical source receiving no longer replays".into());
        }
        let (after_current, configuration, receipt) = prepare_source(self, current, authored)?;
        let instance = self.binding().determination()["identity"]["instance"]
            .as_str()
            .ok_or("native physical instance absent")?;
        let mut after_owner = PerformanceOwner::prepare(&after_current, instance, configuration)?;
        // This exact origin is transferred only from the existing native owner.
        // The operative source and new binding remain independent descendants.
        after_owner.source_origin = self.source_origin.clone();
        let after_state = replay(&after_current)?;
        after_owner
            .binding()
            .validate_source_form_consumer(&after_state)?;
        let transition = prepare_form_transition(
            self.binding().physical_body(),
            &after_state,
            after_owner
                .binding()
                .physical_body()
                .source_coordinate()
                .clone(),
            after_owner.binding().physical_body().request().clone(),
            self.config.controls.body_revision,
            cursor,
            FormTransitionPolicy::ProjectCorrespondingNodes,
        )?;
        if authored.kind() == "material" {
            let material = prepare_material_update(
                self.binding().physical_body(),
                &after_state,
                self.config.controls.body_revision,
                after_owner.config.controls.material.clone(),
                after_owner.config.controls.body_revision,
                after_owner.config.controls.preparation_ref.clone(),
            )?;
            if serde_json::to_value(material).map_err(|e| e.to_string())?
                != serde_json::to_value(&transition.after).map_err(|e| e.to_string())?
            {
                return Err("material operation altered actual source geometry/face/clock".into());
            }
        }
        let receiving_replay =
            source.admit_replayed(&mut after_owner, &after_current, cursor, implementation)?;
        let receiving_snapshot = receiving_replay.retained_snapshot().clone();
        let current_receiving = receiving_replay.into_fresh();
        let mut assets = self.source_assets.clone();
        for name in [
            "native_basis",
            "source_geometry_reading",
            "source_key_preparation",
            "configuration",
            "source_context",
        ] {
            assets[name] = after_owner.source_assets[name].clone();
        }
        assets["operative_native_input"] =
            serde_json::to_value(&after_current.input).map_err(|e| e.to_string())?;
        assets["operative_physical_consumer_projection"] =
            after_owner.source_assets["physical_consumer_projection"].clone();
        assets["source_form_recipe"] =
            serde_json::to_value(&after_owner.config.recipe).map_err(|e| e.to_string())?;
        assets["current_receiving"] = receiving_snapshot.clone();
        assets["receiving_definition"] = current_receiving.definition().snapshot()?;
        assets["receiving_source_inputs"] = current_receiving.source_inputs().clone();
        let before_acoustic = self
            .source_assets
            .get("acoustic_receiving")
            .map(|value| value["packet"].clone());
        let after_acoustic = if let Some(before) = &before_acoustic {
            let birth = decimal(&before["history_origin_sample"])?;
            let origin = decimal(&before["origin_sample"])?;
            // Keep the current receiver trajectory segment and original birth;
            // C++ appends this body's emitter anchor at the actual edit cursor.
            let prepared = after_owner.prepare_acoustic_receiving_segment_for_replay(
                &after_current,
                source,
                birth,
                origin,
                implementation,
            )?;
            let packet = prepared.packet().clone();
            assets["acoustic_receiving"] = prepared.snapshot();
            Some(packet)
        } else {
            None
        };
        let source_record = json!({"schema":PHYSICAL_SOURCE_TRANSITION,"edit_schema":PHYSICAL_EDIT,"original_native_request_id":original_request_id.to_string(),
            "native_sample":cursor.to_string(),"kind":authored.kind(),"authored_edit":authored,
            "before_current_input":current.input,"after_current_input":after_current.input,
            "native_m3_receipt":receipt,"before_m3":current.m3,"after_m3":after_current.m3,
            "before_configuration":self.config,"after_configuration":after_owner.config,
            "before_native_preparation":self.packet()?,"after_native_preparation":after_owner.packet()?,
            "before_current_receiving":self.source_assets["current_receiving"],"after_current_receiving":receiving_snapshot,
            "prepared_physical_transition":transition,"before_acoustic":before_acoustic,"after_acoustic":after_acoustic,
            "policy":"same retained physical body, exact corresponding-node mass projection, original native cursor and complete source lineage"});
        let history = assets
            .as_object_mut()
            .ok_or("native source assets object absent")?
            .entry("physical_transition_history")
            .or_insert_with(|| json!([]));
        history
            .as_array_mut()
            .ok_or("complete physical source history absent")?
            .push(source_record.clone());
        if serde_json::to_vec(&assets)
            .map_err(|e| e.to_string())?
            .len()
            > super::super::MAX_MESSAGE
        {
            return Err(
                "full native physical source assets exceed existing transport bound".into(),
            );
        }
        after_owner.source_assets = assets;
        Ok(PreparedPhysicalSourceDescendant {
            after_current,
            after_owner,
            current_receiving,
            transition,
            source_record,
            before_acoustic,
            after_acoustic,
        })
    }

    /// Actual closed C/Scene operation. The selected lease must own the exact
    /// planned AFTER bundle. A matching imported record cannot call this path.
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub(crate) fn apply_native_physical_edit(
        &mut self,
        current: &CoupledBasis,
        source: &NativePerformanceReceivingSource,
        session: &mut CoupledFieldSession,
        candidate: PreparedNativePhysicalEdit,
        lease: &super::super::performance_act_bridge::NativeActSourceLease<'_>,
    ) -> Result<(Value, CoupledBasis), NativeStoppedExchangeFailure> {
        self.validate_current(current)?;
        if self.source_assets != candidate.before_assets
            || self.reading() != Some(&candidate.before_boundary)
            || serde_json::to_value(current).map_err(|e| e.to_string())?
                != serde_json::to_value(&candidate.before_current).map_err(|e| e.to_string())?
        {
            return Err("physical edit candidate has a stale actual owner/source/boundary".into());
        }
        let instance = self.binding().determination()["identity"]["instance"]
            .as_str()
            .ok_or("actual physical instance absent")?;
        lease.validate_source_assets(instance, candidate.source_assets())?;
        let fresh = self.prepare_native_physical_edit(
            current,
            source,
            candidate.original_request_id,
            candidate.authored.clone(),
        )?;
        if fresh.source_record != candidate.source_record
            || fresh.source_assets() != candidate.source_assets()
        {
            return Err(
                "full physical edit source/recipe/N9/currentness changed before application".into(),
            );
        }
        candidate.current_receiving.validate_current(
            source,
            &candidate.after_owner,
            &candidate.after_current,
            candidate.cursor,
        )?;
        // Every allocation and source derivation happens before this exchange.
        // The C++ worker serializes all P/Engine/M4 preflights before its commit.
        let mut request = self.raw("source-body-transition")?;
        request["original_request_id"] = json!(candidate.original_request_id.to_string());
        request["expected_sample"] = json!(candidate.cursor.to_string());
        request["kind"] = json!(candidate.authored.kind());
        request["cause_ref"] = json!(candidate.authored.cause());
        request["before_packet"] = self.packet()?;
        request["before_native_basis"] =
            serde_json::to_value(self.binding().native_basis()).map_err(|e| e.to_string())?;
        request["after_packet"] = candidate.after_owner.packet()?;
        request["actual_after_packet"] = fresh.after_owner.packet()?;
        request["actual_after_native_basis"] =
            serde_json::to_value(fresh.after_owner.binding().native_basis())
                .map_err(|e| e.to_string())?;
        request["receiving_admission"] = candidate.current_receiving.admission().snapshot()?;
        request["current_receiving_admission"] = fresh.current_receiving.admission().snapshot()?;
        request["native_catalog"] = json!(candidate.after_owner.cells);
        request["body_source"] = json!({"kind":"sourceForm","recipe_ref":candidate.after_owner.config.recipe.provenance.reference,
            "validated_m3_generation":candidate.after_owner.config.controls.expected_m3_generation.to_string()});
        request["before_acoustic"] = json!(candidate.before_acoustic);
        request["prepared_acoustic"] = json!(candidate.after_acoustic);
        request["current_acoustic"] = json!(fresh.after_acoustic);
        let pulse = session.performance_exchange_retained(&request)?;
        let checked = (|| -> Result<(), String> {
            if pulse["accepted"] != true {
                self.validate_reply(&pulse)?;
                return Err(pulse["reason"]
                    .as_str()
                    .unwrap_or("native physical source edit refused")
                    .into());
            }
            if pulse["operation"] != "source-body-transition" {
                return Err("native physical operation acknowledgement differs".into());
            }
            candidate.after_owner.validate_reply(&pulse)?;
            let acknowledgement = &pulse["payload"]["physical_transition"];
            if acknowledgement["schema"] != "ql.native-physical-source-application/v1"
                || acknowledgement["original_native_request_id"] != request["original_request_id"]
                || acknowledgement["native_sample"] != request["expected_sample"]
                || acknowledgement["kind"] != request["kind"]
                || acknowledgement["policy"] != "project-corresponding-nodes"
                || acknowledgement["before_body_revision"]
                    != candidate.before_boundary["scope"]["body_revision"]
                || acknowledgement["after_body_revision"]
                    != pulse["reading"]["scope"]["body_revision"]
                || acknowledgement["before_eigenbasis_identity"]
                    != candidate.before_boundary["physical"]["eigenbasis_identity"]
                || acknowledgement["after_eigenbasis_identity"]
                    != pulse["reading"]["physical"]["eigenbasis_identity"]
                || acknowledgement["before_native_preparation"] != request["before_packet"]
                || acknowledgement["after_native_preparation"] != request["after_packet"]
                || pulse["reading"]["transport_epoch"]
                    != candidate.before_boundary["transport_epoch"]
                || pulse["reading"]["accepted_sequence"]
                    != candidate.before_boundary["accepted_sequence"]
                || pulse["reading"]["samples_elapsed"] != request["expected_sample"]
            {
                return Err(
                    "actual P/body/N9 source edit application lost its original boundary".into(),
                );
            }
            for name in [
                "before_energy_joules",
                "after_energy_joules",
                "external_work_joules",
            ] {
                if acknowledgement[name]
                    .as_f64()
                    .is_none_or(|value| !value.is_finite())
                {
                    return Err("actual physical transition energy/work receipt absent".into());
                }
            }
            if let Some(before) = &candidate.before_acoustic {
                let manifest = &pulse["reading"]["receiving_transport"]["manifest"];
                for name in [
                    "context",
                    "receiver",
                    "source_motion",
                    "receiver_motion",
                    "origin_sample",
                    "end_sample",
                    "history_origin_sample",
                ] {
                    if manifest[name]
                        != candidate.before_boundary["receiving_transport"]["manifest"][name]
                    {
                        return Err(
                            "physical edit reset the original receiver/history/trajectory".into(),
                        );
                    }
                }
                if manifest["body_revision"] != pulse["reading"]["scope"]["body_revision"]
                    || before["history_origin_sample"] != manifest["history_origin_sample"]
                    || pulse["reading"]["receiving_transport"]["samples_elapsed"]
                        != request["expected_sample"]
                {
                    return Err("physical edit has a stale/disconnected actual M4 body".into());
                }
            }
            lease.validate_source_assets(instance, candidate.source_assets())?;
            Ok(())
        })();
        if let Err(reason) = checked {
            let reason = if pulse["accepted"] == true {
                session.performance_invalidate(&reason)
            } else {
                reason
            };
            return Err(NativeStoppedExchangeFailure {
                reason,
                native_receipts: vec![pulse],
            });
        }
        // The same native owner has committed and the full reply qualified.
        // Moves cannot run another body callback, reset voices or rewrite input.
        let after = candidate.after_current;
        self.original_input = candidate.after_owner.original_input;
        self.binding = candidate.after_owner.binding;
        self.sparse = candidate.after_owner.sparse;
        self.config = candidate.after_owner.config;
        self.cells = candidate.after_owner.cells;
        self.return_binding = candidate.after_owner.return_binding;
        self.source_assets = candidate.after_owner.source_assets;
        self.physical_source_history
            .push(NativePhysicalTransitionRecord {
                source: candidate.source_record,
                application: pulse.clone(),
            });
        self.last = Some(pulse.clone());
        Ok((pulse, after))
    }
}

#[cfg(test)]
#[path = "performance_form_tests.rs"]
mod tests;

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[path = "performance_form_cold.rs"]
mod cold;
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub(crate) use cold::PreparedColdPhysicalSource;
