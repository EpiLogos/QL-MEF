import * as THREE from 'three';
import { GPGPUSimulator } from '../../target/point-cloud-source/src/engine/GPGPUSimulator';
import { RetainedFieldBinding } from './retained-field.mjs';
import { InstrumentSession } from './instrument-session.mjs';

const check = (ok: unknown, message: string) => { if (!ok) throw new Error(message); };
const bridge = window as any;
const delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
const animationFrame = () => new Promise<number>(resolve => requestAnimationFrame(resolve));
function percentile(values: number[], fraction: number) {
  check(values.length > 0 && fraction > 0 && fraction <= 1, 'invalid percentile observation');
  const ordered = [...values].sort((a, b) => a - b);
  return ordered[Math.max(0, Math.ceil(ordered.length * fraction) - 1)];
}
async function at(context: AudioContext, time: number) {
  const deadline = performance.now() + 10000;
  while (context.currentTime < time) {
    check(performance.now() < deadline, 'real audio device clock stopped');
    await delay(4);
  }
}
async function main() {
  const ready = await bridge.testNativeReady();
  const changed = await bridge.testChangedBasis();
  const canvas = document.querySelector('canvas')!;
  const renderer = new THREE.WebGLRenderer({ canvas, antialias: false });
  renderer.setSize(64, 64);
  check(renderer.capabilities.isWebGL2, 'requires actual retained WebGL2');
  const simulator = new GPGPUSimulator(renderer, 65536);
  const count = simulator.particleCount;
  const data = new Float32Array(count * 4);
  for (let i = 0; i < count; i++) {
    data[i * 4] = 0.7; data[i * 4 + 1] = -0.5; data[i * 4 + 3] = 0.25 + (i % 7) / 10;
  }
  const texture = () => new THREE.DataTexture(data.slice(), simulator.texWidth, simulator.texHeight, THREE.RGBAFormat, THREE.FloatType);
  const a = texture(), b = texture();
  let seeds = 0;
  const seed = simulator.seedInitialState.bind(simulator);
  simulator.seedInitialState = values => { seeds++; seed(values); };
  simulator.seedInitialState(data);
  const samples = ready.field.targets.length;
  const slotsA = Uint32Array.from({ length: count }, (_, i) => i % samples);
  const slotsB = Uint32Array.from({ length: count }, (_, i) => (i + Math.floor(samples / 2)) % samples);
  const binding = new RetainedFieldBinding(simulator, { initialFrame: ready.field, targetA: a, targetB: b, slotsA, slotsB });
  const config: any = { style: 'particle', fluid: { curlScale: 1, curlSpeed: 0, turbulence: 0,
    vortexStrength: 0, viscosity: 2, returnSpeed: 3, dispersion: 0 },
    interaction: { radius: 0, strength: 0, mode: 'repel' }, relational: { enabled: false } };
  // Warm the actual shader before starting the presentation clock. This is the
  // retained GPU's own integration, not a second CPU implementation.
  simulator.step(1 / 120, 0, config, 0, new THREE.Vector2(20, 20), new THREE.Vector2());
  const resident = binding.checkpoint(renderer);
  const targetObject = binding.targetA, targetArray = binding.targetA.image.data;
  const initialTargets = targetArray.slice();
  const context = new AudioContext({ sampleRate: ready.field.sample_rate });
  await context.suspend();
  const calls: any[] = [], applications: any[] = [];
  let endpointClosed = false;
  const transport = {
    async request(request: any) {
      const start = performance.now();
      const result = await bridge.testNativeExchange(request);
      const complete = performance.now();
      calls.push({ operation: request.command.operation, request_id: request.request_id,
        started_at_ms: start, completed_at_ms: complete, elapsed_ms: complete - start,
        response_bytes: JSON.stringify(result).length,
        generation: result.field.generation, samples_elapsed: result.field.samples_elapsed });
      return result;
    }, close() { endpointClosed = true; }
  };
  const session = new InstrumentSession({ context, owner: {}, transport, initialReceipt: ready,
    fieldBinding: {
      validate(frame: any) { return binding.validate(frame); },
      apply(frame: any) {
        const result = binding.apply(frame);
        applications.push({ generation: frame.generation, samples_elapsed: frame.samples_elapsed,
          device_seconds: context.currentTime, observed_at_ms: performance.now() });
        return result;
      }
    }, blockFrames: 4096, leadSeconds: 0.05, lookaheadSeconds: 0.4,
    maxBlocks: 8, maxQueuedBytes: 8 * 1024 * 1024, gain: 0.25, muted: false });
  const inspected = await session.inspect();
  check(inspected.original.m1 && inspected.original.m2 && inspected.original.m3 && inspected.original_field,
    'complete source basis missing from actual host inspection');
  const views = [session.openView(), session.openView()];
  const readsBefore = calls.length;
  for (let i = 0; i < 1000; i++) for (const view of views) view.read();
  check(calls.length === readsBefore, 'companion created native work');
  check(!JSON.stringify(views[0].read()).includes('source_receipts'), 'companion leaked full source');
  await session.pump();
  const firstEnd = session.reading.audio.target_context_seconds;
  await session.pump(); await session.pump();
  await session.operate({ operation: 'set-axis', axis: 1, phase: { turns: '-2', half_degrees: 37 } });
  await session.operate({ operation: 'replace', basis: changed });
  await session.pump();
  const scheduled = session.reading;
  check(scheduled.acknowledged.samples_elapsed === '16384', 'wrong native advancement');
  check(binding.lastReceipt.samples_elapsed === '0' && initialTargets.every((v: number, i: number) => v === targetArray[i]),
    'future native frame moved targets before its sound');
  check(seeds === 1, 'delivery reseeded particles');
  session.setMuted(true); session.setMuted(false);
  await context.resume();
  await at(context, firstEnd - 0.04);
  session.present();
  check(binding.lastReceipt.samples_elapsed === '0', 'targets precede first end-of-block');
  await at(context, scheduled.audio.target_context_seconds + 0.01);
  session.present();
  check(binding.lastReceipt.samples_elapsed === '16384' && binding.lastReceipt.generation === scheduled.acknowledged.generation,
    'same native sound/field generation did not arrive');
  check(applications[0].device_seconds >= scheduled.audio.target_context_seconds, 'wrong device relation');
  check(targetObject === binding.targetA && targetArray === binding.targetA.image.data, 'resident target identity changed');
  check(data.every((v, i) => i % 4 !== 3 || v === targetArray[i]), 'material/image density lost');
  const afterDelivery = binding.checkpoint(renderer);
  check(resident.position.every((v: number, i: number) => Object.is(v, afterDelivery.position[i])) &&
    resident.velocity.every((v: number, i: number) => Object.is(v, afterDelivery.velocity[i])),
    'target delivery secretly integrated particles');
  for (let i = 0; i < 20; i++) simulator.step(1 / 120, (i + 1) / 120, config, 0, new THREE.Vector2(20, 20), new THREE.Vector2());
  const moving = binding.checkpoint(renderer);
  check(moving.position.some((v: number, i: number) => Math.abs(v - resident.position[i]) > 1e-6), 'retained GPU did not follow native targets');
  session.hold('controlled-device-and-gpu-reentry'); await context.suspend();
  const event = (name: string) => new Promise<void>(resolve => canvas.addEventListener(name, e => { e.preventDefault(); resolve(); }, { once: true }));
  const lost = event('webglcontextlost'); renderer.forceContextLoss(); await lost;
  await delay(100);
  const restored = event('webglcontextrestored'); renderer.forceContextRestore(); await restored;
  binding.restore(renderer, moving);
  await session.recover('explicit-current-read-after-device-return');
  check(session.reading.acknowledged.samples_elapsed === '16384', 'recovery changed native clock');
  await session.pump();
  const recovery = session.reading;
  check(recovery.audio.interval.native_start === '16384' && recovery.audio.device_epoch === 1,
    'recovery replays old PCM or loses native identity');

  // Exercise the actual unsuspended periodic data plane while the retained GPU is
  // integrating. Start the bounded driver while the first recovered interval is
  // still ahead of the device clock. Waiting until after that interval ends before
  // starting would create a real gap, and the receiver must reject the next
  // contiguous native block as late rather than silently starting it "now".
  const liveCallsBefore = calls.length, liveApplicationsBefore = applications.length;
  const liveStart = performance.now(), liveDeviceStart = context.currentTime;
  const liveStartSamples = BigInt(session.reading.acknowledged.samples_elapsed);
  const frameIntervals: number[] = [], generationAges: number[] = [], audioDeadlineSlack: number[] = [];
  let lastAnimation = liveStart, maxQueuedBlocks = 0, maxQueuedBytes = 0;
  await context.resume();
  session.start(8);
  await at(context, recovery.audio.target_context_seconds + 0.01);
  session.present();
  check(binding.lastReceipt.samples_elapsed === '20480' && seeds === 1, 're-entry reset or reseeded material');
  for (let i = 0; i < 72; i++) {
    const stamp = await animationFrame();
    frameIntervals.push(stamp - lastAnimation); lastAnimation = stamp;
    simulator.step(1 / 60, context.currentTime, config, 0, new THREE.Vector2(20, 20), new THREE.Vector2());
    const reading = session.present();
    check(reading.available && !reading.held, 'sustained instrument lost its admitted audio/native owner');
    maxQueuedBlocks = Math.max(maxQueuedBlocks, reading.queued_blocks);
    maxQueuedBytes = Math.max(maxQueuedBytes, reading.queued_bytes);
    const acknowledged = BigInt(reading.acknowledged.samples_elapsed);
    const presented = BigInt(reading.presented.samples_elapsed);
    check(acknowledged >= presented, 'presentation ran ahead of native acknowledgement');
    generationAges.push(Number(acknowledged - presented) / context.sampleRate);
    if (reading.audio?.interval?.start_context_seconds !== undefined) {
      audioDeadlineSlack.push(reading.audio.interval.start_context_seconds - reading.audio.observed_context_seconds);
    }
  }
  session.hold('controlled-sustained-measurement-stop');
  while (session.reading.in_flight) await delay(2);
  const liveCallsAfter = calls.length;
  const liveNativeCalls = calls.slice(liveCallsBefore, liveCallsAfter);
  const liveAdvanceCalls = liveNativeCalls.filter(call => call.operation === 'advance');
  const liveApplications = applications.slice(liveApplicationsBefore);
  const liveAdvancedFrames = BigInt(session.reading.acknowledged.samples_elapsed) - liveStartSamples;
  check(liveAdvanceCalls.length >= 4 && liveAdvancedFrames >= 4n * 4096n,
    'sustained acceptance did not execute enough real native blocks');
  check(generationAges.every(value => value <= 0.5), 'smooth browser became more than 0.5 s stale');
  check(audioDeadlineSlack.length > 0 && audioDeadlineSlack.every(value => value >= 0), 'native audio missed its device deadline');
  check(Math.max(...frameIntervals) <= 500, 'controlled presentation stalled for more than 500 ms');
  check(maxQueuedBlocks <= 8 && maxQueuedBytes <= 8 * 1024 * 1024, 'presentation queue exceeded declared ceiling');
  const coherentMs: number[] = [];
  for (const call of liveAdvanceCalls) {
    const application = liveApplications.find(item => item.samples_elapsed === call.samples_elapsed);
    if (application) coherentMs.push(application.observed_at_ms - call.started_at_ms);
  }
  check(coherentMs.length > 0 && coherentMs.every(value => value >= 0), 'no attributable native-to-coherent-output observation');
  await session.recover('controlled-sustained-measurement-reconcile');
  check(session.reading.queued_blocks === 0 && session.reading.queued_bytes === 0,
    'explicit sustained-run recovery left presentation backlog');
  check(seeds === 1, 'sustained operation reseeded retained particles');
  const sustained = {
    presentation_frames: frameIntervals.length,
    device_seconds: context.currentTime - liveDeviceStart,
    wall_ms: performance.now() - liveStart,
    native_calls: liveNativeCalls.length,
    native_advance_blocks: liveAdvanceCalls.length,
    native_frames_advanced: liveAdvancedFrames.toString(),
    frame_ms: { p50: percentile(frameIntervals, 0.50), p95: percentile(frameIntervals, 0.95),
      p99: percentile(frameIntervals, 0.99), max: Math.max(...frameIntervals),
      over_100ms: frameIntervals.filter(value => value > 100).length, ceiling_ms: 500 },
    generation_age_seconds: { p95: percentile(generationAges, 0.95), max: Math.max(...generationAges), ceiling: 0.5 },
    audio_deadline_slack_seconds: { min: Math.min(...audioDeadlineSlack), samples: audioDeadlineSlack.length,
      missed_deadlines: 0, standing: 'late scheduling is rejected by the native audio receiver before playback' },
    native_to_coherent_output_ms: { samples: coherentMs.length, p95: percentile(coherentMs, 0.95), max: Math.max(...coherentMs) },
    max_queued_blocks: maxQueuedBlocks, max_queued_bytes: maxQueuedBytes,
    coalesced_presentation_frames: session.reading.coalesced_presentation_frames,
    gpu_steps: frameIntervals.length,
    end_queue_blocks: session.reading.queued_blocks, end_queue_bytes: session.reading.queued_bytes,
    standing: 'controlled unsuspended Chromium/SwiftShader WebAudio plus retained GPU run; fixed diagnostic ceilings, not owner-hardware performance budget'
  };

  const debug = renderer.getContext().getExtension('WEBGL_debug_renderer_info');
  const result = { schema: 'ql.k8-managed-browser-acceptance/v1', native_commands: calls.length,
    controlled_native_calls: calls, actual_particles: count, seeds, read_only_view_reads: 2000,
    source_complete_inspection: true, same_native_sound_field_generation: true, pure_future_admission: true,
    preserved_position_velocity: true, retained_target_identity_density: true, actual_gpu_motion: true,
    gpu_recovery_then_explicit_audio_epoch: true, sustained_live_measurement: sustained,
    presentation_applications: applications,
    last_reading: views[0].read(), actual_audio_sample_rate: context.sampleRate,
    renderer: debug ? renderer.getContext().getParameter(debug.UNMASKED_RENDERER_WEBGL) : 'undisclosed',
    standing: 'actual installed Rust/C++ host, browser WebAudio clock and retained WebGL receiver; controlled host transport/geometry, not owner-machine or full Nara/AW performance acceptance' };
  session.dispose(); check(endpointClosed, 'driver did not release its endpoint');
  binding.dispose(); simulator.destroy(); a.dispose(); b.dispose(); renderer.dispose(); await context.close();
  return result;
}
main().then(result => bridge.acceptance = result).catch(error => bridge.acceptance = { error: String(error), stack: error.stack });