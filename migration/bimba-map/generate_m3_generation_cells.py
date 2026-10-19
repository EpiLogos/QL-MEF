#!/usr/bin/env python3
"""Generate the M3-3-2 generation-cell corrections (owner permission, 19 Oct 2026).

The map's #3-3-2 matrix layer holds one GenerationEvent cell per matrix x hexagram.
Each cell is the generation law applied to one hexagram:

  each matrix sends the 8 trigrams to 8 nucleotide pairs (pi_m):
    the palindromic trigrams Qian, Kun, Kan, Li -> TT, AA, CC, GG in every matrix;
    the other four -> the two mixed pairs of the matrix's base partition, both orders
      Matrix 1  complementary   A-T, C-G
      Matrix 2  moving/resting  A-G, T-C
      Matrix 3  same quality    A-C, T-G
  hexagram (lower L, upper U):  ab = pi_m(L), cd = pi_m(U)
    positive codon = a + cd   (the last pair is the upper pair)
    negative codon = ab + d   (the first pair is the lower pair)
    the cell is non-dual exactly when b == c (positive == negative)

Nucleotides keep the ruled coin law and its line names (M3-COIN-1):
  A = 6 Old Yin   (yin,  moving)    T = 9 Old Yang   (yang, moving)
  C = 8 Young Yin (yin,  resting)   G = 7 Young Yang (yang, resting)

The tables are the HMS FR 2.Q.8 tables. The map's own cells agree with them in
Matrices 2 and 3; in Matrix 1 the map's cells carry thunder (Zhen) and mountain (Gen)
swapped. The generator proves the HMS Matrix 1 table instead of assuming it:
  - the map's own COMPLEMENTS trigram edges pair Zhen-Xun and Gen-Dui; the HMS table
    gives each complementary pair reversed base pairs;
  - complementing a hexagram gives the Watson-Crick complement of its cell's codons
    in all 64 cells;
  - each matrix's hexagram action carries every cell to the cell of its partner codons,
    and Matrix 1 then Matrix 2 equals Matrix 3.

Writes (guarded; the property ontology already used by these nodes and relations):
  - 8 Matrix 3 cells absent from the map (lower trigram Xun) are created;
  - YIELDS_CODON, USES_PAIR (with c_2_relation_kind lower | upper | lower-and-upper),
    RESOLVES_TO, INITIATES and IS_CLASSIFIED_AS are set to the law on all 192 cells;
  - the four m_3_3_* binaries (yang = 1 per base) are set to the law;
  - every codon gets c_3_symmetry_class (perfect_palindromic, imperfect_palindromic,
    non_palindromic_non_dual, dual);
  - each nucleotide gets its coin value and line: m_3_2_coin_value, m_3_2_iching_line,
    m_3_2_polarity, m_3_2_mobility (set only where absent).

Read-only against the map (HTTP, accessMode READ). Writes the .cypher, its rollback
and the generated field.
"""
from __future__ import annotations

import json
import sys
import urllib.request
import uuid
from datetime import datetime, timezone
from pathlib import Path

HERE = Path(__file__).resolve().parent
ENDPOINT = "http://100.92.62.101:7474/db/neo4j/query/v2"
STAMP = "2026-10-19"
OUT = HERE / f"{STAMP}-m3-generation-cells.cypher"
ROLLBACK = HERE / f"{STAMP}-m3-generation-cells.rollback.cypher"
FIELD = HERE.parent.parent / "fixtures/kernel/m3-generation-field-v1.json"
CREATED_AT = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%S.000Z")

COIN = {
    "A": (6, "Old Yin", "yin", "moving"),
    "T": (9, "Old Yang", "yang", "moving"),
    "C": (8, "Young Yin", "yin", "resting"),
    "G": (7, "Young Yang", "yang", "resting"),
}
PALINDROMIC = {"Qian": "TT", "Kun": "AA", "Kan": "CC", "Li": "GG"}
MIXED = {
    1: {"Zhen": "GC", "Gen": "TA", "Dui": "AT", "Xun": "CG"},
    2: {"Zhen": "CT", "Dui": "TC", "Gen": "AG", "Xun": "GA"},
    3: {"Zhen": "AC", "Gen": "CA", "Dui": "GT", "Xun": "TG"},
}
PARTNER = {1: dict(zip("ATCG", "TAGC")), 2: dict(zip("ATCG", "GCTA")), 3: dict(zip("ATCG", "CGAT"))}
MATRIX_COORD = {1: "M3-3-2-0", 2: "M3-3-2-1", 3: "M3-3-2-2"}
CLASSIFIER = {
    "perfect_palindromic": "M3-3-0-0-0",
    "imperfect_palindromic": "M3-3-0-0-1",
    "non_palindromic_non_dual": "M3-3-0-1",
    "dual": "M3-3-1",
}


def rel(source: str, target: str, rtype: str, kind: str | None = None) -> str:
    """The relation property map these relations already carry, in the map's own keys."""
    props = (f"c_0_source_coordinate:{q(source)}, c_0_target_coordinate:{q(target)}, "
             f"c_1_relation_family:'correspondential', c_2_relation_type:{q(rtype)}, "
             f"c_3_created_at:{q(CREATED_AT)}, c_3_dataset_branch:'mahamaya-deep'")
    if kind is not None:
        props += f", c_2_relation_kind:{q(kind)}"
    return "{" + props + "}"


def query(statement: str) -> list:
    body = json.dumps({"statement": statement, "accessMode": "READ"}).encode()
    request = urllib.request.Request(ENDPOINT, data=body, headers={"Content-Type": "application/json"})
    reply = json.load(urllib.request.urlopen(request, timeout=120))
    if reply.get("errors"):
        raise SystemExit(reply["errors"])
    return reply["data"]["values"]


def codon_class(c: str) -> str:
    if c[0] == c[2]:
        return "perfect_palindromic" if c[0] == c[1] else "imperfect_palindromic"
    return "non_palindromic_non_dual" if c[0] == c[1] or c[1] == c[2] else "dual"


def yang_bits(s: str) -> str:
    return "".join("1" if COIN[x][2] == "yang" else "0" for x in s)


def pair_of(matrix: int, trigram: str) -> str:
    return PALINDROMIC.get(trigram) or MIXED[matrix][trigram]


def lit(v) -> str:
    return json.dumps(v, ensure_ascii=False)


def q(v: str) -> str:
    return "'" + v.replace("\\", "\\\\").replace("'", "\\'") + "'"


def main() -> None:
    # ---- the map's own facts ----
    hexes = {}  # coordinate -> {name, lower, upper}
    for coord, name, rtype, tri in query(
        "MATCH (h:Hexagram)-[r:HAS_UPPER_TRIGRAM|HAS_LOWER_TRIGRAM]->(t:Trigram) "
        "RETURN h.coordinate, h.c_1_name, type(r), t.c_1_name"
    ):
        h = hexes.setdefault(coord, {"name": name})
        h["upper" if rtype == "HAS_UPPER_TRIGRAM" else "lower"] = tri
    assert len(hexes) == 64 and all("upper" in h and "lower" in h for h in hexes.values())
    # lines bottom -> top (the I Ching reading); the map holds no trigram binaries
    trigram_line = {"Qian": "111", "Kun": "000", "Zhen": "100", "Kan": "010",
                    "Gen": "001", "Xun": "011", "Li": "101", "Dui": "110"}
    line_trigram = {v: k for k, v in trigram_line.items()}
    assert set(trigram_line) == {n for (n,) in query("MATCH (t:Trigram) RETURN t.c_1_name")}
    complements = {tuple(sorted((a, b))) for a, b in query(
        "MATCH (a:Trigram)-[:COMPLEMENTS]-(b:Trigram) RETURN a.c_1_name, b.c_1_name")}
    pairs = {seq: coord for seq, coord in query("MATCH (p:NucleotidePair) RETURN p.p_3_sequence, p.coordinate")}
    codons = {seq: coord for seq, coord in query("MATCH (c:Codon) RETURN c.p_3_sequence, c.coordinate")}
    assert len(pairs) == 16 and len(codons) == 64
    cells = {}
    for coord, ctx, hcoord in query(
        "MATCH (g:GenerationEvent) OPTIONAL MATCH (g)-[:RESOLVES_TO]->(h:Hexagram) "
        "RETURN g.coordinate, g.c_1_context, h.coordinate"
    ):
        cells[coord] = {"context": ctx, "hexagram": hcoord}
    # cell suffix (King Wen ordinal) -> hexagram coordinate, from the map's own RESOLVES_TO
    ordinal_hex = {}
    for coord, c in cells.items():
        n = int(coord.rsplit("-", 1)[1])
        if c["hexagram"]:
            assert ordinal_hex.setdefault(n, c["hexagram"]) == c["hexagram"], (coord, c)
    assert len(ordinal_hex) == 64

    # ---- proofs before any write ----
    for a, b in [("Zhen", "Xun"), ("Gen", "Dui")]:
        assert tuple(sorted((a, b))) in complements, "map COMPLEMENTS edges changed"
        pa, pb = MIXED[1][a], MIXED[1][b]
        assert pa == pb[::-1] and {pa[0], pa[1]} in ({"A", "T"}, {"C", "G"}), (a, b)
    inverse = {"Qian": "Kun", "Kun": "Qian", "Kan": "Li", "Li": "Kan",
               "Zhen": "Xun", "Xun": "Zhen", "Gen": "Dui", "Dui": "Gen"}
    field = []
    for m in (1, 2, 3):
        for n in range(1, 65):
            h = hexes[ordinal_hex[n]]
            ab, cd = pair_of(m, h["lower"]), pair_of(m, h["upper"])
            pos, neg = ab[0] + cd, ab + cd[1]
            field.append({
                "coordinate": f"{MATRIX_COORD[m]}-{n}", "matrix": m, "hexagram_ordinal": n,
                "hexagram": ordinal_hex[n], "hexagram_name": h["name"],
                "lower_trigram": h["lower"], "upper_trigram": h["upper"],
                "lower_pair": ab, "upper_pair": cd, "positive": pos, "negative": neg,
                "cell_class": codon_class(pos) if pos == neg else "dual",
                "non_dual": pos == neg,
            })
    by_key = {(c["matrix"], c["lower_trigram"], c["upper_trigram"]): c for c in field}
    for c in field:  # Matrix 1: complementing the hexagram complements the codons
        if c["matrix"] == 1:
            o = by_key[(1, inverse[c["lower_trigram"]], inverse[c["upper_trigram"]])]
            wc = PARTNER[1]
            assert o["positive"] == "".join(wc[x] for x in c["positive"]), c
    nondual_codons = {c["positive"] for c in field if c["non_dual"]}
    assert nondual_codons == {s for s in codons if codon_class(s) != "dual"}, "fixed points != 40 non-dual"
    assert len(nondual_codons) == 40
    for m in (1, 2, 3):
        assert sum(c["non_dual"] for c in field if c["matrix"] == m) == 16
    assert sum(1 if c["non_dual"] else 2 for c in field) == 336
    for m in (1, 2, 3):  # each matrix's hexagram action carries cells to partner cells
        flip = {1: ("111", "111"), 2: ("101", "010"), 3: ("010", "101")}[m]
        def act(t, m=m, flip=flip):
            bits = trigram_line[t]
            mask = flip[0] if bits[0] == bits[2] else flip[1]
            return line_trigram["".join(str(int(a) ^ int(b)) for a, b in zip(bits, mask))]
        for c in field:
            if c["matrix"] != m:
                continue
            o = by_key[(m, act(c["lower_trigram"]), act(c["upper_trigram"]))]
            assert o["positive"] == "".join(PARTNER[m][x] for x in c["positive"]), (m, c)
            assert o["negative"] == "".join(PARTNER[m][x] for x in c["negative"]), (m, c)

    # ---- the map's own cells agree with the law in Matrices 2 and 3 ----
    yields_now = {}
    for coord, seq, kind in query("MATCH (g:GenerationEvent)-[y:YIELDS_CODON]->(c:Codon) RETURN g.coordinate, c.p_3_sequence, y.c_2_relation_kind"):
        yields_now.setdefault(coord, {})[kind] = seq
    disagree = {1: 0, 2: 0, 3: 0}
    for c in field:
        y = yields_now.get(c["coordinate"])
        if y is None:
            continue
        got = (y.get("positive"), y.get("negative", y.get("positive")))
        if got != (c["positive"], c["negative"]):
            disagree[c["matrix"]] += 1
    print("cells disagreeing with the law, by matrix:", disagree)
    assert disagree[2] == 0 and disagree[3] == 0, disagree
    assert disagree[1] == 28, disagree

    # ---- current map state of every cell (for guards and rollback) ----
    current = {}
    for coord, ys, ps, ks, ini, b in query(
        "MATCH (g:GenerationEvent) "
        "OPTIONAL MATCH (g)-[y:YIELDS_CODON]->(c:Codon) WITH g, collect(DISTINCT [c.coordinate, y.c_2_relation_kind]) AS ys "
        "OPTIONAL MATCH (g)-[:USES_PAIR]->(p:NucleotidePair) WITH g, ys, collect(DISTINCT p.coordinate) AS ps "
        "OPTIONAL MATCH (g)-[:IS_CLASSIFIED_AS]->(k:Classifier) WITH g, ys, ps, collect(DISTINCT k.coordinate) AS ks "
        "OPTIONAL MATCH (mo:MatrixOperator)-[:INITIATES]->(g) "
        "RETURN g.coordinate, ys, ps, ks, collect(DISTINCT mo.coordinate), "
        "[g.m_3_3_positive_codon_binary, g.m_3_3_negative_codon_binary, g.m_3_3_lower_pair_binary, g.m_3_3_upper_pair_binary]"
    ):
        current[coord] = {"yields": sorted(map(tuple, ys)), "pairs": sorted(ps), "classes": sorted(ks),
                          "initiated_by": sorted(ini), "binaries": b}

    lines = [
        "// M3-3-2 generation cells: the generation law on all 192 matrix x hexagram cells.",
        f"// Generated by migration/bimba-map/generate_m3_generation_cells.py at {CREATED_AT}; do not hand-edit.",
        "// Owner permission 19 Oct 2026 for clean, safe map updates under the property ontology.",
        "// Coin law and line names: A=6 Old Yin, T=9 Old Yang, C=8 Young Yin, G=7 Young Yang.",
        "",
        "// @probe",
        "MATCH (g:GenerationEvent) RETURN count(g) AS cells;",
        "MATCH (:GenerationEvent)-[r:YIELDS_CODON]->() RETURN count(r) AS yields;",
        "MATCH (c:Codon) WHERE c.c_3_symmetry_class IS NOT NULL RETURN count(c) AS classified_codons;",
        "",
    ]
    rollback = [
        "// Rollback of the M3-3-2 generation-cell corrections: restores the map's prior relations,",
        "// binaries and absent cells exactly as read before the migration.",
        f"// Generated with {OUT.name} at {CREATED_AT}.",
        "",
        "// @apply restore",
    ]
    stats = {"create_cells": 0, "retarget_yields": 0, "set_pairs": 0, "set_classes": 0,
             "set_initiates": 0, "set_binaries": 0}

    for c in field:
        coord, cur = c["coordinate"], current.get(c["coordinate"])
        g = f"MATCH (g:GenerationEvent {{coordinate:{q(coord)}}})"
        want_bin = [yang_bits(c["positive"]), yang_bits(c["negative"]), yang_bits(c["lower_pair"]), yang_bits(c["upper_pair"])]
        if cur is None:
            stats["create_cells"] += 1
            lines.append(f"// @apply create {coord} ({c['hexagram_name']}, lower {c['lower_trigram']})")
            props = {
                "coordinate": coord, "c_2_uuid": str(uuid.uuid5(uuid.NAMESPACE_OID, coord)),
                "c_1_context": f"M{c['matrix']}-H{c['hexagram_ordinal']}",
                "c_3_dataset_branch": "mahamaya-deep", "c_3_dataset_branch_label": "M3 Mahamaya Deep",
                "c_3_source_dataset": "bimba", "c_4_family": "M", "c_4_layer": "COORDINATE",
                "c_4_ql_position": 3, "c_4_subsystem": 3,
                "m_3_3_positive_codon_binary": want_bin[0], "m_3_3_negative_codon_binary": want_bin[1],
                "m_3_3_lower_pair_binary": want_bin[2], "m_3_3_upper_pair_binary": want_bin[3],
            }
            body = ", ".join(f"{k}:{lit(v)}" for k, v in props.items())
            lines.append(f"MERGE (g:Bimba:GenerationEvent {{coordinate:{q(coord)}}}) ON CREATE SET g += {{{body}}};")
            rollback.append(f"MATCH (g:GenerationEvent {{coordinate:{q(coord)}}}) DETACH DELETE g;")
            cur = {"yields": [], "pairs": [], "classes": [], "initiated_by": [], "binaries": [None] * 4}
        else:
            lines.append(f"// @apply cell {coord} ({c['hexagram_name']}, {c['lower_trigram']}/{c['upper_trigram']})")
        # INITIATES
        mat = MATRIX_COORD[c["matrix"]]
        if cur["initiated_by"] != [mat]:
            stats["set_initiates"] += 1
            lines.append(f"MATCH (mo:MatrixOperator)-[r:INITIATES]->(g:GenerationEvent {{coordinate:{q(coord)}}}) WHERE mo.coordinate <> {q(mat)} DELETE r;")
            lines.append(f"MATCH (mo:MatrixOperator {{coordinate:{q(mat)}}}), (g:GenerationEvent {{coordinate:{q(coord)}}}) "
                         f"MERGE (mo)-[r:INITIATES]->(g) ON CREATE SET r += {rel(mat, coord, 'INITIATES')};")
            for old in cur["initiated_by"]:
                if old != mat:
                    rollback.append(f"MATCH (mo:MatrixOperator {{coordinate:{q(old)}}}), (g:GenerationEvent {{coordinate:{q(coord)}}}) MERGE (mo)-[:INITIATES]->(g);")
            if mat not in cur["initiated_by"] and cur["initiated_by"] is not None and coord in current:
                rollback.append(f"MATCH (:MatrixOperator {{coordinate:{q(mat)}}})-[r:INITIATES]->(:GenerationEvent {{coordinate:{q(coord)}}}) DELETE r;")
        # RESOLVES_TO (verified against the cell's ordinal)
        if coord not in current:
            lines.append(f"MATCH (g:GenerationEvent {{coordinate:{q(coord)}}}), (h:Hexagram {{coordinate:{q(c['hexagram'])}}}) "
                         f"MERGE (g)-[r:RESOLVES_TO]->(h) ON CREATE SET r += {rel(coord, c['hexagram'], 'RESOLVES_TO')};")
        else:
            assert cells[coord]["hexagram"] == c["hexagram"], (coord, cells[coord])
        # YIELDS_CODON
        want_y = [(codons[c["positive"]], "positive")] if c["non_dual"] else sorted([(codons[c["positive"]], "positive"), (codons[c["negative"]], "negative")])
        if sorted(cur["yields"]) != sorted(want_y):
            stats["retarget_yields"] += 1
            lines.append(f"{g}-[r:YIELDS_CODON]->() DELETE r;")
            for tgt, kind in want_y:
                lines.append(f"MATCH (g:GenerationEvent {{coordinate:{q(coord)}}}), (c:Codon {{coordinate:{q(tgt)}}}) "
                             f"CREATE (g)-[r:YIELDS_CODON]->(c) SET r += {rel(coord, tgt, 'YIELDS_CODON', kind)};")
            if coord in current:
                rollback.append(f"MATCH (g:GenerationEvent {{coordinate:{q(coord)}}})-[r:YIELDS_CODON]->() DELETE r;")
                for tgt, kind in cur["yields"]:
                    rollback.append(f"MATCH (g:GenerationEvent {{coordinate:{q(coord)}}}), (c:Codon {{coordinate:{q(tgt)}}}) CREATE (g)-[:YIELDS_CODON {{c_2_relation_kind:{q(kind)}, c_2_relation_type:'YIELDS_CODON', c_1_relation_family:'correspondential', c_3_dataset_branch:'mahamaya-deep', c_0_source_coordinate:{q(coord)}, c_0_target_coordinate:{q(tgt)}}}]->(c);")
        # USES_PAIR, with its role
        lo, up = c["lower_pair"], c["upper_pair"]
        roles = {pairs[lo]: "lower-and-upper"} if lo == up else {pairs[lo]: "lower", pairs[up]: "upper"}
        stats["set_pairs"] += 1
        lines.append(f"{g}-[r:USES_PAIR]->(p:NucleotidePair) WHERE NOT p.coordinate IN {json.dumps(sorted(roles))} DELETE r;")
        for tgt, kind in sorted(roles.items()):
            lines.append(f"MATCH (g:GenerationEvent {{coordinate:{q(coord)}}}), (p:NucleotidePair {{coordinate:{q(tgt)}}}) "
                         f"MERGE (g)-[r:USES_PAIR]->(p) ON CREATE SET r += {rel(coord, tgt, 'USES_PAIR')} SET r.c_2_relation_kind = {q(kind)};")
        if coord in current:
            rollback.append(f"MATCH (g:GenerationEvent {{coordinate:{q(coord)}}})-[r:USES_PAIR]->() DELETE r;")
            for tgt in cur["pairs"]:
                rollback.append(f"MATCH (g:GenerationEvent {{coordinate:{q(coord)}}}), (p:NucleotidePair {{coordinate:{q(tgt)}}}) CREATE (g)-[:USES_PAIR {{c_2_relation_type:'USES_PAIR', c_1_relation_family:'correspondential', c_3_dataset_branch:'mahamaya-deep', c_0_source_coordinate:{q(coord)}, c_0_target_coordinate:{q(tgt)}}}]->(p);")
        # IS_CLASSIFIED_AS
        cls = CLASSIFIER[c["cell_class"]]
        if cur["classes"] != [cls]:
            stats["set_classes"] += 1
            lines.append(f"{g}-[r:IS_CLASSIFIED_AS]->() DELETE r;")
            lines.append(f"MATCH (g:GenerationEvent {{coordinate:{q(coord)}}}), (k:Classifier {{coordinate:{q(cls)}}}) CREATE (g)-[r:IS_CLASSIFIED_AS]->(k) SET r += {rel(coord, cls, 'IS_CLASSIFIED_AS')};")
            if coord in current:
                rollback.append(f"MATCH (g:GenerationEvent {{coordinate:{q(coord)}}})-[r:IS_CLASSIFIED_AS]->() DELETE r;")
                for old in cur["classes"]:
                    rollback.append(f"MATCH (g:GenerationEvent {{coordinate:{q(coord)}}}), (k:Classifier {{coordinate:{q(old)}}}) CREATE (g)-[:IS_CLASSIFIED_AS {{c_2_relation_type:'IS_CLASSIFIED_AS', c_1_relation_family:'correspondential', c_3_dataset_branch:'mahamaya-deep', c_0_source_coordinate:{q(coord)}, c_0_target_coordinate:{q(old)}}}]->(k);")
        # binaries
        if cur["binaries"] != want_bin and coord in current:
            stats["set_binaries"] += 1
            lines.append(f"{g} SET g.m_3_3_positive_codon_binary = {q(want_bin[0])}, g.m_3_3_negative_codon_binary = {q(want_bin[1])}, "
                         f"g.m_3_3_lower_pair_binary = {q(want_bin[2])}, g.m_3_3_upper_pair_binary = {q(want_bin[3])};")
            rollback.append(f"{g} SET g.m_3_3_positive_codon_binary = {lit(cur['binaries'][0])}, g.m_3_3_negative_codon_binary = {lit(cur['binaries'][1])}, "
                            f"g.m_3_3_lower_pair_binary = {lit(cur['binaries'][2])}, g.m_3_3_upper_pair_binary = {lit(cur['binaries'][3])};")
        lines.append("")

    lines.append("// @apply codon symmetry classes (set only where absent; an existing value must already agree)")
    for seq, coord in sorted(codons.items()):
        cls = codon_class(seq)
        lines.append(f"MATCH (c:Codon {{coordinate:{q(coord)}}}) WHERE c.c_3_symmetry_class IS NULL SET c.c_3_symmetry_class = {q(cls)};")
    existing = dict(query("MATCH (c:Codon) WHERE c.c_3_symmetry_class IS NOT NULL RETURN c.p_3_sequence, c.c_3_symmetry_class"))
    for seq, val in existing.items():
        assert val == codon_class(seq), (seq, val)
    rollback.append("// codon classes that this migration added")
    for seq, coord in sorted(codons.items()):
        if seq not in existing:
            rollback.append(f"MATCH (c:Codon {{coordinate:{q(coord)}}}) REMOVE c.c_3_symmetry_class;")

    lines.append("")
    lines.append("// @apply nucleotide coin values and line names (set only where absent)")
    rollback.append("// nucleotide coin properties")
    for coord, sym in query("MATCH (n:Nucleotide) RETURN n.coordinate, n.c_1_symbol"):
        if sym not in COIN:
            continue
        v, line, pol, mob = COIN[sym]
        lines.append(f"MATCH (n:Nucleotide {{coordinate:{q(coord)}}}) WHERE n.m_3_2_coin_value IS NULL "
                     f"SET n.m_3_2_coin_value = {v}, n.m_3_2_iching_line = {q(line)}, n.m_3_2_polarity = {q(pol)}, n.m_3_2_mobility = {q(mob)};")
        rollback.append(f"MATCH (n:Nucleotide {{coordinate:{q(coord)}}}) REMOVE n.m_3_2_coin_value, n.m_3_2_iching_line, n.m_3_2_polarity, n.m_3_2_mobility;")

    lines += [
        "",
        "// @report",
        "MATCH (g:GenerationEvent) RETURN count(g) AS cells;",
        "MATCH (:GenerationEvent)-[r:YIELDS_CODON]->() RETURN count(r) AS yields;",
        "MATCH (g:GenerationEvent)-[:IS_CLASSIFIED_AS]->(k:Classifier) RETURN k.c_1_name AS class, count(g) AS cells ORDER BY class;",
        "MATCH (c:Codon) RETURN c.c_3_symmetry_class AS class, count(c) AS codons ORDER BY class;",
        "MATCH (g:GenerationEvent) WHERE NOT (g)-[:USES_PAIR]->() OR NOT (g)-[:IS_CLASSIFIED_AS]->() OR NOT ()-[:INITIATES]->(g) RETURN count(g) AS incomplete_cells;",
    ]
    OUT.write_text("\n".join(lines) + "\n")
    ROLLBACK.write_text("\n".join(rollback) + "\n")
    FIELD.write_text(json.dumps({
        "schema": "ql.m3-generation-field/v1",
        "source": f"live Bimba map read {CREATED_AT}; law: migration/bimba-map/generate_m3_generation_cells.py",
        "coin": {k: {"value": v[0], "line": v[1], "polarity": v[2], "mobility": v[3]} for k, v in COIN.items()},
        "trigram_pairs": {str(m): {**PALINDROMIC, **MIXED[m]} for m in (1, 2, 3)},
        "cells": field,
    }, indent=1, ensure_ascii=False) + "\n")
    print(json.dumps(stats))
    print(f"wrote {OUT.name}, {ROLLBACK.name}, {FIELD.relative_to(HERE.parent.parent)}")


if __name__ == "__main__":
    main()
