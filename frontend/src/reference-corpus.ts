import type { Document } from './documents'

export function referenceIdPrefix(category: 'rust' | 'go', folder: string) {
  return category + '-ref-' + folder + '-'
}

type Chapter = {
  path: string
  title: string
  summary: string
  source?: string
}

type CorpusLicense = { label: string; url: string }

export type ReferenceCorpus = {
  idPrefix: string
  documents: Document[]
  images: ReadonlyMap<string, string>
  licenses: readonly CorpusLicense[]
}

export function createReferenceCorpus({
  folder, category, source, language, chapters, markdown, images, licenses,
}: {
  folder: string
  category: 'rust' | 'go'
  source: string
  language: string
  chapters: readonly Chapter[]
  markdown: Record<string, string>
  images: Record<string, string>
  licenses: readonly CorpusLicense[]
}): ReferenceCorpus {
  const root = './samples/' + folder + '/'
  const idPrefix = referenceIdPrefix(category, folder)
  const documents = chapters.map(({ path, title, summary, source: chapterSource }): Document => {
    const original = markdown[root + path]
    if (original === undefined) throw new Error('강의 원문 파일 누락: ' + path)
    return {
      id: idPrefix + path.replace(/^chapters\//, '').replace(/\.md$/, '').replaceAll(/[\/_]/g, '-'),
      title,
      summary,
      source: chapterSource ?? source,
      tag: language,
      category,
      body: '',
      markdown: original.replace(/^# .*\r?\n+/, ''),
      samplePath: path,
    }
  })

  return {
    idPrefix,
    documents,
    images: new Map(Object.entries(images).map(([path, url]) => ['/' + path.slice(root.length), url])),
    licenses,
  }
}
