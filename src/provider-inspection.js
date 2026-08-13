import { QL_SCHEMA_VERSION } from "./registry.js";
export const QL_PROVIDER_STATES = Object.freeze(["absent", "ready", "degraded", "incompatible"]);
const list = (value) => Array.isArray(value) ? value : [];
export function inspectQLProvider(provider) {
  if (!provider) return Object.freeze({ state:"absent", capabilities:null, warnings:Object.freeze(["QL provider is absent."]) });
  if (typeof provider.capabilities !== "function") return Object.freeze({ state:"incompatible", capabilities:null, warnings:Object.freeze(["QL provider has no capabilities() operation."]) });
  let c;
  try { c=provider.capabilities(); } catch(error) { return Object.freeze({ state:"incompatible", capabilities:null, warnings:Object.freeze([`QL provider capability negotiation failed: ${error.message}`]) }); }
  if (!c || typeof c !== "object" || !c.provider || !Array.isArray(c.operations) || !Array.isArray(c.deterministicOperations) || !Array.isArray(c.stochasticOperations) || !Array.isArray(c.supportedForms) || !Array.isArray(c.supportedLenses) || !Array.isArray(c.extensionNamespaces)) return Object.freeze({ state:"incompatible", capabilities:c??null, warnings:Object.freeze(["QL provider capabilities are malformed."]) });
  if (c.state!=="ready" && c.state!=="degraded") return Object.freeze({ state:"incompatible", capabilities:c, warnings:Object.freeze([`Unsupported provider state: ${c.state}.`]) });
  const deterministic=new Set(c.deterministicOperations), stochastic=new Set(c.stochasticOperations);
  const overlaps=c.operations.filter((op)=>deterministic.has(op)&&stochastic.has(op));
  const unclassified=c.operations.filter((op)=>!deterministic.has(op)&&!stochastic.has(op));
  if (overlaps.length||unclassified.length) return Object.freeze({ state:"incompatible", capabilities:c, warnings:Object.freeze([`Provider operation classification is invalid: overlap=${overlaps.join(",")}; unclassified=${unclassified.join(",")}.`]) });
  const schemas=list(c.outputSchemaVersions);
  if (schemas.length>0 && !schemas.includes(QL_SCHEMA_VERSION)) return Object.freeze({ state:"incompatible", capabilities:c, warnings:Object.freeze([`Provider does not support output schema ${QL_SCHEMA_VERSION}.`]) });
  return Object.freeze({ state:c.state, capabilities:c, warnings:Object.freeze([...(c.warnings??[])]) });
}
