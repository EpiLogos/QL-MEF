import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { createServer } from 'vite';

const journey = fileURLToPath(new URL('../src/techne/journey/', import.meta.url));

test('the journey modules persist nothing: no storage APIs appear outside the one presence owner', async () => {
  // presence.ts is the lane's ONE persisted PRESENTATION-state owner (the
  // persist.ts discipline): position and draft across reload/re-entry.
  // Every other module — including the surface — holds no storage API at
  // all: drafts stay local until routed, and the Expression substrate stays
  // the only SEMANTIC store.
  const storageFree = ['beats.ts', 'sequence.ts', 'crossing.ts', 'compose.ts', 'vak.ts', 'agent.ts', 'proving-reading.ts', 'JourneyInstrument.tsx', 'register.ts'];
  for (const name of storageFree) {
    const source = await readFile(`${journey}${name}`, 'utf8');
    assert.doesNotMatch(source, /localStorage|sessionStorage|indexedDB|openDatabase|caches\./,
      `${name} must hold no persistence API — drafts are local until routed, and the Expression substrate stays the only store`);
  }
  const presence = await readFile(`${journey}presence.ts`, 'utf8');
  assert.match(presence, /localStorage/, 'presence.ts is the single storage owner');
  const serialises = presence.match(/interface JourneyPresence \{[\s\S]*?\n\}/);
  assert.ok(serialises, 'the presence shape is declared');
  assert.doesNotMatch(serialises[0], /temporal|selector|relations|provenance|disclosure/,
    'presence stores refs and presentation values, never semantic objects');
});

test('the journey derivation modules keep no shadow store: beats, sequence, crossing, compose, vak, agent are pure', async () => {
  for (const name of ['beats.ts', 'sequence.ts', 'crossing.ts', 'compose.ts', 'vak.ts', 'agent.ts']) {
    const source = await readFile(`${journey}${name}`, 'utf8');
    assert.doesNotMatch(source, /\bfetch\s*\(|new\s+Date|crypto\.|document\.|window\./,
      `${name} must stay pure — no I/O, no clock, no DOM`);
  }
});

test('the agent state module never scrapes: no DOM, no window access, JSON-safe publication only', async () => {
  const source = await readFile(`${journey}agent.ts`, 'utf8');
  assert.doesNotMatch(source, /querySelector|getElementById|innerHTML|outerHTML|getBoundingClientRect/,
    'agency state is structured material, never DOM extraction');
});

test('registerJourneySurface mounts the journey surface through the Technē registry and unregisters cleanly', async () => {
  const cradle = fileURLToPath(new URL('..', import.meta.url));
  const server = await createServer({ root: cradle, server: { middlewareMode: true }, appType: 'custom', logLevel: 'error' });
  try {
    const registry = await server.ssrLoadModule('/src/techne/registry.tsx');
    const { registerJourneySurface } = await server.ssrLoadModule('/src/techne/journey/register.ts');
    assert.equal(registry.techneSurface('journey'), undefined, 'nothing is mounted before the lane registers');
    const stop = registerJourneySurface();
    const surface = registry.techneSurface('journey');
    assert.ok(surface, 'the journey instrument is mounted');
    assert.throws(() => registerJourneySurface(), /already registered/, 'one surface per instrument');
    stop();
    assert.equal(registry.techneSurface('journey'), undefined, 'unregistration restores the honest placeholder');
  } finally {
    await server.close();
  }
});

test('the journey surface module compiles through the vite SSR pipeline with the new bridge modules', async () => {
  const cradle = fileURLToPath(new URL('..', import.meta.url));
  const server = await createServer({ root: cradle, server: { middlewareMode: true }, appType: 'custom', logLevel: 'error' });
  try {
    for (const name of ['crossing.ts', 'presence.ts', 'compose.ts', 'vak.ts', 'agent.ts', 'proving-reading.ts']) {
      const mod = await server.ssrLoadModule(`/src/techne/journey/${name}`);
      assert.ok(mod, `${name} loads`);
    }
  } finally {
    await server.close();
  }
});
