use crate::{LocateRequest, LocateResult, ProviderCapabilities, ProviderError};
use crate::{RefractRequest, RelateRequest, SynthesiseRequest};
use crate::{SemanticReading, SemanticRelationReading, SemanticSynthesis};

pub trait QlProvider {
    fn capabilities(&self) -> ProviderCapabilities;
    fn locate(&self, request: LocateRequest) -> Result<LocateResult, ProviderError>;
    fn refract(&self, request: RefractRequest) -> Result<SemanticReading, ProviderError>;
    fn relate(&self, request: RelateRequest) -> Result<SemanticRelationReading, ProviderError>;
    fn synthesise(&self, request: SynthesiseRequest) -> Result<SemanticSynthesis, ProviderError>;
}
