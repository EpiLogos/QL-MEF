import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  TECHNE_CONTRACT,
  instrumentMPrime,
  instrumentReading,
  validateReading,
  validateSession,
} from '../src/techne/contract.ts';
import { loadFixtureReadings } from '../src/techne/fixtures/load-fixtures.mjs';

// tests/ → cradle → surfaces → techne → adapters → repo root.
const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..', '..', '..', '..', '..');

async function loadTb0Specimen() {
  return JSON.parse(await readFile(resolve(repoRoot, 'fixtures/techne/tb0-connective-base-v1.json'), 'utf8'));
}

test('the TB0 rich specimen validates against the mirrored contract (the mechanical TB0-1 addition)', async () => {
  const specimen = await loadTb0Specimen();
  const checked = validateReading(specimen);
  assert.deepEqual(checked.errors, [], checked.errors.join('; '));
  assert.equal(checked.valid, true);
  // The TB0-1 facets the M0′ spine consumes actually ride.
  assert.equal(specimen.contract, TECHNE_CONTRACT);
  assert.ok(specimen.whole.relations.every((relation) => typeof relation.relation_ref === 'string'), 'every relation carries its stable identity');
  assert.ok(specimen.whole.relations.some((relation) => relation.standing === 'interpretation'), 'standing rides verbatim');
  assert.ok(specimen.agency.length >= 4, 'the situated-Agency role floor is disclosed');
});

test('the three T0 fixtures still validate unchanged — purely additive, nothing renamed', async () => {
  for (const reading of await loadFixtureReadings()) {
    const checked = validateReading(reading);
    assert.equal(checked.valid, true, `${reading.reading_ref}: ${checked.errors.join('; ')}`);
  }
});

test('instrument → application cut and M′ office mappings mirror the canonical binding', () => {
  assert.equal(instrumentReading('project'), '4:2-deep');
  assert.equal(instrumentReading('palace'), '4:2-deep');
  assert.equal(instrumentReading('expressions'), '3:3-conjugate');
  assert.equal(instrumentMPrime('project'), 0);
  assert.equal(instrumentMPrime('canvas'), 1);
  assert.equal(instrumentMPrime('timeline'), 2);
  assert.equal(instrumentMPrime('journey'), 3);
  assert.equal(instrumentMPrime('place'), 4);
  assert.equal(instrumentMPrime('palace'), 5);
  assert.equal(instrumentMPrime('expressions'), null);
});

test('the situated-Agency role law is enforced: readings that break it are refused', async () => {
  const base = await loadTb0Specimen();
  const withRole = (role) => validateReading({ ...base, agency: [...base.agency, role] });

  // Anima inhabits the 3:3 conjugate reading and operates only Expressions.
  assert.match(withRole({ role: 'anima', m_index: 4, reading: '4:2-deep' }).errors.join(' '), /Anima_i inhabits the 3:3/);
  assert.match(withRole({ role: 'anima', m_index: 4, instrument: 'project' }).errors.join(' '), /Anima_i operates the Expression reading/);
  // Aletheia/Technē inhabit the 4:2 deep reading.
  assert.match(withRole({ role: 'aletheia', m_index: 0, reading: '3:3-conjugate' }).errors.join(' '), /inhabit the 4:2 deep reading/);
  // Technē operates its own coordinate's deep instrument.
  assert.match(withRole({ role: 'techne', m_index: 0, reading: '4:2-deep', instrument: 'timeline' }).errors.join(' '), /Technē_0 operates its own coordinate's deep instrument/);
  // The field has M′0–M′5 only.
  assert.match(withRole({ role: 'guardian', m_index: 6 }).errors.join(' '), /M′0–M′5 only/);
  // A lawful Technē_0 binding on the deep project instrument passes.
  const lawful = withRole({ role: 'techne', m_index: 0, reading: '4:2-deep', instrument: 'project', guardian_ref: 'actuation:agent:anuttara' });
  assert.equal(lawful.valid, true, lawful.errors.join('; '));
});

test('cut-level disclosure carries the unavailable-reason law', async () => {
  const base = await loadTb0Specimen();
  const withCuts = (cuts) => validateReading({ ...base, disclosure: { ...base.disclosure, application_cuts: cuts } });
  assert.match(withCuts([{ cut: '3:3-conjugate', available: false }]).errors.join(' '), /unavailable application cut requires a reason/);
  assert.match(withCuts([{ cut: '4:6-deep', available: true }]).errors.join(' '), /must be "4:2-deep" or "3:3-conjugate"/);
  assert.equal(withCuts([{ cut: '3:3-conjugate', available: false, reason: 'no Expression binding is disclosed for this subject' }]).valid, true);
});

test('the session carries the TB0 dual-reading state and enforces cut/instrument agreement', async () => {
  const selection = {
    selection_ref: 'ql.techne:selection:s1',
    subject_ref: 'central:source:control:root:Work/Quaternal-Logic/docs/L5-TECHNE-DUAL-READING-LOCK.md',
    reading_ref: 'ql.techne:reading:fixture:tb0@1',
    instrument: 'project',
  };
  const session = (overrides = {}) => ({
    contract: TECHNE_CONTRACT,
    session_ref: 'techne:disclosure-session:test',
    subject_ref: selection.subject_ref,
    selection,
    instrument: 'project',
    reading_ref: selection.reading_ref,
    ...overrides,
  });

  const deep = session({
    application_cut: '4:2-deep',
    whole_ref: 'ql.techne:whole:fixture:l5-techne-contract-ground',
    world_ref: 'central:world:control:root',
    occasion_ref: null,
    return_target_ref: 'aikit:wiki:staging:return-target',
  });
  assert.equal(validateSession(deep).valid, true);

  // Crossing to the conjugate reading rides the expressions instrument.
  const crossed = { ...deep, instrument: 'expressions', application_cut: '3:3-conjugate', scene_focus_ref: 'oi:expression:l5-dual-reading/scene/crossing' };
  assert.equal(validateSession(crossed).valid, true);

  // A cut that disagrees with the instrument is a bug, refused.
  assert.match(validateSession({ ...deep, instrument: 'expressions' }).errors.join(' '), /inconsistent with instrument/);
  assert.match(validateSession({ ...deep, application_cut: '3:3-conjugate' }).errors.join(' '), /inconsistent with instrument/);
});
