import copy
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime, timedelta, timezone
import json
from pathlib import Path
import subprocess
import sys
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).parent))
import kerykeion_snapshot as sky

NOW = datetime(2026, 9, 12, 12, tzinfo=timezone.utc)


def query(**overrides):
    value = dict(schema=sky.REQUEST, epoch='2026-09-12T12:00:00Z', timezone='UTC',
                 mode='historical', perspective='Apparent Geocentric', zodiac='Tropical',
                 ayanamsha=None, observer=None, max_age_seconds=60, backend_policy='allow-moshier')
    value.update(overrides)
    return value


def signed(value):
    value.pop('snapshot_ref', None)
    value['snapshot_ref'] = 'sha256:' + sky.digest(value)
    return value


class SkyTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.saved = sky.produce(query(), now=NOW)

    def test_actual_ten_body_epoch_and_source(self):
        s = self.saved
        self.assertEqual(len(s['bodies']), 10)
        self.assertEqual(s['source_binding']['epogdoon'], [9, 8])
        self.assertTrue(s['source_binding']['earth_body']['outside_planet_array'])
        self.assertEqual(s['source_binding']['receiving_chakras'], 7)
        self.assertEqual(s['julian_day_ut_argument'], 2461296.0)
        self.assertEqual(s['receipt_clock'], 'injected-controlled-clock')
        self.assertTrue(169 < s['bodies'][0]['longitude_degrees'] < 171)
        self.assertEqual(s['provider']['version'], '5.7.1')
        self.assertTrue(all(len(s['provider'][k]) == 64 for k in
                            ('engine_sha256', 'factory_sha256', 'adapter_sha256')))

    def test_sun_descriptor_and_exact_known_legacy_retention(self):
        self.assertEqual(self.saved['source_binding']['sun_role'], 'solar-parent')
        fixture = sky.ROOT / 'fixtures/kernel/sky-snapshot-known-e6d-2026-09-15-v1.json'
        original = json.loads(fixture.read_text())
        before = json.dumps(original, sort_keys=True)
        self.assertEqual(original['provider']['adapter_sha256'], sky.LEGACY_SUN_ADAPTER_SHA256)
        self.assertEqual(original['source_binding']['sun_role'], 'parent-not-chakra-mapped')
        # The original artifact's physical sky, source qualifiers and digest are
        # retained exactly. A separate receipt tells the truth about the label.
        replay = sky.validate_retained_snapshot(original)
        self.assertEqual(json.dumps(replay, sort_keys=True), before)
        qualification = sky.source_binding_qualification(original, retained=True)
        self.assertTrue(qualification['legacy_descriptor_admitted'])
        self.assertEqual(qualification['current_sun_role'], 'solar-parent')
        self.assertFalse(qualification['fresh_current_attested'])
        with self.assertRaisesRegex(sky.SkyError, 'native source binding is stale'):
            sky.source_binding_qualification(original, retained=False)
        refused = subprocess.run([sys.executable, str(Path(sky.__file__)), '-',
                                  '--validate-snapshot'], input=json.dumps(original),
                                 text=True, capture_output=True)
        self.assertEqual(refused.returncode, 2)
        self.assertIn('native source binding is stale', refused.stderr)

    def test_known_legacy_sun_descriptor_is_not_a_general_source_exception(self):
        fixture = sky.ROOT / 'fixtures/kernel/sky-snapshot-known-e6d-2026-09-15-v1.json'
        original = json.loads(fixture.read_text())
        mutations = [
            ('provider', 'adapter_sha256', '0' * 64),
            ('source_binding', 'registry_revision', '0' * 64),
            ('source_binding', 'header_sha256', '0' * 64),
            ('source_binding', 'sun_role', 'wrong-Sun-route'),
            ('source_binding', 'non_sun_operators', 10),
            ('source_binding', 'receiving_chakras', 8),
        ]
        for group, key, value in mutations:
            with self.subTest(group=group, key=key):
                changed = copy.deepcopy(original)
                changed[group][key] = value
                with self.assertRaisesRegex(sky.SkyError, 'native source binding is stale'):
                    sky.validate_retained_snapshot(signed(changed))
        changed = copy.deepcopy(original)
        changed['source_binding']['unexpected'] = True
        with self.assertRaisesRegex(sky.SkyError, 'native source binding is stale'):
            sky.validate_retained_snapshot(signed(changed))
        changed = copy.deepcopy(original)
        changed['source_binding']['earth_body']['source_ref'] = '#2-5-0/1-7'
        with self.assertRaisesRegex(sky.SkyError, 'native source binding is stale'):
            sky.validate_retained_snapshot(signed(changed))
        changed = copy.deepcopy(original)
        changed['bodies'][0]['native_planet_id'] = 1
        with self.assertRaisesRegex(sky.SkyError, 'planet array identity mismatch'):
            sky.validate_retained_snapshot(signed(changed))
        changed = copy.deepcopy(original)
        changed['bodies'][0]['longitude_degrees'] += 1
        with self.assertRaisesRegex(sky.SkyError, 'snapshot digest mismatch'):
            sky.validate_retained_snapshot(changed)

    def test_real_host_current_not_fixture(self):
        now = datetime.now(timezone.utc).replace(microsecond=0)
        s = sky.produce(query(epoch=sky.iso(now), mode='current'))
        self.assertEqual(s['receipt_clock'], 'host-utc-clock')
        sky.validate_snapshot(s, require_current=True)
        out = sky.ROOT / 'target/k8-sky'
        out.mkdir(parents=True, exist_ok=True)
        (out / 'current-snapshot.json').write_text(json.dumps(s, indent=2) + '\n')

    def test_actual_historical_and_adapter_boundaries(self):
        for text in ('1800-01-01T00:00:00Z', '1875-07-26T12:00:00Z',
                     '2000-01-01T12:00:00Z', '2399-12-31T23:59:59Z'):
            s = sky.produce(query(epoch=text), now=NOW)
            self.assertEqual(len(s['bodies']), 10)
        j2000 = sky.produce(query(epoch='2000-01-01T12:00:00Z'), now=NOW)
        self.assertEqual(j2000['julian_day_ut_argument'], 2451545.0)
        self.assertAlmostEqual(j2000['bodies'][0]['longitude_degrees'], 280.36892, places=4)
        self.assertLess(sky.produce(query(epoch='1875-07-26T12:00:00Z'), now=NOW)['epoch_unix_ms'], 0)

    def test_refuse_unqualified_epochs_and_timezones(self):
        for text in ('1799-12-31T23:59:59Z', '2400-01-01T00:00:00Z',
                     '2026-09-12T12:00:00', '2026-09-12T12:00:00.1Z', 'bad'):
            with self.assertRaises(sky.SkyError):
                sky.produce(query(epoch=text), now=NOW)
        with self.assertRaises(sky.SkyError):
            sky.produce(query(timezone='Europe/London'), now=NOW)
        with self.assertRaises(sky.SkyError):
            sky.produce(query(timezone='not/a/zone'), now=NOW)

    def test_equivalent_offset_and_dst_fold(self):
        a = sky.produce(query(epoch='2026-09-12T13:00:00+01:00', timezone='Europe/London'), now=NOW)
        self.assertEqual(a['bodies'], self.saved['bodies'])
        a = sky.produce(query(epoch='2026-10-25T01:30:00+01:00', timezone='Europe/London'), now=NOW)
        b = sky.produce(query(epoch='2026-10-25T01:30:00+00:00', timezone='Europe/London'), now=NOW)
        self.assertEqual(b['epoch_unix_ms'] - a['epoch_unix_ms'], 3600000)
        self.assertNotEqual(a['bodies'], b['bodies'])
        with self.assertRaises(sky.SkyError):
            sky.produce(query(epoch='2026-03-29T01:30:00+00:00', timezone='Europe/London'), now=NOW)

    def test_actual_retrograde_speed(self):
        s = sky.produce(query(epoch='2026-07-10T12:00:00Z'), now=NOW)
        self.assertTrue(s['bodies'][2]['retrograde'])
        self.assertLess(s['bodies'][2]['longitude_speed_degrees_per_day'], 0)
        self.assertTrue(self.saved['bodies'][6]['retrograde'])

    def test_current_freshness_future_and_historical_not_live(self):
        s = sky.produce(query(mode='current'), now=NOW)
        sky.validate_snapshot(s, now=NOW + timedelta(seconds=60), require_current=True)
        for now in (NOW + timedelta(seconds=61), NOW - timedelta(seconds=1)):
            with self.assertRaises(sky.SkyError):
                sky.produce(query(mode='current'), now=now)
            with self.assertRaises(sky.SkyError):
                sky.validate_snapshot(s, now=now, require_current=True)
        with self.assertRaises(sky.SkyError):
            sky.validate_snapshot(self.saved, now=NOW, require_current=True)

    def test_retained_current_origin_preserves_original_without_ephemeris_or_live_claim(self):
        original = sky.produce(query(mode='current'), now=NOW)
        before = json.dumps(original, sort_keys=True)
        later = NOW + timedelta(seconds=61)
        with self.assertRaisesRegex(sky.SkyError, 'not fresh current sky'):
            sky.validate_snapshot(original, now=later, require_current=True)
        with patch('kerykeion.AstrologicalSubjectFactory.from_birth_data',
                   side_effect=AssertionError('retained validation must not recalculate sky')):
            retained = sky.validate_retained_snapshot(original, now=later)
        self.assertEqual(json.dumps(retained, sort_keys=True), before)
        self.assertEqual(retained['request']['mode'], 'current')

    def test_retained_validation_keeps_source_digest_capture_freshness_and_future_guards(self):
        original = sky.produce(query(mode='current'), now=NOW)
        later = NOW + timedelta(days=1)
        changed = copy.deepcopy(original)
        changed['bodies'][0]['longitude_degrees'] += 1
        with self.assertRaisesRegex(sky.SkyError, 'snapshot digest mismatch'):
            sky.validate_retained_snapshot(changed, now=later)
        changed = copy.deepcopy(original)
        changed['source_binding']['registry_revision'] = '0' * 64
        with self.assertRaisesRegex(sky.SkyError, 'native source binding is stale'):
            sky.validate_retained_snapshot(signed(changed), now=later)
        changed = copy.deepcopy(original)
        changed['receipt_utc'] = sky.iso(NOW + timedelta(seconds=61))
        changed['receipt_unix_ms'] = int((NOW + timedelta(seconds=61)).timestamp() * 1000)
        with self.assertRaisesRegex(sky.SkyError, 'not fresh current sky'):
            sky.validate_retained_snapshot(signed(changed), now=later)
        with self.assertRaisesRegex(sky.SkyError, 'receipt is in the future'):
            sky.validate_retained_snapshot(original, now=NOW - timedelta(seconds=1))
        historical = sky.produce(query(epoch='2027-01-01T00:00:00Z'), now=NOW)
        self.assertEqual(sky.validate_retained_snapshot(historical, now=NOW), historical)
        with self.assertRaises(sky.SkyError):
            sky.validate_snapshot(historical, now=NOW, require_current=True)

    def test_retained_cli_is_explicit_and_never_authorizes_new_request_or_m2_attachment(self):
        original = sky.produce(query(mode='current'), now=NOW)
        command = [sys.executable, str(Path(sky.__file__)), '-', '--validate-retained-snapshot']
        positive = subprocess.run(command, input=json.dumps(original), text=True, capture_output=True)
        self.assertEqual(positive.returncode, 0, positive.stderr)
        self.assertEqual(json.loads(positive.stdout), original)
        default = subprocess.run(command[:-1] + ['--validate-snapshot'],
                                 input=json.dumps(original), text=True, capture_output=True)
        self.assertNotEqual(default.returncode, 0)
        self.assertIn('not fresh current sky', default.stderr)
        for args, value in [(command, query()),
                            (command + ['--validate-snapshot'], original),
                            (command + ['--m2-request', 'unused.json'], original)]:
            refused = subprocess.run(args, input=json.dumps(value), text=True, capture_output=True)
            self.assertNotEqual(refused.returncode, 0)
            self.assertFalse(refused.stdout)

    def test_backend_is_returned_not_requested_and_required_files(self):
        self.assertTrue(all(b['backend'] in ('moshier', 'swiss-files') for b in self.saved['bodies']))
        if any(b['backend'] == 'moshier' for b in self.saved['bodies']):
            with self.assertRaisesRegex(sky.SkyError, 'fell back'):
                sky.produce(query(backend_policy='require-swiss-files'), now=NOW)
        else:
            self.assertTrue(self.saved['provider']['used_data_files'])

    def test_true_and_sidereal_have_explicit_policy(self):
        true = sky.produce(query(perspective='True Geocentric'), now=NOW)
        self.assertNotEqual(true['bodies'][0]['longitude_degrees'], self.saved['bodies'][0]['longitude_degrees'])
        sid = sky.produce(query(zodiac='Sidereal', ayanamsha='LAHIRI'), now=NOW)
        self.assertTrue(23 < sid['ayanamsha_degrees'] < 25)
        self.assertNotEqual(sid['bodies'], self.saved['bodies'])
        with self.assertRaises(sky.SkyError):
            sky.produce(query(zodiac='Sidereal'), now=NOW)

    def test_observer_private_and_no_hidden_relocation(self):
        observer = dict(reference='controlled:observer', longitude_degrees=0.1,
                        latitude_degrees=51.5, altitude_metres=10)
        s = sky.produce(query(perspective='Topocentric', observer=observer), now=NOW)
        self.assertEqual(s['scope'], 'observer-private')
        self.assertNotEqual(s['bodies'][1]['longitude_degrees'], self.saved['bodies'][1]['longitude_degrees'])
        with self.assertRaises(sky.SkyError):
            sky.produce(query(observer=observer), now=NOW)
        with self.assertRaises(sky.SkyError):
            sky.produce(query(perspective='Topocentric'), now=NOW)
        observer['latitude_degrees'] = 80
        with self.assertRaises(sky.SkyError):
            sky.produce(query(perspective='Topocentric', observer=observer), now=NOW)

    def test_replay_does_not_call_provider_or_rewrite_original(self):
        original = json.dumps(self.saved, sort_keys=True)
        with patch('kerykeion.AstrologicalSubjectFactory.from_birth_data', side_effect=RuntimeError('offline')):
            replay = sky.validate_snapshot(json.loads(original))
            self.assertEqual(json.dumps(replay, sort_keys=True), original)
        self.assertEqual(json.dumps(self.saved, sort_keys=True), original)

    def test_actual_javascript_round_trip_preserves_the_admitted_occasion(self):
        # Exercise the production JSON Number transport, not a hand-edited
        # stand-in. The provider emits an integral Julian-day float while
        # JavaScript serializes it as an integer. The reference must survive.
        original = json.dumps(self.saved, sort_keys=True)
        transport = subprocess.run(
            ['node', '-e', 'process.stdout.write(JSON.stringify(JSON.parse(require("fs").readFileSync(0,"utf8"))))'],
            input=original, text=True, capture_output=True, check=True)
        received = json.loads(transport.stdout)
        self.assertIs(type(received['julian_day_ut_argument']), int)
        replay = sky.validate_snapshot(received)
        self.assertEqual(replay['snapshot_ref'], self.saved['snapshot_ref'])
        self.assertEqual(replay['bodies'], self.saved['bodies'])
        self.assertEqual(json.dumps(self.saved, sort_keys=True), original)
        # The admission repair must reject a changed determinant even after
        # the same transport; it restores a type, never a lost quantity.
        received['bodies'][0]['longitude_degrees'] += 0.001
        with self.assertRaisesRegex(sky.SkyError, 'digest'):
            sky.validate_snapshot(received)

    def test_provider_failure_missing_body_and_unknown_version(self):
        from kerykeion import AstrologicalSubjectFactory
        with patch.object(AstrologicalSubjectFactory, 'from_birth_data', side_effect=RuntimeError('lost')):
            with self.assertRaises(sky.SkyError):
                sky.produce(query(), now=NOW)
        with patch('kerykeion_snapshot.metadata.version', return_value='future-version'):
            with self.assertRaisesRegex(sky.SkyError, 'unqualified'):
                sky.produce(query(), now=NOW)
        original = AstrologicalSubjectFactory.from_birth_data
        def missing(*args, **kwargs):
            chart = original(*args, **kwargs)
            chart.sun = None
            return chart
        with patch.object(AstrologicalSubjectFactory, 'from_birth_data', side_effect=missing):
            with self.assertRaisesRegex(sky.SkyError, 'missing required body'):
                sky.produce(query(), now=NOW)

    def test_digest_and_resigned_invalid_content_are_rejected(self):
        bad = copy.deepcopy(self.saved)
        bad['bodies'][0]['longitude_degrees'] += 1
        with self.assertRaisesRegex(sky.SkyError, 'digest'):
            sky.validate_snapshot(bad)
        mutations = [lambda s: s['bodies'].pop(),
                     lambda s: s['bodies'][0].update(longitude_degrees=360),
                     lambda s: s['bodies'][0].update(native_planet_id=4),
                     lambda s: s['bodies'][0].update(retrograde=True),
                     lambda s: s['bodies'][0].update(backend='swiss-files'),
                     lambda s: s.update(scope='observer-private'),
                     lambda s: s.update(julian_day_ut_argument=0),
                     lambda s: s.update(receipt_unix_ms=0),
                     lambda s: s.update(standing='observed-live'),
                     lambda s: s['bodies'][0].update(private_natal='forbidden')]
        for mutate in mutations:
            bad = copy.deepcopy(self.saved)
            mutate(bad)
            with self.assertRaises(sky.SkyError):
                sky.validate_snapshot(signed(bad))
        with self.assertRaises(ValueError):
            signed(dict(self.saved, julian_day_ut_argument=float('nan')))

    def test_native_m2_join_preserves_whole_request_and_negative_epoch(self):
        original = json.loads((sky.ROOT / 'fixtures/kernel/m2-engine-request-v1.json').read_text())
        original['at_unix_ms'] = int(NOW.timestamp()) * 1000
        for s in (self.saved, sky.produce(query(epoch='1875-07-26T12:00:00Z'), now=NOW)):
            result = sky.attach_m2(s, original)
            self.assertEqual(len(result['world_observations']), 10)
            for key in original:
                if key != 'world_observations':
                    self.assertEqual(original[key], result[key])
            self.assertEqual(result['world_observations'][1]['planet_id'], 1)
            self.assertEqual(result['world_observations'][1]['provider_ref'], s['snapshot_ref'])
        self.assertEqual(len(original['world_observations']), 2)
        private = dict(self.saved)
        private['scope'] = 'observer-private'
        with self.assertRaises(sky.SkyError):
            sky.attach_m2(signed(private), original)
        bad = dict(original, at_unix_ms=1000)
        with self.assertRaises(sky.SkyError):
            sky.attach_m2(self.saved, bad)
        bad = dict(original, registry_revision='wrong')
        with self.assertRaises(sky.SkyError):
            sky.attach_m2(self.saved, bad)

    def test_serialised_mixed_provider_config_does_not_leak(self):
        work = [query(), query(zodiac='Sidereal', ayanamsha='LAHIRI'),
                query(perspective='True Geocentric')] * 3
        with ThreadPoolExecutor(max_workers=3) as executor:
            results = list(executor.map(lambda q: sky.produce(q, now=NOW), work))
        self.assertEqual(results[0], results[3])
        self.assertEqual(results[1], results[4])
        self.assertEqual(results[2], results[5])
        self.assertNotEqual(results[0], results[1])


if __name__ == '__main__':
    unittest.main()
