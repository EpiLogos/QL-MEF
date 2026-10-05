"""Generate bounded Prime/Pi tool regressions through the actual QL owner.

Controlled response inputs test native refusal and stale admission. They are
identified as counterfactual inputs, never model inference. No QL relation or
harmonic table is duplicated here. Both bodies receive identical requests.
"""
import argparse
import copy
import hashlib
import json
import os
from pathlib import Path

import ql_agent_contracts as c
import ql_agent_corpus as corpus
import ql_agent_evaluation as evaluation

ROOT = Path(__file__).resolve().parents[1]


def generate(args):
    c.require(not os.environ.get("QL_AGENT_DECISION_CONFIG"),
              "negative body-plan generation requires an explicitly unelected decision provider")
    rows, corpus_digest = evaluation.load_suite(args.native_corpus, args.corpus_digest)
    event = json.loads(args.event.read_text())
    event.pop("bindings", None)
    event["observed"] = []
    semantic = {"event": event, "requested_heads": ["lens"]}
    cases = []

    def native(operation, request):
        return evaluation.invoke([args.ql, "epi-agent" if operation == "invoke" else "agent-event",
                                  operation, "-", "--json"], request, 5)[0]

    def add(identifier, tool, operation, request):
        result = (evaluation.invoke([args.decide], request, 6)[0]
                  if operation == "decide" else native(operation, request))
        cases.append({"id": identifier, "tool": tool, "request": request, "expected_receipt": result})
        return result

    def response(projection, labels):
        return {"schema": "ql.agent-decision-response/v1",
                "event_basis_digest": projection["frame"]["event_basis_digest"],
                "frame_digest": c.digest(projection["frame"]), "kernel_basis": projection["frame"]["kernel_basis"],
                "outcome": "answered", "provider": {"provider_ref": "input:controlled-fault-injection",
                "model_ref": "input:controlled-response-not-model-inference",
                "model_revision": "controlled-v1", "runtime_revision": "controlled-v1"},
                "proposals": [{"head_id": "semantic-lens", "label_ids": labels, "spans": []}]}

    projected = native("project", semantic)
    refused = add("impossible-classifier-label", "ql_validate_determination", "validate",
                  {"projection": semantic, "response": response(projected, ["L99"])})
    c.require(refused["admission_status"] == "refused" and not refused["projection"]["determination"]["validated"],
              "native owner admitted impossible label")
    newer = copy.deepcopy(semantic)
    newer["event"]["generation"] += 1
    stale = add("late-answer-newer-event", "ql_validate_determination", "validate",
                {"projection": newer, "response": response(projected, ["L2"])})
    c.require(stale["admission_status"] == "stale" and not stale["projection"]["determination"]["validated"],
              "native owner admitted stale response")
    explicit = copy.deepcopy(semantic)
    explicit["event"]["observed"] = [{"field": "lens", "value": "L2'", "origin": "observed",
                                       "basis_refs": [event["material"]["ref"]]}]
    explicit["requested_heads"] = ["operation"]
    override = add("explicit-coordinate-cannot-be-overruled", "ql_validate_determination", "validate",
                   {"projection": explicit, "response": response(native("project", explicit), ["L3"])})
    c.require(override["admission_status"] == "refused"
              and override["projection"]["determination"]["observed"] == explicit["event"]["observed"],
              "response changed native observed state")
    unavailable = add("no-classifier-ordinary-QL-preserved", "ql_decide", "decide", semantic)
    c.require(unavailable["determination"]["status"] == "unavailable", "explicit unelected provider required")
    empty = copy.deepcopy(semantic)
    empty["event"]["material"]["text"] = ""
    # Material revisions hash exact text bytes, not JSON string spelling.
    empty["event"]["material"]["revision"] = "sha256:" + hashlib.sha256(b"").hexdigest()
    empty["event"]["source_basis"][0]["revision"] = empty["event"]["material"]["revision"]
    missing = add("missing-evidence-no-classifier", "ql_decide", "decide", empty)
    c.require(not missing["decision_head_ids"] and missing["determination"]["status"] == "unresolved",
              "empty evidence acquired classifier eligibility")
    ambiguous = next(row for row in rows if len(row["expected"]["fields"].get("relation-candidates", [])) > 1)
    request = copy.deepcopy(ambiguous["projection"])
    if request.get("requested_heads") == []:
        del request["requested_heads"]
    relation = add("ambiguous-relations-preserved", "ql_project_event", "project", request)
    candidates = next(fact["value"] for fact in relation["determination"]["derived"] if fact["field"] == "relation-candidates")
    c.require(candidates == ambiguous["expected"]["fields"]["relation-candidates"] and not relation["decision_head_ids"],
              "native ambiguity disappeared or became a redundant model head")
    for lens in ("L2", "L2'"):
        request = copy.deepcopy(semantic)
        request["event"]["observed"] = [{"field": field, "value": value, "origin": "observed",
                                           "basis_refs": [event["material"]["ref"]]}
                for field, value in {"lens": lens, "local-position": 3, "coordinate-face": "direct",
                                     "musical-basis": "chromatic"}.items()]
        result = add("conjugate-counterfactual:" + lens, "ql_project_event", "project", request)
        c.require(not result["decision_head_ids"], "explicit conjugate state created a redundant model head")
    add("native-operation", "ql_invoke", "invoke", {"schema": "ql.epi-logos-agent-invocation/v1",
        "position": "#1", "operation": "tda.vietoris-rips", "input": {"metric": "precomputed",
        "complex": "vietoris-rips", "coefficients": 2, "max_homology_dimension": 1, "max_scale": 2,
        "distances": [[0, 1, 1], [1, 0, 1], [1, 1, 0]], "source_basis": {"source_ref": event["material"]["ref"]}}})
    result = {"schema": "ql.body-validation-plan/v1", "cases": cases,
              "scope": "Actual native tools and kernels. Controlled response inputs are not model inference receipts.",
              "source_basis": {"generator_digest": corpus.file_digest(Path(__file__)),
                               "event_digest": corpus.file_digest(args.event), "corpus_digest": corpus_digest}}
    with args.output.open("xb") as stream:
        stream.write(c.canonical(result) + b"\n")
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--native-corpus", type=Path, required=True)
    parser.add_argument("--corpus-digest", required=True)
    parser.add_argument("--event", type=Path, default=ROOT / "fixtures/agent-decision/v1/event-v1.json")
    parser.add_argument("--ql", default="ql-agent")
    parser.add_argument("--decide", default="ql-agent-decide")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps({"native_cases": len(generate(args)["cases"]), "decision_provider_calls": 0}))


if __name__ == "__main__":
    main()
