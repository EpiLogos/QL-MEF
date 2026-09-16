import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { canvasAgencyState } from '../src/techne/canvas/agency.ts';
import { computeLayout, applyManualOverrides } from '../src/techne/canvas/layout.ts';
import { createCanvasView } from '../src/techne/canvas/view.ts';
import { createProposal, proposeRelation, rejectProposal } from '../src/techne/canvas/proposal.ts';

const here = fileURLToPath(new URL('.', import.meta.url));
const tb0 = JSON.parse(
  await readFile(new URL('../../../../../fixtures/techne/tb0-connective-base-v1.json', 'file://' + here), 'utf8'),
);

const layout = computeLayout(tb0);
const view = createCanvasView(tb0, {
  placements: [{ ref: tb0.whole.member_refs[0], x: 0.1, y: 0.1 }],
  frames: [{ frame_ref: 'ql.techne:canvas-frame:agency', label: 'Docs', member_refs: tb0.whole.member_refs.slice(0, 3), z: 1 }],
});
const positioned = applyManualOverrides(layout, { [tb0.whole.member_refs[0]]: { x: 0.1, y: 0.1 } }).nodes
  .map((node) => ({ ref: node.ref, x: node.x, y: node.y, radius: node.role === 'whole' ? 0.018 : 0.009 }));

const proposal = rejectProposal(proposeRelation(createProposal(tb0, {
  from_ref: tb0.whole.member_refs[0],
  to_ref: tb0.whole.member_refs[1],
  relation: 'companion',
  standing: 'interpretation',
})));

const state = canvasAgencyState({
  reading: tb0,
  selection: {
    selection_ref: 'ql.techne:selection:agency-test',
    subject_ref: tb0.subject.subject_ref,
    reading_ref: tb0.reading_ref,
    instrument: 'canvas',
    focus_refs: [tb0.whole.member_refs[0]],
  },
  view,
  placedRefs: new Set(layout.nodes.map((node) => node.ref)),
  arrangedNodes: positioned,
  proposals: [proposal],
});

test('the agency state is JSON-serialisable structured data — the no-DOM law', () => {
  const round = JSON.parse(JSON.stringify(state));
  assert.equal(Object.keys(round).length, Object.keys(state).length);
  for (const key of Object.keys(state)) {
    if (state[key] === undefined) continue;
    assert.deepEqual(round[key], state[key], `${key} survives a JSON round-trip byte-faithfully`);
  }
});

test('exact refs: subject, whole, reading, snapshot, selection, focus — carried, not reconstructed', () => {
  assert.equal(state.subject_ref, tb0.subject.subject_ref);
  assert.equal(state.whole_ref, tb0.whole.whole_ref);
  assert.equal(state.reading_ref, tb0.reading_ref);
  assert.equal(state.snapshot_revision, tb0.snapshot.revision);
  assert.equal(state.selection_ref, 'ql.techne:selection:agency-test');
  assert.deepEqual(state.focus_refs, [tb0.whole.member_refs[0]]);
  assert.equal(state.instrument, 'canvas');
  assert.equal(state.m_prime, 1);
  assert.equal(state.application_cut, '4:2-deep');
});

test('semantic relations ride verbatim with their TB0 evidence discipline', () => {
  const implemented = state.semantic.relations.find((relation) => relation.relation === 'implemented-in');
  assert.equal(implemented.relation_ref, 'ql.techne:relation:fixture:tb0-implemented-in-schema');
  assert.equal(implemented.standing, 'fact');
  assert.equal(implemented.temporal_facet_ref, 'ql.techne:facet:tb0:received');
  assert.ok(state.semantic.relations_outside_placed_whole.some((relation) => relation.relation === 'INSTANTIATES'),
    'the adrift INSTANTIATES target is reported as outside the placed whole');
});

test('presentation is separated and labelled: view, placements, frames, bounds', () => {
  assert.equal(state.view.view_ref, view.view_ref);
  assert.deepEqual(state.view.placement_refs, [tb0.whole.member_refs[0]]);
  assert.deepEqual(state.view.frame_member_refs, tb0.whole.member_refs.slice(0, 3));
  assert.equal(state.view.layout_basis.scheme, 'ql-constellation');
  assert.ok(state.view.bounds.width >= 0);
  assert.match(state.authority_note, /presentation state/);
});

test('proposals appear with status and receipts — an Agency sees suggestions as suggestions', () => {
  const entry = state.proposals[0];
  assert.equal(entry.relation, 'companion');
  assert.equal(entry.status, 'rejected');
  assert.equal(entry.standing, 'interpretation');
  assert.equal(entry.receipt_routed, undefined);
});

test('disclosed native Actions are carried verbatim — routing only, never execution', () => {
  assert.deepEqual(state.actions, tb0.actions);
  assert.match(state.authority_note, /native Actions/);
});

test('the warranted QL reading is exposed with its warrant — and the honest note explains layout standing', () => {
  assert.equal(state.warranted_ql.shape_ref, tb0.ql.shape_ref);
  assert.equal(state.warranted_ql.m_coordinate_ref, tb0.ql.m_coordinate_ref);
  assert.equal(state.warranted_ql.warrant.result_class, 'canonical');
  assert.match(state.warranted_ql_note, /canonical/);
});

test('agency role bindings are verbatim, and the Technē_1 binding is honestly absent when the reading situates none', () => {
  assert.deepEqual(state.agency_roles, tb0.agency);
  assert.equal(state.techne_1_binding, null);
  assert.match(state.techne_1_note, /no Technē_1 canvas binding/);
  assert.match(state.techne_1_note, /techne@M2\/timeline/, 'the roles that ARE situated are named');
});

test('cut disclosure rides in the state — the M1′ ↔ 3:3 crossing is available as data', () => {
  const cuts = state.disclosure.application_cuts.map((cut) => cut.cut).sort();
  assert.deepEqual(cuts, ['3:3-conjugate', '4:2-deep']);
  const expressionsEntry = state.disclosure.instruments.find((entry) => entry.instrument === 'expressions');
  assert.equal(expressionsEntry.available, true, 'the expressions crossing is disclosed available for this reading');
});

test('a reading without a warrant gets the honest null plus note, never invented QL', async () => {
  const absent = JSON.parse(await readFile(new URL('../src/techne/fixtures/absent-facets-v1.json', 'file://' + here), 'utf8'));
  const state2 = canvasAgencyState({
    reading: absent,
    selection: null,
    view: null,
    placedRefs: new Set(),
    arrangedNodes: [],
    proposals: [],
  });
  assert.equal(state2.warranted_ql, null);
  assert.match(state2.warranted_ql_note, /no warranted QL reading/);
  assert.equal(state2.view, null);
});
