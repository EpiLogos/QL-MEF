//! Dialogical Nara semantic contracts (TA3/TA4-contract slice of QL-MEF #201).
//!
//! QL owns Nara meaning, not speech transport: this module defines the bounded
//! dialogue context a caller (Actuation Nara Agency, O:I desktop) supplies, the
//! exact-ref deixis round trip, and the basis-bound Nara→Epii delegation. It
//! publishes no transport types and no provider names; voice stays a body that
//! AIKit resolves and O:I adapts. Speech-era identity is exact:
//!
//! ```text
//! NaraRef != AgentSessionRef != voice body != transport session
//! ```
//!
//! Disclosure is a fact, not an ambient right: sources and readings enter a
//! turn only when they were disclosed, selected, or are structural context.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::context_frame::ContextFrameId;
use crate::vak_profile::{
    ContentPosition, ContentType, ContextSequence, Participation, ThreadForm, constitutional_voice,
};

use super::domain::{EvidenceStanding, M4Branch};
use super::expression::BimbaSelectionBinding;

pub const NARA_DIALOGUE_CONTEXT_CONTRACT: &str = "ql.nara-dialogue-context/v1";
pub const NARA_DEIXIS_CONTRACT: &str = "ql.nara-deixis/v1";
pub const NARA_EPII_DELEGATION_CONTRACT: &str = "ql.nara-epii-delegation/v1";
pub const EPII_ENRICHMENT_CONTRACT: &str = "ql.epii-enrichment/v1";

/// Native development owner of commissioned work. Epii may propose a
/// commission here; Factory remains the execution owner.
pub const DEVELOPMENT_OWNER_REF: &str = "factory";

pub const MAX_DISCLOSED_REFS: usize = 256;
pub const MAX_PINNED_REFS: usize = 64;
pub const MAX_SELECTION_REFS: usize = 64;
pub const MAX_ACTION_REFS: usize = 256;
pub const MAX_SCOPE_REFS: usize = 256;
pub const MAX_CITED_REFS: usize = 256;
pub const MAX_PROPOSED_REFS: usize = 64;
pub const MAX_QUESTIONS: usize = 16;
pub const MAX_BRIEF_LEN: usize = 16_384;

fn text(value: &str, label: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.len() > 4096 || value.chars().any(char::is_control) {
        Err(format!("invalid {label}"))
    } else {
        Ok(())
    }
}

fn long_text(value: &str, label: &str, max: usize) -> Result<(), String> {
    if value.trim().is_empty() || value.len() > max || value.chars().any(char::is_control) {
        Err(format!("invalid {label}"))
    } else {
        Ok(())
    }
}

fn refs(values: &[String], label: &str, max: usize) -> Result<(), String> {
    if values.len() > max {
        return Err(format!("too many {label}"));
    }
    let mut unique = BTreeSet::new();
    for value in values {
        text(value, label)?;
        if !unique.insert(value.as_str()) {
            return Err(format!("duplicate {label}"));
        }
    }
    Ok(())
}

/// M focus of the dialogue turn. A focus refracts the same subject/coordinate/
/// occasion unless an explicit native Action changes state; selection alone
/// remains inspection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MFocus {
    M0,
    M1,
    M2,
    M3,
    M4,
    M5,
}

/// The accepted CF constitutional voice, carried as its spoken name because the
/// canonical `ContextFrameId` does not carry serde. `to_context_frame_id()`
/// round-trips into the accepted kernel identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DialogueFrame {
    Nous,
    Logos,
    Eros,
    Mythos,
    Anima,
    Psyche,
    Sophia,
}

impl DialogueFrame {
    pub const fn to_context_frame_id(self) -> ContextFrameId {
        match self {
            Self::Nous => ContextFrameId::Cf1,
            Self::Logos => ContextFrameId::Cf2,
            Self::Eros => ContextFrameId::Cf3,
            Self::Mythos => ContextFrameId::Cf4,
            Self::Anima => ContextFrameId::Cf5,
            Self::Psyche => ContextFrameId::Cf6,
            Self::Sophia => ContextFrameId::Cf7,
        }
    }

    /// The accepted constitutional voice name, read through the kernel's own
    /// mapping so this binding and `vak_profile` can never drift.
    pub const fn voice(self) -> &'static str {
        constitutional_voice(self.to_context_frame_id())
    }
}

/// The C′ compositional frame governing the act this turn participates in.
/// Field-for-letter per `TA-ONTA-VAK-COMPOSITIONAL-EXECUTION-LOCK.md`:
///
/// ```text
/// CPF participation · CT content_type · CP position
/// CF frame · CFP thread_form · CS sequence
/// ```
///
/// Values are the accepted field's own vocabulary; this binding only carries
/// them into the dialogue turn.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CPrimeDialogueBinding {
    /// CPF — participation regime (dialogical or authorised undertaking).
    pub participation: Participation,
    /// CT — content type of the material being worked.
    pub content_type: ContentType,
    /// CP — where the act stands in the developing whole.
    pub position: ContentPosition,
    /// CF — the constitutional voice organising the act.
    pub frame: DialogueFrame,
    /// CFP — thread form of plural work in performance.
    pub thread_form: ThreadForm,
    /// CS — the actual passage through the positions.
    pub sequence: ContextSequence,
    pub composition_ref: String,
    pub composition_revision: String,
}

impl CPrimeDialogueBinding {
    pub fn validate(&self) -> Result<(), String> {
        text(&self.composition_ref, "C′ composition reference")?;
        text(&self.composition_revision, "C′ composition revision")
    }
}

/// M4 Day/NOW enters the context only where admitted. `admitted_via_ref` names
/// the consent/authority receipt through which the personal occasion was let in.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdmittedOccasion {
    pub day_ref: String,
    pub day_revision: String,
    pub now_ref: String,
    pub now_revision: String,
    pub admitted_via_ref: String,
}

impl AdmittedOccasion {
    pub fn validate(&self) -> Result<(), String> {
        text(&self.day_ref, "Central Day reference")?;
        text(&self.day_revision, "Central Day revision")?;
        text(&self.now_ref, "Central NOW reference")?;
        text(&self.now_revision, "Central NOW revision")?;
        text(&self.admitted_via_ref, "occasion admission authority")
    }
}

/// How a source/reading actually entered Nara's context. Disclosure is
/// attributable to the operation that performed it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DisclosureKind {
    /// Khora entry reading (world/coordinate ground).
    KhoraEntry,
    /// Hen form/profile/source disclosure.
    HenDisclosure,
    /// Explicit personal consent over protected M4 state.
    PersonalConsent,
    /// SharedField projection after audience filtering.
    SharedProjection,
}

/// One disclosed source or reading: a fact about what Nara has actually been
/// given, with the receipt that disclosed it. Not an ambient right to source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DisclosedRef {
    pub ref_id: String,
    pub revision: String,
    pub standing: EvidenceStanding,
    pub disclosure: DisclosureKind,
    pub disclosed_via_ref: String,
}

impl DisclosedRef {
    pub fn validate(&self) -> Result<(), String> {
        text(&self.ref_id, "disclosed reference")?;
        text(&self.revision, "disclosed revision")?;
        text(&self.disclosed_via_ref, "disclosure receipt")
    }
}

/// SharedField relation of the turn, when the coordinate-space is shared. Only
/// safe projected refs travel; private Nara context and raw M4 state do not.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SharedFieldRelation {
    pub shared_field_ref: String,
    pub consent_ref: String,
    pub participant_subject_refs: Vec<String>,
}

impl SharedFieldRelation {
    pub fn validate(&self) -> Result<(), String> {
        text(&self.shared_field_ref, "SharedField reference")?;
        text(&self.consent_ref, "shared-presence consent reference")?;
        refs(
            &self.participant_subject_refs,
            "shared participant subject",
            64,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExpressiveActPhase {
    Composing,
    Active,
    Interrupted,
    Completed,
    Cancelled,
}

impl ExpressiveActPhase {
    pub const fn is_live(self) -> bool {
        matches!(self, Self::Composing | Self::Active)
    }
}

/// An authored, meaningful restore point on the act's own Expression. "Go
/// back" restores one of these; the contract has no bit-exact rewind surface.
/// The checkpoint remembers the Expression revision it was authored on, so a
/// restore is a named return to an authored state, never a GPU rewind claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExpressiveActCheckpoint {
    pub checkpoint_ref: String,
    pub checkpoint_expression_revision: String,
}

impl ExpressiveActCheckpoint {
    pub fn validate(&self) -> Result<(), String> {
        text(&self.checkpoint_ref, "checkpoint reference")?;
        text(
            &self.checkpoint_expression_revision,
            "checkpoint Expression revision",
        )
    }
}

/// Minimal attributable state of the turn's ExpressiveAct. The full operation
/// choreography stays with the O:I Expression substrate and TA0/TA2 grammar;
/// QL carries only what exact dialogue needs: identity, basis revision, the
/// opaque speech-turn handle, and the authored checkpoint. Interruption stops
/// both the voice response and stale pending choreography: the phase leaves
/// the live set and the speech-turn handle is released, so a non-live act
/// cannot still claim a speaking voice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExpressiveActState {
    pub expressive_act_ref: String,
    pub phase: ExpressiveActPhase,
    pub basis_expression_revision: String,
    /// Opaque handle of the speech turn coupled to this act. No transport or
    /// provider semantics ride on it. Live acts only.
    pub speech_turn_ref: Option<String>,
    /// The authored checkpoint this act offers for "go back", when one exists.
    pub checkpoint: Option<ExpressiveActCheckpoint>,
}

impl ExpressiveActState {
    pub fn validate(&self) -> Result<(), String> {
        text(&self.expressive_act_ref, "ExpressiveAct reference")?;
        text(
            &self.basis_expression_revision,
            "ExpressiveAct basis revision",
        )?;
        if let Some(handle) = &self.speech_turn_ref {
            text(handle, "speech turn reference")?;
        }
        if !self.phase.is_live() && self.speech_turn_ref.is_some() {
            return Err(
                "a non-live ExpressiveAct still claims a speech turn: interruption must stop the voice"
                    .into(),
            );
        }
        if let Some(checkpoint) = &self.checkpoint {
            checkpoint.validate()?;
        }
        Ok(())
    }

    /// Barge-in / manual stop. Interruption is only meaningful while the act
    /// is live; it ends the voice coupling immediately. Pending choreography
    /// is treated as stale from here on: safe atomic presentation operations
    /// may finish in the host, but nothing queued may apply to a later
    /// revision through this act.
    pub fn interrupt(&mut self) -> Result<(), String> {
        if !self.phase.is_live() {
            return Err(format!(
                "ExpressiveAct phase {} cannot be interrupted",
                serde_json::to_string(&self.phase).unwrap_or_default()
            ));
        }
        self.phase = ExpressiveActPhase::Interrupted;
        self.speech_turn_ref = None;
        self.validate()
    }

    /// Manual cancellation: the act ends without completion and without a
    /// voice claim.
    pub fn cancel(&mut self) -> Result<(), String> {
        if !self.phase.is_live() && self.phase != ExpressiveActPhase::Interrupted {
            return Err(format!(
                "ExpressiveAct phase {} cannot be cancelled",
                serde_json::to_string(&self.phase).unwrap_or_default()
            ));
        }
        self.phase = ExpressiveActPhase::Cancelled;
        self.speech_turn_ref = None;
        self.validate()
    }

    /// Resume the interrupted act against the live encounter. The act may only
    /// resume on the revision its remaining choreography was composed against;
    /// if the encounter moved on, the stale remainder is refused and a new act
    /// must be composed.
    pub fn resume(&mut self, current_expression_revision: &str) -> Result<(), String> {
        if self.phase != ExpressiveActPhase::Interrupted {
            return Err("only an interrupted ExpressiveAct can resume".into());
        }
        if self.basis_expression_revision != current_expression_revision {
            return Err(format!(
                "stale ExpressiveAct resume: composed against revision {} while the live encounter is at {}",
                self.basis_expression_revision, current_expression_revision
            ));
        }
        self.phase = ExpressiveActPhase::Active;
        self.validate()
    }

    /// "Go back": return the act to one of its authored checkpoints. The host
    /// must actually have moved the live Expression to the checkpoint's
    /// revision (passed here as `current_expression_revision`); the act then
    /// continues from that authored state. This is a named return to an
    /// authored state, never a bit-exact rewind.
    pub fn restore_checkpoint(
        &mut self,
        checkpoint_ref: &str,
        current_expression_revision: &str,
    ) -> Result<(), String> {
        let checkpoint = self
            .checkpoint
            .as_ref()
            .ok_or("ExpressiveAct carries no authored checkpoint to restore")?;
        if checkpoint.checkpoint_ref != checkpoint_ref {
            return Err(format!(
                "checkpoint {checkpoint_ref} is not authored on this ExpressiveAct"
            ));
        }
        if checkpoint.checkpoint_expression_revision != current_expression_revision {
            return Err(format!(
                "checkpoint restore does not match the live Expression: checkpoint was authored at revision {} while the encounter is at {}",
                checkpoint.checkpoint_expression_revision, current_expression_revision
            ));
        }
        self.basis_expression_revision = checkpoint.checkpoint_expression_revision.clone();
        self.phase = ExpressiveActPhase::Active;
        self.speech_turn_ref = None;
        self.validate()
    }
}

/// The bounded semantic context of one dialogical Nara turn.
///
/// A caller supplies exactly this; Nara reads exactly this. Everything in it is
/// derived from actual current state — renderer scraping is not part of the
/// contract, and undisclosed source is not part of the context.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NaraDialogueContext {
    pub schema: String,
    /// Identity of this bounded context instance. Delegations and turns answer
    /// this ref; two encounters over one Expression carry different context refs.
    pub context_ref: String,
    /// The foreground dialogical agent identity. Not a session, voice body, or
    /// transport handle.
    pub nara_ref: String,
    /// The current person/subject.
    pub subject_ref: String,
    /// The AgentSession carrying this turn. Must differ from `nara_ref`.
    pub agent_session_ref: String,
    /// M4/Bimba relation: branch standing and the accepted AW1 selection.
    pub m4_branch: Option<M4Branch>,
    pub coordinate_ref: String,
    pub bimba: Option<BimbaSelectionBinding>,
    /// Expression/Scene/Profile refs with exact revisions.
    pub expression_ref: String,
    pub expression_revision: String,
    pub profile_ref: String,
    pub profile_revision: String,
    pub scene_ref: Option<String>,
    /// Refraction focus of the same subject.
    pub active_m_focus: MFocus,
    /// Semantic selection: attention facts supplied by the host.
    pub pointed_ref: Option<String>,
    pub hovered_ref: Option<String>,
    pub pinned_refs: Vec<String>,
    /// The current occasion, where admitted.
    pub occasion: Option<AdmittedOccasion>,
    /// Sources/readings actually disclosed to Nara.
    pub disclosed: Vec<DisclosedRef>,
    /// Pleroma-resolved actions actually available here (refs only).
    pub available_action_refs: Vec<String>,
    pub c_prime: Option<CPrimeDialogueBinding>,
    pub shared_field: Option<SharedFieldRelation>,
    pub expressive_act: Option<ExpressiveActState>,
}

impl NaraDialogueContext {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != NARA_DIALOGUE_CONTEXT_CONTRACT {
            return Err("unsupported Nara dialogue context contract".into());
        }
        text(&self.context_ref, "dialogue context reference")?;
        text(&self.nara_ref, "Nara reference")?;
        text(&self.subject_ref, "Nara subject")?;
        text(&self.agent_session_ref, "AgentSession reference")?;
        if self.nara_ref == self.agent_session_ref {
            return Err("NaraRef and AgentSessionRef must remain distinct identities".into());
        }
        if let Some(branch) = self.m4_branch {
            if !self.coordinate_ref.starts_with(branch.coordinate()) {
                return Err("dialogue coordinate lies outside its M4 branch".into());
            }
        }
        if self.active_m_focus == MFocus::M4 && self.m4_branch.is_none() {
            return Err("M4 focus requires the M4 branch relation".into());
        }
        text(&self.coordinate_ref, "dialogue coordinate")?;
        text(&self.expression_ref, "Expression reference")?;
        text(&self.expression_revision, "Expression revision")?;
        text(&self.profile_ref, "profile reference")?;
        text(&self.profile_revision, "profile revision")?;
        if let Some(scene) = &self.scene_ref {
            text(scene, "scene reference")?;
        }
        if let Some(bimba) = &self.bimba {
            bimba.validate()?;
        }
        if let Some(pointed) = &self.pointed_ref {
            text(pointed, "pointed reference")?;
        }
        if let Some(hovered) = &self.hovered_ref {
            text(hovered, "hovered reference")?;
        }
        refs(&self.pinned_refs, "pinned reference", MAX_PINNED_REFS)?;
        if let Some(occasion) = &self.occasion {
            occasion.validate()?;
        }
        if self.disclosed.len() > MAX_DISCLOSED_REFS {
            return Err("too many disclosed references".into());
        }
        let mut disclosed = BTreeSet::new();
        for entry in &self.disclosed {
            entry.validate()?;
            if !disclosed.insert(entry.ref_id.as_str()) {
                return Err("duplicate disclosed reference".into());
            }
        }
        refs(
            &self.available_action_refs,
            "available action reference",
            MAX_ACTION_REFS,
        )?;
        if let Some(c_prime) = &self.c_prime {
            c_prime.validate()?;
        }
        if let Some(shared) = &self.shared_field {
            shared.validate()?;
        }
        if let Some(act) = &self.expressive_act {
            act.validate()?;
            if act.basis_expression_revision != self.expression_revision {
                return Err(
                    "active ExpressiveAct is not on the current Expression revision".into(),
                );
            }
        }
        Ok(())
    }

    /// Reconnect continuity. A provider or body change does not create another
    /// dialogue: the same Nara, subject, coordinate, Expression and profile
    /// standing means the same canonical dialogue continues. A new AgentSession
    /// or voice body is expected; a changed Nara, subject or Expression is a
    /// different encounter and refuses.
    pub fn continues(&self, previous: &NaraDialogueContext) -> Result<(), String> {
        previous.validate()?;
        self.validate()?;
        if self.nara_ref != previous.nara_ref {
            return Err("reconnect changed the Nara identity".into());
        }
        if self.subject_ref != previous.subject_ref {
            return Err("reconnect changed the subject".into());
        }
        if self.coordinate_ref != previous.coordinate_ref {
            return Err("reconnect changed the coordinate".into());
        }
        if self.expression_ref != previous.expression_ref {
            return Err("reconnect changed the Expression".into());
        }
        if self.profile_ref != previous.profile_ref
            || self.profile_revision != previous.profile_revision
        {
            return Err("reconnect changed the profile standing".into());
        }
        Ok(())
    }

    /// Structural context: refs the entry itself puts on the table. These are
    /// admitted because Khora/Hen disclosed them by constructing this context.
    fn is_structural(&self, ref_id: &str) -> bool {
        ref_id == self.coordinate_ref
            || ref_id == self.expression_ref
            || ref_id == self.profile_ref
            || self.scene_ref.as_deref() == Some(ref_id)
            || self.bimba.as_ref().is_some_and(|bimba| {
                bimba.selected_source_ref == ref_id
                    || bimba.direct_canonical_ref == ref_id
                    || bimba.conjugate_canonical_ref == ref_id
            })
    }

    fn is_selected(&self, ref_id: &str) -> bool {
        self.pointed_ref.as_deref() == Some(ref_id)
            || self.hovered_ref.as_deref() == Some(ref_id)
            || self.pinned_refs.iter().any(|pinned| pinned == ref_id)
    }

    /// A ref is admitted to the dialogue turn only when it is disclosed,
    /// selected, or structural. Disclosure is a fact carried by this context,
    /// never an ambient right.
    pub fn is_admitted(&self, ref_id: &str) -> bool {
        self.disclosed.iter().any(|entry| entry.ref_id == ref_id)
            || self.is_selected(ref_id)
            || self.is_structural(ref_id)
    }

    /// Admit a candidate set for one turn. Refusal names the offending ref; no
    /// silent narrowing and no silent widening.
    pub fn admit_refs(
        &self,
        candidates: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<Vec<String>, String> {
        let mut admitted = Vec::new();
        let mut unique = BTreeSet::new();
        for candidate in candidates {
            let candidate = candidate.as_ref();
            text(candidate, "turn candidate reference")?;
            if !self.is_admitted(candidate) {
                return Err(format!(
                    "reference {candidate} is not admitted to Nara context: it is neither disclosed, selected, nor structural"
                ));
            }
            if unique.insert(candidate.to_string()) {
                admitted.push(candidate.to_string());
            }
        }
        if admitted.len() > MAX_SCOPE_REFS {
            return Err("too many admitted turn references".into());
        }
        Ok(admitted)
    }

    /// The bounded turn context for a focus: structural anchors plus the
    /// admitted refs of the turn itself.
    pub fn turn_context(
        &self,
        turn_refs: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<BoundedTurnContext, String> {
        let turn_refs = self.admit_refs(turn_refs)?;
        Ok(BoundedTurnContext {
            coordinate_ref: self.coordinate_ref.clone(),
            expression_ref: self.expression_ref.clone(),
            expression_revision: self.expression_revision.clone(),
            turn_refs,
        })
    }

    /// Semantic deixis resolution. Human pointer/selection arrives already
    /// mapped to exact refs; Nara's spoken reference is resolved against its
    /// own bounded context only. The response carries exact refs the host UI
    /// can focus; nothing outside the context is ever invented.
    pub fn resolve_deixis(&self, request: &DeixisRequest) -> Result<DeixisResolution, String> {
        if request.schema != NARA_DEIXIS_CONTRACT {
            return Err("unsupported Nara deixis contract".into());
        }
        request.validate()?;
        if request.nara_ref != self.nara_ref {
            return Err("deixis request belongs to another Nara".into());
        }
        if request.basis_expression_revision != self.expression_revision {
            return Err(format!(
                "deixis request is stale: composed against Expression revision {} while the live context is at {}",
                request.basis_expression_revision, self.expression_revision
            ));
        }
        let outcome = match &request.target {
            DeicticTarget::Exact {
                ref_id,
                target_kind,
            } => {
                if !self.is_admitted(ref_id) {
                    DeixisOutcome::UnresolvedOutsideContext {
                        ref_id: ref_id.clone(),
                    }
                } else {
                    let focus = DeicticFocus {
                        ref_id: ref_id.clone(),
                        target_kind: Some(*target_kind),
                        focus_action: request.kind.default_focus_action(),
                    };
                    let turn_context = self.turn_context([ref_id])?;
                    DeixisOutcome::Focused {
                        focus: vec![focus],
                        turn_context,
                    }
                }
            }
            DeicticTarget::Spoken {
                phrase,
                candidate_refs,
            } => {
                if candidate_refs.is_empty() {
                    return Err(format!(
                        "spoken reference {phrase:?} resolved to no candidate and Nara does not invent refs"
                    ));
                }
                let admitted = self.admit_refs(candidate_refs.iter().map(String::as_str))?;
                let focus = admitted
                    .iter()
                    .map(|ref_id| DeicticFocus {
                        ref_id: ref_id.clone(),
                        target_kind: None,
                        focus_action: request.kind.default_focus_action(),
                    })
                    .collect();
                let turn_context = self.turn_context(admitted.iter().map(String::as_str))?;
                DeixisOutcome::Focused {
                    focus,
                    turn_context,
                }
            }
        };
        Ok(DeixisResolution {
            schema: NARA_DEIXIS_CONTRACT.into(),
            resolution_ref: format!("{}:resolution", request.deixis_ref),
            deixis_ref: request.deixis_ref.clone(),
            turn_ref: request.turn_ref.clone(),
            nara_ref: self.nara_ref.clone(),
            outcome,
        })
    }
}

/// The bounded context actually attached to one dialogue turn.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundedTurnContext {
    pub coordinate_ref: String,
    pub expression_ref: String,
    pub expression_revision: String,
    pub turn_refs: Vec<String>,
}

/// How the deictic target is given.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeicticKind {
    Pointed,
    Hovered,
    Pinned,
    Spoken,
}

impl DeicticKind {
    pub const fn default_focus_action(self) -> FocusAction {
        match self {
            Self::Hovered => FocusAction::Highlight,
            Self::Pointed | Self::Pinned | Self::Spoken => FocusAction::Select,
        }
    }
}

/// What the ref points at, as declared by the host for exact targets. Spoken
/// resolutions leave the kind to the host's default handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SemanticTargetKind {
    Subject,
    Coordinate,
    Relation,
    Source,
    Property,
    Scene,
    Expression,
    Profile,
    Page,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "target", rename_all = "kebab-case", deny_unknown_fields)]
pub enum DeicticTarget {
    Exact {
        ref_id: String,
        target_kind: SemanticTargetKind,
    },
    Spoken {
        phrase: String,
        /// Nara's own candidate resolutions from its bounded context. QL does
        /// not perform the matching; it enforces that every candidate is
        /// admitted and the response carries them exactly.
        candidate_refs: Vec<String>,
    },
}

/// Request: point at an exact subject/relation ref and receive the bounded
/// turn context for it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeixisRequest {
    pub schema: String,
    pub deixis_ref: String,
    pub nara_ref: String,
    pub turn_ref: String,
    pub basis_expression_revision: String,
    pub kind: DeicticKind,
    pub target: DeicticTarget,
    pub requested_at_unix_ms: u64,
}

impl DeixisRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != NARA_DEIXIS_CONTRACT {
            return Err("unsupported Nara deixis contract".into());
        }
        text(&self.deixis_ref, "deixis reference")?;
        text(&self.nara_ref, "deixis Nara reference")?;
        text(&self.turn_ref, "dialogue turn reference")?;
        text(&self.basis_expression_revision, "deixis basis revision")?;
        match &self.target {
            DeicticTarget::Exact { ref_id, .. } => {
                if self.kind == DeicticKind::Spoken {
                    return Err("a spoken deixis kind cannot carry an exact target".into());
                }
                text(ref_id, "exact deictic target")?;
            }
            DeicticTarget::Spoken {
                phrase,
                candidate_refs,
            } => {
                if self.kind != DeicticKind::Spoken {
                    return Err("an exact deictic target cannot carry the spoken kind".into());
                }
                long_text(phrase, "spoken phrase", 4096)?;
                refs(
                    candidate_refs,
                    "spoken candidate reference",
                    MAX_SELECTION_REFS,
                )?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FocusAction {
    Highlight,
    Select,
    Open,
}

/// One exact focus the host UI can act on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeicticFocus {
    pub ref_id: String,
    pub target_kind: Option<SemanticTargetKind>,
    pub focus_action: FocusAction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "kebab-case", deny_unknown_fields)]
pub enum DeixisOutcome {
    Focused {
        focus: Vec<DeicticFocus>,
        turn_context: BoundedTurnContext,
    },
    /// The named ref exists for the host but was never disclosed, selected,
    /// or structural for this Nara. The refusal is exact and named; no ref is
    /// invented in its place.
    UnresolvedOutsideContext { ref_id: String },
}

/// Response: exact refs the UI can focus, plus the bounded turn context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeixisResolution {
    pub schema: String,
    pub resolution_ref: String,
    pub deixis_ref: String,
    pub turn_ref: String,
    pub nara_ref: String,
    pub outcome: DeixisOutcome,
}

impl DeixisResolution {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != NARA_DEIXIS_CONTRACT {
            return Err("unsupported Nara deixis contract".into());
        }
        text(&self.resolution_ref, "deixis resolution reference")?;
        text(&self.deixis_ref, "deixis reference")?;
        text(&self.turn_ref, "dialogue turn reference")?;
        text(&self.nara_ref, "deixis Nara reference")?;
        match &self.outcome {
            DeixisOutcome::Focused {
                focus,
                turn_context,
            } => {
                if focus.is_empty() {
                    return Err("focused deixis resolution lost its focus set".into());
                }
                let mut unique = BTreeSet::new();
                for item in focus {
                    text(&item.ref_id, "deictic focus reference")?;
                    if !unique.insert(item.ref_id.as_str()) {
                        return Err("duplicate deictic focus reference".into());
                    }
                }
                turn_context.validate()?;
                for item in focus {
                    if !turn_context.turn_refs.contains(&item.ref_id) {
                        return Err("deictic focus is missing from its bounded turn context".into());
                    }
                }
            }
            DeixisOutcome::UnresolvedOutsideContext { ref_id } => {
                text(ref_id, "unresolved deictic reference")?;
            }
        }
        Ok(())
    }
}

impl BoundedTurnContext {
    pub fn validate(&self) -> Result<(), String> {
        text(&self.coordinate_ref, "turn coordinate")?;
        text(&self.expression_ref, "turn Expression reference")?;
        text(&self.expression_revision, "turn Expression revision")?;
        refs(&self.turn_refs, "turn reference", MAX_SCOPE_REFS)
    }
}

/// The basis an Epii result is bound to. What makes an enrichment
/// basis-bound rather than free-floating: the exact Expression revision,
/// profile revision, coordinate and registry revision it was produced
/// against, plus the context identity it answers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DelegationBasis {
    pub context_ref: String,
    pub expression_ref: String,
    pub expression_revision: String,
    pub profile_ref: String,
    pub profile_revision: String,
    pub coordinate_ref: String,
    pub bimba_registry_revision: String,
    /// Day/NOW travel only where they were admitted to the context.
    pub occasion: Option<AdmittedOccasion>,
}

impl DelegationBasis {
    pub fn validate(&self) -> Result<(), String> {
        text(&self.context_ref, "delegation context reference")?;
        text(&self.expression_ref, "delegation Expression reference")?;
        text(&self.expression_revision, "delegation Expression revision")?;
        text(&self.profile_ref, "delegation profile reference")?;
        text(&self.profile_revision, "delegation profile revision")?;
        text(&self.coordinate_ref, "delegation coordinate")?;
        text(
            &self.bimba_registry_revision,
            "delegation Bimba registry revision",
        )?;
        if let Some(occasion) = &self.occasion {
            occasion.validate()?;
        }
        Ok(())
    }

    fn matches_context(&self, context: &NaraDialogueContext) -> Result<(), String> {
        if self.expression_ref != context.expression_ref {
            return Err("delegation basis names another Expression".into());
        }
        if self.profile_ref != context.profile_ref
            || self.profile_revision != context.profile_revision
        {
            return Err("delegation basis is not on the current profile standing".into());
        }
        if self.coordinate_ref != context.coordinate_ref {
            return Err("delegation basis names another coordinate".into());
        }
        let registry = context
            .bimba
            .as_ref()
            .map(|bimba| bimba.registry_revision.as_str());
        if registry != Some(self.bimba_registry_revision.as_str()) {
            return Err("delegation basis is not on the current Bimba registry revision".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "kebab-case", deny_unknown_fields)]
pub enum DelegationState {
    Delegated,
    Returned { enrichment_ref: String },
    Withdrawn { reason: String },
}

/// Nara→Epii structured delegation. Epii remains canonical M5′ behind Nara;
/// the delegation hands exactly the admitted scope and nothing else.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EpiiDelegation {
    pub schema: String,
    pub delegation_ref: String,
    pub nara_ref: String,
    /// Epii's own AgentSession. Distinct from the delegating Nara.
    pub epii_session_ref: String,
    pub basis: DelegationBasis,
    pub brief: String,
    /// Exactly what Nara handed over: admitted refs only.
    pub scope_refs: Vec<String>,
    pub delegated_at_unix_ms: u64,
    pub state: DelegationState,
}

impl EpiiDelegation {
    /// Derive a delegation from the live dialogue context. The basis is bound
    /// by construction; the scope is admitted by the context itself, so an
    /// undisclosed ref can never be handed over.
    pub fn from_context(
        delegation_ref: String,
        context: &NaraDialogueContext,
        epii_session_ref: String,
        brief: String,
        scope_candidates: Vec<String>,
        delegated_at_unix_ms: u64,
    ) -> Result<Self, String> {
        context.validate()?;
        text(&delegation_ref, "delegation reference")?;
        text(&epii_session_ref, "Epii AgentSession reference")?;
        if epii_session_ref == context.nara_ref {
            return Err("Epii AgentSession must remain distinct from the delegating Nara".into());
        }
        long_text(&brief, "delegation brief", MAX_BRIEF_LEN)?;
        let scope_refs = context.admit_refs(scope_candidates.iter().map(String::as_str))?;
        let basis = DelegationBasis {
            context_ref: context.context_ref.clone(),
            expression_ref: context.expression_ref.clone(),
            expression_revision: context.expression_revision.clone(),
            profile_ref: context.profile_ref.clone(),
            profile_revision: context.profile_revision.clone(),
            coordinate_ref: context.coordinate_ref.clone(),
            bimba_registry_revision: context
                .bimba
                .as_ref()
                .map(|bimba| bimba.registry_revision.clone())
                .ok_or("dialogue context carries no Bimba selection to delegate against")?,
            occasion: context.occasion.clone(),
        };
        basis.validate()?;
        let delegation = Self {
            schema: NARA_EPII_DELEGATION_CONTRACT.into(),
            delegation_ref,
            nara_ref: context.nara_ref.clone(),
            epii_session_ref,
            basis,
            brief,
            scope_refs,
            delegated_at_unix_ms,
            state: DelegationState::Delegated,
        };
        delegation.validate()?;
        Ok(delegation)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema != NARA_EPII_DELEGATION_CONTRACT {
            return Err("unsupported Nara Epii delegation contract".into());
        }
        text(&self.delegation_ref, "delegation reference")?;
        text(&self.nara_ref, "delegating Nara reference")?;
        text(&self.epii_session_ref, "Epii AgentSession reference")?;
        if self.nara_ref == self.epii_session_ref {
            return Err("NaraRef and Epii AgentSessionRef must remain distinct".into());
        }
        self.basis.validate()?;
        long_text(&self.brief, "delegation brief", MAX_BRIEF_LEN)?;
        refs(
            &self.scope_refs,
            "delegation scope reference",
            MAX_SCOPE_REFS,
        )?;
        match &self.state {
            DelegationState::Delegated => {}
            DelegationState::Returned { enrichment_ref } => {
                text(enrichment_ref, "returned enrichment reference")?;
            }
            DelegationState::Withdrawn { reason } => {
                text(reason, "delegation withdrawal reason")?;
            }
        }
        Ok(())
    }

    /// Receive an enrichment against this delegation. The result is retained
    /// as returned material; receiving it applies nothing.
    pub fn receive_enrichment(&mut self, enrichment: &EpiiEnrichment) -> Result<(), String> {
        self.validate()?;
        enrichment.validate()?;
        if enrichment.delegation_ref != self.delegation_ref {
            return Err("enrichment answers another delegation".into());
        }
        if enrichment.basis_context_ref != self.basis.context_ref
            || enrichment.basis_expression_revision != self.basis.expression_revision
        {
            return Err("enrichment is not bound to this delegation's basis".into());
        }
        match &self.state {
            DelegationState::Delegated => {
                self.state = DelegationState::Returned {
                    enrichment_ref: enrichment.enrichment_ref.clone(),
                };
            }
            DelegationState::Returned { enrichment_ref } => {
                if enrichment_ref == &enrichment.enrichment_ref {
                    return Err("enrichment already received for this delegation".into());
                }
                return Err(
                    "delegation already returned an enrichment; a new answer needs a new delegation"
                        .into(),
                );
            }
            DelegationState::Withdrawn { .. } => {
                return Err("withdrawn delegation cannot receive enrichment".into());
            }
        }
        Ok(())
    }

    /// The application gate. Ok means the enrichment is live against this
    /// exact context and may be *proposed* to the owning human/Nara through
    /// current application operations. It never mutates anything itself, and
    /// a result produced against revision N cannot apply once the live
    /// encounter has moved to revision N+k.
    pub fn apply_gate(
        &self,
        enrichment: &EpiiEnrichment,
        current: &NaraDialogueContext,
    ) -> Result<(), String> {
        self.validate()?;
        enrichment.validate()?;
        current.validate()?;
        if enrichment.delegation_ref != self.delegation_ref {
            return Err("enrichment answers another delegation".into());
        }
        if current.nara_ref != self.nara_ref {
            return Err("application context belongs to another Nara".into());
        }
        self.basis.matches_context(current)?;
        if enrichment.basis_expression_revision != current.expression_revision {
            return Err(format!(
                "stale Epii enrichment: produced against Expression revision {} which can no longer auto-apply to live revision {}",
                enrichment.basis_expression_revision, current.expression_revision
            ));
        }
        for proposal in &enrichment.proposed_focus_refs {
            let in_scope = self.scope_refs.iter().any(|scope| scope == proposal);
            let cited = enrichment
                .coordinate_refs
                .iter()
                .any(|coordinate| coordinate == proposal);
            if !in_scope && !cited {
                return Err(format!(
                    "enrichment proposes focus {proposal} outside the delegated scope and its cited coordinates"
                ));
            }
        }
        Ok(())
    }
}

/// A developmental discrepancy Epii diagnosed, proposed to Factory as
/// commissioned work. This is a proposal only: Factory remains the execution
/// owner, and the proposal carries no authority to change any native owner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EpiiFactoryCommissionProposal {
    pub proposal_ref: String,
    /// What Epii diagnosed, in bounded plain language.
    pub discrepancy: String,
    /// Evidence/source/implementation refs backing the diagnosis.
    pub diagnosis_refs: Vec<String>,
    /// The native owner the work is proposed to. QL's law fixes this to
    /// Factory; execution stays with Factory and the owning human.
    pub proposed_owner_ref: String,
}

impl EpiiFactoryCommissionProposal {
    pub fn validate(&self) -> Result<(), String> {
        text(&self.proposal_ref, "Factory commission proposal reference")?;
        long_text(&self.discrepancy, "diagnosed discrepancy", MAX_BRIEF_LEN)?;
        refs(&self.diagnosis_refs, "diagnosis reference", MAX_CITED_REFS)?;
        if self.proposed_owner_ref != DEVELOPMENT_OWNER_REF {
            return Err(format!(
                "a developmental commission may only be proposed to {DEVELOPMENT_OWNER_REF}, which remains the execution owner"
            ));
        }
        Ok(())
    }
}

/// Structured Epii result: source-bearing, basis-bound, proposal-only.
/// Epii is not silently given the foreground voice; results return through
/// Nara (`nara_ref` on the delegation) to the owning human.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EpiiEnrichment {
    pub schema: String,
    pub enrichment_ref: String,
    pub delegation_ref: String,
    pub basis_context_ref: String,
    pub basis_expression_revision: String,
    pub coordinate_refs: Vec<String>,
    pub source_refs: Vec<String>,
    pub method_refs: Vec<String>,
    pub evidence_refs: Vec<String>,
    /// An enrichment is returned material; it may not inherit source standing.
    pub standing: EvidenceStanding,
    pub synthesis: String,
    pub proposed_focus_refs: Vec<String>,
    /// Proposed scene changes, as refs to host-side operations. QL carries the
    /// proposal, never the choreography.
    pub proposed_scene_change_refs: Vec<String>,
    pub proposed_profile_variant_ref: Option<String>,
    /// Proposed ExpressiveActs (speech + reversible choreography) Nara might
    /// perform, as refs.
    pub proposed_expressive_act_refs: Vec<String>,
    pub proposed_native_action_refs: Vec<String>,
    pub continuing_questions: Vec<String>,
    pub factory_commission_proposal: Option<EpiiFactoryCommissionProposal>,
    pub returned_at_unix_ms: u64,
}

impl EpiiEnrichment {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != EPII_ENRICHMENT_CONTRACT {
            return Err("unsupported Epii enrichment contract".into());
        }
        text(&self.enrichment_ref, "enrichment reference")?;
        text(&self.delegation_ref, "enrichment delegation reference")?;
        text(&self.basis_context_ref, "enrichment basis context")?;
        text(&self.basis_expression_revision, "enrichment basis revision")?;
        refs(
            &self.coordinate_refs,
            "enrichment coordinate reference",
            MAX_CITED_REFS,
        )?;
        refs(
            &self.source_refs,
            "enrichment source reference",
            MAX_CITED_REFS,
        )?;
        refs(
            &self.method_refs,
            "enrichment method reference",
            MAX_CITED_REFS,
        )?;
        refs(
            &self.evidence_refs,
            "enrichment evidence reference",
            MAX_CITED_REFS,
        )?;
        if self.standing == EvidenceStanding::Source {
            return Err("an Epii enrichment cannot claim source standing".into());
        }
        long_text(&self.synthesis, "enrichment synthesis", MAX_BRIEF_LEN)?;
        refs(
            &self.proposed_focus_refs,
            "enrichment focus proposal",
            MAX_PROPOSED_REFS,
        )?;
        refs(
            &self.proposed_scene_change_refs,
            "enrichment scene change proposal",
            MAX_PROPOSED_REFS,
        )?;
        if let Some(variant) = &self.proposed_profile_variant_ref {
            text(variant, "proposed profile variant reference")?;
        }
        refs(
            &self.proposed_expressive_act_refs,
            "enrichment ExpressiveAct proposal",
            MAX_PROPOSED_REFS,
        )?;
        refs(
            &self.proposed_native_action_refs,
            "enrichment native action proposal",
            MAX_PROPOSED_REFS,
        )?;
        if self.continuing_questions.len() > MAX_QUESTIONS {
            return Err("too many continuing questions".into());
        }
        for question in &self.continuing_questions {
            text(question, "continuing question")?;
        }
        if let Some(proposal) = &self.factory_commission_proposal {
            proposal.validate()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source_ref(ref_id: &str) -> DisclosedRef {
        DisclosedRef {
            ref_id: ref_id.into(),
            revision: "r1".into(),
            standing: EvidenceStanding::Source,
            disclosure: DisclosureKind::HenDisclosure,
            disclosed_via_ref: "hen:disclosure:1".into(),
        }
    }

    fn bimba() -> BimbaSelectionBinding {
        BimbaSelectionBinding {
            owner_contract_ref: "ql.aw1-rooted-m-world/v1".into(),
            registry_revision: "registry:r7".into(),
            selected_source_ref: "bimba:source:M4.1".into(),
            direct_canonical_ref: "bimba:#4".into(),
            conjugate_canonical_ref: "pratibimba:#4".into(),
        }
    }

    fn context() -> NaraDialogueContext {
        NaraDialogueContext {
            schema: NARA_DIALOGUE_CONTEXT_CONTRACT.into(),
            context_ref: "dialogue-context:1".into(),
            nara_ref: "nara:a".into(),
            subject_ref: "subject:a".into(),
            agent_session_ref: "session:1".into(),
            m4_branch: Some(M4Branch::Embodied),
            coordinate_ref: "M4.1.1".into(),
            bimba: Some(bimba()),
            expression_ref: "expression:1".into(),
            expression_revision: "rev-7".into(),
            profile_ref: "profile:M4.1".into(),
            profile_revision: "prof-2".into(),
            scene_ref: Some("scene:1".into()),
            active_m_focus: MFocus::M4,
            pointed_ref: Some("bimba:relation:1".into()),
            hovered_ref: None,
            pinned_refs: vec!["bimba:source:M4.1".into()],
            occasion: Some(AdmittedOccasion {
                day_ref: "central:day:2026-09-17".into(),
                day_revision: "d7".into(),
                now_ref: "central:now:abc".into(),
                now_revision: "n3".into(),
                admitted_via_ref: "consent:1".into(),
            }),
            disclosed: vec![source_ref("source:disclosed-1")],
            available_action_refs: vec!["action:expression.focus".into()],
            c_prime: Some(CPrimeDialogueBinding {
                composition_ref: "composition:1".into(),
                composition_revision: "c1".into(),
                participation: Participation::Dialogical,
                content_type: ContentType::DayNow,
                position: ContentPosition::Context,
                frame: DialogueFrame::Psyche,
                thread_form: ThreadForm::Single,
                sequence: ContextSequence::ContextFocused,
            }),
            shared_field: None,
            expressive_act: None,
        }
    }

    #[test]
    fn nara_and_agent_session_are_distinct_identities() {
        let mut value = context();
        value.validate().unwrap();
        value.agent_session_ref = value.nara_ref.clone();
        assert_eq!(
            value.validate().unwrap_err(),
            "NaraRef and AgentSessionRef must remain distinct identities"
        );
    }

    #[test]
    fn m4_focus_requires_the_branch_relation() {
        let mut value = context();
        value.m4_branch = None;
        assert_eq!(
            value.validate().unwrap_err(),
            "M4 focus requires the M4 branch relation"
        );
    }

    #[test]
    fn admission_covers_disclosed_selected_and_structural_only() {
        let value = context();
        value.validate().unwrap();
        assert!(value.is_admitted("source:disclosed-1"));
        assert!(value.is_admitted("bimba:relation:1"));
        assert!(value.is_admitted("bimba:source:M4.1"));
        assert!(value.is_admitted("expression:1"));
        assert!(value.is_admitted("bimba:#4"));
        assert!(value.is_admitted("pratibimba:#4"));
        assert!(!value.is_admitted("source:private-journal"));
        assert_eq!(
            value.admit_refs(["source:private-journal"]).unwrap_err(),
            "reference source:private-journal is not admitted to Nara context: it is neither disclosed, selected, nor structural"
        );
        let admitted = value
            .admit_refs([
                "source:disclosed-1",
                "bimba:relation:1",
                "source:disclosed-1",
            ])
            .unwrap();
        assert_eq!(admitted, vec!["source:disclosed-1", "bimba:relation:1"]);
    }

    #[test]
    fn deixis_round_trip_resolves_exact_and_spoken_refs() {
        let value = context();
        let pointed = DeixisRequest {
            schema: NARA_DEIXIS_CONTRACT.into(),
            deixis_ref: "deixis:1".into(),
            nara_ref: "nara:a".into(),
            turn_ref: "turn:1".into(),
            basis_expression_revision: "rev-7".into(),
            kind: DeicticKind::Pointed,
            target: DeicticTarget::Exact {
                ref_id: "bimba:relation:1".into(),
                target_kind: SemanticTargetKind::Relation,
            },
            requested_at_unix_ms: 10,
        };
        let resolution = value.resolve_deixis(&pointed).unwrap();
        resolution.validate().unwrap();
        let DeixisOutcome::Focused {
            focus,
            turn_context,
        } = resolution.outcome
        else {
            panic!("pointed target should focus");
        };
        assert_eq!(focus.len(), 1);
        assert_eq!(focus[0].ref_id, "bimba:relation:1");
        assert_eq!(focus[0].target_kind, Some(SemanticTargetKind::Relation));
        assert!(
            turn_context
                .turn_refs
                .contains(&"bimba:relation:1".to_string())
        );
        assert_eq!(turn_context.expression_revision, "rev-7");

        let spoken = DeixisRequest {
            schema: NARA_DEIXIS_CONTRACT.into(),
            deixis_ref: "deixis:2".into(),
            nara_ref: "nara:a".into(),
            turn_ref: "turn:1".into(),
            basis_expression_revision: "rev-7".into(),
            kind: DeicticKind::Spoken,
            target: DeicticTarget::Spoken {
                phrase: "that source relation".into(),
                candidate_refs: vec!["source:disclosed-1".into()],
            },
            requested_at_unix_ms: 11,
        };
        let resolution = value.resolve_deixis(&spoken).unwrap();
        resolution.validate().unwrap();
        assert!(matches!(resolution.outcome, DeixisOutcome::Focused { .. }));
    }

    #[test]
    fn deixis_never_invents_refs_outside_the_context() {
        let value = context();
        let outside = DeixisRequest {
            schema: NARA_DEIXIS_CONTRACT.into(),
            deixis_ref: "deixis:3".into(),
            nara_ref: "nara:a".into(),
            turn_ref: "turn:1".into(),
            basis_expression_revision: "rev-7".into(),
            kind: DeicticKind::Pointed,
            target: DeicticTarget::Exact {
                ref_id: "source:private-journal".into(),
                target_kind: SemanticTargetKind::Source,
            },
            requested_at_unix_ms: 12,
        };
        let resolution = value.resolve_deixis(&outside).unwrap();
        assert_eq!(
            resolution.outcome,
            DeixisOutcome::UnresolvedOutsideContext {
                ref_id: "source:private-journal".into()
            }
        );
        let spoken_outside = DeixisRequest {
            schema: NARA_DEIXIS_CONTRACT.into(),
            deixis_ref: "deixis:4".into(),
            nara_ref: "nara:a".into(),
            turn_ref: "turn:1".into(),
            basis_expression_revision: "rev-7".into(),
            kind: DeicticKind::Spoken,
            target: DeicticTarget::Spoken {
                phrase: "my private journal".into(),
                candidate_refs: vec!["source:private-journal".into()],
            },
            requested_at_unix_ms: 13,
        };
        assert!(value.resolve_deixis(&spoken_outside).is_err());
    }

    #[test]
    fn stale_deixis_is_refused() {
        let value = context();
        let request = DeixisRequest {
            schema: NARA_DEIXIS_CONTRACT.into(),
            deixis_ref: "deixis:5".into(),
            nara_ref: "nara:a".into(),
            turn_ref: "turn:1".into(),
            basis_expression_revision: "rev-6".into(),
            kind: DeicticKind::Pointed,
            target: DeicticTarget::Exact {
                ref_id: "bimba:relation:1".into(),
                target_kind: SemanticTargetKind::Relation,
            },
            requested_at_unix_ms: 14,
        };
        let error = value.resolve_deixis(&request).unwrap_err();
        assert!(error.starts_with("deixis request is stale"), "{error}");
    }

    fn enrichment(delegation: &str, basis_revision: &str) -> EpiiEnrichment {
        EpiiEnrichment {
            schema: EPII_ENRICHMENT_CONTRACT.into(),
            enrichment_ref: "enrichment:1".into(),
            delegation_ref: delegation.into(),
            basis_context_ref: "dialogue-context:1".into(),
            basis_expression_revision: basis_revision.into(),
            coordinate_refs: vec!["M4.1.1".into()],
            source_refs: vec!["source:disclosed-1".into()],
            method_refs: vec![],
            evidence_refs: vec!["evidence:1".into()],
            standing: EvidenceStanding::Derived,
            synthesis: "The disclosed source bears on the pointed relation.".into(),
            proposed_focus_refs: vec!["source:disclosed-1".into()],
            proposed_scene_change_refs: vec![],
            proposed_profile_variant_ref: None,
            proposed_expressive_act_refs: vec![],
            proposed_native_action_refs: vec![],
            continuing_questions: vec!["What does the conjugate face hold?".into()],
            factory_commission_proposal: None,
            returned_at_unix_ms: 20,
        }
    }

    #[test]
    fn delegation_scope_admits_disclosed_and_refuses_undisclosed() {
        let value = context();
        let delegation = EpiiDelegation::from_context(
            "delegation:1".into(),
            &value,
            "epii:session:1".into(),
            "What stands behind the pointed relation?".into(),
            vec!["source:disclosed-1".into(), "source:private-journal".into()],
            15,
        )
        .unwrap_err();
        assert!(delegation.contains("not admitted"), "{delegation}");

        let delegation = EpiiDelegation::from_context(
            "delegation:1".into(),
            &value,
            "epii:session:1".into(),
            "What stands behind the pointed relation?".into(),
            vec!["source:disclosed-1".into()],
            15,
        )
        .unwrap();
        assert_eq!(delegation.scope_refs, vec!["source:disclosed-1"]);
    }

    #[test]
    fn enrichment_is_basis_bound_and_refuses_stale_application() {
        let value = context();
        let mut delegation = EpiiDelegation::from_context(
            "delegation:1".into(),
            &value,
            "epii:session:1".into(),
            "deep question".into(),
            vec!["source:disclosed-1".into()],
            15,
        )
        .unwrap();
        let result = enrichment("delegation:1", "rev-7");
        delegation.receive_enrichment(&result).unwrap();

        // Live at the basis revision: the gate admits a proposal (it never
        // applies anything itself).
        delegation.apply_gate(&result, &value).unwrap();

        // The encounter moves on; the late result is retained material only.
        let mut moved = context();
        moved.expression_revision = "rev-8".into();
        let error = delegation.apply_gate(&result, &moved).unwrap_err();
        assert!(
            error.contains("stale Epii enrichment")
                && error.contains("rev-7")
                && error.contains("rev-8"),
            "{error}"
        );
    }

    #[test]
    fn enrichment_cannot_escape_the_delegated_scope() {
        let value = context();
        let delegation = EpiiDelegation::from_context(
            "delegation:1".into(),
            &value,
            "epii:session:1".into(),
            "deep question".into(),
            vec!["source:disclosed-1".into()],
            15,
        )
        .unwrap();
        let mut escaped = enrichment("delegation:1", "rev-7");
        escaped.proposed_focus_refs = vec!["profile:somewhere-else".into()];
        let error = delegation.apply_gate(&escaped, &value).unwrap_err();
        assert!(error.contains("outside the delegated scope"), "{error}");
    }

    #[test]
    fn enrichment_cannot_claim_source_standing() {
        let mut result = enrichment("delegation:1", "rev-7");
        result.standing = EvidenceStanding::Source;
        assert_eq!(
            result.validate().unwrap_err(),
            "an Epii enrichment cannot claim source standing"
        );
    }

    #[test]
    fn expressive_act_interrupt_and_checkpoint_restore() {
        let mut value = context();
        let mut act = ExpressiveActState {
            expressive_act_ref: "act:1".into(),
            phase: ExpressiveActPhase::Active,
            basis_expression_revision: "rev-7".into(),
            speech_turn_ref: Some("speech-turn:1".into()),
            checkpoint: Some(ExpressiveActCheckpoint {
                checkpoint_ref: "checkpoint:authored-1".into(),
                checkpoint_expression_revision: "rev-7".into(),
            }),
        };
        value.expressive_act = Some(act.clone());
        value.validate().unwrap();

        // Barge-in: the voice coupling is released and the act is no longer
        // live against the context.
        act.interrupt().unwrap();
        assert_eq!(act.phase, ExpressiveActPhase::Interrupted);
        assert!(act.speech_turn_ref.is_none());
        value.expressive_act = Some(act.clone());
        value.validate().unwrap();

        // A non-live act may not keep claiming a voice.
        let mut ghost = act.clone();
        ghost.speech_turn_ref = Some("speech-turn:still-alive".into());
        assert_eq!(
            ghost.validate().unwrap_err(),
            "a non-live ExpressiveAct still claims a speech turn: interruption must stop the voice"
        );

        // Resume against a moved encounter is stale and refused.
        let error = act.resume("rev-8").unwrap_err();
        assert!(error.contains("stale ExpressiveAct resume"), "{error}");
        act.resume("rev-7").unwrap();
        assert_eq!(act.phase, ExpressiveActPhase::Active);

        // "Go back" restores the authored checkpoint when the host actually
        // moved the live Expression to it.
        let mut wrong_host = act.clone();
        let error = wrong_host
            .restore_checkpoint("checkpoint:authored-1", "rev-6")
            .unwrap_err();
        assert!(
            error.contains("checkpoint restore does not match"),
            "{error}"
        );
        act.restore_checkpoint("checkpoint:authored-1", "rev-7")
            .unwrap();
        assert_eq!(act.basis_expression_revision, "rev-7");

        // A checkpoint that was never authored on this act cannot be restored.
        let error = act
            .restore_checkpoint("checkpoint:somewhere-else", "rev-7")
            .unwrap_err();
        assert!(
            error.contains("not authored on this ExpressiveAct"),
            "{error}"
        );

        // A stale act (produced against an older revision) cannot ride the
        // current context.
        let mut stale = context();
        stale.expression_revision = "rev-9".into();
        stale.expressive_act = Some(ExpressiveActState {
            expressive_act_ref: "act:2".into(),
            phase: ExpressiveActPhase::Active,
            basis_expression_revision: "rev-7".into(),
            speech_turn_ref: None,
            checkpoint: None,
        });
        assert_eq!(
            stale.validate().unwrap_err(),
            "active ExpressiveAct is not on the current Expression revision"
        );
    }

    #[test]
    fn reconnect_preserves_the_canonical_dialogue() {
        let previous = context();
        let mut reconnected = context();
        reconnected.context_ref = "dialogue-context:2".into();
        reconnected.agent_session_ref = "session:2".into();
        // Same Nara, subject, coordinate, Expression and profile: the same
        // dialogue continues across the body/session change.
        reconnected.continues(&previous).unwrap();

        let mut other_nara = context();
        other_nara.nara_ref = "nara:b".into();
        assert_eq!(
            other_nara.continues(&previous).unwrap_err(),
            "reconnect changed the Nara identity"
        );
        let mut moved = context();
        moved.expression_revision = "rev-8".into();
        moved.expression_ref = "expression:2".into();
        assert_eq!(
            moved.continues(&previous).unwrap_err(),
            "reconnect changed the Expression"
        );
    }

    #[test]
    fn factory_commission_stays_a_proposal_to_the_development_owner() {
        let mut result = enrichment("delegation:1", "rev-7");
        result.factory_commission_proposal = Some(EpiiFactoryCommissionProposal {
            proposal_ref: "commission:proposal:1".into(),
            discrepancy: "The Expression focus operation drops the conjugate face.".into(),
            diagnosis_refs: vec!["evidence:1".into(), "source:disclosed-1".into()],
            proposed_owner_ref: "somewhere-else".into(),
        });
        assert!(
            result
                .validate()
                .unwrap_err()
                .contains("may only be proposed to factory")
        );

        result
            .factory_commission_proposal
            .as_mut()
            .unwrap()
            .proposed_owner_ref = DEVELOPMENT_OWNER_REF.into();
        result.validate().unwrap();
    }

    #[test]
    fn c_prime_frame_round_trips_to_the_accepted_kernel_identity() {
        assert_eq!(
            DialogueFrame::Psyche.to_context_frame_id(),
            ContextFrameId::Cf6
        );
        assert_eq!(
            serde_json::to_string(&Participation::Dialogical).unwrap(),
            "\"dialogical\""
        );
        // Every named frame agrees with the kernel's own constitutional
        // voice mapping — the dialogue binding cannot drift from vak_profile.
        let frames = [
            (DialogueFrame::Nous, "Nous"),
            (DialogueFrame::Logos, "Logos"),
            (DialogueFrame::Eros, "Eros"),
            (DialogueFrame::Mythos, "Mythos"),
            (DialogueFrame::Anima, "Anima"),
            (DialogueFrame::Psyche, "Psyche"),
            (DialogueFrame::Sophia, "Sophia"),
        ];
        for (frame, name) in frames {
            assert_eq!(frame.voice(), name);
        }
    }
}
