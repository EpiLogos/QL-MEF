#!/usr/bin/env bash
# Exact-source finite M2 acceptance. Never treat missing checks as a pass.
set -euo pipefail
cd "$(dirname "$0")/.."
receipt=target/m2-receipt
mkdir -p "$receipt"
test "$(git hash-object fixtures/kernel/m2-reference-vimarsha.rs)" = 7b0bc34cee3388a8d22a615ad2c9facc28016e30
git rev-parse HEAD > "$receipt/source-revision.txt"
python3 scripts/m2-catalogue.py check --receipt "$receipt"
if [[ -n "${M2_BIMBA_SOURCE:-}" ]]; then
  python3 scripts/m2-field-report.py check --source-root "$M2_BIMBA_SOURCE"
elif [[ -d target/m2-bimba-source/Idea/Bimba/Map/datasets/parashakti-deep ]]; then
  python3 scripts/m2-field-report.py check --source-root target/m2-bimba-source
else
  echo 'Pinned deep Bimba source required: set M2_BIMBA_SOURCE to its read-only checkout' >&2
  exit 1
fi
make -C c all
"${CC:-cc}" -std=c11 -O2 -Wall -Wextra -Werror -pedantic -Ic/include \
  scripts/m2-native-probe.c c/build/libql-mef-c.a -lm -o "$receipt/native-probe"
"$receipt/native-probe" > "$receipt/native.tsv"
python3 - <<'PY'
from pathlib import Path
p=Path('target/m2-receipt')
a=(p/'source.tsv').read_text()
b=''.join(line.removeprefix('record\t')+'\n' for line in (p/'native.tsv').read_text().splitlines() if line.startswith('record\t'))
assert a==b, 'native C descriptors differ from independently executed frozen C source'
PY
"${CC:-cc}" -std=c11 -O1 -g -Wall -Wextra -Werror -pedantic -Ic/include \
  -fsanitize=address,undefined -fno-omit-frame-pointer \
  scripts/m2-native-probe.c c/src/m2.c c/src/m_tree.c -lm -o "$receipt/sanitized-probe"
ASAN_OPTIONS=detect_leaks=1 UBSAN_OPTIONS=halt_on_error=1 "$receipt/sanitized-probe" > "$receipt/sanitized.tsv"
cmp "$receipt/native.tsv" "$receipt/sanitized.tsv"
cargo test -p ql-mef --test m2_engine --test m2_vimarsha --locked -- --nocapture 2>&1 | tee "$receipt/m2-tests.log"
cargo run -q -p ql-mef --example m2_engine --locked -- --fixture > "$receipt/engine-frame.json"
python3 scripts/m2-contract.py check --frame "$receipt/engine-frame.json"
# Consumers need only installed headers/staticlib; C++ must not need C sources.
prefix=$(mktemp -d)
trap 'rm -rf "$prefix"' EXIT
make -C c install PREFIX="$prefix"
printf '#include <ql/m2.h>\n#include <cassert>\nint main(){assert(ql_m2_table(QL_M2_MEF_TABLE)->row_count==72);return 0;}\n' > "$receipt/consumer.cpp"
"${CXX:-c++}" -std=c++17 -Wall -Wextra -Werror -I"$prefix/include" "$receipt/consumer.cpp" \
  -L"$prefix/lib" -lql-mef-c -lm -o "$receipt/installed-cpp-consumer"
"$receipt/installed-cpp-consumer"
python3 - <<'PY'
import hashlib,json,subprocess
from pathlib import Path
p=Path('target/m2-receipt')
paths=['c/include/ql/m2.h','c/src/m2.c','c/src/m2_data.inc','crates/ql-mef/src/m2.rs','crates/ql-mef/src/m2_engine.rs',
 'crates/ql-mef/tests/m2_engine.rs','fixtures/kernel/m2-retained-c-v1.json','fixtures/kernel/m2-field-census-summary-v1.json',
 'crates/ql-mef/src/m2_vimarsha.rs','crates/ql-mef/tests/m2_vimarsha.rs','fixtures/kernel/m2-reference-vimarsha.rs','scripts/m2-vimarsha-probe.c',
 'scripts/m2-contract.py','fixtures/kernel/m2-engine-request-v1.schema.json','fixtures/kernel/m2-engine-frame-v1.schema.json',
 'scripts/m2-native-probe.c','scripts/m2-source-probe.c','scripts/test-m2-engine.sh']
receipt={'schema':'ql.m2-acceptance/v1','result':'passed','revision':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),
 'inputs':{path:hashlib.sha256(Path(path).read_bytes()).hexdigest() for path in paths},
 'checks':['independent-retained-C-source','597-coordinate/8876-relation C-Rust observation','finite-operations','ASan+UBSan',
 'source-Vimarsha-32256-cases','closed-wire-shapes','strict-installed-C++17-consumer','same-event/bounded/provider contract','K2-pinned-deep-source-census'],
 'not_claimed':['K4 whole-programme acceptance','semantic equality of contradictory source declarations','live Neo4j or ephemeris','physical resonator solver','experiential validation']}
(p/'acceptance.json').write_text(json.dumps(receipt,indent=2)+'\n')
print('M2 source/C/Rust/installed-consumer scoped acceptance passed')
PY
