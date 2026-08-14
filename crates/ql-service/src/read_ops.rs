use ql_semantic::{LocateRequest, LocateResult, Operation, RefractRequest, SemanticReading};

use crate::{guard::provider_for, QlService, ServiceError};

impl QlService {
    pub fn locate(&self, request: LocateRequest) -> Result<LocateResult, ServiceError> {
        let (provider, _) = provider_for(self, Operation::Locate)?;
        Ok(provider.locate(request)?)
    }

    pub fn refract(&self, request: RefractRequest) -> Result<SemanticReading, ServiceError> {
        let (provider, _) = provider_for(self, Operation::Refract)?;
        Ok(provider.refract(request)?)
    }
}
