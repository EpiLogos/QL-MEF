//! Child of the retained Form cold compiler. All original body and acoustic
//! epochs remain in its existing frame carrier; this is not another P owner.
use super::*;

const ACOUSTIC_TRANSITION: &str = "ql.native-acoustic-source-transition/v1";

impl ReplayedPhysicalSourceFrame {
    /// Reconstruct only this frame's authored acoustic input on the SAME
    /// privately retained source family. Original World/profile/occasion and
    /// ReturnContext remain unchanged; full source-input equality is required.
    pub(crate) fn receiving_source(
        &self,
        original: &NativePerformanceReceivingSource,
    ) -> Result<NativePerformanceReceivingSource, String> {
        let source = if let Some(value) = self.owner.source_assets().get("acoustic_receiving") {
            original.clone().with_acoustic_configuration(
                serde_json::from_value(value["packet"]["configuration"].clone())
                    .map_err(|e| e.to_string())?,
            )?
        } else {
            original.clone().without_acoustic_configuration()
        };
        if source.source_inputs()? != self.owner.source_assets()["receiving_source_inputs"] {
            return Err(
                "historical acoustic epoch changed complete original native source family".into(),
            );
        }
        Ok(source)
    }
}

fn rows<'a>(assets: &'a Value, key: &str) -> Result<&'a [Value], String> {
    match assets.get(key) {
        None => Ok(&[]),
        Some(value) => value
            .as_array()
            .map(Vec::as_slice)
            .ok_or_else(|| format!("full original {key} source array absent")),
    }
}
fn original_order(physical: &[Value], acoustic: &[Value]) -> Result<Vec<Value>, String> {
    let mut all = physical.iter().chain(acoustic).cloned().collect::<Vec<_>>();
    if all
        .iter()
        .any(|row| row.as_object().is_none_or(|v| v.len() != 2))
    {
        return Err("complete original source/application wrapper differs".into());
    }
    for row in &all {
        if row["source"]["schema"] != PHYSICAL_SOURCE_TRANSITION
            && row["source"]["schema"] != ACOUSTIC_TRANSITION
        {
            return Err("unrecognized original native source operation".into());
        }
        decimal(&row["source"]["original_native_request_id"])?;
        decimal(&row["source"]["native_sample"])?;
    }
    all.sort_by_key(|row| {
        decimal(&row["source"]["original_native_request_id"])
            .expect("qualified canonical original ordinal")
    });
    let mut request = 0;
    let mut sample = 0;
    for row in &all {
        let next = decimal(&row["source"]["original_native_request_id"])?;
        let at = decimal(&row["source"]["native_sample"])?;
        if next <= request || at < sample {
            return Err("full native acoustic/body source order regressed or duplicated".into());
        }
        request = next;
        sample = at;
    }
    Ok(all)
}

impl PerformanceOwner {
    pub(crate) fn native_emission_source_history(&self) -> Result<Vec<Value>, String> {
        original_order(
            &self.native_physical_source_history(),
            &self.native_acoustic_source_history(),
        )
    }

    /// The same C source owner independently holds the full original source
    /// epoch and both actual application sidecars. Arbitrary retained JSON,
    /// final current input or a musical basis digest cannot choose an origin.
    pub(in crate::continuous) fn prepare_cold_native_acoustic_source(
        original: &CoupledBasis,
        instance: &str,
        source: &NativePerformanceReceivingSource,
        expected: &Value,
        physical_applications: &[Value],
        acoustic_applications: &[Value],
        lease: &NativeActSourceLease<'_>,
    ) -> Result<PreparedColdPhysicalSource, String> {
        lease.validate_source_assets(instance, expected)?;
        lease.validate_recorded_source_applications(
            instance,
            expected,
            physical_applications,
            acoustic_applications,
        )?;
        let prepared = Self::replay_cold_native_acoustic_source(
            original,
            instance,
            source,
            expected,
            physical_applications,
            acoustic_applications,
        )?;
        lease.validate_source_assets(instance, prepared.final_frame().owner().source_assets())?;
        lease.validate_recorded_source_applications(
            instance,
            expected,
            physical_applications,
            acoustic_applications,
        )?;
        Ok(prepared)
    }

    /// Full deterministic original-owner source compiler, never activation.
    pub(in crate::continuous) fn replay_cold_native_acoustic_source(
        original: &CoupledBasis,
        instance: &str,
        source: &NativePerformanceReceivingSource,
        expected: &Value,
        physical_applications: &[Value],
        acoustic_applications: &[Value],
    ) -> Result<PreparedColdPhysicalSource, String> {
        if expected["schema"] != "ql.retained-performance-source-assets/v1"
            || expected["original_native_input"]
                != serde_json::to_value(&original.input).map_err(|e| e.to_string())?
            || serde_json::to_vec(expected)
                .map_err(|e| e.to_string())?
                .len()
                > crate::continuous::MAX_MESSAGE
            || serde_json::to_vec(&(physical_applications, acoustic_applications))
                .map_err(|e| e.to_string())?
                .len()
                > crate::continuous::MAX_MESSAGE
        {
            return Err(
                "cold acoustic source lost actual original constructor or full transport bounds"
                    .into(),
            );
        }
        validate_form_lineage(original, original)?;
        let physical = rows(expected, "physical_transition_history")?;
        let acoustic = rows(expected, "acoustic_transition_history")?;
        if physical.len() != physical_applications.len()
            || acoustic.len() != acoustic_applications.len()
            || physical
                .iter()
                .zip(physical_applications)
                .any(|(a, b)| b["source"] != *a)
            || acoustic
                .iter()
                .zip(acoustic_applications)
                .any(|(a, b)| b["source"] != *a)
        {
            return Err(
                "cold acoustic source lost complete original physical/acoustic applications".into(),
            );
        }
        if acoustic.is_empty() {
            return Self::replay_cold_native_physical_source(
                original,
                instance,
                source,
                expected,
                physical_applications,
            );
        }
        let all = original_order(physical_applications, acoustic_applications)?;
        let first = &all[0]["source"];
        let configuration: PerformanceConfig =
            serde_json::from_value(if first["schema"] == ACOUSTIC_TRANSITION {
                first["performance_configuration"].clone()
            } else {
                first["before_configuration"].clone()
            })
            .map_err(|e| e.to_string())?;
        let mut current = original.clone();
        let mut owner = Self::prepare(original, instance, configuration)?;
        let mut operative_source = if first["before_acoustic"].is_object() {
            source.clone().with_acoustic_configuration(
                serde_json::from_value(first["before_acoustic"]["configuration"].clone())
                    .map_err(|e| e.to_string())?,
            )?
        } else if first["before_acoustic"] == Value::Null {
            source.clone().without_acoustic_configuration()
        } else {
            return Err(
                "original acoustic before epoch is not an original packet or absence".into(),
            );
        };
        let source_sample = decimal(
            &first["before_current_receiving"]["native_admission"]["operation"]["native_sample"],
        )?;
        let admitted = operative_source.admit_current(&mut owner, original, source_sample)?;
        admitted.validate_current(&operative_source, &owner, original, source_sample)?;
        if admitted.snapshot()? != first["before_current_receiving"] {
            return Err("initial mixed source N9/World/occasion does not replay".into());
        }
        owner.source_assets["receiving_source_inputs"] = admitted.source_inputs().clone();
        owner.source_assets["receiving_definition"] = admitted.definition().snapshot()?;
        owner.source_assets["current_receiving"] = admitted.snapshot()?;
        owner.source_assets["consumer_roles"] = expected
            .get("consumer_roles")
            .filter(|v| v.is_object())
            .ok_or("original mixed native consumer roles absent")?
            .clone();
        if first["before_acoustic"].is_object() {
            let birth = decimal(&first["before_acoustic"]["history_origin_sample"])?;
            let origin = decimal(&first["before_acoustic"]["origin_sample"])?;
            let prepared = owner.prepare_acoustic_receiving_segment(
                original,
                &operative_source,
                birth,
                origin,
            )?;
            if prepared.packet() != &first["before_acoustic"] {
                return Err(
                    "original acoustic installation/trajectory cannot be regenerated".into(),
                );
            }
            owner.source_assets["acoustic_receiving"] = prepared.snapshot();
        }
        let mut frames = Vec::with_capacity(all.len() + 1);
        let mut physical_prefix = Vec::<Value>::new();
        let mut acoustic_prefix = Vec::<Value>::new();
        let mut previous_eigenbasis: Option<Value> = None;
        let mut previous_epoch = 0;
        let mut previous_sequence = 0;
        for retained in &all {
            let record = &retained["source"];
            let pulse = &retained["native_application"];
            let request = decimal(&record["original_native_request_id"])?;
            let sample = decimal(&record["native_sample"])?;
            let frame_sample = decimal(
                &owner.source_assets["current_receiving"]["native_admission"]["operation"]["native_sample"],
            )?;
            let (mut after_owner, after_current, after_source) = if record["schema"]
                == PHYSICAL_SOURCE_TRANSITION
            {
                let authored: AuthoredNativePhysicalEdit =
                    serde_json::from_value(record["authored_edit"].clone())
                        .map_err(|e| e.to_string())?;
                let descendant = owner.prepare_physical_source_descendant_at(
                    &current,
                    &operative_source,
                    request,
                    &authored,
                    sample,
                )?;
                if descendant.source_record != *record {
                    return Err(
                        "mixed source lost full original physical M3/recipe/N9 record".into(),
                    );
                }
                descendant.after_owner.validate_reply(pulse)?;
                let ack = &pulse["payload"]["physical_transition"];
                if pulse["operation"] != "source-body-transition"
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
                    || previous_eigenbasis
                        .as_ref()
                        .is_some_and(|v| ack["before_eigenbasis_identity"] != *v)
                    || ack["after_eigenbasis_identity"]
                        != pulse["reading"]["physical"]["eigenbasis_identity"]
                    || ack["transport_epoch"] != pulse["reading"]["transport_epoch"]
                    || ack["accepted_sequence"] != pulse["reading"]["accepted_sequence"]
                {
                    return Err("mixed cold physical source lost original actual stopped ACK/body/eigenbasis".into());
                }
                for field in [
                    "before_energy_joules",
                    "after_energy_joules",
                    "external_work_joules",
                ] {
                    if ack[field].as_f64().is_none_or(|v| !v.is_finite()) {
                        return Err("mixed cold physical energy/work receipt absent".into());
                    }
                }
                previous_eigenbasis = Some(ack["after_eigenbasis_identity"].clone());
                physical_prefix.push(retained.clone());
                (
                    descendant.after_owner,
                    descendant.after_current,
                    operative_source.clone(),
                )
            } else {
                let config: AcousticConfiguration =
                    serde_json::from_value(record["after_configuration"].clone())
                        .map_err(|e| e.to_string())?;
                let after_source = operative_source
                    .clone()
                    .with_acoustic_configuration(config)?;
                let (assets, actual_record) = match record["kind"].as_str() {
                    Some("install") => {
                        let prepared = owner.prepare_acoustic_installation_descendant_at(
                            &current,
                            &after_source,
                            request,
                            sample,
                        )?;
                        (prepared.after_assets, prepared.record)
                    }
                    Some("replace") => {
                        let prepared = owner.prepare_acoustic_source_descendant_at(
                            &current,
                            &after_source,
                            request,
                            sample,
                        )?;
                        (prepared.after_assets, prepared.record)
                    }
                    _ => return Err("unknown original acoustic source operation kind".into()),
                };
                if actual_record != *record {
                    return Err("mixed source lost full original acoustic configuration/context/N9/trajectory record".into());
                }
                owner.validate_reply(pulse)?;
                let manifest = &pulse["reading"]["receiving_transport"]["manifest"];
                if record["kind"] == "replace" {
                    let ack = &pulse["payload"]["receiving_replacement"];
                    if pulse["operation"] != "receiving-transport-replace"
                        || ack["schema"] != "ql.native-receiving-replacement/v1"
                        || ack["sample"] != record["native_sample"]
                        || ack["after_manifest"] != *manifest
                        || ack["before_manifest"]["eigenbasis"] != manifest["eigenbasis"]
                        || previous_eigenbasis
                            .as_ref()
                            .is_some_and(|v| ack["before_manifest"]["eigenbasis"] != *v)
                        || ack["transport_epoch"] != pulse["reading"]["transport_epoch"]
                        || ack["accepted_sequence"] != pulse["reading"]["accepted_sequence"]
                    {
                        return Err(
                            "mixed source lost original native receiver replacement ACK".into()
                        );
                    }
                    validate_manifest(&ack["before_manifest"], &record["before_acoustic"], &owner)?;
                } else if pulse["operation"] != "receiving-transport-install"
                    || record["before_acoustic"] != Value::Null
                {
                    return Err(
                        "mixed initial acoustic source claimed another native operation".into(),
                    );
                }
                validate_manifest(manifest, &record["after_acoustic"], &owner)?;
                if manifest["eigenbasis"] != pulse["reading"]["physical"]["eigenbasis_identity"]
                    || previous_eigenbasis
                        .as_ref()
                        .is_some_and(|v| manifest["eigenbasis"] != *v)
                {
                    return Err("same-body acoustic epoch changed actual P eigenbasis".into());
                }
                previous_eigenbasis = Some(manifest["eigenbasis"].clone());
                acoustic_prefix.push(retained.clone());
                let mut next = Self::prepare(&current, instance, owner.config.clone())?;
                next.source_origin = owner.source_origin.clone();
                next.source_assets = assets;
                (next, current.clone(), after_source)
            };
            let epoch = decimal(&pulse["reading"]["transport_epoch"])?;
            let sequence = decimal(&pulse["reading"]["accepted_sequence"])?;
            if pulse["accepted"] != true
                || pulse["reading"]["samples_elapsed"] != record["native_sample"]
                || pulse["reading"]["physical"]["samples_elapsed"] != record["native_sample"]
                || !matches!(
                    pulse["reading"]["device"]["state"].as_str(),
                    Some("closed" | "prepared")
                )
                || epoch == 0
                || epoch < previous_epoch
                || (epoch == previous_epoch && sequence < previous_sequence)
            {
                return Err(
                    "mixed native source lost actual original owner/date/transport order".into(),
                );
            }
            if record["after_acoustic"].is_object() || record["schema"] == ACOUSTIC_TRANSITION {
                let packet = &record["after_acoustic"];
                validate_manifest(
                    &pulse["reading"]["receiving_transport"]["manifest"],
                    packet,
                    &after_owner,
                )?;
                if pulse["reading"]["receiving_transport"]["samples_elapsed"]
                    != record["native_sample"]
                {
                    return Err("mixed source lost actual M4 shared cursor".into());
                }
            }
            frames.push(ReplayedPhysicalSourceFrame {
                owner,
                current,
                source_sample: frame_sample,
            });
            after_owner.physical_source_history = physical_prefix
                .iter()
                .map(|row| NativePhysicalTransitionRecord {
                    source: row["source"].clone(),
                    application: row["native_application"].clone(),
                })
                .collect();
            after_owner.acoustic_source_history = acoustic_prefix.clone();
            owner = after_owner;
            current = after_current;
            operative_source = after_source;
            previous_epoch = epoch;
            previous_sequence = sequence;
        }
        if owner.source_assets() != expected
            || operative_source.source_inputs()? != source.source_inputs()?
        {
            return Err(
                "complete final mixed native source differs from original-owner replay".into(),
            );
        }
        owner.validate_current(&current)?;
        if owner.reading().is_some() || frames.iter().any(|v| v.owner.reading().is_some()) {
            return Err("pure mixed source replay claimed native P/readback residency".into());
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

fn validate_manifest(
    manifest: &Value,
    packet: &Value,
    owner: &PerformanceOwner,
) -> Result<(), String> {
    let body = owner.packet()?["physical_body"].clone();
    for (field, value) in [
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
        ("event", &body["event_ref"]),
        ("subject", &body["subject_ref"]),
        ("preparation", &body["request"]["preparation_ref"]),
        ("state", &body["request"]["state_ref"]),
        (
            "source_coordinate",
            &body["source_coordinate"]["source_ref"],
        ),
        ("source_revision", &body["source_revision"]),
        ("sample_rate", &body["request"]["sample_rate"]),
    ] {
        if manifest[field] != *value {
            return Err(format!(
                "original mixed native receiver manifest {field} differs"
            ));
        }
    }
    if decimal(&manifest["body_revision"])? != owner.config.controls.body_revision
        || decimal(&manifest["source_generation"])?
            != body["source_generation"]
                .as_u64()
                .ok_or("actual mixed source generation absent")?
        || manifest["pratibimba"] != (body["source_coordinate"]["face"] == "pratibimba")
        || packet["source_body"] != body
    {
        return Err("original mixed acoustic manifest body/source/face differs".into());
    }
    Ok(())
}

#[cfg(test)]
#[path = "performance_acoustic_source_cold_tests.rs"]
mod tests;
