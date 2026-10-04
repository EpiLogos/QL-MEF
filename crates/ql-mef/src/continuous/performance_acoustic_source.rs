//! Original source history for actual same-owner acoustic source/receiver edits.
//! Only authored configuration is serde input. Complete producer records are
//! regenerated; original accepted stopped worker pulses are retained separately.
use super::*;

pub(super) const ACOUSTIC_SOURCE_TRANSITION: &str = "ql.native-acoustic-source-transition/v1";

pub(in crate::continuous) struct PreparedAcousticSourceDescendant {
    pub(in crate::continuous) original: PreparedAcousticReceiving,
    pub(in crate::continuous) after: PreparedAcousticReceiving,
    pub(in crate::continuous) after_assets: Value,
    pub(in crate::continuous) configuration: AcousticConfiguration,
    pub(in crate::continuous) record: Value,
}

pub(in crate::continuous) struct PreparedAcousticInstallationDescendant {
    pub(in crate::continuous) after: PreparedAcousticReceiving,
    pub(in crate::continuous) after_assets: Value,
    pub(in crate::continuous) record: Value,
}

impl PerformanceOwner {
    /// Complete original stopped applications. The selected C source owner
    /// retains this sidecar; importing bytes cannot grant an operation/lease.
    pub fn native_acoustic_source_history(&self) -> Vec<Value> {
        self.acoustic_source_history.clone()
    }

    pub(crate) fn prepare_stopped_acoustic_installation_assets_at_request(
        &self,
        current: &CoupledBasis,
        original_source: &NativePerformanceReceivingSource,
        after_source: &NativePerformanceReceivingSource,
        original_request_id: u64,
    ) -> Result<PreparedAcousticInstallation, String> {
        // Retain the existing actual stopped-before-install/currentness guard.
        let mut candidate = self.prepare_stopped_acoustic_installation_assets(
            current,
            original_source,
            after_source,
        )?;
        let actual = self.prepare_acoustic_installation_descendant_at(
            current,
            after_source,
            original_request_id,
            candidate.cursor,
        )?;
        if actual.after.snapshot() != candidate.prepared.snapshot() {
            return Err(
                "original initial acoustic source compiler differs from stopped candidate".into(),
            );
        }
        candidate.after_assets = actual.after_assets;
        candidate.original_request_id = original_request_id;
        candidate.source_record = Some(actual.record);
        Ok(candidate)
    }

    pub(in crate::continuous) fn prepare_acoustic_installation_descendant_at(
        &self,
        current: &CoupledBasis,
        after_source: &NativePerformanceReceivingSource,
        original_request_id: u64,
        native_sample: u64,
    ) -> Result<PreparedAcousticInstallationDescendant, String> {
        self.prepare_acoustic_installation_descendant_for_replay_at(
            current,
            after_source,
            original_request_id,
            native_sample,
            ReceivingImplementation::Current,
        )
    }
    pub(in crate::continuous) fn prepare_acoustic_installation_descendant_for_replay_at(
        &self,
        current: &CoupledBasis,
        after_source: &NativePerformanceReceivingSource,
        original_request_id: u64,
        native_sample: u64,
        implementation: ReceivingImplementation,
    ) -> Result<PreparedAcousticInstallationDescendant, String> {
        self.validate_current(current)?;
        if original_request_id == 0
            || self.source_assets.get("acoustic_receiving").is_some()
            || !self.acoustic_source_history.is_empty()
        {
            return Err(
                "original acoustic installation requires its actual first source epoch".into(),
            );
        }
        let before_source = after_source.clone().without_acoustic_configuration();
        if before_source.source_inputs()? != self.source_assets["receiving_source_inputs"] {
            return Err(
                "initial acoustic source changed original complete World/person/occasion".into(),
            );
        }
        if let Some(previous) = self.source_assets["physical_transition_history"]
            .as_array()
            .and_then(|v| v.last())
        {
            if decimal(&previous["original_native_request_id"])? >= original_request_id
                || decimal(&previous["native_sample"])? > native_sample
            {
                return Err(
                    "initial acoustic source precedes an actual physical source operation".into(),
                );
            }
        }
        let source_sample = decimal(
            &self.source_assets["current_receiving"]["native_admission"]["operation"]["native_sample"],
        )?;
        let before = before_source.prepare_retained(
            self,
            current,
            source_sample,
            &self.source_assets["current_receiving"],
        )?;
        if *before.retained_snapshot() != self.source_assets["current_receiving"]
            || source_sample > native_sample
        {
            return Err("initial acoustic source lost actual previous body/N9 admission".into());
        }
        let configuration = after_source
            .acoustic_configuration()
            .ok_or("actual first acoustic configuration absent")?
            .clone();
        let after = self.prepare_acoustic_receiving_segment_for_replay(
            current,
            after_source,
            native_sample,
            native_sample,
            implementation,
        )?;
        let receiving = after_source.begin_replay_preparation(self, current, implementation)?;
        let admitted = receiving.prepare_at(native_sample)?;
        receiving.validate_at(&admitted, native_sample)?;
        let mut assets = self.source_assets.clone();
        assets["source_context"] = admitted.fresh().context().snapshot()?;
        assets["receiving_source_inputs"] = admitted.fresh().source_inputs().clone();
        assets["receiving_definition"] = admitted.fresh().definition().snapshot()?;
        assets["current_receiving"] = admitted.retained_snapshot().clone();
        assets["acoustic_receiving"] = after.snapshot();
        let record = json!({"schema":ACOUSTIC_SOURCE_TRANSITION,"kind":"install",
            "original_native_request_id":original_request_id.to_string(),"native_sample":native_sample.to_string(),
            "performance_configuration":self.config,"native_current_input":current.input,
            "native_preparation":self.packet()?,"immutable_original_input":self.source_assets["original_native_input"],
            "before_configuration":Value::Null,"after_configuration":configuration,
            "before_source_inputs":before_source.source_inputs()?,"after_source_inputs":admitted.fresh().source_inputs(),
            "before_source_context":self.source_assets["source_context"],"after_source_context":admitted.fresh().context().snapshot()?,
            "before_receiving_definition":self.source_assets["receiving_definition"],"after_receiving_definition":admitted.fresh().definition().snapshot()?,
            "before_current_receiving":self.source_assets["current_receiving"],"after_current_receiving":admitted.retained_snapshot().clone(),
            "before_acoustic":Value::Null,"after_acoustic":after.packet(),"history_origin_sample":native_sample.to_string(),
            "policy":"same retained physical body and original World/occasion, exact first native receiver birth"});
        if assets.get("acoustic_transition_history").is_some() {
            return Err(
                "initial acoustic source already retains another installation history".into(),
            );
        }
        assets["acoustic_transition_history"] = json!([record]);
        if serde_json::to_vec(&assets)
            .map_err(|e| e.to_string())?
            .len()
            > crate::continuous::MAX_MESSAGE
        {
            return Err("complete initial acoustic source exceeds existing transport bound".into());
        }
        Ok(PreparedAcousticInstallationDescendant {
            after,
            after_assets: assets,
            record,
        })
    }

    /// Root's existing private Host/Manager wrapper supplies the ORIGINAL
    /// prepare request id. A user configuration/revision/audio ordinal cannot
    /// select it, and a later commit must preserve this same id in its opaque
    /// candidate. The complete actual stopped reading is retained unchanged.
    pub(crate) fn prepare_stopped_acoustic_receiver_assets_at_request(
        &self,
        current: &CoupledBasis,
        after_source: &NativePerformanceReceivingSource,
        original_request_id: u64,
    ) -> Result<PreparedAcousticReceiverUpdate, String> {
        let boundary = self
            .reading()
            .ok_or("actual stopped acoustic source boundary absent")?;
        if !matches!(
            boundary["device"]["state"].as_str(),
            Some("closed" | "prepared")
        ) {
            return Err(
                "actual acoustic source edit requires the attached native owner stopped".into(),
            );
        }
        let cursor = decimal(&boundary["samples_elapsed"])?;
        if decimal(&boundary["physical"]["samples_elapsed"])? != cursor {
            return Err("acoustic source edit lost same native P/audio cursor".into());
        }
        let prepared = self.prepare_acoustic_source_descendant_at(
            current,
            after_source,
            original_request_id,
            cursor,
        )?;
        Ok(PreparedAcousticReceiverUpdate {
            before_assets: self.source_assets.clone(),
            after_assets: prepared.after_assets,
            original: prepared.original,
            after: prepared.after,
            configuration: prepared.configuration,
            native_boundary: boundary.clone(),
            cursor,
            original_request_id,
            source_record: Some(prepared.record),
        })
    }

    /// Pure source compiler for original cold replay. Cursor/request are
    /// evidence only until the same closed C corpus and stopped consumer qualify
    /// them. This method cannot install a port, acquire a clock or advance P.
    pub(in crate::continuous) fn prepare_acoustic_source_descendant_at(
        &self,
        current: &CoupledBasis,
        after_source: &NativePerformanceReceivingSource,
        original_request_id: u64,
        native_sample: u64,
    ) -> Result<PreparedAcousticSourceDescendant, String> {
        self.prepare_acoustic_source_descendant_for_replay_at(
            current,
            after_source,
            original_request_id,
            native_sample,
            ReceivingImplementation::Current,
        )
    }
    pub(in crate::continuous) fn prepare_acoustic_source_descendant_for_replay_at(
        &self,
        current: &CoupledBasis,
        after_source: &NativePerformanceReceivingSource,
        original_request_id: u64,
        native_sample: u64,
        implementation: ReceivingImplementation,
    ) -> Result<PreparedAcousticSourceDescendant, String> {
        self.validate_current(current)?;
        if original_request_id == 0 {
            return Err("actual original acoustic request ordinal required".into());
        }
        let before_configuration: AcousticConfiguration = serde_json::from_value(
            self.source_assets["acoustic_receiving"]["packet"]["configuration"].clone(),
        )
        .map_err(|e| e.to_string())?;
        let before_source = after_source
            .clone()
            .with_acoustic_configuration(before_configuration.clone())?;
        let original =
            self.prepare_retained_acoustic_sources(current, &before_source, native_sample)?;
        if before_source.source_inputs()? != self.source_assets["receiving_source_inputs"] {
            return Err(
                "acoustic edit changed the original complete World/person/occasion/source family"
                    .into(),
            );
        }
        let configuration = after_source
            .acoustic_configuration()
            .ok_or("authored AFTER acoustic configuration absent")?
            .clone();
        configuration.validate(self.binding().physical_body().request().sample_rate)?;
        if configuration.revision <= before_configuration.revision
            || native_sample < original.origin_sample
        {
            return Err(
                "acoustic edit regressed its actual configuration revision/trajectory date".into(),
            );
        }
        let birth = original.history_origin_sample;
        let after = self.prepare_acoustic_receiving_segment_for_replay(
            current,
            after_source,
            birth,
            native_sample,
            implementation,
        )?;
        after.validate_current(self, current, after_source, birth)?;
        let receiving = after_source.begin_replay_preparation(self, current, implementation)?;
        let admitted = receiving.prepare_at(native_sample)?;
        receiving.validate_at(&admitted, native_sample)?;
        let mut assets = self.source_assets.clone();
        assets["source_context"] = admitted.fresh().context().snapshot()?;
        assets["receiving_source_inputs"] = admitted.fresh().source_inputs().clone();
        assets["receiving_definition"] = admitted.fresh().definition().snapshot()?;
        assets["current_receiving"] = admitted.retained_snapshot().clone();
        assets["acoustic_receiving"] = after.snapshot();
        let record = json!({"schema":ACOUSTIC_SOURCE_TRANSITION,"kind":"replace",
            "original_native_request_id":original_request_id.to_string(),"native_sample":native_sample.to_string(),
            "performance_configuration":self.config,"native_current_input":current.input,
            "native_preparation":self.packet()?,"immutable_original_input":self.source_assets["original_native_input"],
            "before_configuration":before_configuration,"after_configuration":configuration,
            "before_source_inputs":before_source.source_inputs()?,"after_source_inputs":admitted.fresh().source_inputs(),
            "before_source_context":self.source_assets["source_context"],"after_source_context":admitted.fresh().context().snapshot()?,
            "before_receiving_definition":self.source_assets["receiving_definition"],"after_receiving_definition":admitted.fresh().definition().snapshot()?,
            "before_current_receiving":self.source_assets["current_receiving"],"after_current_receiving":admitted.retained_snapshot().clone(),
            "before_acoustic":original.packet(),"after_acoustic":after.packet(),
            "history_origin_sample":birth.to_string(),"policy":"same retained physical body and original World/occasion, exact dated emitter and receiver, full original native delay ring"});
        let history = assets
            .as_object_mut()
            .ok_or("acoustic source assets object absent")?
            .entry("acoustic_transition_history")
            .or_insert_with(|| json!([]));
        let history = history
            .as_array_mut()
            .ok_or("complete acoustic source history absent")?;
        if let Some(previous) = history.last() {
            if decimal(&previous["original_native_request_id"])? >= original_request_id
                || decimal(&previous["native_sample"])? > native_sample
            {
                return Err(
                    "actual acoustic source request/time order regressed or duplicated".into(),
                );
            }
        }
        if let Some(previous) = self.source_assets["physical_transition_history"]
            .as_array()
            .and_then(|v| v.last())
        {
            if decimal(&previous["original_native_request_id"])? >= original_request_id
                || decimal(&previous["native_sample"])? > native_sample
            {
                return Err(
                    "acoustic source precedes an actual committed physical source operation".into(),
                );
            }
        }
        history.push(record.clone());
        if serde_json::to_vec(&assets)
            .map_err(|e| e.to_string())?
            .len()
            > crate::continuous::MAX_MESSAGE
        {
            return Err(
                "complete acoustic source/history exceeds existing native transport bound".into(),
            );
        }
        Ok(PreparedAcousticSourceDescendant {
            original,
            after,
            after_assets: assets,
            configuration,
            record,
        })
    }

    pub(super) fn validate_acoustic_source_application(
        &self,
        candidate: &PreparedAcousticReceiverUpdate,
        pulse: &Value,
    ) -> Result<(), String> {
        let Some(source) = &candidate.source_record else {
            return Ok(());
        };
        if source["schema"] != ACOUSTIC_SOURCE_TRANSITION
            || source["kind"] != "replace"
            || decimal(&source["original_native_request_id"])? != candidate.original_request_id
            || decimal(&source["native_sample"])? != candidate.cursor
            || pulse["accepted"] != true
            || pulse["operation"] != "receiving-transport-replace"
        {
            return Err(
                "actual acoustic source application lost original typed producer/operation".into(),
            );
        }
        Ok(())
    }

    pub(super) fn validate_acoustic_installation_source_application(
        &self,
        candidate: &PreparedAcousticInstallation,
        pulse: &Value,
    ) -> Result<(), String> {
        let Some(source) = &candidate.source_record else {
            return Ok(());
        };
        if source["schema"] != ACOUSTIC_SOURCE_TRANSITION
            || source["kind"] != "install"
            || decimal(&source["original_native_request_id"])? != candidate.original_request_id
            || decimal(&source["native_sample"])? != candidate.cursor
            || pulse["accepted"] != true
            || pulse["operation"] != "receiving-transport-install"
            || source["before_acoustic"] != Value::Null
        {
            return Err(
                "actual initial acoustic application lost original no-receiver source epoch".into(),
            );
        }
        Ok(())
    }
}
