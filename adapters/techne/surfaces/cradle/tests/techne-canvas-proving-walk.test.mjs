import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { computeLayout, applyManualOverrides } from '../src/techne/canvas/layout.ts';
import { createCanvasView, applyViewToLayout, createCanvasViewStore } from '../src/techne/canvas/view.ts';
import { createDisclosureSessionStore, coReferenced } from '../src/techne/session.ts';
import { buildExpressionCue } from '../src/techne/expressions/cue.ts';
import { embody } from '../src/techne/expressions/embody.ts';

const here = fileURLToPath(new URL('.', import.meta.url));
const tb0 = JSON.parse(
  await readFile(new URL('../../../../../fixtures/techne/tb0-connective-base-v1.json', 'file://' + here), 'utf8'),
);

// --- The M1′ proving walk, fixture-backed -----------------------------------

test('walk: open the TB0 whole into Canvas with exact refs, basis and session co-reference', () => {
  const store = createDisclosureSessionStore();
  const session = store.setSelection({
    selection_ref: `ql.techne:selection:${tb0.subject.subject_ref}:${tb0.whole.whole_ref}`,
    subject_ref: tb0.subject.subject_ref,
    reading_ref: tb0.reading_ref,
    instrument: 'canvas',
    snapshot_revision: tb0.snapshot.revision ?? undefined,
  });
  assert.equal(session.instrument, 'canvas');
  assert.equal(session.subject_ref, tb0.subject.subject_ref);
  const layout = computeLayout(tb0);
  assert.equal(layout.whole_ref, tb0.whole.whole_ref);
  assert.equal(layout.scheme, 'ql-constellation');
});

test('walk: rearrange/group/frame without semantic mutation — the reading and its relations are byte-stable', () => {
  const before = JSON.stringify(tb0);
  const layout = computeLayout(tb0);
  const arranged = applyManualOverrides(layout, {
    [tb0.whole.member_refs[0]]: { x: 0.2, y: 0.2 },
    [tb0.whole.member_refs[1]]: { x: 0.25, y: 0.2 },
    [tb0.whole.member_refs[2]]: { x: 0.3, y: 0.2 },
  });
  const view = createCanvasView(tb0, {
    placements: Object.entries({
      [tb0.whole.member_refs[0]]: { x: 0.2, y: 0.2 },
      [tb0.whole.member_refs[1]]: { x: 0.25, y: 0.2 },
      [tb0.whole.member_refs[2]]: { x: 0.3, y: 0.2 },
    }).map(([ref, at]) => ({ ref, x: at.x, y: at.y })),
    frames: [{ frame_ref: 'ql.techne:canvas-frame:walk', label: 'Row', member_refs: tb0.whole.member_refs.slice(0, 3), z: 1 }],
  });
  const reapplied = applyViewToLayout(view, computeLayout(tb0));
  assert.equal(Object.keys(reapplied.overrides).length, 3);
  assert.deepEqual(reapplied.dangling_refs, []);
  assert.equal(JSON.stringify(tb0), before, 'arrangement changed nothing semantic');
  assert.equal(tb0.whole.relations.length, 4, 'relation set untouched by framing');
});

test('walk: cross-open to M2′ and back — subject, source basis and selection preserved', () => {
  const store = createDisclosureSessionStore();
  const open = store.setSelection({
    selection_ref: `ql.techne:selection:${tb0.subject.subject_ref}:${tb0.whole.member_refs[0]}`,
    subject_ref: tb0.subject.subject_ref,
    reading_ref: tb0.reading_ref,
    instrument: 'canvas',
    source_ref: tb0.provenance[0].source_ref,
    agent_session_ref: tb0.temporal.find((facet) => facet.kind === 'session')?.session_ref,
    selection_standing: 'current',
  });
  const inTimeline = store.openInInstrument('timeline');
  assert.equal(inTimeline.instrument, 'timeline');
  assert.equal(inTimeline.subject_ref, open.subject_ref);
  assert.equal(inTimeline.reading_ref, open.reading_ref);
  assert.equal(inTimeline.selection.selection_ref, open.selection.selection_ref);
  assert.equal(inTimeline.selection.agent_session_ref, open.selection.agent_session_ref);
  assert.equal(coReferenced(open, inTimeline), true);
  const back = store.openInInstrument('canvas');
  assert.equal(back.instrument, 'canvas');
  assert.deepEqual(back.navigation.map((hop) => [hop.from_instrument, hop.to_instrument]), [['canvas', 'timeline'], ['timeline', 'canvas']]);
  assert.equal(coReferenced(open, back), true);
});

test('walk: M1′ cross-cut — the 3:3 Expression reading composes through the lane’s public entry points', async () => {
  const store = createDisclosureSessionStore();
  const session = store.setSelection({
    selection_ref: `ql.techne:selection:${tb0.subject.subject_ref}`,
    subject_ref: tb0.subject.subject_ref,
    reading_ref: tb0.reading_ref,
    instrument: 'canvas',
  });
  const expressionsEntry = tb0.disclosure.instruments.find((entry) => entry.instrument === 'expressions');
  assert.equal(expressionsEntry?.available, true, 'the reading discloses the 3:3 crossing as available');
  const crossed = store.openInInstrument('expressions');
  assert.equal(crossed.subject_ref, session.subject_ref, 'crossing the cut changes medium, not subject');
  assert.equal(crossed.reading_ref, session.reading_ref);
  assert.equal(coReferenced(session, crossed), true);
  // The Expression bridge's public seam (lane G directory, consumed not modified):
  // the cue carries the same subject/reading basis, the scene focus from the
  // reading's own Expression binding, and the warranted QL cues.
  const cue = buildExpressionCue(tb0, crossed.selection);
  assert.equal(cue.subject.subject_ref, tb0.subject.subject_ref);
  assert.equal(cue.ql?.shape_ref, tb0.ql.shape_ref);
  assert.equal(cue.scene_focus?.scene_ref, tb0.expressions[0].scene_ref);
  assert.equal(cue.scene_focus?.expression_ref, tb0.expressions[0].expression_ref);
  // And back: the Return into the deep field keeps the same subject.
  const returned = store.openInInstrument('canvas');
  assert.equal(returned.subject_ref, tb0.subject.subject_ref);
  // The embodiment gate answers for this reading without fabricating availability.
  const gate = embody(tb0, returned.selection);
  assert.ok(gate, 'the embodiment gate composes over the returned selection');
});

// --- Performance and lifecycle ------------------------------------------------

function syntheticWhole(members, relationsPerMember) {
  const refs = Array.from({ length: members }, (_, index) => `central:source:control:root:Work/Quaternal-Logic/docs/generated-${index}.md`);
  const relations = [];
  for (let i = 0; i < members; i += 1) {
    for (let r = 1; r <= relationsPerMember; r += 1) {
      relations.push({
        relation: r % 2 === 0 ? 'companion' : 'supersedes',
        relation_ref: `ql.techne:relation:gen:${i}:${r}`,
        from_ref: refs[i],
        to_ref: refs[(i + r) % members],
        origin: 'authored',
        origin_ref: null,
        standing: 'fact',
        source_ref: refs[0],
        evidence_refs: [],
        temporal_facet_ref: null,
        derivation_ref: null,
        confidence: null,
      });
    }
  }
  return { ...tb0, whole: { ...tb0.whole, member_refs: refs, relations } };
}

test('walk: realistic bounded field — 300 members, 600 typed relations layout and save under a generous budget', () => {
  const big = syntheticWhole(300, 2);
  const start = performance.now();
  const layout = computeLayout(big);
  const arranged = applyManualOverrides(layout, Object.fromEntries(
    big.whole.member_refs.slice(0, 50).map((ref, index) => [ref, { x: 0.01 * index, y: 0.02 * index }]),
  ));
  const view = createCanvasView(big, {
    placements: arranged.nodes.slice(0, 50).map((node) => ({ ref: node.ref, x: node.x, y: node.y })),
  });
  const reapplied = applyViewToLayout(view, computeLayout(big));
  const elapsed = performance.now() - start;
  assert.equal(layout.nodes.length, 301, 'whole + every member, bounded, no hairball');
  assert.equal(layout.edges.length, 600);
  assert.equal(Object.keys(reapplied.overrides).length, 50);
  assert.ok(elapsed < 500, `layout/arrange/save/apply took ${elapsed.toFixed(1)}ms — well within the generous 500ms budget`);
});

test('lifecycle: saved views and proposals release listeners on unsubscribe — nothing keeps the surface alive', () => {
  const views = createCanvasViewStore();
  let viewNotices = 0;
  const offViews = views.subscribe(() => { viewNotices += 1; });
  views.save(createCanvasView(tb0));
  assert.equal(viewNotices, 1);
  offViews();
  views.save(createCanvasView(tb0));
  assert.equal(viewNotices, 1, 'an unsubscribed listener receives nothing — the store holds no dead subscription');
  assert.equal(views.views().length, 2);
});

test('lifecycle: a fresh surface instance (new store) sees no residue from another session’s artifacts', () => {
  const isolated = createCanvasViewStore();
  isolated.save(createCanvasView(tb0, { placements: [{ ref: tb0.whole.member_refs[0], x: 0.4, y: 0.4 }] }));
  assert.equal(isolated.views().length, 1, 'component-scoped presentation state starts clean per surface lifetime');
});
