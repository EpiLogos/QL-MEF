"""The historical proof is never silently made to cover a different engine."""
import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location('k8_preservation', ROOT / 'scripts/k8-preservation.py')
k8 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(k8)


class PreservedProof(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        for path, content in [('c/Makefile', 'new integration'), ('c/src/m2.c', 'unchanged numeric owner'),
                              ('fixtures/kernel/history/pre-k8-Makefile', 'old integration')]:
            p = self.root / path
            p.parent.mkdir(parents=True, exist_ok=True)
            p.write_text(content)
        self.rev = 'a' * 40
        old = k8.digest(self.root / 'fixtures/kernel/history/pre-k8-Makefile')
        self.proof = {'inputs': {'c/Makefile': old, 'c/src/m2.c': k8.digest(self.root / 'c/src/m2.c')},
                      'checks': ['complete native test contract']}
        p = self.root / 'fixtures/kernel/m2-finite-proof-v1.json'
        p.write_text(json.dumps(self.proof))
        manifest = {'schema': 'ql.k8-build-lineage/v1', 'proofs': {'m2': k8.digest(p)},
                    'integration_input': {'path': 'c/Makefile', 'before_sha256': old,
                                          'after_sha256': k8.digest(self.root / 'c/Makefile'),
                                          'retained_before': 'fixtures/kernel/history/pre-k8-Makefile'}}
        (self.root / k8.MANIFEST).write_text(json.dumps(manifest))
        self.fresh = {'schema': 'ql.m2-acceptance/v1', 'revision': self.rev, 'result': 'passed',
                      'inputs': {path: k8.digest(self.root / path) for path in self.proof['inputs']},
                      'checks': self.proof['checks']}

    def verify(self):
        return k8.verify(self.root, 'm2', self.proof, self.fresh, self.rev)

    def test_new_build_requires_fresh_complete_exact_head_execution(self):
        result = self.verify()
        self.assertEqual(result['integration_changes'], ['c/Makefile'])
        self.assertEqual(result['result'], 'passed')
        self.assertEqual(self.proof['inputs']['c/Makefile'],
                         k8.digest(self.root / 'fixtures/kernel/history/pre-k8-Makefile'))

    def test_unknown_integration_edit_rejected(self):
        (self.root / 'c/Makefile').write_text('unreviewed integration')
        self.fresh['inputs']['c/Makefile'] = k8.digest(self.root / 'c/Makefile')
        with self.assertRaisesRegex(ValueError, 'unreviewed'):
            self.verify()

    def test_changed_numerical_body_cannot_borrow_old_proof(self):
        (self.root / 'c/src/m2.c').write_text('changed numerical owner')
        self.fresh['inputs']['c/src/m2.c'] = k8.digest(self.root / 'c/src/m2.c')
        with self.assertRaisesRegex(ValueError, 'numerical'):
            self.verify()

    def test_changed_old_proof_rejected(self):
        (self.root / 'fixtures/kernel/m2-finite-proof-v1.json').write_text('{}')
        with self.assertRaisesRegex(ValueError, 'historical'):
            self.verify()

    def test_stale_partial_failed_and_wrong_scope_execution_rejected(self):
        for change in ({'revision': 'b'*40}, {'result': 'failed'}, {'checks': []},
                       {'inputs': {'c/Makefile': self.fresh['inputs']['c/Makefile']}},
                       {'schema': 'ql.m3-acceptance/v1'}):
            with self.subTest(change=change):
                value = copy.deepcopy(self.fresh)
                value.update(change)
                with self.assertRaises(ValueError):
                    k8.verify(self.root, 'm2', self.proof, value, self.rev)

    def test_post_execution_input_change_rejected(self):
        (self.root / 'c/src/m2.c').write_text('after execution')
        with self.assertRaisesRegex(ValueError, 'fresh native'):
            self.verify()


if __name__ == '__main__':
    unittest.main()
