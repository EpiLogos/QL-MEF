"""Exercise the actual original and published proof through the native checker.

The positive basis is retained repository/Git material, not a synthesized proof.
Each negative changes a copy of that material and checks the specific refusal.
Numerical execution and readiness are separate from these source-lineage tests.
"""
import hashlib
import importlib.util
import json
from pathlib import Path
import shutil
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
PROOF = "fixtures/kernel/m3-finite-proof-v1.json"
ORIGINAL = "fixtures/kernel/history/m3-finite-proof-212a4d-v1.json"
RECORD = "fixtures/kernel/m3-journey-source-integration-v1.json"
LINEAGE = "fixtures/kernel/k8-build-lineage-v1.json"
QUALIFICATION = "fixtures/kernel/m-ledger-source-requalification-v1.json"
spec = importlib.util.spec_from_file_location("historical_proof_owner", ROOT / "scripts/requalify-m-ledger.py")
owner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(owner)


class HistoricalProofAliasTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        wanted = {Path(name) for name in (
            PROOF, ORIGINAL, RECORD, LINEAGE,
            "docs/kernel-rebuild/APERTURES-AND-CLOCK-CENTRE.md",
        )}

        def copy_paths(real, destination, prefix):
            for child in real.iterdir():
                relative = prefix / child.name
                target = destination / child.name
                if relative in wanted:
                    shutil.copyfile(child, target)
                elif any(relative in name.parents for name in wanted):
                    target.mkdir()
                    copy_paths(child, target, relative)
                else:
                    target.symlink_to(child, target_is_directory=child.is_dir())

        copy_paths(ROOT, self.root, Path())
        qualification = json.loads((ROOT / QUALIFICATION).read_text())
        self.digest = qualification["historical_numerical_proofs_unchanged"][PROOF]
        self.commit = qualification["historical"]["git_commit"]

    def verify(self, digest=None):
        owner.verify_historical_proof(self.root, PROOF, digest or self.digest, self.commit)

    def mutate(self, path, change):
        value = json.loads((self.root / path).read_text())
        change(value)
        (self.root / path).write_text(json.dumps(value))

    def test_actual_published_alias_preserves_original_and_every_source_byte(self):
        paths = (PROOF, ORIGINAL, RECORD, LINEAGE, QUALIFICATION)
        before = {name: (self.root / name).read_bytes() for name in paths}
        self.verify()
        self.assertEqual(before, {name: (self.root / name).read_bytes() for name in paths})
        self.assertNotEqual(hashlib.sha256(before[PROOF]).hexdigest(), self.digest)
        self.assertEqual(hashlib.sha256(before[ORIGINAL]).hexdigest(), self.digest)

    def test_original_filename_remains_qualified_only_by_its_actual_git_digest(self):
        (self.root / PROOF).write_bytes((self.root / ORIGINAL).read_bytes())
        self.verify()
        with self.assertRaisesRegex(ValueError, "historical numerical proof was restamped"):
            self.verify("0" * 64)

    def test_altered_original_is_refused_even_when_record_is_restamped(self):
        self.mutate(ORIGINAL, lambda value: value.update(checks=[]))
        digest = hashlib.sha256((self.root / ORIGINAL).read_bytes()).hexdigest()
        self.mutate(RECORD, lambda value: value["native_history"]["original"].update(sha256=digest))
        with self.assertRaisesRegex(ValueError, "retained historical numerical original changed"):
            self.verify()

    def test_forged_active_proof_cannot_borrow_a_valid_digest(self):
        self.mutate(PROOF, lambda value: value.update(checks=[]))
        digest = hashlib.sha256((self.root / PROOF).read_bytes()).hexdigest()
        self.mutate(RECORD, lambda value: value["native_history"]["successor"].update(sha256=digest))
        with self.assertRaisesRegex(ValueError, "active historical successor differs from published Git source"):
            self.verify()

    def test_unpublished_git_alias_is_refused_against_the_actual_published_source(self):
        self.mutate(RECORD, lambda value: value["native_history"]["successor"].update(
            git_ref=value["native_history"]["original"]["original_git_ref"]))
        with self.assertRaisesRegex(ValueError, "active historical successor differs from published Git source"):
            self.verify()

    def test_missing_or_forged_lineage_cannot_be_restamped_into_acceptance(self):
        for mutation in (lambda value: value["successors"].pop(),
                         lambda value: value["successors"][-1].update(previous_sha256="0" * 64)):
            with self.subTest(mutation=mutation):
                shutil.copyfile(ROOT / LINEAGE, self.root / LINEAGE)
                self.mutate(LINEAGE, mutation)
                digest = hashlib.sha256((self.root / LINEAGE).read_bytes()).hexdigest()
                self.mutate(RECORD, lambda value: value["native_history"]["native_lineage"].update(sha256=digest))
                with self.assertRaisesRegex(ValueError, "native historical lineage missing or forged"):
                    self.verify()

    def test_stale_current_source_record_is_refused(self):
        self.mutate(RECORD, lambda value: value["current_source"].update(numerical_registry_revision="0" * 64))
        with self.assertRaisesRegex(ValueError, "historical integration record has stale current source standing"):
            self.verify()

    def test_changed_alias_requires_the_retained_explicit_integration_record(self):
        (self.root / RECORD).unlink()
        with self.assertRaises(FileNotFoundError):
            self.verify()


if __name__ == "__main__":
    unittest.main()
