#!/usr/bin/env python3
"""Keep accepted engine receipts immutable; prove a changed integration build anew.

This narrowly admits the reviewed packaging input, not changed numerical engine
sources or edited old receipts. The full native acceptance must run first.
"""
from __future__ import annotations
import hashlib
import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = 'fixtures/kernel/k8-build-lineage-v1.json'


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verify(root: Path, engine: str, proof: dict, fresh: dict, revision: str) -> dict:
    if engine not in ('m2', 'm3'):
        raise ValueError('unsupported preservation scope')
    manifest = json.loads((root / MANIFEST).read_text())
    if manifest['schema'] != 'ql.k8-build-lineage/v1':
        raise ValueError('unknown build lineage')
    historical = f'fixtures/kernel/{engine}-finite-proof-v1.json'
    if digest(root / historical) != manifest['proofs'][engine]:
        raise ValueError('historical engine receipt changed')
    if fresh.get('schema') != f'ql.{engine}-acceptance/v1' or fresh.get('result') != 'passed':
        raise ValueError('complete fresh native acceptance is required')
    if fresh.get('revision') != revision or len(revision) != 40:
        raise ValueError('native acceptance is not for this exact head')
    if set(fresh['inputs']) != set(proof['inputs']) or fresh['checks'] != proof['checks']:
        raise ValueError('native acceptance scope changed')
    admitted = []
    for path, original in proof['inputs'].items():
        current = digest(root / path)
        if fresh['inputs'][path] != current:
            raise ValueError('fresh native acceptance input changed: ' + path)
        if current == original:
            continue
        # Deliberately a single non-numerical input. Generalising this list is
        # not an automatic authorization to alter accepted engine operations.
        if path != 'c/Makefile':
            raise ValueError('changed numerical/source proof input: ' + path)
        change = manifest['integration_input']
        if (change['path'] != path or change['before_sha256'] != original
                or change['after_sha256'] != current
                or digest(root / change['retained_before']) != original):
            raise ValueError('unreviewed integration build change')
        admitted.append(path)
    return {'schema': 'ql.k8-engine-preservation/v1', 'engine': engine,
            'historical_proof_sha256': manifest['proofs'][engine],
            'current_revision': revision, 'current_inputs': fresh['inputs'],
            'result': 'passed', 'integration_changes': admitted,
            'standing': 'fresh full native acceptance; historical proof not retargeted'}


def check(engine: str, root: Path = ROOT) -> None:
    proof = json.loads((root / f'fixtures/kernel/{engine}-finite-proof-v1.json').read_text())
    folder = 'target/m2-receipt' if engine == 'm2' else 'target/m3-acceptance'
    fresh = json.loads((root / folder / 'acceptance.json').read_text())
    revision = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=root, text=True).strip()
    receipt = verify(root, engine, proof, fresh, revision)
    (root / folder / 'k8-preservation.json').write_text(json.dumps(receipt, indent=2) + '\n')
