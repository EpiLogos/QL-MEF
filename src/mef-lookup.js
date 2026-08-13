import { QLValidationError } from "./errors.js";
import { canonicalLensRef } from "./registry.js";
import { MEF_MANIFOLD } from "./mef-registry.js";
const BY_ID = new Map(MEF_MANIFOLD.map((item) => [item.id, item]));
export function getMefLens(input) {
  const ref = canonicalLensRef(typeof input === "string" ? { id: input, version: 1 } : input);
  return BY_ID.get(ref.id);
}
export function getMefSublens(input, position) {
  const item = getMefLens(input);
  if (!Number.isInteger(position) || position < 0 || position > 5) throw new QLValidationError("INVALID_SUBLENS_POSITION", "MEF sublens position must be .0 through .5.", { position });
  return Object.freeze({ lens: Object.freeze({ id: item.id, version: item.version }), position, name: item.sublenses[position].name });
}
export const getMefComplement = (input) => getMefLens(getMefLens(input).complement);
export const getMefMobius = (input) => getMefLens(getMefLens(input).mobius);
