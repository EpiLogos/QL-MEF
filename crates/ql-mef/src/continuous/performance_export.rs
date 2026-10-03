//! Source-qualified selected Act playback on the resident native A/P owner.
//! No imported render scope, oscillator, codec, file store or second clock.
//! The existing host/Act lease authorizes this operation and file disclosure.
use super::coupled::{CoupledBasis, CoupledFieldSession};
use super::performance::PerformanceOwner;
use super::performance_receiving::NativePerformanceReceivingSource;
use crate::musical_performance_score::MusicalPerformanceScore;
use crate::musical_performance_source_score::{
    NativeScorePages, RetainedScoreSource, compile_retained_score,
};
use crate::performance_audio::KeyTouch;
use serde::Serialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
const FRAMES: u64 = 512;

fn count(v: &Value) -> Result<u64, String> {
    let s = v.as_str().ok_or("canonical native export counter absent")?;
    let n: u64 = s.parse().map_err(|_| "native export counter invalid")?;
    if s != n.to_string() {
        return Err("native export counter is not canonical".into());
    }
    Ok(n)
}
fn list(v: &Value) -> Result<&[Value], String> {
    v.as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| "native export array absent".into())
}
fn sha(v: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(v))
}
fn selection(v: &Value) -> Value {
    json!({"act_ref":v["act_ref"],"act_revision":v["act_revision"],"act_digest":v["act_digest"],
        "edition_position":v["edition_position"],"expanded_document_sha256":v["expanded_document_sha256"],
        "scene_ref":v["scene_ref"],"scene_revision":v["scene_revision"],"performance_digest":v["performance_digest"]})
}
/// The same C selection supplies complete native wire; QL has no checkpoint codec.
pub trait NativeActCheckpoints {
    fn read_checkpoint(&mut self, index: usize) -> Result<Value, String>;
}
struct Inputs<'a, P> {
    pages: &'a mut P,
    original: BTreeMap<u64, (String, String)>,
}
impl<P: NativeScorePages> NativeScorePages for Inputs<'_, P> {
    fn read_page(&mut self, index: usize) -> Result<Value, String> {
        let page = self.pages.read_page(index)?;
        if page["decoded"]["kind"] == "applied" {
            for receipt in list(&page["decoded"]["value"]["receipts"])? {
                if receipt["application"]["kind"] == 0 && receipt["application"]["applied"] == true
                {
                    let input = &receipt["original_input"];
                    let touch = count(&input["target"]["touch"])?;
                    let binding = (
                        input["input_ref"]
                            .as_str()
                            .ok_or("original native input absent")?
                            .into(),
                        input["target"]["touch_ref"]
                            .as_str()
                            .ok_or("original native touch ref absent")?
                            .into(),
                    );
                    if self
                        .original
                        .insert(touch, binding.clone())
                        .is_some_and(|old| old != binding)
                    {
                        return Err(
                            "one original native touch has conflicting input ownership".into()
                        );
                    }
                }
            }
        }
        Ok(page)
    }
}
#[derive(Clone)]
struct Occurrence {
    sequence: u64,
    effective: u64,
    action: Value,
}
/// Privately compiled full source/Act result, never deserializable admission.
pub struct NativeActRenderPlan {
    score: MusicalPerformanceScore,
    manifest: Value,
    original_inputs: BTreeMap<u64, (String, String)>,
    occurrences: Vec<Occurrence>,
    from: u64,
    to: u64,
}
impl NativeActRenderPlan {
    pub(crate) fn prepare(
        edition_generation: u64,
        sources: &[RetainedScoreSource<'_>],
        manifest: &Value,
        pages: &mut impl NativeScorePages,
        from: u64,
        to: u64,
    ) -> Result<Self, String> {
        // The complete original native bytes prevent retaining a fingerprint
        // label while omitting a leading historical page or original source.
        let bytes = manifest["canonical_performance_bytes"]
            .as_str()
            .ok_or("complete selected native performance bytes absent")?;
        if bytes.is_empty()
            || bytes.len() > 8 * 1024 * 1024
            || manifest["selected_performance_sha256"] != sha(bytes.as_bytes())
            || serde_json::from_str::<Value>(bytes).map_err(|e| e.to_string())?
                != manifest["performance"]
        {
            return Err("native export selected complete performance bytes differ".into());
        }
        let mut input = Inputs {
            pages,
            original: BTreeMap::new(),
        };
        let score = compile_retained_score(edition_generation, sources, manifest, &mut input)?;
        let performance = &manifest["performance"];
        if from >= to || to > count(&performance["duration_samples"])? {
            return Err("native export interval outside selected performance".into());
        }
        // A changed source/body uses the separate atomic native transition owner.
        // No raw context or vector force is collapsed onto the scalar exciter.
        if sources.len() != 1 {
            return Err(
                "native export requires the qualified multi-source transition owner".into(),
            );
        }
        let layers = list(&performance["layers"])?;
        let solo = layers
            .iter()
            .any(|l| l["enabled"] == true && l["solo"] == true);
        let mut occurrences = Vec::new();
        for page in list(&performance["pages"])? {
            for event in list(&page["events"])? {
                let values = list(event)?;
                let layer = values[2].as_u64().ok_or("native layer index absent")? as usize;
                if values[3] != 0 {
                    return Err("native export source transition is unavailable".into());
                }
                if layers[layer]["enabled"] != true || (solo && layers[layer]["solo"] != true) {
                    continue;
                }
                let action = values[4].clone();
                let delay = if let Some(a) = action.get("a") {
                    let route =
                        &performance["routes"][a[0].as_u64().ok_or("route index absent")? as usize];
                    if route["enabled"] != true {
                        continue;
                    }
                    count(&route["delay_samples"])?
                } else {
                    0
                };
                let effective = count(&values[1])?
                    .checked_add(delay)
                    .ok_or("native route timing overflow")?;
                if effective > count(&performance["duration_samples"])? {
                    return Err("native delayed route exceeds performance".into());
                }
                if action.get("f").is_some() || action == "b" || action == "c" {
                    return Err(
                        "native force/context transition requires its typed prepared consumer"
                            .into(),
                    );
                }
                occurrences.push(Occurrence {
                    sequence: count(&values[0])?,
                    effective,
                    action,
                });
            }
        }
        occurrences.sort_by_key(|e| (e.effective, e.sequence));
        for start in 0..occurrences.len() {
            let end = occurrences.partition_point(|e| {
                e.effective < occurrences[start].effective.saturating_add(FRAMES)
            });
            if end - start > 256 {
                return Err(
                    "selected score exceeds the actual native queue bound in one render window"
                        .into(),
                );
            }
        }
        Ok(Self {
            score,
            manifest: manifest.clone(),
            original_inputs: input.original,
            occurrences,
            from,
            to,
        })
    }
    pub fn score(&self) -> &MusicalPerformanceScore {
        &self.score
    }
    pub fn native_selection(&self) -> Value {
        selection(&self.manifest)
    }
    pub fn scope(&self) -> Value {
        json!({"performance_digest":self.manifest["performance_digest"],"sample_rate":self.manifest["performance"]["sample_rate"],
            "channels":1,"start_sample":self.from.to_string(),"frames":(self.to-self.from).to_string(),
            "native_audio_contract":"ql.performance-audio/v1","native_physical_contract":"ql.physical-body/v1"})
    }
    fn checkpoint(
        &self,
        source: &mut impl NativeActCheckpoints,
    ) -> Result<Option<(Value, Value)>, String> {
        let checkpoints = list(&self.manifest["performance"]["checkpoints"])?;
        let candidate = checkpoints
            .iter()
            .enumerate()
            .filter_map(|(i, c)| {
                count(&c["sample"])
                    .ok()
                    .filter(|s| *s <= self.from)
                    .map(|s| (s, i, c))
            })
            .max_by_key(|v| v.0);
        let Some((sample, index, original)) = candidate else {
            return Ok(None);
        };
        if self.from - sample
            > count(&self.manifest["performance"]["replay"]["max_reconstruction_samples"])?
        {
            return Err("native export exceeds bounded checkpoint reconstruction policy".into());
        }
        let delivery = source.read_checkpoint(index)?;
        let part = delivery["canonical_part_bytes"]
            .as_str()
            .ok_or("original checkpoint bytes absent")?;
        let wire = delivery["canonical_native_wire_bytes"]
            .as_str()
            .ok_or("original complete native checkpoint wire absent")?;
        let decoded: Value = serde_json::from_str(part).map_err(|e| e.to_string())?;
        if delivery["schema"] != "oi.expression-native-checkpoint-delivery/v1"
            || selection(&delivery) != selection(&self.manifest)
            || delivery["checkpoint_index"] != index
            || delivery["checkpoint"] != *original
            || decoded != json!({"kind":"checkpoint","value":original})
            || delivery["reading"]["ref"] != sha(part.as_bytes())
            || delivery["native_wire_sha256"] != sha(wire.as_bytes())
            || serde_json::from_str::<Value>(wire).map_err(|e| e.to_string())?
                != delivery["native_management_wire"]
        {
            return Err("native checkpoint changed full selected Act/source/bytes".into());
        }
        let native = delivery["native_management_wire"].clone();
        if native["native_pair"]["audio"] != original["audio"]
            || native["native_pair"]["physical"] != original["physical"]
            || count(&native["native_pair"]["audio"]["cursor"])? != sample
        {
            return Err(
                "complete native checkpoint is disconnected from selected body/audio state".into(),
            );
        }
        Ok(Some((original.clone(), native)))
    }
}
#[derive(Debug, Serialize)]
pub struct NativeRenderRestoration {
    pub transport_acknowledgement: Value,
    /// Exact original unread callback applications/input journal remain native
    /// receipts when ordinary management pulse drains them after restore.
    pub restored_receipts: Vec<Value>,
    pub original_capture_receipt: Value,
    pub saved_native_checkpoint: Value,
    pub before_restoration_checkpoint: Value,
    pub after_restoration_checkpoint: Value,
    pub restored_applications: Vec<Value>,
    pub restored_input_history: Vec<Value>,
    pub original_cursor: String,
    pub original_accepted_sequence: String,
}
#[derive(Debug, Serialize)]
pub struct NativeRenderResult<T> {
    pub result: T,
    pub restoration: NativeRenderRestoration,
}

/// Called only by the existing native Act/FieldHost lease, with the privately
/// compiled same-selection plan. It always attempts original state restitution
/// after mutation, including writer/render failures. No auto file publication.
pub(crate) fn with_stopped_render<T>(
    plan: &NativeActRenderPlan,
    owner: &mut PerformanceOwner,
    current: &CoupledBasis,
    receiving: &NativePerformanceReceivingSource,
    session: &mut CoupledFieldSession,
    checkpoints: &mut impl NativeActCheckpoints,
    consumer: impl FnOnce(&mut NativeActRenderer<'_>) -> Result<T, String>,
) -> Result<NativeRenderResult<Result<T, String>>, String> {
    if plan.manifest["performance"]["native_sources"][0]["native_bundle"] != *owner.source_assets()
    {
        return Err("export owner differs from complete selected native source assets".into());
    }
    let checkpoint = plan.checkpoint(checkpoints)?;
    if checkpoint.is_none() {
        let native_cursor = count(
            &owner
                .reading()
                .ok_or("actual native render reading absent")?["samples_elapsed"],
        )?;
        if native_cursor != 0
            || plan.from
                > count(&plan.manifest["performance"]["replay"]["max_reconstruction_samples"])?
        {
            return Err(
                "native export needs an actual retained checkpoint before any owner mutation"
                    .into(),
            );
        }
    }
    let prepared = prepare_occurrences(plan, owner)?;
    let before =
        owner.owner_stopped_exchange(current, receiving, session, "checkpoint", &json!({}))?;
    require_accepted(&before)?;
    let saved = before["payload"]["checkpoint"].clone();
    let original_cursor = count(&saved["native_pair"]["audio"]["cursor"])?;
    let original_checkpoint_ref = format!("native:export/original/{original_cursor}");
    let mut renderer = NativeActRenderer {
        plan,
        owner,
        current,
        receiving,
        session,
        prepared,
        next: 0,
        restored_occurrences: BTreeSet::new(),
        checkpoint_ref: original_checkpoint_ref.clone(),
        cursor: original_cursor,
    };
    let activity = (|| {
        if let Some((binding, wire)) = checkpoint {
            let ack = renderer.restore(&wire, &binding["checkpoint_ref"])?;
            if ack["payload"]["transport_ack"]["target_sample"] != binding["sample"] {
                return Err("native selected checkpoint acknowledgement differs".into());
            }
            renderer.checkpoint_ref = binding["checkpoint_ref"]
                .as_str()
                .ok_or("checkpoint ref absent")?
                .into();
            for event in list(&binding["queued_events"])? {
                renderer
                    .restored_occurrences
                    .insert(count(&event["recorded_sequence"])?);
            }
        } else {
            if original_cursor != 0
                || plan.from
                    > count(&plan.manifest["performance"]["replay"]["max_reconstruction_samples"])?
            {
                return Err("native export needs an actual retained initial checkpoint".into());
            }
            for path in [
                "/operations/entries",
                "/releases/entries",
                "/pending_operations",
                "/pending_releases",
            ] {
                if !list(
                    saved["native_pair"]["audio"]
                        .pointer(path)
                        .ok_or("original native queue absent")?,
                )?
                .is_empty()
                {
                    return Err(
                        "initial native export owner already contains unmapped queued events"
                            .into(),
                    );
                }
            }
        }
        renderer.next = renderer
            .prepared
            .partition_point(|e| e.0.effective < renderer.cursor);
        while renderer.cursor < plan.from {
            let n = (plan.from - renderer.cursor).min(FRAMES) as u32;
            let mut discard = [0.0; FRAMES as usize];
            renderer.render_into(n, &mut discard[..n as usize])?;
        }
        let result = consumer(&mut renderer)?;
        if renderer.cursor != plan.to {
            return Err("native export consumer did not render the full admitted interval".into());
        }
        Ok(result)
    })();
    let before_restore = renderer.exchange("checkpoint", &json!({}));
    let restore = renderer.restore(&saved, &json!(original_checkpoint_ref));
    let restored = match restore {
        Ok(ack) => before_restore.and_then(|prior_restore| {
            renderer.verify_restored(
                &saved,
                &prior_restore["payload"]["checkpoint"],
                &before,
                ack,
            )
        }),
        Err(error) => Err(error),
    };
    match restored {
        Ok(restoration) => Ok(NativeRenderResult {
            result: activity,
            restoration,
        }),
        Err(error) => Err(format!(
            "native export original state restitution failed; no success/publication: {error}; activity={:?}",
            activity.err()
        )),
    }
}
fn require_accepted(reply: &Value) -> Result<(), String> {
    if reply["accepted"] != true
        || reply["recording"]["failure"] != 0
        || count(&reply["recording"]["dropped_applications"])? != 0
    {
        return Err(format!(
            "actual native export operation refused or recording lost: {}",
            reply["reason"]
        ));
    }
    Ok(())
}
fn action_tag(action: &Value) -> Result<(&str, &Value), String> {
    if let Some(s) = action.as_str() {
        return Ok((s, &Value::Null));
    }
    let obj = action.as_object().ok_or("native authored action absent")?;
    let (tag, args) = obj.iter().next().ok_or("native authored action absent")?;
    Ok((tag, args))
}
fn prepare_occurrences(
    plan: &NativeActRenderPlan,
    owner: &PerformanceOwner,
) -> Result<Vec<(Occurrence, Value, Value)>, String> {
    let p = &plan.manifest["performance"];
    let mut inputs = BTreeMap::new();
    let mut targets = BTreeMap::new();
    let mut out = Vec::new();
    for event in &plan.occurrences {
        let (tag, a) = action_tag(&event.action)?;
        let mut op = json!({"identity":owner.binding().determination()["identity"],"kind":0,"sequence":"0","sample":event.effective.to_string(),"touch":"0","value":0.0,"pitch_hz":0.0,"parameter":0,"late_admitted":false,"has_note":false,"has_determination":false});
        let mut input = Value::Null;
        match tag {
            "n" => {
                let touch = count(&a[0])?;
                let member = count(&a[1])?;
                let pitch = &p["pitches"]
                    [a[2].as_u64().ok_or("native source pitch index absent")? as usize];
                let original = plan
                    .original_inputs
                    .get(&touch)
                    .cloned()
                    .unwrap_or_else(|| {
                        (
                            format!("native:authored-score/input/{}", event.sequence),
                            format!("native:authored-score/touch/{touch}"),
                        )
                    });
                let note = owner.owner_export_touch(KeyTouch {
                    key: u8::try_from(pitch["key"].as_u64().ok_or("native key absent")?)
                        .map_err(|e| e.to_string())?,
                    register: i8::try_from(
                        pitch["register"].as_i64().ok_or("native register absent")?,
                    )
                    .map_err(|e| e.to_string())?,
                    member,
                    touch,
                    touch_ref: original.1,
                })?;
                if note["hertz"] != pitch["hertz"]
                    || note["tuning_ref"] != pitch["tuning_ref"]
                    || note["phase_sin"] != a[4]
                    || note["phase_cos"] != a[5]
                {
                    return Err(
                        "native export selected pitch/phase differs from actual source key".into(),
                    );
                }
                op["has_note"] = json!(true);
                op["note"] = note.clone();
                op["value"] = a[3].clone();
                input = json!(original.0);
                inputs.insert(touch, input.clone());
                targets.insert(touch, note);
            }
            "o" | "e" => {
                let touch = if tag == "o" { count(a)? } else { count(&a[0])? };
                op["touch"] = json!(touch.to_string());
                input = inputs
                    .get(&touch)
                    .cloned()
                    .ok_or("original authored input lifetime absent")?;
                if tag == "o" {
                    op["kind"] = json!(1);
                } else {
                    op["kind"] = json!(3);
                    op["value"] = a[1].clone();
                    if targets[&touch]["hertz"] != a[2] {
                        return Err("changed expression pitch needs a separately source-qualified native target".into());
                    }
                }
            }
            "s" => {
                op["kind"] = json!(2);
                op["value"] = json!(if a.as_bool().ok_or("native sustain state absent")? {
                    1.0
                } else {
                    0.0
                });
            }
            "x" => {
                op["kind"] = json!(4);
            }
            "p" | "a" => {
                if !a[2].is_null() {
                    return Err("note scoped parameter lacks the native instrument consumer".into());
                }
                let (target, value) = if tag == "p" {
                    (
                        p["parameters"][a[0].as_u64().ok_or("parameter index absent")? as usize]
                            .clone(),
                        a[1].as_f64().ok_or("parameter scalar absent")?,
                    )
                } else {
                    let r = &p["routes"][a[0].as_u64().ok_or("native route absent")? as usize];
                    let source = a[1].as_f64().ok_or("native automation magnitude absent")?;
                    let amount =
                        source * r["amount"].as_f64().ok_or("native route amount absent")?;
                    let baseline = r["destination"]["baseline"]
                        .as_f64()
                        .ok_or("native parameter baseline absent")?;
                    let value = match r["transfer"].as_str() {
                        Some("replace") => amount,
                        Some("add") => baseline + amount,
                        Some("multiply") => baseline * amount,
                        _ => return Err("unknown authored native route transfer".into()),
                    };
                    (r["destination"].clone(), value)
                };
                if target["native_owner"] != "ql.performance.Engine"
                    || target["action_ref"] != "ql:native-performance/parameter"
                    || target["scope"] != "instrument"
                    || !value.is_finite()
                    || value
                        < target["minimum"]
                            .as_f64()
                            .ok_or("native parameter minimum absent")?
                    || value
                        > target["maximum"]
                            .as_f64()
                            .ok_or("native parameter maximum absent")?
                {
                    return Err("native route lost exact owner/scope/magnitude bound".into());
                }
                let parameter = super::performance::owner_parameter_id(
                    target["target_ref"]
                        .as_str()
                        .ok_or("native parameter target absent")?,
                )?;
                let unit = match parameter {
                    0 => "N",
                    1 | 2 => "s",
                    3 => "Hz",
                    4..=6 => "linear",
                    _ => return Err("actual native parameter unavailable".into()),
                };
                if target["unit"] != unit {
                    return Err("native export parameter lost its actual physical unit".into());
                }
                op["kind"] = json!(5);
                op["parameter"] = json!(parameter);
                op["value"] = json!(value);
            }
            _ => {
                return Err(
                    "authored score operation has no qualified native export consumer".into(),
                );
            }
        }
        out.push((event.clone(), op, input));
    }
    Ok(out)
}
/// Bounded actual resident worker renderer. A C NativeOfflineRender adapter
/// receives these frames/receipts; it does not synthesize local samples.
pub struct NativeActRenderer<'a> {
    plan: &'a NativeActRenderPlan,
    owner: &'a mut PerformanceOwner,
    current: &'a CoupledBasis,
    receiving: &'a NativePerformanceReceivingSource,
    session: &'a mut CoupledFieldSession,
    prepared: Vec<(Occurrence, Value, Value)>,
    next: usize,
    restored_occurrences: BTreeSet<u64>,
    checkpoint_ref: String,
    cursor: u64,
}
impl NativeActRenderer<'_> {
    pub fn scope(&self) -> Value {
        self.plan.scope()
    }
    fn exchange(&mut self, operation: &str, operands: &Value) -> Result<Value, String> {
        let reply = self.owner.owner_stopped_exchange(
            self.current,
            self.receiving,
            self.session,
            operation,
            operands,
        )?;
        require_accepted(&reply)?;
        Ok(reply)
    }
    fn restore(&mut self, checkpoint: &Value, checkpoint_ref: &Value) -> Result<Value, String> {
        let old = self
            .owner
            .reading()
            .ok_or("native source readback absent")?
            .clone();
        let reply=self.exchange("restore",&json!({"checkpoint":checkpoint,"expected_cursor":old["samples_elapsed"],
            "transaction_ref":"native:act-export/state-restitution","checkpoint_ref":checkpoint_ref}))?;
        let ack = reply["payload"]["transport_ack"].clone();
        if ack["previous_epoch"] != old["transport_epoch"]
            || ack["previous_cursor"] != old["samples_elapsed"]
            || ack["previous_sequence"] != old["accepted_sequence"]
            || count(&ack["epoch"])?
                != count(&old["transport_epoch"])?
                    .checked_add(1)
                    .ok_or("native epoch exhausted")?
            || ack["target_sample"] != checkpoint["native_pair"]["audio"]["cursor"]
            || ack["accepted_sequence"] != checkpoint["native_pair"]["audio"]["accepted_sequence"]
            || ack["checkpoint_ref"] != *checkpoint_ref
        {
            return Err("actual native restore acknowledgement disconnected".into());
        }
        self.cursor = count(&ack["target_sample"])?;
        Ok(reply)
    }
    pub fn render_into(&mut self, frames: u32, mono: &mut [f32]) -> Result<Value, String> {
        let end = self
            .cursor
            .checked_add(u64::from(frames))
            .ok_or("native export sample overflow")?;
        if frames == 0
            || u64::from(frames) > FRAMES
            || mono.len() != frames as usize
            || end > self.plan.to
        {
            return Err("native render block exceeds admitted range/buffer".into());
        }
        while self.next < self.prepared.len() && self.prepared[self.next].0.effective < end {
            let (event, mut op, input) = self.prepared[self.next].clone();
            self.next += 1;
            if event.effective < self.cursor {
                return Err("native render skipped authored event".into());
            }
            if self.restored_occurrences.contains(&event.sequence) {
                continue;
            }
            let sequence =
                count(&self.owner.reading().ok_or("native reading absent")?["accepted_sequence"])?
                    .checked_add(1)
                    .ok_or("native admission sequence exhausted")?;
            op["sequence"] = json!(sequence.to_string());
            let reply = self.exchange("score", &json!({"event":op,"input_ref":input}))?;
            let admission = &reply["payload"]["score_admission"];
            if admission["schema"] != "ql.native-score-admission/v1"
                || admission["queued"] != true
                || count(&admission["event"]["sequence"])? != sequence
                || count(&admission["event"]["requested_sample"])? != event.effective
                || count(&admission["event"]["sample"])? != event.effective
                || admission["input_ref"] != input
                || admission["source"]["identity"]
                    != self.owner.binding().determination()["identity"]
            {
                return Err(
                    "actual native score acceptance differs from selected occurrence".into(),
                );
            }
        }
        let reading = self
            .owner
            .reading()
            .ok_or("native render readback absent")?;
        let score = self.plan.score.snapshot()?;
        let scope = json!({"schema":"ql.native-offline-render-scope/v1","session_ref":reading["session_ref"],"scene_ref":self.plan.manifest["scene_ref"],
            "performance_revision":self.plan.manifest["performance_digest"],"performance_digest":self.plan.manifest["performance_digest"],
            "basis_seal":self.plan.manifest["performance"]["bases"][0]["content_digest"],"event_prefix_seal":score["content_digest"],
            "checkpoint_ref":self.checkpoint_ref,"expected_source":self.owner.binding().determination()["identity"],
            "expected_body_revision":reading["scope"]["body_revision"],"expected_cursor":self.cursor.to_string(),"expected_accepted_sequence":reading["accepted_sequence"]});
        let reply = self.exchange("offline-render", &json!({"scope":scope,"frames":frames}))?;
        let chunk = reply["payload"]["chunk"].clone();
        if chunk["schema"] != "ql.native-offline-render-chunk/v1"
            || chunk["result"] != "accepted"
            || chunk["performance_digest"] != self.plan.manifest["performance_digest"]
            || chunk["state_committed"] != true
            || chunk["capture_complete"] != true
            || count(&chunk["start_sample"])? != self.cursor
            || count(&chunk["end_sample"])? != end
            || count(&chunk["committed_cursor"])? != end
            || chunk["capture_drops"] != "0"
            || chunk["callback_failures"] != "0"
            || chunk["channels"] != 1
            || chunk["sample_rate"] != self.plan.manifest["performance"]["sample_rate"]
            || chunk["source"] != self.owner.binding().determination()["identity"]
        {
            return Err("actual native render output lost source/clock/body/capture".into());
        }
        let output = list(&chunk["interleaved_f32"])?;
        if output.len() != mono.len() {
            return Err("native render omitted output frames".into());
        }
        for (out, value) in mono.iter_mut().zip(output) {
            let sample = value.as_f64().ok_or("native f32 sample absent")?;
            let exact = sample as f32;
            if !exact.is_finite() || f64::from(exact).to_bits() != sample.to_bits() {
                return Err("native render f32 payload is not exact finite output".into());
            }
            *out = exact;
        }
        self.cursor = end;
        Ok(chunk)
    }
    fn verify_restored(
        &mut self,
        saved: &Value,
        before: &Value,
        original_capture: &Value,
        reply: Value,
    ) -> Result<NativeRenderRestoration, String> {
        let after = self.exchange("checkpoint", &json!({}))?;
        let checkpoint = after["payload"]["checkpoint"].clone();
        if checkpoint["native_pair"]["physical"] != saved["native_pair"]["physical"] {
            return Err("native q/v/body did not restitute exactly".into());
        }
        let prefix = vec![reply.clone(), reply["catalog_restoration"].clone()];
        let apps: Vec<_> = prefix
            .iter()
            .flat_map(|r| r["applications"].as_array().into_iter().flatten().cloned())
            .collect();
        let journal: Vec<_> = prefix
            .iter()
            .flat_map(|r| r["input_history"].as_array().into_iter().flatten().cloned())
            .collect();
        let mut original = saved.clone();
        let mut actual = checkpoint.clone();
        original
            .as_object_mut()
            .ok_or("original management checkpoint absent")?
            .remove("transport_epoch");
        actual
            .as_object_mut()
            .ok_or("restored management checkpoint absent")?
            .remove("transport_epoch");
        for (path, drained) in [
            ("/native_pair/audio/applications", &apps),
            ("/input_history", &journal),
        ] {
            let entries = format!("{path}/entries");
            let read = format!("{path}/read");
            let mut received = drained.clone();
            received.extend(
                list(
                    actual
                        .pointer(&entries)
                        .ok_or("restored native observer queue absent")?,
                )?
                .iter()
                .cloned(),
            );
            if original.pointer(&entries) != Some(&json!(received)) {
                return Err(
                    "native original unread applications/input history were lost on restitution"
                        .into(),
                );
            }
            let prior = count(
                original
                    .pointer(&read)
                    .ok_or("original observer read absent")?,
            )?;
            let current = count(
                actual
                    .pointer(&read)
                    .ok_or("restored observer read absent")?,
            )?;
            if prior.checked_add(drained.len() as u64) != Some(current) {
                return Err("native restored observer FIFO cursor differs".into());
            }
            *original
                .pointer_mut(&entries)
                .ok_or("original observer entries absent")? = json!([]);
            *actual
                .pointer_mut(&entries)
                .ok_or("restored observer entries absent")? = json!([]);
            *original
                .pointer_mut(&read)
                .ok_or("original observer read absent")? = json!(current.to_string());
        }
        if original != actual {
            return Err("native phases/touches/sustain/tails/source/queues/input custody did not restitute exactly".into());
        }
        let mut receipts = prefix;
        receipts.push(after);
        Ok(NativeRenderRestoration {
            transport_acknowledgement: reply["payload"]["transport_ack"].clone(),
            original_capture_receipt: original_capture.clone(),
            saved_native_checkpoint: saved.clone(),
            before_restoration_checkpoint: before.clone(),
            after_restoration_checkpoint: checkpoint.clone(),
            restored_applications: apps,
            restored_input_history: journal,
            restored_receipts: receipts,
            original_cursor: count(&saved["native_pair"]["audio"]["cursor"])?.to_string(),
            original_accepted_sequence: count(&saved["native_pair"]["audio"]["accepted_sequence"])?
                .to_string(),
        })
    }
}
