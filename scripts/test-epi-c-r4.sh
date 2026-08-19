#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
REFERENCE_ROOT="$REPO_ROOT/vendor/epi-kernel/reference"
OUT_DIR="$REPO_ROOT/target/epi-c-r4"
CC_BIN=${CC:-cc}
SOURCE_REVISION=$(git -C "$REPO_ROOT" rev-parse HEAD 2>/dev/null || printf unversioned)

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

"$CC_BIN" \
  -std=c11 -Wall -Wextra -Werror -pedantic \
  -I"$REPO_ROOT/c/include" \
  -I"$REFERENCE_ROOT/include" \
  -DQL_C_SOURCE_REVISION='"'"$SOURCE_REVISION"'"' \
  "$REPO_ROOT/migration/epi-kernel/r4-holographic-kernel-parity.c" \
  "$REPO_ROOT/c/src/primitive.c" \
  "$REPO_ROOT/c/src/holographic.c" \
  "$REPO_ROOT/c/src/kernel.c" \
  "$REFERENCE_ROOT/src/kernel.c" \
  -lm \
  -o "$OUT_DIR/r4-parity"

"$OUT_DIR/r4-parity" | tee "$OUT_DIR/r4-parity.txt"

make -C "$REPO_ROOT/c" clean all SOURCE_REVISION="$SOURCE_REVISION"
make -C "$REPO_ROOT/c" install SOURCE_REVISION="$SOURCE_REVISION" DESTDIR="$OUT_DIR/install-root" PREFIX=/ql-mef-c

"$CC_BIN" \
  -std=c11 -Wall -Wextra -Werror -pedantic \
  -I"$OUT_DIR/install-root/ql-mef-c/include" \
  "$REPO_ROOT/migration/epi-kernel/r4-package-smoke.c" \
  "$OUT_DIR/install-root/ql-mef-c/lib/libql-mef-c.a" \
  -lm \
  -o "$OUT_DIR/package-smoke"

"$OUT_DIR/package-smoke" | tee "$OUT_DIR/package-smoke.txt"
grep -Fx "$SOURCE_REVISION" "$OUT_DIR/install-root/ql-mef-c/share/ql-mef-c/source-revision.txt"
grep -Fx "0.1.0" "$OUT_DIR/install-root/ql-mef-c/share/ql-mef-c/api-version.txt"

printf 'R4 native package/install seam: PASS\n'
