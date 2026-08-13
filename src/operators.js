import { canonicalQLAddress, formatQLAddress } from "./address.js";
import { QLUnsupportedError } from "./errors.js";
import { QL_KERNEL_VERSION, QL_SCHEMA_VERSION } from "./registry.js";

export const QL_OPERATOR_REGISTRY = Object.freeze([
  "conjugate-address",
  "complement-address",
  "classify-four-plus-two"
]);

function provenance(operation, input, output) {
  return Object.freeze({
    schemaVersion: QL_SCHEMA_VERSION,
    kernelVersion: QL_KERNEL_VERSION,
    mode: "deterministic",
    operation,
    input: formatQLAddress(input),
    output
  });
}

export function conjugateAddress(input) {
  const address = canonicalQLAddress(input);
  const output = canonicalQLAddress({
    ...address,
    face: address.face === "direct" ? "conjugate" : "direct"
  });
  return Object.freeze({ value: output, provenance: provenance("conjugate-address", address, formatQLAddress(output)) });
}

export function complementAddress(input) {
  const address = canonicalQLAddress(input);
  const output = canonicalQLAddress({ ...address, position: 5 - address.position });
  return Object.freeze({ value: output, provenance: provenance("complement-address", address, formatQLAddress(output)) });
}

export function classifyFourPlusTwo(input) {
  const address = canonicalQLAddress(input);
  const value = address.position === 0 || address.position === 5 ? "implicate" : "explicate";
  return Object.freeze({ value, provenance: provenance("classify-four-plus-two", address, value) });
}

export function applyDeterministicOperator(name, input) {
  switch (name) {
    case "conjugate-address":
      return conjugateAddress(input);
    case "complement-address":
      return complementAddress(input);
    case "classify-four-plus-two":
      return classifyFourPlusTwo(input);
    default:
      throw new QLUnsupportedError("UNSUPPORTED_OPERATOR", `Unsupported deterministic QL operator: ${name}.`, { name });
  }
}
