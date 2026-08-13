import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

import {
  MEF_LENS_REGISTRY,
  QL_FORM_REGISTRY,
  QL_POSITIONS,
  QLValidationError,
  applyDeterministicOperator,
  canonicalLensRef,
  canonicalQLAddress,
  complementAddress,
  conjugateAddress,
  createFixtureQLProvider,
  formatQLAddress,
  parseQLAddress
} from "../src/index.js";

const cases = JSON.parse(await readFile(new URL("../fixtures/q1/cases.json", import.meta.url), "utf8"));
const schemas = JSON.parse(await readFile(new URL("../schemas/v1/contracts.schema.json", import.meta.url), "utf8"));

test("Q1 form registry is versioned and contains only the ratified minimal forms", () => {
  assert.deepEqual(QL_FORM_REGISTRY.map(({ id, version }) => `${id}@${version}`), [
    "sixfold@1",
    "four-plus-two@1",
    "direct-conjugate@1"
  ]);
});

test("QLAddress schema constrains the address frame to sixfold@1", () => {
  const frame = schemas.$defs.QLAddress.properties.frame;
  assert.equal(frame.properties.id.const, "sixfold");
  assert.equal(frame.properties.version.const, 1);
});

test("all twelve canonical MEF LensRefs are addressable without claiming Q2 sublens semantics", () => {
  assert.equal(MEF_LENS_REGISTRY.length, 12);
  for (const lens of MEF_LENS_REGISTRY) {
    assert.deepEqual(canonicalLensRef({ id: lens.id, version: 1 }), { id: lens.id, version: 1 });
  }
});

test("canonical QLAddress format round-trips for every position, face and representative depth", () => {
  for (const position of QL_POSITIONS) {
    for (const face of ["direct", "conjugate"]) {
      for (const depth of [0, 1, 3]) {
        const input = { position, face, depth };
        const canonical = canonicalQLAddress(input);
        assert.deepEqual(parseQLAddress(formatQLAddress(canonical)), canonical);
      }
    }
  }
});

test("conjugation is deterministic, preserves raw position, and is an involution", () => {
  for (const position of QL_POSITIONS) {
    const initial = canonicalQLAddress({ position, face: "direct", depth: 2 });
    const once = conjugateAddress(initial);
    const twice = conjugateAddress(once.value);
    assert.equal(once.value.position, position);
    assert.equal(once.value.face, "conjugate");
    assert.deepEqual(twice.value, initial);
    assert.equal(once.provenance.mode, "deterministic");
  }
});

test("complement is the canonical sum-to-five pairing and an involution", () => {
  for (const position of QL_POSITIONS) {
    const initial = canonicalQLAddress({ position, face: "direct", depth: 0 });
    const once = complementAddress(initial);
    const twice = complementAddress(once.value);
    assert.equal(position + once.value.position, 5);
    assert.deepEqual(twice.value, initial);
  }
});

test("4+2 classification is exact: P0/P5 implicate, P1..P4 explicate", () => {
  const expected = new Map([[0, "implicate"], [1, "explicate"], [2, "explicate"], [3, "explicate"], [4, "explicate"], [5, "implicate"]]);
  for (const [position, classification] of expected) {
    assert.equal(applyDeterministicOperator("classify-four-plus-two", { position }).value, classification);
  }
});

test("valid fixture addresses canonicalise deterministically", () => {
  for (const fixture of cases.validAddresses) {
    const one = canonicalQLAddress(fixture);
    const two = canonicalQLAddress(one);
    assert.deepEqual(two, one);
  }
});

test("negative fixtures fail visibly and client product nouns cannot masquerade as QL positions", () => {
  for (const fixture of cases.invalidAddresses) {
    assert.throws(() => canonicalQLAddress(fixture.value), (error) => error instanceof QLValidationError && typeof error.code === "string", fixture.case);
  }
});

test("operator fixture corpus replays identically", () => {
  for (const fixture of cases.operators) {
    const first = applyDeterministicOperator(fixture.operator, fixture.input);
    const second = applyDeterministicOperator(fixture.operator, fixture.input);
    assert.deepEqual(second, first);
    if (typeof fixture.expected === "string") {
      assert.equal(first.value, fixture.expected);
    } else {
      assert.equal(first.value.position, fixture.expected.position);
      assert.equal(first.value.face, fixture.expected.face);
      assert.equal(first.value.depth, fixture.expected.depth);
    }
  }
});

test("fixture provider advertises no stochastic or deep-research capability", () => {
  const provider = createFixtureQLProvider();
  const capabilities = provider.capabilities();
  assert.deepEqual(capabilities.stochasticOperations, []);
  assert.deepEqual(capabilities.extensionNamespaces, []);
  assert.ok(capabilities.deterministicOperations.includes("conjugate-address"));
  assert.ok(capabilities.deterministicOperations.includes("complement-address"));
  assert.ok(!capabilities.operations.includes("refract"));
  assert.ok(!capabilities.operations.includes("conjugation-runtime"));
});

test("unsupported operators fail explicitly rather than filling symmetry", () => {
  assert.throws(
    () => applyDeterministicOperator("harmonic-64-state", { position: 0 }),
    (error) => error.code === "UNSUPPORTED_OPERATOR"
  );
});
