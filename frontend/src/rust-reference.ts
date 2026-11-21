import index from './samples/rust-reference/index.json'
import copyright from './samples/rust-reference/COPYRIGHT?url&no-inline'
import mitLicense from './samples/rust-reference/LICENSE-MIT?url&no-inline'
import apacheLicense from './samples/rust-reference/LICENSE-APACHE?url&no-inline'
import { createReferenceCorpus } from './reference-corpus'

const markdown = import.meta.glob<string>('./samples/rust-reference/chapters/**/*.md', {
  eager: true, import: 'default', query: '?raw',
})

const images = import.meta.glob<string>('./samples/rust-reference/assets/**/*.{svg,png,jpg,jpeg,gif,webp}', {
  eager: true, import: 'default', query: '?url',
})

export const rustReference = createReferenceCorpus({
  folder: 'rust-reference',
  topic: 'rust',
  source: 'Rust 공식 언어 레퍼런스 · The Rust Reference',
  language: '영어',
  chapters: index.chapters,
  markdown,
  images,
  licenses: [
    { label: '저작권 고지', url: copyright },
    { label: 'MIT', url: mitLicense },
    { label: 'Apache 2.0', url: apacheLicense },
  ],
})
