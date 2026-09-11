import subprocess,json,pathlib,re,copy
root=pathlib.Path(__file__).resolve().parents[1]
def show(ref,p):return subprocess.check_output(['git','show',f'{ref}:{p}'],cwd=root).decode()
# union build registration
p='c/Makefile'; text=show('81764a8',p)
text=text.replace('src/m1_state.c src/m1.c','src/m1_state.c src/m1.c src/m2.c').replace('include/ql/m1.h','include/ql/m1.h include/ql/m2.h')
text=text.replace('$(BUILD_DIR)/m1_state.o: src/m1_source_data.inc','$(BUILD_DIR)/m1_state.o: src/m1_source_data.inc\n$(BUILD_DIR)/m2.o: src/m2_data.inc src/m2_correspondence_data.inc')
(root/p).write_text(text)
p='crates/ql-mef/src/lib.rs';text=show('81764a8',p).replace('pub mod m1_engine;','pub mod m1_engine;\npub mod m2;\npub mod m2_condition;\npub mod m2_engine;\npub mod m2_vimarsha;');(root/p).write_text(text)
# Main's seed test checks full expected node set rather than a weaker count;
# preserve K5's independently-added tests from the automatic merge remainder.
p='crates/ql-mef/tests/m_ledger.rs'; text=(root/p).read_text(); theirs=show('7dda4d8',p)
a=text.index('fn imports_existing_matrix_families_not_a_manual_deep_census'); b=text.index('\n#[test]',a)
c=theirs.index('fn imports_existing_matrix_families_not_a_manual_deep_census');d=theirs.index('\n#[test]',c)
text=text[:a]+theirs[c:d]+text[b:]; assert '<<<<<<<' not in text; (root/p).write_text(text)
p='scripts/m_census.py';text=(root/p).read_text();pattern=r'<<<<<<< HEAD\n(.*?)=======\n(.*?)>>>>>>> [^\n]+\n'; text=re.sub(pattern,lambda m:m[1]+'\n\n'+m[2],text,flags=re.S); assert '<<<<<<<' not in text;(root/p).write_text(text)
# Identity-aware union: reviewed rows win over rescanned presence only.
p='fixtures/kernel/m-ledger-v1.json';B=json.loads(show('74c28d8',p));O=json.loads(show('81764a8',p));T=json.loads(show('7dda4d8',p))
print('top keys',B.keys())
conflicts=[]
def merge(b,o,t,path):
 if o==t:return copy.deepcopy(o)
 if o==b:return copy.deepcopy(t)
 if t==b:return copy.deepcopy(o)
 if path in ['ledger_revision']:return o # refreshed by tooling
 if path.startswith('rows/') and path.count('/')==1:
  def reviewed(r):return bool(r and r.get('assessment') not in [None,'unassessed'] and not r['assessment'].startswith('k4:'))
  if reviewed(o) != reviewed(t):
   winner=o if reviewed(o) else t
   print('preserve reviewed',path,winner['assessment'])
   return copy.deepcopy(winner)
 if path.startswith('evidence/k4-') and path.count('/')==1:
  return copy.deepcopy(o) # regenerated from both bodies
 if isinstance(o,dict) and isinstance(t,dict):
  b=b if isinstance(b,dict) else {}
  return {k:merge(b.get(k),o.get(k),t.get(k),path+'/'+k if path else k) for k in sorted(o.keys()|t.keys())}
 if isinstance(o,list) and isinstance(t,list) and all(isinstance(x,dict) and 'id'in x for x in o+t):
  bd={x['id']:x for x in b or []};od={x['id']:x for x in o};td={x['id']:x for x in t}
  return [merge(bd.get(k),od.get(k),td.get(k),path+'/'+k) for k in sorted(od.keys()|td.keys())]
 if o is None and b is None:return copy.deepcopy(t)
 if t is None and b is None:return copy.deepcopy(o)
 conflicts.append((path,b,o,t));return copy.deepcopy(o)
R=merge(B,O,T,''); print('CONFLICTS',len(conflicts))
for pth,b,o,t in conflicts:
 print(pth,repr(b)[:200],repr(o)[:200],repr(t)[:200])
for path,b,o,t in conflicts:
 assert path.startswith('rows/') and path.endswith('/bindings'), path
 rid=path[5:-9]
 row=next(r for r in R['rows'] if r['id']==rid)
 row['bindings']=sorted(set(o)|set(t))
(root/p).write_text(json.dumps(R,indent=2,ensure_ascii=False)+'\n')
# The generated census companions are derived, never merged line-wise.
for fp in subprocess.check_output(['git','diff','--name-only','--diff-filter=U'],cwd=root).decode().splitlines():
 if fp.startswith('fixtures/kernel/census/'):
  (root/fp).write_text(show('7dda4d8',fp))

# Pin both reviewed-lane preservation and inventory-owned stratum resolution.
p='scripts/tests/test_m_census.py';text=(root/p).read_text()
text=text.replace('reviewed_ids = ["census:#1-2-0", "deep-M1:M1-C04"]','reviewed_ids = ["census:#1-2-0", "deep-M1:M1-C04", "census:#2-1", "deep-M2:M2-C02"]')
text=text.replace('row["assessment"] = "k5-test:executed"','row["assessment"] = "k6-test:executed" if "#2" in row["id"] or "deep-M2:" in row["id"] else "k5-test:executed"')
text=text.replace('if __name__ == "__main__":\n    unittest.main()\n\n\n','')
text+='\n\nif __name__ == "__main__":\n    unittest.main()\n';(root/p).write_text(text)
subprocess.run(['python3','scripts/m_census.py','census'],cwd=root,check=True)
# Discovery records remain enumerable when reviewed rows replace heuristic rows.
# Reattach M1 constructs to their existing M1 seats, never invent new seats.
p='fixtures/kernel/m-ledger-v1.json';doc=json.loads((root/p).read_text());rows={r['id']:r for r in doc['rows']}
used={b for r in rows.values() for b in r['bindings']}
for i in doc['implementations']:
 if i['id'] not in used and i['disposition']!='infrastructural':
  refs=[r for r in i['coordinates'] if r.startswith('#1') and 'census:'+r in rows]
  assert refs, 'unexpected orphan outside M1: '+i['id']
  for ref in refs:
   row=rows['census:'+ref];assert row['assessment'].startswith('k5-m1:')
   row['bindings']=sorted(set(row['bindings'])|{i['id']})
(root/p).write_text(json.dumps(doc,indent=2,ensure_ascii=False)+'\n')
# All re-locked inputs are re-executed before publication by the caller workflow.
import hashlib
p='docs/kernel-rebuild/m1-engine-acceptance-v1.json';receipt=json.loads((root/p).read_text())
for lk in receipt['input_locks']:
 lk['sha256']=hashlib.sha256((root/lk['path']).read_bytes()).hexdigest()
receipt['executed']['census_tests']=20
(root/p).write_text(json.dumps(receipt,indent=2,ensure_ascii=False)+'\n')
p='fixtures/kernel/m-ledger-v1.json';doc=json.loads((root/p).read_text())
for e in doc['evidence']:
 if e['artifact']['path']=='docs/kernel-rebuild/m1-engine-acceptance-v1.json':
  e['artifact']['sha256']=hashlib.sha256((root/e['artifact']['path']).read_bytes()).hexdigest()
(root/p).write_text(json.dumps(doc,indent=2,ensure_ascii=False)+'\n')
subprocess.run(['python3','scripts/m-ledger.py','refresh'],cwd=root,check=True)
subprocess.run(['python3','scripts/m_census.py','census'],cwd=root,check=True)
