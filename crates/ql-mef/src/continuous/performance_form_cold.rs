//! Cold Form/material source replay under the SAME privately selected C Act.
//! Complete original constructor/commands/preparations are regenerated without
//! installing, advancing or restoring P. Actual application custody is separate
//! from deterministic producer records and is verified by the selected lease.
use super::*;
use crate::continuous::performance_act_bridge::NativeActSourceLease;

/// One original source/body basis kept for complete native score compilation.
/// No Deserialize/Clone and no live P, epoch, checkpoint or source grant.
pub(crate) struct ReplayedPhysicalSourceFrame {
    owner: PerformanceOwner,
    current: CoupledBasis,
    source_sample: u64,
}
impl ReplayedPhysicalSourceFrame {
    pub(crate) fn owner(&self) -> &PerformanceOwner {
        &self.owner
    }
    pub(crate) fn current(&self) -> &CoupledBasis {
        &self.current
    }
    /// Actual Return producer for this original source/body frame. The C
    /// compiler still compares its entire original basis/episode/pitches.
    pub(crate) fn prepare_return(
        &self,
        source: &NativePerformanceReceivingSource,
        declared_seed: u64,
    ) -> Result<crate::musical_performance_return::MusicalPerformanceReturn, String> {
        let receiving = source.prepare_current(&self.owner, &self.current, self.source_sample)?;
        receiving.validate_current(source, &self.owner, &self.current, self.source_sample)?;
        if receiving.snapshot()? != self.owner.source_assets()["current_receiving"] {
            return Err("cold historical source Return lost actual receiving ownership".into());
        }
        crate::musical_performance_return::bind_performance_return(
            self.owner.binding(),
            source.original_occasion().cloned(),
            source.return_context().clone(),
            declared_seed,
        )
    }
    pub(crate) fn source_sample(&self) -> u64 {
        self.source_sample
    }
}

/// Keeps every actual old source/body binding until ALL original C pages have
/// compiled; only then may the existing host activate its final native body.
pub(crate) struct PreparedColdPhysicalSource {
    frames: Vec<ReplayedPhysicalSourceFrame>,
}
impl PreparedColdPhysicalSource {
    pub(crate) fn frames(&self) -> &[ReplayedPhysicalSourceFrame] {
        &self.frames
    }
    pub(crate) fn final_frame(&self) -> &ReplayedPhysicalSourceFrame {
        self.frames.last().expect("native replay always has origin")
    }
    pub(crate) fn into_final(self) -> (PerformanceOwner, CoupledBasis) {
        let frame = self
            .frames
            .into_iter()
            .last()
            .expect("native replay has origin");
        (frame.owner, frame.current)
    }
}

impl PerformanceOwner {
    /// The original FieldHost opening supplies `original`: the same native
    /// World/protected constructor. Neither final input nor source-origin JSON
    /// can choose it. The closed lease owns both complete selected source bytes
    /// and original stopped source applications retained by C; audio Kind6 is
    /// unrelated and cannot stand in for these real body transactions.
    pub(in crate::continuous) fn prepare_cold_native_physical_source(
        original: &CoupledBasis,
        instance: &str,
        source: &NativePerformanceReceivingSource,
        expected: &Value,
        original_applications: &[Value],
        lease: &NativeActSourceLease<'_>,
    ) -> Result<PreparedColdPhysicalSource, String> {
        lease.validate_source_assets(instance, expected)?;
        // This method is issued by B's same privately selected C source owner;
        // full equality is against its retained native source sidecar, not a
        // caller Vec, hash or checkpoint token. No source setter lives here.
        lease.validate_recorded_physical_applications(instance, original_applications)?;
        let prepared = Self::replay_cold_native_physical_source(
            original,
            instance,
            source,
            expected,
            original_applications,
        )?;
        lease.validate_source_assets(instance, prepared.final_frame().owner().source_assets())?;
        lease.validate_recorded_physical_applications(instance, original_applications)?;
        Ok(prepared)
    }

    /// Numerical/source compiler only. Production reaches it through the
    /// closed lease above; unit/library evidence never grants activation.
    fn replay_cold_native_physical_source(
        original: &CoupledBasis,
        instance: &str,
        source: &NativePerformanceReceivingSource,
        expected: &Value,
        original_applications: &[Value],
    ) -> Result<PreparedColdPhysicalSource, String> {
        if serde_json::to_vec(expected)
            .map_err(|e| e.to_string())?
            .len()
            > super::super::super::MAX_MESSAGE
            || serde_json::to_vec(original_applications)
                .map_err(|e| e.to_string())?
                .len()
                > super::super::super::MAX_MESSAGE
        {
            return Err(
                "complete cold physical source/history exceeds existing native transport bound"
                    .into(),
            );
        }
        if expected["schema"] != "ql.retained-performance-source-assets/v1"
            || expected["original_native_input"]
                != serde_json::to_value(&original.input).map_err(|e| e.to_string())?
        {
            return Err("cold physical source lost the actual original native constructor".into());
        }
        validate_form_lineage(original, original)?;
        let history = match expected.get("physical_transition_history") {
            Some(value) => value
                .as_array()
                .ok_or("complete physical source history absent")?
                .as_slice(),
            None => &[],
        };
        if history.len() != original_applications.len() {
            return Err("cold physical source lost an original native application".into());
        }
        if history.is_empty() {
            let owner = Self::prepare_cold_act_source(original, instance, source, expected)?;
            let source_sample = decimal(
                &expected["current_receiving"]["native_admission"]["operation"]["native_sample"],
            )?;
            return Ok(PreparedColdPhysicalSource {
                frames: vec![ReplayedPhysicalSourceFrame {
                    owner,
                    current: original.clone(),
                    source_sample,
                }],
            });
        }
        // The complete selected original native source retains its birth
        // configuration in the first actual producer record. Regenerate it
        // against the actual opening constructor; no after_current is origin.
        let first = &history[0];
        let config: PerformanceConfig =
            serde_json::from_value(first["before_configuration"].clone())
                .map_err(|e| e.to_string())?;
        let mut owner = Self::prepare(original, instance, config)?;
        let source_sample = decimal(
            &first["before_current_receiving"]["native_admission"]["operation"]["native_sample"],
        )?;
        let admitted = source.admit_current(&mut owner, original, source_sample)?;
        admitted.validate_current(source, &owner, original, source_sample)?;
        if admitted.snapshot()? != first["before_current_receiving"] {
            return Err("cold source original N9/context/occasion no longer replays".into());
        }
        owner.source_assets["receiving_source_inputs"] = admitted.source_inputs().clone();
        owner.source_assets["receiving_definition"] = admitted.definition().snapshot()?;
        owner.source_assets["current_receiving"] = admitted.snapshot()?;
        let historical_roles = expected
            .get("consumer_roles")
            .filter(|v| v.is_object())
            .ok_or("original native physical consumer roles absent")?;
        owner.source_assets["consumer_roles"] = historical_roles.clone();
        if first["before_acoustic"].is_object() {
            let birth = decimal(&first["before_acoustic"]["history_origin_sample"])?;
            let origin = decimal(&first["before_acoustic"]["origin_sample"])?;
            let acoustic =
                owner.prepare_acoustic_receiving_segment(original, source, birth, origin)?;
            acoustic.validate_current(&owner, original, source, birth)?;
            if acoustic.packet() != &first["before_acoustic"] {
                return Err(
                    "cold source original acoustic receiver segment no longer replays".into(),
                );
            }
            owner.source_assets["acoustic_receiving"] = acoustic.snapshot();
        } else if first["before_acoustic"] != Value::Null {
            return Err("cold source original acoustic packet has wrong type".into());
        }
        let mut current = original.clone();
        let mut frames = Vec::with_capacity(history.len() + 1);
        let mut previous_request = 0;
        let mut previous_sample = source_sample;
        let mut previous_epoch = 0;
        let mut previous_sequence = 0;
        let mut previous_eigenbasis: Option<Value> = None;
        for (index, (record, retained)) in history.iter().zip(original_applications).enumerate() {
            if retained.as_object().map(|v| v.len()) != Some(2)
                || retained["source"] != *record
                || record["schema"] != PHYSICAL_SOURCE_TRANSITION
                || record["edit_schema"] != PHYSICAL_EDIT
            {
                return Err(
                    "cold source has a detached or incomplete physical application record".into(),
                );
            }
            let request = decimal(&record["original_native_request_id"])?;
            let sample = decimal(&record["native_sample"])?;
            if request <= previous_request || sample < previous_sample {
                return Err(
                    "cold source original request/cursor order regressed or duplicated".into(),
                );
            }
            let authored: AuthoredNativePhysicalEdit =
                serde_json::from_value(record["authored_edit"].clone())
                    .map_err(|e| e.to_string())?;
            let mut descendant = owner.prepare_physical_source_descendant_at(
                &current, source, request, &authored, sample,
            )?;
            if descendant.source_record != *record {
                return Err("cold physical source lost full original M3 receipt/config/preparation/N9/recipe".into());
            }
            let pulse = &retained["native_application"];
            descendant.after_owner.validate_reply(pulse)?;
            let ack = &pulse["payload"]["physical_transition"];
            let epoch = decimal(&pulse["reading"]["transport_epoch"])?;
            let sequence = decimal(&pulse["reading"]["accepted_sequence"])?;
            if pulse["accepted"] != true
                || pulse["operation"] != "source-body-transition"
                || pulse["reading"]["samples_elapsed"] != record["native_sample"]
                || !matches!(
                    pulse["reading"]["device"]["state"].as_str(),
                    Some("closed" | "prepared")
                )
                || ack["schema"] != "ql.native-physical-source-application/v1"
                || ack["original_native_request_id"] != record["original_native_request_id"]
                || ack["native_sample"] != record["native_sample"]
                || ack["kind"] != record["kind"]
                || ack["policy"] != "project-corresponding-nodes"
                || decimal(&ack["before_body_revision"])? != owner.config.controls.body_revision
                || decimal(&ack["after_body_revision"])?
                    != descendant.after_owner.config.controls.body_revision
                || ack["before_native_preparation"] != record["before_native_preparation"]
                || ack["after_native_preparation"] != record["after_native_preparation"]
                || ack["after_eigenbasis_identity"]
                    != pulse["reading"]["physical"]["eigenbasis_identity"]
                || ack["before_eigenbasis_identity"]
                    .as_str()
                    .is_none_or(|v| v.is_empty() || v.len() > 2048)
                || ack["transport_epoch"] != pulse["reading"]["transport_epoch"]
                || ack["accepted_sequence"] != pulse["reading"]["accepted_sequence"]
                || epoch == 0
                || epoch < previous_epoch
                || (epoch == previous_epoch && sequence < previous_sequence)
                || previous_eigenbasis
                    .as_ref()
                    .is_some_and(|v| ack["before_eigenbasis_identity"] != *v)
            {
                return Err(
                    "cold source actual P/body application lost original owner/body/time/ordinal"
                        .into(),
                );
            }
            for field in [
                "before_energy_joules",
                "after_energy_joules",
                "external_work_joules",
            ] {
                if ack[field].as_f64().is_none_or(|v| !v.is_finite()) {
                    return Err("cold physical source lost actual energy/work application".into());
                }
            }
            if let Some(before) = &descendant.before_acoustic {
                let manifest = &pulse["reading"]["receiving_transport"]["manifest"];
                let packet = descendant
                    .after_acoustic
                    .as_ref()
                    .ok_or("cold acoustic AFTER packet absent")?;
                if manifest["body_revision"] != ack["after_body_revision"]
                    || manifest["history_origin_sample"] != before["history_origin_sample"]
                    || manifest["origin_sample"] != before["origin_sample"]
                    || manifest["end_sample"] != before["end_sample"]
                    || manifest["context"] != packet["context"]["context"]["reference"]
                    || manifest["receiver"] != packet["context"]["receiver"]["reference"]
                    || pulse["reading"]["receiving_transport"]["samples_elapsed"]
                        != record["native_sample"]
                {
                    return Err(
                        "cold physical source lost the original M4 receiver/history date".into(),
                    );
                }
            }
            let frame_sample = decimal(
                &owner.source_assets["current_receiving"]["native_admission"]["operation"]["native_sample"],
            )?;
            frames.push(ReplayedPhysicalSourceFrame {
                owner,
                current,
                source_sample: frame_sample,
            });
            descendant.after_owner.physical_source_history = original_applications[..=index]
                .iter()
                .map(|value| NativePhysicalTransitionRecord {
                    source: value["source"].clone(),
                    application: value["native_application"].clone(),
                })
                .collect();
            owner = descendant.after_owner;
            current = descendant.after_current;
            previous_request = request;
            previous_sample = sample;
            previous_epoch = epoch;
            previous_sequence = sequence;
            previous_eigenbasis = Some(ack["after_eigenbasis_identity"].clone());
        }
        if owner.source_assets != *expected
            || serde_json::to_value(&owner.immutable_source_origin().input)
                .map_err(|e| e.to_string())?
                != serde_json::to_value(&original.input).map_err(|e| e.to_string())?
        {
            return Err(
                "cold complete final physical source differs from original-owner replay".into(),
            );
        }
        owner.validate_current(&current)?;
        if owner.reading().is_some() || frames.iter().any(|frame| frame.owner.reading().is_some()) {
            return Err("pure cold source replay claimed native P residency/readback".into());
        }
        let source_sample = decimal(
            &owner.source_assets["current_receiving"]["native_admission"]["operation"]["native_sample"],
        )?;
        frames.push(ReplayedPhysicalSourceFrame {
            owner,
            current,
            source_sample,
        });
        Ok(PreparedColdPhysicalSource { frames })
    }
}

#[cfg(test)]
#[path = "performance_form_cold_tests.rs"]
mod tests;

#[path = "performance_acoustic_source_cold.rs"]
mod acoustic_history;
