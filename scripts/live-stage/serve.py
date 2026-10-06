#!/usr/bin/env python3
"""The live stage server: one real coupled host, one page, your speakers.

Holds the actual Rust/C++ host process open (the same ql-field-host the
managed acceptance and the O:I stage transport drive) and bridges the page's
fetch() transport to it over HTTP. Not an acceptance: nothing here scores or
asserts — the owner plays.
"""
from __future__ import annotations
import argparse, copy, json, os, selectors, subprocess, time
from functools import partial
from http.server import ThreadingHTTPServer, SimpleHTTPRequestHandler
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'target/live-stage/page'
BOOT_TIMEOUT = 120.0


class Host:
    """One live ql-field-host process over the bounded newline-JSON protocol."""

    buf = b''

    def __init__(self, host_bin, worker_bin, config_path, instance_ref):
        config = json.loads(Path(config_path).read_text())
        config['instance_ref'] = instance_ref
        path = ROOT / 'target/live-stage/used-config.json'
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(config, allow_nan=False))
        self.process = subprocess.Popen(
            [str(host_bin), str(worker_bin), str(path)],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE,
            stderr=subprocess.PIPE, bufsize=0)
        self.current = self._receive(time.monotonic() + BOOT_TIMEOUT)
        assert self.current.get('status') == 'ready' and self.current.get('available'), \
            'the coupled host did not come up ready'
        self.original = copy.deepcopy(self.current)

    def _receive(self, deadline):
        with selectors.DefaultSelector() as selector:
            selector.register(self.process.stdout, selectors.EVENT_READ)
            while b'\n' not in self.buf:
                wait = deadline - time.monotonic()
                assert wait > 0, 'host acknowledgement timeout'
                if not selector.select(wait):
                    continue
                chunk = os.read(self.process.stdout.fileno(), 1 << 16)
                assert chunk, 'owned native EOF'
                self.buf += chunk
                assert len(self.buf) < 128 * 1024 * 1024
        line, self.buf = self.buf.split(b'\n', 1)
        return json.loads(line)

    def ready(self):
        return self.original

    def exchange(self, packet):
        self.process.stdin.write(
            (json.dumps(packet, separators=(',', ':'), allow_nan=False) + '\n').encode())
        self.process.stdin.flush()
        self.current = self._receive(time.monotonic() + BOOT_TIMEOUT)
        return copy.deepcopy(self.current)

    def close(self):
        try:
            self.process.stdin.close()
        except Exception:
            pass
        try:
            self.process.wait(timeout=15)
        except Exception:
            self.process.kill()
            self.process.wait(timeout=5)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--host-bin', default=ROOT / 'target/debug/ql-field-host')
    parser.add_argument('--worker-bin', default=ROOT / 'cpp/build/ql-field-worker')
    parser.add_argument('--config', default=ROOT / 'target/live-stage/host-config.json')
    parser.add_argument('--port', type=int, default=8787)
    args = parser.parse_args()
    holder = {'stage': Host(args.host_bin, args.worker_bin, args.config, f'live:stage-{os.getpid()}')}

    def reset_stage():
        # One page open is one fresh performance: the host's request sequence
        # is per-session state, so a new page gets a new host.
        try:
            holder['stage'].close()
        except Exception:
            pass
        holder['stage'] = Host(args.host_bin, args.worker_bin, args.config,
                               f'live:stage-{os.getpid()}-{time.monotonic_ns()}')

    stage = holder['stage']
    OUT.mkdir(parents=True, exist_ok=True)
    index = (Path(__file__).parent / 'index.html').read_text()
    (OUT / 'index.html').write_text(index)

    class Handler(SimpleHTTPRequestHandler):
        def log_message(self, *log_args):
            pass

        def end_headers(self):
            self.send_header('Cache-Control', 'no-store')
            super().end_headers()

        def do_GET(self):
            path = self.path.split('?', 1)[0]
            if path == '/native-ready':
                body = json.dumps(holder['stage'].ready()).encode()
                self.send_response(200)
                self.send_header('Content-Type', 'application/json')
                self.send_header('Content-Length', str(len(body)))
                self.end_headers()
                self.wfile.write(body)
                return
            if path in ('/', '/index.html'):
                reset_stage()
                body = index.encode()
                self.send_response(200)
                self.send_header('Content-Type', 'text/html')
                self.send_header('Content-Length', str(len(body)))
                self.end_headers()
                self.wfile.write(body)
                return
            if Path(self.translate_path(path)).is_file():
                super().do_GET()
            else:
                self.send_error(404)

        def do_POST(self):
            if self.path.split('?', 1)[0] != '/native':
                self.send_error(404)
                return
            length = int(self.headers.get('Content-Length', 0))
            packet = json.loads(self.rfile.read(length))
            reply = holder['stage'].exchange(packet)
            body = json.dumps(reply, allow_nan=False).encode()
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.send_header('Content-Length', str(len(body)))
            self.end_headers()
            self.wfile.write(body)

    server = ThreadingHTTPServer(('127.0.0.1', args.port), partial(Handler, directory=str(OUT)))
    print(f'live stage: http://127.0.0.1:{args.port}/  (host pid {stage.process.pid})', flush=True)
    try:
        server.serve_forever()
    finally:
        holder['stage'].close()


if __name__ == '__main__':
    main()
