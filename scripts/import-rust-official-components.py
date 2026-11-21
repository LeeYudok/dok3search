#!/usr/bin/env python3
"""Convert installed Rust Reference and Cargo Book HTML into offline Markdown."""

import argparse
import ast
from hashlib import sha256
import html
from html.parser import HTMLParser
import json
import os
import posixpath
from pathlib import Path, PurePosixPath
import re
import shutil
import subprocess
from urllib.parse import unquote, urlsplit


COMPONENTS = {
    'reference': ('rust-reference', 'The Rust Reference', 'https://doc.rust-lang.org/1.100.0/reference/'),
    'cargo': ('cargo-book', 'The Cargo Book', 'https://doc.rust-lang.org/1.100.0/cargo/'),
}
IMAGE_SUFFIXES = {'.gif', '.jpeg', '.jpg', '.png', '.svg', '.webp'}
MAIN_RE = re.compile(r'<main(?:\s[^>]*)?>(.*?)</main\s*>', re.IGNORECASE | re.DOTALL)
JS_STRING_RE = re.compile(r"this\.innerHTML\s*=\s*('(?:\\.|[^'])*')\s*;", re.DOTALL)
ATTRIBUTE_RE = re.compile(r'\b(href|src)=("([^"]*)"|\'([^\']*)\')', re.IGNORECASE)
SVG_RE = re.compile(r'<svg\b[^>]*>.*?</svg\s*>', re.IGNORECASE | re.DOTALL)
HEADER_LINK_RE = re.compile(
    r'<a\b(?=[^>]*\bclass=["\'][^"\']*\bheader\b[^"\']*["\'])[^>]*>(.*?)</a\s*>',
    re.IGNORECASE | re.DOTALL,
)
LAYOUT_TAG_RE = re.compile(r'</?(?:div|span|section|aside|article|figure|figcaption)\b[^>]*>', re.IGNORECASE)
STYLE_RE = re.compile(r'<style\b[^>]*>.*?</style\s*>', re.IGNORECASE | re.DOTALL)
RAW_ANCHOR_RE = re.compile(r'<a\b([^>]*)>(.*?)</a\s*>', re.IGNORECASE | re.DOTALL)
RAW_TAG_RE = re.compile(r'</?([A-Za-z][A-Za-z0-9-]*)\b[^>]*>', re.IGNORECASE)
INLINE_CODE_RE = re.compile(r'(`+)(.+?)\1')
DOCUMENTATION_TOKEN_RE = re.compile(r'(?m)^(\s*token\s*=\s*")[A-Za-z0-9._~+/=-]{20,}("\s*(?:#.*)?)$')


class TocParser(HTMLParser):
    def __init__(self):
        super().__init__()
        self.items = []
        self.current = None

    def handle_starttag(self, tag, attrs):
        if tag == 'a':
            href = dict(attrs).get('href', '')
            self.current = [href, []]

    def handle_data(self, data):
        if self.current:
            self.current[1].append(data)

    def handle_endtag(self, tag):
        if tag == 'a' and self.current:
            href, title = self.current
            self.items.append((href, ' '.join(''.join(title).split())))
            self.current = None


class MainTextParser(HTMLParser):
    def __init__(self):
        super().__init__()
        self.depth = 0
        self.heading = False
        self.paragraph = False
        self.heading_parts = []
        self.paragraph_parts = []
        self.in_heading = False
        self.in_paragraph = False

    def handle_starttag(self, tag, attrs):
        if tag == 'h1' and not self.heading:
            self.heading = True
            self.in_heading = True
        elif tag == 'p' and not self.paragraph:
            self.paragraph = True
            self.in_paragraph = True

    def handle_endtag(self, tag):
        if tag == 'h1' and self.in_heading:
            self.in_heading = False
        elif tag == 'p' and self.in_paragraph:
            self.in_paragraph = False

    def handle_data(self, data):
        if self.in_heading:
            self.heading_parts.append(data)
        if self.in_paragraph:
            self.paragraph_parts.append(data)


def toc_pages(component):
    toc_files = list(component.glob('toc-*.js'))
    if len(toc_files) != 1:
        raise ValueError(f'{component}: toc-*.js 파일이 하나여야 합니다.')
    script = toc_files[0].read_text(encoding='utf-8')
    match = JS_STRING_RE.search(script)
    if not match:
        raise ValueError(f'{toc_files[0]}: mdBook 목차 문자열을 찾지 못했습니다.')
    parser = TocParser()
    parser.feed(ast.literal_eval(match.group(1)))
    pages = []
    for href, title in parser.items:
        path = unquote(urlsplit(href).path)
        if path.endswith('.html') and not path.startswith('/') and path not in pages:
            pages.append(path)
    if not pages:
        raise ValueError(f'{component}: 목차에서 HTML 페이지를 찾지 못했습니다.')
    return pages


def page_main(source, relative):
    text = (source / relative).read_text(encoding='utf-8')
    match = MAIN_RE.search(text)
    if not match:
        raise ValueError(f'{relative}: main 본문을 찾지 못했습니다.')
    return match.group(1)


def local_target(component, source_page, href):
    parts = urlsplit(href)
    if parts.scheme or parts.netloc or not parts.path.endswith('.html'):
        return None
    target = posixpath.normpath(posixpath.join(posixpath.dirname(source_page), unquote(parts.path)))
    if target == '..' or target.startswith('../') or not (component / target).is_file():
        return None
    return target, parts.fragment


def normalize_markdown(markdown):
    markdown = STYLE_RE.sub('', markdown)
    markdown = re.sub(r'<!--.*?-->', '', markdown, flags=re.DOTALL)
    lines = markdown.splitlines()
    output = []
    in_fence = False
    fence_marker = ''
    for line in lines:
        fence = re.match(r'^\s{0,3}(`{3,}|~{3,})', line)
        if fence:
            marker = fence.group(1)
            if not in_fence:
                in_fence, fence_marker = True, marker[0]
            elif marker[0] == fence_marker:
                in_fence, fence_marker = False, ''
            output.append(line)
            continue
        if in_fence:
            output.append(line)
            continue

        protected = []

        def protect_code(match):
            protected.append(match.group(0))
            return f'\x00CODE{len(protected) - 1}\x00'

        line = INLINE_CODE_RE.sub(protect_code, line)
        line = re.sub(r'<sup>\[(\d+)\]\(#footnote-\1\)</sup>', r'[^\1]', line)
        line = re.sub(r'^(\s*)(\d+)\.\s*<div id="footnote-\2">', r'\1[^\2]: ', line)
        def convert_anchor(match):
            attributes, body = match.group(1), match.group(2)
            href_match = re.search(r'\bhref=(?:"([^"]*)"|\'([^\']*)\')', attributes, re.IGNORECASE)
            label = html.unescape(RAW_TAG_RE.sub('', body)).strip()
            if label.startswith('[') and label.endswith(']'):
                label = label[1:-1]
            href_value = (href_match.group(1) if href_match.group(1) is not None else href_match.group(2)) if href_match else ''
            href = html.unescape(href_value or '')
            if not href or href.lower().startswith('javascript:'):
                return label
            return f'[{label}]({href})'

        line = RAW_ANCHOR_RE.sub(convert_anchor, line)
        line = re.sub(r'<table\b[^>]*>', '\n', line, flags=re.IGNORECASE)
        line = re.sub(r'</table\s*>', '\n', line, flags=re.IGNORECASE)
        line = re.sub(r'<tr\b[^>]*>', '\n', line, flags=re.IGNORECASE)
        line = re.sub(r'</tr\s*>', '\n', line, flags=re.IGNORECASE)
        line = re.sub(r'<t[hd]\b[^>]*>', ' | ', line, flags=re.IGNORECASE)
        line = re.sub(r'</t[hd]\s*>', ' | ', line, flags=re.IGNORECASE)
        line = re.sub(r'<br\s*/?>', '\n', line, flags=re.IGNORECASE)
        line = re.sub(r'<li\b[^>]*>', '\n- ', line, flags=re.IGNORECASE)
        line = re.sub(r'</li\s*>', '\n', line, flags=re.IGNORECASE)
        line = re.sub(r'</?(?:div|p|section|article|aside|figure|figcaption)\b[^>]*>', '\n', line, flags=re.IGNORECASE)
        line = RAW_TAG_RE.sub('', line)
        for index, code in enumerate(protected):
            line = line.replace(f'\x00CODE{index}\x00', code)
        output.append(line)
    return re.sub(r'\n{3,}', '\n\n', '\n'.join(output)).strip()


def redact_documentation_secrets(markdown):
    return DOCUMENTATION_TOKEN_RE.sub(r'\1<redacted-documentation-token>\2', markdown)


def referenced_pages(component, ordered_pages):
    pages = list(ordered_pages)
    seen = set(pages)
    cursor = 0
    while cursor < len(pages):
        relative = pages[cursor]
        cursor += 1
        main = page_main(component, relative)
        for match in ATTRIBUTE_RE.finditer(main):
            if match.group(1).lower() != 'href':
                continue
            target = local_target(component, relative, match.group(3) or match.group(4) or '')
            if target and target[0] not in seen:
                seen.add(target[0])
                pages.append(target[0])
    return pages


def normalize_asset(component, output, source_page, markdown_page, value, source_hashes):
    parts = urlsplit(value)
    if parts.scheme or parts.netloc or not parts.path:
        return None
    source_path = posixpath.normpath(posixpath.join(posixpath.dirname(source_page), unquote(parts.path)))
    if source_path.startswith('../') or Path(source_path).suffix.lower() not in IMAGE_SUFFIXES:
        return None
    source_file = component / source_path
    if not source_file.is_file():
        return None
    asset_path = PurePosixPath('assets') / PurePosixPath(source_path)
    destination = output / Path(*asset_path.parts)
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(source_file, destination)
    source_hashes[source_path] = sha256(source_file.read_bytes()).hexdigest()
    return posixpath.relpath(str(asset_path), start=posixpath.dirname(markdown_page) or '.') + (
        '#' + parts.fragment if parts.fragment else ''
    )


def prepare_output(output):
    if not output.exists():
        return
    actual_files = {str(path.relative_to(output)) for path in output.rglob('*') if path.is_file()}
    if actual_files <= {'README.md'}:
        return
    manifest_path = output / 'provenance.json'
    if not manifest_path.is_file():
        raise ValueError(f'기존 출력에 provenance.json이 없어 안전하게 갱신할 수 없습니다: {output}')
    manifest = json.loads(manifest_path.read_text(encoding='utf-8'))
    generated_hashes = manifest.get('generated_sha256')
    if not isinstance(generated_hashes, dict):
        raise ValueError(f'출력 파일 해시가 없어 안전하게 갱신할 수 없습니다: {manifest_path}')
    expected_files = set(generated_hashes) | {'README.md', 'provenance.json', '.building'}
    if actual_files - expected_files:
        raise ValueError(f'생성기 관리 밖의 파일이 있어 갱신을 중단합니다: {sorted(actual_files - expected_files)[:5]}')
    for relative, expected in generated_hashes.items():
        path = output / relative
        if not path.is_file() or sha256(path.read_bytes()).hexdigest() != expected:
            raise ValueError(f'생성된 출력이 수정되어 갱신을 중단합니다: {relative}')
    (output / '.building').unlink(missing_ok=True)


def convert_component(component_name, docs_root, output_root, license_root, version):
    folder, title, source_url = COMPONENTS[component_name]
    component = docs_root / component_name
    output = output_root / folder
    staging = output_root / f'.{folder}.staging'
    backup = output_root / f'.{folder}.previous'
    if not component.is_dir():
        raise ValueError(f'Rust 문서 구성요소가 없습니다: {component}')
    prepare_output(output)
    if backup.exists():
        if not output.exists():
            os.replace(backup, output)
        else:
            shutil.rmtree(backup)
    if staging.exists():
        if not (staging / '.building').is_file():
            raise ValueError(f'임시 출력에 생성기 표시가 없어 갱신을 중단합니다: {staging}')
        shutil.rmtree(staging)
    staging.mkdir(parents=True, exist_ok=True)
    (staging / '.building').touch()
    if (output / 'README.md').is_file():
        shutil.copyfile(output / 'README.md', staging / 'README.md')
    output = staging

    ordered = toc_pages(component)
    pages = referenced_pages(component, ordered)
    ordered_set = set(ordered)
    extras = sorted(set(pages) - ordered_set)
    pages = ordered + extras
    page_outputs = {
        source_page: str(PurePosixPath('chapters') / PurePosixPath(source_page).with_suffix('.md'))
        for source_page in pages
    }
    source_hashes = {}
    chapter_metadata = []
    for source_page in pages:
        main = page_main(component, source_page)
        text_parser = MainTextParser()
        text_parser.feed(main)
        chapter_title = ' '.join(''.join(text_parser.heading_parts).split()) or PurePosixPath(source_page).stem.replace('-', ' ').title()
        summary = ' '.join(''.join(text_parser.paragraph_parts).split())[:320]
        markdown_page = page_outputs[source_page]

        def rewrite(match):
            attribute = match.group(1).lower()
            quote = match.group(2)[0]
            value = match.group(3) if match.group(3) is not None else match.group(4)
            if attribute == 'src' or Path(urlsplit(value).path).suffix.lower() in IMAGE_SUFFIXES:
                asset = normalize_asset(component, output, source_page, markdown_page, value, source_hashes)
                if asset:
                    return f'{attribute}={quote}{asset}{quote}'
            target = local_target(component, source_page, value)
            if not target:
                parts = urlsplit(value)
                if not parts.scheme and not parts.netloc and parts.path.endswith('.html'):
                    cross_component = posixpath.normpath(posixpath.join(component_name, posixpath.dirname(source_page), unquote(parts.path)))
                    if (docs_root / cross_component).is_file():
                        version_number = version.split()[0]
                        url = f'https://doc.rust-lang.org/{version_number}/{cross_component}'
                        if parts.fragment:
                            url += '#' + parts.fragment
                        return f'{attribute}={quote}{url}{quote}'
                return match.group(0)
            target_page, fragment = target
            markdown_target = page_outputs.get(target_page)
            if not markdown_target:
                return match.group(0)
            path = '' if target_page == source_page else posixpath.relpath(markdown_target, start=posixpath.dirname(markdown_page))
            rewritten = path + ('#' + fragment if fragment else '')
            return f'{attribute}={quote}{rewritten}{quote}'

        main = ATTRIBUTE_RE.sub(rewrite, main)
        inline_index = 0

        def save_inline_svg(match):
            nonlocal inline_index
            inline_index += 1
            asset_path = PurePosixPath('assets/inline') / (PurePosixPath(source_page).stem + f'-{inline_index}.svg')
            destination = output / Path(*asset_path.parts)
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_text(match.group(0), encoding='utf-8')
            source_hashes[str(asset_path)] = sha256(match.group(0).encode()).hexdigest()
            image_href = posixpath.relpath(str(asset_path), start=posixpath.dirname(markdown_page))
            return f'<img alt="" src="{image_href}">'

        main = SVG_RE.sub(save_inline_svg, main)
        main = HEADER_LINK_RE.sub(r'\1', main)
        main = LAYOUT_TAG_RE.sub('', main)
        if shutil.which('pandoc') is None:
            raise RuntimeError('Pandoc이 필요합니다. PATH에서 pandoc 명령을 찾지 못했습니다.')
        result = subprocess.run(
            ['pandoc', '--from=html', '--to=gfm', '--wrap=none'],
            input=main,
            text=True,
            capture_output=True,
            check=True,
        )
        markdown = result.stdout.strip()
        markdown = re.sub(r'^#\s+[^\n]+\n+', '', markdown, count=1)
        markdown = redact_documentation_secrets(normalize_markdown(markdown))
        destination = output / Path(*PurePosixPath(markdown_page).parts)
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_text(f'# {chapter_title}\n\n{markdown}\n', encoding='utf-8')
        source_file = component / source_page
        source_hashes[source_page] = sha256(source_file.read_bytes()).hexdigest()
        chapter_metadata.append({'path': markdown_page, 'title': chapter_title, 'summary': summary})

    for name, source_name in [('LICENSE-MIT', 'MIT.txt'), ('LICENSE-APACHE', 'Apache-2.0.txt')]:
        source_file = license_root / source_name
        if not source_file.is_file():
            raise ValueError(f'Rust 문서 라이선스를 찾지 못했습니다: {source_file}')
        shutil.copyfile(source_file, output / name)
    (output / 'COPYRIGHT').write_text(
        'Documentation from the Rust Project. Copyright remains with the Rust Project Contributors. '
        'Unless a page says otherwise, this material is available under either the MIT License or '
        'Apache License, Version 2.0. See LICENSE-MIT and LICENSE-APACHE.\n',
        encoding='utf-8',
    )
    (output / 'index.json').write_text(json.dumps({
        'source': title,
        'source_url': source_url,
        'toolchain': version,
        'chapters': chapter_metadata,
    }, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    generated_hashes = {
        str(path.relative_to(output)): sha256(path.read_bytes()).hexdigest()
        for path in sorted(output.rglob('*'))
        if path.is_file() and path.name not in {'README.md', 'provenance.json', '.building'}
    }
    (output / 'provenance.json').write_text(json.dumps({
        'source': source_url,
        'toolchain': version,
        'sha256': source_hashes,
        'generated_sha256': generated_hashes,
    }, ensure_ascii=False, indent=2, sort_keys=True) + '\n', encoding='utf-8')
    final_output = output_root / folder
    backup = output_root / f'.{folder}.previous'
    if final_output.exists():
        os.replace(final_output, backup)
    try:
        os.replace(output, final_output)
    except Exception:
        if backup.exists():
            os.replace(backup, final_output)
        raise
    if backup.exists():
        shutil.rmtree(backup)
    (final_output / '.building').unlink(missing_ok=True)
    print(f'{folder}: {len(chapter_metadata)}장 변환, 입력 {len(source_hashes)}개 해시 기록')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--rust-docs-root', type=Path, required=True, help='rust-docs HTML의 상위 디렉터리')
    parser.add_argument('--license-root', type=Path, required=True, help='Rust 설치본 share/doc/rust/licenses 경로')
    parser.add_argument('--output-root', type=Path, required=True, help='검색 자료 출력 디렉터리')
    parser.add_argument('--version', required=True, help='원문 Rust toolchain 버전')
    args = parser.parse_args()
    for component in COMPONENTS:
        convert_component(component, args.rust_docs_root, args.output_root, args.license_root, args.version)


if __name__ == '__main__':
    main()
