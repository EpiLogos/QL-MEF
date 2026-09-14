use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::nara::SourceRevision;

pub const M4_DOMAIN_CONTRACT: &str = "ql.nara-m4-domain/v1";
pub const IDENTITY_SLOT_COUNT: usize = 6;
pub const CENTRE_COUNT: usize = 7;
pub const CONTEXT_BRANCH_COUNT: usize = 6;
pub const INTEGRATION_OFFICE_COUNT: usize = 6;
pub const ELEMENT_COUNT: usize = 4;

fn valid_text(value: &str) -> bool {
    !value.trim().is_empty() && value.len() <= 4096 && !value.chars().any(char::is_control)
}

pub(crate) fn check_text(value: &str, label: &str) -> Result<(), String> {
    if valid_text(value) {
        Ok(())
    } else {
        Err(format!("invalid {label}"))
    }
}

pub(crate) fn check_source(source: &SourceRevision) -> Result<(), String> {
    check_text(&source.source_ref, "source reference")?;
    check_text(&source.revision, "source revision")?;
    check_text(&source.standing_ref, "source standing")
}

pub(crate) fn check_refs(values: &[String], label: &str, max: usize) -> Result<(), String> {
    if values.len() > max {
        return Err(format!("too many {label}"));
    }
    let mut unique = BTreeSet::new();
    for value in values {
        check_text(value, label)?;
        if !unique.insert(value.as_str()) {
            return Err(format!("duplicate {label}"));
        }
    }
    Ok(())
}

pub(crate) fn check_optional_finite(value: Option<f64>, label: &str) -> Result<(), String> {
    if value.is_some_and(|number| !number.is_finite()) {
        Err(format!("non-finite {label}"))
    } else {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvidenceStanding {
    Source,
    AuthoredArchitecture,
    Implementation,
    Observed,
    Derived,
    Reported,
    Proposed,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProtectedRef {
    pub ref_id: String,
    pub revision: String,
    pub owner_ref: String,
}

impl ProtectedRef {
    pub fn validate(&self) -> Result<(), String> {
        check_text(&self.ref_id, "protected reference")?;
        check_text(&self.revision, "protected revision")?;
        check_text(&self.owner_ref, "protected owner")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum M4Branch {
    Identity,
    Embodied,
    Oracle,
    Transformation,
    Context,
    Integration,
}

impl M4Branch {
    pub const fn coordinate(self) -> &'static str {
        match self {
            Self::Identity => "M4.0",
            Self::Embodied => "M4.1",
            Self::Oracle => "M4.2",
            Self::Transformation => "M4.3",
            Self::Context => "M4.4",
            Self::Integration => "M4.5",
        }
    }
}
