//! K9 focused-instrument consumer over the already-owned K8/Nara/Vāk state.
//!
//! This module owns no renderer, graph store, clock, provider, AgentSession or
//! personal producer. It projects the one `PersonalCoupledSession` into stable
//! selection/focus/presentation state for a native host, and routes deliberate
//! clock operations back through that same owner. Bimba and desktop owners pass
//! opaque, source-qualified identities; K9 never re-queries or reconstructs them.

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::continuous::LiftInput;
use crate::continuous::coupled::CoupledBasis;
use crate::continuous::personal::PersonalCoupledSession;
use crate::nara::{EventBasisRefs, PersonalFieldState, SourceRevision, WorldContribution};
use crate::vak_performance::{
    PERFORMANCE_EVENT_CONTRACT, PerformanceObservationMode, VakPerformanceEvent,
};

pub const FOCUSED_INSTRUMENT_CONTRACT: &str = "ql.focused-instrument/v1";
pub const BIMBA_SELECTION_CONTRACT: &str = "ql.focused-instrument-bimba-selection/v1";
pub const VAK_EXPRESSION_BINDING_CONTRACT: &str = "ql.focused-instrument-vak-expression/v1";
pub const OPERATION_OBSERVATION_CONTRACT: &str = "ql.focused-instrument-operation/v1";
pub const NARA_EXPRESSION_SESSION_CONTRACT: &str = "ql.nara-expression-session/v1";
pub const NARA_EXPRESSION_PORTABLE_CUES_CONTRACT: &str = "ql.nara-expression-portable-cues/v1";

fn reference(value: &str, what: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.len() > 16_384 || value.contains('\0') {
        Err(format!("invalid {what}"))
    } else {
        Ok(())
    }
}

fn decimal(value: &Value, what: &str) -> Result<String, String> {
    let value = value
        .as_str()
        .ok_or_else(|| format!("{what} must be an exact decimal string"))?;
    let parsed: u64 = value.parse().map_err(|_| format!("invalid {what}"))?;
    if parsed.to_string() != value {
        return Err(format!("noncanonical {what}"));
    }
    Ok(value.to_owned())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InstrumentFocus {
    M1,
    M2,
    M3,
    M4,
    M5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SelectionTracking {
    Follow,
    Pinned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TemporalPresentation {
    Live,
    Frozen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SelectionStanding {
    Current,
    FieldAdvanced,
    Stale,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VakMedium {
    Literal,
    Glyph,
    Image,
    Musical,
    Enacted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "view", rename_all = "kebab-case", deny_unknown_fields)]
pub enum ClockPresentation {
    Assembled,
    Exploded { pair: Option<u8> },
}

impl ClockPresentation {
    fn validate(&self) -> Result<(), String> {
        if let Self::Exploded { pair: Some(pair) } = self
            && *pair >= 8
        {
            return Err("exploded clock pair must be one of the eight reciprocal pairs".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BimbaSelection {
    pub contract: String,
    pub selection_ref: String,
    pub coordinate_ref: String,
    pub source_ref: String,
    pub source_revision: String,
    pub disclosure_ref: String,
    pub subject_ref: String,
    /// Optional identity shared with K8's material target. Parent/source-only
    /// selections can omit it and remain valid Bimba selections.
    pub field_constituent_ref: Option<String>,
    #[serde(default)]
    pub assertion_refs: Vec<String>,
}

impl BimbaSelection {
    pub fn validate(&self) -> Result<(), String> {
        if self.contract != BIMBA_SELECTION_CONTRACT {
            return Err("unsupported focused-instrument Bimba selection".into());
        }
        for (value, what) in [
            (&self.selection_ref, "selection reference"),
            (&self.coordinate_ref, "Bimba coordinate"),
            (&self.source_ref, "Bimba source"),
            (&self.source_revision, "Bimba source revision"),
            (&self.disclosure_ref, "Bimba disclosure"),
            (&self.subject_ref, "selection subject"),
        ] {
            reference(value, what)?;
        }
        if let Some(value) = &self.field_constituent_ref {
            reference(value, "field constituent")?;
        }
        if self.assertion_refs.len() > 4096 {
            return Err("too many selection assertion references".into());
        }
        for value in &self.assertion_refs {
            reference(value, "selection assertion reference")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VakExpressionBinding {
    pub contract: String,
    pub source_entry_ref: String,
    pub source_revision: String,
    pub literal: Option<String>,
    pub glyph_ref: Option<String>,
    pub image_ref: Option<String>,
    pub musical_performance_ref: Option<String>,
    /// Source-qualified enacted form, e.g. the owner's 0→1 disclosure of `/`.
    /// K9 retains the identity; it does not prescribe renderer mechanics.
    pub enactment_ref: Option<String>,
}

impl VakExpressionBinding {
    pub fn media(&self) -> Vec<VakMedium> {
        let mut result = Vec::new();
        if self.literal.is_some() {
            result.push(VakMedium::Literal);
        }
        if self.glyph_ref.is_some() {
            result.push(VakMedium::Glyph);
        }
        if self.image_ref.is_some() {
            result.push(VakMedium::Image);
        }
        if self.musical_performance_ref.is_some() {
            result.push(VakMedium::Musical);
        }
        if self.enactment_ref.is_some() {
            result.push(VakMedium::Enacted);
        }
        result
    }

    fn validate(&self, performance: Option<&VakPerformanceSummary>) -> Result<(), String> {
        if self.contract != VAK_EXPRESSION_BINDING_CONTRACT {
            return Err("unsupported focused-instrument Vāk expression binding".into());
        }
        reference(&self.source_entry_ref, "Vāk source entry")?;
        reference(&self.source_revision, "Vāk source revision")?;
        if self.media().is_empty() {
            return Err("Vāk expression needs at least one source-qualified medium".into());
        }
        for (value, what) in [
            (self.glyph_ref.as_deref(), "glyph reference"),
            (self.image_ref.as_deref(), "image reference"),
            (self.enactment_ref.as_deref(), "enactment reference"),
        ] {
            if let Some(value) = value {
                reference(value, what)?;
            }
        }
        if let Some(literal) = &self.literal {
            reference(literal, "literal Vāk expression")?;
        }
        if let Some(performance_ref) = &self.musical_performance_ref {
            reference(performance_ref, "musical performance reference")?;
            let performance = performance
                .ok_or("musical Vāk cannot be presented without an actual QL performance event")?;
            if &performance.performance_ref != performance_ref {
                return Err("musical Vāk binding names another performed occasion".into());
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldCursor {
    pub event_ref: String,
    pub subject_ref: String,
    pub profile_generation: u64,
    pub field_generation: String,
    pub samples_elapsed: String,
}

impl FieldCursor {
    fn from_parts(event: &EventBasisRefs, field: &Value) -> Result<Self, String> {
        if field["event_ref"].as_str() != Some(event.event_ref.as_str())
            || field["subject_ref"].as_str() != Some(event.subject_ref.as_str())
        {
            return Err("continuous field receipt belongs to another event or subject".into());
        }
        let generation = decimal(&field["generation"], "field generation")?;
        let samples_elapsed = decimal(&field["samples_elapsed"], "sample cursor")?;
        let profile_generation = field["m2_identity"]["profile_generation"]
            .as_u64()
            .ok_or("field receipt lacks its M2 profile generation")?;
        if profile_generation != event.profile_generation {
            return Err(
                "continuous field receipt disagrees with the current world generation".into(),
            );
        }
        Ok(Self {
            event_ref: event.event_ref.clone(),
            subject_ref: event.subject_ref.clone(),
            profile_generation,
            field_generation: generation,
            samples_elapsed,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldTarget {
    pub identity: u64,
    pub constituent: String,
    pub position: [f64; 3],
}

fn field_target(field: &Value, constituent: &str) -> Result<Option<FieldTarget>, String> {
    let targets = field["targets"]
        .as_array()
        .ok_or("continuous field receipt lacks material targets")?;
    let mut found = None;
    for target in targets {
        if target["constituent"].as_str() != Some(constituent) {
            continue;
        }
        if found.is_some() {
            return Err("continuous field contains duplicate constituent targets".into());
        }
        let position = target["position"]
            .as_array()
            .filter(|values| values.len() == 3)
            .ok_or("selected target has invalid position")?;
        let position = [
            position[0].as_f64().ok_or("invalid selected target x")?,
            position[1].as_f64().ok_or("invalid selected target y")?,
            position[2].as_f64().ok_or("invalid selected target z")?,
        ];
        if !position.into_iter().all(f64::is_finite) {
            return Err("selected target has non-finite position".into());
        }
        found = Some(FieldTarget {
            identity: target["identity"]
                .as_u64()
                .ok_or("selected target lacks stable sample identity")?,
            constituent: constituent.to_owned(),
            position,
        });
    }
    Ok(found)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PerformanceMode {
    Live,
    Replay,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VakPerformanceSummary {
    pub performance_ref: String,
    pub mode: PerformanceMode,
    pub replay_of: Option<String>,
    pub actor_ref: String,
    pub subject_ref: String,
    pub context_frame: String,
    pub musical_role: String,
    pub musical_mode: String,
    pub lens: String,
    pub settled: bool,
    pub has_failure: bool,
    pub has_interruption: bool,
    pub has_late_return: bool,
    pub source_refs: Vec<String>,
}

impl VakPerformanceSummary {
    fn from_event(event: VakPerformanceEvent) -> Self {
        let mode = match event.observation.mode {
            PerformanceObservationMode::Live => PerformanceMode::Live,
            PerformanceObservationMode::Replay => PerformanceMode::Replay,
        };
        Self {
            performance_ref: event.performance_ref,
            mode,
            replay_of: event.observation.replay_of,
            actor_ref: event.factory.actor_ref,
            subject_ref: event.factory.subject_ref,
            context_frame: event.semantics.context_frame,
            musical_role: event.semantics.musical_role,
            musical_mode: event.semantics.musical_mode,
            lens: event.semantics.lens,
            settled: event.settled,
            has_failure: event.has_failure,
            has_interruption: event.has_interruption,
            has_late_return: event.has_late_return,
            source_refs: event.ql_basis_refs.into_iter().collect(),
        }
    }
}

fn vak_performance(basis: &CoupledBasis) -> Result<Option<VakPerformanceSummary>, String> {
    let mut found = None;
    for receipt in &basis.input.source_receipts {
        if receipt.get("contract").and_then(Value::as_str) != Some(PERFORMANCE_EVENT_CONTRACT) {
            continue;
        }
        if found.is_some() {
            return Err("focused instrument received multiple QL Vāk performance events".into());
        }
        let event: VakPerformanceEvent = serde_json::from_value(receipt.clone())
            .map_err(|error| format!("invalid retained QL Vāk performance event: {error}"))?;
        found = Some(VakPerformanceSummary::from_event(event));
    }
    Ok(found)
}

/// Read-only K9 projection of the current producer state. This is intentionally
/// detached from host layout and disclosure policy: a caller still decides what
/// protected Personal/Bimba material may be shown in a surface or AgentSession.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InstrumentOwnerView {
    pub event: EventBasisRefs,
    pub available: bool,
    pub field: Value,
    pub m1: Value,
    pub m2: Value,
    pub m3: Value,
    pub derivation: Value,
    pub personal: Option<PersonalFieldState>,
    pub personal_current: bool,
    pub vak_performance: Option<VakPerformanceSummary>,
}

impl InstrumentOwnerView {
    pub fn from_session(session: &PersonalCoupledSession) -> Result<Self, String> {
        let basis = session.current_basis();
        let event = EventBasisRefs::from_basis(basis)?;
        let field = session.last_field().clone();
        FieldCursor::from_parts(&event, &field)?;
        let vak_performance = vak_performance(basis)?;
        if let Some(performance) = &vak_performance
            && performance.subject_ref != event.subject_ref
        {
            return Err("Vāk performance and focused world name different subjects".into());
        }
        Ok(Self {
            event,
            available: session.available(),
            field,
            m1: basis.m1.clone(),
            m2: basis.m2.clone(),
            m3: basis.m3.clone(),
            derivation: basis.derivation.clone(),
            personal: session.current_personal().cloned(),
            personal_current: session.personal_is_current()?,
            vak_performance,
        })
    }

    pub fn cursor(&self) -> Result<FieldCursor, String> {
        FieldCursor::from_parts(&self.event, &self.field)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FocusDisclosure {
    pub focus: InstrumentFocus,
    pub available: bool,
    pub current: bool,
    pub source_refs: Vec<String>,
    pub payload: Value,
    pub standing: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClockDisclosure {
    pub presentation: ClockPresentation,
    pub owner_clock: Value,
    pub field_ref: String,
    pub centre_ref: String,
    pub standing: String,
}

/// Private, session-local presentation reading for the O:I Expression host.
/// It carries the actual accepted K10 receiver results required to render this
/// Nara, but deliberately omits identity layers, quaternions and constitution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NaraCentreExpressionReading {
    pub locus_ref: String,
    pub ordinal: u8,
    pub label: String,
    pub source: SourceRevision,
    pub m1: WorldContribution,
    pub m2: WorldContribution,
    pub m3: WorldContribution,
    pub resonance: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NaraEarthBodyExpressionReading {
    pub locus_ref: String,
    pub source: SourceRevision,
    pub frame_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NaraExpressionAvailability {
    Available,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NaraResonanceStationDisclosure {
    pub availability: NaraExpressionAvailability,
    pub station_refs: Vec<String>,
    pub standing: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NaraExpressionPortableCues {
    pub schema: String,
    pub subject_ref: String,
    pub event_ref: String,
    pub profile_generation: u64,
    pub personal_reception_generation: u64,
    pub centre_locus_refs: Vec<String>,
    pub earth_body_locus_ref: String,
    pub source_refs: Vec<String>,
    pub cue_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NaraExpressionSession {
    pub schema: String,
    pub subject_ref: String,
    pub event_ref: String,
    pub profile_generation: u64,
    pub personal_reception_generation: u64,
    pub current: bool,
    pub centres: Vec<NaraCentreExpressionReading>,
    pub earth_body: NaraEarthBodyExpressionReading,
    pub resonance_stations: NaraResonanceStationDisclosure,
    pub m1_reading_refs: Vec<String>,
    pub m2_reading_refs: Vec<String>,
    pub m3_reading_refs: Vec<String>,
    pub action_refs: Vec<String>,
    pub m1_presentation: Value,
    pub m2_presentation: Value,
    pub m3_presentation: Value,
    pub portable: NaraExpressionPortableCues,
    pub standing: String,
}

impl NaraExpressionSession {
    fn from_view(
        personal: &PersonalFieldState,
        view: &InstrumentOwnerView,
    ) -> Result<Self, String> {
        if personal.receivers.len() != 7 {
            return Err("Nara Expression requires exactly seven accepted centres".into());
        }
        let mut centres = personal
            .receivers
            .iter()
            .map(|receiver| NaraCentreExpressionReading {
                locus_ref: format!(
                    "ql:nara:{}:centre:{}",
                    personal.subject_id, receiver.ordinal
                ),
                ordinal: receiver.ordinal,
                label: receiver.label.clone(),
                source: receiver.source.clone(),
                m1: receiver.input_basis[0].clone(),
                m2: receiver.input_basis[1].clone(),
                m3: receiver.input_basis[2].clone(),
                resonance: receiver.resonance,
            })
            .collect::<Vec<_>>();
        centres.sort_by_key(|centre| centre.ordinal);
        if centres
            .iter()
            .enumerate()
            .any(|(ordinal, centre)| usize::from(centre.ordinal) != ordinal)
            || centres.iter().any(|centre| {
                !centre.resonance.is_finite()
                    || !centre.m1.value.is_finite()
                    || !centre.m2.value.is_finite()
                    || !centre.m3.value.is_finite()
            })
        {
            return Err("Nara Expression centre reading is incomplete or non-finite".into());
        }
        let earth_body = NaraEarthBodyExpressionReading {
            locus_ref: format!("ql:nara:{}:earth-body", personal.subject_id),
            source: personal.earth_body.source.clone(),
            frame_ref: personal.earth_body.frame_ref.clone(),
        };
        let source_refs = centres
            .iter()
            .map(|centre| format!("{}@{}", centre.source.source_ref, centre.source.revision))
            .chain(std::iter::once(format!(
                "{}@{}",
                earth_body.source.source_ref, earth_body.source.revision
            )))
            .collect::<Vec<_>>();
        let centre_locus_refs = centres
            .iter()
            .map(|centre| centre.locus_ref.clone())
            .collect();
        let portable = NaraExpressionPortableCues {
            schema: NARA_EXPRESSION_PORTABLE_CUES_CONTRACT.into(),
            subject_ref: personal.subject_id.clone(),
            event_ref: personal.event.event_ref.clone(),
            profile_generation: personal.event.profile_generation,
            personal_reception_generation: personal.reception_generation,
            centre_locus_refs,
            earth_body_locus_ref: earth_body.locus_ref.clone(),
            source_refs,
            cue_refs: vec!["ql:nara:focus:m4".into()],
        };
        Ok(Self {
            schema: NARA_EXPRESSION_SESSION_CONTRACT.into(),
            subject_ref: personal.subject_id.clone(),
            event_ref: personal.event.event_ref.clone(),
            profile_generation: personal.event.profile_generation,
            personal_reception_generation: personal.reception_generation,
            current: view.personal_current,
            centres,
            earth_body,
            resonance_stations: NaraResonanceStationDisclosure {
                availability: NaraExpressionAvailability::Unavailable,
                station_refs: Vec::new(),
                standing: "the accepted owner does not disclose cymatic station identities; centres are not substituted".into(),
            },
            m1_reading_refs: vec![personal.event.m1_revision.clone()],
            m2_reading_refs: vec![personal.event.m2_source_ref.clone(), personal.event.m2_contract_ref.clone()],
            m3_reading_refs: vec![personal.event.m3_source_ref.clone(), personal.event.m3_contract_ref.clone()],
            action_refs: Vec::new(),
            m1_presentation: json!({
                "reflection":view.m1.get("reflection").cloned().unwrap_or(Value::Null),
                "rotor":view.m1.get("rotor").cloned().unwrap_or(Value::Null),
                "carrier":view.m1.get("carrier").cloned().unwrap_or(Value::Null),
            }),
            m2_presentation: json!({
                "colour":view.m2.pointer("/condition/colour").cloned().unwrap_or(Value::Null),
                "physical_material":view.m2.pointer("/condition/physical_material").cloned().unwrap_or(Value::Null),
                "m3_form_potential_ref":view.m2.pointer("/condition/m3_form_potential_ref").cloned().unwrap_or(Value::Null),
                "modal":view.m2.get("modal").cloned().unwrap_or(Value::Null),
            }),
            m3_presentation: json!({
                "form":view.m3.get("form").cloned().unwrap_or(Value::Null),
                "clock":view.m3.get("clock").cloned().unwrap_or(Value::Null),
                "transcription":view.m3.get("transcription").cloned().unwrap_or(Value::Null),
            }),
            portable,
            standing: "private session-local K10 presentation reading; portable contains refs and cues only".into(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FocusedInstrumentSnapshot {
    pub schema: String,
    pub available: bool,
    pub event: EventBasisRefs,
    pub live_cursor: FieldCursor,
    pub presented_cursor: FieldCursor,
    pub temporal: TemporalPresentation,
    pub tracking: SelectionTracking,
    pub selection: Option<BimbaSelection>,
    pub selection_standing: Option<SelectionStanding>,
    pub selected_target: Option<FieldTarget>,
    pub focus: FocusDisclosure,
    pub clock: ClockDisclosure,
    pub vak_expression: Option<VakExpressionBinding>,
    pub vak_performance: Option<VakPerformanceSummary>,
    pub personal_current: bool,
    pub nara_expression: Option<NaraExpressionSession>,
    pub standing: String,
}

#[derive(Debug, Clone)]
struct SelectedBimba {
    value: BimbaSelection,
    selected_at: FieldCursor,
}

/// Host-lifetime focused state. It stores only presentation/selection choices
/// and (when requested) one immutable field receipt for a frozen view. The
/// continuous session remains the only numerical state owner.
#[derive(Debug, Clone)]
pub struct FocusedInstrument {
    focus: InstrumentFocus,
    tracking: SelectionTracking,
    clock: ClockPresentation,
    selection: Option<SelectedBimba>,
    vak_expression: Option<VakExpressionBinding>,
    frozen_field: Option<Value>,
}

impl Default for FocusedInstrument {
    fn default() -> Self {
        Self {
            focus: InstrumentFocus::M3,
            tracking: SelectionTracking::Follow,
            clock: ClockPresentation::Assembled,
            selection: None,
            vak_expression: None,
            frozen_field: None,
        }
    }
}

impl FocusedInstrument {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_focus(&mut self, focus: InstrumentFocus) {
        self.focus = focus;
    }

    pub fn set_tracking(&mut self, tracking: SelectionTracking) {
        self.tracking = tracking;
    }

    pub fn assemble_clock(&mut self) {
        self.clock = ClockPresentation::Assembled;
    }

    pub fn explode_clock(&mut self, pair: Option<u8>) -> Result<(), String> {
        let value = ClockPresentation::Exploded { pair };
        value.validate()?;
        self.clock = value;
        Ok(())
    }

    pub fn bind_vak_expression(
        &mut self,
        view: &InstrumentOwnerView,
        binding: VakExpressionBinding,
    ) -> Result<(), String> {
        binding.validate(view.vak_performance.as_ref())?;
        self.vak_expression = Some(binding);
        Ok(())
    }

    pub fn clear_vak_expression(&mut self) {
        self.vak_expression = None;
    }

    pub fn select_bimba(
        &mut self,
        view: &InstrumentOwnerView,
        selection: BimbaSelection,
    ) -> Result<(), String> {
        selection.validate()?;
        if selection.subject_ref != view.event.subject_ref {
            return Err("Bimba selection belongs to another focused subject".into());
        }
        if let Some(constituent) = &selection.field_constituent_ref
            && field_target(&view.field, constituent)?.is_none()
        {
            return Err(
                "Bimba selection names no constituent in the current material field".into(),
            );
        }
        self.selection = Some(SelectedBimba {
            value: selection,
            selected_at: view.cursor()?,
        });
        Ok(())
    }

    pub fn clear_selection(&mut self) {
        self.selection = None;
    }

    pub fn freeze(&mut self, view: &InstrumentOwnerView) -> Result<(), String> {
        view.cursor()?;
        self.frozen_field = Some(view.field.clone());
        Ok(())
    }

    pub fn resume_live(&mut self) {
        self.frozen_field = None;
    }

    pub fn snapshot(
        &mut self,
        view: &InstrumentOwnerView,
    ) -> Result<FocusedInstrumentSnapshot, String> {
        self.clock.validate()?;
        if let Some(binding) = &self.vak_expression {
            binding.validate(view.vak_performance.as_ref())?;
        }
        let live_cursor = view.cursor()?;
        let (temporal, presented_field) = match &self.frozen_field {
            Some(field) => (TemporalPresentation::Frozen, field),
            None => (TemporalPresentation::Live, &view.field),
        };
        let presented_cursor = FieldCursor::from_parts(&view.event, presented_field)?;

        let mut selection_standing = None;
        let mut selected_target = None;
        if let Some(selection) = self.selection.as_mut() {
            let stale = selection.selected_at.event_ref != live_cursor.event_ref
                || selection.selected_at.subject_ref != live_cursor.subject_ref
                || selection.selected_at.profile_generation != live_cursor.profile_generation;
            selection_standing = Some(if stale {
                SelectionStanding::Stale
            } else if self.tracking == SelectionTracking::Pinned
                && selection.selected_at.field_generation != live_cursor.field_generation
            {
                SelectionStanding::FieldAdvanced
            } else {
                SelectionStanding::Current
            });
            if !stale && self.tracking == SelectionTracking::Follow {
                selection.selected_at = live_cursor.clone();
            }
            if let Some(constituent) = &selection.value.field_constituent_ref {
                selected_target = field_target(presented_field, constituent)?;
            }
        }

        let focus = focus_disclosure(self.focus, view)?;
        let owner_clock = presented_field["clock"].clone();
        let field_ref = owner_clock["field_ref"]
            .as_str()
            .ok_or("continuous owner clock lacks field reference")?
            .to_owned();
        let centre_ref = owner_clock["centre_ref"]
            .as_str()
            .ok_or("continuous owner clock lacks centre reference")?
            .to_owned();
        if field_ref != "#3-0" || centre_ref != "#3-5-5/0" {
            return Err("focused clock is not the accepted M3 field/centre".into());
        }

        let nara_expression = view
            .personal
            .as_ref()
            .map(|personal| NaraExpressionSession::from_view(personal, view))
            .transpose()?;
        Ok(FocusedInstrumentSnapshot {
            schema: FOCUSED_INSTRUMENT_CONTRACT.into(),
            available: view.available,
            event: view.event.clone(),
            live_cursor,
            presented_cursor,
            temporal,
            tracking: self.tracking,
            selection: self.selection.as_ref().map(|selection| selection.value.clone()),
            selection_standing,
            selected_target,
            focus,
            clock: ClockDisclosure {
                presentation: self.clock.clone(),
                owner_clock,
                field_ref,
                centre_ref,
                standing: "assembled/exploded is presentation over the same K8 clock; phase changes remain native domain operations".into(),
            },
            vak_expression: self.vak_expression.clone(),
            vak_performance: view.vak_performance.clone(),
            personal_current: view.personal_current,
            nara_expression,
            standing: "K9 consumer projection: one K8 field, source-qualified Bimba selection, source-owned Vāk, Nara reception and native-host companions; no renderer/graph/clock/agent ownership".into(),
        })
    }

    /// Deliberate M3 clock operation through the existing K8 owner. A native
    /// refusal is reported without changing K9 presentation state; transport
    /// loss is `Unknown` and is never retried here.
    pub fn set_clock_axis(
        &mut self,
        session: &mut PersonalCoupledSession,
        axis: u8,
        phase: LiftInput,
    ) -> InstrumentOperationObservation {
        let before = InstrumentOwnerView::from_session(session).and_then(|view| view.cursor());
        let result = session.set_axis_field(axis, phase);
        operation_observation("set-axis", session, before, result)
    }

    pub fn advance(
        &mut self,
        session: &mut PersonalCoupledSession,
        frames: u32,
        muted: bool,
    ) -> InstrumentOperationObservation {
        let before = InstrumentOwnerView::from_session(session).and_then(|view| view.cursor());
        let result = session.advance_field(frames, muted);
        operation_observation("advance", session, before, result)
    }
}

fn focus_disclosure(
    focus: InstrumentFocus,
    view: &InstrumentOwnerView,
) -> Result<FocusDisclosure, String> {
    let (available, current, source_refs, payload, standing) = match focus {
        InstrumentFocus::M1 => (
            true,
            true,
            vec![view.event.m1_revision.clone()],
            view.m1.clone(),
            "M1 focus discloses the retained carrier/topological owner state",
        ),
        InstrumentFocus::M2 => (
            true,
            true,
            vec![
                view.event.m2_source_ref.clone(),
                view.event.m2_contract_ref.clone(),
            ],
            view.m2.clone(),
            "M2 focus discloses the retained spectral/material/aperture owner state",
        ),
        InstrumentFocus::M3 => (
            true,
            true,
            vec![
                view.event.m3_source_ref.clone(),
                view.event.m3_contract_ref.clone(),
            ],
            json!({"m3":view.m3, "clock":view.field["clock"], "derivation":view.derivation}),
            "M3 focus discloses form/inscription and the existing coupled clock",
        ),
        InstrumentFocus::M4 => match &view.personal {
            Some(personal) => (
                true,
                view.personal_current,
                personal
                    .source_revisions
                    .iter()
                    .map(|source| format!("{}@{}", source.source_ref, source.revision))
                    .collect(),
                serde_json::to_value(personal).map_err(|error| error.to_string())?,
                if view.personal_current {
                    "M4 focus discloses the current protected Nara reception"
                } else {
                    "M4 focus retains a stale protected Nara reception without relabelling it current"
                },
            ),
            None => (
                false,
                false,
                Vec::new(),
                Value::Null,
                "M4 focus is unavailable until the Nara owner supplies a reception",
            ),
        },
        InstrumentFocus::M5 => match &view.vak_performance {
            Some(performance) => (
                true,
                true,
                performance.source_refs.clone(),
                serde_json::to_value(performance).map_err(|error| error.to_string())?,
                "M5 focus discloses actual QL/Factory performance evidence; the desktop keeps the canonical AgentSession owner",
            ),
            None => (
                false,
                false,
                Vec::new(),
                Value::Null,
                "M5 focus has no performed Vāk occasion; K9 does not invent an agent action",
            ),
        },
    };
    Ok(FocusDisclosure {
        focus,
        available,
        current,
        source_refs,
        payload,
        standing: standing.into(),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OperationStanding {
    Applied,
    Refused,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InstrumentOperationObservation {
    pub schema: String,
    pub operation: String,
    pub standing: OperationStanding,
    pub before: Option<FieldCursor>,
    pub after: Option<FieldCursor>,
    pub owner_receipt: Option<Value>,
    pub error: Option<String>,
}

fn operation_observation(
    operation: &str,
    session: &PersonalCoupledSession,
    before: Result<FieldCursor, String>,
    result: Result<Value, String>,
) -> InstrumentOperationObservation {
    let before = before.ok();
    match result {
        Ok(receipt) => {
            let after = InstrumentOwnerView::from_session(session)
                .and_then(|view| view.cursor())
                .ok();
            InstrumentOperationObservation {
                schema: OPERATION_OBSERVATION_CONTRACT.into(),
                operation: operation.into(),
                standing: OperationStanding::Applied,
                before,
                after,
                owner_receipt: Some(receipt),
                error: None,
            }
        }
        Err(error) => InstrumentOperationObservation {
            schema: OPERATION_OBSERVATION_CONTRACT.into(),
            operation: operation.into(),
            standing: if session.available() {
                OperationStanding::Refused
            } else {
                OperationStanding::Unknown
            },
            before: before.clone(),
            after: if session.available() { before } else { None },
            owner_receipt: None,
            error: Some(error),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nara::{
        BioQuaternion, ConsentState, EarthBodyState, EventBasisRefs, LifecycleState, ReceiverState,
    };

    fn field(generation: u64, elapsed: u64, x: f64, profile_generation: u64) -> Value {
        json!({
            "schema":"ql.continuous-field/v1",
            "event_ref":"event:1",
            "subject_ref":"nara:1",
            "generation":generation.to_string(),
            "samples_elapsed":elapsed.to_string(),
            "m2_identity":{"profile_generation":profile_generation},
            "clock":{
                "field_ref":"#3-0",
                "centre_ref":"#3-5-5/0",
                "generation":generation.to_string(),
                "inscription":{"turns":"0","half_degrees":0,"double_cover_half_degrees":0},
                "lensing":{"turns":"0","half_degrees":0,"double_cover_half_degrees":0}
            },
            "targets":[{"identity":7,"constituent":"M2-0-2-0-0","position":[x,2.0,3.0]}]
        })
    }

    fn view(generation: u64, elapsed: u64, x: f64, profile_generation: u64) -> InstrumentOwnerView {
        InstrumentOwnerView {
            event: EventBasisRefs {
                event_ref: "event:1".into(),
                subject_ref: "nara:1".into(),
                profile_generation,
                registry_revision: "registry:1".into(),
                m1_revision: "m1:1".into(),
                m2_source_ref: "m2:source".into(),
                m2_contract_ref: "m2:contract".into(),
                m3_source_ref: "m3:source".into(),
                m3_contract_ref: "m3:contract".into(),
            },
            available: true,
            field: field(generation, elapsed, x, profile_generation),
            m1: json!({"owner":"M1"}),
            m2: json!({"owner":"M2"}),
            m3: json!({"owner":"M3"}),
            derivation: json!({"basis":"retained"}),
            personal: None,
            personal_current: false,
            vak_performance: None,
        }
    }

    fn selection() -> BimbaSelection {
        BimbaSelection {
            contract: BIMBA_SELECTION_CONTRACT.into(),
            selection_ref: "selection:1".into(),
            coordinate_ref: "M2-0-2-0-0".into(),
            source_ref: "bimba:node:1".into(),
            source_revision: "bimba:r1".into(),
            disclosure_ref: "bimba:disclosure:1".into(),
            subject_ref: "nara:1".into(),
            field_constituent_ref: Some("M2-0-2-0-0".into()),
            assertion_refs: vec!["assertion:1".into()],
        }
    }

    fn source(name: &str) -> SourceRevision {
        SourceRevision {
            source_ref: format!("source:{name}"),
            revision: "r1".into(),
            standing_ref: "accepted".into(),
        }
    }

    fn personal(profile_generation: u64) -> PersonalFieldState {
        let event = view(1, 0, 1.0, profile_generation).event;
        PersonalFieldState {
            schema: "ql.nara-personal-field/v1".into(),
            subject_id: "nara:1".into(),
            constitution_ref: "protected:constitution:DO-NOT-EXPORT".into(),
            reception_generation: 9,
            event: event.clone(),
            observed_at_unix_ms: 1,
            consent: ConsentState::Granted,
            lifecycle: LifecycleState::Active,
            q_identity: BioQuaternion::IDENTITY,
            q_transit: BioQuaternion::IDENTITY,
            q_activity: BioQuaternion::IDENTITY,
            q_composed: BioQuaternion::IDENTITY,
            ephemeral_source: Some(source("private-journal-DO-NOT-EXPORT")),
            receivers: (0..7)
                .map(|ordinal| ReceiverState {
                    ordinal,
                    label: format!("centre-{ordinal}"),
                    source: source(&format!("centre-{ordinal}")),
                    input_basis: [
                        WorldContribution {
                            basis_ref: event.m1_revision.clone(),
                            source_ref: format!("m1:centre:{ordinal}"),
                            value: ordinal as f64 + 0.1,
                        },
                        WorldContribution {
                            basis_ref: event.m2_source_ref.clone(),
                            source_ref: format!("m2:centre:{ordinal}"),
                            value: ordinal as f64 + 0.2,
                        },
                        WorldContribution {
                            basis_ref: event.m3_source_ref.clone(),
                            source_ref: format!("m3:centre:{ordinal}"),
                            value: ordinal as f64 + 0.3,
                        },
                    ],
                    bioquaternion: BioQuaternion::IDENTITY,
                    receiver_orientation: BioQuaternion::IDENTITY,
                    composed_orientation: BioQuaternion::IDENTITY,
                    orientation_alignment: 1.0,
                    drive: ordinal as f64,
                    resonance: ordinal as f64 + 0.5,
                    reradiation: ordinal as f64 + 0.25,
                })
                .collect(),
            earth_body: EarthBodyState {
                source: source("earth-body"),
                frame_ref: "earth-fixed".into(),
                orientation: BioQuaternion::IDENTITY,
                relation_alignment: 1.0,
            },
            aggregate_resonance: 3.5,
            aggregate_reradiation: 2.5,
            source_revisions: vec![source("private-whole-DO-NOT-EXPORT")],
            standing: "accepted".into(),
        }
    }

    #[test]
    fn pinned_selection_survives_live_field_motion_and_frozen_view_is_explicit() {
        let mut instrument = FocusedInstrument::new();
        let first = view(1, 0, 1.0, 1);
        instrument.select_bimba(&first, selection()).unwrap();
        instrument.set_tracking(SelectionTracking::Pinned);
        instrument.freeze(&first).unwrap();

        let second = view(2, 64, 9.0, 1);
        let snapshot = instrument.snapshot(&second).unwrap();
        assert_eq!(snapshot.temporal, TemporalPresentation::Frozen);
        assert_eq!(
            snapshot.selection_standing,
            Some(SelectionStanding::FieldAdvanced)
        );
        assert_eq!(snapshot.live_cursor.field_generation, "2");
        assert_eq!(snapshot.presented_cursor.field_generation, "1");
        assert_eq!(snapshot.selected_target.unwrap().position[0], 1.0);
        assert_eq!(snapshot.selection.unwrap().source_revision, "bimba:r1");

        instrument.resume_live();
        let snapshot = instrument.snapshot(&second).unwrap();
        assert_eq!(snapshot.temporal, TemporalPresentation::Live);
        assert_eq!(snapshot.selected_target.unwrap().position[0], 9.0);
    }

    #[test]
    fn focus_and_exploded_clock_are_presentation_not_domain_mutation() {
        let source = view(5, 128, 3.0, 1);
        let original = source.field.clone();
        let mut instrument = FocusedInstrument::new();
        instrument.set_focus(InstrumentFocus::M3);
        instrument.explode_clock(Some(3)).unwrap();
        let snapshot = instrument.snapshot(&source).unwrap();
        assert_eq!(snapshot.focus.focus, InstrumentFocus::M3);
        assert_eq!(
            snapshot.clock.presentation,
            ClockPresentation::Exploded { pair: Some(3) }
        );
        assert_eq!(source.field, original);
        assert!(instrument.explode_clock(Some(8)).is_err());
    }

    #[test]
    fn cross_event_reentry_marks_selection_stale_without_retargeting_source() {
        let first = view(1, 0, 1.0, 1);
        let mut instrument = FocusedInstrument::new();
        instrument.select_bimba(&first, selection()).unwrap();
        instrument.set_tracking(SelectionTracking::Pinned);

        let mut later = view(8, 512, 4.0, 2);
        later.event.event_ref = "event:2".into();
        later.field["event_ref"] = json!("event:2");
        let snapshot = instrument.snapshot(&later).unwrap();
        assert_eq!(snapshot.selection_standing, Some(SelectionStanding::Stale));
        assert_eq!(snapshot.selection.unwrap().source_ref, "bimba:node:1");
    }

    #[test]
    fn vak_binding_requires_real_performance_for_music_but_all_media_can_coexist() {
        let mut source = view(1, 0, 1.0, 1);
        let mut instrument = FocusedInstrument::new();
        let binding = VakExpressionBinding {
            contract: VAK_EXPRESSION_BINDING_CONTRACT.into(),
            source_entry_ref: "vak:entry:0/1".into(),
            source_revision: "vak:r1".into(),
            literal: Some("0/1".into()),
            glyph_ref: Some("glyph:0-1".into()),
            image_ref: Some("image:relation".into()),
            musical_performance_ref: Some("performance:1".into()),
            enactment_ref: Some("enactment:0-to-1-as-slash".into()),
        };
        assert!(
            instrument
                .bind_vak_expression(&source, binding.clone())
                .is_err()
        );
        source.vak_performance = Some(VakPerformanceSummary {
            performance_ref: "performance:1".into(),
            mode: PerformanceMode::Replay,
            replay_of: Some("performance:1".into()),
            actor_ref: "epii".into(),
            subject_ref: "nara:1".into(),
            context_frame: "CF2".into(),
            musical_role: "melody".into(),
            musical_mode: "dorian".into(),
            lens: "L1".into(),
            settled: true,
            has_failure: false,
            has_interruption: true,
            has_late_return: true,
            source_refs: vec!["vak:source".into()],
        });
        instrument.bind_vak_expression(&source, binding).unwrap();
        let snapshot = instrument.snapshot(&source).unwrap();
        assert_eq!(snapshot.vak_expression.unwrap().media().len(), 5);
        let performance = snapshot.vak_performance.unwrap();
        assert_eq!(performance.mode, PerformanceMode::Replay);
        assert!(performance.has_interruption);
        assert!(performance.has_late_return);
    }

    #[test]
    fn follow_selection_advances_its_anchor_while_pin_retains_history() {
        let first = view(1, 0, 1.0, 1);
        let second = view(2, 16, 2.0, 1);
        let third = view(3, 32, 3.0, 1);
        let mut instrument = FocusedInstrument::new();
        instrument.select_bimba(&first, selection()).unwrap();
        instrument.snapshot(&second).unwrap();
        instrument.set_tracking(SelectionTracking::Pinned);
        let snapshot = instrument.snapshot(&third).unwrap();
        assert_eq!(
            snapshot.selection_standing,
            Some(SelectionStanding::FieldAdvanced)
        );
    }

    #[test]
    fn nara_expression_projects_actual_independent_centres_and_distinct_earth_body() {
        let mut source = view(1, 0, 1.0, 1);
        source.personal = Some(personal(1));
        source.personal_current = true;
        let snapshot = FocusedInstrument::new().snapshot(&source).unwrap();
        let nara = snapshot.nara_expression.unwrap();
        assert_eq!(nara.centres.len(), 7);
        assert_eq!(nara.centres[6].resonance, 6.5);
        assert_eq!(nara.centres[6].m2.source_ref, "m2:centre:6");
        assert_eq!(nara.earth_body.locus_ref, "ql:nara:nara:1:earth-body");
        assert!(
            nara.centres
                .iter()
                .all(|centre| centre.locus_ref != nara.earth_body.locus_ref)
        );
        assert_eq!(
            nara.resonance_stations.availability,
            NaraExpressionAvailability::Unavailable
        );
        assert!(nara.resonance_stations.station_refs.is_empty());
    }

    #[test]
    fn portable_nara_cues_exclude_live_and_protected_values() {
        let mut source = view(1, 0, 1.0, 1);
        source.personal = Some(personal(1));
        let snapshot = FocusedInstrument::new().snapshot(&source).unwrap();
        let nara = snapshot.nara_expression.unwrap();
        let portable = serde_json::to_string(&nara.portable).unwrap();
        assert!(!portable.contains("DO-NOT-EXPORT"));
        assert!(!portable.contains("resonance"));
        assert!(!portable.contains("bioquaternion"));
        assert!(!portable.contains("constitution"));
        assert_eq!(nara.portable.centre_locus_refs.len(), 7);
    }

    #[test]
    fn stale_personal_generation_remains_stale_in_session_projection() {
        let mut source = view(8, 512, 4.0, 2);
        source.personal = Some(personal(1));
        source.personal_current = false;
        let snapshot = FocusedInstrument::new().snapshot(&source).unwrap();
        assert!(!snapshot.nara_expression.unwrap().current);
    }
}
