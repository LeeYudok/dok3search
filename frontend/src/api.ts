import { categories, documentFormats, documents, isReferenceTopic, isReferenceTopicFilter, searchDocuments, titleMatches, topics, type Document, type ReferenceTopic } from './documents'

export type DataSourceMode = 'api' | 'local-fallback' | 'local-reference'

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
  topic?: ReferenceTopic
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
  topicCounts: Array<{ id: ReferenceTopic; label: string; count: number }>
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
  tags?: string[]
  keywords?: string[]
  date?: string
  category: string
  body: string
  markdown?: string
  samplePath?: string
  topic?: ReferenceTopic
  formats?: DocumentFormatPair
}

export type ClassificationSource = 'rule' | 'manual'

export type DocumentClassification = {
  documentId: string
  summary: string
  category: string
  tags: string[]
  keywords: string[]
  source: ClassificationSource
  manualLocked: boolean
  updatedAtMs: number
  contentHash: string
}

export type UpdateClassificationInput = {
  summary?: string
  category?: string
  tags?: string[]
  keywords?: string[]
  resetToRule?: boolean
}

export class ApiRequestError extends Error {
  readonly status: number
  readonly code?: string

  constructor(message: string, status: number, code?: string) {
    super(message)
    this.name = 'ApiRequestError'
    this.status = status
    this.code = code
  }
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

function optionalTopic(record: Record<string, unknown>): ReferenceTopic | undefined {
  const topic = optionalString(record, 'topic')
  if (topic === undefined) return undefined
  if (!isReferenceTopicFilter(topic)) throw new Error("Invalid API field 'topic'")
  return topic
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

function optionalStringArray(record: Record<string, unknown>, key: string): string[] | undefined {
  const value = record[key]
  if (value === undefined || value === null) return undefined
  if (!Array.isArray(value) || !value.every((item): item is string => typeof item === 'string')) {
    throw new Error(`Invalid API field '${key}': expected string array or null`)
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
    topic: optionalTopic(raw),
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
    topicCounts: [],
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
    tags: optionalStringArray(raw, 'tags'),
    keywords: optionalStringArray(raw, 'keywords'),
    date: optionalString(raw, 'date'),
    category: expectString(raw, 'category'),
    body: expectString(raw, 'body'),
    markdown: optionalString(raw, 'markdown'),
    samplePath: optionalString(raw, 'sample_path'),
    topic: optionalTopic(raw),
    formats,
  }
}

function parseDocumentClassification(raw: unknown): DocumentClassification {
  if (!isRecord(raw)) throw new Error('Invalid classification response payload')
  const source = expectString(raw, 'source')
  if (source !== 'rule' && source !== 'manual') {
    throw new Error("Invalid API field 'source': expected 'rule' or 'manual'")
  }
  return {
    documentId: expectString(raw, 'document_id'),
    summary: expectString(raw, 'summary'),
    category: expectString(raw, 'category'),
    tags: expectStringArray(raw, 'tags'),
    keywords: expectStringArray(raw, 'keywords'),
    source,
    manualLocked: expectBoolean(raw, 'manual_locked'),
    updatedAtMs: expectNumber(raw, 'updated_at_ms'),
    contentHash: expectString(raw, 'content_hash'),
  }
}

function parseApiErrorCode(raw: unknown): string | undefined {
  if (!isRecord(raw) || !isRecord(raw.error)) return undefined
  return typeof raw.error.code === 'string' ? raw.error.code : undefined
}

function parseApiErrorMessage(raw: unknown, fallback: string): string {
  if (!isRecord(raw) || !isRecord(raw.error)) return fallback
  return typeof raw.error.message === 'string' ? raw.error.message : fallback
}

function isProxyOrNetworkUnavailableStatus(status: number): boolean {
  return status === 502 || status === 503 || status === 504
}

export function searchLocalDocuments(
  query: string,
  category: string,
  requestedPage: number,
  limit: number,
  corpus: readonly Document[] = documents,
  topic = '',
): SearchViewResult {
  const matches = searchDocuments(query, corpus)
  const visible = matches.filter((doc) =>
    (!category || doc.category === category) && (!topic || doc.topic === topic),
  )
  if (query && (topic || isReferenceTopic(category))) {
    visible.sort((a, b) => Number(titleMatches(query, b)) - Number(titleMatches(query, a)))
  }
  const pageCount = Math.ceil(visible.length / limit)
  const page = pageCount > 0 && Number.isSafeInteger(requestedPage)
    ? Math.max(1, Math.min(requestedPage, pageCount))
    : 1
  const pageDocuments = visible.slice((page - 1) * limit, page * limit)

  const relatedSearches = query && visible.length
    ? [...new Set(visible.map((doc) => doc.tag))]
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
    topicCounts: topics.map((item) => ({
      ...item,
      count: matches.filter((doc) => doc.topic === item.id).length,
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
      topic: doc.topic,
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
    topic: doc.topic,
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
        result: searchLocalDocuments(params.query, params.category, params.page, params.limit),
        mode: 'local-fallback',
      }
    }

    const contentType = response.headers.get('content-type') ?? ''
    if (!contentType.includes('application/json')) {
      return {
        result: searchLocalDocuments(params.query, params.category, params.page, params.limit),
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
        result: searchLocalDocuments(params.query, params.category, params.page, params.limit),
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
        const doc = localFallbackDocument(id)
        return doc ? { doc, mode: 'local-fallback' } : { doc: null, mode: 'api' }
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
    const doc = localFallbackDocument(id)
    if (doc) return { doc, mode: 'local-fallback' }
    throw err
  }
}

async function parseResponseJsonOrUndefined(response: Response): Promise<unknown> {
  const contentType = response.headers.get('content-type') ?? ''
  if (!contentType.includes('application/json')) return undefined
  try {
    return await response.json()
  } catch {
    return undefined
  }
}

export async function fetchClassification(
  id: string,
  signal?: AbortSignal,
): Promise<DocumentClassification | null> {
  try {
    const response = await fetch('/api/v1/documents/' + encodeURIComponent(id) + '/classification', {
      signal,
    })

    if (
      response.status === 400 ||
      response.status === 404 ||
      isProxyOrNetworkUnavailableStatus(response.status)
    ) {
      return null
    }

    const contentType = response.headers.get('content-type') ?? ''
    if (!contentType.includes('application/json')) {
      return null
    }

    const json: unknown = await response.json()
    if (!response.ok) {
      throw new ApiRequestError(
        parseApiErrorMessage(json, `분류 정보를 불러오지 못했습니다 (${response.status}).`),
        response.status,
        parseApiErrorCode(json),
      )
    }

    return parseDocumentClassification(json)
  } catch (err) {
    if (err instanceof DOMException && err.name === 'AbortError') {
      throw err
    }
    if (err instanceof TypeError) {
      return null
    }
    throw err
  }
}

export async function updateClassification(
  id: string,
  input: UpdateClassificationInput,
): Promise<DocumentClassification> {
  const payload = input.resetToRule
    ? { reset_to_rule: true }
    : {
        summary: input.summary,
        category: input.category,
        tags: input.tags,
        keywords: input.keywords,
        reset_to_rule: false,
      }

  const response = await fetch('/api/v1/documents/' + encodeURIComponent(id) + '/classification', {
    method: 'PATCH',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify(payload),
  })

  const json = await parseResponseJsonOrUndefined(response)
  if (!response.ok) {
    throw new ApiRequestError(
      parseApiErrorMessage(json, `분류 수정에 실패했습니다 (${response.status}).`),
      response.status,
      parseApiErrorCode(json),
    )
  }

  return parseDocumentClassification(json)
}

export type SourceKind = 'local_dir' | 'sqlite'
export type SqliteCursorMode = 'integer_pk' | 'updated_at'

export type SqliteSourceOptions = {
  selectQuery: string
  idColumn: string
  titleColumn: string
  bodyColumn: string
  summaryColumn?: string
  cursorMode: SqliteCursorMode
  cursorColumn: string
  batchLimit?: number
}

export type RegisteredSource = {
  sourceId: string
  label: string
  path: string
  kind?: SourceKind
  sqlite?: SqliteSourceOptions
  defaultCategory: string
  defaultTag: string
  managedByEnv: boolean
  createdAtMs: number
  lastSyncedAtMs?: number
  documentCount: number
  lastError?: string
}

export type SourceListResponse = {
  sources: RegisteredSource[]
}

export type CreateSourceInput = {
  sourceId: string
  label: string
  path: string
  kind?: SourceKind
  sqlite?: SqliteSourceOptions
  defaultCategory?: string
  defaultTag?: string
}

export type DeleteSourceResponse = {
  sourceId: string
  deletedDocuments: number
  activeGeneration?: string
  activeEpoch: number
}

export type IndexJobMode = 'incremental' | 'full'
export type IndexJobState = 'pending' | 'running' | 'succeeded' | 'failed'

export type IndexJobRecord = {
  jobId: string
  sourceId?: string
  trigger: string
  mode: IndexJobMode
  state: IndexJobState
  startedAtMs: number
  finishedAtMs?: number
  indexedDocuments: number
  rebuilt: boolean
  activeGeneration?: string
  activeEpoch: number
  skippedFiles: string[]
  error?: string
}

export type IndexJobListResponse = {
  jobs: IndexJobRecord[]
  activeGeneration?: string
  activeEpoch: number
  refreshIntervalSecs?: number
}

function optionalNumber(record: Record<string, unknown>, key: string): number | undefined {
  const value = record[key]
  if (value === undefined || value === null) return undefined
  if (typeof value !== 'number' || !Number.isFinite(value)) {
    throw new Error(`Invalid API field '${key}': expected finite number or null`)
  }
  return value
}

function parseSqliteOptions(raw: unknown): SqliteSourceOptions | undefined {
  if (raw === undefined || raw === null) return undefined
  if (!isRecord(raw)) throw new Error("Invalid API field 'sqlite'")
  const cursorMode = expectString(raw, 'cursor_mode')
  if (cursorMode !== 'integer_pk' && cursorMode !== 'updated_at') {
    throw new Error("Invalid API field 'cursor_mode'")
  }
  return {
    selectQuery: expectString(raw, 'select_query'),
    idColumn: expectString(raw, 'id_column'),
    titleColumn: expectString(raw, 'title_column'),
    bodyColumn: expectString(raw, 'body_column'),
    summaryColumn: optionalString(raw, 'summary_column'),
    cursorMode,
    cursorColumn: expectString(raw, 'cursor_column'),
    batchLimit: optionalNumber(raw, 'batch_limit'),
  }
}

function serializeSqliteOptions(options: SqliteSourceOptions) {
  return {
    select_query: options.selectQuery,
    id_column: options.idColumn,
    title_column: options.titleColumn,
    body_column: options.bodyColumn,
    summary_column: options.summaryColumn || undefined,
    cursor_mode: options.cursorMode,
    cursor_column: options.cursorColumn,
    batch_limit: options.batchLimit,
  }
}

function parseRegisteredSource(raw: unknown): RegisteredSource {
  if (!isRecord(raw)) throw new Error('Invalid source record payload')
  const kind = optionalString(raw, 'kind') ?? 'local_dir'
  if (kind !== 'local_dir' && kind !== 'sqlite') throw new Error("Invalid API field 'kind'")
  return {
    sourceId: expectString(raw, 'source_id'),
    label: expectString(raw, 'label'),
    path: expectString(raw, 'path'),
    kind,
    sqlite: parseSqliteOptions(raw.sqlite),
    defaultCategory: expectString(raw, 'default_category'),
    defaultTag: expectString(raw, 'default_tag'),
    managedByEnv: expectBoolean(raw, 'managed_by_env'),
    createdAtMs: expectNumber(raw, 'created_at_ms'),
    lastSyncedAtMs: optionalNumber(raw, 'last_synced_at_ms'),
    documentCount: expectNumber(raw, 'document_count'),
    lastError: optionalString(raw, 'last_error'),
  }
}

function parseIndexJobRecord(raw: unknown): IndexJobRecord {
  if (!isRecord(raw)) throw new Error('Invalid index job record payload')
  const mode = expectString(raw, 'mode')
  if (mode !== 'incremental' && mode !== 'full') {
    throw new Error("Invalid API field 'mode'")
  }
  const state = expectString(raw, 'state')
  if (state !== 'pending' && state !== 'running' && state !== 'succeeded' && state !== 'failed') {
    throw new Error("Invalid API field 'state'")
  }
  return {
    jobId: expectString(raw, 'job_id'),
    sourceId: optionalString(raw, 'source_id'),
    trigger: expectString(raw, 'trigger'),
    mode,
    state,
    startedAtMs: expectNumber(raw, 'started_at_ms'),
    finishedAtMs: optionalNumber(raw, 'finished_at_ms'),
    indexedDocuments: expectNumber(raw, 'indexed_documents'),
    rebuilt: expectBoolean(raw, 'rebuilt'),
    activeGeneration: optionalString(raw, 'active_generation'),
    activeEpoch: expectNumber(raw, 'active_epoch'),
    skippedFiles: expectStringArray(raw, 'skipped_files'),
    error: optionalString(raw, 'error'),
  }
}

async function requestAdminJson(url: string, init: RequestInit, fallbackError: string): Promise<unknown> {
  let response: Response
  try {
    response = await fetch(url, init)
  } catch (err) {
    if (err instanceof DOMException && err.name === 'AbortError') throw err
    throw new ApiRequestError('백엔드 API 서버에 연결할 수 없습니다.', 503)
  }

  const contentType = response.headers.get('content-type') ?? ''
  if (!contentType.includes('application/json')) {
    if (isProxyOrNetworkUnavailableStatus(response.status) || response.ok) {
      throw new ApiRequestError('백엔드 API 서버에 연결할 수 없습니다.', 503)
    }
    throw new ApiRequestError(`${fallbackError} (${response.status})`, response.status)
  }

  const json: unknown = await response.json()
  if (!response.ok) {
    throw new ApiRequestError(
      parseApiErrorMessage(json, `${fallbackError} (${response.status})`),
      response.status,
      parseApiErrorCode(json),
    )
  }
  return json
}

export async function fetchSources(signal?: AbortSignal): Promise<SourceListResponse> {
  const json = await requestAdminJson('/api/v1/sources', { signal }, '소스 목록을 불러오지 못했습니다')
  if (!isRecord(json) || !Array.isArray(json.sources)) {
    throw new Error('Invalid source list response')
  }
  return { sources: json.sources.map(parseRegisteredSource) }
}

export async function createSource(input: CreateSourceInput): Promise<RegisteredSource> {
  const json = await requestAdminJson(
    '/api/v1/sources',
    {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify({
        source_id: input.sourceId,
        label: input.label,
        path: input.path,
        kind: input.kind,
        sqlite: input.sqlite ? serializeSqliteOptions(input.sqlite) : undefined,
        default_category: input.defaultCategory || undefined,
        default_tag: input.defaultTag || undefined,
      }),
    },
    '소스 등록에 실패했습니다',
  )
  return parseRegisteredSource(json)
}

export async function deleteSource(sourceId: string): Promise<DeleteSourceResponse> {
  const json = await requestAdminJson(
    '/api/v1/sources/' + encodeURIComponent(sourceId),
    {
      method: 'DELETE',
      headers: { 'content-type': 'application/json' },
    },
    '소스 삭제에 실패했습니다',
  )
  if (!isRecord(json)) throw new Error('Invalid delete source response')
  return {
    sourceId: expectString(json, 'source_id'),
    deletedDocuments: expectNumber(json, 'deleted_documents'),
    activeGeneration: optionalString(json, 'active_generation'),
    activeEpoch: expectNumber(json, 'active_epoch'),
  }
}

export async function fetchIndexJobs(signal?: AbortSignal): Promise<IndexJobListResponse> {
  const json = await requestAdminJson('/api/v1/index/jobs', { signal }, '색인 작업 목록을 불러오지 못했습니다')
  if (!isRecord(json) || !Array.isArray(json.jobs)) {
    throw new Error('Invalid index job list response')
  }
  return {
    jobs: json.jobs.map(parseIndexJobRecord),
    activeGeneration: optionalString(json, 'active_generation'),
    activeEpoch: expectNumber(json, 'active_epoch'),
    refreshIntervalSecs: optionalNumber(json, 'refresh_interval_secs'),
  }
}

export async function triggerIndexJob(input?: {
  sourceId?: string
  mode?: IndexJobMode
}): Promise<IndexJobRecord> {
  const payload = {
    source_id: input?.sourceId || undefined,
    mode: input?.mode ?? 'incremental',
  }
  const json = await requestAdminJson(
    '/api/v1/index/jobs',
    {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify(payload),
    },
    '색인 작업 실행에 실패했습니다',
  )
  return parseIndexJobRecord(json)
}

export type AnalyzeSearchInput = {
  text: string
  splitIdentifiers?: boolean
  charNgrams?: boolean
}

export type AnalyzeSearchResponse = {
  profile: string
  analyzerVersion: number
  normalizedQueryTerms: string[]
  terms: string[]
  charNgrams: string[]
}

export async function analyzeSearchText(input: AnalyzeSearchInput): Promise<AnalyzeSearchResponse> {
  const json = await requestAdminJson(
    '/api/v1/search/analyze',
    {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify({
        text: input.text,
        split_identifiers: input.splitIdentifiers ?? true,
        char_ngrams: input.charNgrams ?? true,
      }),
    },
    '검색어 분석에 실패했습니다',
  )
  if (!isRecord(json)) throw new Error('Invalid analyze response')
  return {
    profile: expectString(json, 'profile'),
    analyzerVersion: expectNumber(json, 'analyzer_version'),
    normalizedQueryTerms: expectStringArray(json, 'normalized_query_terms'),
    terms: expectStringArray(json, 'terms'),
    charNgrams: expectStringArray(json, 'char_ngrams'),
  }
}
