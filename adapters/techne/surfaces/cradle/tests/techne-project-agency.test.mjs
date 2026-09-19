import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { resolveActionRoute } from '../src/techne/adapter.ts';
import {
  agencyLocate,
  agencyRoute,
  agencyTraverse,
  technaeState,
} from '../src/techne/project/agency.ts';
import { boundedView, projectQuery } from '../src/techne/project/projections.ts';
import { validateReading } from '../src/techne/contract.ts';
import { createDisclosureSessionStore } from '../src/techne/session.ts';

// tests/ → cradle → surfaces → techne → adapters → repo root.
const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..', '..', '..', '..', '..');
const specimen = JSON.parse(
  await readFile(resolve(repoRoot, 'fixtures/techne/tb0-connective-base-v1.json'), 'utf8'),
);

const LOCK = 'central:source:control:root:Work/Quaternal-Logic/docs/L5-TECHNE-DUAL-READING-LOCK.md';
const WAYFINDER = 'central:source:control:root:Work/Quaternal-Logic/docs/L5-TECHNE-INSTRUMENT-WAYFINDER.md';

const store = createDisclosureSessionStore();
const session = store.setSelection({
  selection_ref: 'ql.techne:selection:agency-origin',
  subject_ref: specimen.subject.subject_ref,
  reading_ref: specimen.reading_ref,
  instrument: 'project',
  agent_session_ref: 'aikit:agent-session:fixture:l5-techne',
});
const view = boundedView(specimen, projectQuery(specimen, { focus_ref: LOCK, depth: 1 }));

test('the operating state discloses what the Agency may perceive — subject, view, actions, cuts', () => {
  const state = technaeState(specimen, session, view);
  assert.equal(state.subject_ref, specimen.subject.subject_ref);
  assert.equal(state.instrument, 'project');
  assert.equal(state.application_cut, '4:2-deep');
  assert.equal(state.selection_ref, session.selection.selection_ref);
  assert.equal(state.view.node_count, view.nodes.length, 'the Agency sees the same bounded whole, not a rebuild');
  assert.equal(state.view.truncated, false);
  assert.deepEqual(state.actions.map((action) => action.action_ref), ['central.day.read', 'central.now.read', 'oi.expression.open', 'aikit.wiki.stage']);
  assert.deepEqual(state.disclosure.cuts.map((cut) => cut.cut), ['4:2-deep', '3:3-conjugate']);
});

test('the reading’s own agency bindings ride verbatim; an absent Technē_0 is absent with a reason, never minted', () => {
  const state = technaeState(specimen, session, view);
  assert.deepEqual(state.agency, specimen.agency, 'the role floor is the owner’s disclosure');
  assert.equal(state.technae_0, null, 'the specimen binds Technē_2, not Technē_0 — none is invented');
  assert.match(state.technae_0_absence_reason, /none is an Aletheia_0\/Technē_0/);
});

test('a lawful Technē_0 binding is exposed as the operating role; route input names it', async () => {
  const bound = {
    ...specimen,
    agency: [...specimen.agency, {
      role: 'techne',
      m_index: 0,
      reading: '4:2-deep',
      instrument: 'project',
      guardian_ref: 'actuation:agent:anuttara',
      agent_session_ref: 'aikit:agent-session:fixture:l5-techne',
      authority: 'locates, traverses and proposes governed returns over the M0′ instrument',
      privacy: null,
    }],
  };
  assert.equal(validateReading(bound).valid, true);
  const state = technaeState(bound, session, view);
  assert.equal(state.technae_0.role, 'techne');
  assert.equal(state.technae_0.m_index, 0);
  assert.equal(state.technae_0.instrument, 'project');
  assert.equal(state.technae_0.guardian_ref, 'actuation:agent:anuttara', 'the Guardian anchors the role without becoming it');
  assert.equal(state.technae_0_absence_reason, null);
  const route = agencyRoute(state, 'aikit.wiki.stage');
  assert.equal(route.input.acting_role, 'agency:techne:M0');
});

test('LOCATE returns the attributable answer: identity, standing and exact sources', () => {
  const state = technaeState(specimen, session, view);
  const located = agencyLocate(state, specimen);
  assert.equal(located.subject_ref, specimen.subject.subject_ref);
  assert.equal(located.native_owner, 'central');
  assert.equal(located.kind, 'source');
  assert.equal(located.standing, 'architecture-contract');
  assert.ok(located.sources.length === specimen.provenance.length);
  const selected = located.sources.find((source) => source.selector_unit !== null);
  assert.ok(selected, 'an exact selector is disclosed where the owner supplies one');
  assert.equal(selected.selector_unit, 'text_span');
});

test('TRAVERSE moves within the disclosed view and refuses what it does not hold', () => {
  const state = technaeState(specimen, session, view);
  const moved = agencyTraverse(state, view, WAYFINDER);
  assert.equal(moved.focus_ref, WAYFINDER);
  assert.equal(moved.depth, view.query.depth, 'the traverse keeps the bounded depth');
  const refused = agencyTraverse(state, view, 'central:source:control:root:Control/user/placement.json');
  assert.match(refused.refused, /not in the disclosed bounded view/);
});

test('PROPOSE/EXECUTE routes a disclosed Action through the adapter law; the receipt names the owner', () => {
  const state = technaeState(specimen, session, view);
  const route = agencyRoute(state, 'central.day.read');
  assert.equal(route.action_ref, 'central.day.read');
  assert.equal(route.subject_ref, state.subject_ref);
  assert.equal(route.selection_ref, session.selection.selection_ref);
  const receipt = resolveActionRoute(specimen, route);
  assert.equal(receipt.routed, true);
  assert.equal(receipt.native_owner, 'central/ctrl');
  assert.equal(receipt.authority, 'registered-read-action');
  // An undisclosed Action is an explicit refusal, never a silent pass-through.
  const refused = agencyRoute(state, 'central.day.lifecycle');
  assert.match(refused.refused, /not disclosed by this reading/);
});

test('without a session or a view the state still tells the truth', () => {
  const bare = technaeState(specimen, null, null);
  assert.equal(bare.selection_ref, null);
  assert.equal(bare.view.node_count, 0);
  assert.match(bare.view.warnings[0], /no bounded view is computed yet/);
});
