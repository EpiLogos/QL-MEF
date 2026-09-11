"""Record the exact inputs and outputs after the complete M3 acceptance command.

This program alone is not a test runner. test-m3-acceptance.sh calls it only
following successful source, compiler, runtime, observer and consumer checks.
"""
import hashlib,json,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
PATHS=[
 'c/Makefile','c/include/ql/m3.h','c/include/ql/m3_domain.h','c/src/m3.c','c/src/m3_domain.c',
 'crates/ql-core/src/m3_clock.rs','crates/ql-mef/src/m3_engine.rs','crates/ql-mef/src/m3_source.rs','crates/ql-mef/src/m3_state.rs',
 'crates/ql-mef/examples/m3-state.rs','crates/ql-mef/tests/m3_engine.rs','crates/ql-mef/tests/m3_domain.rs','crates/ql-mef/tests/m3_state.rs',
 'fixtures/kernel/m3-domain-v1.json','fixtures/kernel/m3-parent-consumer-v1.json','fixtures/kernel/m-tree-v1.json',
 'migration/epi-kernel/k7-m3-oracle.c','migration/epi-kernel/k7-m3-probe.c','migration/epi-kernel/k7-m3-domain-probe.c',
 'scripts/generate-m3.py','scripts/m3-domain.py','scripts/test-m3-engine.sh','scripts/test-m3-acceptance.sh','scripts/m3-receipt.py',
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
 receipt={'schema':'ql.m3-acceptance/v1','result':'passed','revision':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),
 'inputs':{p:sha(p) for p in PATHS},'outputs':{p:sha(p) for p in outputs},
 'checks':['18704-retained-native-C-fields-per-compiler','4938-C-Rust-finite-records','996-source-nodes-4891-qualified-relations','184-source-matrix-cells',
 '7552-fold-pose-aperture-forms-and-22656-matrix-outcomes','512-environment-rotations','1441-source-backbone-projections',
 '623-independent-coordinate-bindings-128-transcriptions-1441-clock-frames','C-GCC-Clang-ASan-UBSan','installed-C++17-Rust-packet-consumer',
 'parent-subject-generation-atomicity-stale-input-gap-rollback-replay','source-record-mutation-regressions'],
 'not_claimed':['blanket-source-semantic-equivalence','live-provider-freshness','continuous-C++-runtime','desktop-or-agent-experiential-acceptance','entity-quaternion-primary-address-inverse','unimplemented-source-research']}
 (ROOT/'target/m3-acceptance/acceptance.json').write_text(json.dumps(receipt,indent=2)+'\n')
 print('M3 acceptance receipt recorded against exact executed input hashes')
