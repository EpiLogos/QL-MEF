#!/usr/bin/env python3
"""Execute the pinned original graph-schema's own tables, without editing it.

The output is a source recovery receipt, NOT a new canonical C registry. Compile
in a disposable consumer workspace and retain the original crate, all property
owners/types/cardinalities/disclosures, relation types and construction law.
"""
from __future__ import annotations
import argparse
import csv
import hashlib
import json
import subprocess
import shutil
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PATH = "Body/S/S2/graph-schema/src/lib.rs"
CONSUMER = r'''
use epi_s2_graph_schema::*;
use serde_json::{json,Value};
fn property(p:&GraphPropertySpec)->Value {json!({"key":p.key,"coordinate_home":p.coordinate_home,"owner":format!("{:?}",p.owner),"value_type":format!("{:?}",p.value_type),"cardinality":format!("{:?}",p.cardinality),"disclosure":format!("{:?}",p.disclosure),"source_family":p.source_family,"indexed":p.indexed,"compatibility":p.compatibility})}
fn main(){println!("{}",serde_json::to_string(&json!({"schema_version":SCHEMA_VERSION,"q_schema_version":Q_SCHEMA_VERSION,"node_properties":NODE_PROPERTY_SPECS.iter().map(property).collect::<Vec<_>>(),"relationship_properties":RELATIONSHIP_PROPERTY_SPECS.iter().map(property).collect::<Vec<_>>(),"semantic_registry":coordinate_semantic_registry(),"relationship_types":RELATIONSHIP_TYPE_SPECS.iter().map(|r|json!({"rel_type":r.rel_type,"coordinate_home":r.coordinate_home,"source_family":r.source_family,"compatibility":r.compatibility})).collect::<Vec<_>>() })).unwrap());}
'''

def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--original", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--offline", action="store_true")
    args = parser.parse_args()
    manifest = json.loads((ROOT / "docs/integrations/epi-logos/agent-world-disposition.json").read_text())
    with (ROOT / manifest["source_inventory"]).open(newline="") as stream:
        source = next(row for row in csv.DictReader(stream, delimiter="\t")
                      if row["owner"] == "original" and row["path"] == PATH)
    data = (args.original / PATH).read_bytes()
    blob = hashlib.sha1(f"blob {len(data)}\0".encode() + data).hexdigest()
    if blob != source["git_blob"]:
        raise SystemExit("original graph-schema source changed; reconcile its source revision first")
    with tempfile.TemporaryDirectory(prefix="aw0-property-consumer-") as directory:
        root = Path(directory)
        (root / "src").mkdir()
        (root / "src/main.rs").write_text(CONSUMER)
        native = root / "native-graph-schema"
        shutil.copytree((args.original / PATH).resolve().parent.parent, native,
                        ignore=shutil.ignore_patterns("target", ".git"))
        dependency = json.dumps(str(native))
        (root / "Cargo.toml").write_text(
            '[package]\nname="aw-property-source-audit"\nversion="0.1.0"\nedition="2021"\n'
            '[workspace]\n[dependencies]\nserde_json="1"\n'
            f'epi-s2-graph-schema={{path={dependency}}}\n')
        command = ["cargo", "run", "--quiet", "--manifest-path", str(root / "Cargo.toml")]
        if args.offline:
            command.append("--offline")
        result = json.loads(subprocess.check_output(command, timeout=300))
        lock_digest = hashlib.sha256((root / "Cargo.lock").read_bytes()).hexdigest()
    receipt = {"contract": "epi.original-property-source/v1",
               "source": {**source, **manifest["repositories"]["original"]},
               "toolchain": subprocess.check_output(["rustc", "--version"], text=True).strip(),
               "consumer_lock_sha256": lock_digest, "vocabulary": result,
               "vocabulary_sha256": hashlib.sha256(json.dumps(result, sort_keys=True, separators=(",", ":")).encode()).hexdigest(),
               "standing": "EXECUTED-ORIGINAL-SOURCE-TABLES; NOT C PROMOTION OR LIVE GRAPH ACCEPTANCE"}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(receipt, ensure_ascii=False, indent=2) + "\n")
    print(json.dumps({"node_properties":len(result["node_properties"]),
                      "relationship_properties":len(result["relationship_properties"]),
                      "relationship_types":len(result["relationship_types"]),
                      "vocabulary_sha256":receipt["vocabulary_sha256"]}))

if __name__ == "__main__":
    main()
