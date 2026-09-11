#!/usr/bin/env python3
"""M2-only census from the unchanged K2 registry and hash-locked deep source.

This is an evidence projection, NOT a replacement tree or a readiness oracle.
Every coordinate remains referenced even without a numerical descriptor. Rich
source payloads stay at their exact pinned file/pointer; no embeddings are ported.
"""
import argparse
import collections
import hashlib
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'fixtures/kernel/m2-field-census-summary-v1.json'
PREFIX = 'Idea/Bimba/Map/datasets/parashakti-deep/'

def load(path): return json.loads(path.read_text(encoding='utf-8-sig'), strict=False)
def sha(data): return hashlib.sha256(data).hexdigest()
def stable(data): return json.dumps(data, ensure_ascii=False, sort_keys=True, separators=(',', ':')).encode()

def report(source):
    registry = load(ROOT / 'fixtures/kernel/m-tree-v1.json')
    catalogue = load(ROOT / 'fixtures/kernel/m2-retained-c-v1.json')
    locks = [f for f in registry['files'] if f['path'].startswith(PREFIX)]
    if len(locks) != 3: raise ValueError('expected the three K2-pinned deep source files')
    documents = {}
    for lock in locks:
        path = source / lock['path']; raw = path.read_bytes()
        if sha(raw) != lock['sha256']: raise ValueError('not the pinned K2 source: ' + lock['path'])
        documents[path.name] = load(path)
    nodes = {n['source_ref']: n for n in registry['nodes'] if n['root_position'] == 2}
    by_id = {n['id']: n for n in nodes.values()}
    source_nodes = {n['coordinate']: (i, n['filteredProps']) for i,n in enumerate(documents['nodes-full-detail.json'])}
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
            'deep_source_pointer':('/'+str(raw[0])+'/filteredProps') if raw else None,
            'deep_payload_sha256':sha(stable(raw[1])) if raw else None,
            'deep_property_keys':sorted(raw[1]) if raw else [],
            'retained_record_associations':associations[ref],
            'disposition':'retained-descriptor-associated; semantic-equality-unproven' if associations[ref] else 'source-retained; no-leaf-computation-asserted'})
    if len(holdings) != 597: raise ValueError('M2 source field changed; reconcile instead of truncating')
    relations = [r for r in registry['relations'] if r['from_id'] in by_id or r['to_id'] in by_id]
    asma = []; t = tables['asma']
    for i,(row,ref) in enumerate(zip(t['rows'],t['bindings'])):
        if not ref or ref not in source_nodes: continue
        props = source_nodes[ref][1]
        for column,key in [('abjad_value','abjadValue'),('digital_root','digitalRoot')]:
            retained = row[t['columns'].index(column)]
            if key in props and props[key] != retained:
                asma.append({'record':i,'coordinate':ref,'field':key,'retained_c':retained,'bimba':props[key]})
    arithmetic = [{'record':i,'abjad_value':r[6],'stored_digital_root':r[4],
        'arithmetic_digital_root':0 if r[6]==0 else 1+(r[6]-1)%9} for i,r in enumerate(t['rows'])
        if i<99 and r[4] != (0 if r[6]==0 else 1+(r[6]-1)%9)]
    music = []
    for i,(row,ref) in enumerate(zip(tables['maqam']['rows'],tables['maqam']['bindings'])):
        props = source_nodes[ref][1]; literal = props.get('intervalStructure')
        # No accidental/tuning normalization is performed here. A literal
        # Bimba declaration and a C 24-TET pattern are independent assertions.
        music.append({'record':i,'coordinate':ref,'name':props.get('name'),
            'bimba_interval_structure':literal,'retained_c_quartertone_steps':row[2:9],
            'standing':'paired-source-assertions; not-interval-parity'})
    unknown = {name:[i for i,r in enumerate(tables[name]['bindings']) if r is None]
        for name in ['tattva','planet','station','asma','mantra']}
    body = (ROOT/'vendor/epi-kernel/reference/src/m2.c').read_text()
    header = (ROOT/'vendor/epi-kernel/reference/include/m2.h').read_text()
    native_functions = re.findall(r'static inline\s+\w+\s+(\w+)\s*\(',header)
    source_functions = re.findall(r'^(?:M2_Root\*|void|bool|int)\s+(m2_\w+)\s*\(',body,re.M)
    return {'schema':'ql.m2-field-census/v1','standing':'scoped-K6-census; K4-whole-programme-acceptance-not-observed',
        'registry_revision':registry['registry_revision'],'source_repository':registry['source_repository'],
        'source_revision':registry['source_revision'],'source_locks':locks,
        'source_parse_policy':'K2 retained UTF-8 BOM/control-character JSON; strict=False; unchanged source bytes',
        'counts':{'coordinates':len(holdings),'incident_relations':len(relations),
            'deep_node_records':len(documents['nodes-full-detail.json']),
            'deep_planet_records':len(documents['parashakti-planets.json']),
            'deep_relation_records':len(documents['relations.json']),
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
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('action',choices=['refresh','check']);p.add_argument('--source-root',type=Path,required=True)
    args=p.parse_args();value=report(args.source_root);full=encode(value)
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
