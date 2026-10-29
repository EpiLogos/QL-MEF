from datetime import datetime, timedelta, timezone
import copy
import json
import math
from pathlib import Path
import subprocess
import sys
import unittest

sys.path.insert(0, str(Path(__file__).parent))
import hours
from kerykeion_snapshot import SkyError

GREENWICH = {'reference': 'controlled:place:greenwich', 'label': 'Greenwich', 'latitude_degrees': 51.4769,
             'longitude_degrees': -0.0005, 'altitude_metres': 46.0, 'timezone': 'Europe/London'}


def ask(epoch, place=GREENWICH, scope='world-declared'):
    return {'schema': hours.REQUEST, 'epoch': epoch, 'place': copy.deepcopy(place), 'scope': scope,
            'backend_policy': 'allow-moshier'}


def noaa_sunrise_sunset(date, latitude, longitude):
    """An independent low-precision cross-check (the NOAA solar formula; no Swiss Ephemeris code path).

    The fractional year gives the equation of time and the declination; the hour angle of the
    -0.833 degree zenith (upper limb with standard refraction) gives the rise and set, in UTC minutes.
    Good to a minute or two at mid latitudes.
    """
    day = date.timetuple().tm_yday
    gamma = 2 * math.pi / 365 * (day - 1)
    eqtime = 229.18 * (0.000075 + 0.001868 * math.cos(gamma) - 0.032077 * math.sin(gamma)
                       - 0.014615 * math.cos(2 * gamma) - 0.040849 * math.sin(2 * gamma))
    decl = (0.006918 - 0.399912 * math.cos(gamma) + 0.070257 * math.sin(gamma) - 0.006758 * math.cos(2 * gamma)
            + 0.000907 * math.sin(2 * gamma) - 0.002697 * math.cos(3 * gamma) + 0.00148 * math.sin(3 * gamma))
    lat = math.radians(latitude)
    ha = math.degrees(math.acos(math.cos(math.radians(90.833)) / (math.cos(lat) * math.cos(decl)) - math.tan(lat) * math.tan(decl)))
    midnight = datetime(date.year, date.month, date.day, tzinfo=timezone.utc)
    rise = midnight + timedelta(minutes=720 - 4 * (longitude + ha) - eqtime)
    sett = midnight + timedelta(minutes=720 - 4 * (longitude - ha) - eqtime)
    return rise, sett


def parse(text):
    return datetime.fromisoformat(text.replace('Z', '+00:00'))


class DayAtAPlace(unittest.TestCase):
    def test_noon_stands_between_its_sunrise_and_sunset_by_an_independent_formula(self):
        day = hours.produce(ask('2026-09-28T12:00:00Z'))
        self.assertEqual(day['schema'], 'ql.sky-day/v1')
        rise, sett, following = parse(day['sunrise_utc']), parse(day['sunset_utc']), parse(day['next_sunrise_utc'])
        self.assertLess(rise, parse(day['epoch_utc']))
        self.assertLess(parse(day['epoch_utc']), sett)
        ref_rise, ref_set = noaa_sunrise_sunset(rise.date(), GREENWICH['latitude_degrees'], GREENWICH['longitude_degrees'])
        self.assertLess(abs((rise - ref_rise).total_seconds()), 150, (rise, ref_rise))
        self.assertLess(abs((sett - ref_set).total_seconds()), 150, (sett, ref_set))
        next_rise, _ = noaa_sunrise_sunset(following.date(), GREENWICH['latitude_degrees'], GREENWICH['longitude_degrees'])
        self.assertLess(abs((following - next_rise).total_seconds()), 150)
        # 28 September 2026 is a Monday; the hours are the world's, at its declared place.
        self.assertEqual((day['sunrise_local_date'], day['sunrise_iso_weekday']), ('2026-09-28', 1))
        self.assertEqual(day['scope'], 'world-declared')
        self.assertEqual(day['place'], GREENWICH)
        self.assertEqual(hours.validate(day), day)

    def test_before_sunrise_the_day_is_the_one_that_began_at_the_last_sunrise(self):
        day = hours.produce(ask('2026-09-28T03:00:00Z'))
        self.assertEqual((day['sunrise_local_date'], day['sunrise_iso_weekday']), ('2026-09-27', 7))
        self.assertLess(day['sunset_unix_ms'], day['epoch_unix_ms'])
        self.assertLess(day['epoch_unix_ms'], day['next_sunrise_unix_ms'])

    def test_the_weekday_is_the_places_own_civil_date_of_its_sunrise(self):
        # Auckland's sunrise on its Tuesday 29 September falls on the UTC Monday.
        auckland = {'reference': 'controlled:place:auckland', 'label': 'Auckland', 'latitude_degrees': -36.8485,
                    'longitude_degrees': 174.7633, 'altitude_metres': 0.0, 'timezone': 'Pacific/Auckland'}
        day = hours.produce(ask('2026-09-28T23:00:00Z', auckland, 'observer-private'))
        self.assertEqual(day['sunrise_local_date'], '2026-09-29')
        self.assertEqual(day['sunrise_iso_weekday'], 2)
        self.assertEqual(parse(day['sunrise_utc']).date().isoformat(), '2026-09-28')

    def test_a_circumpolar_day_counts_no_hours(self):
        north = dict(GREENWICH, reference='controlled:place:svalbard', label='Longyearbyen', latitude_degrees=78.22,
                     longitude_degrees=15.65, timezone='Arctic/Longyearbyen')
        with self.assertRaisesRegex(SkyError, 'circumpolar'):
            hours.produce(ask('2026-12-21T12:00:00Z', north))

    def test_a_request_names_its_place_and_scope_completely(self):
        for broken in (dict(ask('2026-09-28T12:00:00Z'), scope='shared-geocentric'),
                       dict(ask('2026-09-28T12:00:00Z'), place={k: v for k, v in GREENWICH.items() if k != 'timezone'}),
                       dict(ask('2026-09-28T12:00:00Z'), place=dict(GREENWICH, latitude_degrees=91)),
                       dict(ask('2026-09-28T12:00:00Z'), place=dict(GREENWICH, timezone='Mars/Olympus')),
                       dict(ask('2026-09-28T12:00:00'), extra=True)):
            with self.assertRaises(SkyError):
                hours.request(broken)

    def test_a_changed_reading_is_refused(self):
        day = hours.produce(ask('2026-09-28T12:00:00Z'))
        changed = dict(day, sunrise_iso_weekday=3)
        with self.assertRaisesRegex(SkyError, 'digest'):
            hours.validate(changed)

    def test_the_script_answers_on_stdin_and_refuses_in_words(self):
        script = Path(hours.__file__)
        ok = subprocess.run([sys.executable, str(script), '-'], input=json.dumps(ask('2026-09-28T12:00:00Z')).encode(),
                            capture_output=True, check=False)
        self.assertEqual(ok.returncode, 0, ok.stdout + ok.stderr)
        self.assertEqual(json.loads(ok.stdout)['schema'], 'ql.sky-day/v1')
        bad = subprocess.run([sys.executable, str(script), '-'], input=b'{"schema":"ql.sky-day-request/v1"}',
                             capture_output=True, check=False)
        self.assertEqual(bad.returncode, 2)
        self.assertIn('error', json.loads(bad.stdout))


if __name__ == '__main__':
    unittest.main()
