#!/usr/bin/env python3
"""Actual ephemeris -> installed Rust M2, current and pre-1970 historical inputs."""
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'providers/sky'))
import kerykeion_snapshot as sky

out = ROOT / 'target/k8-sky'
out.mkdir(parents=True, exist_ok=True)
exe = ROOT / 'target/debug/examples/m2_engine'
original = json.loads((ROOT / 'fixtures/kernel/m2-engine-request-v1.json').read_text())
proof = []
now = datetime.now(timezone.utc).replace(microsecond=0)
for mode, epoch in [('current', sky.iso(now)), ('historical', '1875-07-26T12:00:00Z'),
                    ('historical', '2000-01-01T12:00:00Z'), ('historical', '2026-07-10T12:00:00Z')]:
    request = dict(schema=sky.REQUEST, epoch=epoch, timezone='UTC', mode=mode,
                   perspective='Apparent Geocentric', zodiac='Tropical', ayanamsha=None,
                   observer=None, max_age_seconds=3600, backend_policy='allow-moshier')
    snapshot = sky.produce(request)
    m2_input = json.loads(json.dumps(original))
    m2_input['stamp']['identity']['event_ref'] = 'acceptance:sky:' + snapshot['snapshot_ref']
    for field in ('m1_excitation', 'vimarsha'):
        m2_input[field]['stamp']['identity'] = dict(m2_input['stamp']['identity'])
    m2_input['at_unix_ms'] = snapshot['receipt_unix_ms']
    event = sky.execute_m2(snapshot, m2_input, exe)
    frame = event['m2']
    assert len(frame['world']) == 10 and len(frame['aspects']) == 45
    assert len(frame['modal']['form_potential']) == 64
    assert len(frame['modal']['coefficients']) == 72
    assert frame['m1_excitation'] == m2_input['m1_excitation']
    assert frame['vimarsha']['input'] == m2_input['vimarsha']
    for body, reading in zip(snapshot['bodies'], frame['world']):
        assert reading['planetary_power']['index'] == body['native_planet_id']
        # Native decan/source operation must actually run, not be replaced by Python.
        sign = int(body['longitude_degrees'] // 30)
        sector = int((body['longitude_degrees'] % 30) // 10)
        native_index = ((sign % 4) * 9 + (sign // 4) * 3 + sector) * 2
        assert reading['light_decan']['index'] == native_index
        assert reading['shadow_decan']['index'] == native_index + 1
    saved = json.dumps(event, sort_keys=True, allow_nan=False)
    path = out / ('event-' + str(len(proof)) + '.json')
    path.write_text(saved + '\n')
    replay = sky.execute_m2(snapshot, m2_input, exe)
    assert json.dumps(replay, sort_keys=True, allow_nan=False) == saved
    # New current generation must not replace the retained original occasion.
    changed = json.loads(json.dumps(m2_input))
    changed['stamp']['identity']['profile_generation'] += 1
    for field in ('m1_excitation', 'vimarsha'):
        changed[field]['stamp']['identity'] = dict(changed['stamp']['identity'])
    newer = sky.execute_m2(snapshot, changed, exe)
    assert newer['m2']['identity']['profile_generation'] != frame['identity']['profile_generation']
    assert json.dumps(event, sort_keys=True, allow_nan=False) == saved
    proof.append(dict(epoch=snapshot['epoch_utc'], mode=mode, snapshot_ref=snapshot['snapshot_ref'],
                      event_file=path.name, event_sha256=hashlib.sha256(path.read_bytes()).hexdigest(),
                      native_bodies=10, native_aspects=45, distributed_m3_amplitudes=64,
                      exact_replay=True, independent_generation=True))
(out / 'acceptance.json').write_text(json.dumps({'schema': 'ql.k8-sky-acceptance/v1',
    'provider': snapshot['provider'], 'm2_executable_sha256': sky.file_digest(exe), 'cases': proof,
    'limits': ['calculated sky, not sky observation', 'no private Nara or desktop/GPU acceptance',
               'M2 input excitation is the retained controlled acceptance fixture']}, indent=2) + '\n')
print('PASS: 4 real dated skies -> 10 source planets, 45 aspects, full M2, original replay and later generation')
