#!/usr/bin/env python3
"""The QL agent tools' owner provenance, derived instead of hand-copied.

`adapters/pi/ql-agent/provenance.json` pins what the shared QL extension
(ql_project_event, ql_decide, ql_invoke, ...) accepts from the engine: the
SHA-256 of each owner source, and the kernel digest every projection receipt
carries. The extension refuses receipts whose kernel digest differs, so a pin
that drifts from the source silently disables the tools. Actuation's
distribution and Epi-Prime vendor this file; they copy it from here.

  --check            fail when a pinned source hash or the kernel digest is stale
  --write            rewrite the pins from the current source and engine
  --binary PATH      also pin the engine binary's SHA-256 (install time only;
                     it differs per build and platform, so --check ignores it)
  --ql PATH          the engine to read the kernel digest from (default: cargo run)
"""
import argparse
import hashlib
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PROVENANCE = ROOT / 'adapters/pi/ql-agent/provenance.json'
EVENT = ROOT / 'fixtures/agent-decision/v1/event-v1.json'


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def kernel_digest(ql):
    command = [ql] if ql else ['cargo', 'run', '-q', '-p', 'ql-cli', '--locked', '--']
    request = json.dumps({'event': json.loads(EVENT.read_text())})
    out = subprocess.run([*command, 'agent-event', 'project', '-', '--json'], input=request,
                         capture_output=True, text=True, cwd=ROOT, check=True).stdout
    receipt = json.loads(out)
    digests = {receipt[part]['kernel_basis']['digest'] for part in ('frame', 'determination', 'harmonic')}
    if len(digests) != 1:
        raise SystemExit(f'projection carries more than one kernel digest: {sorted(digests)}')
    return digests.pop()


def main():
    parser = argparse.ArgumentParser()
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument('--check', action='store_true')
    mode.add_argument('--write', action='store_true')
    parser.add_argument('--binary')
    parser.add_argument('--ql')
    args = parser.parse_args()
    raw = PROVENANCE.read_text()
    pinned = json.loads(raw)
    current = {name: sha256(ROOT / name) for name in pinned['source_sha256']}
    digest = kernel_digest(args.ql)
    if args.check:
        stale = [name for name, value in current.items() if pinned['source_sha256'][name] != value]
        if pinned['kernel_digest'] != digest:
            stale.append(f"kernel_digest {pinned['kernel_digest']} != {digest}")
        if stale:
            print('QL agent tool provenance is stale; run scripts/ql-agent-provenance.py --write:', file=sys.stderr)
            for item in stale:
                print(f'  {item}', file=sys.stderr)
            return 1
        return 0
    pinned['source_sha256'] = current
    pinned['kernel_digest'] = digest
    if args.binary:
        pinned['native_binary_sha256'] = sha256(Path(args.binary))
    PROVENANCE.write_text(json.dumps(pinned, indent=2, ensure_ascii=False) + ('\n' if raw.endswith('\n') else ''))
    print(json.dumps({'kernel_digest': digest, 'native_binary_sha256': pinned['native_binary_sha256']}))
    return 0


if __name__ == '__main__':
    sys.exit(main())
