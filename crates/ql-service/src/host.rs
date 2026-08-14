use ql_semantic::{Operation, ProviderHealth, ProviderState};

use crate::{CapabilityDecision, QlService, ServiceCapabilities};

impl QlService {
    pub fn capabilities(&self) -> ServiceCapabilities {
        match self.provider.as_deref() {
            None => ServiceCapabilities { health: ProviderHealth::absent(), provider: None },
            Some(provider) => {
                let capabilities = provider.capabilities();
                ServiceCapabilities { health: capabilities.health.clone(), provider: Some(capabilities) }
            }
        }
    }

    pub fn negotiate(&self, operation: Operation) -> CapabilityDecision {
        if operation == Operation::Capabilities {
            return CapabilityDecision { health: self.capabilities().health, operation, supported: true, deterministic: true };
        }
        let Some(provider) = self.provider.as_deref() else {
            return CapabilityDecision { health: ProviderHealth::absent(), operation, supported: false, deterministic: false };
        };
        let capabilities = provider.capabilities();
        let usable = matches!(capabilities.health.state, ProviderState::Available | ProviderState::Degraded);
        CapabilityDecision {
            health: capabilities.health.clone(),
            operation,
            supported: usable && capabilities.supports(operation),
            deterministic: usable && capabilities.is_deterministic(operation),
        }
    }
}
