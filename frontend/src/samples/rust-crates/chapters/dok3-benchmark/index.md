# Crate dok3-benchmark

## Overview

Reproducible search quality and concurrent load benchmarks.

## Items

- `evaluator` — Document-level recall, reciprocal rank, and forbidden-hit checks.
- `load` — Concurrent search load with per-worker warmup and nearest-rank latency percentiles.
- `poc_suite_for_builtin_samples` — A smoke suite for the corpus returned by InMemorySearchRepository::from_seed(None). Built-in documents are public, so the denied case uses a principal with no scopes.

### fn dok3-benchmark::init

```rust
pub fn init()
```

### fn dok3-benchmark::poc_suite_for_builtin_samples

```rust
pub fn poc_suite_for_builtin_samples() -> Vec<QueryCase>
```

A smoke suite for the corpus returned by `InMemorySearchRepository::from_seed(None)`.
Built-in documents are public, so the denied case uses a principal with no scopes.
