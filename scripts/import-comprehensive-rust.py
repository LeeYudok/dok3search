#!/usr/bin/env python3
"""Convert the Comprehensive Rust Korean mdBook print and contents pages to Markdown."""

import argparse
from collections import Counter
from hashlib import sha256
import json
import posixpath
from pathlib import Path, PurePosixPath
import re
import shutil
import subprocess
import tomllib
from urllib.parse import unquote, urljoin, urlsplit

from bs4 import BeautifulSoup, Comment, Tag


COURSE_URL = 'https://google.github.io/comprehensive-rust/ko/'
CHAPTER_BREAK = 'break-before: page; page-break-before: always;'
CHAPTER_PATH = re.compile(r'[a-z0-9_-]+(?:/[a-z0-9_-]+)*\.html')
# The deployed Korean book preserves these older chapter paths.
PRINT_LINKS = {
    'async/concurrency/channels.html': 'concurrency/channels.html',
    'async/pitfall/async-traits.html': 'async/pitfalls/async-traits.html',
    'basic-syntax/string-slices.html': 'types-and-values/strings.html',
}


def read_pages(source):
    contents = BeautifulSoup((source / 'toc.html').read_bytes(), 'lxml')
    chapter_paths = [link['href'] for link in contents.select('ol.chapter a[href]')]
    if len(chapter_paths) != len(set(chapter_paths)) or not all(
        CHAPTER_PATH.fullmatch(path) for path in chapter_paths
    ):
        raise ValueError('목차에 중복 또는 잘못된 장 경로가 있습니다.')

    print_book = BeautifulSoup((source / 'print.html').read_bytes(), 'lxml')
    fragments = [[]]
    for node in print_book.main.children:
        if isinstance(node, Tag) and node.get('style') == CHAPTER_BREAK:
            fragments.append([])
        elif not isinstance(node, Comment):
            fragments[-1].append(str(node))

    if len(chapter_paths) != len(fragments) or not chapter_paths:
        raise ValueError(f'인쇄본({len(fragments)})과 목차({len(chapter_paths)})의 장 수가 다릅니다.')
    return {
        path: BeautifulSoup(''.join(fragment), 'lxml')
        for path, fragment in zip(chapter_paths, fragments)
    }


def relative_path(source, target):
    return posixpath.relpath(target, start=posixpath.dirname(source) or '.')


def chapter_link(source, target, anchor=''):
    if source == target and anchor:
        return '#' + anchor
    return relative_path(source, str(PurePosixPath(target).with_suffix('.md'))) + (
        '#' + anchor if anchor else ''
    )


def convert_links(path, page, pages, anchor_paths, redirects):
    unresolved = set()
    for link in page.select('a[href]'):
        href = link['href']
        target = urlsplit(href)
        if href.startswith('#'):
            identifier = unquote(target.fragment)
            if identifier in anchor_paths:
                chapter, anchor = anchor_paths[identifier]
                link['href'] = chapter_link(path, chapter, anchor)
            else:
                unresolved.add(href)
                link['href'] = COURSE_URL + href
        elif not target.scheme and not target.netloc:
            normalized = target.path.removeprefix('./')
            if normalized not in pages and not normalized.endswith('.html'):
                normalized += '.html'
            if normalized in PRINT_LINKS:
                normalized = PRINT_LINKS[normalized]
            elif normalized not in pages and normalized in redirects:
                normalized = posixpath.normpath(
                    posixpath.join(posixpath.dirname(normalized), redirects[normalized])
                )
            if normalized in pages:
                link['href'] = chapter_link(path, normalized, target.fragment)
            else:
                unresolved.add(href)
                link['href'] = urljoin(COURSE_URL, href)
    return unresolved


def normalize_page(path, page, output, assets):
    for selector in ('a.header', 'style', 'script'):
        for tag in page.select(selector):
            if selector == 'a.header':
                tag.unwrap()
            elif tag.find_parent('svg') is None:
                tag.decompose()
    for note in page.select('details'):
        summary = note.find('summary', recursive=False)
        if summary:
            summary.name = 'strong'
        else:
            summary = page.new_tag('strong')
            summary.string = '발표자 노트'
            note.insert(0, summary)
        label = page.new_tag('p')
        summary.wrap(label)
        note.name = 'blockquote'
        note.attrs = {}
    for index, svg in enumerate(page.select('svg.svgbob'), 1):
        name = str(PurePosixPath(path).with_suffix('')) + f'-diagram-{index}.svg'
        target = output / 'assets' / name
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(str(svg) + '\n', encoding='utf-8')
        tokens = list(dict.fromkeys(item.get_text(strip=True) for item in svg.select('text')))
        image = page.new_tag('img')
        image['src'] = name
        image['alt'] = ' · '.join(filter(None, tokens)) or 'Rust 강의 그림'
        svg.replace_with(image)
        assets.add(name)
    for image in page.select('img[src]'):
        name = image['src']
        if urlsplit(name).scheme or urlsplit(name).netloc:
            image.decompose()
            continue
        if name not in assets:
            raise ValueError(f'누락된 그림: {name}')
        image['src'] = relative_path('chapters/' + path, 'assets/' + name)
        if not image.get('alt'):
            image['alt'] = 'Rust 강의 그림'
    for code in page.select('pre > code'):
        pre = code.parent
        classes = code.get('class', [])
        language = next((name.removeprefix('language-') for name in classes if name.startswith('language-')), '')
        pre.attrs = {'class': language} if language else {}
        code.attrs = {}
        label = next(
            (description for name, description in (
                ('compile_fail', '컴파일 오류 예제'),
                ('should_panic', '패닉 발생 예제'),
            ) if name in classes),
            None,
        )
        if label:
            note = page.new_tag('p')
            note.string = label
            pre.insert_before(note)
    for heading in page.select('h1')[1:]:
        heading.name = 'h2'


def convert(source, output):
    pages = read_pages(source)
    anchors = {}
    for path, page in pages.items():
        for heading in page.select('h1[id], h2[id], h3[id], h4[id]'):
            anchors[heading['id']] = (path, '' if heading is page.h1 else heading['id'])

    redirects = tomllib.loads((source / 'book.toml').read_text())['output']['html']['redirect']
    assets = set()
    for asset in (source / 'assets').rglob('*'):
        if asset.is_file():
            name = asset.relative_to(source / 'assets').as_posix()
            target = output / 'assets' / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(asset, target)
            assets.add(name)

    index = []
    unmatched = Counter()
    for path, page in pages.items():
        title = page.h1.get_text(' ', strip=True)
        normalize_page(path, page, output, assets)
        unmatched.update(convert_links(path, page, pages, anchors, redirects))

        first_paragraph = next(
            (element.get_text(' ', strip=True) for element in page.body.find_all('p', recursive=False)
             if element.get_text(strip=True)),
            '',
        )
        markdown_path = str(PurePosixPath('chapters', path).with_suffix('.md'))
        index.append({'path': markdown_path, 'title': title, 'summary': first_paragraph})
        document = output / markdown_path
        document.parent.mkdir(parents=True, exist_ok=True)
        result = subprocess.run(
            ['pandoc', '--from=html', '--to=gfm-raw_html', '--strip-comments', '--wrap=none'],
            input=str(page.body), text=True, capture_output=True, check=True,
        )
        document.write_text(result.stdout, encoding='utf-8')

    index_metadata = {
        'source': COURSE_URL,
        'printSha256': sha256((source / 'print.html').read_bytes()).hexdigest(),
        'tocSha256': sha256((source / 'toc.html').read_bytes()).hexdigest(),
        'chapters': index,
    }
    (output / 'index.json').write_text(
        json.dumps(index_metadata, ensure_ascii=False, indent=2) + '\n', encoding='utf-8',
    )
    for name in ('LICENSE-APACHE', 'LICENSE-CC-BY'):
        shutil.copyfile(source / name, output / name)
    print(f'{len(index)} Markdown chapters; {len(assets)} local images; {len(unmatched)} unresolved original links')
    for link in sorted(unmatched):
        print(link)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('source', type=Path)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    convert(args.source, args.output)
