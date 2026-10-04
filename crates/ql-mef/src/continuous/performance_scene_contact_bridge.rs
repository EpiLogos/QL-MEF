//! Explicit authored Scene contact, carried by the original qualified Act pipe.
//! Raw public Host commands never dispatch this mode or supply its definition.
use super::super::performance::NativeSceneContactOperation;
use super::*;
use sha2::{Digest, Sha256};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct NativeSceneContactQuery {
    schema: String,
    scene_constructor: Value,
    contact_ref: String,
    retention_reservation: Value,
    #[serde(default)]
    original_contact_origin: Option<Value>,
}

impl NativeActSourceLease<'_> {
    /// Read-only original prefix selection. A CPv3 high-water selects the
    /// exact admitted Contact history even when the body, tuning and receiver
    /// are identical across several retained epochs. Full numerical slot and
    /// force qualification remains the native replay preflight.
    pub(super) fn checkpoint_contacts_match(
        &self,
        source: &Value,
        audio: &Value,
    ) -> Result<bool, String> {
        let records = match source["native_bundle"].get("contact_occurrence_history") {
            None => &[][..],
            Some(value) => value
                .as_array()
                .map(Vec::as_slice)
                .ok_or("selected Contact source prefix is not an array")?,
        };
        let Some(checkpoint) = audio.get("contacts") else {
            if audio["schema"] == "ql.performance-checkpoint/v3" {
                return Err("native CPv3 lost its complete Contact state".into());
            }
            return Ok(records.is_empty());
        };
        if audio["schema"] != "ql.performance-checkpoint/v3"
            || checkpoint["schema"] != "ql.performance-contact-state/v1"
            || checkpoint["history_present"] != true
        {
            return Err("selected native Contact checkpoint schema/history differs".into());
        }
        let high_water = count(&checkpoint["original_request_high_water"])?;
        let mut previous = 0;
        for record in records {
            let ordinal = count(&record["original_native_request_id"])?;
            if record["schema"] != "ql.native-scene-contact-source/v1"
                || ordinal <= previous
                || record["occurrence"]["original_request_id"]
                    != record["original_native_request_id"]
                || record["occurrence"]["constructor_lineage"]
                    != record["scene_constructor"]["instance_ref"]
            {
                return Err("selected source lost its original ordered Contact occurrences".into());
            }
            if record["occurrence"]["constructor_lineage"] != checkpoint["constructor_lineage"] {
                return Ok(false);
            }
            previous = ordinal;
        }
        Ok(high_water != 0 && previous == high_water)
    }

    pub(crate) fn selected_full_scene_source_asset(
        &self,
        instance: &str,
        source_assets: &Value,
    ) -> Result<&Value, String> {
        self.validate_source_assets(instance, source_assets)?;
        self.performance_sources()?
            .iter()
            .find(|source| {
                source["identity"]["instance_ref"] == instance
                    && exact_value(&source["native_bundle"], source_assets)
            })
            .ok_or_else(|| "actual full current Contact source asset absent".into())
    }

    pub(crate) fn selected_scene_contact_definition(
        &self,
        instance: &str,
        source_assets: &Value,
        contact_ref: &str,
        fact: &Value,
    ) -> Result<&Value, String> {
        if self.manifest["schema"] != "oi.expression-native-current-scene-delivery/v1"
            || self.field_registration().is_none()
            || contact_ref.is_empty()
            || contact_ref.len() > 255
            || contact_ref.chars().any(char::is_control)
        {
            return Err("actual current Scene Contact source/name absent".into());
        }
        self.validate_source_assets(instance, source_assets)?;
        let keys = [
            "schema",
            "expression_ref",
            "scene_ref",
            "instance_ref",
            "construction_generation",
            "generation_domain",
            "initial_document_revision",
            "initial_document_sha256",
            "document_revision",
            "document_sha256",
        ];
        let object = fact
            .as_object()
            .ok_or("actual Contact Scene constructor absent")?;
        let doc = self.manifest["canonical_document_bytes"]
            .as_str()
            .ok_or("actual Contact full typed Document bytes absent")?;
        if object.len() != keys.len()
            || keys.iter().any(|key| !object.contains_key(*key))
            || fact["schema"] != "oi.native-document-scene-constructor/v1"
            || fact["expression_ref"] != self.manifest["expression_ref"]
            || fact["scene_ref"] != self.manifest["scene_ref"]
            || fact["document_revision"] != self.manifest["expression_revision"]
            || fact["document_sha256"] != format!("{:x}", Sha256::digest(doc.as_bytes()))
            || fact["generation_domain"] != "native-document-scene-construction"
            || fact["construction_generation"]
                .as_u64()
                .is_none_or(|n| n == 0)
            || fact["initial_document_revision"].as_u64().is_none()
            || fact["document_revision"].as_u64().is_none()
            || fact["instance_ref"]
                .as_str()
                .is_none_or(|s| s.is_empty() || s.len() > 255 || s.chars().any(char::is_control))
            || fact["initial_document_sha256"].as_str().is_none_or(|s| {
                s.len() != 64
                    || !s
                        .bytes()
                        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            })
        {
            return Err("Contact lost its actual Scene constructor/full original Document".into());
        }
        // The caller cannot provide a substitute particle, force, handle, date
        // or geometry. It selects one literal authored definition already in
        // this same C-held full Scene. Existing closed-source validation binds
        // its complete typed Document and reference graph before this getter.
        let definitions = self.manifest["scene"]["performance"]["contact_definitions"]
            .as_array()
            .ok_or("actual Scene authored Contact definitions absent")?;
        let mut matches = definitions
            .iter()
            .filter(|d| d["contact_ref"] == contact_ref);
        let definition = matches
            .next()
            .ok_or("actual Scene Contact definition not authored")?;
        if matches.next().is_some() {
            return Err("actual authored Scene Contact name ambiguous".into());
        }
        Ok(definition)
    }

    pub(crate) fn validate_recorded_contact_admissions(
        &self,
        instance: &str,
        source_assets: &Value,
        admissions: &[Value],
    ) -> Result<(), String> {
        let selected = self.selected_full_scene_source_asset(instance, source_assets)?;
        let originals = match selected.get("native_contact_admission_history") {
            None => &[][..],
            Some(value) => value
                .as_array()
                .map(Vec::as_slice)
                .ok_or("actual native Contact admission sidecar has wrong type")?,
        };
        let records = match source_assets.get("contact_occurrence_history") {
            None => &[][..],
            Some(value) => value
                .as_array()
                .map(Vec::as_slice)
                .ok_or("actual native Contact numerical history has wrong type")?,
        };
        if originals.len() != admissions.len()
            || originals
                .iter()
                .zip(admissions)
                .any(|(original, admission)| !exact_value(original, admission))
            || records.len() != admissions.len()
        {
            return Err("actual selected Contact source/admission history differs".into());
        }
        for (record, original) in records.iter().zip(admissions) {
            let keys = [
                "schema",
                "before_source_assets",
                "source",
                "native_admission",
            ];
            let object = original
                .as_object()
                .ok_or("native Contact original admission absent")?;
            if object.len() != keys.len()
                || keys.iter().any(|key| !object.contains_key(*key))
                || original["schema"] != "ql.native-scene-contact-admission/v1"
                || !exact_value(&original["source"], record)
                || !original["before_source_assets"].is_object()
                || original["native_admission"]["schema"] != "ql.performance-worker-reply/v1"
                || !exact_value(
                    &original["native_admission"]["payload"]["contact_source"],
                    record,
                )
                || original["native_admission"]["payload"]["queue_committed"] != true
            {
                return Err(
                    "actual Contact history lost its full BEFORE epoch/original queued pulse"
                        .into(),
                );
            }
        }
        Ok(())
    }
}

pub(super) fn execute(
    host: &mut FieldHost,
    request: &ActRequest,
    lease: &NativeActSourceLease<'_>,
    original: &super::super::coupled::CoupledBasis,
    current: &super::super::coupled::CoupledBasis,
) -> Result<Value, String> {
    if request.source_bootstrap.is_some()
        || request.scene_consumer.is_some()
        || request.acoustic_configuration.is_some()
        || request.original_acoustic_boundary.is_some()
        || request.physical_edit.is_some()
        || request.physical_origin.is_some()
        || request.acoustic_edit.is_some()
        || request.acoustic_origin.is_some()
    {
        return Err("Scene Contact carries another owner's operands".into());
    }
    lease.validate_field_sources(&request.instance_ref, original, current)?;
    let query: NativeSceneContactQuery = serde_json::from_value(
        request
            .procedural_request
            .clone()
            .ok_or("actual Scene Contact command absent")?,
    )
    .map_err(|e| e.to_string())?;
    if query.schema != "ql.native-scene-contact-command/v1"
        || (request.mode == "contact-apply") != query.original_contact_origin.is_some()
    {
        return Err("actual Scene Contact command/original phase differs".into());
    }
    let seed = count(&json!(
        request
            .declared_seed
            .as_ref()
            .ok_or("native Contact Return seed absent")?
    ))?;
    match host.exchange_performance_scene_contact(NativeSceneContactOperation {
        lease,
        operation: &request.mode,
        contact_ref: &query.contact_ref,
        scene_constructor: &query.scene_constructor,
        retained_reservation: &query.retention_reservation,
        origin: query.original_contact_origin.as_ref(),
    }) {
        Ok(pulse) => {
            let queued = pulse["payload"]["queue_committed"] == true;
            match host.retained_performance_source_artifact(seed) {
                Ok(source) => Ok(json!({"selection":lease.evidence(),"accepted":true,
                    "queue_committed":queued,"application_committed":false,
                    "native_pulse":pulse,"source_artifact":source,
                    "host_receipt":host.recording_controller_receipt(&request.request_id,Some(&pulse))})),
                Err(reason) => {
                    let reason = if queued {
                        host.hold_failed_scene_contact_retention(&reason)
                    } else {
                        reason
                    };
                    Ok(json!({"selection":lease.evidence(),"accepted":false,
                    "queue_committed":queued,"application_committed":false,"reason":reason,
                    "native_pulse":pulse,
                    "host_receipt":host.recording_controller_receipt(&request.request_id,Some(&pulse))}))
                }
            }
        }
        Err(refusal) => Ok(json!({"selection":lease.evidence(),"accepted":false,
            "queue_committed":refusal.queue_committed,"application_committed":false,
            "reason":refusal.reason,"native_pulse":refusal.native_pulse,
            "host_receipt":host.recording_controller_receipt(&request.request_id,refusal.native_pulse.as_ref())})),
    }
}
