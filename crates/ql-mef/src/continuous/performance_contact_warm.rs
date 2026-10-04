//! SAME retained owner's selected Act checkpoint preflight. No owner activation,
//! numerical restore, callback, P advance, imported lease or Contact issuance.
use super::contact_history::{
    ColdNativeContactCorpus, PreparedColdContactSource, QualifiedContactCheckpoint,
};
use super::retained_evidence::{contact_checkpoint_evidence, encoded_bound, exact_value};
use super::*;
use crate::continuous::performance_act_bridge::NativeActSourceLease;
use serde::{Serialize, Serializer, ser::SerializeSeq};

const SOURCE_ASSET_LIMIT: usize = 8 * 1024 * 1024;

/// A different genuine prefix requires Root's selected source re-adoption;
/// numerical restoration cannot silently retain the old body's source labels.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum NativeWarmSourceCheckpointFailureKind {
    Refused,
    NeedSourceRequalification,
}
/// Full original source/CP and all original native numerical replies survive
/// refusal. This diagnostic record cannot mint a source or receiving grant.
pub(crate) struct NativeWarmSourceCheckpointFailure {
    pub(crate) kind: NativeWarmSourceCheckpointFailureKind,
    pub(crate) reason: String,
    pub(crate) original_selection: Value,
    pub(crate) native_receipts: Vec<Value>,
}
impl NativeWarmSourceCheckpointFailure {
    fn before(reason: String, original_selection: &Value) -> Self {
        Self {
            kind: NativeWarmSourceCheckpointFailureKind::Refused,
            reason,
            original_selection: original_selection.clone(),
            native_receipts: Vec::new(),
        }
    }
}

enum WarmReplayedSource {
    Original(PreparedColdPhysicalSource),
    Contact(PreparedColdContactSource),
}
impl WarmReplayedSource {
    fn physical(&self) -> &PreparedColdPhysicalSource {
        match self {
            Self::Original(v) => v,
            Self::Contact(v) => v.physical(),
        }
    }
}
/// Non-Clone/non-Deserialize. It retains genuine original source frames while
/// Root qualifies dated M4 and restores the SAME selected native checkpoint.
pub(crate) struct PreparedWarmNativeSourceCheckpoint {
    replayed: WarmReplayedSource,
    contact: Option<QualifiedContactCheckpoint>,
    selected_source: Value,
    current_epoch: Value,
    operative_current: Value,
    receiving_inputs: Value,
    instance: String,
    checkpoint_ref: String,
    original_wire: String,
    closed_selection: Value,
}
fn sidecar<'a>(asset: &'a Value, key: &str) -> Result<&'a [Value], String> {
    match asset.get(key) {
        None => Ok(&[]),
        Some(value) => value
            .as_array()
            .map(Vec::as_slice)
            .ok_or_else(|| format!("complete warm {key} is not its original array")),
    }
}
struct BorrowedPhysicalHistory<'a>(&'a [form::NativePhysicalTransitionRecord]);
impl Serialize for BorrowedPhysicalHistory<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut sequence = serializer.serialize_seq(Some(self.0.len()))?;
        for record in self.0 {
            sequence.serialize_element(&record.borrowed_snapshot())?;
        }
        sequence.end()
    }
}
#[derive(Serialize)]
struct BorrowedOwnerEpoch<'a> {
    native_bundle: &'a Value,
    native_physical_source_history: BorrowedPhysicalHistory<'a>,
    native_acoustic_source_history: &'a [Value],
    native_contact_admission_history: &'a [Value],
}
fn borrowed_owner_epoch(owner: &PerformanceOwner) -> BorrowedOwnerEpoch<'_> {
    BorrowedOwnerEpoch {
        native_bundle: owner.source_assets(),
        native_physical_source_history: BorrowedPhysicalHistory(&owner.physical_source_history),
        native_acoustic_source_history: &owner.acoustic_source_history,
        native_contact_admission_history: &owner.contact_admission_history,
    }
}
fn owner_epoch(owner: &PerformanceOwner) -> Result<Value, String> {
    let original = borrowed_owner_epoch(owner);
    encoded_bound(&original, SOURCE_ASSET_LIMIT, "current source corpus")?;
    serde_json::to_value(original).map_err(|e| e.to_string())
}
#[derive(Serialize)]
struct BorrowedWarmOriginals<'a> {
    schema: &'static str,
    selected_source: &'a Value,
    current_epoch: BorrowedOwnerEpoch<'a>,
    checkpoint_ref: &'a str,
    original_checkpoint_wire: &'a str,
    closed_selection: &'a Value,
    current_native_pulse: &'a Option<Value>,
}
#[derive(Serialize)]
struct BorrowedWarmReplayOriginals<'a> {
    #[serde(flatten)]
    originals: &'a Value,
    prepared_contact_replay_request: &'a Value,
}
fn same_epoch(current: &Value, selected: &Value) -> Result<bool, String> {
    if !exact_value(&current["native_bundle"], &selected["native_bundle"]) {
        return Ok(false);
    }
    for key in [
        "native_physical_source_history",
        "native_acoustic_source_history",
        "native_contact_admission_history",
    ] {
        if !sidecar(current, key)?
            .iter()
            .zip(sidecar(selected, key)?)
            .all(|(a, b)| exact_value(a, b))
            || sidecar(current, key)?.len() != sidecar(selected, key)?.len()
        {
            return Ok(false);
        }
    }
    Ok(true)
}
fn qualify_source(
    owner: &PerformanceOwner,
    current: &CoupledBasis,
    source: &NativePerformanceReceivingSource,
) -> Result<(), String> {
    owner.validate_current(current)?;
    if !exact_value(
        &owner.original_input,
        &serde_json::to_value(&current.input).map_err(|e| e.to_string())?,
    ) {
        return Err("warm operative native input changed finite numeric bits".into());
    }
    if !exact_value(
        &owner.source_assets()["original_native_input"],
        &serde_json::to_value(&owner.source_origin.input).map_err(|e| e.to_string())?,
    ) {
        return Err("warm owner lost its actual immutable original constructor".into());
    }
    let sample = decimal(
        &owner.source_assets()["current_receiving"]["native_admission"]["operation"]["native_sample"],
    )?;
    let cursor = decimal(
        &owner
            .reading()
            .ok_or("actual current warm native reading absent")?["samples_elapsed"],
    )?;
    let fresh = source.prepare_current(owner, current, cursor)?;
    fresh.validate_current(source, owner, current, cursor)?;
    let prepared = source.prepare_retained(
        owner,
        current,
        sample,
        &owner.source_assets()["current_receiving"],
    )?;
    if !exact_value(
        prepared.retained_snapshot(),
        &owner.source_assets()["current_receiving"],
    ) || !exact_value(
        &source.source_inputs()?,
        &owner.source_assets()["receiving_source_inputs"],
    ) {
        return Err("warm native source changed actual N9/World/occasion/context epoch".into());
    }
    Ok(())
}
fn qualify_checkpoint_body(owner: &PerformanceOwner, saved: &Value) -> Result<(), String> {
    let body = owner.binding().physical_body();
    let request = body.request();
    let physical = &saved["native_pair"]["physical"];
    let identity = &physical["identity"];
    let audio = &saved["native_pair"]["audio"];
    let reading = owner
        .reading()
        .ok_or("actual retained warm native reading absent")?;
    let expected_face = if body.source_coordinate().face == MFace::Pratibimba {
        "pratibimba"
    } else {
        "bimba"
    };
    if saved["schema"] != "ql.performance-management-checkpoint/v1"
        || physical["schema"] != "ql.physical-body-checkpoint/v1"
        || identity["event_ref"] != body.event_ref()
        || identity["subject_ref"] != body.subject_ref()
        || identity["preparation_ref"] != request.preparation_ref
        || identity["state_ref"] != request.state_ref
        || identity["source_coordinate"] != body.source_coordinate().source_ref
        || identity["source_revision"] != body.source_revision()
        || identity["face"] != expected_face
        || decimal(&identity["source_generation"])? != body.source_generation()
        || decimal(&identity["body_revision"])? != request.body_revision
        || physical["basis"]["sample_rate"] != request.sample_rate
        || physical["basis"]["eigenbasis_identity"] != reading["physical"]["eigenbasis_identity"]
        || audio["producer_identity"] != owner.binding().determination()["identity"]
        || audio["sample_rate"] != request.sample_rate
        || decimal(&physical["state"]["samples_elapsed"])? != decimal(&audio["cursor"])?
    {
        return Err(
            "warm selected checkpoint detached original native source/body/face/eigenbasis".into(),
        );
    }
    Ok(())
}
impl PreparedWarmNativeSourceCheckpoint {
    pub(crate) fn physical(&self) -> &PreparedColdPhysicalSource {
        self.replayed.physical()
    }
    /// Repeat directly before and after the only actual restore. The current
    /// numerical cursor may move, but its complete native source epoch cannot.
    pub(crate) fn validate_held(
        &self,
        owner: &PerformanceOwner,
        current: &CoupledBasis,
        source: &NativePerformanceReceivingSource,
        lease: &NativeActSourceLease<'_>,
    ) -> Result<(), String> {
        qualify_source(owner, current, source)?;
        if !exact_value(&self.current_epoch, &owner_epoch(owner)?)
            || !exact_value(
                &self.operative_current,
                &serde_json::to_value(current).map_err(|e| e.to_string())?,
            )
            || !exact_value(&self.receiving_inputs, &source.source_inputs()?)
            || !exact_value(&self.closed_selection, &lease.evidence())
            || !exact_value(
                lease.selected_checkpoint_source(&self.instance)?,
                &self.selected_source,
            )
        {
            return Err("warm prepared checkpoint lost its held complete source epoch".into());
        }
        lease.validate_selected_checkpoint(
            &self.instance,
            &self.checkpoint_ref,
            &self.original_wire,
        )?;
        lease.validate_source_assets(&self.instance, owner.source_assets())?;
        lease.validate_recorded_source_applications(
            &self.instance,
            owner.source_assets(),
            &owner.native_physical_source_history(),
            &owner.native_acoustic_source_history(),
        )?;
        lease.validate_recorded_contact_admissions(
            &self.instance,
            owner.source_assets(),
            &owner.native_contact_admission_history(),
        )?;
        match (&self.replayed, &self.contact) {
            (WarmReplayedSource::Original(_), None) => Ok(()),
            (WarmReplayedSource::Contact(prepared), Some(qualified)) => {
                qualified.validate_prepared(prepared, lease)
            }
            _ => Err(
                "warm Contact checkpoint lacks its original private numeric qualification".into(),
            ),
        }
    }
    pub(crate) fn contact_evidence(
        &self,
        lease: &NativeActSourceLease<'_>,
    ) -> Result<Option<Value>, String> {
        match (&self.replayed, &self.contact) {
            (WarmReplayedSource::Original(_), None) => Ok(None),
            (WarmReplayedSource::Contact(prepared), Some(qualified)) => {
                qualified.validate_prepared(prepared, lease)?;
                Ok(Some(contact_checkpoint_evidence(
                    qualified.original_request(),
                    qualified.original_reply(),
                )?))
            }
            _ => Err("warm Contact evidence lacks actual native numerical reply".into()),
        }
    }
}
impl PerformanceOwner {
    /// Private selected-Act preparation only. It replays from this actual
    /// resident owner's immutable constructor, never from imported final input.
    pub(crate) fn prepare_warm_native_source_checkpoint(
        &self,
        current: &CoupledBasis,
        source: &NativePerformanceReceivingSource,
        session: &mut CoupledFieldSession,
        lease: &NativeActSourceLease<'_>,
        checkpoint_ref: &str,
        original_wire: &str,
    ) -> Result<PreparedWarmNativeSourceCheckpoint, NativeWarmSourceCheckpointFailure> {
        let instance = self.binding().determination()["identity"]["instance"]
            .as_str()
            .ok_or_else(|| {
                NativeWarmSourceCheckpointFailure::before(
                    "actual warm native instance absent".into(),
                    &Value::Null,
                )
            })?
            .to_owned();
        let selected = (|| -> Result<&Value, String> {
            bounded(&instance)?;
            bounded(checkpoint_ref)?;
            if original_wire.is_empty() || original_wire.len() > crate::continuous::MAX_MESSAGE {
                return Err("complete original warm checkpoint exceeds native32MiB bound".into());
            }
            lease.validate_selected_checkpoint(&instance, checkpoint_ref, original_wire)?;
            lease.selected_checkpoint_source(&instance)
        })()
        .map_err(|reason| NativeWarmSourceCheckpointFailure::before(reason, &Value::Null))?;
        // Charge every original while borrowed, including escaped checkpoint
        // text and all three sidecars, BEFORE any retained-envelope clone.
        let closed_selection = lease.evidence();
        let borrowed = BorrowedWarmOriginals {
            schema: "ql.native-warm-source-checkpoint-originals/v1",
            selected_source: selected,
            current_epoch: borrowed_owner_epoch(self),
            checkpoint_ref,
            original_checkpoint_wire: original_wire,
            closed_selection: &closed_selection,
            current_native_pulse: &self.last,
        };
        let limits = (|| -> Result<(), String> {
            encoded_bound(selected, SOURCE_ASSET_LIMIT, "selected source asset")?;
            encoded_bound(
                &borrowed.current_epoch,
                SOURCE_ASSET_LIMIT,
                "current source corpus",
            )?;
            encoded_bound(
                &borrowed,
                crate::continuous::MAX_MESSAGE,
                "source/checkpoint originals",
            )?;
            Ok(())
        })();
        limits.map_err(|reason| NativeWarmSourceCheckpointFailure::before(reason, &Value::Null))?;
        let mut originals = serde_json::to_value(&borrowed)
            .map_err(|e| NativeWarmSourceCheckpointFailure::before(e.to_string(), &Value::Null))?;
        let current_epoch = originals["current_epoch"].clone();
        let preflight = || -> Result<(), String> {
            encoded_bound(selected, SOURCE_ASSET_LIMIT, "selected source asset")?;
            encoded_bound(&current_epoch, SOURCE_ASSET_LIMIT, "current source corpus")?;
            encoded_bound(
                &originals,
                crate::continuous::MAX_MESSAGE,
                "source/checkpoint originals",
            )?;
            lease.validate_source_assets(&instance, &selected["native_bundle"])?;
            lease.validate_recorded_source_applications(
                &instance,
                &selected["native_bundle"],
                sidecar(selected, "native_physical_source_history")?,
                sidecar(selected, "native_acoustic_source_history")?,
            )?;
            lease.validate_recorded_contact_admissions(
                &instance,
                &selected["native_bundle"],
                sidecar(selected, "native_contact_admission_history")?,
            )?;
            qualify_source(self, current, source)
        };
        preflight()
            .map_err(|reason| NativeWarmSourceCheckpointFailure::before(reason, &originals))?;
        if !same_epoch(&current_epoch, selected)
            .map_err(|reason| NativeWarmSourceCheckpointFailure::before(reason, &originals))?
        {
            return Err(NativeWarmSourceCheckpointFailure {
                kind: NativeWarmSourceCheckpointFailureKind::NeedSourceRequalification,
                reason:"warm checkpoint selected a different complete source/sidecar prefix; genuine source re-adoption required".into(),
                original_selection: originals, native_receipts: Vec::new(),
            });
        }
        let prepared = (|| -> Result<WarmReplayedSource, String> {
            let saved: Value = serde_json::from_str(original_wire).map_err(|e| e.to_string())?;
            qualify_checkpoint_body(self, &saved)?;
            let records = match selected["native_bundle"].get("contact_occurrence_history") {
                None => &[][..],
                Some(value) => value
                    .as_array()
                    .map(Vec::as_slice)
                    .ok_or("warm original Contact occurrences are not an array")?,
            };
            let physical = sidecar(selected, "native_physical_source_history")?;
            let acoustic = sidecar(selected, "native_acoustic_source_history")?;
            let contacts = sidecar(selected, "native_contact_admission_history")?;
            if records.is_empty() {
                if !contacts.is_empty() || saved["native_pair"]["audio"]["version"] != 2 {
                    return Err(
                        "warm Contact checkpoint lost original native occurrence custody".into(),
                    );
                }
                Ok(WarmReplayedSource::Original(
                    Self::prepare_cold_native_acoustic_source(
                        &self.source_origin,
                        &instance,
                        source,
                        &selected["native_bundle"],
                        physical,
                        acoustic,
                        lease,
                    )?,
                ))
            } else {
                if saved["native_pair"]["audio"]["version"] != 3 {
                    return Err(
                        "warm native Contact source lost its actual CPv3 continuation".into(),
                    );
                }
                Ok(WarmReplayedSource::Contact(
                    Self::prepare_cold_native_contact_source(
                        &self.source_origin,
                        &instance,
                        source,
                        &selected["native_bundle"],
                        ColdNativeContactCorpus {
                            physical_applications: physical,
                            acoustic_applications: acoustic,
                            contact_admissions: contacts,
                        },
                        lease,
                    )?,
                ))
            }
        })()
        .map_err(|reason| NativeWarmSourceCheckpointFailure::before(reason, &originals))?;
        let frame = prepared.physical().final_frame();
        if !exact_value(frame.owner().source_assets(), self.source_assets())
            || !exact_value(
                &frame.owner().native_packet().map_err(|reason| {
                    NativeWarmSourceCheckpointFailure::before(reason, &originals)
                })?,
                &self.native_packet().map_err(|reason| {
                    NativeWarmSourceCheckpointFailure::before(reason, &originals)
                })?,
            )
            || !exact_value(
                &serde_json::to_value(frame.current()).map_err(|e| {
                    NativeWarmSourceCheckpointFailure::before(e.to_string(), &originals)
                })?,
                &serde_json::to_value(current).map_err(|e| {
                    NativeWarmSourceCheckpointFailure::before(e.to_string(), &originals)
                })?,
            )
        {
            return Err(NativeWarmSourceCheckpointFailure::before(
                "warm original producer cannot replay the exact current body/basis/source".into(),
                &originals,
            ));
        }
        let operative_current = serde_json::to_value(current)
            .map_err(|e| NativeWarmSourceCheckpointFailure::before(e.to_string(), &originals))?;
        let receiving_inputs = source
            .source_inputs()
            .map_err(|reason| NativeWarmSourceCheckpointFailure::before(reason, &originals))?;
        if let WarmReplayedSource::Contact(replayed) = &prepared {
            // Exact deterministic preimage, with no issuance claim on a lost
            // transport. Only the sealed compiler below issues this request.
            let replay_request = replayed
                .verification_request(&instance, checkpoint_ref, original_wire)
                .map_err(|reason| NativeWarmSourceCheckpointFailure::before(reason, &originals))?;
            encoded_bound(
                &BorrowedWarmReplayOriginals {
                    originals: &originals,
                    prepared_contact_replay_request: &replay_request,
                },
                crate::continuous::MAX_MESSAGE,
                "source/checkpoint/preflight originals",
            )
            .map_err(|reason| NativeWarmSourceCheckpointFailure::before(reason, &originals))?;
            originals["prepared_contact_replay_request"] = replay_request;
        }
        let contact = match &prepared {
            WarmReplayedSource::Original(_) => None,
            WarmReplayedSource::Contact(replayed) => Some(
                replayed
                    .qualify_original_native_checkpoint(
                        session,
                        &instance,
                        checkpoint_ref,
                        original_wire,
                        lease,
                    )
                    .map_err(|failure| NativeWarmSourceCheckpointFailure {
                        kind: NativeWarmSourceCheckpointFailureKind::Refused,
                        reason: failure.reason,
                        original_selection: originals.clone(),
                        native_receipts: failure.native_receipts,
                    })?,
            ),
        };
        let native_receipts = contact
            .as_ref()
            .map(|v| vec![v.original_reply().clone()])
            .unwrap_or_default();
        let result = PreparedWarmNativeSourceCheckpoint {
            replayed: prepared,
            contact,
            selected_source: selected.clone(),
            current_epoch,
            operative_current,
            receiving_inputs,
            instance,
            checkpoint_ref: checkpoint_ref.into(),
            original_wire: original_wire.into(),
            closed_selection: lease.evidence(),
        };
        result
            .validate_held(self, current, source, lease)
            .and_then(|()| result.contact_evidence(lease).map(|_| ()))
            .map_err(|reason| NativeWarmSourceCheckpointFailure {
                kind: NativeWarmSourceCheckpointFailureKind::Refused,
                reason,
                original_selection: originals,
                native_receipts,
            })?;
        Ok(result)
    }
}

#[cfg(test)]
mod source_tests {
    use super::*;
    // The actual native worker mixed-source producer is the ONLY corpus. This
    // tests the source-prefix guard without constructing a C lease/positive.
    #[test]
    #[ignore = "requires original real native Form/material/M4 producer corpus"]
    fn actual_native_acoustic_corpus_retains_exact_warm_prefixes() {
        let path = std::env::var("QL_NATIVE_ACOUSTIC_SOURCE_REPLAY_ARTIFACT")
            .expect("actual native mixed-source corpus required; no fallback");
        let corpus: Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        assert_eq!(
            corpus["schema"],
            "ql.actual-native-acoustic-source-replay-component/v1"
        );
        let frames = corpus["frames"].as_array().unwrap();
        assert_eq!(frames.len(), 6);
        assert_eq!(frames[1]["basis"], frames[2]["basis"]);
        assert_ne!(frames[1]["source_assets"], frames[2]["source_assets"]);
        let asset = |frame: &Value| {
            json!({"native_bundle":frame["source_assets"],
            "native_physical_source_history":frame["native_physical_source_history"],
            "native_acoustic_source_history":frame["native_acoustic_source_history"]})
        };
        let mut actual = Vec::new();
        for frame in frames {
            actual.push(asset(frame));
        }
        let repeat_original = || {
            let reopened: Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
            assert!(exact_value(&corpus, &reopened));
            let reopened_frames = reopened["frames"].as_array().unwrap();
            assert_eq!(reopened_frames.len(), actual.len());
            for (held, reopened_frame) in actual.iter().zip(reopened_frames) {
                let reopened_asset = asset(reopened_frame);
                assert!(exact_value(held, &reopened_asset));
                assert!(same_epoch(held, &reopened_asset).unwrap());
                let count = encoded_bound(
                    &reopened_asset,
                    SOURCE_ASSET_LIMIT,
                    "reopened whole producer epoch",
                )
                .unwrap();
                assert!(
                    encoded_bound(&reopened_asset, count, "reopened whole producer epoch").is_ok()
                );
            }
        };
        repeat_original();
        for (index, current) in actual.iter().enumerate() {
            assert!(same_epoch(current, current).unwrap());
            let count =
                encoded_bound(current, SOURCE_ASSET_LIMIT, "actual producer epoch").unwrap();
            assert!(count > 0 && count <= SOURCE_ASSET_LIMIT);
            assert!(encoded_bound(current, count, "actual producer epoch").is_ok());
            assert!(encoded_bound(current, count - 1, "actual producer epoch").is_err());
            repeat_original();
            for (other_index, selected) in actual.iter().enumerate() {
                if index != other_index {
                    assert!(!same_epoch(current, selected).unwrap());
                    repeat_original();
                }
            }
            for key in [
                "native_physical_source_history",
                "native_acoustic_source_history",
            ] {
                if !sidecar(current, key).unwrap().is_empty() {
                    let mut lost = current.clone();
                    lost[key].as_array_mut().unwrap().remove(0);
                    assert!(!same_epoch(&lost, current).unwrap());
                    repeat_original();
                    let mut disconnected = current.clone();
                    disconnected[key][0]
                        .as_object_mut()
                        .unwrap()
                        .remove("native_application")
                        .unwrap();
                    assert!(!same_epoch(&disconnected, current).unwrap());
                    repeat_original();
                    let mut malformed = current.clone();
                    malformed[key] = json!({"lost":"original-array"});
                    assert!(same_epoch(&malformed, current).is_err());
                    repeat_original();
                }
            }
            let mut wrong_face = current.clone();
            let face = wrong_face["native_bundle"]["configuration"]["physical_face"]
                .as_u64()
                .unwrap();
            assert!(face <= 1);
            wrong_face["native_bundle"]["configuration"]["physical_face"] = json!(1 - face);
            assert!(!same_epoch(&wrong_face, current).unwrap());
            repeat_original();
            let mut lost_descendant = current.clone();
            lost_descendant["native_bundle"]
                .as_object_mut()
                .unwrap()
                .remove("source_geometry_reading")
                .unwrap();
            assert!(!same_epoch(&lost_descendant, current).unwrap());
            repeat_original();
            // Derive the negative solely from an actual producer f64 zero.
            // Retain/restoring its complete original value tests the true epoch,
            // not a synthesized source/lease or a made-up native positive.
            fn flip_actual_zero(value: &mut Value) -> bool {
                match value {
                    Value::Number(n) if n.is_f64() && n.as_f64().is_some_and(|v| v == 0.0) => {
                        let original = n.as_f64().unwrap();
                        *n = serde_json::Number::from_f64(f64::from_bits(
                            original.to_bits() ^ (1u64 << 63),
                        ))
                        .unwrap();
                        true
                    }
                    Value::Array(a) => a.iter_mut().any(flip_actual_zero),
                    Value::Object(a) => a.values_mut().any(flip_actual_zero),
                    _ => false,
                }
            }
            let mut signed_zero = current.clone();
            assert!(
                flip_actual_zero(&mut signed_zero["native_bundle"]),
                "genuine native producer f64 zero required; no fallback"
            );
            assert!(!same_epoch(&signed_zero, current).unwrap());
            repeat_original();
            assert!(exact_value(current, &asset(&frames[index])));
            assert!(same_epoch(current, current).unwrap());
            for key in [
                "native_physical_source_history",
                "native_acoustic_source_history",
            ] {
                if !sidecar(current, key).unwrap().is_empty() {
                    let mut signed_zero_sidecar = current.clone();
                    assert!(
                        flip_actual_zero(&mut signed_zero_sidecar[key]),
                        "genuine full native sidecar f64 zero required; no fallback"
                    );
                    assert!(!same_epoch(&signed_zero_sidecar, current).unwrap());
                    repeat_original();
                    assert!(exact_value(current, &asset(&frames[index])));
                    assert!(same_epoch(current, current).unwrap());
                }
            }
            // Independently reopen and repeat all six whole original epochs after
            // each refusal, retaining their original producer bytes throughout.
            assert!(exact_value(current, &asset(&frames[index])));
            assert!(same_epoch(current, current).unwrap());
        }
    }
}
