#!/usr/bin/env python3
"""Compare each ACTUAL C literal to the original CSV, never to generated JSON."""
import argparse
import csv
import json
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
def check(path):
    with (ROOT / 'migration/epi-kernel/m1-return/vortex-modulae.csv').open(newline='') as f:
        csv_rows = list(csv.reader(f))
    seen = set()
    for line in Path(path).read_text().splitlines():
        row = json.loads(line)
        if row.get('kind') != 'source':
            continue
        key = (row['family'], row['row12'], row['col12'])
        if key in seen:
            raise ValueError(f'duplicate C source cell: {key}')
        seen.add(key)
        f, r, c = key
        if not (0 <= f < 6 and 0 <= r < 12 and 0 <= c < 12):
            raise ValueError('invalid C source address')
        y = 4 + r + (r >= 10)
        x = [22, 3, 42, 64, 87, 108][f] + c + (c >= 10)
        if row['raw_literal'] != csv_rows[y][x] or row['digit_root_literal'] != csv_rows[y+34][x]:
            raise ValueError(f'changed source reading at {key}')
        if (row['csv_raw_row'],row['csv_dr_row'],row['csv_column']) != (y+1,y+35,x+1):
            raise ValueError(f'changed source provenance at {key}')
    if seen != {(f,r,c) for f in range(6) for r in range(12) for c in range(12)}:
        raise ValueError('incomplete C source field')
    return len(seen)*2
if __name__ == '__main__':
    parser = argparse.ArgumentParser();parser.add_argument('--observations', required=True)
    args = parser.parse_args();print(f'Actual C literal-source parity: {check(args.observations)} exact raw/DR strings')
