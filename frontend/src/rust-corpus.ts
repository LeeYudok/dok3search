import type { Document } from './documents'

export const rustDocumentPrefix = 'rust-ref-'

type Chapter = {
  path: string
  title: string
  summary: string
}

type CorpusLicense = { label: string; url: string }

export type RustCorpus = {
  idPrefix: string
  documents: Document[]
  images: ReadonlyMap<string, string>
  licenses: readonly CorpusLicense[]
}

export function createRustCorpus({
  folder, source, language, chapters, markdown, images, licenses,
}: {
  folder: string
  source: string
  language: string
  chapters: readonly Chapter[]
  markdown: Record<string, string>
  images: Record<string, string>
  licenses: readonly CorpusLicense[]
}): RustCorpus {
  const root = './samples/' + folder + '/'
  const idPrefix = rustDocumentPrefix + folder + '-'
  const documents = chapters.map(({ path, title, summary }): Document => {
    const original = markdown[root + path]
    if (original === undefined) throw new Error('강의 원문 파일 누락: ' + path)
    return {
      id: idPrefix + path.replace(/^chapters\//, '').replace(/\.md$/, '').replaceAll(/[\/_]/g, '-'),
      title,
      summary,
      source,
      tag: language,
      category: 'rust',
      body: '',
      markdown: original.replace(/^# .*\r?\n+/, ''),
      samplePath: path,
      keywords: path.split(/[/.]+/).includes('borrowing') ? '빌림 대여 borrowing' : undefined,
    }
  })

  return {
    idPrefix,
    documents,
    images: new Map(Object.entries(images).map(([path, url]) => ['/' + path.slice(root.length), url])),
    licenses,
  }
}
