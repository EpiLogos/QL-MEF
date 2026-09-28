#!/usr/bin/env bash
# Rebuild everything QL derives from the Bimba map, in dependency order.
#
#   scripts/refresh-from-bimba-map.sh [--endpoint URL]   (or QL_BIMBA_MAP=URL)
#
# Reads the map read-only (scripts/bimba_map.py), regenerates the registry
# extract and every map-derived table, and re-bases the owner-ratified K8
# promotion onto the new registry. Without an endpoint it reuses the existing
# read in target/bimba-map/map.json. Review the diff; the ledger, census
# receipt and M2/M3 successor proofs follow separately.
set -euo pipefail
cd "$(dirname "$0")/.."

if [[ "${1:-}" == "--endpoint" ]]; then
  python3 scripts/bimba_map.py read --endpoint "$2"
elif [[ -n "${QL_BIMBA_MAP:-}" ]]; then
  python3 scripts/bimba_map.py read
fi
test -f target/bimba-map/map.json || { echo "no map read: pass --endpoint URL or set QL_BIMBA_MAP" >&2; exit 64; }

python3 scripts/generate-m-tree.py --from-map
python3 - <<'PY'
import json
from pathlib import Path
revision = json.loads(Path('fixtures/kernel/m-tree-v1.json').read_text())['registry_revision']
path = Path('c/registry/promotions/k8-apertures-v1.json')
text = path.read_text()
old = json.loads(text)['base_registry_revision']
if old != revision:
    path.write_text(text.replace(old, revision))
    print(f'K8 promotion re-based: {old} -> {revision}')
PY
python3 scripts/k8-structure.py --write-receipt
python3 scripts/m2-catalogue.py refresh
python3 scripts/m2-correspondences.py refresh
python3 scripts/m2-field-report.py refresh
python3 scripts/m3-source-parity.py --refresh-lock > /dev/null
python3 scripts/m3-domain.py --map target/bimba-map/map.json --refresh
echo "refreshed from the Bimba map; review with: git diff --stat"
