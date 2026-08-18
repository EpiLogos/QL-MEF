#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
REFERENCE_ROOT="$REPO_ROOT/vendor/epi-kernel/reference"
INCLUDE_ROOT="$REFERENCE_ROOT/include"
SRC_ROOT="$REFERENCE_ROOT/src"
OUT_DIR="$REPO_ROOT/target/epi-c-characterization"
CC_BIN=${CC:-cc}

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR/objects" "$OUT_DIR/logs" "$OUT_DIR/deps"

printf 'source\tcompile\n' > "$OUT_DIR/compile.tsv"

compile_failures=0
for source in "$SRC_ROOT"/*.c; do
  name=$(basename "$source")
  obj="$OUT_DIR/objects/${name%.c}.o"
  log="$OUT_DIR/logs/${name%.c}.log"
  dep="$OUT_DIR/deps/${name%.c}.d"

  if "$CC_BIN" -std=c11 -Wall -Wextra -I"$INCLUDE_ROOT" -M -MF "$dep" "$source" \
      && "$CC_BIN" -std=c11 -Wall -Wextra -I"$INCLUDE_ROOT" -c "$source" -o "$obj" >"$log" 2>&1; then
    printf '%s\tok\n' "$name" >> "$OUT_DIR/compile.tsv"
  else
    printf '%s\tfail\n' "$name" >> "$OUT_DIR/compile.tsv"
    compile_failures=$((compile_failures + 1))
  fi
done

# Attempt the broadest honest historical link. This uses the frozen historical
# main.c and every translation unit exactly as preserved. A failure is evidence
# about the reference corpus, not a reason to edit it.
link_status=not-attempted
if [[ "$compile_failures" -eq 0 ]]; then
  set +e
  "$CC_BIN" "$OUT_DIR"/objects/*.o -lm -o "$OUT_DIR/epi-reference" \
    >"$OUT_DIR/logs/link.log" 2>&1
  link_rc=$?
  set -e
  if [[ "$link_rc" -eq 0 ]]; then
    link_status=ok
    set +e
    "$OUT_DIR/epi-reference" --help >"$OUT_DIR/logs/run-help.log" 2>&1
    echo $? > "$OUT_DIR/run-help.exit"
    set -e
  else
    link_status=fail
  fi
fi
printf '%s\n' "$link_status" > "$OUT_DIR/link.status"

# Record undefined symbols per successfully compiled object. These are useful
# even when the aggregate link fails because they expose actual cross-TU edges.
: > "$OUT_DIR/undefined-symbols.tsv"
for obj in "$OUT_DIR"/objects/*.o; do
  [[ -e "$obj" ]] || continue
  base=$(basename "$obj" .o)
  nm -u "$obj" 2>/dev/null | awk -v source="$base.c" '{print source "\t" $NF}' \
done | sort -u > "$OUT_DIR/undefined-symbols.tsv"

python3 - "$OUT_DIR" <<'PY'
import json
from pathlib import Path
import sys

out = Path(sys.argv[1])
rows = []
for line in (out / "compile.tsv").read_text().splitlines()[1:]:
    source, status = line.split("\t", 1)
    dep_path = out / "deps" / (Path(source).stem + ".d")
    deps = dep_path.read_text().replace("\\\n", " ").split() if dep_path.exists() else []
    local_headers = sorted({Path(x).name for x in deps if x.endswith(".h")})
    rows.append({"source": source, "compile": status, "local_headers": local_headers})

undefined = {}
for line in (out / "undefined-symbols.tsv").read_text().splitlines():
    if not line.strip():
        continue
    source, symbol = line.split("\t", 1)
    undefined.setdefault(source, []).append(symbol)

report = {
    "schema": "ql-mef.epi-c-reference-characterization/v1",
    "reference_revision": "daa660cbc1b8c5da83828698665a753852cb0287",
    "compiler": "cc -std=c11 -Wall -Wextra",
    "translation_units": rows,
    "aggregate_link": (out / "link.status").read_text().strip(),
    "undefined_symbols": undefined,
    "interpretation": {
        "compile_failures": "frozen-source build facts; never repaired in vendor/epi-kernel/reference",
        "link_failure": "distinguish missing build glue/external dependency/duplicate symbol from source syntax failures using logs and symbol map"
    }
}
(out / "characterization.json").write_text(json.dumps(report, indent=2) + "\n")
PY

cat "$OUT_DIR/characterization.json"

# Characterization itself succeeds even when portions of the historical corpus
# do not compile/link. CI consumes the report; later parity jobs declare which
# failures are blockers for their own promoted scope.
exit 0
