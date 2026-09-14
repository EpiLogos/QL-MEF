//! Immutable Nara occasion and reinterpretation ledger.
//!
//! Exact replay returns the original source-qualified occasion. A later model,
//! commentary or changed interpretation is linked to that occasion but never
//! mutates it or inherits the standing of the original source.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::domain::{EvidenceStanding, ProtectedRef};
use super::{EventBasisRefs, SourceRevision};

pub const NARA_REPLAY_CONTRACT: &str = "ql.nara-replay/v1";

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
    text(&source.source_ref, "replay source")?;
    text(&source.revision, "replay source revision")?;
    text(&source.standing_ref, "replay source standing")
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NaraOccasion {
    pub occasion_ref: String,
    pub subject_id: String,
    pub event: EventBasisRefs,
    pub personal_reception_generation: u64,
    pub identity_revision: String,
    pub day_ref: String,
    pub now_ref: String,
    pub occurrence_at_unix_ms: u64,
    pub receipt_at_unix_ms: u64,
    pub protected_state_ref: ProtectedRef,
    pub activity_refs: Vec<String>,
    pub oracle_packet_refs: Vec<String>,
    pub transformation_phase_refs: Vec<String>,
    pub context_reading_refs: Vec<String>,
    pub integration_return_refs: Vec<String>,
    pub expression_refs: Vec<String>,
    pub source_revisions: Vec<SourceRevision>,
}

impl NaraOccasion {
    pub fn validate(&self) -> Result<(), String> {
        text(&self.occasion_ref, "Nara occasion")?;
        text(&self.subject_id, "Nara occasion subject")?;
        if self.subject_id != self.event.subject_ref {
            return Err("Nara occasion subject does not match its event".into());
        }
        text(&self.identity_revision, "Nara identity revision")?;
        text(&self.day_ref, "Central Day reference")?;
        text(&self.now_ref, "Central NOW reference")?;
        if self.receipt_at_unix_ms < self.occurrence_at_unix_ms {
            return Err("Nara occasion receipt precedes occurrence".into());
        }
        self.protected_state_ref.validate()?;
        refs(&self.activity_refs, "occasion activity reference", 4096)?;
        refs(&self.oracle_packet_refs, "occasion oracle reference", 4096)?;
        refs(
            &self.transformation_phase_refs,
            "occasion transformation reference",
            4096,
        )?;
        refs(
            &self.context_reading_refs,
            "occasion context reference",
            4096,
        )?;
        refs(
            &self.integration_return_refs,
            "occasion integration reference",
            4096,
        )?;
        refs(&self.expression_refs, "occasion expression reference", 4096)?;
        if self.source_revisions.is_empty() || self.source_revisions.len() > 4096 {
            return Err("Nara occasion requires 1..4096 source revisions".into());
        }
        for revision in &self.source_revisions {
            source(revision)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReinterpretationReceipt {
    pub reinterpretation_ref: String,
    pub occasion_ref: String,
    pub interpretation_revision: String,
    pub model_ref: Option<String>,
    pub method_ref: Option<String>,
    pub output_ref: ProtectedRef,
    pub source_refs: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub interpreted_at_unix_ms: u64,
    pub standing: EvidenceStanding,
}

impl ReinterpretationReceipt {
    pub fn validate(&self) -> Result<(), String> {
        text(&self.reinterpretation_ref, "reinterpretation")?;
        text(&self.occasion_ref, "reinterpretation occasion")?;
        text(&self.interpretation_revision, "reinterpretation revision")?;
        if let Some(reference) = &self.model_ref {
            text(reference, "reinterpretation model")?;
        }
        if let Some(reference) = &self.method_ref {
            text(reference, "reinterpretation method")?;
        }
        self.output_ref.validate()?;
        refs(&self.source_refs, "reinterpretation source reference", 1024)?;
        refs(
            &self.evidence_refs,
            "reinterpretation evidence reference",
            1024,
        )?;
        if matches!(
            self.standing,
            EvidenceStanding::Source | EvidenceStanding::Observed
        ) {
            return Err(
                "later reinterpretation cannot inherit Source/Observed standing from the original occasion"
                    .into(),
            );
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NaraReplayLedger {
    pub schema: String,
    pub originals: Vec<NaraOccasion>,
    pub reinterpretations: Vec<ReinterpretationReceipt>,
}

impl Default for NaraReplayLedger {
    fn default() -> Self {
        Self {
            schema: NARA_REPLAY_CONTRACT.into(),
            originals: Vec::new(),
            reinterpretations: Vec::new(),
        }
    }
}

impl NaraReplayLedger {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != NARA_REPLAY_CONTRACT {
            return Err("unsupported Nara replay contract".into());
        }
        if self.originals.len() > 65_536 || self.reinterpretations.len() > 262_144 {
            return Err("Nara replay ledger exceeds contract bounds".into());
        }
        let mut originals = BTreeSet::new();
        for occasion in &self.originals {
            occasion.validate()?;
            if !originals.insert(occasion.occasion_ref.as_str()) {
                return Err("duplicate original Nara occasion".into());
            }
        }
        let mut reinterpretations = BTreeSet::new();
        for receipt in &self.reinterpretations {
            receipt.validate()?;
            if !originals.contains(receipt.occasion_ref.as_str()) {
                return Err("reinterpretation refers to an unknown original occasion".into());
            }
            if !reinterpretations.insert(receipt.reinterpretation_ref.as_str()) {
                return Err("duplicate Nara reinterpretation reference".into());
            }
        }
        Ok(())
    }

    pub fn record_original(&mut self, occasion: NaraOccasion) -> Result<(), String> {
        occasion.validate()?;
        if self
            .originals
            .iter()
            .any(|existing| existing.occasion_ref == occasion.occasion_ref)
        {
            return Err("original Nara occasion already exists".into());
        }
        self.originals.push(occasion);
        Ok(())
    }

    pub fn record_reinterpretation(
        &mut self,
        receipt: ReinterpretationReceipt,
    ) -> Result<(), String> {
        receipt.validate()?;
        if !self
            .originals
            .iter()
            .any(|occasion| occasion.occasion_ref == receipt.occasion_ref)
        {
            return Err("cannot reinterpret an unknown Nara occasion".into());
        }
        if self
            .reinterpretations
            .iter()
            .any(|existing| existing.reinterpretation_ref == receipt.reinterpretation_ref)
        {
            return Err("Nara reinterpretation reference already exists".into());
        }
        self.reinterpretations.push(receipt);
        Ok(())
    }

    pub fn replay_original(&self, occasion_ref: &str) -> Option<&NaraOccasion> {
        self.originals
            .iter()
            .find(|occasion| occasion.occasion_ref == occasion_ref)
    }

    pub fn reinterpretations_for(
        &self,
        occasion_ref: &str,
    ) -> impl Iterator<Item = &ReinterpretationReceipt> {
        self.reinterpretations
            .iter()
            .filter(move |receipt| receipt.occasion_ref == occasion_ref)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
                profile_generation: 7,
                registry_revision: "registry:r1".into(),
                m1_revision: "m1:r1".into(),
                m2_source_ref: "m2:source".into(),
                m2_contract_ref: "ql.m2-engine/v1".into(),
                m3_source_ref: "m3:source".into(),
                m3_contract_ref: "ql.m3-engine/v1".into(),
            },
            personal_reception_generation: 2,
            identity_revision: "identity:r1".into(),
            day_ref: "central:day:2026-09-14".into(),
            now_ref: "central:now:abc".into(),
            occurrence_at_unix_ms: 1,
            receipt_at_unix_ms: 2,
            protected_state_ref: protected("state"),
            activity_refs: vec!["activity:1".into()],
            oracle_packet_refs: Vec::new(),
            transformation_phase_refs: Vec::new(),
            context_reading_refs: Vec::new(),
            integration_return_refs: Vec::new(),
            expression_refs: Vec::new(),
            source_revisions: vec![SourceRevision {
                source_ref: "source:original".into(),
                revision: "r1".into(),
                standing_ref: "authored".into(),
            }],
        }
    }

    #[test]
    fn later_interpretation_does_not_mutate_original_replay() {
        let original = occasion();
        let mut ledger = NaraReplayLedger::default();
        ledger.record_original(original.clone()).unwrap();
        ledger
            .record_reinterpretation(ReinterpretationReceipt {
                reinterpretation_ref: "reinterpretation:2".into(),
                occasion_ref: original.occasion_ref.clone(),
                interpretation_revision: "r2".into(),
                model_ref: Some("model:new".into()),
                method_ref: None,
                output_ref: protected("new-reading"),
                source_refs: vec!["source:commentary".into()],
                evidence_refs: Vec::new(),
                interpreted_at_unix_ms: 5,
                standing: EvidenceStanding::Derived,
            })
            .unwrap();
        assert_eq!(ledger.replay_original("occasion:1"), Some(&original));
        assert_eq!(ledger.reinterpretations_for("occasion:1").count(), 1);
    }

    #[test]
    fn reinterpretation_cannot_claim_original_source_standing() {
        let receipt = ReinterpretationReceipt {
            reinterpretation_ref: "reinterpretation:1".into(),
            occasion_ref: "occasion:1".into(),
            interpretation_revision: "r2".into(),
            model_ref: Some("model:new".into()),
            method_ref: None,
            output_ref: protected("reading"),
            source_refs: Vec::new(),
            evidence_refs: Vec::new(),
            interpreted_at_unix_ms: 5,
            standing: EvidenceStanding::Source,
        };
        assert!(receipt.validate().is_err());
    }
}
