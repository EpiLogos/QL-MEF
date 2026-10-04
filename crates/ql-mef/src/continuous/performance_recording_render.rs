//! A stopped native callback on the same current Scene, Manager and Engine.
//! Its only caller is the OS-qualified C current-Document Act channel. No
//! public control, stored receipt, test flag or raw render scope grants it.
use super::super::performance::NativeStoppedExchangeFailure;
use super::super::performance::retained_evidence::{encoded_bound, exact_value};
use super::super::performance_act_bridge::NativeActSourceLease;
use super::FieldHost;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CurrentRenderSelection {
    schema: String,
    performance_digest: String,
    basis_digest: String,
    checkpoint_ref: String,
    checkpoint_digest: String,
    sample: String,
    event_prefix_digest: String,
}
#[derive(Serialize)]
pub(crate) struct NativeRecordingRenderRefusal {
    pub(crate) reason: String,
    pub(crate) source_artifact: Option<Value>,
    pub(crate) original_request: Option<Value>,
    pub(crate) native_receipts: Vec<Value>,
}
impl From<String> for NativeRecordingRenderRefusal {
    fn from(reason: String) -> Self {
        Self {
            reason,
            source_artifact: None,
            original_request: None,
            native_receipts: Vec::new(),
        }
    }
}
impl From<&str> for NativeRecordingRenderRefusal {
    fn from(reason: &str) -> Self {
        reason.to_owned().into()
    }
}
impl From<NativeStoppedExchangeFailure> for NativeRecordingRenderRefusal {
    fn from(failure: NativeStoppedExchangeFailure) -> Self {
        Self {
            reason: failure.reason,
            source_artifact: None,
            original_request: None,
            native_receipts: failure.native_receipts,
        }
    }
}
pub(crate) struct NativeCapturedRecordingRender {
    pub(crate) source_artifact: Value,
    pub(crate) original_request: Value,
    pub(crate) native_pulse: Value,
}
fn counter(v: &Value) -> Result<u64, String> {
    let text = v
        .as_str()
        .ok_or("actual recording activity counter absent")?;
    let n = text.parse::<u64>().map_err(|e| e.to_string())?;
    if text != n.to_string() {
        return Err("actual recording activity counter is noncanonical".into());
    }
    Ok(n)
}
fn digest(text: &str) -> Result<(), String> {
    let hex = text
        .strip_prefix("sha256:")
        .ok_or("native recording selection digest absent")?;
    if hex.len() != 64
        || !hex
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err("native recording selection digest is not canonical".into());
    }
    Ok(())
}
// A conservative literal union of the actual success/refusal result and its
// native-act wrapper. All unbounded fields are borrowed and charged once here;
// the complete future original worker pulse is reserved separately. Empty
// fields overcharge success and refusal rather than omit either response face.
#[derive(Serialize)]
struct BorrowedRecordingResult<'a> {
    schema: &'static str,
    selection: &'a Value,
    accepted: bool,
    reason: &'static str,
    source_artifact: &'a Value,
    original_worker_request: &'a Value,
    native_pulse: Option<&'a Value>,
    native_receipts: [Option<&'a Value>; 1],
    host_receipt: &'a Value,
}
#[derive(Serialize)]
struct BorrowedRecordingEnvelope<'a> {
    schema: &'static str,
    instance_ref: &'a str,
    request_id: &'a str,
    last_request_id: &'a str,
    available: bool,
    status: &'static str,
    result: BorrowedRecordingResult<'a>,
    error: Option<&'a str>,
}
const RECORDING_REASON_AND_METADATA_RESERVATION: usize = 512 * 1024;
impl FieldHost {
    pub(crate) fn capture_native_recording_render(
        &mut self,
        lease: &NativeActSourceLease<'_>,
        manifest: &Value,
        frames: u32,
        actual_selection: &Value,
        actual_request_id: &str,
    ) -> Result<NativeCapturedRecordingRender, NativeRecordingRenderRefusal> {
        if !self.available()
            || !(1..=512).contains(&frames)
            || manifest["schema"] != "oi.expression-native-current-scene-delivery/v1"
        {
            return Err(
                "native stopped activity requires the actual current Scene and 1..512 frames"
                    .into(),
            );
        }
        let selected: CurrentRenderSelection =
            serde_json::from_value(manifest["native_render_selection"].clone())
                .map_err(|e| e.to_string())?;
        if selected.schema != "oi.native-scene-recording-render-selection/v1"
            || selected.checkpoint_ref.is_empty()
            || selected.checkpoint_ref.len() > 4096
            || selected.checkpoint_ref.chars().any(char::is_control)
        {
            return Err("actual privately selected stopped recording cut absent".into());
        }
        for text in [
            &selected.performance_digest,
            &selected.basis_digest,
            &selected.checkpoint_digest,
            &selected.event_prefix_digest,
        ] {
            digest(text)?;
        }
        let performance = &manifest["scene"]["performance"];
        if performance["content_digest"] != selected.performance_digest {
            return Err("native recording activity selected a different full performance".into());
        }
        let mut bases = performance["bases"]
            .as_array()
            .ok_or("native recording bases absent")?
            .iter()
            .filter(|b| b["content_digest"] == selected.basis_digest);
        let basis = bases
            .next()
            .ok_or("native recording selected basis absent")?;
        if bases.next().is_some() {
            return Err("native recording basis selection is ambiguous".into());
        }
        let seed = counter(&basis["seed"])?;
        let mut cuts = performance["checkpoints"]
            .as_array()
            .ok_or("native recording checkpoints absent")?
            .iter()
            .filter(|c| c["checkpoint_ref"] == selected.checkpoint_ref);
        let cut = cuts
            .next()
            .ok_or("native recording exact current cutoff absent")?;
        if cuts.next().is_some()
            || cut["content_digest"] != selected.checkpoint_digest
            || cut["basis_digest"] != selected.basis_digest
            || cut["sample"] != selected.sample
            || cut["event_prefix_digest"] != selected.event_prefix_digest
            || cut["acknowledged_stopped"] != true
            || !exact_value(&cut["identity"], &basis["identity"])
        {
            return Err(
                "native recording lost complete actual Scene cutoff identity/prefix".into(),
            );
        }
        let start = counter(&cut["sample"])?;
        let end = start
            .checked_add(u64::from(frames))
            .ok_or("native recording callback cursor exhausted")?;
        let current = self.session.session().current_basis().clone();
        {
            let owner = self
                .performance
                .as_ref()
                .ok_or("actual recording Engine absent")?;
            owner.validate_current(&current)?;
            lease.validate_source_assets(&self.instance_ref, owner.source_assets())?;
            encoded_bound(
                owner.source_assets(),
                8 * 1024 * 1024,
                "recording callback source bundle",
            )?;
            let before = owner.acknowledged_stopped_checkpoint_pulse()?;
            let saved = &before["payload"]["checkpoint"];
            if counter(&before["reading"]["samples_elapsed"])? != start
                || counter(&saved["native_pair"]["audio"]["cursor"])? != start
                || !exact_value(&saved["native_pair"]["audio"], &cut["audio"])
                || !exact_value(&saved["native_pair"]["physical"], &cut["physical"])
                || counter(&before["reading"]["accepted_sequence"])?
                    != counter(&cut["audio"]["accepted_sequence"])?
            {
                return Err("current Scene cutoff is stale or disconnected from the SAME native body/queued events".into());
            }
        }
        // This getter replays the actual complete held source and all source
        // sidecars without a worker exchange. Preserve it with every later
        // failure; do not try to reconstruct it after callback commitment.
        let source_artifact = self.retained_performance_source_artifact(seed)?;
        let activity = (|| -> Result<super::super::performance::NativeOwnedRecordingReply, NativeRecordingRenderRefusal> {
            let native_basis = source_artifact["basis"]
                .as_object()
                .ok_or("actual native recording basis producer absent")?;
            let scene_basis = basis
                .as_object()
                .ok_or("actual retained recording basis absent")?;
            if native_basis.len() != scene_basis.len()
                || native_basis.iter().any(|(key, value)| {
                    key != "content_digest"
                        && !scene_basis
                            .get(key)
                            .is_some_and(|held| exact_value(value, held))
                })
            {
                return Err("recording activity changed an actual native basis determinant".into());
            }
            let sources = performance["native_sources"]
                .as_array()
                .ok_or("actual complete recording source epochs absent")?;
            let matching = sources
                .iter()
                .filter(|source| {
                    source["basis_digest"] == selected.basis_digest
                        && exact_value(&source["native_bundle"], &source_artifact["source_assets"])
                        && [
                            "native_physical_source_history",
                            "native_acoustic_source_history",
                            "native_contact_admission_history",
                        ]
                        .iter()
                        .all(|key| {
                            match (source.get(*key), source_artifact.get(*key)) {
                                (None, None) => true,
                                (Some(a), Some(b)) => exact_value(a, b),
                                _ => false,
                            }
                        })
                })
                .count();
            if matching != 1 {
                return Err(
                    "recording callback lost its exact full retained source/sidecars".into(),
                );
            }
            if actual_request_id != lease.native_request_id().to_string() {
                return Err("recording envelope request is not its actual native lease ordinal".into());
            }
            let receipt_before = self.recording_controller_receipt(actual_request_id,None);
            let last_request_id=self.last_request.to_string();
            let receiving = self
                .receiving_source
                .as_ref()
                .ok_or("recording current receiving owner absent")?;
            let owner = self
                .performance
                .as_mut()
                .ok_or("recording native owner lost")?;
            let reading = owner.reading().ok_or("recording native reading lost")?;
            let scope = json!({"schema":"ql.native-offline-render-scope/v1","session_ref":reading["session_ref"],
                "scene_ref":manifest["scene_ref"],"performance_revision":selected.performance_digest,
                "performance_digest":selected.performance_digest,"basis_seal":selected.basis_digest,
                "event_prefix_seal":selected.event_prefix_digest,"checkpoint_ref":selected.checkpoint_ref,
                "expected_source":owner.binding().determination()["identity"],"expected_body_revision":reading["scope"]["body_revision"],
                "expected_cursor":reading["samples_elapsed"],"expected_accepted_sequence":reading["accepted_sequence"]});
            let prepared=owner.prepare_owned_recording_request(
                &current,receiving,&json!({"scope":scope,"frames":frames}))?;
            // SAME Field receipt is stable during this stopped P callback:
            // it changes no Field operation/basis. Reserve the complete parsed
            // worker message plus encoded reason twice (result and receipt),
            // status/availability changes and all outer keys BEFORE P advances.
            // Native field-error content is retained once in native_receipts;
            // it is never duplicated as a second escaped JSON reason string.
            let envelope=BorrowedRecordingEnvelope {
                schema:"ql.native-act-owner-result/v1",
                instance_ref:&self.instance_ref,
                request_id:actual_request_id,
                last_request_id:&last_request_id,
                available:false,status:"refused",error:None,
                result:BorrowedRecordingResult {
                    schema:"ql.native-scene-recording-render/v1",
                    selection:actual_selection,accepted:false,reason:"",
                    source_artifact:&source_artifact,
                    original_worker_request:prepared.original_request(),
                    native_pulse:None,native_receipts:[None],host_receipt:&receipt_before,
                },
            };
            encoded_bound(&envelope,
                super::MAX_HOST_OUTPUT-crate::continuous::MAX_MESSAGE
                    -RECORDING_REASON_AND_METADATA_RESERVATION,
                "complete recording success/refusal envelope and original reply reservation")?;
            let actual=owner.exchange_owned_recording_request(
                self.session.session_mut(),prepared).map_err(|failure| NativeRecordingRenderRefusal {
                    reason:failure.reason,source_artifact:None,original_request:Some(failure.original_request),
                    native_receipts:failure.native_receipts,
                })?;
            let pulse=&actual.native_pulse;
            let post = (|| -> Result<(), String> {
                owner.validate_current(&current)?;
                lease.validate_source_assets(&self.instance_ref, owner.source_assets())?;
                let chunk = &pulse["payload"]["chunk"];
                if pulse["operation"] != "offline-render"
                    || pulse["accepted"] != true
                    || chunk["schema"] != "ql.native-offline-render-chunk/v1"
                    || chunk["result"] != "accepted"
                    || chunk["state_committed"] != true
                    || chunk["capture_complete"] != true
                    || counter(&chunk["start_sample"])? != start
                    || counter(&chunk["end_sample"])? != end
                    || counter(&chunk["committed_cursor"])? != end
                    || counter(&pulse["reading"]["samples_elapsed"])? != end
                    || chunk["capture_drops"] != "0"
                    || chunk["callback_failures"] != "0"
                    || chunk["performance_digest"] != selected.performance_digest
                    || chunk["source"] != owner.binding().determination()["identity"]
                    || chunk["channels"] != 1
                    || chunk["sample_rate"] != performance["sample_rate"]
                    || !chunk["interleaved_f32"].as_array().is_some_and(|samples| {
                        samples.len() == frames as usize
                            && samples.iter().all(|v| {
                                v.as_f64().is_some_and(|v| {
                                    v.is_finite() && f64::from(v as f32).to_bits() == v.to_bits()
                                })
                            })
                    })
                {
                    return Err(
                        "actual stopped callback lost source/body/cursor/captured sound".into(),
                    );
                }
                if !exact_value(
                    &serde_json::to_value(self.session.session().current_basis())
                        .map_err(|e| e.to_string())?,
                    &serde_json::to_value(&current).map_err(|e| e.to_string())?,
                ) {
                    return Err("actual source changed during recording callback".into());
                }
                Ok(())
            })();
            match post {
                Ok(()) => Ok(actual),
                Err(reason) => Err(NativeRecordingRenderRefusal {
                    reason,
                    source_artifact: None,
                    original_request: Some(actual.original_request),
                    native_receipts: vec![actual.native_pulse],
                }),
            }
        })();
        match activity {
            Ok(actual) => Ok(NativeCapturedRecordingRender {
                source_artifact,
                original_request: actual.original_request,
                native_pulse: actual.native_pulse,
            }),
            Err(mut failure) => {
                failure.source_artifact = Some(source_artifact);
                Err(failure)
            }
        }
    }
}
