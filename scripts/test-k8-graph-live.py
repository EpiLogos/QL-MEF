#!/usr/bin/env python3
"""Actual Cypher acceptance on an explicitly disposable, initially empty Neo4j.

Refuses to seed any nonempty target. Never called against owner credentials by
CI. All baseline graph contents are controlled source-addressed witnesses, not
a claimed copy of the owner's live Bimba/property/Personal world.
"""
from concurrent.futures import ThreadPoolExecutor
import importlib.util
import json
import os
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('k8_graph', ROOT / 'scripts/k8-graph.py')
g = importlib.util.module_from_spec(spec); spec.loader.exec_module(g)


def main():
    g.require(os.environ.get('QL_K8_DISPOSABLE_GRAPH') == 'yes-this-is-an-empty-test-database',
              'explicit disposable graph opt-in required')
    plan = g.projection()
    client = g.GraphHTTP(os.environ.get('QL_K8_GRAPH_ENDPOINT', 'http://localhost:7474'), 'neo4j', 'neo4j',
                        os.environ.get('QL_K8_GRAPH_PASSWORD', ''))
    out = ROOT / 'target/k8-graph'; out.mkdir(parents=True, exist_ok=True)

    def transact(fn):
        tx = client.begin()
        try:
            value = fn(tx); tx.commit(); return value
        finally:
            tx.rollback()

    def query(statement, **kwargs):
        return transact(lambda tx: tx.query(statement, **kwargs))

    g.require(query('MATCH (n) RETURN count(n) AS count')[0]['count'] == 0, 'live test target is not empty; refusing all writes')
    components = query('CALL dbms.components() YIELD name, versions, edition RETURN name, versions, edition')
    # Deliberate schema setup is outside the migration transaction, exactly as
    # production operator provisioning. A missing schema is a tested refusal.
    for statement in g.SCHEMA:
        query(statement, lock=g.LOCK)
    baseline_nodes = [{'coordinate': a['coordinate'], 'authored': 'controlled anchor',
                       'source_ref': a['ref'], 'source_id': a['id']} for a in plan['anchors']]
    base = json.loads((ROOT / 'fixtures/kernel/m-tree-v1.json').read_text())
    capture = json.loads((ROOT / 'fixtures/kernel/census/live-bimba-capture-v1.json').read_text())
    by_id = {n['id']: n for n in base['nodes']}
    centre = next(n for n in base['nodes'] if n['source_ref'] == '#3-5-5/0')
    for child_id in centre['children']:
        child = by_id[child_id]
        baseline_nodes.append({'coordinate': capture['joined'][child['source_ref']],
                               'authored': 'controlled degree', 'source_ref': child['source_ref'], 'source_id': child_id})
    g.require(len(centre['children']) == 360, 'incomplete native centre field')
    query('UNWIND $nodes AS props CREATE (n:Bimba) SET n = props RETURN count(n) AS count', nodes=baseline_nodes)
    query('MATCH (a:Bimba {coordinate:$centre}), (b:Bimba) WHERE b.authored = $kind '
          'CREATE (a)-[:CONTROLLED_NATIVE_DEGREE {standing:$standing}]->(b) RETURN count(b) AS count',
          centre='M3-5-(5/0)', kind='controlled degree', standing='source-addressed controlled witness')
    # Pre-existing plural assertions, including the same directed type as a
    # promoted relation, must survive both application and undo byte-for-byte.
    for statement in ('first', 'second'):
        query('MATCH (a:Bimba {coordinate:$a}), (b:Bimba {coordinate:$b}) '
              'CREATE (a)-[:REALISED_AT {assertion:$assertion, standing:$standing}]->(b) RETURN count(a) AS count',
              a='M3-0', b='M3-5-(5/0)', assertion=statement, standing='retained independent source assertion')

    def all_graph():
        return {'nodes': query('MATCH (n) RETURN elementId(n) AS id, labels(n) AS labels, properties(n) AS properties ORDER BY id'),
                'relations': query('MATCH (a)-[r]->(b) RETURN elementId(r) AS id, elementId(a) AS source, '
                                   'elementId(b) AS target, type(r) AS type, properties(r) AS properties ORDER BY id')}

    original = all_graph()
    original_digest = g.digest(original)
    approved = transact(lambda tx: g.review(plan, tx))
    checks, receipts = [], []

    def refuse(fn, name):
        before = all_graph()
        try:
            fn()
        except (ValueError, RuntimeError):
            pass
        else:
            raise AssertionError('expected refusal: ' + name)
        g.require(all_graph() == before, 'refusal mutated graph: ' + name)
        checks.append(name)

    # Same review admitted in two separate concurrent transactions. The shared
    # native integration lock allows exactly one creation and one exact no-op.
    with ThreadPoolExecutor(max_workers=2) as pool:
        futures = [pool.submit(transact, lambda tx: g.operate(plan, approved, tx, 'apply')) for _ in range(2)]
        results = [f.result() for f in futures]
    g.require(sorted(r['changed'] for r in results) == [False, True], 'concurrent apply duplicated/lost structure')
    receipts += results; checks.append('concurrent application exactly once')
    current = transact(lambda tx: tx.snapshot(plan))
    g.require(g.inspect_state(plan, current) == 'applied', 'missing graph postcondition')
    rollback_review = transact(lambda tx: g.review(plan, tx))
    refuse(lambda: transact(lambda tx: g.operate(plan, approved, tx, 'rollback')), 'stale review refuses rollback')
    for _ in range(3):
        receipts.append(transact(lambda tx: g.operate(plan, approved, tx, 'apply')))
    checks.append('repeated application is exact no-op')
    after = all_graph()
    old_node_ids = {n['id'] for n in original['nodes']}
    old_edge_ids = {r['id'] for r in original['relations']}
    g.require([n for n in after['nodes'] if n['id'] in old_node_ids] == original['nodes'], 'old source node mutation')
    g.require([r for r in after['relations'] if r['id'] in old_edge_ids] == original['relations'], 'old relation assertion mutation')
    checks.append('all 360 degree children and plural assertions preserved')

    new_ref = plan['nodes'][0]['properties']['coordinate']
    query('MATCH (n:Bimba {coordinate:$ref}) SET n.reviewed_new_knowledge = $value RETURN count(n) AS count',
          ref=new_ref, value='do not delete')
    refuse(lambda: transact(lambda tx: g.operate(plan, rollback_review, tx, 'rollback')), 'amended node prevents rollback')
    query('MATCH (n:Bimba {coordinate:$ref}) REMOVE n.reviewed_new_knowledge RETURN count(n) AS count', ref=new_ref)
    query('MATCH (n:Bimba {coordinate:$ref}) CREATE (n)-[:CONTROLLED_EXTERNAL]->(:ControlledOther {note:$note}) '
          'RETURN count(n) AS count', ref=new_ref, note='no coordinate on this endpoint')
    refuse(lambda: transact(lambda tx: g.operate(plan, rollback_review, tx, 'rollback')), 'external uncoordinated relation prevents rollback')
    query('MATCH ()-[r:CONTROLLED_EXTERNAL]->(n:ControlledOther) DELETE r, n RETURN count(r) AS count')
    receipts.append(transact(lambda tx: g.operate(plan, rollback_review, tx, 'rollback')))
    g.require(all_graph() == original, 'rollback did not exactly restore the baseline')
    receipts.append(transact(lambda tx: g.operate(plan, rollback_review, tx, 'rollback')))
    checks.append('rollback and repeated rollback restore exact baseline')

    query('MATCH (n:Bimba {coordinate:$ref}) SET n.changed_after_review = true RETURN count(n) AS count', ref='M3-0')
    refuse(lambda: transact(lambda tx: g.operate(plan, approved, tx, 'apply')), 'stale anchor source refuses apply')
    query('MATCH (n:Bimba {coordinate:$ref}) REMOVE n.changed_after_review RETURN count(n) AS count', ref='M3-0')
    query('CREATE (:Bimba {coordinate:$ref, foreign_source:$source})', ref=new_ref, source='independent author')
    refuse(lambda: transact(lambda tx: g.operate(plan, approved, tx, 'apply')), 'preexisting foreign coordinate refuses apply')
    query('MATCH (n:Bimba {coordinate:$ref}) DELETE n', ref=new_ref)
    query('CREATE (:Bimba {coordinate:$ref})', ref='#3-5-5/0')
    refuse(lambda: transact(lambda tx: g.operate(plan, approved, tx, 'apply')), 'parallel centre alias refuses apply')
    query('MATCH (n:Bimba {coordinate:$ref}) DELETE n', ref='#3-5-5/0')

    def fail_after_create(tx):
        tx.lock(plan)
        tx.add(plan)
        raise RuntimeError('controlled post-write failure before commit')
    refuse(lambda: transact(fail_after_create), 'actual transaction rolls back partial writes')
    query('DROP CONSTRAINT ql_k8_lock')
    refuse(lambda: transact(lambda tx: g.operate(plan, approved, tx, 'apply')), 'missing lock constraint refuses apply')
    query(g.SCHEMA[1])
    g.require(all_graph() == original, 'final baseline mismatch')
    result = {'schema': 'ql.k8-graph-acceptance/v1', 'plan_sha256': plan['plan_sha256'],
              'components': components, 'checks': checks, 'operations': receipts,
              'promoted_nodes': len(plan['nodes']), 'promoted_relations': len(plan['relations']),
              'retained_degree_children': 360, 'baseline_sha256': original_digest,
              'restored_sha256': g.digest(all_graph()),
              'standing': 'actual disposable Neo4j transactions, not owner live graph application'}
    for name, data in [('acceptance', result), ('plan', plan), ('review', approved), ('applied-review', rollback_review), ('original', original)]:
        (out / (name + '.json')).write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n')
    print(json.dumps(result))


if __name__ == '__main__':
    main()
