"""Generate source-pinned formal specimens through the actual QL event owner.

This is a structural corpus, not reviewed natural-language training data. Gold
facts, relation overlaps, completion alternatives and harmonics are retained
native results; no QL tables or arithmetic live here. A manifest is published
only after all source and executable digests remain current.
"""
import argparse
import copy
import hashlib
import json
import math
import shutil
import subprocess
import sys
from collections import Counter
from pathlib import Path

import ql_agent_contracts as c
import ql_agent_evaluation as e

SPLITS = {
    "coordinate-local": "train", "coordinate-absolute": "validation",
    "relation-canonical": "train", "relation-reversed": "test",
    "relation-overlap": "test", "relation-noncanonical": "test",
    "completion-d1": "train", "completion-d2-source": "validation",
    "completion-d2-target": "validation", "completion-d3": "train",
    "completion-missing-side": "test", "context-explicit": "train",
    "context-inverse": "test", "constellation-prefix": "validation",
    "missing-structural-input": "test", "missing-semantic-evidence": "test",
}
for _family in tuple(SPLITS):
    if _family.startswith("completion-"):
        SPLITS[_family + "-reversed"] = "test"
        SPLITS[_family + "-overlap"] = "test"
BASES = ("chromatic", "fifths")  # Accepted input enum, not a music-theory table.


def file_digest(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1048576), b""):
            digest.update(block)
    return "sha256:" + digest.hexdigest()


def publish(path, value):
    with path.open("xb") as stream:
        stream.write(c.canonical(value) + b"\n")


def refusal_reason(error, executable, expected):
    # Match the native CLI's actual exit and owner diagnostic. Neither a path
    # nor an unrelated transport error containing a keyword is a refusal.
    prefix = f"{executable} exited 2: ql: "
    message = str(error)
    c.require(message == prefix + expected + "\n",
              "counterfactual failed outside the expected native kernel admission")
    return expected


def specimen(family, identifier, fields, requested_heads=()):
    c.require(family in SPLITS, "unknown structural/template family")
    reference = "ql:corpus:agent-decision/v1/" + family + "/" + identifier
    text = c.canonical(fields).decode("utf-8")
    revision = "sha256:" + hashlib.sha256(text.encode()).hexdigest()
    event = {"schema": "ql.agent-event/v1", "event_ref": "ql:event:" + reference,
             "generation": 1, "occasion_refs": [], "kind": "controlled-formal-specimen",
             "subject": family, "material": {"ref": reference, "revision": revision, "text": text},
             "source_basis": [{"ref": reference, "revision": revision}],
             "observed": [{"field": key, "value": value, "origin": "observed", "basis_refs": [reference]}
                          for key, value in sorted(fields.items())],
             "native_state": [], "provenance_refs": ["https://github.com/EpiLogos/QL-MEF/issues/291"]}
    return {"event": event, "requested_heads": list(requested_heads)}


def counterfactual(request, field, wrong_value):
    result = copy.deepcopy(request)
    event = result["event"]
    fields = {fact["field"]: fact["value"] for fact in event["observed"]}
    fields[field] = wrong_value
    text = c.canonical(fields).decode("utf-8")
    revision = "sha256:" + hashlib.sha256(text.encode()).hexdigest()
    event["material"].update(text=text, revision=revision)
    event["source_basis"][0]["revision"] = revision
    event["observed"] = [{"field": key, "value": value, "origin": "observed",
                           "basis_refs": [event["material"]["ref"]]} for key, value in sorted(fields.items())]
    return result


class NativeCorpus:
    def __init__(self, executable, timeout):
        self.executable, self.timeout = executable, timeout
        self.kernel = None
        self.cases, self.readings, self.refusals = [], [], []
        self.context = {"phase": "native owner discovery"}

    def project(self, request):
        self.context = {"phase": "native projection", "request": request}
        result, _ = e.invoke([str(self.executable), "agent-event", "project", "-", "--json"],
                             request, self.timeout)
        e.check_projection(result)
        c.require(result["event"] == request["event"], "native event basis changed")
        kernel = result["frame"]["kernel_basis"]
        if self.kernel is None:
            self.kernel = kernel
        c.require(kernel == self.kernel, "compiled kernel changed during corpus generation")
        c.require(not result["determination"]["learned"] and "provider" not in result["determination"],
                  "structural generation unexpectedly invoked a classifier")
        return result

    def add(self, family, identifier, fields, requested_heads=()):
        request = specimen(family, identifier, fields, requested_heads)
        projection = self.project(request)
        heads = {head: [] for head in projection["decision_head_ids"]}
        c.require(not heads or family == "missing-semantic-evidence",
                  "structural specimen unexpectedly requires semantic labels")
        expected = {"fields": e.operative_fields(projection["determination"]), "heads": heads,
                    "status": projection["determination"]["status"],
                    "basis_refs": [self.kernel["owner_ref"] + "@" + self.kernel["revision"],
                                   self.kernel["digest"]]}
        e.check_expectation(projection, expected)
        identifier = family + "/" + identifier
        c.require(not any(case["id"] == identifier for case in self.cases), "duplicate corpus case")
        self.cases.append({"schema": "ql.agent-evaluation-case/v1", "id": identifier,
                           "family": family, "split": SPLITS[family],
                           "projection": request, "expected": expected})
        self.readings.append({"id": identifier, "projection": projection})
        c.require(len(self.cases) <= 1024, "bounded structural corpus exceeded")
        return request, projection

    def refuse(self, positive, field, wrong_value, expected_reason):
        request, projection = positive
        negative = counterfactual(request, field, wrong_value)
        self.context = {"phase": "native counterfactual", "request": negative}
        try:
            e.invoke([str(self.executable), "agent-event", "project", "-", "--json"], negative, self.timeout)
        except RuntimeError as error:
            reason = refusal_reason(error, self.executable, expected_reason)
            self.refusals.append({"schema": "ql.agent-corpus-refusal/v1", "field": field,
                                  "positive_event_basis": projection["frame"]["event_basis_digest"],
                                  "kernel_basis": self.kernel, "projection": negative,
                                  "native_error": str(error), "native_reason": reason})
        else:
            raise c.ContractError("impossible counterfactual became a native result: " + field)


def generate(native):
    discovery = native.project(specimen("missing-semantic-evidence", "discovery", {}, ["lens", "context-frame"]))
    labels = {head["field"]: head["labels"] for head in discovery["frame"]["unresolved"]}
    lenses = [label["id"] for label in labels["lens"]]
    frames = [label["id"] for label in labels["context-frame"]]
    inputs = native.project(specimen("missing-structural-input", "positions", {"source-position": 0}))
    positions = next(head["labels"] for head in inputs["frame"]["unresolved"] if head["field"] == "target-position")
    positions = [int(label["id"]) for label in positions]
    c.require(len(lenses) == len(set(lenses)) and bool(lenses), "invalid native lens field")
    c.require(len(positions) == len(set(positions)) and bool(positions), "invalid native position field")
    coordinates = []
    for lens in lenses:
        for position in positions:
            for face in ("direct", "conjugate"):
                for basis in BASES:
                    identifier = f"{lens}/{position}/{face}/{basis}"
                    fields = {"lens": lens, "local-position": position, "coordinate-face": face, "musical-basis": basis}
                    positive = native.add("coordinate-local", identifier, fields, ["lens"])
                    coordinates.append(positive)
                    if face == "direct" and basis == BASES[0]:
                        fields = dict(fields)
                        del fields["local-position"]
                        fields["absolute-position"] = e.operative_fields(positive[1]["determination"])["absolute-position"]
                        native.add("coordinate-absolute", identifier, fields, ["lens"])

    pairs = []
    for source in positions:
        for target in positions:
            fields = {"source-position": source, "target-position": target,
                      "lens": lenses[0], "musical-basis": BASES[0]}
            request = specimen("relation-canonical", f"{source}/{target}", fields)
            reading = native.project(request)
            candidates = e.operative_fields(reading["determination"])["relation-candidates"]
            family = ("relation-overlap" if len(candidates) > 1 else "relation-noncanonical" if not candidates
                      else "relation-reversed" if candidates[0]["reversed"] else "relation-canonical")
            positive = native.add(family, f"{source}/{target}", fields)
            pairs.append((positive, candidates))
            if candidates:
                for degree, side, completion_family in (("D1", None, "completion-d1"), ("D2", "source", "completion-d2-source"),
                                             ("D2", "target", "completion-d2-target"), ("D3", None, "completion-d3"),
                                             ("D2", None, "completion-missing-side")):
                    completed = dict(fields, **{"completion-degree": degree})
                    if side is not None:
                        completed["d2-expansion"] = side
                    if family == "relation-overlap":
                        completion_family += "-overlap"
                    elif family == "relation-reversed":
                        completion_family += "-reversed"
                    native.add(completion_family, f"{source}/{target}", completed)

    contexts = []
    for frame in frames:
        fields = {"context-frame": frame, "lens": lenses[0], "musical-basis": BASES[0]}
        positive = native.add("context-explicit", frame, fields, ["context-frame"])
        contexts.append(positive)
        values = e.operative_fields(positive[1]["determination"])
        del fields["context-frame"]
        for field in ("context-local-position", "context-unit-face", "context-grain"):
            fields[field] = values[field]
        native.add("context-inverse", frame, fields, ["context-frame"])

    members = [{"subject_ref": f"ql:corpus:member:{face}:{position}", "position": position, "face": face}
               for face in ("direct", "conjugate") for position in positions]
    for count in range(len(members) + 1):
        native.add("constellation-prefix", str(count),
                   {"constellation": {"anchor_ref": "ql:corpus:whole", "members": members[:count]},
                    "lens": lenses[0], "musical-basis": BASES[0]})
    native.add("missing-structural-input", "traversal", {"source-position": positions[0]})
    native.add("missing-structural-input", "pitch-face", {"lens": lenses[0], "local-position": positions[0], "musical-basis": BASES[0]})
    native.add("missing-semantic-evidence", "empty-material", {}, ["lens", "context-frame"])

    first = coordinates[0]
    first_values = e.operative_fields(first[1]["determination"])
    values = [e.operative_fields(reading["determination"]) for _, reading in coordinates]
    other_face = next(value["lens-face"] for value in values if value["lens-face"] != first_values["lens-face"])
    lens_error = "observed lens state contradicts kernel derivation"
    native.refuse(first, "lens-face", other_face, lens_error)
    native.refuse(first, "lens-complement", next(lens for lens in lenses if lens != first_values["lens-complement"]), lens_error)
    native.refuse(first, "absolute-position", next(position for position in positions if position != first_values["absolute-position"]), lens_error)
    pitch = next(fact["value"] for fact in first[1]["harmonic"]["harmonic"] if fact["field"] == "pitch-class")
    other_pitch = next(fact["value"] for _, reading in coordinates for fact in reading["harmonic"]["harmonic"]
                       if fact["field"] == "pitch-class" and fact["value"] != pitch)
    native.refuse(first, "pitch-class", other_pitch, lens_error)
    positive, candidates = next(pair for pair in pairs if len(pair[1]) == 1)
    other_family = next(candidate["family"] for _, rows in pairs for candidate in rows
                        if candidate["family"] != candidates[0]["family"])
    relation_error = "observed relation selection contradicts native traversal candidates"
    native.refuse(positive, "relation-family", other_family, relation_error)
    other_index = next(candidate["pair_index"] for _, rows in pairs for candidate in rows
                       if candidate["family"] == candidates[0]["family"] and candidate["pair_index"] != candidates[0]["pair_index"])
    native.refuse(positive, "pair-index", other_index, relation_error)
    context_face = e.operative_fields(contexts[0][1]["determination"])["context-unit-face"]
    other_context = next(e.operative_fields(reading["determination"])["context-unit-face"] for _, reading in contexts
                         if e.operative_fields(reading["determination"])["context-unit-face"] != context_face)
    native.refuse(contexts[0], "context-unit-face", other_context,
                  "observed Context Frame contradicts its native structural state")
    return {"lenses": labels["lens"], "context_frames": labels["context-frame"], "positions": positions}


def run(args):
    executable = shutil.which(args.ql)
    c.require(executable is not None, "actual QL executable unavailable")
    executable = Path(executable).resolve()
    sources = {"ql_executable": executable, "generator": Path(__file__).resolve(),
               "contracts": Path(c.__file__).resolve(), "evaluation": Path(e.__file__).resolve()}
    digests = {key: file_digest(path) for key, path in sources.items()}
    args.output.mkdir(parents=True, exist_ok=False)
    native = NativeCorpus(executable, args.timeout)
    try:
        owners = generate(native)
        c.require(all(file_digest(path) == digests[key] for key, path in sources.items()),
                  "source or executable changed during corpus generation")
        for name, rows in (("suite.jsonl", native.cases), ("native-readings.jsonl", native.readings),
                           ("refusals.jsonl", native.refusals)):
            with (args.output / name).open("xb") as stream:
                for row in rows:
                    stream.write(c.canonical(row) + b"\n")
        suite_digest = file_digest(args.output / "suite.jsonl")
        e.load_suite(args.output / "suite.jsonl", suite_digest)
        report = {"schema": "ql.agent-corpus/v1", "kernel_basis": native.kernel, "source_digests": digests,
                  "suite_digest": suite_digest, "readings_digest": file_digest(args.output / "native-readings.jsonl"),
                  "refusals_digest": file_digest(args.output / "refusals.jsonl"), "cases": len(native.cases),
                  "refusals": len(native.refusals), "families": dict(Counter(case["family"] for case in native.cases)),
                  "split_counts": dict(Counter(case["split"] for case in native.cases)), "family_splits": SPLITS,
                  "native_label_owners": owners, "provider_calls": 0,
                  "standing": "deterministic structural corpus; no trained checkpoint or model benchmark",
                  "not_covered": ["reviewed semantic routing labels", "actual agent traces", "multiplicative geometry",
                                  "all constellation subsets", "native body execution", "model resource/latency"]}
        publish(args.output / "manifest.json", report)
        return report
    except BaseException as error:
        publish(args.output / "failure.json", {"schema": "ql.agent-corpus-failure/v1", "error": str(error),
                "error_type": type(error).__name__, "context": native.context, "source_digests": digests,
                "standing": "failed generation; no frozen corpus manifest"})
        raise


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ql", default="ql")
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--timeout", type=float, default=30)
    args = parser.parse_args()
    if not math.isfinite(args.timeout) or args.timeout <= 0:
        parser.error("timeout must be finite and positive")
    try:
        report = run(args)
    except (c.ContractError, OSError, ValueError, KeyError, StopIteration, RuntimeError, subprocess.TimeoutExpired) as error:
        parser.exit(1, str(error) + "\n")
    print(json.dumps({key: report[key] for key in ("cases", "refusals", "suite_digest", "split_counts")}))


if __name__ == "__main__":
    main()
