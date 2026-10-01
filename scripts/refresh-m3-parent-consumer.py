#!/usr/bin/env python3
"""Qualify a separate controlled M3 input against the current source owner.

The historical request and finite-proof receipt stay byte-identical. This
changes only the new request's registry admission, never an expected output,
native determinant, command, event or original occurrence/receipt time.
Run after the existing M3 source/domain refresh. --check performs no writes.
"""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
HISTORICAL = "fixtures/kernel/m3-parent-consumer-v1.json"
PROOF = "fixtures/kernel/m3-finite-proof-v1.json"
CURRENT = "fixtures/kernel/m3-parent-consumer-current-v1.json"
BASIS = "fixtures/kernel/m3-parent-consumer-current-v1.basis.json"
REGISTRY = "fixtures/kernel/m-tree-v1.json"
DOMAIN = "fixtures/kernel/m3-domain-v1.json"
SELF = "scripts/refresh-m3-parent-consumer.py"


def sha(path: str) -> str:
    return hashlib.sha256((ROOT / path).read_bytes()).hexdigest()


def read(path: str) -> dict:
    return json.loads((ROOT / path).read_text())


def render(value: dict) -> str:
    return json.dumps(value, ensure_ascii=False, indent=2) + "\n"


def qualified() -> tuple[dict, dict]:
    historical, proof, registry, domain = map(read, (HISTORICAL, PROOF, REGISTRY, DOMAIN))
    if sha(HISTORICAL) != proof["inputs"][HISTORICAL]:
        raise ValueError("Historical M3 request differs from its retained finite-proof input; refusing requalification")
    if registry["schema"] != "ql.m-tree/v1" or domain["schema"] != "ql.m3-domain/v1":
        raise ValueError("Unsupported current native source projection")
    for key in ("registry_revision", "source_revision"):
        if not registry.get(key) or registry[key] != domain.get(key):
            raise ValueError("Current M3 domain and native registry do not share their exact source basis")
    if not any(source.get("sha256") == registry["source_revision"] for source in domain["source_files"]):
        raise ValueError("Current M3 domain lacks its exact original Bimba source qualification")
    request = json.loads(json.dumps(historical))
    request["request"]["registry_revision"] = registry["registry_revision"]
    # An explicit equality guard keeps requalification separate from silently
    # regenerating producer expectations or changing fixture determinants.
    restored = json.loads(json.dumps(request))
    restored["request"]["registry_revision"] = historical["request"]["registry_revision"]
    if restored != historical:
        raise ValueError("Current controlled input changed more than registry admission")
    basis = {
        "schema": "ql.m3-parent-consumer-fixture-basis/v1",
        "standing": "current-source-qualified controlled lifecycle/transport input; no expected output or live occasion claim",
        "fixture": {"path": CURRENT, "sha256": hashlib.sha256(render(request).encode()).hexdigest()},
        "historical": {"fixture": HISTORICAL, "sha256": sha(HISTORICAL),
                       "registry_revision": historical["request"]["registry_revision"],
                       "finite_proof": PROOF, "finite_proof_sha256": sha(PROOF),
                       "standing": "retained historical acceptance; actual refusal negative under the current source owner"},
        "current_source": {"source_revision": registry["source_revision"],
                           "registry_revision": registry["registry_revision"],
                           "m3_catalogue_revision": domain["catalogue_revision"]},
        "input_locks": [{"path": path, "sha256": sha(path)} for path in (REGISTRY, DOMAIN, SELF)],
        "changed_paths": ["/request/registry_revision"],
        "not_claimed": ["requalified historical finite-proof outputs", "new semantic expectations",
                        "live provider freshness", "personal identity/chakra meaning", "installed experience"],
    }
    return request, basis


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="verify only the separate current fixture/basis; no writes")
    args = parser.parse_args()
    request, basis = qualified()
    for path, value in ((CURRENT, request), (BASIS, basis)):
        text = render(value)
        if args.check:
            if not (ROOT / path).exists() or (ROOT / path).read_text() != text:
                raise SystemExit(f"Current controlled M3 fixture/basis is stale: {path}; refresh its separate qualification")
        else:
            (ROOT / path).write_text(text)
    print("Current controlled M3 input is source-qualified; historical request/proof and all determinants/commands are unchanged")


if __name__ == "__main__":
    main()
