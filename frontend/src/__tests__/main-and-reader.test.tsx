import { fireEvent, render, screen, waitFor } from '@testing-library/react'
import { afterEach, describe, expect, it, vi } from 'vitest'
import * as api from '../api'
import * as documents from '../documents'
import { DocumentPage } from '../document-reader'
import { HomePage, SearchForm, SearchPage } from '../main'

afterEach(() => {
  vi.restoreAllMocks()
})

describe('HomePage, SearchForm, and SearchPage', () => {
  it('renders HomePage with recommended chips and handles SearchForm autocomplete + keyboard nav', async () => {
    vi.spyOn(api, 'fetchRecommendations').mockResolvedValue([
      { term: 'kordoc', category: 'code', documentCount: 266 },
      { term: 'Tantivy', category: 'guide', documentCount: 12 },
    ])

    render(<HomePage />)
    expect(screen.getByRole('heading', { name: /dok3search/ })).toBeInTheDocument()
    await waitFor(() => {
      expect(screen.getByRole('link', { name: 'kordoc' })).toBeInTheDocument()
    })

    const input = screen.getByRole('searchbox', { name: '문서 검색어' })
    fireEvent.focus(input)
    await waitFor(() => {
      expect(screen.getByRole('listbox', { name: '추천 검색어' })).toBeInTheDocument()
    })

    fireEvent.keyDown(input, { key: 'ArrowDown' })
    expect(screen.getAllByRole('option')[0]).toHaveAttribute('aria-selected', 'true')
    fireEvent.keyDown(input, { key: 'Escape' })
    expect(screen.queryByRole('listbox')).not.toBeInTheDocument()

    expect(screen.queryByRole('button', { name: '검색어 지우기' })).not.toBeInTheDocument()
    fireEvent.change(input, { target: { value: 'kordoc' } })
    const clearButton = screen.getByRole('button', { name: '검색어 지우기' })
    expect(clearButton).toBeInTheDocument()
    fireEvent.click(clearButton)
    expect(input).toHaveValue('')
    expect(screen.queryByRole('button', { name: '검색어 지우기' })).not.toBeInTheDocument()
  })

  it('renders SearchPage hits with symbol badge, code snippet, and empty state', async () => {
    window.history.replaceState({}, '', '/search?q=toArrayBuffer&category=code')
    vi.spyOn(api, 'fetchRecommendations').mockResolvedValue([
      { term: 'kordoc', category: 'code', documentCount: 266 },
    ])
    vi.spyOn(api, 'fetchSearch').mockResolvedValue({
      mode: 'api',
      result: {
        query: 'toArrayBuffer',
        page: 1,
        limit: 15,
        totalHits: 1,
        totalPages: 1,
        totalUnfilteredHits: 1,
        categoryCounts: [{ id: 'code', label: '코드 레퍼런스', count: 1 }],
        topicCounts: [],
        relatedSearches: ['typescript'],
        hits: [
          {
            documentId: 'local-markdown:samples/kordoc/src/utils.ts',
            title: 'kordoc — src/utils.ts',
            summary: 'utils',
            snippet: 'export function toArrayBuffer(buf: Buffer)',
            category: 'code',
            source: 'kordoc',
            tag: 'typescript',
            samplePath: 'samples/kordoc/src/utils.ts',
            hasPairedFormats: false,
            matchedSectionId: 'function-toarraybuffer',
            matchedSectionHeading: 'function toArrayBuffer',
            matchedLineStart: 9,
          },
        ],
        snapshot: { generationId: 'gen-1', epoch: 1 },
      },
    })

    render(<SearchPage />)
    await waitFor(() => {
      expect(screen.getByText('kordoc — src/utils.ts')).toBeInTheDocument()
    })
    expect(document.querySelector('.hit-symbol')?.textContent).toBe('function toArrayBuffer · L9')
    expect(document.querySelector('.snippet-code')).not.toBeNull()
    const marks = [...document.querySelectorAll('mark.search-hl')].map((el) => el.textContent)
    expect(marks).toEqual(['toArrayBuffer', 'toArrayBuffer'])
    expect(screen.getByRole('link', { name: /kordoc — src\/utils\.ts/ })).toHaveAttribute(
      'href',
      expect.stringContaining('#function-toarraybuffer'),
    )
  })

  it('loads guide search via API without importing local sample chunks on category=guide and topic=rust', async () => {
    window.history.replaceState({}, '', '/search?category=guide&topic=rust')
    vi.spyOn(api, 'fetchRecommendations').mockResolvedValue([])
    const loadLocalSpy = vi.spyOn(documents, 'loadLocalSamples')
    const fetchSearchSpy = vi.spyOn(api, 'fetchSearch').mockResolvedValue({
      mode: 'api',
      result: {
        query: '',
        page: 1,
        limit: 15,
        totalHits: 1,
        totalPages: 1,
        totalUnfilteredHits: 2,
        categoryCounts: [{ id: 'guide', label: '개발 가이드', count: 2 }],
        topicCounts: [
          { id: 'rust', label: 'Rust', count: 1 },
          { id: 'go', label: 'Go', count: 1 },
        ],
        relatedSearches: ['rust'],
        hits: [
          {
            documentId: 'local-markdown:samples/rust-book/ch01.md',
            title: 'Getting Started',
            summary: 'rust book',
            snippet: 'Rust intro',
            category: 'guide',
            topic: 'rust',
            source: 'The Rust Programming Language',
            tag: 'rust',
            hasPairedFormats: false,
          },
        ],
        snapshot: { generationId: 'gen-1', epoch: 1 },
      },
    })

    render(<SearchPage />)
    await waitFor(() => {
      expect(screen.getByText('Getting Started')).toBeInTheDocument()
    })
    expect(fetchSearchSpy).toHaveBeenCalledTimes(1)
    expect(fetchSearchSpy).toHaveBeenCalledWith(
      expect.objectContaining({ query: '', category: 'guide', topic: 'rust', page: 1, limit: 15 }),
      expect.any(AbortSignal),
    )
    expect(loadLocalSpy).not.toHaveBeenCalled()
  })

  it('does not load local sample chunks when general search API returns zero hits', async () => {
    window.history.replaceState({}, '', '/search?q=nonexistent-term-xyz')
    vi.spyOn(api, 'fetchRecommendations').mockResolvedValue([])
    const loadLocalSpy = vi.spyOn(documents, 'loadLocalSamples')
    vi.spyOn(api, 'fetchSearch').mockResolvedValue({
      mode: 'api',
      result: {
        query: 'nonexistent-term-xyz',
        page: 1,
        limit: 15,
        totalHits: 0,
        totalPages: 0,
        totalUnfilteredHits: 0,
        categoryCounts: [{ id: 'guide', label: '개발 가이드', count: 0 }],
        topicCounts: [],
        relatedSearches: [],
        hits: [],
        snapshot: { generationId: 'gen-1', epoch: 1 },
      },
    })

    render(<SearchPage />)
    await waitFor(() => {
      expect(screen.getByText(/검색 조건에 맞는 문서가 없습니다/)).toBeInTheDocument()
    })
    expect(loadLocalSpy).not.toHaveBeenCalled()
  })
})

describe('DocumentPage', () => {
  it('renders source code document with CodeDocumentViewer and symbol TOC', async () => {
    window.history.replaceState({}, '', '/docs/local-markdown%3Asamples%2Fkordoc%2Fsrc%2Futils.ts?q=kordoc&category=code')
    vi.spyOn(api, 'fetchDocument').mockResolvedValue({
      mode: 'api',
      doc: {
        id: 'local-markdown:samples/kordoc/src/utils.ts',
        title: 'kordoc — src/utils.ts',
        summary: 'kordoc utils',
        source: 'kordoc',
        tag: 'code',
        tags: ['typescript'],
        keywords: ['toArrayBuffer'],
        category: 'code',
        samplePath: 'samples/kordoc/src/utils.ts',
        body: 'export function toArrayBuffer() {\n  return 1\n}',
        sections: [
          {
            sectionId: 'function-toarraybuffer',
            heading: 'function toArrayBuffer',
            anchor: 'function-toarraybuffer',
            lineStart: 1,
            lineEnd: 3,
          },
        ],
      },
    })
    vi.spyOn(api, 'fetchClassification').mockResolvedValue({
      documentId: 'local-markdown:samples/kordoc/src/utils.ts',
      summary: 'kordoc utils',
      category: 'code',
      tags: ['typescript'],
      keywords: ['toArrayBuffer'],
      source: 'rule',
      manualLocked: false,
      updatedAtMs: 100,
      contentHash: 'abc',
    })

    render(<DocumentPage id="local-markdown:samples/kordoc/src/utils.ts" />)
    await waitFor(() => {
      expect(screen.getByRole('heading', { name: 'kordoc — src/utils.ts' })).toBeInTheDocument()
    })
    expect(screen.getByRole('navigation', { name: '심볼 목차' })).toBeInTheDocument()
    expect(screen.getByText('L1')).toBeInTheDocument()

    fireEvent.click(screen.getByRole('button', { name: '분류 편집' }))
    expect(screen.getByRole('region', { name: '문서 분류 편집' })).toBeInTheDocument()
  })

  it('renders prose markdown with airgap external-link badge', async () => {
    vi.spyOn(api, 'fetchDocument').mockResolvedValue({
      mode: 'api',
      doc: {
        id: 'guide-1',
        title: '가이드 문서',
        summary: '요약입니다',
        source: 'docs',
        tag: 'guide',
        category: 'guide',
        body: '본문',
        markdown: '## 개요\n\n[외부 참고](https://example.com)',
      },
    })
    vi.spyOn(api, 'fetchClassification').mockResolvedValue(null)

    render(<DocumentPage id="guide-1" />)
    await waitFor(() => {
      expect(screen.getByRole('heading', { name: '가이드 문서' })).toBeInTheDocument()
    })
    expect(screen.getByText('외부 참고')).toBeInTheDocument()
    expect(screen.getByText('외부')).toBeInTheDocument()
  })
})
