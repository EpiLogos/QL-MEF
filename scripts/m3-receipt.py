"""Record the exact inputs and outputs after the complete M3 acceptance command.

This program alone is not a test runner. test-m3-acceptance.sh calls it only
following successful source, compiler, runtime, observer and consumer checks.
"""
from collections import Counter
import hashlib,json,os,platform,re,shutil,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
PATHS=[
 'c/Makefile','c/include/ql/m3.h','c/include/ql/m3_domain.h','c/src/m3.c','c/src/m3_domain.c',
 'crates/ql-core/src/m3_clock.rs','crates/ql-mef/src/m3_engine.rs','crates/ql-mef/src/m3_source.rs','crates/ql-mef/src/m3_state.rs',
 'crates/ql-mef/examples/m3-state.rs','crates/ql-mef/tests/m3_engine.rs','crates/ql-mef/tests/m3_domain.rs','crates/ql-mef/tests/m3_state.rs',
 'fixtures/kernel/m3-domain-v1.json','fixtures/kernel/m3-parent-consumer-v1.json','fixtures/kernel/m3-parent-consumer-current-v1.json','fixtures/kernel/m3-parent-consumer-current-v1.basis.json','fixtures/kernel/m-tree-v1.json','fixtures/kernel/bimba-content-v1.json',
 'migration/epi-kernel/k7-m3-oracle.c','migration/epi-kernel/k7-m3-probe.c','migration/epi-kernel/k7-m3-domain-probe.c',
 'scripts/generate-m3.py','scripts/m3-domain.py','scripts/refresh-m3-parent-consumer.py','scripts/test-m3-engine.sh','scripts/test-m3-acceptance.sh','scripts/m3-receipt.py',
 'scripts/m3-source-parity.py','scripts/m3-observation-parity.py','scripts/tests/test_m3_engine.py','scripts/tests/test_m3_domain.py',
 'vendor/epi-kernel/reference/src/m3.c','vendor/epi-kernel/reference/src/m3_clock_lut.c','vendor/epi-kernel/reference/include/m3.h',
]
PATHS+=sorted(str(p.relative_to(ROOT)) for p in (ROOT/'scripts/tests').glob('test_m3*.py'))
PATHS+=sorted(str(p.relative_to(ROOT)) for p in (ROOT/'crates/ql-core/src/pole').glob('*.rs'))
def sha(path):return hashlib.sha256((ROOT/path).read_bytes()).hexdigest()
if __name__=='__main__':
 observation=json.loads((ROOT/'target/m3-acceptance/source-observation.json').read_text())
 assert observation['status']=='passed' and not observation['differences']
 outputs=['target/m3-rust/c-rust-parity.jsonl','target/m3-domain/c-rust.jsonl','target/m3-parent-consumer/handoff.json','target/m3-acceptance/replay.json']
 original=json.loads((ROOT/'fixtures/kernel/bimba-content-v1.json').read_text())
 content=original['content']
 canonical=json.dumps(content,ensure_ascii=False,sort_keys=True,separators=(',',':')).encode()
 assert hashlib.sha256(canonical).hexdigest()==original['source_revision']
 source_nodes={ref for ref in content['nodes'] if re.fullmatch(r'M3(?:-.*)?',ref)}
 source_edges=[edge for edge in content['relations'] if edge[0] in source_nodes]
 registry=json.loads((ROOT/'fixtures/kernel/m-tree-v1.json').read_text())
 assert registry['source_revision']==original['source_revision']==observation['bimba_source_revision']
 counts={path:dict(Counter(json.loads(line)[0] for line in (ROOT/path).read_text().splitlines() if line.strip())) for path in outputs[:2]}
 assert counts[outputs[1]]['source-node']==len(source_nodes)
 assert counts[outputs[1]]['source-edge']==len(source_edges)
 compiler_identity={}
 for command in ['cc','clang','c++','rustc']:
  executable=shutil.which(command)
  assert executable, 'missing executed compiler: '+command
  version=subprocess.check_output([executable,'--version'],text=True).strip()
  compiler_identity[command]={'path':executable,'version':version,'sha256':hashlib.sha256(Path(executable).read_bytes()).hexdigest()}
 oracle={}
 for compiler in ['cc','clang']:
  text=(ROOT/f'target/m3-native/{compiler}-oracle.txt').read_text()
  count=re.fullmatch(r'executed retained-C/native-C parity: (\d+) field comparisons\s*',text)
  assert count, 'missing actual retained-C oracle count: '+compiler
  oracle[compiler]=int(count[1])
 receipt={'schema':'ql.m3-acceptance/v1','result':'passed','revision':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),
 'inputs':{p:sha(p) for p in PATHS},'outputs':{p:sha(p) for p in outputs},
 'checks':['retained-native-C-fields-per-executed-compiler-alias','C-Rust-finite-records',f'{len(source_nodes)}-source-nodes-{len(source_edges)}-qualified-relations','source-matrix-cells',
 'fold-pose-aperture-forms-and-matrix-outcomes','environment-rotations','source-backbone-projections',
 'independent-coordinate-bindings-transcriptions-clock-frames','executed-compiler-identities-ASan-UBSan','installed-C++17-Rust-packet-consumer',
 'parent-subject-generation-atomicity-stale-input-gap-rollback-replay','source-record-mutation-regressions'],
 'current_source':{'fixture':'fixtures/kernel/bimba-content-v1.json','source_revision':original['source_revision'],'fixture_sha256':sha('fixtures/kernel/bimba-content-v1.json'),'nodes':len(source_nodes),'qualified_relations':len(source_edges),'line_change_edges':sum(edge[1]=='LINE_CHANGE' for edge in source_edges),'scope':'Retained admitted full Bimba source replay; no new live graph read.'},
 'observed_record_counts':counts,'retained_native_C_field_comparisons':oracle,
 'execution':{'command_owner':'scripts/test-m3-acceptance.sh','command_owner_sha256':sha('scripts/test-m3-acceptance.sh'),'platform':platform.platform(),'machine':platform.machine(),'compiler_identity':compiler_identity,'CC':os.environ.get('CC'),'CXX':os.environ.get('CXX'),'compiler_aliases_are_distinct_versions':compiler_identity['cc']['version']!=compiler_identity['clang']['version'],'address_and_undefined_sanitizers':True,'leak_sanitizer':platform.system()!='Darwin'},
 'not_claimed':['blanket-source-semantic-equivalence','live-provider-freshness','continuous-C++-runtime','desktop-or-agent-experiential-acceptance','entity-quaternion-primary-address-inverse','unimplemented-source-research']}
 (ROOT/'target/m3-acceptance/acceptance.json').write_text(json.dumps(receipt,indent=2)+'\n')
 print('M3 acceptance receipt recorded against exact executed input hashes')
