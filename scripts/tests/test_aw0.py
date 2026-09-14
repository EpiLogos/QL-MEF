"""Full-field closure checks: source coverage and accepted receipts stay distinct."""
import importlib.util
import json
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location('aw0', Path(__file__).parents[1] / 'aw0.py')
aw0 = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(aw0)


class AwFieldTests(unittest.TestCase):
    def test_independent_full_inventories(self):
        result = aw0.project()
        self.assertEqual(result['counts']['historical-sp'], 36)
        self.assertEqual(result['counts']['m-capability'], 36)
        self.assertEqual(result['counts']['m-inhabitation'], 36)
        self.assertEqual(result['counts']['vak-cell'], 36)
        self.assertEqual(result['counts']['vak-entry'], 109)
        self.assertEqual(result['counts']['deep-capability'], 149)
        self.assertEqual(result['counts']['property-definition'], 194)
        self.assertEqual(result['counts']['thought'], 12)
        self.assertEqual(result['counts']['source-skill'], 65)
        self.assertEqual(result['counts']['thread-form'], 7)
        self.assertEqual(result['counts']['content-type'], 7)
        self.assertEqual(result['counts']['agent-source'], 15)
        self.assertEqual(result['counts']['specialist-mode'], 8)

    def test_all_deep_property_occurrences_keep_their_original_record_handles(self):
        result = aw0.project()
        rows = [r for r in result['records'] if r['inventory'] == 'property-source-key']
        self.assertEqual(len(rows), 5686)
        self.assertEqual(sum(len(r['inputs_and_results']['source_record_indices']) for r in rows), 85474)
        manifest = aw0.load('fixtures/kernel/m-tree-v1.json')
        for row in rows:
            for index in row['inputs_and_results']['source_record_indices']:
                self.assertIn(row['source_identity'], manifest['records'][index]['property_keys'])
        self.assertEqual(result['counts']['original-operational-job'], 10)

    def test_registered_descriptor_matches_actual_complete_projection(self):
        descriptor = aw0.load(aw0.BASE + 'aw0-field.json')
        self.assertEqual(descriptor['schema_version'], aw0.SCHEMA)
        self.assertEqual(descriptor['counts'], aw0.project()['counts'])
        self.assertFalse(descriptor['runtime_acceptance'])
        self.assertFalse(descriptor['runtime_registry'])
        self.assertEqual(descriptor['acceptance_receipts'], 'aw0-acceptance-receipts.json')

    def test_every_row_expands_original_fields_and_native_ownership(self):
        for row in aw0.project()['records']:
            with self.subTest(id=row['id']):
                self.assertTrue(row['meaning'])
                self.assertTrue(row['sources'])
                self.assertTrue(row['native_bindings'])
                self.assertTrue(row['inputs_and_results'])
                self.assertTrue(row['gap']['issue'])
                self.assertTrue(row['authority'])
                self.assertTrue(row['lifecycle_return'])
                self.assertTrue(row['agent_skill_method_participation'])
                self.assertEqual(len(row['tests_required']), 5)
                for source in row['sources'] + row['native_bindings']:
                    self.assertRegex(source['git_blob'], r'^[0-9a-f]{40}$')

    def test_current_disposition_is_receipt_qualified_without_rewriting_baseline_gaps(self):
        result = aw0.project()
        allowed = {'ACCEPTED-NATIVE', 'READY-TO-COMPOSE', 'EXTERNAL-OWNER',
                   'SOURCE-DISCREPANCY', 'RESEARCH-ONLY', 'EPI-GAP'}
        self.assertEqual(sum(result['disposition_counts'].values()), len(result['records']))
        self.assertTrue(set(result['disposition_counts']).issubset(allowed))
        self.assertEqual(result['readiness_reconciliation']['historical']['counts']['READY-TO-COMPOSE'], 21)
        self.assertEqual(result['readiness_reconciliation']['historical']['counts']['EPI-GAP'], 15)
        for row in result['records']:
            with self.subTest(id=row['id']):
                self.assertEqual(row['gap_standing'], 'AW0-BASELINE-GAP-RETAINED-FOR-PROVENANCE')
                self.assertIn(row['disposition'], allowed)
                if row['disposition'] == 'ACCEPTED-NATIVE':
                    self.assertTrue(row['acceptance_receipt_ids'])
                    for receipt_id in row['acceptance_receipt_ids']:
                        self.assertEqual(result['acceptance_overlay']['receipts'][receipt_id]['standing'],
                                         'ACCEPTED-NATIVE')
                if row['disposition'] == 'EPI-GAP':
                    self.assertTrue(row.get('current_dependency'))

    def test_staged_skills_are_not_deduplicated_by_the_display_name(self):
        skills = [r for r in aw0.project()['records'] if r['inventory'] == 'source-skill']
        tmux = [r for r in skills if r['meaning'] == 'tmux']
        self.assertEqual(len(tmux), 2)
        self.assertNotEqual(tmux[0]['source_identity'], tmux[1]['source_identity'])

    def test_node_and_relationship_property_identity_never_alias(self):
        props = [r for r in aw0.project()['records'] if r['inventory'] == 'property-definition']
        keys = {r['source_identity'] for r in props}
        self.assertEqual(sum(k.startswith('Node:') for k in keys), 164)
        self.assertEqual(sum(k.startswith('Relationship:') for k in keys), 30)
        shared = {k.split(':', 1)[1] for k in keys if k.startswith('Node:')} & {k.split(':', 1)[1] for k in keys if k.startswith('Relationship:')}
        self.assertTrue(shared)

    def test_20_40_remains_an_unproved_source_claim_not_an_invented_cross_product(self):
        row = next(r for r in aw0.project()['records'] if r['id'] == 'source-discrepancy:legacy-20-40')
        self.assertEqual(row['inputs_and_results']['current_frame_identities'], 7)
        self.assertEqual(row['inputs_and_results']['standing'], 'SOURCE-CLAIM-NOT-CARDINALITY-PROOF')
        self.assertEqual(row['disposition'], 'SOURCE-DISCREPANCY')

    def test_missing_native_pin_fails_the_whole_projection(self):
        original = aw0.load
        def broken(path):
            value = original(path)
            if path.endswith('aw0-sources-native.json'):
                value['owners']['ai-kit'] = [v for v in value['owners']['ai-kit'] if v[0] != 'crates/aikit-core/src/resource/operative.rs']
            return value
        with patch.object(aw0, 'load', broken):
            with self.assertRaisesRegex(ValueError, 'unpinned source'):
                aw0.project()

    def test_lost_sp_binding_cannot_hide_in_the_other_36_member_fields(self):
        original = aw0.load
        def broken(path):
            value = original(path)
            if path.endswith('aw0-native-bindings.json'):
                value['sp_groups'].pop()
            return value
        with patch.object(aw0, 'load', broken):
            with self.assertRaisesRegex(ValueError, 'every record'):
                aw0.project()

    def test_lost_property_is_not_a_successful_partial_registry(self):
        original = aw0.load
        def broken(path):
            value = original(path)
            if path.endswith('aw-property-source.json.gz'):
                value['properties'].pop()
            return value
        with patch.object(aw0, 'load', broken):
            with self.assertRaisesRegex(ValueError, 'property definitions'):
                aw0.project()

    def test_unknown_gap_is_rejected(self):
        original = aw0.load
        def broken(path):
            value = original(path)
            if path.endswith('aw0-native-bindings.json'):
                value['sp_gap'][0] = 'UNASSIGNED'
            return value
        with patch.object(aw0, 'load', broken):
            with self.assertRaisesRegex(ValueError, 'incomplete source/native disposition'):
                aw0.project()

    def test_ambiguous_current_disposition_fails_closed(self):
        original = aw0.load
        def broken(path):
            value = original(path)
            if path.endswith('aw0-acceptance-receipts.json'):
                duplicate = dict(value['rules'][0])
                duplicate['id'] = 'duplicate-highest-rule'
                value['rules'].append(duplicate)
            return value
        with patch.object(aw0, 'load', broken):
            with self.assertRaisesRegex(ValueError, 'ambiguous current disposition'):
                aw0.project()

    def test_duplicate_rule_identity_fails_closed(self):
        original = aw0.load
        def broken(path):
            value = original(path)
            if path.endswith('aw0-acceptance-receipts.json'):
                value['rules'].append(dict(value['rules'][0]))
            return value
        with patch.object(aw0, 'load', broken):
            with self.assertRaisesRegex(ValueError, 'duplicate disposition rule id'):
                aw0.project()

    def test_unknown_selector_field_cannot_broaden_a_rule(self):
        original = aw0.load
        def broken(path):
            value = original(path)
            if path.endswith('aw0-acceptance-receipts.json'):
                value['rules'][0] = dict(value['rules'][0], selector={'inventoriess': ['source-discrepancy']})
            return value
        with patch.object(aw0, 'load', broken):
            with self.assertRaisesRegex(ValueError, 'unsupported disposition selector'):
                aw0.project()

    def test_dead_selector_cannot_survive_as_unused_policy(self):
        original = aw0.load
        def broken(path):
            value = original(path)
            if path.endswith('aw0-acceptance-receipts.json'):
                value['rules'][0] = dict(value['rules'][0], selector={'ids': ['not-a-real-record:anywhere']})
            return value
        with patch.object(aw0, 'load', broken):
            with self.assertRaisesRegex(ValueError, 'disposition rule matches no source record'):
                aw0.project()

    def test_empty_selector_cannot_match_the_entire_field(self):
        original = aw0.load
        def broken(path):
            value = original(path)
            if path.endswith('aw0-acceptance-receipts.json'):
                value['rules'][0] = dict(value['rules'][0], selector={})
            return value
        with patch.object(aw0, 'load', broken):
            with self.assertRaisesRegex(ValueError, 'nonempty selector'):
                aw0.project()

    def test_unknown_acceptance_receipt_fails_closed(self):
        original = aw0.load
        def broken(path):
            value = original(path)
            if path.endswith('aw0-acceptance-receipts.json'):
                value['rules'][0] = dict(value['rules'][0], receipt_ids=['missing-receipt'])
            return value
        with patch.object(aw0, 'load', broken):
            with self.assertRaisesRegex(ValueError, 'unknown acceptance receipt'):
                aw0.project()

    def test_anonymous_epi_gap_fails_closed(self):
        original = aw0.load
        def broken(path):
            value = original(path)
            if path.endswith('aw0-acceptance-receipts.json'):
                value['rules'][0] = dict(value['rules'][0], disposition='EPI-GAP')
                value['rules'][0].pop('current_dependency', None)
            return value
        with patch.object(aw0, 'load', broken):
            with self.assertRaisesRegex(ValueError, 'EPI-GAP disposition rule has no dependency'):
                aw0.project()

    def test_source_verification_requires_real_matching_bytes(self):
        with tempfile.TemporaryDirectory() as directory:
            with self.assertRaisesRegex(ValueError, 'source pin mismatch'):
                aw0.verify_sources(Path(directory))

    def test_projection_and_original_input_bytes_are_deterministic(self):
        before = {p: p.read_bytes() for p in (aw0.ROOT / aw0.BASE).glob('aw*')}
        first = json.dumps(aw0.project(), ensure_ascii=False, sort_keys=True)
        self.assertEqual(first, json.dumps(aw0.project(), ensure_ascii=False, sort_keys=True))
        self.assertTrue(all(p.read_bytes() == b for p, b in before.items()))


if __name__ == '__main__':
    unittest.main()
