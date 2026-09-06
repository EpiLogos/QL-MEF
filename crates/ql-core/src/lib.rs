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
mod pole;
mod position;
mod relation_classification;
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
pub use pole::{
    AngleDeg10, AngularGrid, ApertureClock, ApertureIndex, CanonicalAddress, Codon64, CodonClass,
    CoinFace, CoinSum, CoinTriple, Element, ElementalQuaternionBasis, FibonacciGround,
    FoldGeometry, FoldMotif, FoldState, FourCharge, INVERSE_SEAM_CONTRACT_REF, MatrixAxis,
    MatrixFamily, Mobility, Nucleotide, PHYSICAL_POLE_FORM_CONTRACT_VERSION,
    POLE_COIN_CONTRACT_REF, POLE_ELEMENTAL_CARRIER_REF, POLE_FOLD_STATE_REF, PairIndex16, Polarity,
    QuaternionComponents, ROTATIONAL_STATE_TOTAL, RetrievalEvidence, RotationalPose,
    SelectionContext, SelectionLaw, SiteProperty, SiteReading, SiteState, Transduction18to16,
    all_poses, carrier, det_shadow, monoid,
};
pub use position::QlPosition;
pub use relation_classification::{RelationPairMatch, classify_relation_pair};
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
