use crate::LensId;

use super::LensDefinition;

pub const L1: LensDefinition = LensDefinition::new(
    LensId::L1,
    "Causal",
    [
        "Svātantrya",
        "Material cause",
        "Efficient cause",
        "Formal cause",
        "Final cause",
        "Icchā Śakti / Will",
    ],
);

pub const L1_PRIME: LensDefinition = LensDefinition::new(
    LensId::L1Prime,
    "Phenomenal",
    [
        "Introversion",
        "Sensation",
        "Feeling",
        "Thinking",
        "Intuition",
        "Extroversion",
    ],
);

pub const L4: LensDefinition = LensDefinition::new(
    LensId::L4,
    "Phenomenological",
    [
        "Sein (Being)",
        "Geworfenheit (Thrownness)",
        "Dasein (Being-there)",
        "Zeit (Temporality)",
        "Besorge (Care)",
        "Gelassenheit (Releasement)",
    ],
);

pub const L4_PRIME: LensDefinition = LensDefinition::new(
    LensId::L4Prime,
    "Scientific",
    [
        "Prompts",
        "Traces",
        "Challenges",
        "Patterns",
        "Discovery",
        "Insight",
    ],
);
