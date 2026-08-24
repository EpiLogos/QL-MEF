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
    ALL_PITCH_CLASSES, AUTHORED_INTERVAL_REFERENCES, AuthoredIntervalReference, CANONICAL_RATIOS,
    CrossOperator, DiatonicCut, FIRST_SPANDA_HORIZONTAL, HarmonicRatio, IONIAN_OFFSETS,
    KERNEL_FAMILY_RELATION, KernelFamilyAddress, LensAnchor, MAJOR_MINOR_CHARACTER_DEGREES,
    MUSICAL_DERIVATION_SOURCE_BLOB, MUSICAL_DERIVATION_SOURCE_PATH,
    MUSICAL_DERIVATION_VENDOR_COMMIT, MUSICAL_HARMONIC_VERSION, MajorMinorCharacterDegree,
    ModeKind, ModeTonicInstance, MusicalBasis, MusicalSquare, NAME_CONTENT, POWER_CONTENT,
    PitchClass, PreMMusicalDerivation, SECOND_SPANDA_VERTICAL, c_p_l_family_views, cf_diatonic_cut,
    cross_interval_deltas, d3_interval_deltas, d3_relation_id, derive_pre_m_music,
    directed_pitch_delta, explicate_coordinates, implicate_coordinates, lens_anchor, lens_anchors,
    lens_kernel_coordinate, mode_tonic_instance, mode_tonic_landscape, musical_square,
    musical_squares, pair_interval_deltas, pitch_at_lens, pitch_name, spanda_cross_reading_ratios,
    transpose,
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
