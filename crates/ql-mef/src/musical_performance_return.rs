//! Retain the actual admitted performance's native score and original occasion.
//! A/B/K/P produce and admit the performance; M3State owns transcription/form
//! commands; NaraOccasion owns the situated episode. This binding is a return
//! to the existing Expression owner, never a second score store or classifier.
use crate::m1_engine::M1Engine;
use crate::m2_relation_plan::source_field;
use crate::m3_state::{M3Command, M3Receipt, M3Request, M3State};
use crate::nara::{EventBasisRefs, replay::NaraOccasion};
use crate::performance_audio::PreparedPerformanceBinding;
use crate::physical_body::prepare_source_body;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

pub const CONTRACT: &str = "ql.musical-performance-return/v1";
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReturnReference {
    pub reference: String,
    pub revision: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReturnContext {
    /// Existing native receiving/protected context. Labels grant no authority.
    pub context: ReturnReference,
    pub receiver: ReturnReference,
    pub source_occasion: Option<ReturnReference>,
    pub protected_state: Option<ReturnReference>,
    pub consent: Option<ReturnReference>,
    pub kind: String,
    pub private: bool,
    pub required_assets: Vec<ReturnReference>,
}
fn text(v: &str) -> Result<(), String> {
    if v.trim().is_empty() || v.len() > 4096 || v.chars().any(char::is_control) {
        return Err("bounded native return reference required".into());
    }
    Ok(())
}
fn hash(v: &Value) -> Result<String, String> {
    Ok(format!(
        "sha256:{:x}",
        Sha256::digest(serde_json::to_vec(v).map_err(|e| e.to_string())?)
    ))
}
impl ReturnReference {
    fn validate(&self) -> Result<(), String> {
        text(&self.reference)?;
        text(&self.revision)
    }
    fn reading(&self) -> Value {
        json!({"ref":self.reference,"revision":self.revision,"availability":"available"})
    }
}
impl ReturnContext {
    fn validate(&self, occasion: Option<&NaraOccasion>) -> Result<(), String> {
        for r in [&self.context, &self.receiver] {
            r.validate()?;
        }
        for r in self
            .protected_state
            .iter()
            .chain(self.source_occasion.iter())
            .chain(self.consent.iter())
            .chain(&self.required_assets)
        {
            r.validate()?;
        }
        if self.required_assets.len() > 256 {
            return Err("native return asset bound exceeded".into());
        }
        if !["world", "personal", "shared"].contains(&self.kind.as_str()) {
            return Err("unknown native return context kind".into());
        }
        if self.kind == "personal"
            && (!self.private || self.protected_state.is_none() || occasion.is_none())
        {
            return Err("personal return needs its protected native occasion".into());
        }
        if self.kind == "shared" && (self.consent.is_none() || occasion.is_none()) {
            return Err("shared return needs its native consent and occasion".into());
        }
        if let Some(o) = occasion {
            if let Some(p) = &self.protected_state {
                if p.reference != o.protected_state_ref.ref_id
                    || p.revision != o.protected_state_ref.revision
                {
                    return Err("return protected state detached from exact native occasion".into());
                }
            }
            if self.source_occasion.as_ref().map(|r| r.reference.as_str())
                != Some(o.occasion_ref.as_str())
            {
                return Err("return context detached from original native occasion".into());
            }
        }
        Ok(())
    }
}
/// Private fields are produced only after recomputing the existing owners.
/// Serialization is retained evidence, never native mutation authorization.
#[derive(Debug, Clone, Serialize)]
pub struct MusicalPerformanceReturn {
    schema: &'static str,
    m1: Value,
    audio_determination: Value,
    m2_plan: Value,
    m3_score: Value,
    m3_request: M3Request,
    m3_commands: Vec<M3Command>,
    m3_receipts: Vec<Value>,
    m4_episode: Option<NaraOccasion>,
    prepared_body: Value,
    notes: Vec<Value>,
    tuning: Value,
    policy_receipts: Value,
    context: ReturnContext,
    seed: String,
    content_digest: String,
}
impl MusicalPerformanceReturn {
    /// Exact native transcription/form replay. No sound, tool/provider call,
    /// semantic projection or mutation of an original episode is performed.
    pub fn replay_score(&self) -> Result<M3State, String> {
        let mut state = M3State::new(self.m3_request.clone())?;
        let mut receipts = Vec::with_capacity(self.m3_commands.len());
        for command in &self.m3_commands {
            receipts.push(
                serde_json::to_value(state.apply(command.clone())?).map_err(|e| e.to_string())?,
            );
        }
        if state.snapshot() != self.m3_score || receipts != self.m3_receipts {
            return Err("retained native score/receipts differ from actual command replay".into());
        }
        Ok(state)
    }
    /// A new native inscription returns its own receipt; it never rewrites
    /// the original source-qualified score or occasion.
    pub fn reinscribe(&self, command: M3Command) -> Result<M3Receipt, String> {
        self.replay_score()?.apply(command)
    }
    pub fn original_occasion(&self) -> Option<&NaraOccasion> {
        self.m4_episode.as_ref()
    }
    pub fn snapshot(&self) -> Result<Value, String> {
        serde_json::to_value(self).map_err(|e| e.to_string())
    }
    /// Whole opaque source outputs for the native Scene's versioned addition.
    /// The receiving Expression owner parses and seals PerformanceBasis, then
    /// stores it through its ordinary Document/Act/Central file operations.
    pub fn expression_basis(&self) -> Result<Value, String> {
        self.replay_score()?;
        let i = &self.audio_determination["identity"];
        let mut sources = Vec::new();
        for (reference, revision) in [
            (
                self.m3_request.stamp.source_ref.as_str(),
                self.m3_score["source_revision"]
                    .as_str()
                    .ok_or("M3 source revision absent")?,
            ),
            (
                self.m3_request.stamp.contract_ref.as_str(),
                self.m3_score["domain_revision"]
                    .as_str()
                    .ok_or("M3 domain revision absent")?,
            ),
            (
                "ql:m1/source-cells",
                self.m1["source"]["revision"]
                    .as_str()
                    .ok_or("M1 source revision absent")?,
            ),
            (
                "ql:m-registry",
                self.audio_determination["registry_revision"]
                    .as_str()
                    .ok_or("registry revision absent")?,
            ),
            (
                self.m2_plan["sourceReceipts"]["sourcePath"]
                    .as_str()
                    .ok_or("M2 source path absent")?,
                self.m2_plan["sourceReceipts"]["sourceRevision"]
                    .as_str()
                    .ok_or("M2 source revision absent")?,
            ),
        ] {
            text(reference)?;
            text(revision)?;
            let r = json!({"ref":reference,"revision":revision,"availability":"available"});
            if !sources.contains(&r) {
                sources.push(r);
            }
        }
        let identity = json!({"instance_ref":i["instance"],"event_ref":i["event"],"subject_ref":i["subject"],
            "m1_revision":i["m1_revision"],"m2_generation":i["m2_generation"],
            "m3_generation":self.m3_score["identity"]["profile_generation"].as_u64().ok_or("M3 generation absent")?.to_string(),
            "occurrence_unix_ms":self.m3_score["occurrence_unix_ms"].as_u64().ok_or("M3 occurrence absent")?.to_string(),
            "receipt_unix_ms":self.m3_score["receipt_unix_ms"].as_u64().ok_or("M3 receipt absent")?.to_string()});
        let context = json!({"kind":self.context.kind,"occasion_mode":"live", "context":self.context.context.reading(),
            "receiver":self.context.receiver.reading(),"source_occasion":self.context.source_occasion.as_ref().map(ReturnReference::reading),
            "protected_state":self.context.protected_state.as_ref().map(ReturnReference::reading),
            "consent":self.context.consent.as_ref().map(ReturnReference::reading),"private":self.context.private});
        Ok(
            json!({"schema":"oi.expression-performance-basis/v1","identity":identity,"sources":sources,
            "m1_coordinate":self.audio_determination["m1_coordinate"],"m1_prime":self.audio_determination["m1_face"]==1,
            "m1":self.m1,"m2_plan":self.m2_plan,"audio_determination":self.audio_determination,
            "m3_score":self.m3_score,"m3_replay":{"request":self.m3_request,"commands":self.m3_commands,"receipts":self.m3_receipts},
            "m4_episode":self.m4_episode,"prepared_body":self.prepared_body,"tuning":self.tuning,
            "force_state":{"unit":"N","preparation_ref":self.prepared_body["request"]["preparation_ref"],
                "state_ref":self.prepared_body["request"]["state_ref"],"exciter":self.prepared_body["request"]["exciter"],
                "source_ref":self.prepared_body["request"]["geometry"]["provenance"]["source_ref"],
                "revision":self.prepared_body["request"]["geometry"]["provenance"]["revision"]},
            "form_state":self.prepared_body["form"],"context":context,"seed":self.seed,
            "required_assets":self.context.required_assets.iter().map(ReturnReference::reading).collect::<Vec<_>>(),"content_digest":""}),
        )
    }
    pub fn expression_pitches(&self, basis: u16) -> Result<Value, String> {
        let pitches = self.notes.iter().map(|n| json!({"basis":basis,"source_coordinate":n["source_coordinate"],
            "source_prime":n["source_face"]==1,"key":n["key"],"pitch_class":n["pitch_class"],"register":n["register_octave"],
            "fundamental_hz":n["fundamental_hz"],"hertz":n["hertz"],"tuning_ref":n["tuning_ref"],
            "exact_ratio":if n["exact_ratio"]==true {json!({"numerator":n["ratio_numerator"],"denominator":n["ratio_denominator"]})}else{Value::Null}})).collect::<Vec<_>>();
        Ok(Value::Array(pitches))
    }
}
/// Receive the current preparation owner, then independently replay M1/M3/P
/// source derivation. An event label or a stale pre-command score cannot pass.
pub fn bind_performance_return(
    prepared: &PreparedPerformanceBinding,
    occasion: Option<NaraOccasion>,
    context: ReturnContext,
    seed: u64,
) -> Result<MusicalPerformanceReturn, String> {
    context.validate(occasion.as_ref())?;
    let basis = prepared.native_basis();
    prepared.validate_native_consumers(basis, prepared.physical_body())?;
    let m1 = M1Engine::new(basis.input.m1.clone())?.snapshot()?;
    if m1 != basis.m1 {
        return Err("native return M1 source disconnected".into());
    }
    prepared
        .relation_plan()
        .source_receipts
        .validate_against(source_field())?;
    let actual_m2 = basis.m2_input.execute()?;
    if serde_json::to_value(actual_m2).map_err(|e| e.to_string())? != basis.m2 {
        return Err("native return M2 plan/consumer disconnected".into());
    }
    let mut state = M3State::new(basis.input.m3.clone())?;
    let mut receipts = Vec::new();
    for command in &basis.input.m3_commands {
        receipts
            .push(serde_json::to_value(state.apply(command.clone())?).map_err(|e| e.to_string())?);
    }
    if state.snapshot() != basis.m3 || receipts != basis.m3_receipts {
        return Err("native return post-command M3 disconnected".into());
    }
    let body = prepare_source_body(
        &state,
        prepared.physical_body().source_coordinate().clone(),
        prepared.physical_body().request().clone(),
    )?;
    if serde_json::to_value(&body).map_err(|e| e.to_string())?
        != serde_json::to_value(prepared.physical_body()).map_err(|e| e.to_string())?
    {
        return Err("native return current score/body/force/form disconnected".into());
    }
    if let Some(o) = &occasion {
        o.validate()?;
        if o.event != EventBasisRefs::from_basis(basis)? {
            return Err("native return original occasion/event basis disconnected".into());
        }
    }
    let packet = serde_json::to_value(prepared).map_err(|e| e.to_string())?;
    let mut tuning = packet["policy_receipts"]["tuning"].clone();
    tuning
        .as_object_mut()
        .ok_or("native tuning policy missing")?
        .insert("available".into(), Value::Bool(true));
    let mut out = MusicalPerformanceReturn {
        schema: CONTRACT,
        m1,
        audio_determination: prepared.determination().clone(),
        m2_plan: serde_json::to_value(prepared.relation_plan()).map_err(|e| e.to_string())?,
        m3_score: state.snapshot(),
        m3_request: basis.input.m3.clone(),
        m3_commands: basis.input.m3_commands.clone(),
        m3_receipts: receipts,
        m4_episode: occasion,
        prepared_body: serde_json::to_value(body).map_err(|e| e.to_string())?,
        notes: prepared.notes().to_vec(),
        tuning,
        policy_receipts: packet["policy_receipts"].clone(),
        context,
        seed: seed.to_string(),
        content_digest: String::new(),
    };
    out.content_digest = hash(&out.snapshot()?)?;
    out.replay_score()?;
    Ok(out)
}
