"""Complete K8 projection and refusal laws, independent of a database service."""
import copy
import importlib.util
import json
from pathlib import Path
import sys
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location('k8_graph', ROOT / 'scripts/k8-graph.py')
graph = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = graph
spec.loader.exec_module(graph)
PLAN = graph.projection()


class Memory:
    """Transaction specimen only; real Cypher is tested by test-k8-graph-live.py."""
    def __init__(self, plan=PLAN):
        self.state = {'target': {'endpoint': 'fixture', 'database': 'fixture', 'identity': 'database-1'},
                      'nodes': [{'element_id': str(i), 'labels': ['Bimba', 'Root'],
                                 'properties': {'coordinate': a['coordinate'], 'authored': ['one', 'two']}}
                                for i, a in enumerate(plan['anchors'])],
                      'relations': [], 'incident': [], 'markers': []}
        self.writes = 0
        self.locked = False

    def snapshot(self, _):
        return copy.deepcopy(self.state)

    def lock(self, _):
        self.locked = True

    def add(self, plan):
        assert self.locked
        self.writes += 1
        self.state['nodes'] += [dict(element_id='new-' + str(i), **{k: copy.deepcopy(n[k]) for k in ('labels', 'properties')})
                                for i, n in enumerate(plan['nodes'])]
        self.state['relations'] = [dict(element_id='edge-' + str(i), **copy.deepcopy(e)) for i, e in enumerate(plan['relations'])]
        self.state['incident'] = [{'element_id': e['element_id']} for e in self.state['relations']]
        self.state['markers'] = [{'element_id': 'marker', 'labels': ['QLStructuralPromotion'], 'properties': graph.marker(plan)}]

    def remove(self, plan):
        assert self.locked
        self.writes += 1
        self.state['nodes'] = [n for n in self.state['nodes'] if 'ql_k8_promotion' not in n['properties']]
        self.state['relations'] = []
        self.state['incident'] = []
        self.state['markers'] = []


class Projection(unittest.TestCase):
    def setUp(self):
        # Source determinism has its own test. Avoid re-running the full source
        # compiler for every state-machine mutation below.
        self.validate = patch.object(graph, 'validate_plan', side_effect=lambda p: graph.require(p == PLAN, 'changed plan'))
        self.validate.start()
        self.addCleanup(self.validate.stop)
        self.store = Memory()
        self.approved = graph.review(PLAN, self.store)

    def apply(self):
        graph.operate(PLAN, self.approved, self.store, 'apply')
        return graph.review(PLAN, self.store)

    def refused(self, approval=None, operation='apply'):
        before = self.store.snapshot(PLAN)
        writes = self.store.writes
        with self.assertRaises(ValueError):
            graph.operate(PLAN, approval or self.approved, self.store, operation)
        self.assertEqual(before, self.store.state)
        self.assertEqual(writes, self.store.writes)

    def test_source_deterministic_complete_projection(self):
        self.assertEqual(PLAN, graph.projection())
        self.assertEqual(len(PLAN['nodes']), 27)
        self.assertEqual(len(PLAN['relations']), 78)
        self.assertEqual(len({n['properties']['ql_k8_id'] for n in PLAN['nodes']}), 27)
        self.assertEqual(len({r['properties']['ql_k8_id'] for r in PLAN['relations']}), 78)
        self.assertEqual(next(a['coordinate'] for a in PLAN['anchors'] if a['ref'] == '#3-5-5/0'), 'M3-5-(5/0)')
        self.assertTrue(all('M3-0-' not in n['properties']['coordinate'] for n in PLAN['nodes']))
        for r in PLAN['relations']:
            self.assertEqual(json.loads(r['properties']['ql_k8_assertion_json'])['repository'], 'EpiLogos/QL-MEF')
        reciprocal = [r for r in PLAN['relations'] if r['type'] == 'STATIC_DIVISOR_RECIPROCAL']
        self.assertEqual(len(reciprocal), 8)
        self.assertEqual([json.loads(r['properties']['ql_k8_properties_json'])['native_indices'] for r in
                          sorted(reciprocal, key=lambda r: r['from_coordinate'])], [[i, 15-i] for i in range(8)])

    def test_apply_repeat_rollback_repeat_exact_old_state(self):
        original = self.store.snapshot(PLAN)
        rollback = self.apply()
        after = self.store.snapshot(PLAN)
        self.assertFalse(graph.operate(PLAN, self.approved, self.store, 'apply')['changed'])
        self.assertEqual(after, self.store.state)
        self.assertTrue(graph.operate(PLAN, rollback, self.store, 'rollback')['changed'])
        self.assertEqual(original, self.store.state)
        self.assertFalse(graph.operate(PLAN, rollback, self.store, 'rollback')['changed'])

    def test_omitted_source_allocation(self):
        plan = copy.deepcopy(PLAN)
        plan['nodes'].pop()
        with self.assertRaises(ValueError): graph.operate(plan, self.approved, self.store, 'apply')
        self.assertEqual(self.store.writes, 0)

    def test_unsafe_relation_kind(self):
        plan = copy.deepcopy(PLAN)
        plan['relations'][0]['type'] = 'X] DELETE n //'
        with self.assertRaises(ValueError): graph.operate(plan, self.approved, self.store, 'apply')
        self.assertEqual(self.store.writes, 0)

    def test_missing_anchor(self):
        self.store.state['nodes'].pop()
        self.refused()

    def test_duplicate_coordinate(self):
        row = copy.deepcopy(self.store.state['nodes'][0]); row['element_id'] = 'duplicate'
        self.store.state['nodes'].append(row)
        self.refused()

    def test_duplicate_canonical_alias(self):
        self.store.state['nodes'].append({'element_id': 'alias', 'labels': ['Bimba'], 'properties': {'coordinate': '#3-5-5/0'}})
        self.refused()

    def test_changed_anchor_after_review(self):
        self.store.state['nodes'][0]['properties']['authored'].append('third')
        self.refused()

    def test_changed_database(self):
        self.store.state['target']['identity'] = 'other-database'
        self.refused()

    def test_changed_endpoint(self):
        self.store.state['target']['endpoint'] = 'another-host'
        self.refused()

    def test_forged_review(self):
        bad = dict(self.approved, anchor_sha256='unreviewed')
        self.refused(bad)

    def test_partial_preexisting_promotion(self):
        self.store.state['nodes'].append({'element_id': 'existing', **{k: PLAN['nodes'][0][k] for k in ('labels', 'properties')}})
        self.refused()

    def test_source_amended_node_cannot_be_deleted(self):
        approved = self.apply()
        self.store.state['nodes'][-1]['properties']['human_reading'] = 'new authored work'
        self.refused(approved, 'rollback')

    def test_changed_label_cannot_be_deleted(self):
        approved = self.apply()
        self.store.state['nodes'][-1]['labels'].append('HumanQualified')
        self.refused(approved, 'rollback')

    def test_source_amended_relation_cannot_be_deleted(self):
        approved = self.apply()
        self.store.state['relations'][0]['properties']['meaning'] = 'another reading'
        self.refused(approved, 'rollback')

    def test_relation_direction_is_load_bearing(self):
        approved = self.apply()
        edge = self.store.state['relations'][0]
        edge['from_coordinate'], edge['to_coordinate'] = edge['to_coordinate'], edge['from_coordinate']
        self.refused(approved, 'rollback')

    def test_duplicate_relation_is_not_last_writer(self):
        approved = self.apply()
        edge = copy.deepcopy(self.store.state['relations'][0]); edge['element_id'] = 'duplicate-edge'
        self.store.state['relations'].append(edge)
        self.refused(approved, 'rollback')

    def test_missing_relation_is_partial(self):
        approved = self.apply()
        self.store.state['relations'].pop()
        self.refused(approved, 'rollback')

    def test_external_relation_blocks_rollback(self):
        approved = self.apply()
        self.store.state['incident'].append({'element_id': 'new-private-knowledge'})
        self.refused(approved, 'rollback')

    def test_marker_amendment_blocks_rollback(self):
        approved = self.apply()
        self.store.state['markers'][0]['properties']['plan_sha256'] = 'another-plan'
        self.refused(approved, 'rollback')

    def test_rollback_requires_current_review(self):
        self.apply()
        self.refused(operation='rollback')

    def test_unrepresentable_target_values_do_not_get_stringified(self):
        self.store.state['nodes'][0]['properties']['bad'] = float('nan')
        self.refused()

    def test_post_mutation_proof_is_required_before_commit(self):
        original = self.store.add
        def broken(plan):
            original(plan); self.store.state['relations'].pop()
        self.store.add = broken
        with self.assertRaisesRegex(ValueError, 'relation'):
            graph.operate(PLAN, self.approved, self.store, 'apply')
        # Actual transaction rollback (not a fictitious Memory guarantee) is
        # independently exercised with the real Neo4j adapter in native CI.

    def test_no_broad_delete_or_auto_retry(self):
        import inspect
        self.assertNotIn('DETACH DELETE', '\n'.join(line for line in inspect.getsource(graph.GraphTransaction.remove).splitlines() if not line.lstrip().startswith('#')))
        self.assertNotIn('execute_write', inspect.getsource(graph.GraphTransaction))


class Transport(unittest.TestCase):
    def test_url_and_credential_boundaries(self):
        for endpoint in ['http://example.org', 'https://a:b@example.org', 'https://example.org/path',
                         'https://example.org?token=abc', 'ftp://localhost', 'https://example.org/#secret']:
            with self.subTest(endpoint=endpoint), self.assertRaises(ValueError):
                graph.GraphHTTP(endpoint, 'neo4j', 'neo4j', 'controlled')
        for database in ['../system', 'a/b', 'bad name', '']:
            with self.assertRaises(ValueError): graph.GraphHTTP('http://localhost:7474', database, 'neo4j', 'controlled')
        self.assertIsInstance(graph.GraphHTTP('https://example.org', 'neo4j', 'reader', 'controlled'), graph.GraphHTTP)

    def test_begin_accepts_only_same_origin_commit_location(self):
        client = graph.GraphHTTP('http://localhost:7474', 'neo4j', 'neo4j', 'controlled')
        client.request = lambda *a: ({'commit': client.url + '/1/commit'}, None)
        self.assertEqual(client.begin().url, client.url + '/1')
        for url in ['https://evil/1/commit', client.url + '/01/../other/commit', client.url + '/1']:
            client.request = lambda *a: ({'commit': url}, None)
            with self.assertRaises(ValueError): client.begin()

    def test_unknown_commit_never_auto_rolls_back_or_retries(self):
        class Failed:
            def __init__(self): self.calls = []
            def request(self, *args):
                self.calls.append(args); raise RuntimeError('lost acknowledgement')
        client = Failed(); tx = graph.GraphTransaction(client, 'fixture/tx/1')
        with self.assertRaises(RuntimeError): tx.commit()
        tx.rollback()
        self.assertEqual(len(client.calls), 1)
        self.assertTrue(tx.commit_attempted)
        self.assertFalse(tx.closed)

    def test_precommit_rollback_is_explicit_once(self):
        class Client:
            def __init__(self): self.calls = []
            def request(self, *args): self.calls.append(args)
        client = Client(); tx = graph.GraphTransaction(client, 'fixture/tx/1')
        tx.rollback(); tx.rollback()
        self.assertEqual(client.calls, [('fixture/tx/1', 'DELETE')])


if __name__ == '__main__':
    unittest.main()
