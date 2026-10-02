#!/usr/bin/env python3
"""Native-owner candidate generation for the current source-qualified scene default.

Only JSON authoring/source qualification and existing native CLI invocation occur
here. No M1/M2/M3 arithmetic, astronomical calculation, binding projection or
worker protocol is implemented in this script. Its output is NOT operative until
the native owner's fixture/include/refresh/tests are deliberately integrated.
"""
import argparse,copy,hashlib,json,os,subprocess,time
from pathlib import Path

def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def read(path):return json.loads(path.read_text())
def write(path,value):path.write_text(json.dumps(value,indent=2,ensure_ascii=False)+'\n')
def main():
 p=argparse.ArgumentParser()
 p.add_argument('--ql-bin',type=Path,required=True)
 p.add_argument('--expected-ql-sha256',required=True)
 p.add_argument('--expected-provider-sha256',required=True)
 p.add_argument('--original-recipe',type=Path,required=True)
 p.add_argument('--expected-recipe-sha256',required=True)
 p.add_argument('--original-m2-catalogue',type=Path,required=True)
 p.add_argument('--current-m2-catalogue',type=Path,required=True)
 p.add_argument('--admitted-historical-sky',type=Path,required=True)
 p.add_argument('--provider-python',type=Path,required=True)
 p.add_argument('--output-dir',type=Path,required=True)
 a=p.parse_args()
 assert digest(a.ql_bin)==a.expected_ql_sha256,'native CLI changed'
 assert digest(a.original_recipe)==a.expected_recipe_sha256,'authored recipe changed'
 old=read(a.original_recipe);oldcat=read(a.original_m2_catalogue);cat=read(a.current_m2_catalogue);sky=read(a.admitted_historical_sky)
 assert old['schema']=='ql.coupled-event-request/v1'
 original_sky=[s for s in old['source_receipts'] if s.get('schema')=='ql.sky-snapshot/v1']
 assert len(original_sky)==1 and sky['request']['mode']=='historical'
 assert sky['epoch_unix_ms']==original_sky[0]['epoch_unix_ms']
 assert {k:v for k,v in sky['request'].items() if k!='mode'}=={k:v for k,v in original_sky[0]['request'].items() if k!='mode'},'astronomical input changed'
 assert sky['source_binding']['registry_revision']==cat['registry_revision']
 assert sky['source_binding']['sun_role']=='solar-parent'
 assert sky['provider']['adapter_sha256']==a.expected_provider_sha256
 # This bounded recovery establishes unchanged structural/numerical tables.
 # A future genuine table change requires source-led review, not silent indexing.
 assert len(oldcat['tables'])==len(cat['tables'])
 for table in oldcat['tables']:
  current=next(t for t in cat['tables'] if t['name']==table['name'])
  assert all(table[k]==current[k] for k in ['name','symbol','scope','columns','rows','bindings']),f"table interpretation changed: {table['name']}"
 a.output_dir.mkdir(parents=True,exist_ok=False)
 env=os.environ.copy();env['QL_NARA_PYTHON']=str(a.provider_python);env['QL_NARA_PROVIDER_CACHE']=str(a.output_dir/'provider-cache')
 receipts=[]
 def native(name,operation,request):
  assert digest(a.ql_bin)==a.expected_ql_sha256
  request_file=a.output_dir/(name+'.request.json');write(request_file,request)
  argv=[str(a.ql_bin),'scene',operation,str(request_file),'--json'];start=time.monotonic()
  child=subprocess.Popen(argv,stdout=subprocess.PIPE,stderr=subprocess.PIPE,env=env)
  try:stdout,stderr=child.communicate(timeout=45);timed_out=False
  except subprocess.TimeoutExpired:
   child.kill();stdout,stderr=child.communicate();timed_out=True
  (a.output_dir/(name+'.stdout.json')).write_bytes(stdout);(a.output_dir/(name+'.stderr.txt')).write_bytes(stderr)
  receipt={'argv':argv,'elapsed_seconds':time.monotonic()-start,'returncode':child.returncode,'timed_out':timed_out,'child_reaped':child.poll() is not None,'ql_sha256':digest(a.ql_bin),'request_sha256':digest(request_file),'stdout_sha256':hashlib.sha256(stdout).hexdigest(),'stderr_sha256':hashlib.sha256(stderr).hexdigest()}
  receipts.append(receipt);write(a.output_dir/'native-operation-receipts.json',receipts)
  assert receipt['ql_sha256']==a.expected_ql_sha256 and child.returncode==0 and not timed_out,(name,stderr.decode())
  return json.loads(stdout)
 # Actual existing native provider validates the complete snapshot and current
 # compiled source. The returned scene here is admission evidence, not default
 # material: compose() does not preserve the complete authored M2 seed.
 admitted=native('provider-admission','compose',{'schema':'ql.scene-request/v1','sky_snapshot':sky,'tick12':old['m1']['tick12'],'cycle':int(old['m1']['cycle'])})
 assert admitted['sky_admission']['snapshot_ref']==sky['snapshot_ref']
 assert admitted['sky_admission']['fresh_current_attested'] is False
 assert admitted['sky_admission']['validator_source']['revision']=='sha256:'+a.expected_provider_sha256
 qualification=admitted['sky_admission']['source_binding_qualification']
 assert qualification['legacy_descriptor_admitted'] is False
 assert qualification['native_sun_route']['chakra_coordinate']=='#2-5-0/1-7'
 assert qualification['native_sun_route']['chakra_index']==7
 seed=copy.deepcopy(old)
 seed['m2']['registry_revision']=sky['source_binding']['registry_revision']
 seed['m3']['registry_revision']=sky['source_binding']['registry_revision']
 binding=native('native-successor','binding',{'schema':'ql.scene-binding-request/v1','instance_ref':'ql:generic-default-source-recovery:2026-10-01','texture':[8,8],'units_per_metre':120,'event':seed,'sky':sky})
 event=binding['host']['basis']
 assert event['m1']==old['m1'] and event['harmonic_source']==old['harmonic_source']
 assert event['m2']['modal_coefficients']==old['m2']['modal_coefficients']
 assert event['m2']['condition']==old['m2']['condition'] and event['m2']['selections']==old['m2']['selections']
 assert all(event['m3'][k]==old['m3'][k] for k in ['address','pose','aperture','clock_steps','matrix_axis','rna','subject_ref'])
 assert event['source_receipts']==[sky]
 assert event['m3']['occurrence_unix_ms']==sky['epoch_unix_ms']
 assert event['m3']['receipt_unix_ms']==sky['receipt_unix_ms']
 assert binding['scene']['snapshot_ref']==sky['snapshot_ref'] and len(binding['scene']['bodies'])==10
 assert len(binding['native_basis']['derivation']['sky_voices'])==9
 write(a.output_dir/'scene-default-event-v2.candidate.json',event)
 write(a.output_dir/'candidate-provenance.json',{'schema':'epi-default-recovery-candidate/v1','standing':'explicit native candidate; output requires source-fenced fixture promotion and compiled default replay; no GUI/installed/readiness proof','original_fixture_sha256':a.expected_recipe_sha256,'original_sky_ref':original_sky[0]['snapshot_ref'],'new_sky_ref':sky['snapshot_ref'],'current_registry':cat['registry_revision'],'ql_sha256':a.expected_ql_sha256,'authored_m1':event['m1'],'authored_m3':{k:event['m3'][k] for k in ['address','pose','aperture','clock_steps','matrix_axis','rna']},'authored_m2_condition':event['m2']['condition'],'raw_event_vs_completed_basis':'The emitted fixture is binding.host.basis; native_basis.input adds source-owned nine-frequency/resonator completion. They are deliberately distinct.'})
if __name__=='__main__':main()
