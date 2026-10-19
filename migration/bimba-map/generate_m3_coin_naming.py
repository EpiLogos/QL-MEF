#!/usr/bin/env python3
"""Generate the M3-2 coin-naming corrections and the hexagram codon slots (19 Oct 2026).

Owner, 19 Oct 2026: the wording must be clean, and old and young are the clean,
canonical names (moving versus resting), not case-coded Yin/yin. The nucleotide
prose in the map still reads the pre-ruling scheme: Adenine "the primary Yang"
(Fire, Pingala), Thymine "the primary Yin" (Water, Ida), Cytosine mercurial and
Guanine earthy, with the pairs and pure codons composed from those characters
(expansion, contraction, expression, integration). The ruled coin law (M3-COIN-1)
and the sealed elemental basis (pole/basis.rs) say:

  A = 6 Old Yin    yin,  moving   Water  Cups       feeling
  T = 9 Old Yang   yang, moving   Fire   Wands      intuition
  C = 8 Young Yin  yin,  resting  Earth  Pentacles  sensation
  G = 7 Young Yang yang, resting  Air    Swords     thinking

Every rewritten phrase is composed from that table. Chemistry and biology
sentences are kept as written. Planetary, deity and letter resonances that make
no polarity or element claim are kept.

The same migration rewrites each hexagram's m_3_1_associated_codons (its six
codon slots, matrix by matrix, positive then negative) from the corrected
generation cells (fixtures/kernel/m3-generation-field-v1.json), since the Matrix 1
correction changed 27 of them and some seed lists were incomplete.

Every write is guarded on the exact current value; the rollback restores it.
Read-only against the map (HTTP, accessMode READ). Writes only the .cypher files.
"""
from __future__ import annotations

import json
import urllib.request
from pathlib import Path

HERE = Path(__file__).resolve().parent
ENDPOINT = "http://100.92.62.101:7474/db/neo4j/query/v2"
STAMP = "2026-10-19"
OUT = HERE / f"{STAMP}-m3-coin-naming.cypher"
ROLLBACK = HERE / f"{STAMP}-m3-coin-naming.rollback.cypher"
FIELD = HERE.parent.parent / "fixtures/kernel/m3-generation-field-v1.json"

LAW = {  # line, coin, polarity, mobility, element, suit, function, nucleotide name
    "A": ("Old Yin", 6, "yin", "moving", "Water", "Cups", "feeling", "adenine"),
    "T": ("Old Yang", 9, "yang", "moving", "Fire", "Wands", "intuition", "thymine"),
    "C": ("Young Yin", 8, "yin", "resting", "Earth", "Pentacles", "sensation", "cytosine"),
    "G": ("Young Yang", 7, "yang", "resting", "Air", "Swords", "thinking", "guanine"),
}
MATRIX_OF_PAIR = {  # the base partition each mixed pair belongs to in the generation law
    frozenset("AT"): "Matrix 1 (complementary: A with T, C with G)",
    frozenset("CG"): "Matrix 1 (complementary: A with T, C with G)",
    frozenset("AG"): "Matrix 2 (moving and resting: A with G, T with C)",
    frozenset("CT"): "Matrix 2 (moving and resting: A with G, T with C)",
    frozenset("AC"): "Matrix 3 (same quality: A with C, T with G)",
    frozenset("GT"): "Matrix 3 (same quality: A with C, T with G)",
}
# Sentences of fact kept verbatim from the current descriptions and functions.
KEEP_DESC = {
    "AT": "This is the primary building block of the double helix.",
    "AG": "It unites two purines.",
    "CC": "These CpG sites often serve as regulatory switches through methylation.",
    "CG": "Its triple bonding creates regions of maximum stability. CpG islands mark regulatory regions where genetic expression is carefully controlled.",
    "GC": "Its triple bonding creates regions of stable structure.",
    "GG": "These often form special structures like G-quadruplexes that regulate genetic function.",
}
KEEP_ARCH = {"AT", "GG"}  # AT's function is neutral; GG keeps its G-quadruplex fact (rewritten below)
KEEP_CONSCIOUSNESS = {"AT", "TA"}  # neutral relational readings


def query(statement: str) -> list:
    body = json.dumps({"statement": statement, "accessMode": "READ"}).encode()
    request = urllib.request.Request(ENDPOINT, data=body, headers={"Content-Type": "application/json"})
    reply = json.load(urllib.request.urlopen(request, timeout=120))
    if reply.get("errors"):
        raise SystemExit(reply["errors"])
    return reply["data"]["values"]


def lit(v) -> str:
    return json.dumps(v, ensure_ascii=False)


def clause(x: str, y: str) -> str:
    _, _, px, mx, *_ = LAW[x]
    _, _, py, my, *_ = LAW[y]
    pol = f"{px} stays {px}" if px == py else f"{px} turns to {py}"
    mob = {("moving", "moving"): "both sites move", ("moving", "resting"): "motion comes to rest",
           ("resting", "moving"): "rest is set in motion", ("resting", "resting"): "both sites rest"}[(mx, my)]
    return f"{pol} and {mob}"


def pair_prose(seq: str, current: dict) -> dict:
    x, y = seq
    lx, _, px, mx, ex, _, fx, nx = LAW[x]
    ly, _, py, my, ey, _, fy, ny = LAW[y]
    out = {}
    if x == y:
        out["c_1_primary_designation"] = f"{seq} Dinucleotide - {lx} Doubled"
        out["c_0_energetic_signature"] = f"{lx.lower()}-{ly.lower()}"
        essence = f"{lx} doubled: {px} {mx} at both sites ({ex})"
        out["c_3_quality"] = f"{lx} doubled"
        out["c_0_core_nature"] = f"The double {nx} pair: {lx} ({ex}, {fx}) doubled"
        lead = f"The {seq} pair doubles {lx}: {px}, {mx}, {ex} at both sites. It belongs to every matrix."
        conscious = f"The consciousness of {lx} doubled upon itself ({ex})"
        arch = f"Creates zones of doubled {lx} ({ex}) in the genetic sequence"
    else:
        out["c_1_primary_designation"] = f"{seq} Dinucleotide - {lx} into {ly}"
        out["c_0_energetic_signature"] = f"{lx.lower()}-{ly.lower()}"
        essence = f"{lx} into {ly}: {clause(x, y)} ({ex} into {ey})"
        out["c_3_quality"] = f"{lx} into {ly}"
        out["c_0_core_nature"] = f"The {nx}-{ny} pair, where {lx} ({ex}, {fx}) meets {ly} ({ey}, {fy})"
        lead = f"The {seq} pair joins {lx} ({ex}) to {ly} ({ey}): {clause(x, y)}. It belongs to {MATRIX_OF_PAIR[frozenset(seq)]}."
        conscious = f"The consciousness of {lx} meeting {ly}"
        arch = f"Creates regions where {lx} meets {ly} in the genetic sequence"
    out["c_0_essence"] = essence
    out["c_0_operational_essence"] = essence
    out["c_1_description"] = f"{lead} {KEEP_DESC[seq]}" if seq in KEEP_DESC else lead
    if current.get("c_3_architectural_function"):
        if seq == "GG":
            out["c_3_architectural_function"] = f"{arch}, often forming special structures like G-quadruplexes"
        elif seq not in KEEP_ARCH:
            out["c_3_architectural_function"] = arch
    if current.get("c_1_consciousness_structure") and seq not in KEEP_CONSCIOUSNESS:
        out["c_1_consciousness_structure"] = conscious
    return out


def codon_prose(seq: str, current: dict) -> dict:
    b = seq[0]
    line, _, pol, mob, el, suit, fn, name = LAW[b]
    amino = {"AAA": "It encodes lysine, the positively charged amino acid that reaches out to form connections.",
             "TTT": "It encodes phenylalanine, the amino acid that forms the hydrophobic core of proteins.",
             "CCC": "It encodes proline, the amino acid that creates turns and flexibility in protein structure.",
             "GGG": "It encodes glycine, the simplest and most flexible amino acid that allows maximum structural freedom."}[seq]
    essence = f"{line} tripled: {pol} {mob} at every site ({el})"
    return {
        "c_1_primary_designation": f"Codon {seq} - {line} Trinity",
        "c_0_essence": essence,
        "c_0_operational_essence": essence,
        "c_3_quality": f"{line} tripled",
        "c_0_core_nature": f"The triple {name}: {line} ({el}, {fn}) at all three sites, a perfect palindrome",
        "c_1_description": f"{seq} is {line} taken to its trinitarian extreme: {pol}, {mob}, {el} at every site. {amino}",
    }


NUCLEOTIDE = {
    "A": {
        "c_1_primary_designation": "Adenine - Old Yin (6), the Moving Yin Nucleotide",
        "c_1_description": ("Adenine is Old Yin: the yin nucleotide that moves, valued 6 in the coin cast. Its element is Water "
                            "and its suit is Cups. As a purine with its double-ring structure, it carries greater informational "
                            "complexity. Its partnership with Thymine through double hydrogen bonds creates the foundational "
                            "stability of the DNA double helix while maintaining the flexibility necessary for replication and transcription."),
        "c_1_operational_symbolics": "Functions as Old Yin in the genetic quaternary: yin in motion, feeling that flows (Water)",
        "c_1_key_principles": ["The principle of yin in motion: receptive and flowing",
                               "Double hydrogen bonding creating stable but flexible connections",
                               "Purine structure representing complexity and information density",
                               "The lunar, water aspect of the genetic dance"],
        "c_1_consciousness_structure": "The feeling awareness that flows toward connection",
        "c_0_essence": "The moving yin in genetic architecture: receptive, lunar and flowing (Water)",
        "c_0_operational_essence": "The moving yin in genetic architecture: receptive, lunar and flowing (Water)",
        "c_5_resonances": ["The Aleph of the genetic alphabet - the primary breath",
                           "Old Yin in the Taoist symbol - dark, receptive, in motion",
                           "The element Water in its flowing aspect",
                           "Ida nadi in the yogic system - lunar channel"],
    },
    "T": {
        "c_1_primary_designation": "Thymine - Old Yang (9), the Moving Yang Nucleotide",
        "c_1_description": ("Thymine is Old Yang: the yang nucleotide that moves, valued 9 in the coin cast. Its element is Fire "
                            "and its suit is Wands. Its unique methylation distinguishes DNA from RNA, where uracil takes its place. "
                            "As a pyrimidine with single-ring structure, it is the simpler partner of the A-T pair."),
        "c_1_operational_symbolics": "Functions as Old Yang in the genetic quaternary: yang in motion, intuition that kindles (Fire)",
        "c_1_key_principles": ["The principle of yang in motion: active and kindling",
                               "Single-ring structure representing simplicity and directness",
                               "The unique methylation that distinguishes DNA from RNA",
                               "The solar, fire aspect of the genetic dance"],
        "c_1_consciousness_structure": "The intuitive awareness that kindles and initiates",
        "c_0_essence": "The moving yang in genetic architecture: active, solar and kindling (Fire)",
        "c_0_operational_essence": "The moving yang in genetic architecture: active, solar and kindling (Fire)",
        "c_5_resonances": ["The Beth of the genetic alphabet - the house/container",
                           "Old Yang in the Taoist symbol - bright, active, in motion",
                           "The element Fire in its creative aspect",
                           "Pingala nadi in the yogic system - solar channel"],
    },
    "C": {
        "c_1_primary_designation": "Cytosine - Young Yin (8), the Resting Yin Nucleotide",
        "c_1_description": ("Cytosine is Young Yin: the yin nucleotide that rests, valued 8 in the coin cast. Its element is Earth "
                            "and its suit is Pentacles. Its ability to form triple hydrogen bonds with Guanine creates regions of "
                            "maximum stability in the DNA structure. Cytosine's susceptibility to methylation makes it a key player "
                            "in epigenetic regulation, allowing genetic expression to be modulated without changing the underlying code."),
        "c_1_operational_symbolics": "Functions as Young Yin in the genetic quaternary: yin at rest, the ground that holds (Earth, sensation)",
        "c_1_key_principles": ["The principle of yin at rest: holding and grounding",
                               "Triple hydrogen bonding creating maximum stability",
                               "Methylation sites for epigenetic regulation",
                               "The earth, sensing aspect of the genetic dance"],
        "c_1_consciousness_structure": "The sensing awareness that holds and grounds what is given",
        "c_0_essence": "The resting yin in genetic architecture: holding and grounding (Earth)",
        "c_0_operational_essence": "The resting yin in genetic architecture: holding and grounding (Earth)",
        "c_5_resonances": ["The Gimel of the genetic alphabet - the messenger",
                           "Mercury as divine messenger between realms",
                           "The element Earth in its resting aspect",
                           "Saraswati - the goddess of speech and learning"],
    },
    "G": {
        "c_1_primary_designation": "Guanine - Young Yang (7), the Resting Yang Nucleotide",
        "c_1_description": ("Guanine is Young Yang: the yang nucleotide that rests, valued 7 in the coin cast. Its element is Air "
                            "and its suit is Swords. As a purine, it provides the complex double-ring framework of its pair. The C-G "
                            "pair's triple hydrogen bonding creates regions of exceptional stability, marking areas of particular "
                            "importance in the genetic structure."),
        "c_1_operational_symbolics": "Functions as Young Yang in the genetic quaternary: yang at rest, the clarity that discerns (Air, thinking)",
        "c_1_key_principles": ["The principle of yang at rest: discerning and integrating",
                               "Complex purine structure for information processing",
                               "Triple bonding creating zones of structural importance",
                               "The air, thinking aspect of the genetic dance"],
        "c_1_consciousness_structure": "The thinking awareness that discerns and integrates",
        "c_0_essence": "The resting yang in genetic architecture: discerning and integrating (Air)",
        "c_0_operational_essence": "The resting yang in genetic architecture: discerning and integrating (Air)",
        "c_5_resonances": ["The Daleth of the genetic alphabet - the door/gateway",
                           "The element Air in its discerning aspect",
                           "The clarity that integrates what is given",
                           "Ganesha - the remover of obstacles and lord of beginnings"],
    },
}
# Exact substring replacements on the M3-2 component's q-fields (as the coin-prose migration did).
M32 = [
    ("q_0_nucleotide_charge_ground",
     "Adenine the primary Yang (purine, double-ring, 2 hydrogen bonds, integrals pp 84 / nn −36) and Thymine the primary Yin (pyrimidine, single-ring, pp 96 / nn −24) stand as the Taoist poles — white-active-expanding against black-passive-contracting, Pingala against Ida, Fire against Water.",
     "Adenine, Old Yin 6 (purine, double-ring, 2 hydrogen bonds, integrals pp 84 / nn −36), and Thymine, Old Yang 9 (pyrimidine, single-ring, pp 96 / nn −24), stand as the moving poles — Ida against Pingala, Water against Fire — while Cytosine, Young Yin 8, and Guanine, Young Yang 7, are the poles at rest, Earth and Air."),
    ("q_0_nucleotide_charge_ground",
     "(`Z₄ ≅ Z₂ × Z₂`, purine/pyrimidine × amino/keto)",
     "(`Z₄ ≅ Z₂ × Z₂`: yin/yang × moving/resting, chemically amino/keto × weak/strong hydrogen bonding; purine/pyrimidine is their difference)"),
    ("q_1_sixty_four_codon_alphabet", 'Codon_AAA the "Pure Yang Trinity"', 'Codon_AAA the "Old Yin Trinity"'),
    ("q_4_language_life_writes_itself",
     "position 1 = real = A = Material cause = Earth = Sensation; i = U = Efficient = Water = Feeling; j = G = Formal = Air = Thinking; k = C = Context = Fire = Intuition.",
     "position 1 = real = C = Material cause = Earth = Sensation; i = T (U) = Context = Fire = Intuition; j = A = Efficient = Water = Feeling; k = G = Formal = Air = Thinking."),
]


def main() -> None:
    nodes = {}
    for coord, labels, props in query(
        "MATCH (n) WHERE n:Nucleotide OR n:NucleotidePair OR (n:Codon AND n.p_3_sequence IN ['AAA','TTT','CCC','GGG']) "
        "RETURN n.coordinate, labels(n), properties(n)"
    ):
        nodes[coord] = (labels, {k: v for k, v in props.items() if not k.startswith("embedding")})
    lines = ["// M3-2 coin naming: the nucleotide, pair and pure-codon wording follows the ruled coin law,",
             "// and each hexagram's six codon slots follow the corrected generation cells.",
             "// Generated by migration/bimba-map/generate_m3_coin_naming.py; do not hand-edit.",
             "// Old and young are the canonical names (moving versus resting): A 6 Old Yin, T 9 Old Yang,",
             "// C 8 Young Yin, G 7 Young Yang.", ""]
    rollback = ["// Rollback of the M3-2 coin-naming corrections: restores every value exactly as read.", ""]
    changes = 0

    def set_prop(coord: str, key: str, old, new) -> None:
        nonlocal changes
        if old == new:
            return
        changes += 1
        guard = f"n.{key} IS NULL" if old is None else f"n.{key} = {lit(old)}"
        lines.append(f"MATCH (n {{coordinate:{lit(coord)}}}) WHERE {guard} SET n.{key} = {lit(new)};")
        rollback.append(f"MATCH (n {{coordinate:{lit(coord)}}}) WHERE n.{key} = {lit(new)} SET n.{key} = {lit(old)};")

    lines.append("// @apply nucleotides")
    for coord, (labels, p) in sorted(nodes.items()):
        if "Nucleotide" in labels and p.get("c_1_symbol") in NUCLEOTIDE:
            for k, v in NUCLEOTIDE[p["c_1_symbol"]].items():
                set_prop(coord, k, p.get(k), v)
    lines.append("")
    lines.append("// @apply pairs")
    for coord, (labels, p) in sorted(nodes.items()):
        if "NucleotidePair" in labels:
            for k, v in pair_prose(p["p_3_sequence"], p).items():
                set_prop(coord, k, p.get(k), v)
    lines.append("")
    lines.append("// @apply pure codons")
    for coord, (labels, p) in sorted(nodes.items()):
        if "Codon" in labels:
            for k, v in codon_prose(p["p_3_sequence"], p).items():
                set_prop(coord, k, p.get(k), v)
    lines.append("")
    lines.append("// @apply M3-2 ground, alphabet and purpose")
    m32 = query("MATCH (n {coordinate:'M3-2'}) RETURN properties(n)")[0][0]
    for key, old, new in M32:
        if old not in m32[key] and new in m32[key]:
            continue  # already applied
        assert old in m32[key], (key, old)
        changes += 1
        lines.append(f"MATCH (n {{coordinate:'M3-2'}}) WHERE n.{key} CONTAINS {lit(old)} SET n.{key} = replace(n.{key}, {lit(old)}, {lit(new)});")
        rollback.append(f"MATCH (n {{coordinate:'M3-2'}}) WHERE n.{key} CONTAINS {lit(new)} SET n.{key} = replace(n.{key}, {lit(new)}, {lit(old)});")
    lines.append("")
    lines.append("// @apply hexagram codon slots (m_3_1_associated_codons), matrix by matrix, positive then negative")
    cells = json.loads(FIELD.read_text())["cells"]
    slots: dict[str, list[str]] = {}
    for c in sorted(cells, key=lambda c: (c["hexagram"], c["matrix"])):
        slots.setdefault(c["hexagram"], []).extend([f"{c['positive']}:{c['matrix']}", f"{c['negative']}:{c['matrix']}"])
    for coord, current in query("MATCH (h:Hexagram) RETURN h.coordinate, h.m_3_1_associated_codons"):
        set_prop(coord, "m_3_1_associated_codons", current, slots[coord])
    lines += ["", "// @report",
              "MATCH (n:Bimba) WHERE (n:Nucleotide OR n:NucleotidePair OR n:Codon) UNWIND keys(n) AS k WITH n, k WHERE n[k] IS :: STRING AND (n[k] CONTAINS 'Pure Yang' OR n[k] CONTAINS 'Pure Yin' OR n[k] CONTAINS 'primary yang' OR n[k] CONTAINS 'primary yin' OR n[k] CONTAINS 'mercury' OR n[k] = 'yang-yang') RETURN count(*) AS old_scheme_strings;",
              "MATCH (n:Nucleotide) RETURN n.c_1_symbol, n.c_1_primary_designation ORDER BY n.coordinate;"]
    OUT.write_text("\n".join(lines) + "\n")
    ROLLBACK.write_text("\n".join(["// @apply restore"] + rollback) + "\n")
    print(f"{changes} guarded writes -> {OUT.name}; rollback -> {ROLLBACK.name}")


if __name__ == "__main__":
    main()
