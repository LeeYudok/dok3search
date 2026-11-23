#!/usr/bin/env python3
"""Convert local rustdoc HTML (std/core/alloc + workspace/used crates) and rustc --explain into offline Markdown."""

import argparse
from concurrent.futures import ThreadPoolExecutor
from hashlib import sha256
import html
from html.parser import HTMLParser
import json
import os
from pathlib import Path, PurePosixPath
import re
import shutil
import subprocess
import tomllib


DOCUMENTATION_TOKEN_RE = re.compile(
    r'(?m)^(\s*token\s*=\s*")[A-Za-z0-9._~+/=-]{20,}("\s*(?:#.*)?)$'
)
REFRESH_RE = re.compile(r'<meta\b[^>]*http-equiv=["\']?refresh["\']?', re.IGNORECASE)
ITEM_FILE_RE = re.compile(
    r'^(struct|enum|trait|fn|macro|primitive|type|constant)\.([A-Za-z0-9_]+)\.html$'
)
WBR_RE = re.compile(r'<wbr\s*/?>', re.IGNORECASE)
MAX_DOC_BYTES = 260_000
MAX_DOCBLOCK_CHARS = 1_600
MAX_METHOD_ITEMS = 40


def redact_secrets(text: str) -> str:
    return DOCUMENTATION_TOKEN_RE.sub(r'\1<redacted-documentation-token>\2', text)


def run_rustc_version(rustc_bin: str) -> tuple[str, str]:
    res = subprocess.run(
        [rustc_bin, '-vV'],
        text=True,
        capture_output=True,
        check=True,
    )
    release = ''
    commit = ''
    first_line = ''
    for line in res.stdout.splitlines():
        if not first_line and line.startswith('rustc '):
            first_line = line.strip()
        if line.startswith('release:'):
            release = line.split(':', 1)[1].strip()
        elif line.startswith('commit-hash:'):
            commit = line.split(':', 1)[1].strip()[:9]
    if first_line:
        toolchain = first_line.removeprefix('rustc ').strip()
    elif release:
        toolchain = f'{release} ({commit})'.strip()
    else:
        toolchain = 'unknown'
    return release or toolchain, toolchain


def copy_rust_licenses(license_root: Path, output: Path) -> None:
    for name, src_name in [('LICENSE-MIT', 'MIT.txt'), ('LICENSE-APACHE', 'Apache-2.0.txt')]:
        src = license_root / src_name
        if not src.is_file():
            raise ValueError(f'Rust 라이선스 파일을 찾지 못했습니다: {src}')
        shutil.copyfile(src, output / name)
    (output / 'COPYRIGHT').write_text(
        'Documentation from the Rust Project. Copyright remains with the Rust Project Contributors. '
        'Unless a page says otherwise, this material is available under either the MIT License or '
        'Apache License, Version 2.0. See LICENSE-MIT and LICENSE-APACHE.\n',
        encoding='utf-8',
    )


def finalize_staged_corpus(
    output_root: Path,
    folder: str,
    staging: Path,
    index_payload: dict,
    provenance_payload: dict,
    readme_text: str,
) -> None:
    (staging / 'README.md').write_text(readme_text, encoding='utf-8')
    (staging / 'index.json').write_text(
        json.dumps(index_payload, ensure_ascii=False, indent=2) + '\n',
        encoding='utf-8',
    )
    generated_hashes = {
        str(path.relative_to(staging).as_posix()): sha256(path.read_bytes()).hexdigest()
        for path in sorted(staging.rglob('*'))
        if path.is_file() and path.name not in {'README.md', 'provenance.json', '.building'}
    }
    provenance_payload['generated_sha256'] = generated_hashes
    (staging / 'provenance.json').write_text(
        json.dumps(provenance_payload, ensure_ascii=False, indent=2, sort_keys=True) + '\n',
        encoding='utf-8',
    )

    final_output = output_root / folder
    backup = output_root / f'.{folder}.previous'
    if backup.exists():
        shutil.rmtree(backup)
    if final_output.exists():
        os.replace(final_output, backup)
    try:
        os.replace(staging, final_output)
    except Exception:
        if backup.exists():
            os.replace(backup, final_output)
        raise
    if backup.exists():
        shutil.rmtree(backup)
    (final_output / '.building').unlink(missing_ok=True)


def init_staging(output_root: Path, folder: str) -> Path:
    staging = output_root / f'.{folder}.staging'
    if staging.exists():
        shutil.rmtree(staging)
    staging.mkdir(parents=True, exist_ok=True)
    (staging / '.building').touch()
    return staging


# ---------------------------------------------------------------------------
# 1. rustc --explain E0001..E0999
# ---------------------------------------------------------------------------

def upgrade_bare_fences_to_rust(text: str) -> str:
    lines = text.splitlines()
    out = []
    in_fence = False
    for line in lines:
        stripped = line.strip()
        if stripped.startswith('```'):
            if not in_fence:
                in_fence = True
                if stripped == '```':
                    out.append('```rust')
                else:
                    out.append(line)
            else:
                in_fence = False
                out.append(line)
        else:
            out.append(line)
    return '\n'.join(out)


def fetch_explain(args: tuple[str, str]) -> tuple[str, str] | None:
    rustc_bin, code = args
    res = subprocess.run(
        [rustc_bin, '--explain', code],
        text=True,
        capture_output=True,
    )
    if res.returncode != 0 or not res.stdout.strip():
        return None
    return code, res.stdout


def build_rustc_error_codes(
    rustc_bin: str,
    output_root: Path,
    license_root: Path,
    version: str,
    toolchain: str,
) -> int:
    folder = 'rustc-error-codes'
    staging = init_staging(output_root, folder)
    copy_rust_licenses(license_root, staging)

    codes = [f'E{n:04d}' for n in range(1, 1000)]
    with ThreadPoolExecutor(max_workers=8) as pool:
        results = list(pool.map(fetch_explain, [(rustc_bin, c) for c in codes]))

    chapters_meta = []
    source_hashes = {}
    for item in results:
        if not item:
            continue
        code, raw = item
        normalized = upgrade_bare_fences_to_rust(redact_secrets(raw.strip()))
        paragraphs = [p.strip() for p in re.split(r'\n\s*\n', normalized) if p.strip()]
        summary = paragraphs[0].replace('\n', ' ') if paragraphs else f'Rust compiler error {code}'
        summary_clean = ' '.join(summary.split())[:220]
        rest = '\n\n'.join(paragraphs[1:]) if len(paragraphs) > 1 else normalized

        rel_md = f'chapters/{code}.md'
        md_path = staging / rel_md
        md_path.parent.mkdir(parents=True, exist_ok=True)
        body = (
            f'# Rust Compiler Error {code}\n\n'
            f'## Summary\n\n{summary_clean}\n\n'
            f'## Explanation\n\n{rest}\n'
        )
        md_path.write_text(body, encoding='utf-8')
        source_hashes[code] = sha256(raw.encode('utf-8')).hexdigest()
        chapters_meta.append({
            'path': rel_md,
            'title': f'Rust Compiler Error {code}',
            'summary': summary_clean,
            'tags': [code, 'error-code', 'rustc'],
        })

    index_payload = {
        'source': 'Rust Compiler Error Index (rustc --explain)',
        'source_url': 'https://doc.rust-lang.org/error_codes/',
        'toolchain': toolchain,
        'version': version,
        'category': 'code',
        'topic': 'rust',
        'chapters': chapters_meta,
    }
    provenance_payload = {
        'source': 'rustc --explain',
        'source_url': 'https://doc.rust-lang.org/error_codes/',
        'toolchain': toolchain,
        'version': version,
        'sha256': source_hashes,
    }
    readme = (
        '# Rust Compiler Error Index (`rustc --explain`)\n\n'
        f'Offline Markdown generated from `rustc --explain` ({toolchain}).\n\n'
        '## Rebuild\n\n'
        '```sh\n'
        'python3 scripts/import-rustdoc-and-errors.py --output-root frontend/src/samples\n'
        '```\n'
    )
    finalize_staged_corpus(output_root, folder, staging, index_payload, provenance_payload, readme)
    print(f'{folder}: {len(chapters_meta)}개 오류 코드 변환')
    return len(chapters_meta)


# ---------------------------------------------------------------------------
# 2. rustdoc HTML -> Markdown parser
# ---------------------------------------------------------------------------

class RustdocPageParser(HTMLParser):
    """Extracts declaration, top docblock, item table, and method summaries from rustdoc HTML."""

    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.skip_depth = 0
        self.skip_tags = {'script', 'style', 'nav', 'rustdoc-topbar', 'rustdoc-toolbar', 'button'}

        self.in_main = False
        self.main_depth = 0

        self.in_h1 = False
        self.h1_parts: list[str] = []

        self.in_item_decl = False
        self.decl_depth = 0
        self.decl_parts: list[str] = []

        self.in_top_doc = False
        self.top_doc_depth = 0
        self.in_top_docblock = False
        self.top_docblock_depth = 0
        self.top_doc_parts: list[str] = []

        self.in_pre = False
        self.in_code = False
        self.in_summary_hideme = False

        self.in_item_table = False
        self.item_table_depth = 0
        self.in_dt = False
        self.in_dd = False
        self.cur_dt: list[str] = []
        self.cur_dd: list[str] = []
        self.items: list[tuple[str, str]] = []

        self.in_impl_block = False
        self.impl_kind = ''  # 'inherent' | 'trait' | 'skip'
        self.in_method = False
        self.method_depth = 0
        self.in_method_sig = False
        self.method_sig_parts: list[str] = []
        self.in_method_doc = False
        self.method_doc_depth = 0
        self.method_doc_parts: list[str] = []
        self.methods: list[tuple[str, str]] = []

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        attr_map = dict(attrs)
        cls = (attr_map.get('class') or '').split()
        elem_id = attr_map.get('id') or ''

        if self.skip_depth > 0:
            self.skip_depth += 1
            return

        if tag in self.skip_tags or (tag == 'a' and ('anchor' in cls or 'doc-anchor' in cls or 'src' in cls)):
            self.skip_depth = 1
            return

        if tag == 'summary' and 'hideme' in cls:
            self.in_summary_hideme = True
            self.skip_depth = 1
            return

        if tag == 'main':
            self.in_main = True
            self.main_depth = 1
            return
        if self.in_main:
            self.main_depth += 1

        if not self.in_main:
            return

        if elem_id in {'synthetic-implementations', 'blanket-implementations'}:
            self.impl_kind = 'skip'

        if tag == 'h1' and not self.h1_parts:
            self.in_h1 = True

        if tag == 'pre' and 'item-decl' in cls and not self.decl_parts:
            self.in_item_decl = True
            self.decl_depth = 1
            return
        if self.in_item_decl:
            self.decl_depth += 1

        if tag == 'details' and 'top-doc' in cls and not self.in_top_doc:
            self.in_top_doc = True
            self.top_doc_depth = 1
            return
        if self.in_top_doc:
            self.top_doc_depth += 1
            if tag == 'div' and 'docblock' in cls and not self.in_top_docblock:
                self.in_top_docblock = True
                self.top_docblock_depth = 1
                return
            if self.in_top_docblock:
                self.top_docblock_depth += 1
                self._docblock_start(tag, cls)

        if tag == 'dl' and 'item-table' in cls and not self.in_item_table:
            self.in_item_table = True
            self.item_table_depth = 1
            return
        if self.in_item_table:
            self.item_table_depth += 1
            if tag == 'dt':
                self.in_dt = True
                self.cur_dt = []
            elif tag == 'dd':
                self.in_dd = True
                self.cur_dd = []

        if self.impl_kind != 'skip' and tag == 'section' and 'method' in cls and not self.in_method:
            self.in_method = True
            self.method_depth = 1
            self.method_sig_parts = []
            self.method_doc_parts = []
            self.in_method_sig = True
            return
        if self.in_method:
            self.method_depth += 1
            if tag == 'div' and 'docblock' in cls and not self.in_method_doc:
                self.in_method_sig = False
                self.in_method_doc = True
                self.method_doc_depth = 1
                return
            if self.in_method_doc:
                self.method_doc_depth += 1

    def _docblock_start(self, tag: str, cls: list[str]) -> None:
        if tag == 'pre':
            self.in_pre = True
            self.top_doc_parts.append('\n```rust\n')
        elif tag == 'code' and not self.in_pre:
            self.in_code = True
            self.top_doc_parts.append('`')
        elif tag in {'h2', 'h3', 'h4', 'h5'}:
            self.top_doc_parts.append('\n**')
        elif tag == 'li':
            self.top_doc_parts.append('\n- ')
        elif tag == 'p':
            self.top_doc_parts.append('\n\n')
        elif tag == 'br':
            self.top_doc_parts.append('\n')

    def handle_endtag(self, tag: str) -> None:
        if self.skip_depth > 0:
            self.skip_depth -= 1
            if self.skip_depth == 0:
                self.in_summary_hideme = False
            return

        if not self.in_main:
            return

        if tag == 'h1' and self.in_h1:
            self.in_h1 = False

        if self.in_item_decl:
            self.decl_depth -= 1
            if self.decl_depth == 0:
                self.in_item_decl = False

        if self.in_top_docblock:
            if tag == 'pre' and self.in_pre:
                self.in_pre = False
                self.top_doc_parts.append('\n```\n')
            elif tag == 'code' and self.in_code and not self.in_pre:
                self.in_code = False
                self.top_doc_parts.append('`')
            elif tag in {'h2', 'h3', 'h4', 'h5'}:
                self.top_doc_parts.append('**\n\n')
            elif tag == 'p':
                self.top_doc_parts.append('\n')
            self.top_docblock_depth -= 1
            if self.top_docblock_depth == 0:
                self.in_top_docblock = False

        if self.in_top_doc:
            self.top_doc_depth -= 1
            if self.top_doc_depth == 0:
                self.in_top_doc = False

        if self.in_item_table:
            if tag == 'dt' and self.in_dt:
                self.in_dt = False
            elif tag == 'dd' and self.in_dd:
                self.in_dd = False
                dt = ' '.join(''.join(self.cur_dt).split())
                dd = ' '.join(''.join(self.cur_dd).split())
                if dt:
                    self.items.append((dt, dd))
            self.item_table_depth -= 1
            if self.item_table_depth == 0:
                self.in_item_table = False

        if self.in_method:
            if self.in_method_doc:
                self.method_doc_depth -= 1
                if self.method_doc_depth == 0:
                    self.in_method_doc = False
            self.method_depth -= 1
            if self.method_depth == 0:
                self.in_method = False
                self.in_method_sig = False
                sig = ' '.join(''.join(self.method_sig_parts).split())
                doc = ' '.join(''.join(self.method_doc_parts).split())
                if sig:
                    self.methods.append((sig, doc[:180]))

        self.main_depth -= 1
        if self.main_depth == 0:
            self.in_main = False

    def handle_data(self, data: str) -> None:
        if self.skip_depth > 0 or not self.in_main:
            return
        if self.in_h1:
            self.h1_parts.append(data)
        if self.in_item_decl:
            self.decl_parts.append(data)
        if self.in_top_docblock:
            self.top_doc_parts.append(data)
        if self.in_dt:
            self.cur_dt.append(data)
        if self.in_dd:
            self.cur_dd.append(data)
        if self.in_method_sig:
            self.method_sig_parts.append(data)
        if self.in_method_doc and len(''.join(self.method_doc_parts)) < 240:
            self.method_doc_parts.append(data)


def parse_rustdoc_html(path: Path) -> RustdocPageParser | None:
    raw = path.read_text(encoding='utf-8', errors='replace')
    if REFRESH_RE.search(raw[:1024]):
        return None
    cleaned = WBR_RE.sub('', raw)
    parser = RustdocPageParser()
    parser.feed(cleaned)
    return parser


def clean_docblock_text(parts: list[str], max_chars: int = MAX_DOCBLOCK_CHARS) -> str:
    text = html.unescape(''.join(parts))
    text = re.sub(r'\n{3,}', '\n\n', text).strip()
    text = redact_secrets(text)
    if len(text) <= max_chars:
        return text
    # Avoid truncating inside an open code fence.
    prefix = text[:max_chars]
    if prefix.count('```') % 2 == 1:
        last_fence = prefix.rfind('```')
        if last_fence > 0:
            prefix = prefix[:last_fence].rstrip()
    return prefix.rstrip() + '\n\n...'


def render_item_section(kind: str, mod_path: str, name: str, parsed: RustdocPageParser) -> str:
    heading = f'{kind} {mod_path}::{name}'
    parts = [f'### {heading}']
    decl = html.unescape(''.join(parsed.decl_parts)).strip()
    if decl:
        parts.append(f'```rust\n{decl}\n```')
    docblock = clean_docblock_text(parsed.top_doc_parts)
    if docblock:
        parts.append(docblock)
    if parsed.methods:
        method_lines = []
        for sig, mdoc in parsed.methods[:MAX_METHOD_ITEMS]:
            sig_clean = sig.removesuffix('ⓘ').strip()
            if mdoc:
                method_lines.append(f'- `{sig_clean}` — {mdoc}')
            else:
                method_lines.append(f'- `{sig_clean}`')
        parts.append('**Methods**\n\n' + '\n'.join(method_lines))
    return '\n\n'.join(parts)


def convert_rustdoc_crate(
    crate_root: Path,
    crate_name: str,
    staging: Path,
    package_version: str,
    category: str,
    source_label: str,
) -> list[dict]:
    if not crate_root.is_dir():
        return []

    modules: dict[Path, list[Path]] = {}
    for html_file in sorted(crate_root.rglob('*.html')):
        if html_file.name in {'all.html', 'index.html'}:
            modules.setdefault(html_file.parent, [])
            continue
        if ITEM_FILE_RE.match(html_file.name):
            modules.setdefault(html_file.parent, []).append(html_file)

    chapters: list[dict] = []
    for mod_dir, item_files in sorted(modules.items()):
        rel_dir = mod_dir.relative_to(crate_root)
        mod_parts = [crate_name] + list(rel_dir.parts)
        mod_path = '::'.join(mod_parts)

        index_html = mod_dir / 'index.html'
        index_parsed = parse_rustdoc_html(index_html) if index_html.is_file() else None
        if index_parsed is None and not item_files:
            continue

        title = f'Crate {crate_name}' if rel_dir == Path('.') else f'Module {mod_path}'
        lines = [f'# {title}']

        overview = ''
        if index_parsed:
            overview = clean_docblock_text(index_parsed.top_doc_parts, max_chars=2_400)
            if overview:
                lines.append(f'## Overview\n\n{overview}')
            if index_parsed.items:
                item_lines = [
                    f'- `{name}` — {desc}' if desc else f'- `{name}`'
                    for name, desc in index_parsed.items[:120]
                ]
                lines.append('## Items\n\n' + '\n'.join(item_lines))

        item_sections: list[str] = []
        for item_file in sorted(item_files):
            m = ITEM_FILE_RE.match(item_file.name)
            if not m:
                continue
            kind, item_name = m.group(1), m.group(2)
            parsed = parse_rustdoc_html(item_file)
            if parsed is None:
                continue
            sec = render_item_section(kind, mod_path, item_name, parsed)
            if sec:
                item_sections.append(sec)

        if len(lines) <= 1 and not item_sections:
            continue

        # Split oversized modules into bounded chunks (< 260 KB) so LocalFileConnector never skips.
        base_rel = PurePosixPath('chapters') / crate_name / (
            'index.md' if rel_dir == Path('.') else PurePosixPath(*rel_dir.parts).with_suffix('.md')
        )
        chunks: list[tuple[PurePosixPath, str, str]] = []
        header_block = '\n\n'.join(lines)
        current_blocks = [header_block] if len(lines) > 1 or not item_sections else [f'# {title}']
        current_len = sum(len(b) for b in current_blocks)
        part_num = 1

        for sec in item_sections:
            if current_len + len(sec) + 4 > MAX_DOC_BYTES and len(current_blocks) > 1:
                out_rel = base_rel if part_num == 1 else base_rel.with_name(
                    f'{base_rel.stem}-part{part_num}.md'
                )
                part_title = title if part_num == 1 else f'{title} (Part {part_num})'
                chunks.append((out_rel, part_title, '\n\n'.join(current_blocks) + '\n'))
                part_num += 1
                current_blocks = [f'# {title} (Part {part_num})']
                current_len = len(current_blocks[0])
            current_blocks.append(sec)
            current_len += len(sec) + 2

        if current_blocks:
            out_rel = base_rel if part_num == 1 else base_rel.with_name(
                f'{base_rel.stem}-part{part_num}.md'
            )
            part_title = title if part_num == 1 else f'{title} (Part {part_num})'
            chunks.append((out_rel, part_title, '\n\n'.join(current_blocks) + '\n'))

        summary_src = overview.splitlines()[0] if overview else f'{title} API reference'
        summary_clean = ' '.join(summary_src.replace('`', '').split())[:200]
        if not summary_clean:
            summary_clean = f'{title} API reference'

        for out_rel, ch_title, content in chunks:
            dest = staging / Path(*out_rel.parts)
            dest.parent.mkdir(parents=True, exist_ok=True)
            dest.write_text(content, encoding='utf-8')
            chapters.append({
                'path': str(out_rel),
                'title': ch_title,
                'summary': summary_clean,
                'source': source_label,
                'category': category,
                'package': crate_name,
                'package_version': package_version,
                'tags': [crate_name, 'rustdoc'],
            })

    return chapters


def build_rust_std(
    rust_docs_root: Path,
    license_root: Path,
    output_root: Path,
    version: str,
    toolchain: str,
) -> int:
    folder = 'rust-std'
    staging = init_staging(output_root, folder)
    copy_rust_licenses(license_root, staging)

    chapters: list[dict] = []
    for crate in ('std', 'core', 'alloc'):
        crate_dir = rust_docs_root / crate
        if not crate_dir.is_dir():
            raise ValueError(f'rust-docs {crate} 디렉터리가 없습니다: {crate_dir}')
        chapters.extend(
            convert_rustdoc_crate(
                crate_dir,
                crate,
                staging,
                package_version=version,
                category='code',
                source_label=f'Rust Standard Library ({crate})',
            )
        )

    index_payload = {
        'source': 'Rust Standard Library (std/core/alloc)',
        'source_url': 'https://doc.rust-lang.org/std/',
        'toolchain': toolchain,
        'version': version,
        'category': 'code',
        'topic': 'rust',
        'chapters': chapters,
    }
    provenance_payload = {
        'source': 'rustup rust-docs',
        'source_url': 'https://doc.rust-lang.org/std/',
        'toolchain': toolchain,
        'version': version,
    }
    readme = (
        '# Rust Standard Library (`std`, `core`, `alloc`)\n\n'
        f'Offline Markdown generated from local `rust-docs` ({toolchain}).\n\n'
        '## Rebuild\n\n'
        '```sh\n'
        'python3 scripts/import-rustdoc-and-errors.py --output-root frontend/src/samples\n'
        '```\n'
    )
    finalize_staged_corpus(output_root, folder, staging, index_payload, provenance_payload, readme)
    print(f'{folder}: {len(chapters)}개 모듈 문서 변환')
    return len(chapters)


def collect_used_crate_versions(workspace_dir: Path) -> list[tuple[str, str, str]]:
    """Returns [(cargo_name, rustdoc_dir_name, version)] for workspace crates + direct deps."""
    cargo_toml = tomllib.loads((workspace_dir / 'Cargo.toml').read_text(encoding='utf-8'))
    lock = tomllib.loads((workspace_dir / 'Cargo.lock').read_text(encoding='utf-8'))

    versions_by_name: dict[str, list[str]] = {}
    for pkg in lock.get('package', []):
        versions_by_name.setdefault(pkg['name'], []).append(pkg['version'])

    selected: list[tuple[str, str]] = []
    seen = set()

    for member in cargo_toml.get('workspace', {}).get('members', []):
        member_toml = tomllib.loads((workspace_dir / member / 'Cargo.toml').read_text(encoding='utf-8'))
        pkg_name = member_toml['package']['name']
        if pkg_name not in seen:
            seen.add(pkg_name)
            selected.append((pkg_name, pkg_name.replace('-', '_')))

        for dep_section in ('dependencies', 'dev-dependencies'):
            for dep_name in member_toml.get(dep_section, {}):
                if dep_name not in seen and not dep_name.startswith('dok3-'):
                    seen.add(dep_name)
                    selected.append((dep_name, dep_name.replace('-', '_')))

    out = []
    for name, doc_dir in selected:
        vers = versions_by_name.get(name, ['0.1.0'])
        out.append((name, doc_dir, vers[0]))
    return out


def build_rust_crates(
    workspace_dir: Path,
    cargo_doc_dir: Path,
    output_root: Path,
    toolchain: str,
) -> int:
    folder = 'rust-crates'
    staging = init_staging(output_root, folder)

    crates = collect_used_crate_versions(workspace_dir)
    chapters: list[dict] = []
    pkg_summary = []

    for pkg_name, doc_dir_name, pkg_ver in crates:
        crate_html_dir = cargo_doc_dir / doc_dir_name
        if not crate_html_dir.is_dir():
            continue
        converted = convert_rustdoc_crate(
            crate_html_dir,
            pkg_name,
            staging,
            package_version=pkg_ver,
            category='code',
            source_label=f'{pkg_name} {pkg_ver}',
        )
        if converted:
            chapters.extend(converted)
            pkg_summary.append({
                'name': pkg_name,
                'version': pkg_ver,
                'document_count': len(converted),
            })

    if not chapters:
        raise ValueError(f'cargo doc 산출물을 찾지 못했습니다: {cargo_doc_dir}')

    index_payload = {
        'source': 'Rust Workspace & Dependency Crates (cargo doc)',
        'toolchain': toolchain,
        'version': toolchain,
        'category': 'code',
        'topic': 'rust',
        'packages': pkg_summary,
        'chapters': chapters,
    }
    provenance_payload = {
        'source': 'cargo doc --no-deps',
        'toolchain': toolchain,
        'packages': pkg_summary,
    }
    readme = (
        '# Rust Workspace & Dependency Crates (`cargo doc`)\n\n'
        f'Offline Markdown generated from `backend/rust/target/doc` ({toolchain}).\n\n'
        '## Rebuild\n\n'
        '```sh\n'
        'cargo doc --manifest-path backend/rust/Cargo.toml --workspace --no-deps\n'
        'python3 scripts/import-rustdoc-and-errors.py --output-root frontend/src/samples\n'
        '```\n'
    )
    finalize_staged_corpus(output_root, folder, staging, index_payload, provenance_payload, readme)
    print(f'{folder}: {len(pkg_summary)}개 크레이트, {len(chapters)}개 모듈 문서 변환')
    return len(chapters)


def main() -> None:
    repo_root = Path(__file__).resolve().parents[1]
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output-root', type=Path, default=repo_root / 'frontend/src/samples')
    parser.add_argument('--workspace', type=Path, default=repo_root / 'backend/rust')
    parser.add_argument('--cargo-doc-dir', type=Path, default=None)
    parser.add_argument('--rustc', default='rustc')
    parser.add_argument('--rust-docs-root', type=Path, default=None)
    parser.add_argument('--license-root', type=Path, default=None)
    args = parser.parse_args()

    sysroot = Path(
        subprocess.check_output([args.rustc, '--print', 'sysroot'], text=True).strip()
    )
    rust_docs_root = args.rust_docs_root or (sysroot / 'share/doc/rust/html')
    license_root = args.license_root or (sysroot / 'share/doc/rust/licenses')
    cargo_doc_dir = args.cargo_doc_dir or (args.workspace / 'target/doc')
    version, toolchain = run_rustc_version(args.rustc)

    build_rustc_error_codes(args.rustc, args.output_root, license_root, version, toolchain)
    build_rust_std(rust_docs_root, license_root, args.output_root, version, toolchain)
    build_rust_crates(args.workspace, cargo_doc_dir, args.output_root, toolchain)


if __name__ == '__main__':
    main()
