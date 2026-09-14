use serde::{Deserialize, Serialize};

use super::{check_refs, check_text};

/// A source-defined capability office is never silently dropped. It is either
/// backed by attributable references or carries an explicit reason why that
/// office has no current implementation/evidence for this Nara occasion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityRefs {
    pub refs: Vec<String>,
    pub absence_reason: Option<String>,
}

impl CapabilityRefs {
    pub fn present(refs: Vec<String>) -> Result<Self, String> {
        let value = Self {
            refs,
            absence_reason: None,
        };
        value.validate()?;
        Ok(value)
    }

    pub fn absent(reason: impl Into<String>) -> Result<Self, String> {
        let value = Self {
            refs: Vec::new(),
            absence_reason: Some(reason.into()),
        };
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<(), String> {
        check_refs(&self.refs, "capability reference", 4096)?;
        match (self.refs.is_empty(), &self.absence_reason) {
            (false, None) => Ok(()),
            (true, Some(reason)) => check_text(reason, "capability absence reason"),
            (false, Some(_)) => Err("capability cannot be both present and absent".into()),
            (true, None) => Err("capability requires references or explicit absence".into()),
        }
    }

    pub fn is_present(&self) -> bool {
        !self.refs.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absence_is_explicit_instead_of_an_empty_silent_office() {
        assert!(CapabilityRefs::absent("source not available").is_ok());
        assert!(
            CapabilityRefs {
                refs: Vec::new(),
                absence_reason: None,
            }
            .validate()
            .is_err()
        );
    }
}
