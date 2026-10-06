// The live stage: the played-note instrument in one page. The native coupled
// host owns every sounding and visible state; this page only dispatches input
// and presents what the owner's reading discloses — the playable addresses
// come from the host's own derivation, never from a page-local map. The
// retained GPU simulator integrates its particles toward the native targets
// (same as the managed acceptance); the overlay 2D canvas draws those targets
// for human eyes.
import * as THREE from 'three';
import { GPGPUSimulator } from '../../target/point-cloud-source/src/engine/GPGPUSimulator';
import { RetainedFieldBinding } from '../../adapters/retained-field/retained-field.mjs';
import { InstrumentSession } from '../../adapters/retained-field/instrument-session.mjs';

(window as any).bundleVersion = 'v3-session-receipt';
const check = (ok: unknown, message: string) => { if (!ok) throw new Error(message); };
const status = (text: string) => { (document.getElementById('status')!).textContent = text; };

async function main() {
  // The host process boots in a few seconds; a fetch that lands during the
  // boot window retries rather than failing the stage.
  let ready = null as any;
  for (let attempt = 0; attempt < 40; attempt++) {
    const response = await fetch('/native-ready');
    ready = await response.json().catch(() => null);
    if (ready?.status === 'ready' && ready.available) break;
    await new Promise(resolve => setTimeout(resolve, 500));
  }
  ready = ready as any;
  check(ready?.schema === 'ql.field-host-receipt/v1' && ready.status === 'ready', 'native host not ready');
  const glCanvas = document.querySelector('canvas.webgl')!;
  const overlay = document.querySelector('canvas.overlay')!;
  overlay.width = 720; overlay.height = 480;
  const renderer = new THREE.WebGLRenderer({ canvas: glCanvas, antialias: false });
  renderer.setSize(720, 480, false);
  check(renderer.capabilities.isWebGL2, 'requires WebGL2');
  const simulator = new GPGPUSimulator(renderer, 65536);
  const count = simulator.particleCount;
  const samples = ready.field.targets.length;
  const data = new Float32Array(count * 4);
  for (let i = 0; i < count; i++) {
    data[i * 4] = 0.9 * Math.cos(2 * Math.PI * i / count);
    data[i * 4 + 1] = 0.9 * Math.sin(2 * Math.PI * i / count);
    data[i * 4 + 3] = 0.4 + (i % 5) / 12;
  }
  const slotsA = Uint32Array.from({ length: count }, (_, i) => i % samples);
  const slotsB = Uint32Array.from({ length: count }, (_, i) => (i + Math.floor(samples / 2)) % samples);
  const texture = () => new THREE.DataTexture(data.slice(), simulator.texWidth, simulator.texHeight, THREE.RGBAFormat, THREE.FloatType);
  const binding = new RetainedFieldBinding(simulator, {
    initialFrame: ready.field, targetA: texture(), targetB: texture(), slotsA, slotsB,
  });
  const config: any = { style: 'particle', fluid: { curlScale: 1, curlSpeed: 0, turbulence: 0,
    vortexStrength: 0, viscosity: 2, returnSpeed: 3, dispersion: 0 },
    interaction: { radius: 0, strength: 0, mode: 'repel' }, relational: { enabled: false } };
  simulator.step(1 / 120, 0, config, 0, new THREE.Vector2(20, 20), new THREE.Vector2());

  const context = new AudioContext({ sampleRate: ready.field.sample_rate });
  await context.suspend();
  let appliedFrame: any = null;
  const session = new InstrumentSession({ context, owner: {}, initialReceipt: ready, transport: {
    async request(request: any) { return fetch('/native', { method: 'POST', body: JSON.stringify(request) }).then(r => r.json()); },
    close() {},
  }, fieldBinding: {
    validate(frame: any) { return binding.validate(frame); },
    apply(frame: any) { appliedFrame = frame; return binding.apply(frame); },
  }, blockFrames: 4096, leadSeconds: 0.06, lookaheadSeconds: 0.4,
    maxBlocks: 8, maxQueuedBytes: 8 * 1024 * 1024, gain: 0.5, muted: true });

  const inspected = await session.inspect();
  const disclosed = inspected.current?.derivation?.played_addresses?.addresses ?? [];
  check(disclosed.length > 0, 'the reading discloses no playable addresses');
  const KEYS = 'asdfghjk';
  const buttons = document.getElementById('keys')!;
  const strike = async (address: any) => {
    try {
      await session.strike([{ mode_ref: address.mode_ref, amplitude: [0.5, 0.0] }]);
      status(`struck ${address.pitch_name} (slot ${address.octet_slot}, ${address.direct_prime_face}) — cursor ${session.reading.acknowledged.samples_elapsed}`);
    } catch (error) { status(`refused: ${error}`); }
  };
  disclosed.forEach((address: any, index: number) => {
    const button = document.createElement('button');
    button.textContent = `${KEYS[index]} — ${address.pitch_name}`;
    button.onclick = () => strike(address);
    buttons.appendChild(button);
  });
  window.addEventListener('keydown', event => {
    const index = KEYS.indexOf(event.key.toLowerCase());
    if (index >= 0 && index < disclosed.length) strike(disclosed[index]);
  });

  document.getElementById('start')!.onclick = async () => {
    (document.getElementById('start') as HTMLButtonElement).disabled = true;
    // The gesture unlocks audio; a webview that never settles resume() must
    // not take the whole stage down with it.
    Promise.resolve().then(() => context.resume()).catch(error => status(`audio resume: ${error}`));
    session.start(8);
    status('live — play the keys');
  };
  document.getElementById('mute')!.onclick = event => {
    const target = event.target as HTMLButtonElement;
    const muted = target.dataset.muted !== 'true';
    target.dataset.muted = String(muted);
    target.textContent = muted ? 'unmute' : 'mute';
    session.setMuted(muted);
  };

  (window as any).__stage = { session, context, played: disclosed };
  let last = performance.now(), lastDraw = 0;
  const frame = (now: number) => {
    try {
    simulator.step(1 / 60, context.currentTime, config, 0, new THREE.Vector2(20, 20), new THREE.Vector2());
    session.present();
    if (appliedFrame && now - lastDraw > 33) {
      lastDraw = now;
      const ctx = (overlay as HTMLCanvasElement).getContext('2d')!;
      ctx.fillStyle = '#05070d';
      ctx.fillRect(0, 0, overlay.width, overlay.height);
      const targets = appliedFrame.targets;
      for (let i = 0; i < targets.length; i++) {
        const [x, y, z] = targets[i].position;
        const scale = 120;
        ctx.fillStyle = `hsl(${210 + z * 120}, 90%, ${55 + Math.min(30, Math.abs(z) * 60)}%)`;
        ctx.beginPath();
        ctx.arc(overlay.width / 2 + x * scale, overlay.height / 2 + y * scale, 1.8 + Math.min(3, Math.abs(z) * 5), 0, 6.29);
        ctx.fill();
      }
    }
    if (now - last > 500) {
      last = now;
      const reading = session.reading;
      status(`cursor ${reading.acknowledged.samples_elapsed} · gen ${reading.acknowledged.generation} · queued ${reading.queued_blocks} · ${reading.muted ? 'muted' : 'audible'}`);
    }
    } catch (error) {
      status(`frame error: ${error}`);
      (window as any).frameError = String(error);
      return;
    }
    requestAnimationFrame(frame);
  };
  requestAnimationFrame(frame);
}
main().then(() => { (window as any).live = true; })
  .catch(error => { status(String(error)); (window as any).live = { error: String(error), stack: (error as any)?.stack ?? null, fetched: performance.getEntriesByType('resource').map(r => r.name) }; });
