import * as THREE from 'three';
import { GPGPUSimulator } from '../../target/point-cloud-source/src/engine/GPGPUSimulator';
import { RetainedFieldBinding } from './retained-field.mjs';
import { InstrumentSession } from './instrument-session.mjs';

const check = (ok: unknown, message: string) => { if (!ok) throw new Error(message); };
const bridge = window as any;
const delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
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
      calls.push({ operation: request.command.operation, request_id: request.request_id,
        elapsed_ms: performance.now() - start, response_bytes: JSON.stringify(result).length,
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
          device_seconds: context.currentTime });
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
  await context.resume(); await at(context, recovery.audio.target_context_seconds + 0.01); session.present();
  check(binding.lastReceipt.samples_elapsed === '20480' && seeds === 1, 're-entry reset or reseeded material');
  const debug = renderer.getContext().getExtension('WEBGL_debug_renderer_info');
  const result = { schema: 'ql.k8-managed-browser-acceptance/v1', native_commands: calls.length,
    controlled_native_calls: calls, actual_particles: count, seeds, read_only_view_reads: 2000,
    source_complete_inspection: true, same_native_sound_field_generation: true, pure_future_admission: true,
    preserved_position_velocity: true, retained_target_identity_density: true, actual_gpu_motion: true,
    gpu_recovery_then_explicit_audio_epoch: true, presentation_applications: applications,
    last_reading: views[0].read(), actual_audio_sample_rate: context.sampleRate,
    renderer: debug ? renderer.getContext().getParameter(debug.UNMASKED_RENDERER_WEBGL) : 'undisclosed',
    standing: 'actual installed Rust/C++ host, browser WebAudio clock and retained WebGL receiver; controlled host transport/geometry, not owner-machine or full Nara acceptance' };
  session.dispose(); check(endpointClosed, 'driver did not release its endpoint');
  binding.dispose(); simulator.destroy(); a.dispose(); b.dispose(); renderer.dispose(); await context.close();
  return result;
}
main().then(result => bridge.acceptance = result).catch(error => bridge.acceptance = { error: String(error), stack: error.stack });
