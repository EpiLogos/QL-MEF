import * as THREE from 'three';
import { GPGPUSimulator } from '__POINT_CLOUD_SOURCE__';
import { MAX_FORMATIONS } from '__FIELD_MODEL_SOURCE__';
import { RetainedFieldBinding } from '__RETAINED_BINDING__';
import { InstrumentSession } from '__INSTRUMENT_SESSION__';
const bridge=window as any;
const check=(v:unknown,m:string)=>{if(!v)throw Error(m);};
const delay=(ms:number)=>new Promise(r=>setTimeout(r,ms));
async function digest(values:Float32Array){const b=await crypto.subtle.digest('SHA-256',values.buffer);return Array.from(new Uint8Array(b),v=>v.toString(16).padStart(2,'0')).join('');}
async function waitForDevice(context:AudioContext,time:number){const end=performance.now()+10000;while(context.currentTime<time){check(performance.now()<end,'actual audio device clock stalled');await delay(4);}}
const nativePhysics:any={style:'particle',fluid:{curlScale:1,curlSpeed:0,turbulence:0,vortexStrength:0,viscosity:2,returnSpeed:3,dispersion:0},interaction:{radius:0,strength:0,mode:'repel'},relational:{enabled:false}};
async function make(name:string){
 const ready=await bridge.nativeSceneOpen(name),canvas=document.createElement('canvas');document.body.append(canvas);
 const renderer=new THREE.WebGLRenderer({canvas,antialias:false});renderer.setSize(64,64);check(renderer.capabilities.isWebGL2,'real WebGL2 required');
 const gl=renderer.getContext(),info=gl.getExtension('WEBGL_debug_renderer_info');
 const environment={browser:navigator.userAgent,webgl2:renderer.capabilities.isWebGL2,renderer:info?gl.getParameter(info.UNMASKED_RENDERER_WEBGL):gl.getParameter(gl.RENDERER),vendor:info?gl.getParameter(info.UNMASKED_VENDOR_WEBGL):gl.getParameter(gl.VENDOR)};
 const sim=new GPGPUSimulator(renderer,4096),count=sim.texWidth*sim.texHeight,initial=new Float32Array(count*4);
 for(let i=0;i<count;i++){initial[i*4]=.7;initial[i*4+1]=-.5;initial[i*4+3]=.25+(i%7)/10;}
 sim.setEntityState({count:1,connectionStart:count,bounds:new Float32Array(MAX_FORMATIONS).fill(count),centers:Array.from({length:MAX_FORMATIONS},()=>new THREE.Vector4(0,0,0,1)),morph:new Float32Array(MAX_FORMATIONS),transforms:Array.from({length:MAX_FORMATIONS},()=>new THREE.Vector3(1,1,0))});
 let seeds=0;const nativeSeed=sim.seedInitialState.bind(sim);sim.seedInitialState=(v:any)=>{seeds++;nativeSeed(v);};sim.seedInitialState(initial);
 const texture=()=>new THREE.DataTexture(initial.slice(),sim.texWidth,sim.texHeight,THREE.RGBAFormat,THREE.FloatType);
 const slotsA=Uint32Array.from({length:count},(_,i)=>i%ready.field.targets.length),slotsB=Uint32Array.from({length:count},(_,i)=>(i+2048)%ready.field.targets.length);
 const binding=new RetainedFieldBinding(sim,{initialFrame:ready.field,targetA:texture(),targetB:texture(),slotsA,slotsB});
 // Warm the real shader before establishing the real device epoch.
 sim.step(1/120,0,nativePhysics,0,new THREE.Vector2(20,20),new THREE.Vector2());
 const resident=binding.checkpoint(renderer),context=new AudioContext({sampleRate:ready.field.sample_rate});await context.suspend();
 check(context.sampleRate===ready.field.sample_rate,'actual audio/native sample rate must agree');
 const applications:any[]=[],calls:any[]=[];let closed=false;
 const transport={async request(q:any){const raw=await bridge.nativeSceneExchange(name,q);let reply=structuredClone(raw);
  if(name==='negative-m1-delta1')reply.field.generation=String(BigInt(q.expected_generation)+1n);
  if(name==='negative-m1-delta4')reply.field.generation=String(BigInt(q.expected_generation)+4n);
  if(name==='negative-replace-delta3')reply.field.generation=String(BigInt(q.expected_generation)+3n);
  if(name==='negative-reordered')reply.request_id=String(BigInt(q.request_id)+1n);
  if(name==='negative-malformed-target')reply.field.targets[0].identity+=1;
  if(name==='negative-sample-drift')reply.field.samples_elapsed=String(BigInt(q.expected_samples_elapsed)+1n);
  if(name==='negative-late-ack')await delay(200);
  calls.push({request:q,actual_native:raw,delivered:reply,negative_injection:name.startsWith('negative-')});return reply;
 },close(){closed=true;void bridge.nativeSceneClose(name);}};
 const session=new InstrumentSession({context,owner:{},transport,initialReceipt:ready,fieldBinding:{validate:(f:any)=>binding.validate(f),apply:(f:any)=>{const result=binding.apply(f);applications.push({generation:f.generation,samples:f.samples_elapsed,deviceTime:context.currentTime});return result;}},blockFrames:512,leadSeconds:.4,lookaheadSeconds:.5,maxBlocks:8,maxQueuedBytes:8*1024*1024,gain:.1,muted:true,timeoutMs:name==='negative-late-ack'?100:20000});
 const copyTargetGpu=(tex:THREE.DataTexture)=>{const output=new THREE.WebGLRenderTarget(sim.texWidth,sim.texHeight,{type:THREE.FloatType,format:THREE.RGBAFormat,depthBuffer:false,stencilBuffer:false});const scene=new THREE.Scene(),camera=new THREE.OrthographicCamera(-1,1,1,-1,0,1);const material=new THREE.ShaderMaterial({uniforms:{source:{value:tex}},vertexShader:'varying vec2 vUV;void main(){vUV=uv;gl_Position=vec4(position.xy,0.,1.);}',fragmentShader:'uniform sampler2D source;varying vec2 vUV;void main(){gl_FragColor=texture2D(source,vUV);}'});const geometry=new THREE.PlaneGeometry(2,2);scene.add(new THREE.Mesh(geometry,material));renderer.setRenderTarget(output);renderer.render(scene,camera);const pixels=new Float32Array(count*4);renderer.readRenderTargetPixels(output,0,0,sim.texWidth,sim.texHeight,pixels);renderer.setRenderTarget(null);output.dispose();geometry.dispose();material.dispose();return pixels;};
 async function prove(frame:any){await context.resume();await waitForDevice(context,session.reading.audio.target_context_seconds+.005);session.present();check(!session.reading.held&&session.reading.available,'source acknowledgement must remain admitted');check(binding.lastReceipt?.generation===frame.generation,'actual field delivery must adopt acknowledged generation');
  let maxGap=0,wGap=0;for(const [tex,slots] of [[binding.targetA,slotsA],[binding.targetB,slotsB]] as const){const pixels=copyTargetGpu(tex);for(let i=0;i<slots.length;i++){for(let axis=0;axis<3;axis++)maxGap=Math.max(maxGap,Math.abs(pixels[i*4+axis]-Math.fround(frame.targets[slots[i]].position[axis])));wGap=Math.max(wGap,Math.abs(pixels[i*4+3]-initial[i*4+3]));}}
  check(maxGap===0&&wGap===0,'actual GPU A/B textures must exactly consume every mapped native target and preserve authored w');
  const preserved=binding.checkpoint(renderer);check(preserved.position.every((v:number,i:number)=>v===resident.position[i])&&preserved.velocity.every((v:number,i:number)=>v===resident.velocity[i]),'delivery must not reset physical state');check(seeds===1,'delivery must not reseed');
  for(let i=0;i<20;i++)sim.step(1/120,(i+1)/120,nativePhysics,0,new THREE.Vector2(20,20),new THREE.Vector2());const moved=binding.checkpoint(renderer);let maxMove=0;for(let i=0;i<moved.position.length;i++)maxMove=Math.max(maxMove,Math.abs(moved.position[i]-preserved.position[i]));check(maxMove>0,'actual retained GPU must receive a physical shader step');
  return{max_target_gap:maxGap,max_w_gap:wGap,seeds,applications,actual_physics_max_move:maxMove,resident_before_sha256:await digest(resident.position),resident_after_delivery_sha256:await digest(preserved.position),resident_after_physics_sha256:await digest(moved.position),audio:session.reading.audio};
 }
 async function close(){session.dispose();await bridge.nativeSceneClose(name);binding.dispose();sim.destroy();renderer.dispose();await context.close();canvas.remove();}
 return{ready,session,binding,context,environment,calls,applications,prove,close,closed:()=>closed};
}
async function main(){const rows:any[]=[];
 for(const [name,command,delta] of [['m1-delta3',{operation:'m1-advance',ticks:1},3],['m1-delta2',{operation:'m1-advance',ticks:72},2],['replace-delta1',{operation:'replace-event',event:null,strike:false},1],['replace-delta2',{operation:'replace-event',event:null,strike:false},2]] as const){
  const c=await make(name);try{const q:any=structuredClone(command);if(q.operation==='replace-event')q.event=await bridge.nativeSceneRecipe(name);const before=c.session.reading.acknowledged;await c.session.operate(q);const raw=c.calls.at(-1).actual_native;check(Number(BigInt(raw.field.generation)-BigInt(before.generation))===delta,'independently predicted actual native delta');check(c.session.reading.acknowledged.generation===raw.field.generation&&c.session.reading.acknowledged.samples_elapsed===before.samples_elapsed,'adapter must preserve actual native cursor');check(raw.field.audio.length===0,'event produces no PCM');const proof=await c.prove(raw.field);await c.session.influence();check(c.session.reading.acknowledged.generation===raw.field.generation,'native influence read leaves control cursor');rows.push({name,delta,passed:true,environment:c.environment,...proof});}finally{await c.close();}}
 // Same actual native owner: known refusals retain a readable owner, separately
 // from deliberately corrupted delivered receipts below.
 {const c=await make('actual-owner-refusals');try{for(const ticks of [0,1000001]){const before=structuredClone(c.session.reading.acknowledged);let failed=false;try{await c.session.operate({operation:'m1-advance',ticks});}catch{failed=true;}check(failed&&c.session.reading.available&&!c.closed(),'known native refusal remains readable');check(JSON.stringify(before)===JSON.stringify(c.session.reading.acknowledged),'clean native refusal leaves cursor');await c.session.influence();}rows.push({name:'actual-owner-refusals',passed:true,environment:c.environment,reading:c.session.reading});}finally{await c.close();}}
 // Real device scheduling: begin suspended before receiver construction, so
 // actual acknowledged PCM and its event targets can be inspected before END.
 {const c=await make('real-queued-pcm-event');try{await c.session.pump();const end=c.session.reading.audio.target_context_seconds;check(c.context.currentTime===0&&end>0,'actual suspended device holds the future PCM interval');const before=c.binding.targetA.image.data.slice();await c.session.operate({operation:'m1-advance',ticks:1});const raw=c.calls.at(-1).actual_native;check(c.binding.targetA.image.data.every((v:number,i:number)=>v===before[i]),'event target must not precede actual queued PCM end');const proof=await c.prove(raw.field);rows.push({name:'real-queued-pcm-event',passed:true,environment:c.environment,end_context_seconds:end,...proof});}finally{await c.close();}}
 for(const name of ['negative-m1-delta1','negative-m1-delta4','negative-replace-delta3','negative-reordered','negative-malformed-target','negative-sample-drift','negative-late-ack']){const c=await make(name);try{const before=structuredClone(c.session.reading.acknowledged);let error='';const q:any=name==='negative-replace-delta3'?{operation:'replace-event',event:await bridge.nativeSceneRecipe('replace-delta1'),strike:false}:{operation:'m1-advance',ticks:1};try{await c.session.operate(q);}catch(e){error=String(e);}check(error&&c.session.reading.held&&!c.session.reading.available&&c.closed(),'invalid/late actual-native acknowledgement must be uncertain and closed');check(JSON.stringify(before)===JSON.stringify(c.session.reading.acknowledged)&&c.applications.length===0,'unknown acknowledgement cannot pretend cursor adoption or target delivery');await delay(250);rows.push({name,passed:true,negative_injection:true,error,actual_native:c.calls.at(-1)?.actual_native??null,reading:c.session.reading,environment:c.environment});}finally{await c.close();}}
 return{schema:'epi.real-native-scene-instrument-browser/v1',passed:true,scope:'Isolated actual frozen native scene owner → canonical InstrumentSession → real browser AudioContext and retained WebGL2 GPU. Component receiving proof; no ordinary app, managed installed whole or physical hardware audio claim.',cases:rows};
}
main().then(v=>bridge.saveBrowserProof(v).then(()=>((window as any).acceptance=v))).catch(e=>{const value={schema:'epi.real-native-scene-instrument-browser/v1',passed:false,error:String(e),stack:e?.stack};void bridge.saveBrowserProof(value).then(()=>((window as any).acceptance=value));});
