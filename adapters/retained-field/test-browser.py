#!/usr/bin/env python3
"""Drive actual retained GPGPU, not a fake renderer. Keep browser/source receipts."""
import functools
import http.server
import json
from pathlib import Path
import socketserver
import threading
from playwright.sync_api import sync_playwright
ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'target/k8-gpu'
OUT.mkdir(exist_ok=True, parents=True)
(OUT / 'index.html').write_text('<!doctype html><canvas></canvas><script type="module" src="./acceptance.js"></script>')
handler = functools.partial(http.server.SimpleHTTPRequestHandler, directory=str(OUT))
with socketserver.TCPServer(('127.0.0.1', 0), handler) as server:
    thread = threading.Thread(target=server.serve_forever, daemon=True); thread.start()
    try:
        with sync_playwright() as p:
            browser = p.chromium.launch(headless=True, args=['--use-angle=swiftshader', '--enable-unsafe-swiftshader'])
            page = browser.new_page(); console = []
            page.on('console', lambda msg: console.append(msg.type + ': ' + msg.text))
            page.on('pageerror', lambda error: console.append('pageerror: ' + str(error)))
            page.goto(f'http://127.0.0.1:{server.server_address[1]}')
            page.wait_for_function('window.acceptance !== undefined', timeout=120000)
            result = page.evaluate('window.acceptance')
            result['browser_version'] = browser.version
            (OUT / 'acceptance.json').write_text(json.dumps(result, indent=2) + '\n')
            (OUT / 'console.log').write_text('\n'.join(console) + '\n')
            browser.close()
            assert 'error' not in result, result
            assert not any('Error' in line or 'pageerror' in line for line in console), console
            print(json.dumps(result, indent=2))
    finally:
        server.shutdown(); thread.join()
