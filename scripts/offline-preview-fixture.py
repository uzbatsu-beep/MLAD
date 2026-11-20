#!/usr/bin/env python3
"""PR48 deterministic synthetic portable project for offline Windows UserSmoke.

Only locally invented test bytes. Never use real CIOPRG/IONAME exports.
Prepare refuses an existing directory, and verify checks SHA plus exact bytes.
Neither operation performs GUI, network, robot or OEM tests.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import pathlib
import tempfile

SOURCE = b"STR #70010\r\nOUT #70020\nEND\n"
IONAME = b"/IONAME\r\n//IN\r\nSynthetic A,Synthetic B,,\r\n"
SOURCE_NAME = "CIOPRG.LST"
IONAME_NAME = "IONAME.DAT"
SIDECAR_NAME = "cell.mlad.json"
PROJECT_NAME = "cell.mladproj.json"
GUARD_NAME = "SYNTHETIC-SHA256.json"
FILES = (SOURCE_NAME, IONAME_NAME, SIDECAR_NAME, PROJECT_NAME)


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def json_bytes(value: object) -> bytes:
    return (json.dumps(value, ensure_ascii=False, indent=2) + "\n").encode("utf-8")


def bundle() -> dict[str, bytes]:
    src_hash = digest(SOURCE)
    sidecar = {
        "format": "MLAD_OFFLINE_SIDECAR",
        "schema_version": 1,
        "source": {
            "kind": SOURCE_NAME,
            "controller": "YRC1000",
            "sha256": src_hash,
            "byte_length": len(SOURCE),
            "line_count": 3,
        },
        "annotations": {
            "rungs": {"L2:OUT70020": "СИНТЕТИКА: тестовая цепь"},
            "addresses": {"#70010": "СИНТЕТИКА: условный контакт"},
            "ioname": {},
        },
        "bookmarks": [{"line": 1, "label": "Синтетическая строка"}],
    }
    sidecar_bytes = json_bytes(sidecar)
    project = {
        "format": "MLAD_OFFLINE_PROJECT",
        "schema_version": 1,
        "source": {
            "relative_path": SOURCE_NAME,
            "sha256": src_hash,
            "byte_length": len(SOURCE),
            "line_count": 3,
            "controller": "YRC1000",
        },
        "ioname": {
            "relative_path": IONAME_NAME,
            "sha256": digest(IONAME),
            "byte_length": len(IONAME),
        },
        "sidecar": {
            "relative_path": SIDECAR_NAME,
            "sha256": digest(sidecar_bytes),
            "byte_length": len(sidecar_bytes),
        },
        "analysis": {
            "model": "SOURCE_ROLE_PREVIEW_V1",
            "source_sha256": src_hash,
            "typed_candidates": 0,
            "semantic_rungs": 1,
            "read_roles": 1,
            "write_roles": 1,
            "unknown_roles": 0,
            "execution_verified": False,
        },
    }
    return {
        SOURCE_NAME: SOURCE,
        IONAME_NAME: IONAME,
        SIDECAR_NAME: sidecar_bytes,
        PROJECT_NAME: json_bytes(project),
    }


def prepare(root: pathlib.Path) -> None:
    if root.exists():
        raise ValueError("Fixture destination already exists: refusal to overwrite")
    root.mkdir(parents=True, exist_ok=False)
    contents = bundle()
    for name, data in contents.items():
        (root / name).write_bytes(data)
    guard = {
        "schema": "MLAD_PR48_SYNTHETIC_FIXTURE_V1",
        "source_kind": "SYNTHETIC_NOT_OEM",
        "user_smoke": "NOT RUN",
        "oem_bench": "NOT RUN",
        "files": [
            {"relative_path": name, "sha256": digest(contents[name]), "bytes": len(contents[name])}
            for name in FILES
        ],
    }
    (root / GUARD_NAME).write_bytes(json_bytes(guard))
    verify(root)


def verify(root: pathlib.Path) -> None:
    guard = json.loads((root / GUARD_NAME).read_bytes())
    if guard.get("schema") != "MLAD_PR48_SYNTHETIC_FIXTURE_V1":
        raise ValueError("Unknown fixture guard version")
    if guard.get("source_kind") != "SYNTHETIC_NOT_OEM":
        raise ValueError("Unsafe source provenance")
    if guard.get("user_smoke") != "NOT RUN" or guard.get("oem_bench") != "NOT RUN":
        raise ValueError("Fixture may not assert manual/OEM acceptance")
    recorded = guard.get("files")
    if not isinstance(recorded, list) or len(recorded) != len(FILES):
        raise ValueError("Incomplete fixture inventory")
    expected = bundle()
    if set(root.iterdir()) != {root / name for name in (*FILES, GUARD_NAME)}:
        raise ValueError("Unexpected fixture files, directories or missing members")
    for i, name in enumerate(FILES):
        record = recorded[i]
        if record.get("relative_path") != name:
            raise ValueError("Fixture file order/name mismatch")
        candidate = root / name
        if candidate.is_symlink() or not candidate.is_file():
            raise ValueError("Fixture must consist of regular files")
        actual = candidate.read_bytes()
        if (
            actual != expected[name]
            or digest(actual) != record.get("sha256")
            or len(actual) != record.get("bytes")
        ):
            raise ValueError(f"Fixture source or metadata changed: {name}")
    project = json.loads((root / PROJECT_NAME).read_bytes())
    sidecar = json.loads((root / SIDECAR_NAME).read_bytes())
    if (
        project["source"]["sha256"] != sidecar["source"]["sha256"]
        or project["source"]["controller"] != sidecar["source"]["controller"]
        or project["analysis"]["execution_verified"] is not False
        or sidecar["source"]["line_count"] != project["source"]["line_count"]
        or project["sidecar"]["sha256"] != digest(expected[SIDECAR_NAME])
        or project["ioname"]["sha256"] != digest(IONAME)
    ):
        raise ValueError("Cross-file project/sidecar linkage mismatch")
    print(f"PASS synthetic project SHA, bytes, attachments and immutable source: {root}")
    print("This is not a GUI/firmware test. UserSmoke=NOT RUN; OEMBench=NOT RUN.")


def self_test() -> None:
    with tempfile.TemporaryDirectory(prefix="mlad-pr48-fixture-") as tmp:
        root = pathlib.Path(tmp) / "demo"
        prepare(root)
        verify(root)
        try:
            prepare(root)
        except ValueError:
            pass
        else:
            raise AssertionError("Prepare must not overwrite an existing project")
        for name in FILES:
            path = root / name
            original = path.read_bytes()
            path.write_bytes(original + b"x")
            try:
                verify(root)
            except ValueError:
                pass
            else:
                raise AssertionError(f"Fixture tampering not detected: {name}")
            path.write_bytes(original)
        verify(root)
    print("PASS PR48 synthetic fixture self-test (4 independent tampering failures)")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--mode", required=True, choices=("prepare", "verify", "self-test"))
    parser.add_argument("--root", type=pathlib.Path)
    args = parser.parse_args()
    if args.mode == "self-test":
        self_test()
        return
    if args.root is None:
        parser.error("--root is required except for --mode self-test")
    if args.mode == "prepare":
        prepare(args.root)
    else:
        verify(args.root)


if __name__ == "__main__":
    main()
