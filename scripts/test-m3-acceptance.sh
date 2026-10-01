#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
# With target/bimba-map/map.json (scripts/bimba_map.py read) the M3 field is
# recomputed from the map; otherwise from the committed m3-domain projection.
mkdir -p target/m3-acceptance
python3 scripts/m3-source-parity.py
if [[ -f target/bimba-map/map.json ]]; then python3 scripts/m3-domain.py --map target/bimba-map/map.json; fi
python3 scripts/m3-domain.py
python3 scripts/refresh-m3-parent-consumer.py --check
python3 -m unittest discover -s scripts/tests -p 'test_m3*.py' -v 2>&1 | tee target/m3-acceptance/python.log
bash scripts/test-m3-engine.sh 2>&1 | tee target/m3-acceptance/native.log
cargo test -p ql-mef --test m3_engine --test m3_domain --test m3_state --locked 2>&1 | tee target/m3-acceptance/rust.log
producer=$(git rev-parse HEAD)
printf '%s\n' "$producer" > target/m3-acceptance/producer.txt
python3 scripts/m3-observation-parity.py \
 --input target/m3-rust/c-rust-parity.jsonl --producer-revision-file target/m3-acceptance/producer.txt \
 --expected-revision "$producer" --output target/m3-acceptance/source-observation.json
python3 scripts/m3-domain.py --c-output target/m3-domain/m3_domain_data.inc
for compiler in cc clang; do
 "$compiler" -std=c11 -O1 -Wall -Wextra -Werror -pedantic -Ic/include -Itarget/m3-domain \
 migration/epi-kernel/k7-m3-domain-probe.c c/src/m3.c c/src/m3_domain.c c/src/m_tree.c -lm -o "target/m3-domain/$compiler"
 "target/m3-domain/$compiler" > "target/m3-domain/$compiler.jsonl"
done
cmp target/m3-domain/cc.jsonl target/m3-domain/clang.jsonl
cmp target/m3-domain/cc.jsonl target/m3-domain/c-rust.jsonl
clang -std=c11 -O1 -g -fsanitize=address,undefined -fno-omit-frame-pointer \
 -Ic/include -Itarget/m3-domain migration/epi-kernel/k7-m3-domain-probe.c \
 c/src/m3.c c/src/m3_domain.c c/src/m_tree.c -lm -o target/m3-domain/sanitized
ASAN_OPTIONS=detect_leaks=1 UBSAN_OPTIONS=halt_on_error=1 target/m3-domain/sanitized > target/m3-domain/sanitized.jsonl
cmp target/m3-domain/cc.jsonl target/m3-domain/sanitized.jsonl
cargo run -q -p ql-mef --example m3-state --locked < fixtures/kernel/m3-parent-consumer-current-v1.json > target/m3-acceptance/replay.json
python3 scripts/m3-receipt.py
