use ql_core::{
    ConjugationDegree, ExpansionSide, PairingError, QlCoordinate, RelationFamily,
    build_d_modulation_frame,
};

use crate::music::pitch_at_lens;
use crate::{LensId, MusicalBasis, PitchClass};

/// Version of the musical projection of the canonical D1→D3 conjugate-completion grammar.
pub const MUSICAL_COMPLETION_VERSION: &str = "1.0.0";

/// Musical rendering of one selected A/B/C relation as it is completed across
/// the direct/prime field.
///
/// The structural law is owned by ql-core:
/// - D1 = selected direct pair / 2 coordinates;
/// - D2 = one-sided conjugate expansion / 3 coordinates;
/// - D3 = complete conjugate square / 4 coordinates.
///
/// Semantic cross operators (same-position, transform, require, complete) are
/// separate kernel relations. Historical D-coordinate provenance for those
/// cross operators must not be read as this completion degree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MusicalCompletionFrame {
    pub family: RelationFamily,
    pub pair_index: u8,
    pub degree: ConjugationDegree,
    pub expansion_side: Option<ExpansionSide>,
    pub coordinates: Vec<QlCoordinate>,
    pub pitches: Vec<PitchClass>,
    pub structural_operator_ref: String,
}

pub fn musical_completion_frame(
    basis: MusicalBasis,
    lens: LensId,
    family: RelationFamily,
    pair_index: u8,
    degree: ConjugationDegree,
    expansion_side: Option<ExpansionSide>,
) -> Result<MusicalCompletionFrame, PairingError> {
    let field = build_d_modulation_frame(family, pair_index, degree, expansion_side)?;
    let structural_operator_ref = field.operator_ref();
    let coordinates = field.coordinates;
    let pitches = coordinates
        .iter()
        .copied()
        .map(|coordinate| pitch_at_lens(basis, lens, coordinate))
        .collect();

    Ok(MusicalCompletionFrame {
        family,
        pair_index,
        degree,
        expansion_side,
        coordinates,
        pitches,
        structural_operator_ref,
    })
}
