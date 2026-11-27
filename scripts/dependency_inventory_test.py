import importlib.util
import unittest
from pathlib import Path

spec = importlib.util.spec_from_file_location("inventory", Path(__file__).with_name("dependency-inventory.py"))
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)

class InventoryTest(unittest.TestCase):
    def test_metadata_gaps_are_explicit(self):
        n = {"packages": {"": {}, "node_modules/a": {"version": "1", "license": "MIT", "resolved": "https://registry.npmjs.org/a", "integrity": "sha512-test"},
                          "node_modules/b": {"version": "1", "resolved": "https://other.invalid/b"}}}
        c = {"package": [{"name": "x", "version": "1", "source": "registry+url"}, {"name": "x", "version": "2", "source": "registry+url", "checksum": "test"}]}
        r = module.inventory(n, c)
        self.assertEqual(r["npm"]["licenses"], {"MIT": 1, "UNKNOWN": 1})
        self.assertEqual(r["npm"]["missing_integrity"], ["node_modules/b"])
        self.assertEqual(r["npm"]["non_registry"], ["node_modules/b"])
        self.assertEqual(r["rust"]["missing_registry_checksum"], ["x"])
        self.assertEqual(r["rust"]["duplicate_name_count"], 1)
        self.assertEqual(r["rust"]["license_metadata_status"], "NOT_COLLECTED")
        self.assertEqual(r["legal_review"], "NOT RUN")

    def test_rust_metadata_does_not_infer_license_from_checksum(self):
        r = module.inventory({"packages": {}}, {"package": []},
             {"packages": [{"name": "x", "version": "1", "license": None, "license_file": None},
                           {"name": "y", "version": "2", "license_file": "/not/published/COPYING"}]})
        self.assertEqual(r["rust"]["unknown_license"], [{"name": "x", "version": "1"}])
        self.assertEqual(r["rust"]["license_metadata_status"], "COLLECTED_NOT_REVIEWED")
        self.assertNotIn("/not/published", str(r))

if __name__ == "__main__":
    unittest.main()
