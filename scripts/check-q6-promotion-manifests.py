#!/usr/bin/env python3
"""Retain the Q6 deterministic promotion boundaries in the canonical gate."""
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def load(name: str) -> dict:
    return json.loads((ROOT / "fixtures" / "q6" / name).read_text(encoding="utf-8"))


def main() -> int:
    context = load("context-frame-promotion-v1.json")
    assert context["capability"] == "ql.mef.context-frame"
    assert context["version"] == "1.0.0"
    assert context["status"] == "specified-formal-structure"
    assert context["automaticInvocation"] is False
    assert context["deterministicBoundary"]["canonicalSelection"] == "3 Name -> conjugate-cross -> 4 Power"
    assert context["deterministicBoundary"]["selectedPartition"] == "5 inner-four + 2 outer-two"
    assert context["deterministicBoundary"]["unselectedPartition"] == "3 inner-four hooks + 2 outer-two anchors"
    assert context["deterministicBoundary"]["alternateConjugateSelection"] == "not-promoted"
    assert context["deterministicBoundary"]["modalReanchoring"] == "not-promoted"
    assert context["deterministicBoundary"]["semanticRoleBinding"] == "not-promoted"
    assert context["deterministicBoundary"]["runtimePolicy"] == "not-promoted"
    assert len(context["frames"]) == 7
    assert len(context["promoted"]["canonicalProgression"]) == 7
    assert len(context["promoted"]["complexificationHooks"]) == 3
    assert len(context["promoted"]["unpickedOuterAnchors"]) == 2
    assert context["promoted"]["completeFormAddresses"] == 12
    assert context["provenance"]["factoryProgramme"]["issue"] == 121
    assert context["provenance"]["factoryProgramme"]["crossRepoEvidence"] == "not-required-for-this-capability"

    rotation = load("mef-rotation-promotion-v1.json")
    assert rotation["capability"] == "ql.mef.rotation"
    assert rotation["version"] == "1.0.0"
    assert rotation["status"] == "specified-formal-structure"
    assert rotation["automaticInvocation"] is False
    assert rotation["deterministicBoundary"]["localToAbsolute"] == "(anchor + local) mod 6"
    assert rotation["deterministicBoundary"]["semanticRoleBinding"] == "not-promoted"
    assert rotation["deterministicBoundary"]["runtimePolicy"] == "not-promoted"
    assert rotation["promoted"]["lensAnchors"] == 12
    assert rotation["promoted"]["localPositionsPerLens"] == 6
    assert rotation["promoted"]["manifoldCoordinates"] == 72
    assert rotation["provenance"]["factoryProgramme"]["issue"] == 121
    assert rotation["provenance"]["factoryProgramme"]["crossRepoEvidence"] == "not-required-for-this-capability"

    pairing = load("pairing-promotion-v1.json")
    assert pairing["capability"] == "ql.pairing"
    assert pairing["version"] == "1.0.0"
    assert pairing["status"] == "specified-formal-structure"
    assert pairing["automaticTraversal"] is False
    assert pairing["promoted"]["withinPassFamilies"] == ["A", "B", "C"]
    assert pairing["promoted"]["softwareModulation"] == ["D1", "D2", "D3"]
    assert pairing["promoted"]["squareApparatus"] == {
        "entries": 9,
        "orientedStructures": 8,
        "unorderedAddressTetrads": 7,
    }
    assert "ql.state64.runtime-semantics" in pairing["researchExtensions"]
    assert "ql.epogdoon.retained-difference-metric" in pairing["researchExtensions"]
    assert "ql.topology.control-rules" in pairing["researchExtensions"]
    assert "ql.mef.context-operational-roles" in pairing["researchExtensions"]
    assert pairing["provenance"]["factoryHead"] == "a654c62f68b82236061986d9215b23257fe53b17"
    assert pairing["provenance"]["factoryPairingGrammarBlob"] == "0d6aa49197dd4d06646ea5a5fb094c03b10a74e4"

    print("Q6 promotion manifests: PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
