import importlib.util
import json
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location('m3_generator', ROOT / 'scripts/generate-m3.py')
M = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(M)

class M3SourceTests(unittest.TestCase):
    def test_source_groups_preserve_all_clock_nodes_and_actual_edges(self):
        registry = json.loads((ROOT / M.REGISTRY).read_text())
        groups, anchors = M.source_groups(registry)
        self.assertEqual([len(g) for g in groups], [1,4,16,64,8,64,3,56,22,360,24,1])
        self.assertEqual(len(set(groups[9]+groups[10])), 384)
        self.assertEqual(groups[9][0], '#3-5-5/0-0/360')
        self.assertEqual(len(set(anchors)), 24)
        self.assertTrue(all(anchors.count(a)==15 for a in set(anchors)))
        self.assertEqual(groups[7][0], '#3-4-2-0')
        self.assertEqual(groups[7][14], '#3-4-1-0')
        self.assertEqual(groups[5][0], '#3-1-1-1')
        self.assertEqual(groups[5][63], '#3-1-0-0')
        self.assertEqual(groups[5][1], '#3-1-2-1')
        self.assertEqual(groups[5][8], '#3-1-1-2')

    def test_hexagram_bindings_reject_independent_relation_mutation(self):
        registry = json.loads((ROOT / M.REGISTRY).read_text())
        registry['relations'].append({'source_kind':'HAS_UPPER_Trigram',
            'from_ref':'#3-1-2-1', 'to_ref':'#3-1-0'})
        with self.assertRaisesRegex(ValueError,'ambiguous'):
            M.source_groups(registry)

    def test_relation_disagreement_fails_instead_of_repairing_source(self):
        registry = json.loads((ROOT / M.REGISTRY).read_text())
        registry['relations'].append({'source_kind':'ANCHORED_BY', 'from_ref':'#3-5-5/0-1', 'to_ref':'#3-5-2-0'})
        with self.assertRaisesRegex(ValueError,'ambiguous'):
            M.source_groups(registry)

    def test_clock_parser_is_exact_and_rejects_duplicate_fraction_and_width(self):
        source = (ROOT / M.CLOCK).read_text()
        rows=M.clock_rows(source)
        self.assertEqual(len(rows),360)
        for d,row in enumerate(rows):
            self.assertEqual(row[0],d)
            self.assertEqual(row[1],2*d)
            self.assertEqual(row[22],d+360)
            self.assertEqual(row[23],(d+180)%360)
        for old,new in [('{   1U,','{0U,'),('{   1U,','{1.5f,'),('{   1U,','{1U,2U,')]:
            self.assertTrue(old in source, "mutation marker missing")
            with self.assertRaises(ValueError):
                M.clock_rows(source.replace(old,new))

    def test_generator_is_deterministic(self):
        self.assertEqual(M.generate(),M.generate())

if __name__ == '__main__':
    unittest.main()
