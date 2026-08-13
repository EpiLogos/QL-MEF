import { QLValidationError } from "./errors.js";

export const QL_KERNEL_VERSION = "0.1.0-q1";
export const QL_SCHEMA_VERSION = "1.0.0";
export const QL_FORM_REGISTRY_VERSION = 1;
export const MEF_LENS_REGISTRY_VERSION = 1;

export const QL_FACES = Object.freeze(["direct", "conjugate"]);
export const QL_POSITIONS = Object.freeze([0, 1, 2, 3, 4, 5]);

export const QL_FORM_REGISTRY = Object.freeze([
  Object.freeze({ id: "sixfold", version: 1, canonical: "qlform:sixfold@1" }),
  Object.freeze({ id: "four-plus-two", version: 1, canonical: "qlform:four-plus-two@1" }),
  Object.freeze({ id: "direct-conjugate", version: 1, canonical: "qlform:direct-conjugate@1" })
]);

export const MEF_LENS_REGISTRY = Object.freeze([
  Object.freeze({ id: "L0", version: 1, name: "Quaternal" }),
  Object.freeze({ id: "L0′", version: 1, name: "Archetypal-Numerical" }),
  Object.freeze({ id: "L1", version: 1, name: "Causal" }),
  Object.freeze({ id: "L1′", version: 1, name: "Phenomenal" }),
  Object.freeze({ id: "L2", version: 1, name: "Logical" }),
  Object.freeze({ id: "L2′", version: 1, name: "Alchemical-Elemental" }),
  Object.freeze({ id: "L3", version: 1, name: "Processual" }),
  Object.freeze({ id: "L3′", version: 1, name: "Chronological" }),
  Object.freeze({ id: "L4", version: 1, name: "Phenomenological" }),
  Object.freeze({ id: "L4′", version: 1, name: "Scientific" }),
  Object.freeze({ id: "L5", version: 1, name: "Para Vāk" }),
  Object.freeze({ id: "L5′", version: 1, name: "Divine Logos" })
]);

const FORM_KEYS = new Set(QL_FORM_REGISTRY.map(({ id, version }) => `${id}@${version}`));
const LENS_KEYS = new Set(MEF_LENS_REGISTRY.map(({ id, version }) => `${id}@${version}`));

export function canonicalFormRef(input) {
  if (!input || typeof input !== "object") {
    throw new QLValidationError("INVALID_FORM_REF", "QLFormRef must be an object.", { input });
  }
  const { id, version } = input;
  if (typeof id !== "string" || !Number.isInteger(version) || version < 1) {
    throw new QLValidationError("INVALID_FORM_REF", "QLFormRef requires string id and positive integer version.", { input });
  }
  if (!FORM_KEYS.has(`${id}@${version}`)) {
    throw new QLValidationError("UNKNOWN_FORM_REF", `Unsupported QLFormRef ${id}@${version}.`, { input });
  }
  return Object.freeze({ id, version });
}

export function canonicalLensRef(input) {
  if (!input || typeof input !== "object") {
    throw new QLValidationError("INVALID_LENS_REF", "LensRef must be an object.", { input });
  }
  const { id, version } = input;
  if (typeof id !== "string" || !Number.isInteger(version) || version < 1) {
    throw new QLValidationError("INVALID_LENS_REF", "LensRef requires string id and positive integer version.", { input });
  }
  if (!LENS_KEYS.has(`${id}@${version}`)) {
    throw new QLValidationError("UNKNOWN_LENS_REF", `Unsupported LensRef ${id}@${version}.`, { input });
  }
  return Object.freeze({ id, version });
}

export function formatFormRef(input) {
  const ref = canonicalFormRef(input);
  return `qlform:${ref.id}@${ref.version}`;
}

export function formatLensRef(input) {
  const ref = canonicalLensRef(input);
  return `meflens:${ref.id}@${ref.version}`;
}
