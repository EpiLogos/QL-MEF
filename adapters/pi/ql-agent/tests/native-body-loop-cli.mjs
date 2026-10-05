import assert from "node:assert/strict";
import { mkdir, readFile, readdir, writeFile } from "node:fs/promises";
import { spawn } from "node:child_process";
import { resolve } from "node:path";
import { createHash } from "node:crypto";

// Actual native CLI/RPC body, its acting provider, owner tools and event hooks.
const [executable, body, cwdArg, eventFile, provider, model, outputFile, scenario="tool", planFile] = process.argv.slice(2);
assert(executable && ["pi","prime"].includes(body) && cwdArg && eventFile && provider && model && outputFile);
assert(["tool","encounter","validation","learned"].includes(scenario));
const validationPlan = scenario === "validation" ? JSON.parse(await readFile(planFile,"utf8")) : undefined;
if (validationPlan) assert.equal(validationPlan.schema,"ql.body-validation-plan/v1");
const learnedPlan = scenario === "learned" ? JSON.parse(await readFile(planFile,"utf8")) : undefined;
if (learnedPlan) assert.equal(learnedPlan.schema,"ql.body-learned-uptake-plan/v1");
const usesTool = scenario !== "encounter";
const cwd = resolve(cwdArg); await mkdir(cwd,{recursive:true});
const event = JSON.parse(await readFile(eventFile,"utf8")); delete event.bindings;
event.observed = [
  {field:"lens",value:"L2'",origin:"observed",basis_refs:[event.material.ref]},
  {field:"local-position",value:3,origin:"observed",basis_refs:[event.material.ref]},
  {field:"coordinate-face",value:"direct",origin:"observed",basis_refs:[event.material.ref]},
  {field:"musical-basis",value:"chromatic",origin:"observed",basis_refs:[event.material.ref]},
];
const trace=[], pending=new Map(), turns=[];
let buffer="", bytes=0, turnBytes=0, stderr="", sequence=0, closed=false;
const child=spawn(executable,["--mode","rpc","--offline","--no-session","--no-context-files",
  "--no-themes","--no-prompt-templates","--provider",provider,"--model",model,"--thinking","low",
  "--tools",validationPlan?"ql_project_event,ql_validate_determination,ql_decide,ql_invoke":learnedPlan?"ql_decide,ql_invoke":usesTool?"ql_project_event":body==="prime"?"ipython":"read"],{cwd,shell:false,detached:true,stdio:["pipe","pipe","pipe"]});
const fail=error=>{for(const p of pending.values())p.reject(error);pending.clear();for(const t of turns)t.reject(error);turns.length=0;};
const receive=row=>{
  trace.push(row);
  if(row.type==="response" && pending.has(row.id)){
    const p=pending.get(row.id);pending.delete(row.id);row.success===false?p.reject(new Error(JSON.stringify(row))):p.resolve(row);
  }
  if(row.type==="agent_end" && turns.length)turns.shift().resolve(row);
};
child.stdout.on("data",chunk=>{
  bytes+=chunk.length;turnBytes+=chunk.length;
  if(bytes>64*1024*1024||turnBytes>8*1024*1024){fail(new Error("native RPC trace exceeds total/per-turn bound"));return;}
  buffer+=chunk.toString();let at;
  while((at=buffer.indexOf("\n"))>=0){const line=buffer.slice(0,at);buffer=buffer.slice(at+1);
    if(line.trim())try{receive(JSON.parse(line));}catch(error){fail(error);}}
});
child.stderr.on("data",chunk=>{bytes+=chunk.length;stderr+=chunk.toString();if(bytes>8*1024*1024)fail(new Error("native diagnostics exceed bound"));});
child.on("error",fail);child.on("close",code=>{closed=true;fail(new Error(`native CLI closed ${code}: ${stderr.slice(-1000)}`));});
const request=(type,fields={})=>new Promise((resolvePromise,reject)=>{
  const id=`ql-body-${++sequence}`;pending.set(id,{resolve:resolvePromise,reject});
  child.stdin.write(JSON.stringify({id,type,...fields})+"\n");
});
const turn=async text=>{
  turnBytes=0;
  const ended=new Promise((resolvePromise,reject)=>turns.push({resolve:resolvePromise,reject}));
  await request("prompt",{message:text});return ended;
};
const timer=setTimeout(()=>{fail(new Error("native body loop exceeded 180 seconds"));try{process.kill(-child.pid,"SIGTERM");}catch{}},180000);
let failure;
try{
  const commands=await request("get_commands");assert(JSON.stringify(commands).includes('ql-mode'));
  await request("prompt",{message:"/ql-mode on"});
  if(learnedPlan){
    const canonical=value=>Array.isArray(value)?value.map(canonical):value&&typeof value==="object"
      ?Object.fromEntries(Object.keys(value).sort().map(key=>[key,canonical(value[key])])):value;
    const same=(left,right)=>assert.deepEqual(canonical(left),canonical(right));
    const callOnce=async (prompt,name,expected)=>{
      const before=trace.length;const completed=await turn(prompt);
      assert(!completed.messages.some(m=>m.role==="assistant"&&(m.stopReason==="error"||m.stopReason==="aborted")),
        "native call must also complete its actual actor turn");
      const calls=new Map();
      for(const row of trace.slice(before)){
        const entries=row.type==="tool_execution_start"?[{name:row.toolName,arguments:row.args,id:row.toolCallId}]
          :row.type==="agent_end"?row.messages.filter(m=>m.role==="assistant").flatMap(m=>m.content.filter(c=>c.type==="toolCall")):[];
        for(const entry of entries){const call={name:entry.name,arguments:entry.arguments,id:entry.id};
          if(calls.has(call.id))same(calls.get(call.id),call);calls.set(call.id,call);}
      }
      assert.equal(calls.size,1,"learned uptake must make exactly one actual call without retries");
      const call=[...calls.values()][0];assert.equal(call.name,name);same(call.arguments,expected);
      const ends=trace.slice(before).filter(row=>row.type==="tool_execution_end");assert.equal(ends.length,1);
      assert.equal(ends[0].toolCallId,call.id);assert(!ends[0].isError);
      return ends[0].result.details.native_receipt;
    };
    // Gold fields and expected receipts stay in the verifier. The acting body
    // sees the event, then must carry the accepted faculty into a real read.
    const decision=await callOnce("Call ql_decide exactly once with this complete JSON argument object. "
      +"Copy every key and value exactly, without adding, removing or moving any key. "
      +"Retain its actual learned determination, including any unresolved standing. Do not retry or replace it. "
      +"Return one sentence after the tool finishes.\n"+JSON.stringify(learnedPlan.request),"ql_decide",learnedPlan.request);
    assert.equal(decision.schema,"ql.agent-projection/v1");
    assert.equal(decision.determination.status,"determined");
    same(decision.event,learnedPlan.expected_event);
    const {provider_ref:previousInvocation,...expectedProvider}=learnedPlan.expected_provider;
    const {provider_ref:actualInvocation,...actualProvider}=decision.determination.provider;
    same(actualProvider,expectedProvider);
    assert(actualInvocation!==previousInvocation,"fresh inference must retain its own native invocation identity");
    const execution=JSON.parse(await readFile(process.env.QL_AGENT_DECISION_CONFIG,"utf8"));
    const files=(await readdir(execution.receipt_dir)).filter(name=>name.endsWith(".receipt.json"));
    assert.equal(files.length,1,"isolated fresh body must retain exactly one actual inference receipt");
    const translated=JSON.parse(await readFile(resolve(execution.receipt_dir,files[0]),"utf8"));
    assert.equal(translated.native_envelope.data.invocation_ref,actualInvocation);
    assert.equal(translated.frame_digest,decision.determination.frame_digest);
    assert.equal(translated.event_basis_digest,decision.determination.event_basis_digest);
    assert.equal(translated.native_envelope.data.outcome,"completed");
    assert.equal(translated.native_envelope.data.requested_model,actualProvider.model_ref);
    assert.equal(translated.native_envelope.data.answer.model,actualProvider.model_ref);
    same(translated.response.provider,decision.determination.provider);
    same(translated.response.proposals,decision.determination.learned);
    same(decision.determination.validated.filter(f=>f.field==="faculty").map(f=>f.value),[learnedPlan.expected_faculty]);
    assert.equal(decision.determination.learned.length,1);
    same(decision.determination.learned[0].label_ids,learnedPlan.expected_faculty);
    assert.equal(decision.determination.refused_candidates.length,0);
    const {position,...unpositioned}=learnedPlan.invocation;
    const invoked=await callOnce("Read the source through ql_invoke exactly once. Fill the missing position from the "
      +"faculty validated in the previous native determination. If there is no validated faculty, keep the reading unresolved "
      +"and do not invoke. Use the following exact operation and input; preserve the real source receipt.\n"
      +JSON.stringify(unpositioned),"ql_invoke",learnedPlan.invocation);
    same(invoked,learnedPlan.expected_invocation_receipt);
    const sourceContinuationStart=trace.length;
    const continued=await turn("Using the actual source receipt, state the source reference and its exact revision in one sentence. Do not call tools.");
    assert(!trace.slice(sourceContinuationStart).some(row=>row.type.startsWith("tool_execution_")));
    assert(!continued.messages.some(m=>m.role==="assistant"&&(m.stopReason==="error"||m.stopReason==="aborted"
      ||m.content.some(c=>c.type==="toolCall"))));
    const text=continued.messages.filter(m=>m.role==="assistant").flatMap(m=>m.content)
      .filter(c=>c.type==="text").map(c=>c.text).join("\n");
    assert(text.includes(learnedPlan.invocation.input.reference));
    assert(text.includes(invoked.result.source.revision),"actual continuation must use the native source revision");
    await request("prompt",{message:"/ql-mode off"});
    const before=trace.length;const ordinary=await turn("Reply with the single word continued. Do not call tools.");
    const ordinaryText=ordinary.messages.filter(m=>m.role==="assistant").flatMap(m=>m.content)
      .filter(c=>c.type==="text").map(c=>c.text).join("\n").trim();
    assert.equal(ordinaryText,"continued");
    assert(!trace.slice(before).some(row=>row.type.startsWith("tool_execution_")));
    assert(!ordinary.messages.some(m=>m.role==="assistant"&&m.content.some(c=>c.type==="toolCall")));
    assert(!ordinary.messages.some(m=>m.role==="assistant"&&(m.stopReason==="error"||m.stopReason==="aborted")));
    process.stdout.write(JSON.stringify({body,scenario,actual_native_calls:2,decision_provider_calls:1,
      accepted_faculty:decision.determination.validated.find(f=>f.field==="faculty").value,
      operation:invoked.operation,source_revision:invoked.result.source.revision,
      actual_continuation_used_source:true,mode_off_ordinary_continuation:true})+"\n");
  }else if(validationPlan){
    const canonical=value=>Array.isArray(value)?value.map(canonical):value&&typeof value==="object"
      ?Object.fromEntries(Object.keys(value).sort().map(key=>[key,canonical(value[key])])):value;
    for(const entry of validationPlan.cases){
      const before=trace.length;
      await turn(`Call ${entry.tool} exactly once with the following exact JSON argument object. `
        +"Copy the object without adding, removing or moving any key. The argument object is already complete. "
        +"For validation, projection contains both event and requested_heads; projection.event contains only the event fields. "
        +"For projection or decide, requested_heads is beside event. Never place requested_heads inside an event. "
        +"This is a controlled admission counterfactual, "
        +"not model inference evidence. Retain refusal/stale/unavailable results without repairing or retrying. "
        +"After the call return one sentence.\n"+JSON.stringify(entry.request));
      // Prime can deliver a completed tool without a separate start event.
      // Its actual assistant tool-call message retains the typed arguments;
      // join that call ID to the real execution end, never to narrated prose.
      const calls=trace.slice(before).flatMap(row=>row.type==="tool_execution_start"
        ?[{name:row.toolName,arguments:row.args,id:row.toolCallId}]
        :row.type==="agent_end"?row.messages.filter(m=>m.role==="assistant")
          .flatMap(m=>m.content.filter(c=>c.type==="toolCall")):[]);
      const uniqueCalls=new Map();
      for(const raw of calls){
        const call={name:raw.name,arguments:raw.arguments,id:raw.id};
        const existing=uniqueCalls.get(call.id);
        if(existing) assert.deepEqual(canonical(call),canonical(existing),"native call ID changed its arguments");
        uniqueCalls.set(call.id,call);
      }
      assert.equal(uniqueCalls.size,1,`${body} must make exactly one call without retry: ${entry.id}`);
      const call=[...uniqueCalls.values()][0];
      assert(call.name===entry.tool&&JSON.stringify(canonical(call.arguments))===JSON.stringify(canonical(entry.request)),
        `${body} must invoke the exact native request: ${entry.id}`);
      const executionResults=trace.slice(before).filter(row=>row.type==="tool_execution_end");
      assert.equal(executionResults.length,1,`${body} must retain exactly one actual execution result: ${entry.id}`);
      const ended=executionResults[0];
      assert.equal(ended.toolCallId,call.id,"execution result differs from the actual native call ID");
      assert(ended&&!ended.isError,`${body} native admission/tool transport failed: ${entry.id}`);
      assert.deepEqual(ended.result.details.native_receipt,entry.expected_receipt,
        `${body} must retain the same native determination: ${entry.id}`);
    }
    const beforeOff=(await request("get_messages")).data.messages.filter(m=>m.customType==="ql-native-event-context").length;
    await request("prompt",{message:"/ql-mode off"});
    const ordinaryStart=trace.length;
    const ordinary=await turn("Reply with the single word continued. Do not call tools.");
    const ordinaryText=ordinary.messages.filter(m=>m.role==="assistant").flatMap(m=>m.content)
      .filter(c=>c.type==="text").map(c=>c.text).join("\n").trim();
    assert.equal(ordinaryText,"continued","mode-off must produce the requested actual ordinary reply");
    assert(!ordinary.messages.some(m=>m.role==="assistant"&&m.content.some(c=>c.type==="toolCall")));
    assert(!trace.slice(ordinaryStart).some(row=>row.type.startsWith("tool_execution_")),
      "ordinary mode-off continuation must not execute a tool");
    const after=(await request("get_messages")).data.messages;
    assert.equal(after.filter(m=>m.customType==="ql-native-event-context").length,beforeOff);
    assert(!after.some(m=>m.role==="assistant"&&m.stopReason==="error"));
    process.stdout.write(JSON.stringify({body,scenario,native_validation_cases:validationPlan.cases.length,
      decision_provider_calls:0,mode_off_ordinary_continuation:true})+"\n");
  }else{
  const stateInput="QL state: "+JSON.stringify({lens:"L2'","local-position":3,
    "coordinate-face":"direct","musical-basis":"chromatic"});
  const expectedRef=usesTool?event.event_ref:
    "ql:event:sha256:"+createHash("sha256").update(stateInput).digest("hex");
  await turn(usesTool?
    "Call ql_project_event once with this exact request, preserve its native result and return one sentence. "
      +"Do not calculate an alternative reading.\n"+JSON.stringify({event,requested_heads:["lens"]}):stateInput);
  const result=trace.find(row=>row.type==="tool_execution_end"&&row.toolName==="ql_project_event"&&!row.isError);
  if(usesTool) assert(result,"real native QL tool must execute successfully");
  else assert(!result,"input-hook proof must not rely on an acting QL tool call");
  const continued = await turn("Continue from the native QL receipt. State its event reference and harmonic pitch class in one sentence. Do not call tools.");
  const reply = continued.messages.filter(m=>m.role==="assistant").flatMap(m=>m.content)
    .filter(c=>c.type==="text").map(c=>c.text).join("\n");
  const messages=(await request("get_messages")).data.messages;
  const readings=messages.filter(m=>m.customType==="ql-native-event-context");
  assert(readings.length>0,"actual next-turn context must contain the owner receipt");
  assert.equal(readings.at(-1).details.body,body);
  const reading=readings.at(-1);
  const native=result?.result.details.native_receipt ?? JSON.parse(reading.content.slice(reading.content.indexOf("\n")+1));
  assert.equal(native.event.event_ref,expectedRef);
  assert.deepEqual(native.decision_head_ids,[]);
  const pitch=native.harmonic.harmonic.find(f=>f.field==="pitch-class").value;
  assert.equal(pitch,11);
  assert(reply.includes(expectedRef),"the acting agent must use the exact retained event reference");
  assert(new RegExp(`\\b${pitch}\\b`).test(reply),"the acting agent must use the native harmonic pitch");
  const beforeOff=(await request("get_messages")).data.messages;
  const count=beforeOff.filter(m=>m.customType==="ql-native-event-context").length;
  await request("prompt",{message:"/ql-mode off"});
  await turn("Reply with the single word continued. Do not call tools.");
  const after=(await request("get_messages")).data.messages;
  assert.equal(after.filter(m=>m.customType==="ql-native-event-context").length,count);
  assert(!after.some(m=>m.role==="assistant"&&m.stopReason==="error"));
  process.stdout.write(JSON.stringify({body,scenario,native_tool_success:!!result,continuation_readings:count,
    actual_reply_event_ref:expectedRef,actual_reply_pitch_class:pitch,
    decision_provider_calls:0,mode_off_ordinary_continuation:true,
    native_validation_cases:validationPlan?.cases.length??0})+"\n");
  }
}catch(error){failure=String(error);throw error;}
finally{
  clearTimeout(timer);
  try{
    await writeFile(outputFile,JSON.stringify({body,scenario,provider,model,failure:failure??null,trace,stderr},null,2)+"\n");
  }finally{
    if(!closed){const exited=new Promise(resolvePromise=>child.once("close",resolvePromise));
      try{process.kill(-child.pid,"SIGTERM");}catch{}
      const kill=setTimeout(()=>{try{process.kill(-child.pid,"SIGKILL");}catch{}},2500);
      await exited;clearTimeout(kill);}
  }
}
