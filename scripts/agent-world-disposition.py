#!/usr/bin/env python3
"""Materialise AW0 from source-owned inventories, never from a chosen demo.

This is a read-only projection in the existing capability-matrix family. It has
no runtime registry, dispatcher, source writer or readiness-upgrade operation.
Exact source files must be supplied for a complete native-source audit. A local
repository-only check explicitly reports which native owners were not observed.
"""
from __future__ import annotations

import argparse
import copy
import csv
import hashlib
import json
import re
import sys
from collections import Counter
from pathlib import Path, PurePosixPath

ROOT = Path(__file__).resolve().parents[1]
CARRIER = "docs/integrations/epi-logos/agent-world-disposition.json"
BASE = "docs/integrations/epi-logos/"
FULL = BASE + "TA-ONTA-FULL-FIELD-LOCK.md"
LANG = "docs/kernel-rebuild/VAK-OIKONOMIA-KNOWLEDGE-RETURN.md"
VAK = "data/epi-bimba-map/anuttara-language-map.md"
SCHEMA = "epi-agent-world-disposition/1"
EXPECTED_SP = {f"SP{i}{j}" for i in range(6) for j in range(6)}
REQUIRED_COUNTS = {
    "source-position": 36, "organ": 6, "m-capability": 36,
    "m-s-prime": 36, "deep-capability": 149, "vak-entry": 109,
    "operative-cell": 36, "c-office": 6, "cf": 7, "cfp": 7,
    "cs": 6, "thought": 12, "property": 6, "epii-law": 6,
}


class CoverageError(ValueError):
    """A missing, ambiguous or changed source cannot become a partial success."""


def need(condition: bool, message: str) -> None:
    if not condition:
        raise CoverageError(message)


def git_blob(data: bytes) -> str:
    return hashlib.sha1(f"blob {len(data)}\0".encode() + data).hexdigest()


def table_rows(text: str, first: str) -> list[tuple[int, list[str]]]:
    """Preserve every cell, including the source language's escaped pipe glyphs."""
    result = []
    for number, line in enumerate(text.splitlines(), 1):
        if not line.startswith("|"):
            continue
        cells = [cell.strip() for cell in re.split(r"(?<!\\)\|", line.strip())[1:-1]]
        if cells and re.fullmatch(first, cells[0]):
            result.append((number, cells))
    return result


def section(text: str, heading: str) -> str:
    lines = text.splitlines()
    starts = [i for i, line in enumerate(lines) if line.startswith(heading)]
    need(len(starts) == 1, f"missing or ambiguous heading: {heading}")
    begin = starts[0]
    level = len(lines[begin]) - len(lines[begin].lstrip("#"))
    end = next((i for i in range(begin + 1, len(lines))
                if re.match(r"^#{1," + str(level) + r"} ", lines[i])), len(lines))
    return "\n".join(lines[begin:end])


def load_manifest(root: Path) -> dict:
    config = json.loads((root / CARRIER).read_text())
    path = PurePosixPath(config["source_inventory"])
    need(not path.is_absolute() and ".." not in path.parts, "source inventory escapes the repository")
    with (root / path).open(newline="") as stream:
        reader = csv.DictReader(stream, delimiter="\t")
        need(reader.fieldnames == ["owner", "path", "git_blob"], "source inventory schema changed")
        sources = list(reader)
    config["sources"] = {s["owner"] + ":" + s["path"]: s for s in sources}
    need(len(config["sources"]) == len(sources), "duplicate source inventory entry")
    return config


def project(root: Path, owners: dict[str, Path], *, complete: bool = False,
            manifest: dict | None = None, property_input: dict | None = None) -> dict:
    config = copy.deepcopy(manifest) if manifest is not None else load_manifest(root)
    need(config.get("schema_version") == SCHEMA, "wrong disposition schema")
    need(config.get("counts") == REQUIRED_COUNTS, "independent coverage obligations changed")
    need(set(config.get("source_positions", {})) == EXPECTED_SP,
         "SP00–SP55 must each be assigned exactly once")
    roots = {**owners, "QL-MEF": root}
    observed, missing = [], set()
    source_text = {}
    for key, source in config["sources"].items():
        owner, path = source["owner"], source["path"]
        need(key == f"{owner}:{path}", f"source key drift: {key}")
        logical = PurePosixPath(path)
        need(not logical.is_absolute() and ".." not in logical.parts, f"unsafe source: {key}")
        need(owner in config["repositories"], f"unregistered owner: {owner}")
        need(re.fullmatch(r"[0-9a-f]{40}", config["repositories"][owner]["revision"]) is not None,
             f"owner revision is not exact: {owner}")
        if owner not in roots:
            missing.add(owner)
            continue
        file = roots[owner] / logical
        need(file.is_file(), f"missing required source: {key}")
        data = file.read_bytes()
        need(git_blob(data) == source["git_blob"], f"source blob changed; reconcile explicitly: {key}")
        source_text[key] = data.decode("utf-8")
        observed.append(key)
    need(not complete or not missing, "unobserved native owners: " + ", ".join(sorted(missing)))

    def source_ref(path: str, owner: str = "QL-MEF", selector: str = "whole file") -> dict:
        key = f"{owner}:{path}"
        need(key in config["sources"], f"unregistered source used: {key}")
        return {**config["sources"][key], **config["repositories"][owner], "selector": selector,
                "observed": key in source_text}

    mechanisms = config["mechanisms"]
    for key, mechanic in mechanisms.items():
        need(mechanic["sources"] and mechanic["native_contract"] and mechanic["effects"]
             and mechanic["authority"], f"incomplete native mechanic: {key}")
        for ref in mechanic["sources"] + mechanic["test_sources"]:
            need(ref in config["sources"], f"unbound native source in {key}: {ref}")

    records = []
    def add(identity: str, kind: str, group: str, meaning: object, refs: list[dict],
            binding: dict | None = None, coordinate: str | None = None) -> None:
        disposition = copy.deepcopy(binding if binding is not None else config["groups"][group])
        need(disposition.get("disposition") in {"EPI-GAP", "NATIVE-GAP", "READY-TO-COMPOSE", "ALREADY-REAL", "RESEARCH-ONLY"},
             f"unknown disposition: {identity}")
        need(disposition.get("dependency") and disposition.get("remaining") and disposition.get("mechanisms"),
             f"unassigned work: {identity}")
        if disposition["disposition"] in {"ALREADY-REAL", "READY-TO-COMPOSE", "RESEARCH-ONLY"}:
            need(disposition.get("evidence_receipt") and disposition.get("evidence_scope"),
                 f"readiness/research standing has no separately scoped receipt: {identity}")
        bindings = []
        for key in disposition["mechanisms"]:
            need(key in mechanisms, f"unknown native mechanic in {identity}: {key}")
            mechanic = mechanisms[key]
            bindings.append({"mechanism": key, **mechanic,
                             "source_refs": [source_ref(config["sources"][s]["path"], config["sources"][s]["owner"])
                                             for s in mechanic["sources"] + mechanic["test_sources"]]})
        records.append({
            "id": identity, "kind": kind, "source_meaning": meaning,
            "source_refs": refs, "coordinate": coordinate,
            "disposition": disposition, "native_bindings": bindings,
            "input_result_contract": "The complete source contract below and the named native API together determine typed inputs/results; unresolved Epi operand/dispatch bindings are not invented.",
            "agent_skill_method": "Use the same native Agent/Agency and source-resolved Skills. METHOD: is a Skill classification, not a second registry; no automatic actor creation or invocation.",
            "lifecycle_return": "Reobserve exact source/subject before effect; retain original occasion, actual result/failure and late Return; changed ground requires native receiving/Recognition, never history restamping.",
            "required_evidence": ["positive native composition", "denied effect/no spawn or write",
                                  "invalid operand", "stale revision", "recovery/late Return"],
            "runtime_completion_claim": False,
        })

    full, lang = (root / FULL).read_text(), (root / LANG).read_text()
    sp = table_rows(full, r"SP[0-5][0-5]")
    need({r[1][0] for r in sp} == EXPECTED_SP and len(sp) == 36, "source SP inventory is incomplete/duplicated")
    for line, cells in sp:
        need(len(cells) == 4 and all(cells), f"incomplete SP source row: {line}")
        identity = cells[0]
        i, j = identity[2:]
        path = f"Idea/Bimba/Seeds/S/S{i}/S{i}'/S{i}-{j}'-SPEC.md"
        original = source_text.get("original:" + path)
        meaning = {"current": cells, "historical_contract": original,
                   "historical_standing": "source-defined intent/current-body descriptions are historical, not current acceptance"}
        add(identity, "source-position", "root", meaning,
            [source_ref(FULL, selector=f"line:{line}"), source_ref(path, "original")],
            config["source_positions"][identity], f"historical:S{i}-{j}′")
    for line, cells in table_rows(full, r"S[0-5]′ .+"):
        add(cells[0], "organ", "organ", cells, [source_ref(FULL, selector=f"line:{line}")], coordinate=cells[0].split()[0])
    root_body = section(full, "## 3.")
    # Every paragraph/table in the root section is retained; a handpicked short
    # list of root keywords cannot silently replace this independently held body.
    for index, paragraph in enumerate(re.split(r"\n\s*\n", root_body)):
        if paragraph.strip():
            add(f"root:{index:02}", "root", "root", paragraph,
                [source_ref(FULL, selector=f"section:3/paragraph:{index}")])
    field = json.loads((root / (BASE + "epi-m-capability-field.json")).read_text())
    add("m-field:standing", "retained-standing", "root", field,
        [source_ref(BASE + "epi-m-capability-field.json")])
    for i in range(6):
        path = BASE + f"epi-m-capability-field-m{i}.json"
        domain = json.loads((root / path).read_text())
        need(len(domain["capabilities"]) == 6, f"domain capability loss: M{i}")
        for cap in domain["capabilities"]:
            add(cap["capability_ref"], "m-capability", "m-capability", cap,
                [source_ref(path, selector="capability_ref:" + cap["capability_ref"])], coordinate=cap["m"])
        # Preserve every non-capability field too (Return, source, curriculum).
        add(f"M{i}:domain", "domain-body", "m-capability",
            {k: v for k, v in domain.items() if k != "capabilities"}, [source_ref(path)])
    path = BASE + "epi-ta-onta-agent-world-capabilities.json"
    matrix = json.loads((root / path).read_text())
    for cell in matrix["cells"]:
        add(f"{cell['m']}->{cell['organ']}", "m-s-prime", "m-s-prime", cell,
            [source_ref(path, selector=f"cell:{cell['m']}/{cell['organ']}")], coordinate=cell["m"])
    add("m-s-prime:body", "domain-body", "m-s-prime",
        {k: v for k, v in matrix.items() if k != "cells"}, [source_ref(path)])
    deep_paths = sorted(root.glob("docs/origami work/M*/*-DEEP-CAPABILITY-COORDINATE-MATRIX.json"))
    need(len(deep_paths) == 4, "deep domain inventory changed")
    for file in deep_paths:
        path = file.relative_to(root).as_posix()
        domain = file.parent.name
        content = json.loads(file.read_text())
        for cap in content["capabilities"]:
            add(cap["id"], "deep-capability", f"deep-{domain}", cap,
                [source_ref(path, selector="capability:" + cap["id"])], coordinate=cap.get("coordinate", domain))
        # All recursive coordinate trees, laws, open questions and drift retain
        # their native schema, even when capabilities use only a small subset.
        for key, value in content.items():
            if key != "capabilities":
                add(f"{domain}:deep:{key}", "deep-body", f"deep-{domain}", value,
                    [source_ref(path, selector=key)], coordinate=domain)
    language_rows = table_rows((root / VAK).read_text(), r"`M0[^`]*`")
    need(len(language_rows) == 109, "complete Vāk source must retain 109 entries")
    for line, cells in language_rows:
        need(len(cells) == 8, f"Vāk formulation split or missing cells at {line}")
        add("vak:" + cells[0].strip("`"), "vak-entry", "vak-entry", cells,
            [source_ref(VAK, selector=f"line:{line}")], coordinate=cells[0].strip("`"))
    ops = table_rows(lang, r"(?:@#|-|\+|x|/|=) .+")
    need(len(ops) == 6 and all(len(c) == 7 for _, c in ops), "missing operation/horizon axis")
    for operation, (line, cells) in enumerate(ops):
        for horizon, meaning in enumerate(cells[1:]):
            add(f"operative:{operation}:{horizon}", "operative-cell", "operative-cell",
                {"operation": cells[0], "horizon": horizon, "expected_result": meaning,
                 "operation_ref": f"M0-5-(0/1)-{operation}", "horizon_ref": f"M0-5-(5/0)-{horizon}"},
                [source_ref(LANG, selector=f"line:{line}/column:{horizon + 1}")], coordinate="M0-5")
    for kind, pattern in [("c-office", r"C[0-5]′ / .+"), ("cf", r"CF[1-7]"),
                          ("cfp", r"CFP[0-5]|Z"), ("cs", r"CS[0-5]")]:
        for line, cells in table_rows(lang, pattern):
            add("c-prime:" + cells[0], kind, "c-prime", cells,
                [source_ref(LANG, selector=f"line:{line}")], coordinate=cells[0])
    # Additional CT/CP/participation, paired direction, register/Oikonomia,
    # specialist/Skill law and all original full subform sources remain present.
    for heading in ["## 1.", "## 2.", "### 4.3", "### 5.1", "### 5.2", "### 5.3"]:
        add("language-law:" + heading, "language-body", "c-prime" if "5." not in heading else "property",
            section(lang, heading), [source_ref(LANG, selector=heading)])
    thoughts = table_rows(lang, r"T[0-5]")
    need(len(thoughts) == 6, "thought direct axis is incomplete")
    for line, cells in thoughts:
        need(len(cells) >= 4, f"thought conjugate absent at {line}")
        for face in ("direct", "prime"):
            add(cells[0] + ("′" if face == "prime" else ""), "thought", "thought",
                {"face": face, "paired_source": cells}, [source_ref(LANG, selector=f"line:{line}/{face}")],
                coordinate=cells[0] + ("′" if face == "prime" else ""))
    for line, cells in table_rows(lang, r"#[0-5]"):
        add("property:" + cells[0], "property", "property", cells,
            [source_ref(LANG, selector=f"line:{line}")], coordinate="local:" + cells[0])
    path = BASE + "epi-epii-operational-capacities.json"
    for capacity in json.loads((root / path).read_text())["capacities"]:
        add("epii-on:" + capacity["target"], "epii-law", "epii-law", capacity,
            [source_ref(path, selector=capacity["target"])], coordinate=capacity["target"])
    for key, source in config["sources"].items():
        if source["owner"] == "original" and not re.search(r"S[0-5]-[0-5]'-SPEC.md$", source["path"]):
            add("original:" + source["path"], "original-body", "original",
                {"full_source": source_text.get(key), "observation": "EXACT" if key in source_text else "NOT-SUPPLIED",
                 "interpretation": "Read through the ratified current lock; source variants are not silently equated."},
                [source_ref(source["path"], "original")])
    # Original prose is independently addressable by section, not hidden in one
    # opaque whole-file receipt. Ignore headings inside implementation examples.
    for key, text in source_text.items():
        owner, path = key.split(":", 1)
        if owner != "original" or not path.endswith(".md"):
            continue
        headings, fence = [], None
        for line, value in enumerate(text.splitlines(), 1):
            mark = re.match(r"^\s*(`{3,}|~{3,})", value)
            if mark:
                token = mark.group(1)[0]
                fence = None if fence == token else token if fence is None else fence
            if fence is None and re.match(r"^#{1,6} ", value):
                headings.append((line, value))
        lines = text.splitlines()
        for index, (line, heading) in enumerate(headings):
            end = headings[index + 1][0] - 1 if index + 1 < len(headings) else len(lines)
            body = "\n".join(lines[line - 1:end])
            group = "property" if "property" in path else "original"
            add(f"original-section:{path}:{line}", "original-section", group,
                {"heading": heading, "full_section": body},
                [source_ref(path, owner, f"lines:{line}-{end}")])
    need(not complete or property_input is not None, "complete AW0 needs executed original property tables")
    if property_input is not None:
        source = config["sources"]["original:Body/S/S2/graph-schema/src/lib.rs"]
        need(property_input.get("contract") == "epi.original-property-source/v1"
             and property_input.get("source", {}).get("git_blob") == source["git_blob"],
             "property vocabulary came from a different source")
        vocab = property_input["vocabulary"]
        checksum = hashlib.sha256(json.dumps(vocab, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
        need(checksum == config["property_vocabulary_sha256"] == property_input["vocabulary_sha256"],
             "property vocabulary content changed or omitted definitions")
        for table, owner in [("node_properties", "Node"), ("relationship_properties", "Relationship")]:
            for definition in vocab[table]:
                need(definition["owner"] == owner, "node/relation property ownership changed")
                add(f"source-property:{owner}:{definition['key']}", "source-property", "property", definition,
                    [source_ref(source["path"], "original", f"{table}/{definition['key']}")],
                    coordinate=definition["coordinate_home"])
        for key, value in vocab.items():
            if key not in ("node_properties", "relationship_properties"):
                add("property-source:" + key, "property-source-body", "property", value,
                    [source_ref(source["path"], "original", key)])
    counts = Counter(row["kind"] for row in records)
    need(all(counts[k] == count for k, count in REQUIRED_COUNTS.items()),
         f"coverage differs: {dict(counts)}")
    identities = [row["id"] for row in records]
    need(len(identities) == len(set(identities)), "duplicate full-field identity")
    # The document's 20/40 statement is retained as a source obligation, not
    # retroactively 'proved' by multiplying the seven currently accepted CFs.
    result = {"schema_version": SCHEMA, "standing": config["standing"],
              "repositories": config["repositories"], "records": records,
              "readiness_reconciliation": config["readiness_reconciliation"],
              "counts": dict(sorted(counts.items())), "observed_sources": sorted(observed),
              "unobserved_owners": sorted(missing), "complete_source_audit": not missing and property_input is not None,
              "property_source_execution": property_input,
              "historical_20_40": {"standing": "EPI-GAP", "dependency": "QL-MEF#94/AW2",
                                   "required": "Enumerate source-defined 20 frames/40 directions against current grammar; no manufactured cardinality proof."},
              "runtime_completion_claim": False}
    result["projection_sha256"] = hashlib.sha256(json.dumps(result, ensure_ascii=False, sort_keys=True,
                                                             separators=(",", ":")).encode()).hexdigest()
    return result


def self_test(root: Path, owners: dict[str, Path]) -> int:
    original = load_manifest(root)
    baseline = project(root, owners)
    need(baseline == project(root, dict(reversed(list(owners.items())))), "owner ordering changes output")
    mutations = []
    missing = copy.deepcopy(original); del missing["source_positions"]["SP55"]; mutations.append(missing)
    unknown = copy.deepcopy(original); unknown["source_positions"]["SP00"]["mechanisms"] = ["invented"]; mutations.append(unknown)
    promoted = copy.deepcopy(original); promoted["source_positions"]["SP00"]["disposition"] = "ALREADY-REAL"; mutations.append(promoted)
    changed = copy.deepcopy(original); changed["sources"]["QL-MEF:" + VAK]["git_blob"] = "0" * 40; mutations.append(changed)
    count = copy.deepcopy(original); count["counts"]["vak-entry"] = 108; mutations.append(count)
    escape = copy.deepcopy(original); escape["sources"]["QL-MEF:../private"] = {"owner":"QL-MEF","path":"../private","git_blob":"0"*40}; mutations.append(escape)
    for number, mutation in enumerate(mutations):
        try:
            project(root, owners, manifest=mutation)
        except CoverageError:
            pass
        else:
            raise CoverageError(f"adversarial mutation {number} was accepted")
    need(table_rows("| `M0` | a\\|b |", r"`M0`")[0][1] == ["`M0`", "a\\|b"], "escaped Vāk pipe lost")
    return len(mutations) + 2


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=ROOT)
    parser.add_argument("--owner", action="append", default=[], metavar="NAME=PATH")
    parser.add_argument("--complete", action="store_true")
    parser.add_argument("--property-input", type=Path)
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    try:
        owners = {}
        for item in args.owner:
            name, path = item.split("=", 1)
            need(name not in owners, f"duplicate native owner: {name}")
            owners[name] = Path(path).resolve()
        result = project(args.root.resolve(), owners, complete=args.complete,
                         property_input=json.loads(args.property_input.read_text()) if args.property_input else None)
        tests = self_test(args.root.resolve(), owners) if args.self_test else 0
        if args.output:
            args.output.parent.mkdir(parents=True, exist_ok=True)
            args.output.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n")
        print(json.dumps({"counts": result["counts"], "projection_sha256": result["projection_sha256"],
                          "complete_source_audit": result["complete_source_audit"],
                          "unobserved_owners": result["unobserved_owners"], "regressions_passed": tests,
                          "runtime_completion_claim": False}, indent=2))
        return 0
    except (CoverageError, OSError, ValueError, KeyError) as error:
        print(f"AW0 source disposition failed: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
