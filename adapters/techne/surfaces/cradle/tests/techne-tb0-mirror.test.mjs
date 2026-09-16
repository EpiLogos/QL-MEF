import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { applicationCutFor, validateReading, validateSession } from '../src/techne/contract.ts';
import { createDisclosureSessionStore } from '../src/techne/session.ts';
import { loadFixtureReadings } from '../src/techne/fixtures/load-fixtures.mjs';

// The pinned TB0 specimen, read from the canonical root fixture.
const specimen = JSON.parse(
  await readFile(new URL('../../../../../fixtures/techne/tb0-connective-base-v1.json', import.meta.url), 'utf8'),
);

test('the cradle mirror accepts the pinned TB0-1 specimen unmodified', () => {
  const checked = validateReading(specimen);
  assert.equal(checked.valid, true, checked.errors.join('; '));
  assert.equal(specimen.agency.length, 4, 'the role floor arrived with the specimen');
  assert.equal(specimen.disclosure.application_cuts.length, 2);
});

test('the T0 fixtures still validate against the adopted mirror — additive, never breaking', async () => {
  for (const reading of await loadFixtureReadings()) {
    const checked = validateReading(reading);
    assert.equal(checked.valid, true, `${reading.reading_ref}: ${checked.errors.join('; ')}`);
  }
});

test('the mirror stays strict: unknown fields and bad agency bindings are still refused', () => {
  const snooping = { ...structuredClone(specimen), layout: 'this is presentation state' };
  assert.equal(validateReading(snooping).valid, false, 'view state remains inexpressible');
  const badRole = {
    ...structuredClone(specimen),
    agency: [{ role: 'emperor', m_index: 2 }],
  };
  const checked = validateReading(badRole);
  assert.equal(checked.valid, false);
  assert.ok(checked.errors.some((error) => error.includes('role')), 'an unknown role is refused');
  const badIndex = {
    ...structuredClone(specimen),
    agency: [{ role: 'techne', m_index: 7 }],
  };
  assert.ok(!validateReading(badIndex).valid, 'm_index stays bounded 0-5 — the field has M′0–M′5 only');
});

test('the TB0 cut law: the application cut agrees with the instrument, in type and validator', () => {
  assert.equal(applicationCutFor('timeline'), '4:2-deep');
  assert.equal(applicationCutFor('canvas'), '4:2-deep');
  assert.equal(applicationCutFor('expressions'), '3:3-conjugate');

  const base = {
    contract: 'ql.techne/v1',
    session_ref: 'techne:disclosure-session:test',
    subject_ref: 'central:source:control:root:x',
    selection: {
      selection_ref: 'ql.techne:selection:x',
      subject_ref: 'central:source:control:root:x',
      reading_ref: 'ql.techne:reading:x',
      instrument: 'timeline',
    },
    instrument: 'timeline',
    reading_ref: 'ql.techne:reading:x',
  };
  assert.equal(validateSession({ ...base, application_cut: '4:2-deep' }).valid, true);
  const mismatch = validateSession({ ...base, application_cut: '3:3-conjugate' });
  assert.equal(mismatch.valid, false, 'timeline cannot carry the conjugate cut');
  assert.ok(mismatch.errors.some((error) => error.includes('application_cut')));
});

test('crossing instruments through the store flips the cut and keeps every identity ref byte-exact', () => {
  const store = createDisclosureSessionStore();
  const opened = store.setSelection({
    selection_ref: 'ql.techne:selection:cut',
    subject_ref: 'central:source:control:root:x',
    reading_ref: 'ql.techne:reading:x',
    source_ref: 'central:source:control:root:x-src',
    instrument: 'timeline',
  });
  assert.equal(opened.application_cut, '4:2-deep');

  const crossed = store.openInInstrument('expressions');
  assert.equal(crossed.application_cut, '3:3-conjugate', 'the M2 → 3:3 crossing flips the cut');
  assert.equal(crossed.session_ref, opened.session_ref, 'the one session continues');
  assert.equal(crossed.subject_ref, opened.subject_ref, 'subject identity survives the cut');
  assert.equal(crossed.reading_ref, opened.reading_ref, 'reading basis survives the cut');
  assert.equal(crossed.selection.source_ref, opened.selection.source_ref, 'source basis survives the cut');
  assert.equal(crossed.selection.selection_ref, opened.selection.selection_ref, 'the selection is co-referenced, not re-minted');
  assert.deepEqual(crossed.selection.focus_refs, opened.selection.focus_refs);
  assert.deepEqual(crossed.navigation, [
    { from_instrument: 'timeline', to_instrument: 'expressions', selection_ref: 'ql.techne:selection:cut' },
  ]);

  const back = store.openInInstrument('timeline');
  assert.equal(back.application_cut, '4:2-deep', 'the return crossing restores the deep cut');
  assert.equal(back.navigation.length, 2, 'hops are recorded');
});
