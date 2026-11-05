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
  const port = { closed: false, effect: null, delay: null, renders: 0, waiting: [],
    async request(request) {
      calls.push(structuredClone(request));
      assert.equal(request.request_id, String(++sequence));
      assert.equal(request.expected_generation, current.generation);
      assert.equal(request.expected_samples_elapsed, current.samples_elapsed);
      current = { ...structuredClone(current), audio: [] };
      if (port.refuseOperation === request.command.operation) {
        return { ...initial, status: 'refused', request_id: request.request_id,
          last_request_id: request.request_id, error: 'controlled refusal', field: structuredClone(current) };
      }
      // Controlled protocol case (host.rs torus_operation): a setting sent for a later sample waits for it, and moves nothing.
      const waits = request.command.operation === 'torus-set' && request.command.at_samples_elapsed !== undefined &&
        BigInt(request.command.at_samples_elapsed) > BigInt(current.samples_elapsed);
      let landed = null;
      if (waits) port.waiting.push({ at_samples_elapsed: request.command.at_samples_elapsed, edit: request.command.edit });
      if (request.command.operation === 'advance') {
        current.samples_elapsed = String(BigInt(current.samples_elapsed) + BigInt(request.command.frames));
        current.audio = Array(request.command.frames).fill(0.125);
        current.targets[0].position[0] = Number(current.samples_elapsed) / 48000;
        // The block that reaches a waiting setting's sample makes it there (scene_field.rs advance); a Fundamental is one
        // modes continuation inside the block.
        const due = port.waiting.filter(set => BigInt(set.at_samples_elapsed) <= BigInt(current.samples_elapsed));
        port.waiting = port.waiting.filter(set => !due.includes(set));
        if (due.length) {
          landed = due.map(set => ({ at_samples_elapsed: set.at_samples_elapsed, edit: set.edit, applied: true,
            moved: Object.keys(set.edit) }));
          current.generation = String(BigInt(current.generation) + (port.landingStep ??
            BigInt(landed.filter(set => set.moved.includes('fundamental')).length)));
        }
      } else if (waits) {
        // Nothing moves before its sample.
      } else if (['m1-advance', 'replace-event'].includes(request.command.operation)) {
        // Controlled protocol cases; actual scene-owner evidence is separate.
        current.generation = String(BigInt(current.generation) +
          (port.eventStep ?? (request.command.operation === 'm1-advance' ? 3n : 2n)));
        current.targets[0].position[1] += 0.5;
      } else if (request.command.operation === 'set-axis') {
        current.generation = String(BigInt(current.generation) + 1n);
        current.clock[request.command.axis ? 'lensing' : 'inscription'] = request.command.phase;
        current.clock.generation = String(BigInt(current.clock.generation) + 1n);
      } else if (request.command.operation === 'strike' || request.command.operation === 'strike-at') {
        current.generation = String(BigInt(current.generation) + 1n);
        current.amplitudes_metres[0][0] += 0.01;
        current.targets[0].position[2] += 0.25;
      } else if (request.command.operation === 'torus-play' || request.command.operation === 'torus-singlet' || (request.command.operation === 'torus-set' &&
        request.command.edit?.fundamental !== undefined && port.fundamentalMoves !== false)) {
        // Controlled protocol case (host.rs torus_operation): a played Torus act
        // is one strike at points; a Fundamental edit one modes continuation.
        current.generation = String(BigInt(current.generation) + (port.torusStep ?? 1n));
        current.amplitudes_metres[0][0] += 0.01;
      } else if (request.command.operation === 'torus-body') {
        // Controlled protocol case (host.rs torus_operation, O-I #852): choosing the body is one body act; it rings from rest.
        current.generation = String(BigInt(current.generation) + (port.bodyStep ?? 1n));
        current.amplitudes_metres[0][0] = 0;
      } else if (request.command.operation === 'torus-restore') {
        // Controlled protocol case (host.rs torus_operation): a restore is the event's modes (and shape) continuation when it
        // moved, the damping when it moved, and the body's state: one to four generations, no samples and no PCM.
        current.generation = String(BigInt(current.generation) + (port.restoreStep ?? 2n));
        current.amplitudes_metres[0][0] = request.command.checkpoint.amplitudes?.[0]?.[0] ?? 0;
      } else if (request.command.operation === 'stage-evaluate') {
        // Controlled protocol case: the host's own compiled plan applies 1..N
        // admitted applications in one envelope, so the generation moves by
        // the plan's own steps while samples and PCM stay put.
        current.generation = String(BigInt(current.generation) + (port.stageStep ?? 2n));
        current.targets[0].position[2] += 0.125;
      }
      let reply = { ...initial, status: 'ok', request_id: request.request_id,
        last_request_id: request.request_id, field: structuredClone(current) };
      if (request.command.operation === 'inspect') reply.sources = { private: 'owner-only-source' };
      if (['influence', 'm1-advance', 'replace-event'].includes(request.command.operation))
        reply.influence = { schema: 'ql.expression-influence/v1', generation: current.generation };
      if (['stage-state', 'stage-evaluate'].includes(request.command.operation))
        reply.stage = port.stageBody ?? { schema: 'ql.stage-state/v1', subject_ref: current.subject_ref, slots: {} };
      // Controlled protocol case: retire, bind and unbind answer with their
      // receipt and the field unchanged (host.rs stage_operation).
      if (['stage-retire', 'stage-bind', 'stage-unbind'].includes(request.command.operation))
        reply.stage = port.stageBody ?? { schema: 'ql.stage-receipt/v1', procedure_ref: request.command.procedure_ref ?? request.command.procedure?.procedure_ref, applied: true };
      // Controlled protocol case: the Tarot score reads answer with the held
      // score, its currentness and the event it was resolved against, the
      // field unchanged (host.rs score_operation).
      if (['score-resolve', 'score-state'].includes(request.command.operation)) {
        reply.score = port.scoreBody ?? { schema: 'ql.tarot-score/v1', subject_ref: current.subject_ref, tokens: [] };
        reply.current = port.scoreCurrent ?? true;
        reply.resolved_event_ref = port.scoreEventRef ?? current.event_ref;
      }
      if (['torus-state', 'torus-set', 'torus-play', 'torus-singlet', 'torus-release', 'torus-checkpoint', 'torus-restore'].includes(request.command.operation))
        reply.torus = { schema: 'ql.m1-prime-session/v1', comma_cents: 0, band: 'rhythm', notes: [] };
      if (request.command.operation === 'torus-body') {
        reply.torus = { schema: 'ql.m1-prime-session/v1', comma_cents: 0, band: 'rhythm', notes: [],
          body: request.command.body === null ? { kind: 'torus' } : { kind: 'form', prepared: { schema: 'ql.physical-body-preparation/v1' } } };
        reply.influence = { schema: 'ql.expression-influence/v1', generation: current.generation, tuning: 'logic' };
      }
      if (request.command.operation === 'torus-set')
        reply.influence = { schema: 'ql.expression-influence/v1', generation: current.generation, tuning: 'logic' };
      if (request.command.operation === 'torus-checkpoint')
        reply.torus_checkpoint = port.checkpointBody ?? { schema: 'ql.torus-checkpoint/v1', taken_at_samples: current.samples_elapsed,
          sample_rate: 48000, amplitudes: [[0.25, -0.5]], voices: ['#3-0'] };
      if (request.command.operation === 'torus-render') {
        // Controlled protocol case (host.rs torus_operation): the render owner's sound, where it ended and its continuation.
        const frames = request.command.frames;
        reply.torus_render = port.renderBody ?? { schema: 'ql.torus-render/v1', from_samples: request.command.checkpoint?.taken_at_samples ?? '9600',
          frames, sample_rate: 48000, audio: Array(frames).fill(0.25), made_way: request.command.acts.filter(act => act.action === 'strike').map(() => []),
          checkpoint: { schema: 'ql.torus-checkpoint/v1', taken_at_samples: '0' }, continues: `render:${++port.renders}` };
      }
      if (request.command.operation === 'torus-restore')
        reply.influence = { schema: 'ql.expression-influence/v1', generation: current.generation, tuning: 'logic' };
      // Controlled protocol case (host.rs torus_operation, O-I 856): the receiver is a receiving setting; the field is unchanged.
      if (request.command.operation === 'torus-receiver') {
        const { receiver, context } = request.command;
        const heard = context ?? (receiver === null ? 'world' : receiver.shared === undefined ? 'personal' : 'shared');
        reply.torus = { schema: 'ql.m1-prime-session/v1', comma_cents: 0, band: 'rhythm', notes: [] };
        reply.torus_receiver = { schema: 'ql.torus-receiver/v1', context: heard,
          gains: [{ mode_ref: '#3-0', gain: heard === 'world' ? 1 : 0.5 }], law: 'controlled' };
        if (receiver?.shared !== undefined) Object.assign(reply.torus_receiver, { shared: receiver.shared, at_metres: receiver.at_metres });
      }
      // Controlled protocol case (host.rs torus_operation, O-I #849): the Torus handed over to a native host; a read.
      if (request.command.operation === 'torus-native')
        reply.torus_native = port.nativeBody ?? { schema: 'ql.torus-native/v1', at_samples_elapsed: current.samples_elapsed,
          generation: current.generation, continuation: { schema: 'ql.continuous-handover/v1' }, keys: { contract: 'ql.torus-keys/v1' } };
      if (landed) {
        reply.torus_set_landed = landed;
        reply.torus = { schema: 'ql.m1-prime-session/v1', comma_cents: 0, band: 'rhythm', notes: [], waiting: port.waiting };
        reply.influence = { schema: 'ql.expression-influence/v1', generation: current.generation, tuning: 'logic' };
      }
      if (request.command.operation === 'torus-play')
        reply.torus_play = { schema: 'ql.torus-play/v1', notes: request.command.keys.map((key, id) => ({ id, key })), made_way: [] };
      // Controlled protocol case (host.rs torus_operation, O-I #771): a singlet is one strike at its codon's places.
      if (request.command.operation === 'torus-singlet')
        reply.torus_singlet = { schema: 'ql.torus-singlet/v1', codon: request.command.codon,
          places: request.command.codon === 8 ? [{ lens: 0, arc: 9 }, { lens: 0, arc: 10 }] : [{ lens: 0, arc: 6 }], points: [] };
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

test('an empty rebased device origin does not count as queued lookahead', async () => {
  const { session, context, calls } = setup({ blockFrames: 8192, leadSeconds: 0.5, lookaheadSeconds: 0.5 });
  try {
    await session.recover('source admission finished');
    assert.equal(session.reading.audio.scheduled_blocks, 0);
    await session.pump();
    assert.deepEqual(calls.map(call => call.command.operation), ['read', 'advance']);
    assert.equal(context.nodes.length, 1);
    assert.equal(context.nodes[0].time, 0.5);
    assert.equal(session.reading.acknowledged.samples_elapsed, '8192');
  } finally { session.dispose(); }
});

test('a late acknowledged PCM block realigns only the device clock without replaying native advance', async () => {
  const { session, context, calls, field } = setup({ blockFrames: 8192, leadSeconds: 0.5, lookaheadSeconds: 0.5 });
  try {
    await session.pump();
    context.currentTime = 0.7; // the prior interval has ended during a main-thread stall
    await session.pump();
    assert.equal(calls.length, 2);
    assert.ok(calls.every(call => call.command.operation === 'advance'));
    assert.equal(session.reading.acknowledged.samples_elapsed, '16384');
    assert.equal(session.reading.audio.native_origin, '8192');
    assert.equal(session.reading.audio.device_epoch, 1);
    assert.equal(session.reading.audio.scheduled_blocks, 1);
    assert.equal(context.nodes.length, 2);
    assert.equal(context.nodes[1].time, 1.2);
    assert.equal(field.last.samples_elapsed, '8192');
    context.currentTime = session.reading.audio.target_context_seconds;
    session.present();
    assert.equal(field.last.samples_elapsed, '16384');
    assert.equal(session.reading.held, false);
  } finally { session.dispose(); }
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
    r => { r.field.audio[0] = NaN; }, r => { r.field.clock.generation = Number.MAX_SAFE_INTEGER + 1; },
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

test('a played strike moves the standing body and cursor as a performance act', async () => {
  const { session, context, field, calls } = setup(); await session.pump();
  const end = session.reading.audio.target_context_seconds;
  const elapsed = session.reading.acknowledged.samples_elapsed;
  assert.equal(session.lastInfluence, null);
  await session.strike([{ mode_ref: 'scene:planet/#2-5-4', amplitude: [0.9, 0] }]);
  assert.deepEqual(calls.at(-1).command,
    { operation: 'strike', strikes: [{ mode_ref: 'scene:planet/#2-5-4', amplitude: [0.9, 0] }] });
  assert.equal(session.reading.acknowledged.generation, '2');
  assert.equal(session.reading.acknowledged.samples_elapsed, elapsed,
    'a played strike elapses no samples');
  // A performance act is not a determinant: no influence rides the reply, and
  // the next data-plane advance continues the same owner.
  assert.equal(session.lastInfluence, null);
  context.currentTime = end; session.present();
  assert.equal(field.last.targets[0].position[2], 0.25);
  await session.pump();
  assert.equal(calls.at(-1).command.operation, 'advance');
  assert.equal(session.reading.acknowledged.generation, '2');
  assert.equal(session.reading.acknowledged.samples_elapsed, String(BigInt(elapsed) + 512n));
  session.dispose();
});

test('a malformed played strike is refused before any native exchange', async () => {
  const { session, calls } = setup(); await session.pump();
  for (const bad of [
    [], 'strike', [{ mode_ref: '', amplitude: [0, 0] }],
    [{ mode_ref: 'scene:planet/#2-5-4', amplitude: [0] }],
    [{ mode_ref: 'scene:planet/#2-5-4', amplitude: [Number.NaN, 0] }],
    [{ mode_ref: 'scene:planet/#2-5-4', amplitude: [1e7, 0] }],
  ]) {
    await assert.rejects(session.strike(bad), /names 1\.\.4096|mode reference|finite metres/);
  }
  assert.equal(calls.length, 1, 'no exchange left the admitted owner');
  assert.equal(session.reading.acknowledged.generation, '1');
  assert.equal(session.reading.available, true);
  session.dispose();
});

test('a refused played strike leaves the owner admitted and unchanged', async () => {
  const { session, port, calls } = setup(); await session.pump();
  port.refuseOperation = 'strike';
  await assert.rejects(session.strike([{ mode_ref: 'scene:planet/none', amplitude: [0.1, 0] }]),
    /controlled refusal/);
  assert.equal(session.reading.acknowledged.generation, '1');
  assert.equal(session.reading.available, true, 'a clean refusal is not transport uncertainty');
  assert.equal(calls.length, 2);
  await session.pump();
  assert.equal(calls.length, 3);
  session.dispose();
});

const POINT = { theta: 1.25, phi: 4.5, amplitude: [0.4, -0.1] };

test('a strike at points is one performance act: the cursor moves, the samples and the clock do not', async () => {
  const { session, context, field, calls } = setup(); await session.pump();
  const end = session.reading.audio.target_context_seconds;
  const elapsed = session.reading.acknowledged.samples_elapsed;
  await session.strikeAt([POINT, { theta: 0, phi: 0, amplitude: [0.1, 0] }]);
  assert.deepEqual(calls.at(-1).command, { operation: 'strike-at',
    points: [{ theta: 1.25, phi: 4.5, amplitude: [0.4, -0.1] }, { theta: 0, phi: 0, amplitude: [0.1, 0] }] });
  assert.equal(session.reading.acknowledged.generation, '2', 'a chord of points is one generation');
  assert.equal(session.reading.acknowledged.samples_elapsed, elapsed, 'a struck point elapses no samples');
  // A performance act, not a determinant: no influence rides the reply, and
  // the struck body's targets present at the end of the block like any strike.
  assert.equal(session.lastInfluence, null);
  context.currentTime = end; session.present();
  assert.equal(field.last.targets[0].position[2], 0.25);
  await session.pump();
  assert.equal(calls.at(-1).command.operation, 'advance');
  assert.equal(session.reading.acknowledged.samples_elapsed, String(BigInt(elapsed) + 512n));
  session.dispose();
});

test('a strike at points copies its points: the caller cannot reshape the act after the call', async () => {
  const { session, calls } = setup(); await session.pump();
  const points = [{ theta: 1, phi: 2, amplitude: [0.2, 0], extra: 'dropped' }];
  await session.strikeAt(points);
  points[0].theta = 5; points[0].amplitude[0] = 0.9;
  assert.deepEqual(calls.at(-1).command.points, [{ theta: 1, phi: 2, amplitude: [0.2, 0] }]);
  session.dispose();
});

test('a malformed strike at points is refused before any native exchange', async () => {
  const { session, calls } = setup(); await session.pump();
  const TAU = 2 * Math.PI;
  for (const [bad, message] of [
    [[], /names 1\.\.64 points/],
    ['point', /names 1\.\.64 points/],
    [Array.from({ length: 65 }, () => POINT), /names 1\.\.64 points/],
    [[null], /theta and phi/],
    [[{ theta: Number.NaN, phi: 1, amplitude: [0, 0] }], /theta and phi/],
    [[{ theta: 1, phi: Infinity, amplitude: [0, 0] }], /theta and phi/],
    [[{ theta: -0.1, phi: 1, amplitude: [0, 0] }], /theta and phi/],
    [[{ theta: 1, phi: TAU, amplitude: [0, 0] }], /theta and phi/],
    [[{ theta: 7, phi: 1, amplitude: [0, 0] }], /theta and phi/],
    [[{ theta: '1', phi: 1, amplitude: [0, 0] }], /theta and phi/],
    [[{ theta: 1, phi: 1, amplitude: [0] }], /two finite metres/],
    [[{ theta: 1, phi: 1, amplitude: [Number.NaN, 0] }], /two finite metres/],
    [[{ theta: 1, phi: 1, amplitude: [1.5, 0] }], /two finite metres/],
    [[POINT, { theta: 1, phi: 1, amplitude: [0, -1.5] }], /two finite metres/],
  ]) {
    await assert.rejects(session.strikeAt(bad), message);
  }
  await session.strikeAt([{ theta: 0, phi: TAU - 1e-9, amplitude: [1, -1] }]);
  assert.equal(calls.length, 2, 'only the one admitted point left the owner');
  assert.equal(session.reading.available, true);
  session.dispose();
});

test('a refused strike at points leaves the owner admitted, unchanged and out of the journal', async () => {
  const { session, port, calls } = setup(); await session.pump();
  port.refuseOperation = 'strike-at';
  await assert.rejects(session.strikeAt([POINT]), /controlled refusal/);
  assert.equal(session.reading.acknowledged.generation, '1');
  assert.equal(session.reading.available, true, 'a clean refusal is not transport uncertainty');
  assert.equal(calls.length, 2);
  assert.deepEqual(session.journal().acts, []);
  port.refuseOperation = null;
  await session.strikeAt([POINT]);
  assert.deepEqual(session.journal().acts.map(act => [act.operation, act.kind]), [['strike-at', 'act']]);
  assert.deepEqual(session.journal().acts[0].command, { operation: 'strike-at', points: [POINT] });
  assert.deepEqual(session.journal().acts[0].acknowledged, { generation: '2', samples_elapsed: '512' });
  session.dispose();
});

test('a strike names the sample it lands at: the owner takes the exact cursor, one generation, no samples', async () => {
  // O-I #809: a player schedules ahead on the audio clock. The act is admitted now and the owner lands it in
  // its render at that sample; absent, the strike lands at the owner's cursor, as before.
  const { session, calls } = setup(); await session.pump();
  assert.equal(session.reading.acknowledged.samples_elapsed, '512');
  await session.strike([{ mode_ref: 'scene:planet/#2-5-4', amplitude: [0.5, 0] }], { at_samples_elapsed: '24512' });
  assert.deepEqual(calls.at(-1).command, { operation: 'strike',
    strikes: [{ mode_ref: 'scene:planet/#2-5-4', amplitude: [0.5, 0] }], at_samples_elapsed: '24512' });
  await session.strikeAt([POINT], { at_samples_elapsed: '512' });
  assert.deepEqual(calls.at(-1).command, { operation: 'strike-at', points: [POINT], at_samples_elapsed: '512' });
  assert.deepEqual(session.reading.acknowledged, { generation: '3', samples_elapsed: '512' });
  assert.deepEqual(session.journal().acts.map(act => act.command.at_samples_elapsed), ['24512', '512']);
  session.dispose();
});

test('a sample behind the acknowledged cursor, or not an exact cursor, is refused before any exchange', async () => {
  const { session, calls } = setup(); await session.pump();
  for (const at of ['511', '0', '0512', '', '-1', '18446744073709551616', 512, 512n, null]) {
    await assert.rejects(session.strike([{ mode_ref: 'scene:planet/#2-5-4', amplitude: [0.1, 0] }], { at_samples_elapsed: at }),
      /already rendered|exact sample cursor/, String(at));
    await assert.rejects(session.strikeAt([POINT], { at_samples_elapsed: at }), /already rendered|exact sample cursor/, String(at));
  }
  await assert.rejects(session.strikeAt([POINT], { at_samples_elapsed: '511' }), /behind the owner's cursor 512/);
  assert.equal(calls.length, 1, 'no exchange left the admitted owner');
  assert.equal(session.reading.available, true);
  session.dispose();
});

test('a strike at points that did not advance the generation is a cursor disagreement', async () => {
  const { session, port } = setup(); await session.pump();
  port.effect = reply => { reply.field.generation = '1'; return reply; };
  await assert.rejects(session.strikeAt([POINT]), /host operation and native cursor disagree/);
  assert.equal(session.reading.available, false, 'an unqualified acknowledgement is transport uncertainty');
});

test('a scene determinant event re-reads the basis through the same serial owner', async () => {
  const { session, context, field, calls } = setup(); await session.pump();
  const end = session.reading.audio.target_context_seconds;
  await session.operate({ operation: 'm1-advance', ticks: 1 });
  assert.equal(calls.at(-1).command.operation, 'm1-advance');
  assert.equal(session.reading.acknowledged.generation, '4');
  // The acknowledgement carried the new influence: no second exchange needed.
  assert.equal(session.lastInfluence.generation, '4');
  // Re-read targets wait behind already scheduled sound, then present.
  context.currentTime = end; session.present();
  assert.equal(field.last.targets[0].position[1], 0.5);
  const influence = await session.influence();
  assert.equal(influence.schema, 'ql.expression-influence/v1');
  assert.equal(session.reading.acknowledged.generation, '4', 'influence is a read');
});

test('each scene operation retains its own exact native commit range', async () => {
  for (const [operation, accepted, refused] of [
    ['m1-advance', [2n, 3n], [0n, 1n, 4n]],
    ['replace-event', [1n, 2n], [0n, 3n]],
  ]) {
    for (const step of accepted) {
      const { session, port } = setup(); port.eventStep = step;
      await session.operate({ operation, ticks: 1, event: {}, strike: true });
      assert.equal(session.reading.acknowledged.generation, String(1n + step));
      session.dispose();
    }
    for (const step of refused) {
      const { session, port } = setup(); port.eventStep = step;
      await assert.rejects(session.operate({ operation, ticks: 1, event: {}, strike: true }),
        /host operation and native cursor disagree/);
      assert.equal(port.closed, true); session.dispose();
    }
  }
});

test('the journal records every admitted owner act in request order and aggregates the advance data plane', async () => {
  const { session, port } = setup();
  await session.pump(); await session.pump();
  await session.strike([{ mode_ref: 'scene:planet/#2-5-4', amplitude: [0.9, 0] }]);
  await session.operate({ operation: 'set-axis', axis: 1, phase: { turns: '-2', half_degrees: 37 } });
  port.refuseOperation = 'set-damping';
  await assert.rejects(session.operate({ operation: 'set-damping', value: 1 }), /controlled refusal/);
  port.refuseOperation = null;
  session.hold('controlled-journal-hold');
  await session.recover('controlled-journal-recovery');
  const j = session.journal();
  assert.equal(j.schema, 'ql.instrument-performance-journal/v1');
  assert.equal(j.instance_ref, 'controlled:instance');
  assert.deepEqual(j.acts.map(act => [act.request_id, act.operation, act.kind]),
    [['3', 'strike', 'act'], ['4', 'set-axis', 'act'], ['6', 'read', 'recovery']]);
  assert.deepEqual(j.acts[0].command,
    { operation: 'strike', strikes: [{ mode_ref: 'scene:planet/#2-5-4', amplitude: [0.9, 0] }] });
  assert.deepEqual(j.acts[0].acknowledged, { generation: '2', samples_elapsed: '1024' });
  assert.deepEqual(j.acts[1].command, { operation: 'set-axis', axis: 1, phase: { turns: '-2', half_degrees: 37 } });
  assert.deepEqual(j.acts[1].acknowledged, { generation: '3', samples_elapsed: '1024' });
  assert.deepEqual(j.acts[2].command, { operation: 'read' });
  assert.deepEqual(j.acts[2].acknowledged, { generation: '3', samples_elapsed: '1024' });
  assert.deepEqual(j.advances, { blocks: 2, frames: 1024, first_samples_elapsed: '512', last_samples_elapsed: '1024' });
  session.dispose();
});

test('the performance journal is frozen, independent and survives disposal', async () => {
  const { session } = setup(); await session.pump();
  await session.strike([{ mode_ref: 'scene:planet/#2-5-4', amplitude: [0.5, 0] }]);
  const j = session.journal();
  assert.ok(Object.isFrozen(j));
  assert.throws(() => { j.instance_ref = 'foreign'; }, TypeError);
  j.acts[0].operation = 'foreign'; j.acts.push({ forged: true }); j.advances.blocks = 99;
  const fresh = session.journal();
  assert.equal(fresh.acts.length, 1);
  assert.equal(fresh.acts[0].operation, 'strike');
  assert.deepEqual(fresh.acts[0].acknowledged, { generation: '2', samples_elapsed: '512' });
  assert.equal(fresh.advances.blocks, 1);
  session.dispose();
  const after = session.journal();
  assert.deepEqual(after.acts, fresh.acts);
  assert.deepEqual(after.advances, fresh.advances);
  assert.equal(after.instance_ref, 'controlled:instance');
});

test('a refused played strike is absent from the performance journal', async () => {
  const { session, port } = setup(); await session.pump();
  port.refuseOperation = 'strike';
  await assert.rejects(session.strike([{ mode_ref: 'scene:planet/none', amplitude: [0.1, 0] }]), /controlled refusal/);
  assert.deepEqual(session.journal().acts, []);
  assert.deepEqual(session.journal().advances,
    { blocks: 1, frames: 512, first_samples_elapsed: '512', last_samples_elapsed: '512' });
  port.refuseOperation = null;
  await session.strike([{ mode_ref: 'scene:planet/#2-5-4', amplitude: [0.25, 0] }]);
  assert.equal(session.journal().acts.length, 1);
  assert.equal(session.journal().acts[0].operation, 'strike');
  session.dispose();
});

test('stage-state is a read: the stage body is disclosed and the field never moves', async () => {
  const { session, calls, port } = setup();
  const stage = { schema: 'ql.stage-state/v1', subject_ref: 'controlled:one', slots: { form: { owner: null } } };
  port.stageBody = stage;
  const disclosed = await session.stageState();
  assert.equal(disclosed, stage);
  assert.deepEqual(calls.at(-1).command, { operation: 'stage-state' });
  assert.equal(session.reading.available, true);
  assert.equal(session.reading.acknowledged.generation, '1', 'a disclosure read does not advance the owner');
  assert.ok(!session.journal().acts.some(act => act.operation === 'stage-state'),
    'a disclosure read is not an owner act');
  session.dispose();
});

test('a stage evaluation is one owner act: the plan advances the generation inside one envelope', async () => {
  const { session, calls } = setup();
  await session.pump();
  const before = session.reading.acknowledged.generation;
  await session.operate({ operation: 'stage-evaluate',
    procedure: { schema: 'ql.stage-procedure/v1', procedure_ref: 'ta-onta:studio:first-vertical', revision: 1,
      subject_ref: 'controlled:one', trigger: { trigger: 'invocation' }, selector: ['material.damping'],
      changes: [{ change: 'damping', per_second: 0.25 }] } });
  assert.equal(calls.at(-1).command.operation, 'stage-evaluate');
  assert.equal(BigInt(session.reading.acknowledged.generation) > BigInt(before), true);
  assert.equal(session.reading.acknowledged.samples_elapsed, '512', 'a plan never schedules data-plane samples');
  const acts = session.journal().acts;
  assert.equal(acts.at(-1).operation, 'stage-evaluate');
  assert.equal(acts.at(-1).command.procedure.procedure_ref, 'ta-onta:studio:first-vertical');
  session.dispose();
});

test('a stage evaluation whose host reply did not advance is a cursor disagreement', async () => {
  const { session, port } = setup();
  port.stageStep = 0n;
  await assert.rejects(session.operate({ operation: 'stage-evaluate', procedure: { schema: 'ql.stage-procedure/v1' } }),
    /native cursor disagree/);
  assert.equal(session.reading.available, false, 'an unexplained acknowledgement ends this lifetime');
});

test('stage retire, bind and unbind are owner acts that leave the field where it was, each keeping its receipt', async () => {
  const { session, calls } = setup();
  await session.pump();
  assert.equal(session.lastStageReceipt, null, 'no stage act yet: no receipt');
  const procedure = { schema: 'ql.stage-procedure/v1', procedure_ref: 'ta-onta:stage:live-fold', revision: 1, subject_ref: 'controlled:one',
    trigger: { trigger: 'determinant', operation: 'strike' }, selector: ['material.damping'], changes: [{ change: 'damping', per_second: 0.25 }] };
  for (const command of [{ operation: 'stage-bind', procedure, max_evaluations: 4 },
    { operation: 'stage-unbind', procedure_ref: 'ta-onta:stage:live-fold' },
    { operation: 'stage-retire', procedure_ref: 'ta-onta:stage:live-fold' }]) {
    const before = session.reading.acknowledged;
    await session.operate(command);
    assert.deepEqual(calls.at(-1).command, command, `${command.operation} rides the envelope as asked`);
    assert.deepEqual(session.reading.acknowledged, before, `${command.operation} advances neither generation nor samples`);
    assert.deepEqual(session.lastStageReceipt, { schema: 'ql.stage-receipt/v1', procedure_ref: 'ta-onta:stage:live-fold', applied: true });
    const act = session.journal().acts.at(-1);
    assert.equal(act.operation, command.operation);
    assert.equal(act.kind, 'act', 'a stage act is an owner act');
  }
  const receipt = session.lastStageReceipt; receipt.applied = false;
  assert.equal(session.lastStageReceipt.applied, true, 'the receipt is a copy');
  await assert.rejects(session.operate({ operation: 'stage-forge', procedure_ref: 'x' }), /idle admitted owner/, 'admitted by exact name');
  session.dispose();
});

test('a stage retire whose host reply advanced is a cursor disagreement', async () => {
  const { session, port } = setup();
  port.effect = reply => ({ ...reply, field: { ...reply.field, generation: String(BigInt(reply.field.generation) + 1n) } });
  await assert.rejects(session.operate({ operation: 'stage-retire', procedure_ref: 'x' }), /native cursor disagree/);
  assert.equal(session.reading.available, false, 'an unexplained acknowledgement ends this lifetime');
});

test('a refused stage act is no act and keeps the last receipt', async () => {
  const { session, port } = setup();
  await session.operate({ operation: 'stage-retire', procedure_ref: 'ta-onta:stage:live-fold' });
  const acts = session.journal().acts.length;
  port.refuseOperation = 'stage-unbind';
  await assert.rejects(session.operate({ operation: 'stage-unbind', procedure_ref: 'nothing-bound' }), /controlled refusal/);
  assert.equal(session.journal().acts.length, acts, 'the refusal is absent from the journal');
  assert.equal(session.lastStageReceipt.procedure_ref, 'ta-onta:stage:live-fold', 'the last receipt stands');
  assert.equal(session.reading.available, true);
  session.dispose();
});

test('a held owner takes no stage disclosure reads', async () => {
  const { session } = setup();
  session.hold('controlled hold');
  await assert.rejects(session.stageState(), /idle admitted owner/);
  session.dispose();
});

test('a Tarot score resolve is a read: the score, its currentness and its event come through, the field never moves', async () => {
  const { session, calls, port } = setup();
  await session.pump();
  const before = session.reading.acknowledged;
  const score = { schema: 'ql.tarot-score/v1', subject_ref: 'controlled:one', basis_revision: 'controlled:basis', tokens: [{ token_ref: 't' }] };
  port.scoreBody = score;
  const command = { operation: 'score-resolve', basis: { subject_ref: 'controlled:one', locus_ref: 'controlled:sky' } };
  const reading = await session.score(command);
  assert.deepEqual(calls.at(-1).command, command, 'the resolve rides the envelope as asked');
  assert.deepEqual(reading, { refused: false, score, current: true, resolved_event_ref: 'controlled:sky' });
  reading.score.tokens.length = 0;
  assert.equal(port.scoreBody.tokens.length, 1, 'the score is a copy');
  assert.deepEqual(session.reading.acknowledged, before, 'a score read advances neither generation nor samples');
  assert.ok(!session.journal().acts.some(act => act.operation === 'score-resolve'), 'a score read is not an owner act');
  assert.equal(session.reading.available, true);
  session.dispose();
});

test('a held Tarot score answers its own currentness: a score resolved against another event is not current', async () => {
  const { session, port } = setup();
  port.scoreCurrent = false; port.scoreEventRef = 'controlled:earlier-sky';
  const reading = await session.score({ operation: 'score-state' });
  assert.equal(reading.current, false, 'the host said not current, and the session says so');
  assert.equal(reading.resolved_event_ref, 'controlled:earlier-sky');
  session.dispose();
});

test('a refused Tarot score comes back as the owner\'s named reason and leaves the owner admitted', async () => {
  const { session, port } = setup();
  port.refuseOperation = 'score-resolve';
  const reading = await session.score({ operation: 'score-resolve', basis: {} });
  assert.deepEqual(reading, { refused: true, error: 'controlled refusal' });
  assert.equal(session.reading.available, true, 'a named refusal is not an unknown owner');
  assert.equal(session.reading.held, false);
  port.refuseOperation = null;
  assert.equal((await session.score({ operation: 'score-state' })).refused, false, 'the next read is admitted');
  session.dispose();
});

test('a Tarot score read that moved the field, or carries no score, is refused', async () => {
  const moved = setup();
  moved.port.effect = reply => ({ ...reply, field: { ...reply.field, generation: String(BigInt(reply.field.generation) + 1n) } });
  await assert.rejects(moved.session.score({ operation: 'score-state' }), /native cursor disagree/);
  assert.equal(moved.session.reading.available, false, 'an unexplained acknowledgement ends this lifetime');
  const empty = setup();
  empty.port.effect = reply => ({ ...reply, score: { schema: 'ql.other/v1' } });
  await assert.rejects(empty.session.score({ operation: 'score-state' }), /no ql.tarot-score\/v1 reading/);
  empty.session.dispose();
});

test('the Tarot score admits only its two reads, and only on an idle admitted owner', async () => {
  const { session, calls } = setup();
  await assert.rejects(session.score({ operation: 'score-forge' }), /score-resolve and score-state/);
  await assert.rejects(session.score({ operation: 'advance', frames: 512 }), /score-resolve and score-state/);
  assert.equal(calls.length, 0, 'refused before any exchange');
  session.hold('controlled hold');
  await assert.rejects(session.score({ operation: 'score-state' }), /idle admitted owner/);
  session.dispose();
});

test('the Torus session: a read, edits that move no native state, and a Fundamental that moves one generation', async () => {
  const { session, calls } = setup();
  const state = await session.torusState();
  assert.equal(state.schema, 'ql.m1-prime-session/v1');
  assert.deepEqual(calls.at(-1).command, { operation: 'torus-state' });
  assert.equal(session.reading.acknowledged.generation, '1', 'the session reading does not advance the owner');
  assert.ok(!session.journal().acts.some(act => act.operation === 'torus-state'), 'a read is not an owner act');
  await session.torusSet({ rate: { mode: 'free', steps_per_second: 440 }, tuning: 'logic' });
  assert.deepEqual(calls.at(-1).command, { operation: 'torus-set',
    edit: { rate: { mode: 'free', steps_per_second: 440 }, tuning: 'logic' } });
  assert.equal(session.reading.acknowledged.generation, '1', 'a session edit moves no native state');
  assert.equal(session.lastTorus.schema, 'ql.m1-prime-session/v1');
  assert.equal(session.lastInfluence.tuning, 'logic', 'an edit answers with the voices the owner reports');
  await session.torusSet({ fundamental: { source: 'hz', hz: 261.6 } });
  assert.equal(session.reading.acknowledged.generation, '2', 'a Fundamental edit is one modes continuation');
  await assert.rejects(session.torusSet({}), /at least one setting/);
  session.dispose();
});

test('a Torus setting for a later sample moves nothing until the block that reaches it lands it there', async () => {
  // O-I 855: torus-set names the sample it lands at, as a strike does, and the owner makes it at that sample inside the block that
  // reaches it; the player no longer waits for the cursor. A Fundamental made there is that block's one modes continuation, named
  // in the block's torus_set_landed with the session reading and the voices it leaves.
  const { session, calls, port, context } = setup({ maxBlocks: 1 });
  // Each block is heard before the next is asked for: the presentation queue holds one.
  const drain = () => { context.currentTime = session.reading.audio.target_context_seconds; session.present(); };
  await session.torusSet({ fundamental: { source: 'hz', hz: 261.6 } }, { at_samples_elapsed: '700' });
  assert.deepEqual(calls.at(-1).command, { operation: 'torus-set', edit: { fundamental: { source: 'hz', hz: 261.6 } },
    at_samples_elapsed: '700' });
  assert.equal(session.reading.acknowledged.generation, '1', 'nothing moves before its sample');
  assert.equal(session.reading.queued_blocks, 0, 'a setting that waits claims no presentation block');
  assert.deepEqual(session.journal().acts.map(act => [act.operation, act.command.at_samples_elapsed]), [['torus-set', '700']]);
  await session.pump();
  assert.deepEqual(session.reading.acknowledged, { generation: '1', samples_elapsed: '512' });
  // With the presentation queue full, a still setting is still admitted; one for the cursor's own sample is made now.
  await session.torusSet({ polyphony: 3 }, { at_samples_elapsed: '2048' });
  await session.torusSet({ tuning: 'logic' }, { at_samples_elapsed: '512' });
  assert.equal(session.reading.acknowledged.generation, '1');
  await assert.rejects(session.torusSet({ polyphony: 3 }, { at_samples_elapsed: '511' }), /setting at sample 511 is behind the owner's cursor 512/);
  await assert.rejects(session.torusSet({ polyphony: 3 }, { at_samples_elapsed: '0700' }), /exact sample cursor/);
  // The block that reaches 700 lands the Fundamental: one generation, the samples of one block, its reading carried.
  drain();
  await session.pump();
  assert.deepEqual(session.reading.acknowledged, { generation: '2', samples_elapsed: '1024' });
  assert.deepEqual(session.lastTorus.waiting, [{ at_samples_elapsed: '2048', edit: { polyphony: 3 } }]);
  assert.equal(session.lastInfluence.generation, '2', 'and the voices the owner reports after it');
  drain();
  await session.pump();
  drain();
  await session.pump();
  assert.deepEqual(session.reading.acknowledged, { generation: '2', samples_elapsed: '2048' },
    'a setting that moves no native state lands in its block and moves no generation');
  assert.deepEqual(session.lastTorus.waiting, []);
  // A block whose generation moved more than the settings it landed account for disagrees with the law.
  await session.torusSet({ fundamental: { source: 'hz', hz: 300 } }, { at_samples_elapsed: '2100' });
  port.landingStep = 2n;
  drain();
  await assert.rejects(session.pump(), /native cursor disagree/);
  session.dispose();
});

test('a played Torus act is one generation and keeps its receipt; a release moves nothing', async () => {
  const { session, calls, port } = setup();
  await session.torusPlay([{ row: 2, column: 1 }, { row: 3, column: 1 }], { amplitude: [0.1, 0] });
  assert.deepEqual(calls.at(-1).command, { operation: 'torus-play',
    keys: [{ row: 2, column: 1 }, { row: 3, column: 1 }], amplitude: [0.1, 0] });
  assert.equal(session.reading.acknowledged.generation, '2', 'a chord of keys is one strike at points');
  assert.equal(session.lastTorusPlay.notes.length, 2);
  await session.torusRelease([{ row: 2, column: 1 }]);
  assert.equal(session.reading.acknowledged.generation, '2', 'the body rings on: no native state moves');
  assert.deepEqual(session.journal().acts.map(act => act.operation), ['torus-play', 'torus-release']);
  // Refused before any exchange: a key off the six rows, an impulse past the policy.
  const sent = calls.length;
  await assert.rejects(session.torusPlay([{ row: 6, column: 0 }]), /row \(0\.\.5\)/);
  await assert.rejects(session.torusPlay([{ row: 0, column: 0 }], { amplitude: [1.5, 0] }), /within 1\.0/);
  assert.equal(calls.length, sent);
  // A host whose play did not move the cursor disagrees with the law.
  port.torusStep = 0n;
  await assert.rejects(session.torusPlay([{ row: 0, column: 0 }]), /disagree/);
  session.dispose();
});

test('a singlet is one strike at its codon\'s place and keeps its receipt; a refused one moves nothing', async () => {
  // O-I #771: an applied M3 event that leaves the cell non-dual strikes the codon at its place on the ring.
  const { session, calls, port } = setup();
  await session.torusSinglet(8, { amplitude: [0.1, 0], at_samples_elapsed: '4800' });
  assert.deepEqual(calls.at(-1).command, { operation: 'torus-singlet', codon: 8, amplitude: [0.1, 0], at_samples_elapsed: '4800' });
  assert.equal(session.reading.acknowledged.generation, '2', 'both arcs of a nonet-collision codon are one strike at points');
  assert.deepEqual(session.lastTorusSinglet.places, [{ lens: 0, arc: 9 }, { lens: 0, arc: 10 }]);
  assert.deepEqual(session.journal().acts.map(act => act.operation), ['torus-singlet']);
  // Refused before any exchange: no codon past the 64, an impulse past the policy.
  const sent = calls.length;
  await assert.rejects(session.torusSinglet(64), /codon is 0 to 63/);
  await assert.rejects(session.torusSinglet(2.5), /codon is 0 to 63/);
  await assert.rejects(session.torusSinglet(5, { amplitude: [1.5, 0] }), /within 1\.0/);
  assert.equal(calls.length, sent);
  // A host whose singlet did not move the cursor disagrees with the law.
  port.torusStep = 0n;
  await assert.rejects(session.torusSinglet(5), /disagree/);
  session.dispose();
});

test('an act that moves no native state presents nothing: a release enters the owner\'s count without claiming a presentation block', async () => {
  // O-I #790: a dense clip at a fast tempo releases as often as it strikes. A release moves no native state (the body rings on),
  // so it has no frame to present; if it claimed a block, a full presentation queue would refuse it and leave its note counted.
  const { session, calls, context, field } = setup({ maxBlocks: 1 });
  await session.pump();
  assert.equal(session.reading.queued_blocks, 1, 'the presentation queue is full');
  await assert.rejects(session.torusPlay([{ row: 0, column: 0 }]), /wait for bounded presentation capacity/,
    'a played act moves the body, so it still waits for a block to present it');
  for (let i = 0; i < 40; i++) await session.torusRelease([{ row: i % 6, column: i }]);
  assert.equal(calls.filter(call => call.command.operation === 'torus-release').length, 40, 'every release reaches the owner');
  assert.equal(session.reading.queued_blocks, 1, 'and none claims a presentation block');
  assert.equal(session.journal().acts.filter(act => act.operation === 'torus-release').length, 40, 'each is an owner act in the journal');
  // A session edit that moves no native state is the same; a Fundamental edit moves the owner and waits like any act that does.
  await session.torusSet({ tuning: 'logic' });
  assert.equal(session.reading.queued_blocks, 1);
  await assert.rejects(session.torusSet({ fundamental: { source: 'hz', hz: 261.6 } }), /wait for bounded presentation capacity/);
  // So are the stage's acts that move nothing.
  await session.operate({ operation: 'stage-unbind', procedure_ref: 'ta-onta:stage:live-fold' });
  assert.equal(session.reading.queued_blocks, 1);
  // Once the block is heard the queue drains and the played act is admitted.
  context.currentTime = session.reading.audio.target_context_seconds; session.present();
  await session.torusPlay([{ row: 0, column: 0 }]);
  assert.equal(field.last.generation, session.reading.acknowledged.generation, 'the played act presents its struck body');
  assert.equal(session.reading.acknowledged.generation, '2', 'and is the one act of all these that moved the owner');
  // A still act whose acknowledgement moved the owner disagrees with the law, as before.
  const moved = setup();
  moved.port.effect = reply => ({ ...reply, field: { ...reply.field, generation: String(BigInt(reply.field.generation) + 1n) } });
  await assert.rejects(moved.session.torusRelease([{ row: 0, column: 0 }]), /native cursor disagree/);
  session.dispose();
});

test('a Torus checkpoint is a read of the body as it rings, and a restore is one owner act that rings it again from there', async () => {
  // O-I #855: a Live take is rendered (and replayed) from the body as it stood when it began. The checkpoint is the owner's
  // ql.torus-checkpoint/v1 (course, form, forces, comma and seeds); restoring it is an act at the owner's own law.
  const { session, calls, port, field } = setup();
  const kept = await session.torusCheckpoint();
  assert.deepEqual(calls.at(-1).command, { operation: 'torus-checkpoint' });
  assert.equal(kept.schema, 'ql.torus-checkpoint/v1');
  assert.equal(session.reading.acknowledged.generation, '1', 'a checkpoint does not advance the owner');
  assert.ok(!session.journal().acts.some(act => act.operation === 'torus-checkpoint'), 'a read is not an owner act');
  kept.amplitudes[0][0] = 9; // the caller's copy is its own
  assert.equal((await session.torusCheckpoint()).amplitudes[0][0], 0.25);
  const restoring = { ...kept, amplitudes: [[0.25, -0.5]] };
  await session.torusRestore(restoring);
  assert.deepEqual(calls.at(-1).command, { operation: 'torus-restore', checkpoint: restoring }, 'the checkpoint travels whole');
  assert.equal(session.reading.acknowledged.generation, '3', 'the damping and the body: two generations');
  assert.equal(session.lastTorus.schema, 'ql.m1-prime-session/v1');
  assert.equal(session.lastInfluence.tuning, 'logic');
  assert.deepEqual(session.journal().acts.map(act => act.operation), ['torus-restore'], 'the restore is an owner act');
  session.present();
  assert.equal(session.reading.queued_blocks, 1, 'the restored body is presented');
  // The owner's restore moves one to six generations (back to the torus from a form, the event's modes and its shape, the
  // damping, the form kept, the body's state); anything else disagrees.
  for (const step of [0n, 7n]) {
    const other = setup(); other.port.restoreStep = step;
    await assert.rejects(other.session.torusRestore(restoring), /native cursor disagree/);
  }
  const six = setup(); six.port.restoreStep = 6n;
  await six.session.torusRestore(restoring);
  assert.equal(six.session.reading.acknowledged.generation, '7');
  // Refused before any exchange: anything that is not a Torus checkpoint.
  const sent = calls.length;
  for (const bad of [null, [], 'checkpoint', { schema: 'ql.m1-prime-session/v1' }])
    await assert.rejects(session.torusRestore(bad), /a Torus restore takes the owner's ql\.torus-checkpoint\/v1/);
  assert.equal(calls.length, sent);
  // A checkpoint reply without its checkpoint is not the owner's answer.
  port.checkpointBody = null; port.effect = reply => { delete reply.torus_checkpoint; return reply; };
  await assert.rejects(session.torusCheckpoint(), /no Torus checkpoint/);
  // A refused restore is no act.
  const refused = setup(); refused.port.refuseOperation = 'torus-restore';
  await assert.rejects(refused.session.torusRestore(restoring), /controlled refusal/);
  assert.equal(refused.session.journal().acts.length, 0);
  assert.ok(field);
  session.dispose();
});

test('choosing the body is one owner act: a form made a body, or the torus by name', async () => {
  // O-I #852 (DLW manual §21.2 Body): the owner prepares the form's ql.form-body-request/v1 and rings the scene's voices through
  // it from the cursor; null is the torus. One body act, journaled; the answer carries the session reading with the body.
  const { session, calls } = setup();
  const fold = { schema: 'ql.form-body-request/v1', event_ref: 'event:a', subject_ref: 'person:a', modes: 8 };
  await session.torusBody(fold);
  assert.deepEqual(calls.at(-1).command, { operation: 'torus-body', body: fold }, 'the request travels whole');
  assert.equal(session.reading.acknowledged.generation, '2', 'one body act');
  assert.equal(session.lastTorus.body.kind, 'form');
  assert.equal(session.lastInfluence.tuning, 'logic');
  session.present();
  assert.equal(session.reading.queued_blocks, 1, 'the chosen body is presented');
  fold.modes = 64; // the caller's copy is its own
  assert.equal(calls.at(-1).command.body.modes, 8);
  await session.torusBody(null);
  assert.deepEqual(calls.at(-1).command, { operation: 'torus-body', body: null });
  assert.equal(session.lastTorus.body.kind, 'torus');
  assert.deepEqual(session.journal().acts.map(act => act.operation), ['torus-body', 'torus-body'], 'each choice is an owner act');
  // Refused before any exchange: anything that is not a form's body request, or the torus.
  const sent = calls.length;
  for (const bad of [undefined, [], 'fold', { schema: 'ql.torus-checkpoint/v1' }])
    await assert.rejects(session.torusBody(bad), /a body is a form's ql\.form-body-request\/v1, or null for the torus/);
  assert.equal(calls.length, sent);
  // A choice that moved the owner by other than one body act disagrees with the law.
  for (const step of [0n, 2n]) {
    const other = setup(); other.port.bodyStep = step;
    await assert.rejects(other.session.torusBody(fold), /native cursor disagree/);
  }
  // A refused choice is no act.
  const refused = setup(); refused.port.refuseOperation = 'torus-body';
  await assert.rejects(refused.session.torusBody(fold), /controlled refusal/);
  assert.equal(refused.session.journal().acts.length, 0);
  session.dispose();
});

test('a take renders offline as a read: the owner neither moves nor sounds, and the render comes back whole', async () => {
  // O-I #855: Export Audio takes a Live take through the M1′ session offline, on a second owner of the scene (host.rs torus-render).
  const { session, calls, port } = setup();
  const checkpoint = await session.torusCheckpoint();
  const acts = [
    { action: 'strike', at_samples_elapsed: '4800', keys: [{ row: 2, column: 3 }], amplitude: [0.3, 0] },
    { action: 'release', at_samples_elapsed: '9600', keys: [{ row: 2, column: 3 }] },
    { action: 'set', at_samples_elapsed: '9600', edit: { polyphony: 2 } },
    // A touch and a singlet are a take's acts too (O-I #855 kept whole): each the owner's own act, at its sample.
    { action: 'strike-at', at_samples_elapsed: '9700', points: [{ theta: 0.75, phi: 2.5, amplitude: [0.3, 0] }, { theta: 4, phi: 5.5, amplitude: [0.3, 0] }] },
    { action: 'singlet', at_samples_elapsed: '9800', codon: 8, amplitude: [0.25, 0] }];
  const render = await session.torusRender({ checkpoint, acts, frames: 4800 });
  assert.deepEqual(calls.at(-1).command, { operation: 'torus-render', checkpoint, acts, frames: 4800 }, 'the take travels whole');
  assert.equal(render.schema, 'ql.torus-render/v1');
  assert.equal(render.audio.length, 4800);
  assert.equal(session.reading.acknowledged.generation, '1', 'a render does not advance the owner');
  assert.ok(!session.journal().acts.some(act => act.operation === 'torus-render'), 'a read is not an owner act');
  const next = await session.torusRender({ continues: render.continues, acts: [], frames: 2400 });
  assert.deepEqual(calls.at(-1).command, { operation: 'torus-render', continues: 'render:1', acts: [], frames: 2400 });
  assert.equal(next.audio.length, 2400);
  // Refused before any exchange.
  const sent = calls.length;
  for (const [bad, words] of [
    [{ acts: [], frames: 4800 }, /either a checkpoint or the render it continues/],
    [{ checkpoint, continues: 'render:1', acts: [], frames: 4800 }, /either a checkpoint or the render it continues/],
    [{ checkpoint: { schema: 'ql.m1-prime-session/v1' }, acts: [], frames: 4800 }, /ql\.torus-checkpoint\/v1/],
    [{ continues: '', acts: [], frames: 4800 }, /continues names the render/],
    [{ checkpoint, acts: [], frames: 0 }, /1 to 1048576 frames/],
    [{ checkpoint, acts: [], frames: 2 ** 20 + 1 }, /1 to 1048576 frames/],
    [{ checkpoint, acts: Array(4097).fill(acts[1]), frames: 4800 }, /at most 4096 acts/],
    [{ checkpoint, acts: [{ ...acts[0], at_samples_elapsed: 4800 }], frames: 4800 }, /exact sample cursor/],
    [{ checkpoint, acts: [{ ...acts[0], keys: [{ row: 6, column: 0 }] }], frames: 4800 }, /row \(0\.\.5\)/],
    [{ checkpoint, acts: [{ ...acts[0], amplitude: [1.5, 0] }], frames: 4800 }, /within 1\.0/],
    [{ checkpoint, acts: [{ action: 'hold', at_samples_elapsed: '0', keys: [] }], frames: 4800 }, /strike, touch, singlet, release or set/],
    [{ checkpoint, acts: [{ ...acts[3], points: [] }], frames: 4800 }, /1\.\.64 points/],
    [{ checkpoint, acts: [{ ...acts[3], points: [{ theta: 7, phi: 1, amplitude: [0.3, 0] }] }], frames: 4800 }, /\[0, 2\*pi\)/],
    [{ checkpoint, acts: [{ ...acts[3], points: [{ theta: 1, phi: 1, amplitude: [1.5, 0] }] }], frames: 4800 }, /within 1\.0/],
    [{ checkpoint, acts: [{ ...acts[4], codon: 64 }], frames: 4800 }, /codon is 0 to 63/],
    [{ checkpoint, acts: [{ ...acts[4], codon: 2.5 }], frames: 4800 }, /codon is 0 to 63/],
    [{ checkpoint, acts: [{ ...acts[4], amplitude: [1.5, 0] }], frames: 4800 }, /within 1\.0/],
    [{ checkpoint, acts: [{ action: 'set', at_samples_elapsed: '0', edit: {} }], frames: 4800 }, /at least one setting/],
  ]) await assert.rejects(session.torusRender(bad), words, JSON.stringify(bad).slice(0, 80));
  assert.equal(calls.length, sent);
  // A render whose answer is not the take's sound, or that moved the owner, is not the owner's answer.
  port.renderBody = { schema: 'ql.torus-render/v1', audio: [0.25] };
  await assert.rejects(session.torusRender({ checkpoint, acts: [], frames: 4800 }), /no render of the take/);
  const moved = setup(); const kept = await moved.session.torusCheckpoint();
  moved.port.effect = reply => ({ ...reply, field: { ...reply.field, generation: String(BigInt(reply.field.generation) + 1n) } });
  await assert.rejects(moved.session.torusRender({ checkpoint: kept, acts: [], frames: 4800 }), /native cursor disagree|read advanced/);
  session.dispose();
});

test('where the Torus is heard from is a receiving setting: the body stands, the listener is never journaled, and the place goes nowhere else', async () => {
  // O-I 856 (DLW §21.2 View: "Receiver On makes your camera the place where the torus is heard"; QL-MEF #289 packet N): a place in
  // the metres the targets are drawn in is the listener's own context, null the world's. The owner's answer says how the body is
  // heard, and the body, its clock and its generation stand.
  const { session, calls } = setup();
  const heard = await session.torusReceiver([4.0625, 0, 0.375]);
  assert.deepEqual(calls.at(-1).command, { operation: 'torus-receiver', receiver: { at_metres: [4.0625, 0, 0.375] } });
  assert.equal(heard.schema, 'ql.torus-receiver/v1');
  assert.equal(heard.context, 'personal');
  assert.equal(session.reading.acknowledged.generation, '1', 'the body does not move');
  assert.equal(session.reading.queued_blocks, 0, 'there is nothing to present');
  assert.equal(session.lastTorus.schema, 'ql.m1-prime-session/v1');
  heard.gains[0].gain = 9; // the caller's copy is its own
  const world = await session.torusReceiver(null);
  assert.deepEqual(calls.at(-1).command, { operation: 'torus-receiver', receiver: null });
  assert.equal(world.context, 'world');
  assert.equal(world.gains[0].gain, 1);
  const journal = JSON.stringify(session.journal());
  assert.ok(!session.journal().acts.some(act => act.operation === 'torus-receiver'), 'the listener is not the performance');
  assert.ok(!journal.includes('4.0625') && !journal.includes('at_metres'), journal);
  // Refused before any exchange: anything that is not three finite metres within the owner's bound, or null.
  const sent = calls.length;
  for (const bad of [undefined, [1, 2], [1, 2, Number.NaN], [1, 2, 2e6], 'here', { at_metres: [1, 2, 3] }])
    await assert.rejects(session.torusReceiver(bad), /a receiver stands at three finite metres/);
  assert.equal(calls.length, sent);
  // An answer without the receiver's reading is not the owner's.
  const bare = setup(); bare.port.effect = reply => { delete reply.torus_receiver; return reply; };
  await assert.rejects(bare.session.torusReceiver(null), /no receiver reading/);
  // A refusal comes back in the owner's words; a held owner takes no receiver.
  const refused = setup(); refused.port.refuseOperation = 'torus-receiver';
  await assert.rejects(refused.session.torusReceiver([1, 2, 3]), /controlled refusal/);
  const held = setup(); held.session.hold('test');
  await assert.rejects(held.session.torusReceiver(null), /idle admitted owner/);
  // An answer whose body moved disagrees with the owner's law.
  const moved = setup();
  moved.port.effect = reply => ({ ...reply, field: { ...reply.field, generation: String(BigInt(reply.field.generation) + 1n) } });
  await assert.rejects(moved.session.torusReceiver([1, 2, 3]), /native cursor disagree/);
  session.dispose();
});


test('a shared receiving context and a return to a context are the owner\'s own receiving acts, never journaled', async () => {
  // QL-MEF #289 packet N (O-I 856): a place shared under a reference is an explicitly shared context, disclosed with the
  // reference; switching context is an explicit act, and returning to one names the context alone, the owner resuming it
  // where it last stood. Neither the reference nor any place goes into the journal.
  const { session, calls } = setup();
  const shared = await session.torusReceiver([1.5, 0, -2], { shared: 'shared-field:circle-7' });
  assert.deepEqual(calls.at(-1).command, { operation: 'torus-receiver', receiver: { at_metres: [1.5, 0, -2], shared: 'shared-field:circle-7' } });
  assert.equal(shared.context, 'shared');
  assert.equal(shared.shared, 'shared-field:circle-7');
  assert.deepEqual(shared.at_metres, [1.5, 0, -2]);
  const own = await session.torusReceiver([4, 0, 0]);
  assert.deepEqual(calls.at(-1).command, { operation: 'torus-receiver', receiver: { at_metres: [4, 0, 0] } }, 'one\'s own place names no reference');
  assert.equal(own.context, 'personal');
  for (const context of ['shared', 'personal', 'world']) {
    const back = await session.torusReceiverReturn(context);
    assert.deepEqual(calls.at(-1).command, { operation: 'torus-receiver', context }, 'a return sends no place');
    assert.equal(back.context, context);
  }
  assert.equal(session.reading.acknowledged.generation, '1', 'the body does not move');
  const journal = JSON.stringify(session.journal());
  assert.ok(!journal.includes('circle-7') && !journal.includes('torus-receiver'), journal);
  // Refused before any exchange: a shared context that stands nowhere or is named by no reference, and a context that is not one.
  const sent = calls.length;
  for (const [place, reference] of [[null, 'shared-field:circle-7'], [[1, 2, 3], 7], [[1, 2, 3], null]])
    await assert.rejects(session.torusReceiver(place, { shared: reference }), /a shared receiving context stands at a place/);
  for (const bad of [undefined, null, 'elsewhere', 'World', { context: 'world' }])
    await assert.rejects(session.torusReceiverReturn(bad), /returns to the world's, the personal or the shared context/);
  assert.equal(calls.length, sent);
  // The owner's refusal of a context never entered comes back in its words; a held owner takes no return.
  const refused = setup(); refused.port.refuseOperation = 'torus-receiver';
  await assert.rejects(refused.session.torusReceiverReturn('personal'), /controlled refusal/);
  const held = setup(); held.session.hold('test');
  await assert.rejects(held.session.torusReceiverReturn('world'), /idle admitted owner/);
  session.dispose();
});
test('the Torus is handed over to a native host as a read: the body stands, nothing is journaled, and the answer is the owner\'s', async () => {
  // O-I #849: a native audio host continues the field exactly as it stands (ql.torus-native/v1). Through O:I's kernel the answer is
  // the digest it relays in the document's place (the kernel keeps the document for the host), so either is the owner's answer.
  const { session, calls } = setup();
  const handed = await session.torusNative();
  assert.deepEqual(calls.at(-1).command, { operation: 'torus-native' });
  assert.equal(handed.schema, 'ql.torus-native/v1');
  assert.equal(session.reading.acknowledged.generation, '1', 'the body does not move');
  assert.equal(session.reading.queued_blocks, 0, 'there is nothing to present');
  assert.ok(!session.journal().acts.some(act => act.operation === 'torus-native'), 'a handover is not the performance');
  handed.keys.contract = 'changed'; // the caller's copy is its own
  const relayed = setup();
  relayed.port.nativeBody = { schema: 'ql.torus-native/v1', sha256: 'sha256:' + '0'.repeat(64), generation: '1', at_samples_elapsed: '0' };
  assert.equal((await relayed.session.torusNative()).sha256, 'sha256:' + '0'.repeat(64));
  // An answer without the handover is not the owner's; a refusal comes back in its words; a held owner hands nothing over.
  const bare = setup(); bare.port.effect = reply => { delete reply.torus_native; return reply; };
  await assert.rejects(bare.session.torusNative(), /no Torus handed over/);
  const refused = setup(); refused.port.refuseOperation = 'torus-native';
  await assert.rejects(refused.session.torusNative(), /controlled refusal/);
  const held = setup(); held.session.hold('test');
  await assert.rejects(held.session.torusNative(), /idle admitted owner/);
  // An answer whose body moved disagrees with the owner's law.
  const moved = setup();
  moved.port.effect = reply => ({ ...reply, field: { ...reply.field, generation: String(BigInt(reply.field.generation) + 1n) } });
  await assert.rejects(moved.session.torusNative(), /native cursor disagree/);
  session.dispose();
});
