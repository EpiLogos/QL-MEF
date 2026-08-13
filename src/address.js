import { QLValidationError } from "./errors.js";
import { canonicalFormRef, QL_FACES, QL_POSITIONS } from "./registry.js";

const DEFAULT_FRAME = Object.freeze({ id: "sixfold", version: 1 });

export function canonicalQLAddress(input) {
  if (!input || typeof input !== "object") {
    throw new QLValidationError("INVALID_QL_ADDRESS", "QLAddress must be an object.", { input });
  }

  const frame = canonicalFormRef(input.frame ?? DEFAULT_FRAME);
  if (frame.id !== "sixfold") {
    throw new QLValidationError(
      "UNSUPPORTED_ADDRESS_FRAME",
      `QLAddress currently supports the sixfold frame only; received ${frame.id}@${frame.version}.`,
      { input }
    );
  }

  const position = input.position;
  const face = input.face ?? "direct";
  const depth = input.depth ?? 0;

  if (!Number.isInteger(position) || !QL_POSITIONS.includes(position)) {
    throw new QLValidationError("INVALID_POSITION", "QLAddress position must be an integer P0..P5.", { input });
  }
  if (!QL_FACES.includes(face)) {
    throw new QLValidationError("INVALID_FACE", "QLAddress face must be direct or conjugate.", { input });
  }
  if (!Number.isInteger(depth) || depth < 0) {
    throw new QLValidationError("INVALID_DEPTH", "QLAddress depth must be a non-negative integer.", { input });
  }

  return Object.freeze({ frame, position, face, depth });
}

export function formatQLAddress(input) {
  const address = canonicalQLAddress(input);
  return `qladdr:${address.frame.id}@${address.frame.version}/${address.face}/P${address.position}/d${address.depth}`;
}

export function parseQLAddress(value) {
  if (typeof value !== "string") {
    throw new QLValidationError("INVALID_QL_ADDRESS_STRING", "Canonical QLAddress must be a string.", { value });
  }
  const match = /^qladdr:([a-z0-9-]+)@(\d+)\/(direct|conjugate)\/P([0-5])\/d(\d+)$/.exec(value);
  if (!match) {
    throw new QLValidationError("INVALID_QL_ADDRESS_STRING", "QLAddress string is not canonical.", { value });
  }
  return canonicalQLAddress({
    frame: { id: match[1], version: Number(match[2]) },
    face: match[3],
    position: Number(match[4]),
    depth: Number(match[5])
  });
}
