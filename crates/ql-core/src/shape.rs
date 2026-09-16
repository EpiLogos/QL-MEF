use crate::{
    Codon64, ConstellationGrain, FoldMotif, PairInstance, QlCoordinate, QlFace, QlPosition,
    RelationFamily, RelationField, WHOLE_ANCHOR_SYMBOL,
};

pub const QL_SHAPE_CONTRACT_VERSION: &str = "1.0.0";
pub const QL_GEOMETRY_SHAPE_VERSION: &str = "1.1.0";
pub const SIX_BY_SIX_SHAPE_REF: &str = "ql:shape:1.0.0:6x6:direct-conjugate";
pub const RELATIONAL_SIXFOLD_SHAPE_REF: &str = "ql:shape:1.0.0:6-plus-6-prime";
pub const RELATIONAL_SIXFOLD_OPERATOR_REF: &str =
    "ql:shape:1.0.0:generation:same-position-direct-conjugate";
pub const SEVEN_FOLD_SHAPE_REF: &str =
    "ql:shape:1.1.0:constellation:partial-conjugate-7";
pub const FOUR_BY_FOUR_BY_FOUR_SHAPE_REF: &str =
    "ql:shape:1.1.0:4x4x4:m3-three-quaternary-sites";
pub const TEN_BY_TEN_SHAPE_REF: &str = "ql:shape:1.1.0:10x10:second-spanda-4-plus-6";
pub const EIGHTEEN_FOLD_SHAPE_REF: &str =
    "ql:shape:1.1.0:18-fold:6-plus-6-prime-plus-relational-6";
pub const COMPRESS_TO_WHOLE_OPERATOR_REF: &str =
    "ql:shape:1.1.0:compression:disclosed-to-0-1";
pub const THREE_TO_ONE_OPERATOR_REF: &str = "ql:shape:1.1.0:compression:3-to-1-recognition";
pub const EIGHTEEN_TO_THREE_OPERATOR_REF: &str =
    "ql:shape:1.1.0:compression:18-by-6-to-threefold";

/// Canonical executable morphology of a QL whole.
///
/// `Constellation` carries the already-developed positive grains from the whole
/// anchor through partial and complete direct/conjugate constellations. The
/// matrix, cubic, relational-sixfold, eighteenfold and decadic variants are
/// higher-order Geometry disclosures of those same QL relations rather than a
/// second structural system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QlShape {
    Constellation(ConstellationGrain),
    FourByFour {
        family: RelationFamily,
        pair_index: u8,
    },
    FourByFourByFour,
    SixBySix,
    RelationalSixfold,
    EighteenFold,
    TenByTen,
}

impl QlShape {
    pub fn shape_ref(self) -> String {
        match self {
            Self::Constellation(ConstellationGrain::Other {
                direct: 6,
                conjugate: 1,
            }) => SEVEN_FOLD_SHAPE_REF.into(),
            Self::Constellation(grain) => format!(
                "ql:shape:{QL_SHAPE_CONTRACT_VERSION}:constellation:{}",
                grain.as_str()
            ),
            Self::FourByFour { family, pair_index } => format!(
                "ql:shape:{QL_SHAPE_CONTRACT_VERSION}:4x4:{}:{pair_index}",
                family.as_str()
            ),
            Self::FourByFourByFour => FOUR_BY_FOUR_BY_FOUR_SHAPE_REF.into(),
            Self::SixBySix => SIX_BY_SIX_SHAPE_REF.into(),
            Self::RelationalSixfold => RELATIONAL_SIXFOLD_SHAPE_REF.into(),
            Self::EighteenFold => EIGHTEEN_FOLD_SHAPE_REF.into(),
            Self::TenByTen => TEN_BY_TEN_SHAPE_REF.into(),
        }
    }

    /// Geometric fold cardinality when this shape is a fold rather than a
    /// matrix/product field. The whole-anchor is the QL Geometry 1-fold even
    /// though it has zero positional members in the structural contract.
    pub const fn fold_count(self) -> Option<u8> {
        match self {
            Self::Constellation(ConstellationGrain::AnchorOnly) => Some(1),
            Self::Constellation(ConstellationGrain::TwoFold) => Some(2),
            Self::Constellation(ConstellationGrain::ThreeFold123)
            | Self::Constellation(ConstellationGrain::ThreeFold450) => Some(3),
            Self::Constellation(ConstellationGrain::FourFold1234) => Some(4),
            Self::Constellation(ConstellationGrain::FourPlusOneGround)
            | Self::Constellation(ConstellationGrain::FourPlusOneSynthesis) => Some(5),
            Self::Constellation(ConstellationGrain::SixFold)
            | Self::RelationalSixfold => Some(6),
            Self::Constellation(ConstellationGrain::Other {
                direct: 6,
                conjugate: 1,
            }) => Some(7),
            Self::Constellation(ConstellationGrain::PartialConjugate8) => Some(8),
            Self::Constellation(ConstellationGrain::PartialConjugate9) => Some(9),
            Self::Constellation(ConstellationGrain::PartialConjugate10) => Some(10),
            Self::Constellation(ConstellationGrain::PartialConjugate11) => Some(11),
            Self::Constellation(ConstellationGrain::TwelveFold) => Some(12),
            Self::EighteenFold => Some(18),
            _ => None,
        }
    }

    pub const fn onefold() -> Self {
        Self::Constellation(ConstellationGrain::AnchorOnly)
    }

    pub const fn canonical_sevenfold() -> Self {
        Self::Constellation(ConstellationGrain::Other {
            direct: 6,
            conjugate: 1,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QlShapeKind {
    Constellation,
    FourByFour,
    FourByFourByFour,
    SixBySix,
    RelationalSixfold,
    EighteenFold,
    TenByTen,
}

impl QlShapeKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Constellation => "constellation",
            Self::FourByFour => "4x4",
            Self::FourByFourByFour => "4x4x4",
            Self::SixBySix => "6x6",
            Self::RelationalSixfold => "6-plus-6-prime",
            Self::EighteenFold => "18-fold",
            Self::TenByTen => "10x10",
        }
    }
}

/// A reversible Geometry reading in which a disclosed 2- or 3-fold whole is
/// presented as the single `0/1` whole-anchor. The disclosed shape remains
/// addressable through the derivation ref; compression never erases it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QlShapeCompression {
    pub presented: QlShape,
    pub disclosed: QlShape,
    pub disclosed_fold: u8,
    pub recognition_superset: Option<QlShape>,
    pub operator_ref: &'static str,
}

impl QlShapeCompression {
    pub fn to_onefold(disclosed: QlShape) -> Option<Self> {
        let disclosed_fold = disclosed.fold_count()?;
        if disclosed_fold != 2 && disclosed_fold != 3 {
            return None;
        }
        let recognition_superset = match disclosed {
            QlShape::Constellation(ConstellationGrain::ThreeFold123) => Some(
                QlShape::Constellation(ConstellationGrain::FourFold1234),
            ),
            _ => None,
        };
        Some(Self {
            presented: QlShape::onefold(),
            disclosed,
            disclosed_fold,
            recognition_superset,
            operator_ref: if disclosed_fold == 3 {
                THREE_TO_ONE_OPERATOR_REF
            } else {
                COMPRESS_TO_WHOLE_OPERATOR_REF
            },
        })
    }

    pub fn derivation_ref(self) -> String {
        format!("{}:from:{}", self.operator_ref, self.disclosed.shape_ref())
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

/// The canonical M3 side of Second Spanda: three articulated quaternary sites.
/// Each site is the same two-bit state already carried by `FoldMotif`, so the
/// form is simultaneously `4^3` and `2^6`, with 64 actual `Codon64` states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FourByFourByFourField {
    pub site_cardinality: usize,
    pub states_per_site: usize,
    pub binary_properties_per_site: usize,
    pub address_cardinality: usize,
}

impl FourByFourByFourField {
    pub fn canonical() -> Self {
        let site_cardinality = FoldMotif::SITES;
        let states_per_site = 4usize;
        let binary_properties_per_site = 2usize;
        let address_cardinality = states_per_site.pow(site_cardinality as u32);
        assert_eq!(address_cardinality, Codon64::COUNT);
        assert_eq!(
            address_cardinality,
            2usize.pow((site_cardinality * binary_properties_per_site) as u32)
        );
        Self {
            site_cardinality,
            states_per_site,
            binary_properties_per_site,
            address_cardinality,
        }
    }

    pub const fn kind(&self) -> QlShapeKind {
        QlShapeKind::FourByFourByFour
    }

    pub const fn shape(&self) -> QlShape {
        QlShape::FourByFourByFour
    }

    pub const fn shape_ref(&self) -> &'static str {
        FOUR_BY_FOUR_BY_FOUR_SHAPE_REF
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

/// The Geometry closure of the direct sixfold, conjugate sixfold and the six
/// same-position relations between them. Eighteen therefore compresses by its
/// sixfold grain to the threefold `{6, 6′, 6↔6′}` without losing the three
/// constituent basis refs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EighteenFoldGeometry {
    pub direct_basis_ref: String,
    pub conjugate_basis_ref: String,
    pub relational_basis_ref: String,
    pub sixfold_cardinality: usize,
    pub fold_cardinality: usize,
    pub compressed_fold_cardinality: usize,
    pub return_anchor_symbol: &'static str,
}

impl EighteenFoldGeometry {
    pub fn canonical() -> Self {
        let relational = RelationalSixfold::canonical();
        let sixfold_cardinality = relational.sites.len();
        Self {
            direct_basis_ref: relational.direct_basis_ref,
            conjugate_basis_ref: relational.conjugate_basis_ref,
            relational_basis_ref: relational.shape_ref().into(),
            sixfold_cardinality,
            fold_cardinality: sixfold_cardinality * 3,
            compressed_fold_cardinality: 3,
            return_anchor_symbol: WHOLE_ANCHOR_SYMBOL,
        }
    }

    pub const fn kind(&self) -> QlShapeKind {
        QlShapeKind::EighteenFold
    }

    pub const fn shape(&self) -> QlShape {
        QlShape::EighteenFold
    }

    pub const fn shape_ref(&self) -> &'static str {
        EIGHTEEN_FOLD_SHAPE_REF
    }

    pub const fn compression_operator_ref(&self) -> &'static str {
        EIGHTEEN_TO_THREE_OPERATOR_REF
    }
}

/// The complete decadic relation-field presentation of the Second-Spanda
/// `4 | 6` split. The first three blocks remain the same 64-count partition as
/// the M3 cubic body; the 6×6 block remains the M2 36-count field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TenByTenProjection {
    pub quaternary_axis: usize,
    pub senary_axis: usize,
    pub axis_cardinality: usize,
    pub block_cardinalities: [usize; 4],
    pub m3_partition_cardinality: usize,
    pub m2_partition_cardinality: usize,
    pub address_cardinality: usize,
}

impl TenByTenProjection {
    pub fn canonical() -> Self {
        let quaternary_axis = 4usize;
        let senary_axis = 6usize;
        let axis_cardinality = quaternary_axis + senary_axis;
        let block_cardinalities = [
            quaternary_axis * quaternary_axis,
            quaternary_axis * senary_axis,
            senary_axis * quaternary_axis,
            senary_axis * senary_axis,
        ];
        let m3_partition_cardinality = block_cardinalities[..3].iter().sum();
        let m2_partition_cardinality = block_cardinalities[3];
        let address_cardinality = axis_cardinality * axis_cardinality;
        Self {
            quaternary_axis,
            senary_axis,
            axis_cardinality,
            block_cardinalities,
            m3_partition_cardinality,
            m2_partition_cardinality,
            address_cardinality,
        }
    }

    pub const fn kind(&self) -> QlShapeKind {
        QlShapeKind::TenByTen
    }

    pub const fn shape(&self) -> QlShape {
        QlShape::TenByTen
    }

    pub const fn shape_ref(&self) -> &'static str {
        TEN_BY_TEN_SHAPE_REF
    }
}

/// The executable Geometry reading of Second Spanda. The canonical body is
/// the M3 `4×4×4` field coupled to the M2 `6×6` field; the `10×10` object is a
/// projection of the same `64 | 36` accounting, never its replacement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecondSpandaGeometry {
    pub m3_quaternary_cubic: FourByFourByFourField,
    pub m2_senary_square: SixBySixField,
    pub decadic_projection: TenByTenProjection,
}

impl SecondSpandaGeometry {
    pub fn canonical() -> Self {
        let m3_quaternary_cubic = FourByFourByFourField::canonical();
        let m2_senary_square = SixBySixField::canonical();
        let decadic_projection = TenByTenProjection::canonical();
        let m3 = m3_quaternary_cubic.address_cardinality;
        let m2 = m2_senary_square.addresses.len();
        assert_eq!(m3, 64);
        assert_eq!(m2, 36);
        assert_eq!(m3 + m2, decadic_projection.address_cardinality);
        assert_eq!(m3, decadic_projection.m3_partition_cardinality);
        assert_eq!(m2, decadic_projection.m2_partition_cardinality);
        Self {
            m3_quaternary_cubic,
            m2_senary_square,
            decadic_projection,
        }
    }

    pub fn totality(&self) -> usize {
        self.m3_quaternary_cubic.address_cardinality + self.m2_senary_square.addresses.len()
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
        assert_eq!(direct.shape_ref(), "ql:shape:1.0.0:constellation:sixfold");
    }

    #[test]
    fn fold_reading_completes_one_through_twelve_without_changing_structural_membership() {
        assert_eq!(QlShape::onefold().fold_count(), Some(1));
        assert_eq!(
            QlShape::Constellation(ConstellationGrain::TwoFold).fold_count(),
            Some(2)
        );
        assert_eq!(QlShape::canonical_sevenfold().fold_count(), Some(7));
        assert_eq!(QlShape::canonical_sevenfold().shape_ref(), SEVEN_FOLD_SHAPE_REF);
        assert_eq!(
            QlShape::Constellation(ConstellationGrain::TwelveFold).fold_count(),
            Some(12)
        );
    }

    #[test]
    fn disclosed_two_and_threefolds_compress_to_the_existing_onefold_anchor() {
        let two = QlShapeCompression::to_onefold(QlShape::Constellation(
            ConstellationGrain::TwoFold,
        ))
        .unwrap();
        assert_eq!(two.presented, QlShape::onefold());
        assert_eq!(two.disclosed_fold, 2);
        assert_eq!(two.operator_ref, COMPRESS_TO_WHOLE_OPERATOR_REF);

        let three = QlShapeCompression::to_onefold(QlShape::Constellation(
            ConstellationGrain::ThreeFold123,
        ))
        .unwrap();
        assert_eq!(three.presented.fold_count(), Some(1));
        assert_eq!(three.disclosed_fold, 3);
        assert_eq!(three.operator_ref, THREE_TO_ONE_OPERATOR_REF);
        assert_eq!(
            three.recognition_superset,
            Some(QlShape::Constellation(ConstellationGrain::FourFold1234))
        );
        assert!(three.derivation_ref().contains("threefold-123"));
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
    fn canonical_m3_cubic_is_the_existing_four_cubed_and_two_to_sixth_field() {
        let field = FourByFourByFourField::canonical();
        assert_eq!(field.site_cardinality, 3);
        assert_eq!(field.states_per_site, 4);
        assert_eq!(field.binary_properties_per_site, 2);
        assert_eq!(field.address_cardinality, 64);
        assert_eq!(field.address_cardinality, Codon64::COUNT);
        assert_eq!(field.shape_ref(), FOUR_BY_FOUR_BY_FOUR_SHAPE_REF);
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
    fn eighteenfold_is_three_sixfolds_and_compresses_to_three() {
        let shape = EighteenFoldGeometry::canonical();
        assert_eq!(shape.sixfold_cardinality, 6);
        assert_eq!(shape.fold_cardinality, 18);
        assert_eq!(shape.compressed_fold_cardinality, 3);
        assert_eq!(shape.shape().fold_count(), Some(18));
        assert_eq!(shape.return_anchor_symbol, "0/1");
        assert_eq!(shape.compression_operator_ref(), EIGHTEEN_TO_THREE_OPERATOR_REF);
    }

    #[test]
    fn second_spanda_geometry_preserves_the_sixty_four_thirty_six_split_in_ten_square() {
        let geometry = SecondSpandaGeometry::canonical();
        assert_eq!(geometry.m3_quaternary_cubic.address_cardinality, 64);
        assert_eq!(geometry.m2_senary_square.addresses.len(), 36);
        assert_eq!(geometry.totality(), 100);
        assert_eq!(geometry.decadic_projection.axis_cardinality, 10);
        assert_eq!(geometry.decadic_projection.block_cardinalities, [16, 24, 24, 36]);
        assert_eq!(geometry.decadic_projection.m3_partition_cardinality, 64);
        assert_eq!(geometry.decadic_projection.m2_partition_cardinality, 36);
        assert_eq!(geometry.decadic_projection.address_cardinality, 100);
    }

    #[test]
    fn portable_shape_fixture_carries_the_same_cardinality_and_return_laws() {
        let fixture = include_str!("../../../fixtures/kernel/ql-shape-contract-v1.json");

        assert!(fixture.contains("\"address_cardinality\": 16"));
        assert!(fixture.contains("\"address_cardinality\": 36"));
        assert!(fixture.contains("\"site_cardinality\": 6"));
        assert!(fixture.contains("\"four_by_four_by_four\""));
        assert!(fixture.contains("\"address_cardinality\": 64"));
        assert!(fixture.contains("\"eighteen_fold\""));
        assert!(fixture.contains("\"fold_cardinality\": 18"));
        assert!(fixture.contains("\"ten_by_ten\""));
        assert!(fixture.contains("\"address_cardinality\": 100"));
        assert!(fixture.contains("\"return_through\": \"0/1\""));
        assert!(fixture.contains("\"shape_address_asserts_semantic_relation\": false"));
    }
}
