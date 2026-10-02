"""QD0 QL-owned wire contracts. No QL arithmetic or decision-provider calls.

This validates the event/frame provenance boundary. Kernel semantic validation,
constraint generation, model execution and harmonic computation belong to QD1+.
"""
import copy
import hashlib
import json
from pathlib import Path
from jsonschema import Draft202012Validator

ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas/ql-agent-contracts-v1.schema.json"
NONEMPTY = {"type": "string", "minLength": 1}
DIGEST = {"type": "string", "pattern": "^sha256:[0-9a-f]{64}$"}

def obj(properties, required=None):
    return {"type": "object", "properties": properties,
            "required": list(properties) if required is None else required,
            "additionalProperties": False}

def arr(items, minimum=0):
    return {"type": "array", "items": items, "minItems": minimum}

def ref(name):
    return {"$ref": "#/$defs/" + name}

def canonical(value):
    return json.dumps(value, ensure_ascii=False, sort_keys=True,
                      separators=(",", ":"), allow_nan=False).encode("utf-8")

def digest(value):
    return "sha256:" + hashlib.sha256(canonical(value)).hexdigest()

SOURCE = obj({"ref": NONEMPTY, "revision": NONEMPTY})
KERNEL = obj({"owner_ref": NONEMPTY, "revision": NONEMPTY, "digest": DIGEST})
FACT = obj({"field": NONEMPTY, "value": {},
            "origin": {"enum": ["observed", "derived"]},
            "basis_refs": arr(NONEMPTY, 1), "rule_ref": NONEMPTY},
           ["field", "value", "origin", "basis_refs"])
FACT["allOf"] = [{"if": {"properties": {"origin": {"const": "derived"}}},
                  "then": {"required": ["rule_ref"]}}]
OBSERVED_FACT = copy.deepcopy(FACT)
OBSERVED_FACT["properties"]["origin"] = {"const": "observed"}
DERIVED_FACT = copy.deepcopy(FACT)
DERIVED_FACT["properties"]["origin"] = {"const": "derived"}
LABEL = obj({"id": NONEMPTY, "description": NONEMPTY,
             "owner_ref": NONEMPTY, "owner_revision": NONEMPTY})
CARD = obj({"min": {"type": "integer", "minimum": 0},
            "max": {"type": "integer", "minimum": 1}})
HEAD = obj({"id": NONEMPTY, "field": NONEMPTY, "labels": arr(ref("label"), 1),
            "cardinality": CARD,
            "ambiguity_policy": {"enum": ["preserve-candidates", "abstain", "single-if-supported"]}})
SPAN = obj({"material_ref": NONEMPTY, "revision": NONEMPTY,
            "start": {"type": "integer", "minimum": 0},
            "end": {"type": "integer", "minimum": 0}, "text": {"type": "string"}})
SELECTION = obj({"head_id": NONEMPTY, "label_ids": arr(NONEMPTY),
                 "match": {"enum": ["any", "all", "exact"]}})
def constraint(kind, fields):
    return obj({"kind": {"const": kind}, "id": NONEMPTY,
                "rule_ref": NONEMPTY, **fields})
CONSTRAINT = {"oneOf": [
    constraint("implication", {"antecedent": arr(ref("selection"), 1),
                               "consequent": arr(ref("selection"), 1)}),
    constraint("exclusion", {"forbidden_together": arr(ref("selection"), 1)}),
    constraint("legal-combination", {"allowed_tuples": arr(arr(ref("selection"), 1), 1)}),
    constraint("candidate-restriction", {"allowed": ref("selection")})]}
PROVIDER = obj({"provider_ref": NONEMPTY, "model_ref": NONEMPTY,
                "model_revision": NONEMPTY, "runtime_revision": NONEMPTY})
LEARNED = obj({"head_id": NONEMPTY, "label_ids": arr(NONEMPTY),
               "confidence": {"type": "number", "minimum": 0, "maximum": 1},
               "spans": arr(ref("span"))}, ["head_id", "label_ids", "spans"])
VALIDATED = obj({"field": NONEMPTY, "value": {},
                 "origin": {"enum": ["observed", "learned"]},
                 "proposal_head": NONEMPTY, "kernel_rule_refs": arr(NONEMPTY, 1),
                 "basis_digest": DIGEST},
                ["field", "value", "origin", "kernel_rule_refs", "basis_digest"])
VALIDATED["allOf"] = [{"if": {"properties": {"origin": {"const": "learned"}}},
                       "then": {"required": ["proposal_head"]}}]
UNRESOLVED = obj({"head_id": NONEMPTY, "reason": NONEMPTY})
REFUSED = obj({"head_id": NONEMPTY, "label_ids": arr(NONEMPTY),
               "origin": {"enum": ["observed", "learned"]},
               "reason": NONEMPTY, "rule_refs": arr(NONEMPTY, 1)})

EVENT = obj({"schema": {"const": "ql.agent-event/v1"}, "event_ref": NONEMPTY,
             "generation": {"type": "integer", "minimum": 0},
             "occasion_refs": arr(NONEMPTY), "source_basis": arr(ref("source"), 1),
             "kind": NONEMPTY, "subject": NONEMPTY,
             "material": obj({"ref": NONEMPTY, "revision": NONEMPTY, "text": {"type": "string"}}),
             "observed": arr(ref("observed_fact")), "native_state": arr(ref("observed_fact")),
             "bindings": obj({"body_ref": NONEMPTY, "session_ref": NONEMPTY,
                              "native_event_refs": arr(NONEMPTY)}, []),
             "provenance_refs": arr(NONEMPTY, 1)},
            ["schema", "event_ref", "generation", "occasion_refs", "source_basis",
             "kind", "subject", "material", "observed", "native_state", "provenance_refs"])
FRAME = obj({"schema": {"const": "ql.agent-decision-frame/v1"}, "frame_ref": NONEMPTY,
             "event_ref": NONEMPTY, "event_basis_digest": DIGEST,
             "source_basis": arr(ref("source"), 1), "kernel_basis": ref("kernel"),
             "determined": obj({"observed": arr(ref("observed_fact")), "derived": arr(ref("derived_fact"))}),
             "unresolved": arr(ref("head")), "constraints": arr(ref("constraint")),
             "requested_evidence": arr(NONEMPTY)})
DETERMINATION = obj({"schema": {"const": "ql.agent-determination/v1"},
             "event_ref": NONEMPTY, "event_basis_digest": DIGEST, "frame_digest": DIGEST,
             "source_basis": arr(ref("source"), 1), "kernel_basis": ref("kernel"),
             "status": {"enum": ["determined", "partial", "unresolved", "unavailable", "refused", "stale"]},
             "observed": arr(ref("observed_fact")), "derived": arr(ref("derived_fact")),
             "learned": arr(ref("learned")), "validated": arr(ref("validated")),
             "unresolved": arr(ref("unresolved")), "refused_candidates": arr(ref("refused")),
             "constraint_application": arr(obj({"constraint_id": NONEMPTY,
                 "result": {"enum": ["passed", "refused", "not-applicable"]}, "reason": NONEMPTY})),
             "provider": ref("provider")},
             ["schema", "event_ref", "event_basis_digest", "frame_digest", "source_basis",
              "kernel_basis", "status", "observed", "derived", "learned", "validated",
              "unresolved", "refused_candidates", "constraint_application"])
HARMONIC = obj({"schema": {"const": "ql.harmonic-event/v1"}, "event_ref": NONEMPTY,
                "source_basis": arr(ref("source"), 1), "kernel_basis": ref("kernel"),
                "determination_digest": DIGEST,
                "formal": arr({"oneOf": [ref("fact"), ref("validated")]}),
                "harmonic": arr(ref("derived_fact"))})
DEFS = {"source": SOURCE, "kernel": KERNEL, "fact": FACT, "label": LABEL,
        "observed_fact": OBSERVED_FACT, "derived_fact": DERIVED_FACT,
        "head": HEAD, "span": SPAN, "selection": SELECTION, "constraint": CONSTRAINT,
        "provider": PROVIDER, "learned": LEARNED, "validated": VALIDATED,
        "unresolved": UNRESOLVED, "refused": REFUSED, "event": EVENT, "frame": FRAME,
        "determination": DETERMINATION, "harmonic": HARMONIC}
SCHEMA = {"$schema": "https://json-schema.org/draft/2020-12/schema",
          "$id": "https://ql.epilogos.org/contracts/ql-agent-contracts-v1.schema.json",
          "title": "QL native agent event and decision contracts",
          "$defs": DEFS,
          "oneOf": [ref(k) for k in ("event", "frame", "determination", "harmonic")]}

class ContractError(ValueError):
    pass

def require(condition, reason):
    if not condition:
        raise ContractError(reason)

def unique(values, reason):
    require(len(values) == len(set(values)), reason)

def validate(document):
    errors = list(Draft202012Validator(SCHEMA).iter_errors(document))
    require(not errors, "schema: " + "; ".join(e.message for e in errors[:2]))
    canonical(document)  # Refuse NaN/Infinity even in arbitrary formal values.
    unique([s["ref"] for s in document["source_basis"]], "duplicate source ref")
    return document

def semantic_basis(event):
    validate(event)
    return digest({k: v for k, v in event.items() if k != "bindings"})

def selections(constraint):
    kind = constraint["kind"]
    if kind == "implication":
        return constraint["antecedent"] + constraint["consequent"]
    if kind == "exclusion":
        return constraint["forbidden_together"]
    if kind == "legal-combination":
        return [s for row in constraint["allowed_tuples"] for s in row]
    return [constraint["allowed"]]

def validate_frame(event, frame):
    validate(event)
    validate(frame)
    require(event["schema"] == "ql.agent-event/v1", "expected event schema")
    require(frame["schema"] == "ql.agent-decision-frame/v1", "expected frame schema")
    require(frame["event_ref"] == event["event_ref"], "wrong event ref")
    require(frame["event_basis_digest"] == semantic_basis(event), "stale event basis")
    require(frame["source_basis"] == event["source_basis"], "source revisions differ")
    observed = event["observed"] + event["native_state"]
    require(all(f["origin"] == "observed" for f in observed), "event facts must be observed")
    require(frame["determined"]["observed"] == observed, "observed facts lost or changed")
    require(all(f["origin"] == "derived" for f in frame["determined"]["derived"]),
            "deterministic facts need derived origin")
    determined = observed + frame["determined"]["derived"]
    unique([f["field"] for f in determined], "duplicate/conflicting determined field")
    unique([h["id"] for h in frame["unresolved"]], "duplicate head")
    unique([h["field"] for h in frame["unresolved"]], "duplicate unresolved field")
    fields = {f["field"] for f in determined}
    for head in frame["unresolved"]:
        require(head["field"] not in fields, "attempt to predict determined field")
        labels = head["labels"]
        unique([x["id"] for x in labels], "duplicate label")
        card = head["cardinality"]
        require(card["min"] <= card["max"] <= len(labels), "impossible cardinality")
    unique([c["id"] for c in frame["constraints"]], "duplicate constraint")
    heads = {h["id"]: h for h in frame["unresolved"]}
    for constraint in frame["constraints"]:
        for selection in selections(constraint):
            require(selection["head_id"] in heads, "constraint names absent head")
            legal = {x["id"] for x in heads[selection["head_id"]]["labels"]}
            unique(selection["label_ids"], "constraint duplicates label")
            require(set(selection["label_ids"]) <= legal, "constraint names illegal label")
            require(selection["label_ids"] or selection["match"] == "exact",
                    "empty constraint predicate must use exact match")
        if constraint["kind"] == "legal-combination":
            rows = constraint["allowed_tuples"]
            row_heads = []
            for row in rows:
                unique([s["head_id"] for s in row], "tuple repeats head")
                require(all(s["match"] == "exact" for s in row), "legal tuple requires exact selections")
                row_heads.append({s["head_id"] for s in row})
            require(all(ids == row_heads[0] for ids in row_heads), "legal tuples cover different heads")
        if constraint["kind"] == "candidate-restriction":
            require(constraint["allowed"]["match"] == "all", "restriction must bound all candidates")
    return frame

def validate_determination(event, frame, determination):
    validate_frame(event, frame)
    validate(determination)
    require(determination["schema"] == "ql.agent-determination/v1", "expected determination schema")
    require(determination["event_ref"] == event["event_ref"], "wrong determination event")
    require(determination["event_basis_digest"] == semantic_basis(event), "stale determination")
    require(determination["frame_digest"] == digest(frame), "stale decision frame")
    require(determination["source_basis"] == frame["source_basis"], "changed source basis")
    require(determination["kernel_basis"] == frame["kernel_basis"], "changed kernel basis")
    require(determination["observed"] == frame["determined"]["observed"], "overridden observed state")
    original_derived = frame["determined"]["derived"]
    require(determination["derived"][:len(original_derived)] == original_derived,
            "overridden derivation")
    heads = {h["id"]: h for h in frame["unresolved"]}
    learned = determination["learned"]
    refused = determination["refused_candidates"]
    if learned or any(x["origin"] == "learned" for x in refused):
        require("provider" in determination, "learned answer lacks provider/model/runtime")
    unique([x["head_id"] for x in learned], "duplicate learned head")
    by_head = {x["head_id"]: x for x in learned}
    for proposal in learned:
        require(proposal["head_id"] in heads, "unrequested learned head")
        head = heads[proposal["head_id"]]
        labels = proposal["label_ids"]
        unique(labels, "duplicate learned label")
        legal = set(labels) <= {x["id"] for x in head["labels"]}
        cardinality = head["cardinality"]["min"] <= len(labels) <= head["cardinality"]["max"]
        exact_refusal = any(x["origin"] == "learned" and x["head_id"] == proposal["head_id"]
                            and x["label_ids"] == labels for x in refused)
        require(legal or exact_refusal, "illegal learned label lacks exact refusal")
        require(cardinality or exact_refusal, "learned cardinality violation lacks exact refusal")
        for span in proposal["spans"]:
            material = event["material"]
            require(span["material_ref"] == material["ref"] and span["revision"] == material["revision"],
                    "evidence points to different material/revision")
            require(0 <= span["start"] <= span["end"] <= len(material["text"]), "span outside source")
            require(material["text"][span["start"]:span["end"]] == span["text"], "fabricated evidence span")
    dispositions = []
    for item in determination["unresolved"] + determination["refused_candidates"]:
        require(item["head_id"] in heads, "disposition names absent head")
        dispositions.append(item["head_id"])
    for item in refused:
        if item["origin"] == "learned":
            require(item["head_id"] in by_head and item["label_ids"] == by_head[item["head_id"]]["label_ids"],
                    "learned refusal lacks original proposal")
    learned_validations = []
    fixed = {f["field"]: f["value"] for f in
             frame["determined"]["observed"] + frame["determined"]["derived"]}
    observations = {f["field"]: f["value"] for f in frame["determined"]["observed"]}
    for admitted in determination["validated"]:
        require(admitted["basis_digest"] == digest(frame), "validation uses stale basis")
        if admitted["origin"] == "learned":
            head_id = admitted["proposal_head"]
            require(head_id in by_head, "validation lacks original learned proposal")
            require(admitted["field"] == heads[head_id]["field"], "validation changed target field")
            require(admitted["value"] == by_head[head_id]["label_ids"], "silent repair of proposal")
            require(admitted["value"], "empty learned selection remains abstention")
            require(set(admitted["value"]) <= {x["id"] for x in heads[head_id]["labels"]},
                    "invalid proposal cannot become operative")
            card = heads[head_id]["cardinality"]
            require(card["min"] <= len(admitted["value"]) <= card["max"],
                    "invalid cardinality cannot become operative")
            learned_validations.append(head_id)
            dispositions.append(head_id)
        else:
            require(admitted["field"] in observations and admitted["value"] == observations[admitted["field"]],
                    "validated observation differs from observed basis")
        require(admitted["field"] not in fixed or admitted["origin"] == "observed",
                "learned validation overrides fixed state")
    unique([x["field"] for x in determination["validated"]], "duplicate validated field")
    # QD1 completes newly admitted semantic coordinates through the same native
    # kernel. The reference validator checks standing/basis, not QL arithmetic.
    post_validation = determination["derived"][len(original_derived):]
    admitted_refs = {frame["frame_ref"] + "#" + x["proposal_head"]
                     for x in determination["validated"] if x["origin"] == "learned"}
    for fact in post_validation:
        require(set(fact["basis_refs"]) & admitted_refs,
                "post-validation derivation lacks accepted semantic basis")
        dispositions.extend(h for h, head in heads.items() if head["field"] == fact["field"])
    unique(dispositions, "head receives contradictory dispositions")
    require(set(dispositions) == set(heads), "unaccounted unresolved head")
    unique([f["field"] for f in determination["observed"] + determination["derived"] +
            [f for f in determination["validated"] if f["origin"] == "learned"]],
           "duplicate determined or admitted field")
    applications = determination["constraint_application"]
    unique([x["constraint_id"] for x in applications], "duplicate constraint receipt")
    constraints = {x["id"] for x in frame["constraints"]}
    require(all(x["constraint_id"] in constraints for x in applications), "undeclared constraint receipt")
    status = determination["status"]
    if status in ("unavailable", "unresolved", "refused", "stale"):
        require(not learned_validations, "non-operative status carries learned validation")
    if status == "unavailable":
        require(not learned and not refused, "unavailable status carries model output")
    if status == "refused":
        require(refused, "refused status lacks refusal")
    if determination["status"] == "determined":
        require(not determination["unresolved"] and not determination["refused_candidates"],
                "definite status with unresolved/refused fields")
        require(set(x["constraint_id"] for x in applications) == constraints,
                "determined status lacks constraint receipts")
        require(all(x["result"] != "refused" for x in applications),
                "determined status carries refused constraint")
    if status == "partial":
        require(learned_validations and (determination["unresolved"] or refused),
                "partial status lacks both admitted and remaining heads")
    if not heads:
        require(not learned and "provider" not in determination, "provider on deterministic-only frame")
        require(status == "determined", "deterministic-only result must be determined")
    # Kernel semantic admission and execution are deliberately not authorised here.
    return determination

def validate_harmonic(determination, harmonic):
    validate(determination)
    validate(harmonic)
    require(determination["schema"] == "ql.agent-determination/v1", "expected determination schema")
    require(harmonic["schema"] == "ql.harmonic-event/v1", "expected harmonic schema")
    for field in ("event_ref", "source_basis", "kernel_basis"):
        require(harmonic[field] == determination[field], "harmonic basis differs: " + field)
    require(harmonic["determination_digest"] == digest(determination), "stale harmonic determination")
    facts = determination["observed"] + determination["derived"] + determination["validated"]
    require(all(f in facts for f in harmonic["formal"]), "harmonic formal state changed determination")
    unique([f["field"] for f in harmonic["formal"]], "duplicate harmonic formal field")
    unique([f["field"] for f in harmonic["harmonic"]], "duplicate harmonic derived field")
    return harmonic

def write_schema(path=SCHEMA_PATH):
    Draft202012Validator.check_schema(SCHEMA)
    Path(path).write_bytes(canonical(SCHEMA) + b"\n")

def main():
    import argparse
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check-schema", action="store_true",
                        help="Require the published schema to equal the source grammar")
    parser.add_argument("--write-schema", action="store_true")
    parser.add_argument("--event", type=Path)
    parser.add_argument("--frame", type=Path)
    parser.add_argument("--determination", type=Path)
    parser.add_argument("--harmonic", type=Path)
    args = parser.parse_args()
    if not any(vars(args).values()):
        parser.error("select --check-schema, --write-schema or a document chain")
    if args.frame and not args.event:
        parser.error("--frame requires --event")
    if args.determination and not (args.event and args.frame):
        parser.error("--determination requires --event and --frame")
    if args.harmonic and not args.determination:
        parser.error("--harmonic requires --determination")
    try:
        if args.check_schema:
            require(json.loads(SCHEMA_PATH.read_text()) == SCHEMA,
                    "published schema differs from source grammar")
        if args.write_schema:
            write_schema()
        documents = {key: json.loads(path.read_text(encoding="utf-8"))
                     for key, path in vars(args).items()
                     if key in ("event", "frame", "determination", "harmonic") and path}
        expected_schemas = {"event": "ql.agent-event/v1", "frame": "ql.agent-decision-frame/v1",
                            "determination": "ql.agent-determination/v1", "harmonic": "ql.harmonic-event/v1"}
        for key, document in documents.items():
            validate(document)
            require(document["schema"] == expected_schemas[key], "expected " + key + " schema")
        if "frame" in documents:
            validate_frame(documents["event"], documents["frame"])
        if "determination" in documents:
            validate_determination(documents["event"], documents["frame"], documents["determination"])
        if "harmonic" in documents:
            validate_harmonic(documents["determination"], documents["harmonic"])
    except (ContractError, ValueError, OSError) as error:
        parser.exit(1, str(error) + "\n")
    print(json.dumps({"schema": "ql.agent-contract-check/v1", "ok": True,
                      "documents_checked": list(documents), "schema_checked": args.check_schema,
                      "scope": "wire/provenance; kernel admission remains QD1"}))

if __name__ == "__main__":
    main()
