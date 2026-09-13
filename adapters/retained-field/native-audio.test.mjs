import { test } from 'node:test';
import assert from 'node:assert/strict';
import { NativeAudioBinding } from './native-audio.mjs';

// This deterministic control-plane double tests refusal/atomicity, not acoustic
// output. test-audio-browser.py separately executes the real Web Audio graph.
class Context {
  sampleRate = 48000; currentTime = 0; state = 'suspended'; destination = {};
  nodes = []; listeners = new Map(); failure = null;
  createGain() { return { gain: { value: 0, setValueAtTime(v) { this.value = v; } }, connect() {}, disconnect() {} }; }
  createBuffer(_channels, length, rate) {
    if (this.failure === 'buffer') throw Error('device allocation refused');
    assert.equal(rate, this.sampleRate);
    return { data: new Float32Array(length), copyToChannel(pcm) { this.data.set(pcm); } };
  }
  createBufferSource() {
    const context = this;
    const node = { buffer: null, playbackRate: { value: 1 }, detune: { value: 0 }, starts: 0, stops: 0, disconnected: false,
      connect() {}, disconnect() { this.disconnected = true; },
      start(t) { if (context.failure === 'start') throw Error('device start refused'); this.time = t; this.starts++; },
      stop() { this.stops++; } };
    this.nodes.push(node); return node;
  }
  addEventListener(k, v) { this.listeners.set(k, v); }
  removeEventListener(k) { this.listeners.delete(k); }
  change(state) { this.state = state; this.listeners.get('statechange')?.(); }
}
function frame(end = 0n, values = [], generation = 1n, edits = {}) {
  return { schema: 'ql.continuous-field/v1', sample_rate: 48000, event_ref: 'controlled:sky', subject_ref: 'controlled:one',
    registry_revision: 'controlled:registry', geometry_ref: 'controlled:geometry', material_ref: 'controlled:material',
    model_ref: 'controlled:model', generation: String(generation), samples_elapsed: String(end), audio: values,
    m2_identity: { event_ref: 'controlled:sky', profile_generation: 1 },
    clock: { field_ref: '#3-0', centre_ref: '#3-5-5/0', generation: '4' }, standing: 'controlled-test-input', ...edits };
}
function setup(options = {}, initial = frame()) {
  const context = new Context(); context.sampleRate = initial.sample_rate;
  const owner = {};
  const binding = new NativeAudioBinding(context, { owner, initialFrame: initial, gain: 0.5, muted: false,
    leadSeconds: 0.02, ...options });
  return { context, owner, binding };
}
function unchanged(binding, f) {
  const before = binding.lastReceipt;
  assert.throws(f);
  assert.deepEqual(binding.lastReceipt, before, 'rejection changed the admitted native receipt');
}

test('native PCM is copied without another oscillator, resampling, mutation or repeat', () => {
  const { context, binding } = setup(); const f = frame(3n, [0.25, -0.5, 0.125]);
  const receipt = binding.apply(f);
  assert.equal(context.nodes.length, 1); assert.deepEqual([...context.nodes[0].buffer.data], f.audio);
  assert.equal(receipt.interval.native_start, '0'); assert.equal(receipt.interval.native_end, '3');
  assert.equal(receipt.interval.target_context_seconds, receipt.interval.end_context_seconds);
  assert.equal(binding.apply(f).status, 'duplicate-not-scheduled'); assert.equal(context.nodes.length, 1);
  f.audio[0] = 0.75; assert.equal(context.nodes[0].buffer.data[0], 0.25);
  unchanged(binding, () => binding.apply(f)); binding.dispose();
});

test('read and source generations do not replay prior PCM; changed generation continues at the same sample cursor', () => {
  const { context, binding } = setup();
  binding.apply(frame(4n, [0.2, 0.1, 0, -0.1]));
  binding.apply(frame(4n)); assert.equal(context.nodes.length, 1);
  binding.apply(frame(4n, [], 2n));
  const next = binding.apply(frame(6n, [0.2, 0.3], 2n));
  assert.equal(next.native.generation, '2'); assert.equal(next.interval.native_start, '4');
  assert.equal(context.nodes[1].time, context.nodes[0].time + 4 / 48000);
  binding.dispose();
});

test('field/subject/source and body clocks remain separate identities', () => {
  const { binding } = setup();
  for (const key of ['event_ref', 'subject_ref', 'registry_revision', 'geometry_ref', 'material_ref', 'model_ref'])
    unchanged(binding, () => binding.apply(frame(1n, [0.1], 1n, { [key]: 'wrong' })));
  unchanged(binding, () => binding.apply(frame(1n, [0.1], 1n, { m2_identity: { event_ref: 'wrong' } })));
  unchanged(binding, () => binding.apply(frame(1n, [0.1], 1n, { clock: { field_ref: '#3-0', centre_ref: '#3-5-360' } })));
  binding.dispose();
});

test('missing, overlapping, stale and conflicting source intervals fail before device mutation', () => {
  const { context, binding } = setup(); binding.apply(frame(2n, [0.1, 0.2]));
  for (const f of [frame(5n, [0.1]), frame(3n, [0.1, 0.2]), frame(1n), frame(2n, [], 0n), frame(3n)])
    unchanged(binding, () => binding.apply(f));
  unchanged(binding, () => binding.apply(frame(2n, [], 1n, { clock: { field_ref: '#3-0', centre_ref: '#3-5-5/0', generation: '99' } })));
  assert.equal(context.nodes.length, 1); binding.dispose();
});

test('PCM shape, nonfinite values, clipping and rate mismatch are refused atomically', () => {
  const { binding } = setup();
  for (const values of [[NaN], [Infinity], [-Infinity], [3.5e38], [3], ['0.1'], [null], new Array(8193).fill(0)])
    unchanged(binding, () => binding.apply(frame(BigInt(values.length), values)));
  unchanged(binding, () => binding.apply(frame(1n, [0.1], 1n, { sample_rate: 44100 })));
  binding.dispose();
});

test('u64 source counters beyond JS safe integer retain exact interval relation', () => {
  const origin = (1n << 63n) + 12345n;
  const { binding } = setup({}, frame(origin));
  const receipt = binding.apply(frame(origin + 3n, [0.1, 0.2, 0.3]));
  assert.equal(receipt.interval.native_start, String(origin)); assert.equal(receipt.interval.native_end, String(origin + 3n));
  for (const value of ['01', '1e3', '-1', '18446744073709551616', 12, ' 3'])
    unchanged(binding, () => binding.apply(frame(origin + 4n, [0.1], 1n, { samples_elapsed: value })));
  binding.dispose();
});

test('bounded queue rejects excess, then frees capacity without retaining historic buffers', () => {
  const { context, binding } = setup({ maxQueuedFrames: 128, maxBlocks: 1 });
  const samples = Array(128).fill(0.1); binding.apply(frame(128n, samples));
  assert.deepEqual(binding.capacity, { blocks: 0, frames: 0, held: false, closed: false });
  unchanged(binding, () => binding.apply(frame(129n, [0.1])));
  context.currentTime = 0.02 + 128 / 48000;
  assert.equal(binding.capacity.blocks, 1);
  binding.apply(frame(256n, samples)); assert.equal(context.nodes.length, 2); binding.dispose();
});

test('mute suppresses presentation while admitted native time and queued nodes continue', () => {
  const { context, binding } = setup(); binding.apply(frame(2n, [0.1, 0.2]));
  binding.setMuted(true); assert.equal(binding.lastReceipt.muted, true);
  binding.apply(frame(4n, [0.3, 0.4])); binding.setMuted(false);
  assert.equal(binding.lastReceipt.native.samples_elapsed, '4'); assert.equal(context.nodes.length, 2);
  assert.ok(context.nodes.every(node => node.starts === 1 && node.stops === 0)); binding.dispose();
});

test('provider loss discards queued presentation; explicit read rebase restores without native reset or lost-audio fiction', () => {
  const { context, binding } = setup(); binding.apply(frame(2n, [0.1, 0.2]));
  binding.hold('provider-lost'); assert.equal(binding.lastReceipt.discarded_blocks, 1);
  unchanged(binding, () => binding.apply(frame(4n, [0.3, 0.4])));
  unchanged(binding, () => binding.rebase(frame(1n), 'backwards'));
  binding.rebase(frame(20n, [], 2n), 'provider-restored-explicit-current-read');
  const next = binding.apply(frame(22n, [0.3, 0.4], 2n));
  assert.equal(next.device_epoch, 1); assert.equal(next.native_origin, '20');
  assert.equal(next.interval.native_start, '20'); assert.equal(context.nodes[0].stops, 1); binding.dispose();
});

test('late input is not silently started now; explicit new epoch is required', () => {
  const { context, binding } = setup(); context.currentTime = 1;
  unchanged(binding, () => binding.apply(frame(2n, [0.1, 0.2])));
  assert.equal(context.nodes.length, 0);
  binding.rebase(frame(2n), 'missed-deadline-current-read');
  assert.ok(binding.apply(frame(4n, [0.1, 0.2])).interval.start_context_seconds >= 1.02); binding.dispose();
});

test('device failures do not acknowledge sound or consume the receiver cursor', () => {
  for (const failure of ['buffer', 'start']) {
    const { context, binding } = setup(); context.failure = failure;
    unchanged(binding, () => binding.apply(frame(2n, [0.1, 0.2])));
    assert.equal(binding.capacity.blocks, 64);
    context.failure = null; binding.apply(frame(2n, [0.1, 0.2])); binding.dispose();
  }
});

test('one native owner has one audio output; read-only companions cannot duplicate it', () => {
  const { context, owner, binding } = setup();
  assert.throws(() => new NativeAudioBinding(context, { owner, initialFrame: frame() }));
  const view = binding.lastReceipt; view.native.subject_ref = 'mutated-companion';
  assert.equal(binding.lastReceipt.native.subject_ref, 'controlled:one');
  const other = setup({}, frame(0n, [], 1n, { subject_ref: 'controlled:two' }));
  binding.apply(frame(2n, [0.1, 0.2])); assert.equal(other.binding.lastReceipt.native.samples_elapsed, '0');
  other.binding.dispose(); binding.dispose();
  const recovered = new NativeAudioBinding(context, { owner, initialFrame: frame(2n) }); recovered.dispose();
});

test('audio context interruption requires explicit recovery; disposal releases all owned nodes', () => {
  const { context, binding } = setup(); context.change('running');
  binding.apply(frame(2n, [0.1, 0.2])); context.change('suspended');
  assert.equal(binding.capacity.held, true); assert.equal(binding.lastReceipt.reason, 'audio-context-interrupted');
  context.change('running'); unchanged(binding, () => binding.apply(frame(4n, [0.1, 0.2])));
  binding.rebase(frame(2n), 'context-resumed-current-read'); binding.apply(frame(4n, [0.1, 0.2]));
  context.change('closed'); assert.equal(binding.capacity.closed, true);
  assert.ok(context.nodes.every(node => node.disconnected)); binding.dispose();
});

test('metadata order is immaterial, unknown receipt values and deep payloads are refused', () => {
  const { binding } = setup();
  const reordered = frame(); reordered.clock = { generation: '4', centre_ref: '#3-5-5/0', field_ref: '#3-0' };
  binding.apply(reordered);
  for (const extra of [Infinity, 9007199254740992, (() => { const a = {}; a.a = a; return a; })()])
    unchanged(binding, () => binding.apply(frame(1n, [0.1], 1n, { m2_identity: { event_ref: 'controlled:sky', extra } })));
  binding.dispose();
});

test('default limits support all native sample-rate boundaries', () => {
  for (const sample_rate of [8000, 44100, 48000, 96000, 192000]) {
    const { binding } = setup({}, frame(0n, [], 1n, { sample_rate })); binding.dispose();
  }
});


test('read/control receipts retain the end-of-block presentation point without scheduling another node', () => {
  const { binding, context } = setup();
  const receipt = binding.apply(frame(128n, Array(128).fill(0.1)));
  const read = binding.apply(frame(128n, [], 2n));
  assert.equal(read.target_context_seconds, receipt.interval.target_context_seconds);
  assert.equal(read.observed_context_seconds, context.currentTime);
  assert.equal(context.nodes.length, 1); binding.dispose();
});

test('thousands of ordinary updates retain bounded live nodes, not the whole playback history', () => {
  const { context, binding } = setup({ maxBlocks: 2, maxQueuedFrames: 1024 });
  const block = Array(512).fill(0.125), origin = binding.lastReceipt.context_origin_seconds;
  for (let i = 0; i < 3000; i++) {
    context.currentTime = Math.max(0, origin + (i * 512 - 128) / 48000);
    binding.apply(frame(BigInt((i + 1) * 512), block));
    assert.ok(binding.capacity.blocks >= 0 && binding.capacity.frames >= 0);
  }
  assert.equal(binding.lastReceipt.native.samples_elapsed, String(3000 * 512));
  assert.ok(context.nodes.slice(0, -2).every(node => node.disconnected)); binding.dispose();
});


test('pure wire validation admits no source cursor, node or presentation state', () => {
  const { context, binding } = setup();
  const before = binding.lastReceipt, capacity = binding.capacity;
  binding.validate(frame(128n, Array(128).fill(0.1)));
  binding.validate(frame(256n, Array(128).fill(0.2)));
  assert.deepEqual(binding.lastReceipt, before); assert.deepEqual(binding.capacity, capacity);
  assert.equal(context.nodes.length, 0);
  unchanged(binding, () => binding.validate(frame(1n, [NaN])));
  binding.hold('explicit-recovery');
  assert.equal(binding.validate(frame(256n)), true);
  binding.dispose();
});
