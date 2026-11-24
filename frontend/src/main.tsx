import { useEffect, useState } from 'react'
import { DotsThreeVerticalIcon as More, FileTextIcon as FileText, MagnifyingGlassIcon as Search, XIcon as X } from '@phosphor-icons/react'
import { createRoot } from 'react-dom/client'
import { type DataSourceMode, fetchRecommendations, fetchSearch, isSourceCodePath, type RecommendedTermItem, type SearchViewResult } from './api'
import { categories, categoryLabels, isReferenceTopic, isReferenceTopicFilter, loadLocalSamples, searchUrl, topics } from './documents'
import { highlightText } from './highlight'
import { referenceIdPrefix, type ReferenceCorpus } from './reference-corpus'
import './style.css'

const pageSize = 15

type SearchState =
  | { status: 'loading' }
  | { status: 'ready'; data: SearchViewResult; mode: DataSourceMode }
  | { status: 'error'; message: string }

const referenceSources = [
  { prefix: referenceIdPrefix('rust', 'comprehensive-rust'), load: () => import('./comprehensive-rust').then(({ comprehensiveRust }) => comprehensiveRust) },
  { prefix: referenceIdPrefix('rust', 'rust-book'), load: () => import('./rust-book').then(({ rustBook }) => rustBook) },
  { prefix: referenceIdPrefix('rust', 'rust-reference'), load: () => import('./rust-reference').then(({ rustReference }) => rustReference) },
  { prefix: referenceIdPrefix('rust', 'cargo-book'), load: () => import('./cargo-book').then(({ cargoBook }) => cargoBook) },
  { prefix: referenceIdPrefix('rust', 'rust-std'), load: () => import('./rust-std').then(({ rustStd }) => rustStd) },
  { prefix: referenceIdPrefix('rust', 'rust-crates'), load: () => import('./rust-crates').then(({ rustCrates }) => rustCrates) },
  { prefix: referenceIdPrefix('rust', 'rustc-error-codes'), load: () => import('./rustc-error-codes').then(({ rustcErrorCodes }) => rustcErrorCodes) },
  { prefix: referenceIdPrefix('go', 'go-reference'), load: () => import('./go-reference').then(({ goReference }) => goReference) },
]

function isReferenceDocumentId(documentId: string) {
  return referenceSources.some(({ prefix }) => documentId.startsWith(prefix))
}

function loadReferenceCorpora(documentId?: string): Promise<ReferenceCorpus[]> {
  const sources = !documentId || isReferenceDocumentId(documentId) ? referenceSources : []
  return Promise.all(sources.map(({ load }) => load()))
}

function inferGuideTopic(hit: SearchViewResult['hits'][number]) {
  const metadata = `${hit.tag} ${hit.source} ${hit.title}`
  if (/\brust\b|러스트/i.test(metadata)) return 'rust' as const
  if (/\bgo(?:lang)?\b|고랭/i.test(metadata)) return 'go' as const
  return undefined
}

function mergeSearchHits(...groups: ReadonlyArray<SearchViewResult['hits']>): SearchViewResult['hits'] {
  const merged = new Map<string, SearchViewResult['hits'][number]>()
  for (const hit of groups.flat()) {
    const existing = merged.get(hit.documentId)
    const topic = existing?.topic ?? hit.topic ?? inferGuideTopic(hit)
    merged.set(hit.documentId, {
      ...(existing ?? hit),
      topic,
      date: existing?.date ?? hit.date,
      samplePath: existing?.samplePath ?? hit.samplePath,
    })
  }
  return [...merged.values()]
}

export async function fetchSearchPage(
  query: string, category: string, topic: string, page: number, signal: AbortSignal,
): Promise<{ result: SearchViewResult; mode: DataSourceMode }> {
  let refPromise: Promise<ReferenceCorpus[]> | undefined
  const getReferences = () => (refPromise ??= loadReferenceCorpora())
  async function searchAllLocal(): Promise<{ result: SearchViewResult; mode: DataSourceMode }> {
    const [{ documents, searchLocalDocuments }, corpora] = await Promise.all([
      loadLocalSamples(),
      getReferences().catch(() => []),
    ])
    const allDocuments = [...new Map(documents.concat(corpora.flatMap(({ documents }) => documents)).map((doc) => [doc.id, doc])).values()]
    const allLocal = searchLocalDocuments(query, '', 1, pageSize, allDocuments)
    const local = searchLocalDocuments(query, '', page, pageSize, allDocuments)
    return {
      result: {
        ...local,
        totalUnfilteredHits: allLocal.totalUnfilteredHits,
        categoryCounts: allLocal.categoryCounts,
      },
      mode: 'local-fallback',
    }
  }
  async function searchGuideMerged(topicFilter = ''): Promise<{ result: SearchViewResult; mode: DataSourceMode }> {
    const localFallback = async () => {
      const [{ documents, searchLocalDocuments }, corpora] = await Promise.all([
        loadLocalSamples(),
        getReferences(),
      ])
      const allDocuments = [...new Map(documents.concat(corpora.flatMap(({ documents }) => documents)).map((doc) => [doc.id, doc])).values()]
      const allLocal = searchLocalDocuments(query, '', 1, pageSize, allDocuments)
      const guideDocuments = allDocuments.filter((doc) => doc.category === 'guide')
      const localGuideAll = searchLocalDocuments(query, 'guide', 1, Math.max(1, guideDocuments.length), guideDocuments)
      const localPage = searchLocalDocuments(
        query, 'guide', page, pageSize, guideDocuments, topicFilter,
      )
      return {
        result: {
          ...localPage,
          totalUnfilteredHits: allLocal.totalUnfilteredHits,
          categoryCounts: allLocal.categoryCounts,
          topicCounts: localGuideAll.topicCounts,
        },
        mode: 'local-reference' as const,
      }
    }

    let apiFirst: SearchViewResult | undefined
    const serverHits: SearchViewResult['hits'] = []
    try {
      const first = await fetchSearch({ query, category: 'guide', page: 1, limit: 100 }, signal)
      if (first.mode !== 'api') return localFallback()
      apiFirst = first.result
      serverHits.push(...first.result.hits)
      for (let serverPage = 2; serverPage <= first.result.totalPages; serverPage++) {
        const next = await fetchSearch({ query, category: 'guide', page: serverPage, limit: 100 }, signal)
        if (next.mode !== 'api') return localFallback()
        serverHits.push(...next.result.hits)
      }
    } catch (error) {
      if (error instanceof DOMException && error.name === 'AbortError') throw error
      return localFallback()
    }

    const allGuideHits = mergeSearchHits(serverHits)
    const visibleHits = topicFilter ? allGuideHits.filter((hit) => hit.topic === topicFilter) : allGuideHits
    const pageCount = Math.ceil(visibleHits.length / pageSize)
    const currentPage = pageCount > 0 ? Math.max(1, Math.min(page, pageCount)) : 1
    const categoryCounts = apiFirst.categoryCounts.filter(({ id }) => categories.some((item) => item.id === id))

    return {
      result: {
        query,
        page: currentPage,
        limit: pageSize,
        totalHits: visibleHits.length,
        totalPages: pageCount,
        totalUnfilteredHits: apiFirst.totalUnfilteredHits,
        categoryCounts,
        topicCounts: topics.map((item) => ({
          ...item,
          count: allGuideHits.filter((hit) => hit.topic === item.id).length,
        })),
        relatedSearches: apiFirst.relatedSearches,
        hits: visibleHits.slice((currentPage - 1) * pageSize, currentPage * pageSize),
        snapshot: apiFirst.snapshot,
      },
      mode: 'api',
    }
  }
  if (isReferenceTopicFilter(topic)) {
    return searchGuideMerged(topic)
  }

  if (category === 'guide') {
    return searchGuideMerged()
  }

  if (category && categories.some((item) => item.id === category)) {
    let search: Awaited<ReturnType<typeof fetchSearch>> | undefined
    try {
      search = await fetchSearch({ query, category, page, limit: pageSize }, signal)
    } catch (error) {
      if (error instanceof DOMException && error.name === 'AbortError') throw error
    }
    if (search?.mode === 'api') {
      const categoryCounts = search.result.categoryCounts.filter(({ id }) => categories.some((item) => item.id === id))
      return {
        result: {
          ...search.result,
          categoryCounts,
          topicCounts: [],
        },
        mode: 'api',
      }
    }

    const [{ documents, searchLocalDocuments }, corpora] = await Promise.all([
      loadLocalSamples(),
      getReferences().catch(() => []),
    ])
    const fallbackDocs = category === 'code'
      ? [...new Map(documents.concat(corpora.flatMap(({ documents }) => documents)).map((doc) => [doc.id, doc])).values()]
      : documents
    const allLocal = searchLocalDocuments(query, '', 1, pageSize, fallbackDocs)
    const local = searchLocalDocuments(query, category, page, pageSize, fallbackDocs)
    return {
      result: {
        ...local,
        totalUnfilteredHits: allLocal.totalUnfilteredHits,
        categoryCounts: allLocal.categoryCounts,
      },
      mode: 'local-fallback',
    }
  }

  let search: Awaited<ReturnType<typeof fetchSearch>>
  try {
    search = await fetchSearch({ query, category, page, limit: pageSize }, signal)
  } catch (error) {
    if (error instanceof DOMException && error.name === 'AbortError') throw error
    return searchAllLocal()
  }
  if (search.mode !== 'api') return searchAllLocal()
  const categoryCounts = search.result.categoryCounts.filter(({ id }) => categories.some((item) => item.id === id))
  return {
    mode: search.mode,
    result: {
      ...search.result,
      categoryCounts,
      topicCounts: [],
    },
  }
}

export function SearchForm({ query = '', category = '', topic = '' }: { query?: string; category?: string; topic?: string }) {
  const [inputValue, setInputValue] = useState(query)
  const [focused, setFocused] = useState(false)
  const [activeIndex, setActiveIndex] = useState(-1)
  const [suggestions, setSuggestions] = useState<RecommendedTermItem[]>([])

  useEffect(() => {
    setInputValue(query)
  }, [query])

  useEffect(() => {
    if (!focused) return
    const controller = new AbortController()
    const timer = window.setTimeout(() => {
      void fetchRecommendations({ query: inputValue, category, limit: 6 }, controller.signal).then(
        (items) => {
          if (!controller.signal.aborted) {
            setSuggestions(items)
            setActiveIndex(-1)
          }
        },
        () => {},
      )
    }, 120)
    return () => {
      window.clearTimeout(timer)
      controller.abort()
    }
  }, [focused, inputValue, category])

  const showDropdown = focused && suggestions.length > 0

  return (
    <div
      className="search-box"
      onBlur={(event) => {
        if (!event.currentTarget.contains(event.relatedTarget)) {
          setFocused(false)
          setActiveIndex(-1)
        }
      }}
    >
      <form role="search" action="/search">
        <Search size={22} aria-hidden="true" />
        <input
          type="search"
          name="q"
          aria-label="문서 검색어"
          aria-autocomplete="list"
          aria-expanded={showDropdown}
          aria-controls={showDropdown ? 'search-suggestions' : undefined}
          placeholder="제목, 키워드, 심볼 또는 path:hwpx ext:ts 검색"
          value={inputValue}
          onChange={(event) => setInputValue(event.target.value)}
          onFocus={() => setFocused(true)}
          onKeyDown={(event) => {
            if (!showDropdown) return
            if (event.key === 'ArrowDown') {
              event.preventDefault()
              setActiveIndex((prev) => (prev + 1) % suggestions.length)
            } else if (event.key === 'ArrowUp') {
              event.preventDefault()
              setActiveIndex((prev) => (prev <= 0 ? suggestions.length - 1 : prev - 1))
            } else if (event.key === 'Enter' && activeIndex >= 0 && suggestions[activeIndex]) {
              event.preventDefault()
              window.location.assign(searchUrl(suggestions[activeIndex].term, category, 1, topic))
            } else if (event.key === 'Escape') {
              setFocused(false)
              setActiveIndex(-1)
            }
          }}
        />
        {category && <input type="hidden" name="category" value={category} />}
        {category === 'guide' && topic && <input type="hidden" name="topic" value={topic} />}
        <button type="submit">검색</button>
      </form>
      {showDropdown && (
        <ul id="search-suggestions" role="listbox" className="search-suggestions" aria-label="추천 검색어">
          {suggestions.map((item, index) => (
            <li key={item.term} role="option" aria-selected={index === activeIndex}>
              <a
                href={searchUrl(item.term, category, 1, topic)}
                onMouseDown={(event) => event.preventDefault()}
                onClick={() => setFocused(false)}
              >
                <Search size={15} aria-hidden="true" />
                <span className="suggestion-term">{highlightText(item.term, inputValue)}</span>
                {item.documentCount > 0 && <span className="suggestion-count">{item.documentCount}건</span>}
              </a>
            </li>
          ))}
        </ul>
      )}
    </div>
  )
}

export function RecommendedChips({
  category = '',
  topic = '',
  label = '추천 검색어',
}: {
  category?: string
  topic?: string
  label?: string
}) {
  const [items, setItems] = useState<RecommendedTermItem[]>([])

  useEffect(() => {
    const controller = new AbortController()
    void fetchRecommendations({ query: '', category, limit: 8 }, controller.signal).then(
      (list) => {
        if (!controller.signal.aborted) setItems(list)
      },
      () => {},
    )
    return () => controller.abort()
  }, [category])

  if (items.length === 0) return null

  return (
    <section className="recommended-chips" aria-label={label}>
      <span className="recommended-label">{label}</span>
      <div className="recommended-list">
        {items.map((item) => (
          <a key={item.term} href={searchUrl(item.term, category || item.category || '', 1, topic)}>
            {item.term}
          </a>
        ))}
      </div>
    </section>
  )
}

export function HomePage() {
  return (
    <div className="home-wrap">
      <main id="content" className="shell home">
        <h1 className="brand home-brand"><img className="logo" src="/favicon.svg" alt="" />dok3search</h1>
        <SearchForm />
        <RecommendedChips label="추천 검색어" />
        <nav className="category-filters" aria-label="문서 카테고리">
          <a href="/search">전체</a>
          {categories.map((item) => <a key={item.id} href={searchUrl('', item.id)}>{item.label}</a>)}
        </nav>
      </main>
      <footer className="shell footer home-footer">
        <span>폐쇄망 문서·코드 검색</span>
        <a href="/admin">관리</a>
      </footer>
    </div>
  )
}

export function SearchPage() {
  const params = new URLSearchParams(window.location.search)
  const query = params.get('q')?.trim() ?? ''
  const rawCategory = params.get('category') ?? ''
  const legacyTopic = isReferenceTopic(rawCategory) ? rawCategory : ''
  const activeCategory = categories.find((item) => item.id === rawCategory) ??
    (legacyTopic ? categories.find((item) => item.id === 'guide') : undefined)
  const category = activeCategory?.id ?? ''
  const requestedTopic = params.get('topic')?.trim() || legacyTopic
  const topic = category === 'guide' && isReferenceTopicFilter(requestedTopic) ? requestedTopic : ''
  const rawPage = Number(params.get('page'))
  const requestedPage = Number.isSafeInteger(rawPage) && rawPage > 0 ? rawPage : 1
  const [openMenu, setOpenMenu] = useState('')
  const [copyStatus, setCopyStatus] = useState('')
  const [searchState, setSearchState] = useState<SearchState>({ status: 'loading' })

  useEffect(() => {
    const controller = new AbortController()
    setSearchState({ status: 'loading' })

    void fetchSearchPage(query, category, topic, requestedPage, controller.signal).then(
      ({ result, mode }) => {
        if (!controller.signal.aborted) {
          setSearchState({ status: 'ready', data: result, mode })
        }
      },
      (error: unknown) => {
        if (controller.signal.aborted) return
        if (error instanceof DOMException && error.name === 'AbortError') return
        const message = error instanceof Error ? error.message : '검색 결과를 불러오지 못했습니다.'
        setSearchState({ status: 'error', message })
      },
    )

    return () => controller.abort()
  }, [query, category, topic, requestedPage])

  const browseDesign = category === 'design' && !query

  function documentUrl(hit: SearchViewResult['hits'][number], page: number) {
    // Search hits carry ids like `local-markdown:samples/.../rust-version.md`;
    // the raw `:` and `/` must be escaped or the URL gains extra path segments
    // and the single-segment /docs/:id route never matches.
    const base = searchUrl(query, category, page, topic).replace('/search', '/docs/' + encodeURIComponent(hit.documentId))
    return hit.matchedSectionId ? base + '#' + encodeURIComponent(hit.matchedSectionId) : base
  }

  async function copyTitle(title: string) {
    try {
      await navigator.clipboard.writeText(title)
      setCopyStatus('문서 제목을 복사했습니다.')
    } catch {
      setCopyStatus('문서 제목을 복사할 수 없습니다.')
    }
  }

  const readyData = searchState.status === 'ready' ? searchState.data : null
  const page = readyData?.page ?? 1
  const pageCount = readyData?.totalPages ?? 0
  const firstPage = Math.max(1, Math.min(page - 2, pageCount - 4))
  const lastPage = Math.min(firstPage + 4, pageCount)

  const countByCategory = new Map(
    readyData?.categoryCounts.map((item) => [item.id, item.count]) ?? [],
  )
  const hasReferenceMatches = readyData?.topicCounts.some(({ count }) => count > 0) ?? false

  return (
    <>
      <a className="skip" href="#content">본문으로 이동</a>
      <header className="top">
        <div className="shell top-inner">
          <a className="brand" href="/" aria-label="dok3search 홈"><img className="logo" src="/favicon.svg" alt="" />dok3search</a>
          <SearchForm query={query} category={category} topic={topic} />
        </div>
      </header>
      <main id="content" className="has-query">
        <h1 className="visually-hidden">문서 검색</h1>
        <div className="shell library">
          <div className="category-filters" role="group" aria-label="문서 카테고리">
              <a href={searchUrl(query)} aria-current={!category ? 'page' : undefined}>
                전체 {!topic && readyData && <span className="category-count">{readyData.totalUnfilteredHits}</span>}
            </a>
            {categories.map((item) => (
              <a key={item.id} href={searchUrl(query, item.id)} aria-current={category === item.id ? 'page' : undefined}>
                {item.label} {!topic && readyData && <span className="category-count">{countByCategory.get(item.id) ?? 0}</span>}
              </a>
            ))}
          </div>
          {category === 'guide' && (
            <nav className="topic-filters" aria-label="개발 가이드 세부 분류">
              <a href={searchUrl(query, 'guide')} aria-current={!topic ? 'page' : undefined}>전체 가이드</a>
              {topics.map((item) => (
                <a key={item.id} href={searchUrl(query, 'guide', 1, item.id)} aria-current={topic === item.id ? 'page' : undefined}>
                  {item.label} {readyData && <span className="category-count">{readyData.topicCounts.find(({ id }) => id === item.id)?.count ?? 0}</span>}
                </a>
              ))}
            </nav>
          )}
          <section id="documents" className="documents" aria-labelledby="documents-heading">
            {query || (activeCategory && !browseDesign) ? (
              <div className="section-heading">
                <h2 id="documents-heading">{query ? '검색 결과' : activeCategory?.label}{topic ? ' · ' + topics.find((item) => item.id === topic)?.label : ''}</h2>
                {query && <a className="text-button" href={searchUrl('', category, 1, topic)}>초기화 <X size={14} /></a>}
              </div>
            ) : <h2 id="documents-heading" className="visually-hidden">검색 결과</h2>}
            {!query && <RecommendedChips category={category} topic={topic} label="추천 검색어" />}

            {searchState.status === 'loading' && (
              <p className="empty" role="status">문서를 불러오는 중입니다.</p>
            )}

            {searchState.status === 'error' && (
              <p className="empty" role="alert">{searchState.message}</p>
            )}

            {readyData && (
              <>
                <span className="visually-hidden" role="status">
                  문서 {readyData.totalHits}개{pageCount > 1 && ' · ' + page + '/' + pageCount + ' 페이지'}
                </span>
                {readyData.hits.length > 0 ? <ul className="document-list">{readyData.hits.map((hit) => (
                  <li className="result-row" key={hit.documentId}>
                    <a className="document" href={documentUrl(hit, page)}>
                      <span className="file-icon"><FileText size={18} aria-hidden="true" /></span>
                      <span className="doc-copy">
                        <small>
                          {hit.topic ? `${topics.find((item) => item.id === hit.topic)?.label} · ${hit.source} · ${hit.tag}` : categoryLabels.get(hit.category as typeof categories[number]['id']) ?? hit.source}
                          {hit.hasPairedFormats && ' · Markdown · YAML'}
                          {hit.matchedSectionHeading && (
                            <span className="hit-symbol">
                              {highlightText(hit.matchedSectionHeading, query, hit.topic ?? hit.category)}
                              {hit.matchedLineStart ? ` · L${hit.matchedLineStart}` : ''}
                            </span>
                          )}
                        </small>
                        <strong>{highlightText(hit.title, query, hit.topic ?? hit.category)}</strong>
                        <span className={isSourceCodePath(hit.samplePath) ? 'snippet snippet-code' : 'snippet'}>{highlightText(hit.snippet, query, hit.topic ?? hit.category)}</span>
                      </span>
                    </a>
                    {hit.date && <time dateTime={hit.date}>{hit.date}</time>}
                    <div className="document-actions"
                      onBlur={(event) => {
                        if (!event.currentTarget.contains(event.relatedTarget)) setOpenMenu('')
                      }}
                      onKeyDown={(event) => {
                        if (event.key === 'Escape' && openMenu === hit.documentId) {
                          event.stopPropagation()
                          event.currentTarget.querySelector('button')?.focus()
                          setOpenMenu('')
                        }
                      }}
                    >
                      <button type="button" className="more-button"
                        aria-label={hit.title + ' 옵션'}
                        aria-expanded={openMenu === hit.documentId}
                        onClick={(event) => {
                          event.currentTarget.focus()
                          setOpenMenu(openMenu === hit.documentId ? '' : hit.documentId)
                        }}
                      ><More size={20} weight="bold" aria-hidden="true" /></button>
                      {openMenu === hit.documentId && (
                        <div className="document-options" onPointerDown={(event) => event.preventDefault()}>
                          <a href={documentUrl(hit, page)}>문서 열기</a>
                          <button type="button" onClick={() => void copyTitle(hit.title)}>제목 복사</button>
                        </div>
                      )}
                    </div>
                  </li>
                ))}</ul> : (
                  <>
                    <p className="empty">{!category && hasReferenceMatches
                      ? '일치하는 Rust·Go 자료는 개발 가이드의 세부 분류에서 열 수 있습니다.'
                      : '검색 조건에 맞는 문서가 없습니다.'}</p>
                    {query && <RecommendedChips category={category} topic={topic} label="추천 검색어" />}
                  </>
                )}
                {readyData.relatedSearches.length > 0 && (
                  <section className="related" aria-labelledby="related-heading">
                    <h3 id="related-heading">관련 검색어</h3>
                    <ul>
                      {readyData.relatedSearches.map((term) => (
                        <li key={term}>
                          <a href={searchUrl(term, category, 1, topic)}>
                            <Search size={16} aria-hidden="true" />{term}
                          </a>
                        </li>
                      ))}
                    </ul>
                  </section>
                )}
                {pageCount > 1 && (
                  <nav className="pagination" aria-label="문서 페이지">
                    {page > 1 && <a href={searchUrl(query, category, page - 1, topic)}>이전</a>}
                    {firstPage > 1 && <span aria-hidden="true">…</span>}
                    {Array.from({ length: lastPage - firstPage + 1 }, (_, index) => firstPage + index).map((number) => (
                      <a key={number} href={searchUrl(query, category, number, topic)} aria-label={number + ' 페이지'} aria-current={number === page ? 'page' : undefined}>{number}</a>
                    ))}
                    {lastPage < pageCount && <span aria-hidden="true">…</span>}
                    {page < pageCount && <a href={searchUrl(query, category, page + 1, topic)}>다음</a>}
                  </nav>
                )}
              </>
            )}
          </section>
        </div>
      </main>
      <footer className="shell footer">
        <span>
          {searchState.status === 'ready' && (searchState.mode === 'api'
            ? `Rust API 연결됨 · ${searchState.data.snapshot.generationId}`
            : searchState.mode === 'local-reference' ? '로컬 개발 참고 자료' : '샘플 데이터로 구동되는 로컬 미리보기')}
        </span>
        <a href="/admin">관리</a>
      </footer>
      <div className="visually-hidden" role="status">{copyStatus}</div>
    </>
  )
}

const path = window.location.pathname
const adminMatch = /^\/admin(?:\/(sources|jobs|analyzers))?\/?$/.exec(path)
// Document ids contain `:` and `/` (`local-markdown:samples/…/*.md`). Encoded
// links keep %2F in the path, and freshly pasted raw links arrive with real
// slashes, so the detail route must not be limited to a single segment.
const detail = /^\/docs\/(.+)\/?$/.exec(path)
const legacyIds: Record<string, string> = {
  'design-design-md': 'design-index-md',
  'design-design-yaml': 'design-map-yaml',
}
let detailId: string | undefined
if (detail) {
  try {
    detailId = decodeURIComponent(detail[1])
  } catch {
    detailId = detail[1]
  }
  const legacyId = legacyIds[detailId]
  detailId = legacyId ?? detailId
}

const rootElement = document.getElementById('root')
if (rootElement) {
  const root = createRoot(rootElement)
  if (adminMatch) {
    const initialTab = adminMatch[1] === 'jobs'
      ? 'jobs'
      : adminMatch[1] === 'analyzers'
        ? 'analyzers'
        : 'sources'
    void import('./admin-page').then(
      ({ AdminPage }) => root.render(<AdminPage initialTab={initialTab} />),
      () => root.render(<main className="shell reader"><p role="alert">관리 화면을 불러올 수 없습니다.</p></main>),
    )
  } else if (detailId) {
    const resolvedIdPromise = detailId.startsWith('design-')
      ? loadLocalSamples().then(
          ({ documents }) =>
            documents.find((doc) => doc.category === 'design' && doc.id.toLowerCase() === detailId!.toLowerCase())?.id ?? detailId!,
          () => detailId!,
        )
      : Promise.resolve(detailId)
    void Promise.all([import('./document-reader'), loadReferenceCorpora(detailId), resolvedIdPromise]).then(
      ([{ DocumentPage }, corpora, resolvedId]) => root.render(<DocumentPage id={resolvedId} corpora={corpora} />),
      () => root.render(<main className="shell reader"><p role="alert">문서를 불러올 수 없습니다.</p></main>),
    )
  } else {
    root.render(path === '/search' ? <SearchPage /> : <HomePage />)
  }
}
