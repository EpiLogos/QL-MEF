//! Deterministic executable QL kernel.
//!
//! Q1 intentionally contains no semantic inference and no Loop Runtime dependency.

mod address;
mod address_parse;
mod apply;
mod deterministic;
mod error;
mod face;
mod form;
mod kernel;
mod operator;
mod pairing;
mod position;
mod shape;
mod structural;

pub use address::QlAddress;
pub use apply::apply_operator;
pub use deterministic::{DeterministicProvenance, DeterministicResult};
pub use error::QlError;
pub use face::QlFace;
pub use form::{QlForm, QlFormRef};
pub use kernel::{
    HOLOGRAPHIC_KERNEL_CONTRACT_VERSION, HOLOGRAPHIC_KERNEL_POINTER_WEB_BLOB,
    HOLOGRAPHIC_KERNEL_REFERENCE_REVISION, KERNEL_VERSION, KernelCapabilities, KernelRelationId,
    QlFamily, SCHEMA_VERSION, VakFamily, VakInstruction, kernel_capabilities,
};
pub use operator::{FourPlusTwoClass, OperatorValue, QlOperator};
pub use pairing::{
    CanonicalCrossPass, D2CrossPassKind, PAIRING_GRAMMAR_VERSION, PairingError,
    build_d_modulation_frame, canonical_cross_pass_d1, canonical_cross_pass_d2,
    canonical_cross_pass_d3,
};
pub use position::QlPosition;
pub use shape::{
    FourByFourField, QL_SHAPE_CONTRACT_VERSION, QlGenerationSite, QlShape, QlShapeAddress,
    QlShapeKind, RELATIONAL_SIXFOLD_OPERATOR_REF, RELATIONAL_SIXFOLD_SHAPE_REF, RelationalSixfold,
    SIX_BY_SIX_SHAPE_REF, SixBySixField,
};
pub use structural::{
    AnchorReturn, ConjugateOpposition, ConjugationDegree, ConstellationGrain, ExpansionSide,
    GroundKind, PairInstance, QlCoordinate, RelationFamily, RelationField,
    STRUCTURAL_CONTRACT_VERSION, StructuralConstellation, StructuralError, StructuralParticipation,
    WHOLE_ANCHOR_SYMBOL, all_d3_fields,
};
