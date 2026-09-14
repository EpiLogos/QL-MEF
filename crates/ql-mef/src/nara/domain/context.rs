use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::nara::SourceRevision;

use super::{
    CONTEXT_BRANCH_COUNT, EvidenceStanding, ProtectedRef, check_refs, check_source, check_text,
};

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
    pub fn validate(&self, branch: ContextBranchKind) -> Result<(), String> {
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
            .is_some_and(|value| !value.is_finite() || !(0.0..=1.0).contains(&value))
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
