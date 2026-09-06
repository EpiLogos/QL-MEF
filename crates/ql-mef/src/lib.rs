//! Complete executable MEF manifold and identity-preserving refraction contracts.
//!
//! Q2 keeps deterministic registry topology separate from semantic/stochastic readings.

mod context_frame;
mod context_frame_target;
mod coordinate;
mod error;
mod identity;
mod lens;
mod m_map;
mod matheme;
mod music;
mod music_completion;
mod provenance;
mod reading;
mod refraction;
mod registry;
mod sublens;
mod vak;
mod vak_oi;

pub use context_frame::{
    CONTEXT_FRAME_GRAMMAR_VERSION, ContextFrameCoordinate, ContextFrameCut, ContextFrameId,
    ContextFrameSelection, MefFormCoordinate, MefGrain, canonical_context_frame_progression,
};
pub use context_frame_target::{
    CONTEXT_FRAME_TARGET_READING_VERSION, ContextFrameReadingOrigin, ContextFrameReadingStatus,
    ContextFrameStructuralProbe, ExternalContextFrameError, ExternalContextFrameReading,
    ExternalSixfoldMapping, read_external_context_frame,
};
pub use coordinate::{MEF_ROTATION_VERSION, MefRotation, MefUnitFace};
pub use error::MefError;
pub use identity::{ClientRef, QlTarget};
pub use lens::{LensFace, LensId, LensRef, MEF_REGISTRY_REVISION, MEF_REGISTRY_VERSION, MefSquare};
pub use m_map::{
    ImplementationBinding, MCoordinate, MFace, MMapIndex, MPathSeparator, MRelation,
    MRelationClass, MRelationEndpoint, ReflectionProof, RelationOrientation, SourcePayload,
    SourceRecordRef,
};
pub use matheme::{
    CIRCUIT_COORDINATES, CIRCUIT_DEGREES, COPULA_SYMBOL, DOUBLE_BEAT_TURNS,
    MATHEME_DERIVATION_CONTRACT_VERSION, MATHEME_DERIVATION_LAYER, MathemeDerivation,
    MathemeTopLine, RECOGNITION_DEGREES, RETAINED_ONE, TOP_LINE, beat, binary_register,
    cardinality_sum, decomposed_totality, derive_matheme, door_ascent, door_descent, double_beat,
    epogdoon, field_cardinality, octave_through_door, one_circuit, position_hexad, ring_octave,
    self_register, standing_whole, totality_ratio, twelve_ring,
};
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
pub use music_completion::{
    MUSICAL_COMPLETION_VERSION, MusicalCompletionFrame, MusicalTraversalCandidate,
    TraversalExpansionSide, classify_musical_traversal, musical_completion_frame,
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
pub use vak::{
    SelfOtherForm, VAK_ENTRY_COUNT, VAK_SOURCE_GIT_BLOB, VAK_SOURCE_PATH, VAK_SOURCE_REPOSITORY,
    VAK_SOURCE_REVISION, VakActionProfile, VakActionRelationKind, VakAddressHorizon,
    VakContextField, VakDivineAct, VakEntry, VakError, VakFormalCoverage, VakHorizonBinding,
    VakNeighbourhood, VakOperatorBinding, VakPath, VakPraxisAspect, VakPraxisReading, VakRPath,
    VakRPathStep, VakRef, VakRefraction, VakRegistry, VakRelation, VakRelationKind, VakRelationOp,
    VakSivaSaktiCell, VakSivaSaktiField, VakSivaSaktiGenerationSite, VakSivaSaktiRelationalSixfold,
    VakSourceProvenance, VakSpeechAct, VakSpeechStance, VakStanding,
};
pub use vak_oi::{
    AIKIT_OPERATIVE_OWNER_REVISION, AIKIT_OPERATIVE_SYNTAX_VERSION, CENTRAL_ACTION_OWNER_REVISION,
    CENTRAL_WORK_LIST_ACTION_REF, FACTORY_ACTION_OWNER_REVISION,
    FACTORY_REQUEST_EVIDENCE_ACTION_REF, VAK_ACTION_PROFILE_CONTRACT,
    VAK_EXPRESSION_READING_CONTRACT, VAK_OI_PRIMITIVE_MATRIX_CONTRACT, VAK_PATH_CONTRACT,
    VAK_RECOGNITION_CONTRACT, VakActionAffordance, VakActionProfileV1, VakExecutionObservationV1,
    VakExpressionReadingV1, VakExpressionSubject, VakGeneralExpressionEvidence, VakOiError,
    VakOiPrimitiveKind, VakOiPrimitiveMatrixV1, VakOiPrimitiveRelation, VakOiRelationKind,
    VakOiSemanticAltitude, VakPathStepV1, VakPathV1, VakPraxisInstantiationV1,
    VakRecognitionProposal, VakRecognitionV1, central_work_list_profile,
    factory_request_evidence_profile, oi_reference_primitive_matrix, recognise_vak_return,
    reconstruct_observed_vak_path,
};

impl MCoordinate {
    /// Project the coordinate face into the existing QL direct/conjugate floor
    /// without collapsing the source-owned recursive M path into one QL address.
    pub const fn ql_face(&self) -> ql_core::QlFace {
        self.face.ql_face()
    }
}
