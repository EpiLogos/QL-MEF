import { NativeAudioBinding } from './native-audio.mjs';
const check = (ok, message) => { if (!ok) throw new Error(message); };
const refused = (fn, message) => { let failed = false; try { fn(); } catch { failed = true; } check(failed, message); };
function compare(output, samples, offset, gain) {
  let maxError = 0, nonzero = 0;
  for (let i = 0; i < output.length; i++) {
    const expected = i >= offset && i < offset + samples.length ? Math.fround(Math.fround(samples[i - offset]) * gain) : 0;
    maxError = Math.max(maxError, Math.abs(output[i] - expected));
    if (Math.abs(output[i]) > 1e-7) nonzero++;
  }
  // Byte-equivalence is the normal result; a numerical tolerance is stated
  // rather than interpreting sub-sample/browser DSP differences as new canon.
  check(maxError <= 2e-7, `native PCM/device graph mismatch: ${maxError}`);
  return { compared_samples: output.length, max_abs_error: maxError, nonzero_samples: nonzero };
}
function contextFor(rate, length) {
  return new OfflineAudioContext({ numberOfChannels: 1, length, sampleRate: rate });
}
async function renderCase(testCase, mode) {
  const pcm = testCase.frames.flatMap(frame => frame.audio);
  const context = contextFor(testCase.rate, pcm.length + 2048);
  const owner = {}; const started = performance.now();
  const binding = new NativeAudioBinding(context, { owner, initialFrame: testCase.initial,
    gain: 0.25, muted: mode !== 'audible', leadSeconds: 128 / testCase.rate,
    maxQueuedFrames: Math.min(32768, testCase.rate * 2) });
  refused(() => new NativeAudioBinding(context, { owner, initialFrame: testCase.initial }), 'duplicate native output owner');
  const origin = Math.round(binding.lastReceipt.context_origin_seconds * testCase.rate);
  let last;
  for (const frame of testCase.frames) last = binding.apply(frame);
  const scheduled = binding.lastReceipt.scheduled_blocks;
  binding.apply(testCase.frames.at(-1));
  check(binding.lastReceipt.scheduled_blocks === scheduled, 'duplicate PCM scheduled twice');
  if (mode === 'resume') binding.setMuted(false);
  const scheduleMs = performance.now() - started;
  const nativeEnd = binding.lastReceipt.native.samples_elapsed;
  const readOnly = binding.lastReceipt; readOnly.native.subject_ref = 'must-not-escape';
  check(binding.lastReceipt.native.subject_ref === testCase.initial.subject_ref, 'companion mutated native selection');
  const renderStarted = performance.now();
  const rendered = await context.startRendering();
  const result = compare(rendered.getChannelData(0), pcm, origin, mode === 'muted' ? 0 : 0.25);
  check(nativeEnd === testCase.frames.at(-1).samples_elapsed, 'mute altered native time');
  check(mode === 'muted' || result.nonzero_samples > 0, 'no native sound reached the real browser graph');
  binding.dispose();
  return { rate: testCase.rate, mode, blocks: testCase.frames.length, native_end: nativeEnd,
    retained_clock_centre: last.native.clock.centre_ref, schedule_ms: scheduleMs,
    browser_render_ms: performance.now() - renderStarted, ...result };
}
async function recoveryCase(testCase, newContext) {
  const context = contextFor(testCase.rate, testCase.recovered.audio.length + 2048), owner = {};
  let binding = new NativeAudioBinding(context, { owner, initialFrame: testCase.initial,
    gain: 0.25, muted: false, leadSeconds: 128 / testCase.rate });
  for (const frame of testCase.frames) binding.apply(frame);
  let outputContext = context;
  if (newContext) {
    binding.dispose();
    // A new device epoch, NOT a replay/reset of the persistent C++ simulation.
    outputContext = contextFor(testCase.rate, testCase.recovered.audio.length + 2048);
    binding = new NativeAudioBinding(outputContext, { owner, initialFrame: testCase.recovery_read,
      gain: 0.25, muted: false, leadSeconds: 128 / testCase.rate });
  } else {
    binding.hold('provider-unavailable');
    refused(() => binding.apply(testCase.recovered), 'held receiver accepted more PCM');
    binding.rebase(testCase.recovery_read, 'provider-return-current-native-read');
    check(binding.lastReceipt.device_epoch === 1, 'missing explicit discontinuity');
  }
  const receipt = binding.apply(testCase.recovered);
  const origin = Math.round(receipt.context_origin_seconds * testCase.rate);
  check(receipt.native_origin === testCase.recovery_read.samples_elapsed, 'recovery reset native sample time');
  const rendered = await outputContext.startRendering();
  const result = compare(rendered.getChannelData(0), testCase.recovered.audio, origin, 0.25);
  if (newContext) {
    const dropped = await context.startRendering();
    check(dropped.getChannelData(0).every(value => value === 0), 'disposed old context still sounded');
  }
  binding.dispose();
  return { new_context: newContext, native_origin: receipt.native_origin, ...result };
}
async function negativeCases(testCase) {
  const context = contextFor(testCase.rate, 8192);
  const binding = new NativeAudioBinding(context, { owner: {}, initialFrame: testCase.initial,
    leadSeconds: 0.01, maxBlocks: 1, maxQueuedFrames: 8192 });
  const first = testCase.frames[0]; binding.apply(first);
  const before = JSON.stringify(binding.lastReceipt);
  for (const mutate of [f => f.subject_ref = 'another-nara', f => f.audio[0] = NaN,
      f => f.samples_elapsed = '99999', f => f.sample_rate = 48001]) {
    const corrupt = structuredClone(testCase.frames[1]); mutate(corrupt);
    refused(() => binding.apply(corrupt), 'invalid real-browser input admitted');
    check(JSON.stringify(binding.lastReceipt) === before, 'invalid input mutated the native receipt');
  }
  refused(() => binding.apply(testCase.frames[1]), 'queue ceiling ignored');
  check(binding.capacity.blocks === 0, 'incorrect capacity');
  binding.dispose();
  const rendered = await context.startRendering();
  check(rendered.getChannelData(0).every(value => value === 0), 'rejected/disposed audio reached output');
  return { invalid_atomic: true, queue_bound: true, disposed_silence: true };
}
async function main() {
  const input = window.nativeAudioCases ?? await (await fetch('./audio-cases.json')).json();
  const results = [];
  for (const testCase of input.cases) results.push(await renderCase(testCase, 'audible'));
  results.push(await renderCase(input.cases[1], 'muted'));
  results.push(await renderCase(input.cases[1], 'resume'));
  const recovery = [await recoveryCase(input.cases[1], false), await recoveryCase(input.cases[1], true)];
  const negative = await negativeCases(input.cases[1]);
  return { schema: 'ql.k8-native-audio-browser-acceptance/v1', native_worker: input.worker_sha256,
    results, recovery, negative,
    limits: ['OfflineAudioContext executes the browser audio graph, not physical speaker or user-gesture acceptance',
      'Controlled geometry/material modes; source musical correspondences do not prove empirical effects',
      'Scheduling deadlines depend on the native host feeding a bounded lookahead queue',
      'Detached cross-process audio ownership remains with the native host'] };
}
main().then(result => window.acceptance = result).catch(error => window.acceptance = { error: String(error), stack: error.stack });
