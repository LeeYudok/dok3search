import { type Document, type ReferenceTopic } from './documents'

export function referenceIdPrefix(category: 'rust' | 'go', folder: string) {
  return category + '-ref-' + folder + '-'
}

type Chapter = {
  path: string
  title: string
  summary: string
  source?: string
  language?: string
}

type CorpusLicense = { label: string; url: string }

export type ReferenceCorpus = {
  idPrefix: string
  folder: string
  documents: Document[]
  topic: ReferenceTopic
  images: ReadonlyMap<string, string>
  licenses: readonly CorpusLicense[]
}

export function createReferenceCorpus({
  folder, topic, source, language, category = 'guide', chapters, markdown, images, licenses,
}: {
  folder: string
  topic: ReferenceTopic
  source: string
  language: string
  category?: 'guide' | 'code'
  chapters: readonly Chapter[]
  markdown: Record<string, string>
  images: Record<string, string>
  licenses: readonly CorpusLicense[]
}): ReferenceCorpus {
  const root = './samples/' + folder + '/'
  const idPrefix = referenceIdPrefix(topic, folder)
  const documents = chapters.map(({ path, title, summary, source: chapterSource, language: chapterLanguage }): Document => {
    const original = markdown[root + path]
    if (original === undefined) throw new Error('강의 원문 파일 누락: ' + path)
    return {
      id: idPrefix + path.replace(/^chapters\//, '').replace(/\.md$/, '').replaceAll(/[\/_]/g, '-'),
      title,
      summary,
      source: chapterSource ?? source,
      tag: chapterLanguage ?? language,
      category,
      topic,
      body: '',
      markdown: original.replace(/^# .*\r?\n+/, ''),
      samplePath: path,
    }
  })

  return {
    idPrefix,
    folder,
    topic,
    documents,
    images: new Map(Object.entries(images).map(([path, url]) => ['/samples/' + folder + '/' + path.slice(root.length), url])),
    licenses,
  }
}
