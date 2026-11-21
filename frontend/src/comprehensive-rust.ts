import index from './samples/comprehensive-rust/index.json'
import contentLicense from './samples/comprehensive-rust/LICENSE-CC-BY?url&no-inline'
import codeLicense from './samples/comprehensive-rust/LICENSE-APACHE?url&no-inline'
import { createReferenceCorpus } from './reference-corpus'

const markdown = import.meta.glob<string>('./samples/comprehensive-rust/chapters/**/*.md', {
  eager: true, import: 'default', query: '?raw',
})

const images = import.meta.glob<string>('./samples/comprehensive-rust/assets/**/*.{svg,png}', {
  eager: true, import: 'default', query: '?url',
})

export const comprehensiveRust = createReferenceCorpus({
  folder: 'comprehensive-rust',
  topic: 'rust',
  source: 'Google Android 팀 · Comprehensive Rust',
  language: '한국어',
  chapters: index.chapters,
  markdown,
  images,
  licenses: [
    { label: '문서 CC BY 4.0', url: contentLicense },
    { label: '코드 Apache 2.0', url: codeLicense },
  ],
})
