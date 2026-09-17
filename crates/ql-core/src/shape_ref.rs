use crate::shape::{
    EIGHTEEN_FOLD_SHAPE_REF, FOUR_BY_FOUR_BY_FOUR_SHAPE_REF, QL_SHAPE_CONTRACT_VERSION, QlShape,
    RELATIONAL_SIXFOLD_SHAPE_REF, SEVEN_FOLD_SHAPE_REF, SIX_BY_SIX_SHAPE_REF, TEN_BY_TEN_SHAPE_REF,
};
use crate::structural::{ConstellationGrain, RelationFamily};

/// Resolve one canonical, versioned QL shape reference back to the shape owned
/// by the kernel. This is the inverse of `QlShape::shape_ref` for every named
/// shape; intentionally anonymous `Other` grains have no reversible public ref.
pub fn resolve_shape_ref(reference: &str) -> Option<QlShape> {
    let exact = match reference {
        FOUR_BY_FOUR_BY_FOUR_SHAPE_REF => Some(QlShape::FourByFourByFour),
        SIX_BY_SIX_SHAPE_REF => Some(QlShape::SixBySix),
        RELATIONAL_SIXFOLD_SHAPE_REF => Some(QlShape::RelationalSixfold),
        EIGHTEEN_FOLD_SHAPE_REF => Some(QlShape::EighteenFold),
        TEN_BY_TEN_SHAPE_REF => Some(QlShape::TenByTen),
        SEVEN_FOLD_SHAPE_REF => Some(QlShape::canonical_sevenfold()),
        _ => None,
    };
    if exact.is_some() {
        return exact;
    }

    let constellation_prefix = format!("ql:shape:{QL_SHAPE_CONTRACT_VERSION}:constellation:");
    if let Some(name) = reference.strip_prefix(&constellation_prefix) {
        let grain = match name {
            "anchor-only" => ConstellationGrain::AnchorOnly,
            "twofold" => ConstellationGrain::TwoFold,
            "threefold-123" => ConstellationGrain::ThreeFold123,
            "threefold-450" => ConstellationGrain::ThreeFold450,
            "fourfold-1234" => ConstellationGrain::FourFold1234,
            "four-plus-one-ground" => ConstellationGrain::FourPlusOneGround,
            "four-plus-one-synthesis" => ConstellationGrain::FourPlusOneSynthesis,
            "sixfold" => ConstellationGrain::SixFold,
            "partial-conjugate-7" => ConstellationGrain::PartialConjugate7,
            "partial-conjugate-8" => ConstellationGrain::PartialConjugate8,
            "partial-conjugate-9" => ConstellationGrain::PartialConjugate9,
            "partial-conjugate-10" => ConstellationGrain::PartialConjugate10,
            "partial-conjugate-11" => ConstellationGrain::PartialConjugate11,
            "twelvefold" => ConstellationGrain::TwelveFold,
            _ => return None,
        };
        let shape = QlShape::Constellation(grain);
        return (shape.shape_ref() == reference).then_some(shape);
    }

    let square_prefix = format!("ql:shape:{QL_SHAPE_CONTRACT_VERSION}:4x4:");
    let rest = reference.strip_prefix(&square_prefix)?;
    let (family, pair_index) = rest.split_once(':')?;
    if pair_index.contains(':') {
        return None;
    }
    let family = match family {
        "A" => RelationFamily::A,
        "B" => RelationFamily::B,
        "C" => RelationFamily::C,
        _ => return None,
    };
    let pair_index = pair_index.parse::<u8>().ok()?;
    family.pair(pair_index).ok()?;
    let shape = QlShape::FourByFour { family, pair_index };
    (shape.shape_ref() == reference).then_some(shape)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_named_geometry_shape_round_trips_its_canonical_ref() {
        let shapes = [
            QlShape::onefold(),
            QlShape::Constellation(ConstellationGrain::TwoFold),
            QlShape::Constellation(ConstellationGrain::ThreeFold123),
            QlShape::Constellation(ConstellationGrain::ThreeFold450),
            QlShape::Constellation(ConstellationGrain::FourFold1234),
            QlShape::Constellation(ConstellationGrain::FourPlusOneGround),
            QlShape::Constellation(ConstellationGrain::FourPlusOneSynthesis),
            QlShape::Constellation(ConstellationGrain::SixFold),
            QlShape::canonical_sevenfold(),
            QlShape::Constellation(ConstellationGrain::PartialConjugate8),
            QlShape::Constellation(ConstellationGrain::PartialConjugate9),
            QlShape::Constellation(ConstellationGrain::PartialConjugate10),
            QlShape::Constellation(ConstellationGrain::PartialConjugate11),
            QlShape::Constellation(ConstellationGrain::TwelveFold),
            QlShape::FourByFour {
                family: RelationFamily::A,
                pair_index: 2,
            },
            QlShape::FourByFourByFour,
            QlShape::SixBySix,
            QlShape::RelationalSixfold,
            QlShape::EighteenFold,
            QlShape::TenByTen,
        ];
        for shape in shapes {
            let reference = shape.shape_ref();
            assert_eq!(resolve_shape_ref(&reference), Some(shape), "{reference}");
        }
    }

    #[test]
    fn resolver_rejects_aliases_unknown_versions_and_anonymous_grains() {
        assert_eq!(
            resolve_shape_ref("ql:shape:1.0.0:constellation:partial-conjugate-7"),
            None
        );
        assert_eq!(
            resolve_shape_ref("ql:shape:9.9.9:10x10:second-spanda-4-plus-6"),
            None
        );
        assert_eq!(
            resolve_shape_ref("ql:shape:1.0.0:constellation:other"),
            None
        );
        assert_eq!(resolve_shape_ref("ql:shape:1.0.0:4x4:A:9"), None);
    }
}
