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
