import { QLValidationError } from "./errors.js";
import { canonicalLensRef } from "./registry.js";
import { getMefSublens } from "./mef-lookup.js";
import { requiredObject, requiredString } from "./contract-utils.js";
import { makeQLTarget } from "./target.js";
import { makeQLProvenance } from "./provenance.js";
export function makeQLReading(input) {
  requiredObject(input, "INVALID_QL_READING", "QLReading");
  const lens = canonicalLensRef(input.lens), sublensPosition = input.sublensPosition ?? null;
  if (sublensPosition !== null) getMefSublens(lens, sublensPosition);
  if (!("reading" in input)) throw new QLValidationError("MISSING_READING", "QLReading requires an explicit reading payload.", { input });
  return Object.freeze({ id: requiredString(input.id, "INVALID_READING_ID", "id"), target: makeQLTarget(input.target), lens, sublensPosition, reading: input.reading, evidenceRefs: Object.freeze([...(input.evidenceRefs ?? [])]), provenance: makeQLProvenance(input.provenance) });
}
export function makeQLRelationReading(input) {
  requiredObject(input, "INVALID_RELATION_READING", "QLRelationReading");
  if (!Array.isArray(input.subjectRefs) || input.subjectRefs.length < 2) throw new QLValidationError("INVALID_RELATION_SUBJECTS", "QLRelationReading requires at least two subject refs.", { input });
  return Object.freeze({ id: requiredString(input.id, "INVALID_READING_ID", "id"), subjectRefs: Object.freeze([...input.subjectRefs]), lenses: Object.freeze((input.lenses ?? []).map(canonicalLensRef)), relation: input.relation, evidenceRefs: Object.freeze([...(input.evidenceRefs ?? [])]), provenance: makeQLProvenance(input.provenance) });
}
export function makeQLSynthesis(input) {
  requiredObject(input, "INVALID_QL_SYNTHESIS", "QLSynthesis");
  if (!Array.isArray(input.inputReadingRefs) || input.inputReadingRefs.length === 0) throw new QLValidationError("MISSING_SYNTHESIS_INPUTS", "QLSynthesis requires input reading refs.", { input });
  return Object.freeze({ id: requiredString(input.id, "INVALID_SYNTHESIS_ID", "id"), inputReadingRefs: Object.freeze([...input.inputReadingRefs]), synthesis: input.synthesis, retainedDifferences: Object.freeze([...(input.retainedDifferences ?? [])]), unresolved: Object.freeze([...(input.unresolved ?? [])]), provenance: makeQLProvenance(input.provenance) });
}
