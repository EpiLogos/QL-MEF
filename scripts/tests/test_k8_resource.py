"""The resource gate must not turn incomplete, slow or growing runs into passes."""
import copy
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

comparison_path=path.parent/'compare-k8-resource.py'
comparison_spec=importlib.util.spec_from_file_location('k8_compare',comparison_path)
comparison=importlib.util.module_from_spec(comparison_spec); comparison_spec.loader.exec_module(comparison)

class BuildComparison(unittest.TestCase):
    def specimens(self):
        conditions=dict(policy={'cycles':1}, exact_source='sha', tree='tree', source_input_sha256='input',
            worker_sha256='worker', platform='linux', cpu_model='cpu', cpu_count=2, python='python',
            mode_count=16, sample_count=256, input_schema='v2', host_sha256='debug', host_build_profile='debug')
        baseline=dict(schema='ql.k8-native-resource-acceptance/v1', measurement_valid=True, resource_pass=True,
            exact_original_replay=True, conditions=conditions, cycles=[dict(host=dict(latency_ms={'p95':50},
                rss_max_bytes=100, deadline_misses=0),source_operation_ms={'maximum':1500})],
            native_deadline_pass=True, source_operation_budget_pass=False)
        candidate=copy.deepcopy(baseline)
        candidate['conditions'].update(host_sha256='release',host_build_profile='release',replay_host_sha256='debug')
        candidate['cross_build_replay']=True
        candidate['cycles'][0]['source_operation_ms']['maximum']=200
        candidate['source_operation_budget_pass']=True
        return baseline,candidate

    def test_same_inputs_and_replay_required(self):
        a,b=self.specimens()
        self.assertTrue(comparison.compare(a,b)['candidate_runtime_budget_pass'])
        for key in ('policy','source_input_sha256','worker_sha256','exact_source','replay_host_sha256'):
            changed=copy.deepcopy(b); changed['conditions'][key]='changed'
            with self.assertRaises(ValueError): comparison.compare(a,changed)

    def test_unmeasured_or_omitted_work_cannot_improve(self):
        for change in ('no-replay','missing-cycle','invalid-timing'):
            a,b=self.specimens()
            if change=='no-replay': b['cross_build_replay']=False
            elif change=='missing-cycle': b['cycles']=[]
            else: b['cycles'][0]['source_operation_ms']['maximum']=float('nan')
            with self.assertRaises(ValueError): comparison.compare(a,b)

    def test_budget_failure_survives_better_median(self):
        a,b=self.specimens(); b['source_operation_budget_pass']=False
        result=comparison.compare(a,b)
        self.assertLess(result['rows'][1]['candidate_to_baseline_ratio'],1)
        self.assertFalse(result['candidate_runtime_budget_pass'])

if __name__=='__main__': unittest.main()
