import test from 'node:test';
import assert from 'node:assert/strict';
import { InstrumentSession } from './instrument-session.mjs';

function frame() {
  return { schema: 'ql.continuous-field/v1', event_ref: 'controlled:sky', subject_ref: 'controlled:one',
    registry_revision: 'controlled:registry', geometry_ref: 'controlled:geometry', material_ref: 'controlled:material',
    model_ref: 'controlled:model', sample_rate: 48000, generation: '1', samples_elapsed: '0',
    standing: 'controlled-transport-test-not-native-numerical-evidence', audio: [],
    clock: { field_ref: '#3-0', centre_ref: '#3-5-5/0', generation: '4',
      inscription: { turns: '0', half_degrees: 11 }, lensing: { turns: '0', half_degrees: 23 } },
    m2_identity: { event_ref: 'controlled:sky', profile_generation: 1 },
    presentation_units_per_metre: 1, amplitudes_metres: [[0, 0]],
    targets: [{ identity: 0, constituent: '#3-0', position: [0, 0, 0] }] };
}
class Context {
  sampleRate = 48000; currentTime = 0; state = 'running'; nodes = []; listeners = []; failure = false;
  destination = {};
  createGain() { return { gain: { value: 1, setValueAtTime(v) { this.value = v; } }, connect() {}, disconnect() {} }; }
  createBuffer(_channels, count) { return { data: new Float32Array(count), copyToChannel(values) { this.data.set(values); } }; }
  createBufferSource() {
    const context = this;
    const node = { playbackRate: { value: 1 }, detune: { value: 0 }, connect() {}, disconnect() { this.disconnected = true; },
      start(time) { if (context.failure) throw new Error('device failed'); this.time = time; context.nodes.push(this); },
      stop() { this.stopped = true; } };
    return node;
  }
  addEventListener(_name, cb) { this.listeners.push(cb); }
  removeEventListener(_name, cb) { this.listeners = this.listeners.filter(fn => fn !== cb); }
  change(state) { this.state = state; for (const cb of this.listeners) cb(); }
}
class Field {
  last = frame(); applications = 0; validations = 0;
  validate(value) {
    this.validations++;
    assert.equal(value.subject_ref, this.last.subject_ref);
    assert.equal(value.targets[0].identity, 0);
    assert.equal(value.targets[0].constituent, '#3-0');
    assert.ok(value.targets[0].position.every(Number.isFinite));
    assert.ok(BigInt(value.samples_elapsed) >= BigInt(this.last.samples_elapsed));
  }
  apply(value) { this.validate(value); this.last = structuredClone(value); this.applications++; }
}
function setup(options = {}) {
  const context = new Context(), field = new Field(), owner = {}, calls = [];
  let current = frame(), sequence = 0n;
  const initial = { schema: 'ql.field-host-receipt/v1', status: 'ready', available: true,
    instance_ref: 'controlled:instance', last_request_id: '0', request_id: null, field: current };
  const port = { closed: false, effect: null, delay: null,
    async request(request) {
      calls.push(structuredClone(request));
      assert.equal(request.request_id, String(++sequence));
      assert.equal(request.expected_generation, current.generation);
      assert.equal(request.expected_samples_elapsed, current.samples_elapsed);
      current = { ...structuredClone(current), audio: [] };
      if (request.command.operation === 'advance') {
        current.samples_elapsed = String(BigInt(current.samples_elapsed) + BigInt(request.command.frames));
        current.audio = Array(request.command.frames).fill(0.125);
        current.targets[0].position[0] = Number(current.samples_elapsed) / 48000;
      } else if (request.command.operation === 'set-axis') {
        current.generation = String(BigInt(current.generation) + 1n);
        current.clock[request.command.axis ? 'lensing' : 'inscription'] = request.command.phase;
        current.clock.generation = String(BigInt(current.clock.generation) + 1n);
      }
      let reply = { ...initial, status: 'ok', request_id: request.request_id,
        last_request_id: request.request_id, field: structuredClone(current) };
      if (request.command.operation === 'inspect') reply.sources = { private: 'owner-only-source' };
      if (port.effect) reply = port.effect(reply);
      if (port.delay) await port.delay;
      return reply;
    }, close() { this.closed = true; } };
  const session = new InstrumentSession({ context, owner, transport: port, initialReceipt: initial,
    fieldBinding: field, leadSeconds: 0.01, gain: 0.5, muted: false, ...options });
  return { context, field, owner, port, initial, calls, session };
}

test('native sound and field are admitted together but targets wait for END-of-block device time', async () => {
  const { session, context, field, calls } = setup();
  await session.pump();
  const time = session.reading.audio.target_context_seconds;
  assert.equal(field.applications, 0); assert.equal(calls.length, 1);
  assert.equal(context.nodes[0].time, 0.01);
  context.currentTime = time - 0.25 / 48000; session.present(); assert.equal(field.applications, 0);
  context.currentTime = time; session.present();
  assert.equal(field.last.samples_elapsed, '512'); assert.equal(field.applications, 1);
  assert.equal(session.reading.presented.samples_elapsed, '512'); session.dispose();
});

test('read-only views cannot advance, inspect private sources or obtain an audio/native control owner', async () => {
  const { session, calls } = setup({ maxViews: 2 });
  const a = session.openView(), b = session.openView();
  assert.deepEqual(Object.keys(a).sort(), ['close', 'read']);
  assert.throws(() => session.openView());
  for (let i = 0; i < 3000; i++) { a.read(); b.read(); }
  assert.equal(calls.length, 0);
  const reading = a.read(); assert.ok(!('native' in reading.audio));
  reading.acknowledged.samples_elapsed = 'foreign';
  assert.equal(b.read().acknowledged.samples_elapsed, '0');
  const sources = await session.inspect(); assert.equal(sources.private, 'owner-only-source');
  assert.ok(!JSON.stringify(a.read()).includes('owner-only-source'));
  a.close(); assert.throws(() => a.read()); session.dispose(); assert.throws(() => b.read());
});

test('one native operation is in flight even when many views and pumps request attention', async () => {
  const { session, port, calls } = setup(); let release;
  port.delay = new Promise(resolve => { release = resolve; });
  const first = session.pump(); await Promise.resolve();
  await Promise.all(Array.from({ length: 50 }, () => session.pump()));
  assert.equal(calls.length, 1); await assert.rejects(session.inspect());
  release(); await first; assert.equal(session.reading.acknowledged.samples_elapsed, '512'); session.dispose();
});

test('queue pressure stops native advancement before generating unheard or unpresentable intervals', async () => {
  const { session, calls, context } = setup({ maxBlocks: 1 });
  await session.pump(); for (let i = 0; i < 100; i++) await session.pump();
  assert.equal(calls.length, 1);
  context.currentTime = session.reading.audio.target_context_seconds;
  await session.pump(); assert.equal(calls.length, 2);
  assert.ok(session.reading.queued_blocks <= 1); session.dispose();
});

test('lookahead is a hard ceiling and an idle/hidden display does not cause an unbounded backlog', async () => {
  const { session, calls } = setup({ lookaheadSeconds: 0.04 });
  for (let i = 0; i < 100; i++) await session.pump();
  assert.equal(calls.length, 2);
  assert.ok(session.reading.audio.target_context_seconds <= 0.04);
  assert.ok(session.reading.queued_blocks <= 2); session.dispose();
});

test('domain phase operation uses the same serial owner and preserves already scheduled audio', async () => {
  const { session, context, field, calls } = setup(); await session.pump();
  const end = session.reading.audio.target_context_seconds;
  await session.operate({ operation: 'set-axis', axis: 1, phase: { turns: '-2', half_degrees: 37 } });
  assert.equal(calls.length, 2); assert.equal(context.nodes.length, 1);
  assert.equal(field.applications, 0);
  context.currentTime = end; session.present();
  assert.equal(field.last.generation, '2'); assert.equal(field.last.samples_elapsed, '512');
  assert.equal(field.last.clock.lensing.half_degrees, 37);
  assert.equal(session.reading.coalesced_presentation_frames, 1); session.dispose();
});

test('mute is presentation-only while native samples and the same material continue', async () => {
  const { session, context, calls } = setup(); session.setMuted(true); await session.pump();
  context.currentTime = session.reading.audio.target_context_seconds; session.setMuted(false); await session.pump();
  assert.equal(calls.length, 2); assert.ok(calls.every(c => c.command.muted === false));
  assert.equal(session.reading.acknowledged.samples_elapsed, '1024'); session.dispose();
});

test('provider failure and late reply never become another clean owner or implicit retry', async () => {
  const { session, port, calls } = setup({ timeoutMs: 100 }); let release;
  port.delay = new Promise(resolve => { release = resolve; });
  await assert.rejects(session.pump(), /timed out/);
  assert.equal(session.reading.available, false); assert.equal(port.closed, true);
  release(); await Promise.resolve(); await Promise.resolve();
  assert.equal(session.reading.acknowledged.samples_elapsed, '0');
  await session.pump(); await assert.rejects(session.recover('cannot-recover-lost-ack'));
  assert.equal(calls.length, 1); session.dispose();
});

test('malformed/foreign/reordered replies fail before scheduling sound or changing targets', async () => {
  for (const mutate of [r => { r.instance_ref = 'foreign'; }, r => { r.request_id = '2'; },
    r => { r.field.subject_ref = 'foreign'; }, r => { r.field.samples_elapsed = '1024'; },
    r => { r.field.targets[0].position[0] = NaN; }, r => { r.field.audio.pop(); },
    r => { r.available = false; }]) {
    const { session, port, context, field } = setup();
    port.effect = r => { mutate(r); return r; };
    await assert.rejects(session.pump());
    assert.equal(context.nodes.length, 0); assert.equal(field.applications, 0);
    assert.equal(session.reading.acknowledged.samples_elapsed, '0'); session.dispose();
  }
});

test('presentation failure keeps actual native acknowledgement and explicit read recovery loses no source identity', async () => {
  const { session, context, calls, field } = setup(); context.failure = true;
  await assert.rejects(session.pump(), /device failed/);
  assert.equal(session.reading.acknowledged.samples_elapsed, '512');
  assert.equal(session.reading.available, true); assert.equal(session.reading.held, true);
  assert.equal(field.applications, 0);
  context.failure = false; await session.recover('device-return-current-read');
  assert.equal(calls[1].command.operation, 'read'); assert.equal(calls[1].expected_samples_elapsed, '512');
  await session.pump(); assert.equal(session.reading.audio.interval.native_start, '512'); session.dispose();
});

test('a hold during native work retains the returned native cursor but does not resume presentation', async () => {
  const { session, port, context } = setup(); let release;
  port.delay = new Promise(resolve => { release = resolve; });
  const pumping = session.pump(); await Promise.resolve(); session.hold('user-held'); release(); await pumping;
  assert.equal(session.reading.held, true); assert.equal(session.reading.acknowledged.samples_elapsed, '512');
  assert.equal(context.nodes.length, 0); port.delay = null;
  await session.recover('user-return'); await session.pump(); assert.equal(context.nodes.length, 1); session.dispose();
});

test('context interruption clears queued targets and requires explicit native read before re-entry', async () => {
  const { session, context, calls } = setup(); await session.pump(); context.change('suspended');
  await session.pump(); assert.equal(session.reading.held, true); assert.equal(session.reading.queued_blocks, 0);
  context.change('running'); await session.pump(); assert.equal(calls.length, 1);
  await session.recover('audio-context-restored'); await session.pump();
  assert.equal(calls[1].command.operation, 'read'); assert.equal(session.reading.audio.device_epoch, 1); session.dispose();
});

test('ordinary updates remain bounded across thousands of blocks and independently readable subjects', async () => {
  const a = setup(), b = setup();
  for (let i = 0; i < 3000; i++) {
    a.context.currentTime = Math.max(0, 0.01 + (i * 512 - 128) / 48000);
    await a.session.pump();
    assert.ok(a.session.reading.queued_blocks <= 16 && a.session.reading.queued_bytes <= 16 * 1024 * 1024);
  }
  assert.equal(a.session.reading.acknowledged.samples_elapsed, String(3000 * 512));
  assert.equal(b.calls.length, 0); assert.equal(b.session.reading.acknowledged.samples_elapsed, '0');
  a.session.dispose(); b.session.dispose();
});

test('a duplicate driver or invalid rate/queue configuration cannot create another audio owner', () => {
  const { session, context, owner, port, initial, field } = setup();
  assert.throws(() => new InstrumentSession({ context, owner, transport: port, initialReceipt: initial, fieldBinding: field }));
  session.dispose();
  assert.throws(() => new InstrumentSession({ context, owner, transport: port, initialReceipt: initial,
    fieldBinding: field, blockFrames: 8192, lookaheadSeconds: 0.01 }));
});
