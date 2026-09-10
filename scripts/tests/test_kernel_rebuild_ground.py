"""K0 guard regressions, including real Git object identity tests."""
import copy
import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("k0_ground", ROOT / "scripts/verify-kernel-rebuild-ground.py")
ground = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ground)


class LedgerTests(unittest.TestCase):
    def setUp(self):
        self.data = json.loads((ROOT / ground.LEDGER).read_text(encoding="utf-8"))

    def rejected(self, change):
        data = copy.deepcopy(self.data)
        change(data)
        with self.assertRaises(ground.GroundError):
            ground.validate_ledger(data)

    def test_current_ledger(self):
        ground.validate_ledger(self.data)

    def test_moving_repository_ref_is_not_a_pin(self):
        self.rejected(lambda d: d["repositories"]["epi"].update(revision="main"))

    def test_duplicate_identity_is_rejected(self):
        self.rejected(lambda d: d["inputs"].append(copy.deepcopy(d["inputs"][0])))

    def test_path_escape_is_rejected(self):
        self.rejected(lambda d: d["inputs"][0].update(path="../private/Control"))

    def test_unknown_source_class_is_rejected(self):
        self.rejected(lambda d: d["inputs"][0].update(classes=["latest-wins"]))

    def test_imported_c_cannot_be_demoted(self):
        self.rejected(lambda d: next(i for i in d["inputs"] if i["id"] == "imported-c").update(standing="DISPOSABLE_HISTORY"))

    def test_imported_rust_cannot_be_demoted(self):
        self.rejected(lambda d: next(i for i in d["inputs"] if i["id"] == "imported-rust").update(standing="SECONDARY_WITNESS_ONLY"))

    def test_live_revision_cannot_be_invented(self):
        self.rejected(lambda d: d["live_graph"].update(database_revision="invented"))

    def test_serialized_proof_cannot_be_called_live(self):
        self.rejected(lambda d: d["live_graph"].update(observation_status="verified"))

    def test_historical_frame_cannot_be_added_as_eighth(self):
        self.rejected(lambda d: d["kernel"]["canonical_context_frames"].append("(4/5/0)"))

    def test_unresolved_handoff_is_rejected(self):
        self.rejected(lambda d: d["handoff"]["K1"]["inputs"].append("missing"))


class ImmutableGitTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.git("init", "-q")
        self.git("config", "user.name", "K0 fixture")
        self.git("config", "user.email", "k0-fixture@example.invalid")
        (self.root / "source.txt").write_text("original\n", encoding="utf-8")
        self.git("add", "source.txt")
        self.git("commit", "-qm", "original source")
        self.revision = self.git("rev-parse", "HEAD").strip()
        self.oid = self.git("rev-parse", "HEAD:source.txt").strip()

    def git(self, *args):
        return subprocess.check_output(["git", "-C", str(self.root), *args], text=True)

    def test_newer_head_does_not_replace_pinned_body(self):
        (self.root / "source.txt").write_text("newer does not mean authoritative\n", encoding="utf-8")
        self.git("add", "source.txt")
        self.git("commit", "-qm", "different source")
        item = {"id": "original", "path": "source.txt", "kind": "blob", "oid": self.oid}
        result = ground.resolve_pin(self.root, self.revision, item)
        self.assertEqual(result["oid"], self.oid)
        self.assertNotEqual(result["oid"], self.git("rev-parse", "HEAD:source.txt").strip())
        self.assertEqual(ground.read_object(self.root, self.revision, "source.txt"), b"original\n")

    def test_wrong_object_digest_fails(self):
        item = {"id": "bad", "path": "source.txt", "kind": "blob", "oid": "0" * 40}
        with self.assertRaises(ground.GroundError):
            ground.resolve_pin(self.root, self.revision, item)

    def test_missing_source_fails_not_skips(self):
        item = {"id": "missing", "path": "missing.txt", "kind": "blob"}
        with self.assertRaises(ground.GroundError):
            ground.resolve_pin(self.root, self.revision, item)

    def test_wrong_object_kind_fails(self):
        item = {"id": "wrong-kind", "path": "source.txt", "kind": "tree"}
        with self.assertRaises(ground.GroundError):
            ground.resolve_pin(self.root, self.revision, item)


if __name__ == "__main__":
    unittest.main()
