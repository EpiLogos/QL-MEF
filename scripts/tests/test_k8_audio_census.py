"""Keep audio presentation evidence separate from historical native readiness."""
from copy import deepcopy
import hashlib
import importlib.util
from pathlib import Path
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location('audio_census_owner', ROOT / 'scripts/k8-census.py')
census = importlib.util.module_from_spec(spec)
spec.loader.exec_module(census)


class AudioCensus(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.inventory = census.project()

    def test_presentation_source_is_owned_without_new_domain_coordinates(self):
        modules = self.inventory['javascript_modules']
        module = next(item for item in modules if item['path'] == 'adapters/retained-field/native-audio.mjs')
        self.assertEqual(module['path'], 'adapters/retained-field/native-audio.mjs')
        self.assertEqual(module['coordinates'], [])
        self.assertEqual(module['disposition'], 'infrastructural')
        self.assertEqual(module['standing'], 'declared-current-module-not-runtime-parity')
        expected = hashlib.sha256((ROOT / module['path']).read_bytes()).hexdigest()
        self.assertEqual(module['sha256'], expected)
        self.assertEqual(self.inventory['sources'][module['path']], expected)

    def test_original_ledger_and_assessments_are_not_restamped(self):
        ledger = census.read('fixtures/kernel/m-ledger-v1.json')
        self.assertEqual(self.inventory['inherited_ledger_revision'], ledger['ledger_revision'])
        self.assertEqual(self.inventory['inherited_ledger_sha256'], census.sha((ROOT / 'fixtures/kernel/m-ledger-v1.json').read_bytes()))
        self.assertEqual(self.inventory['inherited_assessments_sha256'], census.sha(census.canonical(ledger['assessments'])))
        self.assertTrue(all(record['stratum'] != 'javascript' for record in self.inventory['constructs']))

    def test_wrong_owner_and_missing_test_are_refused_not_silently_inherited(self):
        original_read = census.read
        for bad in ['coordinate', 'test', 'stratum']:
            spec = deepcopy(original_read(census.BINDINGS))
            owner = next(module for module in spec['modules'] if module['stratum'] == 'javascript')
            if bad == 'coordinate':
                owner['coordinates'] = ['#1']
            elif bad == 'test':
                owner['tests'] = ['missing-audio-acceptance.py']
            else:
                owner['stratum'] = 'imagined-runtime'
            def read(path):
                return spec if path == census.BINDINGS else original_read(path)
            with self.subTest(bad=bad), patch.object(census, 'read', side_effect=read):
                with self.assertRaises(ValueError):
                    census.project()


if __name__ == '__main__':
    unittest.main()
