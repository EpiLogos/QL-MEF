import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { beats } from '../src/techne/journey/beats.ts';
import {
  applyVakTraversal,
  bindVak,
  CS_PASSAGES,
  sceneCPrime,
  vakPlayOrder,
  vakWarranted,
} from '../src/techne/journey/vak.ts';
import {
  PROVING_CONTEXT_FRAME,
  PROVING_SCENE_CROSSING,
  PROVING_SCENE_GROUND,
  PROVING_SCENE_RETURN,
  PROVING_VAK_SOURCE,
  provingReading,
} from '../src/techne/journey/proving-reading.ts';

const reading = provingReading();
const model = beats(reading);
const SCENES = [PROVING_SCENE_GROUND, PROVING_SCENE_CROSSING, PROVING_SCENE_RETURN];

/** The proving assignment: ground → 4.0, crossing → 4.2, return → 4.5. */
function binding(overrides = {}) {
  return {
    cpf: 'human-engaged',
    cfp: 'CFP0',
    cs: 'CS0',
    direction: 'forward-synthesis',
    cf_ref: PROVING_CONTEXT_FRAME,
    cp: [
      { scene_ref: PROVING_SCENE_GROUND, cp: '4.0' },
      { scene_ref: PROVING_SCENE_CROSSING, cp: '4.2' },
      { scene_ref: PROVING_SCENE_RETURN, cp: '4.5' },
    ],
    ct: [{ scene_ref: PROVING_SCENE_CROSSING, ct: 'CT1' }],
    ...overrides,
  };
}

function bound(overrides = {}) {
  const candidate = binding(overrides);
  const gate = bindVak(candidate, reading, model.beats);
  assert.equal(gate.reason, null, `the proving binding must be accepted: ${gate.reason ?? ''}`);
  return { binding: candidate, traversal: applyVakTraversal(gate.binding, model.beats) };
}

test('the CS passages are the canonical kernel pairs, transcribed exactly', async () => {
  const kernel = await readFile(
    fileURLToPath(new URL('../../../../../crates/ql-mef/src/cprime_oikonomia.rs', import.meta.url)),
    'utf8',
  );
  const source = kernel.slice(kernel.indexOf('const fn pairs(self)'), kernel.indexOf('pub fn passage'));
  assert.ok(source.includes('Self::FullTraverse'), 'the pairs block was located in the accepted producer');
  for (const [profile, pairs] of Object.entries(CS_PASSAGES)) {
    const literal = pairs.map(([left, right]) => `(${left}, ${right})`).join(', ');
    assert.ok(source.includes(literal), `${profile} pairs ${literal} must match crates/ql-mef/src/cprime_oikonomia.rs`);
  }
});

test('a C′ binding exists only over a warranted Vāk reading — absence is honest, not simulated', () => {
  assert.equal(vakWarranted(reading), true, 'the proving reading warrants ql.vak_source_ref + context_frame_ref');
  const unwarranted = { ...reading, ql: undefined };
  assert.equal(vakWarranted(unwarranted), false);
  const gate = bindVak(binding(), unwarranted, model.beats);
  assert.match(gate.reason, /no warranted Vāk binding/);
});

test('CF is carried from the warrant, never invented: a foreign context frame is refused', () => {
  const gate = bindVak(binding({ cf_ref: 'mef:context-frame:CF2' }), reading, model.beats);
  assert.match(gate.reason, /warranted context frame/);
});

test('C′ annotations attach to disclosed scenes only; every scene needs a CP or no binding at all', () => {
  const undisclosed = bindVak(
    binding({ cp: [{ scene_ref: 'oi:expression:l5-dual-reading/scene/nowhere', cp: '4.1' }] }),
    reading,
    model.beats,
  );
  assert.match(undisclosed.reason, /not disclosed by reading/);
  const incomplete = bindVak(binding({ cp: binding().cp.slice(0, 2) }), reading, model.beats);
  assert.match(incomplete.reason, /no binding at all/);
});

test('CS materially changes the traversal (V1): CS0 forward visits ground → return → crossing; CS5 leaves the middle out', () => {
  const full = bound();
  assert.deepEqual(vakPlayOrder(full.traversal), [PROVING_SCENE_GROUND, PROVING_SCENE_RETURN, PROVING_SCENE_CROSSING],
    'CS0 forward: hop 4.0→4.5 plays ground then return; hop 4.2→4.3 plays crossing; the 4.3→4.2 return leg does not replay it');
  assert.deepEqual(full.traversal.unvisited, [], 'CS0 visits every position');

  const direct = bound({ cs: 'CS5' });
  assert.deepEqual(vakPlayOrder(direct.traversal), [PROVING_SCENE_GROUND, PROVING_SCENE_RETURN],
    'CS5 direct synthesis: 4.0→4.5 then 4.5→4.0 (both already visited)');
  assert.deepEqual(direct.traversal.unvisited, [PROVING_SCENE_CROSSING],
    'the passage does not visit 4.2 — disclosed as unvisited, never silently dropped');
});

test('direction materially changes the traversal: returning-inquiry walks the passage the other way', () => {
  const returning = bound({ direction: 'returning-inquiry' });
  assert.deepEqual(vakPlayOrder(returning.traversal), [PROVING_SCENE_RETURN, PROVING_SCENE_GROUND, PROVING_SCENE_CROSSING],
    'CS0 returning-inquiry begins at 4.5 → 4.0: return, then ground; then 4.2 → 4.3: crossing');
});

test('CFP materially changes the step structure: CFP0 one voice separates hop positions, CFP1 chords them', () => {
  const voice = bound({ cfp: 'CFP0' });
  assert.deepEqual(voice.traversal.steps.map((step) => step.scene_refs), [
    [PROVING_SCENE_GROUND],
    [PROVING_SCENE_RETURN],
    [PROVING_SCENE_CROSSING],
  ], 'one voice: each hop walks from-then-to as separate steps');

  const chord = bound({ cfp: 'CFP1' });
  assert.deepEqual(chord.traversal.steps.map((step) => step.scene_refs), [
    [PROVING_SCENE_GROUND, PROVING_SCENE_RETURN],
    [PROVING_SCENE_CROSSING],
  ], 'chord: hop 1 plays 4.0 and 4.5 as ONE step');
  assert.equal(chord.traversal.steps[0].chord, true);
});

test('CFP2 chained performs only chain-compatible passages; CS0 is refused with its reason', () => {
  const chain = bound({ cfp: 'CFP2', cs: 'CS5' });
  assert.deepEqual(vakPlayOrder(chain.traversal), [PROVING_SCENE_GROUND, PROVING_SCENE_RETURN],
    'CS5 hands each hop its from-position: (0,5) → (5,0)');
  const refused = bindVak(binding({ cfp: 'CFP2', cs: 'CS0' }), reading, model.beats);
  assert.match(refused.reason, /chain-compatible|hand each next hop/);
});

test('CFP3 appends the fused whole; CFP4 sustains the last step; CFP5 is refused inside a single traversal', () => {
  const fusion = bound({ cfp: 'CFP3' });
  const last = fusion.traversal.steps.at(-1);
  assert.equal(last.fusion, true);
  assert.deepEqual(last.scene_refs, SCENES, 'the fused step re-visits every scene the passage visited');

  const sustained = bound({ cfp: 'CFP4' });
  assert.equal(sustained.traversal.steps.at(-1).sustained, true, 'the drone sustains the final step');
  assert.equal(sustained.traversal.steps[0].sustained, false);

  const nested = bindVak(binding({ cfp: 'CFP5' }), reading, model.beats);
  assert.match(nested.reason, /parent composition owner/, 'nested/canon needs the parent owner — recorded, not simulated');
});

test('a scene plays at its FIRST visit only — no passage replays a scene', () => {
  const traversal = bound().traversal;
  const played = traversal.steps.flatMap((step) => step.scene_refs);
  assert.equal(new Set(played).size, played.length, 'no repeats inside the traversal');
});

test('scene C′ annotations are readable per scene', () => {
  const gate = bindVak(binding(), reading, model.beats);
  assert.deepEqual(sceneCPrime(gate.binding, PROVING_SCENE_CROSSING), { cp: '4.2', ct: 'CT1' });
  assert.deepEqual(sceneCPrime(gate.binding, PROVING_SCENE_GROUND), { cp: '4.0', ct: null });
});

test('the binding rides the draft, not a store: binding and traversal mutate nothing', () => {
  const before = structuredClone(reading);
  const candidate = binding();
  bound();
  assert.deepEqual(reading, before, 'the reading is never mutated by binding');
  assert.deepEqual(candidate.cp.length, 3);
});
