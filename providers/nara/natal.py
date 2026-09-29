#!/usr/bin/env python3
"""Offline, source-retaining natal calculation through the qualified sky provider.

One JSON request on stdin, one JSON result on stdout. Swiss/Kerykeion globals
are confined to this worker process; imports of produce use the sky worker lock.
This boundary calculates astronomy and an explicitly configured natal chart.
It does not infer personality, prescribe identity weights or map chakral centres.
"""
from __future__ import annotations

import copy
from datetime import date, datetime, timedelta, timezone
from importlib import metadata
import json
import inspect
from pathlib import Path
import re
import sys
from zoneinfo import ZoneInfo, ZoneInfoNotFoundError

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'sky'))
import kerykeion_snapshot as sky_provider

REQUEST = 'ql.nara-natal-request/v1'
SCHEMA = 'ql.nara-natal/v1'
ANGLES = ('Ascendant', 'Medium_Coeli', 'Descendant', 'Imum_Coeli')
HOUSES = ('first', 'second', 'third', 'fourth', 'fifth', 'sixth',
          'seventh', 'eighth', 'ninth', 'tenth', 'eleventh', 'twelfth')


class NatalError(ValueError):
    """Invalid or unresolvable source; no apparent chart is returned."""


def require(condition, message):
    if not condition:
        raise NatalError(message)


def fields(value, expected, label):
    require(isinstance(value, dict) and set(value) == set(expected.split()),
            'invalid-fields: ' + label)


def text(value, label, maximum=512):
    require(isinstance(value, str) and bool(value.strip()) and len(value) <= maximum
            and not any(ord(c) < 32 for c in value), 'invalid-text: ' + label)


def validate_place(place):
    if place is None:
        return None
    fields(place, 'label latitude_degrees longitude_degrees timezone source_ref', 'birthplace')
    for field in ('label', 'timezone', 'source_ref'):
        text(place[field], 'birthplace.' + field)
    try:
        sky_provider.number(place['latitude_degrees'], -90, 90, 'birthplace latitude')
        sky_provider.number(place['longitude_degrees'], -180, 180, 'birthplace longitude')
        return ZoneInfo(place['timezone'])
    except (sky_provider.SkyError, ZoneInfoNotFoundError, ValueError) as exc:
        raise NatalError('invalid-birthplace: ' + str(exc)) from exc


def validate(data):
    fields(data, 'schema person_ref source_revision birth backend_policy', 'request')
    require(data['schema'] == REQUEST, 'unsupported-request-schema')
    text(data['person_ref'], 'person_ref')
    text(data['source_revision'], 'source_revision')
    require(data['backend_policy'] in ('allow-moshier', 'require-swiss-files'),
            'explicit-supported-backend-policy-required')
    birth = data['birth']
    fields(birth, 'date time precision uncertainty_minutes fold place', 'birth')
    place = birth['place']
    zone = validate_place(place)
    precision = birth['precision']
    require(precision in ('exact', 'approximate', 'unknown'), 'invalid-time-precision')
    uncertainty = birth['uncertainty_minutes']
    if precision == 'approximate':
        require(type(uncertainty) is int and 1 <= uncertainty <= 1440,
                'approximate-time-requires-uncertainty-minutes-in-1..1440')
    else:
        require(uncertainty is None, 'uncertainty-only-valid-for-approximate-time')
    fold = birth['fold']
    require(fold is None or (type(fold) is int and fold in (0, 1)), 'invalid-fold')
    if precision == 'unknown':
        require(birth['time'] is None and fold is None, 'unknown-time-must-not-supply-time-or-fold')
    else:
        require(isinstance(birth['time'], str) and
                re.fullmatch(r'\d{2}:\d{2}(:\d{2})?', birth['time']), 'invalid-local-time')
        try:
            datetime.strptime(birth['time'], '%H:%M:%S' if len(birth['time']) == 8 else '%H:%M')
        except ValueError as exc:
            raise NatalError('invalid-local-time') from exc
    if birth['date'] is None:
        return None, 'incomplete-birth-date'
    require(isinstance(birth['date'], str) and
            re.fullmatch(r'\d{4}(-\d{2}(-\d{2})?)?', birth['date']), 'invalid-birth-date')
    pieces = [int(x) for x in birth['date'].split('-')]
    require(1800 <= pieces[0] < 2400, 'birth-date-outside-provider-range-1800..2399')
    try:
        civil = date(pieces[0], pieces[1] if len(pieces) > 1 else 1,
                     pieces[2] if len(pieces) > 2 else 1)
    except ValueError as exc:
        raise NatalError('invalid-birth-date') from exc
    if len(pieces) < 3:
        return None, 'incomplete-birth-date'
    if place is None:
        return None, 'missing-birthplace'
    if precision == 'unknown':
        return None, 'unknown-birth-time'
    naive = datetime.fromisoformat(civil.isoformat() + 'T' + birth['time'])
    candidates = {}
    for candidate_fold in (0, 1):
        candidate = naive.replace(tzinfo=zone, fold=candidate_fold)
        utc = candidate.astimezone(timezone.utc)
        # Roundtrip detects gaps; distinct UTC epochs detect folds independently
        # of DST names, including historic non-DST offset changes.
        if utc.astimezone(zone).replace(tzinfo=None) == naive:
            candidates[candidate_fold] = utc
    require(bool(candidates), 'nonexistent-local-time: correct the clock time or timezone')
    ambiguous = len(set(candidates.values())) > 1
    require(not ambiguous or fold is not None,
            'ambiguous-local-time: choose fold 0 (earlier) or 1 (later)')
    require(ambiguous or fold is None, 'fold-only-valid-for-ambiguous-local-time')
    selected = candidates[fold if ambiguous else 0]
    require(sky_provider.MIN_EPOCH <= selected < sky_provider.MAX_EPOCH,
            'resolved-epoch-outside-provider-range')
    minutes = uncertainty if precision == 'approximate' else 0
    low, high = selected - timedelta(minutes=minutes), selected + timedelta(minutes=minutes)
    require(sky_provider.MIN_EPOCH <= low and high < sky_provider.MAX_EPOCH,
            'uncertainty-window-outside-provider-range')
    local = selected.astimezone(zone)
    return {'epoch_utc': sky_provider.iso(selected),
            'local_datetime': local.isoformat(), 'timezone': place['timezone'],
            'utc_offset_seconds': int(local.utcoffset().total_seconds()),
            'fold': fold if ambiguous else None,
            'precision': precision, 'uncertainty_minutes': uncertainty,
            'uncertainty_window_utc': [sky_provider.iso(low), sky_provider.iso(high)]
                if minutes else None,
            'policy': 'declared-local-time; IANA-roundtrip; no-time-imputation'}, None


def draw_chart(data, resolved, sky):
    from kerykeion import AstrologicalSubjectFactory, ChartDataFactory, ChartDrawer
    epoch = datetime.fromisoformat(resolved['epoch_utc'].replace('Z', '+00:00'))
    place = data['birth']['place']
    approximate = data['birth']['precision'] == 'approximate'
    title = 'Approximate natal chart' if approximate else 'Natal chart'
    # Fixed display strings keep private source labels out of template markup.
    # UTC avoids the provider's separate pytz ambiguity heuristics. The exact
    # entered local civil time and fold remain explicit in time_resolution.
    subject = AstrologicalSubjectFactory.from_birth_data(
        name=title, year=epoch.year, month=epoch.month, day=epoch.day,
        hour=epoch.hour, minute=epoch.minute, seconds=epoch.second,
        city='Birth location', nation='', lng=place['longitude_degrees'],
        lat=place['latitude_degrees'], tz_str='UTC', online=False,
        zodiac_type='Tropical', perspective_type='Apparent Geocentric',
        houses_system_identifier='W', active_points=list(sky_provider.BODIES + ANGLES),
        calculate_lunar_phase=False)
    require(subject.lat == place['latitude_degrees'] and subject.lng == place['longitude_degrees'],
            'provider-relocated-birthplace')
    require(abs(subject.julian_day - sky['julian_day_ut_argument']) < 1e-9,
            'chart-epoch-disagrees-with-native-sky')
    bodies = []
    for native in sky['bodies']:
        point = getattr(subject, native['body'].lower())
        require(abs((point.abs_pos - native['longitude_degrees'] + 180) % 360 - 180) < 1e-7,
                'chart-planet-disagrees-with-native-sky: ' + native['body'])
        bodies.append(point.model_dump(mode='json'))
    chart_data = ChartDataFactory.create_natal_chart_data(
        subject, active_points=list(sky_provider.BODIES + ANGLES))
    svg = ChartDrawer(chart_data, theme='dark', custom_title=title).generate_svg_string()
    require(isinstance(svg, str) and '<svg' in svg and len(svg) <= 4 * 1024 * 1024,
            'invalid-provider-chart')
    import hashlib
    return {'media_type': 'image/svg+xml', 'svg': svg,
            'sha256': hashlib.sha256(svg.encode()).hexdigest(),
            'generator': 'Kerykeion.ChartDataFactory+ChartDrawer',
            'precision': data['birth']['precision'], 'conditional': approximate,
            'uncertainty_minutes': data['birth']['uncertainty_minutes'],
            'uncertainty_policy': 'chart at declared time; uncertainty is not an exact chart',
            'display_time_basis': 'UTC; entered civil time retained in time_resolution',
            'zodiac': 'Tropical', 'perspective': 'Apparent Geocentric',
            'houses_system': 'W', 'houses_system_name': 'Whole Sign',
            'house_policy': 'explicit provider chart convention, not Nara composition authority',
            'bodies': bodies,
            'houses': [getattr(subject, name + '_house').model_dump(mode='json') for name in HOUSES],
            'angles': [getattr(subject, name.lower()).model_dump(mode='json') for name in ANGLES],
            'aspects': [point.model_dump(mode='json') for point in chart_data.aspects],
            'renderer_sha256': sky_provider.file_digest(inspect.getfile(ChartDrawer))}


def produce(data):
    resolved, reason = validate(data)
    result = {'schema': SCHEMA, 'request': copy.deepcopy(data),
              'request_ref': 'sha256:' + sky_provider.digest(data),
              'status': 'unavailable', 'reason': reason,
              'epoch_utc': resolved['epoch_utc'] if resolved else None,
              'time_resolution': resolved, 'sky': None, 'chart': None,
              'provider': {'name': 'Kerykeion', 'kerykeion_version': metadata.version('kerykeion'),
                           'pyswisseph_version': metadata.version('pyswisseph'),
                           'adapter_sha256': sky_provider.file_digest(__file__),
                           'network': 'disabled'},
              'standing': 'calculated-natal-source; no-personality-or-centre-inference'}
    if resolved is None:
        return result
    sky_request = {'schema': sky_provider.REQUEST, 'epoch': resolved['local_datetime'],
                   'timezone': resolved['timezone'], 'mode': 'historical',
                   'perspective': 'Apparent Geocentric', 'zodiac': 'Tropical',
                   'ayanamsha': None, 'observer': None, 'max_age_seconds': 86400,
                   'backend_policy': data['backend_policy']}
    with sky_provider.LOCK:
        result['sky'] = sky_provider.produce(sky_request)
        # Kerykeion silently relocates polar births; preserve valid coordinates
        # and calculated planets while refusing a chart at a different place.
        if abs(data['birth']['place']['latitude_degrees']) > 66:
            result.update(status='partial', reason='chart-latitude-outside-unadjusted-provider-range')
        else:
            result['chart'] = draw_chart(data, resolved, result['sky'])
            result.update(status='available', reason=None)
    return result


def unique_object(pairs):
    result = {}
    for key, value in pairs:
        require(key not in result, 'duplicate-json-field: ' + key)
        result[key] = value
    return result


def main():
    try:
        raw = sys.stdin.buffer.read(65537)
        require(len(raw) <= 65536, 'request-exceeds-64-KiB')
        value = json.loads(raw, object_pairs_hook=unique_object,
                           parse_constant=lambda value: (_ for _ in ()).throw(NatalError('nonfinite-json-number')))
        result = produce(value)
        print(json.dumps(result, ensure_ascii=False, allow_nan=False))
        return 0
    except (NatalError, sky_provider.SkyError, ValueError, TypeError, KeyError,
            ImportError, metadata.PackageNotFoundError) as exc:
        print(json.dumps({'schema': 'ql.nara-natal-error/v1', 'error': str(exc)}, allow_nan=False))
        return 2
    except Exception as exc:
        # Preserve a machine-readable process boundary for operational failures.
        # Do not publish a partial chart or dump private source via a traceback.
        print(json.dumps({'schema': 'ql.nara-natal-error/v1',
                          'error': 'provider-failed: ' + type(exc).__name__}, allow_nan=False))
        return 3


if __name__ == '__main__':
    raise SystemExit(main())
