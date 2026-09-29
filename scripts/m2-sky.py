#!/usr/bin/env python3
"""Build/verify the M2-5 sky projection: the planets, Sun, Earth and chakras
with their map property values, at the registry's own identities.

The compiled registry keeps property keys, not values. M2-5 is the sky, and its
synth reads values from the map: the planetary just octave
(m_2_5_interval_from_root), scale degree and function, Keplerian velocity, the
chakras' anatomy. This is a projection of the map read, not a second authority.
The just ratio is parsed from the map's own interval text and the text is kept
beside it.

  scripts/m2-sky.py --map target/bimba-map/map.json --refresh
  scripts/m2-sky.py            (verify identities against the registry)
"""
from __future__ import annotations
import argparse
import hashlib
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PATH = 'fixtures/kernel/m2-sky-v1.json'
REGISTRY = 'fixtures/kernel/m-tree-v1.json'
DROP = {'embedding', 'c_2_uuid'}


def ql_ref(coordinate: str) -> str:
    # QL spelling: the map coordinate with its brackets removed, '#'-prefixed.
    return '#' + coordinate[1:].replace('(', '').replace(')', '')


def just_ratio(text: str | None) -> list[int] | None:
    match = re.search(r'\((\d+):(\d+)', text or '')
    return [int(match.group(1)), int(match.group(2))] if match else None


def build(map_path: Path) -> dict:
    read = json.loads(map_path.read_text())
    registry = json.loads((ROOT / REGISTRY).read_text())
    ids = {n['source_ref']: n['id'] for n in registry['nodes']}
    nodes = []
    for node in read['nodes'].values():
        props = node.get('properties', node)
        coordinate = props.get('coordinate', '')
        if not coordinate.startswith('M2-5'):
            continue
        ref = ql_ref(coordinate)
        if ref not in ids:
            raise SystemExit(f'm2-sky: {ref} is not in the registry')
        values = {k: v for k, v in sorted(props.items()) if k not in DROP}
        entry = {'ref': ref, 'id': ids[ref], 'properties': values}
        ratio = just_ratio(values.get('m_2_5_interval_from_root'))
        if ratio:
            entry['just_ratio'] = ratio
        nodes.append(entry)
    nodes.sort(key=lambda n: n['ref'])
    return {
        'schema': 'ql.m2-sky/v1',
        'registry_revision': registry['registry_revision'],
        'map_content_sha256': read['content_sha256'],
        'standing': 'projection of the live Bimba map read at the registry identities; '
                    'just_ratio is parsed from m_2_5_interval_from_root, which is retained',
        'nodes': nodes,
    }


def verify(data: dict) -> None:
    registry = json.loads((ROOT / REGISTRY).read_text())
    ids = {n['source_ref']: n['id'] for n in registry['nodes']}
    if data['schema'] != 'ql.m2-sky/v1' or data['registry_revision'] != registry['registry_revision']:
        raise SystemExit('m2-sky: stale against the registry; refresh from the map')
    for node in data['nodes']:
        if ids.get(node['ref']) != node['id']:
            raise SystemExit(f"m2-sky: identity drift at {node['ref']}")
        if node.get('just_ratio') != just_ratio(node['properties'].get('m_2_5_interval_from_root')):
            raise SystemExit(f"m2-sky: just ratio disagrees with its map text at {node['ref']}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument('--map', type=Path)
    parser.add_argument('--refresh', action='store_true')
    args = parser.parse_args()
    if args.refresh:
        if not args.map:
            parser.error('--refresh requires --map')
        data = build(args.map)
        (ROOT / PATH).write_text(json.dumps(data, ensure_ascii=False, indent=1) + '\n')
    data = json.loads((ROOT / PATH).read_text())
    verify(data)
    digest = hashlib.sha256((ROOT / PATH).read_bytes()).hexdigest()
    tuned = sum('just_ratio' in n for n in data['nodes'])
    print(f"M2 sky: {len(data['nodes'])} nodes, {tuned} just ratios; {digest}")


if __name__ == '__main__':
    main()
