#!/usr/bin/env python3
"""Exercise the exported native sky command with a real qualified provider."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]


class SkyExportTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory(prefix='ql-sky-export-test-')
        self.addCleanup(self.directory.cleanup)
        self.home = Path(self.directory.name)
        self.command = self.home / 'ql-sky'
        self.receipt = self.home / 'receipt.json'
        run = subprocess.run([sys.executable, '-B', str(ROOT / 'scripts/export-sky.py'),
                              '--output', str(self.command), '--receipt', str(self.receipt)],
                             capture_output=True, text=True, timeout=30)
        self.assertEqual(run.returncode, 0, run.stderr)
        self.env = dict(os.environ, HOME=str(self.home), XDG_CACHE_HOME=str(self.home / 'cache'),
                        PYTHONDONTWRITEBYTECODE='1')

    def run_sky(self, interpreter):
        self.env['QL_SKY_PYTHON'] = str(interpreter)
        request = json.loads((ROOT / 'fixtures/kernel/sky-snapshot-world-2026-09-28-v4.json').read_text())['request']
        return subprocess.run([str(self.command), '-'], input=json.dumps(request),
                              capture_output=True, text=True, env=self.env, timeout=60)

    def test_export_records_exact_source_and_artifact_bytes(self):
        receipt = json.loads(self.receipt.read_text())
        self.assertEqual(receipt['schema'], 'ql.sky-command-export/v1')
        self.assertEqual(receipt['artifact']['sha256'], hashlib.sha256(self.command.read_bytes()).hexdigest())
        self.assertTrue(os.access(self.command, os.X_OK))
        for record in receipt['source_inputs']:
            self.assertEqual(record['sha256'], hashlib.sha256((ROOT / record['path']).read_bytes()).hexdigest())
        for record in receipt['resources']:
            self.assertEqual(record['sha256'], hashlib.sha256((ROOT / record['path']).read_bytes()).hexdigest())
        self.run_sky(self.home / 'missing-python')
        extracted = self.home / 'cache/ql-sky' / receipt['payload_sha256'][:16]
        for record in receipt['resources']:
            self.assertEqual(record['sha256'], hashlib.sha256((extracted / record['path']).read_bytes()).hexdigest())
        second = self.home / 'ql-sky-second'
        run = subprocess.run([sys.executable, '-B', str(ROOT / 'scripts/export-sky.py'), '--output', str(second)],
                             capture_output=True, text=True, timeout=30)
        self.assertEqual(run.returncode, 0, run.stderr)
        self.assertEqual(self.command.read_bytes(), second.read_bytes())

    def test_actual_managed_provider_produces_source_bound_ten_body_sky(self):
        interpreter = os.environ.get('QL_TEST_SKY_PYTHON')
        if not interpreter:
            self.skipTest('QL_TEST_SKY_PYTHON must name an actual qualified provider interpreter')
        run = self.run_sky(interpreter)
        self.assertEqual(run.returncode, 0, run.stderr)
        sky = json.loads(run.stdout)
        self.assertEqual(sky['schema'], 'ql.sky-snapshot/v1')
        self.assertEqual(len(sky['bodies']), 10)
        self.assertEqual(sky['provider']['version'], '5.7.1')
        self.assertEqual(sky['provider']['wrapper_version'], '2.10.3.2')
        self.assertEqual(sky['provider']['adapter_sha256'], hashlib.sha256((ROOT / 'providers/sky/kerykeion_snapshot.py').read_bytes()).hexdigest())
        self.assertEqual(sky['source_binding']['header_sha256'], hashlib.sha256((ROOT / 'vendor/epi-kernel/reference/include/m2.h').read_bytes()).hexdigest())
        retained = json.loads((ROOT / 'fixtures/kernel/sky-snapshot-world-2026-09-28-v4.json').read_text())
        self.assertEqual(sky['request'], retained['request'])
        self.assertEqual([body['native_planet_id'] for body in sky['bodies']],
                         [body['native_planet_id'] for body in retained['bodies']])
        for actual, expected in zip(sky['bodies'], retained['bodies']):
            self.assertAlmostEqual(actual['longitude_degrees'], expected['longitude_degrees'], places=8)

    def test_actual_default_uv_provider_remains_available_offline(self):
        uv = os.environ.get('QL_TEST_SKY_UV')
        interpreter = os.environ.get('QL_TEST_SKY_PYTHON')
        if not uv or not interpreter:
            self.skipTest('QL_TEST_SKY_UV and QL_TEST_SKY_PYTHON must name actual installed providers')
        run = subprocess.run([sys.executable, '-B', str(ROOT / 'scripts/export-sky.py'),
                              '--output', str(self.command), '--installed-uv', uv],
                             capture_output=True, text=True, timeout=30)
        self.assertEqual(run.returncode, 0, run.stderr)
        self.env.pop('QL_SKY_PYTHON', None)
        self.env.update(PATH='/usr/bin:/bin', UV_OFFLINE='1', UV_NO_CONFIG='1',
                        UV_CACHE_DIR=str(Path(os.environ['HOME']) / '.cache/uv'),
                        UV_PYTHON_INSTALL_DIR=str(Path(interpreter).resolve().parents[2]))
        request = json.loads((ROOT / 'fixtures/kernel/sky-snapshot-world-2026-09-28-v4.json').read_text())['request']
        run = subprocess.run([str(self.command), '-'], input=json.dumps(request),
                             capture_output=True, text=True, env=self.env, timeout=60)
        self.assertEqual(run.returncode, 0, run.stderr)
        self.assertEqual(json.loads(run.stdout)['schema'], 'ql.sky-snapshot/v1')

    def test_invalid_explicit_interpreter_never_falls_back_to_uv(self):
        for interpreter in [self.home / 'missing-python', self.home, '', 'python3']:
            with self.subTest(interpreter=str(interpreter)):
                run = self.run_sky(interpreter)
                self.assertEqual(run.returncode, 2)
                self.assertEqual(run.stdout, '')
                error = json.loads(run.stderr)
                self.assertEqual(error['schema'], 'ql.sky-error/v1')
                self.assertIn('QL_SKY_PYTHON', error['error'])
                self.assertIsNone(error['snapshot'])
        nonexecutable = self.home / 'not-executable'
        nonexecutable.write_text('ordinary data')
        run = self.run_sky(nonexecutable)
        self.assertEqual(run.returncode, 2)
        self.assertIn('QL_SKY_PYTHON', json.loads(run.stderr)['error'])


if __name__ == '__main__':
    unittest.main()
