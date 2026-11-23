import index from './samples/rust-std/index.json'
import copyright from './samples/rust-std/COPYRIGHT?url&no-inline'
import mitLicense from './samples/rust-std/LICENSE-MIT?url&no-inline'
import apacheLicense from './samples/rust-std/LICENSE-APACHE?url&no-inline'
import { createReferenceCorpus } from './reference-corpus'

const markdown = import.meta.glob<string>('./samples/rust-std/chapters/**/*.md', {
  eager: true, import: 'default', query: '?raw',
})

export const rustStd = createReferenceCorpus({
  folder: 'rust-std',
  topic: 'rust',
  category: 'code',
  source: 'Rust Standard Library (std/core/alloc)',
  language: '영어',
  chapters: index.chapters,
  markdown,
  images: {},
  licenses: [
    { label: '저작권 고지', url: copyright },
    { label: 'MIT', url: mitLicense },
    { label: 'Apache 2.0', url: apacheLicense },
  ],
})
