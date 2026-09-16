/**
 * Fixture provenance: these JSON files are byte-exact copies of the
 * canonical conformance fixtures at QL-MEF `fixtures/techne/`
 * (representative-subject-v1.json, development-day-v1.json,
 * absent-facets-v1.json); drift is a contract bug — re-copy, never edit.
 *
 * The directory also carries `tb0-connective-base-v1.json`, a byte-exact
 * copy of the TB0-1 shared specimen (QL-MEF `fixtures/techne/
 * tb0-connective-base-v1.json`, issue #212). It is NOT part of the
 * three-fixture T0 conformance set below (FIXTURE_FILES stays the T0 trio);
 * lanes load it by name with `loadFixture("tb0-connective-base-v1.json")`.
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
];

const here = fileURLToPath(new URL(".", import.meta.url));

export async function loadFixture(name) {
  return JSON.parse(await readFile(`${here}${name}`, "utf8"));
}

export async function loadFixtureReadings() {
  return Promise.all(FIXTURE_FILES.map(loadFixture));
}
