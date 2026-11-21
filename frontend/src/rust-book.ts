import index from './samples/rust-book/index.json'
import copyright from './samples/rust-book/COPYRIGHT?url&no-inline'
import mitLicense from './samples/rust-book/LICENSE-MIT?url&no-inline'
import apacheLicense from './samples/rust-book/LICENSE-APACHE?url&no-inline'
import { createReferenceCorpus } from './reference-corpus'

const markdown = import.meta.glob<string>('./samples/rust-book/chapters/**/*.md', {
  eager: true, import: 'default', query: '?raw',
})

const images = import.meta.glob<string>('./samples/rust-book/assets/**/*.{svg,png}', {
  eager: true, import: 'default', query: '?url',
})

export const rustBook = createReferenceCorpus({
  folder: 'rust-book',
  category: 'rust',
  source: 'Rust 공식 책 · The Rust Programming Language',
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
