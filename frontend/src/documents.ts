export const categories = [
  { id: 'guide', label: '개발 가이드' },
  { id: 'manual', label: '운영 매뉴얼' },
  { id: 'code', label: '코드 레퍼런스' },
  { id: 'policy', label: '내부 규정' },
  { id: 'design', label: 'dok3search-design' },
] as const

export const topics = [
  { id: 'rust', label: 'Rust' },
  { id: 'go', label: 'Go' },
] as const

export type ReferenceTopic = typeof topics[number]['id']

export const categoryLabels = new Map(categories.map(({ id, label }) => [id, label]))

export function searchUrl(query: string, category = '', page = 1, topic = '') {
  const params = new URLSearchParams()
  if (query.trim()) params.set('q', query.trim())
  if (category) params.set('category', category)
  if (category === 'guide' && topics.some((item) => item.id === topic)) params.set('topic', topic)
  if (page > 1) params.set('page', String(page))
  return params.size ? '/search?' + params : '/search'
}

export function documentHeadingId(label: string) {
  return 'section-' + label.toLowerCase().replace(/[^\p{L}\p{N}]+/gu, '-').replace(/-$/, '')
}

type Category = typeof categories[number]['id']

const searchAliases = {
  rust: new Map([
    ['빌림', ['대여', 'borrow']],
  ]),
  go: new Map([
    ['고루틴', ['goroutine']],
    ['채널', ['channel']],
    ['동시성', ['concurrency']],
    ['모듈', ['module']],
    ['벤더링', ['vendor']],
    ['폐쇄망', ['offline', 'goproxy', 'go mod vendor']],
    ['오프라인', ['offline', 'goproxy', 'go mod vendor']],
  ]),
}

export function isReferenceTopic(category: string): category is keyof typeof searchAliases {
  return category === 'rust' || category === 'go'
}

export function expandSearchAliases(term: string, topicOrCategory?: string): readonly string[] {
  if (!topicOrCategory || !isReferenceTopic(topicOrCategory)) return []
  return searchAliases[topicOrCategory].get(term) ?? []
}

export function isReferenceTopicFilter(topic: string): topic is ReferenceTopic {
  return topics.some((item) => item.id === topic)
}

export function matchesTerm(text: string, term: string, category: string) {
  if (
    text.includes(term) || (
      isReferenceTopic(category) &&
      searchAliases[category].get(term)?.some((alias) => text.includes(alias))
    )
  ) return true
  if (/^[가-힣]{4,}$/.test(term)) {
    const compactText = text.replace(/[ \t]+/g, '').replace(/및/g, '')
    const compactTerm = term.replace(/및/g, '')
    return compactTerm.length >= 4 && compactText.includes(compactTerm)
  }
  return false
}

export function titleMatches(query: string, doc: Document) {
  const title = doc.title.toLowerCase()
  return query.toLowerCase().split(/\s+/).every((term) => matchesTerm(title, term, doc.topic ?? doc.category))
}

export type Document = {
  id: string
  title: string
  summary: string
  source: string
  tag: string
  date?: string
  category: Category
  topic?: ReferenceTopic
  body: string
  markdown?: string
  samplePath?: string
}

let localSamplesPromise: Promise<typeof import('./local-samples')> | undefined

export function loadLocalSamples() {
  return (localSamplesPromise ??= import('./local-samples'))
}
