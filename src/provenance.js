import { QLValidationError } from "./errors.js";
import { QL_SCHEMA_VERSION } from "./registry.js";
import { MEF_MANIFOLD_VERSION } from "./mef-registry.js";
import { QL_RESULT_CLASSES } from "./result-classes.js";
import { makeQLProviderRef } from "./provider-ref.js";
import { requiredObject, requiredString } from "./contract-utils.js";
export function makeQLProvenance(input) {
  requiredObject(input, "INVALID_QL_PROVENANCE", "QLProvenance");
  if (!QL_RESULT_CLASSES.includes(input.mode)) throw new QLValidationError("INVALID_RESULT_CLASS", "Unsupported QL result class.", { mode: input.mode });
  return Object.freeze({ schemaVersion: QL_SCHEMA_VERSION, mefRegistryVersion: MEF_MANIFOLD_VERSION, provider: makeQLProviderRef(input.provider), operation: requiredString(input.operation, "INVALID_OPERATION", "operation"), mode: input.mode, inputRefs: Object.freeze([...(input.inputRefs ?? [])]), sourceRevisions: Object.freeze([...(input.sourceRevisions ?? [])]), ...(input.model ? { model: Object.freeze({ ...input.model }) } : {}), warnings: Object.freeze([...(input.warnings ?? [])]) });
}
