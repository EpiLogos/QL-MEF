//! Native source bootstrap on the existing private selected-Act FieldHost.
//! Public stdin, imported source JSON and read receipts cannot mint a lease.
use super::*;
use crate::procedural_source::{
    NativeBootstrapObservation, NativeSourceBootstrap, compile_native_source_bootstrap,
};

pub(crate) struct NativeSourceBootstrapRefusal {
    reason: String,
    native_receipt: Option<Value>,
    native_timing_pulse: Option<Value>,
}
impl NativeSourceBootstrapRefusal {
    pub(crate) fn reason(&self) -> &str {
        &self.reason
    }
    pub(crate) fn native_receipt(&self) -> Option<&Value> {
        self.native_receipt.as_ref()
    }
    pub(crate) fn native_timing_pulse(&self) -> Option<&Value> {
        self.native_timing_pulse.as_ref()
    }
}
impl FieldHost {
    /// Called only after the original HostRequest is admitted by the SAME
    /// native selected-Act route. C28 reads actual current Scene/Act; Root owns
    /// source-read issuance and final DocumentCAS binding retention.
    pub(crate) fn bootstrap_procedural_source(
        &mut self,
        input: &NativeSourceBootstrap,
        lease: &super::super::performance_act_bridge::NativeActSourceLease<'_>,
    ) -> Result<Value, NativeSourceBootstrapRefusal> {
        let mut native_receipt = None;
        let mut native_timing_pulse = None;
        let mut native_source_correspondence = None;
        let result = (|| -> Result<Value, String> {
            input.scene.validate()?;
            lease.validate_field_sources(
                &self.instance_ref,
                self.session.session().original_basis(),
                self.session.session().current_basis(),
            )?;
            // This genuine closed-reader comparison includes complete source
            // projection, principal/contributor read basis, source-read receipt,
            // opaque locus/profile and SAME native Document/CAS. Merely checking
            // supplied fingerprints would not qualify source origin.
            lease.validate_procedural_scene_read(
                &self.instance_ref,
                &input.scene,
                &input.authorship.contributors,
            )?;
            let source = self.retained_procedural_source_artifact()?;
            let (position, timing, receipt) = if self.performance.is_some() {
                // A prepared or sounding instrument keeps A/P/R as sole clock
                // owner. Never read a stale last_field as its timing boundary.
                let current = self.session.session().current_basis().clone();
                let receiving = self
                    .receiving_source
                    .as_ref()
                    .ok_or("actual native performance receiving source absent")?;
                let owner = self
                    .performance
                    .as_mut()
                    .ok_or("actual prepared performance owner disappeared")?;
                let descriptor = match owner.procedural_timing_descriptor(
                    &current,
                    receiving,
                    self.session.session_mut(),
                    lease,
                ) {
                    Ok(descriptor) => descriptor,
                    Err(refusal) => {
                        native_timing_pulse = refusal.native_pulse().cloned();
                        return Err(refusal.reason().to_owned());
                    }
                };
                native_timing_pulse = Some(descriptor.native_pulse().clone());
                native_source_correspondence = Some(descriptor.source_correspondence().clone());
                (
                    descriptor.position().clone(),
                    descriptor.binding().clone(),
                    Value::Null,
                )
            } else {
                let timing = match self.native_field_timing_descriptor(lease) {
                    Ok(timing) => timing,
                    Err(refusal) => {
                        native_receipt = refusal.native_receipt().cloned();
                        return Err(refusal.reason().to_owned());
                    }
                };
                let receipt = self.session.session().last_field().clone();
                native_receipt = Some(receipt.clone());
                let position = NativePosition::from_field(&self.instance_ref, &receipt)?;
                (position, timing, receipt)
            };
            let observed = NativeBootstrapObservation {
                position,
                timing,
                field_source: source.clone(),
                field_receipt: receipt,
                native_timing_pulse: native_timing_pulse.clone(),
                native_act_source: lease.evidence(),
            };
            let mut result = compile_native_source_bootstrap(input, &observed)?;
            if let Some(correspondence) = &native_source_correspondence {
                result["native_resident_source_correspondence"] = correspondence.clone();
                // The actual producer is part of the unchanged original pulse;
                // this read-only convenience copy cannot create a clock grant.
                result["native_timing_owner"] = native_timing_pulse
                    .as_ref()
                    .ok_or("same-pulse Management constructor missing")?["native_timing_owner"]
                    .clone();
            }
            if self.retained_procedural_source_artifact()? != source {
                return Err(
                    "native source bootstrap changed full original/current field source".into(),
                );
            }
            lease.validate_field_sources(
                &self.instance_ref,
                self.session.session().original_basis(),
                self.session.session().current_basis(),
            )?;
            lease.validate_procedural_scene_read(
                &self.instance_ref,
                &input.scene,
                &input.authorship.contributors,
            )?;
            Ok(result)
        })();
        result.map_err(|reason| NativeSourceBootstrapRefusal {
            reason,
            native_receipt,
            native_timing_pulse,
        })
    }
}
