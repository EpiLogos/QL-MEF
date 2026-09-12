#!/usr/bin/env python3
"""Current-source liveness and reviewed K8 dispositions over the accepted ledger.

K4's captured workbooks and proof artifacts are historical inputs, not scratch
files to rewrite on every new runtime source addition. Scan current source, retain
all old unresolved constructs, reject new unowned constructs, and publish the
complete current inventory separately from (unchanged) historical readiness.
"""
from __future__ import annotations
import argparse
import hashlib
import importlib.util
import json
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
RECEIPT = 'fixtures/kernel/k8-census-receipt-v1.json'
BINDINGS = 'c/registry/promotions/k8-bindings-v1.json'


def module(name, path):
    spec = importlib.util.spec_from_file_location(name, ROOT / path)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


census = module('m_census', 'scripts/m_census.py')
structure = module('k8_structure', 'scripts/k8-structure.py')


def read(path):
    return json.loads((ROOT / path).read_text())


def sha(data):
    return hashlib.sha256(data).hexdigest()


def canonical(value):
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(',', ':')).encode()


def project():
    registry, _ = structure.project()
    lookup = {n['source_ref']: n['id'] for n in registry['nodes']}
    ledger = read('fixtures/kernel/m-ledger-v1.json')
    old_orphans = read('fixtures/kernel/census/reports/orphan-implementations.json')
    spec = read(BINDINGS)
    if spec['schema'] != 'ql.k8-implementation-dispositions/v1':
        raise ValueError('unknown implementation disposition schema')
    implementations = {}
    for item in ledger['implementations']:
        implementations.setdefault((item['stratum'], item['path'], item['symbol']), []).append(item['id'])
    unresolved = {(peer, i['path'], i['symbol']) for peer in ('c', 'rust') for i in old_orphans[peer]}
    modules = {}
    for owner in spec['modules']:
        path = owner['path']
        if path in modules or not (ROOT / path).is_file():
            raise ValueError('duplicate/missing implementation source: ' + path)
        if owner['stratum'] not in ('c', 'rust', 'cpp'):
            raise ValueError('unsupported implementation stratum')
        if owner['disposition'] not in ('coordinate-bound', 'cross-coordinate', 'infrastructural'):
            raise ValueError('unknown disposition')
        if any(ref not in lookup for ref in owner['coordinates']):
            raise ValueError('new disposition has unresolved coordinate')
        if not owner['rationale'] or not owner['tests'] or any(not (ROOT / test).is_file() for test in owner['tests']):
            raise ValueError('a disposition needs rationale and existing concrete test owners')
        if owner['kind'] not in ('structural-index', 'computational'):
            raise ValueError('unknown implementation kind')
        if owner['disposition'] == 'coordinate-bound' and len(owner['coordinates']) != 1:
            raise ValueError('coordinate-bound requires exactly one coordinate')
        if owner['disposition'] == 'cross-coordinate' and len(owner['coordinates']) < 2:
            raise ValueError('cross-coordinate requires multiple exact coordinates')
        if (owner['disposition'] == 'infrastructural') != (not owner['coordinates']):
            raise ValueError('infrastructure and coordinate bindings are distinct')
        modules[path] = owner
    discovered = census.scan_constructs()
    records = []
    sources = {}
    for peer in ('c', 'rust'):
        for item in discovered[peer]:
            key = (peer, item['path'], item['symbol'])
            entry = {k: item[k] for k in ('path', 'symbol', 'kind', 'line')}
            entry['stratum'] = peer
            sources[item['path']] = sha((ROOT / item['path']).read_bytes())
            if key in implementations:
                entry.update(standing='inherited-ledger-binding', bindings=implementations[key])
            elif item['path'] in modules:
                owner = modules[item['path']]
                if owner['stratum'] != peer:
                    raise ValueError('implementation stratum mismatch')
                entry.update(standing='reviewed-K8-binding', disposition=owner['disposition'], implementation_kind=owner['kind'],
                             coordinates=owner['coordinates'], tests=owner['tests'])
            elif key in unresolved:
                entry.update(standing='inherited-unresolved', disposition='unassessed')
            else:
                raise ValueError('new construct has no reviewed owner/disposition: ' + ':'.join(key))
            records.append(entry)
    # Actual native public exports cover one-line definitions and macro-instanced
    # registry exports that the historical conservative K4 text scan cannot see.
    subprocess.run(['make', '-s', '-C', str(ROOT / 'c'), 'all'], check=True)
    native = subprocess.check_output(['nm', '-P', '--defined-only', str(ROOT / 'c/build/libql-mef-c.a')], text=True)
    exports = {line.split()[0] for line in native.splitlines() if len(line.split()) >= 2 and line.split()[1] == 'T'}
    public = []
    for path, owner in modules.items():
        if owner['stratum'] != 'c':
            continue
        sources[path] = sha((ROOT / path).read_bytes())
        header = owner['header']
        sources[header] = sha((ROOT / header).read_bytes())
        names = sorted(set(re.findall(r'\b(ql_[a-zA-Z0-9_]+)\s*\(', (ROOT / header).read_text())))
        if not names or not set(names) <= exports:
            raise ValueError('current C API is not in the built native kernel: ' + header)
        for symbol in names:
            public.append({'stratum': 'c', 'path': path, 'symbol': symbol,
                           'disposition': owner['disposition'], 'kind': owner['kind'], 'coordinates': owner['coordinates'],
                           'tests': owner['tests'], 'standing': 'linked-public-export-not-runtime-parity'})
    result = {'schema': 'ql.m-current-inventory/v1',
              'registry_revision': registry['registry_revision'],
              'inherited_ledger_revision': ledger['ledger_revision'],
              'inherited_ledger_sha256': sha((ROOT / 'fixtures/kernel/m-ledger-v1.json').read_bytes()),
              'inherited_assessments_sha256': sha(canonical(ledger['assessments'])),
              'dispositions_sha256': sha(canonical(spec)),
              'sources': sources, 'constructs': records, 'native_exports': public,
              'standing': 'current source and linked inventory; no inherited proof retargeted; runtime parity is separate'}
    return result


def summary(inventory):
    return {'schema': 'ql.k8-current-inventory-receipt/v1',
            'inventory_sha256': sha(canonical(inventory)),
            'registry_revision': inventory['registry_revision'],
            'inherited_ledger_revision': inventory['inherited_ledger_revision'],
            'inherited_ledger_sha256': inventory['inherited_ledger_sha256'],
            'constructs': len(inventory['constructs']),
            'inherited_unresolved': sum(c['standing'] == 'inherited-unresolved' for c in inventory['constructs']),
            'reviewed_K8_constructs': sum(c['standing'] == 'reviewed-K8-binding' for c in inventory['constructs']),
            'linked_K8_public_exports': len(inventory['native_exports']),
            'standing': inventory['standing']}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    parser.add_argument('--write-receipt', action='store_true')
    parser.add_argument('--out', type=Path, default=ROOT / 'target/m-ledger/current')
    args = parser.parse_args()
    inventory = project()
    result = summary(inventory)
    if args.write_receipt:
        (ROOT / RECEIPT).write_text(json.dumps(result, indent=2) + '\n')
    if args.check and read(RECEIPT) != result:
        raise ValueError('current source inventory changed; reconcile reviewed bindings/receipt, not historical K4 proofs')
    args.out.mkdir(parents=True, exist_ok=True)
    (args.out / 'inventory.json').write_text(json.dumps(inventory, ensure_ascii=False, indent=2) + '\n')
    (args.out / 'receipt.json').write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps(result))


if __name__ == '__main__':
    main()
