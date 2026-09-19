import test from 'node:test';
import assert from 'node:assert/strict';
import {
  JOURNEY_PRESENCE_KEY,
  JOURNEY_PRESENCE_SCHEMA,
  basisStanding,
  loadPresence,
  presenceFor,
  savePresence,
} from '../src/techne/journey/presence.ts';
import { beats } from '../src/techne/journey/beats.ts';
import {
  PROVING_EXPRESSION,
  PROVING_SCENE_CROSSING,
  PROVING_SCENE_GROUND,
  PROVING_SUBJECT_REF,
  provingReading,
} from '../src/techne/journey/proving-reading.ts';

const reading = provingReading();

/** A Map-backed storage standing in for localStorage. */
function fakeStorage(initial) {
  const map = new Map(Object.entries(initial ?? {}));
  return {
    getItem: (key) => (map.has(key) ? map.get(key) : null),
    setItem: (key, value) => map.set(key, String(value)),
    dump: () => Object.fromEntries(map),
  };
}

function savedPresence() {
  return presenceFor({
    subject_ref: PROVING_SUBJECT_REF,
    reading_ref: reading.reading_ref,
    snapshot_revision: reading.snapshot?.revision ?? null,
    expression_ref: PROVING_EXPRESSION,
    scene_ref: PROVING_SCENE_CROSSING,
    draft: {
      scene_order: [PROVING_SCENE_CROSSING, PROVING_SCENE_GROUND],
      pace: [{ scene_ref: PROVING_SCENE_GROUND, dwell_seconds: 7 }],
    },
  });
}

test('presence round-trips through storage: position and draft survive reload/re-entry', () => {
  const storage = fakeStorage();
  const presence = savedPresence();
  savePresence(storage, { [PROVING_SUBJECT_REF]: presence });
  const restored = loadPresence(storage);
  assert.deepEqual(restored[PROVING_SUBJECT_REF], presence);
  assert.equal(restored[PROVING_SUBJECT_REF].scene_ref, PROVING_SCENE_CROSSING, 'the exact position returns');
  assert.deepEqual(restored[PROVING_SUBJECT_REF].draft.scene_order, [PROVING_SCENE_CROSSING, PROVING_SCENE_GROUND]);
});

test('presence carries presentation state only — refs and values, never a semantic object', () => {
  const presence = savedPresence();
  const serialised = JSON.stringify(presence);
  assert.equal(presence.schema, JOURNEY_PRESENCE_SCHEMA);
  assert.ok(!serialised.includes('"temporal"'), 'no reading facets inside presence');
  assert.ok(!serialised.includes('"selector"'), 'no source selectors inside presence');
  assert.ok(!serialised.includes('"relations"'), 'no whole relations inside presence');
});

test('every ref a saved presence carries exists in the reading it was built from — no invented identity', () => {
  const model = beats(reading);
  const presence = savedPresence();
  const disclosed = new Set([
    ...model.beats.map((beat) => beat.scene_ref),
    ...model.beats.map((beat) => beat.expression_ref),
  ]);
  assert.ok(disclosed.has(presence.scene_ref));
  assert.ok(disclosed.has(presence.expression_ref));
  for (const scene of presence.draft?.scene_order ?? []) {
    assert.ok(disclosed.has(scene), `draft order names ${scene}, which the reading does not disclose`);
  }
});

test('a corrupt or foreign payload degrades honestly to no presence — never a guess', () => {
  assert.deepEqual(loadPresence(fakeStorage()), {});
  assert.deepEqual(loadPresence(fakeStorage({ [JOURNEY_PRESENCE_KEY]: 'not json at all' })), {});
  assert.deepEqual(loadPresence(fakeStorage({ [JOURNEY_PRESENCE_KEY]: '42' })), {});
  assert.deepEqual(loadPresence(fakeStorage({ [JOURNEY_PRESENCE_KEY]: '["a","list"]' })), {});
  // A drifted entry is dropped, not repaired:
  const drifted = JSON.stringify({
    [PROVING_SUBJECT_REF]: { ...savedPresence(), schema: 'someone-else/v9' },
  });
  assert.deepEqual(loadPresence(fakeStorage({ [JOURNEY_PRESENCE_KEY]: drifted })), {});
  const keyedWrong = JSON.stringify({
    'central:subject:someone-else': savedPresence(),
  });
  assert.deepEqual(loadPresence(fakeStorage({ [JOURNEY_PRESENCE_KEY]: keyedWrong })), {});
  const duplicateOrder = JSON.stringify({
    [PROVING_SUBJECT_REF]: presenceFor({
      subject_ref: PROVING_SUBJECT_REF,
      reading_ref: reading.reading_ref,
      snapshot_revision: null,
      expression_ref: PROVING_EXPRESSION,
      scene_ref: PROVING_SCENE_GROUND,
      draft: { scene_order: [PROVING_SCENE_GROUND, PROVING_SCENE_GROUND], pace: [] },
    }),
  });
  assert.deepEqual(loadPresence(fakeStorage({ [JOURNEY_PRESENCE_KEY]: duplicateOrder })), {}, 'a draft order that repeats a scene is not a journey');
});

test('storage failure degrades: the journey still works, it simply will not restore', () => {
  const throwing = {
    getItem: () => { throw new Error('private mode'); },
    setItem: () => { throw new Error('quota exceeded'); },
  };
  assert.deepEqual(loadPresence(throwing), {});
  assert.doesNotThrow(() => savePresence(throwing, { [PROVING_SUBJECT_REF]: savedPresence() }));
  assert.deepEqual(loadPresence(null), {}, 'no storage at all — presence simply does not survive');
});

test('basis standing: current, field-advanced, stale — the field moving is shown, never hidden', () => {
  const presence = presenceFor({
    subject_ref: PROVING_SUBJECT_REF,
    reading_ref: reading.reading_ref,
    snapshot_revision: null,
    expression_ref: PROVING_EXPRESSION,
    scene_ref: PROVING_SCENE_GROUND,
  });
  assert.equal(basisStanding(presence, reading), 'current');
  assert.equal(
    basisStanding(presence, { ...reading, snapshot: { revision: 'next', basis_ref: null } }),
    'field-advanced',
    'same reading, moved basis — restored AND named',
  );
  assert.equal(
    basisStanding(presence, { ...reading, reading_ref: 'ql.techne:reading:somewhere-else@1' }),
    'stale',
    'a different reading basis no longer applies',
  );
});
