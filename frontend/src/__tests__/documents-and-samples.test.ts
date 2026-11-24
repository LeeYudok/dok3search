import { describe, expect, it } from 'vitest'
import {
  documentHeadingId,
  isReferenceTopic,
  isReferenceTopicFilter,
  matchesTerm,
  searchUrl,
  titleMatches,
} from '../documents'
import {
  documentFormats,
  localFallbackDocument,
  searchDocuments,
  searchLocalDocuments,
} from '../local-samples'
import { createReferenceCorpus, referenceIdPrefix } from '../reference-corpus'

describe('documents helpers', () => {
  it('builds searchUrl with query, category, topic, and page', () => {
    expect(searchUrl('')).toBe('/search')
    expect(searchUrl('  Tantivy  ')).toBe('/search?q=Tantivy')
    expect(searchUrl('rust', 'guide', 2, 'rust')).toBe('/search?q=rust&category=guide&topic=rust&page=2')
    expect(searchUrl('test', 'code', 1, 'rust')).toBe('/search?q=test&category=code')
  })

  it('normalizes documentHeadingId and topic guards', () => {
    expect(documentHeadingId('function toArrayBuffer!')).toBe('section-function-toarraybuffer')
    expect(isReferenceTopic('rust')).toBe(true)
    expect(isReferenceTopic('go')).toBe(true)
    expect(isReferenceTopic('code')).toBe(false)
    expect(isReferenceTopicFilter('go')).toBe(true)
    expect(isReferenceTopicFilter('python')).toBe(false)
  })

  it('matches aliases and unspaced 4+ char Korean compound terms', () => {
    expect(matchesTerm('borrow checker', '빌림', 'rust')).toBe(true)
    expect(matchesTerm('go mod vendor', '폐쇄망', 'go')).toBe(true)
    expect(matchesTerm('인덱스 세대 전환과 롤백', '세대전환', 'manual')).toBe(true)
    expect(titleMatches('세대 전환', {
      id: '1',
      title: '인덱스 세대 전환과 롤백',
      summary: '',
      source: '',
      tag: '',
      category: 'manual',
      body: '',
    })).toBe(true)
  })
})

describe('local-samples search and fallback', () => {
  it('searches local documents and paginates with facets and relatedSearches', () => {
    const hits = searchDocuments('Tantivy')
    expect(hits.length).toBeGreaterThan(0)

    const page = searchLocalDocuments('Tantivy', '', 1, 2)
    expect(page.totalHits).toBeGreaterThan(0)
    expect(page.hits.length).toBeLessThanOrEqual(2)
    expect(page.categoryCounts.length).toBeGreaterThan(0)
    expect(page.snapshot.generationId).toBe('local-samples')
  })

  it('retrieves localFallbackDocument and paired design formats', () => {
    const doc = localFallbackDocument('final-architecture')
    expect(doc).not.toBeNull()
    expect(doc?.category).toBe('design')
    expect(localFallbackDocument('does-not-exist-id')).toBeNull()

    const pair = documentFormats('final-architecture')
    expect(pair === undefined || (pair.markdown.id.length > 0 && pair.yaml.id.length > 0)).toBe(true)
  })
})

describe('reference-corpus builder', () => {
  it('builds a ReferenceCorpus from markdown glob records', () => {
    expect(referenceIdPrefix('rust', 'rust-book')).toBe('rust-ref-rust-book-')
    const corpus = createReferenceCorpus({
      topic: 'rust',
      folder: 'rust-book',
      source: 'The Rust Book',
      language: 'en',
      chapters: [{ path: 'chapters/ch01.md', title: 'Getting Started', summary: 'Hello Rust world.' }],
      markdown: {
        './samples/rust-book/chapters/ch01.md': '# Getting Started\n\nHello Rust world.',
      },
      images: {
        './samples/rust-book/chapters/img/diagram.png': 'data:image/png;base64,AAAA',
      },
      licenses: [{ label: 'MIT', url: '/licenses/MIT' }],
    })
    expect(corpus.documents.length).toBe(1)
    expect(corpus.documents[0].title).toBe('Getting Started')
    expect(corpus.documents[0].category).toBe('guide')
    expect(corpus.images.get('/samples/rust-book/chapters/img/diagram.png')).toBe('data:image/png;base64,AAAA')
  })
})
