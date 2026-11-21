import { useEffect, useMemo, useRef, useState } from 'react'
import Markdown, { defaultUrlTransform } from 'react-markdown'
import remarkGfm from 'remark-gfm'
import remarkParse from 'remark-parse'
import { unified } from 'unified'
import { toString } from 'mdast-util-to-string'
import { fetchDocument, type ReaderDocument } from './api'
import { categories, categoryLabels, designDocuments, documentHeadingId, searchUrl } from './documents'

const designParser = unified().use(remarkParse).use(remarkGfm)

type DocumentSection = { id: string; label: string; line?: number }

type ReaderState =
  | { status: 'loading' }
  | { status: 'ready'; doc: ReaderDocument }
  | { status: 'not-found' }
  | { status: 'error'; message: string }

function markdownSections(markdown: string): DocumentSection[] {
  const ids = new Set<string>()
  return designParser.parse(markdown).children.flatMap((node) => {
    if (node.type !== 'heading' || node.depth !== 2) return []
    const label = toString(node)
    const base = documentHeadingId(label)
    let id = base
    for (let suffix = 2; ids.has(id); suffix++) id = base + '-' + suffix
    ids.add(id)
    return [{ id, label, line: node.position?.start.line }]
  })
}

function MarkdownBody({ markdown, sections, samplePath }: {
  markdown: string
  sections: DocumentSection[]
  samplePath?: string
}) {
  return (
    <div className="markdown-body">
      <Markdown
        remarkPlugins={[remarkGfm]}
        skipHtml
        disallowedElements={['img']}
        components={{
          h2: ({ children, node }) => <h2 id={sections.find((entry) => entry.line === node?.position?.start.line)?.id}>{children}</h2>,
          table: ({ children }) => <div className="markdown-table" tabIndex={0}><table>{children}</table></div>,
        }}
        urlTransform={(url) => {
          const safeUrl = defaultUrlTransform(url)
          if (!samplePath || !safeUrl || /^(?:[a-z]+:|#|\/\/)/i.test(safeUrl)) return safeUrl

          const target = new URL(safeUrl, new URL('/' + samplePath, window.location.origin))
          const linkedDoc = designDocuments.find((entry) => '/' + entry.samplePath === target.pathname)
          if (!linkedDoc) return safeUrl
          const heading = target.hash && linkedDoc.markdown
            ? markdownSections(linkedDoc.markdown).find(({ label }) => {
                const slug = label.toLowerCase().replace(/[.,:/]/g, '').replaceAll(' ', '-')
                return '#' + encodeURIComponent(slug) === target.hash
              })
            : undefined
          const section = heading ? '#' + encodeURIComponent(heading.id) : target.hash
          return '/docs/' + linkedDoc.id + '?category=design' + section
        }}
      >{markdown}</Markdown>
    </div>
  )
}

export function DocumentPage({ id }: { id: string }) {
  const [readerState, setReaderState] = useState<ReaderState>({ status: 'loading' })
  const reader = useRef<HTMLElement>(null)
  const progress = useRef<HTMLDivElement>(null)
  const [activeSection, setActiveSection] = useState('overview')

  useEffect(() => {
    const controller = new AbortController()
    setReaderState({ status: 'loading' })

    void fetchDocument(id, controller.signal).then(
      ({ doc }) => {
        if (controller.signal.aborted) return
        if (!doc) {
          document.title = '문서를 찾을 수 없습니다 · dok3search'
          setReaderState({ status: 'not-found' })
          return
        }
        document.title = doc.title + ' · dok3search'
        setReaderState({ status: 'ready', doc })
      },
      (error: unknown) => {
        if (controller.signal.aborted) return
        if (error instanceof DOMException && error.name === 'AbortError') return
        const message = error instanceof Error ? error.message : '문서를 불러오지 못했습니다.'
        document.title = '문서 오류 · dok3search'
        setReaderState({ status: 'error', message })
      },
    )

    return () => controller.abort()
  }, [id])

  const doc = readerState.status === 'ready' ? readerState.doc : null

  useEffect(() => {
    if (!doc || !reader.current || !progress.current) return

    const article = reader.current
    const bar = progress.current
    const toc = article.querySelector<HTMLElement>('.article-toc')
    const tocLinks = [...(toc?.querySelectorAll<HTMLAnchorElement>('a') ?? [])]
    let frame = 0
    let previousSection = ''

    function update() {
      frame = 0
      const maxScroll = document.documentElement.scrollHeight - window.innerHeight
      const ratio = maxScroll > 0 ? Math.min(1, Math.max(0, window.scrollY / maxScroll)) : 1
      bar.style.transform = 'scaleX(' + ratio + ')'

      const headings = [...article.querySelectorAll<HTMLElement>('.article-copy h2[id]')]
      const offset = 110
      let current = headings[0]
      for (const heading of headings) {
        if (heading.getBoundingClientRect().top > offset) break
        current = heading
      }
      if (maxScroll > 0 && ratio >= .999) current = headings.at(-1) ?? current
      const section = current?.id ?? 'overview'
      if (section === previousSection) return

      previousSection = section
      setActiveSection(section)
      const link = tocLinks.find((item) => item.getAttribute('href') === '#' + section)
      if (toc && link && getComputedStyle(toc).position === 'sticky') {
        const panel = toc.getBoundingClientRect()
        const item = link.getBoundingClientRect()
        if (item.top < panel.top) toc.scrollTop += item.top - panel.top
        if (item.bottom > panel.bottom) toc.scrollTop += item.bottom - panel.bottom
      }
    }

    function scheduleUpdate() {
      if (!frame) frame = window.requestAnimationFrame(update)
    }

    const observer = new ResizeObserver(scheduleUpdate)
    observer.observe(article)
    window.addEventListener('scroll', scheduleUpdate, { passive: true })
    window.addEventListener('resize', scheduleUpdate)
    window.addEventListener('hashchange', scheduleUpdate)
    try {
      const anchor = decodeURIComponent(window.location.hash.slice(1))
      const target = anchor && document.getElementById(anchor)
      if (target && article.contains(target)) target.scrollIntoView()
    } catch {
      // Leave the reader at its normal position for malformed fragments.
    }
    scheduleUpdate()

    return () => {
      observer.disconnect()
      window.removeEventListener('scroll', scheduleUpdate)
      window.removeEventListener('resize', scheduleUpdate)
      window.removeEventListener('hashchange', scheduleUpdate)
      if (frame) window.cancelAnimationFrame(frame)
    }
  }, [doc])

  const sections = useMemo(() => doc?.markdown ? markdownSections(doc.markdown) : [{ label: '본문', id: 'body' }], [doc])
  const fallbackBody = Boolean(doc?.markdown && sections.length === 0)
  const tocSections = [
    ...(doc?.summary ? [{ id: 'overview', label: '요약' }] : []),
    ...(fallbackBody ? [{ id: 'body', label: '본문' }] : sections),
  ]
  const params = new URLSearchParams(window.location.search)
  const category = categories.find((item) => item.id === params.get('category'))?.id
  const page = Number(params.get('page'))
  const backToResults = searchUrl(params.get('q') ?? '', category, Number.isSafeInteger(page) ? page : 1)

  return (
    <>
      <a className="skip" href="#content">본문으로 이동</a>
      <header className="top">
        <div className="shell reader-top">
          <a className="brand" href="/" aria-label="dok3search 홈"><img className="logo" src="/favicon.svg" alt="" />dok3search</a>
          <a className="reader-back" href={backToResults}>검색 결과로 돌아가기</a>
        </div>
        {doc && <div className="read-progress" aria-hidden="true"><div ref={progress} /></div>}
      </header>
      <main id="content" className="shell reader" ref={reader}>
        {readerState.status === 'loading' && (
          <p className="empty" role="status">문서를 불러오는 중입니다.</p>
        )}
        {readerState.status === 'error' && (
          <p className="empty" role="alert">{readerState.message}</p>
        )}
        {readerState.status === 'not-found' && (
          <h1>문서를 찾을 수 없습니다</h1>
        )}
        {doc && (
          <>
            <header className="article-header">
              <a className="article-category" href={searchUrl('', doc.category)}>
                {categoryLabels.get(doc.category as typeof categories[number]['id']) ?? doc.category}
              </a>
              <h1>{doc.title}</h1>
              <p className="article-meta">
                <span>{doc.samplePath ?? doc.source}</span>{doc.date && <time dateTime={doc.date}>{doc.date}</time>}
              </p>
              {doc.formats && (
                <nav className="category-filters article-formats" aria-label="문서 형식">
                  {[
                    { id: doc.formats.markdownId, label: 'Markdown' },
                    { id: doc.formats.yamlId, label: 'YAML' },
                  ].map((format) => (
                    <a key={format.id} href={'/docs/' + format.id + window.location.search} aria-current={format.id === doc.id ? 'page' : undefined}>
                      {format.label}
                    </a>
                  ))}
                </nav>
              )}
            </header>
            <div className="article-layout">
              <article className="article-copy">
                {doc.summary && <section aria-labelledby="overview">
                  <h2 id="overview">요약</h2>
                  <p>{doc.summary}</p>
                </section>}
                {doc.markdown ? (
                  <section>
                    {fallbackBody && <h2 id="body">본문</h2>}
                    <MarkdownBody markdown={doc.markdown} sections={sections} samplePath={doc.samplePath} />
                  </section>
                ) : (
                  <section aria-labelledby="body">
                    <h2 id="body">본문</h2>
                    <p>{doc.body}</p>
                  </section>
                )}
                <a className="article-tag" href={searchUrl(doc.tag)}>{doc.tag}</a>
              </article>
              <nav className="article-toc" aria-label="이 문서의 목차">
                <strong>목차</strong>
                {tocSections.map(({ id: secId, label }) => (
                  <a key={secId} href={'#' + secId} aria-current={activeSection === secId ? 'location' : undefined}>{label}</a>
                ))}
              </nav>
            </div>
          </>
        )}
      </main>
    </>
  )
}
