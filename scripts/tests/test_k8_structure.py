"""Mutation and complete-source preservation checks, not a live graph receipt."""
import copy
import importlib.util
import json
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location('k8_structure', ROOT / 'scripts/k8-structure.py')
k8 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(k8)


class StructuralPromotion(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.base = k8.read(ROOT / 'fixtures/kernel/m-tree-v1.json')
        cls.promotion = k8.read(ROOT / k8.PROMOTION)
        cls.current, cls.receipt = k8.project()
        cls.nodes = {n['source_ref']: n for n in cls.current['nodes']}

    def test_every_original_node_and_relation_survives(self):
        for old in self.base['nodes']:
            new = self.nodes[old['source_ref']]
            for key, value in old.items():
                if key not in ('children', 'subtree_count'):
                    self.assertEqual(value, new[key], (old['source_ref'], key))
            self.assertTrue(set(old['children']).issubset(new['children']))
        relations = {r['id']: r for r in self.current['relations']}
        for old in self.base['relations']:
            self.assertEqual(old, relations[old['id']])
        self.assertEqual(self.base['records'], self.current['records'][:len(self.base['records'])])
        self.assertEqual(self.base['files'], self.current['files'][:len(self.base['files'])])
        self.assertEqual(self.base['bindings'], self.current['bindings'])

    def test_source_authority_is_not_attributed_to_historical_c_experiments(self):
        origin = self.current['files'][-1]
        self.assertEqual(origin['repository'], 'EpiLogos/QL-MEF')
        self.assertEqual(origin['revision'], self.promotion['authority']['revision'])
        self.assertNotEqual(origin['revision'], self.current['source_revision'])
        lines = (ROOT / k8.AUTHORITY_CAPTURE).read_text().splitlines()
        for record in self.current['records'][len(self.base['records']):]:
            line = record['record_index']
            self.assertEqual(record['file'], len(self.current['files']) - 1)
            self.assertEqual(record['payload_sha256'], k8.G.digest({'line': line+1, 'text': lines[line]}))

    def test_twenty_seven_nodes_eighteen_lenses_eight_pairs(self):
        self.assertEqual(len(self.current['nodes']) - len(self.base['nodes']), 27)
        self.assertEqual(len(self.receipt['static_apertures']), 16)
        self.assertEqual(len(self.receipt['additional_apertures']), 2)
        self.assertEqual(len(self.nodes['#2-0-2']['children']), 8)
        for i in range(8):
            pair = self.nodes[f'#2-0-2-{i}']
            self.assertEqual(len(pair['children']), 2)
            for side, native in enumerate((i, 15-i)):
                aperture = self.receipt['static_apertures'][native]
                self.assertEqual(aperture['source_ref'], f'#2-0-2-{i}-{side}')
                self.assertEqual(aperture['native_index'], native)
                self.assertEqual(aperture['division_degrees'] * aperture['segments'], 360)
                self.assertNotEqual(aperture['id'], pair['id'])

    def test_centre_keeps_every_original_degree_child_and_slash(self):
        old = next(n for n in self.base['nodes'] if n['source_ref'] == '#3-5-5/0')
        self.assertEqual(old, self.nodes['#3-5-5/0'])
        self.assertEqual(old['id'], 'f0c7dcb383d337ff')
        self.assertEqual(len(old['children']), 360)
        self.assertNotIn('#3-5-5-0', self.nodes)
        realisations = [r for r in self.current['relations'] if r['source_kind'] == 'REALISED_AT']
        self.assertTrue(any(r['from_ref'] == '#3-0' and r['to_ref'] == '#3-5-5/0' for r in realisations))

    def test_exact_grids_are_distinct_from_lens_catalogues(self):
        self.assertEqual(self.receipt['closures_half_degrees'], [120, 180, 360])
        self.assertEqual([g['positions'] for g in self.receipt['grids']], [60, 18, 16])
        for a in self.receipt['static_apertures']:
            b = self.receipt['static_apertures'][a['reciprocal_native_index']]
            self.assertEqual(b['reciprocal_native_index'], a['native_index'])
            self.assertEqual(a['division_degrees'] * b['division_degrees'], 360)
            self.assertNotEqual(a['reciprocal_native_index'], (a['native_index'] + 8) % 16)

    def test_reproducible_current_projection_and_historical_view(self):
        current, receipt = k8.project()
        self.assertEqual(current, self.current)
        self.assertEqual(receipt, self.receipt)
        self.assertEqual(k8.receipt_summary(receipt), k8.read(ROOT / k8.RECEIPT))
        self.assertEqual(k8.G.model(k8.read(ROOT / k8.G.SNAPSHOT)), self.base)
        no_revision = {k:v for k,v in current.items() if k != 'registry_revision'}
        self.assertEqual(k8.G.digest(no_revision), current['registry_revision'])

    def reject(self, mutation):
        promotion = copy.deepcopy(self.promotion)
        mutation(promotion)
        with self.assertRaises(ValueError):
            k8.validate(promotion, self.base, ROOT)

    def test_centre_alias_and_native_antipode_mutations_rejected(self):
        self.reject(lambda p: p.update(centre='#3-5-360'))
        self.reject(lambda p: p['pairs'][0].update(native_indices=[0, 8]))
        self.reject(lambda p: p['pairs'][0].update(division_degrees=[3, 120]))
        self.reject(lambda p: p['pairs'].pop())

    def test_grid_snap_and_count_mutations_rejected(self):
        self.reject(lambda p: p['grids'][2].update(half_degree_quantum=44))
        self.reject(lambda p: p['grids'][1].update(positions=16))
        self.reject(lambda p: p.update(closures_half_degrees=[120, 180, 720]))
        self.reject(lambda p: p.update(void_elemental_ratio=[8, 9]))

    def test_forged_field_relation_or_authority_is_rejected(self):
        self.reject(lambda p: p['field_relations'][0].update(to='#3-0'))
        self.reject(lambda p: p['field_relations'].pop())
        self.reject(lambda p: p['authority'].update(standing='historical-source'))
        self.reject(lambda p: p['authority'].update(revision='main'))

    def test_restamped_source_or_fake_graph_receipt_rejected(self):
        self.reject(lambda p: p['authority'].update(sha256='0'*64))
        self.reject(lambda p: p.update(base_registry_revision='0'*64))
        self.reject(lambda p: p['authority'].update(path='../../etc/passwd'))
        self.reject(lambda p: p['preservation'].update(live_graph_application='applied'))


if __name__ == '__main__':
    unittest.main()
