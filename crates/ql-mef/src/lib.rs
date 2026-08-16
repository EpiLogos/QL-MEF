//! Complete executable MEF manifold and identity-preserving refraction contracts.
//!
//! Q2 keeps deterministic registry topology separate from semantic/stochastic readings.

mod coordinate;
mod error;
mod identity;
mod lens;
mod provenance;
mod reading;
mod refraction;
mod registry;
mod sublens;

pub use coordinate::{MEF_ROTATION_VERSION, MefRotation, MefUnitFace};
pub use error::MefError;
pub use identity::{ClientRef, QlTarget};
pub use lens::{LensFace, LensId, LensRef, MEF_REGISTRY_REVISION, MEF_REGISTRY_VERSION, MefSquare};
pub use provenance::{
    CONTRACT_SCHEMA_VERSION, InputRefRevision, QlProvenance, QlProviderRef, ResultClass,
};
pub use reading::{QlReading, QlRelationReading, QlSynthesis};
pub use refraction::RefractionContract;
pub use registry::{
    LENS_DEFINITIONS, LensDefinition, SublensDefinition, all_lens_definitions,
    all_sublens_definitions, lens_definition,
};
pub use sublens::SublensRef;
