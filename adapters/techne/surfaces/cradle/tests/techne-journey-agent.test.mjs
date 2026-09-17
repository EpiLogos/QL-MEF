import test from 'node:test';
import assert from 'node:assert/strict';
import { beats } from '../src/techne/journey/beats.ts';
import { draftFromBeats, moveBeat } from '../src/techne/journey/sequence.ts';
import { journeyAgentState, techne3OperationStanding, TECHNE_3_ROLE } from '../src/techne/journey/agent.ts';
import { bindVak, applyVakTraversal } from '../src/techne/journey/vak.ts';
import {
  PROVING_CONTEXT_FRAME,
  PROVING_SCENE_CROSSING,
  PROVING_SCENE_GROUND,
  PROVING_SCENE_RETURN,
  PROVING_SUBJECT_REF,
  provingReading,
} from '../src/techne/journey/proving-reading.ts';

const reading = provingReading();
const model = beats(reading);

function selection(overrides = {}) {
  return {
    selection_ref: 'techne:test-selection:agent',
    subject_ref: PROVING_SUBJECT_REF,
    reading_ref: reading.reading_ref,
    instrument: 'journey',
    agent_session_ref: 'agent-session:test:technē-3',
    selection_standing: 'current',
    ...overrides,
  };
}

function vakBound() {
  const gate = bindVak({
    cpf: 'human-engaged',
    cfp: 'CFP0',
    cs: 'CS0',
    direction: 'forward-synthesis',
    cf_ref: PROVING_CONTEXT_FRAME,
    cp: [
      { scene_ref: PROVING_SCENE_GROUND, cp: '4.0' },
      { scene_ref: PROVING_SCENE_CROSSING, cp: '4.2' },
      { scene_ref: PROVING_SCENE_RETURN, cp: '4.5' },
    ],
    ct: [],
  }, reading, model.beats);
  assert.equal(gate.reason, null);
  return gate.binding;
}

test('the structured state publishes what Aletheia_3 / Technē_3 may perceive — identity, cut, position, basis', () => {
  const state = journeyAgentState({
    reading,
    selection: selection(),
    beats: model.beats,
    drafts: {},
    currentSceneRef: PROVING_SCENE_CROSSING,
    vak: null,
  });
  assert.equal(state.contract, 'ql.techne/v1');
  assert.equal(state.role, TECHNE_3_ROLE);
  assert.equal(state.instrument, 'journey');
  assert.equal(state.m_prime, 3);
  assert.equal(state.application_cut, '4:2-deep');
  assert.equal(state.subject_ref, PROVING_SUBJECT_REF);
  assert.equal(state.reading_ref, reading.reading_ref);
  assert.equal(state.current_scene_ref, PROVING_SCENE_CROSSING);
  assert.equal(state.agent_session_ref, 'agent-session:test:technē-3', 'the owner-grammar agent ref rides verbatim, never minted');
  assert.equal(state.expression_scene_focus_ref, PROVING_SCENE_CROSSING, 'the live 3:3 relation is inspectable');
});

test('the sequence reflects the draft order; the source basis carries standing from the provenance', () => {
  const draft = moveBeat(draftFromBeats(model.beats, reading.expressions[0].expression_ref), PROVING_SCENE_GROUND, 2);
  const state = journeyAgentState({
    reading,
    selection: selection(),
    beats: model.beats,
    drafts: { [reading.expressions[0].expression_ref]: draft },
    currentSceneRef: null,
    vak: null,
  });
  assert.deepEqual(state.sequence[0].scene_refs.map((entry) => entry.scene_ref), [
    PROVING_SCENE_CROSSING,
    PROVING_SCENE_RETURN,
    PROVING_SCENE_GROUND,
  ], 'the draft order is the published order');
  const crossingBasis = state.source_basis.find((entry) => entry.scene_ref === PROVING_SCENE_CROSSING);
  assert.equal(crossingBasis.subject_ref, PROVING_SUBJECT_REF);
  assert.ok(crossingBasis.sources.some((source) => source.standing === 'architecture-contract'),
    'relation/source standing is published, not stripped');
});

test('available actions are the reading disclosed ActionRefs — the only mutation channel, same as the human surface', () => {
  const state = journeyAgentState({ reading, selection: selection(), beats: model.beats, drafts: {}, currentSceneRef: null, vak: null });
  assert.deepEqual(
    state.available_actions.map((action) => action.action_ref).sort(),
    ['aikit.wiki.stage', 'central.day.read', 'central.now.read', 'oi.expression.open'],
  );
  assert.match(state.governance.limits.join(' '), /same refs the human surface uses/);
});

test('C′ appears where actually bound, with visited and unvisited scenes; governance follows the C0′ regime', async () => {
  const unbound = journeyAgentState({ reading, selection: selection(), beats: model.beats, drafts: {}, currentSceneRef: null, vak: null });
  assert.equal(unbound.c_prime, null);
  assert.equal(unbound.governance.cpf_regime, null);
  assert.equal(unbound.governance.techne_3_operation, 'unavailable');

  const binding = vakBound();
  const vak = { binding, traversal: applyVakTraversal(binding, model.beats) };
  const state = journeyAgentState({ reading, selection: selection(), beats: model.beats, drafts: {}, currentSceneRef: null, vak });
  assert.equal(state.c_prime.cs, 'CS0');
  assert.equal(state.c_prime.cf_ref, PROVING_CONTEXT_FRAME);
  assert.deepEqual(state.c_prime.visited_scene_refs, [PROVING_SCENE_GROUND, PROVING_SCENE_RETURN, PROVING_SCENE_CROSSING]);
  assert.deepEqual(state.c_prime.unvisited_scene_refs, []);
  assert.equal(state.governance.cpf_regime, 'human-engaged');
  assert.equal(state.governance.techne_3_operation, 'propose-await-human-acceptance');

  const autonomous = journeyAgentState({
    reading, selection: selection(), beats: model.beats, drafts: {}, currentSceneRef: null,
    vak: { binding: { ...binding, cpf: 'authorised-autonomous' }, traversal: vak.traversal },
  });
  assert.equal(autonomous.governance.techne_3_operation, 'commissioned');
});

test('the regime note distinguishes proposal from commission — authority stays the owner', () => {
  const state = journeyAgentState({ reading, selection: selection(), beats: model.beats, drafts: {}, currentSceneRef: null, vak: null });
  assert.match(techne3OperationStanding(state).note, /unavailable|inspectable/);
  const humanEngaged = { ...state, governance: { ...state.governance, techne_3_operation: 'propose-await-human-acceptance' } };
  assert.match(techne3OperationStanding(humanEngaged).note, /human accepts or refuses/);
  const commissioned = { ...state, governance: { ...state.governance, techne_3_operation: 'commissioned' } };
  assert.match(techne3OperationStanding(commissioned).note, /routes through the owner/);
});

test('the state is JSON-safe — an agent session receives it through the ordinary disclosure channel', () => {
  const state = journeyAgentState({ reading, selection: selection(), beats: model.beats, drafts: {}, currentSceneRef: PROVING_SCENE_CROSSING, vak: null });
  assert.deepEqual(JSON.parse(JSON.stringify(state)), state);
});
