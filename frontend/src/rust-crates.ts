import index from './samples/rust-crates/index.json'
import { createReferenceCorpus } from './reference-corpus'

const markdown = import.meta.glob<string>('./samples/rust-crates/chapters/**/*.md', {
  eager: true, import: 'default', query: '?raw',
})

export const rustCrates = createReferenceCorpus({
  folder: 'rust-crates',
  topic: 'rust',
  category: 'code',
  source: 'Rust Workspace & Dependency Crates (cargo doc)',
  language: '영어',
  chapters: index.chapters,
  markdown,
  images: {},
  licenses: [],
})
