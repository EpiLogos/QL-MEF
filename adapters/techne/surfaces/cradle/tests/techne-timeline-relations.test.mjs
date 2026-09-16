import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import {
  arcLayout,
  causalChains,
  cyclesOf,
  directedChains,
  edgesForProjection,
  projectionAvailability,
  projectionForRelation,
  relationEdges,
  relationField,
  resolveTemporalQualification,
  selectionFocusedOnRelation,
} from '../src/techne/timeline/relations.ts';
import { domainFromFacets } from '../src/techne/timeline/scale.ts';
import { buildLanes, timelineIsEmpty } from '../src/techne/timeline/lanes.ts';
import { createDisclosureSessionStore } from '../src/techne/session.ts';
import { validateSession } from '../src/techne/contract.ts';
import { loadFixtureReadings } from '../src/techne/fixtures/load-fixtures.mjs';

// The lane's primary proving data: the pinned TB0 specimen, read directly
// from the canonical root fixture (no copy, no drift).
const specimen = JSON.parse(
  await readFile(new URL('../../../../../fixtures/techne/tb0-connective-base-v1.json', import.meta.url), 'utf8'),
);
const readings = await loadFixtureReadings();
const developmentDay = readings.find((reading) => reading.subject.subject_ref.startsWith('central:now:'));

const RECEIPT_MS = Date.parse('2026-09-16T17:24:36+01:00');

test('the TB0 specimen builds a relation field with verbatim types, refs and participants', () => {
  const field = relationField(specimen);
  assert.equal(field.edges.length, 4);
  assert.deepEqual(
    field.edges.map((edge) => edge.relation),
    ['supersedes', 'companion', 'INSTANTIATES', 'implemented-in'],
    'relation vocabulary is preserved verbatim, never relabelled',
  );
  assert.ok(field.edges.every((edge) => !edge.derived_id), 'every specimen relation carries a native relation_ref');
  assert.equal(field.edges[2].id, 'ql.techne:relation:fixture:tb0-instantiates-office');
  assert.equal(field.participants.length, 5, 'the distinct refs the four edges actually name');
  const supersedes = field.edges[0];
  assert.equal(supersedes.standing_verbatim, 'architecture-contract', 'the owner standing rides verbatim');
});

test('the trans-temporal INSTANTIATES relation carries NO date — none is invented', () => {
  const edge = relationEdges(specimen).find((candidate) => candidate.relation === 'INSTANTIATES');
  assert.equal(edge.temporal.state, 'trans-temporal');
  assert.equal(edge.temporal.facet_ref, null);
  assert.equal(edge.temporal.fromMs, null);
  assert.equal(edge.temporal.toMs, null);
  assert.equal(edge.temporal.positioned, false);
});

test('a dated relation resolves its temporal qualification against the reading it names', () => {
  const edge = relationEdges(specimen).find((candidate) => candidate.relation === 'implemented-in');
  assert.equal(edge.temporal.state, 'dated');
  assert.equal(edge.temporal.facet_ref, 'ql.techne:facet:tb0:received');
  assert.equal(edge.temporal.facet_kind, 'receipt');
  assert.equal(edge.temporal.fromMs, RECEIPT_MS, 'the dated edge sits at its facet\'s own receipt time');
});

test('a dangling temporal_facet_ref is unresolved and reported, never silently dated or dropped', () => {
  const withDangling = {
    ...structuredClone(specimen),
    whole: {
      ...specimen.whole,
      relations: [
        ...specimen.whole.relations,
        {
          relation: 'CAUSES',
          relation_ref: 'ql.techne:relation:fixture:dangling',
          from_ref: specimen.whole.member_refs[0],
          to_ref: specimen.whole.member_refs[1],
          temporal_facet_ref: 'ql.techne:facet:does-not-exist',
        },
      ],
    },
  };
  const edge = relationEdges(withDangling).find((candidate) => candidate.relation === 'CAUSES');
  assert.equal(edge.temporal.state, 'unresolved');
  assert.match(edge.temporal.problem, /names no facet/);
  const field = relationField(withDangling);
  assert.deepEqual(field.unresolved_temporal_refs, ['ql.techne:facet:does-not-exist']);
});

test('a temporal_facet_ref resolving to an unpositioned facet stays unresolved — continuity is not a date', () => {
  const qualification = resolveTemporalQualification(
    { temporal_facet_ref: 'ql.techne:facet:tb0:now' },
    specimen.temporal,
  );
  assert.equal(qualification.state, 'unresolved', 'the NOW facet carries continuity, not wall-clock time');
  assert.match(qualification.problem, /no wall-clock position/);
});

test('relation families stay distinct; unrecognised types land only in the field', () => {
  assert.equal(projectionForRelation('CAUSES'), 'cause');
  assert.equal(projectionForRelation('influences'), 'cause');
  assert.equal(projectionForRelation('SOURCED_FROM'), 'cause');
  assert.equal(projectionForRelation('ECHOES'), 'echo');
  assert.equal(projectionForRelation('RESONATES_WITH'), 'echo');
  assert.equal(projectionForRelation('INSTANTIATES'), 'echo');
  assert.equal(projectionForRelation('OPPOSES'), 'opposition');
  assert.equal(projectionForRelation('INHERITS'), 'opposition');
  assert.equal(projectionForRelation('TRANSFORMS_INTO'), 'opposition');
  assert.equal(projectionForRelation('supersedes'), 'opposition');
  assert.equal(projectionForRelation('companion'), 'field');
  assert.equal(projectionForRelation('implemented-in'), 'field');
  assert.equal(projectionForRelation('smells_like_purple'), 'field', 'an unknown type never silently becomes causal, echoing or oppositional');

  const field = relationField(specimen);
  assert.deepEqual(field.unclassified_relations.sort(), ['companion', 'implemented-in']);
  assert.deepEqual(edgesForProjection(field.edges, 'echo').map((edge) => edge.relation), ['INSTANTIATES']);
  assert.deepEqual(edgesForProjection(field.edges, 'opposition').map((edge) => edge.relation), ['supersedes']);
});

test('projection availability follows the specimen\'s actual data, with honest reasons', () => {
  const availability = projectionAvailability(specimen);
  const byMode = new Map(availability.map((entry) => [entry.mode, entry]));
  assert.equal(byMode.get('timeline').available, true);
  assert.equal(byMode.get('relations').available, true);
  assert.equal(byMode.get('relations').count, 4);
  assert.equal(byMode.get('cause').available, false);
  assert.match(byMode.get('cause').reason, /no CAUSES/);
  assert.equal(byMode.get('echo').available, true);
  assert.equal(byMode.get('opposition').available, true);
  assert.equal(byMode.get('phase').available, true, 'the specimen discloses a validity interval — a phase exists');
  assert.equal(byMode.get('activity').available, true, 'DAY/NOW/session/run continuity is disclosed');
});

test('a reading with relations but no temporal facets keeps the relation field open and the timeline honestly empty', () => {
  const timeless = {
    ...structuredClone(specimen),
    temporal: undefined,
  };
  delete timeless.temporal;
  const lanes = buildLanes(timeless.temporal);
  assert.equal(timelineIsEmpty(lanes), true, 'the temporal projection is honestly empty');
  const availability = projectionAvailability(timeless);
  const byMode = new Map(availability.map((entry) => [entry.mode, entry]));
  assert.equal(byMode.get('timeline').available, false, 'no date exists to place — the timeline says so');
  assert.match(byMode.get('timeline').reason, /no temporal facets/);
  assert.equal(byMode.get('relations').available, true, 'the trans-temporal relations remain fully disclosable');
  assert.equal(byMode.get('phase').available, false, 'no validity interval, no fabricated phase');
  const dated = relationEdges(timeless).filter((edge) => edge.temporal.state === 'dated');
  assert.deepEqual(dated, [], 'removing the temporal facets un-dates every relation — no date survives without its facet');
});

test('the relation field projection is pure: the reading is never mutated', () => {
  const snapshot = JSON.stringify(specimen);
  relationField(specimen);
  projectionAvailability(specimen);
  directedChains(relationEdges(specimen));
  assert.equal(JSON.stringify(specimen), snapshot, 'projections mutate nothing');
});

test('selecting a relation narrows focus and keeps co-reference byte-identical', () => {
  const store = createDisclosureSessionStore();
  const session = store.setSelection({
    selection_ref: 'ql.techne:selection:m2',
    subject_ref: specimen.subject.subject_ref,
    reading_ref: specimen.reading_ref,
    instrument: 'timeline',
  });
  const edge = relationEdges(specimen).find((candidate) => candidate.relation === 'implemented-in');
  const focused = selectionFocusedOnRelation(session.selection, edge);
  const checked = validateSession({ ...session, selection: focused, instrument: 'timeline', application_cut: '4:2-deep' });
  assert.equal(checked.valid, true, JSON.stringify(checked.errors));
  assert.equal(focused.subject_ref, session.selection.subject_ref, 'subject_ref is byte-identical');
  assert.equal(focused.reading_ref, session.reading_ref);
  assert.deepEqual(focused.focus_refs, [edge.id], 'focus carries the native relation_ref');
  const stored = store.setSelection(focused);
  assert.equal(stored.session_ref, session.session_ref, 'the same basis keeps the one session');
});

test('causal chains follow the owner\'s directions; an undated edge chains exactly like a dated one', () => {
  const reading = {
    ...structuredClone(specimen),
    whole: {
      whole_ref: 'test:whole:chains',
      relations: [
        { relation: 'CAUSES', relation_ref: 'r:a-b', from_ref: 'x:a', to_ref: 'x:b' },
        { relation: 'CAUSES', relation_ref: 'r:b-c', from_ref: 'x:b', to_ref: 'x:c' },
        { relation: 'INFLUENCES', relation_ref: 'r:c-d', from_ref: 'x:c', to_ref: 'x:d' },
        { relation: 'CAUSES', relation_ref: 'r:lonely', from_ref: 'x:e', to_ref: 'x:f' },
      ],
    },
  };
  const field = relationField(reading);
  const chains = causalChains(field);
  assert.equal(chains.length, 2, 'a three-edge chain plus a lone edge');
  assert.deepEqual(chains[0].map((edge) => edge.id), ['r:a-b', 'r:b-c', 'r:c-d']);
  assert.ok(chains[0].every((edge) => edge.temporal.state === 'trans-temporal'), 'undated edges chain structurally — no chronology was inferred to build the chain');
});

test('transformation cycles are reported once and deterministically', () => {
  const reading = {
    ...structuredClone(specimen),
    whole: {
      whole_ref: 'test:whole:cycle',
      relations: [
        { relation: 'TRANSFORMS_INTO', relation_ref: 'r:1', from_ref: 'x:a', to_ref: 'x:b' },
        { relation: 'TRANSFORMS_INTO', relation_ref: 'r:2', from_ref: 'x:b', to_ref: 'x:a' },
      ],
    },
  };
  const field = relationField(reading);
  const cycles = cyclesOf(field.edges);
  assert.equal(cycles.length, 1, 'one cycle, reported once regardless of start edge');
  assert.deepEqual(cycles[0].map((edge) => edge.id).sort(), ['r:1', 'r:2']);
  const again = cyclesOf(relationField(reading).edges);
  assert.deepEqual(again, cycles, 'deterministic');
});

test('the arc layout is deterministic geometry: sorted unique participants, bounded arcs', () => {
  const field = relationField(specimen);
  const refs = new Set();
  for (const edge of field.edges) {
    refs.add(edge.from_ref);
    refs.add(edge.to_ref);
  }
  const layout = arcLayout([...refs], field.edges, 1000);
  assert.equal(layout.nodes.length, 5);
  const xs = layout.nodes.map((node) => node.x);
  assert.deepEqual([...xs].sort((a, b) => a - b), xs, 'participants are sorted along the axis');
  assert.ok(Math.min(...xs) > 0 && Math.max(...xs) < 1000, 'nodes stay inside the view');
  for (const arc of layout.edges) {
    assert.ok(Math.abs(arc.controlY) <= 280, 'arcs stay bounded');
  }
  const again = arcLayout([...refs], field.edges, 1000);
  assert.deepEqual(again, layout, 'the same field always lays out identically');
});

// ---------------------------------------------------------------------------
// Development-day concurrency case — composed from LIVE Central shapes read
// through read-only `ctrl` during this lane (central.now.list, 2026-09-16:
// nine active NOW clearings; central.time.policy: Europe/London, +01:00).
// These are real native refs; the reading around them is composed here as a
// test fixture, exactly like the TB0 specimen composes its refs.
// ---------------------------------------------------------------------------

const LIVE_NOW_REFS = [
  'central:now:control:root:b417a7c3d37cd47c2762deee3224687d59db1a9015a322e9a8d7584b3947adf6',
  'central:now:control:root:17eab19e010bb3c8e2186860dc902ba85e410a372d6aef9fb247758b4fed8f95',
  'central:now:control:root:3c0c2aebe8e77607d76f598d9a2b90a058b874ec65c0b4e33b1fedf4474345e1',
];

test('overlapping Agent NOWs stay distinct lanes; a late receipt stays a receipt — nothing collapses to created_at', () => {
  // Composed from the live shapes: three concurrent NOWs, one DAY, one run
  // whose window was declared by a ProjectCentral handoff
  // (central.project-now.handoff/v1, recorded_at_unix_seconds 1789552600),
  // and a receipt that lands after the day the work happened on.
  const dayStart = Date.parse('2026-09-16T00:00:00+01:00');
  const reading = {
    contract: 'ql.techne/v1',
    reading_ref: 'ql.techne:reading:composed:live-development-day@1',
    subject: {
      subject_ref: LIVE_NOW_REFS[0],
      native_owner: 'central/now',
    },
    whole: { whole_ref: 'central:day:control:root:2026-09-16' },
    temporal: [
      { kind: 'day', day_ref: 'central:day:control:root:2026-09-16', instant: '2026-09-16T00:00:00+01:00', precision: 'day' },
      ...LIVE_NOW_REFS.map((now_ref) => ({ kind: 'now', now_ref })),
      {
        kind: 'occurrence',
        instant: '2026-09-16T11:16:40+01:00',
        precision: 'second',
        session_ref: 'zcode-session-2026-09-16-l5-techne-execution',
        uncertainty: 'author-declared recorded_at on the ProjectCentral handoff',
      },
      {
        kind: 'receipt',
        instant: '2026-09-16T23:06:19+01:00',
        precision: 'second',
        session_ref: 'zcode-session-2026-09-16-l5-techne-execution',
        uncertainty: 'reconciler-declared observed_at on the source-change horizon',
      },
      {
        kind: 'run',
        run_ref: 'factory:run:01JBD3Z4X7Q9M2A6K4P8T3W5NQ',
        attempt_ref: 'factory:run:01JBD3Z4X7Q9M2A6K4P8T3W5NQ#attempt-1',
        return_ref: 'factory:run:01JBD3Z4X7Q9M2A6K4P8T3W5NQ/return',
        interval: { from: '2026-09-15T09:10:00+01:00', to: '2026-09-16T16:40:00+01:00' },
        uncertainty: 'Factory orders by revision, not wall-clock; bounds are the handoff-declared activity window',
      },
    ],
    disclosure: { instruments: [{ instrument: 'timeline', available: true, m_prime: 2, reading: '4:2-deep' }] },
  };
  const lanes = buildLanes(reading.temporal);
  const byOwner = new Map(lanes.map((lane) => [lane.owner, lane]));
  const nowLanes = lanes.filter((lane) => lane.owner === 'now');
  assert.equal(nowLanes.length, 3, 'three concurrent NOWs are three distinct NOW lanes');
  assert.deepEqual(
    nowLanes.flatMap((lane) => lane.items.map((item) => item.now_ref)).sort(),
    [...LIVE_NOW_REFS].sort(),
    'each NOW keeps its own native ref verbatim',
  );
  assert.ok(nowLanes.every((lane) => lane.items.every((item) => !item.positioned)), 'NOWs carry continuity, not wall-clock positions');
  const sessionLane = byOwner.get('session');
  assert.equal(sessionLane.items.length, 2, 'occurrence and receipt stay distinct');
  const receipt = sessionLane.items.find((item) => item.kind === 'receipt');
  const occurrence = sessionLane.items.find((item) => item.kind === 'occurrence');
  assert.ok(receipt.fromMs > occurrence.fromMs, 'the reconciler saw the change after the author recorded it');
  assert.equal(receipt.kind, 'receipt', 'a receipt is never re-labelled an occurrence');

  const runItem = byOwner.get('run').items[0];
  assert.equal(runItem.fromMs, Date.parse('2026-09-15T09:10:00+01:00'));
  assert.equal(runItem.toMs, Date.parse('2026-09-16T16:40:00+01:00'));
  assert.ok(runItem.fromMs < dayStart, 'the run crosses the DAY boundary and stays one run');
  assert.equal(runItem.attempt_ref, 'factory:run:01JBD3Z4X7Q9M2A6K4P8T3W5NQ#attempt-1', 'attempt continuity rides verbatim');
  assert.equal(runItem.return_ref, 'factory:run:01JBD3Z4X7Q9M2A6K4P8T3W5NQ/return', 'Return continuity rides verbatim');

  const domain = domainFromFacets(reading.temporal);
  assert.equal(domain.fromMs, Date.parse('2026-09-15T09:10:00+01:00'), 'the honest domain starts at the run window, not at the DAY');
  assert.equal(domain.toMs, Date.parse('2026-09-17T00:00:00+01:00'), 'the honest domain ends at the DAY band\'s own full civil-day extent (day precision widens honestly past the late receipt)');
});
