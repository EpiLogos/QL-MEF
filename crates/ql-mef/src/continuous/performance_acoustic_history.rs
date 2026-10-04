//! Reconcile the native receiver's dated emitter history with original-owner
//! Form/material replay. JSON describes retained evidence; only the SAME
//! closed C source/checkpoint lease can issue the private qualified operand.
use super::*;
use crate::continuous::performance_act_bridge::NativeActSourceLease;
use crate::continuous::performance_receiving::NativePerformanceReceivingSource;
use std::collections::BTreeSet;

const HISTORY_SAMPLES: u64 = 16_384;
const MAX_SEGMENTS: usize = 256;
const MAX_REFERENCE_BYTES: usize = 256 * 1024;
const REFERENCE_FIELDS: [&str; 6] = [
    "source_coordinate",
    "source_revision",
    "preparation",
    "state",
    "eigenbasis",
    "source_motion",
];

/// No Clone, Deserialize, public fields, caller-selected origin or JSON issuer.
/// Full wire/source/episode and every original native application stay retained
/// until the same saved-receiving consumer has revalidated the closed reader.
pub(crate) struct QualifiedAcousticSourceHistory {
    instance: String,
    checkpoint_ref: String,
    original_wire: String,
    source_assets: Value,
    operative_current: Value,
    source_inputs: Value,
    original_applications: Vec<Value>,
    original_acoustic_applications: Vec<Value>,
    frame_corpus: Value,
    source_history: Value,
    closed_selection: Value,
}

pub(crate) fn qualify_saved_acoustic_physical_history(
    replayed: &PreparedColdPhysicalSource,
    source: &NativePerformanceReceivingSource,
    lease: &NativeActSourceLease<'_>,
    instance: &str,
    checkpoint_ref: &str,
    original_checkpoint_wire: &str,
) -> Result<QualifiedAcousticSourceHistory, String> {
    bounded(instance)?;
    bounded(checkpoint_ref)?;
    let final_frame = replayed.final_frame();
    let applications = final_frame.owner().native_physical_source_history();
    let acoustic_applications = final_frame.owner().native_acoustic_source_history();
    lease.validate_source_assets(instance, final_frame.owner().source_assets())?;
    lease.validate_recorded_source_applications(
        instance,
        final_frame.owner().source_assets(),
        &applications,
        &acoustic_applications,
    )?;
    lease.validate_selected_checkpoint(instance, checkpoint_ref, original_checkpoint_wire)?;
    let selection = lease.evidence();
    let history =
        verify_dated_source_history(replayed, source, instance, original_checkpoint_wire)?;
    let mut frames = Vec::with_capacity(replayed.frames().len());
    for frame in replayed.frames() {
        frames.push(json!({"current":frame.current(),"source":frame.owner().source_assets(),
            "native_preparation":frame.owner().packet()?,"source_sample":frame.source_sample().to_string()}));
    }
    lease.validate_source_assets(instance, final_frame.owner().source_assets())?;
    lease.validate_recorded_source_applications(
        instance,
        final_frame.owner().source_assets(),
        &applications,
        &acoustic_applications,
    )?;
    lease.validate_selected_checkpoint(instance, checkpoint_ref, original_checkpoint_wire)?;
    if selection != lease.evidence() {
        return Err(
            "closed acoustic Act/source/checkpoint selection changed during qualification".into(),
        );
    }
    Ok(QualifiedAcousticSourceHistory {
        instance: instance.into(),
        checkpoint_ref: checkpoint_ref.into(),
        original_wire: original_checkpoint_wire.into(),
        source_assets: final_frame.owner().source_assets().clone(),
        operative_current: serde_json::to_value(final_frame.current())
            .map_err(|e| e.to_string())?,
        source_inputs: source.source_inputs()?,
        original_applications: applications,
        original_acoustic_applications: acoustic_applications,
        frame_corpus: json!(frames),
        source_history: history,
        closed_selection: selection,
    })
}

impl QualifiedAcousticSourceHistory {
    pub(super) fn validate_saved(
        &self,
        owner: &PerformanceOwner,
        current: &CoupledBasis,
        source: &NativePerformanceReceivingSource,
        lease: &NativeActSourceLease<'_>,
        instance: &str,
        checkpoint_ref: &str,
        original_wire: &str,
    ) -> Result<(), String> {
        owner.validate_current(current)?;
        lease.validate_source_assets(instance, owner.source_assets())?;
        lease.validate_recorded_source_applications(
            instance,
            owner.source_assets(),
            &self.original_applications,
            &self.original_acoustic_applications,
        )?;
        lease.validate_selected_checkpoint(instance, checkpoint_ref, original_wire)?;
        if self.instance != instance
            || self.checkpoint_ref != checkpoint_ref
            || self.original_wire != original_wire
            || self.source_assets != *owner.source_assets()
            || self.operative_current != serde_json::to_value(current).map_err(|e| e.to_string())?
            || self.source_inputs != source.source_inputs()?
            || self.original_applications != owner.native_physical_source_history()
            || self.original_acoustic_applications != owner.native_acoustic_source_history()
            || self.closed_selection != lease.evidence()
            || self.frame_corpus.as_array().is_none_or(|v| {
                v.len()
                    != self.original_applications.len()
                        + self.original_acoustic_applications.len()
                        + 1
            })
        {
            return Err("qualified dated history lost its complete original owner/source/episode/checkpoint".into());
        }
        let saved: Value = serde_json::from_str(original_wire).map_err(|e| e.to_string())?;
        require_exact(
            &saved["native_pair"]["audio"]["receiving"]["source_history"],
            &self.source_history,
        )?;
        lease.validate_source_assets(instance, owner.source_assets())?;
        lease.validate_recorded_source_applications(
            instance,
            owner.source_assets(),
            &self.original_applications,
            &self.original_acoustic_applications,
        )?;
        lease.validate_selected_checkpoint(instance, checkpoint_ref, original_wire)?;
        if self.closed_selection != lease.evidence() {
            return Err("closed dated receiver selection changed after revalidation".into());
        }
        Ok(())
    }
}

fn keys(value: &Value, expected: &[&str]) -> Result<(), String> {
    let object = value.as_object().ok_or("dated source object absent")?;
    if object.len() != expected.len() || expected.iter().any(|key| !object.contains_key(*key)) {
        return Err("dated source field set differs from actual native encoding".into());
    }
    Ok(())
}
fn reference(value: &Value) -> Result<&str, String> {
    let text = value.as_str().ok_or("dated source reference absent")?;
    if text.is_empty() || text.len() > 2048 || text.bytes().any(|v| v < 32 || v == 127) {
        return Err("dated native source reference exceeds its actual arena contract".into());
    }
    Ok(text)
}
fn vector(value: &Value, position: bool) -> Result<[f64; 3], String> {
    let array = value
        .as_array()
        .filter(|v| v.len() == 3)
        .ok_or("dated source vector dimensions differ")?;
    let mut result = [0.; 3];
    for (out, value) in result.iter_mut().zip(array) {
        *out = value
            .as_f64()
            .filter(|v| v.is_finite() && (!position || v.abs() <= 1e6))
            .ok_or("dated source vector is nonfinite or outside metres bound")?;
    }
    Ok(result)
}
/// Full recursive comparison preserves binary64 sign, including -0; integers
/// keep their actual decimal-string/native JSON representation and key sets.
fn require_exact(actual: &Value, expected: &Value) -> Result<(), String> {
    let same = match (actual, expected) {
        (Value::Array(a), Value::Array(b)) => {
            a.len() == b.len() && a.iter().zip(b).all(|(x, y)| require_exact(x, y).is_ok())
        }
        (Value::Object(a), Value::Object(b)) => {
            a.len() == b.len()
                && b.iter()
                    .all(|(k, y)| a.get(k).is_some_and(|x| require_exact(x, y).is_ok()))
        }
        (Value::Number(a), Value::Number(b)) if a.is_f64() || b.is_f64() => a
            .as_f64()
            .zip(b.as_f64())
            .is_some_and(|(x, y)| x.to_bits() == y.to_bits()),
        _ => actual == expected,
    };
    if same {
        Ok(())
    } else {
        Err("dated emitter history differs from full original native source replay".into())
    }
}
fn reference_budget(segments: &[Value]) -> Result<(), String> {
    let mut references = BTreeSet::new();
    for segment in segments {
        for field in REFERENCE_FIELDS {
            references.insert(reference(&segment[field])?);
        }
    }
    let bytes = references
        .into_iter()
        .try_fold(0usize, |sum, s| sum.checked_add(s.len() + 1))
        .ok_or("dated reference arena size overflow")?;
    if bytes > MAX_REFERENCE_BYTES {
        return Err("dated source reference arena exceeds native 256KiB capacity".into());
    }
    Ok(())
}
fn earliest(cursor: u64, birth: u64) -> u64 {
    birth.max(cursor.checked_sub(HISTORY_SAMPLES - 1).unwrap_or(birth))
}
fn retained_first(segments: &[Value], cursor: u64, birth: u64) -> Result<usize, String> {
    let floor = earliest(cursor, birth);
    let mut first = 0;
    while first + 1 < segments.len() && decimal(&segments[first + 1]["effective_sample"])? <= floor
    {
        first += 1;
    }
    Ok(first)
}
/// Mirror actual CONTROL append_receiving_source: compact only at a source
/// transition, keep one predecessor for retarded samples, replace same date,
/// then recompute the callback-owned first cursor without dropping old rows.
fn append(segments: &mut Vec<Value>, next: Value, birth: u64) -> Result<(), String> {
    let date = decimal(&next["effective_sample"])?;
    if date < birth || decimal(&next["trajectory_origin_sample"])? > date {
        return Err("dated source precedes native receiving birth/origin".into());
    }
    if !segments.is_empty() {
        let first = retained_first(segments, date, birth)?;
        segments.drain(..first);
        if let Some(at) = segments
            .iter()
            .position(|v| decimal(&v["effective_sample"]).ok() == Some(date))
        {
            segments.truncate(at);
        }
        if segments
            .last()
            .is_some_and(|v| decimal(&v["effective_sample"]).is_ok_and(|old| old > date))
        {
            return Err("dated source transition regressed".into());
        }
    }
    if segments.len() == MAX_SEGMENTS {
        return Err("dated source exceeds native 256 segment capacity".into());
    }
    segments.push(next);
    reference_budget(segments)
}

/// Ordinary numerical/source compiler, never a permission constructor. Its
/// inputs are actual pure replay frames; production wraps it in the closed
/// lease above and retains the entire original wire for native P restoration.
pub(super) fn verify_dated_source_history(
    replayed: &PreparedColdPhysicalSource,
    source: &NativePerformanceReceivingSource,
    instance: &str,
    original_wire: &str,
) -> Result<Value, String> {
    if original_wire.is_empty()
        || original_wire.len() as u64 > crate::continuous::host::MAX_HOST_INPUT
    {
        return Err("dated checkpoint exceeds original native transport bound".into());
    }
    let saved: Value = serde_json::from_str(original_wire).map_err(|e| e.to_string())?;
    if saved["schema"] != "ql.performance-management-checkpoint/v1"
        || saved["native_pair"]["audio"]["has_receiving"] != true
    {
        return Err("dated checkpoint lacks the actual native receiving owner".into());
    }
    let audio = &saved["native_pair"]["audio"];
    let physical = &saved["native_pair"]["physical"];
    if physical["schema"] != "ql.physical-body-checkpoint/v1" || physical["version"] != 1 {
        return Err("dated native physical checkpoint schema/version differs".into());
    }
    let cursor = decimal(&audio["cursor"])?;
    let receiving = &audio["receiving"];
    keys(
        receiving,
        &[
            "schema",
            "version",
            "manifest",
            "samples_elapsed",
            "history_start_sample",
            "history_units",
            "history_linear",
            "source_history",
        ],
    )?;
    if receiving["schema"] != "ql.performance-receiving-checkpoint/v2"
        || receiving["version"] != 2
        || receiving["history_units"] != "linear-pickup"
        || decimal(&receiving["samples_elapsed"])? != cursor
        || decimal(&physical["state"]["samples_elapsed"])? != cursor
    {
        return Err("dated receiver/P/audio checkpoint version or shared cursor differs".into());
    }
    let ring = receiving["history_linear"]
        .as_array()
        .filter(|v| v.len() == HISTORY_SAMPLES as usize)
        .ok_or("dated receiver lost the actual 16384-sample ring")?;
    for value in ring {
        let number = value
            .as_f64()
            .filter(|v| v.is_finite())
            .ok_or("dated receiver ring is nonfinite")?;
        let narrowed = number as f32;
        if !narrowed.is_finite() || f64::from(narrowed).to_bits() != number.to_bits() {
            return Err("dated receiver ring lost native f32 precision or sign".into());
        }
    }
    let frames = replayed.frames();
    let last = replayed.final_frame();
    let applications = last.owner().native_emission_source_history()?;
    if applications.is_empty() || frames.len() != applications.len() + 1 {
        return Err(
            "dated source requires all original physical/acoustic frames and applications".into(),
        );
    }
    if last.owner().binding().determination()["identity"]["instance"] != instance {
        return Err("dated source belongs to another native performance instance".into());
    }
    let birth = decimal(&receiving["history_start_sample"])?;
    if birth > cursor {
        return Err("dated history birth exceeds native cursor".into());
    }
    let mut segments = Vec::new();
    let mut previous_eigenbasis: Option<String> = None;
    for (index, frame) in frames.iter().enumerate() {
        let retained = index.checked_sub(1).map(|at| &applications[at]);
        // Frames before the actual first installation have no receiver and
        // cannot invent a historical emitter below its true native birth.
        if frame
            .owner()
            .source_assets()
            .get("acoustic_receiving")
            .is_none()
        {
            if !segments.is_empty() {
                return Err("dated source removed an already installed receiver".into());
            }
            continue;
        }
        let frame_source = frame.receiving_source(source)?;
        let acoustic = frame.owner().prepare_retained_acoustic_sources(
            frame.current(),
            &frame_source,
            cursor,
        )?;
        let packet = acoustic.packet();
        let body = frame.owner().packet()?["physical_body"].clone();
        let request = &body["request"];
        let effective =
            retained.map_or(Ok(birth), |row| decimal(&row["source"]["native_sample"]))?;
        let eigenbasis = if let Some(retained) = retained {
            let record = &retained["source"];
            let pulse = &retained["native_application"];
            if pulse["accepted"] != true
                || pulse["reading"]["samples_elapsed"] != record["native_sample"]
            {
                return Err("dated emitter lost original actual accepted native date".into());
            }
            if record["schema"] == "ql.native-physical-source-transition/v1" {
                let ack = &pulse["payload"]["physical_transition"];
                if previous_eigenbasis
                    .as_ref()
                    .is_some_and(|old| ack["before_eigenbasis_identity"] != *old)
                    || ack["after_native_preparation"] != frame.owner().packet()?
                    || ack["native_sample"] != record["native_sample"]
                {
                    return Err(
                        "dated emitter lost original native P transition eigenbasis/date".into(),
                    );
                }
                reference(&ack["after_eigenbasis_identity"])?.to_owned()
            } else if record["schema"] == "ql.native-acoustic-source-transition/v1" {
                let manifest = &pulse["reading"]["receiving_transport"]["manifest"];
                if record["after_acoustic"] != *packet
                    || manifest["eigenbasis"] != pulse["reading"]["physical"]["eigenbasis_identity"]
                    || previous_eigenbasis
                        .as_ref()
                        .is_some_and(|old| manifest["eigenbasis"] != *old)
                {
                    return Err(
                        "dated acoustic epoch lost full original packet/unchanged P eigenbasis"
                            .into(),
                    );
                }
                if record["kind"] == "install" {
                    if record["before_acoustic"] != Value::Null
                        || effective != birth
                        || !segments.is_empty()
                    {
                        return Err(
                            "dated initial source backdated/replaced actual receiver birth".into(),
                        );
                    }
                } else if record["kind"] == "replace" {
                    let ack = &pulse["payload"]["receiving_replacement"];
                    if ack["sample"] != record["native_sample"]
                        || ack["after_manifest"] != *manifest
                        || ack["before_manifest"]["eigenbasis"] != manifest["eigenbasis"]
                    {
                        return Err("dated receiver source lost original replacement ACK".into());
                    }
                } else {
                    return Err("unknown original dated acoustic kind".into());
                }
                reference(&manifest["eigenbasis"])?.to_owned()
            } else {
                return Err("unknown original dated source operation".into());
            }
        } else {
            let first = &applications[0];
            if first["source"]["schema"] == "ql.native-physical-source-transition/v1" {
                reference(
                    &first["native_application"]["payload"]["physical_transition"]
                        ["before_eigenbasis_identity"],
                )?
                .to_owned()
            } else if first["source"]["kind"] == "replace" {
                reference(
                    &first["native_application"]["payload"]["receiving_replacement"]
                        ["before_manifest"]["eigenbasis"],
                )?
                .to_owned()
            } else {
                return Err(
                    "initial acoustic origin lacks its actual installed source epoch".into(),
                );
            }
        };
        if effective > cursor
            || decimal(&packet["history_origin_sample"])? != birth
            || packet["source_body"] != body
        {
            return Err("dated frame lost its exact acoustic/body/birth source".into());
        }
        let anchor = vector(&packet["source_position_metres"], true)?;
        let velocity = vector(
            &packet["configuration"]["source_velocity_metres_per_second"],
            false,
        )?;
        let revision = request["body_revision"]
            .as_u64()
            .filter(|v| *v > 0)
            .ok_or("dated body revision absent")?;
        let generation = body["source_generation"]
            .as_u64()
            .ok_or("dated source generation absent")?;
        let face = body["source_coordinate"]["face"]
            .as_str()
            .ok_or("dated source face absent")?;
        if !["bimba", "pratibimba"].contains(&face) {
            return Err("dated source face differs".into());
        }
        let segment = json!({"effective_sample":effective.to_string(),"trajectory_origin_sample":decimal(&packet["origin_sample"] )?.to_string(),
            "body_revision":revision.to_string(),"source_generation":generation.to_string(),"pratibimba":face=="pratibimba",
            "source_coordinate":body["source_coordinate"]["source_ref"],"source_revision":body["source_revision"],
            "preparation":request["preparation_ref"],"state":request["state_ref"],"eigenbasis":eigenbasis,
            "source_motion":packet["configuration"]["source_motion_ref"],"anchor_metres":anchor,"velocity_metres_per_second":velocity});
        append(&mut segments, segment, birth)?;
        previous_eigenbasis = Some(eigenbasis);
    }
    let history = json!({"schema":"ql.receiving-emission-source-history/v1","date_units":"native-audio-sample","anchor_units":"m","velocity_units":"m/s",
        "first_retained":retained_first(&segments,cursor,birth)?,"segments":segments});
    require_exact(&receiving["source_history"], &history)?;
    let final_segment = history["segments"]
        .as_array()
        .and_then(|v| v.last())
        .ok_or("dated final source absent")?;
    let manifest = &receiving["manifest"];
    for field in REFERENCE_FIELDS {
        if manifest[field] != final_segment[field] {
            return Err(
                "saved receiving manifest is stale to its last original emitter source".into(),
            );
        }
    }
    for field in ["body_revision", "source_generation", "pratibimba"] {
        if manifest[field] != final_segment[field] {
            return Err("dated native manifest body/face/generation differs".into());
        }
    }
    let final_packet = last.owner().packet()?["physical_body"].clone();
    for (field, value) in [
        ("event_ref", &final_packet["event_ref"]),
        ("subject_ref", &final_packet["subject_ref"]),
        ("preparation_ref", &final_segment["preparation"]),
        ("state_ref", &final_segment["state"]),
        ("source_coordinate", &final_segment["source_coordinate"]),
        ("source_revision", &final_segment["source_revision"]),
        ("body_revision", &final_segment["body_revision"]),
        ("source_generation", &final_segment["source_generation"]),
        ("face", &final_packet["source_coordinate"]["face"]),
    ] {
        if physical["identity"][field] != *value {
            return Err("saved P checkpoint lost the exact final replayed body source".into());
        }
    }
    for (prefix, provenance) in [
        (
            "geometry",
            &final_packet["request"]["geometry"]["provenance"],
        ),
        (
            "material",
            &final_packet["request"]["material"]["provenance"],
        ),
    ] {
        for (suffix, key) in [
            ("ref", "reference"),
            ("revision", "revision"),
            ("source_ref", "source_ref"),
            ("standing", "standing"),
        ] {
            if physical["identity"][format!("{prefix}_{suffix}")] != provenance[key] {
                return Err(
                    "saved P checkpoint changed complete native geometry/material provenance"
                        .into(),
                );
            }
        }
    }
    if physical["basis"]["eigenbasis_identity"] != final_segment["eigenbasis"]
        || physical["basis"]["sample_rate"] != final_packet["request"]["sample_rate"]
        || manifest["event"] != final_packet["event_ref"]
        || manifest["subject"] != final_packet["subject_ref"]
        || manifest["sample_rate"] != final_packet["request"]["sample_rate"]
        || decimal(&manifest["history_origin_sample"])? != birth
    {
        return Err("dated final native P/receiver eigenbasis/clock/source differs".into());
    }
    Ok(history)
}
