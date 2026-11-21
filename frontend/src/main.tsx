import { useState } from 'react'
import { DotsThreeVerticalIcon as More, FileTextIcon as FileText, MagnifyingGlassIcon as Search, XIcon as X } from '@phosphor-icons/react'
import { createRoot } from 'react-dom/client'
import { categories, categoryLabels, documents, searchUrl } from './documents'
import './style.css'

const pageSize = 15
function SearchForm({ query = '', category = '' }: { query?: string; category?: string }) {
  return (
    <form role="search" action="/search">
      <Search size={22} aria-hidden="true" />
      <input type="search" name="q" aria-label="문서 검색어" placeholder="제목, 키워드 또는 코드 검색" defaultValue={query} />
      {category && <input type="hidden" name="category" value={category} />}
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
  const activeCategory = categories.find((item) => item.id === params.get('category'))
  const category = activeCategory?.id ?? ''
  const requestedPage = Number(params.get('page'))
  const [openMenu, setOpenMenu] = useState('')
  const [copyStatus, setCopyStatus] = useState('')
  const terms = query.toLowerCase().split(/\s+/)
  const matches = documents.filter((doc) => {
    const searchable = [doc.title, doc.summary, doc.source, doc.tag, doc.body, doc.markdown, doc.samplePath].join(' ').toLowerCase()
    return terms.every((term) => searchable.includes(term))
  })
  const visible = category ? matches.filter((doc) => doc.category === category) : matches
  const browseDesign = category === 'design' && !query

  const pageCount = Math.ceil(visible.length / pageSize)
  const page = Number.isSafeInteger(requestedPage) ? Math.max(1, Math.min(requestedPage, pageCount)) : 1
  const pageDocuments = visible.slice((page - 1) * pageSize, page * pageSize)

  const firstPage = Math.max(1, Math.min(page - 2, pageCount - 4))
  const lastPage = Math.min(firstPage + 4, pageCount)
  const matchedCategories = new Set(visible.map((doc) => doc.category))
  const relatedSearches = query && visible.length
    ? [...new Set(documents.filter((doc) => matchedCategories.has(doc.category)).map((doc) => doc.tag))]
        .filter((tag) => !query.toLowerCase().includes(tag.toLowerCase()))
        .slice(0, 6)
    : []

  function documentUrl(id: string) {
    return searchUrl(query, category, page).replace('/search', '/docs/' + id)
  }

  async function copyTitle(title: string) {
    try {
      await navigator.clipboard.writeText(title)
      setCopyStatus('문서 제목을 복사했습니다.')
    } catch {
      setCopyStatus('문서 제목을 복사할 수 없습니다.')
    }
  }

  return (
    <>
      <a className="skip" href="#content">본문으로 이동</a>
      <header className="top">
        <div className="shell top-inner">
          <a className="brand" href="/" aria-label="dok3search 홈"><img className="logo" src="/favicon.svg" alt="" />dok3search</a>
          <SearchForm query={query} category={category} />
        </div>
      </header>
      <main id="content" className="has-query">
        <h1 className="visually-hidden">문서 검색</h1>
        <div className="shell library">
          <div className="category-filters" role="group" aria-label="문서 카테고리">
            <a href={searchUrl(query)} aria-current={!category ? 'page' : undefined}>전체 <span className="category-count">{matches.length}</span></a>
            {categories.map((item) => (
              <a key={item.id} href={searchUrl(query, item.id)} aria-current={category === item.id ? 'page' : undefined}>
                {item.label} <span className="category-count">{matches.filter((doc) => doc.category === item.id).length}</span>
              </a>
            ))}
          </div>
          <section id="documents" className="documents" aria-labelledby="documents-heading">
            {query || (activeCategory && !browseDesign) ? (
              <div className="section-heading">
                <h2 id="documents-heading">{query ? '검색 결과' : activeCategory?.label}</h2>
                {query && <a className="text-button" href={searchUrl('', category)}>초기화 <X size={14} /></a>}
              </div>
            ) : <h2 id="documents-heading" className="visually-hidden">검색 결과</h2>}
            <span className="visually-hidden" role="status">문서 {visible.length}개{pageCount > 1 && ' · ' + page + '/' + pageCount + ' 페이지'}</span>
            {visible.length > 0 ? <ul className="document-list">{pageDocuments.map((doc) => (
              <li className="result-row" key={doc.id}>
                <a className="document" href={documentUrl(doc.id)}>
                  <span className="file-icon"><FileText size={18} aria-hidden="true" /></span>
                  <span className="doc-copy">
                    <small>{categoryLabels.get(doc.category) ?? doc.source} · {doc.tag}</small>
                    <strong>{doc.title}</strong>
                    <span className="snippet">{doc.summary} {doc.body}</span>
                  </span>
                </a>
                {doc.date && <time dateTime={doc.date}>{doc.date}</time>}
                <div className="document-actions"
                  onBlur={(event) => {
                    if (!event.currentTarget.contains(event.relatedTarget)) setOpenMenu('')
                  }}
                  onKeyDown={(event) => {
                    if (event.key === 'Escape' && openMenu === doc.title) {
                      event.stopPropagation()
                      event.currentTarget.querySelector('button')?.focus()
                      setOpenMenu('')
                    }
                  }}
                >
                  <button type="button" className="more-button"
                    aria-label={doc.title + ' 옵션'}
                    aria-expanded={openMenu === doc.title}
                    onClick={(event) => {
                      event.currentTarget.focus()
                      setOpenMenu(openMenu === doc.title ? '' : doc.title)
                    }}
                  ><More size={20} weight="bold" aria-hidden="true" /></button>
                  {openMenu === doc.title && (
                    <div className="document-options" onPointerDown={(event) => event.preventDefault()}>
                      <a href={documentUrl(doc.id)}>문서 열기</a>
                      <button type="button" onClick={() => void copyTitle(doc.title)}>제목 복사</button>
                    </div>
                  )}
                </div>
              </li>
            ))}</ul> : <p className="empty">검색 조건에 맞는 샘플 문서가 없습니다.</p>}
            {relatedSearches.length > 0 && (
              <section className="related" aria-labelledby="related-heading">
                <h3 id="related-heading">관련 검색어</h3>
                <ul>
                  {relatedSearches.map((term) => (
                    <li key={term}>
                      <a href={searchUrl(term, category)}>
                        <Search size={16} aria-hidden="true" />{term}
                      </a>
                    </li>
                  ))}
                </ul>
              </section>
            )}
            {pageCount > 1 && (
              <nav className="pagination" aria-label="문서 페이지">
                {page > 1 && <a href={searchUrl(query, category, page - 1)}>이전</a>}
                {firstPage > 1 && <span aria-hidden="true">…</span>}
                {Array.from({ length: lastPage - firstPage + 1 }, (_, index) => firstPage + index).map((number) => (
                  <a key={number} href={searchUrl(query, category, number)} aria-label={number + ' 페이지'} aria-current={number === page ? 'page' : undefined}>{number}</a>
                ))}
                {lastPage < pageCount && <span aria-hidden="true">…</span>}
                {page < pageCount && <a href={searchUrl(query, category, page + 1)}>다음</a>}
              </nav>
            )}
          </section>
        </div>
      </main>
      <footer className="shell footer"><span>샘플 데이터로 구동되는 로컬 미리보기</span></footer>
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
if (detailId) {
  const title = documents.find((doc) => doc.id === detailId)?.title
  document.title = (title ?? '문서를 찾을 수 없습니다') + ' · dok3search'
}

const root = createRoot(document.getElementById('root')!)
if (detailId) {
  void import('./document-reader').then(({ DocumentPage }) => root.render(<DocumentPage id={detailId} />))
} else {
  root.render(path === '/search' ? <SearchPage /> : <HomePage />)
}
