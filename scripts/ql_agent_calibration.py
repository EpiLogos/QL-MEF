"""Compare thresholds on retained validation inference through native QL and M5.

Only validation cases may be used. Every point reconstructs the actual provider
distribution and replays native admission; it makes no new classifier call.
Unknown gates prevent a recommendation. Test/fresh-body comparison and default
election remain separate acts, never consequences of this research command.
"""
import argparse
import contextlib
import copy
import io
import json
import math
from pathlib import Path
import shutil
import subprocess

import ql_agent_aikit as adapter
import ql_agent_contracts as c
import ql_agent_evaluation as evaluation
import ql_agent_experiment_return as experiment
import ql_agent_receipt_metrics as evidence


def run(args):
    baseline, kernel, baseline_basis = experiment.qualified_metrics(args.metrics)
    c.require(baseline["split"] == "validation", "calibration requires validation, never held-out test cases")
    thresholds = args.thresholds
    c.require(2 <= len(thresholds) <= 9 and len(set(thresholds)) == len(thresholds)
              and all(type(value) in (int, float) and math.isfinite(value) and 0 < value <= 1
                      for value in thresholds), "two to nine distinct finite thresholds in (0,1] required")
    cases, suite_digest = evaluation.load_suite(args.suite, args.suite_digest)
    selected = [case for case in cases if case["split"] == "validation"]
    c.require(suite_digest == baseline["suite_digest"], "calibration suite differs from evaluated suite")
    records = [evaluation.decode(line) for line in (args.metrics.parent / "cases.jsonl").read_text().splitlines()]
    c.require(len(selected) == len(records) and all(
        row["split"] == "validation" and row["id"] == case["id"] and row["family"] == case["family"]
        and row["expected"] == case["expected"]
        and row["projection"]["event"] == case["projection"]["event"]
        for row, case in zip(records, selected)), "validation expectations differ from frozen source")
    c.require(baseline["metrics"]["provider_completed_cases"] > 0
              and baseline["metrics"]["provider_unavailable_cases"] == 0
              and baseline["metrics"]["provider_cancelled_cases"] == 0,
              "completed validation inference required; service failures are not calibration evidence")
    qualified = evidence.collect(args.metrics, args.receipts)
    c.require(all(len(qualified[key]) == 1 for key in ("model_refs", "model_revisions", "runtime_revisions")),
              "calibration requires one exact model and execution revision")
    discovered_ql = shutil.which(args.ql)
    c.require(discovered_ql is not None, "calibration must use the evaluated native QL executable")
    ql = str(Path(discovered_ql).resolve())
    c.require(evidence.file_digest(Path(ql)) == baseline["ql_executable_digest"],
              "calibration must use the evaluated native QL executable")
    # Qualify the regression before allocating output or invoking the owner.
    regression, regression_kernel, _ = experiment.qualified_metrics(args.regression)
    c.require(regression_kernel == kernel and regression["ql_executable_digest"] == baseline["ql_executable_digest"],
              "deterministic regression requires the same native kernel and executable")
    regression_rows = [evaluation.decode(line)
        for line in (args.regression.parent / "cases.jsonl").read_text().splitlines()]
    c.require(regression["metrics"]["provider_calls"] == 0
              and regression["metrics"]["deterministic_bypass_rate"] == 1
              and regression["metrics"]["exact_determination_accuracy"] == 1
              and all(row["provider_calls"] == 0 and row["expected"]["heads"] == {}
                      and row["projection"]["decision_head_ids"] == []
                      and row["projection"]["determination"]["learned"] == []
                      and row["projection"]["determination"].get("provider") is None
                      and not any(fact["origin"] == "learned"
                                  for fact in row["projection"]["determination"]["validated"])
                      and row.get("response") is None and row.get("admission") is None
                      for row in regression_rows),
              "regression must be purely deterministic; learned or semantic test evidence cannot enter calibration")
    sources = {"path": str(Path(__file__).resolve()), "digest": evidence.file_digest(Path(__file__))}
    closure = qualified["evidence"]
    receipts = {}
    for path in sorted(args.receipts.glob("*.receipt.json")):
        receipt = evidence.read(path)
        receipts[receipt["event_basis_digest"], receipt["frame_digest"]] = receipt
    implementation = [{"path": str(Path(module.__file__).resolve()),
                       "digest": evidence.file_digest(Path(module.__file__))}
                      for module in (adapter, c, evaluation, experiment, evidence)]
    frozen = [{"path": str(args.suite.resolve()), "digest": suite_digest}, sources, *implementation,
              {"path": ql, "digest": baseline["ql_executable_digest"]},
              *closure, {"path": str(args.regression.resolve()), "digest": evidence.file_digest(args.regression)},
              {"path": str((args.regression.parent / "cases.jsonl").resolve()),
               "digest": evidence.file_digest(args.regression.parent / "cases.jsonl")}]
    args.output.mkdir(parents=True, exist_ok=False)
    args.owned_output = True
    points = []
    for index, threshold in enumerate(thresholds):
        args.failure_context = {"threshold": threshold, "phase": "native admission replay"}
        output = args.output / f"point-{index:02d}"
        output.mkdir()
        replayed = []
        with (output / "cases.jsonl").open("xb") as stream:
            for original, case in zip(records, selected):
                row = copy.deepcopy(original)
                if row["provider_calls"]:
                    key = row["projection"]["frame"]["event_basis_digest"], c.digest(row["projection"]["frame"])
                    receipt = receipts[key]
                    response = adapter.translate_answer(evaluation.provider_input(row["projection"]),
                        receipt["bindings"], receipt["native_envelope"]["data"], threshold,
                        receipt["model_revision"], receipt["runtime_revision"])
                    admission, elapsed = evaluation.invoke(
                        [ql, "agent-event", "validate", "-", "--json"],
                        {"projection": case["projection"], "response": response}, 5)
                    evaluation.check_projection(admission["projection"])
                    row.update(response=response, admission=admission, projection=admission["projection"],
                               ql_elapsed_seconds=elapsed)
                row["replay"] = {"threshold": threshold, "classifier_calls": 0,
                                 "original_metrics_digest": baseline_basis["sha256"]}
                stream.write(c.canonical(row) + b"\n")
                stream.flush()
                replayed.append(row)
        report = {**baseline, "metrics": evaluation.metrics(replayed),
                  "runner_digest": sources["digest"],
                  "resource": {"scope": "original inference resource figures are not replay measurements"},
                  "replay": {"threshold": threshold, "classifier_calls": 0,
                             "original_basis": baseline_basis},
                  "standing": "Actual validation inference, reconstructed selection and native admission replay; no new inference"}
        (output / "metrics.json").write_bytes(c.canonical(report) + b"\n")
        returned = output / "m5-return.json"
        with contextlib.redirect_stdout(io.StringIO()):
            experiment.run(argparse.Namespace(baseline=args.metrics, candidate=output / "metrics.json",
                regression=args.regression, primary=args.primary, ql=ql, output=returned))
        comparison = evidence.read(returned)
        points.append({"threshold": threshold, "primary_value": report["metrics"][args.primary],
                       "gates": comparison["gates"],
                       "measured_gate_scope_passed": comparison["measured_gate_scope_passed"],
                       "metrics": str((output / "metrics.json").resolve()),
                       "native_m5_return": str(returned.resolve())})
    args.failure_context = {"phase": "source closure verification"}
    c.require(all(evidence.file_digest(Path(item["path"])) == item["digest"] for item in frozen),
              "calibration source or original inference evidence changed during replay")
    eligible = [point for point in points if point["measured_gate_scope_passed"]]
    best = max(eligible, key=lambda point: point["primary_value"], default=None)
    result = {"schema": "ql.agent-validation-calibration/v1", "primary": args.primary,
              "points": points, "recommended_validation_threshold": best["threshold"] if best else None,
              "recommendation_scope": "validation only; all required gates must be measured and pass",
              "classifier_calls": 0, "held_out_semantic_test_used": False,
              "deterministic_regression_split": regression["split"],
              "split_policy": "Validation inference only; the fixed deterministic regression has no semantic heads or learned evidence",
              "default_election": "not attempted", "source_basis": frozen,
              "kernel_basis": kernel, "source": sources}
    (args.output / "calibration.json").write_bytes(c.canonical(result) + b"\n")
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("metrics", "receipts", "suite", "regression", "output"):
        parser.add_argument("--" + name, type=Path, required=True)
    parser.add_argument("--suite-digest", required=True)
    parser.add_argument("--thresholds", type=float, nargs="+", required=True)
    parser.add_argument("--primary", choices=("exact_determination_accuracy", "exact_candidate_set_accuracy"),
                        default="exact_determination_accuracy")
    parser.add_argument("--ql", default="ql-agent")
    args = parser.parse_args()
    try:
        result = run(args)
        print(json.dumps({key: result[key] for key in (
            "primary", "points", "recommended_validation_threshold", "classifier_calls", "held_out_semantic_test_used", "default_election")}))
    except (c.ContractError, OSError, ValueError, KeyError, TypeError, RuntimeError, subprocess.SubprocessError) as error:
        if getattr(args, "owned_output", False):
            (args.output / "failure.json").write_bytes(c.canonical({
                "schema": "ql.agent-calibration-failure/v1", "error": str(error),
                "context": getattr(args, "failure_context", {}), "standing": "failed replay; no recommendation"}) + b"\n")
        parser.exit(1, str(error) + "\n")


if __name__ == "__main__":
    main()
