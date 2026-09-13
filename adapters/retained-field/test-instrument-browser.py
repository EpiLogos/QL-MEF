#!/usr/bin/env python3
"""Actual Rust/C++ owner -> InstrumentSession -> retained GPU and browser audio.

The exposed functions below are a disposable acceptance transport, not production
O:I permissions or desktop IPC. Every native request/response is retained/replayed.
"""
from __future__ import annotations
import functools
import hashlib
import http.server
import importlib.util
import json
import os
from pathlib import Path
import socketserver
import subprocess
import threading
from playwright.sync_api import sync_playwright

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(os.environ.get('K8_INSTRUMENT_OUT_DIR', ROOT / 'target/k8-instrument'))
OUT.mkdir(parents=True, exist_ok=True)
# Reuse the bounded actual host test driver; no fake native response producer.
spec = importlib.util.spec_from_file_location('k8_host_test', ROOT / 'scripts/test-k8-host.py')
hosts = importlib.util.module_from_spec(spec)
spec.loader.exec_module(hosts)
hosts.OUT = OUT
hosts.HOST = ROOT / 'target/k8-cpp/bin/ql-field-host'
hosts.WORKER = ROOT / 'target/k8-cpp/bin/ql-field-worker'
config = hosts.deepcopy(hosts.CONFIG)
config['instance_ref'] = 'controlled:k8-browser:single-owner'
changed = json.loads((hosts.COUPLED / 'changed-event.json').read_text())['basis']['input']
console = []
result = {}

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

# Bundle against the INSTALLED adapter assets, while retaining the exact actual
# Point-Cloud source and current acceptance program. Hash equality is mandatory.
installed = ROOT / 'target/k8-cpp/share/ql-mef-c/adapters/retained-field'
for name in ('retained-field.mjs', 'native-audio.mjs', 'instrument-session.mjs'):
    assert digest(installed / name) == digest(ROOT / 'adapters/retained-field' / name), name
source = (ROOT / 'adapters/retained-field/instrument-acceptance.ts').read_text()
source = source.replace("'./retained-field.mjs'", "'../k8-cpp/share/ql-mef-c/adapters/retained-field/retained-field.mjs'")
source = source.replace("'./instrument-session.mjs'", "'../k8-cpp/share/ql-mef-c/adapters/retained-field/instrument-session.mjs'")
source = source.replace("'../../target/point-cloud-source/", "'../point-cloud-source/")
(OUT / 'installed-acceptance.ts').write_text(source)
subprocess.run([str(ROOT / 'node_modules/.bin/esbuild'), str(OUT / 'installed-acceptance.ts'),
    '--bundle', '--format=esm', f'--outfile={OUT / "acceptance.js"}'], cwd=ROOT, check=True)
(OUT / 'index.html').write_text('<!doctype html><canvas></canvas><script type="module" src="./acceptance.js"></script>')
handler = functools.partial(http.server.SimpleHTTPRequestHandler, directory=str(OUT))
host = hosts.Host(config, 'browser')
initial = hosts.deepcopy(host.current)
try:
    with socketserver.TCPServer(('127.0.0.1', 0), handler) as server:
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()
        try:
            with sync_playwright() as playwright:
                browser = playwright.chromium.launch(headless=True, args=['--use-angle=swiftshader',
                    '--enable-unsafe-swiftshader', '--autoplay-policy=no-user-gesture-required'])
                page = browser.new_page()
                page.on('console', lambda msg: console.append(msg.type + ': ' + msg.text))
                page.on('pageerror', lambda error: console.append('pageerror: ' + str(error)))
                page.expose_function('testNativeReady', lambda: initial)
                page.expose_function('testChangedBasis', lambda: changed)
                page.expose_function('testNativeExchange', lambda packet: host.send(packet=packet))
                page.goto(f'http://127.0.0.1:{server.server_address[1]}/index.html')
                page.wait_for_function('window.acceptance !== undefined', timeout=120000)
                result = page.evaluate('window.acceptance')
                result['browser_version'] = browser.version
                result['host_sha256'] = digest(hosts.HOST)
                result['worker_sha256'] = digest(hosts.WORKER)
                result['installed_adapters_sha256'] = {name: digest(installed / name) for name in (
                    'retained-field.mjs', 'native-audio.mjs', 'instrument-session.mjs')}
                (OUT / 'acceptance.json').write_text(json.dumps(result, indent=2) + '\n')
                browser.close()
        finally:
            server.shutdown()
            thread.join()
finally:
    host.save('browser-native')
    host.close()
    (OUT / 'console.log').write_text('\n'.join(console) + '\n')
assert 'error' not in result and result, result
assert not any('pageerror:' in line for line in console), console
replay = hosts.Host(config, 'browser-replay')
try:
    for request in host.requests:
        replay.send(packet=request)
    assert replay.responses == host.responses, 'browser path does not replay original native operations'
    replay.save('browser-replay')
finally:
    replay.close()
result['exact_original_native_replay'] = True
(OUT / 'acceptance.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps(result, indent=2))
