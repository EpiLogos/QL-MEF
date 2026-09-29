#!/usr/bin/env python3
"""M2-only census from the registry and a read of the Bimba map it was built from.

This is an evidence projection, NOT a replacement tree or a readiness oracle.
Every coordinate remains referenced even without a numerical descriptor. `check`
recomputes when a read of the registry's map is present (scripts/bimba_map.py
read), and otherwise verifies the committed summary against the registry.
"""
import argparse
import collections
import hashlib
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'fixtures/kernel/m2-field-census-summary-v1.json'
import sys
sys.path.insert(0, str(ROOT / 'scripts'))
import bimba_map

def load(path): return json.loads(path.read_text(encoding='utf-8-sig'), strict=False)
def sha(data): return hashlib.sha256(data).hexdigest()
def stable(data): return json.dumps(data, ensure_ascii=False, sort_keys=True, separators=(',', ':')).encode()

def report(read):
    registry = load(ROOT / 'fixtures/kernel/m-tree-v1.json')
    catalogue = load(ROOT / 'fixtures/kernel/m2-retained-c-v1.json')
    if read['content_sha256'] != registry['source_revision']:
        raise ValueError('map read is not the map the registry was built from')
    locks = registry['files']
    nodes = {n['source_ref']: n for n in registry['nodes'] if n['root_position'] == 2}
    by_id = {n['id']: n for n in nodes.values()}
    by_ql = {bimba_map.ql_spelling(c): v['properties'] for c, v in read['nodes'].items() if bimba_map.is_m_coordinate(c)}
    source_nodes = {ref: (registry['records'][n['records'][0]]['record_index'], by_ql[ref]) for ref, n in nodes.items()}
    tables = {t['name']: t for t in catalogue['tables']}
    associations = collections.defaultdict(list)
    for table in catalogue['tables']:
        for i, ref in enumerate(table['bindings']):
            if ref: associations[ref].append(f"{table['name']}:{i}")
    holdings = []
    for ref,node in nodes.items():
        raw = source_nodes.get(ref)
        holdings.append({'coordinate':ref,'registry_node_id':node['id'],
            'source_record_indices':node['records'],'source_names':node['names'],
            'map_record_index':raw[0],
            'map_payload_sha256':sha(stable(raw[1])),
            'map_property_keys':sorted(raw[1]),
            'retained_record_associations':associations[ref],
            'disposition':'retained-descriptor-associated; semantic-equality-unproven' if associations[ref] else 'source-retained; no-leaf-computation-asserted'})
    relations = [r for r in registry['relations'] if r['from_id'] in by_id or r['to_id'] in by_id]
    asma = []; t = tables['asma']
    for i,(row,ref) in enumerate(zip(t['rows'],t['bindings'])):
        if not ref or ref not in source_nodes: continue
        props = source_nodes[ref][1]
        for column,key in [('abjad_value','m_2_4_abjad_value'),('digital_root','m_2_4_digital_root')]:
            retained = row[t['columns'].index(column)]
            if key in props and props[key] != retained:
                asma.append({'record':i,'coordinate':ref,'field':key,'retained_c':retained,'bimba':props[key]})
    arithmetic = [{'record':i,'abjad_value':r[6],'stored_digital_root':r[4],
        'arithmetic_digital_root':0 if r[6]==0 else 1+(r[6]-1)%9} for i,r in enumerate(t['rows'])
        if i<99 and r[4] != (0 if r[6]==0 else 1+(r[6]-1)%9)]
    music = []
    for i,(row,ref) in enumerate(zip(tables['maqam']['rows'],tables['maqam']['bindings'])):
        props = source_nodes[ref][1]
        # The map states ajnas, tonic and dominant, not a full scale spelling.
        # A map statement and a C 24-TET pattern are independent assertions.
        music.append({'record':i,'coordinate':ref,'name':props.get('c_1_name'),
            'bimba_ajnas':props.get('c_2_ajnas'),'bimba_tonic_note':props.get('c_2_tonic_note'),
            'bimba_dominant_note':props.get('c_2_dominant_note'),'retained_c_quartertone_steps':row[2:9],
            'standing':'paired-source-assertions; not-interval-parity'})
    unknown = {name:[i for i,r in enumerate(tables[name]['bindings']) if r is None]
        for name in ['tattva','planet','station','asma','mantra']}
    body = (ROOT/'vendor/epi-kernel/reference/src/m2.c').read_text()
    header = (ROOT/'vendor/epi-kernel/reference/include/m2.h').read_text()
    native_functions = re.findall(r'static inline\s+\w+\s+(\w+)\s*\(',header)
    source_functions = re.findall(r'^(?:M2_Root\*|void|bool|int)\s+(m2_\w+)\s*\(',body,re.M)
    return {'schema':'ql.m2-field-census/v1','standing':'scoped-K6-source-observation-on-accepted-K4-census',
        'registry_revision':registry['registry_revision'],'source_repository':registry['source_repository'],
        'source_revision':registry['source_revision'],'source_locks':locks,
        'source_parse_policy':'read-only Bimba map read; content hash = registry source_revision',
        'counts':{'coordinates':len(holdings),'incident_relations':len(relations),
            'retained_c_records':sum(len(t['rows']) for t in catalogue['tables'])},
        'incident_relation_kinds':dict(sorted(collections.Counter(r['source_kind'] for r in relations).items())),
        'incident_relations_sha256':sha(stable(relations)),
        'native_header_inline_functions':native_functions,'retained_body_functions':source_functions,
        'native_lifecycle_disposition':'legacy arena init/teardown/CLI replaced by native immutable C tables + checked Rust request/frame; reference body retained unchanged',
        'unresolved_exact_bindings':unknown,'asma_paired_value_differences':asma,
        'asma_arithmetic_differences':arithmetic,'maqam_paired_readings':music,'coordinates':holdings}

def encode(value):
    # One coordinate/reading per line keeps this evidence projection reviewable.
    lines=['{']; total=len(value)
    for index,(key,value) in enumerate(value.items()):
        comma=',' if index+1<total else ''
        if key in ('coordinates','maqam_paired_readings','asma_paired_value_differences','asma_arithmetic_differences'):
            lines.append(json.dumps(key)+':[')
            lines.extend(json.dumps(r,ensure_ascii=False,separators=(',',':'))+(',' if i+1<len(value) else '') for i,r in enumerate(value))
            lines.append(']'+comma)
        else: lines.append(json.dumps(key)+':'+json.dumps(value,ensure_ascii=False,separators=(',',':'))+comma)
    text='\n'.join(lines+['}\n'])
    json.loads(text)  # refuse a malformed receipt
    return text

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('action',choices=['refresh','check'])
    p.add_argument('--map',type=Path,default=bimba_map.CACHE,help='read of the map the registry was built from')
    args=p.parse_args()
    registry=load(ROOT/'fixtures/kernel/m-tree-v1.json')
    fresh=args.map.is_file() and bimba_map.load(args.map)['content_sha256']==registry['source_revision']
    if not fresh:
        if args.action=='refresh':raise SystemExit('refresh needs a read of the map the registry was built from')
        summary=load(OUT)
        if summary['registry_revision']!=registry['registry_revision']:raise SystemExit('stale M2 field census: registry revision')
        print('M2 field census: committed summary against registry (no map read)');return
    value=report(bimba_map.load(args.map));full=encode(value)
    receipt=ROOT/'target/m2-receipt';receipt.mkdir(parents=True,exist_ok=True)
    (receipt/'m2-field-census-v1.json').write_text(full)
    summary={k:v for k,v in value.items() if k not in ('coordinates','maqam_paired_readings','incident_relation_kinds','asma_paired_value_differences','asma_arithmetic_differences')}
    summary['schema']='ql.m2-field-census-summary/v1'
    summary['full_census_sha256']=sha(full.encode())
    summary['full_census_artifact']='target/m2-receipt/m2-field-census-v1.json'
    summary['incident_relation_kind_count']=len(value['incident_relation_kinds'])
    summary['causal_resonance_records']=value['incident_relation_kinds'].get('CAUSAL_RESONANCE',0)
    summary['asma_paired_difference_count']=len(value['asma_paired_value_differences'])
    summary['asma_arithmetic_difference_count']=len(value['asma_arithmetic_differences'])
    summary['asma_difference_examples']=value['asma_paired_value_differences'][:4]
    summary['paired_maqam_records']=len(value['maqam_paired_readings'])
    summary['maqam_standing']='All 72 Bimba literal interval declarations retained in full census beside frozen C 24-TET patterns; equivalence not asserted.'
    text=encode(summary)
    if args.action=='refresh': OUT.write_text(text)
    elif not OUT.is_file() or OUT.read_text()!=text: raise SystemExit('stale M2 deep field census')
    print(json.dumps(value['counts'],sort_keys=True));print('Asma paired value differences:',len(value['asma_paired_value_differences']),'; arithmetic differences:',len(value['asma_arithmetic_differences']))
if __name__=='__main__': main()
