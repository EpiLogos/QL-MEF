import { QLValidationError } from "./errors.js";
import { canonicalFormRef } from "./registry.js";
import { requiredObject, requiredString } from "./contract-utils.js";
export function makeQLTarget(input) {
  requiredObject(input, "INVALID_QL_TARGET", "QLTarget");
  if (input.subjectRef === undefined || input.subjectRef === null) throw new QLValidationError("MISSING_SUBJECT_REF", "QLTarget requires an opaque client-owned subjectRef.", { input });
  const frame = input.frame ? canonicalFormRef(input.frame) : undefined;
  return Object.freeze({
    subjectRef: input.subjectRef,
    ...(input.subjectType ? { subjectType: requiredString(input.subjectType, "INVALID_SUBJECT_TYPE", "subjectType") } : {}),
    ...(frame ? { frame } : {}),
    contextRefs: Object.freeze([...(input.contextRefs ?? [])])
  });
}
