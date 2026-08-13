import { mefLens } from "./mef-entry.js";
export const MEF_SQUARE_B = Object.freeze([
  mefLens("L1", "Causal", 1, "day", "B", "L4", "L4′", ["Svātantrya", "Material cause", "Efficient cause", "Formal cause", "Final cause", "Icchā Śakti / Will"]),
  mefLens("L1′", "Phenomenal", 1, "night", "B", "L4′", "L4", ["Introversion", "Sensation", "Feeling", "Thinking", "Intuition", "Extroversion"]),
  mefLens("L4", "Phenomenological", 4, "day", "B", "L1", "L1′", ["Sein (Being)", "Geworfenheit (Thrownness)", "Dasein (Being-there)", "Zeit (Temporality)", "Besorge (Care)", "Gelassenheit (Releasement)"]),
  mefLens("L4′", "Scientific", 4, "night", "B", "L1′", "L1", ["Prompts", "Traces", "Challenges", "Patterns", "Discovery", "Insight"])
]);
