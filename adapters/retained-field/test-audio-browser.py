#!/usr/bin/env python3
"""Installed C++ PCM -> actual browser AudioBuffer graph, with exact receipts.

Run scripts/test-k8-continuous.py first to produce the real dated M2 input.
No oscillator or synthetic replacement of the native field is used here.
"""
from __future__ import annotations
import argparse
from copy import deepcopy
import functools
import hashlib
import http.server
import json
from pathlib import Path
import shutil
import socketserver
import subprocess
import threading
from playwright.sync_api import sync_playwright

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'target/k8-audio'


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def produce(worker: Path, initial: dict, rate: int) -> dict:
    request = deepcopy(initial)
    request['field']['sample_rate'] = rate
    commands = [request]
    process = subprocess.Popen([str(worker)], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    assert process.stdin is not None and process.stdout is not None

    def send(value: dict) -> dict:
        process.stdin.write(json.dumps(value, allow_nan=False, separators=(',', ':')) + '\n')
        process.stdin.flush()
        line = process.stdout.readline()
        if not line:
            raise RuntimeError('native worker exited before its acknowledged field')
        result = json.loads(line)
        if result.get('schema') != 'ql.continuous-field/v1':
            raise RuntimeError(f'native field refused: {result}')
        return result

    first = send(request)
    state = first
    frames = []
    for count in [128, 255, 256, 511, 4096, 8192]:
        command = dict(schema='ql.field-control/v1', operation='advance',
            expected_generation=state['generation'], expected_samples_elapsed=state['samples_elapsed'],
            frames=count, muted=(count == 511))
        commands.append(command)
        state = send(command)
        frames.append(state)
    read = dict(schema='ql.field-control/v1', operation='read')
    commands.append(read)
    recovery_read = send(read)
    # A view has no simulation owner: read leaves clock, amplitudes and targets
    # exactly as they were; only the just-produced PCM interval is absent.
    assert recovery_read == dict(state, audio=[])
    command = dict(schema='ql.field-control/v1', operation='advance',
        expected_generation=state['generation'], expected_samples_elapsed=state['samples_elapsed'],
        frames=1024, muted=False)
    commands.append(command)
    recovered = send(command)
    process.stdin.close()
    assert process.wait(timeout=10) == 0
    process.stdout.close()
    if process.stderr:
        process.stderr.close()
    replay = subprocess.run([str(worker)], input=''.join(json.dumps(v) + '\n' for v in commands),
        text=True, capture_output=True, check=True, timeout=30)
    assert [json.loads(line) for line in replay.stdout.splitlines()] == [first, *frames, recovery_read, recovered]
    assert any(abs(value) > 1e-6 for frame in frames for value in frame['audio'])
    return dict(rate=rate, initial=first, frames=frames, recovery_read=recovery_read, recovered=recovered,
        exact_native_replay=True)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--worker', type=Path, default=ROOT / 'target/k8-cpp/bin/ql-field-worker')
    parser.add_argument('--chromium', type=Path, help='explicit installed Chromium; otherwise Playwright-managed Chromium')
    parser.add_argument('--inline', action='store_true', help='inject the exact local module bodies without HTTP; no import/hosting claim')
    args = parser.parse_args()
    OUT.mkdir(parents=True, exist_ok=True)
    worker = args.worker.resolve()
    initial_path = ROOT / 'target/k8-continuous/initial.json'
    initial = json.loads(initial_path.read_text())
    cases = [produce(worker, initial, rate) for rate in [44100, 48000, 96000]]
    fixture = dict(schema='ql.k8-native-audio-cases/v1', worker_sha256=digest(worker),
        native_initial_sha256=digest(initial_path), cases=cases)
    (OUT / 'audio-cases.json').write_text(json.dumps(fixture, sort_keys=True) + '\n')
    receiver = worker.parent.parent / 'share/ql-mef-c/adapters/retained-field/native-audio.mjs'
    assert receiver.is_file(), 'install the native presentation asset before browser acceptance'
    assert digest(receiver) == digest(ROOT / 'adapters/retained-field/native-audio.mjs'), 'installed receiver/source mismatch'
    shutil.copy2(receiver, OUT / 'native-audio.mjs')
    shutil.copy2(ROOT / 'adapters/retained-field/audio-acceptance.mjs', OUT / 'audio-acceptance.mjs')
    (OUT / 'index.html').write_text('<!doctype html><meta charset="utf-8"><script type="module" src="./audio-acceptance.mjs"></script>')
    handler = functools.partial(http.server.SimpleHTTPRequestHandler, directory=str(OUT))
    with socketserver.TCPServer(('127.0.0.1', 0), handler) as server:
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()
        try:
            with sync_playwright() as playwright:
                browser = playwright.chromium.launch(executable_path=str(args.chromium) if args.chromium else None,
                    headless=True, args=['--no-sandbox'])
                page = browser.new_page()
                errors: list[str] = []
                page.on('pageerror', lambda error: errors.append(str(error)))
                if args.inline:
                    # Local-file execution for browser environments without an HTTP
                    # testing route. Only the module linkage changes, not its body.
                    receiver = (OUT / 'native-audio.mjs').read_text()
                    test = (OUT / 'audio-acceptance.mjs').read_text()
                    assert receiver.count('export class NativeAudioBinding') == 1
                    assert test.count("import { NativeAudioBinding } from './native-audio.mjs';") == 1
                    receiver = receiver.replace('export class NativeAudioBinding', 'class NativeAudioBinding')
                    test = test.replace("import { NativeAudioBinding } from './native-audio.mjs';", '')
                    page.evaluate('(fixture) => { window.nativeAudioCases = fixture; }', fixture)
                    page.add_script_tag(content=receiver + '\n' + test)
                else:
                    page.goto(f'http://127.0.0.1:{server.server_address[1]}/index.html')
                page.wait_for_function('window.acceptance !== undefined', timeout=90000)
                result = page.evaluate('window.acceptance')
                result.update(module_delivery='inline-local-bodies' if args.inline else 'HTTP-modules', browser_version=browser.version, browser_executable=str(args.chromium) if args.chromium else 'playwright-managed',
                    native_initial_sha256=digest(initial_path), fixture_sha256=digest(OUT / 'audio-cases.json'),
                    receiver_sha256=digest(ROOT / 'adapters/retained-field/native-audio.mjs'), installed_receiver_verified=True)
                (OUT / 'browser-acceptance.json').write_text(json.dumps(result, indent=2) + '\n')
                (OUT / 'browser-errors.json').write_text(json.dumps(errors, indent=2) + '\n')
                browser.close()
                assert 'error' not in result and not errors, (result, errors)
                print(json.dumps(result, indent=2))
        finally:
            server.shutdown()
            thread.join()


if __name__ == '__main__':
    main()
