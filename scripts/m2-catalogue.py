#!/usr/bin/env python3
"""Lossless retained-C descriptor import; K2 alone supplies coordinate identities.

`refresh` executes the frozen C oracle, then emits a native C/Rust catalogue.
`check` independently executes the same oracle and compares every named field,
checks exact coordinate bindings, and rejects stale generated native data.
No numeric index is interpreted as a Bimba suffix; null binding is a gap.
"""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile
import unicodedata

ROOT = Path(__file__).resolve().parents[1]
CAT = ROOT / 'fixtures/kernel/m2-retained-c-v1.json'
INC = ROOT / 'c/src/m2_data.inc'
TABLES = [
 ('carrier','M2_ARCHETYPES','#2-0','value'),
 ('mef','M2_MEF_DESC','#2-1','lens position is_inverted l_family_link meaning_id'),
 ('tattva','M2_TATTVA_DESC','#2-2','index division element_id kanchuka_mask meaning_id'),
 ('decan','M2_DECAN_DESC','#2-3','element sign decan face ruling_planet meaning_id'),
 ('planet','M2_PLANET_LUT','#2-5','id group_type prime elem_sig cousto_freq keplerian_vel digital_root ananda_row meaning_id'),
 ('chakra','M2_CHAKRA_LUT','#2-5-0/1','id element_id tattva_idx meaning_id'),
 ('shem','M2_SHEM_DESC','#2-4.5','shem_idx choir position element_id decan_link planet_link meaning_id'),
 ('ratio','M2_MAQAM_RATIOS','#2-4.3','num den'),
 ('maqam','M2_MAQAM_DESC','#2-4.3','family mode_in_family interval0 interval1 interval2 interval3 interval4 interval5 interval6 planet_ruler meaning_id'),
 ('station','M2_MAQAM_SPIRITUAL','#2-4.2','station level meaning_id'),
 ('asma','M2_ASMA_LUT','#2-4.0','name_idx group index_in_group element_id digital_root mirror_idx abjad_value meaning_id'),
 ('mantra','M2_MANTRA_LUT','#2-4.1','mantra_idx matrika_group element_id phase fundamental_frequency meaning_id'),
 ('element','M2_ELEMENTS','#2-2','tattva_idx decan_element mantra_group chakra_idx'),
 ('det','M2_TO_M3_CYMATIC_PROJECTION','#2-5','mask'),
 ('resonance','M2_CAUSAL_RESONANCE_MASKS','#2-1','mask'),
 ('routing','ASMA_36_INTERNAL_MASK + ASMA_64_PROJECTIVE_MASK','#2-4.0','low_64 high_64'),
]
COUNTS = [72,72,36,73,10,8,72,10,72,24,100,100,5,72,36,2]

def digest(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def load(p): return json.loads(p.read_text(encoding='utf-8'))
def key(s):
    # Diacritics and punctuation only. No inferred transliteration/name synonym.
    return ''.join(c.lower() for c in unicodedata.normalize('NFKD',s) if c.isalnum())

def observe():
    with tempfile.TemporaryDirectory(prefix='ql-m2-source-') as temp:
        exe=Path(temp)/'probe'
        subprocess.run([os.environ.get('CC','cc'),'-std=c11','-O2','-ffunction-sections','-fdata-sections',
            '-I'+str(ROOT/'vendor/epi-kernel/reference/include'),str(ROOT/'scripts/m2-source-probe.c'),
            str(ROOT/'vendor/epi-kernel/reference/src/m2.c'),'-Wl,--gc-sections','-lm','-o',str(exe)],check=True)
        text=subprocess.check_output([str(exe)],text=True)
    result={name:[] for name,_,_,_ in TABLES}
    for line in text.splitlines():
        name,index,*values=line.split('\t')
        if int(index)!=len(result[name]): raise ValueError('non-contiguous source rows')
        result[name].append([int(v) for v in values])
    return result,text

def expected_catalogue(values):
    registry=load(ROOT/'fixtures/kernel/m-tree-v1.json')
    nodes={n['source_ref']:n for n in registry['nodes']}
    names={}
    for n in nodes.values():
        if n['root_position']==2:
            for name in n['names']: names.setdefault(key(name),[]).append(n['source_ref'])
    def named(name,prefix):
        candidates=sorted(set(c for c in names.get(key(name),[]) if c.startswith(prefix)))
        return candidates[0] if len(candidates)==1 else None
    def exists(ref): return ref if ref in nodes else None
    # Explicit source spelling, not tree synthesis. The two absent principles
    # are deliberately not mapped to a neighbouring tattva.
    tattvas=['#2-2-0-0/1']+[f'#2-2-0-{i}' for i in range(2,6)]
    tattvas+=['#2-2-1-0/1','#2-2-1-2',None]+[f'#2-2-1-{i}' for i in range(4,8)]
    tattvas+=['#2-2-2-0']+[f'#2-2-2-1-{i}' for i in range(3)]
    tattvas+=[None]+[f'#2-2-2-2-{i}' for i in range(2,6)]
    for group in (3,4,5):
        tattvas+=[f'#2-2-2-{group}-0/1']+[f'#2-2-2-{group}-{i}' for i in range(2,6)]
    assert len(tattvas)==36
    body=(ROOT/'vendor/epi-kernel/reference/src/m2.c').read_text()
    asma_body=body.split('const Asma_Name_Desc M2_ASMA_LUT[100] = {',1)[1].split('\n};',1)[0]
    asma_names={int(i):name.strip() for i,name in re.findall(r'\{\s*(\d+),[^\n]+?/\*\s*(.*?)\s*\*/',asma_body)}
    station_names=['La Maqam','Tawba','Sabr','Shukr','Khawf','Raja','Tawakkul','Rida']
    planet_names=['Sun','Moon','Mercury','Venus','Mars','Jupiter','Saturn','Uranus','Neptune','Pluto']
    tables=[]
    for ti,(name,symbol,scope,columns) in enumerate(TABLES):
        rows=values[name]
        if len(rows)!=COUNTS[ti]: raise ValueError(f'{name}: source count changed')
        bindings=[]
        for i,row in enumerate(rows):
            ref=None
            if name=='tattva': ref=tattvas[i]
            elif name=='decan':
                ref=f'#2-3-{i//18+1}-{(i%18)//6}-{(i%6)//2}-{i%2}' if i<72 else '#2-3-5/0'
            elif name=='planet': ref=named(planet_names[i],'#2-5')
            elif name=='chakra': ref=f'#2-5-0/1-{i}'
            elif name=='shem': ref=f'#2-4.5-{i//9}-'+('0/1' if i%9==0 else str(i%9+1))
            elif name=='maqam':
                family,pos=row[:2]
                suffix=('0/1' if pos==0 else str(pos+1)) if family in (0,3,4,7,8,9) else str(pos)
                ref=f'#2-4.3-{family}-{suffix}'
            elif name=='station':
                parent=named(station_names[i//3],'#2-4.2')
                if parent: ref=parent+'-'+str(i%3)
            elif name=='asma':
                ref=named(asma_names.get(i,''),'#2-4.0')
                if i==99: ref='#2-4.0-0/1'
            elif name=='mantra' and i<50:
                if i<16: ref=f'#2-4.1-0-0-{i}'
                elif i<41:
                    group,local=divmod(i-16,5)
                    suffix='0/1' if local==0 else str(local+1)
                    ref=f'#2-4.1-0-1-{group}-{suffix}'
                elif i<45: ref=f'#2-4.1-0-1-5-{i-41}'
                elif i<49: ref=f'#2-4.1-0-1-6-{i-45}'
                else: ref='#2-4.1-0-1-7-5/0'
            elif name=='element': ref=tattvas[row[0]]
            # MEF current L/L' identities do not become 12 invented M children;
            # rhythm/masks/ratios are branch laws, not new source coordinates.
            if ref is not None and exists(ref) is None: raise ValueError(f'{name}[{i}]: absent {ref}')
            bindings.append(ref)
        tables.append(dict(name=name,symbol=symbol,scope=scope,columns=columns.split(),rows=rows,bindings=bindings))
    return dict(schema='ql.m2-retained-c/v1',registry_revision=registry['registry_revision'],
        standing='retained-source-reading-not-whole-subsystem-parity',
        sources=[dict(path=p,sha256=digest(ROOT/p)) for p in ['vendor/epi-kernel/reference/include/m2.h','vendor/epi-kernel/reference/src/m2.c']],
        tables=tables)

def render(catalogue):
    # Byte-stable rows rather than reformatting the entire registry or ledger.
    parts=['{']
    for k,v in catalogue.items():
        if k!='tables': parts.append(json.dumps(k)+':'+json.dumps(v,ensure_ascii=False,separators=(',',':'))+',')
    parts.append('"tables":[')
    for ti,t in enumerate(catalogue['tables']):
        header={k:v for k,v in t.items() if k not in ('rows','bindings')}
        parts.append(json.dumps(header,ensure_ascii=False,separators=(',',':'))[:-1]+',"rows":[')
        parts += [json.dumps(row,separators=(',',':'))+(',' if i+1<len(t['rows']) else '') for i,row in enumerate(t['rows'])]
        parts.append('],"bindings":'+json.dumps(t['bindings'],ensure_ascii=False,separators=(',',':'))+'}'+(',' if ti+1<len(catalogue['tables']) else ''))
    parts.append(']}\n')
    return '\n'.join(parts)

def native(catalogue):
    registry=load(ROOT/'fixtures/kernel/m-tree-v1.json')
    ids={n['source_ref']:n['id'] for n in registry['nodes']}
    lines=['/* Generated by scripts/m2-catalogue.py. Do not edit. */']
    lines.append('static const char M2_DATA_REGISTRY[] = '+json.dumps(catalogue['registry_revision'])+';')
    for t in catalogue['tables']:
        name=t['name']; lines.append(f'static const QL_M2_Record m2_{name}_rows[] = {{')
        for i,(values,ref) in enumerate(zip(t['rows'],t['bindings'])):
            c_id='UINT64_C(0x'+ids[ref]+')' if ref else 'UINT64_C(0)'
            vals=','.join('UINT64_C('+str(v)+')' if v>2147483647 else str(v) for v in values)
            lines.append('{'+c_id+',{'+vals+'}},')
        lines.append('};')
    lines.append('static const QL_M2_Table m2_tables[] = {')
    for t in catalogue['tables']:
        lines.append('{'+','.join([json.dumps(t['name']),json.dumps(t['symbol']),json.dumps(t['scope']),
            str(len(t['rows'])),str(len(t['columns'])),'{'+','.join(json.dumps(c) for c in t['columns'])+'}',
            'm2_'+t['name']+'_rows'])+'},')
    lines.append('};\n')
    return '\n'.join(lines)

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action',choices=('refresh','check'))
    parser.add_argument('--receipt',type=Path)
    args=parser.parse_args()
    values,text=observe(); expected=expected_catalogue(values)
    products={CAT:render(expected),INC:native(expected)}
    for path,content in products.items():
        if args.action=='refresh': path.write_text(content,encoding='utf-8')
        elif not path.is_file() or path.read_text(encoding='utf-8')!=content:
            raise SystemExit('stale M2 import: '+str(path.relative_to(ROOT)))
    if args.receipt:
        args.receipt.mkdir(parents=True,exist_ok=True)
        (args.receipt/'source.tsv').write_text(text)
        (args.receipt/'catalogue-check.json').write_text(json.dumps(dict(result='pass',
            rows=sum(len(t['rows']) for t in expected['tables']),registry_revision=expected['registry_revision'],
            sources=expected['sources'],catalogue_sha256=digest(CAT)),indent=2)+'\n')
    print('M2: 764 executed retained-C rows, exact K2 bindings and native generation agree')
if __name__=='__main__': main()
