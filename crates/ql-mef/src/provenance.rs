use core::fmt;
use core::str::FromStr;

use crate::{ClientRef, MEF_REGISTRY_VERSION, MefError};

pub const CONTRACT_SCHEMA_VERSION: &str = "1.1.0";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QlProviderRef {
    pub provider: String,
    pub version: String,
}

impl QlProviderRef {
    pub fn new(provider: impl Into<String>, version: impl Into<String>) -> Result<Self, MefError> {
        let provider = provider.into();
        let version = version.into();
        if provider.trim().is_empty() {
            return Err(MefError::EmptyProviderId);
        }
        if version.trim().is_empty() {
            return Err(MefError::EmptyProviderVersion);
        }
        Ok(Self { provider, version })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResultClass {
    Canonical,
    Deterministic,
    SemanticStochastic,
    Research,
}

impl ResultClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Canonical => "canonical",
            Self::Deterministic => "deterministic",
            Self::SemanticStochastic => "semantic-stochastic",
            Self::Research => "research",
        }
    }
}

impl fmt::Display for ResultClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for ResultClass {
    type Err = MefError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "canonical" => Ok(Self::Canonical),
            "deterministic" => Ok(Self::Deterministic),
            "semantic-stochastic" => Ok(Self::SemanticStochastic),
            "research" => Ok(Self::Research),
            other => Err(MefError::UnknownResultClass(other.to_owned())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputRefRevision {
    pub reference: ClientRef,
    pub revision: Option<String>,
}

impl InputRefRevision {
    pub fn new(reference: ClientRef, revision: Option<String>) -> Self {
        Self {
            reference,
            revision,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QlProvenance {
    pub schema_version: &'static str,
    pub mef_registry_version: &'static str,
    pub provider: QlProviderRef,
    pub operation: String,
    pub input_refs: Vec<InputRefRevision>,
    pub model: Option<String>,
    pub config_ref: Option<ClientRef>,
    pub result_class: ResultClass,
    pub warnings: Vec<String>,
}

impl QlProvenance {
    pub fn new(
        provider: QlProviderRef,
        operation: impl Into<String>,
        input_refs: Vec<InputRefRevision>,
        result_class: ResultClass,
    ) -> Self {
        Self {
            schema_version: CONTRACT_SCHEMA_VERSION,
            mef_registry_version: MEF_REGISTRY_VERSION,
            provider,
            operation: operation.into(),
            input_refs,
            model: None,
            config_ref: None,
            result_class,
            warnings: Vec::new(),
        }
    }
}
