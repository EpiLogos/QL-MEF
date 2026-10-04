"""Translate owner-issued QL heads through AIKit's existing decision execution.

This explicit research/tool command is never installed in a blocking body hook.
The native projector supplies every label; AIKit elects and executes the model;
the native QL validator alone admits the returned candidate sets. Each label is
a Noul question so the first transport can retain ambiguity without inventing a
Choice distribution. Thresholds are explicit experimental parameters.
"""
import argparse
import hashlib
import json
import math
import os
import shutil
import signal
import subprocess
import sys
import time
import uuid
from pathlib import Path

import ql_agent_contracts as c


MAX_BYTES = 1024 * 1024


def unique_object(pairs):
    result = {}
    for key, value in pairs:
        c.require(key not in result, "duplicate JSON key: " + key)
        result[key] = value
    return result


def decode(data):
    c.require(len(data) <= 8 * MAX_BYTES, "AIKit receipt exceeds bounded envelope")
    return json.loads(data, object_pairs_hook=unique_object,
                      parse_constant=lambda value: c.require(False, "non-finite JSON: " + value))


def eligible_heads(projection):
    c.require(isinstance(projection, dict) and set(projection) == {
        "schema", "event", "frame", "decision_head_ids"}, "provider input allow-list")
    c.require(projection["schema"] == "ql.agent-projection/v1", "native projection schema")
    c.validate(projection["event"])
    c.validate_frame(projection["event"], projection["frame"])
    ids = projection["decision_head_ids"]
    c.require(isinstance(ids, list) and all(isinstance(i, str) for i in ids)
              and len(ids) == len(set(ids)), "eligible head identity")
    heads = {head["id"]: head for head in projection["frame"]["unresolved"]}
    c.require(set(ids) <= set(heads), "eligible head absent from native frame")
    # Input-required dispositions never acquire model questions.
    c.require(all(not i.startswith("input-") for i in ids),
              "input-required head cannot be classified")
    return [heads[i] for i in ids]


def translate_request(projection, model):
    c.require(isinstance(model, str) and bool(model.strip()), "explicit model required")
    heads = eligible_heads(projection)
    questions, bindings = {}, {}
    for head_index, head in enumerate(heads):
        for label_index, label in enumerate(head["labels"]):
            question_id = f"ql-{head_index}-{label_index}"
            bindings[question_id] = (head["id"], label["id"])
            questions[question_id] = {
                "type": "noul",
                "instructions": {
                    "question": "Does the supplied semantic material support this native candidate? "
                                "Missing evidence is false. Multiple candidates may be supported.",
                    "field": head["field"],
                    "ambiguity_policy": head["ambiguity_policy"],
                    "candidate": label,
                },
                "criteria": {
                    "true": {"candidate": label, "meaning": "supported by supplied semantic material"},
                    "false": {"candidate": label["id"], "meaning": "not supported, or evidence insufficient"},
                },
            }
    c.require(len(questions) <= 256, "native eligible field exceeds packed AIKit question limit")
    request = {
        "model": model,
        "state": {"event": projection["event"],
                  "determined": projection["frame"]["determined"],
                  "constraints": projection["frame"]["constraints"],
                  "kernel_basis": projection["frame"]["kernel_basis"]},
        "questions": questions,
    }
    c.require(len(c.canonical(request)) <= MAX_BYTES, "packed AIKit request exceeds native byte limit")
    return request, bindings


def response_basis(projection):
    return {"schema": "ql.agent-decision-response/v1",
            "event_basis_digest": projection["frame"]["event_basis_digest"],
            "frame_digest": c.digest(projection["frame"]),
            "kernel_basis": projection["frame"]["kernel_basis"],
            "proposals": []}


def unavailable(projection, reason):
    result = response_basis(projection)
    result.update(outcome="unavailable", reason=reason)
    return result


def translate_answer(projection, bindings, receipt, threshold, model_revision, runtime_revision):
    c.require(isinstance(threshold, (int, float)) and not isinstance(threshold, bool)
              and math.isfinite(threshold) and 0 < threshold <= 1, "explicit threshold in (0,1]")
    c.require(isinstance(receipt, dict), "AIKit invocation receipt must be an object")
    c.require(receipt.get("outcome") in ("completed", "failed", "cancelled"),
              "AIKit invocation outcome malformed")
    if receipt["outcome"] != "completed":
        failure = receipt.get("failure")
        c.require(failure is None or isinstance(failure, dict), "AIKit failure must be an object")
        reason = (failure or {}).get("message") or "AIKit decision did not complete"
        c.require(isinstance(reason, str), "AIKit failure message must be text")
        return unavailable(projection, reason)
    answer = receipt.get("answer")
    c.require(isinstance(answer, dict), "AIKit answer must be an object")
    answers = answer.get("answers")
    c.require(isinstance(answers, dict) and set(answers) == set(bindings),
              "AIKit answer must cover exactly the packed native candidates")
    model = answer.get("model")
    invocation = receipt.get("invocation_ref")
    c.require(all(isinstance(value, str) and bool(value.strip())
                  for value in (model, invocation, model_revision, runtime_revision)),
              "returned model and exact provider/runtime revisions required")
    candidates = {head["id"]: [] for head in eligible_heads(projection)}
    for question_id, (head_id, label_id) in bindings.items():
        entry = answers[question_id]
        c.require(isinstance(entry, dict) and set(entry) == {"type", "noul"}
                  and entry["type"] == "noul", "expected actual Noul evidence")
        score = entry["noul"]
        c.require(isinstance(score, (int, float)) and not isinstance(score, bool)
                  and math.isfinite(score) and 0 <= score <= 1, "invalid provider Noul score")
        if score >= threshold:
            candidates[head_id].append((label_id, score))
    result = response_basis(projection)
    result.update(outcome="answered", provider={
        "provider_ref": invocation, "model_ref": model,
        "model_revision": model_revision, "runtime_revision": runtime_revision})
    for head_id, selected in candidates.items():
        proposal = {"head_id": head_id, "label_ids": [label for label, _ in selected], "spans": []}
        # Several independent scores are not a joint probability. Their exact
        # per-label values remain in the retained native invocation receipt.
        if len(selected) == 1:
            proposal["confidence"] = selected[0][1]
        result["proposals"].append(proposal)
    return result


def file_digest(path):
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(MAX_BYTES), b""):
            h.update(chunk)
    return "sha256:" + h.hexdigest()


def run_owned(argv, timeout):
    """Bounded native command; cancellation reaps only this invocation group."""
    def interrupted(signum, _frame):
        raise SystemExit(128 + signum)

    handlers = {signum: signal.getsignal(signum) for signum in (signal.SIGTERM, signal.SIGINT)}
    for signum in handlers:
        signal.signal(signum, interrupted)
    process = None
    try:
        process = subprocess.Popen(argv, stdin=subprocess.DEVNULL, stdout=subprocess.PIPE,
                                   stderr=subprocess.PIPE, start_new_session=True)
        stdout, stderr = process.communicate(timeout=timeout)
    except BaseException:
        if process is not None:
            try:
                os.killpg(process.pid, signal.SIGTERM)
            except ProcessLookupError:
                pass
            try:
                process.communicate(timeout=1)
            except subprocess.TimeoutExpired:
                try:
                    os.killpg(process.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                process.communicate()
        raise
    finally:
        for signum, handler in handlers.items():
            signal.signal(signum, handler)
    c.require(len(stdout) <= 8 * MAX_BYTES and len(stderr) <= MAX_BYTES, "bounded AIKit output")
    return process.returncode, stdout, stderr


def execute(args, projection):
    heads = eligible_heads(projection)
    if not heads:
        return unavailable(projection, "deterministic bypass; no decision provider invoked")
    try:
        config = decode(args.provider_file.read_bytes())
        c.require(isinstance(config, dict) and config.get("schema") == "aikit.decision-provider/v1",
                  "AIKit provider config schema")
        c.require(config.get("mode") in ("none", "managed-local", "endpoint", "hosted"),
                  "AIKit provider mode")
        if config["mode"] == "none":
            return unavailable(projection, "AIKit decision provider disabled")
        limits_key = "jev_limits" if config["mode"] == "hosted" else "limits"
        limits = config.get(limits_key)
        c.require(isinstance(limits, dict), "AIKit provider requires native limits")
        if config["mode"] == "hosted":
            tariff = limits.get("tariff")
            c.require(isinstance(tariff, dict), "AIKit hosted provider requires native tariff")
            model = tariff.get("model_version")
        else:
            model = limits.get("model")
        request, bindings = translate_request(projection, model)
    except (OSError, ValueError, TypeError, c.ContractError) as error:
        return unavailable(projection, "AIKit provider configuration unavailable: " + str(error))
    executable = shutil.which(args.aikit)
    if executable is None:
        return unavailable(projection, "AIKit executable unavailable")
    executable = Path(executable).resolve()
    args.receipt_dir.mkdir(parents=True, exist_ok=True)
    stem = "ql-aikit-" + uuid.uuid4().hex
    request_path = args.receipt_dir / (stem + ".request.json")
    provider_path = args.receipt_dir / (stem + ".provider.json")
    receipt_path = args.receipt_dir / (stem + ".receipt.json")
    record = {"schema": "ql.aikit-translation-receipt/v1",
              "event_basis_digest": projection["frame"]["event_basis_digest"],
              "frame_digest": c.digest(projection["frame"]),
              "request_digest": c.digest(request), "bindings": bindings,
              "provider_config_digest": c.digest(config), "threshold": args.threshold,
              "model_revision": args.model_revision,
              "aikit_executable_digest": file_digest(executable),
              "adapter_digest": file_digest(Path(__file__)),
              "contract_digest": file_digest(Path(c.__file__))}
    runtime_revision = c.digest({key: record[key] for key in (
        "aikit_executable_digest", "adapter_digest", "contract_digest", "provider_config_digest")})
    record["runtime_revision"] = runtime_revision
    with request_path.open("xb") as stream:
        stream.write(c.canonical(request))
    # The execution owner reads the exact elected snapshot whose digest is
    # attributed here, even if the source file changes during the invocation.
    with provider_path.open("xb") as stream:
        stream.write(c.canonical(config))
    started = time.monotonic()
    try:
        code, stdout, stderr = run_owned([
            str(executable), "--json", "decide", "invoke", "--provider-file",
            str(provider_path.resolve()), "--request-file", str(request_path.resolve())], args.timeout)
        record.update(exit_code=code, elapsed_seconds=time.monotonic() - started,
                      stderr=stderr.decode("utf-8", errors="replace"),
                      stdout_digest="sha256:" + hashlib.sha256(stdout).hexdigest())
        envelope = decode(stdout)
        record["native_envelope"] = envelope
        c.require(isinstance(envelope, dict), "AIKit command envelope must be an object")
        if code or envelope.get("ok") is not True:
            response = unavailable(projection, "AIKit invocation failed; retained native receipt " + str(receipt_path))
        else:
            response = translate_answer(projection, bindings, envelope["data"], args.threshold,
                                        args.model_revision, runtime_revision)
    except (SystemExit, KeyboardInterrupt):
        record.update(elapsed_seconds=time.monotonic() - started,
                      failure="caller interrupted; owned client process group reaped",
                      effect_uncertain=True)
        with receipt_path.open("xb") as stream:
            stream.write(c.canonical(record) + b"\n")
        raise
    except (OSError, ValueError, KeyError, TypeError, c.ContractError, subprocess.TimeoutExpired) as error:
        record.update(elapsed_seconds=time.monotonic() - started, failure=str(error))
        response = unavailable(projection, "AIKit invocation unavailable: " + str(error))
    record["response"] = response
    with receipt_path.open("xb") as stream:
        stream.write(c.canonical(record) + b"\n")
    return response


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--provider-file", type=Path, required=True)
    parser.add_argument("--receipt-dir", type=Path, required=True)
    parser.add_argument("--model-revision", required=True,
                        help="Exact installed model/material revision, independently pinned before invocation")
    parser.add_argument("--threshold", type=float, required=True)
    parser.add_argument("--aikit", default="aikit")
    parser.add_argument("--timeout", type=float, default=125)
    args = parser.parse_args()
    c.require(math.isfinite(args.timeout) and 0 < args.timeout <= 3600, "bounded native invocation timeout")
    c.require(math.isfinite(args.threshold) and 0 < args.threshold <= 1, "explicit threshold in (0,1]")
    c.require(bool(args.model_revision.strip()), "exact model revision required")
    data = sys.stdin.buffer.read(MAX_BYTES + 1)
    c.require(len(data) <= MAX_BYTES, "projection input exceeds native byte limit")
    response = execute(args, decode(data))
    sys.stdout.buffer.write(c.canonical(response) + b"\n")


if __name__ == "__main__":
    main()
