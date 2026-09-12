#!/usr/bin/env python3
"""K8 dated sky boundary. No network, personal chart store or symbolic solver.

Kerykeion calculates the requested ten-body field. PySwiss supplies the full
coordinate/speed vector and, critically, the flags the underlying engine really
used. The factory's SWIEPH request alone is not proof of Swiss data-file use.
Calls are serialized because Swiss configuration is process-global. Hosts with
other Swiss users must give this adapter a dedicated worker process.
"""
from __future__ import annotations

import argparse
import copy
from datetime import datetime, timezone
import hashlib
from importlib import metadata
import json
import math
from pathlib import Path
import sys
import subprocess
import threading
from zoneinfo import ZoneInfo

SCHEMA = 'ql.sky-snapshot/v1'
REQUEST = 'ql.sky-request/v1'
BODIES = ('Sun', 'Moon', 'Mercury', 'Venus', 'Mars', 'Jupiter', 'Saturn',
          'Uranus', 'Neptune', 'Pluto')
LOCK = threading.RLock()
ROOT = Path(__file__).resolve().parents[2]
MIN_EPOCH = datetime(1800, 1, 1, tzinfo=timezone.utc)
MAX_EPOCH = datetime(2400, 1, 1, tzinfo=timezone.utc)


class SkyError(ValueError):
    """No partial/live-looking output is published on provider failure."""


def require(condition, message):
    if not condition:
        raise SkyError(message)


def digest(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(',', ':'),
                                    allow_nan=False).encode()).hexdigest()


def file_digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def number(value, lo, hi, label):
    require(type(value) in (int, float) and math.isfinite(value) and lo <= value <= hi,
            'invalid ' + label)
    return value


def epoch(text):
    require(isinstance(text, str), 'epoch requires an offset-bearing ISO timestamp')
    try:
        value = datetime.fromisoformat(text.replace('Z', '+00:00'))
    except ValueError as exc:
        raise SkyError('invalid epoch') from exc
    require(value.tzinfo is not None and value.utcoffset() is not None,
            'ambiguous local epoch: provide explicit UTC offset')
    require(value.microsecond == 0, 'provider boundary has one-second resolution')
    result = value.astimezone(timezone.utc)
    require(MIN_EPOCH <= result < MAX_EPOCH, 'epoch outside adapter range [1800,2400)')
    return result


def iso(value):
    return value.astimezone(timezone.utc).isoformat().replace('+00:00', 'Z')


def request(data):
    require(isinstance(data, dict) and set(data) == {
        'schema', 'epoch', 'timezone', 'mode', 'perspective', 'zodiac',
        'ayanamsha', 'observer', 'max_age_seconds', 'backend_policy'}, 'invalid request fields')
    require(data['schema'] == REQUEST, 'unsupported sky request')
    time = epoch(data['epoch'])
    try:
        zone = ZoneInfo(data['timezone'])
        entered = datetime.fromisoformat(data['epoch'].replace('Z', '+00:00'))
        require(entered.utcoffset() == time.astimezone(zone).utcoffset(),
                'epoch offset does not match supplied IANA timezone')
    except (ValueError, TypeError, KeyError) as exc:
        raise SkyError('invalid timezone') from exc
    require(data['mode'] in ('current', 'historical'), 'invalid epoch mode')
    require(data['perspective'] in ('Apparent Geocentric', 'True Geocentric', 'Topocentric'),
            'unsupported perspective')
    require(data['zodiac'] in ('Tropical', 'Sidereal'), 'unsupported zodiac')
    require((data['zodiac'] == 'Tropical' and data['ayanamsha'] is None) or
            (data['zodiac'] == 'Sidereal' and data['ayanamsha'] == 'LAHIRI'),
            'explicit supported ayanamsha required')
    require(data['backend_policy'] in ('require-swiss-files', 'allow-moshier'),
            'explicit ephemeris fallback policy required')
    require(type(data['max_age_seconds']) is int and 1 <= data['max_age_seconds'] <= 86400,
            'invalid freshness budget')
    observer = data['observer']
    if data['perspective'] == 'Topocentric':
        require(isinstance(observer, dict) and set(observer) == {
            'reference', 'longitude_degrees', 'latitude_degrees', 'altitude_metres'},
            'topocentric reception requires a complete private observer')
        require(isinstance(observer['reference'], str) and 0 < len(observer['reference']) <= 512,
                'invalid observer reference')
        number(observer['longitude_degrees'], -180, 180, 'observer longitude')
        # Kerykeion adjusts extreme latitudes for houses. Refuse hidden relocation.
        number(observer['latitude_degrees'], -66, 66, 'unadjusted provider latitude')
        number(observer['altitude_metres'], -500, 10000, 'observer altitude')
    else:
        require(observer is None, 'shared geocentric field must not carry a private observer')
    return time


def source_bindings():
    header = ROOT / 'vendor/epi-kernel/reference/include/m2.h'
    text = header.read_text()
    for index, name in enumerate(BODIES):
        import re
        require(re.search(r'PLANET_' + name.upper() + r'\s*=\s*' + str(index) + r'\b', text),
                'native planet identity drift: ' + name)
    retained = json.loads((ROOT / 'fixtures/kernel/m2-retained-c-v1.json').read_text())
    require(file_digest(header) == next(x['sha256'] for x in retained['sources']
            if x['path'].endswith('include/m2.h')), 'retained native header digest drift')
    table = next(t for t in retained['tables'] if t['name'] == 'planet')
    require([r[0] for r in table['rows']] == list(range(10)), 'native planet table drift')
    return {'registry_revision': retained['registry_revision'],
            'header': str(header.relative_to(ROOT)), 'header_sha256': file_digest(header),
            'native_table': table['symbol'], 'scope': table['scope'],
            'standing': 'retained-symbolic-model-not-astronomical-measurement',
            'sun_role': 'parent-not-chakra-mapped', 'non_sun_operators': 9,
            'earth_body': {'source_ref': '#2-5-0/1-0', 'role': 'grounding-anchor',
                           'outside_planet_array': True, 'is_eighth_chakra': False},
            'receiving_chakras': 7, 'epogdoon': [9, 8],
            'transpersonal_native_ids': [7, 8, 9],
            'transpersonal_meaning': 'source-preempted-not-invented'}


def produce(data, *, now=None):
    """Calculate one immutable generation; `now` is injectable for controlled tests."""
    time = request(data)
    received = now or datetime.now(timezone.utc)
    require(received.tzinfo is not None, 'receipt clock must have a timezone')
    received = received.astimezone(timezone.utc)
    age = (received - time).total_seconds()
    if data['mode'] == 'current':
        require(0 <= age <= data['max_age_seconds'], 'current epoch is future or stale')
    try:
        import kerykeion
        from kerykeion import AstrologicalSubjectFactory
        import swisseph as swe
        require(metadata.version('kerykeion') == '5.7.1' and
                metadata.version('pyswisseph') == '2.10.3.2', 'unqualified provider version')
        ephe = Path(kerykeion.__file__).parent / 'sweph'
        observer = data['observer']
        longitude, latitude, altitude = (0.0, 0.0, 0.0) if observer is None else (
            observer['longitude_degrees'], observer['latitude_degrees'], observer['altitude_metres'])
        with LOCK:
            chart = AstrologicalSubjectFactory.from_birth_data(
                name='dated-world', year=time.year, month=time.month, day=time.day,
                hour=time.hour, minute=time.minute, seconds=time.second,
                lng=longitude, lat=latitude, altitude=altitude, tz_str='UTC', online=False,
                zodiac_type=data['zodiac'], sidereal_mode=data['ayanamsha'],
                perspective_type=data['perspective'], houses_system_identifier='W',
                active_points=list(BODIES), calculate_lunar_phase=False)
            require(chart.lat == latitude and chart.lng == longitude,
                    'provider relocated observer')
            jd = swe.julday(time.year, time.month, time.day,
                           time.hour + time.minute / 60 + time.second / 3600, swe.GREG_CAL)
            require(abs(chart.julian_day - jd) < 1e-9, 'provider epoch conversion mismatch')
            # Re-establish explicit global flags; never inherit another client's mode.
            swe.set_ephe_path(str(ephe))
            swe.set_topo(longitude, latitude, altitude)
            flags = swe.FLG_SWIEPH | swe.FLG_SPEED
            if data['perspective'] == 'Topocentric':
                flags |= swe.FLG_TOPOCTR
            elif data['perspective'] == 'True Geocentric':
                flags |= swe.FLG_TRUEPOS
            if data['zodiac'] == 'Sidereal':
                swe.set_sid_mode(swe.SIDM_LAHIRI)
                flags |= swe.FLG_SIDEREAL
            bodies, used_files = [], {}
            for native_id, name in enumerate(BODIES):
                point = getattr(chart, name.lower(), None)
                require(point is not None, 'missing required body: ' + name)
                swiss_id = getattr(swe, name.upper())
                vector, returned = swe.calc_ut(jd, swiss_id, flags)
                require(len(vector) == 6 and all(math.isfinite(x) for x in vector),
                        'invalid provider vector: ' + name)
                require(abs((vector[0] - point.abs_pos + 180) % 360 - 180) < 1e-7
                        and abs(vector[3] - point.speed) < 1e-7
                        and point.retrograde == (vector[3] < 0), 'factory/backend mismatch: ' + name)
                backend = {swe.FLG_SWIEPH: 'swiss-files', swe.FLG_MOSEPH: 'moshier',
                           swe.FLG_JPLEPH: 'jpl-files'}.get(returned & (swe.FLG_SWIEPH | swe.FLG_MOSEPH | swe.FLG_JPLEPH))
                require(backend in ('swiss-files', 'moshier'), 'unsupported actual backend')
                require(data['backend_policy'] != 'require-swiss-files' or backend == 'swiss-files',
                        'Swiss files required but provider fell back: ' + name)
                files = []
                if backend == 'swiss-files':
                    # Current-file slots 0 planets, 1 Moon. Ignore unsuccessful attempted paths.
                    slot = 1 if name == 'Moon' else 0
                    path, start, end, denum = swe.get_current_file_data(slot)
                    require(path and Path(path).is_file() and start <= jd <= end,
                            'used ephemeris file cannot be verified')
                    reference = Path(path).name
                    used_files[reference] = {'name': reference, 'sha256': file_digest(path),
                                             'jd_start': start, 'jd_end': end, 'de_number': denum}
                    files.append(reference)
                bodies.append({'body': name, 'native_planet_id': native_id, 'swiss_body_id': swiss_id,
                               'longitude_degrees': vector[0], 'latitude_degrees': vector[1],
                               'distance_au': vector[2], 'longitude_speed_degrees_per_day': vector[3],
                               'latitude_speed_degrees_per_day': vector[4],
                               'radial_speed_au_per_day': vector[5], 'retrograde': vector[3] < 0,
                               'backend': backend, 'returned_flags': returned, 'data_files': files})
            ayanamsha_degrees = swe.get_ayanamsa_ut(jd) if data['zodiac'] == 'Sidereal' else None
        provider = {'name': 'Kerykeion', 'version': metadata.version('kerykeion'),
                    'wrapper': 'pyswisseph', 'wrapper_version': metadata.version('pyswisseph'),
                    'engine_version': swe.version, 'engine_sha256': file_digest(swe.__file__),
                    'factory_sha256': file_digest(Path(kerykeion.__file__).parent /
                                                  'astrological_subject_factory.py'),
                    'adapter_sha256': file_digest(__file__), 'requested_flags': flags,
                    'python_version': sys.version.split()[0], 'pytz_version': metadata.version('pytz'),
                    'network': 'disabled', 'used_data_files': list(used_files.values()),
                    'adapter_epoch_range_utc': [iso(MIN_EPOCH), iso(MAX_EPOCH)]}
    except SkyError:
        raise
    except Exception as exc:
        raise SkyError('ephemeris provider failed: ' + type(exc).__name__ + ': ' + str(exc)) from exc
    value = {'schema': SCHEMA, 'request': copy.deepcopy(data), 'epoch_utc': iso(time),
             'epoch_unix_ms': int(time.timestamp()) * 1000, 'receipt_utc': iso(received),
             'receipt_clock': 'injected-controlled-clock' if now is not None else 'host-utc-clock',
             'receipt_unix_ms': int(received.timestamp() * 1000), 'julian_day_ut_argument': jd,
             'time_scale_policy': 'UTC-as-UT argument; UT1 correction not supplied',
             'reference_frame': 'ecliptic-of-date', 'ayanamsha_degrees': ayanamsha_degrees,
             'house_policy': 'not-emitted; factory auxiliary Whole-Sign houses discarded',
             'scope': 'observer-private' if data['observer'] else 'shared-geocentric',
             'standing': 'calculated-ephemeris-not-observed-sky', 'provider': provider,
             'source_binding': source_bindings(), 'bodies': bodies}
    value['snapshot_ref'] = 'sha256:' + digest(value)
    validate_snapshot(value, now=received, require_current=data['mode'] == 'current')
    return value


def validate_snapshot(value, *, now=None, require_current=False):
    require(isinstance(value, dict) and set(value) == {
        'schema', 'request', 'epoch_utc', 'epoch_unix_ms', 'receipt_utc', 'receipt_clock',
        'receipt_unix_ms', 'julian_day_ut_argument', 'time_scale_policy', 'reference_frame',
        'ayanamsha_degrees', 'house_policy', 'scope', 'standing', 'provider', 'source_binding',
        'bodies', 'snapshot_ref'} and value.get('schema') == SCHEMA, 'unsupported snapshot')
    original = copy.deepcopy(value)
    reference = original.pop('snapshot_ref', None)
    require(reference == 'sha256:' + digest(original), 'snapshot digest mismatch')
    time = request(value['request'])
    require(value['epoch_utc'] == iso(time) and value['epoch_unix_ms'] == int(time.timestamp()) * 1000,
            'snapshot epoch mismatch')
    receipt = datetime.fromisoformat(value['receipt_utc'].replace('Z', '+00:00'))
    require(receipt.tzinfo is not None and int(receipt.timestamp() * 1000) == value['receipt_unix_ms'],
            'receipt time mismatch')
    require(value['receipt_clock'] in ('injected-controlled-clock', 'host-utc-clock'), 'invalid receipt clock')
    require(value['reference_frame'] == 'ecliptic-of-date' and
            value['time_scale_policy'] == 'UTC-as-UT argument; UT1 correction not supplied' and
            value['house_policy'] == 'not-emitted; factory auxiliary Whole-Sign houses discarded' and
            value['standing'] == 'calculated-ephemeris-not-observed-sky', 'frame/standing drift')
    expected_jd = time.timestamp() / 86400 + 2440587.5
    number(value['julian_day_ut_argument'], expected_jd - 1e-8, expected_jd + 1e-8, 'Julian argument')
    require(value['scope'] == ('observer-private' if value['request']['observer'] else 'shared-geocentric'),
            'snapshot scope mismatch')
    require(set(value['provider']) == {'name', 'version', 'wrapper', 'wrapper_version',
        'engine_version', 'engine_sha256', 'factory_sha256', 'adapter_sha256', 'requested_flags',
        'python_version', 'pytz_version', 'network', 'used_data_files', 'adapter_epoch_range_utc'},
        'unknown provider provenance fields')
    require(value['provider']['network'] == 'disabled', 'unexpected network policy')
    for key in ('engine_sha256', 'factory_sha256', 'adapter_sha256'):
        h = value['provider'][key]
        require(isinstance(h, str) and len(h) == 64 and all(c in '0123456789abcdef' for c in h),
                'invalid provider source digest')
    require([b['body'] for b in value['bodies']] == list(BODIES), 'missing/reordered body identities')
    for index, body in enumerate(value['bodies']):
        require(set(body) == {'body', 'native_planet_id', 'swiss_body_id', 'longitude_degrees',
            'latitude_degrees', 'distance_au', 'longitude_speed_degrees_per_day',
            'latitude_speed_degrees_per_day', 'radial_speed_au_per_day', 'retrograde',
            'backend', 'returned_flags', 'data_files'}, 'unknown body fields')
        require(type(body['native_planet_id']) is int and type(body['swiss_body_id']) is int
                and body['swiss_body_id'] == index, 'invalid provider identity')
        require(body['native_planet_id'] == index, 'planet array identity mismatch')
        number(body['longitude_degrees'], 0, math.nextafter(360.0, 0), 'longitude')
        number(body['latitude_degrees'], -90, 90, 'latitude')
        number(body['distance_au'], 0, 1000, 'distance')
        speed = number(body['longitude_speed_degrees_per_day'], -100, 100, 'speed')
        number(body['latitude_speed_degrees_per_day'], -100, 100, 'latitude speed')
        number(body['radial_speed_au_per_day'], -100, 100, 'radial speed')
        require(type(body['retrograde']) is bool and body['retrograde'] == (speed < 0),
                'retrograde/speed mismatch')
        flags = body['returned_flags']
        require(type(flags) is int and flags >= 0, 'invalid returned flags')
        require((flags & 7) == {'moshier': 4, 'swiss-files': 2}.get(body['backend']),
                'actual backend/flags disagree')
        require(bool(flags & 32768) == (value['request']['perspective'] == 'Topocentric') and
                bool(flags & 16) == (value['request']['perspective'] == 'True Geocentric') and
                bool(flags & 65536) == (value['request']['zodiac'] == 'Sidereal') and
                bool(flags & 256), 'coordinate policy/flags disagree')
        require(value['request']['backend_policy'] != 'require-swiss-files' or body['backend'] == 'swiss-files',
                'forbidden ephemeris fallback')
        if body['backend'] == 'moshier':
            require(not body['data_files'], 'analytical backend must not claim data files')
        else:
            require(bool(body['data_files']) and all(any(f['name'] == name
                for f in value['provider']['used_data_files']) for name in body['data_files']),
                'missing used ephemeris file provenance')
    if require_current:
        received = now or datetime.now(timezone.utc)
        age = (received - time).total_seconds()
        require(value['request']['mode'] == 'current' and
                0 <= age <= value['request']['max_age_seconds'], 'snapshot is not fresh current sky')
    return value


def attach_m2(snapshot, m2_request, *, now=None, scope='shared-geocentric'):
    """Add native observations without rewriting excitation, modes or producer standing.

M2 v1 observation timestamps are receipt times (u64); astronomical epoch stays
in this signed-epoch snapshot. This supports historical dates before 1970 without
casting negative times or pretending occurrence and astronomical time are equal.
"""
    validate_snapshot(snapshot, now=now, require_current=snapshot['request']['mode'] == 'current')
    require(scope == snapshot['scope'], 'private observer sky cannot enter a shared event')
    result = copy.deepcopy(m2_request)
    require(snapshot['source_binding'] == source_bindings(), 'native source binding is stale for M2 execution')
    require(result['schema'] == 'ql.m2-engine-request/v1' and
            result['registry_revision'] == snapshot['source_binding']['registry_revision'],
            'M2 input/sky source registry mismatch')
    received = snapshot['receipt_unix_ms']
    require(type(result['at_unix_ms']) is int and
            0 <= received <= result['at_unix_ms'] <= 9007199254740991,
            'M2 event predates sky receipt or exceeds exact integer range')
    result['world_observations'] = [
        {'planet_id': b['native_planet_id'], 'longitude_degrees': b['longitude_degrees'],
         'provider_ref': snapshot['snapshot_ref'], 'source_revision': snapshot['provider']['adapter_sha256'],
         'observed_at_unix_ms': received} for b in snapshot['bodies']]
    return result


def execute_m2(snapshot, m2_request, executable, *, now=None, scope='shared-geocentric'):
    """Use the existing native Rust producer outside any audio/render callback.

    The executable is a host-selected trusted installation, not an agent-supplied
    shell expression. No native output is rebranded as authenticated observation.
    A failed/partial producer returns no composed event.
    """
    import tempfile
    request_value = attach_m2(snapshot, m2_request, now=now, scope=scope)
    with tempfile.TemporaryDirectory(prefix='ql-k8-sky-') as directory:
        path = Path(directory) / 'request.json'
        path.write_text(json.dumps(request_value, allow_nan=False))
        try:
            run = subprocess.run([str(Path(executable).resolve()), str(path)], capture_output=True,
                                 check=True, timeout=120)
            require(len(run.stdout) <= 32 * 1024 * 1024, 'M2 output exceeds 32 MiB')
            frame = json.loads(run.stdout)
        except (OSError, subprocess.SubprocessError, ValueError) as exc:
            raise SkyError('native M2 execution failed: ' + type(exc).__name__) from exc
    require(frame['schema'] == 'ql.m2-engine/v1' and
            frame['identity'] == request_value['stamp']['identity'] and
            frame['registry_revision'] == request_value['registry_revision'] and
            frame['at_unix_ms'] == request_value['at_unix_ms'], 'M2 output identity drift')
    require([w['observation'] for w in frame['world']] == request_value['world_observations'],
            'M2 did not retain the exact sky observations')
    require(frame['modal']['coefficients'] == request_value['modal_coefficients'] and
            frame['resonator'] == request_value['resonator'], 'M2 input preservation failed')
    return {'schema': 'ql.sky-m2-event/v1', 'scope': scope,
            'sky': copy.deepcopy(snapshot), 'm2_input': request_value, 'm2': frame,
            'standing': 'native-source-qualified-computation-not-empirical-correspondence-proof'}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('request', type=Path)
    parser.add_argument('--m2-request', type=Path)
    parser.add_argument('--m2-executable', type=Path)
    args = parser.parse_args()
    try:
        require(args.request.stat().st_size <= 65536, 'sky request exceeds 64 KiB')
        result = produce(json.loads(args.request.read_text()))
        if args.m2_request:
            require(args.m2_request.stat().st_size <= 32 * 1024 * 1024, 'M2 request exceeds 32 MiB')
            m2_input = json.loads(args.m2_request.read_text())
            result = execute_m2(result, m2_input, args.m2_executable) if args.m2_executable else {
                'sky': result, 'm2_request': attach_m2(result, m2_input)}
        else:
            require(args.m2_executable is None, '--m2-executable requires --m2-request')
        print(json.dumps(result, sort_keys=True, allow_nan=False))
    except (SkyError, ValueError, OSError, KeyError, TypeError) as exc:
        print(json.dumps({'schema': 'ql.sky-error/v1', 'error': str(exc), 'snapshot': None}), file=sys.stderr)
        return 2
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
