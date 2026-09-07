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
    AA_STOP_INDEX, AMINO_ACID_VOCABULARY, DNA_RNA_UNIQUE_FORMS, ENTRIES_PER_SUIT,
    M2_TO_M3_CYMATIC_PROJECTION, M2_VIBRATION_CYCLE, M3_CODON_TO_AA, M3_MATRIX_PAIR,
    M3_PAIR_DIFFERENCE_SIGN, M3_RNA_DARK_MASK, M3_RNA_FUNCTIONAL_MASK, MAJOR_ARCANA_COUNT,
    MINOR_ARCANA_COUNT, MajorArcanaCard, MinorArcanaCard, PARASHAKTI_SHADOW_OFFSET,
    POLARIZED_ENTRIES, POLE_ROTATIONAL_PROFILE_REF, POLE_TAROT_BRIDGE_REF, POLE_TRANSCRIPTION_REF,
    RECORDED_PAIR_DIFF_SIGNS, RNA_T_CONTAINING_CODONS, RNA_T_FREE_CODONS, ROTATIONAL_TABLE_ENTRIES,
    RotationalCandidate, RotationalPolarity, RotationalProfile, RotationalStateType,
    TAROT_QUATERNION_COUNT, TRANSCENDENT_TAROT_COUNT, TarotBridge, TarotPip, TarotSuit,
    TranscendentOperator, amino_acid_name, apply_epogdoon_compression, codon_parashakti_frequency,
    compose_rotational_state, generate_rotational_states, is_evolutionary_gap, is_stop_codon,
    m3_codon_amino_index, m3_codon_is_rna_capable, matrix_pair_nucleotides, matrix_partner,
    pair_difference, pair_sum, parashakti_frequency, rotational_profile,
    rotational_total_sum_value, transduce_vibration_to_symbol, wc_anticodon,
};
pub use pole::{
    AngleDeg10, AngularGrid, ApertureClock, ApertureIndex, ApplyOutcome, CanonicalAddress, Codon64,
    CodonClass, CoinFace, CoinSum, CoinTriple, Element, ElementalQuaternionBasis, FibonacciGround,
    FoldGeometry, FoldMotif, FoldState, FourCharge, INVERSE_SEAM_CONTRACT_REF, M3_RES_MATRIX,
    MatrixAxis, MatrixFamily, Mobility, Nucleotide, PHYSICAL_POLE_FORM_CONTRACT_VERSION,
    POLE_COIN_CONTRACT_REF, POLE_ELEMENTAL_CARRIER_REF, POLE_FOLD_STATE_REF, PairIndex16, Polarity,
    QuaternionComponents, RES_ADMITTED_COUNT, RES_GAP_ADDRESSES, RESONANCE_GAP,
    ROTATIONAL_STATE_TOTAL, RetrievalEvidence, RotationalPose, SelectionContext, SelectionLaw,
    SiteProperty, SiteReading, SiteState, Transduction18to16, all_poses, carrier, det_shadow,
    is_resonance_gap, monoid, resonance_entry,
};
pub use pole::{
    DetOverlay, ICHING_GRAMMAR_REF, M2_ELEMENT_RING_POSITIONS, ORIENTATION_CHAIN_REF,
    POLE_ICHING_GRAMMAR_CONTRACT_REF, Quat, RING_QUATERNION_LUT, Trigram, complement,
    compose_hexagram, det_overlay, element_quaternion, flow_clockwise, integral_symmetry_field,
    lower_trigram_id, matrix_axis_quaternion, nuclear_hexagram, nuclear_lower, nuclear_upper,
    palindromic_anchors, polar_opposite_simple, polar_opposite_su2, quadrant, quat_active_state,
    quat_codon_state, quat_from_codon, quat_from_ring_pos, upper_trigram_id,
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
