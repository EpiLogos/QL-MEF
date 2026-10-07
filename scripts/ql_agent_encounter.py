"""QL-owned eligibility and projection of bounded body-native occasions.

Explicit QL state/read carriers request a reading. Ordinary body events remain
ineligible. Every accepted formal value is checked and completed by the native
kernel; this adapter contains no QL tables and invokes no decision provider.
"""
import argparse
import hashlib
import json
import sys
from pathlib import Path

import ql_agent_aikit as a
import ql_agent_contracts as c
from ql_agent_decide import native


def encounter(ql, occasion):
    required = {"schema", "kind", "generation", "text", "body_ref", "session_ref", "native_event_ref"}
    c.require(isinstance(occasion, dict) and set(occasion) == required,
              "bounded native occasion fields required")
    c.require(occasion["schema"] == "ql.agent-native-occasion/v1", "native occasion schema")
    c.require(occasion["kind"] in ("input", "tool-result"), "supported native occasion required")
    c.require(type(occasion["generation"]) is int and 0 <= occasion["generation"] < 2**53,
              "bounded native occasion generation required")
    for key in ("text", "body_ref", "session_ref", "native_event_ref"):
        c.require(isinstance(occasion[key], str), "native occasion string required: " + key)
        if key != "text":
            c.require(bool(occasion[key].strip()), "native occasion reference required: " + key)
    c.require(len(occasion["text"].encode("utf-8")) <= 32768, "native occasion material exceeds 32 KiB")
    text = occasion["text"]
    receipt = {"schema": "ql.agent-encounter/v1", "generation": occasion["generation"],
               "native_event_ref": occasion["native_event_ref"], "provider_calls": 0,
               "projector_revision": "sha256:" + hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    # A tool result cannot promote embedded text into an instruction to read it.
    # Typed tool state is already covered by explicit native QL tool receipts.
    if occasion["kind"] != "input" or not text.startswith(("QL state:", "QL read:")):
        return {**receipt, "disposition": "ineligible", "reason": "no explicit QL reading carrier"}
    observed = {}
    heads = []
    if text.startswith("QL state:"):
        observed = a.decode(text.removeprefix("QL state:").strip().encode("utf-8"))
        c.require(isinstance(observed, dict) and bool(observed), "QL state must contain supplied formal fields")
    else:
        c.require(bool(text.removeprefix("QL read:").strip()), "QL reading needs semantic material")
        heads = ["faculty", "operation"]
    revision = "sha256:" + hashlib.sha256(text.encode("utf-8")).hexdigest()
    reference = "ql:material:" + revision
    event = {"schema": "ql.agent-event/v1", "event_ref": "ql:event:" + revision,
             # This content-addressed event is immutable. Body cancellation age
             # stays in the outer receipt and native binding, never its meaning.
             "generation": 0, "occasion_refs": [],
             "source_basis": [{"ref": reference, "revision": revision}],
             "kind": "body-input", "subject": "Explicit QL body encounter",
             "material": {"ref": reference, "revision": revision, "text": text},
             "observed": [{"field": field, "value": value, "origin": "observed", "basis_refs": [reference]}
                          for field, value in sorted(observed.items())],
             "native_state": [], "provenance_refs": ["ql:owner:agent-encounter:" + receipt["projector_revision"]],
             "bindings": {"body_ref": occasion["body_ref"], "session_ref": occasion["session_ref"],
                          "native_event_refs": [occasion["native_event_ref"]]}}
    projection = native(ql, "project", {"event": event, "requested_heads": heads})
    c.require(projection.get("schema") == "ql.agent-projection/v1", "native projection receipt required")
    return {**receipt, "disposition": "projected", "projection": projection}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ql", default="ql-agent")
    args = parser.parse_args()
    raw = sys.stdin.buffer.read(65537)
    c.require(len(raw) <= 65536, "native occasion envelope exceeds 64 KiB")
    print(json.dumps(encounter(args.ql, a.decode(raw)), ensure_ascii=False, separators=(",", ":")))


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError, KeyError, TypeError, c.ContractError) as error:
        print("QL encounter refused: " + str(error), file=sys.stderr)
        raise SystemExit(2)
