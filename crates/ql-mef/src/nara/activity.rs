//! Nara reception of Central-owned Day/NOW activity and source history.
//!
//! Central remains the editable temporal/source authority. This module carries
//! stable handles, exact source spans and reception/interpretation receipts; it
//! does not create a second journal, transcript or T/T' file store.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::SourceRevision;
use super::domain::{Aw3ThoughtConsumptionReception, EvidenceStanding, ProtectedRef};

pub const NARA_ACTIVITY_CONTRACT: &str = "ql.nara-activity/v1";

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
    text(&source.source_ref, "activity source")?;
    text(&source.revision, "activity source revision")?;
    text(&source.standing_ref, "activity source standing")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TemporalScope {
    RootMetaProject,
    Project,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CentralTemporalRefs {
    pub owner_ref: String,
    pub contract_ref: String,
    pub scope: TemporalScope,
    pub project_ref: Option<String>,
    pub day_ref: String,
    pub day_revision: String,
    pub now_ref: String,
    pub now_revision: String,
    pub flow_dialogue_refs: Vec<String>,
    pub source_history_ref: String,
}

impl CentralTemporalRefs {
    pub fn validate(&self) -> Result<(), String> {
        text(&self.owner_ref, "temporal owner")?;
        text(&self.contract_ref, "temporal contract")?;
        if self.owner_ref != "central" && !self.owner_ref.starts_with("central:") {
            return Err("Nara temporal handles must remain Central-owned".into());
        }
        match (self.scope, &self.project_ref) {
            (TemporalScope::RootMetaProject, None) => {}
            (TemporalScope::Project, Some(project)) => text(project, "Central project")?,
            (TemporalScope::RootMetaProject, Some(_)) => {
                return Err("root meta-project temporal scope cannot claim a child project".into());
            }
            (TemporalScope::Project, None) => {
                return Err("Project temporal scope requires its Central project reference".into());
            }
        }
        text(&self.day_ref, "Central Day")?;
        text(&self.day_revision, "Central Day revision")?;
        text(&self.now_ref, "Central NOW")?;
        text(&self.now_revision, "Central NOW revision")?;
        refs(&self.flow_dialogue_refs, "Flow/Dialogue reference", 256)?;
        text(&self.source_history_ref, "Central source-history reference")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ActivityActorKind {
    Human,
    Agent,
    Provider,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProtectionClass {
    Private,
    Protected,
    ShareableDerived,
    Public,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceSpan {
    pub start_byte: u64,
    pub end_byte: u64,
    pub kind_ref: String,
    pub confidence: Option<f64>,
    pub model_ref: Option<String>,
    pub protection: ProtectionClass,
}

impl SourceSpan {
    pub fn validate(&self, source_len_bytes: Option<u64>) -> Result<(), String> {
        if self.start_byte >= self.end_byte {
            return Err("source span must have positive byte length".into());
        }
        if source_len_bytes.is_some_and(|length| self.end_byte > length) {
            return Err("source span exceeds the retained source length".into());
        }
        text(&self.kind_ref, "source-span kind")?;
        if self
            .confidence
            .is_some_and(|confidence| !confidence.is_finite() || !(0.0..=1.0).contains(&confidence))
        {
            return Err("source-span confidence must be finite and within 0..=1".into());
        }
        if let Some(model) = &self.model_ref {
            text(model, "source-span model")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParseDisposition {
    Parsed,
    Abstained,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParseReceipt {
    pub disposition: ParseDisposition,
    pub model_ref: Option<String>,
    pub model_revision: Option<String>,
    pub source_len_bytes: Option<u64>,
    pub spans: Vec<SourceSpan>,
    pub abstention_reason: Option<String>,
    pub evidence_refs: Vec<String>,
}

impl ParseReceipt {
    pub fn validate(&self) -> Result<(), String> {
        match self.disposition {
            ParseDisposition::Parsed => {
                if self.model_ref.is_none() || self.model_revision.is_none() {
                    return Err("parsed activity requires model identity and revision".into());
                }
                if self.abstention_reason.is_some() {
                    return Err("parsed activity cannot also be abstained".into());
                }
            }
            ParseDisposition::Abstained => {
                if self.spans.is_empty() {
                    if let Some(reason) = &self.abstention_reason {
                        text(reason, "parse abstention reason")?;
                    } else {
                        return Err("abstained parsing requires an explicit reason".into());
                    }
                } else {
                    return Err("abstained parsing cannot emit inferred spans".into());
                }
            }
        }
        if let Some(model) = &self.model_ref {
            text(model, "parse model")?;
        }
        if let Some(revision) = &self.model_revision {
            text(revision, "parse model revision")?;
        }
        if self.spans.len() > 4096 {
            return Err("activity parse exceeds span bounds".into());
        }
        for span in &self.spans {
            span.validate(self.source_len_bytes)?;
        }
        let mut ordered = self.spans.iter().collect::<Vec<_>>();
        ordered.sort_by_key(|span| (span.start_byte, span.end_byte));
        for pair in ordered.windows(2) {
            if pair[0].end_byte > pair[1].start_byte {
                return Err("activity parse spans overlap".into());
            }
        }
        refs(&self.evidence_refs, "parse evidence reference", 256)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActivityOccurrence {
    pub activity_ref: String,
    pub subject_id: String,
    pub actor_kind: ActivityActorKind,
    pub actor_ref: String,
    pub source: SourceRevision,
    pub original_bytes_ref: ProtectedRef,
    pub occurrence_at_unix_ms: u64,
    pub receipt_at_unix_ms: u64,
    pub human_authored: bool,
    pub parse: Option<ParseReceipt>,
    pub identity_proposal_ref: Option<String>,
    pub evidence_refs: Vec<String>,
    pub standing: EvidenceStanding,
}

impl ActivityOccurrence {
    pub fn validate(&self) -> Result<(), String> {
        text(&self.activity_ref, "activity reference")?;
        text(&self.subject_id, "activity subject")?;
        text(&self.actor_ref, "activity actor")?;
        source(&self.source)?;
        self.original_bytes_ref.validate()?;
        if self.receipt_at_unix_ms < self.occurrence_at_unix_ms {
            return Err("activity receipt precedes occurrence".into());
        }
        if self.human_authored && self.actor_kind != ActivityActorKind::Human {
            return Err(
                "Agent/provider output cannot be relabelled human-authored activity".into(),
            );
        }
        if let Some(parse) = &self.parse {
            parse.validate()?;
        }
        if let Some(reference) = &self.identity_proposal_ref {
            text(reference, "identity proposal")?;
        }
        refs(&self.evidence_refs, "activity evidence reference", 256)
    }

    pub const fn is_late_return(&self) -> bool {
        self.receipt_at_unix_ms > self.occurrence_at_unix_ms
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThoughtConsumptionRef {
    /// Compatibility handle over the landed typed AW3 reception. The earlier
    /// free-form owner/result/source fields are deliberately gone: the typed
    /// producer receipt is the authority and `thought_ref` is its stable
    /// consumption identity for existing Nara mutation helpers.
    pub thought_ref: String,
    pub reception: Aw3ThoughtConsumptionReception,
}

impl ThoughtConsumptionRef {
    pub fn from_reception(reception: Aw3ThoughtConsumptionReception) -> Result<Self, String> {
        reception.validate()?;
        let value = Self {
            thought_ref: reception.consumption_id.clone(),
            reception,
        };
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<(), String> {
        text(&self.thought_ref, "thought consumption")?;
        self.reception.validate()?;
        if self.thought_ref != self.reception.consumption_id {
            return Err("thought handle differs from its landed AW3 consumption identity".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NaraActivityLog {
    pub schema: String,
    pub subject_id: String,
    pub temporal: CentralTemporalRefs,
    pub occurrences: Vec<ActivityOccurrence>,
    pub thought_consumptions: Vec<ThoughtConsumptionRef>,
    pub identity_proposal_refs: Vec<String>,
}

impl NaraActivityLog {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != NARA_ACTIVITY_CONTRACT {
            return Err("unsupported Nara activity contract".into());
        }
        text(&self.subject_id, "activity-log subject")?;
        self.temporal.validate()?;
        if self.occurrences.len() > 65_536 {
            return Err("activity log exceeds occurrence bounds".into());
        }
        let mut activity_refs = BTreeSet::new();
        for occurrence in &self.occurrences {
            occurrence.validate()?;
            if occurrence.subject_id != self.subject_id {
                return Err("activity occurrence belongs to another Nara".into());
            }
            if !activity_refs.insert(occurrence.activity_ref.as_str()) {
                return Err("duplicate activity reference".into());
            }
        }
        if self.thought_consumptions.len() > 65_536 {
            return Err("activity log exceeds thought-consumption bounds".into());
        }
        let mut thought_refs = BTreeSet::new();
        for thought in &self.thought_consumptions {
            thought.validate()?;
            let reception = &thought.reception;
            if reception.subject_id != self.subject_id
                || reception.day_ref != self.temporal.day_ref
                || reception.day_revision != self.temporal.day_revision
                || reception.now_ref != self.temporal.now_ref
                || reception.now_revision != self.temporal.now_revision
            {
                return Err("typed AW3 reception is not in this Nara activity Day/NOW".into());
            }
            if !thought_refs.insert(thought.thought_ref.as_str()) {
                return Err("duplicate AW3 thought-consumption receipt".into());
            }
        }
        refs(
            &self.identity_proposal_refs,
            "identity proposal reference",
            4096,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source_ref() -> SourceRevision {
        SourceRevision {
            source_ref: "central:source:day-entry".into(),
            revision: "r7".into(),
            standing_ref: "authored".into(),
        }
    }

    fn protected(name: &str) -> ProtectedRef {
        ProtectedRef {
            ref_id: format!("central:protected:{name}"),
            revision: "r1".into(),
            owner_ref: "central".into(),
        }
    }

    fn occurrence(actor_kind: ActivityActorKind, human_authored: bool) -> ActivityOccurrence {
        ActivityOccurrence {
            activity_ref: "activity:1".into(),
            subject_id: "nara-a".into(),
            actor_kind,
            actor_ref: "actor:1".into(),
            source: source_ref(),
            original_bytes_ref: protected("bytes"),
            occurrence_at_unix_ms: 10,
            receipt_at_unix_ms: 50,
            human_authored,
            parse: Some(ParseReceipt {
                disposition: ParseDisposition::Parsed,
                model_ref: Some("model:parser".into()),
                model_revision: Some("r3".into()),
                source_len_bytes: Some(12),
                spans: vec![SourceSpan {
                    start_byte: 0,
                    end_byte: 5,
                    kind_ref: "dream-image".into(),
                    confidence: Some(0.8),
                    model_ref: Some("model:parser".into()),
                    protection: ProtectionClass::Private,
                }],
                abstention_reason: None,
                evidence_refs: vec!["evidence:parse".into()],
            }),
            identity_proposal_ref: None,
            evidence_refs: Vec::new(),
            standing: EvidenceStanding::Source,
        }
    }

    #[test]
    fn occurrence_and_receipt_remain_distinct_for_late_return() {
        let value = occurrence(ActivityActorKind::Human, true);
        value.validate().unwrap();
        assert!(value.is_late_return());
        assert_eq!(value.occurrence_at_unix_ms, 10);
        assert_eq!(value.receipt_at_unix_ms, 50);
    }

    #[test]
    fn agent_output_cannot_become_human_activity_by_label() {
        let value = occurrence(ActivityActorKind::Agent, true);
        assert_eq!(
            value.validate().unwrap_err(),
            "Agent/provider output cannot be relabelled human-authored activity"
        );
    }

    #[test]
    fn parser_may_abstain_without_fabricating_spans() {
        let receipt = ParseReceipt {
            disposition: ParseDisposition::Abstained,
            model_ref: Some("model:parser".into()),
            model_revision: Some("r3".into()),
            source_len_bytes: Some(100),
            spans: Vec::new(),
            abstention_reason: Some("insufficient evidence".into()),
            evidence_refs: Vec::new(),
        };
        receipt.validate().unwrap();
    }

    #[test]
    fn overlapping_inferred_spans_are_refused() {
        let receipt = ParseReceipt {
            disposition: ParseDisposition::Parsed,
            model_ref: Some("model:parser".into()),
            model_revision: Some("r3".into()),
            source_len_bytes: Some(20),
            spans: vec![
                SourceSpan {
                    start_byte: 0,
                    end_byte: 10,
                    kind_ref: "a".into(),
                    confidence: Some(0.5),
                    model_ref: Some("model:parser".into()),
                    protection: ProtectionClass::Private,
                },
                SourceSpan {
                    start_byte: 5,
                    end_byte: 15,
                    kind_ref: "b".into(),
                    confidence: Some(0.5),
                    model_ref: Some("model:parser".into()),
                    protection: ProtectionClass::Private,
                },
            ],
            abstention_reason: None,
            evidence_refs: Vec::new(),
        };
        assert_eq!(
            receipt.validate().unwrap_err(),
            "activity parse spans overlap"
        );
    }
}
