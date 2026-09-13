#!/usr/bin/env python3
"""Exercise the installed coupled host, not a second numerical implementation.

Run the existing real-sky and k8_coupled producer first. Their complete input,
including source receipts and the explicitly controlled geometry, is reused.
"""
from __future__ import annotations
from copy import deepcopy
import json
import os
from pathlib import Path
import selectors
import signal
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'target/k8-host'
OUT.mkdir(parents=True, exist_ok=True)
HOST = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else ROOT / 'target/debug/ql-field-host'
WORKER = Path(sys.argv[2]).resolve() if len(sys.argv) > 2 else ROOT / 'target/k8-cpp/bin/ql-field-worker'
CONFIG = json.loads((ROOT / 'target/k8-coupled/input.json').read_text())
CONFIG['instance_ref'] = 'controlled:k8-host:primary'


def encoded(value):
    return json.dumps(value, allow_nan=False, separators=(',', ':'))


class Host:
    def __init__(self, config, suffix):
        self.path = OUT / f'config-{suffix}.json'
        self.path.write_text(encoded(config))
        self.process = subprocess.Popen([str(HOST), str(WORKER), str(self.path)], stdin=subprocess.PIPE,
            stdout=subprocess.PIPE, stderr=subprocess.PIPE, bufsize=0)
        self.requests, self.responses = [], []
        self.buffer = b''
        self.current = self.receive()
        assert self.current['status'] == 'ready' and self.current['available']
        self.original = deepcopy(self.current)

    def receive(self):
        assert self.process.stdout
        with selectors.DefaultSelector() as selector:
            selector.register(self.process.stdout, selectors.EVENT_READ)
            while b'\n' not in self.buffer:
                if not selector.select(15):
                    raise AssertionError('host acknowledgement timeout')
                chunk = os.read(self.process.stdout.fileno(), 65536)
                if not chunk:
                    raise AssertionError('host exited before response')
                self.buffer += chunk
                assert len(self.buffer) <= 64 * 1024 * 1024 + 1
        line, self.buffer = self.buffer.split(b'\n', 1)
        result = json.loads(line)
        self.responses.append(result)
        return result

    def packet(self, command):
        return dict(schema='ql.field-host-request/v1', instance_ref=self.current['instance_ref'],
            event_ref=self.current['field']['event_ref'], subject_ref=self.current['field']['subject_ref'],
            request_id=str(int(self.current['last_request_id']) + 1),
            expected_generation=self.current['field']['generation'],
            expected_samples_elapsed=self.current['field']['samples_elapsed'], command=command)

    def send(self, command=None, packet=None):
        request = packet if packet is not None else self.packet(command)
        self.requests.append(deepcopy(request))
        assert self.process.stdin
        self.process.stdin.write((encoded(request) + '\n').encode())
        self.process.stdin.flush()
        self.current = self.receive()
        return self.current

    def close(self, failed=False):
        if self.process.stdin:
            self.process.stdin.close()
        try:
            code = self.process.wait(timeout=10)
        except subprocess.TimeoutExpired:
            self.process.kill()
            self.process.wait(timeout=5)
            raise
        stderr = self.process.stderr.read().decode() if self.process.stderr else ''
        if not failed:
            assert code == 0, stderr
        for stream in (self.process.stdout, self.process.stderr):
            if stream:
                stream.close()

    def save(self, name):
        (OUT / f'{name}-requests.jsonl').write_text(''.join(encoded(v) + '\n' for v in self.requests))
        (OUT / f'{name}-responses.jsonl').write_text(''.join(encoded(v) + '\n' for v in self.responses))


def state(value):
    return dict(value['field'], audio=[])


def main():
    host = Host(CONFIG, 'primary')
    first = state(host.current)
    try:
        inspection = host.send(dict(operation='inspect'))
        assert inspection['sources']['original'] == inspection['sources']['current']
        assert inspection['sources']['original']['input'] == CONFIG['basis']
        assert inspection['sources']['original_field'] == CONFIG['field']
        for _ in range(100):
            result = host.send(dict(operation='read'))
            assert state(result) == first and 'sources' not in result and not result['field']['audio']
        initial_native = json.loads((ROOT / 'target/k8-coupled/original-event.json').read_text())['field']
        assert first == initial_native, 'compact host changed the native full-producer result'
        source_bytes = len(encoded(inspection))
        compact_bytes = len(encoded(result))
        assert source_bytes > compact_bytes * 3, 'data plane unexpectedly repeats full sources'
        exact = host.packet(dict(operation='advance', frames=512, muted=False))
        result = host.send(packet=exact)
        assert result['status'] == 'ok' and len(result['field']['audio']) == 512
        assert result['field']['samples_elapsed'] == '512'
        assert any(abs(v) > 1e-7 for v in result['field']['audio'])
        retained = state(result)
        repeated = host.send(packet=exact)
        assert repeated['status'] == 'refused' and state(repeated) == retained
        assert repeated['field']['audio'] == [], 'refusal replayed previously emitted PCM'
        for key, value in [('instance_ref', 'foreign'), ('subject_ref', 'foreign'), ('event_ref', 'foreign'),
                           ('request_id', '01'), ('expected_samples_elapsed', '0')]:
            bad = host.packet(dict(operation='advance', frames=512, muted=False))
            bad[key] = value
            result = host.send(packet=bad)
            assert result['status'] == 'refused' and state(result) == retained
        bad = host.packet(dict(operation='read'))
        bad['ambient_authority'] = True
        result = host.send(packet=bad)
        assert result['status'] == 'refused' and result['request_id'] is None and state(result) == retained
        for command in [dict(operation='advance', frames=8193, muted=False),
                        dict(operation='set-axis', axis=2, phase=dict(turns='0', half_degrees=0))]:
            result = host.send(command)
            assert result['status'] == 'refused' and state(result) == retained
        result = host.send(dict(operation='set-axis', axis=1, phase=dict(turns='-2', half_degrees=37)))
        assert result['status'] == 'ok'
        assert result['field']['clock']['lensing']['half_degrees'] == 37
        assert result['field']['clock']['inscription'] == retained['clock']['inscription']
        assert result['field']['amplitudes_metres'] == retained['amplitudes_metres']
        assert result['field']['samples_elapsed'] == retained['samples_elapsed']
        changed = json.loads((ROOT / 'target/k8-coupled/changed-event.json').read_text())['basis']['input']
        before = state(result)
        result = host.send(dict(operation='replace', basis=changed))
        assert result['status'] == 'ok', result.get('error')
        assert result['field']['amplitudes_metres'] == before['amplitudes_metres']
        assert result['field']['targets'] == before['targets']
        assert result['field']['clock'] == before['clock']
        observed = host.send(dict(operation='inspect'))
        assert observed['sources']['original']['input'] == CONFIG['basis']
        assert observed['sources']['current']['input'] == changed
        assert state(observed) == state(result) and observed['field']['audio'] == []
        result = host.send(dict(operation='advance', frames=1024, muted=True))
        assert result['status'] == 'ok' and result['field']['samples_elapsed'] == '1536'
        assert result['field']['audio'] == [0.0] * 1024
        result = host.send(dict(operation='advance', frames=1024, muted=False))
        assert result['field']['samples_elapsed'] == '2560' and any(result['field']['audio'])
        assert [(t['identity'], t['constituent']) for t in result['field']['targets']] == [
            (t['identity'], t['constituent']) for t in first['targets']]
        host.save('primary')
    finally:
        host.close()

    replay = Host(CONFIG, 'replay')
    try:
        for request in host.requests:
            replay.send(packet=request)
        assert replay.responses == host.responses, 'original operation replay changed'
        replay.save('replay')
    finally:
        replay.close()

    other = deepcopy(CONFIG)
    other['instance_ref'] = 'controlled:k8-host:independent'
    other['field']['subject_ref'] = 'controlled:k8-other-subject'
    other['basis']['m3']['subject_ref'] = other['field']['subject_ref']
    independent = Host(other, 'independent')
    try:
        assert independent.current['field']['event_ref'] == first['event_ref']
        assert independent.current['field']['subject_ref'] != first['subject_ref']
        assert independent.current['field']['samples_elapsed'] == '0'
        independent.send(dict(operation='advance', frames=128, muted=False))
        assert independent.current['field']['samples_elapsed'] == '128'
        # Controlled local provider-loss test kills ONLY this test's own worker.
        pids = subprocess.check_output(['ps', '-o', 'pid=', '--ppid', str(independent.process.pid)], text=True).split()
        assert len(pids) == 1, 'expected exactly one native numerical child'
        os.kill(int(pids[0]), signal.SIGKILL)
        result = independent.send(dict(operation='read'))
        assert result['status'] == 'unavailable' and not result['available']
        assert result['field']['samples_elapsed'] == '128' and result['field']['audio'] == []
        assert independent.process.wait(timeout=10) != 0
        independent.save('provider-loss')
    finally:
        independent.close(failed=True)

    receipt = dict(schema='ql.k8-host-acceptance/v1', native_commands=len(host.requests),
        read_only_requests=100, one_native_child=True, compact_bytes=compact_bytes, full_inspection_bytes=source_bytes,
        original_and_current_bases=True, stable_samples=True, exact_replay=True,
        stale_scope_and_sequence_refused=True, provider_loss_unavailable=True,
        independent_controlled_subjects_share_sky=True,
        standing='repository installed host; actual dated source input, controlled geometry/subjects, not full Nara acceptance')
    (OUT / 'acceptance.json').write_text(json.dumps(receipt, indent=2) + '\n')
    print(json.dumps(receipt))


if __name__ == '__main__':
    main()
