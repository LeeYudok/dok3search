import { describe, expect, it, vi } from 'vitest'
import {
  analyzeSearchText,
  ApiRequestError,
  createSource,
  deleteSource,
  fetchClassification,
  fetchDocument,
  fetchIndexJobs,
  fetchRecommendations,
  fetchSearch,
  fetchSources,
  isSourceCodePath,
  triggerIndexJob,
  updateClassification,
} from '../api'

function jsonResponse(payload: unknown, status = 200) {
  return new Response(JSON.stringify(payload), {
    status,
    headers: { 'content-type': 'application/json' },
  })
}

describe('api client', () => {
  it('detects source code paths', () => {
    expect(isSourceCodePath('samples/kordoc/src/utils.ts')).toBe(true)
    expect(isSourceCodePath('src/App.tsx')).toBe(true)
    expect(isSourceCodePath('docs/manual.md')).toBe(false)
    expect(isSourceCodePath(undefined)).toBe(false)
  })

  it('fetches search results from API and falls back on 503 or TypeError', async () => {
    const fetchMock = vi.fn().mockResolvedValueOnce(
      jsonResponse({
        query: 'toArrayBuffer',
        page: 1,
        limit: 15,
        total_hits: 1,
        total_pages: 1,
        total_unfiltered_hits: 1,
        category_counts: [{ id: 'code', label: '코드 레퍼런스', count: 1 }],
        related_searches: ['kordoc'],
        hits: [
          {
            document_id: 'local-markdown:samples/kordoc/src/utils.ts',
            title: 'kordoc — src/utils.ts',
            summary: 'utils',
            snippet: 'export function toArrayBuffer',
            category: 'code',
            source: 'kordoc',
            tag: 'typescript',
            sample_path: 'samples/kordoc/src/utils.ts',
            has_paired_formats: false,
            matched_section_id: 'function-toarraybuffer',
            matched_section_heading: 'function toArrayBuffer',
            matched_line_start: 9,
          },
        ],
        snapshot: { generation_id: 'gen-1', epoch: 1 },
      }),
    )
    vi.stubGlobal('fetch', fetchMock)

    const ok = await fetchSearch(
      { query: 'toArrayBuffer', category: 'code', page: 1, limit: 15 },
      new AbortController().signal,
    )
    expect(ok.mode).toBe('api')
    expect(ok.result.hits[0].matchedSectionHeading).toBe('function toArrayBuffer')
    expect(ok.result.hits[0].matchedLineStart).toBe(9)

    fetchMock.mockResolvedValueOnce(new Response('bad gateway', { status: 503 }))
    const fallback503 = await fetchSearch(
      { query: 'Tantivy', category: '', page: 1, limit: 15 },
      new AbortController().signal,
    )
    expect(fallback503.mode).toBe('local-fallback')

    fetchMock.mockRejectedValueOnce(new TypeError('offline'))
    const fallbackNet = await fetchSearch(
      { query: 'Tantivy', category: '', page: 1, limit: 15 },
      new AbortController().signal,
    )
    expect(fallbackNet.mode).toBe('local-fallback')
  })

  it('fetches recommendations and falls back on non-OK', async () => {
    const fetchMock = vi.fn().mockResolvedValueOnce(
      jsonResponse({
        recommendations: [{ term: 'kordoc', category: 'code', document_count: 266 }],
      }),
    )
    vi.stubGlobal('fetch', fetchMock)

    const rec = await fetchRecommendations({ query: 'kor', limit: 5 }, new AbortController().signal)
    expect(rec).toEqual([{ term: 'kordoc', category: 'code', documentCount: 266 }])

    fetchMock.mockResolvedValueOnce(new Response('err', { status: 500 }))
    const fb = await fetchRecommendations({ query: 'tan', limit: 5 }, new AbortController().signal)
    expect(fb.some((r) => r.term === 'Tantivy')).toBe(true)
  })

  it('fetches documents, classifications, sources, jobs, and analyzer tokens', async () => {
    const fetchMock = vi
      .fn()
      .mockResolvedValueOnce(
        jsonResponse({
          id: 'doc-1',
          title: 'utils.ts',
          summary: 'sum',
          source: 'kordoc',
          tag: 'code',
          category: 'code',
          body: 'export const X = 1',
          sections: [
            { section_id: 'const-x', heading: 'const X', anchor: 'const-x', line_start: 1, line_end: 1 },
          ],
        }),
      )
      .mockResolvedValueOnce(
        jsonResponse({
          document_id: 'doc-1',
          summary: 'sum',
          category: 'code',
          tags: ['ts'],
          keywords: ['X'],
          source: 'rule',
          manual_locked: false,
          updated_at_ms: 1000,
          content_hash: 'abc',
        }),
      )
      .mockResolvedValueOnce(
        jsonResponse({ error: { code: 'FORBIDDEN', message: 'admin required' } }, 403),
      )
      .mockResolvedValueOnce(jsonResponse({ sources: [] }))
      .mockResolvedValueOnce(
        jsonResponse({
          source_id: 'my-src',
          label: 'My',
          path: '/tmp/my',
          kind: 'local_dir',
          default_category: 'guide',
          default_tag: 'docs',
          managed_by_env: false,
          created_at_ms: 1,
          document_count: 0,
        }),
      )
      .mockResolvedValueOnce(
        jsonResponse({ source_id: 'my-src', deleted_documents: 0, rebuilt: false, active_epoch: 1 }),
      )
      .mockResolvedValueOnce(
        jsonResponse({ jobs: [], active_epoch: 1 }),
      )
      .mockResolvedValueOnce(
        jsonResponse({
          job_id: 'job-2',
          trigger: 'manual',
          mode: 'full',
          state: 'succeeded',
          started_at_ms: 2,
          indexed_documents: 10,
          rebuilt: true,
          active_epoch: 2,
          skipped_files: [],
        }),
      )
      .mockResolvedValueOnce(
        jsonResponse({
          profile: 'builtin',
          kodic_loaded: false,
          analyzer_version: 1,
          normalized_query_terms: ['rust'],
          terms: ['rust'],
          char_ngrams: ['ru', 'us', 'st'],
        }),
      )

    vi.stubGlobal('fetch', fetchMock)

    const docRes = await fetchDocument('doc-1', new AbortController().signal)
    expect(docRes.doc?.sections?.[0].lineStart).toBe(1)

    const cls = await fetchClassification('doc-1')
    expect(cls?.source).toBe('rule')

    await expect(updateClassification('doc-1', { resetToRule: true })).rejects.toBeInstanceOf(ApiRequestError)

    expect((await fetchSources()).sources).toEqual([])
    expect((await createSource({ sourceId: 'my-src', label: 'My', path: '/tmp/my' })).sourceId).toBe('my-src')
    expect((await deleteSource('my-src')).sourceId).toBe('my-src')
    expect((await fetchIndexJobs()).activeEpoch).toBe(1)
    expect((await triggerIndexJob({ mode: 'full' })).jobId).toBe('job-2')
    expect((await analyzeSearchText({ text: 'rust', splitIdentifiers: true, charNgrams: true })).terms).toEqual(['rust'])
  })
})
