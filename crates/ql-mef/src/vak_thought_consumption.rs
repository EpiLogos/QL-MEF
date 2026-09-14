//! QL interpretation of already-consumed T/T′ working thought.
//!
//! Factory owns RunThoughtField and `factory.run-thought-consumption/v1` lifecycle
//! mutation. Central owns NOW/source material. AIKit owns Wiki/Knowledge and native
//! `= name` Skill/Method registration. QL owns the twelve source meanings and the
//! relation between an actual consumption receipt, an optional performed Vāk event,
//! and the semantic evidence that can be offered to Recognition.
//!
//! This module therefore stores nothing and grants no authority. It validates one
//! immutable Factory receipt, binds each selected interpretation to the recovered
//! `thought:T0 … thought:T5′` inventory identity, optionally proves that the receipt
//! actually consumed evidence from the same performed occasion, and emits a reading.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::vak_composition::{CompositionError, Result};
use crate::vak_performance::{PERFORMANCE_EVENT_CONTRACT, VakPerformanceEvent};

pub const FACTORY_THOUGHT_CONSUMPTION_CONTRACT: &str = "factory.run-thought-consumption/v1";
pub const THOUGHT_CONSUMPTION_PROJECTION_REQUEST: &str = "ql.vak-thought-consumption-projection/v1";
pub const THOUGHT_CONSUMPTION_READING_CONTRACT: &str = "ql.vak-thought-consumption/v1";
pub const THOUGHT_SOURCE_PATH: &str = "docs/kernel-rebuild/VAK-OIKONOMIA-KNOWLEDGE-RETURN.md";
pub const THOUGHT_SOURCE_BLOB: &str = "09f7d29ad6262f85bc2858f7c468f22d0bd398f3";
pub const AW0_SOURCE_PROJECTION_PATH: &str = "scripts/aw0.py";
pub const AW0_SOURCE_PROJECTION_BLOB: &str = "40adf7f2ed53da9e6cdd2e135437c160f035e7ee";
const MAX_ITEMS: usize = 1024;
const MAX_REF: usize = 16_384;

fn error(message: impl Into<String>) -> CompositionError {
    CompositionError(message.into())
}

fn require(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(error(message))
    }
}

fn reference(value: &str, message: &str) -> Result<()> {
    require(
        !value.trim().is_empty() && value.len() <= MAX_REF && !value.contains('\0'),
        message,
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ThoughtMeaning {
    #[serde(rename = "T0")]
    Question,
    #[serde(rename = "T0′")]
    Assumption,
    #[serde(rename = "T1")]
    Trace,
    #[serde(rename = "T1′")]
    Lacuna,
    #[serde(rename = "T2")]
    Challenge,
    #[serde(rename = "T2′")]
    Affordance,
    #[serde(rename = "T3")]
    Pattern,
    #[serde(rename = "T3′")]
    Anomaly,
    #[serde(rename = "T4")]
    Discovery,
    #[serde(rename = "T4′")]
    Concealment,
    #[serde(rename = "T5")]
    Insight,
    #[serde(rename = "T5′")]
    Integration,
}

impl ThoughtMeaning {
    pub const ALL: [Self; 12] = [
        Self::Question,
        Self::Assumption,
        Self::Trace,
        Self::Lacuna,
        Self::Challenge,
        Self::Affordance,
        Self::Pattern,
        Self::Anomaly,
        Self::Discovery,
        Self::Concealment,
        Self::Insight,
        Self::Integration,
    ];

    pub const fn code(self) -> &'static str {
        match self {
            Self::Question => "T0",
            Self::Assumption => "T0′",
            Self::Trace => "T1",
            Self::Lacuna => "T1′",
            Self::Challenge => "T2",
            Self::Affordance => "T2′",
            Self::Pattern => "T3",
            Self::Anomaly => "T3′",
            Self::Discovery => "T4",
            Self::Concealment => "T4′",
            Self::Insight => "T5",
            Self::Integration => "T5′",
        }
    }

    pub fn inventory_ref(self) -> String {
        format!("thought:{}", self.code())
    }

    pub const fn meaning(self) -> &'static str {
        match self {
            Self::Question => "Question: articulate what is open.",
            Self::Assumption => "Assumption: expose what was taken as given.",
            Self::Trace => "Trace: follow a source, observation or inquiry.",
            Self::Lacuna => "Lacuna: identify what the record did not establish.",
            Self::Challenge => "Challenge: meet a difficulty, contradiction or limit.",
            Self::Affordance => "Affordance: recognise the possibility opened through it.",
            Self::Pattern => "Pattern: articulate a relation or recurrence.",
            Self::Anomaly => "Anomaly: locate what does not fit.",
            Self::Discovery => "Discovery: determine what has newly appeared.",
            Self::Concealment => "Concealment: examine what that determination leaves obscured.",
            Self::Insight => "Insight: formulate an integrating understanding.",
            Self::Integration => "Integration: work through how it belongs and returns.",
        }
    }

    pub const fn retrospective(self) -> bool {
        matches!(
            self,
            Self::Assumption
                | Self::Lacuna
                | Self::Affordance
                | Self::Anomaly
                | Self::Concealment
                | Self::Integration
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FactoryThoughtSource {
    pub owner: String,
    pub reference: String,
    pub revision: String,
}

impl FactoryThoughtSource {
    fn validate(&self) -> Result<()> {
        reference(&self.owner, "missing thought source owner")?;
        reference(&self.reference, "missing thought source reference")?;
        reference(&self.revision, "missing thought source revision")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "kebab-case", deny_unknown_fields)]
pub enum FactoryThoughtSourceObservation {
    Current {
        source: FactoryThoughtSource,
        receipt: FactoryThoughtSource,
    },
    Unavailable {
        reason: String,
    },
    Denied {
        reason: String,
    },
    Changed {
        observed: FactoryThoughtSource,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FactoryThoughtConsumptionInput {
    pub thought_id: String,
    pub anchor: FactoryThoughtSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpretation: Option<FactoryThoughtSource>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FactoryThoughtUseKind {
    Context,
    WikiReading,
    Evaluation,
    SkillImprovement,
    RecognisedPraxis,
    ContinuingQuestion,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FactoryThoughtUse {
    pub kind: FactoryThoughtUseKind,
    pub source: FactoryThoughtSource,
    pub receiving: FactoryThoughtSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FactoryRunThoughtLifecycle {
    Active,
    Resolved,
    Superseded,
    Integrated,
    Retained,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FactoryThoughtConsumption {
    pub consumption_id: String,
    pub run_ref: String,
    pub consumer_ref: String,
    pub working_field: FactoryThoughtSource,
    pub inputs: Vec<FactoryThoughtConsumptionInput>,
    pub actual_evidence: Vec<FactoryThoughtSource>,
    pub human_response: Vec<FactoryThoughtSource>,
    pub assessment: FactoryThoughtSource,
    pub uses: Vec<FactoryThoughtUse>,
    pub retention_policy: FactoryThoughtSource,
    pub resulting_lifecycle: FactoryRunThoughtLifecycle,
}

impl FactoryThoughtConsumption {
    fn sources(&self) -> BTreeSet<&FactoryThoughtSource> {
        let mut sources = BTreeSet::from([
            &self.working_field,
            &self.assessment,
            &self.retention_policy,
        ]);
        for input in &self.inputs {
            sources.insert(&input.anchor);
            sources.extend(input.interpretation.iter());
        }
        sources.extend(&self.actual_evidence);
        sources.extend(&self.human_response);
        for use_reading in &self.uses {
            sources.extend([&use_reading.source, &use_reading.receiving]);
        }
        sources
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FactoryThoughtConsumptionReceipt {
    pub contract: String,
    pub command_id: String,
    pub consumption: FactoryThoughtConsumption,
    pub observations: Vec<FactoryThoughtSourceObservation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ThoughtMeaningSelection {
    pub thought_id: String,
    pub meaning: ThoughtMeaning,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ThoughtConsumptionProjectionRequest {
    pub schema: String,
    pub receipt: FactoryThoughtConsumptionReceipt,
    pub interpretations: Vec<ThoughtMeaningSelection>,
    /// Exact refs that appear both in the performed event and in Factory's
    /// `actualEvidence`. Empty when this consumption was not performance-bound.
    #[serde(default)]
    pub performance_evidence_refs: BTreeSet<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performance: Option<VakPerformanceEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ThoughtInterpretationReading {
    pub thought_id: String,
    pub meaning: ThoughtMeaning,
    pub inventory_ref: String,
    pub source_path: String,
    pub source_blob: String,
    pub anchor: FactoryThoughtSource,
    pub interpretation: FactoryThoughtSource,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RecognitionCandidate {
    /// Existing output/praxis identity proposed for naming; QL does not register it.
    pub target_ref: String,
    pub receiving_ref: String,
    pub operation: String,
    pub horizon: String,
    pub source_thought_refs: BTreeSet<String>,
    pub recognition_evidence_refs: BTreeSet<String>,
    pub verification_evidence_refs: BTreeSet<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performance_ref: Option<String>,
    pub standing: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ThoughtConsumptionReading {
    pub contract: String,
    pub consumption_id: String,
    pub run_ref: String,
    pub working_field: FactoryThoughtSource,
    pub interpretations: Vec<ThoughtInterpretationReading>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
    pub factory_receipt: FactoryThoughtConsumptionReceipt,
    pub standing: String,
}

pub fn project_thought_consumption(
    request: ThoughtConsumptionProjectionRequest,
) -> Result<ThoughtConsumptionReading> {
    validate_request(&request)?;
    let receipt = &request.receipt;
    let consumption = &receipt.consumption;
    let selections = request
        .interpretations
        .iter()
        .map(|selection| (selection.thought_id.as_str(), selection.meaning))
        .collect::<BTreeMap<_, _>>();

    let interpretations = consumption
        .inputs
        .iter()
        .map(|input| {
            let meaning = selections[&input.thought_id.as_str()];
            ThoughtInterpretationReading {
                thought_id: input.thought_id.clone(),
                meaning,
                inventory_ref: meaning.inventory_ref(),
                source_path: THOUGHT_SOURCE_PATH.into(),
                source_blob: THOUGHT_SOURCE_BLOB.into(),
                anchor: input.anchor.clone(),
                interpretation: input
                    .interpretation
                    .clone()
                    .expect("validated typed T/T-prime interpretation"),
            }
        })
        .collect::<Vec<_>>();

    let performance_ref = request
        .performance
        .as_ref()
        .map(|event| event.performance_ref.clone());
    let source_thought_refs = interpretations
        .iter()
        .map(|reading| reading.inventory_ref.clone())
        .collect::<BTreeSet<_>>();
    let actual_refs = consumption
        .actual_evidence
        .iter()
        .map(|source| source.reference.clone())
        .collect::<BTreeSet<_>>();
    let human_refs = consumption
        .human_response
        .iter()
        .map(|source| source.reference.clone())
        .collect::<BTreeSet<_>>();

    let mut recognition_candidates = Vec::new();
    let mut continuing_question_refs = BTreeSet::new();
    for use_reading in &consumption.uses {
        match use_reading.kind {
            FactoryThoughtUseKind::RecognisedPraxis => {
                let mut recognition_evidence_refs = BTreeSet::from([
                    use_reading.receiving.reference.clone(),
                    consumption.assessment.reference.clone(),
                ]);
                recognition_evidence_refs.extend(human_refs.iter().cloned());
                recognition_evidence_refs.extend(request.performance_evidence_refs.iter().cloned());

                let mut verification_evidence_refs = actual_refs.clone();
                verification_evidence_refs.insert(use_reading.source.reference.clone());
                verification_evidence_refs
                    .extend(request.performance_evidence_refs.iter().cloned());

                recognition_candidates.push(RecognitionCandidate {
                    target_ref: use_reading.source.reference.clone(),
                    receiving_ref: use_reading.receiving.reference.clone(),
                    operation: "=".into(),
                    horizon: "@5/R#".into(),
                    source_thought_refs: source_thought_refs.clone(),
                    recognition_evidence_refs,
                    verification_evidence_refs,
                    performance_ref: performance_ref.clone(),
                    standing: "QL Recognition proposal only; AIKit must validate native ResourceRef, successful invocation/attempt/Return, authorised promotion, verification and Method reuse".into(),
                });
            }
            FactoryThoughtUseKind::ContinuingQuestion => {
                continuing_question_refs.insert(use_reading.source.reference.clone());
            }
            _ => {}
        }
    }

    let mut ql_basis_refs = BTreeSet::from([
        THOUGHT_SOURCE_PATH.to_owned(),
        format!("git-blob:{THOUGHT_SOURCE_BLOB}"),
        AW0_SOURCE_PROJECTION_PATH.to_owned(),
        format!("git-blob:{AW0_SOURCE_PROJECTION_BLOB}"),
    ]);
    ql_basis_refs.extend(source_thought_refs);
    ql_basis_refs.extend(request.performance_evidence_refs.iter().cloned());

    Ok(ThoughtConsumptionReading {
        contract: THOUGHT_CONSUMPTION_READING_CONTRACT.into(),
        consumption_id: consumption.consumption_id.clone(),
        run_ref: consumption.run_ref.clone(),
        working_field: consumption.working_field.clone(),
        interpretations,
        performance_ref,
        performance_evidence_refs: request.performance_evidence_refs,
        actual_evidence: consumption.actual_evidence.clone(),
        human_response: consumption.human_response.clone(),
        assessment: consumption.assessment.clone(),
        uses: consumption.uses.clone(),
        recognition_candidates,
        continuing_question_refs,
        retention_policy: consumption.retention_policy.clone(),
        resulting_lifecycle: consumption.resulting_lifecycle,
        ql_basis_refs,
        factory_receipt: request.receipt,
        standing: "QL source-qualified reading of an immutable Factory thought-consumption receipt; no source mutation, Skill/Method registration, training grant or task-fitness claim".into(),
    })
}

fn validate_request(request: &ThoughtConsumptionProjectionRequest) -> Result<()> {
    require(
        request.schema == THOUGHT_CONSUMPTION_PROJECTION_REQUEST,
        "unsupported T/T-prime consumption projection request",
    )?;
    validate_receipt(&request.receipt)?;
    validate_interpretations(&request.receipt.consumption, &request.interpretations)?;
    validate_performance(request)
}

fn validate_receipt(receipt: &FactoryThoughtConsumptionReceipt) -> Result<()> {
    require(
        receipt.contract == FACTORY_THOUGHT_CONSUMPTION_CONTRACT,
        "unsupported Factory thought-consumption contract",
    )?;
    reference(&receipt.command_id, "missing Factory consumption command")?;
    let consumption = &receipt.consumption;
    for (value, message) in [
        (&consumption.consumption_id, "missing consumption identity"),
        (&consumption.run_ref, "missing Factory Run reference"),
        (
            &consumption.consumer_ref,
            "missing consumption actor/consumer",
        ),
    ] {
        reference(value, message)?;
    }
    require(
        !consumption.inputs.is_empty() && consumption.inputs.len() <= MAX_ITEMS,
        "T/T-prime consumption requires a bounded nonempty input set",
    )?;
    require(
        !consumption.actual_evidence.is_empty()
            && consumption.actual_evidence.len() <= MAX_ITEMS
            && consumption.human_response.len() <= MAX_ITEMS
            && !consumption.uses.is_empty()
            && consumption.uses.len() <= MAX_ITEMS,
        "consumption must retain actual evidence and at least one useful result",
    )?;
    require(
        consumption.resulting_lifecycle != FactoryRunThoughtLifecycle::Active,
        "consumed working thought cannot remain Active",
    )?;

    let mut thought_ids = BTreeSet::new();
    for input in &consumption.inputs {
        reference(&input.thought_id, "missing Factory RunThought identity")?;
        require(
            thought_ids.insert(input.thought_id.as_str()),
            "duplicate RunThought input in consumption receipt",
        )?;
        input.anchor.validate()?;
        if let Some(interpretation) = &input.interpretation {
            interpretation.validate()?;
        }
    }
    for source in consumption.sources() {
        source.validate()?;
    }

    let expected = consumption.sources();
    require(
        receipt.observations.len() == expected.len(),
        "Factory receipt does not retain one observation per consumed source",
    )?;
    let mut observed = BTreeSet::new();
    for observation in &receipt.observations {
        match observation {
            FactoryThoughtSourceObservation::Current { source, receipt } => {
                source.validate()?;
                receipt.validate()?;
                require(
                    expected.contains(source) && observed.insert(source),
                    "Factory receipt contains foreign or duplicate current-source evidence",
                )?;
            }
            FactoryThoughtSourceObservation::Unavailable { .. }
            | FactoryThoughtSourceObservation::Denied { .. }
            | FactoryThoughtSourceObservation::Changed { .. } => {
                return Err(error(
                    "accepted Factory consumption receipt contains non-current source standing",
                ));
            }
        }
    }
    Ok(())
}

fn validate_interpretations(
    consumption: &FactoryThoughtConsumption,
    selections: &[ThoughtMeaningSelection],
) -> Result<()> {
    require(
        selections.len() == consumption.inputs.len() && selections.len() <= MAX_ITEMS,
        "every consumed T/T-prime input requires exactly one QL meaning selection",
    )?;
    let mut selected = BTreeMap::new();
    for selection in selections {
        reference(
            &selection.thought_id,
            "missing selected RunThought identity",
        )?;
        require(
            selected
                .insert(selection.thought_id.as_str(), selection.meaning)
                .is_none(),
            "duplicate QL meaning selection for one RunThought",
        )?;
    }
    for input in &consumption.inputs {
        let meaning = selected
            .get(input.thought_id.as_str())
            .copied()
            .ok_or_else(|| error("consumed RunThought has no QL T/T-prime meaning"))?;
        let interpretation = input.interpretation.as_ref().ok_or_else(|| {
            error("T/T-prime projection requires Factory's typed interpretation ref")
        })?;
        require(
            interpretation.reference == meaning.inventory_ref(),
            "Factory interpretation ref does not match the recovered QL thought identity",
        )?;
    }
    Ok(())
}

fn validate_performance(request: &ThoughtConsumptionProjectionRequest) -> Result<()> {
    match &request.performance {
        None => require(
            request.performance_evidence_refs.is_empty(),
            "performance evidence cannot be supplied without a performed Vāk event",
        ),
        Some(event) => {
            require(
                event.contract == PERFORMANCE_EVENT_CONTRACT,
                "unsupported Vāk performance event contract",
            )?;
            require(
                event.factory.run_ref == request.receipt.consumption.run_ref,
                "thought consumption and performed Vāk event belong to different Factory Runs",
            )?;
            require(
                !request.performance_evidence_refs.is_empty(),
                "performance-bound consumption requires exact shared evidence refs",
            )?;
            let actual_refs = request
                .receipt
                .consumption
                .actual_evidence
                .iter()
                .map(|source| source.reference.as_str())
                .collect::<BTreeSet<_>>();
            let event_refs = performance_refs(event);
            for reference in &request.performance_evidence_refs {
                require(
                    actual_refs.contains(reference.as_str())
                        && event_refs.contains(reference.as_str()),
                    "performance evidence ref is not present in both Factory consumption and performed event",
                )?;
            }
            Ok(())
        }
    }
}

fn performance_refs(event: &VakPerformanceEvent) -> BTreeSet<&str> {
    let mut refs = BTreeSet::from([
        event.performance_ref.as_str(),
        event.factory.run_ref.as_str(),
        event.ql_binding_ref.as_str(),
        event.ql_binding_revision.as_str(),
    ]);
    refs.extend(event.factory_receipt_refs.iter().map(String::as_str));
    refs.extend(event.ql_basis_refs.iter().map(String::as_str));
    refs.extend(event.factory.source_refs.iter().map(String::as_str));
    for attempt in &event.factory.attempts {
        refs.insert(attempt.execution_ref.as_str());
        refs.extend(attempt.artifact_refs.iter().map(String::as_str));
        refs.extend(attempt.late_artifact_refs.iter().map(String::as_str));
        refs.extend(attempt.evidence_refs.iter().map(String::as_str));
    }
    for material in &event.factory.chain_inputs {
        refs.extend(material.artifact_refs.iter().map(String::as_str));
        refs.extend(material.evidence_refs.iter().map(String::as_str));
    }
    if let Some(stop) = &event.factory.sustained_stop {
        refs.extend(stop.evidence_refs.iter().map(String::as_str));
    }
    refs
}

#[cfg(test)]
mod tests {
    use crate::vak_performance::{
        FACTORY_VAK_PERFORMANCE_CONTRACT, FactoryLegStatus, FactoryVakAttempt,
        FactoryVakPerformanceSnapshot, PerformanceObservation, PerformanceObservationMode,
        PerformanceSemantics,
    };
    use crate::vak_profile::{
        ContentPosition, ContentType, ContextSequence, InquiryDirection, Participation, ThreadForm,
    };

    use super::*;

    fn source(reference: &str) -> FactoryThoughtSource {
        FactoryThoughtSource {
            owner: "native-owner".into(),
            reference: reference.into(),
            revision: "r1".into(),
        }
    }

    fn receipt(
        meaning: ThoughtMeaning,
        use_kind: FactoryThoughtUseKind,
        human_response: bool,
        actual_ref: &str,
    ) -> FactoryThoughtConsumptionReceipt {
        let input = FactoryThoughtConsumptionInput {
            thought_id: "thought-local".into(),
            anchor: source("central:now/report"),
            interpretation: Some(source(&meaning.inventory_ref())),
        };
        let consumption = FactoryThoughtConsumption {
            consumption_id: "consumption:1".into(),
            run_ref: "run:1".into(),
            consumer_ref: "agent:epii".into(),
            working_field: source("central:now/2026-09-13"),
            inputs: vec![input],
            actual_evidence: vec![source(actual_ref)],
            human_response: human_response
                .then(|| source("human:response/1"))
                .into_iter()
                .collect(),
            assessment: source("evaluation:1"),
            uses: vec![FactoryThoughtUse {
                kind: use_kind,
                source: source("skill-candidate:revisit"),
                receiving: source("recognition:1"),
            }],
            retention_policy: source("central:retention/now"),
            resulting_lifecycle: FactoryRunThoughtLifecycle::Integrated,
        };
        let observations = consumption
            .sources()
            .into_iter()
            .map(|expected| FactoryThoughtSourceObservation::Current {
                source: expected.clone(),
                receipt: source(&format!("receipt:{}", expected.reference)),
            })
            .collect();
        FactoryThoughtConsumptionReceipt {
            contract: FACTORY_THOUGHT_CONSUMPTION_CONTRACT.into(),
            command_id: "command:consume-1".into(),
            consumption,
            observations,
        }
    }

    fn performance(evidence_ref: &str) -> VakPerformanceEvent {
        let attempt = FactoryVakAttempt {
            unit_ref: "unit:inspect".into(),
            attempt_index: 0,
            current: true,
            execution_ref: "execution:1".into(),
            actor_ref: "agent:epii".into(),
            whole_ref: "whole:undertaking".into(),
            subject_ref: "subject:nara".into(),
            ql_binding_ref: "ql-binding:1".into(),
            ql_binding_revision: "ql-r1".into(),
            ai_kit_resolve_path_ref: "resolve:1".into(),
            context_resolution_ref: "context:1".into(),
            source_refs: BTreeSet::from(["source:undertaking".into()]),
            model_ref: "model:1".into(),
            provider_ref: "provider:1".into(),
            status: FactoryLegStatus::Returned,
            status_history: vec![FactoryLegStatus::Active, FactoryLegStatus::Returned],
            artifact_refs: BTreeSet::from(["artifact:1".into()]),
            late_artifact_refs: BTreeSet::new(),
            evidence_refs: BTreeSet::from([evidence_ref.into()]),
            failure_reason: None,
        };
        VakPerformanceEvent {
            contract: PERFORMANCE_EVENT_CONTRACT.into(),
            performance_ref: "performance:1".into(),
            observation: PerformanceObservation {
                observation_ref: "observation:1".into(),
                mode: PerformanceObservationMode::Live,
                replay_of: None,
            },
            factory_contract: FACTORY_VAK_PERFORMANCE_CONTRACT.into(),
            factory_receipt_refs: BTreeSet::from(["factory:receipt/1".into()]),
            ql_binding_ref: "ql-binding:1".into(),
            ql_binding_revision: "ql-r1".into(),
            ql_basis_refs: BTreeSet::from(["source:ql".into()]),
            semantics: PerformanceSemantics {
                profile_contract: "ql.vak-composition.profile/v1".into(),
                participation: Participation::AuthorisedUndertaking,
                content: ContentType::Operations,
                position: ContentPosition::Operation,
                context_frame: "CF5".into(),
                constitutional_voice: "Anima".into(),
                thread: ThreadForm::Single,
                sequence: ContextSequence::ThroughOperation,
                direction: InquiryDirection::Forward,
                musical_role: "single-voice".into(),
                musical_mode: "mixolydian".into(),
                musical_mode_index: 4,
                lens: "L0".into(),
                musical_basis: "chromatic".into(),
                frame_pitch: 7,
            },
            settled: true,
            has_failure: false,
            has_interruption: false,
            has_late_return: false,
            factory: FactoryVakPerformanceSnapshot {
                contract: FACTORY_VAK_PERFORMANCE_CONTRACT.into(),
                performance_ref: "performance:1".into(),
                run_ref: "run:1".into(),
                run_revision: 2,
                workflow_source_ref: "workflow:source".into(),
                workflow_source_revision: "workflow-r1".into(),
                workflow_source_digest: "digest:1".into(),
                actor_ref: "agent:epii".into(),
                subject_ref: "subject:nara".into(),
                whole_ref: "whole:undertaking".into(),
                ql_binding_ref: "ql-binding:1".into(),
                ql_binding_revision: "ql-r1".into(),
                ai_kit_resolve_path_ref: "resolve:1".into(),
                context_resolution_ref: "context:1".into(),
                source_refs: BTreeSet::from(["source:undertaking".into()]),
                frame: "CF5".into(),
                thread: "CFP0".into(),
                sequence: "CS2".into(),
                direction: "forward".into(),
                musical_role: "single-voice".into(),
                attempts: vec![attempt],
                chain_inputs: Vec::new(),
                sustained_stop: None,
            },
            standing: "test event".into(),
        }
    }

    fn request(
        meaning: ThoughtMeaning,
        use_kind: FactoryThoughtUseKind,
        with_performance: bool,
        human_response: bool,
    ) -> ThoughtConsumptionProjectionRequest {
        let evidence = "evidence:performance";
        ThoughtConsumptionProjectionRequest {
            schema: THOUGHT_CONSUMPTION_PROJECTION_REQUEST.into(),
            receipt: receipt(meaning, use_kind, human_response, evidence),
            interpretations: vec![ThoughtMeaningSelection {
                thought_id: "thought-local".into(),
                meaning,
            }],
            performance_evidence_refs: if with_performance {
                BTreeSet::from([evidence.into()])
            } else {
                Default::default()
            },
            performance: with_performance.then(|| performance(evidence)),
        }
    }

    #[test]
    fn all_twelve_source_meanings_remain_distinct_and_addressable() {
        let refs = ThoughtMeaning::ALL
            .into_iter()
            .map(ThoughtMeaning::inventory_ref)
            .collect::<BTreeSet<_>>();
        assert_eq!(refs.len(), 12);
        assert!(refs.contains("thought:T0"));
        assert!(refs.contains("thought:T5′"));
        assert_eq!(
            ThoughtMeaning::ALL
                .into_iter()
                .filter(|meaning| meaning.retrospective())
                .count(),
            6
        );
        assert_eq!(
            ThoughtMeaning::Question.meaning(),
            "Question: articulate what is open."
        );
        assert_eq!(
            ThoughtMeaning::Integration.meaning(),
            "Integration: work through how it belongs and returns."
        );
    }

    #[test]
    fn consumption_binds_exact_thought_source_and_actual_performance_evidence() {
        let reading = project_thought_consumption(request(
            ThoughtMeaning::Pattern,
            FactoryThoughtUseKind::Evaluation,
            true,
            true,
        ))
        .unwrap();
        assert_eq!(reading.contract, THOUGHT_CONSUMPTION_READING_CONTRACT);
        assert_eq!(reading.performance_ref.as_deref(), Some("performance:1"));
        assert_eq!(reading.interpretations[0].inventory_ref, "thought:T3");
        assert_eq!(reading.human_response[0].reference, "human:response/1");
        assert!(reading.recognition_candidates.is_empty());
        assert!(reading.ql_basis_refs.contains("thought:T3"));
    }

    #[test]
    fn recognition_is_only_an_evidence_bearing_native_owner_proposal() {
        let reading = project_thought_consumption(request(
            ThoughtMeaning::Affordance,
            FactoryThoughtUseKind::RecognisedPraxis,
            true,
            true,
        ))
        .unwrap();
        let candidate = &reading.recognition_candidates[0];
        assert_eq!(candidate.operation, "=");
        assert_eq!(candidate.horizon, "@5/R#");
        assert_eq!(candidate.target_ref, "skill-candidate:revisit");
        assert!(candidate.source_thought_refs.contains("thought:T2′"));
        assert!(
            candidate
                .recognition_evidence_refs
                .contains("human:response/1")
        );
        assert!(
            candidate
                .verification_evidence_refs
                .contains("evidence:performance")
        );
        assert!(candidate.standing.contains("AIKit"));
    }

    #[test]
    fn missing_human_response_is_preserved_as_absence_not_fabricated_feedback() {
        let reading = project_thought_consumption(request(
            ThoughtMeaning::Insight,
            FactoryThoughtUseKind::Context,
            false,
            false,
        ))
        .unwrap();
        assert!(reading.human_response.is_empty());
        assert!(reading.performance_ref.is_none());
    }

    #[test]
    fn mismatched_typed_interpretation_is_rejected() {
        let mut request = request(
            ThoughtMeaning::Pattern,
            FactoryThoughtUseKind::Evaluation,
            false,
            false,
        );
        request.receipt.consumption.inputs[0].interpretation =
            Some(source(&ThoughtMeaning::Anomaly.inventory_ref()));
        assert!(project_thought_consumption(request).is_err());
    }

    #[test]
    fn performance_relation_requires_shared_actual_evidence_and_same_run() {
        let mut foreign_evidence = request(
            ThoughtMeaning::Trace,
            FactoryThoughtUseKind::Evaluation,
            true,
            false,
        );
        foreign_evidence.performance_evidence_refs = BTreeSet::from(["evidence:foreign".into()]);
        assert!(project_thought_consumption(foreign_evidence).is_err());

        let mut foreign_run = request(
            ThoughtMeaning::Trace,
            FactoryThoughtUseKind::Evaluation,
            true,
            false,
        );
        foreign_run.performance.as_mut().unwrap().factory.run_ref = "run:other".into();
        assert!(project_thought_consumption(foreign_run).is_err());
    }

    #[test]
    fn non_current_source_observation_cannot_be_relabelled_as_consumed_learning() {
        let mut request = request(
            ThoughtMeaning::Lacuna,
            FactoryThoughtUseKind::ContinuingQuestion,
            false,
            false,
        );
        request.receipt.observations[0] = FactoryThoughtSourceObservation::Changed {
            observed: source("changed:source"),
        };
        assert!(project_thought_consumption(request).is_err());
    }
}
