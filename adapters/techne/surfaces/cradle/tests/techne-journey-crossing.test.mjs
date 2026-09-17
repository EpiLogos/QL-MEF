import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { beats } from '../src/techne/journey/beats.ts';
import {
  crossCutGuard,
  crossToExpression,
  cutFor,
  returnPosition,
  sceneFocusOf,
  selectionWithSceneFocus,
} from '../src/techne/journey/crossing.ts';
import {
  PROVING_SCENE_CROSSING,
  PROVING_SCENE_GROUND,
  PROVING_SCENE_RETURN,
  PROVING_SUBJECT_REF,
  provingReading,
} from '../src/techne/journey/proving-reading.ts';
import { createDisclosureSessionStore } from '../src/techne/session.ts';

const reading = provingReading();
const model = beats(reading);

/** A disclosure session standing on the journey instrument over the proving
 * reading — the 4:2 side of the bridge. */
function journeySession() {
  const store = createDisclosureSessionStore();
  store.setSelection({
    selection_ref: 'techne:test-selection:journey-crossing',
    subject_ref: PROVING_SUBJECT_REF,
    reading_ref: reading.reading_ref,
    instrument: 'journey',
  });
  store.openInInstrument('journey');
  return store;
}

test('the application cut is derived from the instrument, never stored (TB0-1 law)', () => {
  assert.equal(cutFor('journey'), '4:2-deep');
  assert.equal(cutFor('expressions'), '3:3-conjugate');
  assert.equal(cutFor('project'), '4:2-deep');
});

test('a crossing onto the occupied cut is refused; the M3 bridge crosses 4:2 ↔ 3:3', () => {
  assert.throws(() => crossCutGuard('journey', 'journey'), /already carries/);
  assert.throws(() => crossCutGuard('project', 'canvas'), /stays inside the 4:2-deep cut/);
  assert.doesNotThrow(() => crossCutGuard('journey', 'expressions'), 'journey → expressions is the bridge');
  assert.doesNotThrow(() => crossCutGuard('expressions', 'journey'), 'and so is the return');
});

test('the scene focus rides the selection focus_refs verbatim, appended exactly once', () => {
  const selection = {
    selection_ref: 'techne:test-selection:journey-crossing',
    subject_ref: PROVING_SUBJECT_REF,
    reading_ref: reading.reading_ref,
    instrument: 'journey',
    focus_refs: ['central:focus:some-member'],
  };
  const carried = selectionWithSceneFocus(selection, PROVING_SCENE_CROSSING);
  assert.deepEqual(carried.focus_refs, ['central:focus:some-member', PROVING_SCENE_CROSSING]);
  assert.equal(carried.selection_ref, selection.selection_ref, 'everything else byte-identical');
  const again = selectionWithSceneFocus(carried, PROVING_SCENE_CROSSING);
  assert.equal(again, carried, 'a crossing that changes nothing rebuilds nothing');
  assert.throws(() => selectionWithSceneFocus(selection, '  '), /empty/);
});

test('crossToExpression carries the exact scene ref and every protected identity across', () => {
  const store = journeySession();
  const session = store.get();
  const before = structuredClone(session);
  const crossing = crossToExpression(session, PROVING_SCENE_CROSSING);
  assert.equal(crossing.target, 'expressions');
  assert.deepEqual(crossing.selection.focus_refs, [PROVING_SCENE_CROSSING]);
  assert.deepEqual(session, before, 'the crossing function mutates nothing');
});

test('the full 4:2 → 3:3 crossing through the session store: subject, basis and agent ref unchanged, scene ref aboard', () => {
  const store = journeySession();
  const session = store.get();
  const crossing = crossToExpression(session, PROVING_SCENE_CROSSING);
  store.setSelection(crossing.selection);
  const crossed = store.openInInstrument(crossing.target);
  assert.equal(crossed.instrument, 'expressions');
  assert.equal(crossed.subject_ref, session.subject_ref);
  assert.equal(crossed.reading_ref, session.reading_ref);
  assert.equal(crossed.selection.selection_ref, session.selection.selection_ref);
  assert.equal(crossed.session_ref, session.session_ref);
  assert.deepEqual(crossed.selection.focus_refs, [PROVING_SCENE_CROSSING], 'the exact scene ref crossed');
  assert.deepEqual(
    crossed.navigation.at(-1),
    { from_instrument: 'journey', to_instrument: 'expressions', selection_ref: session.selection.selection_ref },
    'the hop is recorded in the session navigation',
  );
});

test('Return restores the exact beat by scene-ref equality — never an index guess', () => {
  const store = journeySession();
  const session = store.get();
  const crossing = crossToExpression(session, PROVING_SCENE_RETURN);
  store.setSelection(crossing.selection);
  store.openInInstrument(crossing.target);
  const back = store.openInInstrument('journey');
  const home = returnPosition(back, model.beats);
  assert.equal(home.focus_ref, PROVING_SCENE_RETURN);
  assert.equal(home.beat?.scene_ref, PROVING_SCENE_RETURN);
  assert.equal(home.beat?.title, 'return');
});

test('a returning focus the reading no longer discloses is an honest miss, not a guessed position', () => {
  const store = journeySession();
  const session = store.get();
  const crossing = crossToExpression(session, PROVING_SCENE_GROUND);
  store.setSelection({ ...crossing.selection, focus_refs: ['oi:expression:l5-dual-reading/scene/revoked'] });
  store.openInInstrument(crossing.target);
  const back = store.openInInstrument('journey');
  const home = returnPosition(back, model.beats);
  assert.equal(home.beat, null);
  assert.equal(home.focus_ref, null, 'a focus no disclosed scene carries is not reported as a position');
  assert.equal(sceneFocusOf(back.selection, model.beats), null);
});

test('the proving reading carries the three scenes in binding order', () => {
  assert.deepEqual(model.beats.map((beat) => beat.scene_ref), [
    PROVING_SCENE_GROUND,
    PROVING_SCENE_CROSSING,
    PROVING_SCENE_RETURN,
  ]);
});

test('the proving reading is grounded in the canonical TB0 specimen — every owner ref appears verbatim in the pinned fixture', async () => {
  const specimenPath = fileURLToPath(new URL('../../../../../fixtures/techne/tb0-connective-base-v1.json', import.meta.url));
  const specimen = await readFile(specimenPath, 'utf8');
  const collected = [];
  const push = (value) => { if (typeof value === 'string' && value.trim()) collected.push(value); };
  push(reading.subject.subject_ref);
  reading.subject.readings?.forEach((entry) => push(entry.ref));
  push(reading.whole.whole_ref);
  reading.whole.member_refs?.forEach(push);
  reading.whole.relations?.forEach((relation) => { push(relation.from_ref); push(relation.to_ref); push(relation.origin_ref); });
  reading.whole.focus_refs?.forEach(push);
  const ql = reading.ql;
  for (const key of ['shape_ref', 'constellation_ref', 'lens_ref', 'context_frame_ref', 'vak_source_ref', 'provenance_ref']) push(ql[key]);
  ql.derivation_refs?.forEach(push);
  ql.warrant.evidence_refs.forEach(push);
  reading.temporal?.forEach((facet) => { push(facet.facet_ref); push(facet.day_ref); push(facet.now_ref); push(facet.session_ref); push(facet.run_ref); push(facet.source_ref); push(facet.timezone_policy_ref); });
  reading.spatial?.forEach((place) => { push(place.place_ref); push(place.source_ref); place.hierarchy?.forEach((entry) => push(entry.place_ref)); });
  reading.provenance?.forEach((entry) => push(entry.source_ref));
  reading.expressions?.forEach((binding) => { push(binding.expression_ref); push(binding.composition_ref); push(binding.profile_ref); });
  push(PROVING_SCENE_CROSSING);
  reading.actions?.forEach((action) => { push(action.action_ref); push(action.native_owner); });
  const missing = collected.filter((ref) => !ref.startsWith('ql.techne:reading:') && !ref.includes('/scene/ground') && !ref.includes('/scene/return') && !specimen.includes(ref));
  assert.deepEqual(missing, [], `refs not grounded in the TB0 specimen: ${missing.join(', ')}`);
});
