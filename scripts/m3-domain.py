#!/usr/bin/env python3
"""Build/verify the M3 map-backed domain catalogue using the map audit.

The catalogue is a projection at K2 identities, not a second registry or a new
semantic authority. Source payloads and qualified edges remain source readings.
Native arithmetic is independent; unresolved source assertions are not repaired.
"""
from __future__ import annotations
import argparse
import hashlib
import importlib.util
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PATH = 'fixtures/kernel/m3-domain-v1.json'
SPEC = importlib.util.spec_from_file_location('m3_source', ROOT / 'scripts/m3-source-parity.py')
SOURCE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(SOURCE)


def build(map_path: Path) -> dict:
    registry, read = SOURCE.read_source(map_path)
    projection = SOURCE.project(registry, read)
    audit = SOURCE.Audit(projection).run()
    by_ref = {n['ref']: n for n in projection['nodes']}
    by_address = {h['trigram_derived_address']: h for h in audit['details']['hexagrams']}
    backbones = []
    for node in projection['nodes']:
        if node['role'] != 'clock-backbone':
            continue
        # The map states a governor's codon as EMBODIES_PALINDROMIC_CODON; it states
        # no backbone -> hexagram fact, so that stays an explicit gap.
        embodied = [e for e in projection['relations']
                    if e['from_ref'] == node['ref'] and e['kind'] == 'EMBODIES_PALINDROMIC_CODON']
        codon = by_ref.get(embodied[0]['to_ref']) if len(embodied) == 1 else None
        seq = codon['properties'].get('p_3_sequence') if codon else None
        backbones.append({'id': node['id'], 'ref': node['ref'],
            'codon_id': codon['id'] if codon else None,
            'codon_address': sum('ATCG'.index(n) << s for n, s in zip(seq, (4, 2, 0))) if seq else None,
            'hexagram_id': None, 'hexagram_address': None,
            'source_record': node['record'],
            'standing': 'map-embodied-codon; no map backbone-hexagram fact'})
    result = {'schema': 'ql.m3-domain/v1', 'registry_revision': projection['registry_revision'],
        'source_revision': projection['source_revision'], 'source_files': projection['files'],
        'projection_digest': audit['projection_sha256'],
        'standing': 'source-reading-not-blanket-semantic-parity',
        'nodes': projection['nodes'], 'relations': projection['relations'],
        'hexagrams': audit['details']['hexagrams'], 'matrix_cells': audit['details']['matrices'],
        'clock': audit['details']['clock'], 'backbones': backbones,
        'genetics': audit['details']['genetics'], 'discrepancies': audit['discrepancies']}
    result['catalogue_revision'] = SOURCE.digest(result)
    return result


def canonical(value: object) -> str:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(',', ':')) + '\n'


def verify(data: dict, registry: dict) -> None:
    value = dict(data)
    revision = value.pop('catalogue_revision')
    if SOURCE.digest(value) != revision or data['registry_revision'] != registry['registry_revision']:
        raise ValueError('M3 catalogue/registry revision mismatch')
    nodes = {n['id']: n for n in registry['nodes']}
    relations = {e['id']: e for e in registry['relations']}
    field = {n['id'] for n in registry['nodes'] if n['root_position'] == 3}
    edges = {e['id'] for e in registry['relations'] if e['from_id'] in field}
    if {n['id'] for n in data['nodes']} != field or {e['id'] for e in data['relations']} != edges:
        raise ValueError('incomplete M3 map field')
    if len(data['nodes']) != len(field) or len(data['relations']) != len(edges):
        raise ValueError('duplicate M3 map identity')
    for n in data['nodes']:
        actual = nodes.get(n['id'])
        if actual is None or actual['source_ref'] != n['ref'] or actual['root_position'] != 3 or actual['parent_id'] != n['parent_id']:
            raise ValueError('M3 source node identity mismatch')
    for e in data['relations']:
        actual = relations.get(e['id'])
        if actual is None or any(actual[k] != e[v] for k, v in [('from_id', 'from_id'), ('to_id', 'to_id'), ('source_kind', 'kind'), ('record', 'record')]):
            raise ValueError('M3 source relation identity mismatch')


def c_tables(data: dict) -> str:
    """Compiled finite relations only; semantic node payloads stay in the source reader."""
    def ident(value): return 'UINT64_C(0x' + value + ')' if value else 'UINT64_C(0)'
    def string(value): return json.dumps(value, ensure_ascii=True)
    out = ['/* Generated from ql.m3-domain/v1. Source readings, not readiness claims. */',
        'static const char domain_revision[] = ' + string(data['catalogue_revision']) + ';']
    out.append('static const QL_M3_SourceNode domain_nodes[] = {')
    for n in data['nodes']:
        out.append('{'+','.join([ident(n['id']),str(n['record']),string(n['role'])])+'},')
    out.append('};\nstatic const QL_M3_SourceRelation domain_relations[] = {')
    for e in data['relations']:
        payload = canonical(e['properties']).strip()
        if len(payload.encode()) > 4095:
            raise ValueError('source relation payload exceeds portable C literal; explicit representation needed')
        out.append('{'+','.join([ident(e['id']),ident(e['from_id']),ident(e['to_id']),str(e['record']),string(e['kind']),string(payload)])+'},')
    out.append('};\nstatic const QL_M3_BackboneProjection domain_backbones[] = {')
    for b in data['backbones']:
        out.append('{'+','.join([ident(b['id']),ident(b['codon_id']),ident(b['hexagram_id']),str(b['codon_address'] if b['codon_address'] is not None else 255),str(b['hexagram_address'] if b['hexagram_address'] is not None else 255),str(b['source_record'])])+'},')
    out.append('};')
    by_ref = {n['ref']: n['id'] for n in data['nodes']}
    edge_index = {e['id']: i for i, e in enumerate(data['relations'])}
    links = []
    cells = []
    for cell in data['matrix_cells']:
        # A source role can have multiple qualified assertions. Never squeeze
        # that field into a singular guessed pair or discard duplicate edges.
        values = [ident(cell['id']), ident(by_ref[cell['hexagram_ref']]),
            ident(cell['resolves_relation']), str(cell['family']), str(cell['address']),
            str(len(links)), str(len(cell['pair_relations'])), str(len(cell['codon_relations']))]
        cells.append('{'+','.join(values)+'},')
        links.extend(edge_index[i] for i in cell['pair_relations'] + cell['codon_relations'])
    out.append('static const uint16_t domain_cell_links[] = {' + ','.join(map(str,links)) + '};')
    out.append('static const QL_M3_MatrixCell domain_cells[] = {')
    out.extend(cells)
    out.append('};')
    return '\n'.join(out)+'\n'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--map', type=Path, help='read of the map the registry was built from')
    parser.add_argument('--refresh', action='store_true')
    parser.add_argument('--c-output', type=Path)
    args = parser.parse_args()
    path = ROOT / PATH
    data = build(args.map) if args.map else json.loads(path.read_text())
    verify(data, json.loads((ROOT / SOURCE.REGISTRY).read_text()))
    if args.refresh:
        if not args.map: parser.error('--refresh requires --map')
        path.write_text(canonical(data))
    elif args.map and path.read_text() != canonical(data):
        raise ValueError('source-backed M3 catalogue drift')
    if args.c_output:
        content = c_tables(data)
        args.c_output.parent.mkdir(parents=True, exist_ok=True)
        if not args.c_output.exists() or args.c_output.read_text() != content:
            tmp = args.c_output.with_suffix('.tmp')
            tmp.write_text(content); tmp.replace(args.c_output)
    print(json.dumps({'schema': data['schema'], 'revision': data['catalogue_revision'], 'nodes': len(data['nodes']), 'relations': len(data['relations']), 'backbones': len(data['backbones']), 'matrix_cells': len(data['matrix_cells'])}))


if __name__ == '__main__':
    main()
