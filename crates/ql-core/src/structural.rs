use core::fmt;

use crate::{QlFace, QlPosition};

pub const STRUCTURAL_CONTRACT_VERSION: &str = "2.0.0";
pub const WHOLE_ANCHOR_SYMBOL: &str = "0/1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructuralError {
    InvalidPairIndex(u8),
    EmptyRef(&'static str),
    DuplicateCoordinate { position: u8, face: QlFace },
    TooManyMembers(usize),
    ConjugateWithoutDirect(u8),
    InvalidD2Side,
    InvalidReturnAnchor,
    InvalidGroundPosition(u8),
}

impl fmt::Display for StructuralError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPairIndex(value) => write!(f, "pair index must be 0..2, got {value}"),
            Self::EmptyRef(field) => write!(f, "{field} must be a non-empty stable ref"),
            Self::DuplicateCoordinate { position, face } => {
                write!(f, "duplicate structural coordinate {position}/{face}")
            }
            Self::TooManyMembers(value) => {
                write!(f, "constellation cannot exceed twelve positional members, got {value}")
            }
            Self::ConjugateWithoutDirect(position) => write!(
                f,
                "conjugate position {position} requires its direct position in this structural contract"
            ),
            Self::InvalidD2Side => f.write_str("D2 requires exactly one expanded endpoint"),
            Self::InvalidReturnAnchor => {
                f.write_str("return must route through the constellation whole-anchor")
            }
            Self::InvalidGroundPosition(value) => {
                write!(f, "return ground position must be #0, got #{value}")
            }
        }
    }
}

impl std::error::Error for StructuralError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelationFamily {
    A,
    B,
    C,
}

impl RelationFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
        }
    }

    pub const fn pairs(self) -> [(u8, u8); 3] {
        match self {
            Self::A => [(0, 1), (2, 3), (4, 5)],
            Self::B => [(1, 2), (3, 4), (5, 0)],
            Self::C => [(0, 5), (1, 4), (2, 3)],
        }
    }

    pub fn pair(self, pair_index: u8) -> Result<PairInstance, StructuralError> {
        let pair = self
            .pairs()
            .get(usize::from(pair_index))
            .copied()
            .ok_or(StructuralError::InvalidPairIndex(pair_index))?;
        Ok(PairInstance {
            family: self,
            pair_index,
            left: QlPosition::new(pair.0).expect("relation-family positions are canonical 0..5"),
            right: QlPosition::new(pair.1).expect("relation-family positions are canonical 0..5"),
        })
    }
}

impl fmt::Display for RelationFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PairInstance {
    pub family: RelationFamily,
    pub pair_index: u8,
    pub left: QlPosition,
    pub right: QlPosition,
}

impl PairInstance {
    pub fn operator_ref(self) -> String {
        format!(
            "ql:structural:{STRUCTURAL_CONTRACT_VERSION}:pair:{}:{}",
            self.family.as_str(),
            self.pair_index
        )
    }

    pub const fn positions(self) -> (QlPosition, QlPosition) {
        (self.left, self.right)
    }

    pub const fn d1_oppositions(self) -> [ConjugateOpposition; 2] {
        [
            ConjugateOpposition::new(self.left),
            ConjugateOpposition::new(self.right),
        ]
    }

    pub fn d2(self, side: ExpansionSide) -> RelationField {
        let mut coordinates = vec![
            QlCoordinate::new(self.left, QlFace::Direct),
            QlCoordinate::new(self.right, QlFace::Direct),
        ];
        let expanded = match side {
            ExpansionSide::Left => self.left,
            ExpansionSide::Right => self.right,
        };
        coordinates.push(QlCoordinate::new(expanded, QlFace::Conjugate));
        RelationField {
            pair: self,
            degree: ConjugationDegree::D2,
            expansion_side: Some(side),
            coordinates,
        }
    }

    pub fn d3(self) -> RelationField {
        RelationField {
            pair: self,
            degree: ConjugationDegree::D3,
            expansion_side: None,
            coordinates: vec![
                QlCoordinate::new(self.left, QlFace::Direct),
                QlCoordinate::new(self.right, QlFace::Direct),
                QlCoordinate::new(self.left, QlFace::Conjugate),
                QlCoordinate::new(self.right, QlFace::Conjugate),
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConjugationDegree {
    D1,
    D2,
    D3,
}

impl ConjugationDegree {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::D1 => "D1",
            Self::D2 => "D2",
            Self::D3 => "D3",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExpansionSide {
    Left,
    Right,
}

impl ExpansionSide {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QlCoordinate {
    pub position: QlPosition,
    pub face: QlFace,
}

impl QlCoordinate {
    pub const fn new(position: QlPosition, face: QlFace) -> Self {
        Self { position, face }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConjugateOpposition {
    pub position: QlPosition,
}

impl ConjugateOpposition {
    pub const fn new(position: QlPosition) -> Self {
        Self { position }
    }

    pub const fn coordinates(self) -> [QlCoordinate; 2] {
        [
            QlCoordinate::new(self.position, QlFace::Direct),
            QlCoordinate::new(self.position, QlFace::Conjugate),
        ]
    }

    pub fn operator_ref(self) -> String {
        format!(
            "ql:structural:{STRUCTURAL_CONTRACT_VERSION}:conjugation:D1:position-{}",
            self.position.value()
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationField {
    pub pair: PairInstance,
    pub degree: ConjugationDegree,
    pub expansion_side: Option<ExpansionSide>,
    pub coordinates: Vec<QlCoordinate>,
}

impl RelationField {
    pub fn operator_ref(&self) -> String {
        let side = self
            .expansion_side
            .map(|value| format!(":{}", value.as_str()))
            .unwrap_or_default();
        format!(
            "ql:structural:{STRUCTURAL_CONTRACT_VERSION}:field:{}:{}:{}{}",
            self.pair.family.as_str(),
            self.pair.pair_index,
            self.degree.as_str(),
            side
        )
    }

    pub fn vertex_key(&self) -> Vec<(u8, &'static str)> {
        let mut result = self
            .coordinates
            .iter()
            .map(|coordinate| (coordinate.position.value(), coordinate.face.as_str()))
            .collect::<Vec<_>>();
        result.sort_unstable();
        result
    }

    pub fn structural_key(&self) -> (RelationFamily, u8, ConjugationDegree, Option<ExpansionSide>) {
        (
            self.pair.family,
            self.pair.pair_index,
            self.degree,
            self.expansion_side,
        )
    }
}

pub fn all_d3_fields() -> Result<Vec<RelationField>, StructuralError> {
    let mut fields = Vec::with_capacity(9);
    for family in [RelationFamily::A, RelationFamily::B, RelationFamily::C] {
        for pair_index in 0..3 {
            fields.push(family.pair(pair_index)?.d3());
        }
    }
    Ok(fields)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralParticipation {
    pub subject_ref: String,
    pub coordinate: QlCoordinate,
}

impl StructuralParticipation {
    pub fn new(
        subject_ref: impl Into<String>,
        position: QlPosition,
        face: QlFace,
    ) -> Result<Self, StructuralError> {
        let subject_ref = subject_ref.into();
        if subject_ref.trim().is_empty() {
            return Err(StructuralError::EmptyRef("subject_ref"));
        }
        Ok(Self {
            subject_ref,
            coordinate: QlCoordinate::new(position, face),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GroundKind {
    Own,
    Parent,
    Child,
    Other,
    Conjugate,
}

impl GroundKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Own => "own",
            Self::Parent => "parent",
            Self::Child => "child",
            Self::Other => "other",
            Self::Conjugate => "conjugate",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnchorReturn {
    pub from_ref: String,
    pub through_anchor_ref: String,
    pub target_ground_ref: String,
    pub target_ground_position: QlPosition,
    pub target_face: QlFace,
    pub ground_kind: GroundKind,
}

impl AnchorReturn {
    pub fn new(
        from_ref: impl Into<String>,
        through_anchor_ref: impl Into<String>,
        target_ground_ref: impl Into<String>,
        target_face: QlFace,
        ground_kind: GroundKind,
    ) -> Result<Self, StructuralError> {
        let from_ref = from_ref.into();
        let through_anchor_ref = through_anchor_ref.into();
        let target_ground_ref = target_ground_ref.into();
        if from_ref.trim().is_empty() {
            return Err(StructuralError::EmptyRef("from_ref"));
        }
        if through_anchor_ref.trim().is_empty() {
            return Err(StructuralError::EmptyRef("through_anchor_ref"));
        }
        if target_ground_ref.trim().is_empty() {
            return Err(StructuralError::EmptyRef("target_ground_ref"));
        }
        Ok(Self {
            from_ref,
            through_anchor_ref,
            target_ground_ref,
            target_ground_position: QlPosition::new(0).expect("#0 is canonical"),
            target_face,
            ground_kind,
        })
    }

    pub fn operator_ref(&self) -> String {
        format!(
            "ql:structural:{STRUCTURAL_CONTRACT_VERSION}:return:through-anchor:{}",
            self.ground_kind.as_str()
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralConstellation {
    pub anchor_ref: String,
    pub members: Vec<StructuralParticipation>,
    pub returns: Vec<AnchorReturn>,
}

impl StructuralConstellation {
    pub fn new(
        anchor_ref: impl Into<String>,
        members: Vec<StructuralParticipation>,
        returns: Vec<AnchorReturn>,
    ) -> Result<Self, StructuralError> {
        let anchor_ref = anchor_ref.into();
        if anchor_ref.trim().is_empty() {
            return Err(StructuralError::EmptyRef("anchor_ref"));
        }
        if members.len() > 12 {
            return Err(StructuralError::TooManyMembers(members.len()));
        }
        for (index, member) in members.iter().enumerate() {
            if members[..index]
                .iter()
                .any(|other| other.coordinate == member.coordinate)
            {
                return Err(StructuralError::DuplicateCoordinate {
                    position: member.coordinate.position.value(),
                    face: member.coordinate.face,
                });
            }
        }
        for member in members
            .iter()
            .filter(|member| member.coordinate.face == QlFace::Conjugate)
        {
            let position = member.coordinate.position;
            if !members.iter().any(|candidate| {
                candidate.coordinate.position == position && candidate.coordinate.face == QlFace::Direct
            }) {
                return Err(StructuralError::ConjugateWithoutDirect(position.value()));
            }
        }
        if returns
            .iter()
            .any(|route| route.through_anchor_ref != anchor_ref)
        {
            return Err(StructuralError::InvalidReturnAnchor);
        }
        Ok(Self {
            anchor_ref,
            members,
            returns,
        })
    }

    pub fn direct_positions(&self) -> Vec<u8> {
        let mut values = self
            .members
            .iter()
            .filter(|member| member.coordinate.face == QlFace::Direct)
            .map(|member| member.coordinate.position.value())
            .collect::<Vec<_>>();
        values.sort_unstable();
        values
    }

    pub fn conjugate_positions(&self) -> Vec<u8> {
        let mut values = self
            .members
            .iter()
            .filter(|member| member.coordinate.face == QlFace::Conjugate)
            .map(|member| member.coordinate.position.value())
            .collect::<Vec<_>>();
        values.sort_unstable();
        values
    }

    pub fn grain(&self) -> ConstellationGrain {
        let direct = self.direct_positions();
        let conjugate = self.conjugate_positions();
        if direct.is_empty() && conjugate.is_empty() {
            return ConstellationGrain::AnchorOnly;
        }
        if conjugate.is_empty() {
            return match direct.as_slice() {
                [_, _] => ConstellationGrain::TwoFold,
                [1, 2, 3] => ConstellationGrain::ThreeFold123,
                [0, 4, 5] => ConstellationGrain::ThreeFold450,
                [1, 2, 3, 4] => ConstellationGrain::FourFold1234,
                [0, 1, 2, 3, 4] => ConstellationGrain::FourPlusOneGround,
                [1, 2, 3, 4, 5] => ConstellationGrain::FourPlusOneSynthesis,
                [0, 1, 2, 3, 4, 5] => ConstellationGrain::SixFold,
                _ => ConstellationGrain::Other {
                    direct: direct.len() as u8,
                    conjugate: 0,
                },
            };
        }
        if direct.as_slice() == [0, 1, 2, 3, 4, 5] {
            return match conjugate.len() {
                2 => ConstellationGrain::PartialConjugate8,
                3 => ConstellationGrain::PartialConjugate9,
                4 => ConstellationGrain::PartialConjugate10,
                5 => ConstellationGrain::PartialConjugate11,
                6 => ConstellationGrain::TwelveFold,
                _ => ConstellationGrain::Other {
                    direct: 6,
                    conjugate: conjugate.len() as u8,
                },
            };
        }
        ConstellationGrain::Other {
            direct: direct.len() as u8,
            conjugate: conjugate.len() as u8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstellationGrain {
    AnchorOnly,
    TwoFold,
    ThreeFold123,
    ThreeFold450,
    FourFold1234,
    FourPlusOneGround,
    FourPlusOneSynthesis,
    SixFold,
    PartialConjugate8,
    PartialConjugate9,
    PartialConjugate10,
    PartialConjugate11,
    TwelveFold,
    Other { direct: u8, conjugate: u8 },
}

impl ConstellationGrain {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AnchorOnly => "anchor-only",
            Self::TwoFold => "twofold",
            Self::ThreeFold123 => "threefold-123",
            Self::ThreeFold450 => "threefold-450",
            Self::FourFold1234 => "fourfold-1234",
            Self::FourPlusOneGround => "four-plus-one-ground",
            Self::FourPlusOneSynthesis => "four-plus-one-synthesis",
            Self::SixFold => "sixfold",
            Self::PartialConjugate8 => "partial-conjugate-8",
            Self::PartialConjugate9 => "partial-conjugate-9",
            Self::PartialConjugate10 => "partial-conjugate-10",
            Self::PartialConjugate11 => "partial-conjugate-11",
            Self::TwelveFold => "twelvefold",
            Self::Other { .. } => "other",
        }
    }
}
