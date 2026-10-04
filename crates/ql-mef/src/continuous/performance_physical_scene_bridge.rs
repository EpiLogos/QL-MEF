//! Two closed Scene operations over the same retained FieldHost and P owner.
//! C's opaque native candidate keeps the actual prepare reply between its full
//! BEFORE -> prospective AFTER -> actually applied Scene CAS operations.
use super::*;
use crate::continuous::performance::AuthoredNativePhysicalEdit;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct NativePhysicalApplyOrigin {
    pub(super) original_native_request_id: String,
    pub(super) native_boundary: Value,
}

pub(super) fn validate_operands(request: &ActRequest) -> Result<(), String> {
    let prepare = request.mode == "physical-prepare";
    let apply = request.mode == "physical-apply";
    if prepare != request.physical_edit.is_some()
        || apply != request.physical_origin.is_some()
        || ((prepare || apply) && request.declared_seed.is_none())
    {
        return Err(
            "physical source operands belong only to their exact closed Scene phase".into(),
        );
    }
    if let Some(origin) = &request.physical_origin {
        let prepared_id = count(&json!(origin.original_native_request_id))?;
        if prepared_id == 0 || prepared_id >= count(&json!(request.request_id))? {
            return Err("physical apply lost the earlier actual native prepare ordinal".into());
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
    {
        return Err("physical source operation carries another owner's input".into());
    }
    lease.validate_field_sources(&request.instance_ref, original, current)?;
    let seed = count(&json!(
        request
            .declared_seed
            .as_ref()
            .ok_or("physical Return seed absent")?
    ))?;
    if request.mode == "physical-prepare" {
        let authored = request
            .physical_edit
            .as_ref()
            .ok_or("authored physical edit absent")?
            .clone();
        let candidate =
            host.prepare_performance_physical_edit(lease.native_request_id(), authored)?;
        lease.validate_source_assets(&request.instance_ref, candidate.before_source_assets())?;
        let prepared = host.prepared_physical_scene_source(&candidate, seed)?;
        lease.validate_field_sources(&request.instance_ref, original, current)?;
        return Ok(
            json!({"selection":lease.evidence(),"prepared_physical":prepared,
            "host_receipt":host.recording_controller_receipt(&request.request_id,None)}),
        );
    }
    let origin = request
        .physical_origin
        .as_ref()
        .ok_or("actual native prepared origin absent")?;
    let source_assets = &request.manifest["scene"]["performance"]["native_sources"];
    let sources = source_assets
        .as_array()
        .ok_or("closed physical AFTER source array absent")?;
    let mut selected = sources.iter().filter(|source| {
        source["identity"]["instance_ref"] == request.instance_ref
            && source["native_bundle"]["physical_transition_history"]
                .as_array()
                .and_then(|history| history.last())
                .is_some_and(|record| {
                    record["original_native_request_id"] == origin.original_native_request_id
                })
    });
    let source = selected
        .next()
        .ok_or("actual selected physical AFTER source absent")?;
    if selected.next().is_some() {
        return Err("actual selected physical AFTER source is ambiguous".into());
    }
    let record = source["native_bundle"]["physical_transition_history"]
        .as_array()
        .and_then(|history| history.last())
        .ok_or("complete physical source record absent")?;
    let authored: AuthoredNativePhysicalEdit =
        serde_json::from_value(record["authored_edit"].clone())
            .map_err(|error| error.to_string())?;
    let candidate = host.prepare_performance_physical_edit(
        count(&json!(origin.original_native_request_id))?,
        authored,
    )?;
    if candidate.native_boundary() != &origin.native_boundary
        || candidate.source_record() != record
        || candidate.source_assets() != &source["native_bundle"]
    {
        return Err(
            "original physical candidate/source/body boundary changed before application".into(),
        );
    }
    lease.validate_source_assets(&request.instance_ref, candidate.source_assets())?;
    match host.apply_performance_physical_edit(candidate, lease) {
        Ok(pulse) => {
            // These are getters on the SAME actual AFTER owner. No second
            // Inspect/Advance, manufactured ACK or imported source grant.
            let source = host.retained_performance_source_artifact(seed);
            let field = host.retained_procedural_source_artifact();
            match (source, field) {
                (Ok(source_artifact), Ok(native_field_source)) => Ok(json!({
                    "selection":lease.evidence(),"accepted":true,"application_committed":true,
                    "native_pulse":pulse,"source_artifact":source_artifact,
                    "native_field_source":native_field_source,
                    "host_receipt":host.recording_controller_receipt(&request.request_id,Some(&pulse))
                })),
                (source, field) => Ok(json!({
                    "selection":lease.evidence(),"accepted":false,"application_committed":true,
                    "reason":"actual body committed but complete source retention failed",
                    "source_retention_error":source.err(),"field_retention_error":field.err(),
                    "native_pulse":pulse,
                    "host_receipt":host.recording_controller_receipt(&request.request_id,Some(&pulse))
                })),
            }
        }
        Err(refusal) => Ok(json!({
            "selection":lease.evidence(),"accepted":false,
            "reason":refusal.reason,"native_receipts":refusal.native_receipts,
            "host_receipt":host.recording_controller_refusal(&request.request_id,&refusal.reason)
        })),
    }
}
