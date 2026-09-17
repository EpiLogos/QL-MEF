import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { STANDING_LEGEND, standingOf } from '../src/techne/timeline/standing.ts';
import { relationEdges } from '../src/techne/timeline/relations.ts';
import { loadFixtureReadings } from '../src/techne/fixtures/load-fixtures.mjs';

const specimen = JSON.parse(
  await readFile(new URL('../../../../../fixtures/techne/tb0-connective-base-v1.json', import.meta.url), 'utf8'),
);
const readings = await loadFixtureReadings();
const developmentDay = readings.find((reading) => reading.subject.subject_ref.startsWith('central:now:'));

test('a fact standing with evidence reads sourced', () => {
  const edge = relationEdges(specimen).find((candidate) => candidate.relation === 'implemented-in');
  assert.equal(edge.standing_visual, 'sourced');
  assert.equal(edge.standing_verbatim, 'fact');
  assert.ok(edge.evidence_refs.length > 0);
});

test('an interpretation stays interpreted even when it cites a source — the no-upgrade law', () => {
  const edge = relationEdges(specimen).find((candidate) => candidate.relation === 'INSTANTIATES');
  assert.equal(edge.standing_verbatim, 'interpretation');
  assert.ok(edge.source_ref, 'the interpretation does cite a source');
  assert.equal(edge.standing_visual, 'derived', 'but the visual class stays derived — a cited source does not upgrade it to fact');
});

test('an owner-supplied derivation ref reads derived, whatever the standing', () => {
  const reading = standingOf({ standing: 'recorded', source_ref: 'x:src', evidence_refs: [], derivation_ref: 'ql:derivation:1' });
  assert.equal(reading.visual, 'derived');
  assert.match(reading.rule, /derivation ref/);
});

test('disputed and mythic standings take their own classes before anything else', () => {
  assert.equal(standingOf({ standing: 'disputed', source_ref: 'x:src', evidence_refs: ['x:ev'] }).visual, 'disputed');
  assert.equal(standingOf({ standing: 'allegation', source_ref: 'x:src', evidence_refs: [] }).visual, 'disputed');
  assert.equal(standingOf({ standing: 'mythic' }).visual, 'mythic');
  assert.equal(standingOf({ standing: 'legendary tradition' }).visual, 'mythic');
});

test('an owner-asserted standing with no source reads authored', () => {
  const reading = standingOf({ standing: 'design-commitment' });
  assert.equal(reading.visual, 'authored');
  assert.equal(reading.verbatim, 'design-commitment');
});

test('nothing disclosed reads unavailable — an origin never upgrades standing', () => {
  const unknown = standingOf({ standing: null });
  assert.equal(unknown.visual, 'unavailable');
  assert.match(unknown.rule, /never guessed/);
  // The development-day fixture relations carry origin "observed"/"recorded"
  // with origin refs but no standing/source/evidence fields: standing stays
  // unknown and is shown as unknown.
  const openedUnder = relationEdges(developmentDay).find((edge) => edge.relation === 'opened-under');
  assert.equal(openedUnder.origin, 'observed', 'the origin rides verbatim');
  assert.equal(openedUnder.standing_visual, 'unavailable', 'observed origin is not read as sourced fact');
});

test('the legend covers every visual class — the encoding is never silent', () => {
  const visuals = STANDING_LEGEND.map((entry) => entry.visual).sort();
  assert.deepEqual(visuals, ['authored', 'derived', 'disputed', 'mythic', 'sourced', 'unavailable']);
});
