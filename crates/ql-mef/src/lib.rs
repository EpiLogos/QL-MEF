//! Complete executable MEF manifold and identity-preserving refraction contracts.
//!
//! Q2 keeps deterministic registry topology separate from semantic/stochastic readings.

mod context_frame;
mod coordinate;
mod error;
mod identity;
mod lens;
mod m_map;
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
pub use coordinate::{MEF_ROTATION_VERSION, MefRotation, MefUnitFace};
pub use error::MefError;
pub use identity::{ClientRef, QlTarget};
pub use lens::{LensFace, LensId, LensRef, MEF_REGISTRY_REVISION, MEF_REGISTRY_VERSION, MefSquare};
pub use m_map::{
    ImplementationBinding, MCoordinate, MFace, MMapIndex, MPathSeparator, MRelation,
    MRelationClass, MRelationEndpoint, ReflectionProof, RelationOrientation, SourcePayload,
    SourceRecordRef,
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
