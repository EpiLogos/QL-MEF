//! Nara reception of the accepted #94/AW3 thought-consumption reading.
//!
//! AW3/Factory remain the owners of T/T′ interpretation, lifecycle mutation and
//! Recognition proposals. Nara binds that already-validated reading to this
//! person's Central Day/NOW without re-running its semantics or creating thought
//! silos.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::vak_thought_consumption::{
    FACTORY_THOUGHT_CONSUMPTION_CONTRACT, FactoryRunThoughtLifecycle, FactoryThoughtSource,
    FactoryThoughtUse, RecognitionCandidate, THOUGHT_CONSUMPTION_READING_CONTRACT,
    ThoughtConsumptionReading, ThoughtInterpretationReading,
};

use super::activity::NaraActivityLog;

pub const NARA_AW3_RECEPTION_CONTRACT: &str = "ql.nara-aw3-thought-reception/v1";

fn text(value: &str, label: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.len() > 16_384 || value.contains('\0') {
        Err(format!("invalid {label}"))
    } else {
        Ok(())
    }
}

fn source(value: &FactoryThoughtSource, label: &str) -> Result<(), String> {
    text(&value.owner, label)?;
    text(&value.reference, label)?;
    text(&value.revision, label)
}

fn refs(values: &BTreeSet<String>, label: &str) -> Result<(), String> {
    for value in values {
        text(value, label)?;
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Aw3ThoughtConsumptionReception {
    pub schema: String,
    pub owner_contract_ref: String,
    pub subject_id: String,
    pub day_ref: String,
    pub day_revision: String,
    pub now_ref: String,
    pub now_revision: String,
    pub source_history_ref: String,
    pub consumption_id: String,
    pub run_ref: String,
    pub working_field: FactoryThoughtSource,
    pub interpretations: Vec<ThoughtInterpretationReading>,
    pub performance_ref: Option<String>,
    pub performance_evidence_refs: BTreeSet<String>,
    pub actual_evidence: Vec<FactoryThoughtSource>,
    pub human_response: Vec<FactoryThoughtSource>,
    pub assessment: FactoryThoughtSource,
    pub uses: Vec<FactoryThoughtUse>,
    pub recognition_candidates: Vec<RecognitionCandidate>,
    pub continuing_question_refs: BTreeSet<String>,
    pub retention_policy: FactoryThoughtSource,
    pub resulting_lifecycle: FactoryRunThoughtLifecycle,
    pub ql_basis_refs: BTreeSet<String>,
    pub factory_receipt_contract: String,
    pub factory_command_id: String,
    pub standing: String,
}

impl Aw3ThoughtConsumptionReception {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != NARA_AW3_RECEPTION_CONTRACT {
            return Err("unsupported Nara AW3 reception contract".into());
        }
        if self.owner_contract_ref != THOUGHT_CONSUMPTION_READING_CONTRACT {
            return Err("Nara AW3 reception must consume the accepted QL reading contract".into());
        }
        if self.factory_receipt_contract != FACTORY_THOUGHT_CONSUMPTION_CONTRACT {
            return Err(
                "Nara AW3 reception must retain the accepted Factory receipt contract".into(),
            );
        }
        text(&self.subject_id, "Nara subject")?;
        text(&self.day_ref, "Central Day")?;
        text(&self.day_revision, "Central Day revision")?;
        text(&self.now_ref, "Central NOW")?;
        text(&self.now_revision, "Central NOW revision")?;
        text(&self.source_history_ref, "Central source history")?;
        text(&self.consumption_id, "thought consumption")?;
        text(&self.run_ref, "Factory Run")?;
        text(&self.factory_command_id, "Factory command")?;
        text(&self.standing, "AW3 reception standing")?;
        source(&self.working_field, "working field source")?;
        source(&self.assessment, "assessment source")?;
        source(&self.retention_policy, "retention policy source")?;
        for value in self.actual_evidence.iter().chain(&self.human_response) {
            source(value, "thought evidence source")?;
        }
        let mut thought_ids = BTreeSet::new();
        for interpretation in &self.interpretations {
            text(&interpretation.thought_id, "thought id")?;
            if interpretation.inventory_ref != interpretation.meaning.inventory_ref() {
                return Err(
                    "AW3 thought inventory identity does not match its typed meaning".into(),
                );
            }
            if !thought_ids.insert(interpretation.thought_id.as_str()) {
                return Err("duplicate thought interpretation in one AW3 reception".into());
            }
            source(&interpretation.anchor, "thought anchor")?;
            source(&interpretation.interpretation, "thought interpretation")?;
        }
        for use_reading in &self.uses {
            source(&use_reading.source, "thought use source")?;
            source(&use_reading.receiving, "thought use receiving source")?;
        }
        for candidate in &self.recognition_candidates {
            text(&candidate.target_ref, "Recognition target")?;
            text(&candidate.receiving_ref, "Recognition receiving target")?;
            refs(&candidate.source_thought_refs, "Recognition source thought")?;
            refs(&candidate.recognition_evidence_refs, "Recognition evidence")?;
            refs(
                &candidate.verification_evidence_refs,
                "Recognition verification evidence",
            )?;
        }
        refs(&self.performance_evidence_refs, "performance evidence")?;
        refs(&self.continuing_question_refs, "continuing question")?;
        refs(&self.ql_basis_refs, "QL thought basis")?;
        Ok(())
    }
}

impl NaraActivityLog {
    /// Receive, but do not reinterpret, an accepted AW3 reading into this Nara's
    /// current Central temporal context.
    pub fn receive_aw3_thought_consumption(
        &self,
        reading: &ThoughtConsumptionReading,
    ) -> Result<Aw3ThoughtConsumptionReception, String> {
        self.validate()?;
        if reading.contract != THOUGHT_CONSUMPTION_READING_CONTRACT {
            return Err("unsupported AW3 thought-consumption reading".into());
        }
        if reading.factory_receipt.contract != FACTORY_THOUGHT_CONSUMPTION_CONTRACT {
            return Err("AW3 reading does not retain the accepted Factory receipt".into());
        }
        let receipt = Aw3ThoughtConsumptionReception {
            schema: NARA_AW3_RECEPTION_CONTRACT.into(),
            owner_contract_ref: reading.contract.clone(),
            subject_id: self.subject_id.clone(),
            day_ref: self.temporal.day_ref.clone(),
            day_revision: self.temporal.day_revision.clone(),
            now_ref: self.temporal.now_ref.clone(),
            now_revision: self.temporal.now_revision.clone(),
            source_history_ref: self.temporal.source_history_ref.clone(),
            consumption_id: reading.consumption_id.clone(),
            run_ref: reading.run_ref.clone(),
            working_field: reading.working_field.clone(),
            interpretations: reading.interpretations.clone(),
            performance_ref: reading.performance_ref.clone(),
            performance_evidence_refs: reading.performance_evidence_refs.clone(),
            actual_evidence: reading.actual_evidence.clone(),
            human_response: reading.human_response.clone(),
            assessment: reading.assessment.clone(),
            uses: reading.uses.clone(),
            recognition_candidates: reading.recognition_candidates.clone(),
            continuing_question_refs: reading.continuing_question_refs.clone(),
            retention_policy: reading.retention_policy.clone(),
            resulting_lifecycle: reading.resulting_lifecycle,
            ql_basis_refs: reading.ql_basis_refs.clone(),
            factory_receipt_contract: reading.factory_receipt.contract.clone(),
            factory_command_id: reading.factory_receipt.command_id.clone(),
            standing: reading.standing.clone(),
        };
        receipt.validate()?;
        Ok(receipt)
    }
}
