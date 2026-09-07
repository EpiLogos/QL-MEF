#!/usr/bin/env bash
set -euo pipefail

# Synchronize or verify the frozen Epi C reference corpus.
#
# Usage:
#   scripts/sync-epi-c-reference.sh /path/to/Epi-Logos-C-Experiments
#   scripts/sync-epi-c-reference.sh --check /path/to/Epi-Logos-C-Experiments
#
# The source is read from the locked commit, not from the checkout's working tree.

MODE=sync
if [[ "${1:-}" == "--check" ]]; then
  MODE=check
  shift
fi

if [[ $# -ne 1 ]]; then
  echo "usage: $0 [--check] /path/to/Epi-Logos-C-Experiments" >&2
  exit 64
fi

EPI_REPO=$1
SOURCE_REV=daa660cbc1b8c5da83828698665a753852cb0287
SOURCE_ROOT=Body/S/S0/epi-lib
INCLUDE_TREE=f2b27d99197ee0f1cb9ed95ef52a5dd61a226e54
SRC_TREE=a60dcda1427a6ab3cfcd44565a29f988938d0881
TEST_TREE=9a6ef6505bb4e7622dba0922a07dced9bc49cd79
REPO_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
REFERENCE_ROOT="$REPO_ROOT/vendor/epi-kernel/reference"

if ! git -C "$EPI_REPO" cat-file -e "$SOURCE_REV^{commit}" 2>/dev/null; then
  echo "locked Epi revision is not available in $EPI_REPO: $SOURCE_REV" >&2
  exit 65
fi

actual_include=$(git -C "$EPI_REPO" rev-parse "$SOURCE_REV:$SOURCE_ROOT/include")
actual_src=$(git -C "$EPI_REPO" rev-parse "$SOURCE_REV:$SOURCE_ROOT/src")
actual_test=$(git -C "$EPI_REPO" rev-parse "$SOURCE_REV:$SOURCE_ROOT/test")

[[ "$actual_include" == "$INCLUDE_TREE" ]] || { echo "include tree lock mismatch" >&2; exit 66; }
[[ "$actual_src" == "$SRC_TREE" ]] || { echo "src tree lock mismatch" >&2; exit 66; }
[[ "$actual_test" == "$TEST_TREE" ]] || { echo "test tree lock mismatch" >&2; exit 66; }

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

git -C "$EPI_REPO" archive "$SOURCE_REV" \
  "$SOURCE_ROOT/include" \
  "$SOURCE_ROOT/src" | tar -x -C "$tmp"

frozen="$tmp/$SOURCE_ROOT"

# Registered ratified corrections: content-pinned patch sets applied on top
# of the frozen reference (schema ql-mef.epi-c-kernel-source-lock/v1,
# ratified_corrections). The guarantee becomes: vendored == frozen o patches,
# so the freeze still admits no silent drift.
CORRECTIONS_ROOT="$REPO_ROOT/vendor/epi-kernel/corrections"
CORRECTION_PATCHES=$(python3 - "$REPO_ROOT/migration/epi-kernel/source-lock.json" <<-'PYEOF'
import json, sys
lock = json.load(open(sys.argv[1]))
for correction in lock.get("ratified_corrections", []):
    print(correction["patch"])
PYEOF
)

apply_corrections() {
    # Applies every registered patch with paths resolving from the repo root.
    while IFS= read -r patch_path; do
        [[ -z "$patch_path" ]] && continue
        git -C "$REPO_ROOT" apply "$REPO_ROOT/$patch_path"
    done <<< "$CORRECTION_PATCHES"
}

if [[ "$MODE" == "check" ]]; then
    if [[ -n "$CORRECTION_PATCHES" ]]; then
        # Rebuild the vendored tree from frozen + patches in a scratch
        # checkout and require byte-identity with what is actually vendored.
        apply_root="$REPO_ROOT/target/corrections-check/vendor/epi-kernel/reference"
        rm -rf "$REPO_ROOT/target/corrections-check"
        mkdir -p "$apply_root"
        cp -R "$frozen/include" "$frozen/src" "$apply_root/"
        apply_corrections
        diff -ru -- "$apply_root/include" "$REFERENCE_ROOT/include"
        diff -ru -- "$apply_root/src" "$REFERENCE_ROOT/src"
        rm -rf "$REPO_ROOT/target/corrections-check"
    else
        diff -ru -- "$frozen/include" "$REFERENCE_ROOT/include"
        diff -ru -- "$frozen/src" "$REFERENCE_ROOT/src"
    fi
    echo "Epi C reference matches $SOURCE_REV o ratified corrections"
    echo "historical test tree locked (not bulk-vendored): $TEST_TREE"
    exit 0
fi

mkdir -p "$REFERENCE_ROOT"
rm -rf "$REFERENCE_ROOT/include" "$REFERENCE_ROOT/src"
cp -R "$frozen/include" "$REFERENCE_ROOT/include"
cp -R "$frozen/src" "$REFERENCE_ROOT/src"
if [[ -n "$CORRECTION_PATCHES" ]]; then
    apply_corrections
fi
echo "synchronized Epi C reference from $SOURCE_REV"
echo "include tree: $INCLUDE_TREE"
echo "src tree:     $SRC_TREE"
echo "test tree:    $TEST_TREE (locked; selected source tests enter R1 deliberately)"
