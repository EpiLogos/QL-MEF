"""Read genuine ranked evidence and input costs from retained AIKit receipts.

This supplements a completed native evaluation. It performs no inference,
changes no determinations and never treats independent Noul scores as a
ranked distribution. Every receipt is joined by its exact event/frame basis,
response and adjacent request/provider snapshots before it contributes.
"""
import argparse
import hashlib
import json
import math
from pathlib import Path

import ql_agent_aikit as adapter
import ql_agent_contracts as c
import ql_agent_evaluation as evaluation


def file_digest(path):
    return "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()


def read(path, limit=8 * 1024 * 1024):
    data = path.read_bytes()
    c.require(len(data) <= limit, "bounded research receipt required")
    return evaluation.decode(data)


def ranked_coverage(scores, targets, k):
    # Include all exact ties at a rank boundary. A lexical label order must
    # not pretend a tied provider distribution discriminates two candidates.
    c.require(bool(targets) and targets <= scores.keys(), "ranked target absent from distribution")
    return all(1 + sum(value > scores[target] for value in scores.values()) <= k
               for target in targets)


def qualify_request(projection, receipt, request, provider):
    c.require(c.digest(request) == receipt["request_digest"], "provider request snapshot digest mismatch")
    c.require(c.digest(provider) == receipt["provider_config_digest"], "provider configuration digest mismatch")
    limits = provider.get("jev_limits" if provider.get("mode") == "hosted" else "limits", {})
    model = (limits.get("tariff", {}).get("model_version")
             if provider.get("mode") == "hosted" else limits.get("model"))
    expected_request, expected_bindings = adapter.translate_request(
        evaluation.provider_input(projection), model, receipt["schema_profile"])
    c.require(request == expected_request and c.canonical(receipt["bindings"]) == c.canonical(expected_bindings),
              "provider request/bindings differ from the joined native semantic basis")
    c.require(receipt["runtime_revision"] == c.digest({key: receipt[key] for key in (
        "aikit_executable_digest", "adapter_digest", "contract_digest", "provider_config_digest")}),
              "provider runtime revision differs from retained execution basis")
    return model


def collect(metrics_path, receipt_dir, model_card_path=None, world_path=None):
    report = read(metrics_path)
    c.require(report.get("schema") == "ql.agent-evaluation/v1", "completed evaluation required")
    cases_path = metrics_path.parent / "cases.jsonl"
    cases = [evaluation.decode(line) for line in cases_path.read_text().splitlines()]
    c.require(cases and report["metrics"] == evaluation.metrics(cases),
              "evaluation metrics differ from retained cases")
    receipts = {}
    closure = []
    for path in sorted(receipt_dir.glob("*.receipt.json")):
        receipt = read(path)
        c.require(receipt.get("schema") == "ql.aikit-translation-receipt/v1", "QL translation receipt required")
        key = receipt["event_basis_digest"], receipt["frame_digest"]
        c.require(key not in receipts, "duplicate event/frame translation receipts")
        request_path = path.with_name(path.name.replace(".receipt.json", ".request.json"))
        provider_path = path.with_name(path.name.replace(".receipt.json", ".provider.json"))
        request, provider = read(request_path), read(provider_path)
        c.require(c.digest(request) == receipt["request_digest"], "provider request snapshot digest mismatch")
        c.require(c.digest(provider) == receipt["provider_config_digest"], "provider configuration digest mismatch")
        receipts[key] = receipt, request, provider
        closure.extend({"path": str(item.resolve()), "digest": file_digest(item)}
                       for item in (path, request_path, provider_path))
    measured = ranked = nonempty = empty = 0
    covered = {k: 0 for k in (1, 2, 3)}
    supported_covered = dict(covered)
    abstention_covered = dict(covered)
    input_tokens = output_tokens = input_bytes = 0
    token_cases = 0
    inference_ms = []
    model_refs, model_revisions, runtime_revisions, endpoints = set(), set(), set(), set()
    used = set()
    for case in cases:
        if not case["provider_calls"]:
            continue
        projection = case["projection"]
        key = projection["frame"]["event_basis_digest"], c.digest(projection["frame"])
        c.require(key in receipts, "called case lacks exact event/frame provider receipt")
        receipt, request, provider = receipts[key]
        used.add(key)
        c.require(receipt.get("response") == case["response"], "provider receipt response differs from evaluated response")
        model = qualify_request(projection, receipt, request, provider)
        input_bytes += len(c.canonical(request))
        if case["response"]["outcome"] != "answered":
            continue
        model_revisions.add(receipt["model_revision"])
        runtime_revisions.add(receipt["runtime_revision"])
        envelope = receipt.get("native_envelope")
        c.require(receipt.get("exit_code") == 0 and isinstance(envelope, dict) and envelope.get("ok") is True,
                  "answered case lacks successful native invocation")
        native = envelope["data"]
        c.require(native.get("requested_model") == model and native["answer"]["model"] == model,
                  "native provider model differs from elected request model")
        endpoints.add(native["endpoint"])
        expected_response = adapter.translate_answer(
            evaluation.provider_input(projection), receipt["bindings"], native, receipt["threshold"],
            receipt["model_revision"], receipt["runtime_revision"])
        c.require(expected_response == case["response"], "native evidence does not reconstruct evaluated response")
        measured += 1
        answer = native["answer"]
        model_refs.add(answer["model"])
        latency = answer.get("latency_ms")
        if latency is not None:
            c.require(type(latency) in (int, float) and math.isfinite(latency) and latency >= 0,
                      "invalid provider inference latency")
            inference_ms.append(latency)
        usage = answer.get("usage")
        if usage is not None:
            c.require(all(type(usage.get(name)) is int and usage[name] >= 0
                          for name in ("input_tokens", "output_tokens")), "invalid native token usage")
            input_tokens += usage["input_tokens"]
            output_tokens += usage["output_tokens"]
            token_cases += 1
        for question, binding in receipt["bindings"].items():
            if not isinstance(binding, dict) or binding.get("kind") != "choice":
                continue
            head = binding["head_id"]
            c.require(head in case["expected"]["heads"], "ranked head lacks frozen expectation")
            scores = answer["answers"][question]["probabilities"]
            gold = set(case["expected"]["heads"][head])
            targets = gold or {adapter.ABSTAIN}
            ranked += 1
            nonempty += bool(gold)
            empty += not gold
            for k in covered:
                value = ranked_coverage(scores, targets, k)
                covered[k] += value
                (supported_covered if gold else abstention_covered)[k] += value
    c.require(used == set(receipts), "unrelated provider receipts in evaluation closure")
    cold_load_seconds = None
    if model_card_path is not None:
        c.require(world_path is not None, "model card requires its native Workcell provenance")
        card = read(model_card_path)
        models = card.get("models")
        c.require(isinstance(models, list) and len(models) == 1 and model_refs == {models[0]["id"]},
                  "model card identity differs from evaluated provider")
        model = models[0]
        world = read(world_path)
        c.require(world.get("version") == "workcell.material-world/v1", "native Workcell material world required")
        bindings = [binding for binding in world["binding_graph"]["bindings"]
                    if binding.get("properties", {}).get("endpoint", "").rstrip("/") + "/v1/systemone" in endpoints]
        c.require(len(bindings) == 1 and len(endpoints) == 1, "model card lacks exact native endpoint provenance")
        provenance = bindings[0]["provenance"]
        metadata = ("artifact", "artifact_revision", "sdk_revision", "license", "adapter_sha256",
                    "label_rendering", "cpu_threads")
        c.require(all(key in provenance and str(model.get(key)) == str(provenance[key]) for key in metadata),
                  "model card source metadata differs from native Workcell provenance")
        revision = model.get("checkpoint_digest", model["artifact_revision"])
        # An exact content-addressed candidate ID can itself be the declared
        # revision. Retain that original spelling; only admit it when its suffix
        # names the very checkpoint digest on the native model card.
        pinned_id = ("checkpoint_digest" in model
                     and model["id"].endswith("@" + revision.removeprefix("sha256:")))
        c.require(model_revisions == {revision} or (pinned_id and model_revisions == {model["id"]}),
                  "model card revision differs from evaluated model")
        manifest_path = Path(provenance["manifest"])
        manifest = read(manifest_path)
        c.require(file_digest(manifest_path) == "sha256:" + model["manifest_sha256"]
                  and manifest["artifact_revision"] == model["artifact_revision"]
                  and manifest["sdk_revision"] == model["sdk_revision"], "model card material manifest mismatch")
        closure.append({"path": str(manifest_path.resolve()), "digest": file_digest(manifest_path)})
        if "checkpoint_digest" in model:
            c.require(all(model.get(key) == provenance.get(key) for key in (
                "checkpoint_digest", "candidate_manifest_sha256", "candidate_adapter_sha256")),
                "candidate model card provenance mismatch")
            candidate_path = Path(provenance["candidate_manifest"])
            c.require(file_digest(candidate_path) == "sha256:" + model["candidate_manifest_sha256"],
                      "candidate model card manifest mismatch")
            closure.append({"path": str(candidate_path.resolve()), "digest": file_digest(candidate_path)})
        cold_load_seconds = model.get("load_seconds")
        c.require(type(cold_load_seconds) in (int, float) and math.isfinite(cold_load_seconds)
                  and cold_load_seconds >= 0, "actual model load duration required")
        closure.append({"path": str(model_card_path.resolve()), "digest": file_digest(model_card_path)})
        closure.append({"path": str(world_path.resolve()), "digest": file_digest(world_path)})
    closure.extend({"path": str(path.resolve()), "digest": file_digest(path)}
                   for path in (metrics_path, cases_path))
    return {"schema": "ql.agent-receipt-metrics/v1", "suite_digest": report["suite_digest"],
            "split": report["split"], "model_refs": sorted(model_refs),
            "model_revisions": sorted(model_revisions), "runtime_revisions": sorted(runtime_revisions),
            "answered_cases": measured, "unanswered_cases": len(used) - measured, "ranked_heads": ranked,
            "model_revision_scope": "completed native answers; unavailable attempts do not establish a returned model revision",
            "top_k_candidate_set_coverage": {str(k): evaluation.rate(v, ranked) for k, v in covered.items()},
            "top_k_supported_set_coverage": {str(k): evaluation.rate(v, nonempty) for k, v in supported_covered.items()},
            "top_k_abstention_coverage": {str(k): evaluation.rate(v, empty) for k, v in abstention_covered.items()},
            "rank_scope": "answered Choice heads; complete gold set or provider abstention control; exact ties retained",
            "native_request_bytes": input_bytes, "input_tokens": input_tokens if token_cases else None,
            "output_tokens": output_tokens if token_cases else None, "token_usage_cases": token_cases,
            "reported_inference_latency_cases": len(inference_ms),
            "reported_inference_p50_seconds": evaluation.quantile(inference_ms, .5) / 1000 if inference_ms else None,
            "reported_inference_p95_seconds": evaluation.quantile(inference_ms, .95) / 1000 if inference_ms else None,
            "cold_model_load_seconds": cold_load_seconds,
            "latency_scope": "native provider-reported inference; optional separately recorded cold model load; no warm/cold inference classification inferred",
            "source_digest": file_digest(Path(__file__)), "evidence": closure,
            "operative_mutation": False}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--evaluation", type=Path, required=True)
    parser.add_argument("--receipts", type=Path, required=True)
    parser.add_argument("--model-card", type=Path)
    parser.add_argument("--world", type=Path)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = collect(args.evaluation, args.receipts, args.model_card, args.world)
    with args.output.open("xb") as stream:
        stream.write(c.canonical(result) + b"\n")
    print(json.dumps({key: result[key] for key in ("answered_cases", "ranked_heads", "top_k_candidate_set_coverage",
          "input_tokens", "reported_inference_p95_seconds", "cold_model_load_seconds")}))


if __name__ == "__main__":
    main()
