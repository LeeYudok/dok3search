#!/usr/bin/env python3
"""Import curated technical posts from blog-doksam into frontend/src/samples/doksam-blog."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess
import tomllib

INCLUDED_MISC_SLUGS = {
    "airgap-missing-tools",
    "agent-docs-drift",
    "agent-worker-gate-hook",
    "agents-scaffold-commit-gate",
    "dev-tools-guide",
    "dev-tools-guide-2",
    "docs-as-build-kit",
    "git-incidents-shared-main",
    "million-line-codebase-agents",
    "orca-worktree-workflow",
    "sonarqube-does-not-refactor",
    "web-tech-stack-2026",
    "web-tech-stack-2026-2",
    "why-go-and-rust",
    "why-llm-router-openrouter-orcarouter",
    "windows-wsl-mac-linux-for-ai",
}

IMAGE_RE = re.compile(r"!\[([^\]]*)\]\((/images/[^)]+)\)")
BLOG_LINK_RE = re.compile(r"\]\((/blog/[^)]+)\)")


def classify_slug(slug: str) -> tuple[bool, str, str | None]:
    if slug.startswith("rust-course-"):
        return True, "doksam 기술 블로그 — Rust 강의", "rust"
    if slug.startswith("go-course-"):
        return True, "doksam 기술 블로그 — Go 강의", "go"
    if (
        slug.startswith("claude-code-guide-")
        or slug.startswith("codex-guide-")
        or slug == "codex-0-171-0-changelog"
        or slug.startswith("agy-guide-")
    ):
        return True, "doksam 기술 블로그 — AI 에이전트 가이드", None
    if slug in INCLUDED_MISC_SLUGS:
        return True, "doksam 기술 블로그", None
    return False, "", None


def parse_hugo_post(raw: str) -> tuple[dict, str]:
    if raw.startswith("+++"):
        parts = raw.split("+++", 2)
        if len(parts) == 3:
            meta = tomllib.loads(parts[1])
            return meta, parts[2].lstrip("\r\n")
    return {}, raw


def normalize_body(title: str, body: str) -> str:
    body = IMAGE_RE.sub(lambda m: f"> [그림] {m.group(1) or '다이어그램'}", body)
    body = BLOG_LINK_RE.sub(lambda m: f"](https://blog.doksam.com{m.group(1)})", body)
    return f"# {title}\n\n{body.strip()}\n"


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


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path, required=True, help="blog-doksam site/content/blog directory")
    parser.add_argument("--output", type=Path, required=True, help="output corpus directory (e.g. frontend/src/samples/doksam-blog)")
    args = parser.parse_args()

    chapters_dir = args.output / "chapters"
    chapters_dir.mkdir(parents=True, exist_ok=True)
    for existing in chapters_dir.glob("*.md"):
        existing.unlink()

    chapters = []
    sha256_map: dict[str, str] = {}

    for src in sorted(args.source.glob("*.md")):
        slug = src.stem
        if slug == "_index":
            continue
        include, source_label, topic = classify_slug(slug)
        if not include:
            continue

        raw_bytes = src.read_bytes()
        meta, body = parse_hugo_post(raw_bytes.decode("utf-8"))
        if meta.get("draft", False):
            continue

        title = str(meta.get("title") or slug).strip()
        summary = str(meta.get("description") or "").strip()
        date = str(meta.get("date") or "").strip()
        tags = [str(t).strip() for t in meta.get("tags", []) if str(t).strip()]

        rendered = normalize_body(title, body)
        rel_path = f"chapters/{slug}.md"
        out_file = args.output / rel_path
        out_file.write_text(rendered, encoding="utf-8")
        sha256_map[rel_path] = hashlib.sha256(raw_bytes).hexdigest()

        entry: dict[str, object] = {
            "path": rel_path,
            "title": title,
            "summary": summary,
            "source": source_label,
            "category": "guide",
            "tags": tags,
        }
        if topic:
            entry["topic"] = topic
        if date:
            entry["date"] = date
        chapters.append(entry)

    index_payload = {
        "source": "doksam 기술 블로그",
        "category": "guide",
        "chapters": chapters,
    }
    (args.output / "index.json").write_text(
        json.dumps(index_payload, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )

    provenance = {
        "repository": "ssh://git@gitlab.doksam.com:38929/doksam/blog-doksam.git",
        "commit": git_commit_of(args.source),
        "license": "doksam internal / blog.doksam.com",
        "count": len(chapters),
        "sha256": sha256_map,
    }
    (args.output / "provenance.json").write_text(
        json.dumps(provenance, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )

    readme = (
        "# doksam 기술 블로그 코퍼스\n\n"
        "`blog.doksam.com` (`blog-doksam/site/content/blog/`)에서 선별한 기술 강의·에이전트 가이드·실무 글을 "
        "폐쇄망 검색용 Markdown으로 변환한 코퍼스입니다.\n\n"
        f"- 포함 문서: {len(chapters)}편 (Rust 강의, Go 강의, AI 에이전트 가이드, 폐쇄망/개발 실무)\n"
        "- 제외 문서: 증시 시황(`market-*`) 및 개인/비기술 글\n"
        "- 재생성: `python3 scripts/import-doksam-blog.py --source <blog-doksam>/site/content/blog --output frontend/src/samples/doksam-blog`\n"
    )
    (args.output / "README.md").write_text(readme, encoding="utf-8")
    print(f"imported {len(chapters)} posts into {args.output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
