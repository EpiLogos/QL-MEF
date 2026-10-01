#!/usr/bin/env python3
"""Real source guard over actual native default and controlled corrupt files.

The positive bytes must come from the native generator; this test creates no
positive sky/default fixture. No native computation or receipt is mocked.
"""
import contextlib
import copy
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('native_default_source_guard',
                                            ROOT / 'scripts/check-scene-default-source.py')
guard = importlib.util.module_from_spec(spec)
spec.loader.exec_module(guard)

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

class DefaultSourceTests(unittest.TestCase):
    def test_actual_native_default_is_source_qualified_without_rewriting_the_original(self):
        original_path = ROOT / 'fixtures/kernel/scene-default-event-v1.json'
        actual_path = ROOT / 'fixtures/kernel/scene-default-event-v2.json'
        before = sha(actual_path)
        self.assertEqual(sha(original_path),
                         '31727a18b2e9b0f0773c9359b3150aeff7d89e6ff66227bb54b226350a1ad89e')
        original = json.loads(original_path.read_text())
        actual = json.loads(actual_path.read_text())
        old_sky = next(r for r in original['source_receipts'] if r.get('schema') == 'ql.sky-snapshot/v1')
        new_sky = next(r for r in actual['source_receipts'] if r.get('schema') == 'ql.sky-snapshot/v1')
        # Independent retained authored input/physical source, not values read
        # from the guard's own successful output.
        self.assertEqual(new_sky['bodies'], old_sky['bodies'])
        self.assertEqual(new_sky['epoch_unix_ms'], old_sky['epoch_unix_ms'])
        self.assertEqual(new_sky['request']['mode'], 'historical')
        self.assertEqual(new_sky['source_binding']['sun_role'], 'solar-parent')
        with contextlib.redirect_stdout(io.StringIO()):
            guard.main()
        self.assertEqual(sha(actual_path), before)
        self.assertEqual(sha(original_path),
                         '31727a18b2e9b0f0773c9359b3150aeff7d89e6ff66227bb54b226350a1ad89e')

    def test_source_or_recipe_loss_refuses_the_real_file_input(self):
        actual = json.loads((ROOT / 'fixtures/kernel/scene-default-event-v2.json').read_text())
        original_bytes = (ROOT / 'fixtures/kernel/scene-default-event-v1.json').read_bytes()
        changes = [
            ('stale-m2-registry', lambda x: x['m2'].__setitem__('registry_revision', '0' * 64)),
            ('stale-m3-registry', lambda x: x['m3'].__setitem__('registry_revision', '0' * 64)),
            ('lost-authored-tick', lambda x: x['m1'].__setitem__('tick12', 8)),
            ('lost-authored-aperture', lambda x: x['m3'].__setitem__('aperture', 3)),
            ('lost-embedded-sky', lambda x: x.__setitem__('source_receipts', [])),
            ('wrong-observation-provider', lambda x: x['m2']['world_observations'][0].__setitem__('provider_ref', 'sha256:' + '0' * 64)),
        ]
        for name, change in changes:
            with self.subTest(name=name), tempfile.TemporaryDirectory(prefix='ql-default-source-test-') as directory:
                target = Path(directory)
                (target / 'fixtures/kernel').mkdir(parents=True)
                (target / 'fixtures/kernel/scene-default-event-v1.json').write_bytes(original_bytes)
                changed = copy.deepcopy(actual)
                change(changed)
                (target / 'fixtures/kernel/scene-default-event-v2.json').write_text(json.dumps(changed))
                # Only the receiving file location changes. The actual provider,
                # current native header/source binding and guard stay loaded.
                previous = guard.ROOT
                guard.ROOT = target
                try:
                    with self.assertRaises((ValueError, AssertionError, KeyError, OSError)):
                        with contextlib.redirect_stdout(io.StringIO()):
                            guard.main()
                finally:
                    guard.ROOT = previous

if __name__ == '__main__':
    unittest.main(verbosity=2)
