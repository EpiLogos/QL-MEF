#!/usr/bin/env python3
"""Compile source-attributed maqam -> planetary -> chakral/material readings.

This is a projection of the accepted K2 identities and locked Bimba statements,
not a new tree. Repeated source relation assertions retain every ID. Only the
three explicitly named yantra colours in this source cut receive colour names;
a presentation palette is separate, and no audible-to-optical conversion exists.
"""
from __future__ import annotations
import argparse
import hashlib
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUTPUT = 'fixtures/kernel/m2-correspondences-v1.json'
NATIVE = 'c/src/m2_correspondence_data.inc'
NODE_PATH = 'Idea/Bimba/Map/datasets/parashakti-deep/nodes-full-detail.json'
CONTRACT = 'ql.m2-correspondences/v1'
ROLES = ['TONIC_PLANETARY_RESONANCE', 'DOMINANT_PLANETARY_RESONANCE']

def load(path):
    return json.loads(path.read_text(encoding='utf-8-sig'), strict=False)

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def spelled_steps(literal):
    """An explicit 24-TET rendering policy, not maqam performance authenticity.

    #/flat/natural and '+' (half-sharp after the accidental) are supported.
    Down-arrow notation and non-note specifications stay unsupported: this cut
    uses the arrow inconsistently, so it cannot be silently assigned a pitch.
    """
    parts = literal.split(' - ')
    if len(parts) != 8:
        return None
    notes = []
    for part in parts:
        match = re.fullmatch(r'([A-G])([#♭♮]?)(\+)?(?:\s+\([^)]*\))?(?:\s+with .+)?', part.strip())
        if not match:
            return None
        letter, accidental, half = match.groups()
        note = {'C': 0, 'D': 4, 'E': 8, 'F': 10, 'G': 14, 'A': 18, 'B': 22}[letter]
        note += {'': 0, '#': 2, '♭': -2, '♮': 0}[accidental] + bool(half)
        notes.append(note % 24)
    if notes[0] != notes[-1]:
        return None
    steps = [(note - notes[0]) % 24 for note in notes[:-1]] + [24]
    return steps if all(a < b for a, b in zip(steps, steps[1:])) else None

def compile_field(source):
    reg = load(ROOT / 'fixtures/kernel/m-tree-v1.json')
    catalogue = load(ROOT / 'fixtures/kernel/m2-retained-c-v1.json')
    nodes = {n['source_ref']: n for n in reg['nodes']}
    files = {f['path']: f for f in reg['files']}
    if sha(source / NODE_PATH) != files[NODE_PATH]['sha256']:
        raise ValueError('not the pinned Bimba node source')
    raw = load(source / NODE_PATH)
    content = {n['coordinate']: (i, n['filteredProps']) for i, n in enumerate(raw)}
    tables = {t['name']: t for t in catalogue['tables']}
    used_files = {NODE_PATH}

    def claim(ref, prop):
        index, props = content[ref]
        return {'coordinate': ref, 'node_id': nodes[ref]['id'], 'property': prop,
                'literal': str(props.get(prop, '')), 'path': NODE_PATH,
                'pointer': f'/{index}/filteredProps/{prop}',
                'sha256': files[NODE_PATH]['sha256']}

    def relations(ref, kind):
        matches = [r for r in reg['relations'] if r['from_ref'] == ref and r['source_kind'] == kind]
        groups = {}
        for r in matches:
            if not r['to_ref'] or r['to_ref'] not in content:
                continue
            record = reg['records'][r['record']]
            file = reg['files'][record['file']]
            if file['path'] not in used_files:
                if sha(source / file['path']) != file['sha256']:
                    raise ValueError('not the pinned relation source: ' + file['path'])
                used_files.add(file['path'])
            groups.setdefault(r['to_ref'], []).append({
                'id': r['id'], 'relation_ref': r['relation_ref'], 'kind': kind,
                'from_coordinate': ref, 'to_coordinate': r['to_ref'],
                'path': file['path'], 'sha256': file['sha256'],
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
            element = content[chakra][1].get('elementalCorrespondence', '')
            fibre = {1: 'air', 2: 'fire', 3: 'water', 4: 'earth'}.get(element_id)
            if fibre and fibre.title() not in element:
                raise ValueError('source/native elemental disagreement: ' + chakra)
            yantra = content[chakra][1].get('yantraForm', '')
            colour = None
            # Exact source phrase admission, not a general keyword-colour guess.
            for phrase, name in [('yellow square', 'yellow'), ('silver crescent moon', 'silver'), ('red triangle', 'red')]:
                if phrase in yantra:
                    colour = name
            literal = content[ref][1].get('intervalStructure', '')
            result.append({
                'maqam_index': i, 'role': 'tonic' if role == ROLES[0] else 'dominant',
                'maqam_coordinate': ref, 'maqam_node_id': nodes[ref]['id'],
                'maqam_name': content[ref][1].get('name', ''),
                'planet_index': planet_index, 'planet_coordinate': planet, 'planet_node_id': nodes[planet]['id'],
                'chakra_index': chakra_index, 'chakra_coordinate': chakra, 'chakra_node_id': nodes[chakra]['id'],
                'tattva_coordinate': tattva, 'tattva_node_id': nodes[tattva]['id'] if tattva else None,
                'element_literal': element, 'material_fibre': fibre, 'yantra_literal': yantra,
                'colour_name': colour, 'planetary_mode_literal': content[planet][1].get('planetaryMode', ''),
                'interval_literal': literal, 'spelled_steps24': spelled_steps(literal),
                'musical_relations': musical_links, 'planetary_relations': planetary_links,
                'claims': [claim(ref, 'intervalStructure'), claim(planet, 'planetaryMode'),
                           claim(chakra, 'elementalCorrespondence'), claim(chakra, 'yantraForm')],
            })
    return {'schema': CONTRACT, 'registry_revision': reg['registry_revision'],
            'source_revision': reg['source_revision'], 'source_repository': reg['source_repository'],
            'standing': 'compiled-source-reading; distinct-from-retained-C-ruler-and-authentic-tuning',
            'source_locks': [{'path': p, 'sha256': files[p]['sha256']} for p in sorted(used_files)],
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
    p.add_argument('--source-root', type=Path, default=ROOT / 'target/m2-bimba-source')
    args = p.parse_args()
    field = compile_field(args.source_root)
    for path, text in [(OUTPUT, json.dumps(field, ensure_ascii=False, indent=2) + '\n'), (NATIVE, native(field))]:
        file = ROOT / path
        if args.command == 'refresh':
            file.write_text(text, encoding='utf-8')
        elif not file.exists() or file.read_text(encoding='utf-8') != text:
            raise ValueError('stale M2 correspondence: ' + path)
    print(f"M2 correspondence: {len(field['rules'])} source paths, {len(field['gaps'])} explicit gaps")

if __name__ == '__main__':
    main()
