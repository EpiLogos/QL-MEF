#!/usr/bin/env python3
"""Real sky and accepted M2 -> persistent installed C++ field; controlled geometry.

This proves continuation of declared modal inputs, not a universal plate solver,
Nara reception or the empirical truth of symbolic-musical correspondences.
"""
from copy import deepcopy
from datetime import datetime, timezone
import json
import math
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'providers/sky'))
import kerykeion_snapshot as sky

OUT = ROOT / 'target/k8-continuous'
OUT.mkdir(parents=True, exist_ok=True)
worker = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else ROOT / 'target/k8-cpp/bin/ql-field-worker'
m2_exe = ROOT / 'target/debug/examples/m2_engine'
now = datetime.now(timezone.utc).replace(microsecond=0)
snapshot = sky.produce(dict(schema=sky.REQUEST, epoch=sky.iso(now), timezone='UTC',
    mode='current', perspective='Apparent Geocentric', zodiac='Tropical', ayanamsha=None,
    observer=None, max_age_seconds=3600, backend_policy='allow-moshier'))
request = json.loads((ROOT / 'fixtures/kernel/m2-engine-request-v1.json').read_text())
request['at_unix_ms'] = snapshot['receipt_unix_ms']
request['stamp']['identity']['event_ref'] = 'acceptance:continuous:dated-world'
for key in ('m1_excitation', 'vimarsha'):
    request[key]['stamp']['identity'] = deepcopy(request['stamp']['identity'])
seed = sky.execute_m2(snapshot, request, m2_exe)
reading = seed['m2']['vimarsha']['reading']
# Native accepted Vimarsha owns these frequencies/indices. A supplied nodal
# basis is distinct from its acoustic eigenvalue law; do not equate the two.
request['resonator'] = dict(stamp=deepcopy(request['stamp']), provider_ref='controlled:native-vimarsha-modal-projection',
    geometry_ref='controlled:signed-square-basis', material_ref='controlled:linear-medium',
    material_model_ref='ql.continuous-linear-mode/v1', material_parameters={
        'unit_length': dict(value=1, unit='m', source_ref='controlled:square-definition')},
    modes=[dict(mode_ref=f'controlled:mode/{i}', source_coordinate='#2-1', material_fibre='earth',
        carrier_weights=[dict(carrier=i, weight=1)], frequency_hz=hz,
        amplitude=[0.005, 0.002], excitation=[0.001, 0], damping_per_second=0.25,
        nodal_state_ref=f'controlled:square-node/{i}', antinodal_state_ref=f'controlled:square-antinode/{i}')
        for i, hz in enumerate(reading['audio_octet_hz'])])
event = sky.execute_m2(snapshot, request, m2_exe)
assert event['m2']['vimarsha'] == seed['m2']['vimarsha']
clock = dict(inscription=dict(turns='0', half_degrees=718), lensing=dict(turns='1', half_degrees=17),
             grid_origins=[3, 9, 21], rate_numerators=['9', '8'], rate_denominator=8,
             rate_remainders=['0', '0'], generation='4')
samples = []
for i in range(256):
    x, y = (i % 16) / 15, (i // 16) / 15
    shapes = [[0, 0, math.sin(math.pi * (j % 4 + 1) * x) * math.sin(math.pi * (j // 4 + 1) * y)]
              for j in range(8)]
    samples.append(dict(identity=i, constituent='#3-0', attachment=1 + i % 2,
                        rest_metres=[x - 0.5, y - 0.5, 0], mode_shapes=shapes))
field = dict(subject_ref='controlled:subject/one', sample_rate=48000, clock=clock,
             driver_numerator=720, driver_denominator=1, audio_gains=[1.0] * 8,
             units=dict(amplitude='m', excitation='m/s', shape='dimensionless', position='m', audio='linear'),
             samples=samples)
initial = dict(schema='ql.field-control/v1', operation='initialize', m2=event['m2'], field=field)
(OUT / 'basis.json').write_text(json.dumps(event, sort_keys=True) + '\n')
(OUT / 'initial.json').write_text(json.dumps(initial, sort_keys=True) + '\n')


class Worker:
    def __init__(self):
        self.p = subprocess.Popen([str(worker)], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
        self.requests, self.responses = [], []

    def send(self, value, *, error=False):
        self.p.stdin.write(json.dumps(value, allow_nan=False, separators=(',', ':')) + '\n')
        self.p.stdin.flush()
        line = self.p.stdout.readline()
        assert line, 'worker exited without a receipt'
        result = json.loads(line)
        if error:
            assert result['schema'] == 'ql.field-error/v1' and not result['state_committed'], result
        else:
            assert result['schema'] == 'ql.continuous-field/v1', result
        self.requests.append(deepcopy(value)); self.responses.append(result)
        return result

    def close(self):
        self.p.stdin.close()
        assert self.p.wait(timeout=10) == 0
        self.p.stdout.close()


def control(state, operation, **fields):
    return dict(schema='ql.field-control/v1', operation=operation, expected_generation=state['generation'],
                expected_samples_elapsed=state['samples_elapsed'], **fields)


w = Worker()
state = w.send(initial)
assert len(state['targets']) == 256 and len(state['amplitudes_metres']) == 8
assert state['clock']['centre_ref'] == '#3-5-5/0'
# Validation/rejection must leave the resident field and original basis unchanged.
for command in [dict(control(state, 'advance', frames=32, muted=False), expected_generation='0' + state['generation']),
                dict(control(state, 'advance', frames=32, muted=False), expected_samples_elapsed='-0'),
                control(state, 'set-axis', axis=0, phase=dict(turns='00', half_degrees=0)),
                control(state, 'advance', frames=9000, muted=False),
                control(state, 'advance', frames=0, muted='not-a-boolean'),
                control(state, 'set-axis', axis=1, phase=dict(turns='2', half_degrees=720)),
                dict(control(state, 'advance', frames=64, muted=False), expected_generation='0')]:
    w.send(command, error=True)
    assert w.send(dict(schema='ql.field-control/v1', operation='read')) == state
before = deepcopy(state)
state = w.send(control(state, 'advance', frames=8192, muted=False))
assert len(state['audio']) == 8192 and any(abs(x) > 1e-6 for x in state['audio'])
assert state['clock']['inscription']['turns'] == '1'
assert state['targets'] != before['targets']
assert [p['identity'] for p in state['targets']] == list(range(256))
# A second view only reads, never advances a second copy of physical time.
view = w.send(dict(schema='ql.field-control/v1', operation='read'))
assert view['samples_elapsed'] == state['samples_elapsed']
assert view['targets'] == state['targets'] and view['amplitudes_metres'] == state['amplitudes_metres']
state = w.send(control(state, 'set-axis', axis=1, phase=dict(turns='-2', half_degrees=33)))
assert state['clock']['inscription'] == view['clock']['inscription']
assert state['clock']['lensing']['turns'] == '-2'
assert state['amplitudes_metres'] == view['amplitudes_metres']
# Modes change in a new M2 generation without imposing its counter on the
# independent local clock/material control revision or physical sample cursor.
updated = deepcopy(event['m2_input'])
updated['stamp']['identity']['profile_generation'] += 1
for key in ('resonator', 'vimarsha', 'm1_excitation'):
    updated[key]['stamp']['identity'] = deepcopy(updated['stamp']['identity'])
updated['resonator']['modes'][0]['frequency_hz'] *= 1.1
new_event = sky.execute_m2(snapshot, updated, m2_exe)
old_amplitudes = state['amplitudes_metres']
state = w.send(control(state, 'replace-modes', m2=new_event['m2'], replace_state=False))
assert state['amplitudes_metres'] == old_amplitudes
assert state['m2_identity'] == new_event['m2']['identity']
stale = control(state, 'replace-modes', m2=event['m2'], replace_state=False)
w.send(stale, error=True)
state = w.send(control(state, 'advance', frames=1024, muted=True))
assert all(x == 0 for x in state['audio']) and state['amplitudes_metres'] != old_amplitudes
# Provider loss does not call a provider from the audio/geometry owner. The
# retained source basis remains available; host decides freshness/mute policy.
for _ in range(20):
    state = w.send(control(state, 'advance', frames=256, muted=False))
(OUT / 'last.json').write_text(json.dumps(state, sort_keys=True) + '\n')
second = Worker(); other = deepcopy(initial); other['field']['subject_ref'] = 'controlled:subject/two'
other['m2']['resonator']['modes'][0]['amplitude'] = [0.02, 0]
second_state = second.send(other)
assert second_state['amplitudes_metres'] != before['amplitudes_metres']
assert w.send(dict(schema='ql.field-control/v1', operation='read'))['targets'] == state['targets']
second.close()
replay = Worker()
for request_value, expected in zip(w.requests, w.responses):
    assert replay.send(request_value, error=expected['schema'] == 'ql.field-error/v1') == expected
replay.close(); w.close()
(OUT / 'controls.jsonl').write_text(''.join(json.dumps(v, sort_keys=True) + '\n' for v in w.requests))
(OUT / 'frames.jsonl').write_text(''.join(json.dumps(v, sort_keys=True) + '\n' for v in w.responses))
proof = dict(schema='ql.k8-continuous-acceptance/v1', worker_sha256=sky.file_digest(worker),
    m2_sha256=sky.file_digest(m2_exe), provider_snapshot=snapshot['snapshot_ref'],
    basis_sha256=sky.file_digest(OUT / 'basis.json'), actual_native_frequencies=reading['audio_octet_hz'],
    commands=len(w.requests), exact_replay=True, independent_subjects=True, stable_sample_ids=256,
    stale_and_invalid_atomic=True, continued_during_mute=True, independent_axes=True,
    geometry_standing='controlled supplied square functions, not a measured plate eigenbasis',
    limits=['no Nara reception proof', 'no physical correspondence proof', 'GPU acceptance is a separate actual-renderer test'])
(OUT / 'acceptance.json').write_text(json.dumps(proof, indent=2) + '\n')
print('PASS: actual sky -> full native M2 -> eight native-frequency C++ modes, persistent field, clock, replay and refusal')
