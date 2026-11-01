use crate::{
    Codon64, ConstellationGrain, FoldMotif, FourPlusTwoClass, OperatorValue, PairInstance,
    QlAddress, QlCoordinate, QlFace, QlOperator, QlPosition, RelationFamily, RelationField,
    StructuralConstellation, WHOLE_ANCHOR_SYMBOL, apply_operator,
};

pub const QL_SHAPE_CONTRACT_VERSION: &str = "1.0.0";
pub const QL_GEOMETRY_SHAPE_VERSION: &str = "1.1.0";
pub const SIX_BY_SIX_SHAPE_REF: &str = "ql:shape:1.0.0:6x6:direct-conjugate";
pub const RELATIONAL_SIXFOLD_SHAPE_REF: &str = "ql:shape:1.0.0:6-plus-6-prime";
pub const RELATIONAL_SIXFOLD_OPERATOR_REF: &str =
    "ql:shape:1.0.0:generation:same-position-direct-conjugate";
pub const SEVEN_FOLD_SHAPE_REF: &str = "ql:shape:1.1.0:constellation:partial-conjugate-7";
pub const FOUR_BY_FOUR_BY_FOUR_SHAPE_REF: &str = "ql:shape:1.1.0:4x4x4:m3-three-quaternary-sites";
pub const TEN_BY_TEN_SHAPE_REF: &str = "ql:shape:1.1.0:10x10:second-spanda-4-plus-6";
pub const EIGHTEEN_FOLD_SHAPE_REF: &str = "ql:shape:1.1.0:18-fold:6-plus-6-prime-plus-relational-6";
pub const COMPRESS_TO_WHOLE_OPERATOR_REF: &str = "ql:shape:1.1.0:compression:disclosed-to-0-1";
pub const THREE_TO_ONE_OPERATOR_REF: &str = "ql:shape:1.1.0:compression:3-to-1-recognition";
pub const EIGHTEEN_TO_THREE_OPERATOR_REF: &str = "ql:shape:1.1.0:compression:18-by-6-to-threefold";

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
            Self::Constellation(ConstellationGrain::PartialConjugate7)
            | Self::Constellation(ConstellationGrain::Other {
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

    /// Geometry fold-count, distinct from positional-member count. The anchor
    /// has zero positional members but is the one-fold `0/1` whole.
    pub const fn fold_count(self) -> Option<u8> {
        match self {
            Self::Constellation(ConstellationGrain::AnchorOnly) => Some(1),
            Self::Constellation(ConstellationGrain::TwoFold) => Some(2),
            Self::Constellation(ConstellationGrain::ThreeFold123)
            | Self::Constellation(ConstellationGrain::ThreeFold450) => Some(3),
            Self::Constellation(ConstellationGrain::FourFold1234) => Some(4),
            Self::Constellation(ConstellationGrain::FourPlusOneGround)
            | Self::Constellation(ConstellationGrain::FourPlusOneSynthesis) => Some(5),
            Self::Constellation(ConstellationGrain::SixFold) | Self::RelationalSixfold => Some(6),
            Self::Constellation(ConstellationGrain::PartialConjugate7)
            | Self::Constellation(ConstellationGrain::Other {
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
        Self::Constellation(ConstellationGrain::PartialConjugate7)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QlShapeCompression {
    pub presented: QlShape,
    pub disclosed: QlShape,
    pub disclosed_fold: u8,
    pub recognition_superset: Option<QlShape>,
    pub operator_ref: &'static str,
}

impl QlShapeCompression {
    /// A QL `0/1` whole may present as one fold while retaining a disclosed
    /// two- or three-fold basis. The threefold-123 case also names its existing
    /// fourfold recognition superset.
    pub fn to_onefold(disclosed: QlShape) -> Option<Self> {
        let disclosed_fold = disclosed.fold_count()?;
        if disclosed_fold != 2 && disclosed_fold != 3 {
            return None;
        }
        let recognition_superset = match disclosed {
            QlShape::Constellation(ConstellationGrain::ThreeFold123) => {
                Some(QlShape::Constellation(ConstellationGrain::FourFold1234))
            }
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

/// The operator family of recognition beyond the direct `2 -> 1` / `3 -> 1`.
pub const RECOGNITION_OPERATOR_PREFIX: &str = "ql:shape:1.1.0:recognition";

/// How a sixfold is recognised as its `0/1`. Each is one of QL's own readings
/// of six, and the reading taken is retained, not collapsed: two routes over
/// the same positions stay distinct, as A1 and C2 do.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SixfoldRoute {
    /// `1234 | 05`, divided by the kernel's `classify-four-plus-two`: the
    /// explicate four by `3 + 1`, the implicate pair `5 -> 0`, then `2 -> 1`.
    FourPlusTwo,
    /// `123 | 450`: the two triads, each `3 -> 1`, then `2 -> 1`.
    ThreePlusThree,
    /// The three pairs of one relation family, each `2 -> 1`, then `3 -> 1`.
    Pairs(RelationFamily),
}

impl SixfoldRoute {
    pub const ALL: [Self; 5] = [
        Self::FourPlusTwo,
        Self::ThreePlusThree,
        Self::Pairs(RelationFamily::A),
        Self::Pairs(RelationFamily::B),
        Self::Pairs(RelationFamily::C),
    ];

    pub const fn code(self) -> &'static str {
        match self {
            Self::FourPlusTwo => "4+2",
            Self::ThreePlusThree => "3+3",
            Self::Pairs(RelationFamily::A) => "pairs-A",
            Self::Pairs(RelationFamily::B) => "pairs-B",
            Self::Pairs(RelationFamily::C) => "pairs-C",
        }
    }

    pub fn parse(code: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|route| route.code() == code)
    }

    /// The parts this reading divides the six positions into.
    pub fn parts(self) -> Vec<Vec<u8>> {
        match self {
            Self::FourPlusTwo => {
                let class = |p: u8| {
                    let address = QlAddress::sixfold(p, QlFace::Direct, 0)
                        .expect("positions 0..5 address the sixfold");
                    apply_operator(QlOperator::ClassifyFourPlusTwo, address).value
                };
                let explicate = OperatorValue::FourPlusTwo(FourPlusTwoClass::Explicate);
                let (four, two): (Vec<u8>, Vec<u8>) = (0..6).partition(|p| class(*p) == explicate);
                // The implicate pair is read as the return `5 -> 0`.
                vec![four, two.into_iter().rev().collect()]
            }
            Self::ThreePlusThree => vec![vec![1, 2, 3], vec![4, 5, 0]],
            Self::Pairs(family) => family.pairs().iter().map(|(a, b)| vec![*a, *b]).collect(),
        }
    }
}

/// The route by which a whole reaches its `0/1`, read from its grain.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RecognitionRoute {
    /// A two-fold `2 -> 1`, or the triads `123` / `450` `3 -> 1`.
    Direct,
    /// `1234`: the triad `123` and its recognised fourth.
    ThreePlusOne,
    /// `01234` / `12345`: the four (`3 + 1`) with its ground `0` or its
    /// synthesis `5`, then `2 -> 1`.
    FourPlusOne { with: u8 },
    /// The sixfold by one of its readings.
    Sixfold(SixfoldRoute),
    /// Seven to eleven: the sixfold by its reading, with the night readings
    /// present carried open as their `n <-> n'` pairs until the six complete.
    SixfoldOpen { route: SixfoldRoute, open: Vec<u8> },
    /// All twelve: the sixfold, its conjugate and their coming together (the
    /// six `n <-> n'` pairings, itself a sixfold), each `6 -> 1` by the
    /// reading, then `18 -> 3 -> 1`.
    Eighteen(SixfoldRoute),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecognitionError {
    /// The anchor is already the one-fold `0/1`.
    AlreadyWhole,
    /// Not one of QL's wholes: its recognisable parts are recognised as their
    /// own wholes and the rest stays open.
    NotAWhole(ConstellationGrain),
    /// A reading of six was given for a whole that has no sixfold.
    RouteWithoutSixfold(ConstellationGrain),
}

impl core::fmt::Display for RecognitionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::AlreadyWhole => f.write_str("the anchor is already the one-fold 0/1"),
            Self::NotAWhole(grain) => write!(
                f,
                "{} is not one of QL's wholes; recognise its parts as their own wholes",
                grain.as_str()
            ),
            Self::RouteWithoutSixfold(grain) => {
                write!(f, "a reading of six was given for {}", grain.as_str())
            }
        }
    }
}

/// QL's recognition (`3:1`, `4+2`, `18 -> 3 -> 1`) of one disclosed
/// constellation: the whole presents as the one-fold `0/1` and keeps its
/// basis; the route says how it was reached and names its parts.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Recognition {
    pub disclosed: QlShape,
    pub route: RecognitionRoute,
}

impl Recognition {
    /// The recognition of `form`. `route` chooses the reading of six where
    /// the whole has a sixfold; without one it is `4 + 2`, the backbone.
    pub fn of(
        form: &StructuralConstellation,
        route: Option<SixfoldRoute>,
    ) -> Result<Self, RecognitionError> {
        let grain = form.grain();
        let six = route.unwrap_or(SixfoldRoute::FourPlusTwo);
        let without_route = |r: RecognitionRoute| match route {
            Some(_) => Err(RecognitionError::RouteWithoutSixfold(grain)),
            None => Ok(r),
        };
        let route = match grain {
            ConstellationGrain::AnchorOnly => return Err(RecognitionError::AlreadyWhole),
            ConstellationGrain::TwoFold
            | ConstellationGrain::ThreeFold123
            | ConstellationGrain::ThreeFold450 => without_route(RecognitionRoute::Direct)?,
            ConstellationGrain::FourFold1234 => without_route(RecognitionRoute::ThreePlusOne)?,
            ConstellationGrain::FourPlusOneGround => {
                without_route(RecognitionRoute::FourPlusOne { with: 0 })?
            }
            ConstellationGrain::FourPlusOneSynthesis => {
                without_route(RecognitionRoute::FourPlusOne { with: 5 })?
            }
            ConstellationGrain::SixFold => RecognitionRoute::Sixfold(six),
            ConstellationGrain::PartialConjugate7
            | ConstellationGrain::PartialConjugate8
            | ConstellationGrain::PartialConjugate9
            | ConstellationGrain::PartialConjugate10
            | ConstellationGrain::PartialConjugate11 => RecognitionRoute::SixfoldOpen {
                route: six,
                open: form.conjugate_positions(),
            },
            ConstellationGrain::TwelveFold => RecognitionRoute::Eighteen(six),
            other @ ConstellationGrain::Other { .. } => {
                return Err(RecognitionError::NotAWhole(other));
            }
        };
        let disclosed = match route {
            RecognitionRoute::Eighteen(_) => QlShape::EighteenFold,
            _ => QlShape::Constellation(grain),
        };
        Ok(Self { disclosed, route })
    }

    /// The recognition a presented binding's operator names for `form`, if
    /// that operator is lawful for it.
    pub fn from_operator(form: &StructuralConstellation, operator_ref: &str) -> Option<Self> {
        let routes = core::iter::once(None).chain(SixfoldRoute::ALL.into_iter().map(Some));
        routes
            .filter_map(|route| Self::of(form, route).ok())
            .find(|recognition| recognition.operator_ref() == operator_ref)
    }

    pub const fn presented(&self) -> QlShape {
        QlShape::onefold()
    }

    fn direct(&self) -> Option<QlShapeCompression> {
        match self.route {
            RecognitionRoute::Direct => QlShapeCompression::to_onefold(self.disclosed),
            _ => None,
        }
    }

    pub fn operator_ref(&self) -> String {
        if let Some(direct) = self.direct() {
            return direct.operator_ref.into();
        }
        let p = RECOGNITION_OPERATOR_PREFIX;
        match &self.route {
            RecognitionRoute::Direct => unreachable!("direct routes are two- or three-fold"),
            RecognitionRoute::ThreePlusOne => format!("{p}:3+1"),
            RecognitionRoute::FourPlusOne { .. } => format!("{p}:4+1"),
            RecognitionRoute::Sixfold(route) => format!("{p}:6:{}", route.code()),
            RecognitionRoute::SixfoldOpen { route, .. } => {
                format!("{p}:6:{}:open-D1", route.code())
            }
            RecognitionRoute::Eighteen(route) => format!("{p}:18:{}", route.code()),
        }
    }

    /// The route with its parts named, from the disclosed shape to `0/1`.
    pub fn derivation_ref(&self) -> String {
        if let Some(direct) = self.direct() {
            return direct.derivation_ref();
        }
        let list = |positions: &[u8]| {
            let joined: Vec<String> = positions.iter().map(u8::to_string).collect();
            format!("[{}]", joined.join(","))
        };
        let parts = |route: SixfoldRoute| {
            route
                .parts()
                .iter()
                .map(|part| list(part))
                .collect::<Vec<_>>()
                .join("+")
        };
        let from = format!(
            "{}:from:{}",
            self.operator_ref(),
            self.disclosed.shape_ref()
        );
        match &self.route {
            RecognitionRoute::Direct => unreachable!("direct routes are two- or three-fold"),
            RecognitionRoute::ThreePlusOne => format!("{from}:parts:[1,2,3]+recognised-fourth"),
            RecognitionRoute::FourPlusOne { with } => {
                format!("{from}:parts:[1,2,3,4]+[{with}]")
            }
            RecognitionRoute::Sixfold(route) => format!("{from}:parts:{}", parts(*route)),
            RecognitionRoute::SixfoldOpen { route, open } => {
                format!("{from}:parts:{}:open-D1:{}", parts(*route), list(open))
            }
            RecognitionRoute::Eighteen(route) => format!(
                "{from}:{EIGHTEEN_TO_THREE_OPERATOR_REF}:sixfolds:direct+conjugate+relational:each:{}",
                parts(*route)
            ),
        }
    }
}

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

/// M3/Mahāmāyā side of Second Spanda: three articulated quaternary sites.
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

/// Direct sixfold + conjugate sixfold + their sixfold relation field.
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
        let relational_basis_ref = relational.shape_ref().into();
        Self {
            direct_basis_ref: relational.direct_basis_ref,
            conjugate_basis_ref: relational.conjugate_basis_ref,
            relational_basis_ref,
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

/// `(4+6)^2` projection of the same Second-Spanda `64 | 36` body.
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
    fn existing_shape_refs_stay_stable() {
        assert_eq!(
            QlShape::Constellation(ConstellationGrain::PartialConjugate9).shape_ref(),
            "ql:shape:1.0.0:constellation:partial-conjugate-9"
        );
        assert_eq!(
            QlShape::Constellation(ConstellationGrain::SixFold).shape_ref(),
            "ql:shape:1.0.0:constellation:sixfold"
        );
    }

    #[test]
    fn fold_ladder_has_named_one_and_seven_readings() {
        assert_eq!(QlShape::onefold().fold_count(), Some(1));
        assert_eq!(QlShape::canonical_sevenfold().fold_count(), Some(7));
        assert_eq!(
            QlShape::canonical_sevenfold().shape_ref(),
            SEVEN_FOLD_SHAPE_REF
        );
        assert_eq!(
            QlShape::Constellation(ConstellationGrain::TwelveFold).fold_count(),
            Some(12)
        );
    }

    #[test]
    fn disclosed_two_and_threefolds_compress_to_the_onefold_anchor() {
        let two =
            QlShapeCompression::to_onefold(QlShape::Constellation(ConstellationGrain::TwoFold))
                .unwrap();
        assert_eq!(two.presented, QlShape::onefold());
        assert_eq!(two.operator_ref, COMPRESS_TO_WHOLE_OPERATOR_REF);

        let three = QlShapeCompression::to_onefold(QlShape::Constellation(
            ConstellationGrain::ThreeFold123,
        ))
        .unwrap();
        assert_eq!(three.disclosed_fold, 3);
        assert_eq!(three.operator_ref, THREE_TO_ONE_OPERATOR_REF);
        assert_eq!(
            three.recognition_superset,
            Some(QlShape::Constellation(ConstellationGrain::FourFold1234))
        );
    }

    #[test]
    fn d3_square_is_sixteen_addresses_and_retains_route_identity() {
        let a = FourByFourField::from_pair(RelationFamily::A.pair(1).unwrap());
        let c = FourByFourField::from_pair(RelationFamily::C.pair(2).unwrap());
        assert_eq!(a.axis().len(), 4);
        assert_eq!(a.addresses.len(), 16);
        assert_eq!(a.source.vertex_key(), c.source.vertex_key());
        assert_ne!(a.shape_ref(), c.shape_ref());
    }

    #[test]
    fn second_spanda_is_m3_four_cubed_plus_m2_six_squared() {
        let geometry = SecondSpandaGeometry::canonical();
        assert_eq!(geometry.m3_quaternary_cubic.address_cardinality, 64);
        assert_eq!(geometry.m2_senary_square.addresses.len(), 36);
        assert_eq!(geometry.totality(), 100);
        assert_eq!(
            geometry.decadic_projection.block_cardinalities,
            [16, 24, 24, 36]
        );
        assert_eq!(geometry.decadic_projection.m3_partition_cardinality, 64);
        assert_eq!(geometry.decadic_projection.m2_partition_cardinality, 36);
    }

    #[test]
    fn eighteenfold_is_three_sixfolds_compressing_to_three() {
        let geometry = EighteenFoldGeometry::canonical();
        assert_eq!(geometry.sixfold_cardinality, 6);
        assert_eq!(geometry.fold_cardinality, 18);
        assert_eq!(geometry.compressed_fold_cardinality, 3);
        assert_eq!(geometry.shape().fold_count(), Some(18));
    }

    #[test]
    fn portable_fixture_tracks_the_geometry_extension() {
        let fixture = include_str!("../../../fixtures/kernel/ql-shape-contract-v1.json");
        assert!(fixture.contains("\"four_by_four_by_four\""));
        assert!(fixture.contains("\"eighteen_fold\""));
        assert!(fixture.contains("\"ten_by_ten\""));
        assert!(fixture.contains("\"second_spanda\""));
    }
}
