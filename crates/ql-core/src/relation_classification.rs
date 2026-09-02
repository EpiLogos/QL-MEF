use crate::{QlPosition, RelationFamily};

/// One canonical A/B/C match for a positional pair.
///
/// A positional pair can legitimately participate in more than one relation
/// family. The classifier therefore returns every match rather than imposing a
/// total or precedence-based A/B/C assignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RelationPairMatch {
    pub family: RelationFamily,
    pub pair_index: u8,
    /// True when the queried pair traverses the canonical stored pair in reverse.
    /// Structural family membership is retained while interval/rendering callers
    /// can still preserve traversal direction.
    pub reversed: bool,
}

impl RelationPairMatch {
    pub const fn operator_ref(self) -> &'static str {
        match self.family {
            RelationFamily::A => "A",
            RelationFamily::B => "B",
            RelationFamily::C => "C",
        }
    }
}

/// Classify a positional traversal against the accepted A/B/C relation grammar.
///
/// This function is intentionally partial and ambiguity-preserving:
/// - pairs outside A/B/C return an empty vector;
/// - pairs participating in multiple families return multiple matches;
/// - reverse traversal preserves the family/pair identity and marks direction.
///
/// It does not infer a relation family from arithmetic residue, pitch, matrix
/// family, or any other downstream projection.
pub fn classify_relation_pair(
    left: QlPosition,
    right: QlPosition,
) -> Vec<RelationPairMatch> {
    let left = left.value();
    let right = right.value();
    let mut matches = Vec::with_capacity(2);

    for family in [RelationFamily::A, RelationFamily::B, RelationFamily::C] {
        for (pair_index, (canonical_left, canonical_right)) in
            family.pairs().into_iter().enumerate()
        {
            if left == canonical_left && right == canonical_right {
                matches.push(RelationPairMatch {
                    family,
                    pair_index: pair_index as u8,
                    reversed: false,
                });
            } else if left == canonical_right && right == canonical_left {
                matches.push(RelationPairMatch {
                    family,
                    pair_index: pair_index as u8,
                    reversed: true,
                });
            }
        }
    }

    matches
}
