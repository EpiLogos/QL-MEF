/**
 * Fixture provenance: these JSON files are byte-exact copies of the canonical
 * conformance fixtures at QL-MEF `fixtures/techne/`
 * (representative-subject-v1.json, development-day-v1.json,
 * absent-facets-v1.json, tb0-connective-base-v1.json); drift is a contract
 * bug — re-copy, never edit. The three T0 conformance fixtures load through
 * `loadFixtureReadings()`; the TB0-1 rich specimen loads by name through
 * `loadFixture("tb0-connective-base-v1.json")`.
 *
 * Node-side loader for tests and conformance runs (`node --test` cannot
 * import JSON without import attributes, and the renderer bundle never
 * ships fixtures). Returns validated ql.techne/v1 readings.
 */
import { readFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";

export const FIXTURE_FILES = [
  "representative-subject-v1.json",
  "development-day-v1.json",
  "absent-facets-v1.json",
  "tb0-connective-base-v1.json",
];

const here = fileURLToPath(new URL(".", import.meta.url));

export async function loadFixture(name) {
  return JSON.parse(await readFile(`${here}${name}`, "utf8"));
}

export async function loadFixtureReadings() {
  return Promise.all(FIXTURE_FILES.map(loadFixture));
}
