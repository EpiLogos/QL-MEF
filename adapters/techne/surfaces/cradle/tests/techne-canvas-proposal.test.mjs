import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import {
  createProposal,
  proposeRelation,
  rejectProposal,
  commitProposal,
  disclosedRelationVocabulary,
  disclosedAddressedRefs,
  createProposalStore,
  mintProposalRef,
} from '../src/techne/canvas/proposal.ts';
import { resolveActionRoute } from '../src/techne/adapter.ts';

const here = fileURLToPath(new URL('.', import.meta.url));
const tb0 = JSON.parse(
  await readFile(new URL('../../../../../fixtures/techne/tb0-connective-base-v1.json', 'file://' + here), 'utf8'),
);

const [lockDoc, wayfinderDoc] = tb0.whole.member_refs;
const governedWrite = tb0.actions.find((action) => action.authority === 'governed-write');

test('the vocabulary authority is the reading’s own disclosed relation types — nothing hard-coded', () => {
  assert.deepEqual(disclosedRelationVocabulary(tb0), ['supersedes', 'companion', 'INSTANTIATES', 'implemented-in']);
  assert.equal(disclosedRelationVocabulary(tb0).includes('CONNECTED_TO'), false, 'no generic edge type is offered');
});

test('a proposal carries the actual type, endpoints from addressed refs, and evidence discipline', () => {
  const proposal = proposeRelation(createProposal(tb0, {
    from_ref: lockDoc,
    to_ref: wayfinderDoc,
    relation: 'supersedes',
    standing: 'interpretation',
    source_ref: tb0.provenance[0].source_ref,
    evidence_refs: ['ql-mef:commit:2159c7df8aa8f8db0fbd2fcd800d61feb9b7d0d4'],
    note: 'same supersedes family the field already uses',
  }));
  assert.equal(proposal.status, 'proposed');
  assert.equal(proposal.relation, 'supersedes');
  assert.equal(proposal.outside_disclosed_vocabulary, false);
  assert.ok(proposal.proposal_ref.startsWith('ql.techne:canvas-proposal:'), 'a proposal ref is a presentation artifact ref, never a native relation ref');
  assert.deepEqual(proposal.evidence_refs, ['ql-mef:commit:2159c7df8aa8f8db0fbd2fcd800d61feb9b7d0d4']);
});

test('a type outside the disclosed vocabulary is carried verbatim but flagged, never relabelled', () => {
  const proposal = createProposal(tb0, { from_ref: lockDoc, to_ref: wayfinderDoc, relation: 'ECHOES' });
  assert.equal(proposal.relation, 'ECHOES');
  assert.equal(proposal.outside_disclosed_vocabulary, true);
});

test('endpoints must be refs the reading addresses — no invented field residents', () => {
  assert.throws(() => createProposal(tb0, { from_ref: 'nowhere:thing', to_ref: wayfinderDoc, relation: 'supersedes' }), /not disclosed/);
  // A relation endpoint that is adrift from the whole IS disclosed.
  assert.ok(disclosedAddressedRefs(tb0).has('ql:vak:composition:l5-para-vak'));
});

test('a proposal without a relation type is refused — no silent generic edge', () => {
  assert.throws(() => createProposal(tb0, { from_ref: lockDoc, to_ref: wayfinderDoc, relation: '  ' }), /actual relation type/);
});

test('reject touches NOTHING: reading and store stay byte-identical', () => {
  const before = JSON.stringify(tb0);
  const store = createProposalStore();
  const proposed = proposeRelation(createProposal(tb0, { from_ref: lockDoc, to_ref: wayfinderDoc, relation: 'companion' }));
  store.put(proposed);
  const rejected = rejectProposal(proposed, 'reviewer declines');
  store.put(rejected);
  assert.equal(rejected.status, 'rejected');
  assert.equal(rejected.resolution_note, 'reviewer declines', 'the reviewer reason is recorded verbatim');
  assert.equal(JSON.stringify(tb0), before, 'the reading is untouched');
  const stored = store.find(rejected.proposal_ref);
  assert.equal(stored.receipt, undefined, 'no receipt is fabricated on reject');
  assert.equal(rejectProposal(rejected).status, 'rejected', 'reject is idempotent');
  assert.match(rejectProposal(proposeRelation(createProposal(tb0, { from_ref: lockDoc, to_ref: wayfinderDoc, relation: 'companion' }))).resolution_note, /without mutation/, 'the default note names the law');
});

test('rejecting an already-routed proposal is refused — the owner owns it now', () => {
  const routed = {
    ...proposeRelation(createProposal(tb0, { from_ref: lockDoc, to_ref: wayfinderDoc, relation: 'supersedes' })),
    status: 'routed',
    action_ref: governedWrite.action_ref,
    receipt: { action_ref: governedWrite.action_ref, native_owner: governedWrite.native_owner, routed: true, authority: governedWrite.authority, expected_effects: [] },
  };
  assert.throws(() => rejectProposal(routed), /already routed/);
});

test('commit routes through the reading’s own disclosed Action with the exact typed payload', async () => {
  const proposed = proposeRelation(createProposal(tb0, {
    from_ref: lockDoc,
    to_ref: wayfinderDoc,
    relation: 'supersedes',
    standing: 'fact',
    evidence_refs: ['ql-mef:evidence:test'],
  }));
  const routes = [];
  const recordingRoute = async (reading, route) => {
    routes.push({ reading: reading.reading_ref, route });
    return resolveActionRoute(reading, route);
  };
  const outcome = await commitProposal(tb0, proposed, governedWrite.action_ref, recordingRoute);
  assert.equal(routes.length, 1);
  assert.equal(routes[0].reading, tb0.reading_ref);
  assert.equal(routes[0].route.action_ref, governedWrite.action_ref);
  assert.equal(routes[0].route.subject_ref, tb0.subject.subject_ref);
  assert.equal(routes[0].route.input.relation, 'supersedes');
  assert.equal(routes[0].route.input.standing, 'fact');
  assert.deepEqual(routes[0].route.input.evidence_refs, ['ql-mef:evidence:test']);
  assert.equal(outcome.receipt.routed, true);
  assert.equal(outcome.receipt.native_owner, 'aikit/wiki');
  assert.equal(outcome.proposal.status, 'routed');
  assert.deepEqual(outcome.proposal.receipt, outcome.receipt);
});

test('an undisclosed action yields the owner-side refusal verbatim; the proposal stays a proposal', async () => {
  const proposed = proposeRelation(createProposal(tb0, { from_ref: lockDoc, to_ref: wayfinderDoc, relation: 'companion' }));
  const outcome = await commitProposal(tb0, proposed, 'central.now.write-something-undisclosed');
  assert.equal(outcome.receipt.routed, false);
  assert.match(outcome.receipt.reason, /not disclosed by reading/);
  assert.equal(outcome.proposal.status, 'proposed', 'unrouted means still just a suggestion');
  assert.match(outcome.proposal.resolution_note, /not disclosed/);
  assert.equal(outcome.proposal.receipt.routed, false, 'the refusal receipt is attached verbatim, not hidden');
});

test('a disclosed read action routes with its own authority — the receipt keeps the read/write distinction legible', async () => {
  const read = tb0.actions.find((action) => action.action_ref === 'central.day.read');
  const proposed = proposeRelation(createProposal(tb0, { from_ref: lockDoc, to_ref: wayfinderDoc, relation: 'companion' }));
  const outcome = await commitProposal(tb0, proposed, read.action_ref);
  assert.equal(outcome.receipt.routed, true, 'the routing layer routes any disclosed action — capability, not semantics');
  assert.equal(outcome.receipt.authority, 'registered-read-action', 'the receipt names the read authority verbatim');
  assert.deepEqual(outcome.receipt.expected_effects, ['none — read only'], 'the receipt discloses exactly what the owner disclosed — a read, not a mutation');
});

test('commit refuses a draft and a rejected proposal', async () => {
  const draft = createProposal(tb0, { from_ref: lockDoc, to_ref: wayfinderDoc, relation: 'companion' });
  await assert.rejects(commitProposal(tb0, draft, governedWrite.action_ref), /still a draft/);
  const rejected = rejectProposal(draft);
  await assert.rejects(commitProposal(tb0, rejected, governedWrite.action_ref), /rejected/);
});

test('lifecycle state machine is one-way and honest', async () => {
  const draft = createProposal(tb0, { from_ref: lockDoc, to_ref: wayfinderDoc, relation: 'supersedes' });
  assert.equal(draft.status, 'draft');
  const proposed = proposeRelation(draft);
  assert.throws(() => proposeRelation(proposed), /not draft/);
  const rejected = rejectProposal(proposed);
  await assert.rejects(commitProposal(tb0, rejected, governedWrite.action_ref), /rejected/);
});

test('the proposal store notifies, replaces by ref, and stops notifying on unsubscribe', () => {
  const store = createProposalStore();
  let notifications = 0;
  const off = store.subscribe(() => { notifications += 1; });
  const first = proposeRelation(createProposal(tb0, { from_ref: lockDoc, to_ref: wayfinderDoc, relation: 'supersedes' }));
  const second = proposeRelation(createProposal(tb0, { from_ref: wayfinderDoc, to_ref: lockDoc, relation: 'companion' }));
  store.put(first);
  store.put(second);
  store.put(rejectProposal(second));
  assert.equal(store.proposals().length, 2);
  assert.equal(store.find(second.proposal_ref).status, 'rejected');
  off();
  store.put(first);
  assert.equal(notifications, 3, 'no listener leak after unsubscribe');
  assert.ok(mintProposalRef().startsWith('ql.techne:canvas-proposal:'));
});
