"""Real Kerykeion/Swiss integration tests. No provider mocks or network calls."""
import copy
import json
from pathlib import Path
import subprocess
import sys
import unittest
import xml.etree.ElementTree as ET

sys.path.insert(0, str(Path(__file__).resolve().parent))
import natal


def request():
    return {
        'schema': 'ql.nara-natal-request/v1', 'person_ref': 'test:nara:one',
        'source_revision': 'controlled-profile-v1', 'backend_policy': 'allow-moshier',
        'birth': {'date': '1990-06-15', 'time': '12:30:00', 'precision': 'exact',
                  'uncertainty_minutes': None, 'fold': None,
                  'place': {'label': 'London', 'latitude_degrees': 51.5074,
                            'longitude_degrees': -0.1278, 'timezone': 'Europe/London',
                            'source_ref': 'test:manually-entered-coordinates'}}}


class NatalTest(unittest.TestCase):
    def test_real_chart_and_native_planets_agree(self):
        result = natal.produce(request())
        self.assertEqual(result['status'], 'available')
        self.assertEqual(result['epoch_utc'], '1990-06-15T11:30:00Z')
        self.assertEqual(result['provider']['kerykeion_version'], '5.7.1')
        self.assertEqual(result['sky']['source_binding']['receiving_chakras'], 7)
        self.assertEqual(len(result['chart']['houses']), 12)
        self.assertEqual(len(result['chart']['angles']), 4)
        self.assertEqual(len(result['chart']['bodies']), 10)
        self.assertTrue(ET.fromstring(result['chart']['svg']).tag.endswith('svg'))
        self.assertGreater(len(result['chart']['svg']), 100000)
        for native, chart in zip(result['sky']['bodies'], result['chart']['bodies']):
            self.assertAlmostEqual(native['longitude_degrees'], chart['abs_pos'], places=7)
        self.assertFalse(result['chart']['conditional'])

    def test_correction_recomputes_and_reopen_keeps_input_identity(self):
        source = request()
        first = natal.produce(source)
        reopened = natal.produce(json.loads(json.dumps(source)))
        self.assertEqual(first['request_ref'], reopened['request_ref'])
        self.assertEqual(first['chart']['sha256'], reopened['chart']['sha256'])
        self.assertEqual(source, first['request'])
        corrected = copy.deepcopy(source)
        corrected['source_revision'] = 'controlled-profile-v2'
        corrected['birth']['time'] = '16:30:00'
        second = natal.produce(corrected)
        self.assertNotEqual(first['request_ref'], second['request_ref'])
        self.assertNotEqual(first['chart']['angles'][0]['abs_pos'], second['chart']['angles'][0]['abs_pos'])
        self.assertNotEqual(first['sky']['bodies'][1]['longitude_degrees'], second['sky']['bodies'][1]['longitude_degrees'])

    def test_second_person_and_place_correction_preserve_each_source(self):
        first_source = request()
        second_source = copy.deepcopy(first_source)
        second_source['person_ref'] = 'test:nara:two'
        second_source['birth']['place'].update(
            label='New York', latitude_degrees=40.7128,
            longitude_degrees=-74.0060, timezone='America/New_York')
        first = natal.produce(first_source)
        second = natal.produce(second_source)
        self.assertEqual(first['request']['person_ref'], 'test:nara:one')
        self.assertEqual(second['request']['person_ref'], 'test:nara:two')
        self.assertEqual(second['epoch_utc'], '1990-06-15T16:30:00Z')
        self.assertNotEqual(first['chart']['sha256'], second['chart']['sha256'])
        self.assertNotEqual(first['sky']['bodies'][1]['longitude_degrees'],
                            second['sky']['bodies'][1]['longitude_degrees'])
        corrected = copy.deepcopy(first_source)
        corrected['source_revision'] = 'controlled-profile-v2'
        corrected['birth']['place'].update(latitude_degrees=0, longitude_degrees=0)
        equatorial = natal.produce(corrected)
        self.assertEqual(equatorial['status'], 'available')
        self.assertNotEqual(first['chart']['angles'][0]['abs_pos'],
                            equatorial['chart']['angles'][0]['abs_pos'])
        self.assertEqual(first['sky']['bodies'][1]['longitude_degrees'],
                         equatorial['sky']['bodies'][1]['longitude_degrees'])

    def test_unknown_time_never_invents_noon_chart(self):
        source = request()
        source['birth'].update(time=None, precision='unknown')
        result = natal.produce(source)
        self.assertEqual(result['status'], 'unavailable')
        self.assertIsNone(result['chart'])
        self.assertIsNone(result['sky'])
        self.assertIsNone(result['epoch_utc'])
        self.assertEqual(result['request']['birth']['date'], '1990-06-15')

    def test_partial_date_retained(self):
        source = request()
        source['birth'].update(date='1990-06', time=None, precision='unknown')
        result = natal.produce(source)
        self.assertEqual(result['reason'], 'incomplete-birth-date')
        self.assertIsNone(result['chart'])

    def test_missing_birthplace_is_retained_without_chart(self):
        source = request()
        source['birth']['place'] = None
        result = natal.produce(source)
        self.assertEqual(result['reason'], 'missing-birthplace')
        self.assertEqual(result['status'], 'unavailable')
        self.assertIsNone(result['chart'])
        self.assertEqual(result['request']['birth']['date'], '1990-06-15')
        source['birth']['date'] = '1990-02-31'
        with self.assertRaises(natal.NatalError):
            natal.produce(source)

    def test_approximate_chart_is_conditional(self):
        source = request()
        source['birth'].update(precision='approximate', uncertainty_minutes=30)
        result = natal.produce(source)
        self.assertTrue(result['chart']['conditional'])
        self.assertEqual(result['chart']['uncertainty_minutes'], 30)
        self.assertIn('Approximate', result['chart']['svg'])
        self.assertEqual(result['time_resolution']['uncertainty_window_utc'],
                         ['1990-06-15T11:00:00Z', '1990-06-15T12:00:00Z'])

    def test_dst_gap_is_rejected(self):
        source = request()
        source['birth'].update(date='2024-03-31', time='01:30')
        with self.assertRaisesRegex(natal.NatalError, 'nonexistent-local-time'):
            natal.produce(source)

    def test_dst_fold_requires_explicit_choice_and_changes_result(self):
        source = request()
        source['birth'].update(date='2024-10-27', time='01:30')
        with self.assertRaisesRegex(natal.NatalError, 'ambiguous-local-time'):
            natal.produce(source)
        source['birth']['fold'] = 0
        first = natal.produce(source)
        source['birth']['fold'] = 1
        second = natal.produce(source)
        self.assertEqual(first['epoch_utc'], '2024-10-27T00:30:00Z')
        self.assertEqual(second['epoch_utc'], '2024-10-27T01:30:00Z')
        self.assertNotEqual(first['chart']['sha256'], second['chart']['sha256'])

    def test_polar_coordinates_never_silently_relocate(self):
        source = request()
        source['birth']['place']['latitude_degrees'] = 78.22
        source['birth']['place']['timezone'] = 'Arctic/Longyearbyen'
        result = natal.produce(source)
        self.assertEqual(result['status'], 'partial')
        self.assertEqual(result['reason'], 'chart-latitude-outside-unadjusted-provider-range')
        self.assertIsNone(result['chart'])
        self.assertEqual(len(result['sky']['bodies']), 10)

    def test_invalid_inputs_fail_without_chart(self):
        mutations = [
            ('birth.date', '1990-02-30'), ('birth.time', '25:00'),
            ('birth.place.timezone', 'Invalid/Timezone'),
            ('birth.place.latitude_degrees', True), ('birth.place.longitude_degrees', float('nan')),
            ('birth.place.source_ref', ''), ('birth.precision', 'guess'),
            ('birth.uncertainty_minutes', 1), ('birth.fold', 1)]
        for key, value in mutations:
            with self.subTest(key=key):
                source = request()
                target = source
                parts = key.split('.')
                for part in parts[:-1]:
                    target = target[part]
                target[parts[-1]] = value
                with self.assertRaises(natal.NatalError):
                    natal.produce(source)

    def test_json_process_success_and_validation_failure(self):
        run = subprocess.run([sys.executable, str(Path(natal.__file__))],
                             input=json.dumps(request()), text=True, capture_output=True, timeout=30)
        self.assertEqual(run.returncode, 0, run.stderr)
        self.assertEqual(json.loads(run.stdout)['status'], 'available')
        bad = subprocess.run([sys.executable, str(Path(natal.__file__))],
                             input='{"schema": "bad"}', text=True, capture_output=True, timeout=30)
        self.assertEqual(bad.returncode, 2)
        self.assertEqual(json.loads(bad.stdout)['schema'], 'ql.nara-natal-error/v1')
        self.assertNotIn('<svg', bad.stdout)


if __name__ == '__main__':
    unittest.main()
