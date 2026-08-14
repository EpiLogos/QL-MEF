use ql_semantic::{
    Operation, RelateRequest, SemanticRelationReading, SemanticSynthesis, SynthesiseRequest,
};

use crate::{guard::provider_for, QlService, ServiceError};

impl QlService {
    pub fn relate(&self, request: RelateRequest) -> Result<SemanticRelationReading, ServiceError> {
        let (provider, capabilities) = provider_for(self, Operation::Relate)?;
        let actual = request.subjects.len();
        let limit = capabilities.input_limits.max_relation_subjects;
        if actual > limit {
            return Err(ServiceError::InputLimitExceeded {
                operation: Operation::Relate,
                limit,
                actual,
            });
        }
        Ok(provider.relate(request)?)
    }

    pub fn synthesise(&self, request: SynthesiseRequest) -> Result<SemanticSynthesis, ServiceError> {
        let (provider, capabilities) = provider_for(self, Operation::Synthesise)?;
        let actual = request.readings.len();
        let limit = capabilities.input_limits.max_synthesis_readings;
        if actual > limit {
            return Err(ServiceError::InputLimitExceeded {
                operation: Operation::Synthesise,
                limit,
                actual,
            });
        }
        Ok(provider.synthesise(request)?)
    }
}
