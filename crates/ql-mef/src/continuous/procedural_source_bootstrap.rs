//! Native source bootstrap on the existing private selected-Act FieldHost.
//! Public stdin, imported source JSON and read receipts cannot mint a lease.
use super::*;
use crate::procedural_source::{
    NativeBootstrapObservation, NativeSourceBootstrap, compile_native_source_bootstrap_registered,
};

pub(crate) struct NativeDefinitionSourceContext<'read, 'origin> {
    pub(crate) native_source_correspondence: Option<&'read Value>,
    pub(crate) semantic_origin:
        Option<&'read crate::procedural_conduct::definition::NativeDefinitionSourceOrigin<'origin>>,
}
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
            let scene_consumer = lease.procedural_scene_consumer_fact(
                &self.instance_ref,
                &input.scene,
                &input.authorship.contributors,
            )?;
            let (position, timing, receipt, timing_consumer) = if self.performance.is_some() {
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
                // Same descriptor, same original pulse, genuine R Management
                // constructor. No FIELD clock or extra Inspect in this branch.
                let consumer = descriptor.procedural_consumer_fact()?;
                (
                    descriptor.position().clone(),
                    descriptor.binding().clone(),
                    Value::Null,
                    consumer,
                )
            } else {
                let prepared = match self.prepare_native_field_timing_descriptor(lease) {
                    Ok(prepared) => prepared,
                    Err(refusal) => {
                        native_receipt = refusal.native_receipt().cloned();
                        return Err(refusal.reason().to_owned());
                    }
                };
                // Capture the original receipt BEFORE any consumer/source
                // qualification that can refuse after this real read.
                native_receipt = Some(prepared.native_receipt().clone());
                let consumer = prepared.procedural_source_consumer_fact()?;
                (
                    prepared.position().clone(),
                    prepared.witness().original_binding().clone(),
                    prepared.native_receipt().clone(),
                    consumer,
                )
            };
            let observed = NativeBootstrapObservation {
                position,
                timing,
                field_source: source.clone(),
                field_receipt: receipt,
                native_timing_pulse: native_timing_pulse.clone(),
                native_act_source: lease.evidence(),
            };
            self.compile_procedural_source_observed(
                input,
                lease,
                &observed,
                &scene_consumer,
                &timing_consumer,
                native_source_correspondence.as_ref(),
            )
            .map(|(source, _)| source)
        })();
        result.map_err(|reason| NativeSourceBootstrapRefusal {
            reason,
            native_receipt,
            native_timing_pulse,
        })
    }
    /// Reuse the canonical Source compiler with already-returned SAME C/R observations.
    /// Both bootstrap and definition continuation call this without a second timing read.
    pub(crate) fn compile_procedural_source_observed(
        &self,
        input: &NativeSourceBootstrap,
        lease: &super::super::performance_act_bridge::NativeActSourceLease<'_>,
        observed: &NativeBootstrapObservation,
        scene_consumer: &crate::procedural_consumers::NativeSceneConsumerFact,
        timing_consumer: &crate::procedural_consumers::NativeTimingConsumerFact,
        native_source_correspondence: Option<&Value>,
    ) -> Result<(Value, crate::procedural_consumers::NativeConsumerContract), String> {
        self.compile_procedural_source_observed_for_definition(
            input,
            lease,
            observed,
            scene_consumer,
            timing_consumer,
            NativeDefinitionSourceContext {
                native_source_correspondence,
                semantic_origin: None,
            },
        )
    }
    pub(crate) fn compile_procedural_source_observed_for_definition(
        &self,
        input: &NativeSourceBootstrap,
        lease: &super::super::performance_act_bridge::NativeActSourceLease<'_>,
        observed: &NativeBootstrapObservation,
        scene_consumer: &crate::procedural_consumers::NativeSceneConsumerFact,
        timing_consumer: &crate::procedural_consumers::NativeTimingConsumerFact,
        definition_source: NativeDefinitionSourceContext<'_, '_>,
    ) -> Result<(Value, crate::procedural_consumers::NativeConsumerContract), String> {
        let NativeDefinitionSourceContext {
            native_source_correspondence,
            semantic_origin,
        } = definition_source;
        let source = &observed.field_source;
        // The same original descriptor operation carries the complete
        // current source epoch. No extra Inspect/ordinal or JSON issuer.
        let performance_source_observation = self.retained_performance_source_observation()?;
        if let Some(correspondence) = native_source_correspondence {
            if performance_source_observation["physical_preparation"]
                != correspondence["physical_preparation"]
                || performance_source_observation["performance_sources"]["source_form_recipe"]
                    != correspondence["source_form_recipe"]
                || performance_source_observation["native_reading"]
                    != observed
                        .native_timing_pulse
                        .as_ref()
                        .ok_or("actual original timing pulse absent")?["reading"]
            {
                return Err("same bootstrap source/body epoch detached from its actual original timing pulse".into());
            }
        } else if !performance_source_observation.is_null() {
            return Err("bootstrap retained an acoustic/body source without its actual native correspondence".into());
        }
        let consumer_contract = crate::procedural_consumers::from_registered_native_consumers(
            &input.scene,
            &observed.position,
            &observed.timing,
            scene_consumer,
            timing_consumer,
            observed.native_timing_pulse.as_ref(),
        )?;
        let mut result = if semantic_origin.is_some() {
            crate::procedural_source::compile_native_source_bootstrap_registered_for_definition(
                input,
                observed,
                &consumer_contract,
                semantic_origin,
            )?
        } else {
            compile_native_source_bootstrap_registered(input, observed, &consumer_contract)?
        };
        // Retained exact getter diagnostics do not deserialize into facts.
        result["native_scene_constructor_fact"] = scene_consumer.constructor_fact().clone();
        result["native_timing_consumer_fact"] = timing_consumer.constructor_fact().clone();
        if let Some(correspondence) = native_source_correspondence {
            result["native_resident_source_correspondence"] = correspondence.clone();
            result["native_timing_owner"] = observed
                .native_timing_pulse
                .as_ref()
                .ok_or("same-pulse Management constructor missing")?["native_timing_owner"]
                .clone();
        }
        result["performance_source_observation"] = performance_source_observation.clone();
        if self.retained_performance_source_observation()? != performance_source_observation {
            return Err("same bootstrap changed its complete actual source and original application histories".into());
        }
        if self.retained_procedural_source_artifact()? != *source {
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
        let after_scene = lease.procedural_scene_consumer_fact(
            &self.instance_ref,
            &input.scene,
            &input.authorship.contributors,
        )?;
        if after_scene.constructor_fact() != scene_consumer.constructor_fact() {
            return Err("same native Scene construction changed during Source bootstrap".into());
        }
        Ok((result, consumer_contract))
    }
}
