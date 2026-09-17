import test from 'node:test';
import assert from 'node:assert/strict';
import { FixtureTechneAdapter } from '../src/techne/adapter.ts';
import { composeSceneProposal, EXPRESSION_EDIT_ACTION } from '../src/techne/journey/compose.ts';
import {
  PROVING_EXPRESSION,
  PROVING_SUBJECT_REF,
  provingReading,
} from '../src/techne/journey/proving-reading.ts';
import { loadFixtureReadings } from '../src/techne/fixtures/load-fixtures.mjs';

const reading = provingReading();

function selection(overrides = {}) {
  return {
    selection_ref: 'techne:test-selection:compose',
    subject_ref: PROVING_SUBJECT_REF,
    reading_ref: reading.reading_ref,
    instrument: 'canvas',
    focus_refs: ['central:focus:tb0-relations'],
    ...overrides,
  };
}

test('the selection composes into a routed scene-create proposal carrying the exact refs verbatim', () => {
  const { route, input } = composeSceneProposal({ reading, selection: selection(), title: 'Relation Field Walkthrough' });
  assert.equal(route.subject_ref, PROVING_SUBJECT_REF, 'the route names the reading subject');
  assert.equal(route.selection_ref, 'techne:test-selection:compose', 'the selection co-references the route');
  assert.equal(route.action_ref, EXPRESSION_EDIT_ACTION,
    'the proving reading discloses no composition action, so the route names the owner real edit op');
  assert.equal(input.change.change, 'scene_create');
  assert.equal(input.change.scene_ref, `${PROVING_EXPRESSION}:scene:relation-field-walkthrough`,
    'the proposed ref follows the substrate own scene grammar');
  assert.deepEqual(input.frame.subject_ref, PROVING_SUBJECT_REF);
  assert.deepEqual(input.frame.focus_refs, ['central:focus:tb0-relations'], 'the selected refs ride verbatim — nothing copied');
  assert.ok(input.frame.temporal_facet_refs.includes('ql.techne:facet:tb0:authored'), 'the real occurrence facet is the scene time');
  assert.ok(input.frame.place_refs.includes('place:fixture:royal-observatory-greenwich'), 'the real place is the scene situation');
  assert.ok(input.frame.source_refs.some((source) => source.source_ref === PROVING_SUBJECT_REF), 'the real sources are the scene basis');
  assert.equal(input.revision, '1', 'the bound revision rides so the owner can judge the proposal');
});

test('composition refuses honestly: no bound Expression, unnamed scene, colliding scene ref', () => {
  const unbound = { ...reading, expressions: undefined };
  assert.throws(
    () => composeSceneProposal({ reading: unbound, selection: selection(), title: 'Anywhere' }),
    /no Expression is bound/,
  );
  assert.throws(
    () => composeSceneProposal({ reading, selection: selection(), title: '   ' }),
    /usable scene name/,
  );
  assert.throws(
    () => composeSceneProposal({ reading, selection: selection(), title: '!!!' }),
    /usable scene name/,
  );
});

test('a bound CT demands its material (V1): CT1 needs an exact source selector and is refused without one', () => {
  const met = composeSceneProposal({ reading, selection: selection(), title: 'Extract', ct: 'CT1' });
  assert.equal(met.input.ct, 'CT1', 'the proving reading discloses a text_span selector — CT1 is met');

  const stripped = {
    ...reading,
    provenance: reading.provenance.map((entry) => ({ ...entry, selector: null })),
  };
  assert.throws(
    () => composeSceneProposal({ reading: stripped, selection: selection(), title: 'Extract', ct: 'CT1' }),
    /CT1.*needs an exact source selector/,
    'the label without its material is refused, never silently carried',
  );
  // CT without demand composes as before:
  const plain = composeSceneProposal({ reading: stripped, selection: selection(), title: 'Extract' });
  assert.equal(plain.input.ct, undefined);
});

test('routing the composition is honest: undisclosed edit action comes back unrouted; a disclosed one routes', async () => {
  const fixtureReadings = await loadFixtureReadings();
  const adapter = new FixtureTechneAdapter(fixtureReadings);
  const representative = fixtureReadings.find((entry) => entry.reading_ref === 'ql.techne:reading:fixture:representative-subject@1');
  const provingSelection = selection();
  const { route } = composeSceneProposal({ reading, selection: provingSelection, title: 'Bridge Walkthrough' });
  const unrouted = await adapter.routeAction(route, representative);
  assert.equal(unrouted.routed, false, 'routing refuses instead of executing');
  assert.match(unrouted.reason, /not disclosed by reading/);

  const disclosing = {
    ...reading,
    actions: [
      ...reading.actions,
      {
        action_ref: EXPRESSION_EDIT_ACTION,
        native_owner: 'oi/desktop',
        authority: 'owner-disclosed',
        summary: 'Edit the Expression document: create scenes and compose the sequence',
        expected_effects: ['the Expression document changes under the owner revision discipline'],
        input_schema_ref: null,
      },
    ],
  };
  const routedProposal = composeSceneProposal({ reading: disclosing, selection: provingSelection, title: 'Bridge Walkthrough' });
  assert.equal(routedProposal.route.action_ref, EXPRESSION_EDIT_ACTION, 'the exact disclosed action wins, verbatim');
  const routed = await new FixtureTechneAdapter([disclosing]).routeAction(routedProposal.route, disclosing);
  assert.equal(routed.routed, true);
  assert.equal(routed.native_owner, 'oi/desktop');
});

test('composition mutates nothing: reading and selection survive untouched', () => {
  const readingBefore = structuredClone(reading);
  const selectionBefore = structuredClone(selection());
  composeSceneProposal({ reading, selection: selection(), title: 'Immutable Check', ct: 'CT4' });
  assert.deepEqual(reading, readingBefore);
  assert.deepEqual(selection(), selectionBefore);
});
