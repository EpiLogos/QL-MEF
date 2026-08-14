use ql_semantic::{LocateRequest, LocateResult, RefractRequest, RelateRequest, SynthesiseRequest};
use ql_semantic::{SemanticReading, SemanticRelationReading, SemanticSynthesis};

use crate::{QlService, ServiceCapabilities, ServiceError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceRequest {
    Capabilities,
    Locate(LocateRequest),
    Refract(RefractRequest),
    Relate(RelateRequest),
    Synthesise(SynthesiseRequest),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceResponse<C> {
    Capabilities(C),
    Locate(LocateResult),
    Refract(SemanticReading),
    Relate(SemanticRelationReading),
    Synthesise(SemanticSynthesis),
}

impl QlService {
    pub fn dispatch(
        &self,
        request: ServiceRequest,
    ) -> Result<ServiceResponse<ServiceCapabilities>, ServiceError> {
        match request {
            ServiceRequest::Capabilities => Ok(ServiceResponse::Capabilities(self.capabilities())),
            ServiceRequest::Locate(value) => self.locate(value).map(ServiceResponse::Locate),
            ServiceRequest::Refract(value) => self.refract(value).map(ServiceResponse::Refract),
            ServiceRequest::Relate(value) => self.relate(value).map(ServiceResponse::Relate),
            ServiceRequest::Synthesise(value) => {
                self.synthesise(value).map(ServiceResponse::Synthesise)
            }
        }
    }
}
