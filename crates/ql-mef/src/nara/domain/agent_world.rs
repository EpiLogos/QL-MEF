//! K10 reception of the now-landed agent-world producer contracts.
//!
//! This is a join, not a replacement owner. AW1 keeps Bimba/property identity;
//! ql-wiki/Hen keeps Wiki participation; #94 keeps C′/Vāk semantics, operative
//! currentness and T/T′/Recognition projection; Factory/AIKit keep execution,
//! Resolve and native Recognition registration. Nara retains only the exact
//! source-qualified readings needed to relate those producer truths to one
//! accepted Personal occasion.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::aw1_world::{AW1_ROOTED_WORLD_CONTRACT, RootedMWorld};
use crate::nara::activity::NaraActivityLog;
use crate::nara::{PersonalFieldState, SourceRevision};
use crate::property::{Compact, PROPERTY_VERSION, PropertyOwner};
use crate::vak_performance::{PERFORMANCE_EVENT_CONTRACT, VakPerformanceEvent};
use crate::vak_scope::{
    CPrimeOperativeBinding, OPERATIVE_BINDING_CONTRACT, OperativeScopeObservation,
};

use super::Aw3ThoughtConsumptionReception;

pub const NARA_AGENT_WORLD_RECEPTION_CONTRACT: &str = "ql.nara-agent-world-reception/v1";
pub const NARA_BIMBA_KNOWLEDGE_CONTRACT: &str = "ql.nara-bimba-knowledge-reception/v1";
pub const NARA_OPERATIVE_RECEPTION_CONTRACT: &str = "ql.nara-operative-reception/v1";
/// ql-wiki depends on ql-mef, so ql-mef cannot import its Rust type without a
/// crate cycle. This exact contract identity is the transport seam for the real
/// ql-wiki `WikiParticipation`; Nara does not define a second Wiki object model.
pub const WIKI_PARTICIPATION_CONTRACT_REF: &str = "ql-mef/wiki-participation/v1";

fn text(value: &str, label: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.len() > 16_384 || value.contains('\0') {
        Err(format!("invalid {label}"))
    } else {
        Ok(())
    }
}

fn source(source: &SourceRevision) -> Result<(), String> {
    text(&source.source_ref, "source reference")?;
    text(&source.revision, "source revision")?;
    text(&source.standing_ref, "source standing")
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WikiParticipationRef {
    pub contract_ref: String,
    pub participation_ref: String,
    pub revision: u64,
    pub bimba_registry_revision: String,
    /// Canonical Bimba/Pratibimba member identities retained by ql-wiki.
    pub bimba_member_refs: BTreeSet<String>,
    /// Source lineage supplied by the ql-wiki owner; no Wiki body is copied.
    pub source_revisions: Vec<SourceRevision>,
}

impl WikiParticipationRef {
    pub fn validate(&self) -> Result<(), String> {
        if self.contract_ref != WIKI_PARTICIPATION_CONTRACT_REF {
            return Err("unsupported ql-wiki participation contract".into());
        }
        text(&self.participation_ref, "Wiki participation")?;
        if self.revision == 0 {
            return Err("Wiki participation revision must be positive".into());
        }
        text(
            &self.bimba_registry_revision,
            "Wiki Bimba registry revision",
        )?;
        if self.bimba_member_refs.is_empty() {
            return Err("Wiki participation must retain a Bimba member".into());
        }
        for value in &self.bimba_member_refs {
            text(value, "Wiki Bimba member")?;
        }
        if self.source_revisions.is_empty() {
            return Err("Wiki participation must retain source lineage".into());
        }
        for value in &self.source_revisions {
            source(value)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BimbaKnowledgeReception {
    pub schema: String,
    pub registry_revision: String,
    pub selected_id: crate::m_tree::MTreeId,
    pub selected_source_ref: String,
    pub direct_ref: String,
    pub conjugate_ref: String,
    pub properties: Vec<Compact>,
    pub wiki_participations: Vec<WikiParticipationRef>,
}

impl BimbaKnowledgeReception {
    pub fn from_landed(
        world: &RootedMWorld,
        properties: Vec<Compact>,
        wiki_participations: Vec<WikiParticipationRef>,
    ) -> Result<Self, String> {
        if world.version != AW1_ROOTED_WORLD_CONTRACT {
            return Err("unsupported rooted Bimba world contract".into());
        }
        text(&world.registry_revision, "Bimba registry revision")?;
        text(&world.selected_source_ref, "selected Bimba source")?;
        text(&world.direct.canonical_ref, "direct Bimba reference")?;
        text(&world.conjugate.canonical_ref, "conjugate Bimba reference")?;
        if world.direct.canonical_ref == world.conjugate.canonical_ref {
            return Err("Bimba and Pratibimba identities collapsed".into());
        }
        for property in &properties {
            if property.version != PROPERTY_VERSION {
                return Err("unsupported Bimba property contract".into());
            }
            if property.subject.registry_revision != world.registry_revision
                || property.subject.owner != PropertyOwner::Node
                || property.subject.id != world.selected_id
            {
                return Err("property is not about the selected rooted Bimba node".into());
            }
        }
        for participation in &wiki_participations {
            participation.validate()?;
            if participation.bimba_registry_revision != world.registry_revision {
                return Err("Wiki participation is from another Bimba registry revision".into());
            }
            if !participation
                .bimba_member_refs
                .contains(&world.direct.canonical_ref)
                && !participation
                    .bimba_member_refs
                    .contains(&world.conjugate.canonical_ref)
            {
                return Err("Wiki participation does not contain the selected Bimba pair".into());
            }
        }
        Ok(Self {
            schema: NARA_BIMBA_KNOWLEDGE_CONTRACT.into(),
            registry_revision: world.registry_revision.clone(),
            selected_id: world.selected_id,
            selected_source_ref: world.selected_source_ref.clone(),
            direct_ref: world.direct.canonical_ref.clone(),
            conjugate_ref: world.conjugate.canonical_ref.clone(),
            properties,
            wiki_participations,
        })
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema != NARA_BIMBA_KNOWLEDGE_CONTRACT {
            return Err("unsupported Nara Bimba knowledge contract".into());
        }
        text(&self.registry_revision, "Bimba registry revision")?;
        text(&self.selected_source_ref, "selected Bimba source")?;
        text(&self.direct_ref, "direct Bimba reference")?;
        text(&self.conjugate_ref, "conjugate Bimba reference")?;
        if self.direct_ref == self.conjugate_ref {
            return Err("Bimba and Pratibimba identities collapsed".into());
        }
        for property in &self.properties {
            if property.version != PROPERTY_VERSION
                || property.subject.registry_revision != self.registry_revision
                || property.subject.owner != PropertyOwner::Node
                || property.subject.id != self.selected_id
            {
                return Err("retained property no longer matches the selected Bimba node".into());
            }
        }
        for participation in &self.wiki_participations {
            participation.validate()?;
            if participation.bimba_registry_revision != self.registry_revision {
                return Err("retained Wiki participation is from another registry".into());
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "kebab-case", deny_unknown_fields)]
pub enum OperativeCurrentness {
    Current,
    Stale {
        observed_binding_ref: String,
        observed_binding_revision: String,
        differences: Vec<String>,
    },
    Missing {
        current_whole_ref: String,
        reason: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OperativePerformanceReception {
    pub schema: String,
    pub binding_ref: String,
    pub binding_revision: String,
    pub world_ref: String,
    pub world_generation: String,
    pub whole_ref: String,
    pub subject_ref: String,
    pub method_skill_ref: Option<String>,
    pub currentness: OperativeCurrentness,
    pub performance: Option<VakPerformanceEvent>,
}

impl OperativePerformanceReception {
    pub fn from_landed(
        binding: &CPrimeOperativeBinding,
        observation: &OperativeScopeObservation,
        performance: Option<&VakPerformanceEvent>,
    ) -> Result<Self, String> {
        if binding.contract != OPERATIVE_BINDING_CONTRACT {
            return Err("unsupported C-prime operative binding".into());
        }
        for (value, label) in [
            (&binding.binding_ref, "operative binding"),
            (&binding.binding_revision, "operative binding revision"),
            (&binding.world_ref, "operative world"),
            (&binding.world_generation, "operative world generation"),
            (&binding.whole_ref, "operative whole"),
            (&binding.subject_ref, "operative subject"),
        ] {
            text(value, label)?;
        }
        let currentness = match observation {
            OperativeScopeObservation::Current { binding: observed } => {
                if observed != binding {
                    return Err("Current operative observation differs from its binding".into());
                }
                OperativeCurrentness::Current
            }
            OperativeScopeObservation::Stale {
                observed,
                differences,
            } => {
                if differences.is_empty() {
                    return Err("stale operative observation requires explicit differences".into());
                }
                OperativeCurrentness::Stale {
                    observed_binding_ref: observed.binding_ref.clone(),
                    observed_binding_revision: observed.binding_revision.clone(),
                    differences: differences.clone(),
                }
            }
            OperativeScopeObservation::Missing {
                binding_ref,
                current_whole_ref,
                reason,
            } => {
                if binding_ref != &binding.binding_ref {
                    return Err("missing operative observation names another binding".into());
                }
                text(current_whole_ref, "current operative whole")?;
                text(reason, "missing operative reason")?;
                OperativeCurrentness::Missing {
                    current_whole_ref: current_whole_ref.clone(),
                    reason: reason.clone(),
                }
            }
        };
        if let Some(event) = performance {
            if event.contract != PERFORMANCE_EVENT_CONTRACT
                || event.ql_binding_ref != binding.binding_ref
                || event.ql_binding_revision != binding.binding_revision
                || event.factory.ql_binding_ref != binding.binding_ref
                || event.factory.ql_binding_revision != binding.binding_revision
                || event.factory.subject_ref != binding.subject_ref
                || event.factory.whole_ref != binding.whole_ref
            {
                return Err(
                    "Vāk performance is not the performed occasion of this operative binding"
                        .into(),
                );
            }
        }
        let receipt = Self {
            schema: NARA_OPERATIVE_RECEPTION_CONTRACT.into(),
            binding_ref: binding.binding_ref.clone(),
            binding_revision: binding.binding_revision.clone(),
            world_ref: binding.world_ref.clone(),
            world_generation: binding.world_generation.clone(),
            whole_ref: binding.whole_ref.clone(),
            subject_ref: binding.subject_ref.clone(),
            method_skill_ref: binding.method_skill_ref.clone(),
            currentness,
            performance: performance.cloned(),
        };
        receipt.validate()?;
        Ok(receipt)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema != NARA_OPERATIVE_RECEPTION_CONTRACT {
            return Err("unsupported Nara operative reception contract".into());
        }
        for (value, label) in [
            (&self.binding_ref, "operative binding"),
            (&self.binding_revision, "operative binding revision"),
            (&self.world_ref, "operative world"),
            (&self.world_generation, "operative world generation"),
            (&self.whole_ref, "operative whole"),
            (&self.subject_ref, "operative subject"),
        ] {
            text(value, label)?;
        }
        if let Some(method) = &self.method_skill_ref {
            text(method, "Method-classified Skill")?;
        }
        match &self.currentness {
            OperativeCurrentness::Current => {}
            OperativeCurrentness::Stale {
                observed_binding_ref,
                observed_binding_revision,
                differences,
            } => {
                text(observed_binding_ref, "observed operative binding")?;
                text(
                    observed_binding_revision,
                    "observed operative binding revision",
                )?;
                if differences.is_empty() {
                    return Err("stale operative reception lost its differences".into());
                }
            }
            OperativeCurrentness::Missing {
                current_whole_ref,
                reason,
            } => {
                text(current_whole_ref, "current operative whole")?;
                text(reason, "missing operative reason")?;
            }
        }
        if let Some(event) = &self.performance {
            if event.contract != PERFORMANCE_EVENT_CONTRACT
                || event.ql_binding_ref != self.binding_ref
                || event.ql_binding_revision != self.binding_revision
            {
                return Err("retained Vāk performance is detached from its binding".into());
            }
        }
        Ok(())
    }

    pub const fn is_current(&self) -> bool {
        matches!(self.currentness, OperativeCurrentness::Current)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NaraAgentWorldReception {
    pub schema: String,
    pub subject_id: String,
    pub personal_event_ref: String,
    pub personal_profile_generation: u64,
    pub day_ref: String,
    pub day_revision: String,
    pub now_ref: String,
    pub now_revision: String,
    pub bimba: BimbaKnowledgeReception,
    pub operative: OperativePerformanceReception,
    pub thought_consumptions: Vec<Aw3ThoughtConsumptionReception>,
}

impl NaraAgentWorldReception {
    pub fn from_personal_state(
        personal: &PersonalFieldState,
        activity: &NaraActivityLog,
        bimba: BimbaKnowledgeReception,
        operative: OperativePerformanceReception,
        thought_consumptions: Vec<Aw3ThoughtConsumptionReception>,
    ) -> Result<Self, String> {
        activity.validate()?;
        bimba.validate()?;
        operative.validate()?;
        if personal.subject_id != activity.subject_id
            || personal.subject_id != operative.subject_ref
        {
            return Err("Personal, activity and operative subjects are not the same Nara".into());
        }
        if personal.event.registry_revision != bimba.registry_revision {
            return Err(
                "Personal occasion and Bimba selection use different registry revisions".into(),
            );
        }
        if personal.event.event_ref != operative.world_ref {
            return Err(
                "operative scope is not correlated to the accepted Personal world event".into(),
            );
        }
        for thought in &thought_consumptions {
            thought.validate()?;
            if thought.subject_id != personal.subject_id
                || thought.day_ref != activity.temporal.day_ref
                || thought.day_revision != activity.temporal.day_revision
                || thought.now_ref != activity.temporal.now_ref
                || thought.now_revision != activity.temporal.now_revision
            {
                return Err("AW3 thought reception is not in this Nara's current Day/NOW".into());
            }
            if let Some(performance_ref) = &thought.performance_ref {
                let Some(performance) = &operative.performance else {
                    return Err(
                        "performance-bound AW3 reading has no retained Vāk performance".into(),
                    );
                };
                if performance_ref != &performance.performance_ref {
                    return Err("AW3 reading and Vāk performance name different occasions".into());
                }
            }
            for candidate in &thought.recognition_candidates {
                if let Some(performance_ref) = &candidate.performance_ref {
                    let Some(performance) = &operative.performance else {
                        return Err(
                            "performance-bound Recognition candidate has no Vāk performance".into(),
                        );
                    };
                    if performance_ref != &performance.performance_ref {
                        return Err(
                            "Recognition candidate is detached from its performed occasion".into(),
                        );
                    }
                }
            }
        }
        let receipt = Self {
            schema: NARA_AGENT_WORLD_RECEPTION_CONTRACT.into(),
            subject_id: personal.subject_id.clone(),
            personal_event_ref: personal.event.event_ref.clone(),
            personal_profile_generation: personal.event.profile_generation,
            day_ref: activity.temporal.day_ref.clone(),
            day_revision: activity.temporal.day_revision.clone(),
            now_ref: activity.temporal.now_ref.clone(),
            now_revision: activity.temporal.now_revision.clone(),
            bimba,
            operative,
            thought_consumptions,
        };
        receipt.validate()?;
        Ok(receipt)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema != NARA_AGENT_WORLD_RECEPTION_CONTRACT {
            return Err("unsupported Nara agent-world reception contract".into());
        }
        for (value, label) in [
            (&self.subject_id, "Nara subject"),
            (&self.personal_event_ref, "Personal world event"),
            (&self.day_ref, "Central Day"),
            (&self.day_revision, "Central Day revision"),
            (&self.now_ref, "Central NOW"),
            (&self.now_revision, "Central NOW revision"),
        ] {
            text(value, label)?;
        }
        self.bimba.validate()?;
        self.operative.validate()?;
        if self.operative.subject_ref != self.subject_id
            || self.operative.world_ref != self.personal_event_ref
        {
            return Err(
                "retained operative reception is no longer on this Personal occasion".into(),
            );
        }
        for thought in &self.thought_consumptions {
            thought.validate()?;
            if thought.subject_id != self.subject_id
                || thought.day_ref != self.day_ref
                || thought.day_revision != self.day_revision
                || thought.now_ref != self.now_ref
                || thought.now_revision != self.now_revision
            {
                return Err("retained AW3 reading is no longer on this Nara Day/NOW".into());
            }
        }
        Ok(())
    }

    pub fn recognition_candidates(
        &self,
    ) -> impl Iterator<Item = &crate::vak_thought_consumption::RecognitionCandidate> {
        self.thought_consumptions
            .iter()
            .flat_map(|reading| reading.recognition_candidates.iter())
    }
}
