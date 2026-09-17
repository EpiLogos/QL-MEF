import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  DEFAULT_PROJECT_DEPTH,
  DEFAULT_PROJECT_EDGE_BUDGET,
  DEFAULT_PROJECT_NODE_BUDGET,
  boundedView,
  focusSelection,
  graphProjection,
  listProjection,
  projectQuery,
  projectSelectionRef,
  tierDisclosure,
  treeProjection,
} from '../src/techne/project/projections.ts';
import { validateSelection } from '../src/techne/contract.ts';
import { createDisclosureSessionStore } from '../src/techne/session.ts';

// tests/ → cradle → surfaces → techne → adapters → repo root.
const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..', '..', '..', '..', '..');
const specimen = JSON.parse(
  await readFile(resolve(repoRoot, 'fixtures/techne/tb0-connective-base-v1.json'), 'utf8'),
);

const LOCK = 'central:source:control:root:Work/Quaternal-Logic/docs/L5-TECHNE-DUAL-READING-LOCK.md';
const WAYFINDER = 'central:source:control:root:Work/Quaternal-Logic/docs/L5-TECHNE-INSTRUMENT-WAYFINDER.md';
const VAK = 'ql:vak:composition:l5-para-vak';

test('the query defaults to the reading’s own focus within AIKit-parity budgets', () => {
  const query = projectQuery(specimen);
  assert.equal(query.focus_ref, specimen.whole.focus_refs[0], 'the reading’s focus_refs lead');
  assert.equal(query.depth, DEFAULT_PROJECT_DEPTH, 1);
  assert.equal(query.max_nodes, DEFAULT_PROJECT_NODE_BUDGET, 96);
  assert.equal(query.max_edges, DEFAULT_PROJECT_EDGE_BUDGET, 192);
  assert.throws(() => projectQuery(specimen, { max_nodes: 0 }), /bounded-whole law/, 'zero budgets are refused');
});

test('the bounded view expands from the focus and holds boundary endpoints honestly', () => {
  const view = boundedView(specimen, projectQuery(specimen, { focus_ref: LOCK, depth: 1 }));
  const refs = view.nodes.map((node) => node.ref);
  assert.equal(view.nodes[0].ref, LOCK, 'the focus is the first node');
  assert.ok(refs.includes(WAYFINDER), 'a first-hop member is present');
  assert.ok(refs.includes(VAK), 'a first-hop endpoint outside the whole is still named');
  const vak = view.nodes.find((node) => node.ref === VAK);
  assert.equal(vak.member, false, 'the Vāk composition is a boundary endpoint, not a member');
  assert.deepEqual(view.boundary_refs, [VAK]);
  assert.equal(view.truncated, false, 'the fixture neighbourhood fits the budgets');
  // Every edge drawn touches the focus at depth 1.
  assert.ok(view.edges.every((edge) => edge.from_ref === LOCK || edge.to_ref === LOCK));
  // TB0 identity rides verbatim on edges.
  const supersedes = view.edges.find((edge) => edge.relation === 'supersedes');
  assert.equal(supersedes.relation_ref, 'ql.techne:relation:fixture:tb0-supersedes-grouping');
  assert.equal(supersedes.standing, 'architecture-contract');
  assert.equal(supersedes.temporal_facet_ref, null, 'the trans-temporal relation carries no manufactured date');
});

test('depth 2 reaches further through members; the reading is never written', () => {
  const depth1 = boundedView(specimen, projectQuery(specimen, { focus_ref: LOCK, depth: 1 }));
  const depth2 = boundedView(specimen, projectQuery(specimen, { focus_ref: LOCK, depth: 2 }));
  assert.ok(depth2.nodes.length >= depth1.nodes.length, 'expansion never shrinks the neighbourhood');
  const structured = JSON.parse(JSON.stringify(specimen));
  boundedView(specimen, projectQuery(specimen, { focus_ref: LOCK, depth: 3 }));
  assert.deepEqual(specimen, structured, 'projections are pure — the reading keeps its shape');
});

test('budgets truncate with honesty instead of growing a hairball', () => {
  const view = boundedView(specimen, projectQuery(specimen, { focus_ref: LOCK, depth: 1, max_nodes: 2 }));
  assert.equal(view.truncated, true, 'the stop is reported');
  assert.ok(view.warnings.some((warning) => /node budget/.test(warning)), 'the warning names the bound');
  assert.ok(view.nodes.length <= 2);
});

test('LIST, TREE and GRAPH are projections of the one view — same refs everywhere', () => {
  const view = boundedView(specimen, projectQuery(specimen, { focus_ref: LOCK, depth: 2 }));
  const expected = view.nodes.map((node) => node.ref).sort();
  const listRefs = listProjection(view).map((row) => row.ref).sort();
  const graphRefs = graphProjection(view).nodes.map((node) => node.ref).sort();
  const collected = [];
  const walk = (node) => { collected.push(node.ref); node.children.forEach(walk); };
  walk(treeProjection(view));
  assert.deepEqual(listRefs, expected);
  assert.deepEqual(graphRefs, expected);
  assert.deepEqual(collected.sort(), expected, 'the tree covers the same nodes, attached or detached');
  // The graph keeps every edge with its identity; the focus sits at the centre.
  assert.deepEqual(graphProjection(view).edges, view.edges);
  const focus = graphProjection(view).nodes.find((node) => node.ref === LOCK);
  assert.deepEqual(focus.at, { x: 0, y: 0 });
});

test('membership is a disclosed fact: members beyond the bounded depth stay in the view, marked unreached', () => {
  const view = boundedView(specimen, projectQuery(specimen, { focus_ref: LOCK, depth: 1 }));
  const sessionSchema = 'central:source:control:root:Work/Quaternal-Logic/schemas/techne/ql-techne-session-v1.schema.json';
  const node = view.nodes.find((candidate) => candidate.ref === sessionSchema);
  assert.ok(node, 'a disclosed member is never dropped for want of relations');
  assert.equal(node.member, true);
  assert.equal(node.distance, null, 'unreached within the bounded depth — honest, not hidden');
  // LIST sorts reached nodes before unreached members.
  const rows = listProjection(view);
  const wayfinderIndex = rows.findIndex((row) => row.ref === WAYFINDER);
  const sessionIndex = rows.findIndex((row) => row.ref === sessionSchema);
  assert.ok(wayfinderIndex < sessionIndex, 'reached nodes sort before unreached members');
});

test('the tree is the relation spanning-tree; members no relation reaches hold under the detached root, not an invented containment', () => {
  const view = boundedView(specimen, projectQuery(specimen, { focus_ref: LOCK, depth: 1 }));
  const root = treeProjection(view);
  assert.equal(root.ref, LOCK);
  assert.equal(root.reached_by, null);
  const wayfinder = root.children.find((child) => child.ref === WAYFINDER);
  assert.ok(wayfinder, 'a member related to the focus hangs from it');
  assert.equal(wayfinder.reached_by.relation, 'supersedes', 'the reaching relation is carried verbatim');
  // The EPI-TA-ONTA matrix and the session schema are disclosed members no
  // fixture relation touches: they appear under the detached root with
  // reached_by null, never absorbed by a fabricated CONTAINS edge.
  const detached = root.children.filter((child) => child.reached_by === null && child.ref !== root.ref);
  const detachedRefs = detached.map((child) => child.ref);
  assert.ok(detachedRefs.includes('central:source:control:root:Work/Quaternal-Logic/docs/integrations/epi-logos/EPI-TA-ONTA-AGENT-WORLD-CAPABILITY-MATRIX.md'));
  assert.ok(detachedRefs.includes('central:source:control:root:Work/Quaternal-Logic/schemas/techne/ql-techne-session-v1.schema.json'));
});

test('a focus selection keeps the subject and source basis and uses the shared selection-ref grammar', () => {
  const store = createDisclosureSessionStore();
  const session = store.setSelection({
    selection_ref: 'ql.techne:selection:origin',
    subject_ref: specimen.subject.subject_ref,
    reading_ref: specimen.reading_ref,
    instrument: 'canvas',
    source_ref: 'central:source:control:root:Work/Quaternal-Logic/docs/L5-TECHNE-DUAL-READING-LOCK.md',
    source_revision: 'r1',
    agent_session_ref: 'aikit:agent-session:fixture:l5-techne',
  });
  const focused = focusSelection(specimen, session.selection, WAYFINDER);
  assert.equal(focused.subject_ref, session.selection.subject_ref, 'one subject across the switch');
  assert.equal(focused.reading_ref, session.reading_ref);
  assert.equal(focused.source_ref, session.selection.source_ref, 'the source basis survives');
  assert.equal(focused.source_revision, session.selection.source_revision);
  assert.equal(focused.agent_session_ref, session.selection.agent_session_ref);
  assert.equal(focused.instrument, 'project');
  assert.deepEqual(focused.focus_refs, [WAYFINDER]);
  assert.equal(validateSelection(focused).valid, true);
  assert.equal(focused.selection_ref, projectSelectionRef(specimen.subject.subject_ref, WAYFINDER));
  assert.equal(focused.selection_ref, `ql.techne:selection:${specimen.subject.subject_ref}:${WAYFINDER}`, 'same grammar the Canvas lane derives');
  assert.throws(() => focusSelection(specimen, session.selection, '   '), /verbatim/, 'an empty focus is refused');
  const moved = store.setSelection(focused);
  assert.equal(moved.session_ref, session.session_ref, 'same basis: the session persists');
  assert.equal(moved.instrument, 'project');
});

test('semantic zoom tiers disclose more or less of the SAME view — never a re-query', () => {
  const field = tierDisclosure('field');
  const locality = tierDisclosure('locality');
  const detail = tierDisclosure('detail');
  assert.deepEqual(field, { labels: false, relationFacets: false, countsOnly: true }, 'the far-out tier is counts and the focus');
  assert.equal(locality.labels, true);
  assert.equal(locality.relationFacets, false, 'relation identity/standing wait for the detail tier');
  assert.deepEqual(detail, { labels: true, relationFacets: true, countsOnly: false });
});
