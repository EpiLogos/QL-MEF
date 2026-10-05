"""Generate reviewed faculty training families without changing the frozen test.

Only independently reviewed narrow owner purposes become semantic seeds. All
wording from a capability stays in one split. Gold capability refs, faculty
markers and owning filenames never enter the acting event. Native QL supplies
the legal field and admits each supervised mapping before publication.
"""
import argparse
import json
from pathlib import Path
from types import SimpleNamespace

import ql_agent_contracts as c
import ql_agent_corpus as corpus
import ql_agent_semantic_corpus as semantic

ROOT = Path(__file__).resolve().parents[1]
BASE = ROOT / "fixtures/agent-decision/v1"
TEMPLATES = ("{purpose}", "The concern is: {purpose}.",
             "Find the relevant faculty for this concern: {purpose}.",
             "{purpose}. Keep the source basis and request required inputs before execution.")


def generate(args):
    seeds = json.loads(args.seeds.read_text())
    heldout = json.loads(args.heldout_review.read_text())
    input_hashes = {str(path):corpus.file_digest(path) for path in (args.seeds,args.heldout_review)}
    negative_path = getattr(args, "negative_review", None)
    negatives = []
    if negative_path is not None:
        negative_review = json.loads(negative_path.read_text())
        c.require(negative_review["schema"] == "ql.reviewed-semantic-specimens/v1"
                  and negative_review["reviewer_ref"] == seeds["reviewer_ref"]
                  and negative_review["source_issue"] == seeds["source_issue"],
                  "independently reviewed negative mapping required")
        frozen_text = {" ".join(row["text"].casefold().split()) for row in heldout["cases"]}
        frozen_family = {row["family"] for row in heldout["cases"]}
        negatives = negative_review["cases"]
        c.require(negatives and all(row["expected_fields"] == {"faculty": []}
                  and row["split"] in ("train", "validation") and row["family"] not in frozen_family
                  and " ".join(row["text"].casefold().split()) not in frozen_text for row in negatives),
                  "negative training data overlaps frozen test or lacks explicit abstention standing")
        input_hashes[str(negative_path)] = corpus.file_digest(negative_path)
    c.require(seeds["schema"] == "ql.reviewed-semantic-training-seeds/v1", "reviewed seed contract required")
    excluded = {basis.get("capability_ref") for row in heldout["cases"] for basis in row["owner_basis"]}
    approved = set(seeds["capability_refs"])
    validation = set(seeds["validation_capability_refs"])
    c.require(validation <= approved, "validation family lacks reviewed standing")
    args.output.mkdir(parents=True, exist_ok=False)
    rows, found, ownership = [], set(), {}
    for path in sorted((ROOT / "docs/integrations/epi-logos").glob("epi-m-capability-field-m*.json")):
        owner = json.loads(path.read_text())
        domain = owner["m"]
        c.require(domain in {"M" + str(i) for i in range(6)}, "current source faculty identity required")
        for capability in owner["capabilities"]:
            reference = capability["capability_ref"]
            if reference not in approved:
                continue
            c.require(reference not in found, "duplicate owner capability")
            found.add(reference)
            if reference in excluded:
                continue
            purpose = capability["for_what"]
            c.require(isinstance(purpose, str) and purpose.strip(), "exact owner purpose required")
            c.require(reference not in purpose and domain not in purpose, "gold owner marker in semantic material")
            split = "validation" if reference in validation else "train"
            ownership[reference] = {"path":path.relative_to(ROOT).as_posix(),
                                    "revision":corpus.file_digest(path), "split":split}
            for index, template in enumerate(TEMPLATES):
                rows.append({"id":f"seed-{reference.lower()}-{index}", "family":"capability:" + reference,
                    "split":split, "text":template.format(purpose=purpose),
                    "expected_fields":{"faculty":["#" + domain[1:]]},
                    "owner_basis":[{"path":path.relative_to(ROOT).as_posix(), "capability_ref":reference}]})
    c.require(found == approved, "reviewed capabilities missing from current Source")
    rows.extend(negatives)
    c.require(rows and any(row["split"] == "train" for row in rows)
              and any(row["split"] == "validation" for row in rows), "nonempty disjoint train/validation required")
    reviewed = {"schema":"ql.reviewed-semantic-specimens/v1", "reviewer_ref":seeds["reviewer_ref"],
                "source_issue":seeds["source_issue"], "standing":seeds["standing"], "cases":rows}
    review_path = args.output / "generated-reviewed-seeds.json"
    corpus.publish(review_path, reviewed)
    result = semantic.generate(SimpleNamespace(ql=args.ql, reviewed=review_path,
                                             output=args.output/"native", timeout=args.timeout))
    result.update(split_policy="whole capability/template families; frozen test capabilities excluded",
                  not_covered=["full semantic grammar", "actual agent traces", "independent unseen paraphrases"],
                  generator_digest=corpus.file_digest(Path(__file__)), seed_digest=corpus.file_digest(args.seeds),
                  heldout_review_digest=corpus.file_digest(args.heldout_review), ownership=ownership,
                  excluded_capabilities=sorted(excluded & approved),
                  hard_negative_cases=len(negatives),
                  negative_review_digest=corpus.file_digest(negative_path) if negative_path else None,
                  counts={split:sum(row["split"]==split for row in rows) for split in ("train","validation")})
    c.require(all(corpus.file_digest(Path(path)) == revision for path,revision in input_hashes.items())
              and all(corpus.file_digest(ROOT/owner["path"]) == owner["revision"] for owner in ownership.values()),
              "reviewed seed/held-out/source basis changed during generation")
    corpus.publish(args.output/"manifest.json", result)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ql",default="ql-agent")
    parser.add_argument("--seeds",type=Path,default=BASE/"semantic-training-seeds.json")
    parser.add_argument("--heldout-review",type=Path,default=BASE/"semantic-reviewed.json")
    parser.add_argument("--negative-review",type=Path,
                        help="Explicit independently reviewed whole-family negative specimens; frozen test never enters training")
    parser.add_argument("--output",type=Path,required=True)
    parser.add_argument("--timeout",type=float,default=5)
    args = parser.parse_args()
    c.require(0 < args.timeout <= 30, "bounded native timeout required")
    result = generate(args)
    print(json.dumps({key:result[key] for key in ("counts","suite_digest","excluded_capabilities")}))


if __name__ == "__main__":
    main()
