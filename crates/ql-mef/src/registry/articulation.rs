use crate::LensId;

use super::LensDefinition;

pub const L0: LensDefinition = LensDefinition::new(
    LensId::L0,
    "Quaternal",
    [
        "Why",
        "What",
        "How",
        "Whom/Which/When",
        "Where/Why-for",
        "Why-so/Why-not",
    ],
);

pub const L0_PRIME: LensDefinition = LensDefinition::new(
    LensId::L0Prime,
    "Archetypal-Numerical",
    [
        "One/Unity",
        "Two/Duality",
        "Three/Triad",
        "Four/Quaternity",
        "Five/Pentad",
        "Six/Hexad",
    ],
);

pub const L5: LensDefinition = LensDefinition::new(
    LensId::L5,
    "Para Vāk",
    [
        "Anuttara/Asambhava",
        "Para Vāk",
        "Paśyantī",
        "Madhyamā",
        "Vaikharī",
        "Mātṛkā",
    ],
);

pub const L5_PRIME: LensDefinition = LensDefinition::new(
    LensId::L5Prime,
    "Divine Logos",
    [
        "Arche",
        "Apokalypsis",
        "Dynamis",
        "Sophia",
        "Parousia",
        "Epi-Logos",
    ],
);
