import test from 'node:test';
import assert from 'node:assert/strict';
import * as THREE from 'three';
import { FocusedInstrumentSession } from './focused-instrument-session.mjs';

function frame() {
  return { schema: 'ql.continuous-field/v1', event_ref: 'event:one', subject_ref: 'subject:nara',
    registry_revision: 'registry:r1', geometry_ref: 'geometry:one', material_ref: 'material:one',
    model_ref: 'model:one', sample_rate: 48000, generation: '1', samples_elapsed: '0',
    standing: 'controlled-k9-browser-test-not-physical-evidence', audio: [], presentation_units_per_metre: 1,
    clock: { field_ref: '#3-0', centre_ref: '#3-5-5/0', generation: '1',
      inscription: { turns: '0', half_degrees: 1 }, lensing: { turns: '0', half_degrees: 2 } },
    m2_identity: { event_ref: 'event:one', profile_generation: 1 }, amplitudes_metres: [[0, 0]],
    targets: [
      { identity: 1, constituent: '#1', position: [-0.25, 0, 0] },
      { identity: 2, constituent: '#2', position: [0.25, 0, 0] },
    ] };
}
function snapshot(current = frame()) {
  const cursor = { event_ref: current.event_ref, subject_ref: current.subject_ref, profile_generation: 1,
    field_generation: current.generation, samples_elapsed: current.samples_elapsed };
  return { schema: 'ql.focused-instrument/v1', available: true,
    event: { event_ref: current.event_ref, subject_ref: current.subject_ref, profile_generation: 1,
      registry_revision: current.registry_revision }, live_cursor: { ...cursor }, presented_cursor: { ...cursor },
    temporal: 'live', tracking: 'follow', selection: null, selection_standing: null, selected_target: null,
    focus: { focus: 'm5', available: true, current: true, source_refs: ['source:m5'], payload: {}, standing: 'controlled' },
    clock: { presentation: { view: 'assembled' }, owner_clock: current.clock, field_ref: '#3-0', centre_ref: '#3-5-5/0', standing: 'controlled' },
    vak_expression: null, vak_performance: null, personal_current: false, standing: 'controlled' };
}
function selection() {
  return { contract: 'ql.focused-bimba-selection/v1', selection_ref: 'bimba:#2', coordinate_ref: '#2',
    source_ref: '#2', source_revision: 'registry:r1', disclosure_ref: 'disclosure:#2', subject_ref: 'subject:nara',
    field_constituent_ref: '#2', assertion_refs: ['relation:one'] };
}
function bimba() {
  return { contract: 'ql.focused-host-bimba-navigation/v1', source_revision: 'registry:r1', selected_ref: null,
    items: [{ selection: selection(), label: 'Parāśakti', face: 'bimba', depth: 1, relation_refs: ['relation:one'] }],
    standing: 'controlled-rooted-bimba' };
}
class Context {
  sampleRate = 48000; currentTime = 0; state = 'running'; destination = {}; nodes = []; listeners = [];
  createGain() { return { gain: { value: 1, setValueAtTime(v) { this.value = v; } }, connect() {}, disconnect() {} }; }
  createBuffer(_channels, count) { return { data: new Float32Array(count), copyToChannel(values) { this.data.set(values); } }; }
  createBufferSource() {
    const context = this;
    return { playbackRate: { value: 1 }, detune: { value: 0 }, connect() {}, disconnect() {}, stop() {},
      start(time) { this.time = time; context.nodes.push(this); } };
  }
  addEventListener(_name, cb) { this.listeners.push(cb); }
  removeEventListener(_name, cb) { this.listeners = this.listeners.filter(value => value !== cb); }
}
function texture(width = 2, height = 1) {
  const data = new Float32Array(width * height * 4);
  for (let i = 0; i < width * height; i++) data[i * 4 + 3] = 1;
  const result = new THREE.DataTexture(data, width, height, THREE.RGBAFormat, THREE.FloatType);
  result.needsUpdate = true;
  return result;
}
function setup() {
  const context = new Context(); let current = frame(), view = snapshot(current), seq = 0n;
  const calls = [], targetWrites = [], recovery = new Set(); let checkpointCount = 0, restoreCount = 0, paused = false;
  const nav = bimba();
  const ready = { schema: 'ql.focused-host-receipt/v1', instance_ref: 'focused:one', request_id: null,
    last_request_id: '0', status: 'ready', available: true, error: null, snapshot: view, bimba: nav,
    owner_receipt: null, standing: 'controlled' };
  const transport = { closed: false, refuseNext: false,
    async request(request) {
      calls.push(structuredClone(request));
      assert.equal(request.request_id, String(++seq));
      assert.equal(request.expected_generation, view.live_cursor.field_generation);
      assert.equal(request.expected_samples_elapsed, view.live_cursor.samples_elapsed);
      if (this.refuseNext) {
        this.refuseNext = false;
        return { ...ready, status: 'refused', request_id: request.request_id, last_request_id: request.request_id,
          error: 'controlled-refusal', snapshot: structuredClone(view), bimba: undefined };
      }
      const command = request.command; let owner = null;
      if (command.operation === 'set-focus') view.focus = { ...view.focus, focus: command.focus };
      else if (command.operation === 'select-bimba') {
        assert.equal(command.selection_ref, 'bimba:#2');
        view.selection = selection(); view.selection_standing = 'current';
        view.selected_target = structuredClone(current.targets[1]); nav.selected_ref = command.selection_ref;
      } else if (command.operation === 'advance') {
        current = { ...structuredClone(current), audio: [] };
        if (command.frames > 0) {
          current.samples_elapsed = String(BigInt(current.samples_elapsed) + BigInt(command.frames));
          current.audio = Array(command.frames).fill(0.05);
          current.targets[1].position[1] = Number(current.samples_elapsed) / 48000;
        }
        owner = structuredClone(current);
      } else if (command.operation === 'set-clock-axis') {
        current = { ...structuredClone(current), generation: String(BigInt(current.generation) + 1n), audio: [] };
        current.clock[command.axis ? 'lensing' : 'inscription'] = command.phase; owner = structuredClone(current);
      }
      const previousView = view;
      view = snapshot(current);
      view.focus = { ...view.focus, focus: previousView.focus.focus };
      view.tracking = previousView.tracking;
      if (previousView.selection) {
        view.selection = structuredClone(previousView.selection);
        view.selection_standing = command.operation === 'advance' && command.frames > 0
          ? 'field-advanced' : previousView.selection_standing;
        const target = current.targets.find(item => item.constituent === previousView.selection.field_constituent_ref);
        view.selected_target = target ? structuredClone(target) : null;
      }
      if (command.operation === 'set-focus') view.focus = { ...view.focus, focus: command.focus };
      if (command.operation === 'select-bimba') {
        view.selection = selection(); view.selection_standing = 'current'; view.selected_target = structuredClone(current.targets[1]);
      }
      return { ...ready, status: 'ok', request_id: request.request_id, last_request_id: request.request_id,
        error: null, snapshot: structuredClone(view), bimba: command.operation === 'bimba' ? structuredClone(nav) : undefined,
        owner_receipt: owner };
    }, close() { this.closed = true; } };
  const targetA = texture(), targetB = texture();
  const port = { texWidth: 2, texHeight: 1, particleCount: 2, targetA, targetB,
    currentPosTarget: {}, currentVelTarget: {}, nextPosTarget: {}, nextVelTarget: {},
    setTargetTextures(a, b, centre) { targetWrites.push({ a, b, centre }); } };
  const lease = { retainedTargetPort: () => port,
    checkpointRetainedField(binding) { checkpointCount++; return { schema: 'controlled-checkpoint', receipt: binding.lastReceipt, index: checkpointCount }; },
    restoreRetainedField(_binding, checkpoint) { assert.equal(checkpoint.schema, 'controlled-checkpoint'); restoreCount++; return lease; },
    onRecoveryRequired(listener) { recovery.add(listener); return () => recovery.delete(listener); },
    inspect: () => ({ paused }), pause(value = true) { paused = value; return lease; }, resume() { paused = false; return lease; }, renderOnce() { return lease; } };
  const session = new FocusedInstrumentSession({ ref: 'focused-source:one', title: 'Epi / Nara', transport,
    initialReceipt: ready, audioContext: context, correspondence: { sourceRef: 'mapping:fixture', revision: 'r1', slotsA: [0, 1], slotsB: [1, 0] },
    audio: { gain: 0.5, muted: false, leadSeconds: 0.01 }, accompanying: { ref: 'agent-session:epii', project: 'QL-MEF', space: 'epi-nara' } });
  return { context, transport, calls, lease, session, recovery, targetWrites,
    get checkpointCount() { return checkpointCount; }, get restoreCount() { return restoreCount; }, get paused() { return paused; } };
}
const delay = ms => new Promise(resolve => setTimeout(resolve, ms));

test('focus, Bimba and native field work share one exact focused-host sequence', async () => {
  const s = setup();
  const detach = await s.session.attachExpression(s.lease);
  assert.equal(s.calls[0].command.operation, 'advance'); assert.equal(s.calls[0].command.frames, 0);
  assert.equal(s.targetWrites.length, 1); assert.equal(s.checkpointCount, 1);
  await s.session.command({ kind: 'set-focus', focus: 'm2' });
  await s.session.command({ kind: 'receive-personal', input: { event_ref: 'event:test', receivers: [] } });
  await s.session.command({ kind: 'select-bimba', selection: selection() });
  const result = await s.session.command({ kind: 'advance', frames: 128, muted: false });
  assert.equal(result.standing, 'applied');
  assert.deepEqual(s.calls.map(call => call.request_id), ['1', '2', '3', '4', '5']);
  assert.deepEqual(s.calls.map(call => call.command.operation), ['advance', 'set-focus', 'receive-personal', 'select-bimba', 'advance']);
  assert.equal((await s.session.read()).focus.focus, 'm2'); // encounter focus survives field advancement
  assert.equal((await s.session.readBimba()).selected_ref, 'bimba:#2');
  assert.equal(s.context.nodes.length, 1);
  s.context.currentTime = s.session.reading.audio.target_context_seconds;
  await delay(15);
  assert.ok(s.checkpointCount >= 2); assert.equal(s.session.reading.native_cursor.samples_elapsed, '128');
  detach(); s.session.dispose();
});

test('refusal consumes one sequence without changing native cursor or inventing retry', async () => {
  const s = setup(); const detach = await s.session.attachExpression(s.lease);
  const before = s.session.reading.native_cursor;
  s.transport.refuseNext = true;
  const result = await s.session.command({ kind: 'explode-clock', pair: 3 });
  assert.equal(result.standing, 'refused'); assert.match(result.error, /controlled-refusal/);
  assert.deepEqual(s.session.reading.native_cursor, before);
  assert.equal(s.calls.length, 2); assert.equal(s.calls[1].request_id, '2');
  detach(); s.session.dispose();
});

test('context loss holds immediately; restoration uses checkpoint plus zero-frame native acknowledgement', async () => {
  const s = setup(); const detach = await s.session.attachExpression(s.lease);
  for (const listener of s.recovery) listener('lost');
  await Promise.resolve();
  assert.equal(s.paused, true); assert.equal(s.session.reading.recovering, true);
  const before = s.calls.length;
  for (const listener of s.recovery) listener('restored');
  for (let i = 0; i < 20 && s.calls.length === before; i++) await delay(5);
  assert.equal(s.calls.length, before + 1);
  assert.equal(s.calls.at(-1).command.operation, 'advance'); assert.equal(s.calls.at(-1).command.frames, 0);
  for (let i = 0; i < 20 && s.session.reading.recovering; i++) await delay(5);
  assert.equal(s.restoreCount, 1); assert.equal(s.session.reading.recovering, false); assert.equal(s.paused, false);
  assert.equal(s.session.reading.audio.device_epoch, 1);
  detach(); s.session.dispose();
});

test('sample correspondence is mandatory and source-qualified rather than silently generated', () => {
  const s = setup(); s.session.dispose();
  assert.throws(() => new FocusedInstrumentSession({ ref: 'focused:bad', transport: s.transport,
    initialReceipt: { schema: 'ql.focused-host-receipt/v1', status: 'ready', available: true, instance_ref: 'x',
      last_request_id: '0', snapshot: snapshot(), bimba: bimba() }, correspondence: { slotsA: [0], slotsB: [0] } }),
    /correspondence source/);
});
