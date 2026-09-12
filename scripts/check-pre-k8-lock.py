#!/usr/bin/env python3
"""Validate the pre-K8 planning lock; this is not runtime acceptance.

The check is offline and read-only. Existing matrix/engine validators remain
independent. --self-test exercises omissions and misbindings in the real docs.
"""
from __future__ import annotations
import argparse
import re
import sys
import unittest
from pathlib import Path
from urllib.parse import unquote

FILES = {
    "wayfinder": "docs/KERNEL-REBUILD-WAYFINDER.md",
    "phase": "docs/kernel-rebuild/PRE-K8-AGENT-WORLD-LOCK.md",
    "field": "docs/integrations/epi-logos/TA-ONTA-FULL-FIELD-LOCK.md",
    "language": "docs/kernel-rebuild/VAK-OIKONOMIA-KNOWLEDGE-RETURN.md",
    "index": "docs/integrations/epi-logos/EPI-CAPABILITY-MATRIX-FIELD-INDEX.md",
}

class PlanError(ValueError):
    pass

def need(ok: bool, message: str) -> None:
    if not ok:
        raise PlanError(message)

def rows(text: str, prefix: str) -> list[list[str]]:
    pattern = r"^\| (?:" + prefix + r") \|"
    return [[c.strip() for c in line.strip().strip("|").split("|")]
            for line in text.splitlines() if re.match(pattern, line)]

def exact(values: list[str], expected: set[str], label: str) -> None:
    need(len(values) == len(set(values)), f"duplicate {label}")
    need(set(values) == expected, f"incomplete {label}: {set(values) ^ expected}")

def check(docs: dict[str, str], root: Path) -> dict[str, int]:
    field, lang, phase = docs["field"], docs["language"], docs["phase"]
    sp = rows(field, r"SP[0-5][0-5]")
    exact([r[0] for r in sp], {f"SP{i}{j}" for i in range(6) for j in range(6)}, "S-prime scope")
    for r in sp:
        need(len(r) == 4 and all(r), f"missing function/binding/evidence: {r[0]}")
        need(f"historical:S{r[0][2]}-{r[0][3]}′" in r[1], f"source identity changed: {r[0]}")
    for name in ("S0′ Khora", "S1′ Hen", "S2′ Pleroma", "S3′ Chronos", "S4′ Anima", "S5′ Aletheia"):
        need(name in field, f"missing organ {name}")
    cp = rows(lang, r"C[0-5]′ / [A-Z]+")
    exact([r[0] for r in cp], {"C0′ / CPF", "C1′ / CT", "C2′ / CP", "C3′ / CF", "C4′ / CFP", "C5′ / CS"}, "C-prime offices")
    need(any(r[0] == "C1′ / CT" and r[1] == "Content Type" for r in cp), "CT is Content Type")
    need(any(r[0] == "C5′ / CS" and r[1] == "Context Sequence" for r in cp), "CS is Context Sequence")
    cf = rows(lang, r"CF[1-7]")
    exact([r[0] for r in cf], {f"CF{i}" for i in range(1, 8)}, "seven CFs")
    expected_cf = ["`(00/00)`", "`(0/1)`", "`(0/1/2)`", "`(0/1/2/3)`", "`(4.0/1-4.4/5)`", "`(4.5/0)`", "`(5/0)`"]
    need([r[1] for r in cf] == expected_cf, "CF identity/order changed")
    need([r[2] for r in cf] == list("CDEFGAB"), "reference diatonic mapping changed")
    thread = rows(lang, r"CFP[0-5]|Z")
    exact([r[0] for r in thread], {f"CFP{i}" for i in range(6)} | {"Z"}, "thread forms")
    sequence = rows(lang, r"CS[0-5]")
    exact([r[0] for r in sequence], {f"CS{i}" for i in range(6)}, "sequences")
    thought = rows(lang, r"T[0-5]")
    exact([r[0] for r in thought], {f"T{i}" for i in range(6)}, "T forms")
    names = [("Question", "Assumption"), ("Trace", "Lacuna"), ("Challenge", "Affordance"), ("Pattern", "Anomaly"), ("Discovery", "Concealment"), ("Insight", "Integration")]
    for i, (r, pair) in enumerate(zip(thought, names)):
        need(len(r) == 4 and r[2] == f"T{i}′", "T-prime pairing changed")
        need(r[1].startswith(pair[0] + ":") and r[3].startswith(pair[1] + ":"), "thought meaning changed")
    props = rows(lang, r"#[0-5]")
    exact([r[0] for r in props], {f"#{i}" for i in range(6)}, "property sixfold")
    need(props[0][1] == "M coordinate / subject ground", "property local #0 must be M coordinate")
    ops = rows(lang, r"@# Potential|- Distinguish|\+ Affirm|x Relate|/ Contextualise|= Express")
    exact([r[0] for r in ops], {"@# Potential", "- Distinguish", "+ Affirm", "x Relate", "/ Contextualise", "= Express"}, "operation axis")
    need(all(len(r) == 7 and all(r) for r in ops), "missing one of 36 language cells")
    acceptance = rows(phase, r"B\d{2}")
    exact([r[0] for r in acceptance], {f"B{i:02}" for i in range(1, 13)}, "B acceptance families")
    for term in ("AW0", "AW1", "AW2", "AW3", "#267", "#217", "owner-machine", "all 109", "= name"):
        need(term.lower() in phase.lower(), f"missing dispatch/coverage obligation: {term}")
    for term in ("produced in NOW", "consumed", "human", "METHOD:", "Oikonomia", "109", "Rupa", "Sattva"):
        need(term.lower() in lang.lower(), f"missing ratified language/lifecycle: {term}")
    need("full planning and capability lock" in docs["wayfinder"], "Wayfinder lacks pre-K8 entry")
    need("#94 AW0–AW3" in docs["wayfinder"], "Wayfinder lacks parallel capability owner")
    links = 0
    for key, text in docs.items():
        source = root / FILES[key]
        need(text.count("```") % 2 == 0, f"unclosed code fence: {source}")
        for target in re.findall(r"(?<!!)\[[^\]\n]+\]\(([^)\n]+)\)", text):
            if target.startswith(("https://", "http://", "mailto:", "#")):
                continue
            resolved = (source.parent / unquote(target.split("#", 1)[0])).resolve()
            need(resolved.is_relative_to(root.resolve()), f"unexpected external local link: {target}")
            need(resolved.is_file(), f"broken local link in {FILES[key]}: {target}")
            links += 1
    return {"source_positions": len(sp), "C_prime_offices": len(cp), "context_frames": len(cf), "thread_forms": len(thread), "sequences": len(sequence), "thought_readings": len(thought) * 2, "property_offices": len(props), "operative_cells": len(ops) * 6, "acceptance_families": len(acceptance), "local_links": links}

def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    root = args.root.resolve()
    try:
        docs = {k: (root / p).read_text(encoding="utf-8") for k, p in FILES.items()}
        counts = check(docs, root)
    except (OSError, PlanError) as exc:
        print(f"pre-K8 plan check failed: {exc}", file=sys.stderr)
        return 1
    print("pre-K8 planning coverage: " + ", ".join(f"{k}={v}" for k, v in counts.items()))
    if args.self_test:
        class MutationTests(unittest.TestCase):
            def reject(self, key: str, old: str, new: str) -> None:
                self.assertIn(old, docs[key])
                changed = dict(docs)
                changed[key] = changed[key].replace(old, new, 1)
                with self.assertRaises(PlanError):
                    check(changed, root)
            def test_missing_source_position(self):
                self.reject("field", "| SP23 |", "| removed-SP23 |")
            def test_wrong_source_identity(self):
                self.reject("field", "historical:S3-2′", "historical:S3-1′")
            def test_wrong_CT(self):
                self.reject("language", "| C1′ / CT | Content Type |", "| C1′ / CT | Time |")
            def test_wrong_CF(self):
                self.reject("language", "| CF6 | `(4.5/0)` |", "| CF6 | `(4/5/0)` |")
            def test_missing_Z(self):
                self.reject("language", "| Z |", "| removed-Z |")
            def test_missing_sequence(self):
                self.reject("language", "| CS4 |", "| removed-CS4 |")
            def test_thought_meaning(self):
                self.reject("language", "| T2′ | Affordance:", "| T2′ | Unknown:")
            def test_property_ground(self):
                self.reject("language", "| #0 | M coordinate / subject ground |", "| #0 | Property bag |")
            def test_missing_language_cell(self):
                self.reject("language", "Locate candidate structures.", "")
            def test_missing_acceptance(self):
                self.reject("phase", "| B09 |", "| removed-B09 |")
            def test_broken_navigation(self):
                self.reject("phase", "](LIVING-INSTRUMENT-ARCHITECTURE.md)", "](missing-approved-world.md)")
        result = unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(MutationTests))
        if not result.wasSuccessful():
            return 1
    print("Planning only: runtime, providers, source mutation and installed-human acceptance remain separate.")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
