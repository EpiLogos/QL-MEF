//! Transport-independent provider contracts for QL/MEF formal and semantic operations.

mod capability;
mod error;
mod provider;
mod request;
mod result;

pub use capability::{InputLimits, Operation, ProviderCapabilities, ProviderClass, ProviderHealth, ProviderState};
pub use error::ProviderError;
pub use provider::QlProvider;
pub use request::{LocateRequest, RefractRequest, RelateRequest, SynthesiseRequest};
pub use result::{LocateResult, LocateStatus, SemanticDisclosure, SemanticReading, SemanticRelationReading, SemanticStatus, SemanticSynthesis, SynthesisDisclosure};
