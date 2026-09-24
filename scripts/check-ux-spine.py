#!/usr/bin/env python3
"""Read-only UX scope/practice conformance. A successful check proves no human experience."""
from __future__ import annotations
import argparse, copy, json, pathlib, re, sys, unittest
P=pathlib.Path
ROOT=P(__file__).resolve().parents[1]
DOC='docs/kernel-rebuild/'
METHODS={'ql-operation','bimba-cypher','refraction-adapter-authoring','ql-experience-prepare','ql-experience-walk'}
SKILLS=METHODS|{'ql-foundations','ql-evidence-report'}
TEXTS=['UX-INTENT-SOURCE-MINUTE.md','UX-SPINE-RECONCILIATION.md','AGENT-PRACTICE-AND-BOOTSTRAP.md']

def load(root: P) -> dict:return json.loads((root/DOC/'ux-spine-trace.json').read_text())
def require(value: bool, message: str):
    if not value:raise ValueError(message)
def description(text: str)->str:
    m=re.search(r'^description:\s*(.+)$',text,re.M);require(bool(m),'missing description')
    value=m.group(1).strip()
    return json.loads(value) if value.startswith('"') else value

def validate(data: dict, root: P, oi: P|None=None, override: dict|None=None)->dict:
    override=override or {}
    def read(path):return override.get(str(path),(root/path).read_text() if (root/path).is_file() else '')
    stories=data['stories'];ids={s['id'] for s in stories}
    require(len(stories)==12 and ids=={f'UX{i:02}' for i in range(1,13)},'story inventory')
    require({s['extension_state'] for s in stories}=={f'L{i}' for i in range(12)},'host extension inventory')
    practice_ids={p['id'] for p in data['practices']};require(len(practice_ids)==12,'practice inventory')
    all_criteria=set()
    for f,prefix in [('PARENT-SURFACES-INTEGRATION.md','A'),('PRE-K8-AGENT-WORLD-LOCK.md','B')]:
        all_criteria|=set(re.findall(r'^\|\s*('+prefix+r'\d{2})\s*\|',read(DOC+f),re.M))
    require(all_criteria=={f'A{i:02}' for i in range(1,19)}|{f'B{i:02}' for i in range(1,13)},'canonical A/B changed; reconcile scope explicitly')
    require({a for s in stories for a in s['acceptance']}==all_criteria,'missing or invented acceptance')
    minute=read(DOC+TEXTS[0]);walk=read(DOC+TEXTS[1]);practice=read(DOC+TEXTS[2])
    for story in stories:
        require(story['practices'] and set(story['practices'])<=practice_ids,'missing practice link')
        require(story['dependencies'],'missing native implementation dependency')
        require(story['intent'] and all(u in minute for u in story['intent']),'missing original intent')
        require('### '+story['id']+' ' in walk,'missing human story')
        require(story['status']=='specified' and story['human_assessment'] is None and not story['evidence'],'source index must not manufacture live/human receipt')
        require(story['host_states'],'missing host spine')
        if oi:
            host=(oi/'docs/cradle/03-UX-STATES.md').read_text()
            for state in story['host_states']+[story['extension_state']]:require(state in host,f'unknown host state {state}')
    for item in data['practices']:
        path=item.get('canonical_skill')
        if path:require(read(path).startswith('---\n'),'missing canonical practice source')
        else:require(item.get('gap_owner') or item.get('native_owners') or item.get('dependencies') or item.get('support'),'unbound practice gap')
    source_rows=data['source_position_coverage'];sp={x['source_record'] for x in source_rows}
    require(len(source_rows)==36 and sp=={f'SP{i}{j}' for i in range(6) for j in range(6)},'missing S′ source position')
    for row in source_rows:
        require(set(row['stories'])<=ids and bool(row['stories']),'SP story link')
        require(set(row['practices'])<=practice_ids and bool(row['practices']),'SP practice link')
    native_caps=set()
    for i in range(6):
        path=f'docs/integrations/epi-logos/epi-m-capability-field-m{i}.json'
        native_caps|={c['capability_ref'] for c in json.loads(read(path))['capabilities']}
    rows=data['m_capability_coverage']
    require(len(rows)==len(native_caps) and {r['capability_ref'] for r in rows}==native_caps,'M capability coverage lost')
    for row in rows:require(row['stories'] and set(row['stories'])<=ids,'M story link')
    require(len(data['full_field_rules'])==9,'full-field qualification lost')
    for folder in SKILLS:
        path=f'skills/{folder}/SKILL.md';text=read(path)
        require(text.startswith('---\n') and re.search(r'^name:',text,re.M),f'bad Skill {folder}')
        desc=description(text);require(desc.startswith('METHOD:')==(folder in METHODS),f'wrong Method classification {folder}')
        if folder in {'ql-experience-prepare','ql-experience-walk','ql-evidence-report','bimba-cypher'}:
            for token in ['## Inputs','## Outputs','## Verification','## Authority','## Continuity']:require(token in text,f'{folder} missing {token}')
    for token in ['source','discovery','projection','harness','METHOD:','Central','consum','B0','B1','B2']:require(token.lower() in practice.lower(),f'practice loses {token}')
    for token in ['0→1','109','18','Oikonomia','T/T′','METHOD:']:
        require(any(token in read(DOC+t) for t in TEXTS),f'authorial specificity lost: {token}')
    links=0
    paths=[DOC+t for t in TEXTS]+[f'skills/{s}/SKILL.md' for s in SKILLS]
    for path in paths:
        for raw in re.findall(r'\]\(([^)]+)\)',read(path)):
            if '://' in raw or raw.startswith('#') or raw.startswith('mailto:'):continue
            target=raw.split('#')[0]
            if target:
                require((root/path).parent.joinpath(target).exists(),f'broken link {path}: {raw}');links+=1
    return {'stories':12,'acceptance_families':len(all_criteria),'source_positions':len(sp),'M_capabilities':len(native_caps),'practices':12,'skills':len(SKILLS),'methods':len(METHODS),'local_links':links,'scope':'source coverage only; no runtime/harness/human acceptance'}

def main()->int:
    ap=argparse.ArgumentParser(description=__doc__);ap.add_argument('--root',type=P,default=ROOT);ap.add_argument('--oi-root',type=P);ap.add_argument('--prepare',choices=[f'UX{i:02}' for i in range(1,13)]);ap.add_argument('--self-test',action='store_true');args=ap.parse_args();data=load(args.root)
    result=validate(data,args.root,args.oi_root)
    if args.self_test:
        class Mutations(unittest.TestCase):
            def reject(self,fn):
                d=copy.deepcopy(data);fn(d)
                with self.assertRaises(ValueError):validate(d,args.root,args.oi_root)
            def test_story(self):self.reject(lambda d:d['stories'].pop())
            def test_A(self):self.reject(lambda d:[s.update(acceptance=[a for a in s['acceptance'] if a!='A18']) for s in d['stories']])
            def test_B(self):self.reject(lambda d:[s.update(acceptance=[a for a in s['acceptance'] if a!='B11']) for s in d['stories']])
            def test_SP(self):self.reject(lambda d:d['source_position_coverage'].pop())
            def test_M(self):self.reject(lambda d:d['m_capability_coverage'].pop())
            def test_human(self):self.reject(lambda d:d['stories'][0].update(status='human-validated',human_assessment='invented'))
            def test_fake_evidence(self):self.reject(lambda d:d['stories'][0].update(evidence=['green CI']))
            def test_dependencies(self):self.reject(lambda d:d['stories'][0].update(dependencies=[]))
            def test_practice(self):self.reject(lambda d:d['stories'][0].update(practices=['invented']))
            def test_full_field(self):self.reject(lambda d:d['full_field_rules'].pop('Vak109'))
            def test_method(self):
                path='skills/ql-experience-walk/SKILL.md';text=(args.root/path).read_text().replace('description: "METHOD:', 'description: "')
                with self.assertRaises(ValueError):validate(data,args.root,args.oi_root,{path:text})
            def test_broken_link(self):
                path=DOC+TEXTS[1]
                with self.assertRaises(ValueError):validate(data,args.root,args.oi_root,{path:(args.root/path).read_text()+'\n[bad](absent-file.md)'})
        checked=unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(Mutations));require(checked.wasSuccessful(),'mutation tests failed');result['mutations']=checked.testsRun
    if args.prepare:
        story=next(s for s in data['stories'] if s['id']==args.prepare)
        result={'standing':'specified','readiness':'not-assessed','story':story,'practices':[p for p in data['practices'] if p['id'] in story['practices']],'required_readiness':['exact versions and current source','root/child World, actor, subject and NOW','real provider and Action contracts','Skill discovered/current projection/actual harness loading','data disclosure and mutation authority','failure, interruption and recovery path','human assessment pending until actual use'],'side_effects':'none','instructions':'Read the complete human story and source minute. Native dependencies must be observed before inviting the person. This output is not a passed preflight.'}
    print(json.dumps(result,indent=2,ensure_ascii=False));return 0
if __name__=='__main__':
    try:sys.exit(main())
    except (ValueError,KeyError,OSError,json.JSONDecodeError) as exc:print(f'UX conformance failed: {exc}',file=sys.stderr);sys.exit(1)
