import * as THREE from 'three';
import { GPGPUSimulator } from '../../target/point-cloud-source/src/engine/GPGPUSimulator';
import { RetainedFieldBinding } from './retained-field.mjs';
const check = (condition: unknown, message: string) => { if (!condition) throw new Error(message); };
async function main() {
  const frames = (await (await fetch('./frames.jsonl')).text()).trim().split('\n').map(s => JSON.parse(s))
    .filter(f => f.schema === 'ql.continuous-field/v1');
  const canvas = document.querySelector('canvas')!;
  const renderer = new THREE.WebGLRenderer({canvas, antialias: false}); renderer.setSize(64, 64);
  check(renderer.capabilities.isWebGL2, 'requires actual WebGL2');
  const simulator = new GPGPUSimulator(renderer, 65536);
  const count = simulator.particleCount;
  const data = new Float32Array(count * 4);
  for (let i = 0; i < count; i++) { data[i * 4] = 0.7; data[i * 4 + 1] = -0.5; data[i * 4 + 3] = 0.25 + (i % 7) / 10; }
  const a = new THREE.DataTexture(data, simulator.texWidth, simulator.texHeight, THREE.RGBAFormat, THREE.FloatType);
  const b = new THREE.DataTexture(data.slice(), simulator.texWidth, simulator.texHeight, THREE.RGBAFormat, THREE.FloatType);
  // Controlled correspondence repeated into all actual resident GPU texels;
  // production correspondence comes from the retained image/mesh sampler.
  const slotsA = Uint32Array.from({length: count}, (_, i) => i % 256);
  const slotsB = Uint32Array.from({length: count}, (_, i) => (i + 128) % 256);
  let seeds = 0; const seed = simulator.seedInitialState.bind(simulator);
  simulator.seedInitialState = values => { seeds++; seed(values); };
  simulator.seedInitialState(data);
  const binding = new RetainedFieldBinding(simulator, {initialFrame: frames[0], targetA: a, targetB: b, slotsA, slotsB});
  const textures = [binding.targetA, binding.targetB], arrays = textures.map(t => t.image.data);
  const original = binding.checkpoint(renderer);
  for (const frame of frames) binding.apply(frame);
  check(seeds === 1, 'ordinary update reseeded particles');
  const afterUpdate = binding.checkpoint(renderer);
  check(original.position.every((v, i) => v === afterUpdate.position[i]), 'target update mutated resident position');
  check(original.velocity.every((v, i) => v === afterUpdate.velocity[i]), 'target update mutated resident velocity');
  check(textures[0] === binding.targetA && arrays[0] === binding.targetA.image.data, 'target buffer identity lost');
  check(data.every((v, i) => i % 4 !== 3 || (v === arrays[0][i] && v === arrays[1][i])), 'image density overwritten');
  const last = frames.at(-1); const corrupt = structuredClone(last); corrupt.targets[0].position[0] = NaN;
  let refused = false; try { binding.apply(corrupt); } catch { refused = true; } check(refused, 'invalid target accepted');
  refused = false; try { binding.apply(frames[0]); } catch { refused = true; } check(refused, 'stale target accepted');
  const config: any = {style:'particle', fluid:{curlScale:1,curlSpeed:0,turbulence:0,vortexStrength:0,viscosity:2,returnSpeed:3,dispersion:0},
    interaction:{radius:0,strength:0,mode:'repel'}, relational:{enabled:false}};
  for (let i = 0; i < 60; i++) simulator.step(1/120, i/120, config, 0, new THREE.Vector2(20,20), new THREE.Vector2());
  const moving = binding.checkpoint(renderer);
  check(moving.position.some((v, i) => Math.abs(v - original.position[i]) > 1e-6), 'retained GPU did not integrate targets');
  check(moving.velocity.some(v => Math.abs(v) > 1e-7), 'velocity did not survive');
  check(seeds === 1, 'step reseeded');
  const event = (name: string) => new Promise<void>(resolve => canvas.addEventListener(name, e => {e.preventDefault();resolve();}, {once:true}));
  const lost = event('webglcontextlost'); renderer.forceContextLoss(); await lost;
  await new Promise(resolve => setTimeout(resolve, 100));
  const restored = event('webglcontextrestored'); renderer.forceContextRestore(); await restored;
  binding.restore(renderer, moving);
  const recovered = binding.checkpoint(renderer);
  check(moving.position.every((v,i)=>Object.is(v,recovered.position[i])) && moving.velocity.every((v,i)=>Object.is(v,recovered.velocity[i])), 'exact recovery failed');
  check(seeds === 1, 'recovery silently zeroed velocities');
  simulator.step(1/120, 1, config, 0, new THREE.Vector2(20,20), new THREE.Vector2());
  const continued = binding.checkpoint(renderer);
  check(continued.position.some((v,i)=>v!==recovered.position[i]), 'recovered simulator did not continue');
  const gl = renderer.getContext(), debug = gl.getExtension('WEBGL_debug_renderer_info');
  const result = {schema:'ql.k8-retained-gpu-acceptance/v1', source:'040627d0ea40032fe7c7b8c98be9a246be9db7f6',
    actual_particles:count, native_frames:frames.length, seeds, retained_texture_identity:true, density_preserved:true,
    actual_gpu_integration:true, exact_position_velocity_recovery:true, stale_invalid_refused:true,
    renderer:debug ? gl.getParameter(debug.UNMASKED_RENDERER_WEBGL) : gl.getParameter(gl.RENDERER),
    limits:['controlled sample correspondence', 'software WebGL CI is not owner GPU acceptance', 'outer desktop/PointCloudField mounting remains native host work']};
  binding.dispose(); simulator.destroy(); a.dispose(); b.dispose(); renderer.dispose();
  return result;
}
main().then(result => (window as any).acceptance = result).catch(error => (window as any).acceptance = {error:String(error), stack:error.stack});
