//! Private timing factory of the resident native performance owner. The
//! selected Act/lease capability comes only from C's closed owner operation;
//! JSON evidence, instance names and imported timing epochs cannot mint it.
use super::*;
use crate::continuous::performance_act_bridge::NativeActSourceLease;
use crate::continuous::performance_receiving::NativePerformanceReceivingSource;
use crate::procedural_composition::TimingBinding;
use crate::procedural_conduct::NativePosition;
use crate::procedural_timing::NativeTimingWitness;

/// Native selectors, not imported clock or queue receipts. This enum has no
/// Deserialize implementation or public host/browser command constructor.
pub(crate) enum NativeTimingMoment {
    Boundary,
    Score(u64),
    Clock(u64),
    Applied(u64),
}
impl NativeTimingMoment {
    fn selector(self) -> (&'static str, u64) {
        match self {
            Self::Boundary => ("boundary", 0),
            Self::Score(sequence) => ("score", sequence),
            Self::Clock(sequence) => ("clock", sequence),
            Self::Applied(ordinal) => ("applied", ordinal),
        }
    }
}

/// All native applications/input journal entries from the same pulse remain
/// available to C/S. The witness is never serialized as an admission grant.
pub(crate) struct PreparedProceduralTiming {
    position: NativePosition,
    witness: NativeTimingWitness,
    native_pulse: Value,
}
impl PreparedProceduralTiming {
    pub(crate) fn position(&self) -> &NativePosition {
        &self.position
    }
    pub(crate) fn witness(&self) -> &NativeTimingWitness {
        &self.witness
    }
    pub(crate) fn native_pulse(&self) -> &Value {
        &self.native_pulse
    }
}

/// Original owner domain descriptor from ONE qualified actual native boundary.
/// No Deserialize/Serialize/Clone/Default or caller binding constructor.
/// The request cursor is the native admission horizon; position remains the
/// independently copied committed P/output cursor from that SAME pulse.
pub(crate) struct PreparedProceduralTimingDescriptor {
    binding: TimingBinding,
    position: NativePosition,
    native_fact: Value,
    native_pulse: Value,
}
impl PreparedProceduralTimingDescriptor {
    pub(crate) fn binding(&self) -> &TimingBinding {
        &self.binding
    }
    pub(crate) fn position(&self) -> &NativePosition {
        &self.position
    }
    pub(crate) fn native_pulse(&self) -> &Value {
        &self.native_pulse
    }
}

/// A refused observation never drops a pulse already drained by the SAME
/// native worker. The private C/S caller retains native_pulse even though it
/// receives no operative witness. This type has no serde/import constructor.
pub(crate) struct NativeTimingRefusal {
    reason: String,
    native_pulse: Option<Value>,
}
impl NativeTimingRefusal {
    pub(crate) fn reason(&self) -> &str {
        &self.reason
    }
    pub(crate) fn native_pulse(&self) -> Option<&Value> {
        self.native_pulse.as_ref()
    }
}
impl From<String> for NativeTimingRefusal {
    fn from(reason: String) -> Self {
        Self {
            reason,
            native_pulse: None,
        }
    }
}
impl From<&str> for NativeTimingRefusal {
    fn from(reason: &str) -> Self {
        reason.to_owned().into()
    }
}

impl PerformanceOwner {
    /// Called while the SAME closed selected-Act/lease operation holds this
    /// owner exclusively. Source and receiving inputs execute their actual
    /// native producers again before and after the existing worker exchange.
    /// No independent clock, queue, q/v integration or opaque Value grant.
    fn prepare_procedural_timing_descriptor(
        &mut self,
        current: &CoupledBasis,
        source: &NativePerformanceReceivingSource,
        session: &mut CoupledFieldSession,
        act_lease: &NativeActSourceLease<'_>,
        moment: NativeTimingMoment,
    ) -> Result<PreparedProceduralTimingDescriptor, NativeTimingRefusal> {
        self.validate_current(current)?;
        let instance = self.binding.determination()["identity"]["instance"]
            .as_str()
            .ok_or("native timing instance absent")?
            .to_owned();
        act_lease.validate_source_assets(&instance, self.source_assets())?;
        let retained_cursor = decimal(
            &self.source_assets["current_receiving"]["native_admission"]["operation"]["native_sample"],
        )?;
        let receiving = source.prepare_current(self, current, retained_cursor)?;
        receiving.validate_current(source, self, current, retained_cursor)?;
        if receiving.snapshot()? != self.source_assets["current_receiving"] {
            return Err("procedural timing lost exact current original receiving source".into());
        }
        let (selected, ordinal) = moment.selector();
        let mut request = self.raw("timing")?;
        request["moment"] = json!(selected);
        request["ordinal"] = json!(ordinal.to_string());
        let pulse = self
            .exchange_retaining_parsed_pulse(session, request)
            .map_err(|(reason, native_pulse)| NativeTimingRefusal {
                reason,
                native_pulse,
            })?;
        // All failure paths AFTER this real exchange retain its complete pulse.
        // A reason-only error would discard original committed applications and
        // input history; the next Inspect cannot recreate that drained cohort.
        let prepared = (|| -> Result<(NativePosition, TimingBinding), String> {
            if pulse["accepted"] != true {
                return Err(pulse["reason"]
                    .as_str()
                    .unwrap_or("native timing fact refused")
                    .into());
            }
            let fact = &pulse["payload"]["timing_fact"];
            let binding = &fact["binding"];
            let domain = binding
                .as_object()
                .ok_or("native timing domain descriptor absent")?;
            if domain.len() != 5
                || [
                    "owner_ref",
                    "domain",
                    "epoch_ref",
                    "requested_cursor",
                    "time_mapping_ref",
                ]
                .iter()
                .any(|key| !domain.contains_key(*key))
            {
                return Err(
                    "complete explicit native timing domain/mapping descriptor required".into(),
                );
            }
            for key in [
                "requested_cursor",
                "admitted_cursor",
                "applied_cursor",
                "committed_cursor",
                "admission_horizon",
                "native_position",
                "source",
                "transport_epoch",
                "queued",
                "device_callbacks_running",
            ] {
                if fact.get(key).is_none() {
                    return Err(format!("native timing fact lacks {key}"));
                }
            }
            if decimal(&binding["requested_cursor"])? != decimal(&fact["admission_horizon"])? {
                return Err(
                    "native timing domain descriptor has a different actual admission horizon"
                        .into(),
                );
            }
            if fact["schema"] != "ql.native-performance-timing-fact/v1"
                || fact["moment"] != selected
                || &fact["source"] != self.binding.determination()
                || binding["owner_ref"] != pulse["reading"]["session_ref"]
                || binding["epoch_ref"]
                    != format!(
                        "ql:performance/transport-epoch/{}",
                        decimal(&fact["transport_epoch"])?
                    )
                || binding["domain"] != "native_samples"
                || !binding["time_mapping_ref"].is_null()
                || fact["transport_epoch"] != pulse["reading"]["transport_epoch"]
                || fact["committed_cursor"] != pulse["reading"]["samples_elapsed"]
                || fact["committed_cursor"] != pulse["reading"]["physical"]["samples_elapsed"]
            {
                return Err(
                "exact original timing owner/domain/epoch/mapping or native source/cursor differs"
                    .into(),
            );
            }
            let position: NativePosition = serde_json::from_value(fact["native_position"].clone())
                .map_err(|e| e.to_string())?;
            let scope = &pulse["reading"]["scope"];
            if position.instance_ref != instance
                || scope["event_ref"] != position.event_ref
                || scope["subject_ref"] != position.subject_ref
                || pulse["reading"]["physical"]["source_generation"] != position.generation
                || pulse["reading"]["samples_elapsed"] != position.samples_elapsed
            {
                return Err(
                    "procedural position is detached from actual copied native body".into(),
                );
            }
            // Native fact dates remain separate and canonical; NONE is an
            // explicit unapplied boundary, never an inferred application.
            decimal(&fact["requested_cursor"])?;
            decimal(&fact["admitted_cursor"])?;
            if !fact["applied_cursor"].is_null() {
                decimal(&fact["applied_cursor"])?;
            }
            let native_binding = TimingBinding {
                owner_ref: binding["owner_ref"]
                    .as_str()
                    .ok_or("native timing owner absent")?
                    .into(),
                domain: binding["domain"]
                    .as_str()
                    .ok_or("native timing domain absent")?
                    .into(),
                epoch_ref: binding["epoch_ref"]
                    .as_str()
                    .ok_or("native timing epoch absent")?
                    .into(),
                requested_cursor: decimal(&binding["requested_cursor"])?,
                // Explicit field is checked NULL above. No omitted mapping is
                // granted identity standing, and no field clock is substituted.
                time_mapping_ref: None,
            };
            // Revalidate the same selected Act and current original receiving
            // source after its actual boundary/queue/application observation.
            act_lease.validate_source_assets(&instance, self.source_assets())?;
            let receiving_after = source.prepare_current(self, current, retained_cursor)?;
            receiving_after.validate_current(source, self, current, retained_cursor)?;
            if receiving_after.snapshot()? != self.source_assets["current_receiving"] {
                return Err(
                    "native source/receiving currentness changed during timing preparation".into(),
                );
            }
            Ok((position, native_binding))
        })();
        match prepared {
            Ok((position, binding)) => Ok(PreparedProceduralTimingDescriptor {
                position,
                binding,
                native_fact: pulse["payload"]["timing_fact"].clone(),
                native_pulse: pulse,
            }),
            Err(reason) => Err(NativeTimingRefusal {
                reason,
                native_pulse: Some(pulse),
            }),
        }
    }
    /// Native #281 performance branch of SourceBootstrap. The same held C/Act
    /// lease and current receiving producer admit the original owner binding;
    /// no caller TimingBinding or field/audio clock alias enters this method.
    pub(crate) fn procedural_timing_descriptor(
        &mut self,
        current: &CoupledBasis,
        source: &NativePerformanceReceivingSource,
        session: &mut CoupledFieldSession,
        act_lease: &NativeActSourceLease<'_>,
    ) -> Result<PreparedProceduralTimingDescriptor, NativeTimingRefusal> {
        self.prepare_procedural_timing_descriptor(
            current,
            source,
            session,
            act_lease,
            NativeTimingMoment::Boundary,
        )
    }

    pub(crate) fn procedural_timing_witness(
        &mut self,
        current: &CoupledBasis,
        source: &NativePerformanceReceivingSource,
        session: &mut CoupledFieldSession,
        original_binding: TimingBinding,
        act_lease: &NativeActSourceLease<'_>,
        moment: NativeTimingMoment,
    ) -> Result<PreparedProceduralTiming, NativeTimingRefusal> {
        let descriptor =
            self.prepare_procedural_timing_descriptor(current, source, session, act_lease, moment)?;
        let witness = (|| -> Result<NativeTimingWitness, String> {
            let current_binding = &descriptor.binding;
            if original_binding.owner_ref != current_binding.owner_ref
                || original_binding.domain != current_binding.domain
                || original_binding.epoch_ref != current_binding.epoch_ref
                || original_binding.time_mapping_ref != current_binding.time_mapping_ref
            {
                return Err(
                    "exact original native timing owner/domain/epoch/mapping differs".into(),
                );
            }
            // Authored requested_cursor remains unchanged. Actual requested,
            // resolved admission and committed application come from this fact.
            let fact = &descriptor.native_fact;
            let requested = decimal(&fact["requested_cursor"])?;
            let admitted = decimal(&fact["admitted_cursor"])?;
            let applied = if fact["applied_cursor"].is_null() {
                None
            } else {
                Some(decimal(&fact["applied_cursor"])?)
            };
            NativeTimingWitness::from_native_owner(
                original_binding,
                descriptor.position.clone(),
                requested,
                admitted,
                applied,
                fact.clone(),
                act_lease.evidence(),
            )
        })();
        match witness {
            Ok(witness) => Ok(PreparedProceduralTiming {
                position: descriptor.position,
                witness,
                native_pulse: descriptor.native_pulse,
            }),
            Err(reason) => Err(NativeTimingRefusal {
                reason,
                native_pulse: Some(descriptor.native_pulse),
            }),
        }
    }
}
