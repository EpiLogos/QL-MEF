use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::{INTEGRATION_OFFICE_COUNT, EvidenceStanding, ProtectedRef, check_refs, check_text};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IntegrationOffice {
    CurriculumMap,
    CoreEpiLogosVoice,
    MethodTransparencyLab,
    IntegrationLab,
    PedagogyLab,
    LogosCycleEngine,
}

impl IntegrationOffice {
    pub const fn coordinate(self) -> &'static str {
        match self {
            Self::CurriculumMap => "M4.5.0",
            Self::CoreEpiLogosVoice => "M4.5.1",
            Self::MethodTransparencyLab => "M4.5.2",
            Self::IntegrationLab => "M4.5.3",
            Self::PedagogyLab => "M4.5.4",
            Self::LogosCycleEngine => "M4.5.5",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntegrationReturn {
    pub return_ref: String,
    pub office: IntegrationOffice,
    pub input_refs: Vec<String>,
    pub source_refs: Vec<String>,
    pub method_ref: Option<String>,
    pub output_ref: ProtectedRef,
    pub evidence_refs: Vec<String>,
    pub human_response_ref: Option<ProtectedRef>,
    pub retention_policy_ref: String,
    pub standing: EvidenceStanding,
}

impl IntegrationReturn {
    pub fn validate(&self, office: IntegrationOffice) -> Result<(), String> {
        if self.office != office {
            return Err("integration Return is seated in the wrong M4.5 office".into());
        }
        check_text(&self.return_ref, "integration Return")?;
        check_refs(&self.input_refs, "integration input reference", 512)?;
        check_refs(&self.source_refs, "integration source reference", 512)?;
        if let Some(reference) = &self.method_ref {
            check_text(reference, "integration method")?;
        }
        self.output_ref.validate()?;
        check_refs(&self.evidence_refs, "integration evidence reference", 512)?;
        if let Some(reference) = &self.human_response_ref {
            reference.validate()?;
        }
        check_text(&self.retention_policy_ref, "integration retention policy")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntegrationOfficeState {
    pub office: IntegrationOffice,
    pub available: bool,
    pub unavailable_reason: Option<String>,
    pub returns: Vec<IntegrationReturn>,
}

impl IntegrationOfficeState {
    pub fn validate(&self) -> Result<(), String> {
        if self.available && self.unavailable_reason.is_some() {
            return Err("available M4.5 office cannot carry an unavailable reason".into());
        }
        if !self.available && self.unavailable_reason.is_none() {
            return Err("unavailable M4.5 office requires a reason".into());
        }
        if let Some(reason) = &self.unavailable_reason {
            check_text(reason, "M4.5 unavailable reason")?;
        }
        if self.returns.len() > 4096 {
            return Err("too many Returns in one M4.5 office".into());
        }
        let mut ids = BTreeSet::new();
        for returned in &self.returns {
            returned.validate(self.office)?;
            if !ids.insert(returned.return_ref.as_str()) {
                return Err("duplicate M4.5 Return reference".into());
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntegrationField {
    pub offices: Vec<IntegrationOfficeState>,
    /// #94/Factory/AIKit-owned operative Returns or recognised praxis remain
    /// external-owner objects rather than becoming a second Ta-Onta store here.
    pub operative_return_refs: Vec<String>,
}

impl IntegrationField {
    pub fn validate(&self) -> Result<(), String> {
        if self.offices.len() != INTEGRATION_OFFICE_COUNT {
            return Err("M4.5 requires all six integration offices".into());
        }
        let mut offices = BTreeSet::new();
        for office in &self.offices {
            office.validate()?;
            if !offices.insert(office.office) {
                return Err("duplicate M4.5 integration office".into());
            }
        }
        if offices.len() != INTEGRATION_OFFICE_COUNT {
            return Err("M4.5 integration office set is incomplete".into());
        }
        check_refs(
            &self.operative_return_refs,
            "operative Return reference",
            512,
        )
    }
}
