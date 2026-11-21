import { categories, documentFormats, documents, searchDocuments } from './documents'

export type DataSourceMode = 'api' | 'local-fallback'

export type SearchHitItem = {
  documentId: string
  title: string
  summary: string
  snippet: string
  category: string
  source: string
  tag: string
  date?: string
  samplePath?: string
  hasPairedFormats: boolean
}

export type CategoryCountItem = {
  id: string
  label: string
  count: number
}

export type SnapshotInfo = {
  generationId: string
  epoch: number
}

export type SearchViewResult = {
  query: string
  page: number
  limit: number
  totalHits: number
  totalPages: number
  totalUnfilteredHits: number
  categoryCounts: CategoryCountItem[]
  relatedSearches: string[]
  hits: SearchHitItem[]
  snapshot: SnapshotInfo
}

export type DocumentFormatPair = {
  markdownId: string
  yamlId: string
}

export type ReaderDocument = {
  id: string
  title: string
  summary: string
  source: string
  tag: string
  date?: string
  category: string
  body: string
  markdown?: string
  samplePath?: string
  formats?: DocumentFormatPair
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value)
}

function expectString(record: Record<string, unknown>, key: string): string {
  const value = record[key]
  if (typeof value !== 'string') {
    throw new Error(`Invalid API field '${key}': expected string`)
  }
  return value
}

function optionalString(record: Record<string, unknown>, key: string): string | undefined {
  const value = record[key]
  if (value === undefined || value === null) return undefined
  if (typeof value !== 'string') {
    throw new Error(`Invalid API field '${key}': expected string or null`)
  }
  return value
}

function expectNumber(record: Record<string, unknown>, key: string): number {
  const value = record[key]
  if (typeof value !== 'number' || !Number.isFinite(value)) {
    throw new Error(`Invalid API field '${key}': expected finite number`)
  }
  return value
}

function expectBoolean(record: Record<string, unknown>, key: string): boolean {
  const value = record[key]
  if (typeof value !== 'boolean') {
    throw new Error(`Invalid API field '${key}': expected boolean`)
  }
  return value
}

function expectStringArray(record: Record<string, unknown>, key: string): string[] {
  const value = record[key]
  if (!Array.isArray(value) || !value.every((item): item is string => typeof item === 'string')) {
    throw new Error(`Invalid API field '${key}': expected string array`)
  }
  return value
}

function parseSearchHit(raw: unknown): SearchHitItem {
  if (!isRecord(raw)) throw new Error('Invalid search hit entry')
  return {
    documentId: expectString(raw, 'document_id'),
    title: expectString(raw, 'title'),
    summary: expectString(raw, 'summary'),
    snippet: expectString(raw, 'snippet'),
    category: expectString(raw, 'category'),
    source: expectString(raw, 'source'),
    tag: expectString(raw, 'tag'),
    date: optionalString(raw, 'date'),
    samplePath: optionalString(raw, 'sample_path'),
    hasPairedFormats: expectBoolean(raw, 'has_paired_formats'),
  }
}

function parseCategoryCount(raw: unknown): CategoryCountItem {
  if (!isRecord(raw)) throw new Error('Invalid category_counts entry')
  return {
    id: expectString(raw, 'id'),
    label: expectString(raw, 'label'),
    count: expectNumber(raw, 'count'),
  }
}

function parseSearchResponse(raw: unknown): SearchViewResult {
  if (!isRecord(raw)) throw new Error('Invalid search response payload')
  const rawCounts = raw.category_counts
  const rawHits = raw.hits
  const rawSnapshot = raw.snapshot
  if (!Array.isArray(rawCounts) || !Array.isArray(rawHits) || !isRecord(rawSnapshot)) {
    throw new Error('Invalid search response structure')
  }

  return {
    query: expectString(raw, 'query'),
    page: expectNumber(raw, 'page'),
    limit: expectNumber(raw, 'limit'),
    totalHits: expectNumber(raw, 'total_hits'),
    totalPages: expectNumber(raw, 'total_pages'),
    totalUnfilteredHits: expectNumber(raw, 'total_unfiltered_hits'),
    categoryCounts: rawCounts.map(parseCategoryCount),
    relatedSearches: expectStringArray(raw, 'related_searches'),
    hits: rawHits.map(parseSearchHit),
    snapshot: {
      generationId: expectString(rawSnapshot, 'generation_id'),
      epoch: expectNumber(rawSnapshot, 'epoch'),
    },
  }
}

function parseDocumentDetail(raw: unknown): ReaderDocument {
  if (!isRecord(raw)) throw new Error('Invalid document response payload')
  let formats: DocumentFormatPair | undefined
  if (raw.formats !== undefined && raw.formats !== null) {
    if (!isRecord(raw.formats)) throw new Error('Invalid document formats field')
    formats = {
      markdownId: expectString(raw.formats, 'markdown_id'),
      yamlId: expectString(raw.formats, 'yaml_id'),
    }
  }

  return {
    id: expectString(raw, 'id'),
    title: expectString(raw, 'title'),
    summary: expectString(raw, 'summary'),
    source: expectString(raw, 'source'),
    tag: expectString(raw, 'tag'),
    date: optionalString(raw, 'date'),
    category: expectString(raw, 'category'),
    body: expectString(raw, 'body'),
    markdown: optionalString(raw, 'markdown'),
    samplePath: optionalString(raw, 'sample_path'),
    formats,
  }
}

function parseApiErrorMessage(raw: unknown, fallback: string): string {
  if (!isRecord(raw) || !isRecord(raw.error)) return fallback
  return typeof raw.error.message === 'string' ? raw.error.message : fallback
}

function isProxyOrNetworkUnavailableStatus(status: number): boolean {
  return status === 502 || status === 503 || status === 504
}

function localFallbackSearch(query: string, category: string, requestedPage: number, limit: number): SearchViewResult {
  const matches = searchDocuments(query)
  const visible = category ? matches.filter((doc) => doc.category === category) : matches
  const pageCount = Math.ceil(visible.length / limit)
  const page = pageCount > 0 && Number.isSafeInteger(requestedPage)
    ? Math.max(1, Math.min(requestedPage, pageCount))
    : 1
  const pageDocuments = visible.slice((page - 1) * limit, page * limit)

  const matchedCategories = new Set(visible.map((doc) => doc.category))
  const relatedSearches = query && visible.length
    ? [...new Set(documents.filter((doc) => matchedCategories.has(doc.category)).map((doc) => doc.tag))]
        .filter((tag) => !query.toLowerCase().includes(tag.toLowerCase()))
        .slice(0, 6)
    : []

  return {
    query,
    page,
    limit,
    totalHits: visible.length,
    totalPages: pageCount,
    totalUnfilteredHits: matches.length,
    categoryCounts: categories.map((item) => ({
      id: item.id,
      label: item.label,
      count: matches.filter((doc) => doc.category === item.id).length,
    })),
    relatedSearches,
    hits: pageDocuments.map((doc) => ({
      documentId: doc.id,
      title: doc.title,
      summary: doc.summary,
      snippet: (doc.summary + ' ' + doc.body).trim(),
      category: doc.category,
      source: doc.source,
      tag: doc.tag,
      date: doc.date,
      samplePath: doc.samplePath,
      hasPairedFormats: Boolean(documentFormats(doc.id)),
    })),
    snapshot: {
      generationId: 'local-samples',
      epoch: 0,
    },
  }
}

function localFallbackDocument(id: string): ReaderDocument | null {
  const doc = documents.find((item) => item.id === id)
  if (!doc) return null
  const formats = documentFormats(id)
  return {
    id: doc.id,
    title: doc.title,
    summary: doc.summary,
    source: doc.source,
    tag: doc.tag,
    date: doc.date,
    category: doc.category,
    body: doc.body,
    markdown: doc.markdown,
    samplePath: doc.samplePath,
    formats: formats
      ? { markdownId: formats.markdown.id, yamlId: formats.yaml.id }
      : undefined,
  }
}

export async function fetchSearch(
  params: { query: string; category: string; page: number; limit: number },
  signal: AbortSignal,
): Promise<{ result: SearchViewResult; mode: DataSourceMode }> {
  try {
    const response = await fetch('/api/v1/search', {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify({
        query: params.query,
        filters: params.category ? { category: params.category } : {},
        page: params.page,
        limit: params.limit,
        include_snippets: true,
      }),
      signal,
    })

    if (isProxyOrNetworkUnavailableStatus(response.status)) {
      return {
        result: localFallbackSearch(params.query, params.category, params.page, params.limit),
        mode: 'local-fallback',
      }
    }

    const contentType = response.headers.get('content-type') ?? ''
    if (!contentType.includes('application/json')) {
      return {
        result: localFallbackSearch(params.query, params.category, params.page, params.limit),
        mode: 'local-fallback',
      }
    }

    const json: unknown = await response.json()
    if (!response.ok) {
      throw new Error(parseApiErrorMessage(json, `검색 요청에 실패했습니다 (${response.status}).`))
    }

    return {
      result: parseSearchResponse(json),
      mode: 'api',
    }
  } catch (err) {
    if (err instanceof DOMException && err.name === 'AbortError') {
      throw err
    }
    if (err instanceof TypeError) {
      return {
        result: localFallbackSearch(params.query, params.category, params.page, params.limit),
        mode: 'local-fallback',
      }
    }
    throw err
  }
}

export async function fetchDocument(
  id: string,
  signal: AbortSignal,
): Promise<{ doc: ReaderDocument | null; mode: DataSourceMode }> {
  try {
    const response = await fetch('/api/v1/documents/' + encodeURIComponent(id), {
      signal,
    })

    if (response.status === 404) {
      const contentType = response.headers.get('content-type') ?? ''
      if (contentType.includes('application/json')) {
        return { doc: null, mode: 'api' }
      }
      return { doc: localFallbackDocument(id), mode: 'local-fallback' }
    }

    if (isProxyOrNetworkUnavailableStatus(response.status)) {
      return { doc: localFallbackDocument(id), mode: 'local-fallback' }
    }

    const contentType = response.headers.get('content-type') ?? ''
    if (!contentType.includes('application/json')) {
      return { doc: localFallbackDocument(id), mode: 'local-fallback' }
    }

    const json: unknown = await response.json()
    if (!response.ok) {
      throw new Error(parseApiErrorMessage(json, `문서 조회에 실패했습니다 (${response.status}).`))
    }

    return {
      doc: parseDocumentDetail(json),
      mode: 'api',
    }
  } catch (err) {
    if (err instanceof DOMException && err.name === 'AbortError') {
      throw err
    }
    if (err instanceof TypeError) {
      return { doc: localFallbackDocument(id), mode: 'local-fallback' }
    }
    throw err
  }
}
