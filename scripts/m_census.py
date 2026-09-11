#!/usr/bin/env python3
"""K4 automated M1/M2/M3 pre-vertical coverage census.

Joins the canonical Bimba field (a read-only live Neo4j capture plus the K2
registry's serialized snapshot) to the accepted M ledger, discovers the real C
and Rust implementation bodies coordinate-by-coordinate, and emits:

  * per-root M1/M2/M3 census workbooks (row-level classification),
  * orphan / missing-C-binding / missing-Rust-port / disagreement reports,
  * the bounded work graph consumed by K5 (#129), K6 (#130) and K7 (#131),
  * an updated ``fixtures/kernel/m-ledger-v1.json`` carrying discovered
    implementations, scoped readiness claims, evidence and discrepancies.

Method laws this tool implements:

  * The join is traced through what each Bimba structure IS: coordinate
    identity resolves through the K2 registry (with its one sanctioned M<->#
    root alias and its equal-numeric-path alternate groups), and code anchors
    are matched against the coordinate's own names, labels and inherited
    ancestor names from the Bimba field. Nothing is matched blindly from a
    fixture to a coordinate.
  * A name anchor binds only when every distinctive token of the construct's
    own identifier is explained by the coordinate's self-or-ancestor Bimba
    content, and the deepest such coordinate wins. Incidental name words
    (``arena`` in "Vibrational Arena of Archetypal Powers") cannot bind
    memory machinery to a coordinate: code-generic identifiers are stopped,
    and memory/arena machinery is dispositioned infrastructural.
  * Live-only and serialized-only coordinates are recorded as deltas pending a
    reviewed registry promotion; they are never silently merged.
  * A construct is bound only where the evidence trail is recorded; ambiguous
    constructs stay in the orphan report instead of being forced.
  * Readiness claims in the ledger are exactly what the committed scan
    receipts support: implementation presence, never execution.

Live graph access is read-only. Credentials are operator-supplied through the
environment; no historical default is embedded or copied.
"""

from __future__ import annotations

import argparse
import base64
import datetime
import hashlib
import json
import os
import re
import sys
import urllib.request
from collections import Counter, defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
LEDGER = Path("fixtures/kernel/m-ledger-v1.json")
REGISTRY = Path("fixtures/kernel/m-tree-v1.json")
CENSUS_DIR = Path("fixtures/kernel/census")
LIVE_WORKDIR = Path("target/k4-live")
CENSUS_SCHEMA = "ql.m-census/v1"
CENSUS_ROOTS = ("M1", "M2", "M3")
LIVE_CAPTURE_ARTIFACT = "fixtures/kernel/census/live-bimba-capture-v1.json"
C_DISCOVERY_ARTIFACT = "fixtures/kernel/census/discovery-c-v1.json"
RUST_DISCOVERY_ARTIFACT = "fixtures/kernel/census/discovery-rust-v1.json"

# Registry roots use # spelling; the live graph uses M spelling. This is the
# one alias the K2 registry sanctions (M<N> and #<N> resolve identically,
# including descendants), so it is also the only normalization applied here.

# Words too generic to carry a code<->Bimba join, on either side. The code
# side additionally covers common identifier vocabulary (arena, hash, lut...)
# whose presence in a Bimba name is incidental, not structural.
GENERIC_WORDS = frozenset("""
the and for with without through via from into over under between that these
those this all any each other others not yes one two three four five six seven
eight nine ten zero zeroth first second third new old node nodes system
subsystem matrix matrices field fields state states data set get put print
show info verify init init teardown teardown test tests wrapper dispatch cli
api main root master base base component components structure index registry
module file files function functions table tables entry value values type
types kind kinds name names label labels size buffer buf ctx tmp misc utils
common util helper helpers part parts group groups class order series stage
level layer frame dynamic dynamics process processes principle principles
quality qualities element elements relation relations property properties key
keys core nature essence description meaning role purpose power powers arena
lut hash parse pass ref ptr idx out ret result results input output
primary secondary tertiary general specific deep deeper deepst nested internal
external upper lower upper
""".split())
CODE_ONLY_WORDS = frozenset("""
param params argc argv std libc null true false const static inline extern
struct enum union typedef signed unsigned char int float double void long
short case default goto sizeof return
""".split())


def stem(word: str) -> str:
    if len(word) > 4 and word.endswith("s") and not word.endswith("ss"):
        return word[:-1]
    return word


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def canonical_json(value) -> str:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":"))


def read_json(path: Path):
    return json.loads(path.read_text(encoding="utf-8-sig"))


def write_json(path: Path, value) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def utc_now_iso() -> str:
    return datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def content_lock(root: Path, path: str) -> dict:
    data = (root / path).read_bytes()
    return {"path": path, "sha256": sha256_bytes(data)}


# ---------------------------------------------------------------------------
# Live Neo4j capture (read-only)
# ---------------------------------------------------------------------------


class LiveGraph:
    """Minimal read-only Neo4j HTTP transaction client."""

    def __init__(self, endpoint: str, database: str, user: str, password: str):
        self.url = endpoint.rstrip("/") + f"/db/{database}/tx/commit"
        self.auth = base64.b64encode(f"{user}:{password}".encode()).decode()

    def run(self, statement: str):
        payload = json.dumps({"statements": [{"statement": statement}]}).encode()
        req = urllib.request.Request(self.url, data=payload, headers={
            "Content-Type": "application/json",
            "Authorization": f"Basic {self.auth}",
        })
        with urllib.request.urlopen(req, timeout=300) as resp:
            body = json.loads(resp.read().decode())
        if body.get("errors"):
            raise SystemExit(f"live query failed: {body['errors']}")
        return body


LIVE_STATEMENTS = {
    "identity": "CALL dbms.components() YIELD name, versions, edition "
                "RETURN name, versions, edition",
    "nodes": "MATCH (n) WHERE n.coordinate IS NOT NULL "
             "RETURN id(n) AS exportLocalId, labels(n) AS labels, properties(n) AS properties "
             "ORDER BY exportLocalId",
    "relations": "MATCH (n)-[r]->(m) WHERE n.coordinate IS NOT NULL AND m.coordinate IS NOT NULL "
                 "RETURN id(r) AS exportLocalRelationId, id(n) AS sourceExportLocalId, "
                 "n.coordinate AS sourceCoordinate, type(r) AS relationType, "
                 "properties(r) AS relationProperties, id(m) AS targetExportLocalId, "
                 "m.coordinate AS targetCoordinate ORDER BY exportLocalRelationId",
}


def capture_live(args) -> int:
    password = os.environ.get(args.password_env, "")
    if not password:
        raise SystemExit(
            f"no live password: set ${args.password_env} in the environment; "
            "the census never embeds or copies historical credentials"
        )
    graph = LiveGraph(args.endpoint, args.database, args.user, password)
    started = utc_now_iso()
    raw, counts = {}, {}
    for name, statement in LIVE_STATEMENTS.items():
        body = graph.run(statement)
        raw[name] = body
        counts[name] = len(body["results"][0]["data"]) if body.get("results") else 0
    finished = utc_now_iso()

    LIVE_WORKDIR.mkdir(parents=True, exist_ok=True)
    raw_files = {}
    for name, body in raw.items():
        path = LIVE_WORKDIR / f"live-{name}.json"
        path.write_bytes(json.dumps(body, ensure_ascii=False, separators=(",", ":")).encode())
        raw_files[name] = {"path": str(path), "sha256": sha256_bytes(path.read_bytes())}

    identity = raw["identity"]["results"][0]["data"][0]["row"]
    receipt = {
        "schema": "ql.m-census-live-capture/v1",
        "captured_utc_window": {"started": started, "finished": finished},
        "database": {
            "endpoint": args.endpoint,
            "database": args.database,
            "component": identity[0],
            "versions": identity[1],
            "edition": identity[2],
        },
        "access": "read-only; operator-supplied credentials via environment, never retained",
        "statement_row_counts": counts,
        "raw_responses": raw_files,
        "note": (
            "exportLocalId/exportLocalRelationId are store-local, not canonical kernel IDs. "
            "The node property 'coordinate' is authoritative for this graph; the historical "
            "export spelling 'bimbaCoordinate' is not present in the live schema."
        ),
    }
    write_json(LIVE_WORKDIR / "capture-receipt.json", receipt)
    print(f"captured {counts['nodes']} nodes / {counts['relations']} relations -> {LIVE_WORKDIR}")
    return 0


def load_live_capture(workdir: Path | None = None) -> dict:
    workdir = workdir or LIVE_WORKDIR
    nodes_body = read_json(workdir / "live-nodes.json")
    rels_body = read_json(workdir / "live-relations.json")
    receipt = read_json(workdir / "capture-receipt.json")
    nodes: dict[str, list[dict]] = {}
    for datum in nodes_body["results"][0]["data"]:
        row = datum["row"]
        node = {"exportLocalId": row[0], "labels": row[1], "properties": row[2]}
        coordinate = node["properties"].get("coordinate")
        if isinstance(coordinate, str) and coordinate:
            nodes.setdefault(coordinate, []).append(node)
    relations = []
    for datum in rels_body["results"][0]["data"]:
        row = datum["row"]
        relations.append({
            "exportLocalRelationId": row[0],
            "sourceExportLocalId": row[1],
            "sourceCoordinate": row[2],
            "relationType": row[3],
            "relationProperties": row[4],
            "targetExportLocalId": row[5],
            "targetCoordinate": row[6],
        })
    return {"receipt": receipt, "nodes": nodes, "relations": relations}


def normalize_live_node(live_ref: str, node: dict) -> dict:
    """The join-relevant content of one live coordinate node."""
    properties = node["properties"]
    names = []
    for key in NAME_KEYS:
        value = properties.get(key)
        if isinstance(value, str) and value.strip():
            names.append(value.strip())
    function_text = ""
    for key, value in sorted(properties.items()):
        if key.endswith("architectural_function") and isinstance(value, str):
            function_text = value
            break
    labels = [x for x in node["labels"] if x not in GENERIC_LABELS]
    return {"names": names, "labels": labels,
            "architectural_function": function_text, "live_spelling": live_ref}


def load_live_content() -> tuple[dict, dict, str]:
    """The normalized live Bimba content and its observation receipt.

    Prefers a fresh local capture (``target/k4-live``); otherwise rebuilds
    from the committed capture artifact so the census reruns deterministically
    in CI without a database connection.
    """
    if (LIVE_WORKDIR / "live-nodes.json").exists():
        raw = load_live_capture()
        content = {ref: normalize_live_node(ref, nodes[0])
                   for ref, nodes in raw["nodes"].items()}
        return content, raw["receipt"]
    doc = read_json(ROOT / LIVE_CAPTURE_ARTIFACT)
    return doc["nodes"], doc["source_receipt"]


# ---------------------------------------------------------------------------
# Registry access and coordinate identity
# ---------------------------------------------------------------------------


def load_registry() -> dict:
    return read_json(ROOT / REGISTRY)


def numeric_path(ref: str) -> tuple[str, tuple[int, ...]] | None:
    """Equal-numeric-path identity: root digit plus every numeric part in order.

    The K2 registry retains multiple source spellings of one numeric path as
    explicit alternate-notation groups, so numeric-path equality is the join
    key between live and serialized spellings. Separator form is not identity.
    Prime spellings (``M1'``) are deep-instrument coordinates, not M-tree
    nodes: they carry no numeric-path identity and stay live-only deltas.
    """
    if "'" in ref:
        return None
    match = re.match(r"^[#M]([0-5])", ref)
    if not match:
        return None
    return match.group(1), tuple(int(x) for x in re.findall(r"[0-9]+", ref))


def join_live_to_registry(live: dict, registry: dict) -> dict:
    """Classify every live M-grammar coordinate against the K2 registry."""
    index: dict[tuple, list[dict]] = defaultdict(list)
    for node in registry["nodes"]:
        if node["source_ref"] == "M":
            continue
        key = numeric_path(node["source_ref"])
        if key:
            index[key].append(node)
    live_m = sorted(c for c in live["nodes"] if re.match(r"^M[0-5]", c))
    live_paths = {numeric_path(c) for c in live_m}
    joined, ambiguous, live_only = {}, {}, []
    for coordinate in live_m:
        hits = index.get(numeric_path(coordinate), [])
        if len(hits) == 1:
            joined[coordinate] = hits[0]
        elif len(hits) > 1:
            ambiguous[coordinate] = [n["source_ref"] for n in hits]
        else:
            live_only.append(coordinate)
    serialized_only = sorted(
        n["source_ref"] for n in registry["nodes"]
        if re.match(r"^#[0-5]", n["source_ref"])
        and numeric_path(n["source_ref"]) not in live_paths
    )
    return {
        "live_m_coordinates": live_m,
        "joined": joined,
        "ambiguous": ambiguous,
        "live_only": live_only,
        "serialized_only": serialized_only,
        "by_ref": {n["source_ref"]: n for n in registry["nodes"]},
        "ref_by_id": {n["id"]: n["source_ref"] for n in registry["nodes"]},
        "by_numeric_path": index,
    }


def canonical_resolver(live_join: dict):
    """Registry-canonical ref for any equal-numeric-path spelling."""
    index = live_join["by_numeric_path"]

    def resolve(ref: str) -> str | None:
        key = numeric_path(ref)
        if key is None:
            return ref
        hits = index.get(key) or []
        return hits[0]["source_ref"] if len(hits) == 1 else None

    return resolve


def m_root(source_ref: str) -> str | None:
    match = re.match(r"^[#M]([0-5])", source_ref)
    return f"M{match.group(1)}" if match else None


def in_census_scope(source_ref: str) -> bool:
    return "'" not in source_ref and m_root(source_ref) in CENSUS_ROOTS


# ---------------------------------------------------------------------------
# Bimba anchor content
# ---------------------------------------------------------------------------

NAME_KEYS = (
    "c_1_name", "c_1_sanskrit_name", "c_1_english_name", "c_1_chinese_name",
    "c_1_primary_designation", "name",
)
GENERIC_LABELS = frozenset({"Bimba", "Coordinate", "GraphMeta", "Root", "Subsystem"})


def content_tokens(text: str) -> set[str]:
    """Stemmed, stopword-filtered tokens of a Bimba name or label."""
    out = set()
    for word in re.findall(r"[A-Za-z]+", text.lower()):
        if len(word) >= 3 and word not in GENERIC_WORDS and word not in CODE_ONLY_WORDS:
            out.add(stem(word))
    return out


def camel_split(symbol: str) -> list[str]:
    parts = re.sub(r"([a-z0-9])([A-Z])", r"\1 \2", symbol)
    return [p for p in re.split(r"[^A-Za-z0-9]+", parts) if p]


def symbol_tokens(symbol: str) -> set[str]:
    """Stemmed, stopword-filtered tokens of a code identifier."""
    out = set()
    for part in camel_split(symbol):
        low = part.lower()
        if len(low) >= 3 and low not in GENERIC_WORDS and low not in CODE_ONLY_WORDS:
            out.add(stem(low))
    return out


def anchor_tokens(names: list[str], labels: list[str]) -> set[str]:
    tokens: set[str] = set()
    for text in names:
        tokens |= content_tokens(text)
    for label in labels:
        if label in GENERIC_LABELS:
            continue
        tokens |= content_tokens(label)
        for part in camel_split(label):
            low = part.lower()
            if len(low) >= 3 and low not in GENERIC_WORDS and low not in CODE_ONLY_WORDS:
                tokens.add(stem(low))
    return tokens


class BimbaField:
    """Per-registry-coordinate searchable Bimba content for the join."""

    def __init__(self, registry: dict, live_join: dict, live_content: dict):
        self.registry = registry
        self.join = live_join
        by_ref = live_join["by_ref"]
        self.ref_by_id = live_join["ref_by_id"]
        self.live: dict[str, dict] = {}
        for live_ref, refs in self._live_refs(live_join).items():
            for ref in refs:
                self.live.setdefault(ref, live_content[live_ref])
        self.content: dict[str, dict] = {}
        self.self_tokens: dict[str, frozenset] = {}
        for ref, node in by_ref.items():
            if not in_census_scope(ref):
                continue
            live = self.live.get(ref)
            names = list(node.get("names") or [])
            labels: list[str] = []
            function_text = ""
            if live:
                names.extend(live["names"])
                labels = live["labels"]
                function_text = live["architectural_function"]
            self.content[ref] = {
                "names": names,
                "labels": labels,
                "architectural_function": function_text,
                "live_spelling": live["live_spelling"] if live else None,
            }
            self.self_tokens[ref] = frozenset(anchor_tokens(names, labels))
        # Inherited anchors: an M1-2-0 construct carrying the ancestor name
        # (Ananda) traces to M1-2-0 through its parent's own Bimba content.
        self.ancestor_tokens: dict[str, frozenset] = {}
        for ref in sorted(self.self_tokens, key=lambda r: len(numeric_path(r)[1])):
            tokens = set(self.self_tokens[ref])
            parent_ref = self.parent_of(ref)
            if parent_ref and parent_ref in self.ancestor_tokens:
                tokens |= self.ancestor_tokens[parent_ref]
            self.ancestor_tokens[ref] = frozenset(tokens)

    def _live_refs(self, live_join: dict) -> dict[str, list[str]]:
        refs: dict[str, list[str]] = {}
        for live_ref, node in live_join["joined"].items():
            refs.setdefault(live_ref, []).append(node["source_ref"])
        for live_ref, group in live_join["ambiguous"].items():
            refs.setdefault(live_ref, []).extend(group)
        return refs

    def parent_of(self, ref: str) -> str | None:
        node = self.join["by_ref"].get(ref)
        if not node:
            return None
        return self.ref_by_id.get(node.get("parent_id"))

    def is_ancestor(self, ancestor: str, descendant: str) -> bool:
        walker = descendant
        for _ in range(64):
            walker = self.parent_of(walker)
            if walker is None:
                return False
            if walker == ancestor:
                return True
        return False

    def summary(self, ref: str) -> dict:
        return self.content.get(
            ref, {"names": [], "labels": [], "architectural_function": "", "live_spelling": None})


# ---------------------------------------------------------------------------
# Code discovery (C and Rust)
# ---------------------------------------------------------------------------

C_DIRECTORIES = ("vendor/epi-kernel/reference/src", "c/src")
RUST_DIRECTORIES = ("crates",)
COORDINATE_SPELLING = re.compile(r"[#M][0-5](?:[-./]\(?[0-9]+/?\)?)*")
SPELLING_STRIP = re.compile(r"[()\s]")


def spelling_key(ref: str) -> str:
    """Word-safe spelling of a live coordinate for symbol-presence checks."""
    return SPELLING_STRIP.sub("", ref)
# Translation units whose content is memory/runtime machinery, per the R1
# characterization of the frozen corpus; they carry no coordinate identity.
INFRASTRUCTURAL_C_FILES = {
    "vendor/epi-kernel/reference/src/arena.c": "arena memory machinery",
}
C_KEYWORD_SYMBOLS = frozenset({
    "if", "while", "for", "switch", "return", "sizeof", "Static_assert",
    "_Static_assert", "assert", "defined", "include", "ifndef", "ifdef", "endif",
})


def normalize_spelling(token: str) -> str | None:
    if token.startswith("M"):
        token = "#" + token[1:]
    token = SPELLING_STRIP.sub("", token)
    if re.fullmatch(r"#[0-5](?:[-./][0-9]+)*", token):
        return token
    return None


def extract_c_constructs(rel_path: str, text: str) -> list[dict]:
    """Functions and file-scope const tables with their local comment text."""
    constructs = []
    for pattern, kind in (
        (r"^(?:const\s+)?[A-Za-z_][A-Za-z0-9_ \t\*]*?\**([A-Za-z_][A-Za-z0-9_]*)\s*\([^;]*$", "function"),
        (r"^const\s+[A-Za-z_][A-Za-z0-9_ \t\*]*?\**([A-Z][A-Za-z0-9_]*)\s*(?:\[[0-9]*\])?\s*=", "const-table"),
    ):
        for match in re.finditer(pattern, text, re.MULTILINE):
            symbol = match.group(1)
            if symbol in C_KEYWORD_SYMBOLS:
                continue
            start = match.start()
            window = text[max(0, start - 600):start + 1200]
            comment = " ".join(re.findall(r"/\*+.*?\*+|//[^\n]*", window, re.DOTALL))[:800]
            constructs.append({
                "symbol": symbol, "path": rel_path, "kind": kind,
                "line": text.count("\n", 0, start) + 1, "context": comment,
            })
    return constructs


def extract_rust_constructs(rel_path: str, text: str) -> list[dict]:
    constructs = []
    pattern = re.compile(
        r"^(?:pub(?:\([^)]*\))?\s+)?(fn|struct|enum|const)\s+([A-Za-z_][A-Za-z0-9_]*)",
        re.MULTILINE)
    for match in pattern.finditer(text):
        keyword, symbol = match.groups()
        start = match.start()
        docs = " ".join(re.findall(r"///[^\n]*|//![^\n]*", text[max(0, start - 500):start]))
        constructs.append({
            "symbol": symbol, "path": rel_path, "kind": keyword,
            "line": text.count("\n", 0, start) + 1, "context": docs[:600],
        })
    return constructs


def scan_constructs() -> dict[str, list[dict]]:
    discovered: dict[str, list[dict]] = {"c": [], "rust": []}
    for stratum, directories, extractor, suffixes in (
        ("c", C_DIRECTORIES, extract_c_constructs, (".c", ".h")),
        ("rust", RUST_DIRECTORIES, extract_rust_constructs, (".rs",)),
    ):
        seen: set[str] = set()
        collected: list[dict] = []
        for directory in directories:
            for path in sorted((ROOT / directory).rglob("*")):
                if path.suffix not in suffixes or not path.is_file():
                    continue
                rel = path.relative_to(ROOT).as_posix()
                if rel in seen or "/tests/" in f"/{rel}" or "/benches/" in f"/{rel}":
                    continue
                seen.add(rel)
                text = path.read_text(encoding="utf-8", errors="replace")
                collected.extend(extractor(rel, text))
        if stratum == "c":
            # A header declaration and its translation-unit definition are one
            # construct; keep the definition when both were scanned.
            defined = {c["symbol"] for c in collected if c["path"].endswith(".c")}
            collected = [c for c in collected
                         if c["path"].endswith(".c") or c["symbol"] not in defined]
        discovered[stratum] = collected
    return discovered


def spelling_hits(text: str) -> set[str]:
    hits = set()
    for match in COORDINATE_SPELLING.finditer(text):
        normalized = normalize_spelling(match.group())
        if normalized and in_census_scope(normalized):
            hits.add(normalized)
    return hits


def bind_constructs(constructs: list[dict], field: BimbaField, stratum: str) -> tuple[list[dict], list[dict]]:
    """Bind discovered constructs to M1/M2/M3 coordinates via traced anchors.

    A spelling hit binds the exact coordinate. Otherwise a name anchor binds
    only where the construct's full distinctive token set is explained by that
    coordinate's self-or-ancestor Bimba content; the deepest such coordinate
    wins. Everything else is reported, not forced.
    """
    scope_refs = sorted(field.self_tokens)
    bound, orphans = [], []
    for construct in constructs:
        if stratum == "c" and construct["path"] in INFRASTRUCTURAL_C_FILES:
            orphans.append({**construct, "stratum": stratum,
                            "reason": f"infrastructural: {INFRASTRUCTURAL_C_FILES[construct['path']]}"})
            continue
        text = f"{construct['symbol']}\n{construct['context']}"
        tokens = symbol_tokens(construct["symbol"])
        spellings = spelling_hits(text)
        if not tokens and not spellings:
            orphans.append({**construct, "stratum": stratum,
                            "reason": "no distinctive tokens or coordinate spelling"})
            continue
        direct = {ref for ref in scope_refs if ref in spellings}
        inherited = set()
        if tokens:
            for ref in scope_refs:
                if ref in direct:
                    continue
                explained = field.self_tokens.get(ref, frozenset()) | field.ancestor_tokens.get(ref, frozenset())
                if tokens <= explained:
                    inherited.add(ref)
        # Spelling evidence and fully-explained name anchors both bind; the
        # deepest matched coordinate wins below.
        matched = direct | inherited
        deepest = sorted(
            r for r in matched
            if not any(o != r and field.is_ancestor(o, r) for o in matched)
        )
        if not deepest:
            orphans.append({**construct, "stratum": stratum,
                            "reason": "no token/spelling anchor matched an M1/M2/M3 coordinate",
                            "tokens": sorted(tokens)})
            continue
        if len(deepest) > 12:
            orphans.append({**construct, "stratum": stratum,
                            "reason": f"ambiguous across {len(deepest)} coordinates",
                            "candidates": deepest})
            continue
        trail = []
        for ref in deepest:
            how = "coordinate-spelling" if ref in spellings else "bimba-name-anchor"
            hit_tokens = sorted(tokens & (field.self_tokens.get(ref, frozenset())
                                          | field.ancestor_tokens.get(ref, frozenset())))
            trail.append({"coordinate": ref, "how": how, "matched_tokens": hit_tokens})
        bound.append({**construct, "stratum": stratum, "coordinates": deepest, "trail": trail})
    return bound, orphans


# ---------------------------------------------------------------------------
# Classification
# ---------------------------------------------------------------------------


def classify_coordinates(field: BimbaField, bound: list[dict]) -> dict[str, dict]:
    """Per-coordinate classification for one stratum.

    A coordinate is ``implemented`` when a construct's deepest anchor set
    contains it, ``partial`` when it is only covered through its descendants'
    constructs (aggregate coverage), and ``unimplemented`` otherwise. Every
    construct records its extended coordinates (deepest anchors plus the
    census-scope ancestors they partially realize) so aggregate rows can hold
    legitimate intersecting bindings.
    """
    deepest_owner: dict[str, list[dict]] = defaultdict(list)
    aggregate: dict[str, list[dict]] = defaultdict(list)
    for construct in bound:
        extended = set(construct["coordinates"])
        for ref in list(extended):
            walker = ref
            for _ in range(64):
                walker = field.parent_of(walker)
                if walker is None or not in_census_scope(walker):
                    break
                extended.add(walker)
        construct["extended_coordinates"] = sorted(extended)
        for ref in construct["coordinates"]:
            deepest_owner[ref].append(construct)
        for ref in extended:
            aggregate[ref].append(construct)
    classification = {}
    for ref in field.self_tokens:
        if deepest_owner.get(ref):
            classification[ref] = {"state": "implemented", "constructs": aggregate[ref]}
        elif aggregate.get(ref):
            classification[ref] = {
                "state": "partial",
                "constructs": aggregate[ref],
                "covered_by_descendants": sorted({
                    c["coordinates"][0] for c in aggregate[ref] if ref not in c["coordinates"]
                }),
            }
        else:
            classification[ref] = {"state": "unimplemented", "constructs": []}
    return classification


# ---------------------------------------------------------------------------
# Ledger assembly
# ---------------------------------------------------------------------------


def assessment_profile(c_state: str, rust_state: str, neo4j_indexed: bool) -> tuple[str, dict]:
    readiness = {s: {"status": "unassessed", "warrant": "unassessed", "evidence": []}
                 for s in ("source", "c", "rust", "cpp", "neo4j", "application", "instrument")}
    key_parts = []
    for stratum, state in (("c", c_state), ("rust", rust_state)):
        if state in ("implemented", "partial"):
            readiness[stratum] = {"status": state, "warrant": "implemented",
                                  "evidence": [f"k4-{stratum}-discovery"]}
        elif state == "unimplemented":
            readiness[stratum] = {"status": "unimplemented", "warrant": "unassessed", "evidence": []}
        key_parts.append(f"{stratum}={state}")
    if neo4j_indexed:
        readiness["neo4j"] = {"status": "structural-index-only", "warrant": "observed",
                              "evidence": ["k4-live-capture"]}
        key_parts.append("neo4j=structural-index-only")
    parity = {a: [] for a in ("source", "coordinate", "relation", "operational", "experiential")}
    return "k4:" + ",".join(key_parts), {"readiness": readiness, "parity": parity}


def disposition_for(state: str) -> str:
    return "bound" if state in ("implemented", "partial") else (
        "unimplemented" if state == "unimplemented" else "unassessed")


def impl_id(stratum: str, construct: dict) -> str:
    return f"{stratum}:{construct['path']}:{construct['symbol']}"


def build_census(args) -> int:
    registry = load_registry()
    live_content, live_receipt = load_live_content()
    live_join = join_live_to_registry({"nodes": live_content}, registry)
    field = BimbaField(registry, live_join, live_content)

    discovered = scan_constructs()
    c_bound, c_orphans = bind_constructs(discovered["c"], field, "c")
    rust_bound, rust_orphans = bind_constructs(discovered["rust"], field, "rust")
    c_class = classify_coordinates(field, c_bound)
    rust_class = classify_coordinates(field, rust_bound)

    ledger = read_json(ROOT / LEDGER)
    assessments = dict(ledger["assessments"])
    rows = list(ledger["rows"])
    row_by_id = {r["id"]: r for r in rows}
    implementations = list(ledger["implementations"])
    impl_by_id = {i["id"]: i for i in implementations}

    def add_impl(record: dict) -> str:
        if record["id"] not in impl_by_id:
            implementations.append(record)
            impl_by_id[record["id"]] = record
        return record["id"]

    # Infrastructural C machinery: explicit disposition, no coordinate claims.
    for path, why in INFRASTRUCTURAL_C_FILES.items():
        for symbol in sorted({c["symbol"] for c in discovered["c"] if c["path"] == path}):
            add_impl({
                "id": f"c:{path}:{symbol}",
                "stratum": "c", "kind": "computational",
                "path": path, "symbol": symbol,
                "disposition": "infrastructural",
                "coordinates": [], "relations": [],
                "rationale": f"K4 census: {why}; no coordinate identity (R1 characterization).",
                "structure": [],
            })

    for stratum, bound in (("c", c_bound), ("rust", rust_bound)):
        for construct in bound:
            add_impl({
                "id": impl_id(stratum, construct),
                "stratum": stratum, "kind": "computational",
                "path": construct["path"], "symbol": construct["symbol"],
                "disposition": "coordinate-bound",
                "coordinates": construct.get("extended_coordinates")
                               or construct["coordinates"],
                "relations": [],
                "rationale": "K4 census discovery: " + "; ".join(
                    f"{t['coordinate']} via {t['how']} ({','.join(t['matched_tokens'])})"
                    for t in construct["trail"]),
                "structure": [],
            })

    # --- normalized live capture artifact (committed evidence + CI input) ------
    capture_doc = {
        "schema": "ql.m-census-live-capture/v1",
        "source_receipt": live_receipt,
        "nodes": live_content,
        "joined": {
            node["source_ref"]: live_ref
            for live_ref, node in live_join["joined"].items()
        },
        "ambiguous_alternate_group_joins": live_join["ambiguous"],
        "spelling_keys": sorted({
            spelling_key(r)
            for r in set(live_join["joined"]) | set(live_join["ambiguous"])
        }),
        "live_only_coordinates": live_join["live_only"],
        "serialized_only_coordinates": live_join["serialized_only"],
    }
    write_json(ROOT / LIVE_CAPTURE_ARTIFACT, capture_doc)

    # --- per-coordinate census rows -------------------------------------------
    capture_subjects, c_subjects, rust_subjects = [], [], []
    added_rows = 0
    order = sorted(field.content, key=lambda r: (len(numeric_path(r)[1]), r))
    for ref in order:
        root = m_root(ref)
        c_entry = c_class.get(ref, {"state": "unimplemented", "constructs": []})
        rust_entry = rust_class.get(ref, {"state": "unimplemented", "constructs": []})
        live_spelling = field.summary(ref)["live_spelling"]
        neo4j_indexed = live_spelling is not None
        profile_name, profile = assessment_profile(
            c_entry["state"], rust_entry["state"], neo4j_indexed)
        assessments.setdefault(profile_name, profile)

        row_id = f"census:{ref}"
        bindings = {impl_id("c", c) for c in c_entry["constructs"]}
        bindings |= {impl_id("rust", c) for c in rust_entry["constructs"]}
        if neo4j_indexed:
            # The aggregate live-graph record accrues every coordinate that
            # joins through this live node (unique and alternate-group joins).
            neo4j_id = add_impl({
                "id": f"neo4j:live:{live_spelling}",
                "stratum": "neo4j", "kind": "structural-index",
                "path": LIVE_CAPTURE_ARTIFACT, "symbol": spelling_key(live_spelling),
                "disposition": "coordinate-bound",
                "coordinates": [], "relations": [],
                "rationale": "K4 live Bimba capture: coordinate node observed in the live "
                             f"graph as {live_spelling}; structural presence only.",
                "structure": [],
            })
            if ref not in impl_by_id[neo4j_id]["coordinates"]:
                impl_by_id[neo4j_id]["coordinates"].append(ref)
                impl_by_id[neo4j_id]["coordinates"].sort()
            bindings.add(neo4j_id)
            capture_subjects.append(row_id)
        if c_entry["state"] in ("implemented", "partial"):
            c_subjects.append(row_id)
        if rust_entry["state"] in ("implemented", "partial"):
            rust_subjects.append(row_id)

        # Bindings must intersect the row's coordinates; discovered constructs
        # may bind several coordinates, so the intersection is validated here.
        valid = []
        for binding in sorted(bindings):
            record = impl_by_id[binding]
            if record["disposition"] == "infrastructural" or set(record["coordinates"]) & {ref}:
                valid.append(binding)
        row = {
            "id": row_id,
            "role": "; ".join(dict.fromkeys(field.summary(ref)["names"][:3])) or f"{root} coordinate",
            "scope": ref,
            "coordinates": [ref],
            "source": None,
            "assessment": profile_name,
            "bindings": valid,
            "dispositions": {"c": disposition_for(c_entry["state"]),
                             "rust": disposition_for(rust_entry["state"])},
            "invariants": [],
            "relations": [],
            "dependencies": [],
        }
        if row_id not in row_by_id:
            rows.append(row)
            row_by_id[row_id] = row
            added_rows += 1
        else:
            row_by_id[row_id] = row
            for index, existing in enumerate(rows):
                if existing["id"] == row_id:
                    rows[index] = row
                    break

    # census-row dependency edges: parent coordinate first
    for ref in order:
        row = row_by_id[f"census:{ref}"]
        parent = field.parent_of(ref)
        row["dependencies"] = (
            [f"census:{parent}"] if parent and in_census_scope(parent) and f"census:{parent}" in row_by_id
            else [])

    # --- capability rows: bind discovered implementations + coordinate deps ----
    # Matrix rows carry the matrices' own coordinate spellings; bindings are
    # matched through resolved registry identity, never raw strings.
    resolve = canonical_resolver(live_join)
    for row in rows:
        if row["source"] is None or row["id"].startswith("ql.m-index"):
            continue
        coords = [c for c in row["coordinates"] if in_census_scope(c)]
        if not coords:
            continue
        resolved_row_coords = {resolve(c) for c in row["coordinates"]}
        for stratum, state_map in (("c", c_class), ("rust", rust_class)):
            bound_ids = {b for b in row["bindings"] if b.startswith(f"{stratum}:")}
            all_unimplemented = True
            for ref in coords:
                entry = state_map.get(resolve(ref) or ref,
                                      {"state": "unimplemented", "constructs": []})
                for construct in entry.get("constructs", []):
                    record = impl_by_id[impl_id(stratum, construct)]
                    if {resolve(x) for x in record["coordinates"]} & resolved_row_coords:
                        bound_ids.add(record["id"])
                if entry["state"] != "unimplemented":
                    all_unimplemented = False
            if bound_ids:
                row["bindings"] = sorted(
                    {b for b in row["bindings"] if not b.startswith(f"{stratum}:")} | bound_ids)
                row["dispositions"][stratum] = "bound"
            elif all_unimplemented:
                row["dispositions"][stratum] = "unimplemented"
        row["dependencies"] = sorted(
            set(row["dependencies"])
            | {f"census:{ref}" for ref in coords if f"census:{ref}" in row_by_id})

    # --- evidence --------------------------------------------------------------
    evidence = list(ledger["evidence"])

    def add_evidence(record: dict) -> None:
        # The census owns its evidence records; a re-run refreshes the artifact
        # locks and scoped subjects instead of leaving a stale lock behind.
        for index, existing in enumerate(evidence):
            if existing["id"] == record["id"]:
                evidence[index] = record
                return
        evidence.append(record)

    registry_revision = registry["registry_revision"]
    write_json(ROOT / C_DISCOVERY_ARTIFACT, {
        "schema": "ql.m-census-discovery/v1", "stratum": "c",
        "construct_count": len(discovered["c"]), "bound_count": len(c_bound),
        "orphan_count": len(c_orphans),
        "bound": [{k: v for k, v in c.items() if k != "context"} for c in c_bound],
    })
    write_json(ROOT / RUST_DISCOVERY_ARTIFACT, {
        "schema": "ql.m-census-discovery/v1", "stratum": "rust",
        "construct_count": len(discovered["rust"]), "bound_count": len(rust_bound),
        "orphan_count": len(rust_orphans),
        "bound": [{k: v for k, v in r.items() if k != "context"} for r in rust_bound],
    })
    add_evidence({
        "id": "k4-live-capture", "kind": "observation",
        "artifact": content_lock(ROOT, LIVE_CAPTURE_ARTIFACT),
        "registry_revision": registry_revision,
        "subjects": sorted(set(capture_subjects)), "strata": ["neo4j", "source"],
        "axes": ["coordinate"], "result": "present",
    })
    add_evidence({
        "id": "k4-c-discovery", "kind": "implementation",
        "artifact": content_lock(ROOT, C_DISCOVERY_ARTIFACT),
        "registry_revision": registry_revision,
        "subjects": sorted(set(c_subjects)), "strata": ["c"], "axes": ["operational"],
        "result": "present",
    })
    add_evidence({
        "id": "k4-rust-discovery", "kind": "implementation",
        "artifact": content_lock(ROOT, RUST_DISCOVERY_ARTIFACT),
        "registry_revision": registry_revision,
        "subjects": sorted(set(rust_subjects)), "strata": ["rust"], "axes": ["operational"],
        "result": "present",
    })

    # --- ledger discrepancies for live/serialized spelling divergence ----------
    discrepancies = list(ledger["discrepancies"])
    discrepancy_ids = {d["id"] for d in discrepancies}
    composite_groups = composite_divergence_groups(live_join)
    for group in composite_groups:
        if group["discrepancy_id"] not in discrepancy_ids:
            discrepancies.append({
                "id": group["discrepancy_id"],
                "subjects": group["registry_coordinates"],
                "axis": "coordinate",
                "from_peer": "bimba", "to_peer": "c", "state": "open",
                "detail": json.dumps(group, ensure_ascii=False, sort_keys=True),
                "current_authority": {
                    "peer": "c",
                    "reference": "docs/KERNEL-RECURSIVE-M-REGISTRY.md",
                    "reason": "The K2 registry stays the executable coordinate tree at its "
                              "accepted source lock; the live spelling divergence is retained, "
                              "not merged. Promotion runs through reviewed registry regeneration.",
                },
                "proposal": None, "decision": None, "promotion": None,
                "history": [{"state": "open", "reference": "docs/kernel-rebuild/K4-M123-CENSUS.md"}],
            })

    # --- ledger write -----------------------------------------------------------
    ledger["assessments"] = dict(sorted(assessments.items()))
    ledger["rows"] = rows
    ledger["implementations"] = implementations
    ledger["evidence"] = evidence
    ledger["discrepancies"] = discrepancies
    ledger["ledger_revision"] = hashlib.sha256(
        canonical_json({k: v for k, v in ledger.items() if k != "ledger_revision"}).encode()
    ).hexdigest()
    write_json(ROOT / LEDGER, ledger)

    # --- workbooks, reports, work graph ----------------------------------------
    workbooks = {}
    for root in CENSUS_ROOTS:
        entries = []
        for ref in order:
            if m_root(ref) != root:
                continue
            entries.append({
                "coordinate": ref,
                "live_spelling": field.summary(ref)["live_spelling"],
                "names": field.summary(ref)["names"],
                "labels": field.summary(ref)["labels"],
                "c": summarize_class(c_class.get(ref, {"state": "unimplemented"})),
                "rust": summarize_class(rust_class.get(ref, {"state": "unimplemented"})),
                "row": f"census:{ref}",
            })
        workbooks[root] = entries
        write_json(ROOT / CENSUS_DIR / f"census-{root.lower()}.json", {
            "schema": CENSUS_SCHEMA, "root": root,
            "registry_revision": registry_revision,
            "ledger_revision": ledger["ledger_revision"],
            "live_capture": LIVE_CAPTURE_ARTIFACT,
            "coordinate_count": len(entries),
            "counts": {
                "c": dict(Counter(e["c"]["state"] for e in entries)),
                "rust": dict(Counter(e["rust"]["state"] for e in entries)),
            },
            "coordinates": entries,
        })

    reports = build_reports(field, live_content, live_join, c_orphans, rust_orphans,
                            c_class, rust_class)
    for name, report in reports.items():
        write_json(ROOT / CENSUS_DIR / "reports" / f"{name}.json", report)

    write_json(ROOT / CENSUS_DIR / "work-graph.json", build_work_graph(
        workbooks, registry_revision, ledger["ledger_revision"]))

    print(f"census complete: {added_rows} census rows added; "
          f"C bound {len(c_bound)}/{len(discovered['c'])} (orphans {len(c_orphans)}), "
          f"rust bound {len(rust_bound)}/{len(discovered['rust'])} (orphans {len(rust_orphans)})")
    return 0


def summarize_class(entry: dict) -> dict:
    out = {"state": entry.get("state", "unimplemented")}
    if entry.get("covered_by_descendants"):
        out["covered_by_descendants"] = sorted(set(entry["covered_by_descendants"]))
    if entry.get("constructs"):
        out["constructs"] = sorted({f"{c['path']}:{c['symbol']}" for c in entry["constructs"]})
    return out


# ---------------------------------------------------------------------------
# Reports
# ---------------------------------------------------------------------------


def composite_divergence_groups(live_join: dict) -> list[dict]:
    """Live/serialized spellings of one composite segment that diverge.

    Pair each serialized-only coordinate with the live-only spelling sharing
    the longest common numeric-path prefix; identity is NOT asserted.
    """
    live_only_paths = {numeric_path(r): r for r in live_join["live_only"]
                       if numeric_path(r) is not None}
    groups: dict[tuple, dict] = {}
    for reg_ref in live_join["serialized_only"]:
        reg_path = numeric_path(reg_ref)
        best, best_len = None, 0
        for path, live_ref in live_only_paths.items():
            if path[0] != reg_path[0]:
                continue
            common = 0
            for a, b in zip(path[1], reg_path[1]):
                if a != b:
                    break
                common += 1
            if common > best_len:
                best, best_len = (path, live_ref), common
        if best is None or best_len < 3:
            continue
        key = (reg_path[0], reg_path[1][:best_len])
        entry = groups.setdefault(key, {
            "discrepancy_id": f"k4-live-spelling-composite-m{key[0]}-"
                              + "-".join(str(x) for x in key[1]),
            "common_numeric_prefix": list(key[1]),
            "registry_coordinates": [],
            "live_spellings": [],
        })
        entry["registry_coordinates"].append(reg_ref)
        if best[1] not in entry["live_spellings"]:
            entry["live_spellings"].append(best[1])
    return list(groups.values())


def build_reports(field, live_content, live_join, c_orphans, rust_orphans,
                  c_class, rust_class) -> dict:
    by_ref = live_join["by_ref"]
    live_labels = {ref: content["labels"] for ref, content in live_content.items()}

    live_only_detail = [{
        "live_coordinate": ref,
        "labels": live_labels.get(ref, []),
        "note": "present in the live graph, absent from the K2 registry at its accepted "
                "source lock; pending reviewed registry promotion, not merged by the census",
    } for ref in live_join["live_only"]]

    delta_report = {
        "schema": "ql.m-census-report/v1", "report": "live-serialized-delta",
        "joined_unique": len(live_join["joined"]),
        "ambiguous_alternate_groups": live_join["ambiguous"],
        "live_only": live_only_detail,
        "serialized_only": [{
            "registry_coordinate": ref,
            "names": by_ref[ref].get("names") or [],
            "note": "present in the K2 serialized snapshot, not found in the live capture",
        } for ref in live_join["serialized_only"]],
        "composite_divergence_groups": composite_divergence_groups(live_join),
        "resolution_policy": "recorded, not merged; registry promotion requires the reviewed "
                             "K2 regeneration path with a new source lock",
    }

    ordered = sorted(field.self_tokens, key=lambda r: (len(numeric_path(r)[1]), r))
    missing_c = {
        "schema": "ql.m-census-report/v1", "report": "missing-c-structural-bindings",
        "policy": "coordinates whose Bimba content exists without a discovered native-C body; "
                  "absence here is a bounded work item, not a deletion",
        "coordinates": [{
            "coordinate": ref,
            "names": field.summary(ref)["names"],
            "rust_state": rust_class.get(ref, {}).get("state", "unimplemented"),
        } for ref in ordered if c_class.get(ref, {}).get("state") == "unimplemented"],
    }
    missing_rust = {
        "schema": "ql.m-census-report/v1", "report": "missing-partial-rust-ports",
        "unported": [], "rust_only": []}
    for ref in ordered:
        c_state = c_class.get(ref, {}).get("state")
        rust_state = rust_class.get(ref, {}).get("state")
        if c_state in ("implemented", "partial") and rust_state != "implemented":
            missing_rust["unported"].append({
                "coordinate": ref, "names": field.summary(ref)["names"],
                "c_state": c_state, "rust_state": rust_state,
                "c_bodies": sorted({f"{x['path']}:{x['symbol']}"
                                    for x in c_class[ref].get("constructs", [])}),
            })
        elif rust_state == "implemented" and c_state == "unimplemented":
            missing_rust["rust_only"].append({
                "coordinate": ref, "names": field.summary(ref)["names"],
                "rust_bodies": sorted({f"{x['path']}:{x['symbol']}"
                                       for x in rust_class[ref].get("constructs", [])}),
            })
    orphans = {
        "schema": "ql.m-census-report/v1", "report": "orphan-implementations",
        "policy": "discovered constructs with no confident coordinate binding; the ledger "
                  "forbids orphan implementations, so these remain reported here until seated "
                  "(infrastructural machinery is dispositioned, not orphaned)",
        "c": sorted(c_orphans, key=lambda x: (x["path"], x["symbol"])),
        "rust": sorted(rust_orphans, key=lambda x: (x["path"], x["symbol"])),
    }
    disagreements = {
        "schema": "ql.m-census-report/v1", "report": "bimba-c-rust-disagreements",
        "composite_segment_spelling_divergence": composite_divergence_groups(live_join),
        "ledger_discrepancies": [g["discrepancy_id"] for g in composite_divergence_groups(live_join)],
        "note": "each entry preserves both readings; none is auto-resolved by the census. "
                "Ledger-level discrepancies are emitted only where subjects resolve to "
                "registry coordinates; live-only nodes stay in the delta report.",
    }
    return {
        "live-serialized-delta": delta_report,
        "missing-c-structural-bindings": missing_c,
        "missing-partial-rust-ports": missing_rust,
        "orphan-implementations": orphans,
        "disagreements": disagreements,
    }


# ---------------------------------------------------------------------------
# Work graph
# ---------------------------------------------------------------------------


def build_work_graph(workbooks, registry_revision, ledger_revision) -> dict:
    """Bounded vertical work orders for K5/K6/K7 (#129/#130/#131)."""
    issues = {"M1": 129, "M2": 130, "M3": 131}
    verticals = {}
    for root, entries in workbooks.items():
        frontier = [e["coordinate"] for e in entries
                    if e["c"]["state"] != "implemented" or e["rust"]["state"] != "implemented"]
        verticals[root] = {
            "issue": issues[root],
            "workbook": f"fixtures/kernel/census/census-{root.lower()}.json",
            "coordinate_count": len(entries),
            "counts": {
                "c": dict(Counter(e["c"]["state"] for e in entries)),
                "rust": dict(Counter(e["rust"]["state"] for e in entries)),
            },
            "open_coordinates": len(frontier),
            "next_coordinates": frontier[:24],
            "missing_c_report": "fixtures/kernel/census/reports/missing-c-structural-bindings.json",
            "missing_rust_report": "fixtures/kernel/census/reports/missing-partial-rust-ports.json",
        }
    return {
        "schema": "ql.m-census-work-graph/v1",
        "registry_revision": registry_revision,
        "ledger": "fixtures/kernel/m-ledger-v1.json",
        "ledger_revision": ledger_revision,
        "note": "bounded per-coordinate work. Dependencies are encoded in the M ledger rows: "
                "census rows depend on their parent coordinate, capability rows on their "
                "coordinates; coverage queries expand the requested vertical only.",
        "verticals": verticals,
    }


# ---------------------------------------------------------------------------
# Entry point
# ---------------------------------------------------------------------------


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)

    live = sub.add_parser("capture-live", help="read-only capture of the live Bimba graph")
    live.add_argument("--endpoint", default="http://localhost:7474")
    live.add_argument("--database", default="neo4j")
    live.add_argument("--user", default="neo4j")
    live.add_argument("--password-env", default="EPI_BIMBA_PASSWORD",
                      help="environment variable holding the database password")
    live.set_defaults(func=capture_live)

    census = sub.add_parser("census", help="run the M1/M2/M3 census and update the ledger")
    census.set_defaults(func=build_census)

    args = parser.parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    sys.exit(main())
