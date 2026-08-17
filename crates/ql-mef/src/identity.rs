use core::fmt;

use crate::MefError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClientRef(String);

impl ClientRef {
    pub fn new(value: impl Into<String>) -> Result<Self, MefError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(MefError::EmptyClientRef);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ClientRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QlTarget {
    pub subject: ClientRef,
    pub subject_type: Option<String>,
    pub frame_ref: Option<String>,
    pub context_refs: Vec<ClientRef>,
}

impl QlTarget {
    pub fn new(subject: ClientRef) -> Self {
        Self {
            subject,
            subject_type: None,
            frame_ref: None,
            context_refs: Vec::new(),
        }
    }
}
