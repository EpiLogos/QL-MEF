"""Explicit QL decision instrument shared by agent bodies.

All projection, candidate meaning, admission and harmonics execute in the
native QL owner. This adapter composes those operations with AIKit execution.
It has no implicit provider election and never runs during body entry.
"""
import argparse
import json
import math
import os
import shutil
import sys
from pathlib import Path

import ql_agent_aikit as a
import ql_agent_contracts as c


def native(ql, operation, request):
    executable = shutil.which(ql)
    c.require(executable is not None, "native QL executable unavailable")
    code, stdout, stderr = a.run_owned(
        [executable, "agent-event", operation, "-", "--json"], 5,
        c.canonical(request))
    c.require(code == 0, "native QL refused: " + stderr.decode("utf-8", errors="replace")[:4096])
    return a.decode(stdout)


def validate_execution(args):
    c.require(isinstance(args.threshold, (int, float)) and not isinstance(args.threshold, bool)
              and math.isfinite(args.threshold) and 0 < args.threshold <= 1,
              "explicit threshold in (0,1]")
    c.require(isinstance(args.model_revision, str) and bool(args.model_revision.strip()),
              "exact model revision required")


def decide(args, request):
    projection = native(args.ql, "project", request)
    c.require(projection.get("schema") == "ql.agent-projection/v1", "native projection schema")
    # This return precedes provider configuration, executable discovery, receipt
    # allocation and inference. Ordinary deterministic QL needs none of them.
    if not projection["decision_head_ids"]:
        return projection
    configuration = getattr(args, "configuration", None)
    if configuration:
        config = a.decode(Path(configuration).read_bytes())
        c.require(isinstance(config, dict) and config.get("schema") == "ql.decision-execution/v1",
                  "explicit decision execution configuration schema")
        allowed = {"schema", "provider_file", "receipt_dir", "model_revision", "threshold", "schema_profile", "timeout"}
        c.require(set(config) <= allowed, "decision execution configuration fields")
        for key in allowed - {"schema"}:
            if key in config:
                setattr(args, key, Path(config[key]) if key in ("provider_file", "receipt_dir") else config[key])
        c.require(isinstance(args.timeout, (int, float)) and not isinstance(args.timeout, bool)
                  and 0 < args.timeout <= 180, "explicit bounded execution timeout required")
    basis = {key: projection[key] for key in ("schema", "event", "frame", "decision_head_ids")}
    if args.provider_file is None:
        response = a.unavailable(basis, "no decision provider elected; ordinary QL remains available")
    elif args.receipt_dir is None or args.threshold is None or args.model_revision is None:
        response = a.unavailable(basis, "explicit threshold, model revision and retained receipt directory required")
    else:
        validate_execution(args)
        response = a.execute(args, basis)
    admitted = native(args.ql, "validate", {"projection": request, "response": response})
    c.require(admitted.get("schema") == "ql.agent-decision-admission/v1", "native admission schema")
    return admitted if getattr(args, "admission", False) else admitted["projection"]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ql", default="ql-agent")
    parser.add_argument("--aikit", default="aikit")
    parser.add_argument("--provider-file", type=Path)
    parser.add_argument("--receipt-dir", type=Path)
    parser.add_argument("--model-revision")
    parser.add_argument("--threshold", type=float)
    parser.add_argument("--schema-profile", choices=a.SCHEMA_PROFILES, default="compact")
    parser.add_argument("--timeout", type=float, default=120)
    parser.add_argument("--configuration", default=os.environ.get("QL_AGENT_DECISION_CONFIG"))
    parser.add_argument("--admission", action="store_true",
                        help="Return the native admission, including the raw response, for a body to re-admit with its own bound owner")
    args = parser.parse_args()
    c.require(0 < args.timeout <= 180, "explicit bounded execution timeout required")
    raw = sys.stdin.buffer.read(a.MAX_BYTES + 1)
    c.require(len(raw) <= a.MAX_BYTES, "projection input exceeds native byte limit")
    request = a.decode(raw)
    c.require(len(c.canonical(request)) <= a.MAX_BYTES, "bounded QL request")
    print(json.dumps(decide(args, request), ensure_ascii=False, separators=(",", ":")))


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError, KeyError, TypeError, c.ContractError) as error:
        print("QL decision instrument refused: " + str(error), file=sys.stderr)
        raise SystemExit(2)
