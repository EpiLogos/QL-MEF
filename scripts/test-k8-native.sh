#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
out="${K8_OUT:-target/k8-native}"
mkdir -p "$out"
python3 scripts/k8-structure.py --check --out "$out/generated"
python3 -m unittest discover -s scripts/tests -p test_k8_structure.py -v 2>&1 | tee "$out/structure-tests.log"
make -C c all
cc -std=c11 -Wall -Wextra -Werror -pedantic -Ic/include migration/epi-kernel/k8/native.c c/build/libql-mef-c.a -lm -o "$out/native"
"$out/native" | tee "$out/native-observation.json"
make -C c BUILD_DIR="$(realpath "$out")/sanitized" CFLAGS='-std=c11 -O1 -g -Wall -Wextra -Werror -pedantic -fsanitize=address,undefined -fno-omit-frame-pointer' all
cc -std=c11 -O1 -g -Wall -Wextra -Werror -pedantic -fsanitize=address,undefined -fno-omit-frame-pointer -Ic/include migration/epi-kernel/k8/native.c "$out/sanitized/libql-mef-c.a" -lm -o "$out/native-sanitized"
ASAN_OPTIONS=detect_leaks=1:abort_on_error=1 UBSAN_OPTIONS=halt_on_error=1 "$out/native-sanitized" | tee "$out/sanitized-observation.json"
make -C c install DESTDIR="$(realpath "$out")/installed" PREFIX=/ql-mef-c
c++ -std=c++17 -Wall -Wextra -Werror -pedantic -I"$out/installed/ql-mef-c/include" cpp/tests/k8_installed.cpp "$out/installed/ql-mef-c/lib/libql-mef-c.a" -lm -o "$out/installed-consumer"
"$out/installed-consumer" | tee "$out/installed-consumer.log"
cmp fixtures/kernel/m-tree-v1.json "$out/installed/ql-mef-c/share/ql-mef-c/m-tree-v1.json"
cmp "$out/generated/m-tree-v2.json" "$out/installed/ql-mef-c/share/ql-mef-c/m-tree-v2.json"
python3 scripts/m-ledger.py check 2>&1 | tee "$out/accepted-ledger.log"
printf 'K8 structural/native/installed acceptance passed; live graph and desktop NOT CLAIMED\n'
