import { useEffect, useState } from 'react'
import { DotsThreeVerticalIcon as More, FileTextIcon as FileText, MagnifyingGlassIcon as Search, XIcon as X } from '@phosphor-icons/react'
import { createRoot } from 'react-dom/client'
import { type DataSourceMode, fetchSearch, searchLocalDocuments, type SearchViewResult } from './api'
import { categories, categoryLabels, documents, isReferenceTopic, isReferenceTopicFilter, searchUrl, topics } from './documents'
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
  { prefix: referenceIdPrefix('go', 'go-reference'), load: () => import('./go-reference').then(({ goReference }) => goReference) },
]

function loadReferenceCorpora(documentId?: string): Promise<ReferenceCorpus[]> {
  return Promise.all(referenceSources
    .filter(({ prefix }) => !documentId || documentId.startsWith(prefix))
    .map(({ load }) => load()))
}

async function fetchSearchPage(
  query: string, category: string, topic: string, page: number, signal: AbortSignal,
): Promise<{ result: SearchViewResult; mode: DataSourceMode }> {
  const references = loadReferenceCorpora()
  if (isReferenceTopicFilter(topic)) {
    const [corpora, api] = await Promise.all([
      references,
      fetchSearch({ query, category: '', page: 1, limit: pageSize }, signal),
    ])
    const corpus = documents.filter((doc) => doc.topic).concat(corpora.flatMap(({ documents }) => documents))
    const local = searchLocalDocuments(query, 'guide', page, pageSize, corpus, topic)
    return {
      result: {
        ...local,
        totalUnfilteredHits: api.result.totalUnfilteredHits,
        categoryCounts: api.result.categoryCounts,
      },
      mode: 'local-reference',
    }
  }

  const [search, corpora] = await Promise.all([
    fetchSearch({ query, category, page, limit: pageSize }, signal),
    references.catch(() => []),
  ])
  const referenceDocuments = documents.filter((doc) => doc.topic).concat(corpora.flatMap(({ documents }) => documents))
  const local = searchLocalDocuments(query, '', 1, pageSize, referenceDocuments)
  const categoryCounts = search.result.categoryCounts.filter(({ id }) => categories.some((item) => item.id === id))
  return {
    mode: search.mode,
    result: {
      ...search.result,
      categoryCounts,
      topicCounts: local.topicCounts,
    },
  }
}

function SearchForm({ query = '', category = '', topic = '' }: { query?: string; category?: string; topic?: string }) {
  return (
    <form role="search" action="/search">
      <Search size={22} aria-hidden="true" />
      <input type="search" name="q" aria-label="문서 검색어" placeholder="제목, 키워드 또는 코드 검색" defaultValue={query} />
      {category && <input type="hidden" name="category" value={category} />}
      {category === 'guide' && topic && <input type="hidden" name="topic" value={topic} />}
      <button type="submit">검색</button>
    </form>
  )
}

function HomePage() {
  return (
    <main id="content" className="shell home">
      <h1 className="brand home-brand"><img className="logo" src="/favicon.svg" alt="" />dok3search</h1>
      <SearchForm />
      <nav className="category-filters" aria-label="문서 카테고리">
        <a href="/search">전체</a>
        {categories.map((item) => <a key={item.id} href={searchUrl('', item.id)}>{item.label}</a>)}
      </nav>
    </main>
  )
}

function SearchPage() {
  const params = new URLSearchParams(window.location.search)
  const query = params.get('q')?.trim() ?? ''
  const rawCategory = params.get('category') ?? ''
  const legacyTopic = isReferenceTopic(rawCategory) ? rawCategory : ''
  const activeCategory = categories.find((item) => item.id === rawCategory) ??
    (legacyTopic ? categories.find((item) => item.id === 'guide') : undefined)
  const category = activeCategory?.id ?? ''
  const requestedTopic = params.get('topic') ?? legacyTopic
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

  function documentUrl(id: string, page: number) {
    return searchUrl(query, category, page, topic).replace('/search', '/docs/' + id)
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
                    <a className="document" href={documentUrl(hit.documentId, page)}>
                      <span className="file-icon"><FileText size={18} aria-hidden="true" /></span>
                      <span className="doc-copy">
                        <small>
                          {hit.topic ? `${topics.find((item) => item.id === hit.topic)?.label} · ${hit.source} · ${hit.tag}` : categoryLabels.get(hit.category as typeof categories[number]['id']) ?? hit.source}
                          {hit.hasPairedFormats && ' · Markdown · YAML'}
                        </small>
                        <strong>{hit.title}</strong>
                        <span className="snippet">{hit.snippet}</span>
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
                          <a href={documentUrl(hit.documentId, page)}>문서 열기</a>
                          <button type="button" onClick={() => void copyTitle(hit.title)}>제목 복사</button>
                        </div>
                      )}
                    </div>
                  </li>
                ))}</ul> : <p className="empty">{!category && hasReferenceMatches
                  ? '일치하는 Rust·Go 자료는 개발 가이드의 세부 분류에서 열 수 있습니다.'
                  : '검색 조건에 맞는 문서가 없습니다.'}</p>}
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
      </footer>
      <div className="visually-hidden" role="status">{copyStatus}</div>
    </>
  )
}

const path = window.location.pathname
const detail = /^\/docs\/([a-z0-9-]+)\/?$/.exec(path)
const legacyIds: Record<string, string> = {
  'design-design-md': 'design-index-md',
  'design-design-yaml': 'design-map-yaml',
}
const detailId = detail ? legacyIds[detail[1]] ?? detail[1] : undefined

const rootElement = document.getElementById('root')
if (rootElement) {
  const root = createRoot(rootElement)
  if (detailId) {
    void Promise.all([import('./document-reader'), loadReferenceCorpora(detailId)]).then(
      ([{ DocumentPage }, [corpus]]) => root.render(<DocumentPage id={detailId} corpus={corpus} />),
      () => root.render(<main className="shell reader"><p role="alert">문서를 불러올 수 없습니다.</p></main>),
    )
  } else {
    root.render(path === '/search' ? <SearchPage /> : <HomePage />)
  }
}
