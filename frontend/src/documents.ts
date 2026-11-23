import { parse } from 'yaml'
import designIndex from './samples/design/index.yaml?raw'
import manualSource from '../../docs/manual/dok3search-manual.md?raw'

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

export function isReferenceTopicFilter(topic: string): topic is ReferenceTopic {
  return topics.some((item) => item.id === topic)
}

function matchesTerm(text: string, term: string, category: string) {
  if (
    text.includes(term) || (
      isReferenceTopic(category) &&
      searchAliases[category].get(term)?.some((alias) => text.includes(alias))
    )
  ) return true
  if (/^[가-힣]{4,}$/.test(term)) {
    const compactText = text.replace(/\s+/g, '').replace(/및/g, '')
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

const designFiles = import.meta.glob<string>('./samples/design/**/*.{md,yaml,yml}', {
  eager: true, import: 'default', query: '?raw',
})

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null
}

const designManifest: unknown = parse(designIndex)
const designGroups: unknown[] = isRecord(designManifest) && Array.isArray(designManifest.document_tree)
  ? designManifest.document_tree
  : []
const designPaths = designGroups.flatMap((group) => {
  if (!isRecord(group) || !Array.isArray(group.files)) return []
  return group.files.filter((path): path is string => typeof path === 'string')
})
const designOrder = new Map(designPaths.map((path, index) => [path, index]))

type DesignPair = { id: string; title: string; markdown: string; yaml: string }
const rawPairs: unknown[] = isRecord(designManifest) && Array.isArray(designManifest.entrypoints)
  ? designManifest.entrypoints
  : []
const designPairs = rawPairs.filter((entry): entry is DesignPair =>
  isRecord(entry) && typeof entry.id === 'string' && typeof entry.title === 'string' &&
  typeof entry.markdown === 'string' && typeof entry.yaml === 'string',
)

export const designDocuments: Document[] = Object.entries(designFiles).map(([path, content]): Document => {
  const samplePath = path.slice('./samples/design/'.length)
  const isMarkdown = samplePath.endsWith('.md')
  const isArchitecture = samplePath === 'dok3search-final-architecture.md'
  const pair = designPairs.find(({ markdown, yaml }) => samplePath === markdown || samplePath === yaml)
  const heading = /^# ([^\r\n]+)\r?\n/.exec(content)
  const markdown = isMarkdown
    ? content.replace(/^# [^\r\n]+\r?\n+/, '')
    : '```yaml\n' + content.trimEnd() + '\n```'
  const firstParagraph = markdown.split(/\r?\n\s*\r?\n/).find((part) => /^[\p{L}\p{N}]/u.test(part))
  const summaryByFile: Record<string, string> = {
    'index.md': '설계 개요와 자료를 읽는 순서입니다.',
    'index.yaml': '설계 문서의 경로, 그룹, 진입점을 기록한 파일입니다.',
    'map.md': '화면, 구성요소, 계약, 운영 절차가 어떻게 연결되는지 보여 주는 설계 문서입니다.',
    'map.yaml': '항목 ID, 관계 유형, 근거 문서와 절을 구조화한 설계 맵입니다.',
    'dok3search-final-architecture.md': '색인과 검색, 문서 열람, API 연동, 장애 복구를 다루는 dok3search 최종 설계입니다.',
  }
  const summary = summaryByFile[samplePath] ??
    firstParagraph?.replace(/[`*]/g, '').replace(/\s+/g, ' ').slice(0, 180) ??
    (isMarkdown ? '' : '설계 저장소의 YAML 예시입니다.')

  return {
    id: isArchitecture ? 'final-architecture' : 'design-' + samplePath.replace(/[^a-zA-Z0-9]+/g, '-'),
    title: (pair?.title ?? heading?.[1].replace(/[`*]/g, '') ?? samplePath) + (samplePath.endsWith('-download.md') ? ' (다운로드본)' : ''),
    summary,
    source: 'dok3search-design',
    tag: isArchitecture ? '아키텍처' : isMarkdown ? 'Markdown' : 'YAML',
    date: isArchitecture ? '2026-11-20' : undefined,
    category: 'design',
    body: '',
    markdown,
    samplePath,
  }
}).sort((a, b) => {
  const rank = (doc: Document) => designOrder.get(doc.samplePath ?? '') ?? designPaths.length
  return rank(a) - rank(b) || (a.samplePath ?? '').localeCompare(b.samplePath ?? '')
})

const designFormatsById = new Map<string, { markdown: Document; yaml: Document }>()
for (const pair of designPairs) {
  const markdown = designDocuments.find((doc) => doc.samplePath === pair.markdown)
  const yaml = designDocuments.find((doc) => doc.samplePath === pair.yaml)
  if (!markdown || !yaml) continue

  const formats = { markdown, yaml }
  designFormatsById.set(markdown.id, formats)
  designFormatsById.set(yaml.id, formats)
}

export function documentFormats(id: string) {
  return designFormatsById.get(id)
}

const manualHeading = /^# ([^\r\n]+)\r?\n/.exec(manualSource)

// Mirrors the backend seed (`dok3-search/src/seed.rs`): same ID, title, and H1-stripped Markdown.
const manualDocument: Document = {
  id: 'dok3search-manual',
  title: manualHeading?.[1].trim() ?? 'dok3search 제품·운영 매뉴얼',
  summary: '기동 모드, 환경 변수, HTTP API, MCP 도구, 재색인, 백업과 복구 절차를 정리한 제품·운영 매뉴얼입니다.',
  source: 'dok3search 운영 매뉴얼',
  tag: 'dok3search',
  date: '2026-11-22',
  category: 'manual',
  body: '',
  markdown: manualSource.replace(/^# [^\r\n]+\r?\n+/, ''),
}

export const documents: Document[] = [
  ...designDocuments,
  {
    id: 'index-rollbacks',
    title: '인덱스 세대 전환과 롤백',
    summary: '신규 인덱스를 검증하고 검색 트래픽을 전환하는 운영 절차입니다.',
    source: '운영 가이드', tag: '색인', date: '2026-11-20', category: 'manual',
    body: '현재 세대의 검색을 유지한 채 새 인덱스를 준비합니다. 문서 수와 대표 검색어 결과를 검증한 뒤 활성 세대를 전환합니다. 전환 후 이상이 있으면 이전에 검증된 세대를 다시 활성화합니다.',
  },
  {
    id: 'rust-workspace',
    title: 'Rust 워크스페이스와 검색 계약',
    summary: '검색 요청, 문서 모델, 결과 스냅샷의 경계와 예시를 확인합니다.',
    source: '개발 문서', tag: 'Rust', date: '2026-11-19', category: 'guide',
    topic: 'rust',
    body: '검색 요청은 질의어, 필터, 페이지 크기를 담습니다. 응답의 각 결과는 문서 ID와 섹션을 포함합니다. 화면과 검색 엔진은 이 계약을 사용해 독립적으로 개발할 수 있습니다.',
  },
  {
    id: 'tantivy-query',
    title: 'Tantivy 쿼리와 검색 필드',
    summary: '제목·본문·태그에 검색어를 적용하는 쿼리 구성 예시입니다.',
    source: '검색 엔진 참고 자료', tag: 'Tantivy', date: '2026-11-18', category: 'code',
    body: '정확한 심볼과 문서 제목을 우선 검색합니다. 일반 텍스트 질의는 제목, 본문, 태그를 대상으로 평가하고 결과에 일치 구간을 표시합니다.',
  },
  {
    id: 'ingestion-policy',
    title: '폐쇄망 자료 반입 기준',
    summary: '문서 팩을 반입하기 전에 확인할 출처, 권한, 무결성 기준입니다.',
    source: '자료 관리 기준', tag: '폐쇄망', date: '2026-11-17', category: 'policy',
    body: '반입 목록에 원본 출처와 이용 권한을 기록합니다. 파일 체크섬을 검증하고 승인된 자료만 지정된 내부 저장소에 등록합니다.',
  },
  {
    id: 'worker-guide',
    title: 'Rust 색인 워커 실행 가이드',
    summary: '색인 작업을 시작하고 상태를 확인하는 방법입니다.',
    source: '개발 문서', tag: 'Rust', date: '2026-11-16', category: 'guide',
    topic: 'rust',
    body: '작업을 등록한 뒤 상태 조회로 대기, 실행, 완료 여부를 확인합니다.',
  },
  {
    id: 'incremental-index',
    title: 'Tantivy 증분 색인 점검',
    summary: '변경된 문서를 색인에 반영하는 운영 점검표입니다.',
    source: '운영 가이드', tag: 'Tantivy', date: '2026-11-15', category: 'manual',
    body: '변경 목록을 확인하고 작업이 끝나면 대표 문서를 다시 검색합니다.',
  },
  {
    id: 'search-api',
    title: '문서 검색 API 요청 예시',
    summary: '검색어, 카테고리 필터, 페이지 요청 예시입니다.',
    source: 'API 계약', tag: 'Tantivy', date: '2026-11-14', category: 'code',
    body: '페이지 크기와 조회 위치를 전달하고 응답에 포함된 문서 식별자를 사용합니다.',
  },
  {
    id: 'document-visibility',
    title: '문서 가시성과 권한 분류',
    summary: '업무 문서를 등록할 때 접근 범위를 지정하는 기준입니다.',
    source: '자료 관리 기준', tag: '색인', date: '2026-11-13', category: 'policy',
    body: '문서를 분류하고 그에 맞는 열람 그룹을 연결합니다.',
  },
  {
    id: 'test-data',
    title: 'Rust 테스트용 검색 데이터 만들기',
    summary: '검색 검증에 사용할 문서 구성 예시입니다.',
    source: '개발 문서', tag: 'Rust', date: '2026-11-12', category: 'guide',
    topic: 'rust',
    body: '한글과 영문 검색어로 제목 일치와 본문 일치를 각각 검증합니다.',
  },
  {
    id: 'search-fields',
    title: 'Tantivy 검색 필드 설정 예시',
    summary: '제목과 본문에 다른 검색 우선순위를 지정합니다.',
    source: '검색 엔진 참고 자료', tag: 'Tantivy', date: '2026-11-11', category: 'code',
    body: '동일한 검색어가 여러 필드에 등장할 때 제목 일치에 우선순위를 부여합니다.',
  },
  {
    id: 'search-health',
    title: 'Tantivy 검색 서비스 상태 점검',
    summary: '최근 색인과 대표 쿼리로 검색 상태를 확인합니다.',
    source: '운영 가이드', tag: 'Tantivy', date: '2026-11-10', category: 'manual',
    body: '대표 쿼리를 실행해 결과 수와 문서 ID가 예상과 일치하는지 살핍니다.',
  },
  {
    id: 'index-approval',
    title: 'Tantivy 색인 반입 승인 기준',
    summary: '색인용 문서 묶음의 검증과 승인 항목입니다.',
    source: '자료 관리 기준', tag: 'Tantivy', date: '2026-11-09', category: 'policy',
    body: '출처와 버전을 등록하고 반입 목록의 체크섬을 확인합니다.',
  },
  {
    id: 'source-paths',
    title: '문서 수집 경로 설정',
    summary: '수집할 폴더와 제외 규칙을 지정하는 예시입니다.',
    source: '개발 문서', tag: '커넥터', date: '2026-11-08', category: 'guide',
    body: '수집 폴더를 지정하고 임시 파일과 중복 경로를 제외합니다.',
  },
  {
    id: 'offline-verification',
    title: '오프라인 배포 파일 검증',
    summary: '설치 전 배포 파일의 목록과 체크섬을 대조합니다.',
    source: '운영 가이드', tag: '배포', date: '2026-11-07', category: 'manual',
    body: '배포 목록에 적힌 체크섬과 받은 파일의 체크섬을 비교합니다.',
  },
  {
    id: 'search-snapshot',
    title: '검색 결과 스냅샷 계약',
    summary: '페이지 이동 시 사용하는 검색 결과 식별자를 설명합니다.',
    source: 'API 계약', tag: '검색', date: '2026-11-06', category: 'code',
    body: '응답이 반환한 스냅샷 식별자를 다음 페이지 요청에 함께 보냅니다.',
  },
  {
    id: 'retention-policy',
    title: '문서 보존 기간 관리',
    summary: '문서 분류에 따라 보존 기간을 기록하는 방법입니다.',
    source: '자료 관리 기준', tag: '보존', date: '2026-11-05', category: 'policy',
    body: '등록 시 문서 종류와 보존 기한을 기록하고 만료 여부를 정기적으로 확인합니다.',
  },
  {
    id: 'korean-analysis',
    title: '한글 검색어 분석 예시',
    summary: '띄어쓰기와 복합어가 검색에 미치는 영향을 살펴봅니다.',
    source: '개발 문서', tag: '한국어', date: '2026-11-04', category: 'guide',
    body: '같은 뜻의 띄어쓰기 변형을 비교해 제목과 본문에서 어느 구간이 일치하는지 확인합니다.',
  },
  {
    id: 'retry-index',
    title: '색인 실패 작업 재시도',
    summary: '실패한 색인 작업을 재실행하기 전에 확인할 항목입니다.',
    source: '운영 가이드', tag: '색인', date: '2026-11-03', category: 'manual',
    body: '실패한 입력과 오류 사유를 확인한 다음 영향 범위를 좁혀 다시 실행합니다.',
  },
  {
    id: 'section-api',
    title: '문서 섹션 조회 응답 예시',
    summary: '문서 ID와 절 ID로 본문을 읽는 응답 예시입니다.',
    source: 'API 계약', tag: '문서', date: '2026-11-02', category: 'code',
    body: '응답에는 절 제목과 본문, 원본 버전과 출처가 포함됩니다.',
  },
  {
    id: 'source-registration',
    title: '내부 문서 출처 등록 기준',
    summary: '문서에 출처와 소유 부서를 남기는 기준입니다.',
    source: '자료 관리 기준', tag: '출처', date: '2026-11-01', category: 'policy',
    body: '소유 부서와 원본 위치를 기록해 정정이 필요할 때 원문을 찾을 수 있게 합니다.',
  },
  manualDocument,
]

export function searchDocuments(query: string, corpus: readonly Document[] = documents) {
  const terms = query.toLowerCase().split(/\s+/)

  return corpus.filter((doc) => {
    const formats = documentFormats(doc.id)
    if (formats?.yaml === doc) return false
    if (!query) return true

    const variants = formats ? [formats.markdown, formats.yaml] : [doc]
    return variants.some((variant) => {
      const searchable = [variant.title, variant.summary, variant.source, variant.tag, variant.topic, variant.body, variant.markdown, variant.samplePath].join(' ').toLowerCase()
      return terms.every((term) => matchesTerm(searchable, term, variant.topic ?? variant.category))
    })
  })
}
