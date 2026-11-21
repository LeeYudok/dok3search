import { useEffect, useMemo, useRef, useState } from 'react'
import Markdown, { defaultUrlTransform } from 'react-markdown'
import remarkGfm from 'remark-gfm'
import remarkParse from 'remark-parse'
import { unified } from 'unified'
import { toString } from 'mdast-util-to-string'
import { fetchDocument, type ReaderDocument } from './api'
import { categories, categoryLabels, designDocuments, documentHeadingId, isReferenceTopic, searchUrl, topics, type Document } from './documents'
import type { ReferenceCorpus } from './reference-corpus'

const designParser = unified().use(remarkParse).use(remarkGfm)

type DocumentSection = { id: string; label: string; line?: number; depth?: number; aliases?: string[] }

function isExternalUrl(url: string) {
  try {
    const target = new URL(url, window.location.origin)
    return target.origin !== window.location.origin || (target.protocol !== 'http:' && target.protocol !== 'https:')
  } catch {
    return true
  }
}

type ReaderState =
  | { status: 'loading' }
  | { status: 'ready'; doc: ReaderDocument }
  | { status: 'not-found' }
  | { status: 'error'; message: string }

function markdownSections(markdown: string, includeSubsections = false): DocumentSection[] {
  const ids = new Set<string>()
  const nodes = designParser.parse(markdown).children
  return nodes.flatMap((node, index) => {
    if (node.type !== 'heading' || node.depth === 1 || (node.depth !== 2 && !includeSubsections)) return []
    const label = toString(node)
    const base = documentHeadingId(label)
    let id = base
    for (let suffix = 2; ids.has(id); suffix++) id = base + '-' + suffix
    ids.add(id)
    const previous = nodes[index - 1]
    const ruleLink = previous?.type === 'paragraph' && previous.children.length === 1 && previous.children[0].type === 'link'
      ? previous.children[0].url
      : undefined
    const aliases = ruleLink?.startsWith('#r-') ? [ruleLink] : undefined
    return [{ id, label, line: node.position?.start.line, depth: node.depth, aliases }]
  })
}

function sectionForHash(sections: readonly DocumentSection[], hash: string, corpus?: ReferenceCorpus) {
  const punctuation = corpus ? /[^\p{L}\p{N}_ -]/gu : /[.,:/]/g
  return sections.find(({ label, id, aliases }) => {
    const slug = label.toLowerCase().replace(punctuation, '').replaceAll(' ', '-')
    return aliases?.includes(hash) || '#' + encodeURIComponent(slug) === hash || '#' + encodeURIComponent(id) === hash
  })
}

function MarkdownBody({ markdown, sections, samplePath, corpus, corpora }: {
  markdown: string
  sections: DocumentSection[]
  samplePath?: string
  corpus?: ReferenceCorpus
  corpora?: readonly ReferenceCorpus[]
}) {
  function headingId(line?: number) {
    return sections.find((entry) => entry.line === line)?.id
  }
  const referenceDocuments = corpora?.flatMap((item) => item.documents.map((doc) => ({ corpus: item, doc }))) ?? []

  return (
    <div className="markdown-body">
      <Markdown
        remarkPlugins={[remarkGfm]}
        skipHtml
        disallowedElements={corpus ? [] : ['img']}
        components={{
          h2: ({ children, node }) => <h2 id={headingId(node?.position?.start.line)}>{children}</h2>,
          h3: ({ children, node }) => <h3 id={headingId(node?.position?.start.line)}>{children}</h3>,
          h4: ({ children, node }) => <h4 id={headingId(node?.position?.start.line)}>{children}</h4>,
          h5: ({ children, node }) => <h5 id={headingId(node?.position?.start.line)}>{children}</h5>,
          h6: ({ children, node }) => <h6 id={headingId(node?.position?.start.line)}>{children}</h6>,
          img: ({ src, alt }) => src ? <img src={src} alt={alt ?? ''} loading="lazy" /> : null,
          a: ({ href, title, children }) => !href
            ? <span>{children}</span>
            : isExternalUrl(href)
              ? <span className="external-reference" title={href}>
                {children}<span className="external-reference-label" aria-hidden="true">외부</span>
                <span className="visually-hidden">, 폐쇄망에서 열 수 없는 외부 링크</span>
              </span>
              : <a href={href} title={title}>{children}</a>,
          table: ({ children }) => <div className="markdown-table" tabIndex={0}><table>{children}</table></div>,
        }}
        urlTransform={(url, key) => {
          const safeUrl = defaultUrlTransform(url)
          if (!safeUrl) return ''
          if (isExternalUrl(safeUrl)) return key === 'src' ? '' : safeUrl
          if (safeUrl.startsWith('#')) {
            const section = sectionForHash(sections, safeUrl, corpus)
            return section ? '#' + encodeURIComponent(section.id) : ''
          }
          if (!samplePath) return ''

          const basePath = (corpus ? '/samples/' + corpus.folder : '') + '/' + samplePath
          const target = new URL(safeUrl, new URL(basePath, window.location.origin))
          if (key === 'src') {
            return corpora?.map((item) => item.images.get(target.pathname)).find(Boolean) ?? ''
          }

          const referenceMatch = referenceDocuments.find(({ corpus: item, doc }) =>
            '/samples/' + item.folder + '/' + doc.samplePath === target.pathname,
          )
          const linkedDoc: Document | undefined = referenceMatch?.doc ??
            designDocuments.find((entry) => '/' + entry.samplePath === target.pathname)
          if (!linkedDoc) return ''
          const linkedCorpus = referenceMatch?.corpus ?? corpus
          const heading = target.hash && linkedDoc.markdown
            ? sectionForHash(markdownSections(linkedDoc.markdown, Boolean(linkedCorpus)), target.hash, linkedCorpus)
            : undefined
          const section = heading ? '#' + encodeURIComponent(heading.id) : ''
          if (safeUrl.startsWith('#')) return section
          const params = new URLSearchParams({ category: linkedDoc.category })
          if (linkedDoc.topic) params.set('topic', linkedDoc.topic)
          return '/docs/' + linkedDoc.id + '?' + params + section
        }}
      >{markdown}</Markdown>
    </div>
  )
}

export function DocumentPage({ id, corpora }: { id: string; corpora?: readonly ReferenceCorpus[] }) {
  const corpus = corpora?.find((item) => item.documents.some((doc) => doc.id === id))
  const localDoc = corpus?.documents.find((item) => item.id === id)
  const [apiState, setApiState] = useState<ReaderState>({ status: 'loading' })
  const readerState: ReaderState = corpora?.length
    ? (localDoc ? { status: 'ready', doc: localDoc } : { status: 'not-found' })
    : apiState
  const reader = useRef<HTMLElement>(null)
  const progress = useRef<HTMLDivElement>(null)
  const [activeSection, setActiveSection] = useState('overview')

  useEffect(() => {
    if (corpora?.length) {
      document.title = (localDoc?.title ?? '문서를 찾을 수 없습니다') + ' · dok3search'
      return
    }

    const controller = new AbortController()
    setApiState({ status: 'loading' })

    void fetchDocument(id, controller.signal).then(
      ({ doc }) => {
        if (controller.signal.aborted) return
        if (!doc) {
          document.title = '문서를 찾을 수 없습니다 · dok3search'
          setApiState({ status: 'not-found' })
          return
        }
        document.title = doc.title + ' · dok3search'
        setApiState({ status: 'ready', doc })
      },
      (error: unknown) => {
        if (controller.signal.aborted) return
        if (error instanceof DOMException && error.name === 'AbortError') return
        const message = error instanceof Error ? error.message : '문서를 불러오지 못했습니다.'
        document.title = '문서 오류 · dok3search'
        setApiState({ status: 'error', message })
      },
    )

    return () => controller.abort()
  }, [id, corpora, localDoc])

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

  const sections = useMemo(
    () => doc?.markdown ? markdownSections(doc.markdown, Boolean(corpus)) : [{ label: '본문', id: 'body' }],
    [doc, corpus],
  )
  const mainSections = sections.filter(({ depth }) => depth === undefined || depth === 2)
  const fallbackBody = Boolean(doc?.markdown && mainSections.length === 0)
  const readerSummary = corpus ? '' : doc?.summary
  const tocSections = [
    ...(readerSummary ? [{ id: 'overview', label: '요약' }] : []),
    ...(fallbackBody ? [{ id: 'body', label: '본문' }] : mainSections),
  ]
  const params = new URLSearchParams(window.location.search)
  const rawCategory = params.get('category') ?? ''
  const legacyTopic = isReferenceTopic(rawCategory) ? rawCategory : ''
  const category = categories.find((item) => item.id === rawCategory)?.id ?? (legacyTopic ? 'guide' : undefined)
  const requestedTopic = params.get('topic') ?? legacyTopic
  const topic = topics.find((item) => item.id === requestedTopic)?.id ?? ''
  const page = Number(params.get('page'))
  const backToResults = searchUrl(params.get('q') ?? '', category, Number.isSafeInteger(page) ? page : 1, topic)

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
              <a className="article-category" href={searchUrl('', doc.category, 1, doc.topic)}>
                {categoryLabels.get(doc.category as typeof categories[number]['id']) ?? doc.category}
              </a>
              {doc.topic && <a className="article-tag" href={searchUrl('', 'guide', 1, doc.topic)}>
                {topics.find((item) => item.id === doc.topic)?.label}
              </a>}
              <h1>{doc.title}</h1>
              <p className="article-meta">
                <span>{corpus ? doc.source : doc.samplePath ?? doc.source}</span>{doc.date && <time dateTime={doc.date}>{doc.date}</time>}
                {corpus?.licenses.map(({ label, url }) => <a href={url} key={label}>{label}</a>)}
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
                {readerSummary && <section aria-labelledby="overview">
                  <h2 id="overview">요약</h2>
                  <p>{readerSummary}</p>
                </section>}
                {doc.markdown ? (
                  <section>
                    {fallbackBody && <h2 id="body">본문</h2>}
                    <MarkdownBody markdown={doc.markdown} sections={sections} samplePath={doc.samplePath} corpus={corpus} corpora={corpora} />
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
