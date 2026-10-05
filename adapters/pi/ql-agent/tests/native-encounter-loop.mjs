import assert from "node:assert/strict";
import { mkdir, writeFile } from "node:fs/promises";
import { resolve, join } from "node:path";
import { pathToFileURL } from "node:url";

// Actual SDK hook dispatch and installed owner processes. No acting provider
// or model is substituted to make a synthetic end-to-end claim.
const [packageArg, cwdArg, sdkArg, outputPath] = process.argv.slice(2);
assert(packageArg && cwdArg && sdkArg && outputPath);
const [packageRoot, cwd, sdkRoot] = [packageArg, cwdArg, sdkArg].map(p => resolve(p));
await mkdir(cwd, {recursive:true});
const sdk = await import(pathToFileURL(join(sdkRoot, "dist/index.js")).href);
const loader = new sdk.DefaultResourceLoader({cwd, agentDir:join(cwd,"agent"),
  additionalExtensionPaths:[packageRoot], noExtensions:true, noSkills:true,
  noPromptTemplates:true, noThemes:true, noContextFiles:true});
await loader.reload(); assert.deepEqual(loader.getExtensions().errors, []);
const {session} = await sdk.createAgentSession({cwd, agentDir:join(cwd,"agent"),
  resourceLoader:loader, sessionManager:sdk.SessionManager.inMemory(cwd), tools:["ql_project_event"]});
const custom = name => session.sessionManager.getEntries().filter(e =>
  e.type === "custom" && e.customType === name);
const status = async () => {await session.prompt("/ql-status"); return custom("ql-body-status").at(-1).data;};
const settle = async () => {
  const end = performance.now()+8000;
  while(performance.now()<end) {
    const value = await status(); if(!value.owner_projection_pending) return value;
    await new Promise(r=>setTimeout(r,20));
  }
  assert.fail("actual owner projection did not settle within its transport bound");
};
const text = lens => "QL state: "+JSON.stringify({lens,"local-position":3,
  "coordinate-face":"direct","musical-basis":"chromatic"});
const checks=[];
try {
  await session.bindExtensions({mode:"headless"});
  await session.prompt("/ql-mode on");
  const runner = session.extensionRunner;
  await runner.emitInput("An ordinary edit concern",undefined,"interactive");
  assert.equal((await status()).owner_projector_requests,0);
  checks.push("ordinary-input:no-projector-or-classifier-call");
  const start=performance.now();
  await runner.emitInput(text("L2'"),undefined,"interactive");
  const entryMs=performance.now()-start;
  assert(entryMs<1000,"native input dispatch must return before its background projection");
  const accepted = await settle();
  assert.equal(accepted.owner_projector_requests,1);
  assert(accepted.event_ref);
  const continuation = await runner.emitBeforeAgentStart("Continue",undefined,"",{});
  const message=continuation.messages.find(m=>m.customType==="ql-native-event-context");
  assert(message,"actual next-turn hook carries the owner result");
  const native=JSON.parse(message.content.slice(message.content.indexOf("\n")+1));
  assert.deepEqual(native.decision_head_ids,[]);
  assert.equal(native.harmonic.harmonic.find(f=>f.field==="pitch-class").value,11);
  checks.push("explicit-state:actual-native-pitch-and-deterministic-bypass");
  await runner.emitInput(text("L2'"),undefined,"interactive");
  await runner.emitInput(text("L0"),undefined,"interactive");
  await settle();
  const current=await runner.emitBeforeAgentStart("Continue",undefined,"",{});
  const result=JSON.parse(current.messages[0].content.slice(current.messages[0].content.indexOf("\n")+1));
  assert.equal(result.determination.observed.find(f=>f.field==="lens").value,"L0");
  checks.push("newer-occasion:owned-background-cancellation-and-current-basis");
  await runner.emitInput(text("L2'"),undefined,"interactive");
  await runner.emitInput("Continue ordinary work",undefined,"interactive");
  await new Promise(r=>setTimeout(r,100));
  assert.equal((await status()).event_ref,null);
  assert(!(await runner.emitBeforeAgentStart("Continue",undefined,"",{}))?.messages?.length);
  checks.push("newer-ordinary-input:cancels-unfinished-QL-occasion");
  await runner.emitInput(text("L2'"),undefined,"interactive");
  await runner.emitToolCall({type:"tool_call",toolCallId:"native-read-occasion",toolName:"read",
    input:{path:join(packageRoot,"package.json")}});
  await new Promise(r=>setTimeout(r,100));
  assert.equal((await status()).event_ref,null);
  assert(!(await runner.emitBeforeAgentStart("Continue",undefined,"",{}))?.messages?.length);
  checks.push("newer-ordinary-tool:cancels-unfinished-QL-occasion");
  await runner.emitInput('QL state: {"lens":"L99"}',undefined,"interactive");
  assert.equal((await settle()).event_ref,null);
  assert.match(custom("ql-body-reading-unavailable").at(-1).data.reason,/INVALID_LENS_REF|invalid lens|lens ref/i);
  const refused=await runner.emitBeforeAgentStart("Continue",undefined,"",{});
  assert(!refused?.messages?.length);
  checks.push("impossible-state:native-refusal-clears-context");
  await runner.emitInput("QL read: Study these distance relations.",undefined,"interactive");
  await settle();
  const unresolved=await runner.emitBeforeAgentStart("Continue",undefined,"",{});
  const frame=JSON.parse(unresolved.messages[0].content.slice(unresolved.messages[0].content.indexOf("\n")+1));
  assert.deepEqual(new Set(frame.decision_head_ids),new Set(["semantic-faculty","semantic-operation"]));
  assert.deepEqual(frame.determination.learned,[]);
  checks.push("semantic-concern:native-frame-with-zero-implicit-provider-calls");
  await runner.emitInput(text("L2'"),undefined,"interactive");
  await session.prompt("/ql-mode off");
  await new Promise(r=>setTimeout(r,100));
  assert.equal((await status()).event_ref,null);
  const off=await runner.emitBeforeAgentStart("Continue",undefined,"",{});
  assert(!off?.messages?.length);
  checks.push("mode-off:cancels-pending-and-preserves-ordinary-entry");
  await writeFile(outputPath,JSON.stringify({schema:"ql.native-encounter-loop-proof/v1",
    checks,input_dispatch_ms:entryMs,decision_provider_calls:0,entries:session.sessionManager.getEntries()},null,2)+"\n");
  process.stdout.write(JSON.stringify({checks:checks.length,input_dispatch_ms:entryMs,decision_provider_calls:0})+"\n");
} catch(error) {
  await writeFile(outputPath+".failure.json",JSON.stringify({error:String(error),checks,
    entries:session.sessionManager.getEntries()},null,2)+"\n"); throw error;
} finally {session.dispose();}
