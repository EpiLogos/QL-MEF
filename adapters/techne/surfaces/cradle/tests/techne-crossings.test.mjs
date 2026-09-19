import test from 'node:test';
import assert from 'node:assert/strict';
import {
  deepEntryInstrument,
  requestTechneCrossing,
  resolveCrossing,
  subscribeTechneCrossing,
} from '../src/techne/open.ts';
import { createDisclosureSessionStore } from '../src/techne/session.ts';
import { loadFixture, loadFixtureReadings } from '../src/techne/fixtures/load-fixtures.mjs';

const tb0 = await loadFixture('tb0-connective-base-v1.json');
const developmentDay = (await loadFixtureReadings()).find((reading) => reading.subject.subject_ref.startsWith('central:now:'));

const storeOn = (fixture, instrument, overrides = []) => {
  const store = createDisclosureSessionStore();
  const session = store.setSelection({
    selection_ref: 'ql.techne:selection:crossing-test',
    subject_ref: fixture.subject.subject_ref,
    reading_ref: fixture.reading_ref,
    source_ref: fixture.provenance[0].source_ref,
    source_revision: fixture.provenance[0].source_revision ?? null,
    instrument,
    agent_session_ref: 'aikit:agent-session:fixture:l5-techne',
    ...overrides,
  }, {
    occasion_ref: 'central:now:control:root:b417a7c3d37cd47c2762deee3224687d59db1a9015a322e9a8d7584b3947adf6',
    return_target_ref: 'central:source:control:root:Control/user/day/2026-09-16/day.md',
    whole_ref: fixture.whole.whole_ref,
  });
  return { store, session };
};

test('Open as Expression resolves from the reading: target, cut, and the disclosed crossing action', () => {
  const { session } = storeOn(tb0, 'timeline');
  const resolution = resolveCrossing(tb0, session, 'open-as-expression');
  assert.equal(resolution.available, true);
  assert.equal(resolution.crossing.target, 'expressions');
  assert.equal(resolution.crossing.cut, '3:3-conjugate');
  assert.equal(resolution.crossing.crossesCut, true);
  // The leg corresponds to the native ActionRef the reading discloses — the
  // oi.expression.open crossing action — routing stays with the adapter seam.
  assert.equal(resolution.crossing.action_ref, 'oi.expression.open');
});

test('open-deep-instrument defaults to the suggested entry; a refused coordinate carries its reason', () => {
  const { session } = storeOn(tb0, 'expressions');
  // The TB0 specimen suggests Journey and situates Technē_2 on the timeline;
  // the default entry derives from the disclosure either way.
  const defaultLeg = resolveCrossing(tb0, session, 'open-deep-instrument');
  assert.equal(defaultLeg.available, true);
  assert.equal(defaultLeg.crossing.cut, '4:2-deep');
  // A named coordinate is gated by its own disclosure entry: the fixture's
  // Palace is deliberately unavailable (no integral composition is bound).
  const palaceLeg = resolveCrossing(tb0, session, 'open-deep-instrument', 'palace');
  assert.equal(palaceLeg.available, false);
  assert.match(palaceLeg.reason, /integral composition/);
});

test('the deep entry instrument derives from the disclosure, never hard-coded', () => {
  assert.equal(deepEntryInstrument(tb0.disclosure), 'journey', 'the fixture suggests Journey — the formal M3 crossing');
  assert.equal(deepEntryInstrument(developmentDay.disclosure), 'timeline', 'the development-day fixture suggests the timeline');
});

test('Return to Journey position carries the bound Expression Scene — the M3 position survives', () => {
  const { session } = storeOn(tb0, 'expressions');
  const resolution = resolveCrossing(tb0, session, 'return-to-journey-position');
  assert.equal(resolution.available, true);
  assert.equal(resolution.crossing.target, 'journey');
  assert.equal(resolution.crossing.scene_focus_ref, tb0.expressions[0].scene_ref);
});

test('source/graph depth and Return to Project ground address M0 from both readings', () => {
  // From the 3:3 reading the depth is the crossing into the 4:2 ground.
  const conjugate = storeOn(tb0, 'expressions');
  for (const kind of ['open-source-graph-depth', 'return-to-project-ground']) {
    const resolution = resolveCrossing(tb0, conjugate.session, kind);
    assert.equal(resolution.available, true, kind);
    assert.equal(resolution.crossing.target, 'project');
    assert.equal(resolution.crossing.crossesCut, true, `${kind} crosses from 3:3 into the 4:2 ground`);
    assert.equal(resolution.crossing.cut, '4:2-deep');
  }
  // From a 4:2 deep instrument the same legs degrade to a same-cut hop on
  // the ground — still gated on the disclosure, never a fake crossing.
  const deep = storeOn(tb0, 'timeline');
  const hop = resolveCrossing(tb0, deep.session, 'return-to-project-ground');
  assert.equal(hop.available, true);
  assert.equal(hop.crossing.target, 'project');
  assert.equal(hop.crossing.crossesCut, false, 'ground from the deep side is a navigation hop, not a crossing');
  // And an unavailable ground entry is refused with its reason.
  const groundless = {
    ...tb0,
    disclosure: {
      ...tb0.disclosure,
      instruments: tb0.disclosure.instruments.map((entry) => entry.instrument === 'project'
        ? { ...entry, available: false, reason: 'no project ground is disclosed for this subject' }
        : entry),
    },
  };
  const refusedHop = resolveCrossing(groundless, deep.session, 'return-to-project-ground');
  assert.equal(refusedHop.available, false);
  assert.match(refusedHop.reason, /no project ground is disclosed/);
});

test('Open Agent/Epii depth changes no cut and rides a disclosed agent-owned action', () => {
  const { session } = storeOn(tb0, 'expressions');
  const resolution = resolveCrossing(tb0, session, 'open-agent-depth');
  assert.equal(resolution.available, true);
  assert.equal(resolution.crossing.crossesCut, false);
  assert.equal(resolution.crossing.target, session.instrument, 'the aperture is summonable without leaving the reading');
  assert.equal(resolution.crossing.action_ref, 'aikit.wiki.stage', 'the first disclosed agent-owned action rides the leg');
});

test('with no disclosed agent action and no companion session ref, the Agent depth is honestly unavailable', () => {
  // The development-day reading discloses only central-owned actions.
  const bare = storeOn(developmentDay, 'canvas', { agent_session_ref: null });
  const resolution = resolveCrossing(developmentDay, bare.session, 'open-agent-depth');
  assert.equal(resolution.available, false);
  assert.match(resolution.reason, /Agent depth is unavailable/);
});

test('an application cut the disclosure marks unavailable is refused with its recorded reason', () => {
  const unexpressive = {
    ...tb0,
    disclosure: {
      ...tb0.disclosure,
      application_cuts: tb0.disclosure.application_cuts.map((entry) => entry.cut === '3:3-conjugate'
        ? { cut: '3:3-conjugate', available: false, reason: 'no Expression profile is bound to this subject' }
        : entry),
    },
  };
  const { session } = storeOn(tb0, 'timeline');
  const resolution = resolveCrossing(unexpressive, session, 'open-as-expression');
  assert.equal(resolution.available, false);
  assert.match(resolution.reason, /no Expression profile is bound/);
});

test('one subject, one crossing: a session on another subject is refused', () => {
  const elsewhere = storeOn(developmentDay, 'canvas');
  const resolution = resolveCrossing(tb0, elsewhere.session, 'open-as-expression');
  assert.equal(resolution.available, false);
  assert.match(resolution.reason, /one subject, one crossing/);
});

test('requestTechneCrossing emits the crossed session to the shell consumer', () => {
  const { store, session } = storeOn(tb0, 'timeline');
  const resolution = resolveCrossing(tb0, session, 'open-as-expression');
  assert.equal(resolution.available, true);
  const crossed = store.crossCut(resolution.crossing.target).session;
  const seen = [];
  const stop = subscribeTechneCrossing((request) => seen.push(request));
  requestTechneCrossing({ crossing: resolution.crossing, session: crossed });
  stop();
  assert.equal(seen.length, 1);
  assert.equal(seen[0].crossing.kind, 'open-as-expression');
  assert.equal(seen[0].session.instrument, 'expressions');
  assert.equal(seen[0].session.occasion_ref, session.occasion_ref, 'the occasion rides the emitted crossing');
});

test('the same-cut refusal is the crossing law, not the affordance', () => {
  const { session } = storeOn(tb0, 'expressions');
  const resolution = resolveCrossing(tb0, session, 'open-as-expression');
  assert.equal(resolution.available, false);
  assert.match(resolution.reason, /already occupies the 3:3-conjugate reading/);
});
