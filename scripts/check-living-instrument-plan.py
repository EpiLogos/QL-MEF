#!/usr/bin/env python3
"""Check the post-K7 authored plan, never runtime or experiential readiness.

No source matrices, registry, ledger, evidence or issue states are modified.
The finite checks protect the adopted aperture table and document navigation.
"""
from __future__ import annotations

import argparse
import json
import re
from fractions import Fraction
from pathlib import Path
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parents[1]
WAYFINDER = "docs/KERNEL-REBUILD-WAYFINDER.md"
PARENT = "docs/kernel-rebuild/PARENT-SURFACES-INTEGRATION.md"
APERTURES = "docs/kernel-rebuild/APERTURES-AND-CLOCK-CENTRE.md"
ARCHITECTURE = "docs/kernel-rebuild/LIVING-INSTRUMENT-ARCHITECTURE.md"
PATHS = (WAYFINDER, PARENT, APERTURES, ARCHITECTURE)
ACCEPTED = (
    "bb32250109417e46ee37b9d253e7ae115234ec24",
    "c421080371838c87f25ecee3fd53561f75a035f0",
    "276f23e46c625a83c5dee7c1fc64f32761333ac6",
)
STATIC = (1, 2, 4, 8, 9, 10, 12, 15, 24, 30, 36, 40, 45, 90, 180, 360)
PAIR = re.compile(
    r"^\| M2-0-2-(\d+) \| M2-0-2-(\d+)-0 / (\d+)° × (\d+) / (\d+)"
    r" \| M2-0-2-(\d+)-1 / (\d+)° × (\d+) / (\d+) \|$", re.MULTILINE
)
LINK = re.compile(r"\[[^\]\n]+\]\(([^)\n]+)\)")


class PlanError(ValueError):
    """A source-plan invariant or local navigation link is invalid."""


def require(condition: bool, message: str) -> None:
    if not condition:
        raise PlanError(message)


def validate_apertures(text: str) -> None:
    rows = PAIR.findall(text)
    require(len(rows) == 8, "aperture table must have eight complete pair rows")
    seen: set[int] = set()
    for raw in rows:
        p, left, d, count, index, right, rd, rcount, rindex = map(int, raw)
        require(0 <= p < 8 and p not in seen, "duplicate or invalid pair identity")
        seen.add(p)
        require(left == p == right, "lens leaves must belong to their pair")
        require(index == p and rindex == 15 - p, "native lens identity changed")
        require(d == STATIC[p] and rd == STATIC[15 - p], "division law changed")
        require(d * count == 360 and rd * rcount == 360, "invalid lens partition")
        require(d * rd == 360, "pair is not reciprocal")
    require(seen == set(range(8)), "missing pair identity")
    for coordinate in ("M2-0-0", "M2-0-1", "M2-0-2", "M3-5-5/0"):
        require(coordinate in text, f"missing architectural identity {coordinate}")
    # Exact independent witnesses for the adopted three-grid relationships.
    quanta = (Fraction(6), Fraction(20), Fraction(45, 2))
    for a, b, expected in ((0, 1, 60), (0, 2, 90), (1, 2, 180)):
        multiples = [n * quanta[a] for n in range(1, 61)
                     if (n * quanta[a] / quanta[b]).denominator == 1]
        require(min(multiples) == expected, "three-grid closure changed")
    require(quanta[2] / quanta[1] == Fraction(9, 8), "epogdoon witness changed")


def heading_ids(text: str) -> set[str]:
    ids: set[str] = set()
    counts: dict[str, int] = {}
    fenced = False
    for line in text.splitlines():
        if line.startswith("```"):
            fenced = not fenced
        if fenced or not re.match(r"^#{1,6} ", line):
            continue
        label = re.sub(r"^#{1,6} ", "", line).strip().lower()
        slug = re.sub(r"[^\w\- ]", "", label).replace(" ", "-")
        ordinal = counts.get(slug, 0)
        counts[slug] = ordinal + 1
        ids.add(slug if ordinal == 0 else f"{slug}-{ordinal}")
    return ids


def validate_links(root: Path, path: str, text: str) -> int:
    total = 0
    for raw in LINK.findall(text):
        url = urlsplit(raw)
        if url.scheme or url.netloc:
            continue
        target = ((root / path).parent / unquote(url.path)).resolve() if url.path else (root / path).resolve()
        require(target.is_relative_to(root), f"{path}: link escapes repository: {raw}")
        require(target.is_file(), f"{path}: missing link target: {raw}")
        if url.fragment:
            require(target.suffix == ".md", f"{path}: unsupported local fragment: {raw}")
            require(unquote(url.fragment) in heading_ids(target.read_text(encoding="utf-8")),
                    f"{path}: missing heading: {raw}")
        total += 1
    return total


def validate(root: Path) -> dict[str, object]:
    root = root.resolve()
    documents = {path: (root / path).read_text(encoding="utf-8") for path in PATHS}
    for path in (WAYFINDER, PARENT):
        for sha in ACCEPTED:
            require(sha in documents[path], f"{path}: accepted engine receipt missing")
    validate_apertures(documents[APERTURES])
    cases = re.findall(r"^\| (A\d{2}) \|", documents[PARENT], re.MULTILINE)
    require(cases == [f"A{i:02}" for i in range(1, 19)], "acceptance cases must retain A01–A18 once each")
    for path, text in documents.items():
        require("0→1" in text, f"{path}: enacted-slash correction missing")
    for package in ("K8.0", "K8.1", "K8.2", "K8.3", "K9.0", "K9.1", "K9.2", "K9.3",
                    "K10.0", "K10.1", "K10.2", "K10.3", "K10.4"):
        require(package in documents[WAYFINDER], f"missing dispatch package {package}")
    links = sum(validate_links(root, path, text) for path, text in documents.items())
    return {"result": "passed", "scope": "authored-plan-integrity-only", "documents": len(documents),
            "local_links": links, "aperture_identities": 18, "pair_containers": 8,
            "acceptance_cases": len(cases), "runtime_acceptance": "not-claimed"}


def mutation_checks(root: Path) -> int:
    original = (root / APERTURES).read_text(encoding="utf-8")
    mutations = (
        ("M2-0-2-0-0 / 1° × 360 / 0", "M2-0-2-0-0 / 1° × 360 / 1"),
        ("M2-0-2-7-1 / 24° × 15 / 8", "M2-0-2-7-1 / 24° × 15 / 9"),
        ("M2-0-2-3-0 / 8° × 45 / 3", "M2-0-2-3-0 / 8° × 44 / 3"),
        ("M2-0-2-6-1 / 30° × 12 / 9", "M2-0-2-5-1 / 30° × 12 / 9"),
        ("| M2-0-2-7 |", "| M2-0-2-6 |"),
    )
    for before, after in mutations:
        require(before in original, f"mutation witness vanished: {before}")
        try:
            validate_apertures(original.replace(before, after, 1))
        except PlanError:
            continue
        raise PlanError(f"invalid aperture mutation accepted: {after}")
    return len(mutations)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=ROOT)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    try:
        result = validate(args.root)
        if args.self_test:
            result["rejected_mutations"] = mutation_checks(args.root.resolve())
    except (OSError, ValueError) as error:
        parser.exit(1, f"instrument plan: {error}\n")
    print(json.dumps(result, ensure_ascii=False, sort_keys=True))


if __name__ == "__main__":
    main()
