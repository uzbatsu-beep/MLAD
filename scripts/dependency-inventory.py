#!/usr/bin/env python3
"""Report lockfile metadata, not legal clearance or a vulnerability scan."""
import argparse
import collections
import hashlib
import json
from pathlib import Path
import tomllib

ROOT = Path(__file__).resolve().parents[1]

def inventory(npm, cargo, metadata=None):
    packages = [{"path": path, "version": p.get("version"), "license": p.get("license"),
                 "resolved": p.get("resolved"), "integrity": p.get("integrity"),
                 "dev": p.get("dev", False), "optional": p.get("optional", False)}
                for path, p in sorted(npm["packages"].items()) if path]
    crates = cargo["package"]
    versions = collections.Counter(p["name"] for p in crates)
    rust_licenses = []
    if metadata is not None:
        rust_licenses = [{"name": p["name"], "version": p["version"],
                          "source": p.get("source"), "license": p.get("license"),
                          "license_file": bool(p.get("license_file"))}
                         for p in metadata["packages"]]
    return {"npm": {"count": len(packages),
                    "licenses": dict(collections.Counter(p["license"] or "UNKNOWN" for p in packages)),
                    "packages": packages,
                    "missing_integrity": [p["path"] for p in packages if not p["integrity"]],
                    "non_registry": [p["path"] for p in packages if not (p["resolved"] or "").startswith("https://registry.npmjs.org/")]},
            "rust": {"count": len(crates),
                     "registry_count": sum(p.get("source", "").startswith("registry+") for p in crates),
                     "missing_registry_checksum": [p["name"] for p in crates if p.get("source", "").startswith("registry+") and not p.get("checksum")],
                     "duplicate_name_count": sum(v > 1 for v in versions.values()),
                     "packages": [{"name": p["name"], "version": p["version"], "source": p.get("source"), "checksum": p.get("checksum")} for p in crates],
                     "license_metadata_status": "COLLECTED_NOT_REVIEWED" if metadata is not None else "NOT_COLLECTED",
                     "licenses": rust_licenses,
                     "unknown_license": [{"name": p["name"], "version": p["version"]} for p in rust_licenses if not p["license"] and not p["license_file"]]},
            "legal_review": "NOT RUN", "advisory_scan": "SEPARATE WORKFLOW"}

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--cargo-metadata", type=Path)
    args = parser.parse_args()
    npm_path = ROOT / "package-lock.json"
    cargo_path = ROOT / "src-tauri/Cargo.lock"
    result = inventory(json.loads(npm_path.read_text(encoding="utf-8")),
                       tomllib.loads(cargo_path.read_text(encoding="utf-8")),
                       json.loads(args.cargo_metadata.read_text(encoding="utf-8")) if args.cargo_metadata else None)
    result["lockfile_sha256"] = {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in (npm_path, cargo_path)}
    print(json.dumps(result, indent=2, ensure_ascii=False))

if __name__ == "__main__":
    main()
