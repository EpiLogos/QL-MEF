export function makeServiceResult({ operation, providerState, status, value = null, classification = null, provenance = null, warnings = [] }) {
  return Object.freeze({ operation, providerState, status, value, classification, provenance, warnings: Object.freeze([...warnings]) });
}
