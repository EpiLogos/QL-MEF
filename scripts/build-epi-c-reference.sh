#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
REFERENCE_ROOT="$REPO_ROOT/vendor/epi-kernel/reference"
OUT_DIR="$REPO_ROOT/target/epi-c-reference"
CC_BIN=${CC:-cc}

mkdir -p "$OUT_DIR"

"$CC_BIN" \
  -std=c11 \
  -Wall \
  -Wextra \
  -I"$REFERENCE_ROOT/include" \
  "$REPO_ROOT/migration/epi-kernel/reference-smoke.c" \
  "$REFERENCE_ROOT/src/kernel.c" \
  -lm \
  -o "$OUT_DIR/kernel-smoke"

"$OUT_DIR/kernel-smoke"
