#!/usr/bin/env python3
"""Actual native SceneInstrument, canonical browser receiver, real AudioContext/GPU.
Explicit inputs; no supplied-owner or mocked positive native response. Output only.
"""
from pathlib import Path
import argparse, copy, datetime, functools, hashlib, http.server, json, os, selectors, subprocess, threading, time
from playwright.sync_api import sync_playwright

def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def save(p,x):Path(p).parent.mkdir(parents=True,exist_ok=True);Path(p).write_text(json.dumps(x,ensure_ascii=False,indent=2,allow_nan=False)+'\n')
def pin(p):return {'path':str(Path(p).resolve()),'sha256':sha(p)}
class Host:
 def __init__(self,name,cfg,out,base,register):
  self.name=name;self.cfg=cfg;self.dir=out/'native'/name;self.dir.mkdir(parents=True);self.buf=b'';self.i=0;self.closed=False
  save(self.dir/'host-config.json',base)
  self.argv=[cfg['host_bin'],cfg['worker_bin'],str(self.dir/'host-config.json')]
  self.p=subprocess.Popen(self.argv,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,bufsize=0)
  self.row={'case':name,'pid':self.p.pid,'argv':self.argv,'closed':False};register(self);self.current=self.receive();save(self.dir/'000-ready.json',self.current)
  assert self.current['status']=='ready' and self.current['available'],'actual native ready'
 def receive(self):
  with selectors.DefaultSelector() as s:
   s.register(self.p.stdout,selectors.EVENT_READ);deadline=time.monotonic()+40
   while b'\n' not in self.buf:
    wait=deadline-time.monotonic();assert wait>0 and s.select(wait),'owned native acknowledgement timeout'
    b=os.read(self.p.stdout.fileno(),65536);assert b,'owned native EOF';self.buf+=b;assert len(self.buf)<128*1024*1024
  line,self.buf=self.buf.split(b'\n',1);return json.loads(line)
 def exchange(self,q):
  assert not self.closed,'owned host closed';self.i+=1;stem=f'{self.i:03d}-{q["command"]["operation"]}';save(self.dir/(stem+'-issued.json'),q)
  self.p.stdin.write((json.dumps(q,separators=(',',':'),allow_nan=False)+'\n').encode());self.p.stdin.flush();self.current=self.receive();save(self.dir/(stem+'-response.json'),self.current);return copy.deepcopy(self.current)
 def close(self):
  if self.closed:return self.row
  self.closed=True;self.p.stdin.close()
  try:self.row['exit_code']=self.p.wait(timeout=15)
  except subprocess.TimeoutExpired:
   self.row['owned_timeout_kill']=True;self.p.kill();self.row['exit_code']=self.p.wait(timeout=5)
  (self.dir/'stderr.log').write_text(self.p.stderr.read().decode());self.row['stderr']=pin(self.dir/'stderr.log');self.p.stdout.close();self.p.stderr.close();self.row['closed']=True;self.row['operations']=self.i;save(self.dir/'closure.json',self.row)
  assert self.row['exit_code']==0,'native process closure refused';return self.row
class QuietHandler(http.server.SimpleHTTPRequestHandler):
 def log_message(self,*args):pass

def main():
 ap=argparse.ArgumentParser();ap.add_argument('--config',type=Path,required=True);args=ap.parse_args();cfg=json.loads(args.config.read_text());out=Path(cfg['output_dir']);out.mkdir(parents=True,exist_ok=True)
 assert not (out/'receipt.json').exists(),'retain prior runs; choose new output_dir'
 binding=json.loads(Path(cfg['closed_scene_binding_file']).read_text());assert set(binding)=={'schema','host','presentation'};base=binding['host'];changed=json.loads(Path(cfg['changed_caller_recipe_file']).read_text())
 adapter=Path(cfg['adapter_directory']);qualified={**{k:Path(cfg[k]) for k in ['host_bin','worker_bin','closed_scene_binding_file','changed_caller_recipe_file','point_cloud_source','field_model_source']},'instrument_session_source':adapter/'instrument-session.mjs','retained_field_source':adapter/'retained-field.mjs','native_audio_source':adapter/'native-audio.mjs'}
 for role,path in qualified.items():
  assert sha(path)==cfg['expected_sha256'][role],f'independently supplied source qualification differs: {role}'
 for pointer,expected in cfg['expected_binding_values'].items():
  actual=binding
  for component in pointer.split('/'):actual=actual[component]
  assert actual==expected,f'independently qualified lawful-case determinant differs: {pointer}'
 source=Path(cfg['browser_test_source']);text=source.read_text().replace('__POINT_CLOUD_SOURCE__',cfg['point_cloud_source']).replace('__FIELD_MODEL_SOURCE__',cfg['field_model_source']).replace('__RETAINED_BINDING__',str(adapter/'retained-field.mjs')).replace('__INSTRUMENT_SESSION__',str(adapter/'instrument-session.mjs'))
 rendered=out/'browser-acceptance.ts';rendered.write_text(text)
 env={**os.environ,'NODE_PATH':cfg['node_modules']};argv=[cfg['esbuild_bin'],str(rendered),'--bundle','--format=esm','--platform=browser','--outfile='+str(out/'acceptance.js'),'--metafile='+str(out/'bundle-inputs.json')]
 build=subprocess.run(argv,env=env,capture_output=True,text=True);(out/'build.log').write_text(build.stdout+build.stderr);assert build.returncode==0,build.stderr
 (out/'index.html').write_text('<!doctype html><meta charset="utf-8"><title>Real native component proof</title><script type="module" src="/acceptance.js"></script>')
 report={'schema':'epi.real-native-instrument-browser-receipt/v1','started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'passed':False,'scope':'Separate actual native SceneInstrument + canonical InstrumentSession + browser AudioContext/WebGL2 retained receiver. No ordinary/managed installed whole, sound heard, physical hardware, source readiness/H acceptance. Negative delivered-receipt injection is explicit and only after actual native response.','inputs':[pin(args.config),pin(cfg['closed_scene_binding_file']),pin(cfg['changed_caller_recipe_file']),pin(cfg['host_bin']),pin(cfg['worker_bin']),pin(source),pin(__file__),pin(adapter/'instrument-session.mjs'),pin(adapter/'native-audio.mjs'),pin(adapter/'retained-field.mjs')],'build':{'argv':argv,'exit_code':build.returncode,'log':pin(out/'build.log'),'bundle':pin(out/'acceptance.js'),'metafile':pin(out/'bundle-inputs.json')},'owned_processes':[],'errors':[]}
 meta=json.loads((out/'bundle-inputs.json').read_text());report['compiled_inputs']=[]
 for p in meta['inputs']:
  pp=Path(p).resolve();assert pp.is_file(),str(pp);report['compiled_inputs'].append(pin(pp))
 hosts={};browser=None;server=http.server.ThreadingHTTPServer(('127.0.0.1',0),functools.partial(QuietHandler,directory=str(out)));threading.Thread(target=server.serve_forever,daemon=True).start()
 def open_host(name):
  assert name not in hosts
  def register(h):hosts[name]=h;report['owned_processes'].append(h.row)
  h=Host(name,cfg,out,base,register);return copy.deepcopy(h.current)
 def exchange(name,q):return hosts[name].exchange(q)
 def close(name):return hosts[name].close()
 def recipe(name):return copy.deepcopy(changed if name=='replace-delta2' else base['basis'])
 def proof(v):save(out/'browser-proof.json',v)
 try:
  with sync_playwright() as pw:
   launch_args=['--use-angle=swiftshader','--enable-unsafe-swiftshader','--autoplay-policy=no-user-gesture-required'];browser=pw.chromium.launch(headless=True,args=launch_args,executable_path=cfg.get('browser_executable'));report['browser']={'version':browser.version,'headless':True,'args':launch_args}
   page=browser.new_page(viewport={'width':800,'height':600});page.on('pageerror',lambda e:report['errors'].append(str(e)));page.on('console',lambda m:report['errors'].append('console:error:'+m.text) if m.type=='error' else None)
   page.expose_function('nativeSceneOpen',open_host);page.expose_function('nativeSceneExchange',exchange);page.expose_function('nativeSceneClose',close);page.expose_function('nativeSceneRecipe',recipe);page.expose_function('saveBrowserProof',proof)
   page.goto(f'http://127.0.0.1:{server.server_address[1]}/',wait_until='domcontentloaded');page.wait_for_function('window.acceptance !== undefined',timeout=240000)
   result=page.evaluate('window.acceptance');save(out/'browser-proof.json',result);report['result']=pin(out/'browser-proof.json');report['passed']=result['passed'];report['case_count']=len(result.get('cases',[]));report['failure']=result.get('error');page.screenshot(path=str(out/'browser.png'))
   browser.close();browser=None
 except Exception as e:report['failure']=repr(e)
 finally:
  for h in hosts.values():
   try:h.close()
   except Exception as e:report['errors'].append('closure:'+repr(e))
  if browser:
   try:browser.close()
   except Exception:pass
  server.shutdown();server.server_close();report['finished_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat();report['all_owned_closed']=all(x['closed'] and x.get('exit_code')==0 for x in report['owned_processes']);report['passed']=report['passed'] and report['all_owned_closed'];save(out/'receipt.json',report)
 print(json.dumps({'passed':report['passed'],'failure':report.get('failure'),'cases':report.get('case_count'),'receipt':pin(out/'receipt.json'),'all_owned_closed':report['all_owned_closed']}))
 return 0 if report['passed'] else 1
if __name__=='__main__':raise SystemExit(main())
