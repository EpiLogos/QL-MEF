use ql_core::{
    CanonicalCrossPass, D2CrossPassKind, KernelRelationId, QlCoordinate, QlFace, QlFamily,
    QlPosition, RelationFamily, canonical_cross_pass_d1, canonical_cross_pass_d2,
    canonical_cross_pass_d3,
};

use crate::{ContextFrameCut, ContextFrameId, LensId, MefUnitFace};

pub const MUSICAL_HARMONIC_VERSION: &str = "1.0.0";
pub const MUSICAL_DERIVATION_SOURCE_PATH: &str = "docs/sources/ql-musical-derivation-v3.md";
pub const MUSICAL_DERIVATION_SOURCE_BLOB: &str = "6414c56c6241c3da46e1ea6fdcd7a09b6b66c5aa";
pub const MUSICAL_DERIVATION_VENDOR_COMMIT: &str = "9429a9fb5173f32138799046e8e2a4d7a2d86968";
pub const KERNEL_FAMILY_RELATION: KernelRelationId = KernelRelationId::FamilySamePosition;
pub const FIRST_SPANDA_HORIZONTAL: (u8, u8) = (3, 3);
pub const SECOND_SPANDA_VERTICAL: (u8, u8) = (4, 2);
pub const IONIAN_OFFSETS: [u8; 7] = [0, 2, 4, 5, 7, 9, 11];
pub const NAME_CONTENT: [&str; 6] = ["Truth", "Mind", "Word", "Logos", "Son", "Image"];
pub const POWER_CONTENT: [&str; 6] = ["Play", "Need", "Sacrifice", "Decision", "Love", "Work"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HarmonicRatio {
    numerator: u32,
    denominator: u32,
}

impl HarmonicRatio {
    pub const fn new(numerator: u32, denominator: u32) -> Option<Self> {
        if numerator == 0 || denominator == 0 {
            return None;
        }
        let divisor = gcd(numerator, denominator);
        Some(Self {
            numerator: numerator / divisor,
            denominator: denominator / divisor,
        })
    }
    const fn canonical(numerator: u32, denominator: u32) -> Self {
        Self {
            numerator,
            denominator,
        }
    }
    pub const fn numerator(self) -> u32 {
        self.numerator
    }
    pub const fn denominator(self) -> u32 {
        self.denominator
    }
    pub const fn reciprocal(self) -> Self {
        Self {
            numerator: self.denominator,
            denominator: self.numerator,
        }
    }
    pub const fn multiply(self, rhs: Self) -> Self {
        reduce(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
    pub const fn divide(self, rhs: Self) -> Self {
        reduce(
            self.numerator * rhs.denominator,
            self.denominator * rhs.numerator,
        )
    }
}

const fn gcd(mut left: u32, mut right: u32) -> u32 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left
}
const fn reduce(numerator: u32, denominator: u32) -> HarmonicRatio {
    let divisor = gcd(numerator, denominator);
    HarmonicRatio {
        numerator: numerator / divisor,
        denominator: denominator / divisor,
    }
}

pub const CANONICAL_RATIOS: [HarmonicRatio; 8] = [
    HarmonicRatio::canonical(1, 1),
    HarmonicRatio::canonical(4, 3),
    HarmonicRatio::canonical(3, 4),
    HarmonicRatio::canonical(3, 2),
    HarmonicRatio::canonical(2, 3),
    HarmonicRatio::canonical(16, 9),
    HarmonicRatio::canonical(9, 8),
    HarmonicRatio::canonical(2, 1),
];

pub fn spanda_cross_reading_ratios() -> [HarmonicRatio; 4] {
    [
        HarmonicRatio::canonical(4, 3),
        HarmonicRatio::canonical(2, 3),
        HarmonicRatio::canonical(3, 4),
        HarmonicRatio::canonical(3, 2),
    ]
}

pub type PitchClass = u8;
pub const ALL_PITCH_CLASSES: [PitchClass; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
pub const fn pitch_name(pitch: PitchClass) -> &'static str {
    match pitch % 12 {
        0 => "C",
        1 => "C#",
        2 => "D",
        3 => "D#",
        4 => "E",
        5 => "F",
        6 => "F#",
        7 => "G",
        8 => "G#",
        9 => "A",
        10 => "A#",
        _ => "B",
    }
}
pub const fn transpose(pitch: PitchClass, semitones: u8) -> PitchClass {
    (pitch + semitones) % 12
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MusicalBasis {
    Chromatic,
    Fifths,
}
impl MusicalBasis {
    pub const ALL: [Self; 2] = [Self::Chromatic, Self::Fifths];
    pub const fn generator_ratio(self) -> HarmonicRatio {
        match self {
            Self::Chromatic => HarmonicRatio::canonical(9, 8),
            Self::Fifths => HarmonicRatio::canonical(3, 2),
        }
    }
    pub const fn generator_semitones(self) -> u8 {
        match self {
            Self::Chromatic => 2,
            Self::Fifths => 7,
        }
    }
    pub const fn conjugate_axis_semitones(self) -> u8 {
        match self {
            Self::Chromatic => 1,
            Self::Fifths => 6,
        }
    }
    pub fn pitch_at(self, coordinate: QlCoordinate) -> PitchClass {
        let n = coordinate.position.value();
        let direct = match self {
            Self::Chromatic => (2 * n) % 12,
            Self::Fifths => (7 * n) % 12,
        };
        transpose(
            direct,
            if coordinate.face == QlFace::Conjugate {
                self.conjugate_axis_semitones()
            } else {
                0
            },
        )
    }
    pub fn helix(self, face: QlFace) -> [PitchClass; 6] {
        std::array::from_fn(|i| self.pitch_at(QlCoordinate::new(position(i as u8), face)))
    }
    pub fn substrate(self) -> [PitchClass; 12] {
        let direct = self.helix(QlFace::Direct);
        let prime = self.helix(QlFace::Conjugate);
        std::array::from_fn(|i| if i < 6 { direct[i] } else { prime[i - 6] })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KernelFamilyAddress {
    pub family: QlFamily,
    pub coordinate: QlCoordinate,
}
pub const fn c_p_l_family_views(coordinate: QlCoordinate) -> [KernelFamilyAddress; 3] {
    [
        KernelFamilyAddress {
            family: QlFamily::C,
            coordinate,
        },
        KernelFamilyAddress {
            family: QlFamily::P,
            coordinate,
        },
        KernelFamilyAddress {
            family: QlFamily::L,
            coordinate,
        },
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LensAnchor {
    pub lens: LensId,
    pub basis: MusicalBasis,
    pub coordinate: QlCoordinate,
    pub pitch: PitchClass,
}
pub fn lens_kernel_coordinate(lens: LensId) -> QlCoordinate {
    QlCoordinate::new(position(lens.index()), lens.face().kernel_face())
}
pub fn lens_anchor(basis: MusicalBasis, lens: LensId) -> LensAnchor {
    let coordinate = lens_kernel_coordinate(lens);
    LensAnchor {
        lens,
        basis,
        coordinate,
        pitch: basis.pitch_at(coordinate),
    }
}
pub fn lens_anchors(basis: MusicalBasis) -> [LensAnchor; 12] {
    std::array::from_fn(|i| lens_anchor(basis, LensId::ALL[i]))
}
pub fn pitch_at_lens(basis: MusicalBasis, lens: LensId, local: QlCoordinate) -> PitchClass {
    transpose(basis.pitch_at(local), lens_anchor(basis, lens).pitch)
}

pub fn explicate_coordinates() -> Vec<QlCoordinate> {
    [QlFace::Direct, QlFace::Conjugate]
        .into_iter()
        .flat_map(|face| (1_u8..=4).map(move |n| QlCoordinate::new(position(n), face)))
        .collect()
}
pub fn implicate_coordinates() -> Vec<QlCoordinate> {
    [QlFace::Direct, QlFace::Conjugate]
        .into_iter()
        .flat_map(|face| {
            [0_u8, 5]
                .into_iter()
                .map(move |n| QlCoordinate::new(position(n), face))
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MusicalSquare {
    pub family: RelationFamily,
    pub pair_index: u8,
    pub coordinates: [QlCoordinate; 4],
    pub pitches: [PitchClass; 4],
}
pub fn musical_square(
    basis: MusicalBasis,
    lens: LensId,
    family: RelationFamily,
    pair_index: u8,
) -> Option<MusicalSquare> {
    let (left, right) = family.pairs().get(usize::from(pair_index)).copied()?;
    let coordinates = [
        QlCoordinate::new(position(left), QlFace::Direct),
        QlCoordinate::new(position(right), QlFace::Direct),
        QlCoordinate::new(position(left), QlFace::Conjugate),
        QlCoordinate::new(position(right), QlFace::Conjugate),
    ];
    Some(MusicalSquare {
        family,
        pair_index,
        coordinates,
        pitches: coordinates.map(|c| pitch_at_lens(basis, lens, c)),
    })
}
pub fn musical_squares(basis: MusicalBasis, lens: LensId) -> Vec<MusicalSquare> {
    let mut out = Vec::with_capacity(9);
    for family in [RelationFamily::A, RelationFamily::B, RelationFamily::C] {
        for pair_index in 0..3 {
            out.push(musical_square(basis, lens, family, pair_index).expect("canonical pair"));
        }
    }
    out
}

pub fn directed_pitch_delta(from: PitchClass, to: PitchClass) -> u8 {
    (to + 12 - from) % 12
}
pub fn pair_interval_deltas(basis: MusicalBasis, family: RelationFamily, face: QlFace) -> [u8; 3] {
    let pairs = family.pairs();
    std::array::from_fn(|i| {
        let (a, b) = pairs[i];
        directed_pitch_delta(
            basis.pitch_at(QlCoordinate::new(position(a), face)),
            basis.pitch_at(QlCoordinate::new(position(b), face)),
        )
    })
}
pub fn d3_interval_deltas(basis: MusicalBasis, family: RelationFamily) -> [u8; 3] {
    match canonical_cross_pass_d3(family) {
        CanonicalCrossPass::D3 { pairs, .. } => {
            pairs.map(|p| directed_pitch_delta(basis.pitch_at(p[0]), basis.pitch_at(p[1])))
        }
        _ => unreachable!("canonical D3"),
    }
}
pub fn d3_relation_id(family: RelationFamily) -> KernelRelationId {
    canonical_cross_pass_d3(family).kernel_relation_id()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CrossOperator {
    SamePosition,
    Transform,
    Require,
    Complete,
}
impl CrossOperator {
    pub const fn relation_id(self) -> KernelRelationId {
        match self {
            Self::SamePosition => KernelRelationId::CrossSamePosition,
            Self::Transform => KernelRelationId::CrossTransform,
            Self::Require => KernelRelationId::CrossRequire,
            Self::Complete => KernelRelationId::CrossComplete,
        }
    }
}
pub fn cross_interval_deltas(basis: MusicalBasis, operator: CrossOperator) -> [u8; 6] {
    std::array::from_fn(|i| {
        let p = position(i as u8);
        let pass = match operator {
            CrossOperator::SamePosition => canonical_cross_pass_d1(p),
            CrossOperator::Transform => canonical_cross_pass_d2(D2CrossPassKind::Transform, p),
            CrossOperator::Require => canonical_cross_pass_d2(D2CrossPassKind::Require, p),
            CrossOperator::Complete => canonical_cross_pass_d2(D2CrossPassKind::Complete, p),
        };
        let coordinates = match pass {
            CanonicalCrossPass::D1 { coordinates, .. }
            | CanonicalCrossPass::D2 { coordinates, .. } => coordinates,
            CanonicalCrossPass::D3 { .. } => unreachable!("D1/D2 cross"),
        };
        directed_pitch_delta(
            basis.pitch_at(coordinates[0]),
            basis.pitch_at(coordinates[1]),
        )
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuthoredIntervalReference {
    pub operator: &'static str,
    pub chromatic: &'static str,
    pub fifths: &'static str,
}
pub const AUTHORED_INTERVAL_REFERENCES: [AuthoredIntervalReference; 8] = [
    AuthoredIntervalReference {
        operator: "A",
        chromatic: "Whole-tone x 3 (uniform)",
        fifths: "Perfect 5th x 3 (uniform)",
    },
    AuthoredIntervalReference {
        operator: "B",
        chromatic: "Whole-tone x 2 + minor 3rd at cycle-close",
        fifths: "Perfect 4th x 2 + minor 2nd at cycle-close",
    },
    AuthoredIntervalReference {
        operator: "C",
        chromatic: "Minor 7th, tritone, whole-tone",
        fifths: "Major 7th, minor 3rd, perfect 5th",
    },
    AuthoredIntervalReference {
        operator: "D1",
        chromatic: "Minor 2nd x 6",
        fifths: "Tritone x 6",
    },
    AuthoredIntervalReference {
        operator: "D2-transform",
        chromatic: "Minor 3rd uniform",
        fifths: "Minor 2nd + tritone at close",
    },
    AuthoredIntervalReference {
        operator: "D2-require",
        chromatic: "Minor 2nd uniform",
        fifths: "Minor 2nd + perfect 4th at boundary",
    },
    AuthoredIntervalReference {
        operator: "D2-complete",
        chromatic: "11, 9, 3, 1, 6, 3 st",
        fifths: "5, 3, 1, 1, 3, 6 st",
    },
    AuthoredIntervalReference {
        operator: "D3",
        chromatic: "Identical to bimba A, B, C",
        fifths: "Identical to bimba A, B, C",
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModeKind {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}
impl ModeKind {
    pub const ALL: [Self; 7] = [
        Self::Ionian,
        Self::Dorian,
        Self::Phrygian,
        Self::Lydian,
        Self::Mixolydian,
        Self::Aeolian,
        Self::Locrian,
    ];
    pub const fn index(self) -> usize {
        match self {
            Self::Ionian => 0,
            Self::Dorian => 1,
            Self::Phrygian => 2,
            Self::Lydian => 3,
            Self::Mixolydian => 4,
            Self::Aeolian => 5,
            Self::Locrian => 6,
        }
    }
    pub const fn context_frame(self) -> ContextFrameId {
        match self {
            Self::Ionian => ContextFrameId::Cf1,
            Self::Dorian => ContextFrameId::Cf2,
            Self::Phrygian => ContextFrameId::Cf3,
            Self::Lydian => ContextFrameId::Cf4,
            Self::Mixolydian => ContextFrameId::Cf5,
            Self::Aeolian => ContextFrameId::Cf6,
            Self::Locrian => ContextFrameId::Cf7,
        }
    }
    pub const fn form_pattern(self) -> [MefUnitFace; 7] {
        use MefUnitFace::{Name as N, Power as P};
        match self {
            Self::Ionian => [N, N, N, P, P, P, P],
            Self::Dorian => [N, N, P, P, P, P, N],
            Self::Phrygian => [N, P, P, P, P, N, N],
            Self::Lydian => [N, N, N, N, P, P, P],
            Self::Mixolydian => [N, N, N, P, P, P, N],
            Self::Aeolian => [N, N, P, P, P, N, N],
            Self::Locrian => [N, P, P, N, P, N, N],
        }
    }
    pub fn relative_offsets(self) -> [u8; 7] {
        let i = self.index();
        let tonic = IONIAN_OFFSETS[i];
        std::array::from_fn(|degree| (IONIAN_OFFSETS[(i + degree) % 7] + 12 - tonic) % 12)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DiatonicCut {
    pub lens: LensId,
    pub anchor_basis: MusicalBasis,
    pub lens_tonic: PitchClass,
    pub pitches: [PitchClass; 7],
    pub frames: [ContextFrameId; 7],
    pub forms: [MefUnitFace; 7],
}
pub fn cf_diatonic_cut(anchor_basis: MusicalBasis, lens: LensId) -> DiatonicCut {
    let cut = ContextFrameCut::canonical(lens);
    let selected = cut.selected();
    let lens_tonic = lens_anchor(anchor_basis, lens).pitch;
    let pitches = std::array::from_fn(|i| {
        let c = selected[i].coordinate();
        let face = match c.unit_face() {
            MefUnitFace::Name => QlFace::Direct,
            MefUnitFace::Power => QlFace::Conjugate,
        };
        transpose(
            MusicalBasis::Chromatic.pitch_at(QlCoordinate::new(c.local_position(), face)),
            lens_tonic,
        )
    });
    DiatonicCut {
        lens,
        anchor_basis,
        lens_tonic,
        pitches,
        frames: std::array::from_fn(|i| selected[i].frame()),
        forms: std::array::from_fn(|i| selected[i].coordinate().unit_face()),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModeTonicInstance {
    pub lens: LensId,
    pub anchor_basis: MusicalBasis,
    pub mode: ModeKind,
    pub context_frame: ContextFrameId,
    pub scale_beneath_tonic: PitchClass,
    pub tonic: PitchClass,
    pub pitches: [PitchClass; 7],
    pub form_pattern: [MefUnitFace; 7],
}
pub fn mode_tonic_instance(
    anchor_basis: MusicalBasis,
    lens: LensId,
    mode: ModeKind,
) -> ModeTonicInstance {
    let cut = cf_diatonic_cut(anchor_basis, lens);
    let i = mode.index();
    let pitches = std::array::from_fn(|degree| cut.pitches[(i + degree) % 7]);
    ModeTonicInstance {
        lens,
        anchor_basis,
        mode,
        context_frame: mode.context_frame(),
        scale_beneath_tonic: cut.lens_tonic,
        tonic: pitches[0],
        pitches,
        form_pattern: mode.form_pattern(),
    }
}
pub fn mode_tonic_landscape(anchor_basis: MusicalBasis) -> Vec<ModeTonicInstance> {
    LensId::ALL
        .into_iter()
        .flat_map(|lens| {
            ModeKind::ALL
                .into_iter()
                .map(move |mode| mode_tonic_instance(anchor_basis, lens, mode))
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MajorMinorCharacterDegree {
    pub degree: u8,
    pub major_position: u8,
    pub major_face: MefUnitFace,
    pub minor_position: u8,
    pub minor_face: MefUnitFace,
    pub major_offset: u8,
    pub minor_offset: u8,
}
pub const MAJOR_MINOR_CHARACTER_DEGREES: [MajorMinorCharacterDegree; 3] = [
    MajorMinorCharacterDegree {
        degree: 3,
        major_position: 2,
        major_face: MefUnitFace::Name,
        minor_position: 1,
        minor_face: MefUnitFace::Power,
        major_offset: 4,
        minor_offset: 3,
    },
    MajorMinorCharacterDegree {
        degree: 6,
        major_position: 4,
        major_face: MefUnitFace::Power,
        minor_position: 4,
        minor_face: MefUnitFace::Name,
        major_offset: 9,
        minor_offset: 8,
    },
    MajorMinorCharacterDegree {
        degree: 7,
        major_position: 5,
        major_face: MefUnitFace::Power,
        minor_position: 5,
        minor_face: MefUnitFace::Name,
        major_offset: 11,
        minor_offset: 10,
    },
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreMMusicalDerivation {
    pub basis: MusicalBasis,
    pub direct_helix: [PitchClass; 6],
    pub conjugate_helix: [PitchClass; 6],
    pub lens_anchors: [LensAnchor; 12],
    pub explicate_coordinates: Vec<QlCoordinate>,
    pub implicate_coordinates: Vec<QlCoordinate>,
    pub lens_zero_squares: Vec<MusicalSquare>,
    pub reference_diatonic: DiatonicCut,
    pub mode_tonic_landscape: Vec<ModeTonicInstance>,
}
pub fn derive_pre_m_music(basis: MusicalBasis) -> PreMMusicalDerivation {
    PreMMusicalDerivation {
        basis,
        direct_helix: basis.helix(QlFace::Direct),
        conjugate_helix: basis.helix(QlFace::Conjugate),
        lens_anchors: lens_anchors(basis),
        explicate_coordinates: explicate_coordinates(),
        implicate_coordinates: implicate_coordinates(),
        lens_zero_squares: musical_squares(basis, LensId::L0),
        reference_diatonic: cf_diatonic_cut(basis, LensId::L0),
        mode_tonic_landscape: mode_tonic_landscape(basis),
    }
}

fn position(value: u8) -> QlPosition {
    QlPosition::new(value).expect("canonical musical positions are modulo six")
}
