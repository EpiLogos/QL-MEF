import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { relationField, relationEdges } from '../src/techne/timeline/relations.ts';
import { expressionCrossing, relationFieldState } from '../src/techne/timeline/state.ts';

const specimen = JSON.parse(
  await readFile(new URL('../../../../../fixtures/techne/tb0-connective-base-v1.json', import.meta.url), 'utf8'),
);

const selectionOn = (edgeId) => ({
  selection_ref: 'ql.techne:selection:m2-crossing',
  subject_ref: specimen.subject.subject_ref,
  reading_ref: specimen.reading_ref,
  instrument: 'timeline',
  focus_refs: [edgeId],
});

test('the structured state carries the projection, verbatim relations, Actions and Agency role bindings', () => {
  const state = relationFieldState(specimen, 'relations');
  assert.equal(state.schema, 'ql.techne.relation-field-state/v1');
  assert.equal(state.instrument, 'timeline');
  assert.equal(state.m_prime, 2);
  assert.equal(state.projection, 'relations');
  assert.equal(state.subject_ref, specimen.subject.subject_ref);
  assert.equal(state.reading_ref, specimen.reading_ref);
  assert.equal(state.snapshot_revision, null);
  assert.equal(state.relations.length, 4);
  const instantiates = state.relations.find((edge) => edge.relation === 'INSTANTIATES');
  assert.equal(instantiates.standing_verbatim, 'interpretation', 'standing is verbatim, never laundered');
  assert.equal(instantiates.temporal_qualification.state, 'trans-temporal');
  assert.equal(instantiates.native_ref, 'ql.techne:relation:fixture:tb0-instantiates-office');
  assert.deepEqual(
    state.disclosed_actions.map((action) => action.action_ref),
    specimen.actions.map((action) => action.action_ref),
    'Actions are the reading\'s own, verbatim',
  );
  const techne = state.agency.find((role) => role.role === 'techne');
  assert.equal(techne.m_index, 2, 'the specimen binds Technē_2 to this instrument');
  assert.equal(techne.instrument, 'timeline');
});

test('the derivation split names exactly the entries whose ids are derived, and unresolved refs', () => {
  const noRefs = {
    ...structuredClone(specimen),
    whole: {
      whole_ref: 'test:whole:derived',
      relations: [
        { relation: 'CAUSES', from_ref: 'x:a', to_ref: 'x:b' },
        { relation: 'ECHOES', relation_ref: 'r:native', from_ref: 'x:a', to_ref: 'x:c' },
      ],
    },
  };
  const state = relationFieldState(noRefs, 'relations');
  assert.deepEqual(state.derivation.derived_ids, ['derived:techne:relation[0]'], 'the relation with no native ref is flagged derived');
  assert.deepEqual(state.derivation.unresolved_temporal_refs, []);
});

test('the session time window passes through when one is in force; none is invented', () => {
  assert.equal(relationFieldState(specimen, 'timeline').time_window, null);
  const window = { from: '2026-09-15T00:00:00+01:00', to: '2026-09-16T00:00:00+01:00' };
  assert.deepEqual(relationFieldState(specimen, 'timeline', null, window).time_window, window);
});

test('a source-backed focused relation warrants the M2 crossing, carrying the disclosed Action', () => {
  const edge = relationEdges(specimen).find((candidate) => candidate.relation === 'implemented-in');
  const crossing = expressionCrossing(specimen, selectionOn(edge.id), relationField(specimen));
  assert.equal(crossing.warranted, true);
  assert.equal(crossing.action_ref, 'oi.expression.open', 'the crossing rides the reading\'s own disclosed Action');
  assert.match(crossing.reason, /no resonance, correspondence or musical meaning/, 'the warrant asserts nothing harmonic');
});

test('an interpretation is refused even though it cites a source — the no-upgrade law at the crossing', () => {
  const edge = relationEdges(specimen).find((candidate) => candidate.relation === 'INSTANTIATES');
  const crossing = expressionCrossing(specimen, selectionOn(edge.id), relationField(specimen));
  assert.equal(crossing.warranted, false);
  assert.match(crossing.reason, /interpretation/);
  assert.match(crossing.reason, /only source-backed or canonically derived/);
});

test('an owner-supplied derivation ref warrants the crossing as canonically derived', () => {
  const derived = {
    ...structuredClone(specimen),
    whole: {
      ...specimen.whole,
      relations: [
        {
          relation: 'RESONATES_WITH',
          relation_ref: 'r:derived-echo',
          from_ref: specimen.whole.member_refs[0],
          to_ref: specimen.whole.member_refs[1],
          derivation_ref: 'ql:fixture:derived-echo-derivation',
          standing: null,
        },
      ],
    },
  };
  const field = relationField(derived);
  const crossing = expressionCrossing(derived, selectionOn('r:derived-echo'), field);
  assert.equal(crossing.warranted, true);
  assert.match(crossing.reason, /canonically derived/);
});

test('an unavailable 3:3 reading refuses the crossing with the disclosure\'s own reason', () => {
  const closed = {
    ...structuredClone(specimen),
    disclosure: {
      ...specimen.disclosure,
      instruments: specimen.disclosure.instruments.map((entry) => (
        entry.instrument === 'expressions'
          ? { ...entry, available: false, reason: 'no Expression is bound to this subject' }
          : entry
      )),
      application_cuts: [{ cut: '3:3-conjugate', available: false, reason: 'no Expression is bound to this subject' }],
    },
  };
  const edge = relationEdges(closed).find((candidate) => candidate.relation === 'implemented-in');
  const crossing = expressionCrossing(closed, selectionOn(edge.id));
  assert.equal(crossing.warranted, false);
  assert.match(crossing.reason, /no Expression is bound to this subject/);
});

test('no focused relation means no warrant — a plain instrument change needs none', () => {
  const crossing = expressionCrossing(specimen, { ...selectionOn('nothing'), focus_refs: [] });
  assert.equal(crossing.warranted, false);
  assert.match(crossing.reason, /no relation is focused/);
});
