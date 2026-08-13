import { mefLens } from "./mef-entry.js";
export const MEF_SQUARE_C = Object.freeze([
  mefLens("L2", "Logical", 2, "day", "C", "L3", "L3′", ["Tetralemmaic ground", "IS", "IS-NOT", "BOTH", "NEITHER", "SILENCE"]),
  mefLens("L2′", "Alchemical-Elemental", 2, "night", "C", "L3′", "L3", ["Aether", "Earth", "Water", "Air", "Fire", "Salt"]),
  mefLens("L3", "Processual", 3, "day", "C", "L2", "L2′", ["Concrescent desire", "Actual occasion", "Ingression", "Eternal objects", "Community integration", "Satisfaction/Perishing"]),
  mefLens("L3′", "Chronological", 3, "night", "C", "L2′", "L2", ["Spirit (Geist)", "Spring", "Summer", "Autumn", "Winter", "Life (Aufhebung)"])
]);
