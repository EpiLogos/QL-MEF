#!/usr/bin/env python3
"""Add/verify the scoped K6 inventory in the ONE accepted ql.m-ledger/v1.

No new coordinate tree/schema. Refresh keeps every other vertical's entries.
A retained table is structural-index-only, not computational readiness. Finite
operations are separately assessed, while broad source capabilities stay partial.
"""
from __future__ import annotations
import argparse, copy, hashlib, importlib.util, json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
spec=importlib.util.spec_from_file_location('mledger',ROOT/'scripts/m-ledger.py')
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
PREFIX='k6-m2:'
PROOF='fixtures/kernel/m2-finite-proof-v1.json'
SUMMARY='fixtures/kernel/m2-field-census-summary-v1.json'
DOC='docs/kernel-rebuild/m2-engine-v1.md'

def load(path):return json.loads((ROOT/path).read_text())
def write(path,value): (ROOT/path).write_text(json.dumps(value,ensure_ascii=False,indent=2)+'\n')
def lock(path):return m.lock(ROOT,path)
def row_id(name):return PREFIX+name

def materialize(previous):
    # Start from the canonical importer, retaining K5/K7 inventories verbatim.
    result=m.refresh(ROOT,copy.deepcopy(previous))
    original_order={key:{v["id"]:i for i,v in enumerate(result[key])} for key in ("rows","implementations","evidence","discrepancies")}
    prior_discrepancies={d["id"]:copy.deepcopy(d) for d in result["discrepancies"] if d["id"].startswith(PREFIX)}
    for key in ('rows','implementations','evidence','discrepancies'):
        result[key]=[v for v in result[key] if not v['id'].startswith(PREFIX)]
    result['assessments']={k:v for k,v in result['assessments'].items() if not k.startswith(PREFIX)}
    cat=load('fixtures/kernel/m2-retained-c-v1.json')
    sources=[r for r in result['rows'] if r['source'] is not None and r['scope']=='M2']
    new=[];impls=[]
    def add(name,role,coords,c,rust,kind='computational',path='crates/ql-mef/src/m2.rs'):
        ident=row_id(name);bindings=[]
        for peer,symbols,where in [('c',c,'c/src/m2.c'),('rust',rust,path)]:
            for symbol in symbols:
                binding=f'{ident}:{peer}:{symbol}';bindings.append(binding)
                impls.append({'id':binding,'stratum':peer,'kind':kind,'path':where,'symbol':symbol,
                    'disposition':'cross-coordinate' if len(coords)>1 else 'coordinate-bound',
                    'coordinates':coords,'relations':[],'structure':[],
                    'rationale':role+'; scoped holding/operation, never the full semantics of the coordinate.'})
        new.append({'id':ident,'role':role,'scope':'M2','coordinates':coords,'source':None,
            'assessment':row_id('index' if kind=='structural-index' else ('finite' if c else 'rust-producer')),
            'bindings':bindings,'dispositions':{p:('bound' if symbols else 'planned') for p,symbols in [('c',c),('rust',rust)]},
            'invariants':['same-K2-identities','not-whole-subsystem-readiness'],'relations':[],'dependencies':[]})
        return ident
    for table in cat['tables']:
        coords=list(dict.fromkeys([table['scope']]+[c for c in table['bindings'] if c is not None]))
        add('table-'+table['name'],'Retained '+table['name']+' descriptor association; source values/names may disagree',
            coords,['ql_m2_table','ql_m2_record'],['catalogue','reading'],'structural-index')
    ops=[
      ('ground','Numerical cardinality/phi and distinct register factorisations',['#2-0'],['ql_m2_ground'],['ground_factors']),
      ('carrier','Four distinct 72-reading flatten/unflatten operations',['#2-0','#2-1','#2-2','#2-3','#2-4.5'],['ql_m2_flatten','ql_m2_unflatten'],['from_axes','axes']),
      ('signature','Element/chakra/phase packed carrier and inverse',['#2-2','#2-5'],['ql_m2_signature','ql_m2_unpack_signature'],['elemental_signature','unpack_signature']),
      ('tattva-step','Bounded manifestation/reabsorption traversal, not ontology reduction',['#2-2'],['ql_m2_tattva_step'],['tattva_step']),
      ('element-order','Explicit decan F/E/A/W order to EFWA material fibre conversion',['#2-3','#2-5'],['ql_m2_decan_to_fibre'],['decan_to_fibre']),
      ('asma-route','Retained internal/projective masks and arithmetic digital root (not source-value correction)',['#2-4.0'],['ql_m2_asma_route','ql_m2_digital_root'],['asma_is_projective','asma_is_internal','digital_root']),
      ('planet','Retained outer-planet preemption and finite longitude aspects',['#2-5','#2-3'],['ql_m2_planet_preempted','ql_m2_aspect'],['planet_is_preempted','aspect']),
      ('maqam','Playback of declared retained 24-TET patterns; Bimba tuning equivalence unproven',['#2-4.3'],['ql_m2_maqam_pitch'],['maqam_pitches']),
      ('det','Keep scalar floor transform, expansion and historical OR mask distinct',['#2-5'],['ql_m2_scalar_compress','ql_m2_scalar_expand','ql_m2_legacy_det'],['scalar_compress','scalar_expand','legacy_det']),
      ('templateure','Bounded modal Q, power, and authoritative I4 tensor 18-to-16 coherent fold',['#2-0','#2-5'],['ql_m2_fibre_target','ql_m2_modal_quadrature','ql_m2_modal_power','ql_m2_modal_transduce'],['fibre_target','quadrature','total_power','form_potential']),
    ]
    for args in ops:add(*args)
    add('vimarsha','Active M2-1 source reader with supplied M1 ratio/M3 pose; seven musical modes are not Context Frames',
        ['#2-1'],['ql_m2_vimarsha'],['read_seed','read_from_pose'],path='crates/ql-mef/src/m2_vimarsha.rs')
    add('mef-bridge','C-grouped/Rust-interleaved lens bridge; preserves seven canonical Context Frames',['#2-1'],[],['mef_sublens','from_sublens','context_condition'])
    add('world-face','Resolve paired decan readings from supplied longitude, not an ephemeris',['#2-3','#2-5'],[],['situated_decan','linked_readings','causal_resonances'])
    add('engine','Same-event versioned full-field engine frame and validated provider modal/material handoff',['#2'],[],['execute','validate'],path='crates/ql-mef/src/m2_engine.rs')
    field=load('fixtures/kernel/m2-correspondences-v1.json')
    correspondences=list(dict.fromkeys(c for rule in field['rules'] for c in
        [rule['maqam_coordinate'],rule['planet_coordinate'],rule['chakra_coordinate'],rule['tattva_coordinate']] if c))
    add('correspondence-source','Compiled source-attributed maqam/planet/chakra/element/colour paths, not retained-ruler equivalence',
        correspondences,['ql_m2_correspondence','ql_m2_correspondence_at'],['correspondence_field','rule'],
        'structural-index',path='crates/ql-mef/src/m2_condition.rs')
    add('correspondence-pitch','Separate retained and explicitly spelled 24-TET operations with unsupported-source rejection',
        list(dict.fromkeys(rule['maqam_coordinate'] for rule in field['rules'])),
        ['ql_m2_correspondence_pitch'],['condition_pitches'],path='crates/ql-mef/src/m2_condition.rs')
    add('condition-producer','Joint event/MEF/drive/source-path/colour/material/world and distinct M3 transformation outputs',
        ['#2','#2-1','#2-2','#2-3','#2-4','#2-5'],[],['resolve'],path='crates/ql-mef/src/m2_condition.rs')
    result['rows']+=new;result['implementations']+=impls
    subjects=[r['id'] for r in new+sources]
    # Every evidence claim remains scoped to what the executable proof measures.
    proof_evidence=row_id('finite-proof'); census_evidence=row_id('census')
    for kind,path,eid,strata,axes in [
        ('test-receipt',PROOF,proof_evidence,['source','c','rust'],['source','coordinate','relation','operational']),
        ('observation',SUMMARY,census_evidence,['source'],['source','coordinate','relation'])]:
        result['evidence'].append({'id':eid,'kind':kind,'artifact':lock(path),'registry_revision':result['registry']['revision'],
            'subjects':subjects,'strata':strata,'axes':axes,'result':'passed'})
    def assessment(c_status,r_status,parity=False):
        a=m.assessment()
        a['readiness']['source']={'status':'partial','warrant':'observed','evidence':[census_evidence]}
        for peer,status in [('c',c_status),('rust',r_status)]:
            if status!='unassessed':a['readiness'][peer]={'status':status,'warrant':'tested','evidence':[proof_evidence]}
        if parity:
            for axis in ['source','coordinate','relation','operational']:
                a['parity'][axis]=[{'from_peer':'c','to_peer':'rust','status':'equivalent','warrant':'tested','evidence':[proof_evidence]}]
        return a
    result['assessments'].update({row_id('index'):assessment('structural-index-only','structural-index-only',True),
        row_id('source-index'):assessment('structural-index-only','structural-index-only'),
        row_id('finite'):assessment('verified','verified',True),row_id('rust-producer'):assessment('unassessed','verified'),
        row_id('source-only'):assessment('unassessed','unassessed'),row_id('partial'):assessment('partial','partial'),
        row_id('rust-partial'):assessment('unassessed','partial')})
    # Exact intersection through K2 aliases; never alter imported coordinates.
    reg=load('fixtures/kernel/m-tree-v1.json');lookup={}
    for n in reg['nodes']:
        lookup[n['source_ref']]=n['id']
        if n['source_ref'].startswith('#'):lookup['M'+n['source_ref'][1:]]=n['id']
        for alias in n['aliases']:lookup[alias]=n['id']
    for row in sources:
        if row['assessment'] != 'unassessed' and not row['assessment'].startswith(PREFIX):
            continue  # A later reviewed assessment belongs to its owner; never overwrite it.
        if any(row['dispositions'].get(peer) not in (None, 'planned') for peer in ('cpp','neo4j')):
            raise ValueError('M2 source row has later embodiment bindings; preserve and reconcile: '+row['id'])
        ids={lookup.get(c) for c in row['coordinates']}-{None}
        relevant=[r for r in new if ids.intersection(lookup.get(c) for c in r['coordinates'])]
        # Empty imported coordinate sets stay empty; consume the engine through
        # dependencies rather than inventing bindings. M2-C18 belongs to M1/M2'.
        row['dependencies']=sorted(set([d for d in row['dependencies'] if not d.startswith(PREFIX)]+[r['id'] for r in relevant]+[row_id('engine')]))
        row['bindings']=sorted(set([b for b in row['bindings'] if not b.startswith(PREFIX)]+[b for r in relevant for b in r['bindings']]))
        row['invariants']=list(dict.fromkeys(row['invariants']+['K6: broad capability is not closed by retained lookup or finite subset','K6: see M2 discrepancy register and provider boundaries']))
        computed={peer:any(i['id'] in row['bindings'] and i['stratum']==peer and i['kind']=='computational' for i in impls) for peer in ['c','rust']}
        indexed={peer:any(i['id'] in row['bindings'] and i['stratum']==peer for i in impls) for peer in ['c','rust']}
        row['assessment']=row_id('partial' if all(computed.values()) else 'rust-partial' if computed['rust'] else 'source-index' if all(indexed.values()) else 'source-only')
        for peer in ['c','rust']:
            has_binding=any(i['id'] in row['bindings'] and i['stratum']==peer for i in result['implementations'])
            row['dispositions'][peer]='bound' if has_binding else row['dispositions'].get(peer,'planned')
        row['dispositions'].update({'cpp':'planned','neo4j':'planned'})
    differences=[
      ('colour-source','source','bimba','rust','The locked Bimba cut yields 127 unique tonic/dominant-to-chakra paths and 17 missing paths. Only explicit yellow-square, silver-crescent and red-triangle yantra colours are admitted; other centres do not acquire a guessed palette. Retained C rulers, source tonic and source dominant readings remain separate.'),
      ('tattva','coordinate','bimba','c','Two retained principles (Vidya and Ear) lack exact K2 bindings; retain records 7 and 16 unbound rather than aliasing neighbours.'),
      ('planet','coordinate','bimba','c','Uranus (mod-10 id 7) is absent from the accepted serialized M2 coordinates; Earth is separate ground, not a replacement.'),
      ('asma','source','bimba','c','122 matched-name Abjad/digital-root field disagreements and 14 stored-vs-arithmetic digital-root discrepancies remain explicit; 34 names lack unique normalized bindings. Hidden name 99 keeps its sentinel.'),
      ('station','source','bimba','c','C uses LaMaqam/Tawba/Sabr/Shukr/Khawf/Raja/Tawakkul/Rida; Bimba includes Wara/Zuhd/Faqr instead. Bind 15 levels by station name; leave 9 unresolved.'),
      ('mantra','coordinate','bimba','c','Matrika has 50 source leaves; the second 50 retained mantra records remain branch-held without invented Malini leaves or claimed phonemic-permutation parity.'),
      ('maqam','source','bimba','c','All 72 Bimba interval declarations remain paired with retained C 24-TET patterns in the full census; identical musical tuning is not asserted.'),
      ('shem','source','bimba','c','Shem choir labels differ in places; the explicit 8-by-9 structural slot binding is not a semantic name-equivalence claim.'),
      ('resonance','relation','c','bimba','Historical estimated same-position masks are not the full causal-resonance graph. The source-preserving K2 M2 field has 672 CAUSAL_RESONANCE records; return actual registered relations separately.'),
      ('mef-source','source','bimba','rust','Six older Bimba MEF branch labels do not replace the twelve current L/L-prime lenses. The source tree remains asymmetric; no twelve-node tree is fabricated.'),
      ('continuous','experiential','rust','cpp','K8/K9 must supply and validate physical mode eigenstructure, materials, geometry, excitation/damping and experiential resonance. This producer validates/transports those inputs, not a physical solver.'),
      ('m1-m3','operational','rust','cpp','Vimarsha consumes explicit M1 harmonic ratio and actual shared M3 pose under one event. The retained 84-to-472 projection is not promoted as a new core law or assumed to be the final K5/K7 engine ABI.'),
    ]
    for name,axis,fr,to,detail in differences:
        result['discrepancies'].append({'id':row_id('difference-'+name),'subjects':['M2'],'axis':axis,'from_peer':fr,'to_peer':to,'state':'open','detail':detail,
            'current_authority':{'peer':'bimba','reference':DOC,'reason':'Preserve exact source identity and separate computational readings; no silent canon promotion.'},
            'proposal':None,'decision':None,'promotion':None,'history':[{'state':'open','reference':DOC}]})
    for i,d in enumerate(result['discrepancies']):
        prior=prior_discrepancies.get(d['id'])
        if prior and (prior['state']!='open' or prior.get('proposal') or prior.get('decision')):
            result['discrepancies'][i]=prior  # Preserve the discrepancy lifecycle on refresh.
    for key,order in original_order.items():
        result[key].sort(key=lambda v:order.get(v['id'],len(order)))
    result['assessments']=dict(sorted(result['assessments'].items()))
    result=m.refresh(ROOT,result)
    return result

def proof_refresh():
    receipt=load('target/m2-receipt/acceptance.json')
    if receipt['result']!='passed':raise ValueError('run the complete M2 acceptance first')
    for path,sha in receipt['inputs'].items():
        if lock(path)['sha256']!=sha:raise ValueError('acceptance is stale: '+path)
    parity=load('target/m2-receipt/rust-parity.json');vim=load('target/m2-receipt/vimarsha-parity.json');condition=load('target/m2-receipt/condition-parity.json')
    if parity['result']!='pass' or vim['result']!='passed' or condition['result']!='passed':raise ValueError('missing executed peer proof')
    write(PROOF,{'schema':'ql.m2-finite-proof/v1','result':'passed','standing':'exact input hashes executed; final CI acceptance.json binds execution to the accepted Git revision',
        'inputs':receipt['inputs'],'finite_observation':parity,'vimarsha_observation':vim,'condition_observation':condition,'checks':receipt['checks'],
        'not_claimed':receipt['not_claimed']+['whole-source capability completion from descriptor lookup']})

def main():
    p=argparse.ArgumentParser();p.add_argument('command',choices=['refresh','check']);a=p.parse_args()
    if a.command=='refresh':
        proof_refresh();write(m.LEDGER,materialize(load(m.LEDGER)))
    proof=load(PROOF)
    for path,sha in proof['inputs'].items():
        if lock(path)['sha256']!=sha:raise ValueError('stale finite proof input: '+path)
    # Fresh executable observations must accompany CI/local acceptance.
    for key,path in [('finite_observation','rust-parity.json'),('vimarsha_observation','vimarsha-parity.json'),('condition_observation','condition-parity.json')]:
        if proof[key]!=load('target/m2-receipt/'+path):raise ValueError('fresh observations do not match checked-in scoped proof')
    ledger=load(m.LEDGER);m.verify(ROOT,ledger)
    if materialize(ledger)!=ledger:raise ValueError('K6 inventory or evidence locks are stale; refresh without replacing other verticals')
    print('K6 shared-ledger inventory, scoped evidence, source dispositions and gaps verified')
if __name__=='__main__':main()
