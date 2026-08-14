use ql_semantic::{
    Operation, RelateRequest, SemanticRelationReading, SemanticSynthesis, SynthesiseRequest,
};

use crate::{QlService, ServiceError, guard::provider_for};

impl QlService {
    pub fn relate(&self, request: RelateRequest) -> Result<SemanticRelationReading, ServiceError> {
        let (provider, capabilities) = provider_for(self, Operation::Relate)?;
        let actual = request.subjects.len();
        if actual < 2 {
            return Err(ServiceError::InvalidRequest(
                "relate requires at least two subjects",
            ));
        }
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
        if actual == 0 {
            return Err(ServiceError::InvalidRequest(
                "synthesise requires at least one reading",
            ));
        }
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
