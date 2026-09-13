#!/usr/bin/env python3
"""Bounded, attributable resource proof over the installed *same* K8 host.

This is a controlled native measurement, not a browser/GPU/device benchmark or
an Epii/provider workload. Deadline and resource standing remain separate from
measurement validity. No timing, source, or command is dropped to improve a score.
"""
from __future__ import annotations
import hashlib
import importlib.util
import json
import math
import os
from pathlib import Path
import platform
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[1]
# Declared before observation. This cut is intentionally bounded and repeatable.
POLICY = dict(cycles=3, blocks_per_cycle=128, block_frames=4096, warmup_blocks=8,
    host_rss_ceiling_bytes=512*1024*1024, worker_rss_ceiling_bytes=256*1024*1024,
    rss_growth_ceiling_bytes=32*1024*1024, fd_growth_ceiling=0,
    response_ceiling_bytes=64*1024*1024, idle_seconds=0.2, idle_cpu_ceiling_seconds=0.05,
    native_deadline='one block duration; any miss is recorded, never discarded',
    source_operation_budget_ms=400, percentile_rule='nearest rank; no trimmed samples')


def digest(value):
    raw = json.dumps(value, allow_nan=False, sort_keys=True, separators=(',', ':')).encode()
    return hashlib.sha256(raw).hexdigest()


def summary(values):
    if not values:
        return dict(count=0, minimum=None, p50=None, p95=None, p99=None, maximum=None)
    if any(not math.isfinite(v) or v < 0 for v in values):
        raise ValueError('measurement must be finite and nonnegative')
    ordered = sorted(values)
    rank = lambda p: ordered[max(0, math.ceil(len(ordered)*p)-1)]
    return dict(count=len(values), minimum=ordered[0], p50=rank(.5), p95=rank(.95),
        p99=rank(.99), maximum=ordered[-1])


def assess(samples, *, deadline_ms, ceiling, growth):
    if not samples or not math.isfinite(deadline_ms) or deadline_ms <= 0:
        raise ValueError('complete measured samples and a positive declared deadline required')
    latency = summary([v['elapsed_ms'] for v in samples])
    memory = [v['rss_bytes'] for v in samples]
    if any(not isinstance(v, int) or v < 0 for v in memory):
        raise ValueError('RSS must be actual nonnegative bytes, not unknown coerced to zero')
    misses = sum(v['elapsed_ms'] > deadline_ms for v in samples)
    return dict(latency_ms=latency, deadline_ms=deadline_ms, deadline_misses=misses,
        deadline_pass=misses == 0, rss_max_bytes=max(memory), rss_growth_bytes=max(memory)-memory[0],
        resource_pass=max(memory) <= ceiling and max(memory)-memory[0] <= growth)


def proc(pid):
    root = Path('/proc') / str(pid)
    status = dict(line.split(':', 1) for line in (root/'status').read_text().splitlines() if ':' in line)
    # The command name may contain spaces and parentheses; fields start after its LAST ')'.
    fields = (root/'stat').read_text().rsplit(')', 1)[1].split()
    return dict(pid=pid, rss_bytes=int(status['VmRSS'].split()[0])*1024,
        cpu_seconds=(int(fields[11])+int(fields[12]))/os.sysconf('SC_CLK_TCK'),
        fd_count=len(list((root/'fd').iterdir())))


def main():
    if platform.system() != 'Linux':
        raise RuntimeError('this receipt requires actual Linux /proc observations; no invented portable values')
    coupled = Path(os.environ.get('K8_COUPLED_DIR', ROOT/'target/k8-coupled-v2'))
    out = Path(os.environ.get('K8_RESOURCE_OUT_DIR', ROOT/'target/k8-resource'))
    out.mkdir(parents=True, exist_ok=True)
    os.environ['K8_COUPLED_DIR'] = str(coupled)
    os.environ['K8_HOST_OUT_DIR'] = str(out)
    spec = importlib.util.spec_from_file_location('k8_resource_host', ROOT/'scripts/test-k8-host.py')
    hosts = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(hosts)
    hosts.HOST = ROOT/'target/k8-cpp/bin/ql-field-host'
    hosts.WORKER = ROOT/'target/k8-cpp/bin/ql-field-worker'
    config = hosts.CONFIG
    changed = json.loads((coupled/'changed-event.json').read_text())['basis']['input']
    conditions = dict(policy=POLICY, platform=platform.platform(), python=sys.version,
        cpu_model=next((v.split(':', 1)[1].strip() for v in Path('/proc/cpuinfo').read_text().splitlines()
                       if v.startswith('model name')), None),
        cpu_count=os.cpu_count(), exact_source=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),
        tree=subprocess.check_output(['git','rev-parse','HEAD^{tree}'],cwd=ROOT,text=True).strip(),
        source_input_sha256=digest(config), input_schema=config['basis']['schema'],
        mode_count=len(config['basis']['m2']['resonator']['modes']), sample_count=len(config['field']['samples']),
        host_sha256=hashlib.sha256(hosts.HOST.read_bytes()).hexdigest(),
        worker_sha256=hashlib.sha256(hosts.WORKER.read_bytes()).hexdigest(),
        standing='controlled native host/pipe with real dated sky, supplied geometry and subject; not full composed desktop load')
    (out/'conditions.json').write_text(json.dumps(conditions,indent=2)+'\n')
    rate = config['field']['sample_rate']
    deadline = POLICY['block_frames']*1000/rate
    cycles = []
    for cycle in range(POLICY['cycles']):
        host = hosts.Host(config, f'resource-{cycle}')
        log_path = out/f'operations-{cycle}.jsonl'
        initial = hosts.state(host.current)
        expected_samples = int(initial['samples_elapsed'])
        stable_targets = [(v['identity'],v['constituent']) for v in initial['targets']]
        pids = subprocess.check_output(['ps','-o','pid=','--ppid',str(host.process.pid)],text=True).split()
        assert len(pids) == 1, 'exactly one actual numerical child per instance'
        child = int(pids[0]); samples=[]; source_times=[]; receipt_count=0; command_ms=0.0; response_bytes=0
        try:
            with log_path.open('w') as journal:
                def exchange(command):
                    nonlocal receipt_count, command_ms, response_bytes
                    start=time.perf_counter_ns()
                    response=host.send(command)
                    elapsed=(time.perf_counter_ns()-start)/1e6
                    assert response['status']=='ok' and response['available'], response.get('error')
                    command_ms+=elapsed
                    size=len(hosts.encoded(response).encode())
                    response_bytes=max(response_bytes,size)
                    assert size<=POLICY['response_ceiling_bytes'], 'host output ceiling exceeded'
                    packet=host.requests[-1]
                    journal.write(json.dumps(dict(request=packet, response_sha256=digest(response),elapsed_ms=elapsed),
                        allow_nan=False,separators=(',',':'))+'\n')
                    # This instrumented consumer has a fixed live-memory footprint.
                    # Exact operation evidence is streamed, not accumulated in the test driver.
                    host.requests.clear(); host.responses.clear(); receipt_count+=1
                    return response, elapsed
                for _ in range(POLICY['warmup_blocks']):
                    response,_=exchange(dict(operation='advance',frames=POLICY['block_frames'],muted=False))
                    expected_samples+=POLICY['block_frames']
                start_host=proc(host.process.pid); start_worker=proc(child)
                command_ms=0.0
                begin=time.perf_counter()
                for block in range(POLICY['blocks_per_cycle']):
                    muted=block % 16 < 4
                    response,elapsed=exchange(dict(operation='advance',frames=POLICY['block_frames'],muted=muted))
                    field=response['field']; expected_samples+=POLICY['block_frames']
                    assert field['samples_elapsed']==str(expected_samples)
                    assert len(field['audio'])==POLICY['block_frames']
                    assert [(v['identity'],v['constituent']) for v in field['targets']]==stable_targets
                    assert 'sources' not in response
                    if muted: assert not any(field['audio'])
                    else: assert all(math.isfinite(v) for v in field['audio'])
                    h,w=proc(host.process.pid),proc(child)
                    samples.append(dict(elapsed_ms=elapsed,rss_bytes=h['rss_bytes'],worker_rss_bytes=w['rss_bytes'],
                        host_fd=h['fd_count'],worker_fd=w['fd_count']))
                    if block % 32==0:
                        before=hosts.state(response)
                        read,_=exchange(dict(operation='read'))
                        assert hosts.state(read)==before, 'inspection caused hidden integration'
                    if block==63:
                        before=hosts.state(response)
                        replace,elapsed=exchange(dict(operation='replace',basis=changed))
                        source_times.append(elapsed)
                        for key in ('samples_elapsed','amplitudes_metres','clock','targets'):
                            assert replace['field'][key]==before[key], f'replacement reset {key}'
                        inspect,elapsed=exchange(dict(operation='inspect'))
                        source_times.append(elapsed)
                        assert inspect['sources']['original']['input']==config['basis']
                        assert inspect['sources']['current']['input']==changed
                span=time.perf_counter()-begin
                observation_seconds=max(0.0,span-command_ms/1000)
                end_host=proc(host.process.pid); end_worker=proc(child)
                before=hosts.state(host.current); idle_h=proc(host.process.pid); idle_w=proc(child)
                time.sleep(POLICY['idle_seconds'])
                after_h=proc(host.process.pid); after_w=proc(child)
                idle_cpu=after_h['cpu_seconds']-idle_h['cpu_seconds']+after_w['cpu_seconds']-idle_w['cpu_seconds']
                read,_=exchange(dict(operation='read'))
                assert hosts.state(read)==before, 'idle time manufactured native integration'
                assert idle_cpu<=POLICY['idle_cpu_ceiling_seconds'], 'idle native owner consumed undeclared work'
                assert max(v['host_fd'] for v in samples)<=start_host['fd_count']+POLICY['fd_growth_ceiling']
                assert max(v['worker_fd'] for v in samples)<=start_worker['fd_count']+POLICY['fd_growth_ceiling']
                host_assessment=assess(samples,deadline_ms=deadline,ceiling=POLICY['host_rss_ceiling_bytes'],growth=POLICY['rss_growth_ceiling_bytes'])
                worker_assessment=assess([dict(elapsed_ms=v['elapsed_ms'],rss_bytes=v['worker_rss_bytes']) for v in samples],
                    deadline_ms=deadline,ceiling=POLICY['worker_rss_ceiling_bytes'],growth=POLICY['rss_growth_ceiling_bytes'])
                assert host_assessment['resource_pass'] and worker_assessment['resource_pass'], 'measured resource ceiling/growth exceeded'
                cycles.append(dict(cycle=cycle,measured_blocks=len(samples),commands=receipt_count,
                    native_seconds=POLICY['blocks_per_cycle']*POLICY['block_frames']/rate,wall_seconds=span,
                    observation_and_validation_seconds=observation_seconds,peak_response_bytes=response_bytes,
                    host_cpu_seconds=end_host['cpu_seconds']-start_host['cpu_seconds'],
                    worker_cpu_seconds=end_worker['cpu_seconds']-start_worker['cpu_seconds'],
                    host=host_assessment,worker=worker_assessment,source_operation_ms=summary(source_times),
                    source_budget_pass=max(source_times)<=POLICY['source_operation_budget_ms'],
                    idle_cpu_seconds=idle_cpu,stable_samples=True,no_idle_or_read_integration=True,
                    single_numerical_child=True,fd_growth=0))
        finally:
            host.close()
        assert not Path('/proc',str(child)).exists(), 'native numerical child leaked after owner disposal'
        # Original replay must reproduce every retained acknowledgement, including
        # the original dated source and replacement. Timings are not replay inputs.
        replay=hosts.Host(config,f'resource-replay-{cycle}')
        try:
            for line in log_path.open():
                record=json.loads(line)
                response=replay.send(packet=record['request'])
                assert digest(response)==record['response_sha256'], 'resource run lost original replay fidelity'
                replay.requests.clear(); replay.responses.clear()
        finally:
            replay.close()
    receipt=dict(schema='ql.k8-native-resource-acceptance/v1',conditions=conditions,cycles=cycles,
        measured_blocks=sum(v['measured_blocks'] for v in cycles),measurement_valid=True,
        native_deadline_pass=all(v['host']['deadline_pass'] for v in cycles),
        source_operation_budget_pass=all(v['source_budget_pass'] for v in cycles),
        resource_pass=True,exact_original_replay=True,repeated_owner_disposal=True,
        unmeasured=['GPU/VRAM','browser/audio-device scheduling','Epii/model/provider work','full Bimba/Epii/background/remote composed load',
                    'owner-machine interaction and cancellation latency','human sensory judgement'])
    (out/'acceptance.json').write_text(json.dumps(receipt,indent=2)+'\n')
    print(json.dumps(receipt))


if __name__=='__main__':
    main()
