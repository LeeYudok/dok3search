import { useEffect, useMemo, useRef, useState } from 'react'
import Markdown, { defaultUrlTransform } from 'react-markdown'
import remarkGfm from 'remark-gfm'
import remarkParse from 'remark-parse'
import { unified } from 'unified'
import { toString } from 'mdast-util-to-string'
import { categories, categoryLabels, documentHeadingId, designDocuments, designDocumentUrl, designPairs, documents, searchUrl } from './documents'

const designParser = unified().use(remarkParse).use(remarkGfm)

type DocumentSection = { id: string; label: string; line?: number }

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
  const doc = documents.find((item) => item.id === id)
  const pair = designPairs.find((item) => item.markdown === doc?.samplePath || item.yaml === doc?.samplePath)
  const companionPath = pair && (pair.markdown === doc?.samplePath ? pair.yaml : pair.markdown)
  const reader = useRef<HTMLElement>(null)
  const progress = useRef<HTMLDivElement>(null)
  const [activeSection, setActiveSection] = useState('overview')

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
        {doc ? (
          <>
            <header className="article-header">
              <a className="article-category" href={searchUrl('', doc.category)}>{categoryLabels.get(doc.category)}</a>
              <h1>{doc.title}</h1>
              <p className="article-meta">
                <span>{doc.samplePath ?? doc.source}</span>{doc.date && <time dateTime={doc.date}>{doc.date}</time>}
                {companionPath && <a className="reader-back" href={designDocumentUrl(companionPath)}>{companionPath} 보기</a>}
              </p>
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
                {tocSections.map(({ id, label }) => (
                  <a key={id} href={'#' + id} aria-current={activeSection === id ? 'location' : undefined}>{label}</a>
                ))}
              </nav>
            </div>
          </>
        ) : <h1>문서를 찾을 수 없습니다</h1>}
      </main>
    </>
  )
}
