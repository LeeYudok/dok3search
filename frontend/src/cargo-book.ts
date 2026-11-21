import index from './samples/cargo-book/index.json'
import copyright from './samples/cargo-book/COPYRIGHT?url&no-inline'
import mitLicense from './samples/cargo-book/LICENSE-MIT?url&no-inline'
import apacheLicense from './samples/cargo-book/LICENSE-APACHE?url&no-inline'
import { createReferenceCorpus } from './reference-corpus'

const markdown = import.meta.glob<string>('./samples/cargo-book/chapters/**/*.md', {
  eager: true, import: 'default', query: '?raw',
})

const images = import.meta.glob<string>('./samples/cargo-book/assets/**/*.{svg,png,jpg,jpeg,gif,webp}', {
  eager: true, import: 'default', query: '?url',
})

export const cargoBook = createReferenceCorpus({
  folder: 'cargo-book',
  topic: 'rust',
  source: 'Cargo 공식 가이드 · The Cargo Book',
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
