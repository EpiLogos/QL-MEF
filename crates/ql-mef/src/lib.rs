//! Complete executable MEF manifold and identity-preserving refraction contracts.
//!
//! Q2 keeps deterministic registry topology separate from semantic/stochastic readings.

mod context_frame;
mod coordinate;
mod error;
mod identity;
mod lens;
mod music;
mod provenance;
mod reading;
mod refraction;
mod registry;
mod sublens;

pub use context_frame::{
    CONTEXT_FRAME_GRAMMAR_VERSION, ContextFrameCoordinate, ContextFrameCut, ContextFrameId,
    ContextFrameSelection, MefFormCoordinate, MefGrain, canonical_context_frame_progression,
};
pub use coordinate::{MEF_ROTATION_VERSION, MefRotation, MefUnitFace};
pub use error::MefError;
pub use identity::{ClientRef, QlTarget};
pub use lens::{LensFace, LensId, LensRef, MEF_REGISTRY_REVISION, MEF_REGISTRY_VERSION, MefSquare};
pub use music::{
    EPOGDOON_FOLD_SEMANTICS, EPOGDOON_FOLD_SEMANTICS_PROVENANCE, HARMONIC_RELATIONS,
    HarmonicRatio, HarmonicRelation, M3_CLOCK_APERTURES, M3_CLOCK_APERTURE_EVIDENCE,
    M3_CLOCK_APERTURE_PROVENANCE, M3ClockAperture, MUSICAL_HARMONIC_VERSION,
    MusicalEvidenceClass, TONIC_CONTEXT_FRAME_EVIDENCE, TONIC_CONTEXT_FRAME_PROVENANCE,
    TonicContextFrame, epogdoon_72_to_64, epogdoon_preimage_width, harmonic_relation,
    m3_clock_aperture, tonic_context_frame_landscape,
};
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
