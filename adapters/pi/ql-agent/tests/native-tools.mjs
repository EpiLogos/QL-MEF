import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdir, readFile, realpath } from "node:fs/promises";
import { resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { pathToFileURL } from "node:url";

const [packageArgument, workingArgument, sdkArgument, eventPath, mode = "native"] = process.argv.slice(2);
assert(packageArgument && workingArgument && sdkArgument && eventPath);
const [packageRoot, workingDir, sdkRoot] = [packageArgument, workingArgument, sdkArgument].map(p => resolve(p));
await mkdir(workingDir, {recursive:true});
assert(["native", "unavailable-command"].includes(mode));
const sdk = await import(pathToFileURL(resolve(sdkRoot, "dist/index.js")).href);
const { validateToolArguments } = await import(pathToFileURL(resolve(sdkRoot,
  "node_modules/@earendil-works/pi-ai/dist/utils/validation.js")).href);
const sdkPackage = JSON.parse(await readFile(resolve(sdkRoot, "package.json"), "utf8"));
assert.equal(sdkPackage.name, "@earendil-works/pi-coding-agent");
assert.equal(sdkPackage.version, "0.84.4");
const packageManifest = JSON.parse(await readFile(resolve(packageRoot, "package.json"), "utf8"));
assert.equal(packageManifest.pi.extensions.length, 1, "one exact owner extension must be contributed");
const event = JSON.parse(await readFile(eventPath, "utf8"));
event.observed = [
  { field: "lens", value: "L2'", origin: "observed", basis_refs: [event.material.ref] },
  { field: "local-position", value: 3, origin: "observed", basis_refs: [event.material.ref] },
  { field: "coordinate-face", value: "direct", origin: "observed", basis_refs: [event.material.ref] },
  { field: "musical-basis", value: "chromatic", origin: "observed", basis_refs: [event.material.ref] },
];
const request = { event, requested_heads: ["lens"] };
const names = ["ql_project_event", "ql_decision_frame", "ql_harmonic_read", "ql_validate_determination", "ql_decide", "ql_invoke"];
const loader = new sdk.DefaultResourceLoader({
  cwd: workingDir, agentDir: resolve(workingDir, "agent"), additionalExtensionPaths: [packageRoot],
  noExtensions: true, noSkills: true, noPromptTemplates: true, noThemes: true, noContextFiles: true,
});
await loader.reload();
assert.deepEqual(loader.getExtensions().errors, []);
const { session } = await sdk.createAgentSession({
  cwd: workingDir, agentDir: resolve(workingDir, "agent"), resourceLoader: loader,
  sessionManager: sdk.SessionManager.inMemory(workingDir), tools: names,
});
const executable = process.env.QL_AGENT_BIN || "ql-agent";
const native = (operation, value) => {
  const result = spawnSync(executable, [operation === "invoke" ? "epi-agent" : "agent-event", operation, "-", "--json"], {
    cwd: workingDir, input: JSON.stringify(value), encoding: "utf8", timeout: 5000, maxBuffer: 8 * 1024 * 1024,
  });
  assert.ifError(result.error);
  assert.equal(result.status, 0, result.stderr);
  return JSON.parse(result.stdout);
};
const canonical = value => {
  if (Array.isArray(value)) return value.map(canonical);
  if (value && typeof value === "object") return Object.fromEntries(Object.keys(value).sort().map(k => [k, canonical(value[k])]));
  return value;
};
const digest = value => "sha256:" + createHash("sha256").update(JSON.stringify(canonical(value))).digest("hex");
const checks = [];
try {
  await session.bindExtensions({ mode: "headless" });
  await session.prompt("/ql-mode on");
  const lastCustom = name => session.sessionManager.getEntries()
    .filter(entry => entry.type === "custom" && entry.customType === name).at(-1)?.data;
  assert.equal(lastCustom("ql-body-mode").enabled, true);
  assert.equal(lastCustom("ql-body-mode").implicit_provider_calls, 0);
  checks.push("ql-mode:real-native-command-enables-without-inference");
  const module = await realpath(resolve(packageRoot, packageManifest.pi.extensions[0]));
  for (const name of names) {
    const registered = session.getAllTools().find(t => t.name === name);
    assert(registered, `${name} must be discovered by native Pi`);
    assert.equal(await realpath(registered.sourceInfo.path), module);
  }
  const invoke = (name, value, signal = new AbortController().signal) => {
    const tool = session.agent.state.tools.find(t => t.name === name);
    assert(tool, `${name} must be active`);
    const validated = validateToolArguments(tool, {type:"toolCall",id:`native-${name}`,name,arguments:value});
    return tool.execute(`native-${name}`, validated, signal);
  };
  if (mode === "unavailable-command") {
    for (const name of names.slice(0, 4)) {
      const input = name === "ql_validate_determination" ? { projection: request, response: {} } : request;
      await assert.rejects(invoke(name, input), /unknown command [`']agent-event[`']/);
      checks.push(`${name}:actual-installed-command-refused`);
    }
  } else {
    for (const [name, operation] of [[names[0], "project"], [names[1], "frame"], [names[2], "harmonic"]]) {
      const expected = native(operation, request);
      const result = await invoke(name, request);
      assert.deepEqual(result.details.native_receipt, expected);
      assert.deepEqual(JSON.parse(result.content[0].text), expected);
      if (operation === "project") {
        assert.deepEqual(expected.decision_head_ids, []);
        assert(!Object.hasOwn(expected.determination, "provider"));
      }
      checks.push(`${name}:exact-native-owner-receipt`);
    }
    const semantic = structuredClone(request);
    semantic.event.observed = [];
    const projected = native("project", semantic);
    assert(projected.decision_head_ids.length > 0, "unavailable admission requires a genuinely unresolved native head");
    const response = {
      schema: "ql.agent-decision-response/v1", event_basis_digest: projected.frame.event_basis_digest,
      frame_digest: digest(projected.frame), kernel_basis: projected.frame.kernel_basis,
      outcome: "unavailable", reason: "controlled unavailable response; no service call", proposals: [],
    };
    const admission = { projection: semantic, response };
    const expected = native("validate", admission);
    const result = await invoke(names[3], admission);
    assert.deepEqual(result.details.native_receipt, expected);
    assert.equal(expected.admission_status, "unavailable");
    assert.equal(expected.projection.determination.status, "unavailable");
    assert.equal(result.details.native_receipt.admission_status, "unavailable");
    assert.equal(result.details.native_receipt.projection.determination.status, "unavailable");
    assert.deepEqual(expected.response, response);
    assert.deepEqual(expected.projection.determination.validated, []);
    checks.push("ql_validate_determination:exact-native-unavailable-admission");
    const invalid = structuredClone(request);
    invalid.event.observed[0].value = "L99";
    await assert.rejects(invoke(names[0], invalid), /QL native project refused/);
    checks.push("impossible-observed-lens:kernel-refused");
    const determined = await invoke("ql_decide", request);
    assert.deepEqual(determined.details.native_receipt, native("project", request));
    checks.push("ql_decide:deterministic-bypass-exact-owner-receipt");
    const noProvider = await invoke("ql_decide", semantic);
    assert.equal(noProvider.details.native_receipt.determination.status, "unavailable");
    assert.deepEqual(noProvider.details.native_receipt.determination.validated, []);
    checks.push("ql_decide:no-provider-remains-unavailable");
    const invocation = {
      schema: "ql.epi-logos-agent-invocation/v1", position: "#1", operation: "tda.vietoris-rips",
      input: { metric: "precomputed", complex: "vietoris-rips", coefficients: 2, max_homology_dimension: 1,
        max_scale: 2, distances: [[0, 1, 1], [1, 0, 1], [1, 1, 0]], source_basis: { source_ref: event.material.ref } },
    };
    const invoked = await invoke("ql_invoke", invocation);
    assert.deepEqual(invoked.details.native_receipt, native("invoke", invocation));
    checks.push("ql_invoke:actual-native-persistent-homology");
    await assert.rejects(invoke("ql_invoke", { ...invocation, position: "#2" }), /not admitted/);
    checks.push("ql_invoke:wrong-faculty-refused-by-owner");
  }
  await session.prompt("/ql-status");
  const beforeOff = lastCustom("ql-body-status");
  assert.equal(beforeOff.body, "pi");
  assert.equal(beforeOff.implicit_provider_calls, 0);
  // These direct SDK tool calls test the instruments, not an acting-agent
  // event stream. The separate native-event-loop gate exercises that stream.
  assert.equal(beforeOff.native_tool_results, 0);
  await session.prompt("/ql-mode off");
  if (mode === "native") {
    const result = await invoke("ql_project_event", request);
    assert.deepEqual(result.details.native_receipt, native("project", request));
  }
  await session.prompt("/ql-status");
  const afterOff = lastCustom("ql-body-status");
  assert.equal(afterOff.enabled, false);
  assert.equal(afterOff.event_ref, null);
  assert.equal(afterOff.native_tool_results, beforeOff.native_tool_results);
  checks.push("ql-mode:real-native-command-disables-and-preserves-native-tools");
  const cancelled = new AbortController();
  cancelled.abort();
  await assert.rejects(invoke(names[0], request, cancelled.signal), /cancelled before launch/);
  checks.push("pre-cancelled:refused-before-launch");
  const oversized = structuredClone(request);
  oversized.event.material.text = "x".repeat(1024 * 1024);
  await assert.rejects(invoke(names[0], oversized), /1 MiB transport bound/);
  checks.push("oversized:refused-before-launch");
  process.stdout.write(JSON.stringify({
    schema: "ql.pi-native-tools-proof/v1", sdk: sdkPackage.name, version: sdkPackage.version,
    module, executable, mode, checks, model_calls: 0,
    native_owner_success: mode === "native",
  }) + "\n");
} finally { session.dispose(); }
