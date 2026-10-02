"""Exercise the complete admitted source capsule, including tamper refusal."""
import importlib.util
import json
from pathlib import Path
import sys
import unittest

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts"))
spec = importlib.util.spec_from_file_location("bimba_content", ROOT / "scripts/bimba-content.py")


class NativeSourceContent(unittest.TestCase):
    def test_complete_source_and_exact_personal_locus(self):
        self.assertIsNotNone(spec)
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        capsule = json.loads((ROOT / "fixtures/kernel/bimba-content-v1.json").read_text())
        module.validate(capsule)
        nodes = capsule["content"]["nodes"]
        # Full Epi content exceeds the deliberately narrower M-tree projection.
        self.assertGreater(len(nodes), 1910)
        for name in ("M", "M'", "M0'", "M4'", "M4.4.4.4"):
            self.assertIn(name, nodes)
        locus = nodes["M4.4.4.4"]["properties"]
        self.assertEqual(locus["c_2_uuid"], "dcb274c1-fbbc-5914-b27d-dea979c78558")
        self.assertGreater(len(locus), 60)
        edges = capsule["content"]["relations"]
        self.assertTrue(any(a == "M4.4.4.3" and kind == "FLOWS_TO" and b == "M4.4.4.4"
                            for a, kind, b, _ in edges))
        self.assertTrue(any(a == "M4.4.4.4" and kind == "REFLECTS_FOUNDATION" and b == "M0"
                            for a, kind, b, _ in edges))
        self.assertTrue(any(a == "M2-3-4-0-2" and kind == "RULED_BY" and b == "M2-5-4"
                            for a, kind, b, _ in edges))

    def test_altered_property_or_qualified_edge_is_not_an_admitted_source(self):
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        original = json.loads((ROOT / "fixtures/kernel/bimba-content-v1.json").read_text())
        for part in ("property", "relation"):
            altered = json.loads(json.dumps(original))
            if part == "property":
                altered["content"]["nodes"]["M4.4.4.4"]["properties"]["c_2_uuid"] = "wrong-personal-branch"
            else:
                altered["content"]["relations"][0][3]["qualification"] = "invented"
            with self.assertRaisesRegex(ValueError, "content hash"):
                module.validate(altered)


if __name__ == "__main__":
    unittest.main()
