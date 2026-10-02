"""Observe native QL label owners and generate portable QD0 wire specimens.

No classifier, kernel arithmetic or inferred semantic label is introduced.
The explicit --output directory owns the generated observation and fixtures.
"""
import argparse
import hashlib
import json
import shutil
import subprocess
import time
from pathlib import Path

import ql_agent_contracts as c

SOURCE = "ql:fixture:agent-decision/v1/semantic-specimen.txt"

def observe(executable, arguments):
    started = time.monotonic()
    argv = [str(executable), *arguments, "--json"]
    result = subprocess.run(argv, capture_output=True, text=True, timeout=30)
    if result.returncode:
        raise RuntimeError(f"{arguments}: {result.stderr or result.stdout}")
    data = json.loads(result.stdout)
    return {"argv": ["ql", *arguments, "--json"], "data": data,
            "elapsed_seconds": time.monotonic() - started, "stdout_digest": c.digest(data)}

def generate(executable, output, basis):
    receipt = {"schema": "ql.agent-label-owner-observation/v1",
               "standing": "actual native label observation; not model or body acceptance",
               "lenses": observe(executable, ["mef", "lenses"]),
               "context_frames": observe(executable, ["context-frame", "list"])}
    binary_hash = hashlib.sha256()
    with executable.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1048576), b""):
            binary_hash.update(chunk)
    receipt["executable_digest"] = "sha256:" + binary_hash.hexdigest()
    material = (basis / "semantic-specimen.txt").read_text(encoding="utf-8")
    event = json.loads((basis / "event-v1.json").read_text())
    event["material"] = {"ref": SOURCE, "revision": "sha256:" + hashlib.sha256(material.encode()).hexdigest(),
                         "text": material}
    event["source_basis"] = [{"ref": SOURCE, "revision": event["material"]["revision"]}]
    frame = json.loads((basis / "decision-frame-v1.json").read_text())
    frame["event_basis_digest"] = c.semantic_basis(event)
    frame["source_basis"] = event["source_basis"]
    frame["kernel_basis"] = {"owner_ref": "ql:owner:mef-lens-registry",
        "revision": receipt["lenses"]["data"]["registryVersion"],
        "digest": receipt["lenses"]["stdout_digest"]}
    frame["unresolved"][0]["labels"] = [{"id": lens["code"], "description": lens["name"],
        "owner_ref": lens["lensRef"], "owner_revision": receipt["lenses"]["data"]["registryVersion"]}
        for lens in receipt["lenses"]["data"]["lenses"]]
    frame["unresolved"][0]["cardinality"]["max"] = len(frame["unresolved"][0]["labels"])
    determination = json.loads((basis / "unavailable-determination-v1.json").read_text())
    determination.update(event_basis_digest=c.semantic_basis(event), frame_digest=c.digest(frame),
        source_basis=frame["source_basis"], kernel_basis=frame["kernel_basis"])
    c.validate_determination(event, frame, determination)
    output.mkdir(parents=True, exist_ok=True)
    for name, document in (("native-label-receipts", receipt), ("event-v1", event),
                           ("decision-frame-v1", frame), ("unavailable-determination-v1", determination)):
        (output / (name + ".json")).write_bytes(c.canonical(document) + b"\n")
    (output / "semantic-specimen.txt").write_text(material, encoding="utf-8")
    return receipt

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ql", default="ql", help="Exact native ql executable to observe")
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--check-labels", action="store_true", help="Refuse owner drift from retained label receipts")
    args = parser.parse_args()
    selected = shutil.which(args.ql)
    if selected is None:
        parser.error("native QL executable unavailable: " + args.ql)
    basis = Path(__file__).resolve().parents[1] / "fixtures/agent-decision/v1"
    try:
        receipt = generate(Path(selected).resolve(), args.output, basis)
        if args.check_labels:
            retained = json.loads((basis / "native-label-receipts.json").read_text())
            for key in ("lenses", "context_frames"):
                c.require(receipt[key]["data"] == retained[key]["data"], "native owner label drift: " + key)
    except (OSError, ValueError, RuntimeError, subprocess.TimeoutExpired) as error:
        parser.exit(1, str(error) + "\n")
    print(json.dumps({"ok": True, "lenses": len(receipt["lenses"]["data"]["lenses"]),
        "context_frames": len(receipt["context_frames"]["data"]["frames"]),
        "executable_digest": receipt["executable_digest"], "output": str(args.output)}))

if __name__ == "__main__":
    main()
