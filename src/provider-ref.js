import { requiredObject, requiredString } from "./contract-utils.js";
export function makeQLProviderRef(input) {
  const p = requiredObject(input, "INVALID_PROVIDER_REF", "provider");
  return Object.freeze({ id: requiredString(p.id, "INVALID_PROVIDER_REF", "provider.id"), version: requiredString(p.version, "INVALID_PROVIDER_REF", "provider.version") });
}
