"""New source cannot disappear behind the historical census receipt."""
import copy
import importlib.util
from pathlib import Path
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location('k8_census', ROOT / 'scripts/k8-census.py')
k8 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(k8)


class CurrentInventory(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.baseline = k8.project()
        cls.discovered = k8.census.scan_constructs()

    def test_complete_inventory_has_linked_current_exports(self):
        self.assertGreater(len(self.baseline['constructs']), 1400)
        symbols = {i['symbol'] for i in self.baseline['native_exports']}
        self.assertIn('ql_clock_advance', symbols)
        self.assertIn('ql_m2_aperture_at', symbols)
        self.assertIn('ql_m_live_resolve', symbols)
        self.assertEqual(self.baseline['inherited_ledger_revision'], k8.read('fixtures/kernel/m-ledger-v1.json')['ledger_revision'])
        self.assertTrue(any(i['standing'] == 'inherited-unresolved' for i in self.baseline['constructs']))

    def test_new_unowned_construct_is_not_inherited_as_an_old_orphan(self):
        discovered = copy.deepcopy(self.discovered)
        discovered['c'].append({'path': 'c/src/m2.c', 'symbol': 'new_unowned_operation', 'kind': 'function', 'line': 1})
        with patch.object(k8.census, 'scan_constructs', return_value=discovered):
            with self.assertRaisesRegex(ValueError, 'no reviewed owner'):
                k8.project()

    def test_wrong_coordinate_or_missing_test_cannot_be_a_disposition(self):
        original = k8.read
        for field, value in [('coordinates', ['#2-999999']), ('tests', ['not/a/real/test'])]:
            with self.subTest(field=field):
                def read(path):
                    result = original(path)
                    if path == k8.BINDINGS:
                        result['modules'][0][field] = value
                    return result
                with patch.object(k8, 'read', side_effect=read):
                    with self.assertRaises(ValueError):
                        k8.project()

    def test_current_inventory_does_not_mutate_any_historical_census_or_ledger(self):
        paths = list((ROOT / 'fixtures/kernel/census').rglob('*.json')) + [ROOT / 'fixtures/kernel/m-ledger-v1.json']
        before = {p: k8.sha(p.read_bytes()) for p in paths}
        k8.project()
        self.assertEqual(before, {p: k8.sha(p.read_bytes()) for p in paths})


if __name__ == '__main__':
    unittest.main()
