# Module dok3-benchmark::evaluator

## Overview

Document-level recall, reciprocal rank, and forbidden-hit checks.

## Items

- `evaluate` — Computes macro-averaged metrics across cases. Cases with no expected documents score zero for both ranking metrics and are still checked for ACL violations.

### enum dok3-benchmark::evaluator::QueryGroup

```rust
pub enum QueryGroup {
    Rust,
    ApiError,
    Korean,
    CodeIdentifier,
    Acl,
}
```

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn cmp(&self, other: &Self) -> Ordering`
- `1.21.0 (const: unstable) · fn max(self, other: Self) -> Selfwhere Self: Sized,`
- `1.21.0 (const: unstable) · fn min(self, other: Self) -> Selfwhere Self: Sized,`
- `1.50.0 (const: unstable) · fn clamp(self, min: Self, max: Self) -> Selfwhere Self: Sized,`
- `fn clamp_to<R>(self, range: R) -> Selfwhere Self: Sized, R: ClampBounds<Self>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn partial_cmp(&self, other: &Self) -> Option<Ordering>`
- `1.0.0 (const: unstable) · fn lt(&self, other: &Rhs) -> bool`
- `1.0.0 (const: unstable) · fn le(&self, other: &Rhs) -> bool`
- `1.0.0 (const: unstable) · fn gt(&self, other: &Rhs) -> bool`
- `1.0.0 (const: unstable) · fn ge(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### fn dok3-benchmark::evaluator::evaluate

```rust
pub fn evaluate(
    service: &SearchService,
    cases: &[QueryCase],
    config: EvalConfig,
) -> Result<EvalReport, Dok3Error>
```

Computes macro-averaged metrics across cases. Cases with no expected documents
score zero for both ranking metrics and are still checked for ACL violations.

### struct dok3-benchmark::evaluator::CaseResult

```rust
pub struct CaseResult {
    pub id: String,
    pub group: QueryGroup,
    pub recall_at_k: f64,
    pub reciprocal_rank_at_k: f64,
    pub forbidden_hits: Vec<DocumentId>,
}
```

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-benchmark::evaluator::EvalConfig

```rust
pub struct EvalConfig {
    pub k: usize,
}
```

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`

### struct dok3-benchmark::evaluator::EvalReport

```rust
pub struct EvalReport {
    pub k: usize,
    pub total_cases: usize,
    pub overall_recall_at_k: f64,
    pub overall_mrr_at_k: f64,
    pub by_group: BTreeMap<QueryGroup, GroupResult>,
    pub case_results: Vec<CaseResult>,
    pub acl_violations: usize,
}
```

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-benchmark::evaluator::GroupResult

```rust
pub struct GroupResult {
    pub total_cases: usize,
    pub recall_at_k: f64,
    pub mrr_at_k: f64,
}
```

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-benchmark::evaluator::QueryCase

```rust
pub struct QueryCase {
    pub id: String,
    pub group: QueryGroup,
    pub query: String,
    pub category_filter: Option<String>,
    pub tags: Vec<String>,
    pub expected_doc_ids: Vec<DocumentId>,
    pub forbidden_doc_ids: Vec<DocumentId>,
    pub principal: Principal,
}
```

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`
