use crate::{KernelRelationId, QlCoordinate, QlFace, QlPosition, RelationFamily};

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
pub fn classify_relation_pair(left: QlPosition, right: QlPosition) -> Vec<RelationPairMatch> {
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

/// Every kernel relation one participation bears to another, read from their
/// coordinates alone.
///
/// The positional laws are the holographic contract's: same position across
/// faces is `n <-> n'`; across faces at different positions the direct
/// endpoint `n` is read against `(n+1)'`, `(n-1)'` and `(5-n)'`; on one face
/// `n -> 5-n` is the mirror complement and `n -> n+1` the successor. A/B/C
/// membership is reported for any two distinct positions, with direction, so a
/// pair with one conjugate endpoint still shows the field it expands. Overlaps
/// are kept: `2 -> 3'` is both transform and complete.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoordinatePairReading {
    pub relations: Vec<KernelRelationId>,
    pub families: Vec<RelationPairMatch>,
}

pub fn classify_coordinate_pair(left: QlCoordinate, right: QlCoordinate) -> CoordinatePairReading {
    let (l, r) = (left.position.value(), right.position.value());
    let mut relations = Vec::new();
    if l == r {
        relations.push(if left.face == right.face {
            KernelRelationId::PositionIdentity
        } else {
            KernelRelationId::CrossSamePosition
        });
        return CoordinatePairReading {
            relations,
            families: Vec::new(),
        };
    }
    let families = classify_relation_pair(left.position, right.position);
    if left.face == right.face {
        for m in &families {
            let id = match m.family {
                RelationFamily::A => KernelRelationId::PairA,
                RelationFamily::B => KernelRelationId::PairB,
                RelationFamily::C => KernelRelationId::PairC,
            };
            if !relations.contains(&id) {
                relations.push(id);
            }
        }
        if r == 5 - l {
            relations.push(KernelRelationId::MirrorComplement);
        }
        if r == (l + 1) % 6 {
            relations.push(KernelRelationId::PositionSuccessor);
        }
    } else {
        let (n, c) = if left.face == QlFace::Direct {
            (l, r)
        } else {
            (r, l)
        };
        if c == (n + 1) % 6 {
            relations.push(KernelRelationId::CrossTransform);
        }
        if c == (n + 5) % 6 {
            relations.push(KernelRelationId::CrossRequire);
        }
        if c == 5 - n {
            relations.push(KernelRelationId::CrossComplete);
        }
    }
    CoordinatePairReading {
        relations,
        families,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(position: u8, face: QlFace) -> QlCoordinate {
        QlCoordinate::new(QlPosition::new(position).unwrap(), face)
    }

    #[test]
    fn coordinate_pairs_carry_the_contract_laws_and_keep_overlaps() {
        use KernelRelationId::*;
        use QlFace::{Conjugate as C, Direct as D};
        let read = |a, b| classify_coordinate_pair(a, b).relations;
        assert_eq!(read(at(3, D), at(3, C)), [CrossSamePosition]);
        assert_eq!(read(at(3, D), at(3, D)), [PositionIdentity]);
        assert_eq!(read(at(1, D), at(2, C)), [CrossTransform]);
        assert_eq!(read(at(2, C), at(1, D)), [CrossTransform]);
        assert_eq!(read(at(1, D), at(0, C)), [CrossRequire]);
        assert_eq!(read(at(1, D), at(4, C)), [CrossComplete]);
        assert_eq!(read(at(2, D), at(3, C)), [CrossTransform, CrossComplete]);
        // (2,3) is A[1] and C[2]: both fields, the mirror and the successor.
        assert_eq!(
            read(at(2, D), at(3, D)),
            [PairA, PairC, MirrorComplement, PositionSuccessor]
        );
        // (5,0) is B[2] and, reversed, C[0]: 5 -> 0 is also the mirror and the successor.
        assert_eq!(
            read(at(5, C), at(0, C)),
            [PairB, PairC, MirrorComplement, PositionSuccessor]
        );
        assert_eq!(read(at(0, D), at(5, D)), [PairB, PairC, MirrorComplement]);
        assert!(read(at(0, D), at(2, D)).is_empty());
        let fields = classify_coordinate_pair(at(4, D), at(1, C)).families;
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].family, RelationFamily::C);
        assert!(fields[0].reversed);
    }
}
