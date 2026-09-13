#!/usr/bin/env python3
"""Reviewed K8 graph projection through the existing source compiler.

No default endpoint, credential, automatic schema change or production write.
Planning is offline. Review reads a bounded target. Apply/rollback require that
exact review and explicit confirmation of the compiled plan. The HTTP client
extends the existing census transport with an explicit, non-retrying transaction.
"""
from __future__ import annotations

import argparse
import base64
import copy
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import sys
from urllib.error import HTTPError, URLError
from urllib.parse import urlsplit
from urllib.request import Request, build_opener, HTTPRedirectHandler

ROOT = Path(__file__).resolve().parents[1]
LOCK = 'QL-MEF/shared-structural-integration/v1'
MAX_RESPONSE = 16 * 1024 * 1024
MAX_ROWS = 10000
SCHEMA = [
    'CREATE CONSTRAINT ql_k8_coordinate IF NOT EXISTS FOR (n:Bimba) REQUIRE n.coordinate IS UNIQUE',
    'CREATE CONSTRAINT ql_k8_lock IF NOT EXISTS FOR (n:QLStructuralPromotionLock) REQUIRE n.id IS UNIQUE',
    'CREATE CONSTRAINT ql_k8_marker IF NOT EXISTS FOR (n:QLStructuralPromotion) REQUIRE n.id IS UNIQUE',
    'MERGE (:QLStructuralPromotionLock {id: $lock})',
]


def require(ok, message):
    if not ok:
        raise ValueError(message)


def canonical(value):
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(',', ':'), allow_nan=False)


def digest(value):
    return hashlib.sha256(canonical(value).encode()).hexdigest()


def load_structure():
    spec = importlib.util.spec_from_file_location('k8_structure', ROOT / 'scripts/k8-structure.py')
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def projection():
    """Pure derived adapter, not an editable second registry or a readiness claim."""
    structure = load_structure()
    current, receipt = structure.project()
    base = structure.read(ROOT / 'fixtures/kernel/m-tree-v1.json')
    capture_path = ROOT / 'fixtures/kernel/census/live-bimba-capture-v1.json'
    capture = structure.read(capture_path)
    before_nodes = {n['id'] for n in base['nodes']}
    before_edges = {r['id'] for r in base['relations']}
    added = [n for n in current['nodes'] if n['id'] not in before_nodes]
    relations = [r for r in current['relations'] if r['id'] not in before_edges]
    by_ref = {n['source_ref']: n for n in current['nodes']}
    new_refs = {n['source_ref'] for n in added}
    anchors = sorted({r[k] for r in relations for k in ('from_ref', 'to_ref')} - new_refs)
    # Captured joins preserve parentheses/slash spelling; do not manufacture an
    # alias for the actual M3-5-(5/0) graph centre or reparent its degree children.
    bindings = {ref: capture['joined'][ref] for ref in anchors}
    require(bindings['#3-5-5/0'] == 'M3-5-(5/0)', 'captured centre binding drift')
    for ref in new_refs:
        require(re.fullmatch(r'#2-0-[012](?:-[0-7](?:-[01])?)?', ref), 'unreviewed graph allocation')
        bindings[ref] = 'M' + ref[1:]
    origin = receipt['authority']

    def source(record):
        r = current['records'][record]
        f = current['files'][r['file']]
        return {'repository': f['repository'], 'revision': f['revision'], 'path': f['path'],
                'git_blob': f['git_blob'], 'sha256': f['sha256'], 'line': r['record_index'] + 1,
                'record': record, 'payload_sha256': r['payload_sha256'], 'property_keys': r['property_keys']}

    nodes = []
    for node in sorted(added, key=lambda n: n['source_ref']):
        refs = [source(i) for i in node['records']]
        props = {'coordinate': bindings[node['source_ref']], 'name': node['names'][0],
                 'ql_k8_id': node['id'], 'ql_k8_source_ref': node['source_ref'],
                 'ql_k8_promotion': receipt['promotion_id'],
                 'ql_k8_registry_revision': current['registry_revision'],
                 'ql_k8_parent_id': node['parent_id'], 'ql_k8_assertions_json': canonical(refs),
                 'ql_k8_standing': origin['standing']}
        nodes.append({'ref': node['source_ref'], 'labels': ['Bimba', 'Coordinate', 'QLK8PromotionNode'],
                      'properties': props})
    edges = []
    for rel in sorted(relations, key=lambda r: r['id']):
        require(re.fullmatch('[A-Z][A-Z0-9_]*', rel['source_kind']), 'unsafe relation type')
        props = {'ql_k8_id': rel['id'], 'ql_k8_relation_ref': rel['relation_ref'],
                 'ql_k8_promotion': receipt['promotion_id'],
                 'ql_k8_registry_revision': current['registry_revision'],
                 'ql_k8_from_id': rel['from_id'], 'ql_k8_to_id': rel['to_id'],
                 'ql_k8_orientation': rel['orientation'],
                 'ql_k8_assertion_json': canonical(source(rel['record'])),
                 'ql_k8_properties_json': canonical(receipt['relation_properties'][rel['id']]),
                 'ql_k8_standing': origin['standing']}
        edges.append({'from_coordinate': bindings[rel['from_ref']],
                      'to_coordinate': bindings[rel['to_ref']], 'type': rel['source_kind'],
                      'properties': props})
    plan = {'schema': 'ql.k8-graph-plan/v1', 'promotion_id': receipt['promotion_id'],
            'registry_revision': current['registry_revision'], 'base_registry_revision': base['registry_revision'],
            'authority': origin, 'source_promotion_sha256': receipt['promotion_sha256'],
            'captured_join_sha256': hashlib.sha256(capture_path.read_bytes()).hexdigest(),
            'anchors': [{'ref': ref, 'id': by_ref[ref]['id'], 'coordinate': bindings[ref]} for ref in anchors],
            'nodes': nodes, 'relations': edges,
            'standing': 'reviewed source projection; pending owner live graph application'}
    require(len(nodes) == 27 and len(edges) == 78, 'incomplete K8 graph projection')
    plan['plan_sha256'] = digest(plan)
    return plan


def validate_plan(plan):
    require(plan == projection(), 'plan differs from the current reviewed source projection')


def marker(plan):
    return {'id': plan['promotion_id'], 'plan_sha256': plan['plan_sha256'],
            'registry_revision': plan['registry_revision'], 'standing': 'structural-application-only'}


def coordinates(plan):
    return sorted({a['coordinate'] for a in plan['anchors']} |
                  {n['properties']['coordinate'] for n in plan['nodes']})


def candidates(plan):
    # Detect parallel canonical-source and M-prefixed aliases instead of silently
    # selecting one. Actual centre binding is from the captured native join.
    return sorted(set(coordinates(plan)) | {a['ref'] for a in plan['anchors']} |
                  {n['ref'] for n in plan['nodes']} | {'M3-5-5/0'})


def node_value(row):
    return {'labels': sorted(row['labels']), 'properties': row['properties']}


def edge_value(row):
    return {k: row[k] for k in ('from_coordinate', 'to_coordinate', 'type', 'properties')}


def inspect_state(plan, state):
    """Reject ambiguous, partial, foreign and amended objects; never repair them."""
    require(set(state) == {'target', 'nodes', 'relations', 'incident', 'markers'}, 'invalid target snapshot')
    require(all(len(state[k]) <= MAX_ROWS for k in ('nodes', 'relations', 'incident', 'markers')),
            'target scope exceeds bounded review')
    require(len({n['element_id'] for n in state['nodes']}) == len(state['nodes']), 'duplicate node witness')
    by_coordinate = {}
    for node in state['nodes']:
        ref = node['properties'].get('coordinate')
        require(ref in coordinates(plan) and ref not in by_coordinate, 'ambiguous/foreign graph coordinate')
        by_coordinate[ref] = node
    for anchor in plan['anchors']:
        node = by_coordinate.get(anchor['coordinate'])
        require(node is not None and 'Bimba' in node['labels'], 'missing/non-Bimba exact graph anchor')
    actual_new = {k: v for k, v in by_coordinate.items() if k not in {a['coordinate'] for a in plan['anchors']}}
    if not actual_new and not state['relations'] and not state['markers']:
        require(not state['incident'], 'orphaned graph links')
        return 'absent'
    require(len(actual_new) == len(plan['nodes']), 'partial/conflicting promoted node set')
    for node in plan['nodes']:
        row = actual_new.get(node['properties']['coordinate'])
        require(row is not None and node_value(row) == {k: node[k] for k in ('labels', 'properties')},
                'promoted node was amended or belongs to another source')
    require(len(state['markers']) == 1 and node_value(state['markers'][0]) ==
            {'labels': ['QLStructuralPromotion'], 'properties': marker(plan)}, 'missing/conflicting application marker')
    expected = {e['properties']['ql_k8_id']: e for e in plan['relations']}
    require(len(state['relations']) == len(expected), 'partial/duplicate/conflicting relation set')
    observed = set()
    for row in state['relations']:
        ident = row['properties'].get('ql_k8_id')
        require(ident in expected and ident not in observed and edge_value(row) == expected[ident],
                'relation identity, direction, properties or source changed')
        observed.add(ident)
    # No DETACH DELETE: added external knowledge must be preserved. Even an
    # uncoordinated relation with no coordinate on its other endpoint blocks undo.
    owned_element_ids = {r['element_id'] for r in state['relations']}
    require(all(r['element_id'] in owned_element_ids for r in state['incident']),
            'external relation attaches to promoted state; rollback/repair requires review')
    return 'applied'


def anchor_digest(plan, state):
    refs = {a['coordinate'] for a in plan['anchors']}
    return digest([n for n in state['nodes'] if n['properties'].get('coordinate') in refs])


def review(plan, store):
    validate_plan(plan)
    state = store.snapshot(plan)
    status = inspect_state(plan, state)
    result = {'schema': 'ql.k8-graph-review/v1', 'plan_sha256': plan['plan_sha256'],
              'target': state['target'], 'anchor_sha256': anchor_digest(plan, state),
              'state_sha256': digest(state), 'observed': status,
              'standing': 'target observation only; no write authority granted'}
    result['review_sha256'] = digest(result)
    return result


def operate(plan, approved, tx, operation):
    """One transaction; caller commits only after this complete pre/post proof."""
    validate_plan(plan)
    require(operation in ('apply', 'rollback'), 'unknown graph operation')
    require(approved.get('schema') == 'ql.k8-graph-review/v1' and
            approved.get('plan_sha256') == plan['plan_sha256'], 'review belongs to another plan')
    check = dict(approved)
    supplied = check.pop('review_sha256', None)
    require(supplied == digest(check), 'review integrity mismatch')
    tx.lock(plan)
    before = tx.snapshot(plan)
    status = inspect_state(plan, before)
    require(approved['target'] == before['target'], 'review is for another graph/database')
    require(approved['anchor_sha256'] == anchor_digest(plan, before), 'reviewed source anchor changed')
    wanted = 'applied' if operation == 'apply' else 'absent'
    if status == wanted:
        return {'operation': operation, 'changed': False, 'result': wanted, 'before_sha256': digest(before),
                'after_sha256': digest(before), 'plan_sha256': plan['plan_sha256']}
    require(approved['state_sha256'] == digest(before), 'target changed since review')
    if operation == 'apply':
        tx.add(plan)
    else:
        tx.remove(plan)
    after = tx.snapshot(plan)
    require(inspect_state(plan, after) == wanted and
            anchor_digest(plan, before) == anchor_digest(plan, after), 'post-operation source preservation failed')
    return {'operation': operation, 'changed': True, 'result': wanted, 'before_sha256': digest(before),
            'after_sha256': digest(after), 'plan_sha256': plan['plan_sha256']}


class NoRedirect(HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        raise ValueError('graph redirects refused; credentials stay at the explicit endpoint')


class GraphHTTP:
    """Neo4j 5 transactional HTTP. No ambient credentials or automatic retries."""
    def __init__(self, endpoint, database, user, password, timeout=30):
        split = urlsplit(endpoint)
        require(split.scheme in ('http', 'https') and split.hostname and not split.username and
                not split.password and split.path in ('', '/') and not split.query and not split.fragment,
                'explicit graph origin required, without URL credentials/path')
        require(split.scheme == 'https' or split.hostname in ('localhost', '127.0.0.1', '::1'),
                'unencrypted graph credentials allowed only on loopback')
        require(re.fullmatch(r'[a-zA-Z0-9][a-zA-Z0-9._-]{0,62}', database), 'invalid database name')
        require(user and ':' not in user and password and 0 < timeout <= 120, 'explicit credentials/timeout required')
        self.endpoint, self.database = endpoint.rstrip('/'), database
        self.url = self.endpoint + '/db/' + database + '/tx'
        self.headers = {'Content-Type': 'application/json', 'Authorization':
                        'Basic ' + base64.b64encode((user + ':' + password).encode()).decode()}
        self.timeout, self.opener = timeout, build_opener(NoRedirect())

    def request(self, url, method, statements=None):
        payload = None if statements is None else canonical({'statements': statements}).encode()
        req = Request(url, data=payload, method=method, headers=self.headers)
        try:
            with self.opener.open(req, timeout=self.timeout) as response:
                data = response.read(MAX_RESPONSE + 1)
                location = response.headers.get('Location')
        except (HTTPError, URLError, TimeoutError, OSError) as error:
            raise RuntimeError('graph transport failed; no automatic retry') from error
        require(len(data) <= MAX_RESPONSE, 'excessive graph response')
        body = json.loads(data)
        require(isinstance(body, dict) and isinstance(body.get('errors'), list), 'malformed graph response')
        if body['errors']:
            raise RuntimeError('graph rejected query: ' + str(body['errors'][0].get('code', 'unknown')))
        return body, location

    def begin(self):
        body, location = self.request(self.url, 'POST', [])
        commit_url = body.get('commit', '')
        require(isinstance(commit_url, str) and commit_url.endswith('/commit'), 'missing graph commit location')
        transaction_url = commit_url[:-7]
        require(not location or location == transaction_url, 'graph transaction locations disagree')
        suffix = transaction_url.removeprefix(self.url + '/')
        require(re.fullmatch('[0-9]+', suffix), 'invalid graph transaction location')
        return GraphTransaction(self, self.url + '/' + suffix)


class GraphTransaction:
    def __init__(self, client, url):
        self.client, self.url = client, url
        self.commit_attempted = False
        self.closed = False

    def query(self, statement, **parameters):
        body, _ = self.client.request(self.url, 'POST', [{'statement': statement, 'parameters': parameters}])
        require(len(body.get('results', [])) == 1, 'missing graph result')
        result = body['results'][0]
        require(len(result['data']) <= MAX_ROWS, 'excessive graph rows')
        return [dict(zip(result['columns'], row['row'])) for row in result['data']]

    def snapshot(self, plan):
        identity = self.query('CALL db.info() YIELD id, name RETURN id, name')
        require(len(identity) == 1 and identity[0]['name'] == self.client.database, 'graph database identity missing')
        node_ids = [n['properties']['ql_k8_id'] for n in plan['nodes']]
        edge_ids = [r['properties']['ql_k8_id'] for r in plan['relations']]
        nodes = self.query('MATCH (n) WHERE n.coordinate IN $coordinates OR n.ql_k8_promotion = $promotion '
                           'OR n.ql_k8_id IN $ids RETURN elementId(n) AS element_id, labels(n) AS labels, '
                           'properties(n) AS properties ORDER BY element_id LIMIT 10001',
                           coordinates=candidates(plan), promotion=plan['promotion_id'], ids=node_ids)
        relations = self.query('MATCH (a)-[r]->(b) WHERE r.ql_k8_promotion = $promotion OR r.ql_k8_id IN $ids '
                               'RETURN elementId(r) AS element_id, a.coordinate AS from_coordinate, '
                               'b.coordinate AS to_coordinate, type(r) AS type, properties(r) AS properties '
                               'ORDER BY element_id LIMIT 10001', promotion=plan['promotion_id'], ids=edge_ids)
        incident = self.query('MATCH (a)-[r]->(b) WHERE a.coordinate IN $new OR b.coordinate IN $new '
                              'OR a:QLStructuralPromotion AND a.id = $promotion '
                              'OR b:QLStructuralPromotion AND b.id = $promotion '
                              'RETURN DISTINCT elementId(r) AS element_id ORDER BY element_id LIMIT 10001',
                              new=[n['properties']['coordinate'] for n in plan['nodes']], promotion=plan['promotion_id'])
        markers = self.query('MATCH (n:QLStructuralPromotion {id:$promotion}) RETURN elementId(n) AS element_id, '
                             'labels(n) AS labels, properties(n) AS properties ORDER BY element_id', promotion=plan['promotion_id'])
        for node in nodes + markers:
            node['labels'].sort()
        return {'target': {'endpoint': self.client.endpoint, 'database': self.client.database, 'identity': identity[0]['id']},
                'nodes': nodes, 'relations': relations, 'incident': incident, 'markers': markers}

    def lock(self, plan):
        constraints = self.query('SHOW CONSTRAINTS YIELD type, entityType, labelsOrTypes, properties '
                                 'RETURN type, entityType, labelsOrTypes, properties')
        for label, prop in (('Bimba', 'coordinate'), ('QLStructuralPromotionLock', 'id'), ('QLStructuralPromotion', 'id')):
            require(any(c['entityType'] == 'NODE' and c['type'] == 'UNIQUENESS' and
                        c['labelsOrTypes'] == [label] and c['properties'] == [prop] for c in constraints),
                    'reviewed graph schema prerequisite missing: ' + label + '.' + prop)
        lock = self.query('MATCH (n:QLStructuralPromotionLock {id:$lock}) SET n.id = n.id '
                          'RETURN labels(n) AS labels, properties(n) AS properties', lock=LOCK)
        require(len(lock) == 1 and lock[0]['labels'] == ['QLStructuralPromotionLock'] and
                lock[0]['properties'] == {'id': LOCK}, 'shared structural lock is missing/conflicting')
        # Same-value dependent SETs acquire locks before inspecting mutable state;
        # no marker property is borrowed from or removed from an existing node.
        self.query('MATCH (n) WHERE n.coordinate IN $coordinates SET n.coordinate = n.coordinate '
                   'RETURN count(n) AS locked', coordinates=candidates(plan))
        self.query('MATCH ()-[r]->() WHERE r.ql_k8_promotion = $promotion '
                   'SET r.ql_k8_id = r.ql_k8_id RETURN count(r) AS locked', promotion=plan['promotion_id'])

    def add(self, plan):
        self.query('UNWIND $nodes AS item CREATE (n:Bimba:Coordinate:QLK8PromotionNode) '
                   'SET n = item.properties RETURN count(n) AS count', nodes=plan['nodes'])
        # Only compiler-produced, uppercase-whitelisted types enter query text;
        # every value, identifier and source assertion is still a parameter.
        for kind in sorted({r['type'] for r in plan['relations']}):
            require(re.fullmatch('[A-Z][A-Z0-9_]*', kind), 'unsafe compiled relation type')
            edges = [r for r in plan['relations'] if r['type'] == kind]
            self.query('UNWIND $edges AS item MATCH (a:Bimba {coordinate:item.from_coordinate}), '
                       '(b:Bimba {coordinate:item.to_coordinate}) CREATE (a)-[r:' + kind + ']->(b) '
                       'SET r = item.properties RETURN count(r) AS count', edges=edges)
        self.query('CREATE (n:QLStructuralPromotion) SET n = $properties RETURN count(n) AS count', properties=marker(plan))

    def remove(self, plan):
        # Exact equality/foreign-reference checks precede this bounded deletion.
        # Plain DELETE refuses concurrent external links; DETACH DELETE is banned.
        self.query('MATCH ()-[r]->() WHERE r.ql_k8_promotion = $promotion DELETE r '
                   'RETURN count(r) AS count', promotion=plan['promotion_id'])
        self.query('MATCH (n:QLK8PromotionNode {ql_k8_promotion:$promotion}) DELETE n '
                   'RETURN count(n) AS count', promotion=plan['promotion_id'])
        self.query('MATCH (n:QLStructuralPromotion {id:$promotion}) DELETE n RETURN count(n) AS count', promotion=plan['promotion_id'])

    def commit(self):
        self.commit_attempted = True
        self.client.request(self.url + '/commit', 'POST', [])
        self.closed = True

    def rollback(self):
        if not self.closed and not self.commit_attempted:
            try:
                self.client.request(self.url, 'DELETE')
            finally:
                self.closed = True


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('operation', choices=('plan', 'schema', 'review', 'apply', 'rollback'))
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--endpoint')
    parser.add_argument('--database')
    parser.add_argument('--user')
    parser.add_argument('--password-env', default='QL_K8_GRAPH_PASSWORD')
    parser.add_argument('--review', type=Path)
    parser.add_argument('--confirm-plan')
    args = parser.parse_args()
    require(not args.out.exists(), 'refusing to overwrite an existing graph receipt')
    plan = projection()
    if args.operation == 'plan':
        result = plan
    elif args.operation == 'schema':
        result = {'schema': 'ql.k8-graph-schema/v1', 'statements': SCHEMA, 'parameters': {'lock': LOCK},
                  'standing': 'operator-reviewed provisioning only; never run implicitly by apply'}
    else:
        require(args.endpoint and args.database and args.user, 'explicit endpoint, database and user required')
        client = GraphHTTP(args.endpoint, args.database, args.user, os.environ.get(args.password_env, ''))
        if args.operation != 'review':
            require(args.confirm_plan == plan['plan_sha256'] and args.review, 'explicit exact plan/review confirmation required')
        tx = client.begin()
        try:
            if args.operation == 'review':
                result = review(plan, tx)
                tx.rollback()
            else:
                approved = json.loads(args.review.read_text())
                result = operate(plan, approved, tx, args.operation)
                result.update(schema='ql.k8-graph-operation/v1', target=approved['target'],
                              review_sha256=approved['review_sha256'],
                              standing='explicit transaction; structural graph evidence only')
                try:
                    tx.commit()
                    result['commit_standing'] = 'acknowledged'
                except Exception:
                    result['commit_standing'] = 'unknown; inspect target with a new review; do not auto-retry'
                    args.out.parent.mkdir(parents=True, exist_ok=True)
                    args.out.write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n')
                    raise
        finally:
            tx.rollback()
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n')
    print(json.dumps({'operation': args.operation, 'plan_sha256': plan['plan_sha256'], 'receipt': str(args.out)}))


if __name__ == '__main__':
    try:
        main()
    except (ValueError, RuntimeError, OSError, KeyError) as error:
        print(str(error), file=sys.stderr)
        raise SystemExit(1)
