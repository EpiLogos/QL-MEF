//! Native acoustic receiver configuration and source preparation. Configuration
//! is authored numerical input, never private Act/context/source authority.
//! Geometry is the actual immutable SourceForm body, not an invented emitter.
use super::*;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use crate::continuous::performance_act_bridge::NativeActSourceLease;
use crate::continuous::performance_receiving::NativePerformanceReceivingSource;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AcousticConfiguration {
    pub schema: String,
    pub source_ref: String,
    pub source_motion_ref: String,
    pub receiver_motion_ref: String,
    pub policy_ref: String,
    pub policy_revision: String,
    pub standing: String,
    pub revision: u64,
    pub source_translation_metres: [f64; 3],
    pub receiver_position_metres: [f64; 3],
    pub receiver_forward: [f64; 3],
    pub source_velocity_metres_per_second: [f64; 3],
    pub receiver_velocity_metres_per_second: [f64; 3],
    pub speed_metres_per_second: f64,
    pub minimum_distance_metres: f64,
    pub directivity: AcousticDirectivity,
    pub propagation_delay: bool,
    pub span_samples: u32,
}
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum AcousticDirectivity {
    Omnidirectional,
    Cardioid,
}
impl AcousticConfiguration {
    pub fn validate(&self, rate: u32) -> Result<(), String> {
        if self.schema != "ql.native-acoustic-receiving-configuration/v1"
            || !["architecture-model", "reference", "tunable-model"]
                .contains(&self.standing.as_str())
            || self.revision == 0
            || !(8000..=192000).contains(&rate)
            || self.span_samples == 0
            || u64::from(self.span_samples) > u64::from(rate) * 60
            || !self.speed_metres_per_second.is_finite()
            || !(50.0..=2000.0).contains(&self.speed_metres_per_second)
            || !self.minimum_distance_metres.is_finite()
            || !(0.001..=1000.0).contains(&self.minimum_distance_metres)
        {
            return Err("native acoustic policy/units/finite segment differs".into());
        }
        for reference in [
            &self.source_ref,
            &self.source_motion_ref,
            &self.receiver_motion_ref,
            &self.policy_ref,
            &self.policy_revision,
            &self.standing,
        ] {
            bounded(reference)?;
        }
        for vector in [
            &self.source_translation_metres,
            &self.receiver_position_metres,
        ] {
            if vector.iter().any(|v| !v.is_finite() || v.abs() > 1e6) {
                return Err("native acoustic geometry requires bounded metres".into());
            }
        }
        if self
            .receiver_forward
            .iter()
            .any(|v| !v.is_finite() || v.abs() > 1.0)
            || (self.receiver_forward.iter().map(|v| v * v).sum::<f64>() - 1.0).abs() > 1e-10
        {
            return Err("native receiver orientation must be normalized".into());
        }
        for vector in [
            &self.source_velocity_metres_per_second,
            &self.receiver_velocity_metres_per_second,
        ] {
            let square = vector.iter().map(|v| v * v).sum::<f64>();
            if vector.iter().any(|v| !v.is_finite())
                || !square.is_finite()
                || square > (self.speed_metres_per_second * 0.025).powi(2)
            {
                return Err("native acoustic speed exceeds admitted audio-band policy".into());
            }
        }
        Ok(())
    }
}
/// Immutable native producer output; serde bytes retain evidence only.
/// No Deserialize, public constructor, independent oscillator/body or clock.
pub struct PreparedAcousticReceiving {
    packet: Value,
    current_receiving: Value,
    history_origin_sample: u64,
    origin_sample: u64,
}
impl PreparedAcousticReceiving {
    pub fn packet(&self) -> &Value {
        &self.packet
    }
    pub fn current_receiving(&self) -> &Value {
        &self.current_receiving
    }
    pub fn snapshot(&self) -> Value {
        json!({"schema":"ql.current-native-acoustic-receiving/v1",
            "packet":self.packet,"current_receiving":self.current_receiving})
    }
    pub fn validate_current(
        &self,
        owner: &PerformanceOwner,
        current: &CoupledBasis,
        source: &NativePerformanceReceivingSource,
        original_native_birth: u64,
    ) -> Result<(), String> {
        let actual = owner.prepare_acoustic_receiving_segment(
            current,
            source,
            original_native_birth,
            self.origin_sample,
        )?;
        if self.history_origin_sample != original_native_birth
            || actual.snapshot() != self.snapshot()
        {
            return Err(
                "complete original acoustic/source/context/body preparation changed".into(),
            );
        }
        Ok(())
    }
}
/// Complete independently prepared candidate assets for the existing Scene
/// CAS. This numerical/source preparation grants no operation or private read.
/// No Deserialize, Clone, public constructor or mutable payload getter.
pub struct PreparedAcousticReceiverUpdate {
    before_assets: Value,
    after_assets: Value,
    original: PreparedAcousticReceiving,
    after: PreparedAcousticReceiving,
    configuration: AcousticConfiguration,
    native_boundary: Value,
    cursor: u64,
}
impl PreparedAcousticReceiverUpdate {
    pub fn before_source_assets(&self) -> &Value {
        &self.before_assets
    }
    /// Exact original copied reading, not an inferred epoch/queue acknowledgement.
    pub fn native_boundary(&self) -> &Value {
        &self.native_boundary
    }
    pub fn source_assets(&self) -> &Value {
        &self.after_assets
    }
    pub fn preparation(&self) -> &PreparedAcousticReceiving {
        &self.after
    }
    pub fn configuration(&self) -> &AcousticConfiguration {
        &self.configuration
    }
    pub fn native_segment_origin(&self) -> u64 {
        self.cursor
    }
}
/// Pure initial candidate for the existing Scene CAS. The native source and
/// copied stopped output boundary qualify its preparation, never a serialized
/// candidate. No Deserialize, Clone, public constructor or mutable getter.
pub(crate) struct PreparedAcousticInstallation {
    before_assets: Value,
    after_assets: Value,
    prepared: PreparedAcousticReceiving,
    configuration: AcousticConfiguration,
    boundary: Value,
    cursor: u64,
}
impl PreparedAcousticInstallation {
    pub(crate) fn before_source_assets(&self) -> &Value {
        &self.before_assets
    }
    pub(crate) fn source_assets(&self) -> &Value {
        &self.after_assets
    }
    pub(crate) fn configuration(&self) -> &AcousticConfiguration {
        &self.configuration
    }
    #[cfg(test)]
    pub(crate) fn preparation(&self) -> &PreparedAcousticReceiving {
        &self.prepared
    }
    /// Exact copied native reading; this is evidence, never a clock/source grant.
    pub(crate) fn native_boundary(&self) -> &Value {
        &self.boundary
    }
}
/// Refusal retains the actual worker pulse already drained by this operation.
/// No imported serde constructor can manufacture either a success or a pulse.
pub(crate) struct AcousticRefusal {
    reason: String,
    native_pulse: Option<Value>,
}
impl AcousticRefusal {
    pub(crate) fn retaining_native_pulse(reason: String, pulse: Value) -> Self {
        Self {
            reason,
            native_pulse: Some(pulse),
        }
    }
    pub(crate) fn reason(&self) -> &str {
        &self.reason
    }
    pub(crate) fn native_pulse(&self) -> Option<&Value> {
        self.native_pulse.as_ref()
    }
}
impl From<String> for AcousticRefusal {
    fn from(reason: String) -> Self {
        Self {
            reason,
            native_pulse: None,
        }
    }
}
impl From<&str> for AcousticRefusal {
    fn from(reason: &str) -> Self {
        reason.to_owned().into()
    }
}
impl PerformanceOwner {
    /// Pure control preparation from the actual privately constructed current
    /// source. The supplied cursor is not authority: staging below reads it
    /// from the SAME actual stopped Manager; private install rechecks it.
    pub fn prepare_acoustic_receiving_source(
        &self,
        current: &CoupledBasis,
        source: &NativePerformanceReceivingSource,
        original_native_birth: u64,
    ) -> Result<PreparedAcousticReceiving, String> {
        self.prepare_acoustic_receiving_segment(
            current,
            source,
            original_native_birth,
            original_native_birth,
        )
    }
    /// Pure complete producer for a later receiving trajectory segment. The
    /// original pickup/history birth is immutable; this cursor is numerical
    /// input only until the actual stopped owner and private reader admit it.
    fn prepare_acoustic_receiving_segment(
        &self,
        current: &CoupledBasis,
        source: &NativePerformanceReceivingSource,
        original_native_birth: u64,
        native_segment_origin: u64,
    ) -> Result<PreparedAcousticReceiving, String> {
        self.validate_current(current)?;
        if native_segment_origin < original_native_birth {
            return Err("native acoustic segment precedes original history birth".into());
        }
        let config = source
            .acoustic_configuration()
            .ok_or("native receiving source has no original acoustic configuration")?;
        let body = self.binding().physical_body();
        config.validate(body.request().sample_rate)?;
        let end = native_segment_origin
            .checked_add(u64::from(config.span_samples))
            .ok_or("native acoustic segment cursor overflow")?;
        let receiving = source.prepare_current(self, current, native_segment_origin)?;
        receiving.validate_current(source, self, current, native_segment_origin)?;
        let context = source.return_context();
        let metric = &body.request().geometry;
        let pickup = &body.request().pickup;
        if pickup.node_weights.len() != metric.nodes.len() {
            return Err("actual physical pickup/source topology differs".into());
        }
        // Existing immutable metric body/pickup determines the emitter rest
        // point. This is an observation projection; no modal eigenmode retune.
        let mut source_point = config.source_translation_metres;
        // The same constant-velocity emitter path continues from its actual
        // original birth. A later receiver segment must not reset the source
        // position to its initial metric rest point.
        let elapsed_seconds = (native_segment_origin - original_native_birth) as f64
            / f64::from(body.request().sample_rate);
        if native_segment_origin != original_native_birth {
            for (value, velocity) in source_point
                .iter_mut()
                .zip(config.source_velocity_metres_per_second)
            {
                *value += velocity * elapsed_seconds;
            }
        }
        for (node, weight) in metric.nodes.iter().zip(&pickup.node_weights) {
            for (value, rest) in source_point.iter_mut().zip(node.rest_metres) {
                *value += rest * weight;
            }
        }
        if source_point.iter().any(|v| !v.is_finite() || v.abs() > 1e6) {
            return Err("actual metric acoustic pickup source exceeds bounds".into());
        }
        Ok(PreparedAcousticReceiving {
            packet: json!({"schema":"ql.native-acoustic-receiving-preparation/v1",
                "source_body":body,"configuration":config,
                "context":context,"source_position_metres":source_point,
                "origin_sample":native_segment_origin.to_string(),"end_sample":end.to_string(),
                "history_origin_sample":original_native_birth.to_string(),
                "units":{"distance":"m","velocity":"m/s","cursor":"native-audio-sample","signal":"linear-pickup"}}),
            current_receiving: receiving.snapshot()?,
            history_origin_sample: original_native_birth,
            origin_sample: native_segment_origin,
        })
    }
    /// Regenerate the ORIGINAL saved receiver segment under the exact closed
    /// source/checkpoint selection. Its origin/birth are retained producer
    /// inputs; neither the current fresh owner cursor nor the saved checkpoint
    /// can authorize a new receiver, source, protected occasion or trajectory.
    /// This is a pure preparation: no assets, P state, queue or clock change.
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub(crate) fn prepare_saved_acoustic_receiving(
        &self,
        current: &CoupledBasis,
        source: &NativePerformanceReceivingSource,
        lease: &NativeActSourceLease<'_>,
        original_checkpoint_wire: &str,
        checkpoint_ref: &str,
    ) -> Result<Option<PreparedAcousticReceiving>, String> {
        self.validate_current(current)?;
        bounded(checkpoint_ref)?;
        if original_checkpoint_wire.is_empty()
            || original_checkpoint_wire.len() as u64 > crate::continuous::host::MAX_HOST_INPUT
        {
            return Err("complete selected acoustic checkpoint exceeds transport bound".into());
        }
        let instance = self.binding().determination()["identity"]["instance"]
            .as_str()
            .ok_or("native saved acoustic instance absent")?;
        lease.validate_source_assets(instance, self.source_assets())?;
        lease.validate_selected_checkpoint(instance, checkpoint_ref, original_checkpoint_wire)?;
        let saved: Value =
            serde_json::from_str(original_checkpoint_wire).map_err(|e| e.to_string())?;
        if saved["schema"] != "ql.performance-management-checkpoint/v1"
            || !saved["native_pair"]["audio"].is_object()
        {
            return Err("complete selected acoustic Management checkpoint differs".into());
        }
        let audio = &saved["native_pair"]["audio"];
        // Historical checkpoints without a receiving encoding keep their
        // exact original bytes. They must not acquire an acoustic operand.
        let encoded = audio.get("has_receiving");
        let enabled = match encoded {
            None => false,
            Some(v) => v
                .as_bool()
                .ok_or("saved receiving discriminant is not boolean")?,
        };
        if !enabled {
            if audio.get("receiving").is_some_and(|v| !v.is_null()) {
                return Err("disabled saved receiving carries unowned receiver state".into());
            }
            return Ok(None);
        }
        let saved_cursor = decimal(&audio["cursor"])?;
        let retained = &self.source_assets["acoustic_receiving"];
        let packet = &retained["packet"];
        let birth = decimal(&packet["history_origin_sample"])?;
        let origin = decimal(&packet["origin_sample"])?;
        // The original typed receiving factory already contains the complete
        // authored configuration. Do not manufacture it from checkpoint JSON.
        let original = self.prepare_acoustic_receiving_segment(current, source, birth, origin)?;
        if original.snapshot() != *retained {
            return Err("original saved acoustic source cannot be regenerated".into());
        }
        let before = source.prepare_current(self, current, origin)?;
        before.validate_current(source, self, current, origin)?;
        if before.snapshot()? != self.source_assets["current_receiving"]
            || before.context().snapshot()? != self.source_assets["source_context"]
            || *before.source_inputs() != self.source_assets["receiving_source_inputs"]
            || before.definition().snapshot()? != self.source_assets["receiving_definition"]
        {
            return Err("full original saved acoustic receiving/source/context differs".into());
        }
        let checkpoint = &audio["receiving"];
        let manifest = &checkpoint["manifest"];
        let end = decimal(&original.packet["end_sample"])?;
        if checkpoint["schema"] != "ql.performance-receiving-checkpoint/v1"
            || saved_cursor < origin
            || saved_cursor > end
            || decimal(&checkpoint["samples_elapsed"])? != saved_cursor
            || decimal(&checkpoint["history_start_sample"])? != birth
            || manifest["origin_sample"].as_str() != Some(origin.to_string().as_str())
            || manifest["end_sample"].as_str() != Some(end.to_string().as_str())
            || manifest["history_origin_sample"].as_str() != Some(birth.to_string().as_str())
        {
            return Err("saved acoustic cursor/segment/history birth differs".into());
        }
        // The existing C++ checkpoint decoder and factory validate every
        // manifest field and all 16384 exact f32 samples before mutation. Rust
        // retains the whole selected original wire; it owns no competing codec
        // or receiving-identity formula. These native source refs are checked
        // here as well as by that complete numerical admission.
        for (name, expected) in [
            (
                "context",
                &original.packet["context"]["context"]["reference"],
            ),
            (
                "receiver",
                &original.packet["context"]["receiver"]["reference"],
            ),
            (
                "source_motion",
                &original.packet["configuration"]["source_motion_ref"],
            ),
            (
                "receiver_motion",
                &original.packet["configuration"]["receiver_motion_ref"],
            ),
            ("policy", &original.packet["configuration"]["policy_ref"]),
            (
                "policy_revision",
                &original.packet["configuration"]["policy_revision"],
            ),
            ("standing", &original.packet["configuration"]["standing"]),
        ] {
            if manifest[name] != *expected {
                return Err("saved acoustic manifest does not belong to original source".into());
            }
        }
        // Current source/occasion/grants must also remain valid at the saved
        // cursor. This independent N9 admission does not retag the original
        // receiver segment's origin or current_receiving evidence.
        let now = source.prepare_current(self, current, saved_cursor)?;
        now.validate_current(source, self, current, saved_cursor)?;
        original.validate_current(self, current, source, birth)?;
        lease.validate_source_assets(instance, self.source_assets())?;
        lease.validate_selected_checkpoint(instance, checkpoint_ref, original_checkpoint_wire)?;
        Ok(Some(original))
    }
    /// Same-source receiver update. A changed emitter path would require
    /// its full historical retarded trajectory, so this bounded operation
    /// preserves the original source anchor/velocity/ref. Full source/body/
    /// context recompile precedes preparation; no previous packet is a grant.
    pub fn prepare_acoustic_receiver_update(
        &self,
        current: &CoupledBasis,
        after_source: &NativePerformanceReceivingSource,
        original: &PreparedAcousticReceiving,
        native_segment_origin: u64,
    ) -> Result<PreparedAcousticReceiving, String> {
        let before: AcousticConfiguration =
            serde_json::from_value(original.packet["configuration"].clone())
                .map_err(|e| e.to_string())?;
        let old_source = after_source
            .clone()
            .with_acoustic_configuration(before.clone())?;
        original.validate_current(self, current, &old_source, original.history_origin_sample)?;
        let after = after_source
            .acoustic_configuration()
            .ok_or("receiver update has no original native configuration")?;
        if native_segment_origin < original.origin_sample
            || after.revision <= before.revision
            || after.source_ref != before.source_ref
            || after.source_motion_ref != before.source_motion_ref
            || !after
                .source_translation_metres
                .iter()
                .zip(before.source_translation_metres)
                .all(|(a, b)| a.to_bits() == b.to_bits())
            || !after
                .source_velocity_metres_per_second
                .iter()
                .zip(before.source_velocity_metres_per_second)
                .all(|(a, b)| a.to_bits() == b.to_bits())
        {
            return Err("receiver update changed original emitter history or revision".into());
        }
        self.prepare_acoustic_receiving_segment(
            current,
            after_source,
            original.history_origin_sample,
            native_segment_origin,
        )
    }
    /// Pure native candidate preparation for an authored receiver edit. The
    /// caller retains the complete returned assets through the existing Scene
    /// CAS. Only a subsequently selected closed native lease can apply it.
    pub fn prepare_acoustic_receiver_assets(
        &self,
        current: &CoupledBasis,
        after_source: &NativePerformanceReceivingSource,
        native_segment_origin: u64,
    ) -> Result<PreparedAcousticReceiverUpdate, String> {
        self.validate_current(current)?;
        let retained = &self.source_assets["acoustic_receiving"];
        let packet = &retained["packet"];
        let before_configuration: AcousticConfiguration =
            serde_json::from_value(packet["configuration"].clone()).map_err(|e| e.to_string())?;
        let birth = decimal(&packet["history_origin_sample"])?;
        let before_origin = decimal(&packet["origin_sample"])?;
        let before_source = after_source
            .clone()
            .with_acoustic_configuration(before_configuration)?;
        let original =
            self.prepare_acoustic_receiving_segment(current, &before_source, birth, before_origin)?;
        if original.snapshot() != *retained {
            return Err("original operative acoustic source cannot be regenerated".into());
        }
        let before = before_source.prepare_current(self, current, before_origin)?;
        before.validate_current(&before_source, self, current, before_origin)?;
        if before.snapshot()? != self.source_assets["current_receiving"]
            || before.context().snapshot()? != self.source_assets["source_context"]
            || *before.source_inputs() != self.source_assets["receiving_source_inputs"]
            || before.definition().snapshot()? != self.source_assets["receiving_definition"]
        {
            return Err("original operative receiving/context/source assets differ".into());
        }
        let after = self.prepare_acoustic_receiver_update(
            current,
            after_source,
            &original,
            native_segment_origin,
        )?;
        let admitted = after_source.prepare_current(self, current, native_segment_origin)?;
        admitted.validate_current(after_source, self, current, native_segment_origin)?;
        after.validate_current(self, current, after_source, birth)?;
        let mut after_assets = self.source_assets.clone();
        after_assets["source_context"] = admitted.context().snapshot()?;
        after_assets["receiving_source_inputs"] = admitted.source_inputs().clone();
        after_assets["receiving_definition"] = admitted.definition().snapshot()?;
        after_assets["current_receiving"] = admitted.snapshot()?;
        after_assets["acoustic_receiving"] = after.snapshot();
        Ok(PreparedAcousticReceiverUpdate {
            before_assets: self.source_assets.clone(),
            after_assets,
            original,
            after,
            configuration: after_source
                .acoustic_configuration()
                .ok_or("after acoustic configuration missing")?
                .clone(),
            native_boundary: self
                .reading()
                .ok_or("actual acoustic receiver preparation boundary absent")?
                .clone(),
            cursor: native_segment_origin,
        })
    }
    /// Uses the existing copied actual stopped boundary for candidate source
    /// preparation. Only final native numerical guard admits this cursor.
    pub(crate) fn prepare_stopped_acoustic_receiver_assets(
        &self,
        current: &CoupledBasis,
        after_source: &NativePerformanceReceivingSource,
    ) -> Result<PreparedAcousticReceiverUpdate, String> {
        let reading = self
            .reading()
            .ok_or("actual acoustic owner reading absent")?;
        if !matches!(
            reading["device"]["state"].as_str(),
            Some("closed" | "prepared")
        ) {
            return Err("actual stopped acoustic receiver preparation required".into());
        }
        let cursor = decimal(&reading["samples_elapsed"])?;
        if reading["physical"]["samples_elapsed"].as_str() != Some(cursor.to_string().as_str()) {
            return Err("actual acoustic preparation body/output cursor differs".into());
        }
        self.prepare_acoustic_receiver_assets(current, after_source, cursor)
    }
    /// The first native stopped boundary survives Scene CAS. A fresh candidate
    /// cannot silently accept intervening queue/epoch/source/body changes.
    pub(crate) fn validate_stopped_acoustic_receiver_candidate(
        &self,
        current: &CoupledBasis,
        after_source: &NativePerformanceReceivingSource,
        candidate: &PreparedAcousticReceiverUpdate,
    ) -> Result<(), String> {
        self.validate_current(current)?;
        if self.source_assets != candidate.before_assets {
            return Err("operative source changed after acoustic candidate preparation".into());
        }
        let fresh = self.prepare_stopped_acoustic_receiver_assets(current, after_source)?;
        if fresh.before_assets != candidate.before_assets
            || fresh.after_assets != candidate.after_assets
            || fresh.original.snapshot() != candidate.original.snapshot()
            || fresh.after.snapshot() != candidate.after.snapshot()
            || fresh.native_boundary != candidate.native_boundary
            || fresh.cursor != candidate.cursor
        {
            return Err(
                "complete native acoustic candidate/source/boundary currentness differs".into(),
            );
        }
        Ok(())
    }
    /// Same existing private source reader selects the exact authored AFTER
    /// assets. It is not an original->candidate grant. The actual owner also
    /// checks its original operative source and stopped output before mutation.
    /// No additional inspect/drain is performed: the final Control guard
    /// validates this existing copied cursor and its full actual ring.
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub(crate) fn replace_prepared_acoustic_receiving(
        &mut self,
        current: &CoupledBasis,
        after_source: &NativePerformanceReceivingSource,
        session: &mut CoupledFieldSession,
        candidate: &PreparedAcousticReceiverUpdate,
        source_lease: &NativeActSourceLease<'_>,
    ) -> Result<Value, AcousticRefusal> {
        self.validate_current(current)?;
        let instance = self.binding.determination()["identity"]["instance"]
            .as_str()
            .ok_or("actual acoustic performance instance absent")?
            .to_owned();
        source_lease.validate_source_assets(&instance, candidate.source_assets())?;
        self.validate_stopped_acoustic_receiver_candidate(current, after_source, candidate)?;
        let fresh = self.prepare_stopped_acoustic_receiver_assets(current, after_source)?;
        let reading = self
            .reading()
            .ok_or("actual acoustic stopped reading absent")?;
        if !matches!(
            reading["device"]["state"].as_str(),
            Some("closed" | "prepared")
        ) || reading["samples_elapsed"].as_str() != Some(candidate.cursor.to_string().as_str())
            || reading["physical"]["samples_elapsed"].as_str()
                != Some(candidate.cursor.to_string().as_str())
        {
            return Err("actual stopped acoustic receiver cursor differs".into());
        }
        let before_packet = candidate.original.packet();
        let before_manifest = &reading["receiving_transport"]["manifest"];
        let before_config = &before_packet["configuration"];
        for (field, expected) in [
            ("context", &before_packet["context"]["context"]["reference"]),
            (
                "receiver",
                &before_packet["context"]["receiver"]["reference"],
            ),
            ("source_motion", &before_config["source_motion_ref"]),
            ("receiver_motion", &before_config["receiver_motion_ref"]),
            ("policy", &before_config["policy_ref"]),
            ("policy_revision", &before_config["policy_revision"]),
            ("origin_sample", &before_packet["origin_sample"]),
            ("end_sample", &before_packet["end_sample"]),
            (
                "history_origin_sample",
                &before_packet["history_origin_sample"],
            ),
        ] {
            if before_manifest[field] != *expected {
                return Err("actual original operative acoustic manifest differs".into());
            }
        }
        // Retain the exact real BEFORE cut; no imported acknowledgement can
        // replace the held owner/source/currentness checks above.
        let before_manifest = before_manifest.clone();
        let before_epoch = reading["transport_epoch"].clone();
        let before_sequence = reading["accepted_sequence"].clone();
        // Clone every source asset/packet before the performative exchange.
        // A refused operation publishes no authored/operative source relabel.
        let after_assets = candidate.after_assets.clone();
        let mut request = self.raw("receiving-transport-replace")?;
        request["before_acoustic"] = candidate.original.packet().clone();
        request["prepared_acoustic"] = candidate.after.packet().clone();
        request["current_acoustic"] = fresh.after.packet().clone();
        request["expected_sample"] = json!(candidate.cursor.to_string());
        let pulse = self
            .exchange_retaining_parsed_pulse(session, request)
            .map_err(|(reason, native_pulse)| AcousticRefusal {
                reason,
                native_pulse,
            })?;
        let checked = (|| -> Result<(), String> {
            if pulse["accepted"] != true {
                return Err(pulse["reason"]
                    .as_str()
                    .unwrap_or("native acoustic receiver replacement refused")
                    .into());
            }
            let reading = &pulse["reading"];
            let after_packet = candidate.after.packet();
            let manifest = &reading["receiving_transport"]["manifest"];
            let acknowledgement = &pulse["payload"]["receiving_replacement"];
            if acknowledgement["schema"] != "ql.native-receiving-replacement/v1"
                || acknowledgement["sample"].as_str() != Some(candidate.cursor.to_string().as_str())
                || acknowledgement["transport_epoch"] != before_epoch
                || acknowledgement["accepted_sequence"] != before_sequence
                || acknowledgement["before_manifest"] != before_manifest
                || acknowledgement["after_manifest"] != *manifest
                || reading["transport_epoch"] != before_epoch
                || reading["accepted_sequence"] != before_sequence
            {
                return Err("actual stopped receiver replacement acknowledgement differs".into());
            }
            if reading["samples_elapsed"].as_str() != Some(candidate.cursor.to_string().as_str())
                || reading["physical"]["samples_elapsed"].as_str()
                    != Some(candidate.cursor.to_string().as_str())
                || reading["receiving_transport"]["samples_elapsed"].as_str()
                    != Some(candidate.cursor.to_string().as_str())
            {
                return Err("actual applied acoustic receiver/source cursor differs".into());
            }
            for (field, expected) in [
                ("context", &after_packet["context"]["context"]["reference"]),
                (
                    "receiver",
                    &after_packet["context"]["receiver"]["reference"],
                ),
                (
                    "source_motion",
                    &after_packet["configuration"]["source_motion_ref"],
                ),
                (
                    "receiver_motion",
                    &after_packet["configuration"]["receiver_motion_ref"],
                ),
                ("policy", &after_packet["configuration"]["policy_ref"]),
                (
                    "policy_revision",
                    &after_packet["configuration"]["policy_revision"],
                ),
                ("origin_sample", &after_packet["origin_sample"]),
                ("end_sample", &after_packet["end_sample"]),
                (
                    "history_origin_sample",
                    &after_packet["history_origin_sample"],
                ),
            ] {
                if manifest[field] != *expected {
                    return Err("actual after acoustic receiver manifest differs".into());
                }
            }
            candidate.after.validate_current(
                self,
                current,
                after_source,
                candidate.after.history_origin_sample,
            )?;
            source_lease.validate_source_assets(&instance, &after_assets)?;
            Ok(())
        })();
        match checked {
            Ok(()) => {
                self.source_assets = after_assets;
                Ok(pulse)
            }
            Err(reason) => {
                // An accepted but invalid/currentness-lost reply requires the
                // existing native owner to hold; preserve its complete pulse.
                let reason = if pulse["accepted"] == true {
                    session.performance_invalidate(&reason)
                } else {
                    reason
                };
                Err(AcousticRefusal {
                    reason,
                    native_pulse: Some(pulse),
                })
            }
        }
    }
    /// Pure initial preparation before the authored Scene CAS. All original
    /// source/body/context inputs are replayed; no exchange, pulse drain or
    /// source publication takes place. The copied boundary is rechecked at
    /// installation and the C++ owner admits its actual stopped cursor.
    pub(crate) fn prepare_stopped_acoustic_installation_assets(
        &self,
        current: &CoupledBasis,
        original_source: &NativePerformanceReceivingSource,
        after_source: &NativePerformanceReceivingSource,
    ) -> Result<PreparedAcousticInstallation, String> {
        self.validate_current(current)?;
        let reading = self
            .reading()
            .ok_or("actual acoustic owner reading absent")?;
        if !matches!(
            reading["device"]["state"].as_str(),
            Some("closed" | "prepared")
        ) || reading["receiving_transport"].is_object()
            || self.source_assets.get("acoustic_receiving").is_some()
            || original_source.acoustic_configuration().is_some()
        {
            return Err(
                "initial acoustic candidate requires actual stopped owner without a receiver"
                    .into(),
            );
        }
        let cursor = decimal(&reading["samples_elapsed"])?;
        if reading["physical"]["samples_elapsed"].as_str() != Some(cursor.to_string().as_str()) {
            return Err("initial acoustic body/output cursor differs".into());
        }
        let before_cursor = decimal(
            &self.source_assets["current_receiving"]["native_admission"]["operation"]["native_sample"],
        )?;
        let original = original_source.prepare_current(self, current, before_cursor)?;
        original.validate_current(original_source, self, current, before_cursor)?;
        if original.snapshot()? != self.source_assets["current_receiving"]
            || original.context().snapshot()? != self.source_assets["source_context"]
            || *original.source_inputs() != self.source_assets["receiving_source_inputs"]
            || original.definition().snapshot()? != self.source_assets["receiving_definition"]
        {
            return Err("full original operative acoustic source/context differs".into());
        }
        // An initial receiver may add only its typed acoustic configuration;
        // a valid other person/context/occasion/source is still a substitution.
        let mut after_inputs = after_source.source_inputs()?;
        let added = after_inputs
            .as_object_mut()
            .ok_or("native source inputs absent")?
            .remove("acoustic_receiving")
            .ok_or("initial acoustic configuration absent")?;
        let configuration = after_source
            .acoustic_configuration()
            .ok_or("initial acoustic configuration absent")?
            .clone();
        if after_inputs != *original.source_inputs()
            || added != serde_json::to_value(&configuration).map_err(|e| e.to_string())?
        {
            return Err("initial acoustic candidate changed original source inputs".into());
        }
        let prepared = self.prepare_acoustic_receiving_source(current, after_source, cursor)?;
        let admitted = after_source.prepare_current(self, current, cursor)?;
        admitted.validate_current(after_source, self, current, cursor)?;
        admitted
            .context()
            .validate_binding(current, self.binding())?;
        prepared.validate_current(self, current, after_source, cursor)?;
        let mut after_assets = self.source_assets.clone();
        after_assets["source_context"] = admitted.context().snapshot()?;
        after_assets["receiving_source_inputs"] = admitted.source_inputs().clone();
        after_assets["receiving_definition"] = admitted.definition().snapshot()?;
        after_assets["current_receiving"] = admitted.snapshot()?;
        after_assets["acoustic_receiving"] = prepared.snapshot();
        Ok(PreparedAcousticInstallation {
            before_assets: self.source_assets.clone(),
            after_assets,
            prepared,
            configuration,
            boundary: reading.clone(),
            cursor,
        })
    }
    pub(crate) fn validate_stopped_acoustic_installation_candidate(
        &self,
        current: &CoupledBasis,
        original_source: &NativePerformanceReceivingSource,
        after_source: &NativePerformanceReceivingSource,
        candidate: &PreparedAcousticInstallation,
    ) -> Result<PreparedAcousticInstallation, String> {
        if self.source_assets != candidate.before_assets {
            return Err("operative source changed after initial acoustic preparation".into());
        }
        let fresh = self.prepare_stopped_acoustic_installation_assets(
            current,
            original_source,
            after_source,
        )?;
        if fresh.before_assets != candidate.before_assets
            || fresh.after_assets != candidate.after_assets
            || fresh.prepared.snapshot() != candidate.prepared.snapshot()
            || fresh.boundary != candidate.boundary
            || fresh.cursor != candidate.cursor
        {
            return Err("full initial acoustic candidate or native boundary changed".into());
        }
        Ok(fresh)
    }
    /// The fresh closed current Scene/recorded source lease must select the
    /// COMPLETE authored after-assets. Original resident state is independently
    /// requalified before the actual stopped native installation. Publication
    /// follows a genuine accepted, current reply; all refusal pulses survive.
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub(crate) fn install_acoustic_candidate(
        &mut self,
        current: &CoupledBasis,
        original_source: &NativePerformanceReceivingSource,
        after_source: &NativePerformanceReceivingSource,
        session: &mut CoupledFieldSession,
        candidate: &PreparedAcousticInstallation,
        source_lease: &NativeActSourceLease<'_>,
    ) -> Result<Value, AcousticRefusal> {
        self.validate_current(current)?;
        let instance = self.binding.determination()["identity"]["instance"]
            .as_str()
            .ok_or("actual acoustic performance instance absent")?
            .to_owned();
        source_lease.validate_source_assets(&instance, candidate.source_assets())?;
        let fresh = self.validate_stopped_acoustic_installation_candidate(
            current,
            original_source,
            after_source,
            candidate,
        )?;
        let after_assets = candidate.after_assets.clone();
        let mut request = self.raw("receiving-transport-install")?;
        request["prepared_acoustic"] = candidate.prepared.packet().clone();
        request["current_acoustic"] = fresh.prepared.packet().clone();
        request["expected_sample"] = json!(candidate.cursor.to_string());
        let pulse = self
            .exchange_retaining_parsed_pulse(session, request)
            .map_err(|(reason, native_pulse)| AcousticRefusal {
                reason,
                native_pulse,
            })?;
        let checked = (|| -> Result<(), String> {
            if pulse["accepted"] != true {
                return Err(pulse["reason"]
                    .as_str()
                    .unwrap_or("native initial acoustic install refused")
                    .into());
            }
            let reading = &pulse["reading"];
            let packet = candidate.prepared.packet();
            let manifest = &reading["receiving_transport"]["manifest"];
            if reading["transport_epoch"] != candidate.boundary["transport_epoch"]
                || reading["accepted_sequence"] != candidate.boundary["accepted_sequence"]
                || reading["samples_elapsed"].as_str()
                    != Some(candidate.cursor.to_string().as_str())
                || reading["physical"]["samples_elapsed"].as_str()
                    != Some(candidate.cursor.to_string().as_str())
                || reading["receiving_transport"]["samples_elapsed"].as_str()
                    != Some(candidate.cursor.to_string().as_str())
            {
                return Err("actual initial acoustic output/queue/epoch boundary differs".into());
            }
            for (name, expected) in [
                ("context", &packet["context"]["context"]["reference"]),
                ("receiver", &packet["context"]["receiver"]["reference"]),
                (
                    "source_motion",
                    &packet["configuration"]["source_motion_ref"],
                ),
                (
                    "receiver_motion",
                    &packet["configuration"]["receiver_motion_ref"],
                ),
                ("policy", &packet["configuration"]["policy_ref"]),
                (
                    "policy_revision",
                    &packet["configuration"]["policy_revision"],
                ),
                ("standing", &packet["configuration"]["standing"]),
                ("origin_sample", &packet["origin_sample"]),
                ("end_sample", &packet["end_sample"]),
                ("history_origin_sample", &packet["history_origin_sample"]),
            ] {
                if manifest[name] != *expected {
                    return Err(
                        "actual initial acoustic manifest differs from original native preparation"
                            .into(),
                    );
                }
            }
            candidate
                .prepared
                .validate_current(self, current, after_source, candidate.cursor)?;
            source_lease.validate_source_assets(&instance, &after_assets)?;
            Ok(())
        })();
        match checked {
            Ok(()) => {
                self.source_assets = after_assets;
                Ok(pulse)
            }
            Err(reason) => {
                let reason = if pulse["accepted"] == true {
                    session.performance_invalidate(&reason)
                } else {
                    reason
                };
                Err(AcousticRefusal {
                    reason,
                    native_pulse: Some(pulse),
                })
            }
        }
    }
    /// Existing source owner stages a reading, not a performative grant. Full
    /// source assets are then retained through the existing C Scene/Act CAS.
    /// No output, force, q/v or callback clock advances in this operation.
    pub(crate) fn stage_acoustic_receiving(
        &mut self,
        current: &CoupledBasis,
        source: &NativePerformanceReceivingSource,
        session: &mut CoupledFieldSession,
    ) -> Result<Value, AcousticRefusal> {
        self.validate_current(current)?;
        let request = self.raw("inspect")?;
        let pulse = self
            .exchange_retaining_parsed_pulse(session, request)
            .map_err(|(reason, native_pulse)| AcousticRefusal {
                reason,
                native_pulse,
            })?;
        let result = (|| -> Result<PreparedAcousticReceiving, String> {
            if pulse["accepted"] != true || pulse["reading"]["device"]["state"] == "running" {
                return Err("actual stopped acoustic preparation required".into());
            }
            if pulse["reading"]["receiving_transport"].is_object() {
                return Err(
                    "installed acoustic receiving requires prepared history-preserving replacement"
                        .into(),
                );
            }
            let cursor = decimal(&pulse["reading"]["samples_elapsed"])?;
            if pulse["reading"]["physical"]["samples_elapsed"].as_str()
                != Some(cursor.to_string().as_str())
            {
                return Err("native acoustic preparation detached from actual P cursor".into());
            }
            self.prepare_acoustic_receiving_source(current, source, cursor)
        })();
        let staged = (|| -> Result<(), String> {
            let prepared = result?;
            let birth = decimal(&prepared.packet["origin_sample"])?;
            let admitted = source.prepare_current(self, current, birth)?;
            // Complete every fallible producer/snapshot operation before the
            // sole publication. A refused stage cannot partially relabel the
            // resident source/context while keeping an older audio receiver.
            let mut assets = self.source_assets.clone();
            assets["source_context"] = admitted.context().snapshot()?;
            assets["receiving_source_inputs"] = admitted.source_inputs().clone();
            assets["receiving_definition"] = admitted.definition().snapshot()?;
            assets["current_receiving"] = admitted.snapshot()?;
            assets["acoustic_receiving"] = prepared.snapshot();
            admitted.validate_current(source, self, current, birth)?;
            admitted
                .context()
                .validate_binding(current, self.binding())?;
            prepared.validate_current(self, current, source, birth)?;
            self.source_assets = assets;
            Ok(())
        })();
        match staged {
            Ok(()) => Ok(pulse),
            Err(reason) => Err(AcousticRefusal {
                reason,
                native_pulse: Some(pulse),
            }),
        }
    }
    /// Only the SAME closed selected-Act lease can install staged original
    /// receiver inputs before play. Config/manifest JSON never grants it.
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub(crate) fn install_prepared_acoustic_receiving(
        &mut self,
        current: &CoupledBasis,
        source: &NativePerformanceReceivingSource,
        session: &mut CoupledFieldSession,
        act_lease: &NativeActSourceLease<'_>,
    ) -> Result<Value, AcousticRefusal> {
        self.validate_current(current)?;
        let instance = self.binding.determination()["identity"]["instance"]
            .as_str()
            .ok_or("actual acoustic performance instance absent")?
            .to_owned();
        act_lease.validate_source_assets(&instance, self.source_assets())?;
        let original = self.source_assets["acoustic_receiving"].clone();
        let birth = decimal(&original["packet"]["origin_sample"])?;
        let prepared = self.prepare_acoustic_receiving_source(current, source, birth)?;
        if prepared.snapshot() != original {
            return Err(
                "original acoustic source asset cannot be independently regenerated".into(),
            );
        }
        let mut request = self.raw("receiving-transport-install")?;
        request["prepared_acoustic"] = prepared.packet.clone();
        request["current_acoustic"] = prepared.packet.clone();
        request["expected_sample"] = json!(birth.to_string());
        let pulse = self
            .exchange_retaining_parsed_pulse(session, request)
            .map_err(|(reason, native_pulse)| AcousticRefusal {
                reason,
                native_pulse,
            })?;
        let checked = (|| -> Result<(), String> {
            if pulse["accepted"] != true {
                return Err(pulse["reason"]
                    .as_str()
                    .unwrap_or("native acoustic install refused")
                    .into());
            }
            if pulse["reading"]["samples_elapsed"].as_str() != Some(birth.to_string().as_str())
                || pulse["reading"]["physical"]["samples_elapsed"].as_str()
                    != Some(birth.to_string().as_str())
                || pulse["reading"]["receiving_transport"]["manifest"]["context"]
                    != source.return_context().context.reference
                || pulse["reading"]["receiving_transport"]["manifest"]["receiver"]
                    != source.return_context().receiver.reference
            {
                return Err(
                    "actual native acoustic receiver/output/source admission differs".into(),
                );
            }
            prepared.validate_current(self, current, source, birth)?;
            act_lease.validate_source_assets(&instance, self.source_assets())?;
            Ok(())
        })();
        match checked {
            Ok(()) => Ok(pulse),
            Err(reason) => Err(AcousticRefusal {
                reason,
                native_pulse: Some(pulse),
            }),
        }
    }
}
