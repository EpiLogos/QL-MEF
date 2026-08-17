use ql_semantic::{Operation, ProviderCapabilities, ProviderHealth};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceCapabilities {
    pub health: ProviderHealth,
    pub provider: Option<ProviderCapabilities>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityDecision {
    pub health: ProviderHealth,
    pub operation: Operation,
    pub supported: bool,
    pub deterministic: bool,
}
