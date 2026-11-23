# Module dok3-search::seed

## Overview

Built-in Korean sample documents and optional `dok3search-design` loader.

## Items

- `document_heading_id` — Matches frontend documentHeadingId: 'section-' + label.toLowerCase().replace(/[^\p{L}\p{N}]+/gu, '-').replace(/-$/, '').
- `load_seed_dataset` — Loads dok3search-design files (if design_dir exists), the 20 built-in samples, and the built-in dok3search-manual Knowledge Pack document (appended last to keep sample ranks).

### fn dok3-search::seed::document_heading_id

```rust
pub fn document_heading_id(label: &str) -> String
```

Matches frontend `documentHeadingId`: `'section-' + label.toLowerCase().replace(/[^\p{L}\p{N}]+/gu, '-').replace(/-$/, '')`.

### fn dok3-search::seed::load_seed_dataset

```rust
pub fn load_seed_dataset(design_dir: Option<&Path>) -> SeedDataset
```

Loads `dok3search-design` files (if `design_dir` exists), the 20 built-in samples, and the
built-in `dok3search-manual` Knowledge Pack document (appended last to keep sample ranks).

### struct dok3-search::seed::SeedDataset

```rust
pub struct SeedDataset {
    pub documents: Vec<Document>,
    pub pairs: Vec<DocumentFormatLinks>,
}
```

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
