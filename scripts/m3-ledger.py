#!/usr/bin/env python3
"""Reconcile K7's exact holdings, finite proofs and open source findings in the existing M ledger.

K4 discovery rows, the common tree/schema, and other vertical namespaces stay
owned by their existing producers. Source-index parity never asserts whole-M3
readiness. Reviewed discrepancy decisions survive a reproducible refresh.
"""
from __future__ import annotations
import argparse,copy,importlib.util,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
spec=importlib.util.spec_from_file_location('mledger',ROOT/'scripts/m-ledger.py')
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
PREFIX='k7-m3:'
PROOF='fixtures/kernel/m3-finite-proof-v1.json'
SUMMARY='fixtures/kernel/m3-coverage-v1.json'
DOC='docs/KERNEL-M3-ENGINE-CONTRACT.md'
def load(p):return json.loads((ROOT/p).read_text())
def write(p,v):(ROOT/p).write_text(json.dumps(v,ensure_ascii=False,indent=2)+'\n')
def owned(i):return i.startswith(PREFIX)
def materialize(previous):
 result=m.refresh(ROOT,copy.deepcopy(previous))
 order={k:{v['id']:i for i,v in enumerate(result[k])} for k in ['rows','implementations','evidence','discrepancies']}
 prior={v['id']:copy.deepcopy(v) for v in result['discrepancies']}
 for k in order:result[k]=[v for v in result[k] if not owned(v['id'])]
 result['assessments']={k:v for k,v in result['assessments'].items() if not owned(k)}
 cat=load('fixtures/kernel/m3-domain-v1.json');nodes=cat['nodes'];byref={n['ref']:n for n in nodes}
 registry=load('fixtures/kernel/m-tree-v1.json');lookup={}
 for n in registry['nodes']:
  lookup[n['source_ref']]=n['id']
  if n['source_ref'].startswith('#'):lookup['M'+n['source_ref'][1:]]=n['id']
  for alias in n['aliases']:lookup[alias]=n['id']
 def coords(*roles):return [n['ref'] for n in nodes if n['role'] in roles]
 new=[];implementations=[]
 def add(name,description,coordinates,c_bindings,rust_bindings,index=False):
  ident=PREFIX+name;bindings=[]
  coordinates=list(dict.fromkeys(coordinates))
  for peer,items in [('c',c_bindings),('rust',rust_bindings)]:
   for path,symbol in items:
    bid=ident+':'+peer+':'+symbol;bindings.append(bid)
    implementations.append({'id':bid,'stratum':peer,'kind':'structural-index' if index else 'computational','path':path,'symbol':symbol,
      'coordinates':coordinates,'relations':[],'structure':[],
      'disposition':'cross-coordinate' if len(coordinates)>1 else 'coordinate-bound',
      'rationale':description+'; this scoped operation/reading does not certify the whole coordinate.'})
  new.append({'id':ident,'role':description,'scope':'M3','coordinates':coordinates,'source':None,
   'assessment':PREFIX+('index' if index else 'finite' if c_bindings else 'producer'),
   'bindings':bindings,'dispositions':{'c':'bound' if c_bindings else 'planned','rust':'bound' if rust_bindings else 'planned','cpp':'planned'},
   'invariants':['one-accepted-K2-registry','qualified-source-not-blanket-semantic-equivalence','exact-discrete-before-continuous'],
   'relations':[],'dependencies':[]})
 def cb(*s,path='c/src/m3.c'):return [(path,x) for x in s]
 def rb(*s,path='crates/ql-mef/src/m3_engine.rs'):return [(path,x) for x in s]
 for role in sorted({n['role'] for n in nodes}):
  add('source-'+role,'Complete source '+role+' payload identity/qualification; no computational completion inferred',coords(role),
      cb('ql_m3_source_node_at',path='c/src/m3_domain.c'),rb('node',path='crates/ql-mef/src/m3_source.rs'),True)
 add('source-relations','All 4891 exact qualified source relations, preserving duplicate/null endpoints',[n['ref'] for n in nodes],
     cb('ql_m3_source_relation_at',path='c/src/m3_domain.c'),rb('relations',path='crates/ql-mef/src/m3_source.rs'),True)
 add('source-matrices','All 184 source matrix cells with independently attributed pair/codon links',coords('matrix-cell'),
     cb('ql_m3_matrix_cell_at','ql_m3_matrix_cell_relation',path='c/src/m3_domain.c'),rb('matrix_cells','relation',path='crates/ql-mef/src/m3_source.rs'),True)
 add('codon','Ratified four-state nucleotide, 64 codons and four-charge computation',coords('nucleotide','dna-codon'),
     cb('ql_m3_codon'),rb('four_charge','site_values',path='crates/ql-core/src/pole/codon.rs'))
 add('pair','All 16 ratified sum/difference pair operations',coords('dinucleotide'),cb('ql_m3_pair'),rb('pair_sum','pair_difference',path='crates/ql-core/src/pole/transcription.rs'))
 add('iching','Independent source address binding and native trigram/nuclear/complement law',coords('trigram','hexagram'),cb('ql_m3_trigram','ql_m3_hexagram'),rb('nuclear_hexagram','complement',path='crates/ql-core/src/pole/iching.rs'))
 add('line-change','Complete 384 native XOR transitions, separate from stale source edge assertions',coords('hexagram'),cb('ql_m3_line_change'),rb('line_change',path='crates/ql-core/src/pole/codon.rs'))
 add('matrix','Three exact native family operations and explicit resonance gaps',['#3-3-2-0','#3-3-2-1','#3-3-2-2'],cb('ql_m3_apply_matrix','ql_m3_matrix_partner'),rb('apply_matrix',path='crates/ql-core/src/pole/fold.rs'))
 add('rotation','472 lawful pose ordinals and 512 ranked/sweep candidate rotations',coords('dna-codon','phase-codon'),cb('ql_m3_pose_ordinal','ql_m3_rotations'),rb('generate_rotational_states',path='crates/ql-core/src/pole/rotational.rs')+rb('ordinal',path='crates/ql-core/src/pole/pose.rs'))
 add('quaternion','Codon orientation seed, rotations and environment-conditioned bin; not primary-address inverse',coords('dna-codon'),cb('ql_m3_quaternion','ql_m3_active_state'),rb('quat_codon_state','quat_active_state',path='crates/ql-core/src/pole/quaternion.rs'))
 add('tarot','Retained 56-card exact cover and 22 Major records; source associations separate',coords('minor-arcana','major-arcana'),cb('ql_m3_minor','ql_m3_major'),rb('minor','major',path='crates/ql-core/src/pole/tarot.rs'))
 add('transcription','128 exact DNA/RNA readings at unchanged nucleotide polarity',coords('dna-codon','phase-codon','rna-codon'),cb('ql_m3_transcribe'),rb('transcribe'))
 add('qualified-genetics','Plural and conditional source translations with exact RNA and expression refs',coords('phase-codon','rna-codon','amino-or-translation-signal'),[],rb('genetic',path='crates/ql-mef/src/m3_source.rs'))
 add('reception','Keep 72-to-64 scalar compression, roundtrip loss and OR-mask transduction distinct',['#3-0'],cb('ql_m3_transduce','ql_m3_epogdoon'),rb('transduce_vibration_to_symbol','apply_epogdoon_compression',path='crates/ql-core/src/pole/transcription.rs'))
 add('clock','Unwrapped checked 360/720 clock, source governor/clockwise/opposite identities',coords('clock-degree','clock-backbone'),cb('ql_m3_clock','ql_m3_clock_advance'),rb('clock')+rb('advance',path='crates/ql-core/src/m3_clock.rs'))
 add('clock-record','26-column legacy clock record retained as estimates/placeholders, not canon',coords('clock-degree'),cb('ql_m3_clock_record'),rb('recorded_clock',path='crates/ql-core/src/m3_clock.rs'),True)
 add('clock-prototype','24 independent source codon/hexagram backbone prototypes, not current selected form',coords('clock-backbone'),cb('ql_m3_backbone_projection',path='c/src/m3_domain.c'),rb('backbone',path='crates/ql-mef/src/m3_source.rs'),True)
 add('form','All lawful fold/pose/aperture projections and matrix outcomes in EFWA order',['#3-2','#3-3','#3-5'],cb('ql_m3_form','ql_m3_form_apply_matrix','ql_m3_cast',path='c/src/m3_domain.c'),rb('from_codon','from_cast','apply_matrix',path='crates/ql-core/src/pole/fold.rs'))
 add('aperture','Accepted 18-lens relation: 16 static plus Fibonacci ground and Anuttara ring',['#3-5'],cb('ql_m3_form',path='c/src/m3_domain.c'),rb('division_deg10','complement_deg10','orientation',path='crates/ql-core/src/pole/aperture.rs'))
 add('parent-producer','Atomic same-event/subject/generation Cosmic-Personal-M3-prime state and replay; owner identity remains separate',['#3','#3-0','#3-3','#3-5'],[],rb('new','apply','snapshot',path='crates/ql-mef/src/m3_state.rs'))
 result['rows']+=new;result['implementations']+=implementations
 source_rows=[r for r in result['rows'] if r['scope']=='M3' and r['source'] is not None]
 for name,path,strata,axes in [('proof',PROOF,['source','c','rust'],['source','coordinate','relation','operational']),('coverage',SUMMARY,['source'],['source','coordinate','relation'])]:
  result['evidence'].append({'id':PREFIX+name,'kind':'test-receipt' if name=='proof' else 'observation','artifact':m.lock(ROOT,path),
   'registry_revision':result['registry']['revision'],'subjects':[r['id'] for r in new+source_rows],'strata':strata,'axes':axes,'result':'passed'})
 def assessment(c,r,index=False,parity=False):
  a=m.assessment();a['readiness']['source']={'status':'partial','warrant':'observed','evidence':[PREFIX+'coverage']}
  for p,status in [('c',c),('rust',r)]:
   if status!='unassessed':a['readiness'][p]={'status':status,'warrant':'tested','evidence':[PREFIX+'proof']}
  if parity:
   for axis in ['source','coordinate','relation']+([] if index else ['operational']):
    a['parity'][axis]=[{'from_peer':'c','to_peer':'rust','status':'equivalent','warrant':'tested','evidence':[PREFIX+'proof']}]
  return a
 result['assessments'].update({PREFIX+'index':assessment('structural-index-only','structural-index-only',True,True),
   PREFIX+'finite':assessment('verified','verified',parity=True),PREFIX+'producer':assessment('unassessed','verified'),
   PREFIX+'partial':assessment('partial','partial'),PREFIX+'source-only':assessment('unassessed','unassessed')})
 for row in source_rows:
  if row['assessment']!='unassessed' and not owned(row['assessment']):
   # K4 discovery assessments retain their standing; K7 dependencies/readings
   # describe new evidence without replacing another reviewed assessment.
   preserve_assessment=True
  else:preserve_assessment=False
  ids={lookup.get(c) for c in row['coordinates']}-{None}
  relevant=[r for r in new if ids.intersection(lookup.get(c) for c in r['coordinates'])]
  row['dependencies']=sorted(set([d for d in row['dependencies'] if not owned(d)]+[r['id'] for r in relevant]+[PREFIX+'parent-producer']))
  row['bindings']=sorted(set([b for b in row['bindings'] if not owned(b)]+[b for r in relevant for b in r['bindings']]))
  if not preserve_assessment:row['assessment']=PREFIX+('partial' if row['bindings'] else 'source-only')
  row['invariants']=list(dict.fromkeys(row['invariants']+['K7: broad source capability is not certified by a finite operator or descriptor','K7: current discrepancy lifecycle belongs to ql.m-ledger/v1','K7: Cosmic/Personal/deep consumers retain one event and parent subject']))
  for p in ['c','rust']:
   if any(i['id'] in row['bindings'] and i['stratum']==p for i in result['implementations']):row['dispositions'][p]='bound'
 # Source audit owns exact retained assertions; the canonical ledger owns the
 # mutable discrepancy lifecycle. Never replace a reviewed decision on refresh.
 present={d['id'] for d in result['discrepancies']}
 for d in cat['discrepancies']:
  if d['id'] not in present:result['discrepancies'].append(copy.deepcopy(d));present.add(d['id'])
 for name,detail,axis,fr,to in [
  ('continuous','K8/K9/K10 consume the installed discrete contract; continuous simulation, real desktop bindings and experiential acceptance are not supplied by K7.','experiential','rust','cpp'),
  ('inverse','Entity quaternion to primary address remains the declared selection-law research seam; an explicit form choice never certifies an inverse.','operational','bimba','rust'),
  ('research','ORF/protein, Clifford/Pauli and 137 hypotheses retain source depth without fabricated biological/physical operation.','operational','bimba','rust'),
  ('aperture-source','Accepted Rust 18-lens canon is promoted into native C with exhaustive parity. Older imported 16+1 prose remains a source discrepancy, not current clock authority.','source','bimba','c')]:
  ident=PREFIX+'difference-'+name
  result['discrepancies'].append(prior.get(ident,{'id':ident,'subjects':['M3'],'axis':axis,'from_peer':fr,'to_peer':to,'state':'open','detail':detail,
   'current_authority':{'peer':'rust','reference':DOC,'reason':'Retain current warranted native law and exact source separately; consumer and research work is explicit.'},
   'proposal':None,'decision':None,'promotion':None,'history':[{'state':'open','reference':DOC}]}))
 for k,o in order.items():result[k].sort(key=lambda v:o.get(v['id'],len(o)))
 result['assessments']=dict(sorted(result['assessments'].items()))
 return m.refresh(ROOT,result)
def coverage():
 cat=load('fixtures/kernel/m3-domain-v1.json');ledger=load(m.LEDGER)
 census={r['coordinate']:r for r in load('fixtures/kernel/census/census-m3.json')['coordinates']}
 rows=[{'coordinate':n['ref'],'id':n['id'],'source_role':n['role'],'k4_row':'census:'+n['ref'],
 'c':'qualified-source-descriptor','rust':'full-source-payload-and-qualified-relations',
 'operational_disposition':'see-scoped-k7-operation-rows; descriptor-is-not-whole-coordinate-implementation',
 'k4_discovery':{'c':census[n['ref']]['c']['state'],'rust':census[n['ref']]['rust']['state']}} for n in cat['nodes']]
 return {'schema':'ql.m3-coverage/v1','registry_revision':cat['registry_revision'],'domain_revision':cat['catalogue_revision'],
  'accepted_k4':'5b24b95d17234ab5d23d84e658c0cc06434b41a3','coordinates':rows,'source_relations':len(cat['relations']),
  'source_capability_ids':[r['id'] for r in ledger['rows'] if r['scope']=='M3' and r['source']],
  'source_discrepancy_templates':len(cat['discrepancies']),'contract':DOC,
  'standing':'complete known coordinate/source coverage, scoped finite operations, broad capabilities retain incomplete research/provider/experiential depth'}
def refresh_proof():
 r=load('target/m3-acceptance/acceptance.json')
 if r['result']!='passed':raise ValueError('M3 acceptance must pass first')
 for p,h in r['inputs'].items():
  if m.lock(ROOT,p)['sha256']!=h:raise ValueError('M3 proof input changed after execution: '+p)
 write(PROOF,{k:v for k,v in r.items() if k!='revision'}|{'schema':'ql.m3-finite-proof/v1','standing':'executed exact input hashes; final CI receipt names the Git head; not whole-source readiness'})
def main():
 p=argparse.ArgumentParser();p.add_argument('command',choices=['refresh','check']);a=p.parse_args()
 if a.command=='refresh':
  refresh_proof();write(SUMMARY,coverage());write(m.LEDGER,materialize(load(m.LEDGER)))
 proof=load(PROOF)
 for path,h in proof['inputs'].items():
  if m.lock(ROOT,path)['sha256']!=h:raise ValueError('stale M3 proof input: '+path)
 if load(SUMMARY)!=coverage():raise ValueError('M3 coverage stale')
 ledger=load(m.LEDGER);m.verify(ROOT,ledger)
 if materialize(ledger)!=ledger:raise ValueError('K7 ledger stale; refresh preserving other verticals')
 print('K7 exact coordinate holdings, finite evidence and discrepancy lifecycles verified')
if __name__=='__main__':main()
