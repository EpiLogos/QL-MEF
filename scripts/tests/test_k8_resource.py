"""The resource gate must not turn incomplete, slow or growing runs into passes."""
import importlib.util
from pathlib import Path
import unittest

path=Path(__file__).resolve().parents[1]/'test-k8-resource.py'
spec=importlib.util.spec_from_file_location('k8_resource',path)
m=importlib.util.module_from_spec(spec); spec.loader.exec_module(m)

class ResourceMeasurement(unittest.TestCase):
    def test_percentiles_keep_tails(self):
        result=m.summary(list(range(1,101)))
        self.assertEqual((result['p50'],result['p95'],result['p99'],result['maximum']),(50,95,99,100))

    def test_missed_deadline_does_not_change_resource_standing(self):
        result=m.assess([{'elapsed_ms':1,'rss_bytes':10},{'elapsed_ms':11,'rss_bytes':11}],deadline_ms=10,ceiling=20,growth=5)
        self.assertFalse(result['deadline_pass']); self.assertEqual(result['deadline_misses'],1)
        self.assertTrue(result['resource_pass'])

    def test_memory_growth_and_ceiling_are_independent(self):
        values=[{'elapsed_ms':1,'rss_bytes':10},{'elapsed_ms':2,'rss_bytes':19}]
        self.assertFalse(m.assess(values,deadline_ms=10,ceiling=20,growth=5)['resource_pass'])
        self.assertFalse(m.assess(values,deadline_ms=10,ceiling=18,growth=10)['resource_pass'])
        self.assertTrue(m.assess(values,deadline_ms=10,ceiling=20,growth=10)['resource_pass'])

    def test_missing_or_invalid_measurements_cannot_pass(self):
        for values in ([],[{'elapsed_ms':float('nan'),'rss_bytes':10}],[{'elapsed_ms':1,'rss_bytes':None}],
                       [{'elapsed_ms':-1,'rss_bytes':10}]):
            with self.assertRaises(ValueError): m.assess(values,deadline_ms=10,ceiling=20,growth=5)
        self.assertIsNone(m.summary([])['maximum'])

if __name__=='__main__': unittest.main()
