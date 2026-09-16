import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { createServer } from 'vite';

const project = fileURLToPath(new URL('../src/techne/project/', import.meta.url));

test('the project modules persist nothing: no storage APIs appear anywhere in the lane', async () => {
  const modules = ['ground.ts', 'projections.ts', 'agency.ts', 'ProjectGround.tsx', 'register.ts'];
  for (const name of modules) {
    const source = await readFile(`${project}${name}`, 'utf8');
    assert.doesNotMatch(source, /localStorage|sessionStorage|indexedDB|openDatabase|caches\./,
      `${name} must hold no persistence API — presentation state stays component-local and the ground owns no store`);
  }
});

test('the projection and agency models are pure: no I/O, no clock, no randomness, no DOM', async () => {
  for (const name of ['ground.ts', 'projections.ts', 'agency.ts']) {
    const source = await readFile(`${project}${name}`, 'utf8');
    assert.doesNotMatch(source, /\bfetch\s*\(|new\s+Date|Date\.now|crypto\.|Math\.random|document\.|window\./,
      `${name} must stay pure — the same reading and query always yield the same view`);
  }
});

test('the surface mints no stochastic identity and never fabricates a QL coordinate', async () => {
  const surface = await readFile(`${project}ProjectGround.tsx`, 'utf8');
  assert.doesNotMatch(surface, /crypto\.randomUUID|Math\.random|Date\.now/, 'the surface mints identity or time');
  assert.doesNotMatch(surface, /ql:structural|mef:lens:/, 'the surface must never mint a QL coordinate or lens — warranted readings only');
});

test('registerProjectSurface mounts the M0′ instrument through the Technē registry and unregisters cleanly', async () => {
  const cradle = fileURLToPath(new URL('..', import.meta.url));
  const server = await createServer({ root: cradle, server: { middlewareMode: true }, appType: 'custom', logLevel: 'error' });
  try {
    const registry = await server.ssrLoadModule('/src/techne/registry.tsx');
    const projectModule = await server.ssrLoadModule('/src/techne/project/register.ts');
    assert.equal(registry.techneSurface('project'), undefined, 'nothing is mounted before the lane registers');
    const stop = projectModule.registerProjectSurface();
    const surface = registry.techneSurface('project');
    assert.ok(surface, 'the M0′ instrument is mounted');
    assert.throws(() => projectModule.registerProjectSurface(), /already registered/, 'one surface per instrument');
    stop();
    assert.equal(registry.techneSurface('project'), undefined, 'unregistration restores the honest placeholder');
  } finally {
    await server.close();
  }
});

test('the whole M0′ proving leg over the TB0 specimen: ground → bounded view → selection → cross-open → routed action', async () => {
  const cradle = fileURLToPath(new URL('..', import.meta.url));
  const server = await createServer({ root: cradle, server: { middlewareMode: true }, appType: 'custom', logLevel: 'error' });
  try {
    const contract = await server.ssrLoadModule('/src/techne/contract.ts');
    const adapter = await server.ssrLoadModule('/src/techne/adapter.ts');
    const sessionModule = await server.ssrLoadModule('/src/techne/session.ts');
    const projections = await server.ssrLoadModule('/src/techne/project/projections.ts');
    const agency = await server.ssrLoadModule('/src/techne/project/agency.ts');
    const { readFile: read } = await import('node:fs/promises');
    const repoRoot = resolve(cradle, '..', '..', '..', '..');
    const specimen = JSON.parse(await read(resolve(repoRoot, 'fixtures/techne/tb0-connective-base-v1.json'), 'utf8'));

    // A registered source serves the specimen — the same spine a live
    // provider feeds when one is wired (the fixture stands in here).
    const source = adapter.createTechneSource({ ref: 'fixture:m0-proving', title: 'TB0 specimen', read: async () => specimen });
    const stopSource = adapter.registerTechneSource(source);
    try {
      const reading = await source.reading(specimen.subject.subject_ref);
      assert.equal(reading.reading_ref, specimen.reading_ref);

      // Real Project → whole → subject spine over the ONE session.
      const store = sessionModule.createDisclosureSessionStore();
      const session = store.setSelection({
        selection_ref: 'ql.techne:selection:m0-proving',
        subject_ref: specimen.subject.subject_ref,
        reading_ref: specimen.reading_ref,
        instrument: 'project',
        agent_session_ref: 'aikit:agent-session:fixture:l5-techne',
      });
      assert.equal(session.application_cut, '4:2-deep');

      // Bounded local whole from the reading's own focus, LIST/TREE/GRAPH same refs.
      const query = projections.projectQuery(reading);
      const view = projections.boundedView(reading, query);
      const listRefs = projections.listProjection(view).map((row) => row.ref).sort();
      const graphRefs = projections.graphProjection(view).nodes.map((node) => node.ref).sort();
      assert.deepEqual(listRefs, graphRefs);

      // A click moves the selection on the same subject/basis and records no hop.
      const target = view.nodes.find((node) => node.member && node.distance === 1).ref;
      const clicked = projections.focusSelection(reading, session.selection, target);
      const moved = store.setSelection(clicked);
      assert.equal(moved.subject_ref, session.subject_ref);
      assert.equal(moved.session_ref, session.session_ref);
      assert.equal(contract.validateSession(moved).valid, true);

      // Cross-open into M1′ Canvas and back — one session, two hops, same subject.
      const canvasSession = store.openInInstrument('canvas');
      assert.equal(canvasSession.application_cut, '4:2-deep');
      const backSession = store.openInInstrument('project');
      assert.equal(backSession.subject_ref, session.subject_ref);
      assert.equal(backSession.navigation.length, 2);
      assert.deepEqual(backSession.navigation.map((hop) => hop.to_instrument), ['canvas', 'project']);

      // Cross into the conjugate 3:3 reading: the cut follows the instrument.
      const expressionSession = store.openInInstrument('expressions');
      assert.equal(expressionSession.application_cut, '3:3-conjugate');
      assert.equal(expressionSession.subject_ref, session.subject_ref, 'cut crossing preserves the subject');

      // Technē_0 path: locate → traverse → route a disclosed action → owner receipt.
      const state = agency.technaeState(reading, backSession, view);
      const located = agency.agencyLocate(state, reading);
      assert.equal(located.subject_ref, specimen.subject.subject_ref);
      const traverse = agency.agencyTraverse(state, view, located.subject_ref);
      assert.equal(traverse.focus_ref, located.subject_ref);
      const route = agency.agencyRoute(state, 'central.day.read');
      const receipt = adapter.resolveActionRoute(reading, route);
      assert.equal(receipt.routed, true);
      assert.equal(receipt.native_owner, 'central/ctrl');
    } finally {
      stopSource();
    }
  } finally {
    await server.close();
  }
});
