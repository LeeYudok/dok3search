import index from './samples/go-reference/index.json'
import contentLicense from './samples/go-reference/LICENSE-CC-BY?url&no-inline'
import codeLicense from './samples/go-reference/LICENSE-BSD?url&no-inline'
import tutorialLicense from './samples/go-reference/LICENSE-LGWT-MIT?url&no-inline'
import { createReferenceCorpus } from './reference-corpus'

const markdown = import.meta.glob<string>('./samples/go-reference/chapters/**/*.md', {
  eager: true, import: 'default', query: '?raw',
})

const images = import.meta.glob<string>('./samples/go-reference/assets/**/*.svg', {
  eager: true, import: 'default', query: '?url',
})

export const goReference = createReferenceCorpus({
  folder: 'go-reference',
  topic: 'go',
  source: index.source,
  language: '영어',
  chapters: index.chapters,
  markdown,
  images,
  licenses: [
    { label: 'Go 공식 문서 CC BY 4.0', url: contentLicense },
    { label: 'Go 공식 예제 BSD 3-Clause', url: codeLicense },
    { label: 'Learn Go with Tests MIT', url: tutorialLicense },
  ],
})
