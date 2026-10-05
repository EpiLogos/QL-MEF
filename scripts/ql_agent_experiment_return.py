"""Compare frozen QL evaluations and return the experiment through native M5.

This is an evaluation front door, not a model registry or provider election.
Unmeasured gates remain unmeasured. Native Epii receives the numerical result;
its Return does not mutate canonical ground or install a default.
"""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess

import ql_agent_contracts as c
import ql_agent_evaluation as evaluation


def qualified_metrics(path):
    data = path.read_bytes()
    c.require(len(data) <= 1024 * 1024, "bounded evaluation receipt required")
    document = json.loads(data)
    c.require(document.get("schema") == "ql.agent-evaluation/v1", "completed native evaluation required")
    records = path.parent / "cases.jsonl"
    rows = [json.loads(row) for row in records.read_text().splitlines()]
    c.require(len(rows) == document["metrics"]["cases"] and bool(rows), "exact retained case closure required")
    c.require(document["metrics"] == evaluation.metrics(rows),
              "published metrics differ from the retained actual cases")
    c.require(all(row["projection"]["frame"]["kernel_basis"] == rows[0]["projection"]["frame"]["kernel_basis"]
                  for row in rows), "evaluation mixed native kernels")
    return document, rows[0]["projection"]["frame"]["kernel_basis"], {
        "path": str(path.resolve()), "sha256": "sha256:" + hashlib.sha256(data).hexdigest(),
        "cases_sha256": "sha256:" + hashlib.sha256(records.read_bytes()).hexdigest()}


def run(args):
    baseline, kernel, baseline_basis = qualified_metrics(args.baseline)
    candidate, candidate_kernel, candidate_basis = qualified_metrics(args.candidate)
    regression, regression_kernel, regression_basis = qualified_metrics(args.regression)
    for key in ("suite_digest", "split", "ql_executable_digest"):
        c.require(baseline[key] == candidate[key], "incomparable evaluations: " + key)
    c.require(kernel == candidate_kernel == regression_kernel, "comparison requires the same native kernel")
    before, after, deterministic = baseline["metrics"], candidate["metrics"], regression["metrics"]
    primary_before, primary_after = before[args.primary], after[args.primary]
    c.require(type(primary_before) in (int, float) and type(primary_after) in (int, float),
              "measured numerical primary metric required")
    # None is never silently converted to a passing zero. These are measured
    # gates in the retained suite's scope, not a universal acceptance claim.
    def measured_gate(metrics, key, target):
        value = metrics.get(key)
        return None if value is None else value == target
    gates = {
        "primary_improved": primary_after > primary_before,
        "refused_result_never_operative": measured_gate(after,"refused_result_operative_rate",0),
        "missing_evidence_abstention": measured_gate(after,"missing_evidence_abstention_rate",1),
        "ambiguity_preserved": measured_gate(after,"ambiguity_preservation",1),
        "unsupported_certainty_absent": measured_gate(after,"operative_unsupported_certainty_rate",0),
        "deterministic_bypass": deterministic.get("deterministic_bypass_rate") == 1
            and deterministic.get("provider_calls") == 0
            and deterministic.get("exact_determination_accuracy") == 1,
        "all_semantic_cases_completed": after.get("provider_completed_cases") == after["cases"],
    }
    result = {"schema": "ql.agent-experiment-comparison/v1", "primary": args.primary,
        "baseline_value": primary_before, "candidate_value": primary_after,
        "delta": primary_after - primary_before, "gates": gates,
        "measured_gate_scope_passed": all(value is True for value in gates.values()), "kernel_basis": kernel,
        "source_basis": {"baseline": baseline_basis, "candidate": candidate_basis,
                         "deterministic_regression": regression_basis},
        "unmeasured_candidate_metrics": after.get("not_measured", []),
        "fresh_candidate_uptake_proved": False, "default_election": "not attempted"}
    invocation = {"schema": "ql.epi-logos-agent-invocation/v1", "position": "#5", "operation": "logos.return",
        "input": {"inquiry_ref": "https://github.com/EpiLogos/QL-MEF/issues/291",
            "T": "Improve held-out QL discrimination while retaining executable QL as authority.",
            "C": c.canonical({"primary": args.primary, "baseline": primary_before}).decode(),
            "T_prime": c.canonical({"candidate": primary_after, "delta": result["delta"], "gates": gates}).decode(),
            "C_prime": "Numerical research Return. No default election or fresh candidate uptake is claimed.",
            "source_refs": ["https://github.com/EpiLogos/QL-MEF/issues/291", kernel["owner_ref"] + "@" + kernel["revision"]],
            "evidence_refs": [basis["path"] + "@" + basis["sha256"]
                              for basis in result["source_basis"].values()]}}
    executable = shutil.which(args.ql)
    c.require(executable is not None, "native QL executable unavailable")
    c.require("sha256:" + hashlib.sha256(Path(executable).resolve().read_bytes()).hexdigest()
              == candidate["ql_executable_digest"], "M5 Return must use the evaluated QL executable")
    native = subprocess.run([executable, "epi-agent", "invoke", "-", "--json"],
        input=c.canonical(invocation), capture_output=True, timeout=5, check=True)
    c.require(len(native.stdout) <= 1024 * 1024, "bounded native Epii Return required")
    result["native_return"] = json.loads(native.stdout)
    c.require(result["native_return"].get("schema") == "ql.epi-logos-agent-invocation-result/v1",
              "unexpected native M5 Return receipt")
    args.output.parent.mkdir(parents=True, exist_ok=True)
    with args.output.open("xb") as stream:
        stream.write(c.canonical(result) + b"\n")
    print(json.dumps({key: result[key] for key in ("primary", "baseline_value", "candidate_value", "delta", "gates", "default_election")}))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("baseline", "candidate", "regression", "output"):
        parser.add_argument("--" + name, type=Path, required=True)
    parser.add_argument("--primary", choices=("exact_determination_accuracy", "exact_candidate_set_accuracy"),
                        default="exact_determination_accuracy")
    parser.add_argument("--ql", default="ql-agent")
    run(parser.parse_args())


if __name__ == "__main__":
    main()
