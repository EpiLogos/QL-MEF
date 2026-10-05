import assert from "node:assert/strict";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { resolve, join } from "node:path";
import { pathToFileURL } from "node:url";

// Real Pi agent turns and hooks with an explicitly permitted acting provider.
// The decision provider is never called: the event is already determined.
const [packageArg, cwdArg, sdkArg, eventPath, provider, modelId, outputPath] = process.argv.slice(2);
assert(packageArg && cwdArg && sdkArg && eventPath && provider && modelId && outputPath);
const [packageRoot, cwd, sdkRoot] = [packageArg, cwdArg, sdkArg].map(p => resolve(p));
await mkdir(cwd, {recursive:true});
const sdk = await import(pathToFileURL(join(sdkRoot, "dist/index.js")).href);
const globalDir = sdk.getAgentDir();
const runtime = await sdk.ModelRuntime.create({ authPath: join(globalDir, "auth.json"),
  modelsPath: join(globalDir, "models.json"), refreshOnCreate: false });
const model = runtime.getModel(provider, modelId);
assert(model, "explicit acting model must exist in the real native runtime");
const event = JSON.parse(await readFile(eventPath, "utf8"));
delete event.bindings;
event.observed = [
  {field:"lens",value:"L2'",origin:"observed",basis_refs:[event.material.ref]},
  {field:"local-position",value:3,origin:"observed",basis_refs:[event.material.ref]},
  {field:"coordinate-face",value:"direct",origin:"observed",basis_refs:[event.material.ref]},
  {field:"musical-basis",value:"chromatic",origin:"observed",basis_refs:[event.material.ref]},
];
const loader = new sdk.DefaultResourceLoader({ cwd, agentDir: join(cwd, "agent"),
  additionalExtensionPaths:[packageRoot], noExtensions:true, noSkills:true,
  noPromptTemplates:true, noThemes:true, noContextFiles:true });
await loader.reload(); assert.deepEqual(loader.getExtensions().errors, []);
const {session} = await sdk.createAgentSession({ cwd, agentDir:join(cwd,"agent"),
  modelRuntime:runtime, model, thinkingLevel:"low", resourceLoader:loader,
  sessionManager:sdk.SessionManager.inMemory(cwd), tools:["ql_project_event"] });
let expired = false;
const timer = setTimeout(() => { expired = true; void session.abort(); }, 180000);
const custom = name => session.sessionManager.getEntries().filter(e =>
  e.type === "custom" && e.customType === name);
const readings = () => session.sessionManager.getEntries().filter(e =>
  e.type === "custom_message" && e.customType === "ql-native-event-context");
try {
  await session.bindExtensions({mode:"headless"});
  await session.prompt("/ql-mode on");
  const activeModeEntry = custom("ql-body-mode").at(-1).id;
  await session.prompt("Call ql_project_event once with this exact request. Preserve its native result; "
    + "do not calculate an alternative reading. Return one sentence.\n"
    + JSON.stringify({event,requested_heads:["lens"]}));
  assert(!expired);
  const bodyEvents = custom("ql-body-event");
  assert(bodyEvents.length > 0, "real agent tool results must reach the native event hook");
  assert.equal(bodyEvents.at(-1).data.event_ref, event.event_ref);
  await session.prompt("Continue from the native QL receipt. State its event reference and harmonic pitch class in one sentence. Do not call tools.");
  assert(!expired);
  assert(readings().length > 0, "native receipt must enter actual subsequent agent context");
  const lastReading = readings().at(-1);
  const native = JSON.parse(lastReading.content.slice(lastReading.content.indexOf("\n") + 1));
  assert.equal(native.event.event_ref, event.event_ref);
  assert.deepEqual(native.decision_head_ids, []);
  assert(!Object.hasOwn(native.determination, "provider"));
  assert.equal(lastReading.details.body, "pi");
  const continued = session.sessionManager.getEntries().filter(e=>e.type==="message"
    && e.message.role==="assistant").at(-1).message;
  const reply = continued.content.filter(c=>c.type==="text").map(c=>c.text).join("\n");
  const pitch = native.harmonic.harmonic.find(f=>f.field==="pitch-class").value;
  assert(reply.includes(event.event_ref), "acting agent must use the exact native event reference");
  assert(new RegExp(`\\b${pitch}\\b`).test(reply), "acting agent must use the native harmonic pitch");
  const count = readings().length;
  const invalid = structuredClone(event);
  invalid.observed[0].value = "L99";
  await session.prompt("Call ql_project_event once with this exact invalid request. Do not repair it or retry. "
    +"Return one sentence describing its refusal.\n"+JSON.stringify({event:invalid,requested_heads:["lens"]}));
  assert(!expired);
  const failedCall = session.sessionManager.getEntries().filter(e=>e.type==="message"
    && e.message.role==="toolResult" && e.message.toolName==="ql_project_event").at(-1);
  assert.equal(failedCall.message.isError,true,"actual native owner must refuse the impossible coordinate");
  const afterFailureCount = readings().length;
  await session.prompt("/ql-status");
  assert.equal(custom("ql-body-status").at(-1).data.event_ref,null,
    "a failed newer owner call must clear the previous event basis");
  await session.prompt("Reply with the single word continued. Do not call tools.");
  assert(!expired); assert.equal(readings().length,afterFailureCount,
    "a failed newer QL call must not reinject the superseded reading");
  await session.prompt("/ql-mode off");
  await session.prompt("Reply with the single word continued. Do not call tools.");
  assert(!expired); assert.equal(readings().length, afterFailureCount);
  assert(!session.sessionManager.getEntries().some(e => e.type === "message"
    && e.message.role === "assistant" && e.message.stopReason === "error"));
  await session.navigateTree(activeModeEntry, {summarize:false});
  await session.prompt("/ql-status");
  const branchStatus = custom("ql-body-status").at(-1).data;
  assert.equal(branchStatus.enabled, true, "mode comes from the active branch, not abandoned off markers");
  assert.equal(branchStatus.event_ref, null, "branch navigation clears the abandoned reading");
  await session.prompt("Reply with the single word continued. Do not call tools.");
  assert(!expired);
  assert.equal(session.sessionManager.getBranch().filter(e =>
    e.type === "custom_message" && e.customType === "ql-native-event-context").length, 0,
    "a later branch's receipt must not reach the earlier branch's next turn");
  await writeFile(outputPath, JSON.stringify({schema:"ql.native-event-loop-proof/v1",
    body:"pi", provider, model:modelId, acting_agent_turns:6, decision_provider_calls:0,
    native_tool_events:bodyEvents.length, event_ref:event.event_ref,
    continuation_readings:count, actual_reply_event_ref:event.event_ref,actual_reply_pitch_class:pitch,
    failed_newer_call_clears_reading:true, mode_off_ordinary_continuation:true, branch_navigation_stale_receipt_refused:true,
    entries:session.sessionManager.getEntries()}, null, 2) + "\n");
  process.stdout.write(JSON.stringify({body:"pi",native_tool_events:bodyEvents.length,
    continuation_readings:count,decision_provider_calls:0,mode_off_ordinary_continuation:true})+"\n");
} catch (error) {
  await writeFile(outputPath + ".failure.json", JSON.stringify({
    standing:"failed actual event-loop execution", error:String(error),
    entries:session.sessionManager.getEntries()}, null, 2) + "\n");
  throw error;
} finally { clearTimeout(timer); session.dispose(); }
