#!/usr/bin/env python3
"""Refuse a stale embedded scene default after source refresh; never retag it.

A regeneration uses refresh-scene-default.py with a hashed current native CLI,
actual historical provider output and the retained authored starting recipe.
This check is source/admission proof only, never native runtime acceptance.
"""
import importlib.util
import json
from pathlib import Path
import sys

ROOT=Path(__file__).resolve().parents[1]
spec=importlib.util.spec_from_file_location('sky_default_source',ROOT/'providers/sky/kerykeion_snapshot.py')
sky=importlib.util.module_from_spec(spec)
spec.loader.exec_module(sky)

def main():
    default=ROOT/'fixtures/kernel/scene-default-event-v2.json'
    original=ROOT/'fixtures/kernel/scene-default-event-v1.json'
    value=json.loads(default.read_text()); old=json.loads(original.read_text())
    assert value['schema']=='ql.coupled-event-request/v1'
    receipts=[r for r in value['source_receipts'] if r.get('schema')=='ql.sky-snapshot/v1']
    assert len(receipts)==1,'default needs its exact dated ten-body source receipt'
    saved=receipts[0]
    sky.validate_retained_snapshot(saved)
    q=sky.source_binding_qualification(saved,retained=False)
    assert q['legacy_descriptor_admitted'] is False,'new default must emit the corrected source descriptor'
    assert saved['request']['mode']=='historical'
    assert value['m2']['registry_revision']==value['m3']['registry_revision']==saved['source_binding']['registry_revision']
    assert value['m1']==old['m1'] and value['harmonic_source']==old['harmonic_source'],'authored M1 recipe drift'
    for key in ('condition','selections','modal_coefficients'):
        assert value['m2'][key]==old['m2'][key],f'authored M2 choice drift: {key}'
    for key in ('address','pose','aperture','clock_steps','matrix_axis','rna','subject_ref'):
        assert value['m3'][key]==old['m3'][key],f'authored M3 recipe drift: {key}'
    assert len(value['m2']['world_observations'])==10
    assert all(o['provider_ref']==saved['snapshot_ref'] and o['source_revision']==saved['provider']['adapter_sha256']
               and o['observed_at_unix_ms']==saved['receipt_unix_ms'] for o in value['m2']['world_observations'])
    assert value['m3']['occurrence_unix_ms']==saved['epoch_unix_ms']
    assert value['m3']['receipt_unix_ms']==saved['receipt_unix_ms']
    print(json.dumps({'schema':'ql.scene-default-source-check/v1','snapshot_ref':saved['snapshot_ref'],
                     'registry_revision':saved['source_binding']['registry_revision'],
                     'standing':'source/admission and retained recipe check; native default replay still required'}))

if __name__=='__main__':
    try:
        main()
    except (OSError,KeyError,ValueError,AssertionError) as exc:
        print('scene default is not current: '+str(exc)+'; regenerate through the hashed current native owner without changing the retained recipe',file=sys.stderr)
        raise SystemExit(2)
