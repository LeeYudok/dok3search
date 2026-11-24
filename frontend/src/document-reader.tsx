import { ArrowCounterClockwise, Check, LockSimple, PencilSimple, SlidersHorizontal, Tag, X } from '@phosphor-icons/react'
import { type FormEvent, useEffect, useMemo, useRef, useState } from 'react'
import Markdown, { defaultUrlTransform } from 'react-markdown'
import remarkGfm from 'remark-gfm'
import remarkParse from 'remark-parse'
import { unified } from 'unified'
import { toString } from 'mdast-util-to-string'
import {
  ApiRequestError,
  type DocumentClassification,
  fetchClassification,
  fetchDocument,
  isSourceCodePath,
  type ReaderDocument,
  updateClassification,
} from './api'
import { CodeDocumentViewer, parseSymbolOutlineItem } from './code-viewer'
import { categories, categoryLabels, documentHeadingId, isReferenceTopic, loadLocalSamples, searchUrl, topics, type Document } from './documents'
import type { ReferenceCorpus } from './reference-corpus'

const designParser = unified().use(remarkParse).use(remarkGfm)

type DocumentSection = { id: string; label: string; line?: number; depth?: number; aliases?: string[] }

type MarkdownNode = {
  type: string
  tagName?: string
  url?: string
  properties?: { href?: unknown }
  position?: { start?: { line?: number } }
  children?: readonly MarkdownNode[]
}

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

function grammarAnchorForNode(node: MarkdownNode | null | undefined) {
  const link = node?.children?.find((child) => {
    if (child.type !== 'link' && !(child.type === 'element' && child.tagName === 'a')) return false
    const href = child.url ?? (typeof child.properties?.href === 'string' ? child.properties.href : undefined)
    return Boolean(href?.startsWith('#railroad-'))
  })
  const href = link?.url ?? (typeof link?.properties?.href === 'string' ? link.properties.href : undefined)
  if (!href) return undefined
  return 'grammar-' + href.slice('#railroad-'.length)
}

function markdownBlockAnchors(markdown: string) {
  const ids = new Set<string>()
  const byLine = new Map<number, string>()
  for (const node of designParser.parse(markdown).children as MarkdownNode[]) {
    const anchor = grammarAnchorForNode(node)
    if (!anchor) continue
    ids.add(anchor)
    const line = node.position?.start?.line
    if (line) byLine.set(line, anchor)
  }
  return { ids, byLine }
}

function decodedHash(hash: string) {
  try {
    return decodeURIComponent(hash.slice(1))
  } catch {
    return hash.slice(1)
  }
}

function sectionIdForHash(
  sections: readonly DocumentSection[],
  hash: string,
  corpus?: ReferenceCorpus,
  blockAnchorIds?: ReadonlySet<string>,
) {
  const raw = decodedHash(hash)
  const railroadAnchor = raw.startsWith('railroad-') ? 'grammar-' + raw.slice('railroad-'.length) : undefined
  if (railroadAnchor && blockAnchorIds?.has(railroadAnchor)) return railroadAnchor
  for (const id of blockAnchorIds ?? []) {
    if (id === raw || '#' + encodeURIComponent(id) === hash) return id
  }
  const punctuation = corpus ? /[^\p{L}\p{N}_ -]/gu : /[.,:/]/g
  return sections.find(({ label, id, aliases }) => {
    const slug = label.toLowerCase().replace(punctuation, '').replaceAll(' ', '-')
    return aliases?.includes(hash) || aliases?.includes('#' + raw) ||
      '#' + encodeURIComponent(slug) === hash || '#' + encodeURIComponent(id) === hash
  })?.id
}

function MarkdownBody({ markdown, sections, samplePath, corpus, corpora, designDocs = [] }: {
  markdown: string
  sections: DocumentSection[]
  samplePath?: string
  corpus?: ReferenceCorpus
  corpora?: readonly ReferenceCorpus[]
  designDocs?: readonly Document[]
}) {
  function headingId(line?: number) {
    return sections.find((entry) => entry.line === line)?.id
  }
  const referenceDocuments = corpora?.flatMap((item) => item.documents.map((doc) => ({ corpus: item, doc }))) ?? []
  const blockAnchors = markdownBlockAnchors(markdown)

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
          p: ({ children, node }) => <p id={blockAnchors.byLine.get(node?.position?.start?.line ?? 0)}>{children}</p>,
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
            const sectionId = sectionIdForHash(sections, safeUrl, corpus, blockAnchors.ids)
            return sectionId ? '#' + encodeURIComponent(sectionId) : ''
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
            designDocs.find((entry) => '/' + entry.samplePath === target.pathname)
          if (!linkedDoc) return ''
          const linkedCorpus = referenceMatch?.corpus ?? corpus
          const sectionId = target.hash && linkedDoc.markdown
            ? sectionIdForHash(
                markdownSections(linkedDoc.markdown, Boolean(linkedCorpus)),
                target.hash,
                linkedCorpus,
                markdownBlockAnchors(linkedDoc.markdown).ids,
              )
            : undefined
          const section = sectionId ? '#' + encodeURIComponent(sectionId) : ''
          if (safeUrl.startsWith('#')) return section
          const params = new URLSearchParams({ category: linkedDoc.category })
          if (linkedDoc.topic) params.set('topic', linkedDoc.topic)
          return '/docs/' + linkedDoc.id + '?' + params + section
        }}
      >{markdown}</Markdown>
    </div>
  )
}

const MAX_CLASSIFICATION_SUMMARY_CHARS = 200

function parseCommaSeparatedTerms(raw: string): string[] {
  const terms: string[] = []
  const seen = new Set<string>()
  for (const part of raw.split(',')) {
    const trimmed = part.trim()
    if (!trimmed) continue
    const lowered = trimmed.toLowerCase()
    if (seen.has(lowered)) continue
    seen.add(lowered)
    terms.push(trimmed)
  }
  return terms
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

  const [classification, setClassification] = useState<DocumentClassification | null>(null)
  const [editingClassification, setEditingClassification] = useState(false)
  const [clsCategory, setClsCategory] = useState('guide')
  const [clsSummary, setClsSummary] = useState('')
  const [clsTags, setClsTags] = useState('')
  const [clsKeywords, setClsKeywords] = useState('')
  const [clsSaving, setClsSaving] = useState(false)
  const [clsError, setClsError] = useState<string | null>(null)
  const [clsNotice, setClsNotice] = useState<string | null>(null)
  const [designDocs, setDesignDocs] = useState<readonly Document[]>([])

  useEffect(() => {
    if (corpora?.length) return
    let active = true
    void loadLocalSamples().then(({ designDocuments }) => {
      if (active) setDesignDocs(designDocuments)
    }, () => {})
    return () => { active = false }
  }, [corpora])

  function syncDraftFromClassification(next: DocumentClassification) {
    setClsCategory(next.category)
    setClsSummary(next.summary)
    setClsTags(next.tags.join(', '))
    setClsKeywords(next.keywords.join(', '))
  }

  useEffect(() => {
    if (corpora?.length) {
      document.title = (localDoc?.title ?? '문서를 찾을 수 없습니다') + ' · dok3search'
      setClassification(null)
      setEditingClassification(false)
      setClsError(null)
      setClsNotice(null)
      return
    }

    const controller = new AbortController()
    setApiState({ status: 'loading' })
    setClassification(null)
    setEditingClassification(false)
    setClsError(null)
    setClsNotice(null)

    void fetchDocument(id, controller.signal).then(
      ({ doc, mode }) => {
        if (controller.signal.aborted) return
        if (!doc) {
          document.title = '문서를 찾을 수 없습니다 · dok3search'
          setApiState({ status: 'not-found' })
          return
        }
        document.title = doc.title + ' · dok3search'
        setApiState({ status: 'ready', doc })

        if (mode === 'api') {
          void fetchClassification(doc.id, controller.signal).then(
            (cls) => {
              if (controller.signal.aborted || !cls) return
              setClassification(cls)
              syncDraftFromClassification(cls)
            },
            () => {
              // Ignore optional classification load errors so reading remains uninterrupted.
            },
          )
        }
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

  function applyUpdatedClassification(updated: DocumentClassification) {
    setClassification(updated)
    syncDraftFromClassification(updated)
    setEditingClassification(false)
    setClsError(null)
    setClsNotice(updated.manualLocked ? '수동 분류를 저장했습니다.' : '규칙 자동 분류로 되돌렸습니다.')
    setApiState((prev) => {
      if (prev.status !== 'ready') return prev
      return {
        status: 'ready',
        doc: {
          ...prev.doc,
          category: updated.category,
          summary: updated.summary,
          tag: updated.tags[0] ?? prev.doc.tag,
          tags: updated.tags,
          keywords: updated.keywords,
        },
      }
    })
  }

  function formatClassificationError(err: unknown): string {
    if (err instanceof ApiRequestError && err.status === 403) {
      return '관리자 권한이 필요합니다. 서버를 DOK3_ENABLE_ADMIN=true 환경 변수로 실행한 뒤 다시 시도하세요.'
    }
    return err instanceof Error ? err.message : '분류 수정 중 오류가 발생했습니다.'
  }

  async function handleSaveClassification(event: FormEvent) {
    event.preventDefault()
    if (!classification) return
    setClsSaving(true)
    setClsError(null)
    setClsNotice(null)
    try {
      const updated = await updateClassification(classification.documentId, {
        category: clsCategory,
        summary: clsSummary,
        tags: parseCommaSeparatedTerms(clsTags),
        keywords: parseCommaSeparatedTerms(clsKeywords),
      })
      applyUpdatedClassification(updated)
    } catch (err) {
      setClsError(formatClassificationError(err))
    } finally {
      setClsSaving(false)
    }
  }

  async function handleResetClassification() {
    if (!classification) return
    setClsSaving(true)
    setClsError(null)
    setClsNotice(null)
    try {
      const updated = await updateClassification(classification.documentId, { resetToRule: true })
      applyUpdatedClassification(updated)
    } catch (err) {
      setClsError(formatClassificationError(err))
    } finally {
      setClsSaving(false)
    }
  }

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

      const headings = [...article.querySelectorAll<HTMLElement>('.article-copy h2[id], .article-copy [data-symbol-anchor]')]
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

  const isCodeDoc = !corpus && Boolean(doc && isSourceCodePath(doc.samplePath) && doc.body.trim())
  const codeOutline = useMemo(() => {
    if (!isCodeDoc || !doc?.sections?.length) return []
    return doc.sections
      .map(parseSymbolOutlineItem)
      .filter((item): item is NonNullable<typeof item> => item !== null)
  }, [isCodeDoc, doc])

  const sections = useMemo(
    () => doc?.markdown ? markdownSections(doc.markdown, Boolean(corpus)) : [{ label: '본문', id: 'body' }],
    [doc, corpus],
  )
  const mainSections = sections.filter(({ depth }) => depth === undefined || depth === 2)
  const fallbackBody = Boolean(doc?.markdown && mainSections.length === 0)
  const readerSummary = corpus || isCodeDoc ? '' : doc?.summary
  const tocSections = [
    ...(readerSummary ? [{ id: 'overview', label: '요약' }] : []),
    ...(fallbackBody ? [{ id: 'body', label: '본문' }] : mainSections),
  ]
  const params = new URLSearchParams(window.location.search)
  const rawCategory = params.get('category') ?? ''
  const legacyTopic = isReferenceTopic(rawCategory) ? rawCategory : ''
  const category = categories.find((item) => item.id === rawCategory)?.id ?? (legacyTopic ? 'guide' : undefined)
  const requestedTopic = params.get('topic')?.trim() || legacyTopic
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
      <main id="content" className={isCodeDoc ? 'shell reader reader-code' : 'shell reader'} ref={reader}>
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
              <div className="article-eyebrow">
                <a className="article-category" href={searchUrl('', doc.category, 1, doc.topic)}>
                  {categoryLabels.get(doc.category as typeof categories[number]['id']) ?? doc.category}
                </a>
                {doc.topic && <a className="article-tag" href={searchUrl('', 'guide', 1, doc.topic)}>
                  {topics.find((item) => item.id === doc.topic)?.label}
                </a>}
                {classification && !corpus && (
                  <>
                    <span className={classification.manualLocked ? 'cls-badge cls-badge-manual' : 'cls-badge'}>
                      {classification.manualLocked
                        ? <LockSimple size={13} weight="bold" aria-hidden="true" />
                        : <SlidersHorizontal size={13} aria-hidden="true" />}
                      {classification.manualLocked
                        ? '수동 수정 유지됨'
                        : classification.source === 'agent'
                          ? 'AI 자동 분류'
                          : '규칙 자동 분류'}
                    </span>
                    <button
                      type="button"
                      className="cls-toggle-button"
                      aria-expanded={editingClassification}
                      onClick={() => {
                        setClsError(null)
                        setClsNotice(null)
                        syncDraftFromClassification(classification)
                        setEditingClassification((prev) => !prev)
                      }}
                    >
                      <PencilSimple size={13} aria-hidden="true" />
                      {editingClassification ? '편집 닫기' : '분류 편집'}
                    </button>
                  </>
                )}
              </div>
              <h1>{doc.title}</h1>
              <p className="article-meta">
                <span>{corpus ? doc.source : doc.samplePath ?? doc.source}</span>{doc.date && <time dateTime={doc.date}>{doc.date}</time>}
                {corpus?.licenses.map(({ label, url }) => <a href={url} key={label}>{label}</a>)}
                {!corpus && <a href={'/api/v1/documents/' + encodeURIComponent(doc.id) + '/original'} download>원본 내려받기</a>}
              </p>
              {clsNotice && !editingClassification && (
                <p className="cls-notice" role="status">{clsNotice}</p>
              )}
              {editingClassification && classification && !corpus && (
                <section className="cls-panel" aria-label="문서 분류 편집">
                  <form className="cls-form" onSubmit={(event) => void handleSaveClassification(event)}>
                    <div className="cls-field">
                      <label htmlFor="cls-category">카테고리</label>
                      <select
                        id="cls-category"
                        value={clsCategory}
                        disabled={clsSaving}
                        onChange={(event) => setClsCategory(event.target.value)}
                      >
                        {categories.map((item) => (
                          <option key={item.id} value={item.id}>{item.label} ({item.id})</option>
                        ))}
                      </select>
                    </div>
                    <div className="cls-field">
                      <label htmlFor="cls-summary">
                        요약 <span>({clsSummary.length}/{MAX_CLASSIFICATION_SUMMARY_CHARS})</span>
                      </label>
                      <textarea
                        id="cls-summary"
                        rows={3}
                        maxLength={MAX_CLASSIFICATION_SUMMARY_CHARS}
                        value={clsSummary}
                        disabled={clsSaving}
                        onChange={(event) => setClsSummary(event.target.value)}
                      />
                    </div>
                    <div className="cls-field-row">
                      <div className="cls-field">
                        <label htmlFor="cls-tags">태그 (쉼표 구분, 최대 16개)</label>
                        <input
                          id="cls-tags"
                          type="text"
                          value={clsTags}
                          disabled={clsSaving}
                          onChange={(event) => setClsTags(event.target.value)}
                        />
                      </div>
                      <div className="cls-field">
                        <label htmlFor="cls-keywords">키워드 (쉼표 구분, 최대 16개)</label>
                        <input
                          id="cls-keywords"
                          type="text"
                          value={clsKeywords}
                          disabled={clsSaving}
                          onChange={(event) => setClsKeywords(event.target.value)}
                        />
                      </div>
                    </div>
                    {clsError && <p className="cls-error" role="alert">{clsError}</p>}
                    <div className="cls-actions">
                      <button type="submit" className="cls-btn-primary" disabled={clsSaving}>
                        <Check size={14} weight="bold" aria-hidden="true" />
                        {clsSaving ? '저장 중...' : '저장'}
                      </button>
                      <button
                        type="button"
                        className="cls-btn-secondary"
                        disabled={clsSaving || !classification.manualLocked}
                        onClick={() => void handleResetClassification()}
                      >
                        <ArrowCounterClockwise size={14} aria-hidden="true" />
                        규칙 재분류
                      </button>
                      <button
                        type="button"
                        className="cls-btn-secondary"
                        disabled={clsSaving}
                        onClick={() => {
                          syncDraftFromClassification(classification)
                          setEditingClassification(false)
                          setClsError(null)
                        }}
                      >
                        <X size={14} aria-hidden="true" />
                        취소
                      </button>
                    </div>
                  </form>
                </section>
              )}
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
                {isCodeDoc && doc ? (
                  <CodeDocumentViewer
                    body={doc.body}
                    samplePath={doc.samplePath}
                    sections={doc.sections ?? []}
                    activeAnchor={activeSection}
                    markdownFallbackRender={() =>
                      doc.markdown ? (
                        <MarkdownBody markdown={doc.markdown} sections={sections} samplePath={doc.samplePath} corpus={corpus} corpora={corpora} designDocs={designDocs} />
                      ) : (
                        <pre><code>{doc.body}</code></pre>
                      )
                    }
                  />
                ) : doc.markdown ? (
                  <section>
                    {fallbackBody && <h2 id="body">본문</h2>}
                    <MarkdownBody markdown={doc.markdown} sections={sections} samplePath={doc.samplePath} corpus={corpus} corpora={corpora} designDocs={designDocs} />
                  </section>
                ) : (
                  <section aria-labelledby="body">
                    <h2 id="body">본문</h2>
                    <p>{doc.body}</p>
                  </section>
                )}
                <div className="article-terms">
                  {(doc.tags?.length ? doc.tags : [doc.tag]).map((tag) => (
                    <a className="article-tag" key={'tag-' + tag} href={searchUrl(tag)}>
                      <Tag size={12} aria-hidden="true" />
                      {tag}
                    </a>
                  ))}
                  {(doc.keywords ?? []).map((kw) => (
                    <a className="article-keyword" key={'kw-' + kw} href={searchUrl(kw)}>{kw}</a>
                  ))}
                </div>
              </article>
              <nav className="article-toc" aria-label={isCodeDoc ? '심볼 목차' : '이 문서의 목차'}>
                <strong>{isCodeDoc ? '심볼' : '목차'}</strong>
                {isCodeDoc && codeOutline.length > 0
                  ? codeOutline.map((sym) => (
                      <a
                        key={sym.id}
                        href={'#' + sym.id}
                        className="toc-sym-row"
                        aria-current={activeSection === sym.id ? 'location' : undefined}
                      >
                        <span className="toc-sym-kind">{sym.kind}</span>
                        <span className="toc-sym-name">{sym.name}</span>
                        <span className="toc-sym-line">L{sym.lineStart}</span>
                      </a>
                    ))
                  : tocSections.map(({ id: secId, label }) => (
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
