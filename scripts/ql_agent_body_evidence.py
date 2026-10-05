"""Replay real fresh-body learned source uptake without invoking a model.

Both traces must retain actual calls, their execution results, one joined AIKit
invocation, source uptake and ordinary mode-off continuation. QL admission and
the source read are replayed through the native owner. Per-invocation audit
identity remains distinct from the compared QL values.
"""
import argparse
import json
import math
from pathlib import Path

import ql_agent_contracts as c
import ql_agent_evaluation as e
import ql_agent_receipt_metrics as metrics


def read(path):
    data = Path(path).read_bytes()
    c.require(len(data) <= 64 * 1024 * 1024, "bounded retained body evidence required")
    return json.loads(data)


def equal(left, right):
    """JSON value equality, retaining booleans while permitting 1.0 → 1 transport."""
    if type(left) in (int, float) and type(right) in (int, float):
        return math.isfinite(left) and math.isfinite(right) and left == right
    if type(left) is not type(right):
        return False
    if isinstance(left, dict):
        return left.keys() == right.keys() and all(equal(value, right[key]) for key, value in left.items())
    if isinstance(left, list):
        return len(left) == len(right) and all(equal(a, b) for a, b in zip(left, right))
    return left == right


def text(end):
    assistants = [message for message in end["messages"] if message["role"] == "assistant"]
    c.require(assistants and all(message.get("stopReason") not in ("error", "aborted") for message in assistants),
              "actual successful assistant continuation required")
    c.require(not any(item["type"] == "toolCall" for message in assistants for item in message["content"]),
              "continuation introduced another tool call")
    return "\n".join(item["text"] for message in assistants for item in message["content"]
                     if item["type"] == "text").strip()


def qualify(trace_path, config_path, body, plan, ql):
    document = read(trace_path)
    c.require(document.get("body") == body and document.get("scenario") == "learned"
              and document.get("failure") is None, "successful actual learned body trace required")
    trace, calls, ends, turns = document["trace"], {}, [], []
    for row in trace:
        entries = []
        if row["type"] == "tool_execution_start":
            entries = [{"id": row["toolCallId"], "name": row["toolName"], "arguments": row["args"]}]
        elif row["type"] == "agent_end":
            turns.append(row)
            entries = [item for message in row["messages"] if message["role"] == "assistant"
                       for item in message["content"] if item["type"] == "toolCall"]
        elif row["type"] == "tool_execution_end":
            ends.append(row)
        for entry in entries:
            call = {key: entry[key] for key in ("id", "name", "arguments")}
            c.require(isinstance(call["id"], str) and call["id"], "actual call identity required")
            c.require(call["id"] not in calls or equal(calls[call["id"]], call), "call identity changed arguments")
            calls[call["id"]] = call
    c.require(len(calls) == len(ends) == 2 and len(turns) == 4,
              "exactly two native calls and four actor turns required; retries are not parity")
    c.require(all(message.get("stopReason") not in ("error", "aborted") for turn in turns
                  for message in turn["messages"] if message["role"] == "assistant"),
              "actual actor turn failed or aborted")
    c.require(len({row["toolCallId"] for row in ends}) == 2, "duplicate native execution result")
    native = []
    for end, name, request in zip(ends, ("ql_decide", "ql_invoke"), (plan["request"], plan["invocation"])):
        c.require(end.get("isError") is False, "native tool execution failed")
        call = calls.get(end["toolCallId"])
        c.require(call is not None and call["name"] == end["toolName"] == name
                  and equal(call["arguments"], request), "actual typed request differs from shared plan")
        native.append(end["result"]["details"]["native_receipt"])
    decision, source = native
    c.require(equal(decision["event"], plan["expected_event"]), "native event basis differs")
    c.require(decision["determination"]["status"] == "determined", "learned determination not accepted")
    provider = decision["determination"]["provider"]
    c.require({k:v for k,v in provider.items() if k != "provider_ref"}
              == {k:v for k,v in plan["expected_provider"].items() if k != "provider_ref"},
              "different learned model/runtime")
    c.require([fact["value"] for fact in decision["determination"]["validated"] if fact["field"] == "faculty"]
              == [plan["expected_faculty"]], "learned faculty differs from reviewed expectation")
    configuration = read(config_path)
    paths = sorted(Path(configuration["receipt_dir"]).glob("*.receipt.json"))
    c.require(len(paths) == 1, "one isolated retained provider receipt required")
    path = paths[0]
    receipt = read(path)
    request_path = path.with_name(path.name.replace(".receipt.json", ".request.json"))
    provider_path = path.with_name(path.name.replace(".receipt.json", ".provider.json"))
    elected = metrics.qualify_request(decision, receipt, read(request_path), read(provider_path))
    invocation = receipt["native_envelope"]["data"]
    c.require(receipt.get("exit_code") == 0 and receipt["native_envelope"].get("ok") is True
              and invocation["outcome"] == "completed" and invocation["invocation_ref"] == provider["provider_ref"]
              and invocation["requested_model"] == invocation["answer"]["model"] == elected,
              "fresh native inference custody differs from acting determination")
    c.require(len(invocation["attempts"]) == 1 and invocation["attempts"][0]["http_status"] == 200
              and invocation["attempts"][0]["effect_uncertain"] is False, "one completed provider attempt required")
    reconstructed = metrics.adapter.translate_answer(e.provider_input(decision), receipt["bindings"], invocation,
        receipt["threshold"], receipt["model_revision"], receipt["runtime_revision"])
    c.require(reconstructed == receipt["response"], "retained native evidence differs from translation")
    admitted, _ = e.invoke([ql, "agent-event", "validate", "-", "--json"],
                           {"projection": plan["request"], "response": reconstructed}, 5)
    c.require(equal(admitted["projection"], decision), "acting receipt differs from replayed native admission")
    c.require(plan["invocation"]["operation"] == "anuttara.read", "bounded source-reading replay only")
    actual_source, _ = e.invoke([ql, "epi-agent", "invoke", "-", "--json"], plan["invocation"], 5)
    c.require(equal(source, actual_source) and equal(source, plan["expected_invocation_receipt"]),
              "acting source receipt differs from the actual native owner")
    continuation = text(turns[2])
    c.require(plan["invocation"]["input"]["reference"] in continuation
              and source["result"]["source"]["revision"] in continuation,
              "actual continuation did not use native source reference/revision")
    c.require(text(turns[3]) == "continued", "ordinary mode-off continuation missing")
    c.require(not any(row["type"].startswith("tool_execution_") for row in trace[trace.index(turns[1]) + 1:]),
              "source or mode-off continuation executed a tool")
    return {"body": body, "decision": decision, "source": source,
            "provider_invocation": provider["provider_ref"], "native_calls": 2, "provider_attempts": 1,
            "actual_continuation_used_source": True, "mode_off_continuation": True,
            "evidence": [{"path": str(Path(p).resolve()), "digest": metrics.file_digest(Path(p))}
                         for p in (trace_path, config_path, path, request_path, provider_path)]}


def compare(prime, pi):
    left, right = prime["decision"], pi["decision"]
    c.require(prime["provider_invocation"] != pi["provider_invocation"], "fresh bodies share one invocation receipt")
    c.require(equal(left["event"], right["event"]) and equal(left["frame"], right["frame"]),
              "body-specific semantic event or candidate field")
    for key in ("observed", "derived", "learned", "validated", "unresolved", "refused_candidates", "constraint_application"):
        c.require(equal(left["determination"][key], right["determination"][key]), "body determination differs: " + key)
    for key in ("event_ref", "source_basis", "kernel_basis", "formal", "harmonic"):
        c.require(equal(left["harmonic"][key], right["harmonic"][key]), "body harmonic reading differs: " + key)
    c.require(equal(prime["source"], pi["source"]), "body native source result differs")
    return {"schema": "ql.fresh-trained-body-parity/v1", "parity": True, "bodies": [prime, pi],
            "scope": "One controlled faculty concern followed by a real source read; no general model acceptance",
            "identity_scope": "Event/frame, accepted QL values and computed harmonic values match. Each exact harmonic determination digest retains its distinct inference audit identity.",
            "default_election": "not attempted"}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("prime_trace", "prime_config", "pi_trace", "pi_config", "plan", "output"):
        parser.add_argument("--" + name.replace("_", "-"), type=Path, required=True)
    parser.add_argument("--ql", default="ql-agent")
    args = parser.parse_args()
    plan = read(args.plan)
    c.require(plan["schema"] == "ql.body-learned-uptake-plan/v1", "learned uptake plan required")
    prime = qualify(args.prime_trace, args.prime_config, "prime", plan, args.ql)
    pi = qualify(args.pi_trace, args.pi_config, "pi", plan, args.ql)
    result = compare(prime, pi)
    result["source_digest"] = metrics.file_digest(Path(__file__))
    result["plan_digest"] = metrics.file_digest(args.plan)
    with args.output.open("xb") as stream:
        stream.write(c.canonical(result) + b"\n")
    print(json.dumps({"parity": result["parity"], "native_calls_per_body": 2, "provider_attempts_per_body": 1}))


if __name__ == "__main__":
    main()
