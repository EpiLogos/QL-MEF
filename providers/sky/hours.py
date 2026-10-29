#!/usr/bin/env python3
"""The day at a place: the sunrise before an instant, the sunset after it and the next sunrise.

The planets of the day and of the hour are counted from these three times at a
place (DLW manual §22.1; #254 D12: the observer's place and time, Nara's own
or the world's declared place). This adapter states only the astronomy. The
Swiss Ephemeris engine the dated sky already runs (kerykeion_snapshot.py)
finds the Sun's rising and setting at the place: the upper limb on the
horizon with the engine's standard refraction. No ruler, order or meaning is
computed here; QL's M2 reads the rulers from the map (crates/ql-mef/src/m2_hours.rs).
"""
from __future__ import annotations

import argparse
import copy
from datetime import datetime, timedelta, timezone
from importlib import metadata
import json
from pathlib import Path
import sys
from zoneinfo import ZoneInfo

sys.path.insert(0, str(Path(__file__).resolve().parent))
from kerykeion_snapshot import LOCK, SkyError, digest, epoch, file_digest, iso, number, require  # noqa: E402

SCHEMA = 'ql.sky-day/v1'
REQUEST = 'ql.sky-day-request/v1'
SCOPES = ('world-declared', 'observer-private')
METHOD = 'swiss-ephemeris rise_trans: Sun upper limb on the horizon, standard refraction'
MINUTE = 1.0 / 1440.0


def place(data):
    require(isinstance(data, dict) and set(data) == {
        'reference', 'label', 'latitude_degrees', 'longitude_degrees', 'altitude_metres', 'timezone'},
        'a place names its reference, label, latitude, longitude, altitude and IANA timezone')
    require(isinstance(data['reference'], str) and 0 < len(data['reference']) <= 512, 'invalid place reference')
    require(isinstance(data['label'], str) and 0 < len(data['label']) <= 256, 'invalid place label')
    number(data['latitude_degrees'], -90, 90, 'place latitude')
    number(data['longitude_degrees'], -180, 180, 'place longitude')
    number(data['altitude_metres'], -500, 10000, 'place altitude')
    try:
        zone = ZoneInfo(data['timezone'])
    except (ValueError, TypeError, KeyError) as exc:
        raise SkyError('invalid place timezone') from exc
    return zone


def request(data):
    require(isinstance(data, dict) and set(data) == {'schema', 'epoch', 'place', 'scope', 'backend_policy'},
            'invalid day request fields')
    require(data['schema'] == REQUEST, 'unsupported day request')
    time = epoch(data['epoch'])
    zone = place(data['place'])
    require(data['scope'] in SCOPES, 'a day is counted at the world\'s declared place or at a private observer\'s')
    require(data['backend_policy'] in ('require-swiss-files', 'allow-moshier'),
            'explicit ephemeris fallback policy required')
    return time, zone


def unix_ms(jd):
    return round((jd - 2440587.5) * 86400000)


def utc_of(ms):
    return datetime.fromtimestamp(ms / 1000, tz=timezone.utc).replace(microsecond=0)


def produce(data):
    """The three times around one instant at one place. A place where the Sun does not rise or set that day is refused."""
    time, zone = request(data)
    try:
        import kerykeion
        import swisseph as swe
        require(metadata.version('kerykeion') == '5.7.1' and
                metadata.version('pyswisseph') == '2.10.3.2', 'unqualified provider version')
        where = data['place']
        geopos = (where['longitude_degrees'], where['latitude_degrees'], where['altitude_metres'])
        jd = swe.julday(time.year, time.month, time.day,
                        time.hour + time.minute / 60 + time.second / 3600, swe.GREG_CAL)
        with LOCK:
            swe.set_ephe_path(str(Path(kerykeion.__file__).parent / 'sweph'))
            flags = swe.FLG_SWIEPH

            def event(start, rsmi, what):
                res, tret = swe.rise_trans(start, swe.SUN, rsmi, geopos, 0.0, 0.0, flags)
                require(res == 0, 'the Sun does not ' + what + ' at ' + where['label'] +
                        ' near this day (circumpolar): no hours are counted there')
                return tret[0]

            # The last sunrise at or before the instant, and the next one after it.
            rise = event(jd - 1.5, swe.CALC_RISE, 'rise')
            require(rise <= jd, 'no sunrise in the day and a half before the instant')
            following = event(rise + MINUTE, swe.CALC_RISE, 'rise')
            while following <= jd:
                rise, following = following, event(following + MINUTE, swe.CALC_RISE, 'rise')
            sunset = event(rise + MINUTE, swe.CALC_SET, 'set')
            require(rise < sunset < following, 'the Sun does not rise and set once between two sunrises here')
            vector, returned = swe.calc_ut(rise, swe.SUN, flags)
            backend = {swe.FLG_SWIEPH: 'swiss-files', swe.FLG_MOSEPH: 'moshier', swe.FLG_JPLEPH: 'jpl-files'}.get(
                returned & (swe.FLG_SWIEPH | swe.FLG_MOSEPH | swe.FLG_JPLEPH))
            require(backend in ('swiss-files', 'moshier'), 'unsupported actual backend')
            require(data['backend_policy'] != 'require-swiss-files' or backend == 'swiss-files',
                    'Swiss files required but the provider fell back')
            provider = {'name': 'Swiss Ephemeris', 'wrapper': 'pyswisseph',
                        'wrapper_version': metadata.version('pyswisseph'), 'engine_version': swe.version,
                        'kerykeion_version': metadata.version('kerykeion'), 'backend': backend,
                        'adapter_sha256': file_digest(__file__), 'network': 'disabled'}
    except SkyError:
        raise
    except Exception as exc:
        raise SkyError('ephemeris provider failed: ' + type(exc).__name__ + ': ' + str(exc)) from exc
    times = {name: unix_ms(value) for name, value in (('sunrise', rise), ('sunset', sunset), ('next_sunrise', following))}
    local = utc_of(times['sunrise']).astimezone(zone)
    value = {'schema': SCHEMA, 'request': copy.deepcopy(data), 'epoch_utc': iso(time),
             'epoch_unix_ms': int(time.timestamp()) * 1000, 'scope': data['scope'], 'place': copy.deepcopy(data['place']),
             'sunrise_utc': iso(utc_of(times['sunrise'])), 'sunrise_unix_ms': times['sunrise'],
             'sunset_utc': iso(utc_of(times['sunset'])), 'sunset_unix_ms': times['sunset'],
             'next_sunrise_utc': iso(utc_of(times['next_sunrise'])), 'next_sunrise_unix_ms': times['next_sunrise'],
             'sunrise_local_date': local.date().isoformat(), 'sunrise_iso_weekday': local.isoweekday(),
             'method': METHOD, 'time_scale_policy': 'UTC-as-UT argument; UT1 correction not supplied',
             'standing': 'calculated-ephemeris-not-observed-sky', 'provider': provider}
    value['day_ref'] = 'sha256:' + digest(value)
    validate(value)
    return value


def validate(value):
    require(isinstance(value, dict) and value.get('schema') == SCHEMA, 'unsupported day reading')
    original = copy.deepcopy(value)
    reference = original.pop('day_ref', None)
    require(reference == 'sha256:' + digest(original), 'day digest mismatch')
    time, zone = request(value['request'])
    require(value['epoch_utc'] == iso(time) and value['epoch_unix_ms'] == int(time.timestamp()) * 1000, 'day epoch mismatch')
    require(value['place'] == value['request']['place'] and value['scope'] == value['request']['scope'], 'day place mismatch')
    rise, sunset, following = value['sunrise_unix_ms'], value['sunset_unix_ms'], value['next_sunrise_unix_ms']
    require(all(type(x) is int for x in (rise, sunset, following)), 'day times are whole milliseconds')
    require(rise <= value['epoch_unix_ms'] < following and rise < sunset < following, 'day times out of order')
    require(following - rise < 2 * 86400000, 'a day runs less than two days from sunrise to sunrise')
    local = utc_of(rise).astimezone(zone)
    require(value['sunrise_local_date'] == local.date().isoformat() and
            value['sunrise_iso_weekday'] == local.isoweekday(), 'day weekday mismatch')
    for key in ('sunrise', 'sunset', 'next_sunrise'):
        require(value[key + '_utc'] == iso(utc_of(value[key + '_unix_ms'])), key + ' mismatch')
    return value


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('request', type=Path)
    args = parser.parse_args()
    try:
        if str(args.request) == '-':
            source = sys.stdin.buffer.read(65537)
        else:
            source = args.request.read_bytes()
        require(len(source) <= 65536, 'day request exceeds 64 KiB')
        print(json.dumps(produce(json.loads(source)), sort_keys=True, allow_nan=False))
    except (SkyError, ValueError, OSError, KeyError, TypeError) as exc:
        print(json.dumps({'schema': 'ql.sky-error/v1', 'error': str(exc), 'day': None}))
        return 2
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
