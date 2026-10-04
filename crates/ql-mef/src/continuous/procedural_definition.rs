//! One original private definition request, one SAME C/R timing observation.
//! Staged state is committed only after full current native reader checks.
use super::*;
use crate::procedural_source::NativeBootstrapObservation;

impl FieldHost {
    pub(crate) fn execute_native_definition(
        &mut self,
        request: ConductRequest,
        lease: &super::super::performance_act_bridge::NativeActSourceLease<'_>,
        field_receipt: &mut Option<Value>,
        timing_pulse: &mut Option<Value>,
    ) -> Result<Value, String> {
        let bootstrap = match &request {
            ConductRequest::InstallPrepared { input } => &input.source_bootstrap,
            ConductRequest::SourceContinue { input } => &input.source_bootstrap,
            _ => return Err("private definition route changed its original action".into()),
        };
        bootstrap.scene.validate()?;
        lease.validate_procedural_scene_read(
            &self.instance_ref,
            &bootstrap.scene,
            &bootstrap.authorship.contributors,
        )?;
        let before_source = self.retained_procedural_source_artifact()?;
        let scene = lease.procedural_scene_consumer_fact(
            &self.instance_ref,
            &bootstrap.scene,
            &bootstrap.authorship.contributors,
        )?;
        let binding = self
            .procedural
            .request_timing(&request)?
            .ok_or("native original definition timing unavailable")?;
        let staged = if binding.domain == procedural_field_timing::NATIVE_FIELD_TIMING_DOMAIN {
            let actual = match self.field_procedural_timing_witness(binding, lease) {
                Ok(actual) => actual,
                Err(refusal) => {
                    *field_receipt = refusal.native_receipt().cloned();
                    return Err(refusal.reason().to_owned());
                }
            };
            // Keep a real drained/read result even if later source qualification refuses.
            *field_receipt = Some(actual.native_receipt().clone());
            let consumer = actual.procedural_source_consumer_fact()?;
            let observed = NativeBootstrapObservation {
                position: actual.position().clone(),
                timing: actual.witness().event_binding(),
                field_source: before_source.clone(),
                field_receipt: actual.native_receipt().clone(),
                native_timing_pulse: None,
                native_act_source: lease.evidence(),
            };
            let semantic_origin = self.procedural.source_continuation_origin(&request)?;
            let (source, contract) = self.compile_procedural_source_observed_for_definition(
                bootstrap,
                lease,
                &observed,
                &scene,
                &consumer,
                super::procedural_source_bootstrap::NativeDefinitionSourceContext {
                    native_source_correspondence: None,
                    semantic_origin: semantic_origin.as_ref(),
                },
            )?;
            match request {
                ConductRequest::InstallPrepared { input } => {
                    self.procedural.stage_prepared_install(
                        *input,
                        actual.position().clone(),
                        actual.witness(),
                        &source,
                        &contract,
                    )?
                }
                ConductRequest::SourceContinue { input } => {
                    self.procedural.stage_source_continuation(
                        *input,
                        actual.position().clone(),
                        actual.witness(),
                        &source,
                        &contract,
                    )?
                }
                _ => unreachable!(),
            }
        } else {
            let actual = match self.procedural_timing_witness(
                binding,
                lease,
                super::super::performance::NativeTimingMoment::Boundary,
            ) {
                Ok(actual) => actual,
                Err(refusal) => {
                    *timing_pulse = refusal.native_pulse().cloned();
                    return Err(refusal.reason().to_owned());
                }
            };
            *timing_pulse = Some(actual.native_pulse().clone());
            let consumer = actual.procedural_consumer_fact()?;
            let observed = NativeBootstrapObservation {
                position: actual.position().clone(),
                timing: actual.boundary_binding().clone(),
                field_source: before_source.clone(),
                field_receipt: Value::Null,
                native_timing_pulse: Some(actual.native_pulse().clone()),
                native_act_source: lease.evidence(),
            };
            let semantic_origin = self.procedural.source_continuation_origin(&request)?;
            let (source, contract) = self.compile_procedural_source_observed_for_definition(
                bootstrap,
                lease,
                &observed,
                &scene,
                &consumer,
                super::procedural_source_bootstrap::NativeDefinitionSourceContext {
                    native_source_correspondence: Some(actual.source_correspondence()),
                    semantic_origin: semantic_origin.as_ref(),
                },
            )?;
            match request {
                ConductRequest::InstallPrepared { input } => {
                    self.procedural.stage_prepared_install(
                        *input,
                        actual.position().clone(),
                        actual.witness(),
                        &source,
                        &contract,
                    )?
                }
                ConductRequest::SourceContinue { input } => {
                    self.procedural.stage_source_continuation(
                        *input,
                        actual.position().clone(),
                        actual.witness(),
                        &source,
                        &contract,
                    )?
                }
                _ => unreachable!(),
            }
        };
        // The source input is immutable in the staged checkpoint. It remains
        // borrowed here only for the genuine post-read; no mutation was installed.
        let scene_read = staged.source_bootstrap();
        lease.validate_procedural_scene_read(
            &self.instance_ref,
            &scene_read.scene,
            &scene_read.authorship.contributors,
        )?;
        lease.validate_field_sources(
            &self.instance_ref,
            self.session.session().original_basis(),
            self.session.session().current_basis(),
        )?;
        if self.retained_procedural_source_artifact()? != before_source
            || lease
                .procedural_scene_consumer_fact(
                    &self.instance_ref,
                    &scene_read.scene,
                    &scene_read.authorship.contributors,
                )?
                .constructor_fact()
                != scene.constructor_fact()
        {
            return Err("native source/Scene constructor changed before definition commit".into());
        }
        self.procedural.commit_definition(staged)
    }
}
