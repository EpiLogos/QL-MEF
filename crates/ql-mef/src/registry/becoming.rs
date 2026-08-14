use crate::LensId;

use super::LensDefinition;

pub const L2: LensDefinition = LensDefinition::new(LensId::L2, "Logical", [
    "Tetralemmaic ground", "IS", "IS-NOT", "BOTH", "NEITHER", "SILENCE",
]);

pub const L2_PRIME: LensDefinition = LensDefinition::new(LensId::L2Prime, "Alchemical-Elemental", [
    "Aether", "Earth", "Water", "Air", "Fire", "Salt",
]);

pub const L3: LensDefinition = LensDefinition::new(LensId::L3, "Processual", [
    "Concrescent desire", "Actual occasion", "Ingression", "Eternal objects", "Community integration", "Satisfaction/Perishing",
]);

pub const L3_PRIME: LensDefinition = LensDefinition::new(LensId::L3Prime, "Chronological", [
    "Spirit (Geist)", "Spring", "Summer", "Autumn", "Winter", "Life (Aufhebung)",
]);
