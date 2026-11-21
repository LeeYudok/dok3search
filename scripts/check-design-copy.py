#!/usr/bin/env python3
"""Verify the bundled design snapshot against its recorded source."""

import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import sys


ROOT = Path(__file__).resolve().parents[1]
COPY = ROOT / "frontend/src/samples/design"
MANIFEST = COPY / "provenance.json"


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def run_git(source: Path, *args: str) -> bytes:
    return subprocess.check_output(["git", "-C", str(source), *args])


def differences(expected: dict[str, str], actual: dict[str, str]) -> list[str]:
    return sorted(name for name in expected.keys() | actual.keys() if expected.get(name) != actual.get(name))


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path, help="설계 원본 Git 저장소의 HEAD도 대조")
    args = parser.parse_args()

    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    expected = manifest["sha256"]
    local = {
        path.relative_to(COPY).as_posix(): digest(path.read_bytes())
        for path in COPY.rglob("*")
        if path.is_file() and path != MANIFEST
    }
    changed = differences(expected, local)
    if changed:
        print("설계 사본 불일치:", *changed, sep="\n", file=sys.stderr)
        return 1

    revision = manifest["commit"]
    if args.source:
        actual_revision = run_git(args.source, "rev-parse", "HEAD").decode().strip()
        if actual_revision != revision:
            print(f"설계 원본 HEAD 불일치 ({actual_revision}, 기록값 {revision})", file=sys.stderr)
            return 1
        paths = run_git(args.source, "ls-tree", "-rz", "--name-only", revision).decode().split("\0")
        source = {name: digest(run_git(args.source, "show", f"{revision}:{name}")) for name in paths if name}
        changed = differences(expected, source)
        if changed:
            print("설계 원본 불일치:", *changed, sep="\n", file=sys.stderr)
            return 1

    print(f"설계 사본 {len(local)}개 검증 통과 ({revision})")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, ValueError, KeyError, subprocess.CalledProcessError) as error:
        sys.exit(f"설계 사본 검사 실패 ({error})")
