import { canonicalQLAddress } from "./address.js";
import { applyDeterministicOperator, QL_OPERATOR_REGISTRY } from "./operators.js";
import {
  MEF_LENS_REGISTRY,
  MEF_LENS_REGISTRY_VERSION,
  QL_FORM_REGISTRY,
  QL_FORM_REGISTRY_VERSION,
  QL_KERNEL_VERSION,
  QL_SCHEMA_VERSION
} from "./registry.js";

export const FIXTURE_PROVIDER_REF = Object.freeze({ id: "fixture-q1", version: QL_KERNEL_VERSION });

export function fixtureProviderCapabilities() {
  return Object.freeze({
    provider: FIXTURE_PROVIDER_REF,
    schemaVersion: QL_SCHEMA_VERSION,
    kernelVersion: QL_KERNEL_VERSION,
    formRegistryVersion: QL_FORM_REGISTRY_VERSION,
    lensRegistryVersion: MEF_LENS_REGISTRY_VERSION,
    supportedForms: QL_FORM_REGISTRY.map(({ id, version }) => Object.freeze({ id, version })),
    supportedLenses: MEF_LENS_REGISTRY.map(({ id, version }) => Object.freeze({ id, version })),
    operations: Object.freeze(["validate-address", ...QL_OPERATOR_REGISTRY]),
    deterministicOperations: Object.freeze(["validate-address", ...QL_OPERATOR_REGISTRY]),
    stochasticOperations: Object.freeze([]),
    extensionNamespaces: Object.freeze([])
  });
}

export function createFixtureQLProvider() {
  return Object.freeze({
    capabilities: fixtureProviderCapabilities,
    validateAddress: canonicalQLAddress,
    apply: applyDeterministicOperator
  });
}
