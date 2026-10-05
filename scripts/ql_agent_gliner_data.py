"""Pack native reviewed faculty cases for the pinned GLiNER training protocol.

QL owns the frame and supervised mapping. AIKit owns the provider translation.
Verifier fields never enter classifier text. Test cases cannot become training
examples through this command.
"""
import argparse
import importlib.util
import json
from pathlib import Path

import ql_agent_aikit as adapter
import ql_agent_contracts as c
import ql_agent_corpus as corpus
import ql_agent_evaluation as evaluation


def generate(args):
    cases, digest = evaluation.load_suite(args.suite, args.suite_digest)
    spec = importlib.util.spec_from_file_location("aikit_gliner_packing", args.provider_adapter)
    provider = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(provider)
    args.output.mkdir(parents=True, exist_ok=False)
    data, refs = {"train":[], "validation":[]}, {"train":[], "validation":[]}
    for row in cases:
        c.require(row["split"] in data, "held-out test cannot enter the training pack")
        projection, _ = evaluation.invoke([args.ql,"agent-event","project","-","--json"], row["projection"],5)
        evaluation.check_projection(projection)
        basis = {key:projection[key] for key in ("schema","event","frame","decision_head_ids")}
        request, bindings = adapter.translate_request(basis, args.model, args.schema_profile)
        c.require(len(bindings) == 1 and next(iter(bindings.values())).get("kind") == "choice",
                  "first training candidate requires one native single-cardinality head")
        text, tasks, question_bindings = provider.plan_request(request,args.model,args.label_rendering)
        task_id, task = next(iter(tasks.items()))
        head = bindings[question_bindings[task_id]]["head_id"]
        gold = row["expected"]["heads"][head]
        c.require(len(gold) <= 1, "single-cardinality gold mapping required")
        selected = gold[0] if gold else adapter.ABSTAIN
        # SDK Classification is single-label softmax. The inference carrier's
        # multi_label flag returns all probabilities; its class_act is softmax.
        example = {"input":text,"output":{"classifications":[{"task":task_id,
            "labels":list(task["labels"]),"label_descriptions":task["labels"],"true_label":[selected]}]}}
        data[row["split"]].append(example)
        refs[row["split"]].append({"id":row["id"],"family":row["family"],
                                  "event_basis_digest":projection["frame"]["event_basis_digest"]})
    c.require(all(data.values()), "nonempty training and validation packs required")
    files = {}
    for split, rows in data.items():
        path = args.output/(split+".jsonl")
        with path.open("xb") as stream:
            for row in rows:stream.write(c.canonical(row)+b"\n")
        files[split] = {"path":str(path),"sha256":corpus.file_digest(path),"cases":len(rows)}
    manifest = {"schema":"ql.gliner-training-pack/v1","suite_digest":digest,"files":files,
        "families":refs,"source_digests":{"generator":corpus.file_digest(Path(__file__)),
            "ql_adapter":corpus.file_digest(Path(adapter.__file__)),
            "aikit_adapter":corpus.file_digest(args.provider_adapter)},
        "schema_profile":args.schema_profile,"label_rendering":args.label_rendering,
        "held_out_test_included":False,"provider_calls":0}
    corpus.publish(args.output/"manifest.json",manifest)
    return manifest


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--suite",type=Path,required=True)
    parser.add_argument("--suite-digest",required=True)
    parser.add_argument("--provider-adapter",type=Path,required=True)
    parser.add_argument("--model",required=True)
    parser.add_argument("--schema-profile",choices=("choice","semantic-choice","semantic-packed"),default="semantic-choice")
    parser.add_argument("--label-rendering",choices=("canonical","criteria"),default="canonical")
    parser.add_argument("--ql",default="ql-agent")
    parser.add_argument("--output",type=Path,required=True)
    result=generate(parser.parse_args())
    print(json.dumps({"files":result["files"],"held_out_test_included":False}))


if __name__ == "__main__":main()
