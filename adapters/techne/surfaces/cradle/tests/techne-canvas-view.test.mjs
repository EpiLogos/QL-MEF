import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { createCanvasView, parseView, serializeView, validateCanvasView, viewDrift, applyViewToLayout, bringFrameToFront, sendFrameToBack, framesInDrawOrder, createCanvasViewStore } from '../src/techne/canvas/view.ts';
import { computeLayout, applyManualOverrides } from '../src/techne/canvas/layout.ts';
import { validateReading } from '../src/techne/contract.ts';

const here = fileURLToPath(new URL('.', import.meta.url));
const tb0 = JSON.parse(
  await readFile(new URL('../../../../../fixtures/techne/tb0-connective-base-v1.json', 'file://' + here), 'utf8'),
);

// A no-warrant reading (T0 absent-facets fixture, byte-exact copy in the
// cradle fixture dir) for the honest radial basis.
const absentFacets = JSON.parse(await readFile(new URL('../src/techne/fixtures/absent-facets-v1.json', 'file://' + here), 'utf8'));
assert.equal(validateReading(absentFacets).valid, true);

test('an authored view captures refs and presentation only — no field for semantics', () => {
  const view = createCanvasView(tb0, {
    placements: [{ ref: tb0.whole.member_refs[0], x: 0.1, y: -0.2 }],
    frames: [{ frame_ref: 'ql.techne:canvas-frame:f1', label: 'The sources', member_refs: tb0.whole.member_refs.slice(0, 2), z: 1 }],
  });
  assert.equal(view.subject_ref, tb0.subject.subject_ref);
  assert.equal(view.reading_ref, tb0.reading_ref);
  assert.deepEqual(view.layout_basis.scheme, 'ql-constellation');
  assert.equal(view.layout_basis.warranted_ql?.shape_ref, tb0.ql.shape_ref, 'the warrant refs ride verbatim');
  assert.equal(viewAddressedHasRelationField(view), false, 'structurally: a view cannot hold a typed relation');
  const json = JSON.stringify(serializeView(view));
  assert.ok(!json.includes(tb0.whole.relations[0].relation), 'no relation vocabulary leaks into the artifact');
});

function viewAddressedHasRelationField(view) {
  return 'relations' in view || 'member_refs' in view;
}

test('the layout basis is honest: no warrant → radial, never QL standing', () => {
  const view = createCanvasView(absentFacets);
  assert.equal(view.layout_basis.scheme, 'radial');
  assert.equal(view.layout_basis.warranted_ql, null);
});

test('a QL scheme without warrant refs is refused, and view refs stay in the presentation namespace', () => {
  const forged = { ...createCanvasView(tb0), view_ref: 'aikit:wiki:some-subject' };
  const checked = validateCanvasView(forged);
  assert.equal(checked.valid, false);
  assert.ok(checked.errors.some((error) => error.includes('namespace')));

  const unbasisd = { ...createCanvasView(tb0), layout_basis: { ...createCanvasView(tb0).layout_basis, warranted_ql: null } };
  assert.equal(validateCanvasView(unbasisd).valid, false, 'ql-constellation scheme demands its warranted refs');

  const unschemed = { ...createCanvasView(absentFacets), layout_basis: { scheme: 'ql-constellation', warranted_ql: null } };
  assert.equal(validateCanvasView(unschemed).valid, false, 'a radial reading cannot claim the QL scheme');

  const smuggled = { ...createCanvasView(absentFacets), relations: [{ relation: 'CAUSES', from_ref: 'a', to_ref: 'b' }] };
  assert.equal(validateCanvasView(smuggled).valid, false, 'semantic fields are unknown keys — refused');
});

test('applyViewToLayout layers placements, reports dangling refs, never invents nodes', () => {
  const view = createCanvasView(tb0, {
    placements: [
      { ref: tb0.whole.member_refs[0], x: 0.2, y: 0.3 },
      { ref: 'central:source:control:root:Work/Quaternal-Logic/docs/removed-later.md', x: 0.9, y: 0.9 },
    ],
  });
  const layout = computeLayout(tb0);
  const before = JSON.stringify(layout);
  const applied = applyViewToLayout(view, layout);
  assert.deepEqual(JSON.parse(JSON.stringify(layout)), JSON.parse(before), 'the layout input is untouched');
  assert.equal(applied.overrides[tb0.whole.member_refs[0]].x, 0.2);
  assert.deepEqual(applied.dangling_refs, ['central:source:control:root:Work/Quaternal-Logic/docs/removed-later.md'], 'drift is reported');
  const recombined = applyManualOverrides(applied.layout, applied.overrides);
  assert.equal(recombined.nodes.length, layout.nodes.length, 'no node was invented');
});

test('viewDrift reports what a refreshed reading no longer discloses', () => {
  const view = createCanvasView(tb0, { placements: [{ ref: tb0.whole.member_refs[0], x: 0, y: 0 }] });
  const pruned = structuredClone(tb0);
  pruned.whole.member_refs = pruned.whole.member_refs.slice(1);
  const drift = viewDrift(view, pruned);
  assert.deepEqual(drift.missing_refs, [tb0.whole.member_refs[0]]);
});

test('frames round-trip through serialize/parse and z-order is presentation', () => {
  const base = createCanvasView(tb0, {
    frames: [
      { frame_ref: 'ql.techne:canvas-frame:a', label: 'A', member_refs: [tb0.whole.member_refs[0]], z: 0 },
      { frame_ref: 'ql.techne:canvas-frame:b', label: 'B', member_refs: [tb0.whole.member_refs[1]], z: 5 },
    ],
  });
  const front = bringFrameToFront(base, 'ql.techne:canvas-frame:a');
  assert.equal(front.frames.find((frame) => frame.frame_ref === 'ql.techne:canvas-frame:a').z, 6);
  const back = sendFrameToBack(front, 'ql.techne:canvas-frame:b');
  assert.equal(back.frames.find((frame) => frame.frame_ref === 'ql.techne:canvas-frame:b').z, -1);
  assert.deepEqual(framesInDrawOrder(back).map((frame) => frame.frame_ref), ['ql.techne:canvas-frame:b', 'ql.techne:canvas-frame:a']);

  const round = parseView(serializeView(back));
  assert.deepEqual(round, back);
});

test('an authored view never mutates the reading it was authored over', () => {
  const before = JSON.stringify(tb0);
  createCanvasView(tb0, { placements: [{ ref: tb0.whole.member_refs[2], x: 5, y: 5 }] });
  applyViewToLayout(createCanvasView(tb0), computeLayout(tb0));
  assert.equal(JSON.stringify(tb0), before);
});

test('the presentation view store saves, replaces, deletes and notifies; unsubscribing leaks nothing', () => {
  const store = createCanvasViewStore();
  let notifications = 0;
  const off = store.subscribe(() => { notifications += 1; });
  const view = createCanvasView(tb0);
  store.save(view);
  store.save({ ...view, view_ref: view.view_ref });
  assert.equal(store.views().length, 1, 'same view_ref replaces, not duplicates');
  const other = createCanvasView(tb0);
  store.save(other);
  assert.equal(store.forSubject(tb0.subject.subject_ref).length, 2);
  assert.equal(store.delete(other.view_ref), true);
  assert.equal(store.delete(other.view_ref), false);
  off();
  store.save(createCanvasView(tb0));
  assert.equal(notifications, 4, 'notifications stop after unsubscribe — no listener leak');
});

test('parse refuses a corrupt artifact instead of best-effort repairing it', () => {
  assert.throws(() => parseView({ view_ref: 'ql.techne:canvas:x' }), /invalid/);
  assert.throws(() => parseView({ ...serializeView(createCanvasView(tb0)), viewport: { x: 'NaN', y: 0, scale: 1 } }), /viewport/);
});
