use core::fmt;

use crate::{
    ConjugationDegree, ExpansionSide, QlCoordinate, QlFace, QlPosition, RelationFamily,
    RelationField, StructuralError,
};

/// Version of the promoted pairing/square grammar.
///
/// This is separate from the existing Wiki structural-contract version so the
/// QW0 contract remains stable while the deeper formal grammar can evolve under
/// its own explicit promotion boundary.
pub const PAIRING_GRAMMAR_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum D2CrossPassKind {
    Transform,
    Require,
    Complete,
}

impl D2CrossPassKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Transform => "transform",
            Self::Require => "require",
            Self::Complete => "complete",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalCrossPass {
    D1 {
        position: QlPosition,
        coordinates: [QlCoordinate; 2],
    },
    D2 {
        kind: D2CrossPassKind,
        position: QlPosition,
        coordinates: [QlCoordinate; 2],
    },
    D3 {
        family: RelationFamily,
        pairs: [[QlCoordinate; 2]; 3],
    },
}

impl CanonicalCrossPass {
    pub fn operator_ref(&self) -> String {
        match self {
            Self::D1 { position, .. } => format!(
                "ql:pairing:{PAIRING_GRAMMAR_VERSION}:cross:D1:position-{}",
                position.value()
            ),
            Self::D2 { kind, position, .. } => format!(
                "ql:pairing:{PAIRING_GRAMMAR_VERSION}:cross:D2:{}:position-{}",
                kind.as_str(),
                position.value()
            ),
            Self::D3 { family, .. } => format!(
                "ql:pairing:{PAIRING_GRAMMAR_VERSION}:cross:D3:{}",
                family.as_str()
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PairingError {
    MissingD2ProjectionSide,
    UnexpectedProjectionSide(ConjugationDegree),
    Structural(StructuralError),
}

impl fmt::Display for PairingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingD2ProjectionSide => {
                f.write_str("D2 modulation requires exactly one projection side")
            }
            Self::UnexpectedProjectionSide(degree) => write!(
                f,
                "{} modulation does not accept a projection side",
                degree.as_str()
            ),
            Self::Structural(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for PairingError {}

impl From<StructuralError> for PairingError {
    fn from(value: StructuralError) -> Self {
        Self::Structural(value)
    }
}

/// Build the software conjugate-modulation frame over one selected A/B/C pair.
///
/// This keeps the software D1/D2/D3 modulation distinct from the canonical
/// cross-pass D1/D2/D3 relations below:
/// - D1: the selected two-element direct pair;
/// - D2: the pair plus exactly one conjugate projection;
/// - D3: the complete four-coordinate square.
pub fn build_d_modulation_frame(
    family: RelationFamily,
    pair_index: u8,
    degree: ConjugationDegree,
    projection_side: Option<ExpansionSide>,
) -> Result<RelationField, PairingError> {
    let pair = family.pair(pair_index)?;
    match degree {
        ConjugationDegree::D1 => {
            if projection_side.is_some() {
                return Err(PairingError::UnexpectedProjectionSide(degree));
            }
            Ok(RelationField {
                pair,
                degree,
                expansion_side: None,
                coordinates: vec![
                    QlCoordinate::new(pair.left, QlFace::Direct),
                    QlCoordinate::new(pair.right, QlFace::Direct),
                ],
            })
        }
        ConjugationDegree::D2 => {
            let side = projection_side.ok_or(PairingError::MissingD2ProjectionSide)?;
            Ok(pair.d2(side))
        }
        ConjugationDegree::D3 => {
            if projection_side.is_some() {
                return Err(PairingError::UnexpectedProjectionSide(degree));
            }
            Ok(pair.d3())
        }
    }
}

/// Canonical same-position cross: `(n, n')`.
pub fn canonical_cross_pass_d1(position: QlPosition) -> CanonicalCrossPass {
    CanonicalCrossPass::D1 {
        position,
        coordinates: [
            QlCoordinate::new(position, QlFace::Direct),
            QlCoordinate::new(position, QlFace::Conjugate),
        ],
    }
}

/// Canonical D2 cross-position family.
///
/// `transform`: `(n, (n+1)')`
/// `require`: `(n, (n-1)')`
/// `complete`: `(n, (5-n)')`
pub fn canonical_cross_pass_d2(kind: D2CrossPassKind, position: QlPosition) -> CanonicalCrossPass {
    let n = position.value();
    let conjugate_position = match kind {
        D2CrossPassKind::Transform => (n + 1) % 6,
        D2CrossPassKind::Require => (n + 5) % 6,
        D2CrossPassKind::Complete => 5 - n,
    };
    let conjugate_position =
        QlPosition::new(conjugate_position).expect("canonical D2 remains inside 0..5");
    CanonicalCrossPass::D2 {
        kind,
        position,
        coordinates: [
            QlCoordinate::new(position, QlFace::Direct),
            QlCoordinate::new(conjugate_position, QlFace::Conjugate),
        ],
    }
}

/// Canonical D3 invariance: the selected A/B/C pairing family reproduced on
/// the conjugate face without collapsing family or pair-index provenance.
pub fn canonical_cross_pass_d3(family: RelationFamily) -> CanonicalCrossPass {
    let pairs = family.pairs().map(|(left, right)| {
        let left = QlPosition::new(left).expect("canonical family positions are 0..5");
        let right = QlPosition::new(right).expect("canonical family positions are 0..5");
        [
            QlCoordinate::new(left, QlFace::Conjugate),
            QlCoordinate::new(right, QlFace::Conjugate),
        ]
    });
    CanonicalCrossPass::D3 { family, pairs }
}
