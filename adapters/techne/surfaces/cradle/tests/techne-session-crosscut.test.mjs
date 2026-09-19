import test from 'node:test';
import assert from 'node:assert/strict';
import { createDisclosureSessionStore, coReferenced } from '../src/techne/session.ts';
import { loadFixture } from '../src/techne/fixtures/load-fixtures.mjs';

const tb0 = await loadFixture('tb0-connective-base-v1.json');

const SUBJECT = tb0.subject.subject_ref;
const READING = tb0.reading_ref;
const SCENE = tb0.expressions[0].scene_ref;
const OCCASION = 'central:now:control:root:b417a7c3d37cd47c2762deee3224687d59db1a9015a322e9a8d7584b3947adf6';
const RETURN_TARGET = 'central:source:control:root:Control/user/day/2026-09-16/day.md';
const WORLD = 'central:world:root';
const AGENT_SESSION = 'aikit:agent-session:fixture:l5-techne';

function openProject() {
  const store = createDisclosureSessionStore();
  const session = store.setSelection({
    selection_ref: 'ql.techne:selection:x3',
    subject_ref: SUBJECT,
    reading_ref: READING,
    source_ref: tb0.provenance[0].source_ref,
    source_revision: tb0.provenance[0].source_revision ?? null,
    instrument: 'project',
    agent_session_ref: AGENT_SESSION,
    selection_standing: 'current',
  }, {
    whole_ref: tb0.whole.whole_ref,
    occasion_ref: OCCASION,
    return_target_ref: RETURN_TARGET,
    world_ref: WORLD,
    scene_focus_ref: SCENE,
  });
  return { store, session };
}

/** The identity that must be byte-equal at every step of any walk. */
function identity(session) {
  return {
    session_ref: session.session_ref,
    subject_ref: session.subject_ref,
    reading_ref: session.reading_ref,
    selection_ref: session.selection.selection_ref,
    source_ref: session.selection.source_ref,
    source_revision: session.selection.source_revision ?? null,
    agent_session_ref: session.selection.agent_session_ref ?? null,
    occasion_ref: session.occasion_ref,
    return_target_ref: session.return_target_ref,
    whole_ref: session.whole_ref,
    world_ref: session.world_ref,
    scene_focus_ref: session.scene_focus_ref,
  };
}

test('X3.4 — project → deep instrument → Expression → another deep instrument holds ONE session', () => {
  const { store } = openProject();
  const first = store.get();
  assert.equal(first.application_cut, '4:2-deep');

  const canvas = store.openInInstrument('canvas');
  const expression = store.crossCut('expressions').session;
  const timeline = store.crossCut('timeline').session;

  for (const [label, step] of [['canvas', canvas], ['expressions', expression], ['timeline', timeline]]) {
    assert.equal(step.session_ref, first.session_ref, `${label}: the session is the same session`);
    assert.equal(step.selection.agent_session_ref, AGENT_SESSION, `${label}: the companion ref is never re-minted`);
    assert.equal(step.occasion_ref, OCCASION, `${label}: the occasion survives`);
  }
  assert.deepEqual(identity(timeline), identity(first), 'every identity ref is byte-equal after the full walk');
  assert.deepEqual(
    timeline.navigation.map((hop) => [hop.from_instrument, hop.to_instrument]),
    [['project', 'canvas'], ['canvas', 'expressions'], ['expressions', 'timeline']],
  );
  assert.equal(coReferenced(first, timeline), true, 'the first and last views co-reference one field');
  // The cut actually moved and came back through the conjugate leg.
  assert.equal(expression.application_cut, '3:3-conjugate');
  assert.equal(timeline.application_cut, '4:2-deep');
});

test('X3.1 — the M3 bridge: Journey scene → live Expression → exact Journey position', () => {
  const { store } = openProject();
  const first = store.get();
  const expression = store.crossCut('expressions').session;
  assert.equal(expression.scene_focus_ref, SCENE, 'the focused Scene rides into the lived reading');
  const journey = store.crossCut('journey').session;
  assert.equal(journey.scene_focus_ref, SCENE, 'the exact Scene ref returns to the Journey position');
  assert.equal(journey.session_ref, first.session_ref, 'one session across the whole bridge');
  assert.deepEqual(identity(journey), identity(first), 'the return lands byte-identical');
});

test('X3.2 — the M4 bridge: world occasion → Nara/Expression → same World basis, privacy legible', () => {
  const { store } = openProject();
  const place = store.openInInstrument('place');
  assert.equal(place.spatial_focus_ref, undefined, 'no spatial focus is invented by the hop itself');
  const expression = store.crossCut('expressions').session;
  assert.equal(expression.occasion_ref, OCCASION, 'the current occasion is the same occasion in the lived reading');
  assert.equal(expression.world_ref, WORLD, 'the World focus survives the crossing');
  const worldBack = store.crossCut('place').session;
  assert.deepEqual(identity(worldBack), identity(place), 'the World return lands on the identical basis');
  // Privacy: the reading's anima binding carries the Nara-private disclosure
  // limit; crossing state never strips or widens it (the bridge renders it
  // verbatim with a privacy marker).
  const anima = tb0.agency.find((role) => role.role === 'anima');
  assert.match(anima.privacy, /Nara-private/);
  assert.equal(anima.guardian_ref, 'actuation:agent:nara', 'the Guardian stays the stewardship relation');
});

test('the situated roles name native machinery only — no new runtime, no new canonical identity', () => {
  for (const role of tb0.agency) {
    if (role.agent_session_ref) {
      assert.match(role.agent_session_ref, /^aikit:agent-session:/, 'agent sessions ride the AIKit grammar');
    }
    if (role.guardian_ref) {
      assert.match(role.guardian_ref, /^actuation:agent:/, 'guardians are the canonical Actuation identities');
    }
  }
  assert.equal(tb0.agency.filter((role) => role.role === 'guardian').length, 1, 'stewardship stays with the Guardian');
});

test('the crossing refusal and the companion continuity hold at the store too', () => {
  const { store } = openProject();
  assert.throws(() => store.crossCut('canvas'), /other application cut/, 'project → canvas is no cut crossing');
  const cleared = createDisclosureSessionStore();
  assert.throws(() => cleared.crossCut('expressions'), /No DisclosureSession is open/);
  // Ground replacement on the same basis keeps the session and its ground.
  const { store: same } = openProject();
  const before = same.get();
  const moved = same.setSelection({ ...before.selection, selection_ref: 'ql.techne:selection:x3-2' });
  assert.equal(moved.session_ref, before.session_ref);
  assert.equal(moved.occasion_ref, OCCASION, 'the ground persists across a same-basis re-selection');
  assert.equal(moved.selection.selection_ref, 'ql.techne:selection:x3-2');
});
