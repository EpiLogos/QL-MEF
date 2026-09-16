import test from 'node:test';
import assert from 'node:assert/strict';
import { createServer } from 'vite';
import { fileURLToPath } from 'node:url';
import {
  TECHNE_CONTRACT,
  instrumentReading,
  validateReading,
  validateSession,
} from '../src/techne/contract.ts';
import { resolveActionRoute } from '../src/techne/adapter.ts';
import { createDisclosureSessionStore } from '../src/techne/session.ts';
import { loadFixture } from '../src/techne/fixtures/load-fixtures.mjs';
import { renderMapModel, renderStreetModel } from '../src/techne/place/modes.ts';
import {
  GEOGRAPHY_RELATIONS,
  MYTHIC_RELATION,
  carriedOccasion,
  depthChain,
  placeRelations,
  referenceFrameLadder,
  relationStandingClass,
  routeModel,
  worldState,
} from '../src/techne/place/world.ts';

const cradle = fileURLToPath(new URL('..', import.meta.url));
const server = await createServer({ root: cradle, server: { middlewareMode: true }, appType: 'custom', logLevel: 'error' });
try {
  const registry = await server.ssrLoadModule('/src/techne/registry.tsx');
  const place = await server.ssrLoadModule('/src/techne/place/register.ts');
  const { placeFocusFromSession, registerPlaceSurface, selectionForPlace } = place;

  const tb0 = await loadFixture('tb0-connective-base-v1.json');
  const facets = tb0.spatial;
  const greenwich = facets[0];
  const avalon = facets[1];

  test('the World surface mounts under the place registration; duplicates are refused', () => {
    const stop = registerPlaceSurface();
    assert.equal(typeof registry.techneSurface('place'), 'function', 'the M4′ World body mounts as the place instrument');
    assert.throws(() => registerPlaceSurface(), /already registered/);
    stop();
    assert.equal(registry.techneSurface('place'), undefined);
  });

  // --- the mechanical TB0-1 mirror adoption --------------------------------

  test('the TB0-1 specimen validates against the surface mirror (mechanical field adoption)', () => {
    const checked = validateReading(tb0);
    assert.deepEqual(checked, { valid: true, errors: [] }, JSON.stringify(checked.errors));
    assert.equal(tb0.spatial[0].relation, 'OCCURRED_AT');
    assert.equal(tb0.spatial[1].relation, 'MYTH_LOCATED_AT');
  });

  test('the session cut law: application_cut must agree with the instrument', () => {
    const base = {
      contract: TECHNE_CONTRACT,
      session_ref: 'techne:disclosure-session:world-test',
      subject_ref: tb0.subject.subject_ref,
      selection: {
        selection_ref: 'ql.techne:selection:world-test',
        subject_ref: tb0.subject.subject_ref,
        reading_ref: tb0.reading_ref,
        instrument: 'place',
      },
      instrument: 'place',
      reading_ref: tb0.reading_ref,
    };
    assert.equal(instrumentReading('place'), '4:2-deep');
    assert.equal(instrumentReading('expressions'), '3:3-conjugate');
    assert.equal(validateSession({ ...base, application_cut: '4:2-deep' }).valid, true);
    const inconsistent = validateSession({ ...base, application_cut: '3:3-conjugate' });
    assert.equal(inconsistent.valid, false);
    assert.ok(inconsistent.errors.some((error) => error.includes('inconsistent with instrument')));
  });

  test('the situated-Agency binding law is enforced in the mirror', () => {
    const mutated = structuredClone(tb0);
    const anima = mutated.agency.find((role) => role.role === 'anima');
    anima.reading = '4:2-deep';
    let checked = validateReading(mutated);
    assert.equal(checked.valid, false);
    assert.ok(checked.errors.some((error) => error.includes('Anima_i inhabits the 3:3 conjugate reading')));
    const techne = structuredClone(tb0);
    const binding = techne.agency.find((role) => role.role === 'techne');
    binding.instrument = 'place';
    checked = validateReading(techne);
    assert.equal(checked.valid, false, 'Technē_2 cannot operate the M4′ instrument');
    assert.ok(checked.errors.some((error) => error.includes("operates its own coordinate's deep instrument")));
  });

  // --- hard geography distinctions and honest identity ---------------------

  test('the hard geography relations are preserved verbatim and grouped without collapsing', () => {
    assert.deepEqual(GEOGRAPHY_RELATIONS, ['OCCURRED_AT', 'LOCATED_IN', 'OPERATED_IN', 'TRAVELLED_TO', 'MYTH_LOCATED_AT']);
    const groups = placeRelations(facets);
    assert.deepEqual(groups.map((group) => group.relation), ['OCCURRED_AT', 'MYTH_LOCATED_AT']);
    assert.equal(groups[0].standingClass, 'factual');
    assert.equal(groups[1].standingClass, 'mythic');
    assert.equal(relationStandingClass('MYTH_LOCATED_AT'), 'mythic');
    assert.equal(relationStandingClass('SOME_NATIVE_RELATION'), 'owner-vocabulary', 'owner vocabulary is never relabelled');
    assert.equal(relationStandingClass(null), 'owner-vocabulary');
  });

  test('a mythic location keeps its identity without coordinates — it never becomes a historical site', () => {
    assert.equal(avalon.precision, 'unlocated');
    assert.equal(avalon.geometry, undefined, 'the mythic place discloses no geometry');
    assert.match(avalon.uncertainty, /mythic locus/);
    assert.equal(avalon.identity.names[0].name, 'Avalon');
    const street = renderStreetModel(facets, { selectedRef: avalon.place_ref });
    assert.equal(street.relation, 'MYTH_LOCATED_AT');
    assert.equal(street.geometry.kind, 'none', 'no geometry is invented for an unlocated identity');
    assert.equal(street.names[0].name, 'Avalon', 'identity is independent of coordinates');
  });

  test('the factual place carries validity, uncertainty and historically valid hierarchy verbatim', () => {
    assert.equal(greenwich.relation, 'OCCURRED_AT');
    assert.equal(greenwich.valid_from, '1675');
    assert.match(greenwich.uncertainty, /gazetteer-derived/);
    const street = renderStreetModel(facets, { selectedRef: greenwich.place_ref });
    assert.equal(street.relation, 'OCCURRED_AT');
    assert.equal(street.uncertainty, greenwich.uncertainty, 'owner-supplied uncertainty, never inferred');
    // Historical polity vs current geography: county-of-london 1889–1965,
    // greater-london 1965→ — containment changes while the identity stands.
    const relations = street.hierarchy.map((row) => `${row.place_ref} ${row.valid_from}→${row.valid_to}`);
    assert.deepEqual(relations, [
      'place:fixture:greenwich 1675→null',
      'place:fixture:county-of-london 1889→1965',
      'place:fixture:greater-london 1965→null',
    ]);
  });

  test('one subject at different places over time keeps one subject and stable place identities', () => {
    const sojourn = {
      ...tb0,
      spatial: [
        { ...greenwich, place_ref: 'place:fixture:first-site', valid_from: '1675', valid_to: '1710' },
        { ...greenwich, place_ref: 'place:fixture:second-site', valid_from: '1710', valid_to: null },
      ],
    };
    const checked = validateReading(sojourn);
    assert.deepEqual(checked, { valid: true, errors: [] }, JSON.stringify(checked.errors));
    assert.equal(sojourn.subject.subject_ref, tb0.subject.subject_ref, 'the subject never splits');
    assert.equal(placeRelations(sojourn.spatial)[0].facets.length, 2, 'both sojourns render under one relation group');
  });

  // --- movement: presentation ordering, never a minted route identity ------

  test('the movement overlay orders the reading’s own TRAVELLED_TO facets by their validity bounds', () => {
    const travelling = {
      place_ref: 'place:test:leg',
      relation: 'TRAVELLED_TO',
      identity: { names: [{ name: 'A Leg of the Journey' }] },
      geometry: { type: 'point', coordinates: [4.9, 52.37] },
      precision: 'approximate',
    };
    const facetsForRoute = [
      { ...travelling, place_ref: 'place:test:leg-b', valid_from: '1700-06' },
      { ...travelling, place_ref: 'place:test:leg-a', valid_from: '1698' },
      { ...avalon, relation: 'TRAVELLED_TO', geometry: undefined, valid_from: '1690' },
      { ...travelling, place_ref: 'place:test:leg-c', valid_from: null },
    ];
    const route = routeModel(facetsForRoute, null);
    assert.ok(route, 'three georeferenced movement facets make an ordering');
    assert.deepEqual(route.stops.map((stop) => stop.place_ref), [
      'place:test:leg-a',
      'place:test:leg-b',
      'place:test:leg-c',
    ], 'validity order; open bounds order last, never an invented date');
    assert.match(route.note, /presentation ordering/);
    assert.match(route.note, /no route identity is minted/);
    assert.equal(routeModel([travelling], null), null, 'one stop is no route');
    assert.equal(routeModel([{ ...avalon, relation: 'TRAVELLED_TO' }], null), null, 'unlocated movement stops are not forced into a path');
  });

  // --- carried world occasion and the provider-bound depth chain -----------

  test('the shared world occasion rides the reading’s own DAY/NOW/session/run refs, verbatim', () => {
    const occasion = carriedOccasion(tb0.temporal);
    assert.ok(occasion.day_refs.includes('central:day:control:root:2026-09-16'), 'Central owns the DAY');
    assert.ok(occasion.now_refs.length >= 1, 'Central owns the NOW');
    assert.ok(occasion.session_refs.length >= 1);
    assert.ok(occasion.run_refs.length >= 1);
    assert.ok(occasion.timezone_policy_refs.every((ref) => ref.includes('civil-time-policy')));
  });

  test('producer depth is provider-bound: M2/M3/M4 unavailable with reasons; M1 only when warranted', () => {
    const depths = depthChain(tb0);
    assert.deepEqual(depths.map((depth) => depth.depth), ['m1-harmonic', 'm2-planetary', 'm3-world-clock', 'm4-nara-earthbody']);
    assert.equal(depths[0].availability, 'absent-facet', 'the fixture’s warranted QL facet discloses no harmonic reading');
    assert.equal(depths[0].reason, 'the warranted QL facet discloses no harmonic reading');
    for (const depth of depths.slice(1)) {
      assert.equal(depth.availability, 'producer-unavailable');
      assert.ok(depth.reason.length > 40, `the honest reason names the absent producer leg: ${depth.reason}`);
    }
    assert.match(depths[1].reason, /ql\.m2-engine\/v1/);
    assert.match(depths[2].reason, /ql\.m3-state\/v1/);
    assert.match(depths[3].reason, /ql\.nara-personal-field\/v1/);
    const withHarmonic = { ...tb0, ql: { ...tb0.ql, harmonic_reading: 'a warranted harmonic reading' } };
    assert.deepEqual(depthChain(withHarmonic)[0], {
      depth: 'm1-harmonic',
      m_prime: 1,
      availability: 'disclosed',
      disclosed: ['a warranted harmonic reading'],
      reason: null,
      privacy: [],
    });
  });

  test('the scale ladder stands only where the reading stands; nothing is fabricated to complete it', () => {
    const ladder = referenceFrameLadder(tb0);
    assert.deepEqual(ladder.map((rung) => rung.rung), ['solar-planetary', 'earth-occasion', 'geography', 'places-routes', 'nara-situated']);
    assert.equal(ladder[0].availability, 'producer-unavailable');
    assert.equal(ladder[1].availability, 'disclosed', 'the carried DAY/NOW refs stand at the Earth-occasion rung');
    assert.ok(ladder[1].disclosed.includes('central:day:control:root:2026-09-16'));
    assert.equal(ladder[2].availability, 'disclosed');
    assert.match(ladder[2].disclosed[0], /2 place facets disclosed/);
    assert.equal(ladder[3].availability, 'disclosed');
    assert.equal(ladder[4].availability, 'producer-unavailable');
    const empty = referenceFrameLadder({ ...tb0, spatial: [], temporal: [] });
    assert.equal(empty[1].availability, 'absent-facet');
    assert.equal(empty[2].availability, 'absent-facet');
  });

  // --- structured World state for Aletheia_4 / Technē_4 --------------------

  test('worldState exposes the selected relation, validity, uncertainty, occasion, privacy, cuts and Actions', () => {
    const state = worldState(tb0, { mode: 'map', window: { from: null, to: null }, selectedRef: greenwich.place_ref });
    assert.equal(state.subject_ref, tb0.subject.subject_ref);
    assert.equal(state.reading_ref, tb0.reading_ref);
    assert.equal(state.selected.relation, 'OCCURRED_AT');
    assert.equal(state.selected.standing_class, 'factual');
    assert.equal(state.selected.uncertainty, greenwich.uncertainty);
    assert.equal(state.selected.valid_from, '1675');
    assert.equal(state.selected.source_ref, 'aikit:source:fixture:place-gazetteer');
    assert.deepEqual(state.relations.map((group) => group.relation), ['OCCURRED_AT', 'MYTH_LOCATED_AT']);
    assert.deepEqual(state.privacy_constraints, ['Nara-private personal state stays outside the 3:3 disclosure']);
    assert.deepEqual(state.application_cuts, tb0.disclosure.application_cuts, 'disclosed cuts ride verbatim');
    assert.deepEqual(state.actions, tb0.actions, 'disclosed Actions ride verbatim');
    assert.ok(state.cross_open.includes('timeline') && state.cross_open.includes('expressions'));
    assert.equal(state.cross_open.includes('place'), false, 'the current aperture is never its own cross-open target');
    const avalonState = worldState(tb0, { mode: 'map', window: null, selectedRef: avalon.place_ref });
    assert.equal(avalonState.selected.standing_class, 'mythic');
    assert.equal(avalonState.selected.uncertainty, avalon.uncertainty);
  });

  test('Technē_4 native operation routes disclosed Actions and refuses undisclosed ones — never executes', () => {
    const state = worldState(tb0, { mode: 'map', window: null, selectedRef: null });
    const dayRead = state.actions.find((action) => action.action_ref === 'central.day.read');
    assert.ok(dayRead, 'the reading discloses the Central read legs');
    const receipt = resolveActionRoute(tb0, { action_ref: 'central.day.read', subject_ref: tb0.subject.subject_ref });
    assert.equal(receipt.routed, true);
    assert.equal(receipt.native_owner, 'central/ctrl');
    assert.equal(receipt.authority, 'registered-read-action');
    assert.deepEqual(receipt.expected_effects, ['none — read only']);
    const refused = resolveActionRoute(tb0, { action_ref: 'central.day.close', subject_ref: tb0.subject.subject_ref });
    assert.equal(refused.routed, false, 'an undisclosed Action never routes');
    assert.match(refused.reason, /routing refused/);
  });

  test('the 3:3 crossing keeps the cut agreeing, carries subject/sources/occasion, and records the hop', () => {
    const store = createDisclosureSessionStore();
    const base = store.setSelection({
      selection_ref: 'ql.techne:selection:world-crossing',
      subject_ref: tb0.subject.subject_ref,
      reading_ref: tb0.reading_ref,
      source_ref: 'aikit:source:fixture:place-gazetteer',
      instrument: 'place',
      agent_session_ref: 'aikit:agent-session:fixture:l5-techne',
    });
    const crossed = store.openInInstrument('expressions');
    assert.equal(crossed.session_ref, base.session_ref, 'same session across the cut');
    assert.equal(crossed.subject_ref, base.subject_ref);
    assert.equal(crossed.reading_ref, base.reading_ref);
    assert.equal(crossed.selection.source_ref, 'aikit:source:fixture:place-gazetteer', 'source basis untouched');
    assert.equal(crossed.selection.agent_session_ref, 'aikit:agent-session:fixture:l5-techne', 'one AgentSession, never minted');
    assert.equal(crossed.application_cut, '3:3-conjugate', 'the cut moved with the instrument');
    assert.equal(validateSession(crossed).valid, true);
    assert.deepEqual(crossed.navigation, [{ from_instrument: 'place', to_instrument: 'expressions', selection_ref: 'ql.techne:selection:world-crossing' }]);
    const back = store.openInInstrument('place');
    assert.equal(back.application_cut, '4:2-deep', 'returning re-enters the deep cut');
    assert.equal(validateSession(back).valid, true);
  });

  // --- Map ↔ Relation ↔ Graph exact round-trip -----------------------------

  test('place → timeline (Relation) → project (Graph) → place preserves exact refs at every hop', () => {
    const store = createDisclosureSessionStore();
    const origin = store.setSelection(selectionForPlace(store.setSelection({
      selection_ref: 'ql.techne:selection:round-trip',
      subject_ref: tb0.subject.subject_ref,
      reading_ref: tb0.reading_ref,
      source_ref: 'aikit:source:fixture:place-gazetteer',
      source_revision: '7',
      instrument: 'timeline',
      agent_session_ref: 'aikit:agent-session:fixture:l5-techne',
    }), greenwich.place_ref));
    const atPlace = store.openInInstrument('place');
    const atRelation = store.openInInstrument('timeline');
    const atGraph = store.openInInstrument('project');
    const back = store.openInInstrument('place');
    for (const hop of [atPlace, atRelation, atGraph, back]) {
      assert.equal(validateSession(hop).valid, true, `${hop.instrument} stays contract-valid`);
      assert.equal(hop.subject_ref, origin.subject_ref, 'one subject');
      assert.equal(hop.reading_ref, origin.reading_ref, 'one reading basis');
      assert.equal(hop.selection.source_ref, origin.selection.source_ref, 'source carried verbatim');
      assert.equal(hop.selection.source_revision, origin.selection.source_revision, 'revision carried verbatim');
      assert.deepEqual(hop.selection.focus_refs, [greenwich.place_ref], 'the place focus rides every hop verbatim');
      assert.equal(hop.session_ref, origin.session_ref, 'one session');
    }
    assert.deepEqual(back.navigation.map((hop) => hop.to_instrument), ['timeline', 'project', 'place'],
      'the in-place focus move records no hop; the three projections do');
    assert.equal(back.navigation.length, 3);
  });

  // --- Journey Scene → real Place reopen -----------------------------------

  test('a Journey-style shared focus reopens the exact place in the World aperture', () => {
    const store = createDisclosureSessionStore();
    // What a Journey Scene frame does (M3′): set the shared selection’s focus
    // to a real Place ref and open the World aperture.
    const base = store.setSelection({
      selection_ref: 'ql.techne:selection:scene-frame',
      subject_ref: tb0.subject.subject_ref,
      reading_ref: tb0.reading_ref,
      instrument: 'journey',
    });
    const sceneFocus = store.setSelection({
      ...base.selection,
      focus_refs: [avalon.place_ref],
    });
    assert.equal(placeFocusFromSession(sceneFocus, facets), 'place:fixture:avalon', 'the scene’s place ref reopens as the World selection');
    assert.equal(placeFocusFromSession(sceneFocus, [greenwich]), null, 'a ref the reading does not disclose is never minted');
    assert.equal(placeFocusFromSession(null, facets), null);
    const sessionWithSpatial = {
      ...sceneFocus,
      selection: { ...sceneFocus.selection, focus_refs: [] },
      spatial_focus_ref: greenwich.place_ref,
    };
    assert.equal(placeFocusFromSession(sessionWithSpatial, facets), greenwich.place_ref, 'the session’s spatial focus is honoured too');
    const projected = renderMapModel(facets, { selectedRef: placeFocusFromSession(sceneFocus, facets) });
    const selected = projected.projection.places.find((place) => place.relation === MYTHIC_RELATION);
    assert.ok(selected, 'the mythic place projects into the honest no-coordinate register');
  });
} finally {
  await server.close();
}
