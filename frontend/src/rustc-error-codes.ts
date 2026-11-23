import index from './samples/rustc-error-codes/index.json'
import copyright from './samples/rustc-error-codes/COPYRIGHT?url&no-inline'
import mitLicense from './samples/rustc-error-codes/LICENSE-MIT?url&no-inline'
import apacheLicense from './samples/rustc-error-codes/LICENSE-APACHE?url&no-inline'
import { createReferenceCorpus } from './reference-corpus'

const markdown = import.meta.glob<string>('./samples/rustc-error-codes/chapters/**/*.md', {
  eager: true, import: 'default', query: '?raw',
})

export const rustcErrorCodes = createReferenceCorpus({
  folder: 'rustc-error-codes',
  topic: 'rust',
  category: 'code',
  source: 'Rust Compiler Error Index (rustc --explain)',
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
