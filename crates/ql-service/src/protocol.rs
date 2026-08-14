use ql_semantic::{LocateRequest, LocateResult, RefractRequest, RelateRequest, SynthesiseRequest};
use ql_semantic::{SemanticReading, SemanticRelationReading, SemanticSynthesis};

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
