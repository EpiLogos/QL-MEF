import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { createServer } from 'vite';
import { createSurfaceLifecycle } from '@epilogos/oi-design-system/techne-surface-lifecycle';

// The host-closure modules use the O:I extensionless import style, so they
// load through the vite SSR transform like the other surface tests.
const cradle = fileURLToPath(new URL('..', import.meta.url));
const server = await createServer({ root: cradle, server: { middlewareMode: true }, appType: 'custom', logLevel: 'error' });
try {
  var { surfaceExpressionChanges, makeTestBinding, openBinding, splitOff, closeSurface } =
    await server.ssrLoadModule('/src/surface/engine.ts');
  var { freshLayout } = await server.ssrLoadModule('/src/surface/types.ts');
  var { resetFocusedInstrumentSources, focusedInstrumentSource } =
    await server.ssrLoadModule('/src/instrument/source.ts');
  // Loaded from the SAME server instance: the registration must write the
  // K9 registry this test reads, not a parallel module instance.
  var { registerTechneFocusedInstrumentSource } =
    await server.ssrLoadModule('/src/techne/expressions/register.ts');
} finally {
  await server.close();
}

// ---------------------------------------------------------------------------
// A controlled frame clock: the lifecycle module takes its scheduler and
// document by injection, so the hidden/suspend/resume law is provable here
// without a real renderer. fire() passes rAF timestamps relative to the real
// clock so delta gating is exercised honestly.
// ---------------------------------------------------------------------------

function fakeFrameClock() {
  const state = { cb: null, cancelled: 0, scheduled: 0 };
  const scheduleFrame = (cb) => { state.cb = cb; state.scheduled += 1; return state.scheduled; };
  const cancelFrame = () => { if (state.cb) { state.cb = null; state.cancelled += 1; } };
  const doc = {
    hidden: false,
    onvis: null,
    addEventListener(_kind, fn) { this.onvis = fn; },
    removeEventListener() { this.onvis = null; },
  };
  const fire = (offsetMs = 16) => { const cb = state.cb; state.cb = null; cb?.(performance.now() + offsetMs); };
  return { state, scheduleFrame, cancelFrame, doc, fire };
}

test('X3.5 — the surface lifecycle releases on hide and never double-acquires across crossings', () => {
  const { state, scheduleFrame, cancelFrame, doc, fire } = fakeFrameClock();
  const acquired = [];
  const released = [];
  const rendered = [];
  const lifecycle = createSurfaceLifecycle({
    acquire: () => acquired.push(1),
    release: () => released.push(1),
    render: (delta) => rendered.push(delta),
    needsFrame: () => false,
    document: doc,
    requestAnimationFrame: scheduleFrame,
    cancelAnimationFrame: cancelFrame,
  });

  assert.equal(lifecycle.state, 'released');
  lifecycle.start();
  assert.equal(lifecycle.state, 'active');
  assert.equal(acquired.length, 1);
  fire(); // one frame
  assert.equal(rendered.length, 1, 'a dirty surface repaints once');

  // The surface is hidden (the person crossed to the other reading): the
  // loop suspends, cancels the pending frame, and releases nothing yet.
  doc.hidden = true;
  doc.onvis();
  assert.equal(lifecycle.state, 'suspended');
  assert.equal(state.cancelled, 1, 'no hidden frame keeps running');
  assert.equal(released.length, 0, 'suspension is not teardown');
  fire();
  assert.equal(rendered.length, 1, 'a hidden surface advances nothing');

  // Back in focus: resume re-bases, resources are shared, not re-acquired.
  doc.hidden = false;
  doc.onvis();
  assert.equal(lifecycle.state, 'active');
  assert.equal(acquired.length, 1, 're-focus reuses the live resources');

  // Terminal dispose releases exactly once, idempotently.
  lifecycle.dispose();
  lifecycle.dispose();
  assert.equal(lifecycle.state, 'released');
  assert.equal(released.length, 1, 'renderer resources release exactly once');
});

test('X3.5 — a paused surface repaints at delta 0; a playing surface advances bounded steps', () => {
  const { state, scheduleFrame, cancelFrame, doc, fire } = fakeFrameClock();
  const rendered = [];
  const lifecycle = createSurfaceLifecycle({
    render: (delta) => rendered.push(delta),
    document: doc,
    requestAnimationFrame: scheduleFrame,
    cancelAnimationFrame: cancelFrame,
  });
  lifecycle.start();
  lifecycle.markFrameNeeded();
  fire(60000); // a background stall of a minute
  assert.equal(rendered[0], 0, 'a stalled/paused surface repaints once with zero delta — no jump');
  lifecycle.setPlaying(true);
  fire(60000);
  assert.ok(rendered[1] > 0 && rendered[1] <= 0.05, `a playing surface advances at most one bounded step (got ${rendered[1]})`);
  lifecycle.dispose();
});

test('X3.5 — repeated cut crossings keep one instrument host: register, cross away, return, still one', () => {
  resetFocusedInstrumentSources();
  const owner = {
    ref: 'ql:k9:owner-lifecycle',
    title: 'Focused instrument (owner stand-in)',
    read: async () => ({ event: { event_ref: 'e', subject_ref: 's:1', profile_generation: 1 } }),
    readBimba: async () => ({ standing: 'owner' }),
    command: async (command) => ({ standing: 'applied', operation: command.kind }),
  };
  // A scene binding makes the subject embodyable through the gate.
  const reading = {
    contract: 'ql.techne/v1',
    reading_ref: 'r',
    subject: { subject_ref: 's:1', native_owner: 'central' },
    expressions: [{ expression_ref: 'expression:s:1', scene_ref: 'expression:s:1:scene:main' }],
    disclosure: { instruments: [] },
  };
  const selection = { selection_ref: 'sel', subject_ref: 's:1', reading_ref: 'r', instrument: 'expressions' };

  // Opening the Expression reading registers the embodiment alias.
  const first = registerTechneFocusedInstrumentSource({ reading, selection, owner });
  assert.equal(first.registered, true);
  assert.ok(focusedInstrumentSource('ql.techne:source:s:1'));
  // Crossing away releases the alias (the host unregisters on surface close).
  first.unregister();
  assert.equal(focusedInstrumentSource('ql.techne:source:s:1'), undefined, 'nothing lives invisibly behind the crossed surface');
  // Crossing back re-registers cleanly — one host, never a second.
  const second = registerTechneFocusedInstrumentSource({ reading, selection, owner });
  assert.equal(second.registered, true);
  assert.ok(focusedInstrumentSource('ql.techne:source:s:1'));
  second.unregister();
  assert.equal(focusedInstrumentSource('ql.techne:source:s:1'), undefined);
  resetFocusedInstrumentSources();
});

test('X3.5 — the layout engine emits the split/close intents a host releases on', () => {
  let layout = freshLayout();
  const a = makeTestBinding(layout, 'test');
  layout = openBinding(layout, a);
  const b = makeTestBinding(layout, 'test');
  layout = openBinding(layout, b);
  // Split the focused surface beside (the workbench grammar), then close it.
  const split = splitOff(layout, b.id, 'h');
  const splitChanges = surfaceExpressionChanges(layout, split);
  assert.ok(splitChanges.some((change) => change.intent === 'split'), 'splitting beside is visible to the presentation host');
  const closed = closeSurface(split, b.id);
  const closeChanges = surfaceExpressionChanges(split, closed);
  assert.ok(closeChanges.some((change) => change.intent === 'close'), 'closing the crossed-out surface emits the release intent');
  assert.equal(closeChanges.some((change) => change.intent === 'open'), false, 'a close emits no open');
});

test('the expressions lane grows no renderer of its own — the Global Expression Stage stays the one host', async () => {
  const expressions = fileURLToPath(new URL('../src/techne/expressions/', import.meta.url));
  for (const name of ['TechneExpressionBridge.tsx', 'cue.ts', 'embody.ts', 'register.ts']) {
    const source = await readFile(`${expressions}${name}`, 'utf8');
    assert.doesNotMatch(source, /requestAnimationFrame|setInterval/,
      `${name} must not grow an animation or timer loop — one renderer per active surface`);
    assert.doesNotMatch(source, /createElement\(["'`]canvas|getContext\(|new\s+THREE|WebGL/,
      `${name} must not create a renderer, canvas or WebGL context`);
    assert.doesNotMatch(source, /localStorage|sessionStorage|indexedDB|caches\./,
      `${name} must hold no persistence API`);
  }
});
