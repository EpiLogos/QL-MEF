use crate::{ContextFrameId, LensId};

/// First executable music/harmonic cut promoted from the accepted QL/MEF kernel.
pub const MUSICAL_HARMONIC_VERSION: &str = "0.1.0";

/// Evidence/decision class carried by every promoted musical relation.
///
/// This is intentionally more specific than [`crate::ResultClass`]: it records
/// whether a musical statement is directly authored/accepted, is a finite
/// consequence of accepted relations, remains research, or is an explicit open
/// edge awaiting authored determination.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MusicalEvidenceClass {
    AuthoredAccepted,
    FormallyDerivable,
    ResearchProposed,
    OpenEdge,
}

impl MusicalEvidenceClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoredAccepted => "authored-accepted",
            Self::FormallyDerivable => "formally-derivable",
            Self::ResearchProposed => "research-proposed",
            Self::OpenEdge => "open-edge",
        }
    }
}

/// Exact positive rational relation used by the harmonic layer.
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

/// One finite relation in the ratified M1 harmonic ratio field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HarmonicRelation {
    pub id: &'static str,
    pub ratio: HarmonicRatio,
    pub evidence: MusicalEvidenceClass,
    pub provenance_ref: &'static str,
    pub musical_consequence: &'static str,
}

/// Exact ratio vocabulary ratified by the current M1/Paramaśiva capability
/// matrix. Ordering follows the matrix's authored `musical_theory.ratios` list.
pub const HARMONIC_RELATIONS: [HarmonicRelation; 8] = [
    HarmonicRelation {
        id: "ratio.1:1",
        ratio: HarmonicRatio::canonical(1, 1),
        evidence: MusicalEvidenceClass::AuthoredAccepted,
        provenance_ref: "M1 musical_theory.ratios; M1-C13",
        musical_consequence: "identity / unison reference",
    },
    HarmonicRelation {
        id: "ratio.4:3",
        ratio: HarmonicRatio::canonical(4, 3),
        evidence: MusicalEvidenceClass::AuthoredAccepted,
        provenance_ref: "M1 musical_theory.ratios; Second Spanda",
        musical_consequence: "primary 4:3 harmonic relation",
    },
    HarmonicRelation {
        id: "ratio.3:4",
        ratio: HarmonicRatio::canonical(3, 4),
        evidence: MusicalEvidenceClass::AuthoredAccepted,
        provenance_ref: "M1 musical_theory.ratios",
        musical_consequence: "reciprocal of 4:3",
    },
    HarmonicRelation {
        id: "ratio.3:2",
        ratio: HarmonicRatio::canonical(3, 2),
        evidence: MusicalEvidenceClass::AuthoredAccepted,
        provenance_ref: "M1 musical_theory.ratios",
        musical_consequence: "composes with 4:3 to close 2:1",
    },
    HarmonicRelation {
        id: "ratio.2:3",
        ratio: HarmonicRatio::canonical(2, 3),
        evidence: MusicalEvidenceClass::AuthoredAccepted,
        provenance_ref: "M1 musical_theory.ratios",
        musical_consequence: "reciprocal of 3:2",
    },
    HarmonicRelation {
        id: "ratio.16:9",
        ratio: HarmonicRatio::canonical(16, 9),
        evidence: MusicalEvidenceClass::AuthoredAccepted,
        provenance_ref: "M1 Second Spanda; musical_theory.ratios",
        musical_consequence: "Second-Spanda harmonic field; 9:8 completes it to 2:1",
    },
    HarmonicRelation {
        id: "ratio.9:8",
        ratio: HarmonicRatio::canonical(9, 8),
        evidence: MusicalEvidenceClass::AuthoredAccepted,
        provenance_ref: "M1 musical_theory; M2-C17; M3 formal_laws.epogdoon",
        musical_consequence: "epogdoon generator and exact 72-to-64 handoff ratio",
    },
    HarmonicRelation {
        id: "ratio.2:1",
        ratio: HarmonicRatio::canonical(2, 1),
        evidence: MusicalEvidenceClass::AuthoredAccepted,
        provenance_ref: "M1 musical_theory.ratios and derived_relations",
        musical_consequence: "octave closure of the authored ratio products",
    },
];

/// Resolve one exact canonical harmonic relation by reduced ratio.
pub fn harmonic_relation(ratio: HarmonicRatio) -> Option<&'static HarmonicRelation> {
    HARMONIC_RELATIONS
        .iter()
        .find(|relation| relation.ratio == ratio)
}

/// Exact 72 -> 64 epogdoon/DET mapping ratified in the current M3 matrix.
///
/// The *mapping law* is accepted and deterministic. The richer semantic meaning
/// of the fold remains research/open in M3, so this function deliberately does
/// not assign a musical or metaphysical interpretation to each collision.
pub const fn epogdoon_72_to_64(source_index: u8) -> Option<u8> {
    if source_index >= 72 {
        return None;
    }
    Some(((source_index as u16 * 8) / 9) as u8)
}

/// Number of 72-space source indices mapping to a 64-space target.
pub const fn epogdoon_preimage_width(target_index: u8) -> Option<u8> {
    if target_index >= 64 {
        return None;
    }

    let mut source = 0_u8;
    let mut count = 0_u8;
    while source < 72 {
        if let Some(target) = epogdoon_72_to_64(source) {
            if target == target_index {
                count += 1;
            }
        }
        source += 1;
    }
    Some(count)
}

/// One of Mahāmāyā's sixteen static clock apertures.
///
/// These are 360-degree division grammars from the M3 transcription/world-clock
/// field. They are deliberately named `ClockAperture`, not `LensId`: the latter
/// is the canonical 12-lens L/L' MEF registry, while this table is an M3-local
/// clock structure with its own reciprocal relation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct M3ClockAperture {
    pub index: u8,
    pub sectors: u16,
    pub arc_degrees: u16,
    pub reciprocal_index: u8,
}

pub const M3_CLOCK_APERTURES: [M3ClockAperture; 16] = [
    M3ClockAperture { index: 0, sectors: 1, arc_degrees: 360, reciprocal_index: 15 },
    M3ClockAperture { index: 1, sectors: 2, arc_degrees: 180, reciprocal_index: 14 },
    M3ClockAperture { index: 2, sectors: 4, arc_degrees: 90, reciprocal_index: 13 },
    M3ClockAperture { index: 3, sectors: 8, arc_degrees: 45, reciprocal_index: 12 },
    M3ClockAperture { index: 4, sectors: 9, arc_degrees: 40, reciprocal_index: 11 },
    M3ClockAperture { index: 5, sectors: 10, arc_degrees: 36, reciprocal_index: 10 },
    M3ClockAperture { index: 6, sectors: 12, arc_degrees: 30, reciprocal_index: 9 },
    M3ClockAperture { index: 7, sectors: 15, arc_degrees: 24, reciprocal_index: 8 },
    M3ClockAperture { index: 8, sectors: 24, arc_degrees: 15, reciprocal_index: 7 },
    M3ClockAperture { index: 9, sectors: 30, arc_degrees: 12, reciprocal_index: 6 },
    M3ClockAperture { index: 10, sectors: 36, arc_degrees: 10, reciprocal_index: 5 },
    M3ClockAperture { index: 11, sectors: 40, arc_degrees: 9, reciprocal_index: 4 },
    M3ClockAperture { index: 12, sectors: 45, arc_degrees: 8, reciprocal_index: 3 },
    M3ClockAperture { index: 13, sectors: 90, arc_degrees: 4, reciprocal_index: 2 },
    M3ClockAperture { index: 14, sectors: 180, arc_degrees: 2, reciprocal_index: 1 },
    M3ClockAperture { index: 15, sectors: 360, arc_degrees: 1, reciprocal_index: 0 },
];

pub fn m3_clock_aperture(index: u8) -> Option<&'static M3ClockAperture> {
    M3_CLOCK_APERTURES.get(usize::from(index))
}

pub const M3_CLOCK_APERTURE_EVIDENCE: MusicalEvidenceClass =
    MusicalEvidenceClass::AuthoredAccepted;
pub const M3_CLOCK_APERTURE_PROVENANCE: &str =
    "M3 current_human_ratification + clock_system.lenses/lens_reciprocals";

/// One tonic-lens x Context-Frame/modal address in the authored 12 x 7 = 84
/// landscape. This carries existing LensId and ContextFrameId identities rather
/// than creating a second pitch/mode coordinate system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TonicContextFrame {
    pub tonic_lens: LensId,
    pub context_frame: ContextFrameId,
}

/// Enumerate the complete deterministic 84-address tonic/mode landscape.
pub fn tonic_context_frame_landscape() -> impl Iterator<Item = TonicContextFrame> {
    LensId::ALL.into_iter().flat_map(|tonic_lens| {
        ContextFrameId::ALL
            .into_iter()
            .map(move |context_frame| TonicContextFrame {
                tonic_lens,
                context_frame,
            })
    })
}

/// Provenance for the 84-address landscape itself.
pub const TONIC_CONTEXT_FRAME_EVIDENCE: MusicalEvidenceClass =
    MusicalEvidenceClass::AuthoredAccepted;
pub const TONIC_CONTEXT_FRAME_PROVENANCE: &str =
    "M1 musical_theory.mode_tonic_landscape: 12 lens tonics x 7 Context Frame modes = 84";

/// The current M3 matrix explicitly leaves the interpretation of epogdoon fold
/// collisions open even though the integer mapping is executable.
pub const EPOGDOON_FOLD_SEMANTICS: MusicalEvidenceClass = MusicalEvidenceClass::OpenEdge;
pub const EPOGDOON_FOLD_SEMANTICS_PROVENANCE: &str =
    "M3-C02 state=implemented-fold-semantics-open";
