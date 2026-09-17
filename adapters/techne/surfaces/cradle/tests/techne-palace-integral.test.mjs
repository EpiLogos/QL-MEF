import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { loadFixture } from '../src/techne/fixtures/load-fixtures.mjs';
import { FixtureTechneAdapter } from '../src/techne/adapter.ts';
import {
  addBookmark,
  bindComposition,
  compositionRegions,
  deriveComposition,
  palaceBasis,
  palaceClaim,
  palaceComposition,
  palaceTraversal,
  placeAt,
  removeBookmark,
  resetPalaceCompositions,
  storeComposition,
} from '../src/techne/palace/palace-state.ts';
import { locusKey } from '../src/techne/palace/composition.ts';
import {
  arrangeRegion,
  defaultRegionPlacements,
  PALACE_REGION_ORDER,
  palaceRegions,
  regionArchaeology,
  regionOrder,
} from '../src/techne/palace/regions.ts';
import {
  palaceReturnAction,
  palaceReturnInput,
  palaceReturnRoute,
  returnCrossing,
} from '../src/techne/palace/return.ts';
import { palaceAgentState } from '../src/techne/palace/agent-state.ts';

const tb0 = await loadFixture('tb0-connective-base-v1.json');
const representative = await loadFixture('representative-subject-v1.json');
const absent = await loadFixture('absent-facets-v1.json');

test('the TB0 specimen composes regions from every disclosed ref family, in traversal order', () => {
  const regions = palaceRegions(tb0);
  assert.deepEqual(
    regions.map((region) => region.key),
    ['ground', 'canvas', 'relation', 'journey', 'world', 'expression', 'sources', 'agency'],
    'the canonical M0′→M5′ order, plus the sources/agency closes',
  );
  const byKey = new Map(regions.map((region) => [region.key, region]));
  // M0′: the bounded whole plus its six members.
  assert.equal(byKey.get('ground').m_prime, 0);
  assert.equal(byKey.get('ground').entries.length, 7);
  assert.ok(byKey.get('ground').entries.some((entry) => entry.ref === 'ql.techne:whole:fixture:l5-techne-contract-ground'));
  // M1′: the warranted constellation and shape.
  assert.equal(byKey.get('canvas').m_prime, 1);
  assert.deepEqual(
    byKey.get('canvas').entries.map((entry) => entry.ref).sort(),
    ['ql.techne:constellation:fixture:l5-dual-reading', 'ql:shape:1.0.0:6x6:direct-conjugate'],
  );
  // M2′: day, now, session, run, attempt and Return continuity refs.
  assert.deepEqual(
    byKey.get('relation').entries.map((entry) => entry.label).sort(),
    ['attempt', 'day', 'now', 'return', 'run', 'session'],
  );
  assert.ok(byKey.get('relation').entries.some((entry) => entry.ref === 'factory:run:01JBD3Z4X7Q9M2A6K4P8T3W5NQ#attempt-1'));
  assert.ok(byKey.get('relation').entries.some((entry) => entry.ref === 'factory:run:01JBD3Z4X7Q9M2A6K4P8T3W5NQ/return'));
  // M3′: the bound scene; M4′: both place readings with their native relations.
  assert.deepEqual(byKey.get('journey').entries.map((entry) => entry.ref), ['oi:expression:l5-dual-reading/scene/crossing']);
  assert.deepEqual(
    byKey.get('world').entries.map((entry) => entry.note),
    ['OCCURRED_AT · approximate', 'MYTH_LOCATED_AT · unlocated'],
  );
  // 3:3: the Expression binding, verbatim, with its composition ref named.
  assert.deepEqual(
    byKey.get('expression').entries.map((entry) => [entry.ref, entry.note]),
    [['oi:expression:l5-dual-reading', 'composition oi:composition:l5-dual-reading-integral']],
  );
  // Sources and agencies close the walk; agencies are co-referenced, not openable.
  assert.equal(byKey.get('sources').entries.length, 2);
  assert.ok(byKey.get('sources').entries.every((entry) => entry.open_instrument === null));
  assert.equal(byKey.get('agency').entries.length, 4);
  assert.ok(byKey.get('agency').entries.every((entry) => entry.open_instrument === null));
});

test('every region entry is a ref the reading itself discloses — nothing is minted', () => {
  const regions = palaceRegions(tb0);
  const disclosed = new Set([
    tb0.whole.whole_ref,
    ...(tb0.whole.member_refs ?? []),
    tb0.ql.shape_ref,
    tb0.ql.constellation_ref,
    ...(tb0.expressions ?? []).flatMap((binding) => [binding.expression_ref, binding.scene_ref].filter(Boolean)),
    ...(tb0.provenance ?? []).map((entry) => entry.source_ref),
  ]);
  for (const facet of tb0.temporal ?? []) {
    for (const carrier of ['day_ref', 'now_ref', 'session_ref', 'run_ref', 'attempt_ref', 'return_ref']) {
      if (facet[carrier]) disclosed.add(facet[carrier]);
    }
  }
  for (const place of tb0.spatial ?? []) disclosed.add(place.place_ref);
  for (const binding of tb0.agency ?? []) {
    for (const carrier of ['agent_session_ref', 'guardian_ref']) {
      if (binding[carrier]) disclosed.add(binding[carrier]);
    }
  }
  for (const region of regions) {
    for (const entry of region.entries) {
      assert.ok(disclosed.has(entry.ref), `${entry.ref} is not disclosed by the reading`);
    }
  }
});

test('architecture honesty: the canvas region stands in the warrant, the others say free composition', () => {
  const regions = palaceRegions(tb0);
  const byKey = new Map(regions.map((region) => [region.key, region]));
  const canvasArchaeology = regionArchaeology(tb0, byKey.get('canvas'));
  assert.match(canvasArchaeology, /canonical/);
  assert.match(canvasArchaeology, /ql-mef:provider:registry-disclosure/);
  assert.match(canvasArchaeology, /ql:shape:1\.0\.0:6x6:direct-conjugate/);
  const groundArchaeology = regionArchaeology(tb0, byKey.get('ground'));
  assert.match(groundArchaeology, /free mnemonic composition/);
  assert.match(groundArchaeology, /no warranted QL structure/);
  // A reading with no warranted QL facet never grows a pseudo-QL region.
  const representativeRegions = palaceRegions(representative);
  const representativeCanvas = representativeRegions.find((region) => region.key === 'canvas');
  assert.match(regionArchaeology(representative, representativeCanvas), /warranted QL reading/);
});

test('the composition store recovers state on re-entry within the session and scopes it by basis', () => {
  resetPalaceCompositions();
  const first = palaceComposition(tb0);
  assert.equal(first.bound, false, 'a fresh composition is unbound');
  const bound = bindComposition(tb0);
  assert.equal(bound.bound, true);
  const bookmarked = storeComposition(addBookmark(bound, 'ground', tb0.whole.whole_ref));
  assert.equal(bookmarked.bookmarks.length, 1);

  // Re-entry (a fresh surface mount over the same reading) recovers it.
  const reEntered = palaceComposition(tb0);
  assert.equal(reEntered.basis, palaceBasis(tb0));
  assert.equal(reEntered.bound, true, 'the binding survives re-entry');
  assert.deepEqual(reEntered.bookmarks, bookmarked.bookmarks, 'bookmarks survive re-entry');

  // A different basis never shares the composition.
  const other = palaceComposition(representative);
  assert.notEqual(other.basis, reEntered.basis);
  assert.equal(other.bound, false);
  resetPalaceCompositions();
  assert.equal(palaceComposition(tb0).bound, false, 'reset starts a clean session');
});

test('region arrangements are deterministic, movable, and pure over the composition', () => {
  const ground = palaceRegions(tb0).find((region) => region.key === 'ground');
  const first = defaultRegionPlacements(ground.entries);
  const second = defaultRegionPlacements(ground.entries);
  assert.deepEqual(first, second);
  assert.deepEqual(regionOrder(first).slice(0, 2), [tb0.whole.whole_ref, tb0.whole.member_refs[0]], 'reading order over the loci');

  const moved = arrangeRegion(first, tb0.whole.whole_ref, { room: { column: 2, row: 1 }, locus: 3 });
  assert.equal(locusKey(moved.find((placement) => placement.ref === tb0.whole.whole_ref).locus), '1:2:3');
  assert.deepEqual(regionOrder(moved).at(-1), tb0.whole.whole_ref);
  // The source list is untouched.
  assert.deepEqual(regionOrder(first).at(0), tb0.whole.whole_ref);

  const composition = deriveComposition(tb0);
  const placed = placeAt(composition, 'ground', tb0.whole.whole_ref, { room: { column: 2, row: 1 }, locus: 3 });
  assert.equal(placed.placements.ground.find((p) => p.ref === tb0.whole.whole_ref).locus.room.column, 2);
  assert.equal(composition.placements.ground.find((p) => p.ref === tb0.whole.whole_ref).locus.room.column, 0, 'the prior composition is untouched');
});

test('bookmarks are idempotent, removable, and carry the region with the ref', () => {
  const composition = deriveComposition(tb0);
  const once = addBookmark(composition, 'ground', tb0.whole.whole_ref);
  const twice = addBookmark(once, 'ground', tb0.whole.whole_ref);
  assert.equal(twice.bookmarks.length, 1, 'a repeated bookmark is idempotent');
  assert.equal(twice.bookmarks[0].ref, tb0.whole.whole_ref);
  assert.equal(twice.bookmarks[0].region, 'ground');
  assert.ok(twice.bookmarks[0].locus, 'the bookmarked locus comes from the arrangement');
  const removed = removeBookmark(twice, tb0.whole.whole_ref);
  assert.deepEqual(removed.bookmarks, []);
});

test('the guided traversal walks the whole in canonical order and stays deterministic', () => {
  const composition = deriveComposition(tb0);
  const steps = palaceTraversal(composition);
  assert.deepEqual(
    [...new Set(steps.map((step) => step.region))],
    PALACE_REGION_ORDER.filter((key) => composition.placements[key]),
  );
  assert.equal(steps.length, compositionRegions(composition).reduce((sum, region) => sum + region.entries.length, 0));
  const everyRef = new Set(steps.map((step) => step.ref));
  for (const region of compositionRegions(composition)) {
    for (const entry of region.entries) assert.ok(everyRef.has(entry.ref));
  }
  assert.deepEqual(steps, palaceTraversal(deriveComposition(tb0)), 'traversal is a pure function of the composition');
});

test('the Return leg routes the disclosed governed-write action; refusal mutates nothing', async () => {
  resetPalaceCompositions();
  const before = JSON.stringify(tb0);
  assert.equal(palaceReturnAction(tb0).action_ref, 'aikit.wiki.stage', 'the TB0 specimen names the governed-write Return leg');

  // Unbound composition: no proposal is invented.
  const unbound = palaceComposition(tb0);
  assert.equal(palaceReturnInput(tb0, unbound), null);
  assert.equal(palaceReturnRoute(tb0, unbound), null);

  // Bound: the route shapes the composition's native refs and the substrate change.
  const bound = bindComposition(tb0);
  const route = palaceReturnRoute(tb0, bound);
  assert.equal(route.action_ref, 'aikit.wiki.stage');
  assert.equal(route.subject_ref, tb0.subject.subject_ref);
  assert.equal(route.input.basis_ref, tb0.reading_ref);
  assert.ok(route.input.composed_refs.includes(tb0.whole.whole_ref));
  assert.deepEqual(
    route.input.scene_compose,
    { expression_ref: 'oi:expression:l5-dual-reading', scene_ref: 'oi:expression:l5-dual-reading/scene/crossing', elements: ['oi:expression:l5-dual-reading'] },
  );

  // Accepted route: the owner receipt, with the owner's own authority.
  const adapter = new FixtureTechneAdapter([tb0, representative, absent]);
  const receipt = await adapter.routeAction(route, tb0);
  assert.equal(receipt.routed, true);
  assert.equal(receipt.native_owner, 'aikit/wiki');
  assert.equal(receipt.authority, 'governed-write');
  assert.deepEqual(receipt.expected_effects, ['staged proposal only; no direct mutation of authored ground']);

  // Rejected leg: an undisclosed action routes nowhere and mutates nothing.
  const refused = await adapter.routeAction({ action_ref: 'wiki.commit.direct', subject_ref: tb0.subject.subject_ref, selection_ref: null, input: route.input }, tb0);
  assert.equal(refused.routed, false);
  assert.match(refused.reason, /not disclosed/);
  assert.equal(JSON.stringify(tb0), before, 'the reading is byte-identical after both legs');

  // After the routed Return, the renewed ground re-opens into M0′.
  assert.deepEqual(returnCrossing(tb0), { instrument: 'project' });
  resetPalaceCompositions();
});

test('a reading with no governed-write action has an honest Return absence', () => {
  assert.equal(palaceReturnAction(absent), null);
  assert.equal(palaceReturnAction(representative).action_ref, 'aikit.wiki.stage', 'the T0 fixture also discloses the governed-write stage action');
  const composition = bindComposition(absent);
  assert.equal(palaceReturnRoute(absent, composition), null, 'no Return action disclosed — no route is invented');
});

test('the structured agent state co-references verbatim refs and names unresolved evidence', () => {
  resetPalaceCompositions();
  const composition = bindComposition(tb0);
  const state = palaceAgentState(tb0, composition);
  assert.equal(state.basis.subject_ref, tb0.subject.subject_ref);
  assert.equal(state.basis.reading_ref, tb0.reading_ref);
  assert.equal(state.composition.bound, true);
  assert.deepEqual(state.expressions, tb0.expressions, 'Expression bindings ride verbatim');
  assert.deepEqual(state.actions, tb0.actions, 'native ActionRefs ride verbatim — the same Actions a human uses');
  assert.deepEqual(state.agency, tb0.agency, 'the situated-Agency floor rides verbatim');
  assert.equal(state.return_action_ref, 'aikit.wiki.stage');
  assert.ok(state.composition.regions.some((region) => region.key === 'ground' && region.entry_refs.length === 7));
  // Unresolved evidence is named with its reason, never smoothed over.
  const unprovenanced = state.unresolved.find((entry) => entry.ref === 'central:day:control:root:2026-09-16');
  assert.ok(unprovenanced, 'the day continuity ref rides without provenance in the specimen — a Recognition target');
  assert.match(unprovenanced.reason, /no provenance entry or facet source/);
  assert.ok(!state.unresolved.some((entry) => entry.ref === 'place:fixture:avalon'), 'a place grounding itself in a gazetteer is not flagged');
  assert.ok(!state.unresolved.some((entry) => entry.ref === 'place:fixture:royal-observatory-greenwich'), 'grounded refs are not flagged');
  resetPalaceCompositions();
});

test('the honest claim: an unclaimed composition ref is claimable, an empty subject is not', () => {
  // The TB0 specimen deliberately discloses the Palace unavailable while its
  // Expression names a composition ref — the surface may honestly claim it.
  const tb0Claim = palaceClaim(tb0);
  assert.equal(tb0Claim.disclosedAvailable, false);
  assert.equal(tb0Claim.claimable, true);
  assert.match(tb0Claim.reason, /no integral composition is bound/);
  // The absent-facets specimen has nothing to compose: the reason stands.
  const absentClaim = palaceClaim(absent);
  assert.equal(absentClaim.claimable, false);
  assert.equal(absentClaim.reason, 'no Expression composition available for this subject');
  // The representative fixture discloses the Palace available outright.
  const representativeClaim = palaceClaim(representative);
  assert.equal(representativeClaim.disclosedAvailable, true);
  assert.equal(representativeClaim.claimable, false);
  assert.equal(representativeClaim.reason, null);
});

test('the palace persists nothing: no storage APIs appear anywhere in the lane', async () => {
  const here = fileURLToPath(new URL('../src/techne/palace/', import.meta.url));
  const files = [
    'composition.ts',
    'recall.ts',
    'regions.ts',
    'palace-state.ts',
    'return.ts',
    'agent-state.ts',
    'register.ts',
    'PalaceInstrument.tsx',
  ];
  for (const file of files) {
    const source = await readFile(`${here}${file}`, 'utf8');
    // Strip comments first: the laws are documented in prose that names the
    // forbidden APIs; only real usage is a violation.
    const code = source.replace(/\/\*[\s\S]*?\*\//g, '').replace(/^\s*\/\/.*$/gm, '');
    assert.doesNotMatch(code, /localStorage|sessionStorage|indexedDB|openDatabase|caches\./,
      `${file} must hold no persistence API — the composition is session-scoped presentation state, and the native owners stay the only stores`);
  }
});

test('the palace mints no stochastic identity anywhere in its modules', async () => {
  const here = fileURLToPath(new URL('../src/techne/palace/', import.meta.url));
  const files = [
    'composition.ts',
    'recall.ts',
    'regions.ts',
    'palace-state.ts',
    'return.ts',
    'agent-state.ts',
    'register.ts',
    'PalaceInstrument.tsx',
  ];
  for (const file of files) {
    const source = await readFile(`${here}${file}`, 'utf8');
    assert.doesNotMatch(source, /crypto\.randomUUID|Math\.random|Date\.now/, `${file} mints identity or time`);
  }
});
