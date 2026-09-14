//! Nara → O:I Expression projection contract.
//!
//! O:I owns the reusable Expression Stage, targets, library, rendering and any
//! compatible authoring envelope such as `oi.journey/1`. QL-MEF publishes a
//! protected, source-qualified domain projection rather than a second renderer,
//! library or portable artifact format.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::aw1_world::{AW1_ROOTED_WORLD_CONTRACT, RootedMWorld};

use super::domain::{EvidenceStanding, M4Branch, ProtectedRef};
use super::replay::NaraOccasion;
use super::SourceRevision;

pub const NARA_EXPRESSION_PROJECTION_CONTRACT: &str = "ql.nara-expression-projection/v1";
pub const EXPRESSION_HOST_OWNER: &str = "oi";

fn text(value: &str, label: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.len() > 4096 || value.chars().any(char::is_control) {
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

fn source(source: &SourceRevision) -> Result<(), String> {
    text(&source.source_ref, "expression source")?;
    text(&source.revision, "expression source revision")?;
    text(&source.standing_ref, "expression source standing")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExpressionIntent {
    Identity,
    Embodied,
    Oracle,
    Transformation,
    Context,
    Integration,
    WholePersonalField,
    Replay,
}

impl ExpressionIntent {
    pub const fn branch(self) -> Option<M4Branch> {
        match self {
            Self::Identity => Some(M4Branch::Identity),
            Self::Embodied => Some(M4Branch::Embodied),
            Self::Oracle => Some(M4Branch::Oracle),
            Self::Transformation => Some(M4Branch::Transformation),
            Self::Context => Some(M4Branch::Context),
            Self::Integration => Some(M4Branch::Integration),
            Self::WholePersonalField | Self::Replay => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExpressionOccasionMode {
    Live,
    OriginalReplay,
    Reinterpretation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExpressionDisclosure {
    PrivateLocal,
    SharedPresence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExpressionTargetRef {
    pub owner_ref: String,
    pub target_ref: String,
    pub host_contract_ref: String,
}

impl ExpressionTargetRef {
    pub fn validate(&self) -> Result<(), String> {
        text(&self.owner_ref, "Expression target owner")?;
        if self.owner_ref != EXPRESSION_HOST_OWNER && !self.owner_ref.starts_with("oi:") {
            return Err("Nara Expressions must target the O:I-owned expression surface".into());
        }
        text(&self.target_ref, "Expression target")?;
        text(&self.host_contract_ref, "Expression host contract")
    }
}

/// Compact consumption of #94/AW1's accepted rooted Bimba selection. The graph
/// and full `RootedMWorld` remain with their owner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BimbaSelectionBinding {
    pub owner_contract_ref: String,
    pub registry_revision: String,
    pub selected_source_ref: String,
    pub direct_canonical_ref: String,
    pub conjugate_canonical_ref: String,
}

impl BimbaSelectionBinding {
    pub fn from_rooted_world(world: &RootedMWorld) -> Result<Self, String> {
        if world.version != AW1_ROOTED_WORLD_CONTRACT {
            return Err("unsupported rooted Bimba world contract".into());
        }
        let binding = Self {
            owner_contract_ref: world.version.clone(),
            registry_revision: world.registry_revision.clone(),
            selected_source_ref: world.selected_source_ref.clone(),
            direct_canonical_ref: world.direct.canonical_ref.clone(),
            conjugate_canonical_ref: world.conjugate.canonical_ref.clone(),
        };
        binding.validate()?;
        Ok(binding)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.owner_contract_ref != AW1_ROOTED_WORLD_CONTRACT {
            return Err("Bimba selection is not the accepted AW1 rooted-world contract".into());
        }
        text(&self.registry_revision, "Bimba registry revision")?;
        text(&self.selected_source_ref, "Bimba selected source")?;
        text(
            &self.direct_canonical_ref,
            "Bimba direct canonical reference",
        )?;
        text(
            &self.conjugate_canonical_ref,
            "Bimba conjugate canonical reference",
        )?;
        if self.direct_canonical_ref == self.conjugate_canonical_ref {
            return Err("Bimba direct/conjugate faces were collapsed".into());
        }
        Ok(())
    }
}

/// Same-session seam for the #94-owned Epii/M5 operation. K10 keeps the exact
/// session/Return identity but does not implement another M5 or Recognition loop.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EpiiSessionBinding {
    pub owner_contract_ref: String,
    pub agent_session_ref: String,
    pub operation_return_refs: Vec<String>,
}

impl EpiiSessionBinding {
    pub fn validate(&self) -> Result<(), String> {
        text(&self.owner_contract_ref, "Epii owner contract")?;
        text(&self.agent_session_ref, "Epii AgentSession reference")?;
        refs(
            &self.operation_return_refs,
            "Epii operation Return reference",
            4096,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SharedPresenceConsent {
    pub consent_ref: ProtectedRef,
    pub participant_subjects: Vec<String>,
    pub allowed_expression_refs: Vec<String>,
    pub allowed_target_refs: Vec<String>,
    pub granted_at_unix_ms: u64,
    pub expires_at_unix_ms: Option<u64>,
}

impl SharedPresenceConsent {
    pub fn validate(&self) -> Result<(), String> {
        self.consent_ref.validate()?;
        refs(
            &self.participant_subjects,
            "shared-presence participant",
            64,
        )?;
        if self.participant_subjects.len() < 2 {
            return Err("shared presence requires at least two consented participants".into());
        }
        refs(
            &self.allowed_expression_refs,
            "shared-presence expression",
            4096,
        )?;
        if self.allowed_expression_refs.is_empty() {
            return Err("shared presence requires an explicit expression allow-list".into());
        }
        refs(&self.allowed_target_refs, "shared-presence target", 256)?;
        if self.allowed_target_refs.is_empty() {
            return Err("shared presence requires an explicit target allow-list".into());
        }
        if self
            .expires_at_unix_ms
            .is_some_and(|expires| expires <= self.granted_at_unix_ms)
        {
            return Err("shared-presence consent expiry must follow grant time".into());
        }
        Ok(())
    }

    pub fn permits(
        &self,
        subject_id: &str,
        expression_ref: &str,
        target_ref: &str,
        at_unix_ms: u64,
    ) -> bool {
        let not_expired = match self.expires_at_unix_ms {
            Some(expires) => at_unix_ms < expires,
            None => true,
        };
        self.participant_subjects
            .iter()
            .any(|subject| subject == subject_id)
            && self
                .allowed_expression_refs
                .iter()
                .any(|allowed| allowed == expression_ref)
            && self
                .allowed_target_refs
                .iter()
                .any(|allowed| allowed == target_ref)
            && not_expired
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NaraExpressionProjection {
    pub schema: String,
    pub expression_ref: String,
    pub subject_id: String,
    pub occasion_ref: String,
    pub event_ref: String,
    pub profile_generation: u64,
    pub personal_reception_generation: u64,
    pub intent: ExpressionIntent,
    pub m4_coordinate_ref: String,
    pub occasion_mode: ExpressionOccasionMode,
    pub target: ExpressionTargetRef,
    pub protected_state_ref: ProtectedRef,
    pub constituent_refs: Vec<String>,
    pub selection_refs: Vec<String>,
    pub source_revisions: Vec<SourceRevision>,
    pub bimba_selection: Option<BimbaSelectionBinding>,
    pub epii_session: Option<EpiiSessionBinding>,
    pub host_artifact_refs: Vec<String>,
    pub cue_refs: Vec<String>,
    pub disclosure: ExpressionDisclosure,
    pub standing: EvidenceStanding,
}

impl NaraExpressionProjection {
    pub fn from_occasion(
        expression_ref: String,
        occasion: &NaraOccasion,
        intent: ExpressionIntent,
        m4_coordinate_ref: String,
        occasion_mode: ExpressionOccasionMode,
        target: ExpressionTargetRef,
        protected_state_ref: ProtectedRef,
    ) -> Result<Self, String> {
        let projection = Self {
            schema: NARA_EXPRESSION_PROJECTION_CONTRACT.into(),
            expression_ref,
            subject_id: occasion.subject_id.clone(),
            occasion_ref: occasion.occasion_ref.clone(),
            event_ref: occasion.event.event_ref.clone(),
            profile_generation: occasion.event.profile_generation,
            personal_reception_generation: occasion.personal_reception_generation,
            intent,
            m4_coordinate_ref,
            occasion_mode,
            target,
            protected_state_ref,
            constituent_refs: Vec::new(),
            selection_refs: Vec::new(),
            source_revisions: occasion.source_revisions.clone(),
            bimba_selection: None,
            epii_session: None,
            host_artifact_refs: Vec::new(),
            cue_refs: Vec::new(),
            disclosure: ExpressionDisclosure::PrivateLocal,
            standing: EvidenceStanding::Derived,
        };
        projection.validate()?;
        Ok(projection)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema != NARA_EXPRESSION_PROJECTION_CONTRACT {
            return Err("unsupported Nara Expression projection contract".into());
        }
        text(&self.expression_ref, "Expression reference")?;
        text(&self.subject_id, "Expression subject")?;
        text(&self.occasion_ref, "Expression occasion")?;
        text(&self.event_ref, "Expression event")?;
        text(&self.m4_coordinate_ref, "Expression M4 coordinate")?;
        if let Some(branch) = self.intent.branch()
            && !self.m4_coordinate_ref.starts_with(branch.coordinate())
        {
            return Err("Expression intent does not match its M4 coordinate".into());
        }
        self.target.validate()?;
        self.protected_state_ref.validate()?;
        refs(
            &self.constituent_refs,
            "Expression constituent reference",
            4096,
        )?;
        refs(
            &self.selection_refs,
            "Expression selection reference",
            4096,
        )?;
        if self.source_revisions.is_empty() || self.source_revisions.len() > 4096 {
            return Err("Expression projection requires 1..4096 source revisions".into());
        }
        for revision in &self.source_revisions {
            source(revision)?;
        }
        if let Some(binding) = &self.bimba_selection {
            binding.validate()?;
        }
        if let Some(binding) = &self.epii_session {
            binding.validate()?;
        }
        refs(
            &self.host_artifact_refs,
            "Expression host artifact reference",
            256,
        )?;
        refs(&self.cue_refs, "Expression cue reference", 256)
    }

    pub fn bind_bimba(mut self, world: &RootedMWorld) -> Result<Self, String> {
        self.validate()?;
        let binding = BimbaSelectionBinding::from_rooted_world(world)?;
        if self
            .source_revisions
            .iter()
            .all(|revision| revision.revision != world.registry_revision)
        {
            self.constituent_refs
                .push(format!("registry:{}", world.registry_revision));
        }
        self.selection_refs
            .push(binding.selected_source_ref.clone());
        self.bimba_selection = Some(binding);
        self.validate()?;
        Ok(self)
    }

    pub fn bind_epii_session(mut self, binding: EpiiSessionBinding) -> Result<Self, String> {
        binding.validate()?;
        self.epii_session = Some(binding);
        self.validate()?;
        Ok(self)
    }

    pub fn share_with(
        mut self,
        consent: &SharedPresenceConsent,
        at_unix_ms: u64,
    ) -> Result<Self, String> {
        self.validate()?;
        consent.validate()?;
        if !consent.permits(
            &self.subject_id,
            &self.expression_ref,
            &self.target.target_ref,
            at_unix_ms,
        ) {
            return Err(
                "shared-presence consent does not permit this Expression projection".into(),
            );
        }
        self.disclosure = ExpressionDisclosure::SharedPresence;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aw1_world::resolve_rooted_m_world;
    use crate::m_tree::native_m_registry;
    use crate::nara::EventBasisRefs;

    fn source_ref() -> SourceRevision {
        SourceRevision {
            source_ref: "source:m4".into(),
            revision: "r1".into(),
            standing_ref: "source".into(),
        }
    }

    fn protected(name: &str) -> ProtectedRef {
        ProtectedRef {
            ref_id: format!("central:protected:{name}"),
            revision: "r1".into(),
            owner_ref: "central".into(),
        }
    }

    fn occasion() -> NaraOccasion {
        NaraOccasion {
            occasion_ref: "occasion:1".into(),
            subject_id: "nara-a".into(),
            event: EventBasisRefs {
                event_ref: "event:1".into(),
                subject_ref: "nara-a".into(),
                profile_generation: 1,
                registry_revision: "registry:r1".into(),
                m1_revision: "m1:r1".into(),
                m2_source_ref: "m2:source".into(),
                m2_contract_ref: "m2:contract".into(),
                m3_source_ref: "m3:source".into(),
                m3_contract_ref: "m3:contract".into(),
            },
            personal_reception_generation: 1,
            identity_revision: "identity:r1".into(),
            day_ref: "central:day:1".into(),
            now_ref: "central:now:1".into(),
            occurrence_at_unix_ms: 1,
            receipt_at_unix_ms: 1,
            protected_state_ref: protected("whole-state"),
            activity_refs: Vec::new(),
            oracle_packet_refs: Vec::new(),
            transformation_phase_refs: Vec::new(),
            context_reading_refs: Vec::new(),
            integration_return_refs: Vec::new(),
            expression_refs: Vec::new(),
            source_revisions: vec![source_ref()],
        }
    }

    fn projection(expression_ref: &str) -> NaraExpressionProjection {
        NaraExpressionProjection::from_occasion(
            expression_ref.into(),
            &occasion(),
            ExpressionIntent::Embodied,
            "M4.1.1".into(),
            ExpressionOccasionMode::Live,
            ExpressionTargetRef {
                owner_ref: "oi".into(),
                target_ref: "viewport".into(),
                host_contract_ref: "oi.expression-stage/current".into(),
            },
            protected("embodied-state"),
        )
        .unwrap()
    }

    #[test]
    fn projection_contains_refs_not_raw_personal_state() {
        let encoded = serde_json::to_string(&projection("expression:1")).unwrap();
        assert!(!encoded.contains("q_composed"));
        assert!(!encoded.contains("aggregate_resonance"));
        assert!(!encoded.contains("journal_bytes"));
        assert!(encoded.contains("central:protected:embodied-state"));
    }

    #[test]
    fn accepted_aw1_selection_is_consumed_without_a_second_graph_store() {
        let world = resolve_rooted_m_world(native_m_registry(), "#4").unwrap();
        let value = projection("expression:1").bind_bimba(&world).unwrap();
        let binding = value.bimba_selection.unwrap();
        assert_eq!(binding.owner_contract_ref, AW1_ROOTED_WORLD_CONTRACT);
        assert_eq!(binding.selected_source_ref, world.selected_source_ref);
        assert_eq!(binding.direct_canonical_ref, world.direct.canonical_ref);
        assert_eq!(
            binding.conjugate_canonical_ref,
            world.conjugate.canonical_ref
        );
    }

    #[test]
    fn shared_presence_is_explicitly_allow_listed() {
        let consent = SharedPresenceConsent {
            consent_ref: protected("share-consent"),
            participant_subjects: vec!["nara-a".into(), "nara-b".into()],
            allowed_expression_refs: vec!["expression:1".into()],
            allowed_target_refs: vec!["viewport".into()],
            granted_at_unix_ms: 10,
            expires_at_unix_ms: Some(100),
        };
        let shared = projection("expression:1").share_with(&consent, 50).unwrap();
        assert_eq!(shared.disclosure, ExpressionDisclosure::SharedPresence);
        assert!(projection("expression:2").share_with(&consent, 50).is_err());
    }
}
