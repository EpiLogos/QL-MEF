//! Complete source-qualified M4/Nara domain state above the early Personal receiver.
//!
//! This module retains all six M4 branches as distinct offices. It deliberately
//! does not turn historical C stubs, model interpretation, or rendered amplitude
//! into personal truth. Sensitive source bodies are represented through protected
//! references; owners such as Central and O:I remain responsible for storage and
//! presentation.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::{BioQuaternion, ConsentState, EventBasisRefs, SourceRevision};

pub const M4_DOMAIN_CONTRACT: &str = "ql.nara-m4-domain/v1";
pub const IDENTITY_SLOT_COUNT: usize = 6;
pub const CENTRE_COUNT: usize = 7;
pub const CONTEXT_BRANCH_COUNT: usize = 6;
pub const INTEGRATION_OFFICE_COUNT: usize = 6;
pub const ELEMENT_COUNT: usize = 4;

fn valid_text(value: &str) -> bool {
    !value.trim().is_empty() && value.len() <= 4096 && !value.chars().any(char::is_control)
}

fn check_text(value: &str, label: &str) -> Result<(), String> {
    if valid_text(value) {
        Ok(())
    } else {
        Err(format!("invalid {label}"))
    }
}

fn check_source(source: &SourceRevision) -> Result<(), String> {
    check_text(&source.source_ref, "source reference")?;
    check_text(&source.revision, "source revision")?;
    check_text(&source.standing_ref, "source standing")
}

fn check_refs(values: &[String], label: &str, max: usize) -> Result<(), String> {
    if values.len() > max {
        return Err(format!("too many {label}"));
    }
    let mut unique = BTreeSet::new();
    for value in values {
        check_text(value, label)?;
        if !unique.insert(value.as_str()) {
            return Err(format!("duplicate {label}"));
        }
    }
    Ok(())
}

fn check_optional_finite(value: Option<f64>, label: &str) -> Result<(), String> {
    if value.is_some_and(|number| !number.is_finite()) {
        Err(format!("non-finite {label}"))
    } else {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvidenceStanding {
    Source,
    AuthoredArchitecture,
    Implementation,
    Observed,
    Derived,
    Reported,
    Proposed,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProtectedRef {
    pub ref_id: String,
    pub revision: String,
    pub owner_ref: String,
}

impl ProtectedRef {
    pub fn validate(&self) -> Result<(), String> {
        check_text(&self.ref_id, "protected reference")?;
        check_text(&self.revision, "protected revision")?;
        check_text(&self.owner_ref, "protected owner")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum M4Branch {
    Identity,
    Embodied,
    Oracle,
    Transformation,
    Context,
    Integration,
}

impl M4Branch {
    pub const fn coordinate(self) -> &'static str {
        match self {
            Self::Identity => "M4.0",
            Self::Embodied => "M4.1",
            Self::Oracle => "M4.2",
            Self::Transformation => "M4.3",
            Self::Context => "M4.4",
            Self::Integration => "M4.5",
        }
    }
}

// ---------------------------------------------------------------------------
// M4.0 — identity constitution

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IdentitySlotKind {
    BirthdateName,
    NatalChart,
    JungianAssessment,
    GeneKeys,
    HumanDesign,
    ArchetypalQuintessence,
}

impl IdentitySlotKind {
    pub const fn coordinate(self) -> &'static str {
        match self {
            Self::BirthdateName => "M4.0.0",
            Self::NatalChart => "M4.0.1",
            Self::JungianAssessment => "M4.0.2",
            Self::GeneKeys => "M4.0.3",
            Self::HumanDesign => "M4.0.4",
            Self::ArchetypalQuintessence => "M4.0.5",
        }
    }

    pub const fn ordinal(self) -> u8 {
        match self {
            Self::BirthdateName => 0,
            Self::NatalChart => 1,
            Self::JungianAssessment => 2,
            Self::GeneKeys => 3,
            Self::HumanDesign => 4,
            Self::ArchetypalQuintessence => 5,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdentityEvidenceSlot {
    pub kind: IdentitySlotKind,
    pub coordinate_ref: String,
    pub source: Option<SourceRevision>,
    pub protected_value_ref: Option<ProtectedRef>,
    pub evidence_refs: Vec<String>,
    pub tensions: Vec<String>,
    pub absence_reason: Option<String>,
    pub standing: EvidenceStanding,
}

impl IdentityEvidenceSlot {
    pub fn validate(&self) -> Result<(), String> {
        if self.coordinate_ref != self.kind.coordinate() {
            return Err("identity slot coordinate does not match its M4.0 office".into());
        }
        if let Some(source) = &self.source {
            check_source(source)?;
        }
        if let Some(value) = &self.protected_value_ref {
            value.validate()?;
        }
        check_refs(&self.evidence_refs, "identity evidence reference", 256)?;
        check_refs(&self.tensions, "identity tension", 64)?;
        if let Some(reason) = &self.absence_reason {
            check_text(reason, "identity absence reason")?;
        }
        if self.protected_value_ref.is_some() && self.absence_reason.is_some() {
            return Err("identity slot cannot be both present and absent".into());
        }
        if self.protected_value_ref.is_none() && self.absence_reason.is_none() {
            return Err("identity slot must retain a value reference or explicit absence".into());
        }
        if self.standing == EvidenceStanding::Unavailable && self.absence_reason.is_none() {
            return Err("unavailable identity standing requires an absence reason".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdentityField {
    pub identity_revision: String,
    pub slots: Vec<IdentityEvidenceSlot>,
    pub identity_hash_ref: Option<ProtectedRef>,
    pub identity_quaternion_ref: Option<ProtectedRef>,
    pub m3_form_address_ref: Option<String>,
    pub derivation_refs: Vec<String>,
}

impl IdentityField {
    pub fn validate(&self) -> Result<(), String> {
        check_text(&self.identity_revision, "identity revision")?;
        if self.slots.len() != IDENTITY_SLOT_COUNT {
            return Err("M4.0 requires all six identity offices, with absence retained explicitly".into());
        }
        let mut kinds = BTreeSet::new();
        for slot in &self.slots {
            slot.validate()?;
            if !kinds.insert(slot.kind) {
                return Err("duplicate M4.0 identity office".into());
            }
        }
        if kinds.len() != IDENTITY_SLOT_COUNT {
            return Err("M4.0 identity office set is incomplete".into());
        }
        if let Some(reference) = &self.identity_hash_ref {
            reference.validate()?;
        }
        if let Some(reference) = &self.identity_quaternion_ref {
            reference.validate()?;
        }
        if let Some(reference) = &self.m3_form_address_ref {
            check_text(reference, "M3 form address reference")?;
        }
        check_refs(&self.derivation_refs, "identity derivation reference", 256)
    }
}

// ---------------------------------------------------------------------------
// M4.1 — embodied / chakral field

/// Canonical quaternion/element order is fixed as Earth, Fire, Water, Air.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ElementalEfwa {
    pub earth: f64,
    pub fire: f64,
    pub water: f64,
    pub air: f64,
}

impl ElementalEfwa {
    pub fn validate(self) -> Result<(), String> {
        if [self.earth, self.fire, self.water, self.air]
            .into_iter()
            .all(f64::is_finite)
        {
            Ok(())
        } else {
            Err("non-finite EFWA elemental state".into())
        }
    }

    pub const fn as_array(self) -> [f64; ELEMENT_COUNT] {
        [self.earth, self.fire, self.water, self.air]
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CentreMode {
    pub mode_ref: String,
    pub amplitude: f64,
    pub phase_radians: f64,
    pub source: SourceRevision,
}

impl CentreMode {
    fn validate(&self) -> Result<(), String> {
        check_text(&self.mode_ref, "centre mode")?;
        check_source(&self.source)?;
        if self.amplitude.is_finite() && self.phase_radians.is_finite() {
            Ok(())
        } else {
            Err("non-finite centre mode".into())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CentreCoupling {
    pub target_ordinal: u8,
    pub coefficient: f64,
    pub phase_offset_radians: f64,
    pub source: SourceRevision,
}

impl CentreCoupling {
    fn validate(&self, own_ordinal: u8) -> Result<(), String> {
        if usize::from(self.target_ordinal) >= CENTRE_COUNT || self.target_ordinal == own_ordinal {
            return Err("invalid centre coupling target".into());
        }
        if !self.coefficient.is_finite() || !self.phase_offset_radians.is_finite() {
            return Err("non-finite centre coupling".into());
        }
        check_source(&self.source)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CentreEmbodiment {
    pub ordinal: u8,
    pub label: String,
    pub source: SourceRevision,
    pub amplitude: Option<f64>,
    pub phase_radians: Option<f64>,
    pub modes: Vec<CentreMode>,
    pub couplings: Vec<CentreCoupling>,
    pub body_zone_refs: Vec<String>,
    pub sense_refs: Vec<String>,
    pub action_refs: Vec<String>,
    pub feedback_refs: Vec<String>,
    pub standing: EvidenceStanding,
}

impl CentreEmbodiment {
    pub fn validate(&self) -> Result<(), String> {
        if usize::from(self.ordinal) >= CENTRE_COUNT {
            return Err("centre ordinal outside the seven-centre field".into());
        }
        check_text(&self.label, "centre label")?;
        check_source(&self.source)?;
        check_optional_finite(self.amplitude, "centre amplitude")?;
        check_optional_finite(self.phase_radians, "centre phase")?;
        if self.modes.len() > 128 || self.couplings.len() > CENTRE_COUNT - 1 {
            return Err("centre mode/coupling field exceeds contract bounds".into());
        }
        for mode in &self.modes {
            mode.validate()?;
        }
        let mut targets = BTreeSet::new();
        for coupling in &self.couplings {
            coupling.validate(self.ordinal)?;
            if !targets.insert(coupling.target_ordinal) {
                return Err("duplicate centre coupling target".into());
            }
        }
        check_refs(&self.body_zone_refs, "body-zone reference", 128)?;
        check_refs(&self.sense_refs, "sense reference", 128)?;
        check_refs(&self.action_refs, "embodied action reference", 128)?;
        check_refs(&self.feedback_refs, "embodied feedback reference", 128)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EarthBodyEmbodiment {
    pub source: SourceRevision,
    pub frame_ref: String,
    pub amplitude: Option<f64>,
    pub phase_radians: Option<f64>,
    pub relation_refs: Vec<String>,
    pub standing: EvidenceStanding,
}

impl EarthBodyEmbodiment {
    fn validate(&self) -> Result<(), String> {
        check_source(&self.source)?;
        check_text(&self.frame_ref, "EarthBody frame")?;
        check_optional_finite(self.amplitude, "EarthBody amplitude")?;
        check_optional_finite(self.phase_radians, "EarthBody phase")?;
        check_refs(&self.relation_refs, "EarthBody relation reference", 128)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmbodiedField {
    pub reception_generation: u64,
    pub elemental_efwa: ElementalEfwa,
    pub centres: Vec<CentreEmbodiment>,
    pub earth_body: EarthBodyEmbodiment,
    pub nadi_refs: Vec<String>,
    pub sushumna_ref: Option<String>,
    pub temporal_astrology_refs: Vec<String>,
    pub materia_refs: Vec<String>,
    pub operation_refs: Vec<String>,
    pub consent: ConsentState,
}

impl EmbodiedField {
    pub fn validate(&self) -> Result<(), String> {
        self.elemental_efwa.validate()?;
        if self.centres.len() != CENTRE_COUNT {
            return Err("M4.1 requires exactly seven independently active centres".into());
        }
        let mut ordinals = BTreeSet::new();
        let mut labels = BTreeSet::new();
        for centre in &self.centres {
            centre.validate()?;
            if !ordinals.insert(centre.ordinal) || !labels.insert(centre.label.as_str()) {
                return Err("duplicate centre ordinal or label".into());
            }
        }
        if ordinals != BTreeSet::from([0, 1, 2, 3, 4, 5, 6]) {
            return Err("seven centre ordinals must be complete".into());
        }
        self.earth_body.validate()?;
        check_refs(&self.nadi_refs, "nadi reference", 256)?;
        if let Some(reference) = &self.sushumna_ref {
            check_text(reference, "sushumna reference")?;
        }
        check_refs(&self.temporal_astrology_refs, "temporal astrology reference", 256)?;
        check_refs(&self.materia_refs, "materia reference", 256)?;
        check_refs(&self.operation_refs, "embodied operation reference", 256)
    }
}

// ---------------------------------------------------------------------------
// M4.2 — oracle

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OracleSystem {
    TarotRws,
    TarotThoth,
    TarotMarseille,
    TarotQl,
    IChingCoins,
    IChingYarrow,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OracleHygiene {
    Clear,
    Warning,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OracleEntropyReceipt {
    pub entropy_ref: ProtectedRef,
    pub method_ref: String,
    pub provider_ref: String,
    pub observed_at_unix_ms: u64,
    pub evidence_refs: Vec<String>,
}

impl OracleEntropyReceipt {
    fn validate(&self) -> Result<(), String> {
        self.entropy_ref.validate()?;
        check_text(&self.method_ref, "oracle entropy method")?;
        check_text(&self.provider_ref, "oracle entropy provider")?;
        check_refs(&self.evidence_refs, "oracle entropy evidence reference", 128)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OracleToken {
    pub token_ref: String,
    pub position: u16,
    pub reversed: bool,
    pub changing: bool,
    pub source_refs: Vec<String>,
}

impl OracleToken {
    fn validate(&self) -> Result<(), String> {
        check_text(&self.token_ref, "oracle token")?;
        check_refs(&self.source_refs, "oracle token source reference", 64)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OracleOriginalPacket {
    pub packet_ref: String,
    pub subject_id: String,
    pub event_ref: String,
    pub profile_generation: u64,
    pub system: OracleSystem,
    pub tradition_ref: String,
    pub deck_or_method_ref: String,
    pub spread_ref: Option<String>,
    pub query_ref: ProtectedRef,
    pub entropy: OracleEntropyReceipt,
    pub tokens: Vec<OracleToken>,
    pub cast_degree: Option<u16>,
    pub cast_at_unix_ms: u64,
    pub vak_ref: Option<String>,
    pub original_payload_ref: ProtectedRef,
    pub source_revisions: Vec<SourceRevision>,
    pub consent: ConsentState,
    pub hygiene: OracleHygiene,
}

impl OracleOriginalPacket {
    pub fn validate(&self) -> Result<(), String> {
        check_text(&self.packet_ref, "oracle packet")?;
        check_text(&self.subject_id, "oracle subject")?;
        check_text(&self.event_ref, "oracle event")?;
        check_text(&self.tradition_ref, "oracle tradition")?;
        check_text(&self.deck_or_method_ref, "oracle deck/method")?;
        if let Some(reference) = &self.spread_ref {
            check_text(reference, "oracle spread")?;
        }
        self.query_ref.validate()?;
        self.entropy.validate()?;
        if self.tokens.is_empty() || self.tokens.len() > 78 {
            return Err("oracle packet requires 1..78 retained cast tokens".into());
        }
        let mut token_positions = BTreeSet::new();
        for token in &self.tokens {
            token.validate()?;
            if !token_positions.insert(token.position) {
                return Err("oracle packet has duplicate token positions".into());
            }
        }
        if self.cast_degree.is_some_and(|degree| degree >= 720) {
            return Err("oracle cast degree must be on the 0..719 double-cover ring".into());
        }
        if let Some(reference) = &self.vak_ref {
            check_text(reference, "oracle Vāk reference")?;
        }
        self.original_payload_ref.validate()?;
        if self.source_revisions.is_empty() || self.source_revisions.len() > 256 {
            return Err("oracle packet requires 1..256 source revisions".into());
        }
        for source in &self.source_revisions {
            check_source(source)?;
        }
        if self.consent != ConsentState::Granted {
            return Err("oracle packet requires explicit granted consent".into());
        }
        if self.hygiene == OracleHygiene::Blocked {
            return Err("blocked oracle packet cannot be admitted as a cast".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OracleInterpretation {
    pub interpretation_ref: String,
    pub packet_ref: String,
    pub interpretation_revision: String,
    pub model_ref: Option<String>,
    pub output_ref: ProtectedRef,
    pub source_refs: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub standing: EvidenceStanding,
    pub interpreted_at_unix_ms: u64,
}

impl OracleInterpretation {
    fn validate(&self, packet_ref: &str) -> Result<(), String> {
        check_text(&self.interpretation_ref, "oracle interpretation")?;
        if self.packet_ref != packet_ref {
            return Err("oracle interpretation belongs to a different original packet".into());
        }
        check_text(&self.interpretation_revision, "oracle interpretation revision")?;
        if let Some(reference) = &self.model_ref {
            check_text(reference, "oracle model")?;
        }
        self.output_ref.validate()?;
        check_refs(&self.source_refs, "oracle interpretation source reference", 256)?;
        check_refs(&self.evidence_refs, "oracle interpretation evidence reference", 256)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OracleRecord {
    pub original: OracleOriginalPacket,
    pub interpretations: Vec<OracleInterpretation>,
}

impl OracleRecord {
    pub fn validate(&self) -> Result<(), String> {
        self.original.validate()?;
        if self.interpretations.len() > 1024 {
            return Err("too many oracle reinterpretations".into());
        }
        let mut ids = BTreeSet::new();
        for interpretation in &self.interpretations {
            interpretation.validate(&self.original.packet_ref)?;
            if !ids.insert(interpretation.interpretation_ref.as_str()) {
                return Err("duplicate oracle interpretation reference".into());
            }
        }
        Ok(())
    }

    pub fn add_reinterpretation(
        &mut self,
        interpretation: OracleInterpretation,
    ) -> Result<(), String> {
        interpretation.validate(&self.original.packet_ref)?;
        if self
            .interpretations
            .iter()
            .any(|existing| existing.interpretation_ref == interpretation.interpretation_ref)
        {
            return Err("oracle interpretation reference already exists".into());
        }
        self.interpretations.push(interpretation);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// M4.3 — transformation

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TransformationSafety {
    Clear,
    HeldArousal,
    Contraindicated,
    ConsentRequired,
    SourceUnavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransformationPhase {
    pub phase_ref: String,
    pub storey: u8,
    pub decan: u8,
    pub stroke: u8,
    pub operation_refs: Vec<String>,
    pub protocol_ref: String,
    pub container_ref: String,
    pub source_revisions: Vec<SourceRevision>,
    pub opened_at_unix_ms: u64,
    pub closed_at_unix_ms: Option<u64>,
    pub safety: TransformationSafety,
    pub feedback_refs: Vec<String>,
}

impl TransformationPhase {
    pub fn validate(&self) -> Result<(), String> {
        check_text(&self.phase_ref, "transformation phase")?;
        if self.storey >= 12 || self.decan >= 3 || self.stroke >= 24 {
            return Err("transformation phase lies outside the 12x3/24-stroke field".into());
        }
        check_refs(&self.operation_refs, "transformation operation reference", 7)?;
        check_text(&self.protocol_ref, "transformation protocol")?;
        check_text(&self.container_ref, "transformation container")?;
        if self.source_revisions.is_empty() || self.source_revisions.len() > 256 {
            return Err("transformation phase requires 1..256 source revisions".into());
        }
        for source in &self.source_revisions {
            check_source(source)?;
        }
        if self
            .closed_at_unix_ms
            .is_some_and(|closed| closed < self.opened_at_unix_ms)
        {
            return Err("transformation phase closes before it opens".into());
        }
        check_refs(&self.feedback_refs, "transformation feedback reference", 256)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransformationHistory {
    pub history_ref: String,
    pub subject_id: String,
    pub phases: Vec<TransformationPhase>,
    pub standing: EvidenceStanding,
}

impl TransformationHistory {
    pub fn validate(&self) -> Result<(), String> {
        check_text(&self.history_ref, "transformation history")?;
        check_text(&self.subject_id, "transformation subject")?;
        if self.phases.len() > 8192 {
            return Err("transformation phase history exceeds contract bounds".into());
        }
        let mut ids = BTreeSet::new();
        let mut previous_opened = None;
        for phase in &self.phases {
            phase.validate()?;
            if !ids.insert(phase.phase_ref.as_str()) {
                return Err("duplicate transformation phase reference".into());
            }
            if previous_opened.is_some_and(|prior| phase.opened_at_unix_ms < prior) {
                return Err("transformation history is not occurrence ordered".into());
            }
            previous_opened = Some(phase.opened_at_unix_ms);
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// M4.4 — context / Jungian / phenomenological depth

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ContextBranchKind {
    Gebser,
    Ontological,
    Epistemological,
    JungianDepth,
    Phenomenological,
    TrikaKashmir,
}

impl ContextBranchKind {
    pub const fn coordinate(self) -> &'static str {
        match self {
            Self::Gebser => "M4.4.0",
            Self::Ontological => "M4.4.1",
            Self::Epistemological => "M4.4.2",
            Self::JungianDepth => "M4.4.3",
            Self::Phenomenological => "M4.4.4",
            Self::TrikaKashmir => "M4.4.5",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextReading {
    pub reading_ref: String,
    pub coordinate_ref: String,
    pub source: SourceRevision,
    pub protected_content_ref: ProtectedRef,
    pub model_ref: Option<String>,
    pub confidence: Option<f64>,
    pub evidence_refs: Vec<String>,
    pub standing: EvidenceStanding,
}

impl ContextReading {
    fn validate(&self, branch: ContextBranchKind) -> Result<(), String> {
        check_text(&self.reading_ref, "context reading")?;
        if !self.coordinate_ref.starts_with(branch.coordinate()) {
            return Err("context reading coordinate lies outside its M4.4 branch".into());
        }
        check_source(&self.source)?;
        self.protected_content_ref.validate()?;
        if let Some(reference) = &self.model_ref {
            check_text(reference, "context model")?;
        }
        if self
            .confidence
            .is_some_and(|confidence| !confidence.is_finite() || !(0.0..=1.0).contains(&confidence))
        {
            return Err("context confidence must be finite and within 0..=1".into());
        }
        check_refs(&self.evidence_refs, "context evidence reference", 256)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextBranchState {
    pub branch: ContextBranchKind,
    pub readings: Vec<ContextReading>,
    pub absence_reason: Option<String>,
}

impl ContextBranchState {
    pub fn validate(&self) -> Result<(), String> {
        if self.readings.is_empty() && self.absence_reason.is_none() {
            return Err("context branch must retain readings or explicit absence".into());
        }
        if !self.readings.is_empty() && self.absence_reason.is_some() {
            return Err("context branch cannot be both populated and absent".into());
        }
        if let Some(reason) = &self.absence_reason {
            check_text(reason, "context absence reason")?;
        }
        if self.readings.len() > 4096 {
            return Err("context branch exceeds reading bounds".into());
        }
        let mut ids = BTreeSet::new();
        for reading in &self.readings {
            reading.validate(self.branch)?;
            if !ids.insert(reading.reading_ref.as_str()) {
                return Err("duplicate context reading reference".into());
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextField {
    pub branches: Vec<ContextBranchState>,
    pub personal_pratibimba_ref: Option<ProtectedRef>,
    pub recognition_refs: Vec<String>,
}

impl ContextField {
    pub fn validate(&self) -> Result<(), String> {
        if self.branches.len() != CONTEXT_BRANCH_COUNT {
            return Err("M4.4 requires all six contextual branches".into());
        }
        let mut branches = BTreeSet::new();
        for branch in &self.branches {
            branch.validate()?;
            if !branches.insert(branch.branch) {
                return Err("duplicate M4.4 contextual branch".into());
            }
        }
        if branches.len() != CONTEXT_BRANCH_COUNT {
            return Err("M4.4 contextual branch set is incomplete".into());
        }
        if let Some(reference) = &self.personal_pratibimba_ref {
            reference.validate()?;
        }
        check_refs(&self.recognition_refs, "M4.4 recognition reference", 256)
    }
}

// ---------------------------------------------------------------------------
// M4.5 — internal integration / pedagogy Return

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IntegrationOffice {
    CurriculumMap,
    CoreEpiLogosVoice,
    MethodTransparencyLab,
    IntegrationLab,
    PedagogyLab,
    LogosCycleEngine,
}

impl IntegrationOffice {
    pub const fn coordinate(self) -> &'static str {
        match self {
            Self::CurriculumMap => "M4.5.0",
            Self::CoreEpiLogosVoice => "M4.5.1",
            Self::MethodTransparencyLab => "M4.5.2",
            Self::IntegrationLab => "M4.5.3",
            Self::PedagogyLab => "M4.5.4",
            Self::LogosCycleEngine => "M4.5.5",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntegrationReturn {
    pub return_ref: String,
    pub office: IntegrationOffice,
    pub input_refs: Vec<String>,
    pub source_refs: Vec<String>,
    pub method_ref: Option<String>,
    pub output_ref: ProtectedRef,
    pub evidence_refs: Vec<String>,
    pub human_response_ref: Option<ProtectedRef>,
    pub retention_policy_ref: String,
    pub standing: EvidenceStanding,
}

impl IntegrationReturn {
    fn validate(&self, office: IntegrationOffice) -> Result<(), String> {
        if self.office != office {
            return Err("integration Return is seated in the wrong M4.5 office".into());
        }
        check_text(&self.return_ref, "integration Return")?;
        check_refs(&self.input_refs, "integration input reference", 512)?;
        check_refs(&self.source_refs, "integration source reference", 512)?;
        if let Some(reference) = &self.method_ref {
            check_text(reference, "integration method")?;
        }
        self.output_ref.validate()?;
        check_refs(&self.evidence_refs, "integration evidence reference", 512)?;
        if let Some(reference) = &self.human_response_ref {
            reference.validate()?;
        }
        check_text(&self.retention_policy_ref, "integration retention policy")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntegrationOfficeState {
    pub office: IntegrationOffice,
    pub available: bool,
    pub unavailable_reason: Option<String>,
    pub returns: Vec<IntegrationReturn>,
}

impl IntegrationOfficeState {
    pub fn validate(&self) -> Result<(), String> {
        if self.available && self.unavailable_reason.is_some() {
            return Err("available M4.5 office cannot carry an unavailable reason".into());
        }
        if !self.available && self.unavailable_reason.is_none() {
            return Err("unavailable M4.5 office requires a reason".into());
        }
        if let Some(reason) = &self.unavailable_reason {
            check_text(reason, "M4.5 unavailable reason")?;
        }
        if self.returns.len() > 4096 {
            return Err("too many Returns in one M4.5 office".into());
        }
        let mut ids = BTreeSet::new();
        for returned in &self.returns {
            returned.validate(self.office)?;
            if !ids.insert(returned.return_ref.as_str()) {
                return Err("duplicate M4.5 Return reference".into());
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntegrationField {
    pub offices: Vec<IntegrationOfficeState>,
    /// Opaque references to #94/Factory/AIKit operative Returns or recognised
    /// praxis. They remain external-owner objects rather than a second store.
    pub operative_return_refs: Vec<String>,
}

impl IntegrationField {
    pub fn validate(&self) -> Result<(), String> {
        if self.offices.len() != INTEGRATION_OFFICE_COUNT {
            return Err("M4.5 requires all six integration offices".into());
        }
        let mut offices = BTreeSet::new();
        for office in &self.offices {
            office.validate()?;
            if !offices.insert(office.office) {
                return Err("duplicate M4.5 integration office".into());
            }
        }
        if offices.len() != INTEGRATION_OFFICE_COUNT {
            return Err("M4.5 integration office set is incomplete".into());
        }
        check_refs(&self.operative_return_refs, "operative Return reference", 512)
    }
}

// ---------------------------------------------------------------------------
// Whole M4 state

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M4DomainState {
    pub schema: String,
    pub subject_id: String,
    pub event: EventBasisRefs,
    pub identity: IdentityField,
    pub embodied: EmbodiedField,
    pub oracle: Vec<OracleRecord>,
    pub transformation: TransformationHistory,
    pub context: ContextField,
    pub integration: IntegrationField,
    pub q_composed: BioQuaternion,
    pub source_revisions: Vec<SourceRevision>,
    pub standing: String,
}

impl M4DomainState {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != M4_DOMAIN_CONTRACT {
            return Err("unsupported M4 domain contract".into());
        }
        check_text(&self.subject_id, "M4 subject")?;
        if self.subject_id != self.event.subject_ref {
            return Err("M4 subject does not match the accepted M1/M2/M3 event".into());
        }
        self.identity.validate()?;
        self.embodied.validate()?;
        if self.oracle.len() > 4096 {
            return Err("M4.2 oracle history exceeds contract bounds".into());
        }
        let mut oracle_ids = BTreeSet::new();
        for record in &self.oracle {
            record.validate()?;
            if record.original.subject_id != self.subject_id
                || record.original.event_ref != self.event.event_ref
                || record.original.profile_generation != self.event.profile_generation
            {
                return Err("oracle record does not belong to this M4 occasion".into());
            }
            if !oracle_ids.insert(record.original.packet_ref.as_str()) {
                return Err("duplicate oracle packet reference".into());
            }
        }
        self.transformation.validate()?;
        if self.transformation.subject_id != self.subject_id {
            return Err("transformation history belongs to another Nara".into());
        }
        self.context.validate()?;
        self.integration.validate()?;
        self.q_composed.normalized()?;
        if self.source_revisions.is_empty() || self.source_revisions.len() > 4096 {
            return Err("M4 domain requires 1..4096 source revisions".into());
        }
        for source in &self.source_revisions {
            check_source(source)?;
        }
        check_text(&self.standing, "M4 standing")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source(name: &str) -> SourceRevision {
        SourceRevision {
            source_ref: format!("source:{name}"),
            revision: "r1".into(),
            standing_ref: "source".into(),
        }
    }

    fn protected(name: &str) -> ProtectedRef {
        ProtectedRef {
            ref_id: format!("protected:{name}"),
            revision: "r1".into(),
            owner_ref: "central".into(),
        }
    }

    fn identity_slot(kind: IdentitySlotKind, present: bool) -> IdentityEvidenceSlot {
        IdentityEvidenceSlot {
            kind,
            coordinate_ref: kind.coordinate().into(),
            source: present.then(|| source("identity")),
            protected_value_ref: present.then(|| protected("identity")),
            evidence_refs: Vec::new(),
            tensions: Vec::new(),
            absence_reason: (!present).then(|| "not supplied".into()),
            standing: if present {
                EvidenceStanding::Source
            } else {
                EvidenceStanding::Unavailable
            },
        }
    }

    #[test]
    fn identity_retains_all_six_offices_without_fabricating_optional_evidence() {
        let field = IdentityField {
            identity_revision: "identity:r1".into(),
            slots: vec![
                identity_slot(IdentitySlotKind::BirthdateName, true),
                identity_slot(IdentitySlotKind::NatalChart, true),
                identity_slot(IdentitySlotKind::JungianAssessment, true),
                identity_slot(IdentitySlotKind::GeneKeys, false),
                identity_slot(IdentitySlotKind::HumanDesign, false),
                identity_slot(IdentitySlotKind::ArchetypalQuintessence, true),
            ],
            identity_hash_ref: Some(protected("hash")),
            identity_quaternion_ref: Some(protected("identity-q")),
            m3_form_address_ref: Some("m3:form:42".into()),
            derivation_refs: vec!["derivation:identity".into()],
        };
        field.validate().unwrap();
        assert_eq!(field.slots.len(), 6);
        assert_eq!(field.slots[3].absence_reason.as_deref(), Some("not supplied"));
    }

    #[test]
    fn seven_centres_are_independent_and_earth_body_is_not_an_eighth_peer() {
        let centres = (0..CENTRE_COUNT)
            .map(|ordinal| CentreEmbodiment {
                ordinal: ordinal as u8,
                label: format!("centre-{ordinal}"),
                source: source(&format!("centre-{ordinal}")),
                amplitude: Some(ordinal as f64 / 7.0),
                phase_radians: Some(ordinal as f64 / 10.0),
                modes: Vec::new(),
                couplings: Vec::new(),
                body_zone_refs: Vec::new(),
                sense_refs: Vec::new(),
                action_refs: Vec::new(),
                feedback_refs: Vec::new(),
                standing: EvidenceStanding::Observed,
            })
            .collect();
        let field = EmbodiedField {
            reception_generation: 4,
            elemental_efwa: ElementalEfwa {
                earth: 0.1,
                fire: 0.2,
                water: 0.3,
                air: 0.4,
            },
            centres,
            earth_body: EarthBodyEmbodiment {
                source: source("earth-body"),
                frame_ref: "earth-fixed".into(),
                amplitude: Some(0.25),
                phase_radians: Some(0.5),
                relation_refs: vec!["relation:ground".into()],
                standing: EvidenceStanding::Derived,
            },
            nadi_refs: vec!["nadi:ida".into(), "nadi:pingala".into()],
            sushumna_ref: Some("nadi:sushumna".into()),
            temporal_astrology_refs: Vec::new(),
            materia_refs: Vec::new(),
            operation_refs: Vec::new(),
            consent: ConsentState::Granted,
        };
        field.validate().unwrap();
        assert_eq!(field.centres.len(), 7);
        assert_eq!(field.earth_body.frame_ref, "earth-fixed");
    }

    #[test]
    fn oracle_keeps_original_packet_when_interpretation_changes() {
        let original = OracleOriginalPacket {
            packet_ref: "oracle:1".into(),
            subject_id: "nara-a".into(),
            event_ref: "event:1".into(),
            profile_generation: 1,
            system: OracleSystem::IChingCoins,
            tradition_ref: "tradition:iching".into(),
            deck_or_method_ref: "coins".into(),
            spread_ref: None,
            query_ref: protected("query"),
            entropy: OracleEntropyReceipt {
                entropy_ref: protected("entropy"),
                method_ref: "os-csprng".into(),
                provider_ref: "provider:entropy".into(),
                observed_at_unix_ms: 1,
                evidence_refs: vec!["evidence:entropy".into()],
            },
            tokens: (0..6)
                .map(|position| OracleToken {
                    token_ref: format!("line:{position}"),
                    position,
                    reversed: false,
                    changing: position == 2,
                    source_refs: vec!["source:iching".into()],
                })
                .collect(),
            cast_degree: Some(123),
            cast_at_unix_ms: 2,
            vak_ref: Some("vak:M0-5".into()),
            original_payload_ref: protected("packet"),
            source_revisions: vec![source("iching")],
            consent: ConsentState::Granted,
            hygiene: OracleHygiene::Clear,
        };
        let mut record = OracleRecord {
            original: original.clone(),
            interpretations: Vec::new(),
        };
        record
            .add_reinterpretation(OracleInterpretation {
                interpretation_ref: "interpretation:later".into(),
                packet_ref: original.packet_ref.clone(),
                interpretation_revision: "r2".into(),
                model_ref: Some("model:2".into()),
                output_ref: protected("interpretation"),
                source_refs: vec!["source:commentary".into()],
                evidence_refs: Vec::new(),
                standing: EvidenceStanding::Derived,
                interpreted_at_unix_ms: 3,
            })
            .unwrap();
        record.validate().unwrap();
        assert_eq!(record.original, original);
    }

    #[test]
    fn transformation_preserves_source_defined_phase_ranges() {
        let history = TransformationHistory {
            history_ref: "history:1".into(),
            subject_id: "nara-a".into(),
            phases: vec![TransformationPhase {
                phase_ref: "phase:1".into(),
                storey: 11,
                decan: 2,
                stroke: 23,
                operation_refs: vec!["op:conjunction".into()],
                protocol_ref: "protocol:11:2".into(),
                container_ref: "container:temenos".into(),
                source_revisions: vec![source("transformation")],
                opened_at_unix_ms: 1,
                closed_at_unix_ms: Some(2),
                safety: TransformationSafety::Clear,
                feedback_refs: Vec::new(),
            }],
            standing: EvidenceStanding::Source,
        };
        history.validate().unwrap();
    }

    #[test]
    fn canonical_element_array_is_earth_fire_water_air() {
        let value = ElementalEfwa {
            earth: 1.0,
            fire: 2.0,
            water: 3.0,
            air: 4.0,
        };
        assert_eq!(value.as_array(), [1.0, 2.0, 3.0, 4.0]);
    }
}