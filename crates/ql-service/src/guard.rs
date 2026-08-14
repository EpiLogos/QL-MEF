use ql_semantic::{Operation, ProviderCapabilities, ProviderState, QlProvider};

use crate::{QlService, ServiceError};

pub(crate) fn provider_for(
    service: &QlService,
    operation: Operation,
) -> Result<(&dyn QlProvider, ProviderCapabilities), ServiceError> {
    let provider = service.provider.as_deref().ok_or(ServiceError::ProviderAbsent)?;
    let capabilities = provider.capabilities();
    match capabilities.health.state {
        ProviderState::Absent => return Err(ServiceError::ProviderAbsent),
        ProviderState::Incompatible => {
            return Err(ServiceError::ProviderIncompatible(capabilities.health.detail.clone()));
        }
        ProviderState::Available | ProviderState::Degraded => {}
    }
    if !capabilities.supports(operation) {
        return Err(ServiceError::UnsupportedOperation(operation));
    }
    Ok((provider, capabilities))
}
