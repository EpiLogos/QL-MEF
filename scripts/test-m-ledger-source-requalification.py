#!/usr/bin/env python3
"""Challenge the current bootstrap with actual retained execution/source evidence."""
from __future__ import annotations
import argparse
import copy
import importlib.util
import json
import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('requalification', ROOT / 'scripts/requalify-m-ledger.py')
owner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(owner)

# The commit whose M ledger had every engine stripped (the defect this guards).
STRIPPED_LEDGER_COMMIT = 'db5c5325a5161c1113c4651d99ff5992b9b7c270'


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
        current = {d['id']: d for d in ledger['discrepancies']}
        for old in prior['discrepancies']:
            restored = copy.deepcopy(current[old['id']])
            for part in ('decision', 'proposal'):
                if old.get(part):
                    restored[part]['evidence'] = old[part]['evidence']
            self.assertEqual(restored, old)
        owner.verify_added_discrepancies(ROOT, prior, ledger, receipt['successor_verticals'])
        # A real fresh source observation cannot confer numerical/GPU readiness;
        # only a published successor proof at the current registry can, and only
        # inside that engine's own vertical.
        owner.verify_readiness(ROOT, receipt, ledger)
        for ident, assessment in ledger['assessments'].items():
            if ident.startswith(tuple(prefix for _, prefix in owner.ENGINE_OWNERS.values())) or ident == 'k2-index':
                continue
            self.assertTrue(all(c['status'] in ('unassessed', 'unimplemented') for c in assessment['readiness'].values()))
            self.assertTrue(all(not claims for claims in assessment['parity'].values()))
        old_by_id = {e['id']: e for e in prior['evidence']}
        current_by_id = {e['id']: e for e in ledger['evidence']}
        shared = old_by_id.keys() & current_by_id.keys()
        # The engines' own evidence ids pre-date the requalification; they stand
        # here only as the composed successor vertical, locking the current files.
        composed = tuple(owner.ENGINE_OWNERS[e][1] for e in receipt['successor_verticals'])
        self.assertEqual({i for i in shared if not i.startswith(composed)}, {'k2-index-declaration'})
        # This is a new finite coordinate-index source observation, with a new
        # current manifest hash. It carries source-declared structure only.
        self.assertEqual(current_by_id['k2-index-declaration']['kind'], 'source')
        self.assertNotEqual(old_by_id['k2-index-declaration']['artifact']['sha256'], current_by_id['k2-index-declaration']['artifact']['sha256'])
        self.assertEqual(current_by_id['k2-index-declaration']['registry_revision'], self.registry['registry_revision'])

    def test_committed_ledger_is_a_fixed_point_of_each_composed_owner(self):
        receipt = owner.m.read(ROOT / owner.RECEIPT)
        ledger = owner.m.read(ROOT / owner.m.LEDGER)
        self.assertEqual(set(receipt['successor_verticals']), {'m2', 'm3'})
        for engine in receipt['successor_verticals']:
            self.assertEqual(owner.load_owner(ROOT, engine).materialize(ledger), ledger, engine)

    def test_requalification_from_a_stripped_ledger_composes_the_published_successors(self):
        args = argparse.Namespace(previous_git_ref=owner.m.read(ROOT / owner.RECEIPT)['historical']['git_commit'],
                                  m1_c_log=ROOT / owner.REPLAYS / 'm1-native-c-replay.log',
                                  rust_log=ROOT / owner.REPLAYS / 'native-rust-c-replay.log',
                                  m3_audit=ROOT / owner.REPLAYS / 'm3-source-audit.json')
        # The ledger as the requalification first left it (every engine stripped,
        # on main before successors were composed), and the composed ledger itself:
        # both lead to the same fixed point.
        stripped = json.loads(subprocess.check_output(
            ['git', 'show', STRIPPED_LEDGER_COMMIT + ':' + owner.m.LEDGER.as_posix()], cwd=ROOT))
        self.assertFalse(any(e['id'].startswith('k6-m2:') and e['kind'] == 'test-receipt' for e in stripped['evidence']))
        for base in (stripped, owner.m.read(ROOT / owner.m.LEDGER)):
            ledger, receipt = owner.build(args, base=base, write_replays=False)
            self.assertEqual(set(receipt['successor_verticals']), {'m2', 'm3'})
            for engine in receipt['successor_verticals']:
                self.assertEqual(owner.load_owner(ROOT, engine).materialize(ledger), ledger, engine)
            self.assertEqual(owner.m.encode(ledger), (ROOT / owner.m.LEDGER).read_text())
            self.assertEqual(owner.m.encode(receipt), (ROOT / owner.RECEIPT).read_text())

    def test_readiness_outside_a_composed_vertical_is_refused(self):
        receipt = owner.m.read(ROOT / owner.RECEIPT)
        ledger = owner.m.read(ROOT / owner.m.LEDGER)
        for ident, assessment in ledger['assessments'].items():
            if ident.startswith('k5-m1:'):
                assessment['readiness']['rust'] = {'status': 'verified', 'warrant': 'tested', 'evidence': ['k6-m2:finite-proof']}
                break
        with self.assertRaisesRegex(ValueError, 'readiness without successor evidence'):
            owner.verify_readiness(ROOT, receipt, ledger)

    def test_tested_readiness_must_cite_the_successor_proof(self):
        receipt = owner.m.read(ROOT / owner.RECEIPT)
        ledger = owner.m.read(ROOT / owner.m.LEDGER)
        ledger['assessments']['k6-m2:finite']['readiness']['rust']['evidence'] = ['k6-m2:census']
        with self.assertRaisesRegex(ValueError, 'not backed by the successor proof'):
            owner.verify_readiness(ROOT, receipt, ledger)

    def test_a_receipt_that_composes_nothing_cannot_carry_successor_evidence(self):
        receipt = owner.m.read(ROOT / owner.RECEIPT)
        ledger = owner.m.read(ROOT / owner.m.LEDGER)
        receipt['successor_verticals'] = {}
        with self.assertRaisesRegex(ValueError, 'not a published successor'):
            owner.verify_readiness(ROOT, receipt, ledger)

    def test_an_engine_whose_proof_is_not_a_published_successor_stays_stripped(self):
        locks = owner.m.read(ROOT / owner.RECEIPT)['historical_numerical_proofs_unchanged']
        self.assertEqual(set(owner.successor_engines(ROOT, locks)), {'m2', 'm3'})
        # The active proof is the historical original: nothing was published.
        unchanged = {path: owner.sha((ROOT / path).read_bytes()) for path in locks}
        self.assertEqual(owner.successor_engines(ROOT, unchanged), {})
        # A lineage that no longer publishes the active proof does not qualify it.
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            for child in ROOT.iterdir():
                if child.name not in ('.git', 'target'):
                    (root / child.name).symlink_to(child, target_is_directory=child.is_dir())
            for name in ('fixtures',):
                (root / name).unlink()
                shutil.copytree(ROOT / name, root / name, symlinks=True)
            lineage = json.loads((root / owner.LINEAGE).read_text())
            lineage['proofs']['m3'] = '0' * 64
            (root / owner.LINEAGE).write_text(json.dumps(lineage))
            self.assertEqual(set(owner.successor_engines(root, locks)), {'m2'})

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
