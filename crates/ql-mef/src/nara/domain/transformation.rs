use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::nara::SourceRevision;

use super::{EvidenceStanding, check_refs, check_source, check_text};

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
        check_refs(
            &self.operation_refs,
            "transformation operation reference",
            7,
        )?;
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
        check_refs(
            &self.feedback_refs,
            "transformation feedback reference",
            256,
        )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_defined_phase_extents_remain_reachable() {
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
                source_revisions: vec![SourceRevision {
                    source_ref: "source:transformation".into(),
                    revision: "r1".into(),
                    standing_ref: "source".into(),
                }],
                opened_at_unix_ms: 1,
                closed_at_unix_ms: Some(2),
                safety: TransformationSafety::Clear,
                feedback_refs: Vec::new(),
            }],
            standing: EvidenceStanding::Source,
        };
        history.validate().unwrap();
    }
}
