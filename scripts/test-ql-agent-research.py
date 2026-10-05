#!/usr/bin/env python3
"""Exercise the actual native research pipeline with an actual AIKit adapter.

This gate generates data and controlled native validation plans. It makes no
model call and claims neither trained quality nor acting-body acceptance.
Outputs, including failed training-pack attempts, remain in the supplied field.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import signal
import shutil
import subprocess
import sys
import time

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]


def digest(path):
    return "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()


def read(path):
    return json.loads(path.read_text())


def rows(path):
    return [json.loads(line) for line in path.read_text().splitlines()]


def run_program(name, argv, cwd, env, output, receipt, timeout, expected_failure=None):
    """Retain every actual attempt and allow native owners to reap descendants."""
    started = time.monotonic()
    process = None
    stdout, stderr = b"", b""
    failure = None
    handlers = {sig: signal.getsignal(sig) for sig in (signal.SIGTERM, signal.SIGINT)}

    def interrupted(sig, _frame):
        raise SystemExit(128 + sig)

    for sig in handlers:
        signal.signal(sig, interrupted)
    try:
        process = subprocess.Popen(argv, cwd=cwd, env=env, stdout=subprocess.PIPE,
                                   stderr=subprocess.PIPE, start_new_session=True)
        stdout, stderr = process.communicate(timeout=timeout)
    except BaseException as error:
        failure = error
        if process is not None:
            try:
                os.killpg(process.pid, signal.SIGTERM)
            except ProcessLookupError:
                pass
            try:
                stdout, stderr = process.communicate(timeout=3)
            except subprocess.TimeoutExpired:
                try:
                    os.killpg(process.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                stdout, stderr = process.communicate()
    finally:
        for sig, handler in handlers.items():
            signal.signal(sig, handler)
        (output / (name + ".stdout")).write_bytes(stdout)
        (output / (name + ".stderr")).write_bytes(stderr)
        receipt["commands"].append({"name": name, "arguments": list(map(str, argv)),
            "exit_code": process.returncode if process is not None else None,
            "seconds": time.monotonic() - started,
            "outcome": "timed-out" if isinstance(failure, subprocess.TimeoutExpired)
                       else "interrupted" if isinstance(failure, SystemExit)
                       else "failed" if failure is not None else "exited",
            "failure": None if failure is None else type(failure).__name__ + ": " + str(failure),
            "stdout_sha256": digest(output / (name + ".stdout")),
            "stderr_sha256": digest(output / (name + ".stderr"))})
    if failure is not None:
        raise failure
    if expected_failure is None:
        if process.returncode != 0:
            raise ValueError(name + " failed: " + stderr.decode(errors="replace")[:4096])
    elif process.returncode == 0 or expected_failure not in stderr.decode(errors="replace"):
        raise ValueError(name + " did not fail at the required native boundary")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ql", required=True)
    parser.add_argument("--provider-adapter", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    executable = shutil.which(args.ql)
    if executable is None:
        raise ValueError("actual native QL executable required")
    ql = Path(executable).resolve(strict=True)
    adapter = args.provider_adapter.resolve(strict=True)
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    sources = [ql, adapter, Path(__file__), *sorted((ROOT / "scripts").glob("ql_agent*.py")),
               ROOT / "fixtures/agent-decision/v1/event-v1.json",
               ROOT / "fixtures/kernel/ql-shape-contract-v1.json",
               *sorted((ROOT / "fixtures/agent-decision/v1").glob("semantic-*.json")),
               *sorted((ROOT / "docs/integrations/epi-logos").glob("epi-m-capability-field-m*.json"))]
    closure = {str(path): digest(path) for path in sources}
    receipt = {"schema": "ql.native-research-pipeline-check/v1", "status": "running",
               "source_digests": closure, "commands": [], "provider_calls": 0,
               "standing": "actual native CLI/data/dispatch gate; controlled responses are not model evidence"}
    started = time.monotonic()

    def run(name, arguments, *, expected_failure=None):
        env = os.environ.copy()
        env.pop("QL_AGENT_DECISION_CONFIG", None)
        env["PYTHONDONTWRITEBYTECODE"] = "1"
        run_program(name, [sys.executable, *map(str, arguments)], ROOT, env, output,
                    receipt, 120, expected_failure)

    try:
        semantic = output / "semantic"
        run("reviewed-corpus", ["scripts/ql_agent_semantic_corpus.py", "--ql", ql,
            "--output", semantic])
        frozen = read(semantic / "manifest.json")
        if frozen["cases"] != 9 or frozen["provider_calls"] != 0:
            raise ValueError("frozen reviewed corpus changed its case/call boundary")
        training = output / "training"
        run("reviewed-training", ["scripts/ql_agent_semantic_training.py", "--ql", ql,
            "--negative-review", "fixtures/agent-decision/v1/semantic-hard-negatives.json",
            "--output", training])
        train_manifest = read(training / "manifest.json")
        suite = training / "native/suite.jsonl"
        native_rows = rows(suite)
        frozen_rows = rows(semantic / "suite.jsonl")
        frozen_families = {row["family"] for row in frozen_rows}
        if any(row["split"] == "test" or row["family"] in frozen_families for row in native_rows):
            raise ValueError("frozen test family entered generated training")
        families = {}
        for row in native_rows:
            previous = families.setdefault(row["family"], row["split"])
            if previous != row["split"]:
                raise ValueError("training/validation template family leaked")
        pack = output / "pack"
        packing = ["scripts/ql_agent_gliner_data.py", "--ql", ql,
            "--provider-adapter", adapter, "--model", "gliner2.5-decide",
            "--schema-profile", "semantic-packed", "--label-rendering", "criteria"]
        run("native-training-pack", [*packing, "--suite", suite, "--suite-digest",
            train_manifest["suite_digest"], "--output", pack])
        packed = read(pack / "manifest.json")
        if packed["held_out_test_included"] or packed["provider_calls"] != 0:
            raise ValueError("training pack claimed test or inference")
        for split in ("train", "validation"):
            examples = rows(pack / (split + ".jsonl"))
            source_rows = [row for row in native_rows if row["split"] == split]
            if len(examples) != len(source_rows) or len(examples) != train_manifest["counts"][split]:
                raise ValueError("packing lost source rows")
            for example, source in zip(examples, source_rows):
                classes = example["output"]["classifications"]
                gold = source["expected"]["heads"]["semantic-faculty"]
                target = gold or ["__ql_provider_abstain__"]
                if len(classes) != 1 or classes[0]["true_label"] != target:
                    raise ValueError("packing changed the reviewed target/abstention")
                if "expected_fields" in example["input"] or "basis_refs" in example["input"]:
                    raise ValueError("verifier representation entered classifier material")
        rejected = output / "refused-test-pack"
        run("frozen-test-refused", [*packing, "--suite", semantic / "suite.jsonl",
            "--suite-digest", frozen["suite_digest"], "--output", rejected],
            expected_failure="held-out test cannot enter the training pack")
        if (rejected / "manifest.json").exists():
            raise ValueError("failed training pack published a successful manifest")
        grammar = output / "semantic-grammar"
        run("reviewed-grammar-corpus", ["scripts/ql_agent_semantic_corpus.py", "--ql", ql,
            "--reviewed", "fixtures/agent-decision/v1/semantic-grammar-reviewed.json",
            "--output", grammar])
        grammar_manifest = read(grammar / "manifest.json")
        grammar_rows = rows(grammar / "suite.jsonl")
        if grammar_manifest["cases"] != 40 or any(row["split"] != "test" for row in grammar_rows):
            raise ValueError("additional reviewed grammar lost its frozen test boundary")
        rejected_grammar = output / "refused-grammar-test-pack"
        run("grammar-test-refused", [*packing, "--suite", grammar / "suite.jsonl",
            "--suite-digest", grammar_manifest["suite_digest"], "--output", rejected_grammar],
            expected_failure="held-out test cannot enter the training pack")
        if (rejected_grammar / "manifest.json").exists():
            raise ValueError("grammar test refusal published a successful training manifest")
        formal = output / "formal"
        run("native-formal-corpus", ["scripts/ql_agent_corpus.py", "--ql", ql,
            "--output", formal])
        formal_manifest = read(formal / "manifest.json")
        regression = output / "regression"
        run("research-dispatch", ["scripts/ql_agent_research.py", "evaluate",
            "--ql", ql, "--suite", formal / "suite.jsonl", "--suite-digest",
            formal_manifest["suite_digest"], "--split", "test", "--output", regression])
        metrics = read(regression / "metrics.json")["metrics"]
        if metrics["exact_determination_accuracy"] != 1 or metrics["provider_calls"] != 0:
            raise ValueError("native research dispatcher lost deterministic bypass/results")
        disabled = output / "disabled.provider.json"
        disabled.write_text('{"schema":"aikit.decision-provider/v1","mode":"none"}\n')
        # This executable runs the production decision command against the
        # supplied native binary and an actual explicit disabled provider.
        decide = output / "decision-disabled"
        decide.write_text("#!" + sys.executable + "\nimport os\n"
            "os.execv(" + repr(sys.executable) + ", " + repr([sys.executable,
            str(ROOT / "scripts/ql_agent_decide.py"), "--ql", str(ql),
            "--provider-file", str(disabled)]) + ")\n")
        decide.chmod(0o700)
        plan = output / "body-plan.json"
        run("native-body-plan", ["scripts/ql_agent_body_plan.py", "--ql", ql,
            "--decide", decide, "--native-corpus", formal / "suite.jsonl",
            "--corpus-digest", formal_manifest["suite_digest"], "--output", plan])
        cases = read(plan)["cases"]
        if len(cases) != 9 or len({row["id"] for row in cases}) != 9:
            raise ValueError("body plan lost original nine actual native calls")
        by_id = {row["id"]: row for row in cases}
        if by_id["native-operation"]["expected_receipt"]["operation"] != "tda.vietoris-rips":
            raise ValueError("native body-plan invocation failed to retain exact operation")
        if any(digest(Path(path)) != revision for path, revision in closure.items()):
            raise ValueError("source/executable changed during pipeline gate")
        receipt.update(status="passed", reviewed_cases=frozen["cases"],
            reviewed_grammar_cases=grammar_manifest["cases"], grammar_test_refused=True,
            training_counts=train_manifest["counts"], frozen_test_refused=True,
            deterministic_test_accuracy=metrics["exact_determination_accuracy"], native_plan_cases=len(cases))
    except BaseException as error:
        receipt.update(status="failed", failure=type(error).__name__ + ": " + str(error))
        raise
    finally:
        receipt["seconds"] = time.monotonic() - started
        (output / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print(json.dumps({key: value for key, value in receipt.items()
                      if key not in ("source_digests", "commands")}))


if __name__ == "__main__":
    main()
