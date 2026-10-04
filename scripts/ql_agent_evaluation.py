"""Bounded, reproducible evaluation through the actual QL event CLI.

The frozen JSONL suite contains independently sourced expectations. The optional
provider command receives only the native projection, never verifier answers.
It must return ql.agent-decision-response/v1; execution/election remain AIKit's.
This runner does not load weights, call a provider from a body hook, or train.
"""
import argparse
import hashlib
import json
import math
import os
import resource
import shutil
import signal
import subprocess
import sys
import time
from pathlib import Path

import ql_agent_contracts as c


def unique_object(pairs):
    result = {}
    for key, value in pairs:
        c.require(key not in result, "duplicate JSON key: " + key)
        result[key] = value
    return result


def decode(data):
    return json.loads(data, object_pairs_hook=unique_object,
                      parse_constant=lambda value: c.require(False, "non-finite JSON: " + value))


def load_suite(path, expected_digest):
    data = path.read_bytes()
    digest = "sha256:" + hashlib.sha256(data).hexdigest()
    c.require(digest == expected_digest, "frozen suite digest mismatch")
    cases, ids, families = [], set(), {}
    for line in data.decode("utf-8").splitlines():
        case = decode(line)
        c.require(isinstance(case, dict) and set(case) == {
            "schema", "id", "family", "split", "projection", "expected"}, "evaluation case fields")
        c.require(case["schema"] == "ql.agent-evaluation-case/v1", "evaluation case schema")
        for field in ("id", "family"):
            c.require(isinstance(case[field], str) and bool(case[field].strip()), "empty " + field)
        c.require(case["id"] not in ids, "duplicate case id")
        ids.add(case["id"])
        c.require(case["split"] in ("train", "validation", "test"), "unknown split")
        previous = families.setdefault(case["family"], case["split"])
        c.require(previous == case["split"], "structural/template family leaks across splits")
        request = case["projection"]
        c.require(isinstance(request, dict) and set(request) <= {"event", "requested_heads"}
                  and "event" in request, "projection request fields")
        c.validate(request["event"])
        expected = case["expected"]
        c.require(isinstance(expected, dict) and set(expected) == {
            "fields", "heads", "status", "basis_refs"}, "expected fields")
        c.require(isinstance(expected["fields"], dict), "expected formal fields")
        c.require(isinstance(expected["heads"], dict), "expected candidate sets")
        for head, labels in expected["heads"].items():
            c.require(isinstance(head, str) and bool(head) and isinstance(labels, list)
                      and all(isinstance(label, str) and bool(label) for label in labels)
                      and len(labels) == len(set(labels)), "expected candidate set")
        c.require(expected["fields"] or expected["heads"], "empty evaluation expectation")
        c.require(expected["status"] in ("determined", "partial", "unresolved", "refused", "unavailable", "stale"),
                  "expected status")
        c.require(isinstance(expected["basis_refs"], list) and bool(expected["basis_refs"])
                  and all(isinstance(ref, str) and bool(ref.strip()) for ref in expected["basis_refs"]),
                  "expectation requires independent kernel/reviewed source basis")
        cases.append(case)
    c.require(bool(cases), "empty suite")
    return cases, digest


def invoke(argv, document, timeout):
    started = time.monotonic()
    def interrupted(signum, _frame):
        raise SystemExit(128 + signum)

    handlers = {signum: signal.getsignal(signum) for signum in (signal.SIGTERM, signal.SIGINT)}
    for signum in handlers:
        signal.signal(signum, interrupted)
    process = None
    try:
        process = subprocess.Popen(argv, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                   stderr=subprocess.PIPE, start_new_session=True)
        stdout, stderr = process.communicate(input=c.canonical(document), timeout=timeout)
    except BaseException:
        if process is not None:
            try:
                os.killpg(process.pid, signal.SIGTERM)
            except ProcessLookupError:
                pass
            try:
                process.communicate(timeout=3)
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
    elapsed = time.monotonic() - started
    if process.returncode:
        raise RuntimeError(f"{argv[0]} exited {process.returncode}: " +
                           stderr.decode("utf-8", errors="replace")[:4096])
    return decode(stdout), elapsed


def check_projection(projection):
    c.require(projection.get("schema") == "ql.agent-projection/v1", "native projection schema")
    c.validate_frame(projection["event"], projection["frame"])
    c.validate_determination(projection["event"], projection["frame"], projection["determination"])
    c.validate_harmonic(projection["determination"], projection["harmonic"])


def provider_input(projection):
    # Explicit allow-list prevents a future research field from leaking gold
    # answers into provider state. Source/operation/kernel meaning stays native.
    return {key: projection[key] for key in ("schema", "event", "frame", "decision_head_ids")}


def check_expectation(projection, expected):
    c.require(set(expected["heads"]) == set(projection["decision_head_ids"]),
              "gold candidate heads must cover the exact native eligible field")
    heads = {head["id"]: head for head in projection["frame"]["unresolved"]}
    for head, labels in expected["heads"].items():
        c.require(set(labels) <= {label["id"] for label in heads[head]["labels"]},
                  "gold candidates must belong to the native legal field")


def operative_fields(determination):
    result = {}
    for standing in ("observed", "derived", "validated"):
        for fact in determination[standing]:
            if standing == "validated" and fact["origin"] == "observed":
                continue
            c.require(fact["field"] not in result, "duplicate operative field")
            result[fact["field"]] = fact["value"]
    return result


def rate(numerator, denominator):
    return numerator / denominator if denominator else None


def quantile(values, fraction):
    if not values:
        return None
    ordered = sorted(values)
    return ordered[max(0, math.ceil(fraction * len(ordered)) - 1)]


def metrics(records):
    exact = head_exact = head_count = ambiguity = ambiguity_count = 0
    unsupported = operative_unsupported = unsupported_count = abstention = abstention_count = 0
    tp = fp = fn = 0
    labels = {}
    operations = operation_count = bypass = bypass_count = 0
    before_refused = after_invalid = proposal_cases = 0
    admission_refused = stale = 0
    refinement_latency = []
    for record in records:
        expected, projection = record["expected"], record["projection"]
        determination = projection["determination"]
        admission_status = (record.get("admission") or {}).get("admission_status")
        invalid_response = admission_status in ("refused", "stale")
        admission_refused += record["provider_calls"] > 0 and admission_status == "refused"
        stale += record["provider_calls"] > 0 and admission_status == "stale"
        raw = (record.get("response") or {}).get("proposals", determination["learned"])
        fields = operative_fields(determination)
        heads = {head["id"]: head for head in projection["frame"]["unresolved"]}
        check_expectation(projection, expected)
        predictions = {head: set() for head in expected["heads"]}
        admitted = {fact["proposal_head"] for fact in determination["validated"]
                    if fact["origin"] == "learned"}
        for proposal in determination["learned"]:
            if proposal["head_id"] in admitted:
                predictions[proposal["head_id"]] = set(proposal["label_ids"])
        # A second requested head may be discharged by native deterministic
        # completion of an admitted head (operation -> faculty, for example).
        for head in predictions:
            field = heads[head]["field"]
            if field in fields and head not in admitted:
                value = fields[field]
                c.require(isinstance(value, str) or (isinstance(value, list) and
                          all(isinstance(label, str) for label in value)),
                          "semantic field lacks native label representation")
                predictions[head] = {value} if isinstance(value, str) else set(value)
        sets_exact = True
        for head, gold_labels in expected["heads"].items():
            gold, prediction = set(gold_labels), predictions[head]
            head_count += 1
            head_exact += prediction == gold
            sets_exact &= prediction == gold
            tp += len(gold & prediction)
            fp += len(prediction - gold)
            fn += len(gold - prediction)
            for label in gold | prediction:
                counts = labels.setdefault((heads[head]["field"], label), [0, 0, 0])
                counts[0] += label in gold and label in prediction
                counts[1] += label in prediction and label not in gold
                counts[2] += label in gold and label not in prediction
            if len(gold) > 1:
                ambiguity_count += 1
                ambiguity += gold <= prediction
            if not gold:
                abstention_count += 1
                abstention += not prediction and not invalid_response
                unsupported_count += 1
                unsupported += any(proposal["head_id"] == head and bool(proposal["label_ids"])
                                   for proposal in raw)
                operative_unsupported += bool(prediction)
            if heads[head]["field"] == "operation":
                operation_count += 1
                operations += gold == prediction
        response_is_expected = not invalid_response or expected["status"] == admission_status
        exact += (response_is_expected and sets_exact and determination["status"] == expected["status"] and
                  all(field in fields and fields[field] == value
                      for field, value in expected["fields"].items()))
        if not projection["decision_head_ids"]:
            bypass_count += 1
            bypass += record["provider_calls"] == 0
        if determination["learned"]:
            proposal_cases += 1
            before_refused += bool(determination["refused_candidates"])
            refused = {refusal["head_id"] for refusal in determination["refused_candidates"]}
            after_invalid += bool(admitted & refused)
        if record["provider_elapsed_seconds"] is not None:
            refinement_latency.append(record["provider_elapsed_seconds"])
    macro = [2 * a / (2 * a + b + d) for a, b, d in labels.values()]
    return {
        "cases": len(records), "exact_determination_accuracy": rate(exact, len(records)),
        "exact_candidate_set_accuracy": rate(head_exact, head_count),
        "micro_f1": rate(2 * tp, 2 * tp + fp + fn),
        "macro_f1": rate(sum(macro), len(macro)),
        "ambiguity_preservation": rate(ambiguity, ambiguity_count),
        "unsupported_certainty_rate": rate(unsupported, unsupported_count),
        "operative_unsupported_certainty_rate": rate(operative_unsupported, unsupported_count),
        "missing_evidence_abstention_rate": rate(abstention, abstention_count),
        "kernel_refusal_rate": rate(before_refused, proposal_cases),
        "kernel_refusal_scope": "cases with retained learned proposals in the determination",
        "admission_refusal_rate": rate(admission_refused, sum(record["provider_calls"] for record in records)),
        "stale_response_rate": rate(stale, sum(record["provider_calls"] for record in records)),
        "refused_result_operative_rate": rate(after_invalid, proposal_cases),
        "operation_routing_accuracy": rate(operations, operation_count),
        "deterministic_bypass_rate": rate(bypass, bypass_count),
        "provider_calls": sum(record["provider_calls"] for record in records),
        "provider_elapsed_p50_seconds": quantile(refinement_latency, .5),
        "provider_elapsed_p95_seconds": quantile(refinement_latency, .95),
        "native_input_bytes": sum(record["input_bytes"] for record in records),
        # The protocol currently has supplied confidence, not ranked
        # distributions or thermal/runtime state. Absence is not a zero score.
        "not_measured": ["top_k_coverage", "conjugate_invariance", "complement_invariance",
                         "reversal_invariance", "constraint_violation_before_validation",
                         "constraint_violation_after_validation", "end_to_end_tool_success",
                         "prime_pi_parity", "cold_latency", "warm_latency", "peak_vram", "tokens"],
    }


def run(args):
    cases, suite_digest = load_suite(args.suite, args.suite_digest)
    selected = [case for case in cases if case["split"] == args.split]
    c.require(bool(selected), "selected split has no cases")
    executable = shutil.which(args.ql)
    c.require(executable is not None, "actual QL executable unavailable")
    executable = Path(executable).resolve()
    binary_digest = "sha256:" + hashlib.sha256(executable.read_bytes()).hexdigest()
    sources = {"runner_digest": Path(__file__).resolve(), "contracts_digest": Path(c.__file__).resolve()}
    source_digests = {key: "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()
                      for key, path in sources.items()}
    args.output.mkdir(parents=True, exist_ok=False)
    args.owned_output = True
    records = []
    with (args.output / "cases.jsonl").open("xb") as evidence:
        for case in selected:
            args.failure_context = {"case_id": case["id"], "phase": "projection",
                                    "request": case["projection"], "suite_digest": suite_digest,
                                    "ql_executable_digest": binary_digest, **source_digests}
            projection, elapsed = invoke([str(executable), "agent-event", "project", "-", "--json"],
                                         case["projection"], args.timeout)
            args.failure_context["projection"] = projection
            check_projection(projection)
            c.require(projection["event"] == case["projection"]["event"], "changed native event basis")
            check_expectation(projection, case["expected"])
            response = admission = None
            provider_elapsed = None
            calls = 0
            if args.provider_command and projection["decision_head_ids"]:
                calls = 1
                args.failure_context["phase"] = "provider"
                response, provider_elapsed = invoke(args.provider_command, provider_input(projection), args.timeout)
                args.failure_context.update(phase="admission", response=response,
                                            provider_elapsed_seconds=provider_elapsed)
                admission, validation_elapsed = invoke(
                    [str(executable), "agent-event", "validate", "-", "--json"],
                    {"projection": case["projection"], "response": response}, args.timeout)
                projection = admission["projection"]
                args.failure_context["admission"] = admission
                elapsed += validation_elapsed
                check_projection(projection)
                c.require(projection["event"] == case["projection"]["event"], "changed admission event basis")
            record = {"id": case["id"], "family": case["family"], "split": case["split"],
                      "expected": case["expected"], "projection": projection,
                      "response": response, "admission": admission,
                      "provider_calls": calls, "provider_elapsed_seconds": provider_elapsed,
                      "ql_elapsed_seconds": elapsed, "input_bytes": len(c.canonical(case["projection"]))}
            evidence.write(c.canonical(record) + b"\n")
            evidence.flush()
            records.append(record)
    args.failure_context = {"phase": "final source/metrics verification", "suite_digest": suite_digest,
                            "ql_executable_digest": binary_digest, **source_digests}
    c.require("sha256:" + hashlib.sha256(executable.read_bytes()).hexdigest() == binary_digest,
              "QL executable changed during evaluation")
    c.require(all("sha256:" + hashlib.sha256(path.read_bytes()).hexdigest() == source_digests[key]
                  for key, path in sources.items()), "evaluation/contract source changed during run")
    rss = resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss
    report = {"schema": "ql.agent-evaluation/v1", "suite_digest": suite_digest,
              "split": args.split, "ql_executable_digest": binary_digest,
              **source_digests,
              "provider_program": args.provider_command[0] if args.provider_command else None,
              "provider_command_digest": c.digest(args.provider_command) if args.provider_command else None,
              "metrics": metrics(records),
              "resource": {"peak_child_rss_bytes": rss if sys.platform == "darwin" else rss * 1024,
                           "scope": "all child processes in this runner; not model-only RSS"},
              "standing": "actual CLI evaluation in the explicitly declared expected-field/head scope"}
    (args.output / "metrics.json").write_bytes(c.canonical(report) + b"\n")
    return report


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--suite", type=Path, required=True)
    parser.add_argument("--suite-digest", required=True)
    parser.add_argument("--split", choices=("train", "validation", "test"), default="test")
    parser.add_argument("--ql", default="ql")
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--timeout", type=float, default=120)
    parser.add_argument("--provider-command", nargs=argparse.REMAINDER)
    args = parser.parse_args()
    if not math.isfinite(args.timeout) or args.timeout <= 0:
        parser.error("timeout must be finite and positive")
    try:
        print(json.dumps(run(args)["metrics"]))
    except (c.ContractError, OSError, ValueError, KeyError, RuntimeError, subprocess.TimeoutExpired) as error:
        if getattr(args, "owned_output", False):
            (args.output / "failure.json").write_bytes(c.canonical({
                "schema": "ql.agent-evaluation-failure/v1", "error_type": type(error).__name__,
                "error": str(error), "context": args.failure_context,
                "standing": "failed execution; no completed benchmark or score"}) + b"\n")
        parser.exit(1, str(error) + "\n")


if __name__ == "__main__":
    main()
