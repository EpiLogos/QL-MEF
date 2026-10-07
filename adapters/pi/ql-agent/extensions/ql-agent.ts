import { spawn } from "node:child_process";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { Type } from "typebox";
import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";
import { registerQlEventContext } from "./ql-event-context.ts";

// Carry the exact QL-owned wire definitions. This extension has no lens,
// relation, operation, completion, modal or musical interpretation tables.
const schemaBytes = readFileSync(new URL("../ql-agent-contracts-v1.schema.json", import.meta.url));
const provenance = JSON.parse(readFileSync(new URL("../provenance.json", import.meta.url), "utf8"));
const schemaHash = createHash("sha256").update(schemaBytes).digest("hex");
if (schemaHash !== provenance.source_sha256["schemas/ql-agent-contracts-v1.schema.json"]) {
  throw new Error("QL tool contract differs from the retained owner source");
}
const definitions = JSON.parse(schemaBytes.toString("utf8")).$defs;
// Some provider tool-schema carriers omit $defs. Inline owner references
// mechanically so the acting model sees the actual object shape. The retained
// contract remains the source; this performs no QL interpretation or repair.
function inlineOwnerSchema(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(inlineOwnerSchema);
  if (!value || typeof value !== "object") return value;
  const record = value as Record<string, unknown>;
  if (typeof record.$ref === "string") {
    const prefix = "#/$defs/";
    const name = record.$ref.startsWith(prefix) ? record.$ref.slice(prefix.length) : "";
    if (!Object.hasOwn(definitions, name)) throw new Error("Unresolved QL owner schema reference");
    const siblings = Object.fromEntries(Object.entries(record).filter(([key]) => key !== "$ref"));
    return inlineOwnerSchema({ ...definitions[name], ...siblings });
  }
  return Object.fromEntries(Object.entries(record).map(([key, item]) => [key, inlineOwnerSchema(item)]));
}
type ProjectionRequest = { event: unknown; requested_heads?: string[] };
const projectionSchema = {
  type: "object",
  additionalProperties: false,
  required: ["event"],
  properties: {
    event: { $ref: "#/$defs/event" },
    requested_heads: {
      type: "array", items: { type: "string", minLength: 1 }, uniqueItems: true,
      description: "Request only relevant semantic heads; the QL owner validates their names and eligibility. Omit for deterministic-only projection.",
    },
  },
};
const projectionParameters = Type.Unsafe<ProjectionRequest>(inlineOwnerSchema(projectionSchema) as Record<string, unknown>);
const admissionParameters = Type.Unsafe<{ projection: ProjectionRequest; response: unknown }>(inlineOwnerSchema({
  type: "object", additionalProperties: false, required: ["projection", "response"],
  properties: { projection: projectionSchema, response: { type: "object" } },
}) as Record<string, unknown>);

type Operation = "project" | "frame" | "harmonic" | "validate" | "invoke" | "decide" | "encounter";
const receiptSchemas: Record<Operation, string> = {
  project: "ql.agent-projection/v1", frame: "ql.agent-decision-frame/v1",
  harmonic: "ql.harmonic-event/v1", validate: "ql.agent-decision-admission/v1",
  invoke: "ql.epi-logos-agent-invocation-result/v1",
  decide: "ql.agent-projection/v1",
  encounter: "ql.agent-encounter/v1",
};

async function nativeReceipt(operation: Operation, request: unknown, cwd: string, signal?: AbortSignal) {
  if (signal?.aborted) throw new Error("QL native invocation cancelled before launch");
  const input = Buffer.from(JSON.stringify(request), "utf8");
  if (input.length > 1024 * 1024) throw new Error("QL request exceeds the 1 MiB transport bound");
  const executable = operation === "encounter" ? process.env.QL_AGENT_ENCOUNTER_BIN || "ql-agent-encounter"
    : operation === "decide" ? process.env.QL_AGENT_DECIDE_BIN || "ql-agent-decide"
    : process.env.QL_AGENT_BIN || "ql-agent";
  const arguments_ = operation === "decide" || operation === "encounter" ? []
    : [operation === "invoke" ? "epi-agent" : "agent-event", operation, "-", "--json"];
  const output = await new Promise<Buffer>((resolve, reject) => {
    const child = spawn(executable, arguments_, {
      cwd, shell: false, stdio: ["pipe", "pipe", "pipe"],
    });
    let failure: Error | undefined;
    let stdoutBytes = 0;
    let stderrBytes = 0;
    let hardStop: ReturnType<typeof setTimeout> | undefined;
    const stdout: Buffer[] = [];
    const stderr: Buffer[] = [];
    const stop = (error: Error) => {
      if (failure) return;
      failure = error;
      child.kill("SIGTERM");
      // The shared carrier needs up to one second to reap a separately grouped
      // client after TERM. Preserve that opportunity before the outer KILL.
      hardStop = setTimeout(() => child.kill("SIGKILL"), 2500);
      hardStop.unref();
    };
    const abort = () => stop(new Error("QL native invocation cancelled"));
    const timeoutMs = operation === "decide" ? 190000 : 5000;
    const timeout = setTimeout(() => stop(new Error(`QL native invocation exceeded ${timeoutMs / 1000} seconds`)), timeoutMs);
    timeout.unref();
    signal?.addEventListener("abort", abort, { once: true });
    if (signal?.aborted) abort();
    const clean = () => {
      clearTimeout(timeout);
      if (hardStop) clearTimeout(hardStop);
      signal?.removeEventListener("abort", abort);
    };
    child.stdout.on("data", (bytes: Buffer) => {
      stdoutBytes += bytes.length;
      if (stdoutBytes > 8 * 1024 * 1024) stop(new Error("QL receipt exceeds the 8 MiB transport bound"));
      else stdout.push(bytes);
    });
    child.stderr.on("data", (bytes: Buffer) => {
      stderrBytes += bytes.length;
      if (stderrBytes > 256 * 1024) stop(new Error("QL diagnostic exceeds the transport bound"));
      else stderr.push(bytes);
    });
    child.stdin.on("error", (error: Error) => stop(error));
    child.on("error", (error: Error) => {
      failure ||= error;
    });
    child.on("close", (code, exitSignal) => {
      clean();
      if (failure) reject(failure);
      else if (code !== 0) reject(new Error(`QL native ${operation} refused (exit ${code}, signal ${exitSignal}): ${Buffer.concat(stderr).toString("utf8").trim()}`));
      else resolve(Buffer.concat(stdout));
    });
    child.stdin.end(input);
  });
  const text = new TextDecoder("utf-8", { fatal: true }).decode(output);
  const receipt = JSON.parse(text);
  if (!receipt || Array.isArray(receipt) || receipt.schema !== receiptSchemas[operation]) {
    throw new Error("QL executable returned an unexpected native receipt schema");
  }
  if (operation === "encounter" && (receipt.provider_calls !== 0
      || receipt.projector_revision !== "sha256:" + provenance.source_sha256["scripts/ql_agent_encounter.py"])) {
    throw new Error("QL encounter differs from its installed deterministic owner provenance");
  }
  if (operation !== "invoke" && (operation !== "encounter" || receipt.disposition === "projected")) {
    const basis = receipt.kernel_basis || receipt.frame?.kernel_basis || receipt.projection?.frame?.kernel_basis;
    if (basis?.digest !== provenance.kernel_digest) {
      throw new Error("QL native receipt differs from the installed kernel provenance");
    }
  }
  return {
    content: [{ type: "text" as const, text }],
    details: { native_receipt: receipt, executable, operation, input_bytes: input.length, output_bytes: output.length },
  };
}

export default function (pi: ExtensionAPI, body: "pi" | "prime" = "pi") {
  registerQlEventContext(pi, body, (request, cwd, signal) => nativeReceipt("encounter", request, cwd, signal));
  // Registration performs no CLI, provider or model call. These are explicit
  // native tools, so ordinary body entry and tool execution do not wait for ML.
  const readings = [
    ["ql_project_event", "Project QL event", "Project the exact sourced event, complete deterministic QL state and retain unresolved semantic heads.", "project"],
    ["ql_decision_frame", "Inspect QL decision frame", "Read the kernel-owned remaining heads, legal candidates, ambiguity policies and constraints.", "frame"],
    ["ql_harmonic_read", "Read QL harmonics", "Compute the musical rendering of the formal event with the actual QL owner.", "harmonic"],
    ["ql_decide", "Determine QL event", "Complete deterministic QL first, request only remaining semantic heads through an explicitly elected AIKit provider, and admit the answer with the native kernel. An unavailable provider remains unresolved. This explicit tool may take time; it never blocks body entry.", "decide"],
  ] as const;
  for (const [name, label, description, operation] of readings) {
    pi.registerTool({ name, label, description, parameters: projectionParameters,
      async execute(_id, request, signal, _update, ctx) {
        return nativeReceipt(operation, request, ctx.cwd, signal);
      },
    });
  }
  pi.registerTool({
    name: "ql_validate_determination", label: "Validate QL determination",
    description: "Admit an exact provider response against the original QL event and kernel; retain refusal, ambiguity, unavailability and stale results.",
    parameters: admissionParameters,
    async execute(_id, request, signal, _update, ctx) {
      return nativeReceipt("validate", request, ctx.cwd, signal);
    },
  });
  pi.registerTool({
    name: "ql_invoke", label: "Invoke native QL operation",
    description: "Invoke an exact current QL faculty operation with its required input. The native owner checks operation membership and inputs; inspect its receipts and unresolved requirements first.",
    parameters: Type.Object({
      schema: Type.Literal("ql.epi-logos-agent-invocation/v1"),
      position: Type.String({ pattern: "^#[0-5]$" }),
      operation: Type.String({ minLength: 1 }),
      input: Type.Record(Type.String(), Type.Unknown()),
    }, { additionalProperties: false }),
    async execute(_id, request, signal, _update, ctx) {
      return nativeReceipt("invoke", request, ctx.cwd, signal);
    },
  });
}
