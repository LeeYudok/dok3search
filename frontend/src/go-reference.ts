import index from './samples/go-reference/index.json'
import contentLicense from './samples/go-reference/LICENSE-CC-BY?url&no-inline'
import codeLicense from './samples/go-reference/LICENSE-BSD?url&no-inline'
import { createReferenceCorpus } from './reference-corpus'

const markdown = import.meta.glob<string>('./samples/go-reference/chapters/**/*.md', {
  eager: true, import: 'default', query: '?raw',
})

const images = import.meta.glob<string>('./samples/go-reference/assets/**/*.svg', {
  eager: true, import: 'default', query: '?url',
})

export const goReference = createReferenceCorpus({
  folder: 'go-reference',
  category: 'go',
  source: index.source,
  language: '영어',
  chapters: index.chapters,
  markdown,
  images,
  licenses: [
    { label: '문서 CC BY 4.0', url: contentLicense },
    { label: '코드 BSD 3-Clause', url: codeLicense },
  ],
})
