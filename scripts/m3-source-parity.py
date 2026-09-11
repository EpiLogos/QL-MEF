#!/usr/bin/env python3
"""Lossless, source-locked M3 binding projection and independent semantic audit.

K7 support for the native engine, not a second M registry or an accepted K4 census.
The complete payload and every duplicate/qualified edge are retained in generated
output. The small checked-in fixture locks that output without copying the graph.
Source assertions and derived addresses never overwrite one another. Findings use
the existing K3 discrepancy vocabulary and remain OPEN until ledger reconciliation.
"""
from __future__ import annotations

import argparse
from collections import Counter, defaultdict
import hashlib
import json
from pathlib import Path
import re
import sys
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
DATA = "Idea/Bimba/Map/datasets/mahamaya-deep/"
REGISTRY = "fixtures/kernel/m-tree-v1.json"
LOCK = "fixtures/kernel/m3-source-bindings-v1.json"
SCHEMA = "ql.m3-source-bindings/v1"
VALUES = dict(zip("ATCG", (6, 9, 8, 7), strict=True))
# M3-COIN-1 is an existing owner-ratified correction, not a new source inference.
COIN_AUTHORITY = "vendor/epi-kernel/corrections/M3-COIN-1.patch"


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def canonical(value: Any) -> bytes:
    return json.dumps(value, ensure_ascii=False, sort_keys=True,
                      separators=(",", ":")).encode("utf-8")


def digest(value: Any) -> str:
    return hashlib.sha256(canonical(value)).hexdigest()


def load_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8-sig"))


def render(value: Any) -> str:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, indent=2) + "\n"


def role(ref: str, props: dict) -> str:
    """Roles select source records; they neither create coordinates nor assert parity."""
    patterns = (
        (r"#3-2-[1-4]", "nucleotide"),
        (r"#3-2-[1-4]-[1-4]", "dinucleotide"),
        (r"#3-2-[1-4]-[1-4]-[1-4]", "dna-codon"),
        (r"#3-1-[0-7]", "trigram"),
        (r"#3-1-[0-7]-[0-7]", "hexagram"),
        (r"#3-3-2-[0-2]-[0-9]+", "matrix-cell"),
        (r"#3-4\.0-[1-4]-[0-9]+", "phase-codon"),
        (r"#3-4-[1-4]-[0-9]+", "minor-arcana"),
        (r"#3-4-5/0-[0-9]+", "major-arcana"),
        (r"#3-3-4-[0-9]+", "amino-or-translation-signal"),
        (r"#3-5-[1-4]-[0-5]", "clock-backbone"),
    )
    for pattern, label in patterns:
        if re.fullmatch(pattern, ref):
            return label
    if ref.startswith("#3-3-3-") and re.fullmatch(r"[AUCG]{3}", props.get("sequence", "")):
        return "rna-codon"
    if ref.startswith("#3-5-5/0-") and "degree" in props:
        return "clock-degree"
    if ref.startswith("#3-3-5"):
        return "karyotype"
    return "source-coordinate"  # Retain the whole remaining field, not just numeric roles.


def read_source(source_root: Path, repo_root: Path = ROOT) -> tuple[dict, list, list]:
    registry = load_json(repo_root / REGISTRY)
    require(registry["schema"] == "ql.m-tree/v1", "unsupported shared registry")
    payloads = []
    for filename in ("nodes-full-detail.json", "relations.json"):
        relative = DATA + filename
        expected = next((f for f in registry["files"] if f["path"] == relative), None)
        require(expected is not None, "source absent from K2 file lock: " + relative)
        raw = (source_root / relative).read_bytes()
        require(len(raw) == expected["bytes"], "source byte length drift: " + relative)
        require(hashlib.sha256(raw).hexdigest() == expected["sha256"], "source SHA-256 drift: " + relative)
        blob = hashlib.sha1(b"blob " + str(len(raw)).encode() + b"\0" + raw).hexdigest()
        require(blob == expected["git_blob"], "source Git blob drift: " + relative)
        data = json.loads(raw.decode("utf-8-sig"))
        require(isinstance(data, list), "expected source record array: " + relative)
        payloads.append(data)
    return registry, payloads[0], payloads[1]


def project(registry: dict, raw_nodes: list, raw_edges: list) -> dict:
    """Join full source records onto K2 IDs, checking every record and directed edge."""
    require(digest({k: v for k, v in registry.items() if k != "registry_revision"}) == registry["registry_revision"],
            "registry revision/content disagreement")
    nodes_by_ref = {n["source_ref"]: n for n in registry["nodes"]}
    require(len(nodes_by_ref) == len(registry["nodes"]), "duplicate registry coordinate")
    files = {f["path"]: i for i, f in enumerate(registry["files"])}
    node_file, edge_file = (files[DATA + f] for f in ("nodes-full-detail.json", "relations.json"))
    records = registry["records"]
    source_nodes = {n["source_ref"]: n for n in registry["nodes"]
                    if any(records[i]["file"] == node_file for i in n["records"])}
    source_edges = {records[e["record"]]["record_index"]: e for e in registry["relations"]
                    if records[e["record"]]["file"] == edge_file}
    require(len(source_edges) == len(raw_edges), "source relation count drift")
    require(set(source_edges) == set(range(len(raw_edges))), "source relation index gap")
    seen = set()
    nodes = []
    for index, raw in enumerate(raw_nodes):
        require(set(raw) == {"coordinate", "filteredProps"}, "unclassified source node wrapper")
        ref = raw["coordinate"]
        require(ref not in seen, "duplicate source coordinate: " + ref)
        seen.add(ref)
        require(ref in source_nodes, "coordinate absent from K2 source: " + ref)
        node = source_nodes[ref]
        bindings = [i for i in node["records"] if records[i]["file"] == node_file]
        require(len(bindings) == 1, "ambiguous node source record: " + ref)
        record = records[bindings[0]]
        require(record["record_index"] == index, "node source order drift: " + ref)
        require(record["payload_sha256"] == digest(raw), "node payload drift: " + ref)
        props = raw["filteredProps"]
        require(props.get("bimbaCoordinate", ref) == ref, "source self-coordinate disagreement: " + ref)
        nodes.append({"id": node["id"], "ref": ref, "record": bindings[0],
                      "source_record_index": index, "role": role(ref, props),
                      "parent_id": node["parent_id"], "properties": props})
    require(seen == set(source_nodes), "source node coverage drift")
    require(seen == {n["source_ref"] for n in registry["nodes"] if n["root_position"] == 3},
            "M3 registry field is not exhausted by this source")
    edges = []
    for index, raw in enumerate(raw_edges):
        require(set(raw) == {"source", "target", "relType", "relProperties"}, "unclassified source relation wrapper")
        known = source_edges[index]
        record = records[known["record"]]
        require((raw["source"], raw["relType"], raw["target"]) ==
                (known["from_ref"], known["source_kind"], known["to_ref"]),
                f"relation endpoint/type drift at source record {index}")
        require(digest(raw["relProperties"]) == record["payload_sha256"],
                f"relation property drift at source record {index}")
        edges.append({"id": known["id"], "ref": known["relation_ref"],
                      "record": known["record"], "source_record_index": index,
                      "from_ref": raw["source"], "to_ref": raw["target"],
                      "from_id": known["from_id"], "to_id": known["to_id"],
                      "kind": raw["relType"], "properties": raw["relProperties"]})
    return {"schema": SCHEMA, "registry_revision": registry["registry_revision"],
            "source_revision": registry["source_revision"],
            "source_repository": registry["source_repository"],
            "files": [registry["files"][i] for i in (node_file, edge_file)],
            "nodes": nodes, "relations": edges}


class Audit:
    def __init__(self, projection: dict):
        self.projection = projection
        self.nodes = {n["ref"]: n for n in projection["nodes"]}
        require(len(self.nodes) == len(projection["nodes"]), "duplicate projected coordinate")
        self.out: dict[tuple, list] = defaultdict(list)
        self.groups: dict[str, list] = defaultdict(list)
        self.findings: list[dict] = []
        for node in projection["nodes"]:
            self.groups[node["role"]].append(node)
        edge_ids = [e["id"] for e in projection["relations"]]
        require(len(set(edge_ids)) == len(edge_ids), "duplicate projected relation ID")
        for edge in projection["relations"]:
            self.out[edge["from_ref"], edge["kind"]].append(edge)

    def edges(self, ref: str, kind: str) -> list:
        return self.out.get((ref, kind), [])

    def one(self, ref: str, kind: str) -> dict:
        found = self.edges(ref, kind)
        require(len(found) == 1, f"expected one {kind} at {ref}, found {len(found)}")
        return found[0]

    def finding(self, code: str, subjects: list[str], axis: str,
                reference: str, detail: dict, authority: str, peer: str = "c", authority_peer: str | None = None) -> None:
        identity = "k7-source-" + code + "-" + digest([reference, detail])[:16]
        self.findings.append({
            "id": identity, "subjects": subjects, "axis": axis,
            "from_peer": "bimba", "to_peer": peer, "state": "open",
            "detail": canonical({"code": code, "reference": reference, **detail}).decode(),
            "current_authority": {"peer": authority_peer or peer, "reference": authority,
                                  "reason": "Retain the source assertion and the separately warranted native law; no source mutation or acceptance is inferred."},
            "proposal": None, "decision": None, "promotion": None,
            "history": [{"state": "open", "reference": reference}],
        })

    def hexagrams(self) -> list:
        trigrams = {n["ref"]: int(n["properties"]["binaryRepresentation"], 2)
                    for n in self.groups["trigram"]}
        require(len(trigrams) == 8 and set(trigrams.values()) == set(range(8)), "incomplete trigram field")
        result = []
        for node in self.groups["hexagram"]:
            ref, props = node["ref"], node["properties"]
            upper, lower = self.one(ref, "HAS_UPPER_Trigram"), self.one(ref, "HAS_LOWER_Trigram")
            require(upper["to_ref"] in trigrams and lower["to_ref"] in trigrams,
                    "hexagram has a non-trigram endpoint: " + ref)
            address = (trigrams[upper["to_ref"]] << 3) | trigrams[lower["to_ref"]]
            declared = int(props["binaryCode"], 2)
            row = {"ref": ref, "id": node["id"], "king_wen": props["number"],
                   "source_binary_code": declared, "trigram_derived_address": address,
                   "upper_relation": upper["id"], "lower_relation": lower["id"]}
            if declared != address:
                self.finding("hexagram-code", ["deep-M3:M3-C05", "deep-M3:M3-C07"], "coordinate", ref,
                             {"source_binary_code": declared, "trigram_derived_address": address},
                             "crates/ql-core/src/pole/iching.rs")
            result.append(row)
        by_ref = {r["ref"]: r for r in result}
        require(len(result) == 64 and {r["trigram_derived_address"] for r in result} == set(range(64)),
                "trigram relations do not produce a complete 64-address field")
        require({r["king_wen"] for r in result} == set(range(1, 65)), "King Wen ordinal coverage drift")
        by_address = {r["trigram_derived_address"]: r["ref"] for r in result}
        for row in result:
            props = self.nodes[row["ref"]]["properties"]
            address = row["trigram_derived_address"]
            expected_nuclear = {"upperNuclearBinary": format((address >> 2) & 7, "03b"),
                                "lowerNuclearBinary": format((address >> 1) & 7, "03b"),
                                "nuclearCoordinate": by_address[(((address >> 2) & 7) << 3) | ((address >> 1) & 7)]}
            differences = {k: {"source": props.get(k), "native": v} for k, v in expected_nuclear.items() if props.get(k) != v}
            if differences:
                self.finding("nuclear-register", ["deep-M3:M3-C07"], "relation", row["ref"],
                             {"differences": differences}, "crates/ql-core/src/pole/iching.rs", peer="rust")
            changes = self.edges(row["ref"], "LINE_CHANGE")
            require(len(changes) == 6 and {e["properties"].get("line") for e in changes} == set(range(1, 7)),
                    "incomplete/duplicate six-line relation field: " + row["ref"])
            for edge in changes:
                require(edge["to_ref"] in by_ref, "LINE_CHANGE has non-hexagram endpoint")
                line = edge["properties"]["line"]
                expected = row["trigram_derived_address"] ^ (1 << (line - 1))
                actual = by_ref[edge["to_ref"]]["trigram_derived_address"]
                if actual != expected:
                    self.finding("line-change", ["deep-M3:M3-C06"], "relation", edge["ref"],
                                 {"source": row["ref"], "line": line, "source_target": edge["to_ref"],
                                  "source_target_address": actual, "xor_target": by_address[expected],
                                  "xor_target_address": expected}, "crates/ql-core/src/pole/iching.rs")
        return sorted(result, key=lambda row: row["trigram_derived_address"])

    def matrices(self, hexagrams: list) -> list:
        by_ref = {r["ref"]: r for r in hexagrams}
        result = []
        for node in self.groups["matrix-cell"]:
            ref = node["ref"]
            resolved = self.one(ref, "RESOLVES_TO")
            require(resolved["to_ref"] in by_ref, "matrix does not resolve a hexagram: " + ref)
            uses, yields = self.edges(ref, "USES_Pair"), self.edges(ref, "YIELDS_CODON")
            require({e["properties"].get("type") for e in yields} == {"positive", "negative"}
                    and len(yields) == 2, "matrix codon valences incomplete: " + ref)
            roles = {e["properties"].get("role") for e in uses}
            if roles != {"upper", "lower"}:
                self.finding("matrix-missing-pair-role", ["deep-M3:M3-C09"], "relation", ref,
                             {"roles": sorted(roles), "expected_roles": ["lower", "upper"]},
                             REGISTRY, authority_peer="bimba")
            for edge in uses + yields:
                want = "dinucleotide" if edge["kind"] == "USES_Pair" else "dna-codon"
                if edge["to_ref"] is None:
                    self.finding("matrix-unresolved-endpoint", ["deep-M3:M3-C09"], "relation", edge["ref"],
                                 {"source": ref, "kind": edge["kind"], "role": edge["properties"],
                                  "source_target": None}, REGISTRY, authority_peer="bimba")
                else:
                    require(edge["to_ref"] in self.nodes and self.nodes[edge["to_ref"]]["role"] == want,
                            "matrix pair/codon endpoint has wrong role: " + ref)
            # A source cell may assert more than two pair edges. Retain them ALL.
            result.append({"ref": ref, "id": node["id"], "family": int(ref.split("-")[3]),
                           "hexagram_ref": resolved["to_ref"], "resolves_relation": resolved["id"],
                           "address": by_ref[resolved["to_ref"]]["trigram_derived_address"],
                           "pair_relations": [e["id"] for e in uses],
                           "codon_relations": [e["id"] for e in yields]})
        sizes = Counter(row["family"] for row in result)
        require(sizes == {0: 64, 1: 64, 2: 56}, "three matrix field cardinality drift")
        for family in range(3):
            addresses = [r["address"] for r in result if r["family"] == family]
            require(len(set(addresses)) == len(addresses), "duplicate resolved address in matrix family")
        source_gaps = sorted(set(range(64)) - {r["address"] for r in result if r["family"] == 2})
        native_gaps = [5, 21, 26, 34, 42, 53, 58, 61]
        if source_gaps != native_gaps:
            self.finding("resonance-admissibility", ["deep-M3:M3-C09", "deep-M3:M3-C16"], "operational", "#3-3-2-2",
                         {"source_absent_hexagram_addresses": source_gaps, "native_gap_addresses": native_gaps,
                          "note": "The native header calls these hexagram gap positions. Equal cardinality is not address parity."},
                         "crates/ql-core/src/pole/fold.rs", peer="rust")
        return sorted(result, key=lambda r: (r["family"], r["address"]))

    def genetics(self) -> dict:
        dna = {n["properties"]["sequence"]: n for n in self.groups["dna-codon"]}
        require(len(dna) == 64 and all(re.fullmatch(r"[ATCG]{3}", s) for s in dna), "incomplete DNA codon field")
        rna = {n["properties"]["sequence"]: n for n in self.groups["rna-codon"]}
        require(len(rna) == 37, "RNA field cardinality drift")
        transcription = []
        for sequence, node in sorted(dna.items()):
            edges = self.edges(node["ref"], "TRANSCRIBES_TO")
            if "T" not in sequence:
                require(not edges, "shared codon unexpectedly has a distinct RNA endpoint")
                continue
            require(len(edges) == 1, "T-containing codon lacks unique RNA transcription: " + sequence)
            expected = sequence.replace("T", "U")
            require(expected in rna and edges[0]["to_ref"] == rna[expected]["ref"],
                    "RNA relation is not exact T-to-U transcription: " + sequence)
            transcription.append({"dna_ref": node["ref"], "rna_ref": rna[expected]["ref"],
                                  "dna": sequence, "rna": expected, "relation_id": edges[0]["id"]})
        require({r["rna_ref"] for r in transcription} == {n["ref"] for n in rna.values()}, "orphan RNA codon")
        phase = []
        for node in self.groups["phase-codon"]:
            ref, props = node["ref"], node["properties"]
            sequence = props["sequence"]
            require(sequence in dna, "phase codon sequence absent from DNA field")
            reflection = self.one(ref, "REFLECTS_DNA_FORM")
            require(reflection["to_ref"] == dna[sequence]["ref"], "phase/DNA reflection drift: " + ref)
            card = self.one(ref, "GOVERNS_TAROT_EXPRESSION")
            require(self.nodes[card["to_ref"]]["role"] == "minor-arcana", "phase codon governs non-Minor card")
            translations = self.edges(ref, "TRANSLATES_TO")
            conditional = self.edges(ref, "CONDITIONALLY_TRANSLATES_TO")
            require(bool(translations), "phase codon without a translation outcome: " + ref)
            for edge in translations + conditional:
                require(self.nodes[edge["to_ref"]]["role"] == "amino-or-translation-signal", "wrong translation endpoint")
            phase.append({"ref": ref, "id": node["id"], "sequence": sequence,
                          "dna_ref": reflection["to_ref"], "state_count": props["stateCount"],
                          "native_state_count": 8 if len(set(sequence)) == 3 else 7,
                          "tarot_ref": card["to_ref"], "reflection_relation": reflection["id"],
                          "tarot_relation": card["id"],
                          "translation_relations": [e["id"] for e in translations],
                          "conditional_translation_relations": [e["id"] for e in conditional],
                          "court_relations": [e["id"] for e in self.edges(ref, "PAIRED_AS_COURT")]})
        require(len(phase) == 64 and {r["sequence"] for r in phase} == set(dna), "phase/DNA coverage drift")
        states = Counter(row["state_count"] for row in phase)
        require(set(states) <= {7, 8}, "unknown orientation count")
        for row in phase:
            if row["state_count"] != row["native_state_count"]:
                self.finding("orientation-count", ["deep-M3:M3-C12", "deep-M3:M3-C13"], "operational", row["ref"],
                             {"sequence": row["sequence"], "source_state_count": row["state_count"],
                              "native_state_count": row["native_state_count"]},
                             "crates/ql-core/src/pole/codon.rs", peer="rust")
        require({r["tarot_ref"] for r in phase} == {n["ref"] for n in self.groups["minor-arcana"]}, "Minor exact-cover drift")
        charges = []
        for node in self.groups["dna-codon"] + self.groups["phase-codon"]:
            props = node["properties"]
            x, y, z = (VALUES[c] for c in props["sequence"])
            expected = dict(zip(("pp", "nn", "np", "pn"), (x+y+z, x-y-z, x-y+z, x+y-z), strict=True))
            source = {k: props.get("inner_charge_" + k) for k in expected}
            charges.append({"ref": node["ref"], "source": source, "ratified_derivation": expected})
            if source != expected:
                self.finding("codon-charge", ["deep-M3:M3-C10", "deep-M3:M3-C11"], "operational", node["ref"],
                             {"source": source, "ratified_derivation": expected}, COIN_AUTHORITY)
        pairs = []
        # Preserve the existing native pair descriptor, including its directional signs.
        native_pairs = ((12,0),(15,-3),(14,-2),(13,1),(15,3),(18,0),(17,-1),(16,2),
                        (14,2),(17,-1),(16,0),(15,1),(13,1),(16,-2),(15,-1),(14,0))
        for node in self.groups["dinucleotide"]:
            props = node["properties"]
            sequence = props["sequence"]
            idx = "ATCG".index(sequence[0])*4 + "ATCG".index(sequence[1])
            source = [props["sumValue"], props["differenceValue"]]
            native = list(native_pairs[idx])
            pairs.append({"ref": node["ref"], "sequence": sequence, "source": source, "native": native})
            if source != native:
                self.finding("pair-descriptor", ["deep-M3:M3-C08"], "operational", node["ref"],
                             {"source": source, "native": native}, COIN_AUTHORITY)
        major_associations = []
        for node in self.groups["major-arcana"]:
            major_associations.append({"ref": node["ref"], "id": node["id"],
                                       "relations": {kind: [e["id"] for e in self.edges(node["ref"], kind)] for kind in
                                                     ("EMBODIES_AS", "PROVIDES_VESSEL_FOR", "ARCHETYPAL_CASCADE")}})
        return {"rna": transcription, "phase": sorted(phase, key=lambda r: r["sequence"]),
                "charges": charges, "pairs": pairs, "major_associations": major_associations}

    def clock(self) -> list:
        degrees = self.groups["clock-degree"]
        by_degree = {n["properties"]["degree"]: n for n in degrees}
        require(len(degrees) == 360 and set(by_degree) == set(range(360)), "incomplete dynamic degree field")
        backbone = {n["ref"]: n for n in self.groups["clock-backbone"]}
        require(len(backbone) == 24, "incomplete independent backbone field")
        require(not set(backbone) & {n["ref"] for n in degrees}, "backbone collapsed into dynamic degree flags")
        result = []
        arcs: dict[str, list] = defaultdict(list)
        for degree, node in sorted(by_degree.items()):
            ref = node["ref"]
            anchor = self.one(ref, "ANCHORED_BY")
            flow = self.one(ref, "FLOWS_CLOCKWISE")
            opposite = self.one(ref, "POLAR_OPPOSITE")
            require(anchor["to_ref"] in backbone, "dynamic degree does not bind independent backbone")
            governors = [e for e in self.edges(anchor["to_ref"], "GOVERNS_DEGREE_ARC") if e["to_ref"] == ref]
            require(len(governors) == 1, "clock anchor/governor inverse drift: " + ref)
            require(flow["to_ref"] == by_degree[(degree+1) % 360]["ref"], "clock flow drift: " + ref)
            require(opposite["to_ref"] == by_degree[(degree+180) % 360]["ref"], "clock opposition drift: " + ref)
            arcs[anchor["to_ref"]].append(degree)
            result.append({"degree": degree, "ref": ref, "id": node["id"],
                           "backbone_ref": anchor["to_ref"], "backbone_id": backbone[anchor["to_ref"]]["id"],
                           "anchor_relation": anchor["id"], "governor_relation": governors[0]["id"],
                           "clockwise_ref": flow["to_ref"], "clockwise_relation": flow["id"],
                           "opposite_ref": opposite["to_ref"], "opposite_relation": opposite["id"]})
        require(set(arcs) == set(backbone), "orphan clock backbone")
        for ref, arc in arcs.items():
            require(len(arc) == 15 and all((arc[i]+1) % 360 == arc[i+1] for i in range(14)),
                    "non-contiguous/non-15-degree governor arc: " + ref)
            require(len(self.edges(ref, "GOVERNS_DEGREE_ARC")) == 15, "extra governor relation: " + ref)
        # Degree properties/relations do not supply the legacy approximate hexagram,
        # zero-valued Tarot/Ananda/archetype entries. Record absent warrant explicitly.
        self.finding("legacy-clock-placeholder", ["deep-M3:M3-C24", "deep-M3:M3-C26"], "relation", "#3-5",
                     {"dynamic_coordinates": 360, "independent_backbone_coordinates": 24,
                      "legacy_file": "vendor/epi-kernel/reference/src/m3_clock_lut.c",
                      "unwarranted_as_source_facts": ["computed hexagram approximation", "tarot_card_id=0", "m1_ananda_value=0", "m0_archetype=0", "degree flag as backbone identity"]},
                     REGISTRY, authority_peer="bimba")
        return result

    def run(self) -> dict:
        self.findings = []
        hexagrams = self.hexagrams()
        matrices = self.matrices(hexagrams)
        genetics = self.genetics()
        clock = self.clock()
        self.findings.sort(key=lambda f: f["id"])
        details = {"hexagrams": hexagrams, "matrices": matrices, "genetics": genetics, "clock": clock}
        kinds = Counter(e["kind"] for e in self.projection["relations"])
        return {"schema": "ql.m3-source-audit/v1", "registry_revision": self.projection["registry_revision"],
                "source_revision": self.projection["source_revision"],
                "projection_sha256": digest(self.projection), "details": details,
                "counts": {"nodes": len(self.nodes), "relations": len(self.projection["relations"]),
                           "roles": dict(sorted((r, len(ns)) for r, ns in self.groups.items())),
                           "relation_kinds": dict(sorted(kinds.items())),
                           "source_orientation_states": sum(r["state_count"] for r in genetics["phase"]),
                           "native_orientation_states": sum(r["native_state_count"] for r in genetics["phase"]),
                           "transcribed_rna": len(genetics["rna"]),
                           "open_findings": len(self.findings)},
                "discrepancies": self.findings,
                "standing": {"source": "exact-locked-record-join", "coordinate": "existing-K2-IDs",
                             "semantic_parity": "explicit-discrepancies-retained",
                             "c_rust_execution": "not-claimed-by-this-source-audit",
                             "K4_acceptance": "not-claimed", "neo4j_live": "not-observed",
                             "cpp_embodiment": "not-executed", "experiential": "not-claimed"}}


def lock_for(projection: dict, audit: dict, repo_root: Path = ROOT) -> dict:
    return {"schema": "ql.m3-source-bindings-lock/v1", "generator": "scripts/m3-source-parity.py",
            "registry_revision": projection["registry_revision"],
            "source_repository": projection["source_repository"], "source_revision": projection["source_revision"],
            "source_files": projection["files"], "projection_sha256": digest(projection),
            "audit_sha256": digest(audit), "counts": audit["counts"],
            "detail_sha256": {name: digest(data) for name, data in audit["details"].items()},
            "finding_counts": dict(sorted(Counter(json.loads(d["detail"])["code"] for d in audit["discrepancies"]).items())),
            "native_authority_lock": {p: hashlib.sha256((repo_root / p).read_bytes()).hexdigest() for p in
                                      (COIN_AUTHORITY, "crates/ql-core/src/pole/iching.rs", "crates/ql-core/src/pole/codon.rs",
                                       "crates/ql-core/src/pole/fold.rs", "vendor/epi-kernel/reference/src/m3.c",
                                       "vendor/epi-kernel/reference/src/m3_clock_lut.c")},
            "standing": audit["standing"]}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-root", type=Path, required=True,
                        help="read-only checkout of the K2-locked Epi source revision")
    parser.add_argument("--output", type=Path, default=ROOT / "target/m3-source")
    parser.add_argument("--refresh-lock", action="store_true", help="explicitly record this reviewed source/audit result")
    args = parser.parse_args()
    try:
        registry, nodes, edges = read_source(args.source_root)
        projection = project(registry, nodes, edges)
        audit = Audit(projection).run()
        lock = lock_for(projection, audit)
        args.output.mkdir(parents=True, exist_ok=True)
        (args.output / "bindings.json").write_text(render(projection), encoding="utf-8")
        (args.output / "audit.json").write_text(render(audit), encoding="utf-8")
        # These are K3-shaped OPEN records for the canonical ledger owner to reconcile,
        # not a new independent lifecycle, blanket readiness change, or auto-promotion.
        (args.output / "ledger-discrepancies.json").write_text(render(audit["discrepancies"]), encoding="utf-8")
        if args.refresh_lock:
            (ROOT / LOCK).write_text(render(lock), encoding="utf-8")
        else:
            require(load_json(ROOT / LOCK) == lock, "M3 source/audit lock drift; inspect evidence, do not auto-promote")
        print(render({"status": "passed", "counts": audit["counts"], "finding_counts": lock["finding_counts"],
                      "projection_sha256": lock["projection_sha256"], "audit_sha256": lock["audit_sha256"],
                      "standing": audit["standing"]}), end="")
        return 0
    except (ValueError, KeyError, TypeError, OSError) as exc:
        print("M3 source audit failed: " + str(exc), file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
