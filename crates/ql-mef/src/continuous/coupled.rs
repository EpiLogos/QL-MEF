//! Whole-event composition over the accepted M1, M2 and M3 producers.
//!
//! This owner binds their actual values before native material continuation. It
//! does not advance another clock, infer a Nara identity, sample a replacement
//! mesh or turn a source-reading quaternion into a physical pose declaration.
use std::collections::BTreeSet;
use std::path::Path;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::{FieldInput, FieldSession, LiftInput};
use crate::m1_engine::{EngineConfig, M1Engine};
use crate::m2::Reading72;
use crate::m2_engine::{M2Request, VimarshaInput};
use crate::m3_state::{M3Command, M3Request, M3State};
use crate::{ContextFrameId, LensId, ModeKind, SublensRef};

pub const REQUEST: &str = "ql.coupled-event-request/v1";
pub const REQUEST_V2: &str = "ql.coupled-event-request/v2";
pub const REQUEST_V3: &str = "ql.coupled-event-request/v3";
pub const CONTRACT: &str = "ql.coupled-event/v1";
const FACTORY_VAK_CONTRACT: &str = "factory.vak-orchestration/v1";
const FACTORY_QL_PROFILE_CONTRACT: &str = "ql.vak-composition.profile/v1";
const FACTORY_AIKIT_SCOPE_CONTRACT: &str = "aikit.operative-scope/v1";

/// A deliberate instrument mapping, not a claim that symbolic frequencies are
/// measured eigenvalues. Modes not listed here keep their supplied frequency.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FrequencyBinding {
    pub mode_ref: String,
    pub octet_index: u8,
}

/// A source-qualified M2 correspondence pitch, distinct from Vimarsha's octet.
/// The complete condition selects the maqam, role, tuning and tonic. A missing
/// source path or unsupported spelling is a refusal, never a fallback tone.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConditionFrequencyBinding {
    pub mode_ref: String,
    pub pitch_index: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(tag = "selection", rename_all = "kebab-case", deny_unknown_fields)]
pub enum HarmonicSource {
    SelectedSourceRow,
    CanonicalBasis { index: u8 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoupledInput {
    pub schema: String,
    pub m1: EngineConfig,
    pub m2: M2Request,
    pub m3: M3Request,
    pub m3_commands: Vec<M3Command>,
    pub harmonic_source: HarmonicSource,
    pub frequency_bindings: Vec<FrequencyBinding>,
    /// Versions 2 and 3 only. An empty vector is omitted so legacy v1 replay
    /// retains its original serialized input and derivation, not an upgrade.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub condition_frequency_bindings: Vec<ConditionFrequencyBinding>,
    /// Complete host-admitted receipts (for example the original sky snapshot or
    /// the accepted Factory Vāk performance in v3). Retention is not authentication
    /// or permission to disclose their contents.
    pub source_receipts: Vec<Value>,
}

/// Full immutable owner outputs, not a reduced modal packet standing in for M.
#[derive(Debug, Clone, Serialize)]
pub struct CoupledBasis {
    pub input: CoupledInput,
    pub m1: Value,
    pub m2_input: M2Request,
    pub m2: Value,
    pub m3: Value,
    pub m3_receipts: Vec<Value>,
    pub derivation: Value,
}

struct FactoryVakPerformance<'a> {
    receipt: &'a Value,
    source_receipt_index: usize,
    frame_index: usize,
}

fn receipt_ref<'a>(value: &'a Value, field: &str) -> Result<&'a str, String> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty() && value.len() <= 16_384 && !value.contains('\0'))
        .ok_or_else(|| format!("Factory Vāk performance has invalid {field}"))
}

fn receipt_ref_array(value: &Value, field: &str, required: bool) -> Result<(), String> {
    let values = value
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("Factory Vāk performance has invalid {field}"))?;
    if (required && values.is_empty()) || values.len() > 4096 {
        return Err(format!("Factory Vāk performance has invalid {field}"));
    }
    for entry in values {
        let Some(entry) = entry.as_str() else {
            return Err(format!("Factory Vāk performance has invalid {field}"));
        };
        if entry.trim().is_empty() || entry.len() > 16_384 || entry.contains('\0') {
            return Err(format!("Factory Vāk performance has invalid {field}"));
        }
    }
    Ok(())
}

fn factory_vak_performance(receipts: &[Value]) -> Result<Option<FactoryVakPerformance<'_>>, String> {
    let mut found = None;
    for (source_receipt_index, receipt) in receipts.iter().enumerate() {
        if receipt.get("contract").and_then(Value::as_str) != Some(FACTORY_VAK_CONTRACT) {
            continue;
        }
        if found.is_some() {
            return Err("multiple Factory Vāk performance receipts are ambiguous".into());
        }
        for field in [
            "performanceRef",
            "runRef",
            "workflowSourceRef",
            "workflowSourceRevision",
            "workflowSourceDigest",
            "actorRef",
            "subjectRef",
            "wholeRef",
            "qlBindingRef",
            "qlBindingRevision",
            "aiKitResolvePathRef",
            "contextResolutionRef",
        ] {
            receipt_ref(receipt, field)?;
        }
        if receipt.get("runRevision").and_then(Value::as_u64).is_none() {
            return Err("Factory Vāk performance has invalid runRevision".into());
        }
        receipt_ref_array(receipt, "sourceRefs", true)?;
        let frame_index = match receipt_ref(receipt, "frame")? {
            "CF1" => 0,
            "CF2" => 1,
            "CF3" => 2,
            "CF4" => 3,
            "CF5" => 4,
            "CF6" => 5,
            "CF7" => 6,
            _ => return Err("Factory Vāk performance has unknown Context Frame".into()),
        };
        let thread = receipt_ref(receipt, "thread")?;
        let expected_role = match thread {
            "CFP0" => "single-voice",
            "CFP1" => "chord",
            "CFP2" => "melody",
            "CFP3" => "fusion",
            "CFP4" => "drone",
            "CFP5" => "canon",
            _ => return Err("Factory Vāk performance has unknown CFP thread".into()),
        };
        if receipt_ref(receipt, "musicalRole")? != expected_role {
            return Err("Factory Vāk performance thread/musical role disagree".into());
        }
        if !matches!(receipt_ref(receipt, "sequence")?, "CS0" | "CS1" | "CS2" | "CS3" | "CS4" | "CS5") {
            return Err("Factory Vāk performance has unknown Context Sequence".into());
        }
        if !matches!(receipt_ref(receipt, "direction")?, "forward" | "returning") {
            return Err("Factory Vāk performance has unknown Context Sequence direction".into());
        }
        let attempts = receipt
            .get("attempts")
            .and_then(Value::as_array)
            .ok_or("Factory Vāk performance has invalid attempts")?;
        if attempts.is_empty() || attempts.len() > 4096 {
            return Err("Factory Vāk performance has no bounded actual attempt".into());
        }
        let actor = receipt_ref(receipt, "actorRef")?;
        let subject = receipt_ref(receipt, "subjectRef")?;
        let ql_binding = receipt_ref(receipt, "qlBindingRef")?;
        let ql_revision = receipt_ref(receipt, "qlBindingRevision")?;
        for attempt in attempts {
            for field in [
                "unitRef",
                "executionRef",
                "actorRef",
                "wholeRef",
                "subjectRef",
                "qlBindingRef",
                "qlBindingRevision",
                "aiKitResolvePathRef",
                "contextResolutionRef",
                "modelRef",
                "providerRef",
                "status",
            ] {
                receipt_ref(attempt, field)?;
            }
            if attempt.get("attemptIndex").and_then(Value::as_u64).is_none()
                || attempt.get("current").and_then(Value::as_bool).is_none()
            {
                return Err("Factory Vāk performance has invalid attempt identity".into());
            }
            if receipt_ref(attempt, "actorRef")? != actor
                || receipt_ref(attempt, "subjectRef")? != subject
                || receipt_ref(attempt, "qlBindingRef")? != ql_binding
                || receipt_ref(attempt, "qlBindingRevision")? != ql_revision
            {
                return Err("Factory Vāk attempt changed actor/subject/QL identity".into());
            }
            receipt_ref_array(attempt, "sourceRefs", true)?;
            receipt_ref_array(attempt, "statusHistory", true)?;
            receipt_ref_array(attempt, "artifactRefs", false)?;
            receipt_ref_array(attempt, "lateArtifactRefs", false)?;
            receipt_ref_array(attempt, "evidenceRefs", false)?;
        }
        found = Some(FactoryVakPerformance {
            receipt,
            source_receipt_index,
            frame_index,
        });
    }
    Ok(found)
}

impl CoupledInput {
    pub fn compose(&self) -> Result<CoupledBasis, String> {
        if !matches!(self.schema.as_str(), REQUEST | REQUEST_V2 | REQUEST_V3)
            || (self.schema == REQUEST && !self.condition_frequency_bindings.is_empty())
            || self.m3_commands.len() > 64
            || self
                .frequency_bindings
                .len()
                .saturating_add(self.condition_frequency_bindings.len())
                > 4096
            || self.source_receipts.len() > 64
            || serde_json::to_vec(self).map_err(|e| e.to_string())?.len() > super::MAX_MESSAGE
        {
            return Err("unsupported or excessive whole-event input".into());
        }
        let performance = factory_vak_performance(&self.source_receipts)?;
        match (self.schema.as_str(), performance.as_ref()) {
            (REQUEST_V3, None) => {
                return Err("v3 requires one actual Factory Vāk performance receipt".into());
            }
            (REQUEST | REQUEST_V2, Some(_)) => {
                return Err("Factory Vāk performance requires explicit v3 input".into());
            }
            _ => {}
        }
        // Validate the complete seed before deriving a changed reading from it.
        // Original stamps, observations, selections and physical inputs survive.
        self.m2.validate()?;
        let event = &self.m2.stamp.identity;
        if self.m1.event_ref != event.event_ref
            || self.m3.stamp.identity.event_ref != event.event_ref
        {
            return Err("M1/M2/M3 must refer to the same event".into());
        }
        if let Some(performance) = performance.as_ref()
            && receipt_ref(performance.receipt, "subjectRef")? != self.m3.subject_ref
        {
            return Err("Factory Vāk performance and M3 must retain the same subject".into());
        }
        let m1 = M1Engine::new(self.m1.clone())?;
        let mut m3 = M3State::new(self.m3.clone())?;
        let mut receipts = Vec::new();
        for command in &self.m3_commands {
            let receipt = m3.apply(command.clone())?;
            // Native provisional gaps remain qualified, unchanged receipts.
            // They are not guessed targets and do not cancel the valid basis.
            receipts.push(serde_json::to_value(receipt).map_err(|e| e.to_string())?);
        }
        let ratio = match self.harmonic_source {
            HarmonicSource::SelectedSourceRow => crate::m1::source_ratio(
                self.m1.family,
                self.m1.row12,
            )?
            .ok_or(
                "selected M1 row has no admitted harmonic ratio; choose an explicit native basis",
            )?,
            HarmonicSource::CanonicalBasis { index } => crate::m1::ratio_basis()?
                .get(usize::from(index))
                .cloned()
                .ok_or("harmonic basis index outside the native eight")?,
        };
        let ratio16 = [
            u16::try_from(ratio.ratio[0]).map_err(|_| "harmonic numerator exceeds M2 input")?,
            u16::try_from(ratio.ratio[1]).map_err(|_| "harmonic denominator exceeds M2 input")?,
        ];
        let clock = m1.pole_identity()?;
        let lens = LensId::ALL[usize::from(self.m1.lens12)];
        let sublens = SublensRef::canonical(lens, clock.tick12() % 6).map_err(|e| e.to_string())?;
        let reading = Reading72::from_sublens(sublens);
        let m1_cf = ContextFrameId::ALL[usize::from(self.m1.context_frame - 1)];
        let cf = ContextFrameId::ALL[
            performance
                .as_ref()
                .map_or(usize::from(self.m1.context_frame - 1), |value| value.frame_index),
        ];
        let mode = ModeKind::ALL
            .into_iter()
            .find(|m| m.context_frame() == cf)
            .ok_or("missing native Context Frame to musical-mode relation")?;
        let m1_frame = m1.snapshot()?;
        let m3_frame = m3.snapshot();
        let mut request = self.m2.clone();
        // This is a new derived M2 reading at its own composition generation,
        // not a relabelling of M1/M3/Factory source revisions as fresh observations.
        request.tick12 = clock.tick12();
        request.degree720 = clock.degree720();
        if !request.mef_conditions.contains(&reading.index()) {
            request.mef_conditions.push(reading.index());
        }
        for context_frame in [m1_cf, cf] {
            if !request.context_frames.iter().any(|v| v == context_frame.code()) {
                request.context_frames.push(context_frame.code().into());
            }
        }
        if let Some(condition) = request.condition.as_mut() {
            condition.active_mef_condition = reading.index();
        }
        let mut stamp = request.stamp.clone();
        stamp.contract_ref = CONTRACT.into();
        stamp.source_ref = if performance.is_some() {
            format!("{}:M1/M3/Factory-Vak-derived-reading", event.event_ref)
        } else {
            format!("{}:M1/M3-derived-reading", event.event_ref)
        };
        request.vimarsha = Some(VimarshaInput {
            stamp,
            lens: reading.axes()[0],
            musical_mode: mode.index() as u8,
            harmonic_ratio: ratio16,
            pose_ordinal: u16::try_from(m3.fold().rotational_pose().ordinal())
                .map_err(|_| "native pose ordinal outside M2 input")?,
            pose_source_ref: m3_frame["form"]["codon"]["ref"]
                .as_str()
                .ok_or("native M3 lacks its source codon")?
                .into(),
        });
        let initial = request.execute()?;
        let audio = initial
            .vimarsha
            .as_ref()
            .ok_or("missing joined Vimarsha reading")?
            .reading
            .audio_octet_hz;
        let mut seen = BTreeSet::new();
        for binding in &self.frequency_bindings {
            let frequency = *audio
                .get(usize::from(binding.octet_index))
                .ok_or("octet index outside the native eight")?;
            if !seen.insert(&binding.mode_ref) {
                return Err("duplicate material-mode frequency binding".into());
            }
            let mode = request
                .resonator
                .as_mut()
                .ok_or("frequency binding has no supplied resonator")?
                .modes
                .iter_mut()
                .find(|m| m.mode_ref == binding.mode_ref)
                .ok_or("frequency binding does not name an existing material mode")?;
            mode.frequency_hz = f64::from(frequency);
        }
        for binding in &self.condition_frequency_bindings {
            let condition = initial
                .condition
                .as_ref()
                .ok_or("condition frequency binding requires an explicit M2 condition")?;
            condition
                .source_path
                .as_ref()
                .ok_or("condition frequency binding has no admitted correspondence source path")?;
            let frequency = *condition
                .musical
                .pitches_hz
                .get(usize::from(binding.pitch_index))
                .ok_or("condition pitch unavailable: unsupported tuning or index outside the native eight")?;
            if !seen.insert(&binding.mode_ref) {
                return Err(
                    "duplicate material-mode frequency binding across musical buses".into(),
                );
            }
            let mode = request
                .resonator
                .as_mut()
                .ok_or("condition frequency binding has no supplied resonator")?
                .modes
                .iter_mut()
                .find(|m| m.mode_ref == binding.mode_ref)
                .ok_or("condition frequency binding does not name an existing material mode")?;
            mode.frequency_hz = frequency;
        }
        let frame = request.execute()?;
        let mut derivation = json!({
            "harmonic_ratio":ratio, "mef_sublens":sublens.to_string(),
            "mef_table_index":reading.index(), "context_frame":cf.code(),
            "musical_mode":format!("{mode:?}"),
            "frequency_bindings":self.frequency_bindings,
            "source_generations":"M1 revision, M3 operation generation, Factory Run/attempt identity and M2 composition generation remain distinct",
            "material_standing":"explicit native musical projection onto supplied modes, not measured eigenvalue evidence",
            "source_receipts_standing":"host-supplied retained receipts, not authentication by this composer"
        });
        if matches!(self.schema.as_str(), REQUEST_V2 | REQUEST_V3) {
            derivation["condition_frequency_bindings"] = json!(self.condition_frequency_bindings);
            derivation["musical_sources"] = json!({
                "vimarsha":"m2.vimarsha.reading.audio_octet_hz",
                "condition":"m2.condition.musical.pitches_hz",
                "condition_provenance":"m2.condition.source_path, source_revision, correspondence_ref and musical.tuning",
                "policy":"disjoint explicit mode bindings; unbound modes retain supplied frequencies; no missing-path or tuning fallback"
            });
        }
        if let Some(performance) = performance {
            derivation["m1_context_frame"] = json!(m1_cf.code());
            derivation["factory_vak_performance"] = json!({
                "contract":FACTORY_VAK_CONTRACT,
                "source_receipt_index":performance.source_receipt_index,
                "performance_ref":receipt_ref(performance.receipt, "performanceRef")?,
                "run_ref":receipt_ref(performance.receipt, "runRef")?,
                "run_revision":performance.receipt["runRevision"],
                "actor_ref":receipt_ref(performance.receipt, "actorRef")?,
                "subject_ref":receipt_ref(performance.receipt, "subjectRef")?,
                "whole_ref":receipt_ref(performance.receipt, "wholeRef")?,
                "ql_binding_ref":receipt_ref(performance.receipt, "qlBindingRef")?,
                "ql_binding_revision":receipt_ref(performance.receipt, "qlBindingRevision")?,
                "ai_kit_resolve_path_ref":receipt_ref(performance.receipt, "aiKitResolvePathRef")?,
                "context_resolution_ref":receipt_ref(performance.receipt, "contextResolutionRef")?,
                "frame":receipt_ref(performance.receipt, "frame")?,
                "thread":receipt_ref(performance.receipt, "thread")?,
                "sequence":receipt_ref(performance.receipt, "sequence")?,
                "direction":receipt_ref(performance.receipt, "direction")?,
                "musical_role":receipt_ref(performance.receipt, "musicalRole")?,
                "attempts":performance.receipt["attempts"].as_array().map_or(0, Vec::len),
                "standing":"actual Factory performance selects the active QL Context Frame/mode relation; Factory is execution provenance, not a pitch source"
            });
        }
        Ok(CoupledBasis {
            input: self.clone(),
            m1: m1_frame,
            m2_input: request,
            m2: serde_json::to_value(frame).map_err(|e| e.to_string())?,
            m3: m3_frame,
            m3_receipts: receipts,
            derivation,
        })
    }
}

/// One full basis and one existing native continuation. Surface focus/read is
/// deliberately absent from mutation operations. No second numerical owner.
pub struct CoupledFieldSession {
    field: FieldSession,
    original: CoupledBasis,
    current: CoupledBasis,
    original_field: FieldInput,
}
impl CoupledFieldSession {
    pub fn open(
        executable: &Path,
        input: CoupledInput,
        field: FieldInput,
        timeout: Duration,
    ) -> Result<Self, String> {
        let basis = input.compose()?;
        if basis.m3["subject_ref"] != field.subject_ref {
            return Err("field and M3 must retain the same subject".into());
        }
        let original_field = field.clone();
        let field = FieldSession::open(executable, basis.m2_input.clone(), field, timeout)?;
        Ok(Self {
            field,
            original: basis.clone(),
            current: basis,
            original_field,
        })
    }
    pub fn original_basis(&self) -> &CoupledBasis {
        &self.original
    }
    pub fn original_field(&self) -> &FieldInput {
        &self.original_field
    }
    pub fn current_basis(&self) -> &CoupledBasis {
        &self.current
    }
    pub fn available(&self) -> bool {
        self.field.available()
    }
    pub fn snapshot(&self) -> Value {
        json!({"schema":CONTRACT, "available":self.available(),
            "basis":self.current, "field":self.field.last_receipt(),
            "standing":"native modal continuation from complete retained M1/M2/M3 bases; original discrete clocks are not relabelled as continuous samples"})
    }
    /// Compact data-plane access. Full bases remain in this same owner and are
    /// available through snapshot/inspection, not copied into every PCM block.
    pub fn last_field(&self) -> &Value {
        self.field.last_receipt()
    }
    pub fn read_field(&mut self) -> Result<Value, String> {
        self.field.read()
    }
    pub fn advance_field(&mut self, frames: u32, muted: bool) -> Result<Value, String> {
        self.field.advance(frames, muted)
    }
    pub fn set_axis_field(&mut self, axis: u8, phase: LiftInput) -> Result<Value, String> {
        self.field.set_axis(axis, phase)
    }
    pub fn read(&mut self) -> Result<Value, String> {
        self.read_field()?;
        Ok(self.snapshot())
    }
    pub fn advance(&mut self, frames: u32, muted: bool) -> Result<Value, String> {
        self.advance_field(frames, muted)?;
        Ok(self.snapshot())
    }
    pub fn set_axis(&mut self, axis: u8, phase: LiftInput) -> Result<Value, String> {
        self.set_axis_field(axis, phase)?;
        Ok(self.snapshot())
    }
    /// Full derivation completes before any native mutation. The old whole basis
    /// remains the last acknowledged one if the native operation is refused or
    /// its transport has unknown standing. No partial M1/M3 publication occurs.
    pub fn replace_field(&mut self, input: CoupledInput) -> Result<Value, String> {
        let next = input.compose()?;
        if next.m3["subject_ref"] != self.current.m3["subject_ref"] {
            return Err("cannot change subject during continuation".into());
        }
        self.field.replace_modes(next.m2_input.clone(), false)?;
        self.current = next;
        Ok(self.last_field().clone())
    }
    pub fn replace(&mut self, input: CoupledInput) -> Result<Value, String> {
        self.replace_field(input)?;
        Ok(self.snapshot())
    }
}