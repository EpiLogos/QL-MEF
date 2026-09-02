use crate::{
    ConstellationGrain, PairInstance, QlCoordinate, QlFace, QlPosition, RelationFamily,
    RelationField, WHOLE_ANCHOR_SYMBOL,
};

pub const QL_SHAPE_CONTRACT_VERSION: &str = "1.0.0";
pub const SIX_BY_SIX_SHAPE_REF: &str = "ql:shape:1.0.0:6x6:direct-conjugate";
pub const RELATIONAL_SIXFOLD_SHAPE_REF: &str = "ql:shape:1.0.0:6-plus-6-prime";
pub const RELATIONAL_SIXFOLD_OPERATOR_REF: &str =
    "ql:shape:1.0.0:generation:same-position-direct-conjugate";

/// Canonical executable morphology of a QL whole.
///
/// `Constellation` carries the already-developed positive grains from the whole
/// anchor through partial and complete direct/conjugate constellations. The
/// matrix and relational-sixfold variants are higher-order fields disclosed from
/// those same coordinates rather than a second structural system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QlShape {
    Constellation(ConstellationGrain),
    FourByFour {
        family: RelationFamily,
        pair_index: u8,
    },
    SixBySix,
    RelationalSixfold,
}

impl QlShape {
    pub fn shape_ref(self) -> String {
        match self {
            Self::Constellation(grain) => format!(
                "ql:shape:{QL_SHAPE_CONTRACT_VERSION}:constellation:{}",
                grain.as_str()
            ),
            Self::FourByFour { family, pair_index } => format!(
                "ql:shape:{QL_SHAPE_CONTRACT_VERSION}:4x4:{}:{pair_index}",
                family.as_str()
            ),
            Self::SixBySix => SIX_BY_SIX_SHAPE_REF.into(),
            Self::RelationalSixfold => RELATIONAL_SIXFOLD_SHAPE_REF.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QlShapeKind {
    Constellation,
    FourByFour,
    SixBySix,
    RelationalSixfold,
}

impl QlShapeKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Constellation => "constellation",
            Self::FourByFour => "4x4",
            Self::SixBySix => "6x6",
            Self::RelationalSixfold => "6-plus-6-prime",
        }
    }
}

/// One address in a QL relational accounting field.
///
/// An address identifies where a relation can be inspected, compared or
/// generated. It does not assert that a semantic relation exists between the
/// two coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QlShapeAddress {
    pub row: QlCoordinate,
    pub column: QlCoordinate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FourByFourField {
    pub source: RelationField,
    pub addresses: Vec<QlShapeAddress>,
}

impl FourByFourField {
    /// Promote one canonical D3 pair-completion into its complete 4×4
    /// relational accounting field while retaining A/B/C + pair provenance.
    pub fn from_pair(pair: PairInstance) -> Self {
        let source = pair.d3();
        let axis = source.coordinates.clone();
        let addresses = cartesian_addresses(&axis, &axis);
        Self { source, addresses }
    }

    pub const fn kind(&self) -> QlShapeKind {
        QlShapeKind::FourByFour
    }

    pub const fn shape(&self) -> QlShape {
        QlShape::FourByFour {
            family: self.source.pair.family,
            pair_index: self.source.pair.pair_index,
        }
    }

    pub fn axis(&self) -> &[QlCoordinate] {
        &self.source.coordinates
    }

    pub fn shape_ref(&self) -> String {
        self.shape().shape_ref()
    }

    pub fn derivation_ref(&self) -> String {
        self.source.operator_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SixBySixField {
    pub direct_axis: Vec<QlCoordinate>,
    pub conjugate_axis: Vec<QlCoordinate>,
    pub addresses: Vec<QlShapeAddress>,
}

impl SixBySixField {
    /// Canonical sixfold × conjugate-sixfold relational accounting field.
    pub fn canonical() -> Self {
        let direct_axis = canonical_axis(QlFace::Direct);
        let conjugate_axis = canonical_axis(QlFace::Conjugate);
        let addresses = cartesian_addresses(&direct_axis, &conjugate_axis);
        Self {
            direct_axis,
            conjugate_axis,
            addresses,
        }
    }

    pub const fn kind(&self) -> QlShapeKind {
        QlShapeKind::SixBySix
    }

    pub const fn shape(&self) -> QlShape {
        QlShape::SixBySix
    }

    pub const fn shape_ref(&self) -> &'static str {
        SIX_BY_SIX_SHAPE_REF
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QlGenerationSite {
    pub position: QlPosition,
    pub direct: QlCoordinate,
    pub conjugate: QlCoordinate,
}

impl QlGenerationSite {
    pub fn operator_ref(self) -> String {
        format!(
            "{RELATIONAL_SIXFOLD_OPERATOR_REF}:position-{}",
            self.position.value()
        )
    }
}

/// The deterministic structural basis of `6 / 6′ -> 6+6′`.
///
/// The kernel identifies the six same-position direct/conjugate relation sites
/// and the whole operation that binds them. It deliberately does not invent
/// semantic generated content for those sites; clients may do so through an
/// attributable contemplation/reading operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationalSixfold {
    pub sites: Vec<QlGenerationSite>,
    pub direct_basis_ref: String,
    pub conjugate_basis_ref: String,
    pub return_anchor_symbol: &'static str,
}

impl RelationalSixfold {
    pub fn canonical() -> Self {
        let sites = (0_u8..6)
            .map(|value| {
                let position = QlPosition::new(value)
                    .expect("relational-sixfold positions are canonical 0..5");
                QlGenerationSite {
                    position,
                    direct: QlCoordinate::new(position, QlFace::Direct),
                    conjugate: QlCoordinate::new(position, QlFace::Conjugate),
                }
            })
            .collect();
        Self {
            sites,
            direct_basis_ref: format!("ql:shape:{QL_SHAPE_CONTRACT_VERSION}:sixfold:direct"),
            conjugate_basis_ref: format!("ql:shape:{QL_SHAPE_CONTRACT_VERSION}:sixfold:conjugate"),
            return_anchor_symbol: WHOLE_ANCHOR_SYMBOL,
        }
    }

    pub const fn kind(&self) -> QlShapeKind {
        QlShapeKind::RelationalSixfold
    }

    pub const fn shape(&self) -> QlShape {
        QlShape::RelationalSixfold
    }

    pub const fn shape_ref(&self) -> &'static str {
        RELATIONAL_SIXFOLD_SHAPE_REF
    }

    pub const fn operator_ref(&self) -> &'static str {
        RELATIONAL_SIXFOLD_OPERATOR_REF
    }
}

fn canonical_axis(face: QlFace) -> Vec<QlCoordinate> {
    (0_u8..6)
        .map(|value| {
            let position = QlPosition::new(value).expect("shape-axis positions are canonical 0..5");
            QlCoordinate::new(position, face)
        })
        .collect()
}

fn cartesian_addresses(rows: &[QlCoordinate], columns: &[QlCoordinate]) -> Vec<QlShapeAddress> {
    let mut addresses = Vec::with_capacity(rows.len() * columns.len());
    for row in rows {
        for column in columns {
            addresses.push(QlShapeAddress {
                row: *row,
                column: *column,
            });
        }
    }
    addresses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn existing_constellation_grains_are_positive_canonical_shapes() {
        let partial = QlShape::Constellation(ConstellationGrain::PartialConjugate9);
        let direct = QlShape::Constellation(ConstellationGrain::SixFold);

        assert_eq!(
            partial.shape_ref(),
            "ql:shape:1.0.0:constellation:partial-conjugate-9"
        );
        assert_eq!(
            direct.shape_ref(),
            "ql:shape:1.0.0:constellation:sixfold"
        );
    }

    #[test]
    fn d3_square_expands_to_sixteen_addresses_and_retains_route_identity() {
        let a = FourByFourField::from_pair(RelationFamily::A.pair(1).unwrap());
        let c = FourByFourField::from_pair(RelationFamily::C.pair(2).unwrap());

        assert_eq!(a.axis().len(), 4);
        assert_eq!(a.addresses.len(), 16);
        assert_eq!(a.source.vertex_key(), c.source.vertex_key());
        assert_ne!(a.shape_ref(), c.shape_ref());
        assert_ne!(a.derivation_ref(), c.derivation_ref());
    }

    #[test]
    fn canonical_six_by_six_is_direct_against_conjugate() {
        let field = SixBySixField::canonical();

        assert_eq!(field.direct_axis.len(), 6);
        assert_eq!(field.conjugate_axis.len(), 6);
        assert_eq!(field.addresses.len(), 36);
        assert!(
            field
                .direct_axis
                .iter()
                .all(|coordinate| coordinate.face == QlFace::Direct)
        );
        assert!(
            field
                .conjugate_axis
                .iter()
                .all(|coordinate| coordinate.face == QlFace::Conjugate)
        );
    }

    #[test]
    fn relational_sixfold_exposes_six_same_position_generation_sites() {
        let shape = RelationalSixfold::canonical();

        assert_eq!(shape.sites.len(), 6);
        for (index, site) in shape.sites.iter().enumerate() {
            assert_eq!(site.position.value(), index as u8);
            assert_eq!(site.direct.position, site.conjugate.position);
            assert_eq!(site.direct.face, QlFace::Direct);
            assert_eq!(site.conjugate.face, QlFace::Conjugate);
        }
        assert_eq!(shape.return_anchor_symbol, "0/1");
        assert_eq!(shape.shape_ref(), RELATIONAL_SIXFOLD_SHAPE_REF);
    }

    #[test]
    fn portable_shape_fixture_carries_the_same_cardinality_and_return_laws() {
        let fixture = include_str!("../../../fixtures/kernel/ql-shape-contract-v1.json");

        assert!(fixture.contains("\"address_cardinality\": 16"));
        assert!(fixture.contains("\"address_cardinality\": 36"));
        assert!(fixture.contains("\"site_cardinality\": 6"));
        assert!(fixture.contains("\"return_through\": \"0/1\""));
        assert!(fixture.contains("\"shape_address_asserts_semantic_relation\": false"));
    }
}
