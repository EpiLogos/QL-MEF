#!/usr/bin/env python3
"""Compile maqam -> planetary -> chakral/material readings from the Bimba map.

Relations and identities come from the registry (itself built from the map);
literal values come from a map read (scripts/bimba_map.py). `refresh` needs the
read. `check` recomputes when a read of the same map is present, and otherwise
verifies the committed field against the registry and its native rendering.

Only the three explicitly named yantra colours receive colour names; a
presentation palette is separate, and no audible-to-optical conversion exists.
The map carries no full maqam scale spelling, planetary mode or chakral element
literal (the July seed's intervalStructure, planetaryMode and
elementalCorrespondence): those stay empty gaps, so no Bimba-spelled tuning is
claimed and the retained 24-TET intervals remain the kernel's tuning.
"""
from __future__ import annotations
import argparse
import hashlib
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'scripts'))
import bimba_map
OUTPUT = 'fixtures/kernel/m2-correspondences-v1.json'
NATIVE = 'c/src/m2_correspondence_data.inc'
CONTRACT = 'ql.m2-correspondences/v1'
ROLES = ['TONIC_PLANETARY_RESONANCE', 'DOMINANT_PLANETARY_RESONANCE']

def load(path):
    return json.loads(path.read_text(encoding='utf-8-sig'), strict=False)

def compile_field(read):
    reg = load(ROOT / 'fixtures/kernel/m-tree-v1.json')
    catalogue = load(ROOT / 'fixtures/kernel/m2-retained-c-v1.json')
    if read['content_sha256'] != reg['source_revision']:
        raise ValueError('map read is not the map the registry was built from')
    nodes = {n['source_ref']: n for n in reg['nodes']}
    by_ref = {bimba_map.ql_spelling(c): v['properties'] for c, v in read['nodes'].items()
              if bimba_map.is_m_coordinate(c)}
    tables = {t['name']: t for t in catalogue['tables']}
    source = reg['files'][0]

    def claim(ref, prop):
        record = reg['records'][nodes[ref]['records'][0]]
        return {'coordinate': ref, 'node_id': nodes[ref]['id'], 'property': prop,
                'literal': str(by_ref[ref].get(prop, '')), 'path': source['path'],
                'pointer': f"/{record['record_index']}/{prop}", 'sha256': source['sha256']}

    def relations(ref, kind):
        groups = {}
        for r in reg['relations']:
            if r['from_ref'] != ref or r['source_kind'] != kind or r['to_ref'] not in nodes:
                continue
            record = reg['records'][r['record']]
            groups.setdefault(r['to_ref'], []).append({
                'id': r['id'], 'relation_ref': r['relation_ref'], 'kind': kind,
                'from_coordinate': ref, 'to_coordinate': r['to_ref'],
                'path': source['path'], 'sha256': source['sha256'],
                'record_index': record['record_index'], 'payload_sha256': record['payload_sha256']})
        return [(target, sorted(items, key=lambda r: r['id'])) for target, items in sorted(groups.items())]

    result = []
    gaps = []
    for i, ref in enumerate(tables['maqam']['bindings']):
        for role in ROLES:
            links = relations(ref, role)
            if len(links) != 1:
                gaps.append(f'{ref}/{role}: {len(links)} source targets; no implicit choice')
                continue
            planet, musical_links = links[0]
            links = relations(planet, 'PLANETARY_RESONANCE')
            if len(links) != 1:
                gaps.append(f'{planet}/PLANETARY_RESONANCE: {len(links)} source targets; no implicit choice')
                continue
            chakra, planetary_links = links[0]
            if planet not in tables['planet']['bindings'] or chakra not in tables['chakra']['bindings']:
                raise ValueError('unresolved source/native identity in correspondence')
            planet_index = tables['planet']['bindings'].index(planet)
            chakra_index = tables['chakra']['bindings'].index(chakra)
            chakra_row = tables['chakra']['rows'][chakra_index]
            element_id, tattva_index = chakra_row[1:3]
            tattva = tables['tattva']['bindings'][tattva_index] if tattva_index < 36 else None
            fibre = {1: 'air', 2: 'fire', 3: 'water', 4: 'earth'}.get(element_id)
            yantra = str(by_ref[chakra].get('c_1_yantra_form', ''))
            colour = None
            # Exact source phrase admission, not a general keyword-colour guess.
            for phrase, name in [('yellow square', 'yellow'), ('silver crescent moon', 'silver'), ('red triangle', 'red')]:
                if phrase in yantra:
                    colour = name
            result.append({
                'maqam_index': i, 'role': 'tonic' if role == ROLES[0] else 'dominant',
                'maqam_coordinate': ref, 'maqam_node_id': nodes[ref]['id'],
                'maqam_name': str(by_ref[ref].get('c_1_name', '')),
                'planet_index': planet_index, 'planet_coordinate': planet, 'planet_node_id': nodes[planet]['id'],
                'chakra_index': chakra_index, 'chakra_coordinate': chakra, 'chakra_node_id': nodes[chakra]['id'],
                'tattva_coordinate': tattva, 'tattva_node_id': nodes[tattva]['id'] if tattva else None,
                'element_literal': '', 'material_fibre': fibre, 'yantra_literal': yantra,
                'colour_name': colour, 'planetary_mode_literal': '',
                'interval_literal': '', 'spelled_steps24': None,
                'musical_relations': musical_links, 'planetary_relations': planetary_links,
                'claims': [claim(ref, 'c_1_name'), claim(chakra, 'c_1_yantra_form')],
            })
    return {'schema': CONTRACT, 'registry_revision': reg['registry_revision'],
            'source_revision': reg['source_revision'], 'source_repository': reg['source_repository'],
            'standing': 'compiled-source-reading; distinct-from-retained-C-ruler-and-authentic-tuning',
            'source_locks': [{'path': source['path'], 'sha256': source['sha256']}],
            'rules': result, 'gaps': gaps}

def native(field):
    lines = ['/* Generated source reading, not a second registry. */',
             'static const QL_M2_Correspondence m2_correspondences[] = {']
    def ident(value):
        return 'UINT64_C(0x' + value + ')' if value else 'UINT64_C(0)'
    def text(value):
        return json.dumps(value, ensure_ascii=False) if value is not None else 'NULL'
    for r in field['rules']:
        fibre = ['earth', 'fire', 'water', 'air'].index(r['material_fibre']) if r['material_fibre'] else 255
        values = [ident(r[k]) for k in ['maqam_node_id', 'planet_node_id', 'chakra_node_id', 'tattva_node_id']]
        values += [ident(r['musical_relations'][0]['id']), ident(r['planetary_relations'][0]['id'])]
        values += [str(r['maqam_index']), str(int(r['role'] == 'dominant')), str(r['planet_index']), str(r['chakra_index']), str(fibre), str(int(r['spelled_steps24'] is not None))]
        values += ['{' + ','.join(map(str, r['spelled_steps24'] or [0] * 8)) + '}']
        values += [text(r[k]) for k in ['colour_name', 'element_literal', 'planetary_mode_literal', 'interval_literal']]
        lines.append('{' + ','.join(values) + '},')
    lines.append('};\n')
    return '\n'.join(lines)

def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('command', choices=['refresh', 'check'])
    p.add_argument('--map', type=Path, default=bimba_map.CACHE, help='map read (scripts/bimba_map.py read)')
    args = p.parse_args()
    reg = load(ROOT / 'fixtures/kernel/m-tree-v1.json')
    committed = load(ROOT / OUTPUT) if (ROOT / OUTPUT).is_file() else None
    fresh = args.map.is_file() and bimba_map.load(args.map)['content_sha256'] == reg['source_revision']
    if args.command == 'refresh' and not fresh:
        raise SystemExit('refresh needs a read of the map the registry was built from: scripts/bimba_map.py read')
    field = compile_field(bimba_map.load(args.map)) if fresh else committed
    if field is None or field['registry_revision'] != reg['registry_revision']:
        raise SystemExit('stale M2 correspondence: registry revision')
    for path, text in [(OUTPUT, json.dumps(field, ensure_ascii=False, indent=2) + '\n'), (NATIVE, native(field))]:
        file = ROOT / path
        if args.command == 'refresh':
            file.write_text(text, encoding='utf-8')
        elif not file.exists() or file.read_text(encoding='utf-8') != text:
            raise ValueError('stale M2 correspondence: ' + path)
    basis = 'recomputed from the map read' if fresh else 'committed field against registry (no map read)'
    print(f"M2 correspondence: {len(field['rules'])} paths, {len(field['gaps'])} explicit gaps; {basis}")

if __name__ == '__main__':
    main()
