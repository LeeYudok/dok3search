#!/usr/bin/env python3
"""Import source-only files (src/**) from LeeYudok/kordoc into frontend/src/samples/kordoc."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess

SOURCE_EXTS = {".ts", ".tsx", ".mts", ".cts", ".js", ".jsx", ".mjs", ".cjs"}
JSDOC_LINE_RE = re.compile(r"^\s*(?:/\*\*|\*|//)\s?(.*)$")


def git_commit_of(path: Path) -> str:
    try:
        return (
            subprocess.check_output(
                ["git", "-C", str(path), "rev-parse", "HEAD"],
                stderr=subprocess.DEVNULL,
            )
            .decode()
            .strip()
        )
    except (OSError, subprocess.CalledProcessError):
        return "unknown"


def extract_summary(text: str, rel_str: str) -> str:
    for line in text.splitlines()[:25]:
        m = JSDOC_LINE_RE.match(line)
        if not m:
            continue
        cleaned = m.group(1).strip().rstrip("*/").strip()
        if not cleaned or cleaned.startswith("@") or cleaned.startswith("!"):
            continue
        if len(cleaned) >= 8:
            return cleaned[:180]
    return f"kordoc TypeScript 소스 모듈 ({rel_str})"


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Import LeeYudok/kordoc src/** source-only corpus into dok3search."
    )
    parser.add_argument("--source", required=True, type=Path, help="Path to kordoc repo checkout")
    parser.add_argument(
        "--output",
        default=Path("frontend/src/samples/kordoc"),
        type=Path,
        help="Output corpus directory",
    )
    args = parser.parse_args()

    src_root = args.source / "src"
    if not src_root.is_dir():
        raise SystemExit(f"error: kordoc src/ directory not found at {src_root}")

    pkg_path = args.source / "package.json"
    pkg = json.loads(pkg_path.read_text(encoding="utf-8")) if pkg_path.is_file() else {}
    version = str(pkg.get("version") or "0.0.0")
    license_name = str(pkg.get("license") or "MIT")
    commit = git_commit_of(args.source)

    out_dir: Path = args.output
    out_src = out_dir / "src"
    if out_src.exists():
        shutil.rmtree(out_src)
    out_dir.mkdir(parents=True, exist_ok=True)

    sha_map: dict[str, str] = {}
    chapters: list[dict[str, object]] = []

    license_src = args.source / "LICENSE"
    if license_src.is_file():
        license_bytes = license_src.read_bytes()
        (out_dir / "LICENSE").write_bytes(license_bytes)
        sha_map["LICENSE"] = sha256_bytes(license_bytes)

    readme_text = (
        "# kordoc 소스 코퍼스 (HWP·HWPX·PDF → Markdown 파서)\n\n"
        f"- 출처: `https://github.com/LeeYudok/kordoc` (`v{version}`, 커밋 `{commit[:12]}`)\n"
        f"- 라이선스: {license_name}\n"
        "- 범위: `src/**` TypeScript 구현 소스만 반입 (`tests/`, `bench/`, 바이너리 픽스처 제외)\n\n"
        "재생성 명령:\n\n"
        "```bash\n"
        "python3 scripts/import-kordoc-source.py --source <kordoc-dir> --output frontend/src/samples/kordoc\n"
        "```\n"
    )
    readme_bytes = readme_text.encode("utf-8")
    (out_dir / "README.md").write_bytes(readme_bytes)
    sha_map["README.md"] = sha256_bytes(readme_bytes)
    chapters.append(
        {
            "path": "README.md",
            "display_title": "kordoc 소스 코퍼스 안내 (HWP·HWPX·PDF → Markdown)",
            "summary": "LeeYudok/kordoc TypeScript 파서 소스 코퍼스의 출처, 라이선스 및 재생성 방법입니다.",
            "source": "kordoc",
            "category": "code",
            "version": version,
            "tags": ["kordoc", "typescript", "hwp", "hwpx", "pdf"],
        }
    )

    for path in sorted(src_root.rglob("*")):
        if not path.is_file():
            continue
        if path.suffix.lower() not in SOURCE_EXTS:
            continue
        name_lower = path.name.lower()
        if name_lower.endswith(".test.ts") or name_lower.endswith(".spec.ts"):
            continue
        raw = path.read_bytes()
        if not raw.strip():
            continue
        try:
            text = raw.decode("utf-8")
        except UnicodeDecodeError:
            continue

        rel_from_src = path.relative_to(src_root).as_posix()
        rel_str = f"src/{rel_from_src}"
        dest = out_dir / rel_str
        dest.parent.mkdir(parents=True, exist_ok=True)
        dest.write_bytes(raw)
        sha_map[rel_str] = sha256_bytes(raw)

        parts = rel_from_src.split("/")
        tags = ["kordoc", "typescript"]
        if len(parts) > 1:
            top_mod = parts[0].lower()
            if top_mod not in tags:
                tags.append(top_mod)

        chapters.append(
            {
                "path": rel_str,
                "display_title": f"kordoc — {rel_str}",
                "summary": extract_summary(text, rel_str),
                "source": "kordoc",
                "category": "code",
                "version": version,
                "tags": tags,
            }
        )

    index_payload = {
        "source": "kordoc",
        "category": "code",
        "version": version,
        "chapters": chapters,
    }
    (out_dir / "index.json").write_text(
        json.dumps(index_payload, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )

    provenance_payload = {
        "repository": "https://github.com/LeeYudok/kordoc",
        "commit": commit,
        "release": f"v{version}",
        "license": license_name,
        "scope": "src-only",
        "count": len(chapters),
        "sha256": sha_map,
    }
    (out_dir / "provenance.json").write_text(
        json.dumps(provenance_payload, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )

    print(f"Imported {len(chapters)} kordoc corpus documents (v{version}, {commit[:12]}) -> {out_dir}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
