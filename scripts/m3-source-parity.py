#!/usr/bin/env python3
"""M3 binding projection over the Bimba map and independent semantic audit.

Node properties come from a read-only map read (scripts/bimba_map.py) of the map
the registry was built from; relations come from the registry. The small
checked-in fixture locks the output without copying the graph. Map assertions
and derived addresses never overwrite one another. Findings use the existing K3
discrepancy vocabulary and remain OPEN until ledger reconciliation.

The current map carries no asserted hexagram/trigram binary. Native addresses
retain the kernel's declared trigram table (as in generate-m3.py). Separately,
literal trigram line descriptions and directed upper/lower relations supply a
source-derived bottom-to-top line pattern. LINE_CHANGE must agree with its
prose's named line and Yin/Yang polarity as well as the single-bit native law.
Missing source warrant cannot silently fall back to a native address.
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
sys.path.insert(0, str(ROOT / "scripts"))
import bimba_map  # noqa: E402
# The source trigram ids are NOT their three-bit line patterns (generate-m3.py).
TRIGRAM_BITS = dict(zip((f"#3-1-{i}" for i in range(8)), (7, 0, 1, 6, 2, 5, 4, 3), strict=True))
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


def source_trigram_lines(ref: str, description: Any) -> tuple[int, ...]:
    """Decode the source's physical line description without native IDs/bits."""
    require(isinstance(description, str), "unreadable source trigram line description: " + ref)
    text = " ".join(description.split()).casefold()
    if text == "three solid yang lines":
        return (1, 1, 1)
    if text == "three broken yin lines":
        return (0, 0, 0)
    match = re.fullmatch(r"(yin|yang) line (below|between|above) two (yin|yang) lines", text)
    require(match is not None and match[1] != match[3],
            "unreadable source trigram line description: " + ref)
    single, position, pair = match.groups()
    lines = [int(pair == "yang")] * 3
    lines[("below", "between", "above").index(position)] = int(single == "yang")
    return tuple(lines)


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
    if ref.startswith("#3-3-3-") and re.fullmatch(r"[AUCG]{3}", str(props.get("p_3_sequence", ""))):
        return "rna-codon"
    if ref.startswith("#3-5-5/0-") and "m_3_5_degree" in props:
        return "clock-degree"
    if ref.startswith("#3-3-5"):
        return "karyotype"
    return "source-coordinate"  # Retain the whole remaining field, not just numeric roles.


def read_source(map_path: Path = bimba_map.CACHE, repo_root: Path = ROOT) -> tuple[dict, dict]:
    registry = load_json(repo_root / REGISTRY)
    require(registry["schema"] == "ql.m-tree/v1", "unsupported shared registry")
    read = bimba_map.load(map_path)
    require(read["content_sha256"] == registry["source_revision"],
            "map read is not the map the registry was built from")
    return registry, read


def project(registry: dict, read: dict) -> dict:
    """Join map properties and registry relations onto K2 IDs for the whole M3 field."""
    require(digest({k: v for k, v in registry.items() if k != "registry_revision"}) == registry["registry_revision"],
            "registry revision/content disagreement")
    by_ql = {bimba_map.ql_spelling(c): c for c in read["nodes"] if bimba_map.is_m_coordinate(c)}
    records = registry["records"]
    field = [n for n in registry["nodes"] if n["root_position"] == 3]
    require(all(n["source_ref"] in by_ql for n in field), "M3 registry node absent from the map read")
    nodes = []
    for node in field:
        ref = node["source_ref"]
        props = read["nodes"][by_ql[ref]]["properties"]
        nodes.append({"id": node["id"], "ref": ref, "record": node["records"][0],
                      "source_record_index": records[node["records"][0]]["record_index"],
                      "role": role(ref, props), "parent_id": node["parent_id"], "properties": props})
    ids = {n["id"] for n in field}
    map_props = {(a, kind, b): props for a, kind, b, props in read["relations"]}
    edges = []
    for rel in registry["relations"]:
        if rel["from_id"] not in ids:
            continue
        ends = [by_ql.get(r) or r.removeprefix("bimba:") for r in (rel["from_ref"], rel["to_ref"])]
        key = (ends[0], rel["source_kind"], ends[1])
        require(key in map_props, "registry relation absent from the map read: " + rel["relation_ref"])
        edges.append({"id": rel["id"], "ref": rel["relation_ref"], "record": rel["record"],
                      "source_record_index": records[rel["record"]]["record_index"],
                      "from_ref": rel["from_ref"], "to_ref": rel["to_ref"],
                      "from_id": rel["from_id"], "to_id": rel["to_id"],
                      "kind": rel["source_kind"], "properties": map_props[key]})
    return {"schema": SCHEMA, "registry_revision": registry["registry_revision"],
            "source_revision": registry["source_revision"],
            "source_repository": registry["source_repository"],
            "files": registry["files"], "nodes": nodes, "relations": edges}


def projection_for(map_path: Path = bimba_map.CACHE, repo_root: Path = ROOT) -> dict:
    """The M3 projection from a read of the registry's map, else the committed one.

    fixtures/kernel/m3-domain-v1.json holds the full projection (map properties and
    registry relations) as of its registry revision, so CI checks run without a
    map read; `m3-domain.py --map ... --refresh` is what moves it.
    """
    registry = load_json(repo_root / REGISTRY)
    if map_path.is_file() and bimba_map.load(map_path)["content_sha256"] == registry["source_revision"]:
        return project(*read_source(map_path, repo_root))
    domain = load_json(repo_root / "fixtures/kernel/m3-domain-v1.json")
    require(domain["registry_revision"] == registry["registry_revision"], "committed M3 domain is stale")
    return {"schema": SCHEMA, "registry_revision": registry["registry_revision"],
            "source_revision": registry["source_revision"],
            "source_repository": registry["source_repository"],
            "files": registry["files"], "nodes": domain["nodes"], "relations": domain["relations"]}


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
        trigrams = {n["ref"]: TRIGRAM_BITS[n["ref"]] for n in self.groups["trigram"]}
        require(len(trigrams) == 8, "incomplete trigram field")
        source_trigrams = {n["ref"]: source_trigram_lines(n["ref"], n["properties"].get("c_1_lines_description"))
                           for n in self.groups["trigram"]}
        source_lines = {}
        result = []
        for node in self.groups["hexagram"]:
            ref = node["ref"]
            upper, lower = self.one(ref, "HAS_UPPER_TRIGRAM"), self.one(ref, "HAS_LOWER_TRIGRAM")
            require(upper["to_ref"] in trigrams and lower["to_ref"] in trigrams,
                    "hexagram has a non-trigram endpoint: " + ref)
            address = (trigrams[upper["to_ref"]] << 3) | trigrams[lower["to_ref"]]
            source_lines[ref] = source_trigrams[lower["to_ref"]] + source_trigrams[upper["to_ref"]]
            result.append({"ref": ref, "id": node["id"], "symbol": node["properties"].get("c_1_symbol"),
                           "trigram_derived_address": address,
                           "upper_relation": upper["id"], "lower_relation": lower["id"]})
        by_ref = {r["ref"]: r for r in result}
        require(len(result) == 64 and {r["trigram_derived_address"] for r in result} == set(range(64)),
                "trigram relations do not produce a complete 64-address field")
        by_source_lines = {lines: ref for ref, lines in source_lines.items()}
        require(len(by_source_lines) == 64, "source line descriptions do not produce a complete 64-pattern field")
        by_address = {r["trigram_derived_address"]: r["ref"] for r in result}
        for row in result:
            props = self.nodes[row["ref"]]["properties"]
            address = row["trigram_derived_address"]
            nuclear = by_address[(((address >> 2) & 7) << 3) | ((address >> 1) & 7)]
            if props.get("c_3_nuclear_coordinate") != nuclear:
                self.finding("nuclear-register", ["deep-M3:M3-C07"], "relation", row["ref"],
                             {"differences": {"c_3_nuclear_coordinate": {"source": props.get("c_3_nuclear_coordinate"),
                                                                         "native": nuclear}}},
                             "crates/ql-core/src/pole/iching.rs", peer="rust")
            bits = []
            for edge in self.edges(row["ref"], "LINE_CHANGE"):
                require(edge["to_ref"] in by_ref, "LINE_CHANGE has non-hexagram endpoint")
                flip = address ^ by_ref[edge["to_ref"]]["trigram_derived_address"]
                single = flip != 0 and flip & (flip - 1) == 0
                bits.append(flip.bit_length() - 1 if single else None)
                if not single:
                    self.finding("line-change", ["deep-M3:M3-C06"], "relation", edge["ref"],
                                 {"source": row["ref"], "source_target": edge["to_ref"],
                                  "flipped_address_bits": format(flip, "06b")}, "crates/ql-core/src/pole/iching.rs")
                self.line_change_qualification(edge, source_lines, by_source_lines)
            if sorted(b for b in bits if b is not None) != list(range(6)):
                self.finding("line-change-field", ["deep-M3:M3-C06"], "relation", row["ref"],
                             {"single_bit_lines": sorted(b + 1 for b in bits if b is not None),
                              "expected_lines": list(range(1, 7))}, "crates/ql-core/src/pole/iching.rs")
        return sorted(result, key=lambda row: row["trigram_derived_address"])

    def line_change_qualification(self, edge: dict, source_lines: dict, by_source_lines: dict) -> None:
        """Retain disagreements between a directed edge and its full qualifier."""
        source, target = source_lines[edge["from_ref"]], source_lines[edge["to_ref"]]
        changed = [i + 1 for i, (a, b) in enumerate(zip(source, target, strict=True)) if a != b]
        qualification = edge["properties"].get("c_1_relation_description")
        match = re.match(r"(First|Second|Third|Fourth|Fifth|Sixth) line changes from (Yin|Yang) to (Yin|Yang)\b",
                         qualification, re.IGNORECASE) if isinstance(qualification, str) else None
        declared, expected_ref, errors = None, None, []
        if match is None:
            errors.append("unreadable-qualification")
        else:
            ordinal, before, after = (value.casefold() for value in match.groups())
            line = ("first", "second", "third", "fourth", "fifth", "sixth").index(ordinal) + 1
            declared = {"line": line, "from": before.capitalize(), "to": after.capitalize()}
            expected = list(source)
            expected[line - 1] ^= 1
            expected_ref = by_source_lines[tuple(expected)]
            if changed != [line]:
                errors.append("named-line-target")
            if source[line - 1] != int(before == "yang"):
                errors.append("source-polarity")
            if target[line - 1] != int(after == "yang"):
                errors.append("target-polarity")
        if errors:
            self.finding("line-change-qualification", ["deep-M3:M3-C06"], "relation", edge["ref"],
                         {"source": edge["from_ref"], "source_target": edge["to_ref"],
                          "qualification": qualification, "declared": declared,
                          "actual_changed_lines": changed, "line_flip_target_ref": expected_ref,
                          "source_lines": list(source), "target_lines": list(target), "errors": errors},
                         REGISTRY, peer="rust", authority_peer="bimba")

    def matrices(self, hexagrams: list) -> list:
        by_ref = {r["ref"]: r for r in hexagrams}
        result = []
        for node in self.groups["matrix-cell"]:
            ref = node["ref"]
            resolved = self.one(ref, "RESOLVES_TO")
            require(resolved["to_ref"] in by_ref, "matrix does not resolve a hexagram: " + ref)
            uses, yields = self.edges(ref, "USES_PAIR"), self.edges(ref, "YIELDS_CODON")
            # A non-dual cell yields one codon: the map holds its positive and
            # negative readings as a single edge (the seed's parallel pair).
            kinds = sorted(e["properties"].get("c_2_relation_kind") for e in yields)
            require(kinds in (["negative", "positive"], ["positive"]), "matrix codon valences incomplete: " + ref)
            for edge in uses + yields:
                want = "dinucleotide" if edge["kind"] == "USES_PAIR" else "dna-codon"
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
                           "non_dual": len(yields) == 1,
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
        dna = {n["properties"]["p_3_sequence"]: n for n in self.groups["dna-codon"]}
        require(len(dna) == 64 and all(re.fullmatch(r"[ATCG]{3}", s) for s in dna), "incomplete DNA codon field")
        rna = {n["properties"]["p_3_sequence"]: n for n in self.groups["rna-codon"]}
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
            sequence = props["p_3_sequence"]
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
                          "dna_ref": reflection["to_ref"], "state_count": props["c_3_state_count"],
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
            x, y, z = (VALUES[c] for c in props["p_3_sequence"])
            expected = dict(zip(("pp", "nn", "np", "pn"), (x+y+z, x-y-z, x-y+z, x+y-z), strict=True))
            source = {k: props.get("c_3_inner_charge_" + k) for k in expected}
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
            sequence = props["p_3_sequence"]
            idx = "ATCG".index(sequence[0])*4 + "ATCG".index(sequence[1])
            source = [props["c_3_sum_value"], props["c_3_difference_value"]]
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
        by_degree = {n["properties"]["m_3_5_degree"]: n for n in degrees}
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
                "standing": {"source": "bimba-map-read-join", "coordinate": "existing-K2-IDs",
                             "semantic_parity": "explicit-discrepancies-retained",
                             "c_rust_execution": "not-claimed-by-this-source-audit",
                             "K4_acceptance": "not-claimed", "neo4j_live": "read-only map read; content hash = registry source_revision",
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
    parser.add_argument("--map", type=Path, default=bimba_map.CACHE,
                        help="read of the map the registry was built from (scripts/bimba_map.py read)")
    parser.add_argument("--output", type=Path, default=ROOT / "target/m3-source")
    parser.add_argument("--refresh-lock", action="store_true", help="explicitly record this reviewed source/audit result")
    args = parser.parse_args()
    try:
        projection = projection_for(args.map)
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
