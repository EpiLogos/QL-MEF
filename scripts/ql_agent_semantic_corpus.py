"""Freeze independently reviewed semantic cases against the executable QL grammar.

Gold is reviewed source material, never classifier output. Native admission is
used only to obtain the owner's exact operative representation and completion.
Verifier answers and reviewed alternatives stay outside provider input.
"""
import argparse
import hashlib
import json
import shutil
from pathlib import Path

import ql_agent_contracts as c
import ql_agent_corpus as corpus
import ql_agent_evaluation as e

ROOT = Path(__file__).resolve().parents[1]


def generate(args):
    executable = shutil.which(args.ql)
    c.require(executable is not None, "actual native QL executable required")
    executable = Path(executable).resolve()
    reviewed = json.loads(args.reviewed.read_text())
    c.require(reviewed["schema"] == "ql.reviewed-semantic-specimens/v1", "reviewed specimen schema")
    paths = {"ql": executable, "generator": Path(__file__).resolve(), "reviewed": args.reviewed,
             "contracts": Path(c.__file__), "evaluation": Path(e.__file__)}
    for row in reviewed["cases"]:
        for ref in row["owner_basis"]:
            path = ROOT / ref["path"]
            paths[ref["path"]] = path
            if path.suffix == ".json":
                owner = json.loads(path.read_text())
                c.require(any(cap["capability_ref"] == ref["capability_ref"] for cap in owner["capabilities"]),
                          "reviewed capability no longer belongs to the current owner")
    hashes = {name: corpus.file_digest(path) for name, path in paths.items()}
    args.output.mkdir(parents=True, exist_ok=False)
    cases, readings = [], []
    kernel = None
    for row in reviewed["cases"]:
        revision = "sha256:" + hashlib.sha256(row["text"].encode()).hexdigest()
        reference = "ql:reviewed-semantic:v1:" + revision[7:23]
        request = {"event": {"schema":"ql.agent-event/v1", "event_ref":"ql:event:" + revision[7:23],
            "generation":1,"occasion_refs":[],"kind":"controlled-semantic-concern",
            "subject":"Source-grounded agent concern",
            "material":{"ref":reference,"revision":revision,"text":row["text"]},
            "source_basis":[{"ref":reference,"revision":revision}],"observed":[],"native_state":[],
            "provenance_refs":[reviewed["source_issue"]]}, "requested_heads":list(row["expected_fields"])}
        projection, _ = e.invoke([str(executable),"agent-event","project","-","--json"],request,args.timeout)
        e.check_projection(projection)
        if kernel is None:
            kernel = projection["frame"]["kernel_basis"]
        c.require(kernel == projection["frame"]["kernel_basis"], "kernel changed during generation")
        heads = {h["id"]: row["expected_fields"][h["field"]]
                 for h in projection["frame"]["unresolved"] if h["id"] in projection["decision_head_ids"]}
        c.require(len(heads) == len(row["expected_fields"]), "reviewed heads changed eligibility")
        response = {"schema":"ql.agent-decision-response/v1",
            "event_basis_digest":projection["frame"]["event_basis_digest"],
            "frame_digest":c.digest(projection["frame"]),"kernel_basis":kernel,"outcome":"answered",
            "provider":{"provider_ref":"ql:reviewed-gold-codec","model_ref":"ql:controlled-review-labels",
              "model_revision":hashes["reviewed"],"runtime_revision":hashes["generator"]},
            "proposals":[{"head_id":head,"label_ids":labels,"spans":[]} for head,labels in heads.items()]}
        admitted, _ = e.invoke([str(executable),"agent-event","validate","-","--json"],
                              {"projection":request,"response":response},args.timeout)
        c.require(admitted["admission_status"] not in ("stale","refused"), "reviewed mapping failed native admission")
        gold = admitted["projection"]
        expected = {"fields":e.operative_fields(gold["determination"]),"heads":heads,
            "status":"determined" if all(heads.values()) else "unresolved",
            "basis_refs":[hashes["reviewed"],kernel["digest"]]}
        c.require(gold["determination"]["status"] == expected["status"], "unexpected native completion")
        e.check_expectation(projection, expected)
        cases.append({"schema":"ql.agent-evaluation-case/v1","id":row["id"],"family":row["family"],
                      "split":row["split"],"projection":request,"expected":expected})
        readings.append({"id":row["id"],"projection":projection,"reviewed_native_completion":gold,
                         "reviewed_alternatives":row.get("reviewed_alternatives"),"owner_basis":row["owner_basis"]})
    c.require(all(corpus.file_digest(path) == hashes[name] for name,path in paths.items()),
              "review/source/executable changed during generation")
    for name,rows in (("suite.jsonl",cases),("native-readings.jsonl",readings)):
        with (args.output/name).open("xb") as stream:
            for row in rows:
                stream.write(c.canonical(row)+b"\n")
    digest = corpus.file_digest(args.output/"suite.jsonl")
    e.load_suite(args.output/"suite.jsonl",digest)
    manifest = {"schema":"ql.reviewed-semantic-corpus/v1","cases":len(cases),"suite_digest":digest,
                "kernel_basis":kernel,"source_digests":hashes,"reviewer_ref":reviewed["reviewer_ref"],
                "split_policy":"whole concern families; split is explicitly supplied by the reviewed specimen source",
                "provider_calls":0,"standing":reviewed["standing"],
                "not_covered":["semantic training data","actual agent traces","faculty multi-label ambiguity",
                               "operation execution success","full grammar coverage"]}
    corpus.publish(args.output/"manifest.json",manifest)
    return manifest


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ql",default="ql-agent")
    parser.add_argument("--reviewed",type=Path,default=ROOT/"fixtures/agent-decision/v1/semantic-reviewed.json")
    parser.add_argument("--output",type=Path,required=True)
    parser.add_argument("--timeout",type=float,default=5)
    args = parser.parse_args()
    c.require(0 < args.timeout <= 30,"bounded native timeout required")
    result = generate(args)
    print(json.dumps({key:result[key] for key in ("cases","suite_digest","provider_calls")}))


if __name__ == "__main__":
    main()
