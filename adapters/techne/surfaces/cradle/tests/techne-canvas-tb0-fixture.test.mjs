import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { validateReading, validateSession } from '../src/techne/contract.ts';
import { computeLayout } from '../src/techne/canvas/layout.ts';

// The canonical TB0 rich specimen, read from its canonical location at the
// repo root (`fixtures/techne/tb0-connective-base-v1.json`, QL-MEF #212) —
// one copy, never a private re-export.
const here = fileURLToPath(new URL('.', import.meta.url));
const tb0 = JSON.parse(
  await readFile(new URL('../../../../../fixtures/techne/tb0-connective-base-v1.json', 'file://' + here), 'utf8'),
);

test('the TB0 rich specimen validates against the TB0-1 field set of the TS mirror', () => {
  const checked = validateReading(tb0);
  assert.deepEqual(checked, { valid: true, errors: [] }, JSON.stringify(checked.errors));
});

test('TB0 relation evidence discipline survives the mirror: identity, standing, source, temporal qualification', () => {
  const implemented = tb0.whole.relations.find((relation) => relation.relation === 'implemented-in');
  assert.equal(implemented.relation_ref, 'ql.techne:relation:fixture:tb0-implemented-in-schema');
  assert.equal(implemented.standing, 'fact');
  assert.equal(implemented.temporal_facet_ref, 'ql.techne:facet:tb0:received', 'the dated relation carries its real temporal qualification ref');
  const instantiates = tb0.whole.relations.find((relation) => relation.relation === 'INSTANTIATES');
  assert.equal(instantiates.standing, 'interpretation');
  assert.equal(instantiates.temporal_facet_ref, null, 'the trans-temporal relation carries no manufactured date');
});

test('the TB0 specimen still fails closed on unknown fields (mirror law untouched)', () => {
  const mutated = structuredClone(tb0);
  mutated.canvas_view = { invented: true };
  const checked = validateReading(mutated);
  assert.equal(checked.valid, false);
  assert.ok(checked.errors.some((error) => error.includes('canvas_view')));
});

test('layout opens the TB0 whole with exact refs and the warranted QL scheme', () => {
  const layout = computeLayout(tb0);
  assert.equal(layout.scheme, 'ql-constellation', 'the warranted ql facet engages the QL constellation scheme');
  assert.equal(layout.whole_ref, tb0.whole.whole_ref);
  const placedRefs = layout.nodes.map((node) => node.ref);
  for (const member of tb0.whole.member_refs) assert.ok(placedRefs.includes(member), `member ${member} is placed`);
  assert.equal(layout.edges.length, tb0.whole.relations.length, 'every disclosed relation is drawn, vocabulary verbatim');
  assert.deepEqual(
    layout.edges.map((edge) => edge.relation).sort(),
    ['INSTANTIATES', 'companion', 'implemented-in', 'supersedes'],
    'no relation type is relabelled or dropped',
  );
  // The INSTANTIATES target (the Vāk composition) is not a whole member: it
  // stays honestly adrift rather than being invented into the member set.
  const placed = new Set(placedRefs);
  assert.ok(!placed.has('ql:vak:composition:l5-para-vak'));
});

test('the TB0 cut and agency state is legible to the canvas lane', () => {
  assert.deepEqual(
    tb0.disclosure.application_cuts.map((cut) => cut.cut).sort(),
    ['3:3-conjugate', '4:2-deep'],
    'both application cuts are disclosed available',
  );
  const palace = tb0.disclosure.instruments.find((entry) => entry.instrument === 'palace');
  assert.equal(palace.available, false);
  assert.ok(palace.reason.length > 0, 'the unavailable instrument carries its reason');
  const canvasDegraded = tb0.disclosure.degraded.find((note) => note.instrument === 'canvas');
  assert.ok(canvasDegraded, 'canvas enters degraded: warranted layout, no authored view');
  const technēRoles = tb0.agency.filter((role) => role.role === 'techne');
  assert.deepEqual(technēRoles.map((role) => [role.m_index, role.instrument]), [[2, 'timeline']],
    'role bindings are carried verbatim — the specimen situates Technē_2 on timeline, none is re-pointed at canvas');
});

test('a TB0-shaped session with the TB0-1 session fields validates against the mirror', () => {
  const session = {
    contract: 'ql.techne/v1',
    session_ref: 'techne:disclosure-session:tb0-test',
    subject_ref: tb0.subject.subject_ref,
    selection: {
      selection_ref: 'ql.techne:selection:tb0-test',
      subject_ref: tb0.subject.subject_ref,
      reading_ref: tb0.reading_ref,
      instrument: 'canvas',
    },
    instrument: 'canvas',
    application_cut: '4:2-deep',
    whole_ref: tb0.whole.whole_ref,
    occasion_ref: 'ql.techne:occasion:fixture:tb0',
    return_target_ref: 'ql.techne:whole:fixture:l5-techne-contract-ground',
    reading_ref: tb0.reading_ref,
  };
  const checked = validateSession(session);
  assert.deepEqual(checked, { valid: true, errors: [] }, JSON.stringify(checked.errors));
});
