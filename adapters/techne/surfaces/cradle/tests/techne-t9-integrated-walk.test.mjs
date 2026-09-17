import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  composeCentralGroundReading,
  centralGroundSource,
  LANES,
  LOCK_DOC,
  TB0_PIN_DOC,
  VAK_LOCK_DOC,
  TB0_ISSUE,
  T9_ISSUE,
  OI_ABSORPTION_DOC,
  OI_CAMPAIGN,
  SURFACES_PORT_RECORD,
  QL_GUARDIAN_EXPRESSION,
  CIVIL_TIME_POLICY,
  WHOLE_REF,
  AGENT_SESSION_REF,
  CONVERGED_HEAD,
  TB0_HEAD,
} from '../src/techne/providers/central-ground.ts';
import { registerTechneSource, resolveActionRoute, techneSource } from '../src/techne/adapter.ts';
import { validateReading, validateSelection } from '../src/techne/contract.ts';
import {
  boundedView,
  focusSelection,
  graphProjection,
  listProjection,
  projectQuery,
  projectSelectionRef,
  tierDisclosure,
  treeProjection,
} from '../src/techne/project/projections.ts';
import { createCanvasView, viewAddressedRefs, viewDrift } from '../src/techne/canvas/view.ts';
import {
  commitProposal,
  createProposal,
  proposeRelation,
  rejectProposal,
} from '../src/techne/canvas/proposal.ts';
import { relationField, resolveTemporalQualification } from '../src/techne/timeline/relations.ts';
import { expressionCrossing, relationFieldState } from '../src/techne/timeline/state.ts';
import { beats } from '../src/techne/journey/beats.ts';
import { composeSceneProposal } from '../src/techne/journey/compose.ts';
import { crossToExpression, returnPosition } from '../src/techne/journey/crossing.ts';
import { applyVakTraversal, bindVak, vakPlayOrder, vakWarranted } from '../src/techne/journey/vak.ts';
import { carriedOccasion, depthChain, placeRelations, referenceFrameLadder } from '../src/techne/place/world.ts';
import { palaceAvailability, palaceElements } from '../src/techne/palace/composition.ts';
import { deriveComposition } from '../src/techne/palace/palace-state.ts';
import { palaceRegions } from '../src/techne/palace/regions.ts';
import { palaceReturnAction, palaceReturnRoute, returnCrossing } from '../src/techne/palace/return.ts';
import { createDisclosureSessionStore } from '../src/techne/session.ts';

// tests/ → cradle → surfaces → techne → adapters → repo root.
const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..', '..', '..', '..', '..');
const capture = JSON.parse(
  await readFile(resolve(repoRoot, 'adapters/techne/surfaces/cradle/fixtures/central-ground-t9-capture-v1.json'), 'utf8'),
);

/** The REAL reading: the Technē dual-reading convergence over the actual
 * Central ground (the #220 subject), composed by the central-ground provider
 * and validated against the pinned TB0-1 contract on every composition. */
const reading = composeCentralGroundReading(capture);

test('T9 ground — one real subject, one bounded whole, exact revisions', () => {
  assert.equal(validateReading(reading).valid, true, 'the provider composes a contract-valid reading');
  assert.equal(reading.subject.subject_ref, T9_ISSUE);
  assert.equal(reading.snapshot.revision, CONVERGED_HEAD, 'the snapshot is the converged fan-out head');
  assert.equal(reading.whole.whole_ref, WHOLE_REF);
  for (const ref of [LOCK_DOC, TB0_PIN_DOC, VAK_LOCK_DOC, TB0_ISSUE, OI_CAMPAIGN, SURFACES_PORT_RECORD, QL_GUARDIAN_EXPRESSION, CIVIL_TIME_POLICY]) {
    assert.ok(reading.whole.member_refs.includes(ref), `the whole addresses the real source ${ref}`);
  }
  assert.equal(
    reading.whole.member_refs.filter((ref) => ref.startsWith('github:EpiLogos/QL-MEF/issues/21')).length,
    8,
    'the whole carries #212 and the seven lane issues',
  );
  // The temporal facts come from the real capture: the latest closed DAY and
  // the live civil-time frame.
  const day = reading.temporal.find((facet) => facet.kind === 'day');
  assert.equal(day.day_ref, capture.day_read.data.day_ref);
  assert.ok(reading.temporal.some((facet) => facet.timezone_policy_ref === CIVIL_TIME_POLICY));
  // Every lane's occurrence (head commit) and receipt (issue comment) pair
  // is present with DISTINCT instants — real work time vs recorded time.
  for (const lane of LANES) {
    const occ = reading.temporal.find((f) => f.facet_ref === `tf:occ:lane-${lane.issue_number}`);
    const rec = reading.temporal.find((f) => f.facet_ref === `tf:rec:lane-${lane.issue_number}`);
    assert.equal(occ.instant, lane.head_instant, `lane ${lane.issue_number} occurrence is the real head commit instant`);
    assert.equal(rec.instant, lane.receipt_instant, `lane ${lane.issue_number} receipt is the real comment instant`);
    assert.notEqual(occ.instant, rec.instant, 'occurrence and receipt stay distinct facts');
  }
  // The one place facet carries exactly what the native policy asserts —
  // region precision, uncertainty stated, no geometry manufactured.
  const place = reading.spatial[0];
  assert.equal(place.relation, 'OPERATED_IN');
  assert.equal(place.precision, 'region');
  assert.equal(place.geometry, undefined, 'no geometry is invented for the civil-time frame');
  assert.match(place.uncertainty, /no surveyed geometry/);
});

// ---------------------------------------------------------------------------
// M0′ — Project / Wiki / Graph over the real ground
// ---------------------------------------------------------------------------

test("M0′ — one bounded whole, three projections, same refs, inspectable provenance", () => {
  const query = projectQuery(reading, { depth: 2 });
  assert.equal(query.focus_ref, T9_ISSUE, 'the focus is the reading’s own focus (the #220 subject)');
  const view = boundedView(reading, query);
  const list = listProjection(view);
  const graph = graphProjection(view);
  const listRefs = list.map((node) => node.ref).sort();
  const graphRefs = graph.nodes.map((node) => node.ref).sort();
  assert.deepEqual(listRefs, graphRefs, 'LIST and GRAPH project the same members');
  assert.ok(view.nodes.length >= 20, 'the converged ground is a real neighbourhood');
  assert.ok(
    view.edges.some((edge) => edge.relation === 'parent-connective-base'),
    'the fan-out law is visible in the graph',
  );
  assert.ok(
    view.edges.some((edge) => edge.relation === 'feeds' && edge.to_ref === OI_CAMPAIGN),
    'the #65 absorption edge rides its real ref',
  );
  // Semantic zoom narrows disclosure by tier; the computed view never
  // re-queries.
  const locality = tierDisclosure('locality');
  assert.equal(locality.labels, true);
  assert.equal(locality.countsOnly, false);
  // Provenance: the lock entry names its exact revision and selector.
  const lockProvenance = reading.provenance.find((entry) => entry.source_ref === LOCK_DOC);
  assert.equal(lockProvenance.source_revision, CONVERGED_HEAD);
  assert.equal(lockProvenance.standing, 'owner-ratified-conceptual-architecture');
  // One registered source for the reading — no duplicate store.
  const unregister = registerTechneSource(centralGroundSource(capture));
  try {
    assert.ok(techneSource(reading.reading_ref), 'the source resolves by its reading ref');
  } finally {
    unregister();
  }
});

// ---------------------------------------------------------------------------
// Session — one continuing disclosure session over the real ground
// ---------------------------------------------------------------------------

function openGroundSession() {
  const store = createDisclosureSessionStore();
  const selection = {
    selection_ref: projectSelectionRef(reading.subject.subject_ref, T9_ISSUE),
    subject_ref: reading.subject.subject_ref,
    source_ref: LOCK_DOC,
    source_revision: CONVERGED_HEAD,
    focus_refs: [T9_ISSUE],
    reading_ref: reading.reading_ref,
    snapshot_revision: CONVERGED_HEAD,
    instrument: 'project',
    agent_session_ref: AGENT_SESSION_REF,
    selection_standing: 'current',
  };
  assert.equal(validateSelection(selection).valid, true);
  const session = store.setSelection(selection, {
    whole_ref: WHOLE_REF,
    project_ref: 'central:project:quaternal-logic',
    context_frame_ref: 'mef:context-frame:CF4',
    occasion_ref: capture.day_read.data.day_ref,
    return_target_ref: SURFACES_PORT_RECORD.replace(
      'l5-techne-surfaces-ported-to-ql-mef-home-2026-09-16',
      'T9-RETURN',
    ),
  });
  return { store, session };
}

test('session — the ground session opens on the real subject with the dual-reading state', () => {
  const { session } = openGroundSession();
  assert.equal(session.instrument, 'project');
  assert.equal(session.application_cut, '4:2-deep', 'a deep instrument holds the 4:2 cut (derived, never hand-set)');
  assert.equal(session.selection.agent_session_ref, AGENT_SESSION_REF, 'the native AIKit session ref rides the selection');
  assert.equal(session.whole_ref, WHOLE_REF);
  assert.equal(session.occasion_ref, capture.day_read.data.day_ref, 'the real DAY is the shared occasion');
});

// ---------------------------------------------------------------------------
// M1′ — Canvas: pure presentation vs one governed semantic proposal
// ---------------------------------------------------------------------------

test("M1′ — an authored arrangement leaves the reading byte-identical; a proposal routes through the reading's own Actions", async () => {
  // Pure presentation: an authored View over the real members (placements and
  // frames are view data, never semantics).
  const memberRefs = reading.whole.member_refs.slice(0, 8);
  const view = createCanvasView(reading, {
    title: undefined,
    placements: memberRefs.map((ref, index) => ({ ref, x: index * 90, y: (index % 3) * 60 })),
    frames: [{ frame_ref: 'ql.techne:frame:t9-walk', label: 'T9 convergence', member_refs: memberRefs, z: 1 }],
  });
  const drift = viewDrift(view, reading);
  assert.equal(drift.missing_refs.length, 0, 'no drift: every placed ref is a real member');
  assert.equal(drift.frames_losing_members.length, 0);
  assert.deepEqual(viewAddressedRefs(view), {
    placement_refs: memberRefs,
    frame_member_refs: memberRefs,
  }, 'the view addresses exactly the real refs');
  const before = JSON.stringify(reading);
  // Semantic: one honest proposal and one governed proposal.
  const vocabulary = [...new Set(reading.whole.relations.map((relation) => relation.relation))];
  assert.ok(vocabulary.includes('consumes'), 'the vocabulary is the reading’s own');
  const outside = createProposal(reading, {
    from_ref: LOCK_DOC,
    to_ref: OI_ABSORPTION_DOC,
    relation: 'echoes',
    note: 'walk leg A — rejected before any commit',
  });
  assert.equal(outside.outside_disclosed_vocabulary, true, 'a foreign type is flagged, never silent');
  const proposed = proposeRelation(outside);
  const rejected = rejectProposal(proposed, 'the walk rejects this proposal first — no native mutation may follow');
  assert.equal(rejected.status, 'rejected');
  assert.equal(JSON.stringify(reading), before, 'a rejected proposal leaves the reading byte-identical');
  // The governed proposal: the T9 lane feeds the renewed project record.
  const governed = proposeRelation(createProposal(reading, {
    from_ref: T9_ISSUE,
    to_ref: SURFACES_PORT_RECORD,
    relation: 'feeds',
    standing: 'sourced',
    source_ref: `${T9_ISSUE} (body: "This lane feeds the existing O:I #65 convergence/experience campaign")`,
    evidence_refs: [OI_CAMPAIGN],
    note: 'walk leg B — committed through the project register’s governed-write Return',
  }));
  assert.equal(governed.outside_disclosed_vocabulary, false);
  const outcome = await commitProposal(reading, governed, 'projectcentral.now.return');
  assert.equal(outcome.receipt.routed, true, 'the commit routes through the disclosed governed-write Action');
  assert.equal(outcome.receipt.native_owner, 'central/ctrl');
  assert.equal(outcome.route.input.relation, 'feeds');
  assert.equal(outcome.route.input.from_ref, T9_ISSUE);
  assert.equal(JSON.stringify(reading), before, 'routing attaches a receipt; the reading itself is only read');
  // An undisclosed action refuses at the routing seam — the reject leg at the
  // native boundary.
  const unrouted = resolveActionRoute(reading, {
    action_ref: 't9.shadow.write',
    subject_ref: reading.subject.subject_ref,
    selection_ref: null,
    input: {},
  });
  assert.equal(unrouted.routed, false, 'an undisclosed Action is refused with its reason');
  assert.match(unrouted.reason, /not disclosed/);
});

// ---------------------------------------------------------------------------
// M2′ — Relation Field / Timeline over real relations
// ---------------------------------------------------------------------------

test("M2′ — dated, trans-temporal, standing, occurrence-vs-receipt, and the crossing warrant", () => {
  const field = relationField(reading);
  // One chronological relation: the lane's implemented-in, dated through its
  // own occurrence facet (the real head-commit instant).
  const implemented = field.edges.find((edge) => edge.relation === 'implemented-in');
  assert.ok(implemented, 'the fan-out implementation relations are present');
  assert.equal(implemented.temporal.state, 'dated');
  assert.ok(implemented.temporal.facet_ref.startsWith('tf:occ:lane-'));
  // One non-chronological relation: the Guardian's stewardship — deliberately
  // trans-temporal, and no date is manufactured for it.
  const stewards = field.edges.find((edge) => edge.relation === 'stewards');
  assert.ok(stewards);
  assert.equal(stewards.temporal.state, 'trans-temporal');
  assert.equal(stewards.temporal.facet_ref, null);
  // Standing rides verbatim; the sourcing is carried, not upgraded.
  assert.equal(stewards.standing_verbatim, 'authored');
  assert.equal(implemented.standing_verbatim, 'sourced');
  // Occurrence vs receipt: two lane facets with distinct kinds resolve as
  // distinct temporal facts.
  const occ = resolveTemporalQualification(reading, `tf:occ:lane-213`, implemented);
  const rec = reading.temporal.find((f) => f.facet_ref === 'tf:rec:lane-213');
  assert.equal(rec.kind, 'receipt');
  assert.ok(rec.instant !== occ?.facet?.instant, 'the receipt is not the occurrence');
  // The structured relation-field state carries the disclosed Actions and
  // the situated Agency roles verbatim.
  const { session } = openGroundSession();
  const state = relationFieldState(reading, 'relations', session.selection, null);
  assert.equal(state.m_prime, 2);
  assert.ok(state.disclosed_actions.some((action) => action.action_ref === 'projectcentral.now.return'));
  assert.ok(state.agency.some((role) => role.role === 'techne' && role.instrument === 'palace'));
  // The M2 → 3:3 crossing warrant: focused, source-backed relation warrants;
  // an interpretation standing would refuse.
  const crossing = expressionCrossing(reading, { ...session.selection, focus_refs: [implemented.id] }, field);
  assert.equal(crossing.warranted, true, 'a source-backed focused relation warrants the 3:3 crossing');
});

// ---------------------------------------------------------------------------
// M3′ — Journey / Scenes: compose from real refs, C′ sequence, exact return
// ---------------------------------------------------------------------------

const SCENE_TITLES = [
  'the-connective-base-pins',
  'seven-lanes-fan-out',
  'convergence-and-return',
];

function extendedReading() {
  // The walk's session composition: the three composed scenes ride the real
  // Guardian expression binding (composeSceneProposal mints their refs in the
  // substrate's own grammar). Scene persistence in a live substrate is the
  // declared O:I port-back join — the proposals are the attributable act.
  const scenes = SCENE_TITLES.map((title) => ({
    expression_ref: QL_GUARDIAN_EXPRESSION,
    revision: null,
    scene_ref: `${QL_GUARDIAN_EXPRESSION}:scene:${title}`,
    composition_ref: null,
    profile_ref: null,
  }));
  return { ...reading, expressions: [...reading.expressions, ...scenes] };
}

test("M3′ — composition proposes real scenes; reorder-style composition copies nothing; the crossing returns exactly", () => {
  const { session } = openGroundSession();
  // Zero beats before composition: the honest absence.
  const empty = beats(reading);
  assert.equal(empty.beats.length, 0);
  assert.match(empty.unavailableReason, /bound Expressions disclose no scene refs/);
  // Compose three scenes from the real selection. CT burdens follow the
  // material actually carried: source extract, executable operation, and the
  // situated context (CT5's integration burden needs warranted derivation
  // refs this ground does not disclose — so it is not claimed).
  const proposals = SCENE_TITLES.map((title, index) =>
    composeSceneProposal({
      reading,
      selection: session.selection,
      title,
      ct: ['CT1', 'CT2', 'CT4'][index],
    }),
  );
  for (const proposal of proposals) {
    assert.equal(proposal.route.input.change.change, 'scene_create');
    assert.equal(proposal.route.input.frame.subject_ref, reading.subject.subject_ref);
    assert.ok(
      proposal.route.input.frame.source_refs.some((source) => source.source_ref === LOCK_DOC),
      'the scene frame carries the real source basis',
    );
    assert.ok(
      proposal.route.input.frame.temporal_facet_refs.some((ref) => ref.startsWith('tf:occ:')),
      'the scene frame carries the real occurrence facets',
    );
    assert.deepEqual(proposal.route.input.frame.place_refs, ['iana:timezone:Europe/London']);
  }
  // The extended reading is the same subject and the same relations — scene
  // composition added Expression bindings, not a shadow Journey ontology.
  const extended = extendedReading();
  assert.deepEqual(extended.whole, reading.whole, 'composition copies no semantic subjects');
  // The journey now exists over the composed scenes.
  const model = beats(extended);
  assert.equal(model.beats.length, 3);
  assert.ok(model.beats.every((beat) => beat.frame.subject_ref === reading.subject.subject_ref));
  // C′: the reading warrants Vāk (kernel vak source + context frame), so a
  // binding can exist — and the passage changes the actual traversal.
  assert.equal(vakWarranted(extended), true);
  const mk = (cs) => ({
    cpf: 'human-engaged',
    ct: model.beats.map((beat, index) => ({ scene_ref: beat.scene_ref, ct: ['CT1', 'CT2', 'CT4'][index] })),
    cp: model.beats.map((beat, index) => ({ scene_ref: beat.scene_ref, cp: ['4.0', '4.2', '4.5'][index] })),
    cf_ref: extended.ql.context_frame_ref,
    cfp: 'CFP0',
    cs,
    direction: 'forward-synthesis',
  });
  const gate0 = bindVak(mk('CS0'), extended, model.beats);
  const gate5 = bindVak(mk('CS5'), extended, model.beats);
  assert.equal(gate0.reason, null, 'CS0 (full traverse) binds over the warranted reading');
  assert.equal(gate5.reason, null, 'CS5 (direct synthesis) binds too');
  const traversal0 = applyVakTraversal(gate0.binding, model.beats);
  const traversal5 = applyVakTraversal(gate5.binding, model.beats);
  assert.notDeepEqual(vakPlayOrder(traversal0), vakPlayOrder(traversal5), 'CS changes the real sequence (V1)');
  assert.ok(traversal5.steps.length < traversal0.steps.length, 'direct synthesis is materially shorter than the full traverse');
  // Live 3:3: cross onto the second scene and return to the exact position.
  const { store } = openGroundSession();
  const start = store.get();
  const second = model.beats[1];
  const crossing = crossToExpression(start, second.scene_ref);
  assert.equal(crossing.selection.focus_refs.at(-1), second.scene_ref, 'the exact scene ref rides the crossing focus');
  assert.equal(crossing.selection.focus_refs[0], T9_ISSUE, 'the deep selection is carried alongside the scene focus');
  store.setSelection(crossing.selection);
  const crossedSession = store.openInInstrument(crossing.target);
  assert.equal(crossedSession.instrument, 'expressions');
  assert.equal(crossedSession.application_cut, '3:3-conjugate');
  assert.equal(crossedSession.selection.agent_session_ref, AGENT_SESSION_REF, 'one AgentSession across the cut');
  const back = returnPosition(store.get(), model.beats);
  assert.equal(back.beat.scene_ref, second.scene_ref, 'the return lands on the exact beat by ref equality');
});

// ---------------------------------------------------------------------------
// M4′ — World / Places: the carried occasion, honest depth chain
// ---------------------------------------------------------------------------

test("M4′ — the real occasion carries; the scale ladder stands only where the ground stands", () => {
  const occasion = carriedOccasion(reading.temporal);
  assert.ok(occasion.day_refs.includes(capture.day_read.data.day_ref), 'the real DAY rides the carried occasion');
  assert.ok(occasion.now_refs.length > 0, 'real NOW refs ride the carried occasion');
  assert.ok(occasion.timezone_policy_refs.includes(CIVIL_TIME_POLICY), 'the civil-time frame rides the occasion');
  const groups = placeRelations(reading.spatial);
  assert.equal(groups.length, 1);
  assert.equal(groups[0].relation, 'OPERATED_IN');
  assert.equal(groups[0].standingClass, 'factual', 'the policy scope is a factual relation, not a mythic one');
  // The wider reference-frame ladder discloses provider-bound depth honestly:
  // every producer-bound rung carries its availability and reason.
  const ladder = referenceFrameLadder(reading);
  const geography = ladder.find((rung) => rung.rung === 'geography');
  assert.ok(geography, 'the geography rung exists');
  const deepRungs = ladder.filter((rung) => rung.rung === 'solar-planetary' || rung.rung === 'nara-situated');
  assert.ok(
    deepRungs.every((rung) => rung.availability !== 'disclosed' && typeof rung.reason === 'string'),
    'solar/Nara depths carry their unavailability reason — nothing simulated',
  );
  const depths = depthChain(reading);
  assert.ok(depths.length > 0);
  assert.ok(
    depths.every((depth) => depth.availability === 'disclosed' ? depth.disclosed.length > 0 : typeof depth.reason === 'string' || depth.availability === 'absent-facet'),
    'the depth chain names why each unavailable depth is unavailable',
  );
  // The Nara/Expression crossing keeps the occasion byte-exact and the
  // privacy terms intact.
  const { store, session } = openGroundSession();
  const crossed = store.crossCut('expressions');
  assert.equal(crossed.session.application_cut, '3:3-conjugate');
  assert.equal(crossed.session.occasion_ref, session.occasion_ref, 'the occasion survives the cut crossing unchanged');
  const anima = reading.agency.find((role) => role.role === 'anima');
  assert.equal(crossed.session.selection.agent_session_ref, anima.agent_session_ref, 'the Anima session is the same native session');
});

// ---------------------------------------------------------------------------
// M5′ — Palace / Integral Whole and the governed Return legs
// ---------------------------------------------------------------------------

test("M5′ — the integral whole composes the real ref families; the Return routes governed; re-open lands on M0′", () => {
  const { session } = openGroundSession();
  const regions = palaceRegions(reading);
  assert.ok(regions.length >= 5, 'the palace composes the real regions of this ground');
  const regionKeys = regions.map((region) => region.key);
  assert.ok(regionKeys.includes('ground'), 'the M0′ ground region exists');
  assert.ok(regionKeys.includes('relation'), 'the M2′ relation region exists');
  const availability = palaceAvailability(reading);
  assert.equal(availability.available, true, 'the real ground composes an inhabitable whole');
  const elements = palaceElements(reading);
  assert.ok(elements.length > 0);
  // The integral composition over the real regions (presentation bind; the
  // reading's own disclosure is untouched by it).
  const composition = deriveComposition(reading);
  assert.deepEqual(composition.regions.map((region) => region.key), regionKeys);
  composition.bound = true;
  // The Return: the reading's governed-write action is the project register's
  // own Return action — and the re-open crossing lands on the project instrument.
  const returnAction = palaceReturnAction(reading);
  assert.equal(returnAction.action_ref, 'projectcentral.now.return');
  const route = palaceReturnRoute(reading, composition);
  assert.ok(route, 'the Return route composes from the real composition');
  assert.equal(route.action_ref, 'projectcentral.now.return');
  const reopen = returnCrossing(reading);
  assert.deepEqual(reopen, { instrument: 'project' }, 'the 5→0 re-open is the M0′ ground instrument');
  assert.equal(session.instrument, 'project');
});

// ---------------------------------------------------------------------------
// Session continuity — one companion across the whole walk
// ---------------------------------------------------------------------------

test('continuity — project → timeline → expressions → palace → project holds ONE session', () => {
  const { store } = openGroundSession();
  let session = store.get();
  const identity = () => [
    session.subject_ref,
    session.reading_ref,
    session.selection.selection_ref,
    session.selection.source_ref,
    session.selection.agent_session_ref,
    session.whole_ref,
    session.occasion_ref,
    session.return_target_ref,
  ];
  const atStart = identity();
  const hops = [];
  session = store.openInInstrument('timeline');
  hops.push([session.instrument, session.application_cut]);
  session = store.crossCut('expressions').session;
  hops.push([session.instrument, session.application_cut]);
  session = store.openInInstrument('palace');
  hops.push([session.instrument, session.application_cut]);
  session = store.openInInstrument('project');
  hops.push([session.instrument, session.application_cut]);
  assert.deepEqual(hops, [
    ['timeline', '4:2-deep'],
    ['expressions', '3:3-conjugate'],
    ['palace', '4:2-deep'],
    ['project', '4:2-deep'],
  ]);
  assert.deepEqual(identity(), atStart, 'subject, basis, selection, AgentSession, whole, occasion and Return target survive the whole walk byte-exact');
  assert.equal(session.selection.agent_session_ref, AGENT_SESSION_REF, 'one continuing canonical companion session');
  // A crossing onto the occupied cut is refused — the same-cut law.
  assert.throws(() => store.crossCut('project'), /cut/);
});

// ---------------------------------------------------------------------------
// 5→0 — the renewed ground is the live one (runs after the executed Return)
// ---------------------------------------------------------------------------

test('5→0 — the executed Return is visible in the live project register (renewed M0′ ground)', async () => {
  const { execFile } = await import('node:child_process');
  const { promisify } = await import('node:util');
  const run = promisify(execFile);
  const { stdout } = await run('ctrl', ['--json', 'action', 'run', 'projectcentral.now.inspect', '{"project":"Quaternal-Logic"}'], {
    encoding: 'utf8',
  });
  const inspect = JSON.parse(stdout);
  const records = inspect?.data?.records ?? inspect?.data?.nows ?? inspect?.data?.records_list ?? [];
  const all = JSON.stringify(inspect);
  assert.match(all, /t9-integrated-field|T9 integrated field/i,
    'the renewed project NOW field carries the T9 return — the accepted difference is live in native ground');
});
