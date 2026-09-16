import test from 'node:test';
import assert from 'node:assert/strict';
import {
  crossCutSession,
  disclosedCut,
  instrumentReading,
  mPrimeOf,
  validateReading,
  validateSession,
} from '../src/techne/contract.ts';
import { loadFixture, loadFixtureReadings } from '../src/techne/fixtures/load-fixtures.mjs';

const tb0 = await loadFixture('tb0-connective-base-v1.json');
const t0Readings = await loadFixtureReadings();

const sessionOn = (instrument, overrides = {}) => ({
  contract: 'ql.techne/v1',
  session_ref: 'techne:disclosure-session:tb0-test',
  subject_ref: tb0.subject.subject_ref,
  selection: {
    selection_ref: 'ql.techne:selection:tb0-test',
    subject_ref: tb0.subject.subject_ref,
    reading_ref: tb0.reading_ref,
    source_ref: tb0.provenance[0].source_ref,
    source_revision: tb0.provenance[0].source_revision ?? null,
    instrument,
    agent_session_ref: 'aikit:agent-session:fixture:l5-techne',
    ...overrides,
  },
  instrument,
  application_cut: instrumentReading(instrument),
  occasion_ref: 'central:now:control:root:b417a7c3d37cd47c2762deee3224687d59db1a9015a322e9a8d7584b3947adf6',
  return_target_ref: 'central:source:control:root:Control/user/day/2026-09-16/day.md',
  reading_ref: tb0.reading_ref,
  navigation: [],
});

test('the TB0-1 rich specimen validates against the cradle TS mirror', () => {
  const checked = validateReading(tb0);
  assert.deepEqual(checked, { valid: true, errors: [] }, JSON.stringify(checked.errors));
  // Cut-level disclosure: both readings available, with reasons law intact.
  assert.equal(tb0.disclosure.application_cuts.length, 2);
  assert.deepEqual(disclosedCut(tb0.disclosure, '3:3-conjugate'), { cut: '3:3-conjugate', available: true });
  // The situated-Agency role floor: guardian, anima, aletheia, techne.
  assert.deepEqual(tb0.agency.map((role) => role.role), ['guardian', 'anima', 'aletheia', 'techne']);
  // The anima binding inhabits the 3:3 reading and operates Expressions.
  const anima = tb0.agency.find((role) => role.role === 'anima');
  assert.equal(anima.reading, '3:3-conjugate');
  assert.equal(anima.instrument, 'expressions');
  assert.equal(anima.guardian_ref, 'actuation:agent:nara');
  // The techne binding operates its own coordinate's deep instrument.
  const techne = tb0.agency.find((role) => role.role === 'techne');
  assert.equal(techne.m_index, 2);
  assert.equal(techne.instrument, 'timeline');
  // The Guardian stewardship spans both readings and is not bound to one.
  assert.equal(tb0.agency.find((role) => role.role === 'guardian').reading, undefined);
});

test('the TB0-1 additions are purely additive: every T0 fixture validates unchanged', () => {
  for (const reading of t0Readings) {
    const checked = validateReading(reading);
    assert.deepEqual(checked, { valid: true, errors: [] }, JSON.stringify(checked.errors));
  }
});

test('the TB0-1 fixture carries the honest-facet fields the pin describes', () => {
  // Relation standing and deliberate trans-temporality (no manufactured date).
  const instantiates = tb0.whole.relations.find((relation) => relation.relation === 'INSTANTIATES');
  assert.equal(instantiates.standing, 'interpretation');
  assert.ok(instantiates.temporal_facet_ref == null, 'trans-temporal relations carry no temporal qualification');
  const implemented = tb0.whole.relations.find((relation) => relation.relation === 'implemented-in');
  assert.equal(implemented.standing, 'fact');
  assert.ok(implemented.temporal_facet_ref, 'dated relations resolve a real temporal facet');
  // Place relation types are preserved verbatim and never interchangeable.
  const greenwich = tb0.spatial.find((place) => place.relation === 'OCCURRED_AT');
  const avalon = tb0.spatial.find((place) => place.relation === 'MYTH_LOCATED_AT');
  assert.equal(greenwich.precision, 'approximate');
  assert.ok(greenwich.uncertainty);
  assert.equal(avalon.precision, 'unlocated');
  assert.ok(avalon.geometry == null, 'a mythic place is truthfully unlocated — no geometry is invented');
  // Attempt/Return continuity ride real facets: the attempt on the run, the
  // late Return on the session facet it returns through.
  const run = tb0.temporal.find((facet) => facet.kind === 'run');
  assert.ok(run.attempt_ref, 'attempt continuity is carried');
  const carried = tb0.temporal.find((facet) => facet.kind === 'session');
  assert.equal(carried.return_ref, `${run.run_ref}/return`, 'the Return resolves against the same run');
  // The warranted reading names its own M coordinate and Return.
  assert.equal(tb0.ql.m_coordinate_ref, 'ql:structural:5.0.0');
  assert.ok(tb0.ql.return_ref);
});

test('an unavailable application cut without a reason is refused', () => {
  const mutated = structuredClone(tb0);
  mutated.disclosure.application_cuts[1].available = false;
  delete mutated.disclosure.application_cuts[1].reason;
  const checked = validateReading(mutated);
  assert.equal(checked.valid, false);
  assert.ok(checked.errors.some((error) => error.includes('unavailable application cut requires a reason')));
});

// --- the situated-Agency binding law, executable ---------------------------

test('Anima_i inhabits the 3:3 reading and operates only the Expression reading', () => {
  assert.equal(validateReading({ ...tb0, agency: [{ role: 'anima', m_index: 4, reading: '4:2-deep' }] }).valid, false);
  assert.ok(validateReading({ ...tb0, agency: [{ role: 'anima', m_index: 4, reading: '4:2-deep' }] }).errors[0].includes('3:3'));
  assert.equal(validateReading({ ...tb0, agency: [{ role: 'anima', m_index: 4, instrument: 'timeline' }] }).valid, false);
  assert.equal(validateReading({ ...tb0, agency: [{ role: 'anima', m_index: 4, reading: '3:3-conjugate', instrument: 'expressions' }] }).valid, true);
});

test('Aletheia_i/Technē_i inhabit the 4:2 reading; Technē_i operates its own coordinate', () => {
  assert.equal(validateReading({ ...tb0, agency: [{ role: 'aletheia', m_index: 2, reading: '3:3-conjugate' }] }).valid, false);
  assert.equal(validateReading({ ...tb0, agency: [{ role: 'techne', m_index: 2, reading: '3:3-conjugate' }] }).valid, false);
  // Technē_2 operating another coordinate's instrument is refused…
  assert.equal(validateReading({ ...tb0, agency: [{ role: 'techne', m_index: 2, reading: '4:2-deep', instrument: 'journey' }] }).valid, false);
  // …including a deep-instrument mismatch of index, and the conjugate aperture.
  assert.equal(validateReading({ ...tb0, agency: [{ role: 'techne', m_index: 3, reading: '4:2-deep', instrument: 'journey' }] }).valid, true);
  assert.equal(validateReading({ ...tb0, agency: [{ role: 'techne', m_index: 2, instrument: 'expressions' }] }).valid, false);
});

test('no role binding becomes a Guardian replacement, and m_index stays on the field', () => {
  assert.equal(validateReading({ ...tb0, agency: [{ role: 'guardian', m_index: 5, guardian_ref: 'actuation:agent:epii' }] }).valid, true);
  assert.equal(validateReading({ ...tb0, agency: [{ role: 'techne', m_index: 7 }] }).valid, false);
});

// --- the dual-reading crossing (the canonical cross_cut, as a pure function)

test('crossCutSession moves the cut and carries every identity ref byte-exact', () => {
  const before = sessionOn('timeline');
  const { session: crossed, cut } = crossCutSession(before, 'expressions');
  assert.equal(cut, '3:3-conjugate');
  assert.equal(crossed.instrument, 'expressions');
  assert.equal(crossed.application_cut, '3:3-conjugate');
  // Cross-cut identity law: subject, basis, sources, occasion, Return target.
  assert.equal(crossed.subject_ref, before.subject_ref);
  assert.equal(crossed.reading_ref, before.reading_ref);
  assert.equal(crossed.selection.selection_ref, before.selection.selection_ref);
  assert.equal(crossed.selection.source_ref, before.selection.source_ref);
  assert.equal(crossed.selection.agent_session_ref, before.selection.agent_session_ref);
  assert.equal(crossed.occasion_ref, before.occasion_ref);
  assert.equal(crossed.return_target_ref, before.return_target_ref);
  // The co-referenced pair and the same native Actions on both sides.
  assert.ok(crossed.selection.subject_ref === before.selection.subject_ref);
  assert.equal(crossed.navigation.length, before.navigation.length + 1);
  assert.deepEqual(crossed.navigation.at(-1), {
    from_instrument: 'timeline',
    to_instrument: 'expressions',
    selection_ref: before.selection.selection_ref,
  });
  assert.equal(validateSession(crossed).valid, true);
});

test('crossCutSession refuses a crossing onto the occupied cut, and drifted sessions', () => {
  assert.throws(() => crossCutSession(sessionOn('timeline'), 'canvas'), /other application cut/);
  assert.throws(() => crossCutSession(sessionOn('expressions'), 'expressions'), /other application cut/);
  const drifted = sessionOn('timeline');
  drifted.application_cut = '3:3-conjugate'; // inconsistent with the instrument
  assert.throws(() => crossCutSession(drifted, 'expressions'), /drifted/);
});

test('the reading-kind helpers mirror the canonical binding exactly', () => {
  assert.deepEqual(['project', 'canvas', 'timeline', 'journey', 'place', 'palace'].map(instrumentReading), Array(6).fill('4:2-deep'));
  assert.equal(instrumentReading('expressions'), '3:3-conjugate');
  assert.deepEqual(
    ['project', 'canvas', 'timeline', 'journey', 'place', 'palace', 'expressions'].map(mPrimeOf),
    [0, 1, 2, 3, 4, 5, null],
  );
});
