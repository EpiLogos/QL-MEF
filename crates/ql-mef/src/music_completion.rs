use ql_core::{
    ConjugationDegree, ExpansionSide, PairingError, QlCoordinate, QlPosition, RelationFamily,
    RelationPairMatch, build_d_modulation_frame, classify_relation_pair,
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

/// Which endpoint of the observed source→target traversal carries the one-sided
/// conjugate expansion when the event is at D2.
///
/// This is traversal evidence, not another QL structural side. It is translated
/// into the canonical pair's `ExpansionSide` after relation classification so a
/// reversed walk does not silently swap the meaning of source and target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraversalExpansionSide {
    Source,
    Target,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MusicalTraversalCandidate {
    pub relation: RelationPairMatch,
    pub frame: MusicalCompletionFrame,
}

fn canonical_expansion_side(
    relation: RelationPairMatch,
    traversal_side: TraversalExpansionSide,
) -> ExpansionSide {
    match (traversal_side, relation.reversed) {
        (TraversalExpansionSide::Source, false) | (TraversalExpansionSide::Target, true) => {
            ExpansionSide::Left
        }
        (TraversalExpansionSide::Target, false) | (TraversalExpansionSide::Source, true) => {
            ExpansionSide::Right
        }
    }
}

/// Classify an actual positional traversal against the accepted A/B/C grammar
/// and render every valid completion candidate through the canonical music API.
///
/// The result is intentionally zero-to-many. QL does not invent a family for a
/// noncanonical pair and does not discard real overlap such as `(2,3)` or the
/// direction-sensitive `(0,5)/(5,0)` case.
pub fn classify_musical_traversal(
    basis: MusicalBasis,
    lens: LensId,
    source: QlPosition,
    target: QlPosition,
    degree: ConjugationDegree,
    d2_expansion: Option<TraversalExpansionSide>,
) -> Result<Vec<MusicalTraversalCandidate>, PairingError> {
    let matches = classify_relation_pair(source, target);
    let mut candidates = Vec::with_capacity(matches.len());

    for relation in matches {
        let expansion_side = match degree {
            ConjugationDegree::D2 => {
                d2_expansion.map(|side| canonical_expansion_side(relation, side))
            }
            ConjugationDegree::D1 | ConjugationDegree::D3 => None,
        };
        let frame = musical_completion_frame(
            basis,
            lens,
            relation.family,
            relation.pair_index,
            degree,
            expansion_side,
        )?;
        candidates.push(MusicalTraversalCandidate { relation, frame });
    }

    Ok(candidates)
}
