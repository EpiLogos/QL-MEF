//! Complete executable MEF manifold and identity-preserving refraction contracts.
//!
//! Q2 keeps deterministic registry topology separate from semantic/stochastic readings.

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
    VAK_SOURCE_REVISION, VakActionProfile, VakActionRelationKind, VakAddressHorizon, VakEntry,
    VakError, VakHorizonBinding, VakNeighbourhood, VakOperatorBinding, VakPath, VakPraxisAspect,
    VakPraxisReading, VakRef, VakRefraction, VakRegistry, VakRelation, VakRelationKind,
    VakRelationOp, VakSourceProvenance, VakStanding,
};

impl MCoordinate {
    /// Project the coordinate face into the existing QL direct/conjugate floor
    /// without collapsing the source-owned recursive M path into one QL address.
    pub const fn ql_face(&self) -> ql_core::QlFace {
        self.face.ql_face()
    }
}
