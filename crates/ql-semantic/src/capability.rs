use ql_core::QlFormRef;
use ql_mef::{LensRef, QlProviderRef};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProviderClass {
    FormalKernel,
    SemanticRefraction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProviderState {
    Absent,
    Available,
    Degraded,
    Incompatible,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderHealth {
    pub state: ProviderState,
    pub detail: Option<String>,
}

impl ProviderHealth {
    pub const fn available() -> Self {
        Self {
            state: ProviderState::Available,
            detail: None,
        }
    }

    pub const fn absent() -> Self {
        Self {
            state: ProviderState::Absent,
            detail: None,
        }
    }

    pub fn degraded(detail: impl Into<String>) -> Self {
        Self {
            state: ProviderState::Degraded,
            detail: Some(detail.into()),
        }
    }

    pub fn incompatible(detail: impl Into<String>) -> Self {
        Self {
            state: ProviderState::Incompatible,
            detail: Some(detail.into()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operation {
    Capabilities,
    Locate,
    Refract,
    Relate,
    Synthesise,
}

impl Operation {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Capabilities => "capabilities",
            Self::Locate => "locate",
            Self::Refract => "refract",
            Self::Relate => "relate",
            Self::Synthesise => "synthesise",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputLimits {
    pub max_relation_subjects: usize,
    pub max_synthesis_readings: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderCapabilities {
    pub provider: QlProviderRef,
    pub health: ProviderHealth,
    pub classes: Vec<ProviderClass>,
    pub supported_forms: Vec<QlFormRef>,
    pub supported_lenses: Vec<LensRef>,
    pub operations: Vec<Operation>,
    pub extension_namespaces: Vec<String>,
    pub deterministic_operations: Vec<Operation>,
    pub input_limits: InputLimits,
    pub output_schema_versions: Vec<String>,
}

impl ProviderCapabilities {
    pub fn supports(&self, operation: Operation) -> bool {
        self.operations.contains(&operation)
    }

    pub fn is_deterministic(&self, operation: Operation) -> bool {
        self.deterministic_operations.contains(&operation)
    }
}
