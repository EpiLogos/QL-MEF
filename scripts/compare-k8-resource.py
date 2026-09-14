#!/usr/bin/env python3
"""Compare two measured build profiles without changing input or success policy."""
import json
from pathlib import Path
import statistics
import sys


def compare(baseline, candidate):
    for value in (baseline, candidate):
        if value.get('schema') != 'ql.k8-native-resource-acceptance/v1' or not all(
            value.get(k) is True for k in ('measurement_valid', 'resource_pass', 'exact_original_replay')):
            raise ValueError('complete, resource-admitted, replay-verified measurements required')
    before, after = baseline['conditions'], candidate['conditions']
    for key in ('policy', 'exact_source', 'tree', 'source_input_sha256', 'worker_sha256',
                'platform', 'cpu_model', 'cpu_count', 'python', 'mode_count', 'sample_count', 'input_schema'):
        if key not in before or before[key] != after.get(key):
            raise ValueError(f'comparison changed a controlled condition: {key}')
    if not candidate.get('cross_build_replay') or after['replay_host_sha256'] != before['host_sha256']:
        raise ValueError('candidate operations must replay exactly through the baseline binary')
    rows = []
    for field, read in (
        ('block_p95_ms', lambda c: c['host']['latency_ms']['p95']),
        ('source_operation_max_ms', lambda c: c['source_operation_ms']['maximum']),
        ('host_rss_max_bytes', lambda c: c['host']['rss_max_bytes']),
        ('native_deadline_misses', lambda c: c['host']['deadline_misses']),
    ):
        a, b = [read(c) for c in baseline['cycles']], [read(c) for c in candidate['cycles']]
        if len(a) != before['policy']['cycles'] or len(b) != len(a):
            raise ValueError('comparison omitted a declared cycle')
        if any(not isinstance(v, (int, float)) or not (0 <= v < float('inf')) for v in a + b):
            raise ValueError('missing or invalid observation')
        ma, mb = statistics.median(a), statistics.median(b)
        rows.append(dict(metric=field, baseline_cycles=a, candidate_cycles=b,
                         baseline_median=ma, candidate_median=mb,
                         candidate_to_baseline_ratio=mb / ma if ma else None))
    return dict(schema='ql.k8-build-profile-comparison/v1', rows=rows,
        same_input_policy_and_worker=True, exact_cross_build_replay=True,
        baseline_profile=before['host_build_profile'], candidate_profile=after['host_build_profile'],
        candidate_deadline_pass=candidate['native_deadline_pass'],
        candidate_source_operation_budget_pass=candidate['source_operation_budget_pass'],
        candidate_runtime_budget_pass=candidate['native_deadline_pass'] and candidate['source_operation_budget_pass'],
        standing='same-runner sequential build-profile observations; not a randomized causal study or whole desktop acceptance')


if __name__ == '__main__':
    if len(sys.argv) != 4:
        raise SystemExit('usage: compare-k8-resource.py BASELINE_JSON CANDIDATE_JSON OUTPUT_JSON')
    result = compare(*(json.loads(Path(p).read_text()) for p in sys.argv[1:3]))
    Path(sys.argv[3]).write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps(result))
