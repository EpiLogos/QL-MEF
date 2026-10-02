#!/usr/bin/env python3
"""Challenge the current bootstrap with actual retained execution/source evidence."""
from __future__ import annotations
import copy
import importlib.util
import json
import unittest
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('requalification', ROOT / 'scripts/requalify-m-ledger.py')
owner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(owner)

class ActualSourceRequalification(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.registry = owner.m.read(ROOT / owner.m.REGISTRY)
        cls.c_log = (ROOT / owner.REPLAYS / 'm1-native-c-replay.log').read_bytes()
        cls.rust_log = (ROOT / owner.REPLAYS / 'native-rust-c-replay.log').read_bytes()
        cls.audit = owner.m.read(ROOT / owner.REPLAYS / 'm3-source-audit.json')

    def test_actual_current_executions_admit_the_source_application_only(self):
        owner.verify_replays(self.c_log, self.rust_log, self.audit, self.registry)
        receipt = owner.m.read(ROOT / owner.RECEIPT)
        ledger = owner.m.read(ROOT / owner.m.LEDGER)
        prior, _ = owner.history(receipt['historical']['git_commit'])
        self.assertEqual(sum(d['state'] == 'applied' for d in ledger['discrepancies']), 131)
        for old, current in zip(prior['discrepancies'], ledger['discrepancies']):
            restored = copy.deepcopy(current)
            for part in ('decision', 'proposal'):
                if old.get(part):
                    restored[part]['evidence'] = old[part]['evidence']
            self.assertEqual(restored, old)
        # A real fresh source observation cannot confer numerical/GPU readiness.
        for ident, assessment in ledger['assessments'].items():
            if ident == 'k2-index':
                continue
            self.assertTrue(all(c['status'] in ('unassessed', 'unimplemented') for c in assessment['readiness'].values()))
            self.assertTrue(all(not claims for claims in assessment['parity'].values()))
        old_by_id = {e['id']: e for e in prior['evidence']}
        current_by_id = {e['id']: e for e in ledger['evidence']}
        shared = old_by_id.keys() & current_by_id.keys()
        self.assertEqual(shared, {'k2-index-declaration'})
        # This is a new finite coordinate-index source observation, with a new
        # current manifest hash. It carries source-declared structure only.
        self.assertEqual(current_by_id['k2-index-declaration']['kind'], 'source')
        self.assertNotEqual(old_by_id['k2-index-declaration']['artifact']['sha256'], current_by_id['k2-index-declaration']['artifact']['sha256'])
        self.assertEqual(current_by_id['k2-index-declaration']['registry_revision'], self.registry['registry_revision'])

    def test_reintroduced_coin_source_error_is_refused_even_without_summary_findings(self):
        changed = copy.deepcopy(self.audit)
        # Preserve the successful summary, but disconnect one actual source
        # determinant from its ratified reading. No self-derived green allowed.
        changed['details']['genetics']['charges'][0]['source'] = 'disconnected-current-source'
        with self.assertRaisesRegex(ValueError, 'disagree with the ratified law'):
            owner.verify_replays(self.c_log, self.rust_log, changed, self.registry)

    def test_missing_actual_c_rust_consumer_cannot_be_replaced_by_green_metadata(self):
        removed = self.rust_log.replace(b'test actual_c_descriptors_equal_every_rust_manifest_record ... ok', b'consumer absent; summary remains green')
        self.assertNotEqual(removed, self.rust_log)
        with self.assertRaisesRegex(ValueError, 'actual current Rust/C replay is missing'):
            owner.verify_replays(self.c_log, removed, self.audit, self.registry)

    def test_old_source_receipt_does_not_qualify_the_current_registry(self):
        changed = copy.deepcopy(self.audit)
        changed['source_revision'] = 'c9d67075d7c25cb9038d5cfb9761f1237b38c433d65203e197d1cebcb07ab31d'
        with self.assertRaisesRegex(ValueError, 'not for the current admitted source'):
            owner.verify_replays(self.c_log, self.rust_log, changed, self.registry)

if __name__ == '__main__':
    unittest.main()
