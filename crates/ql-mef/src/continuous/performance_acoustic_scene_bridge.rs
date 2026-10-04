//! Closed current Scene acoustic prepare/apply; no imported source grants.
use super::*;
use crate::continuous::performance::AcousticConfiguration;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct NativeAcousticApplyOrigin {
    pub(super) original_native_request_id: String,
    pub(super) native_boundary: Value,
}

pub(super) fn validate_operands(request: &ActRequest) -> Result<(), String> {
    let prepare = request.mode == "acoustic-source-prepare";
    let apply = request.mode == "acoustic-source-apply";
    if prepare != request.acoustic_edit.is_some()
        || apply != request.acoustic_origin.is_some()
        || ((prepare || apply) && request.declared_seed.is_none())
    {
        return Err(
            "acoustic source operands belong only to their exact closed Scene phase".into(),
        );
    }
    if let Some(origin) = &request.acoustic_origin {
        let id = count(&json!(origin.original_native_request_id))?;
        if id == 0 || id >= count(&json!(request.request_id))? {
            return Err("acoustic application lost its original native preparation ordinal".into());
        }
    }
    Ok(())
}

pub(super) fn execute(
    host: &mut FieldHost,
    request: &ActRequest,
    lease: &NativeActSourceLease<'_>,
    original: &super::super::coupled::CoupledBasis,
    current: &super::super::coupled::CoupledBasis,
) -> Result<Value, String> {
    if request.source_bootstrap.is_some()
        || request.procedural_request.is_some()
        || request.acoustic_configuration.is_some()
        || request.original_acoustic_boundary.is_some()
        || request.physical_edit.is_some()
        || request.physical_origin.is_some()
    {
        return Err("acoustic source operation carries another owner's input".into());
    }
    lease.validate_field_sources(&request.instance_ref, original, current)?;
    let seed = count(&json!(
        request
            .declared_seed
            .as_ref()
            .ok_or("actual acoustic Return seed absent")?
    ))?;
    if request.mode == "acoustic-source-prepare" {
        let candidate = host.prepare_performance_acoustic_edit(
            lease.native_request_id(),
            request
                .acoustic_edit
                .as_ref()
                .ok_or("authored acoustic configuration absent")?
                .clone(),
        )?;
        lease.validate_source_assets(&request.instance_ref, candidate.before_source_assets())?;
        let prepared = host.prepared_acoustic_scene_source(&candidate, seed)?;
        lease.validate_field_sources(&request.instance_ref, original, current)?;
        return Ok(
            json!({"selection":lease.evidence(),"prepared_acoustic":prepared,
            "host_receipt":host.recording_controller_receipt(&request.request_id,None)}),
        );
    }
    let origin = request
        .acoustic_origin
        .as_ref()
        .ok_or("actual acoustic preparation origin absent")?;
    let sources = request.manifest["scene"]["performance"]["native_sources"]
        .as_array()
        .ok_or("closed acoustic AFTER source array absent")?;
    let mut selected = sources.iter().filter(|source| {
        source["identity"]["instance_ref"] == request.instance_ref
            && source["native_bundle"]["acoustic_transition_history"]
                .as_array()
                .and_then(|history| history.last())
                .is_some_and(|record| {
                    record["original_native_request_id"] == origin.original_native_request_id
                })
    });
    let source = selected
        .next()
        .ok_or("actual selected acoustic AFTER source absent")?;
    if selected.next().is_some() {
        return Err("actual acoustic AFTER source epoch is ambiguous".into());
    }
    let record = source["native_bundle"]["acoustic_transition_history"]
        .as_array()
        .and_then(|history| history.last())
        .ok_or("complete original acoustic producer record absent")?;
    let configuration: AcousticConfiguration =
        serde_json::from_value(record["after_configuration"].clone())
            .map_err(|error| error.to_string())?;
    let candidate = host.prepare_performance_acoustic_edit(
        count(&json!(origin.original_native_request_id))?,
        configuration,
    )?;
    if candidate.native_boundary() != &origin.native_boundary
        || candidate.source_record()? != record
        || candidate.source_assets() != &source["native_bundle"]
    {
        return Err(
            "complete original acoustic source/body/native boundary changed before application"
                .into(),
        );
    }
    lease.validate_source_assets(&request.instance_ref, candidate.source_assets())?;
    match host.apply_performance_acoustic_edit(&candidate, lease) {
        Ok(pulse) => {
            let source = host.retained_performance_source_artifact(seed);
            let field = host.retained_procedural_source_artifact();
            match (source, field) {
                (Ok(source_artifact), Ok(native_field_source)) => Ok(json!({
                    "selection":lease.evidence(),"accepted":true,"application_committed":true,
                    "native_pulse":pulse,"source_artifact":source_artifact,"native_field_source":native_field_source,
                    "host_receipt":host.recording_controller_receipt(&request.request_id,Some(&pulse))})),
                (source, field) => Ok(json!({
                    "selection":lease.evidence(),"accepted":false,"application_committed":true,
                    "reason":"actual receiving operation committed but full original source retention failed",
                    "source_retention_error":source.err(),"field_retention_error":field.err(),"native_pulse":pulse,
                    "host_receipt":host.recording_controller_receipt(&request.request_id,Some(&pulse))})),
            }
        }
        Err(refusal) => Ok(json!({"selection":lease.evidence(),"accepted":false,
            "application_committed":refusal.native_pulse().and_then(|pulse|pulse["accepted"].as_bool()),
            "reason":refusal.reason(),"native_pulse":refusal.native_pulse(),
            "host_receipt":host.recording_controller_refusal(&request.request_id,refusal.reason())})),
    }
}
