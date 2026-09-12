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
pub const CONTRACT: &str = "ql.coupled-event/v1";

/// A deliberate instrument mapping, not a claim that symbolic frequencies are
/// measured eigenvalues. Modes not listed here keep their supplied frequency.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FrequencyBinding {
    pub mode_ref: String,
    pub octet_index: u8,
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
    /// Complete host-admitted receipts (for example the original sky snapshot).
    /// Retention is not authentication or permission to disclose their contents.
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
impl CoupledInput {
    pub fn compose(&self) -> Result<CoupledBasis, String> {
        if self.schema != REQUEST
            || self.m3_commands.len() > 64
            || self.frequency_bindings.len() > 4096
            || self.source_receipts.len() > 64
            || serde_json::to_vec(self).map_err(|e| e.to_string())?.len() > super::MAX_MESSAGE
        {
            return Err("unsupported or excessive whole-event input".into());
        }
        // Validate the complete seed before deriving a changed reading from it.
        // Original stamps, observations, selections and physical inputs survive.
        self.m2.validate()?;
        let event = &self.m2.stamp.identity;
        if self.m1.event_ref != event.event_ref || self.m3.stamp.identity.event_ref != event.event_ref {
            return Err("M1/M2/M3 must refer to the same event".into());
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
            HarmonicSource::SelectedSourceRow => crate::m1::source_ratio(self.m1.family, self.m1.row12)?
                .ok_or("selected M1 row has no admitted harmonic ratio; choose an explicit native basis")?,
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
        let cf = ContextFrameId::ALL[usize::from(self.m1.context_frame - 1)];
        let mode = ModeKind::ALL.into_iter().find(|m| m.context_frame() == cf)
            .ok_or("missing native Context Frame to musical-mode relation")?;
        let m1_frame = m1.snapshot()?;
        let m3_frame = m3.snapshot();
        let mut request = self.m2.clone();
        // This is a new derived M2 reading at its own composition generation,
        // not a relabelling of M1/M3 source revisions as fresh observations.
        request.tick12 = clock.tick12();
        request.degree720 = clock.degree720();
        if !request.mef_conditions.contains(&reading.index()) {
            request.mef_conditions.push(reading.index());
        }
        if !request.context_frames.iter().any(|v| v == cf.code()) {
            request.context_frames.push(cf.code().into());
        }
        if let Some(condition) = request.condition.as_mut() {
            condition.active_mef_condition = reading.index();
        }
        let mut stamp = request.stamp.clone();
        stamp.contract_ref = CONTRACT.into();
        stamp.source_ref = format!("{}:M1/M3-derived-reading", event.event_ref);
        request.vimarsha = Some(VimarshaInput {
            stamp,
            lens: reading.axes()[0],
            musical_mode: mode.index() as u8,
            harmonic_ratio: ratio16,
            pose_ordinal: u16::try_from(m3.fold().rotational_pose().ordinal())
                .map_err(|_| "native pose ordinal outside M2 input")?,
            pose_source_ref: m3_frame["form"]["codon"]["ref"].as_str()
                .ok_or("native M3 lacks its source codon")?.into(),
        });
        let initial = request.execute()?;
        let audio = initial.vimarsha.as_ref().ok_or("missing joined Vimarsha reading")?
            .reading.audio_octet_hz;
        let mut seen = BTreeSet::new();
        for binding in &self.frequency_bindings {
            let frequency = *audio.get(usize::from(binding.octet_index))
                .ok_or("octet index outside the native eight")?;
            if !seen.insert(&binding.mode_ref) {
                return Err("duplicate material-mode frequency binding".into());
            }
            let mode = request.resonator.as_mut().ok_or("frequency binding has no supplied resonator")?
                .modes.iter_mut().find(|m| m.mode_ref == binding.mode_ref)
                .ok_or("frequency binding does not name an existing material mode")?;
            mode.frequency_hz = f64::from(frequency);
        }
        let frame = request.execute()?;
        Ok(CoupledBasis {
            input: self.clone(),
            m1: m1_frame,
            m2_input: request,
            m2: serde_json::to_value(frame).map_err(|e| e.to_string())?,
            m3: m3_frame,
            m3_receipts: receipts,
            derivation: json!({
                "harmonic_ratio":ratio, "mef_sublens":sublens.to_string(),
                "mef_table_index":reading.index(), "context_frame":cf.code(),
                "musical_mode":format!("{mode:?}"),
                "frequency_bindings":self.frequency_bindings,
                "source_generations":"M1 revision, M3 operation generation and M2 composition generation remain distinct",
                "material_standing":"explicit native musical projection onto supplied modes, not measured eigenvalue evidence",
                "source_receipts_standing":"host-supplied retained receipts, not authentication by this composer"
            }),
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
    pub fn open(executable: &Path, input: CoupledInput, field: FieldInput, timeout: Duration) -> Result<Self, String> {
        let basis = input.compose()?;
        if basis.m3["subject_ref"] != field.subject_ref {
            return Err("field and M3 must retain the same subject".into());
        }
        let original_field = field.clone();
        let field = FieldSession::open(executable, basis.m2_input.clone(), field, timeout)?;
        Ok(Self { field, original: basis.clone(), current: basis, original_field })
    }
    pub fn original_basis(&self) -> &CoupledBasis { &self.original }
    pub fn original_field(&self) -> &FieldInput { &self.original_field }
    pub fn current_basis(&self) -> &CoupledBasis { &self.current }
    pub fn available(&self) -> bool { self.field.available() }
    pub fn snapshot(&self) -> Value {
        json!({"schema":CONTRACT, "available":self.available(),
            "basis":self.current, "field":self.field.last_receipt(),
            "standing":"native modal continuation from complete retained M1/M2/M3 bases; original discrete clocks are not relabelled as continuous samples"})
    }
    pub fn read(&mut self) -> Result<Value, String> { self.field.read()?; Ok(self.snapshot()) }
    pub fn advance(&mut self, frames: u32, muted: bool) -> Result<Value, String> {
        self.field.advance(frames, muted)?;
        Ok(self.snapshot())
    }
    pub fn set_axis(&mut self, axis: u8, phase: LiftInput) -> Result<Value, String> {
        self.field.set_axis(axis, phase)?;
        Ok(self.snapshot())
    }
    /// Full derivation completes before any native mutation. The old whole basis
    /// remains the last acknowledged one if the native operation is refused or
    /// its transport has unknown standing. No partial M1/M3 publication occurs.
    pub fn replace(&mut self, input: CoupledInput) -> Result<Value, String> {
        let next = input.compose()?;
        if next.m3["subject_ref"] != self.current.m3["subject_ref"] {
            return Err("cannot change subject during continuation".into());
        }
        self.field.replace_modes(next.m2_input.clone(), false)?;
        self.current = next;
        Ok(self.snapshot())
    }
}
