use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::{MetaKnowledgeProjection, MetaProvenance, ProjectedRelation};

pub const META_RECOGNITION_CONTRACT: &str = "ql-mef/meta-recognition/v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecognitionError {
    InvalidObservation(String),
    DuplicateConflict(String),
    UnknownCandidate(String),
    AlreadyDecided(String),
    InvalidDecision(String),
    MissingMetaRef(String),
}

impl core::fmt::Display for RecognitionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidObservation(value) => write!(f, "invalid recognition observation: {value}"),
            Self::DuplicateConflict(value) => {
                write!(f, "observation already proposed with different content: {value}")
            }
            Self::UnknownCandidate(value) => write!(f, "unknown amendment candidate {value}"),
            Self::AlreadyDecided(value) => write!(f, "amendment candidate already decided {value}"),
            Self::InvalidDecision(value) => write!(f, "invalid recognition decision: {value}"),
            Self::MissingMetaRef(value) => write!(f, "recognised amendment references unknown meta ref {value}"),
        }
    }
}

impl std::error::Error for RecognitionError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RecognitionState {
    Proposed,
    Recognised,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalObservation {
    pub observation_ref: String,
    pub source_provider_ref: String,
    pub source_target_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_revision: Option<String>,
    pub suggested_from_meta_ref: String,
    pub suggested_to_meta_ref: String,
    pub suggested_relation: String,
    pub rationale: String,
    pub occurrences: u32,
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    #[serde(default)]
    pub extensions: Map<String, Value>,
}

impl ExternalObservation {
    pub fn validate(&self) -> Result<(), RecognitionError> {
        for (field, value) in [
            ("observation_ref", self.observation_ref.as_str()),
            ("source_provider_ref", self.source_provider_ref.as_str()),
            ("source_target_ref", self.source_target_ref.as_str()),
            ("suggested_from_meta_ref", self.suggested_from_meta_ref.as_str()),
            ("suggested_to_meta_ref", self.suggested_to_meta_ref.as_str()),
            ("suggested_relation", self.suggested_relation.as_str()),
            ("rationale", self.rationale.as_str()),
        ] {
            if value.trim().is_empty() {
                return Err(RecognitionError::InvalidObservation(format!(
                    "{field} cannot be empty"
                )));
            }
        }
        if self.occurrences == 0 {
            return Err(RecognitionError::InvalidObservation(
                "occurrences must be at least one".into(),
            ));
        }
        if self.evidence_refs.iter().any(|value| value.trim().is_empty()) {
            return Err(RecognitionError::InvalidObservation(
                "evidence refs cannot be empty".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaAmendmentCandidate {
    pub contract: String,
    pub candidate_ref: String,
    pub observation: ExternalObservation,
    pub state: RecognitionState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecognitionDecision {
    pub decision_ref: String,
    pub reviewer_ref: String,
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl RecognitionDecision {
    fn validate(&self) -> Result<(), RecognitionError> {
        if self.decision_ref.trim().is_empty() || self.reviewer_ref.trim().is_empty() {
            return Err(RecognitionError::InvalidDecision(
                "decision_ref and reviewer_ref must be non-empty".into(),
            ));
        }
        if self.evidence_refs.iter().any(|value| value.trim().is_empty()) {
            return Err(RecognitionError::InvalidDecision(
                "decision evidence refs cannot be empty".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecognisedMetaAmendment {
    pub contract: String,
    pub candidate_ref: String,
    pub observation_ref: String,
    pub from_meta_ref: String,
    pub to_meta_ref: String,
    pub relation: String,
    pub rationale: String,
    pub decision_ref: String,
    pub reviewer_ref: String,
    pub occurrences: u32,
    pub provenance: Vec<MetaProvenance>,
}

#[derive(Debug, Clone, Default)]
pub struct RecognitionLedger {
    candidates: BTreeMap<String, MetaAmendmentCandidate>,
    decisions: BTreeMap<String, RecognitionDecision>,
}

impl RecognitionLedger {
    pub fn propose(
        &mut self,
        observation: ExternalObservation,
    ) -> Result<MetaAmendmentCandidate, RecognitionError> {
        observation.validate()?;
        let candidate_ref = format!(
            "ql-mef:proposal:{}",
            sanitise_ref(observation.observation_ref.as_str())
        );
        let candidate = MetaAmendmentCandidate {
            contract: META_RECOGNITION_CONTRACT.into(),
            candidate_ref: candidate_ref.clone(),
            observation,
            state: RecognitionState::Proposed,
        };
        if let Some(existing) = self.candidates.get(&candidate_ref) {
            if existing == &candidate {
                return Ok(existing.clone());
            }
            return Err(RecognitionError::DuplicateConflict(candidate_ref));
        }
        self.candidates.insert(candidate_ref, candidate.clone());
        Ok(candidate)
    }

    pub fn candidate(&self, candidate_ref: &str) -> Option<&MetaAmendmentCandidate> {
        self.candidates.get(candidate_ref)
    }

    pub fn recognise(
        &mut self,
        candidate_ref: &str,
        decision: RecognitionDecision,
    ) -> Result<RecognisedMetaAmendment, RecognitionError> {
        decision.validate()?;
        let candidate = self
            .candidates
            .get_mut(candidate_ref)
            .ok_or_else(|| RecognitionError::UnknownCandidate(candidate_ref.into()))?;
        if candidate.state != RecognitionState::Proposed {
            return Err(RecognitionError::AlreadyDecided(candidate_ref.into()));
        }
        candidate.state = RecognitionState::Recognised;
        let mut provenance = vec![MetaProvenance {
            source_ref: candidate.observation.source_target_ref.clone(),
            source_revision: candidate.observation.source_revision.clone(),
        }];
        provenance.extend(
            candidate
                .observation
                .evidence_refs
                .iter()
                .chain(decision.evidence_refs.iter())
                .map(|reference| MetaProvenance {
                    source_ref: reference.clone(),
                    source_revision: None,
                }),
        );
        self.decisions
            .insert(candidate_ref.into(), decision.clone());
        Ok(RecognisedMetaAmendment {
            contract: META_RECOGNITION_CONTRACT.into(),
            candidate_ref: candidate.candidate_ref.clone(),
            observation_ref: candidate.observation.observation_ref.clone(),
            from_meta_ref: candidate.observation.suggested_from_meta_ref.clone(),
            to_meta_ref: candidate.observation.suggested_to_meta_ref.clone(),
            relation: candidate.observation.suggested_relation.clone(),
            rationale: candidate.observation.rationale.clone(),
            decision_ref: decision.decision_ref,
            reviewer_ref: decision.reviewer_ref,
            occurrences: candidate.observation.occurrences,
            provenance,
        })
    }

    pub fn reject(
        &mut self,
        candidate_ref: &str,
        decision: RecognitionDecision,
    ) -> Result<(), RecognitionError> {
        decision.validate()?;
        let candidate = self
            .candidates
            .get_mut(candidate_ref)
            .ok_or_else(|| RecognitionError::UnknownCandidate(candidate_ref.into()))?;
        if candidate.state != RecognitionState::Proposed {
            return Err(RecognitionError::AlreadyDecided(candidate_ref.into()));
        }
        candidate.state = RecognitionState::Rejected;
        self.decisions.insert(candidate_ref.into(), decision);
        Ok(())
    }
}

pub fn apply_recognised_amendment(
    projection: &mut MetaKnowledgeProjection,
    amendment: &RecognisedMetaAmendment,
) -> Result<(), RecognitionError> {
    let refs = projection.canonical_refs();
    for reference in [&amendment.from_meta_ref, &amendment.to_meta_ref] {
        if !refs.contains(reference.as_str()) {
            return Err(RecognitionError::MissingMetaRef(reference.clone()));
        }
    }
    if projection.relations.iter().any(|relation| {
        relation.from_ref == amendment.from_meta_ref
            && relation.to_ref == amendment.to_meta_ref
            && relation.relation == amendment.relation
            && relation.origin_ref.as_deref() == Some(amendment.decision_ref.as_str())
    }) {
        return Ok(());
    }
    let next_id = projection
        .relations
        .iter()
        .map(|relation| relation.projection_id)
        .max()
        .unwrap_or(0)
        + 1;
    projection.relations.push(ProjectedRelation {
        projection_id: next_id,
        from_ref: amendment.from_meta_ref.clone(),
        to_ref: amendment.to_meta_ref.clone(),
        relation: amendment.relation.clone(),
        origin: "recognised".into(),
        origin_ref: Some(amendment.decision_ref.clone()),
    });
    Ok(())
}

fn sanitise_ref(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.') {
                character
            } else {
                '-'
            }
        })
        .collect()
}
