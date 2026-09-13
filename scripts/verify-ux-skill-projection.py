#!/usr/bin/env python3
"""Exercise real AIKit Skill delivery in a new disposable home; never model/human acceptance."""
from __future__ import annotations
import argparse, hashlib, json, os, pathlib, shutil, subprocess, sys, tomllib
P = pathlib.Path
MEMBERS = ('ql-foundations','ql-operation','ql-experience-prepare','ql-evidence-report','ql-experience-walk')
METHODS = {'ql-operation','bimba-cypher','refraction-adapter-authoring','ql-experience-prepare','ql-experience-walk'}
def sha(b: bytes) -> str: return hashlib.sha256(b).hexdigest()
def main() -> int:
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--aikit',required=True,type=P); ap.add_argument('--source-root',type=P,default=P(__file__).resolve().parents[1]);ap.add_argument('--output',required=True,type=P)
    args=ap.parse_args(); root=args.output.resolve(); binary=args.aikit.resolve(strict=True); source=args.source_root.resolve(strict=True)
    if root.exists(): raise ValueError('output must be a new disposable directory')
    root.mkdir(parents=True); home=root/'home';home.mkdir();bindir=root/'bin';bindir.mkdir();(bindir/'aikit').symlink_to(binary)
    env={'HOME':str(home),'AIKIT_HOME':str(home/'.aikit'),'XDG_CONFIG_HOME':str(home/'.config'),'XDG_DATA_HOME':str(home/'.local/share'),'XDG_STATE_HOME':str(home/'.local/state'),'AIKIT_CONTEXT_ID':'ctx_ql_ux_delivery','PATH':'/usr/bin:/bin','LANG':'C.UTF-8','TZ':'UTC'}
    copied=root/'reviewed-skills';shutil.copytree(source/'skills',copied,symlinks=False)
    inputs={f.parent.name: f.read_bytes() for f in copied.glob('*/SKILL.md')}
    if not set(MEMBERS)<=inputs.keys(): raise ValueError('missing source practice')
    # This is a controlled, explicitly reviewed local source, not trust in personal ground.
    (root/'source-review.json').write_text(json.dumps({'source_root':str(source),'sha256':{k:sha(v) for k,v in inputs.items()},'scope':'controlled test input only'},indent=2)+'\n')
    commands=[]
    def run(*cmd: str) -> dict:
        p=subprocess.run([str(bindir/'aikit'),'--json',*cmd],cwd=root,env=env,text=True,capture_output=True,timeout=120)
        commands.append({'argv':list(cmd),'returncode':p.returncode,'stdout':p.stdout,'stderr':p.stderr})
        (root/'commands.json').write_text(json.dumps(commands,indent=2)+'\n')
        if p.returncode: raise ValueError(f'native command failed: {cmd}; output retained')
        value=json.loads(p.stdout)
        if not value.get('ok'): raise ValueError(f'native error: {cmd}')
        return value['data']
    version=subprocess.run([str(bindir/'aikit'),'--version'],cwd=root,env=env,text=True,capture_output=True,check=True).stdout.strip()
    run('init');run('source','add-directory','ql-ux',str(copied));sync=run('source','sync','ql-ux');run('source','promote','ql-ux');show=run('source','show','ql-ux')
    registry=P(show['active_registry']).resolve()
    if not registry.is_relative_to(root): raise ValueError('registry escaped controlled home')
    records={}
    for f in registry.rglob('manifest.toml'):
        m=tomllib.loads(f.read_text()); folder=m['id'].rsplit('/',1)[-1]
        if folder not in inputs: continue
        payload=f.parent/m['skill']['root']/'SKILL.md'
        if payload.read_bytes()!=inputs[folder]: raise ValueError(f'source import changed {folder}')
        if m['description'].startswith('METHOD:') != (folder in METHODS): raise ValueError(f'Method classification differs: {folder}')
        records[folder]=m
    if set(records)!=set(inputs): raise ValueError('native catalogue omitted source Skills')
    ids=[records[x]['id'] for x in MEMBERS]
    run('set','create','epi-experience',*ids)
    withheld=run('set','show','epi-experience')
    if withheld['projected'] or len(withheld['withheld'])!=len(MEMBERS): raise ValueError('membership silently enabled Skills')
    for ident in ids: run('enable',ident,'--scope','global')
    observations=[]
    def inspect(phase: str, expected: set[str]):
        applied=run('apply');gen=applied['generation'];native_set=run('set','show','epi-experience')
        current=home/'.aikit/state/contexts'/env['AIKIT_CONTEXT_ID']/'current'
        if not current.is_symlink(): raise ValueError('no native current generation pointer')
        target=current.resolve(strict=True)
        if not target.is_relative_to(root): raise ValueError('generation escaped controlled home')
        metadata=json.loads((target/'metadata.json').read_text())
        if metadata['generation_id']!=gen or metadata['context_id']!=env['AIKIT_CONTEXT_ID']:raise ValueError('wrong current generation/context')
        all_bodies={}; hashes={}
        for provider,path in [('claude','.claude/skills'),('codex','.agents/skills')]:
            found=set(); base=target/'projections'/provider/path
            for f in base.glob('*/SKILL.md'):
                if not f.resolve().is_relative_to(root): raise ValueError('projection escaped controlled home')
                folder=f.parent.name
                if folder not in inputs:raise ValueError('unexpected Skill')
                data=f.read_bytes()
                if data!=inputs[folder]:raise ValueError(f'projected body changed {folder}')
                found.add(folder);hashes[f'{provider}/{folder}']=sha(data)
            if found!=expected: raise ValueError(f'{phase}/{provider}: expected {expected}, found {found}')
            all_bodies[provider]=sorted(found)
        methods=run('method','list');active={m['id'].rsplit('/',1)[-1] for m in methods['methods'] if m['active']}
        if active != expected & METHODS: raise ValueError('active Method discovery differs from actual projection')
        observations.append({'phase':phase,'generation':gen,'context':metadata['context_id'],'bodies':all_bodies,'sha256':hashes,'native_set':native_set,'active_methods':sorted(active)})
    inspect('enabled',set(MEMBERS))
    run('disable',records['ql-experience-prepare']['id'],'--scope','global');inspect('disconnected',set(MEMBERS)-{'ql-experience-prepare'})
    run('enable',records['ql-experience-prepare']['id'],'--scope','global');inspect('reconnected',set(MEMBERS))
    result={'schema':'ql.ux-skill-delivery-evidence/1','standing':'exercised-native-projection','binary_version':version,'binary_sha256':sha(binary.read_bytes()),'source_snapshot':sync['candidate_snapshot'],'sources':{k:sha(v) for k,v in inputs.items()},'set_members':ids,'observations':observations,'harness_loaded':False,'agent_used':False,'human_validated':False,'scope':'Disposable native source/catalogue/set/scope/current Claude and Codex projection; not harness loading or Epi runtime.'}
    (root/'result.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2));return 0
if __name__=='__main__':
    try:sys.exit(main())
    except (ValueError,OSError,subprocess.SubprocessError,KeyError) as exc:print(f'projection check failed: {exc}',file=sys.stderr);sys.exit(1)
