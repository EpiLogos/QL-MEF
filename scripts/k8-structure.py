#!/usr/bin/env python3
"""Promote the ratified K8 source through the existing M registry compiler.

The accepted v1 snapshot remains a historical compatibility view. The current
projection is the same tree plus reviewed source records, not an independently
authored registry. Large deterministic products belong in the build/install
output; the source amendment, authority locks and receipts are retained in Git.
No network or live Neo4j mutation occurs here.
"""
from __future__ import annotations

import argparse
import copy
import hashlib
import importlib.util
import json
import math
import os
import tempfile
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
PROMOTION = Path('c/registry/promotions/k8-apertures-v1.json')
RECEIPT = Path('fixtures/kernel/k8-structure-receipt-v1.json')


def compiler():
    spec = importlib.util.spec_from_file_location('m_tree_generator', ROOT / 'scripts/generate-m-tree.py')
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


G = compiler()


def read(path):
    return json.loads(path.read_text(encoding='utf-8'))


def require(ok, message):
    if not ok:
        raise ValueError(message)


def blob(data):
    return hashlib.sha1(b'blob ' + str(len(data)).encode() + b'\0' + data).hexdigest()


def validate(p, base, root):
    require(p['schema'] == 'ql.m-structural-promotion/v1', 'unknown promotion schema')
    require(p['base_registry_revision'] == base['registry_revision'], 'promotion base drift')
    expected = ('#2-0', '#2-0-0', '#2-0-1', '#2-0-2', '#3-0', '#3-5-5/0', '#3-5')
    require(tuple(p[k] for k in ('root', 'void', 'fibonacci', 'static_field', 'field', 'centre', 'clock')) == expected,
            'ratified aperture/centre allocation drift')
    require(p['id'] == 'K8-APERTURES-CENTRE-2026-09-11', 'wrong versioned promotion identity')
    expected_relations = [
        ('#3-0', '#3-5-5/0', 'REALISED_AT'),
        ('#3-5', '#2-0', 'RECEIVES_APERTURE_ARCHITECTURE'),
        ('#3-0', '#1', 'RECEIVES_ROTATIONAL_TOPOLOGICAL_CAPACITY'),
        ('#3-0', '#2', 'RECEIVES_SPECTRAL_WORLD_CONDITION'),
        ('#2-0-0', '#0', 'DERIVED_FROM_ANUTTARA'),
        ('#2-4', '#2-0', 'SYMBOLIC_WORLD_APERTURE_ASSOCIATION'),
        ('#2-5', '#3-0', 'PLANETARY_CONDITION_RECEIVED_IN_FIELD'),
    ]
    require([(r['from'], r['to'], r['kind']) for r in p['field_relations']] == expected_relations,
            'unreviewed field relation change')
    pairs = p['pairs']
    require(len(pairs) == 8, 'exactly eight reciprocal containers required')
    divisions = [1, 2, 4, 8, 9, 10, 12, 15, 24, 30, 36, 40, 45, 90, 180, 360]
    seen = set()
    for i, pair in enumerate(pairs):
        require(pair['ref'] == f'#2-0-2-{i}', 'pair coordinate drift')
        require(pair['native_indices'] == [i, 15-i], 'reciprocity is not ring antipodality')
        require(pair['division_degrees'] == [divisions[i], divisions[15-i]], 'native division drift')
        require(math.prod(pair['division_degrees']) == 360, 'not a reciprocal divisor pair')
        seen.update(pair['native_indices'])
    require(seen == set(range(16)), 'missing or duplicate native aperture')
    quanta = [g['half_degree_quantum'] for g in p['grids']]
    require(quanta == [12, 40, 45], 'angular grids must be exact half-degree quanta')
    require([g['positions'] for g in p['grids']] == [60, 18, 16], 'grid counts drift')
    require([g['ref'] for g in p['grids']] == [p['fibonacci'], '#2-2', p['void']], 'grid owner drift')
    require([math.lcm(quanta[a], quanta[b]) for a, b in ((0,1),(0,2),(1,2))] == p['closures_half_degrees'],
            'closure law drift')
    require(math.lcm(*quanta) == p['all_grid_closure_half_degrees'] == 360, 'common closure drift')
    require(p['void_elemental_ratio'] == [9,8] and 45 * 8 == 40 * 9, 'epogdoon drift')
    authority = p['authority']
    require(authority['repository'] == 'EpiLogos/QL-MEF' and authority['issue'] == 132,
            'wrong promotion authority')
    require(authority['standing'] == 'owner-ratified-structural-promotion'
            and len(authority['revision']) == 40
            and all(c in '0123456789abcdef' for c in authority['revision']), 'invalid source authority')
    path = (root / authority['path']).resolve()
    require(path.is_relative_to(root.resolve()) and path.is_file(), 'unsafe/missing authority path')
    data = path.read_bytes()
    require(blob(data) == authority['git_blob'] and hashlib.sha256(data).hexdigest() == authority['sha256'],
            'ratified authority source changed; review a successor, do not restamp old evidence')
    text = data.decode('utf-8')
    require('M3-5-5/0' in text and '18 aperture identities' in text, 'wrong authority body')
    for pair in pairs:
        for leaf in (0,1):
            require(pair['ref'][1:].replace('2-', 'M2-', 1) + f'-{leaf}' in text,
                    'leaf absent from ratified source')
    require(p['preservation']['live_graph_application'] == 'pending-live-application',
            'serialized compilation cannot claim live graph application')
    return data


def project(root=ROOT, promotion=None):
    base = read(root / 'fixtures/kernel/m-tree-v1.json')
    # Reproduce the accepted compiler floor, rather than trusting a second tree.
    require(G.model(read(root / G.SNAPSHOT)) == base, 'accepted registry/compiler floor drift')
    p = copy.deepcopy(promotion if promotion is not None else read(root / PROMOTION))
    authority_bytes = validate(p, base, root)
    result = copy.deepcopy(base)
    result['schema'] = 'ql.m-tree/v2'
    result['registry_lineage'] = [{
        'registry_revision': base['registry_revision'],
        'promotion_id': p['id'],
        'promotion_sha256': G.digest(p),
        'source_repository': p['authority']['repository'],
        'source_revision': p['authority']['revision'],
        'source_path': p['authority']['path'],
        'source_git_blob': p['authority']['git_blob'],
        'standing': p['authority']['standing'],
    }]
    authority_file = len(result['files'])
    result['files'].append({
        'path': p['authority']['path'], 'git_blob': p['authority']['git_blob'],
        'sha256': p['authority']['sha256'], 'record_class': 'owner-ratified-markdown',
        'bytes': len(authority_bytes), 'repository': p['authority']['repository'],
        'revision': p['authority']['revision'],
    })
    source_lines = authority_bytes.decode().splitlines()
    by_ref = {n['source_ref']: n for n in result['nodes']}
    added = []
    relation_properties = {}

    def record(payload, needle):
        matches = [i for i,line in enumerate(source_lines) if needle in line]
        require(bool(matches), 'missing ratified source assertion: ' + needle)
        index = len(result['records'])
        line = matches[0]
        result['records'].append({
            'file': authority_file, 'record_index': line,
            'payload_sha256': G.digest({'line': line+1, 'text': source_lines[line]}),
            'property_keys': sorted(payload),
        })
        return index

    def node(ref, name, parent, payload, needle):
        require(ref not in by_ref, 'promotion would overwrite an existing identity: ' + ref)
        require(parent in by_ref, 'promotion parent is missing: ' + parent)
        ident = G.stable_id('ql.m-node/v1', ref)
        require(ident not in {n['id'] for n in by_ref.values()}, 'promotion ID collision')
        n = {'id':ident, 'source_ref':ref, 'parent_id':by_ref[parent]['id'],
             'root_id':by_ref['#2']['id'], 'root_position':2,
             'local_segment':ref[len(parent)+1:], 'separator':'-',
             'depth':by_ref[parent]['depth']+1, 'lexical_depth':ref.count('-'),
             'lexical_parent_source_ref':parent, 'parent_basis':'source-parent',
             'source_parent_refs':[parent], 'structural_status':'source-declared',
             'aggregate':False, 'names':[name], 'aliases':[],
             'records':[record(payload,needle)], 'children':[], 'subtree_count':1}
        by_ref[ref] = n
        added.append(n)

    node(p['void'], 'Anuttara-derived void-ring aperture', p['root'], p['grids'][2], 'M2-0-0  Anuttara')
    node(p['fibonacci'], 'Fibonacci/Pisano ground aperture', p['root'], p['grids'][0], 'M2-0-1  Fibonacci')
    node(p['static_field'], 'Sixteenfold static aperture field', p['root'], {'static_apertures':16}, 'M2-0-2  Sixteenfold')
    aperture_records = []
    for i,pair in enumerate(p['pairs']):
        needle = f'| M2-0-2-{i} |'
        node(pair['ref'], f'Reciprocal aperture pair {i}', p['static_field'], pair, needle)
        for leaf,(native,division) in enumerate(zip(pair['native_indices'], pair['division_degrees'])):
            ref = f"{pair['ref']}-{leaf}"
            payload = {'native_index':native, 'division_degrees':division, 'segments':360//division,
                       'pair':pair['ref'], 'reciprocal_native_index':15-native,
                       'void_orientation_half_degrees':45*native}
            node(ref, f'Static aperture {division} degrees x {360//division}', pair['ref'], payload, needle)
            aperture_records.append({'source_ref':ref, 'id':by_ref[ref]['id'], **payload})

    # Only the ancestor child/subtree summaries change; the original records,
    # names, parentage, source IDs and all degree/backbone leaves survive intact.
    result['nodes'] = [by_ref['M'], *(by_ref[r] for r in sorted(by_ref) if r != 'M')]
    for n in result['nodes']:
        n['children'] = []
        n['subtree_count'] = 1
    by_id = {n['id']:n for n in result['nodes']}
    for n in result['nodes'][1:]:
        by_id[n['parent_id']]['children'].append(n['id'])
    for n in sorted(result['nodes'], key=lambda n:n['depth'], reverse=True):
        if n['parent_id'] is not None:
            by_id[n['parent_id']]['subtree_count'] += n['subtree_count']
    added_relations = []

    def relation(a,b,kind,properties=None,needle='M2-0 owns the complete eighteen-aperture architecture',undirected=False):
        require(a in by_ref and b in by_ref, 'unresolved promoted relation endpoint')
        ref = f"k8:{p['id']}:{kind}:{a}>{b}"
        payload = {'from':a,'to':b,'kind':kind,**(properties or {})}
        r = {'relation_ref':ref, 'source_kind':kind, 'from_ref':a, 'to_ref':b,
             'orientation':'undirected' if undirected else 'directed',
             'cross_m':a[1]!=b[1], 'record':record(payload,needle),
             'id':G.stable_id('ql.m-relation/v1',ref), 'class':'bimba-source',
             'from_id':by_ref[a]['id'], 'to_id':by_ref[b]['id']}
        added_relations.append(r)
        relation_properties[r['id']] = payload

    for n in added:
        relation(by_id[n['parent_id']]['source_ref'],n['source_ref'],'CONTAINS_PROMOTED_APERTURE_STRUCTURE')
    for i,pair in enumerate(p['pairs']):
        relation(pair['ref']+'-0',pair['ref']+'-1','STATIC_DIVISOR_RECIPROCAL',pair,
                 needle=f'| M2-0-2-{i} |',undirected=True)
    for a in aperture_records:
        relation(a['source_ref'],p['void'],'ORIENTED_ON_VOID_RING',
                 {'native_index':a['native_index'],'half_degrees':a['void_orientation_half_degrees']},
                 needle='Every static lens retains division angle')
        relation(a['source_ref'],p['fibonacci'],'GROUNDED_IN_FIBONACCI',
                 needle='Every static lens retains division angle')
    for i,(a,b) in enumerate(((0,1),(0,2),(1,2))):
        relation(p['grids'][a]['ref'],p['grids'][b]['ref'],'EXACT_GRID_CLOSURE',
                 {'half_degrees':p['closures_half_degrees'][i]},
                 needle='Exact half-degree quanta 12, 40 and 45',undirected=True)
    relation(p['void'],'#2-2','VOID_ELEMENTAL_EPOGDOON',{'ratio':[9,8]},needle='22.5° / 20° = 9/8')
    for r in p['field_relations']:
        relation(r['from'],r['to'],r['kind'],needle=('M3-5-5/0' if r['kind']=='REALISED_AT' else
                 'M1 rotational/topological carrier' if r['to']=='#1' else
                 'M2 spectral/vibrational differentiation' if r['to']=='#2' else
                 'void ring retains its M0/Anuttara derivation' if r['to']=='#0' else
                 'M2 symbolic-world branches keep their native identities' if r['from']=='#2-4' else
                 'live planetary–Nara instrument' if r['from']=='#2-5' else
                 'M3 references these exact M2 identities'))
    result['relations'] = sorted([*result['relations'],*added_relations],key=lambda r:r['relation_ref'])
    require(len({r['id'] for r in result['relations']}) == len(result['relations']), 'relation collision')
    result.pop('registry_revision')
    result['registry_revision'] = G.digest(result)
    before = {n['id']:n for n in base['nodes']}
    for n in result['nodes']:
        if n['id'] in before:
            for k,v in before[n['id']].items():
                if k not in ('children','subtree_count'):
                    require(n[k] == v, 'unreviewed change to old coordinate: '+n['source_ref']+':'+k)
    receipt = {
        'schema':'ql.k8-structural-receipt/v1', 'promotion_id':p['id'],
        'base_registry_revision':base['registry_revision'], 'registry_revision':result['registry_revision'],
        'authority':p['authority'], 'promotion_sha256':G.digest(p),
        'preserved_nodes':len(base['nodes']), 'added_nodes':len(added),
        'node_count':len(result['nodes']), 'preserved_relations':len(base['relations']),
        'added_relations':len(added_relations), 'relation_count':len(result['relations']),
        'static_apertures':sorted(aperture_records,key=lambda a:a['native_index']),
        'additional_apertures':[{'role':'void','ref':p['void'],'id':by_ref[p['void']]['id']},
                               {'role':'fibonacci','ref':p['fibonacci'],'id':by_ref[p['fibonacci']]['id']}],
        'field':{'ref':p['field'],'id':by_ref[p['field']]['id']},
        'centre':{'ref':p['centre'],'id':by_ref[p['centre']]['id']},
        'grids':p['grids'], 'closures_half_degrees':p['closures_half_degrees'],
        'relation_properties':relation_properties,
        'graph_standing':'pending-live-application',
    }
    require(receipt['added_nodes']==27 and len(receipt['static_apertures'])+2==18,'cardinality drift')
    return result,receipt


def receipt_summary(receipt):
    return {k:v for k,v in receipt.items() if k not in ('static_apertures','additional_apertures','relation_properties')}


def emit(path, text):
    """Atomic outputs; retain mtimes for reproducible no-op builds."""
    data = text.encode('utf-8')
    if path.is_file() and path.read_bytes() == data:
        return
    with tempfile.NamedTemporaryFile(dir=path.parent, delete=False) as stream:
        name = stream.name
        stream.write(data)
    try:
        os.replace(name, path)
    finally:
        if os.path.exists(name):
            os.unlink(name)


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root',type=Path,default=ROOT)
    parser.add_argument('--out',type=Path)
    parser.add_argument('--check',action='store_true')
    parser.add_argument('--write-receipt',action='store_true')
    args=parser.parse_args()
    try:
        result,receipt=project(args.root)
        text=json.dumps(receipt_summary(receipt),ensure_ascii=False,sort_keys=True,indent=2)+'\n'
        if args.write_receipt:
            emit(args.root/RECEIPT,text)
        if args.check:
            require((args.root/RECEIPT).read_text(encoding='utf-8')==text,'promotion receipt drift')
        if args.out:
            args.out.mkdir(parents=True,exist_ok=True)
            emit(args.out/'m-tree-v2.json',G.render(result))
            emit(args.out/'m_tree_live_data.inc',G.tables(result))
            emit(args.out/'k8-structure-receipt-v1.json',json.dumps(receipt,ensure_ascii=False,sort_keys=True,indent=2)+'\n')
            by_ref={n['source_ref']:n for n in result['nodes']}
            aperture_rows=['/* Generated from the ratified M2-owned architecture. */', 'static const QL_M2_Aperture m2_apertures[] = {']
            static=receipt['static_apertures']
            for a in static:
                reciprocal=static[a['reciprocal_native_index']]['id']
                aperture_rows.append('{' + ','.join([G.cid(a['id']),G.cid(by_ref[a['pair']]['id']),G.cid(reciprocal),'QL_M2_STATIC_APERTURE',str(a['division_degrees']*2),str(a['segments']),str(a['void_orientation_half_degrees']),str(a['native_index'])]) + '},')
            for a,kind,quantum,count in zip(receipt['additional_apertures'],['QL_M2_VOID_APERTURE','QL_M2_FIBONACCI_APERTURE'],[45,12],[16,60]):
                aperture_rows.append('{' + ','.join([G.cid(a['id']),G.cid(None),G.cid(None),kind,str(quantum),str(count),'0','QL_M2_NO_NATIVE_APERTURE']) + '},')
            aperture_rows.append('};')
            emit(args.out/'m2_aperture_data.inc','\n'.join(aperture_rows)+'\n')
            origins=['/* Generated source origins; indices share m-tree-v2.json files. */', 'static const QL_M_SourceOrigin m_live_origins[] = {']
            for f in result['files']:
                origins.append('{' + G.cstr(f.get('repository',result['source_repository'])) + ',' + G.cstr(f.get('revision',result['source_revision'])) + '},')
            origins.append('};')
            origins.append('static const char m_live_base_revision[] = '+G.cstr(receipt['base_registry_revision'])+';')
            emit(args.out/'m_tree_live_origins.inc','\n'.join(origins)+'\n')
        print(f"K8 current M registry: {receipt['node_count']} nodes, {receipt['relation_count']} relations; "
              f"{receipt['registry_revision']}; live graph NOT CLAIMED")
    except (ValueError,KeyError,OSError) as error:
        sys.exit(f'K8 promotion: {error}')


if __name__=='__main__':
    main()
