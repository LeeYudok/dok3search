# Module dok3-domain::change

## Overview

Durable change journal contracts for incremental indexing and generation catch-up.

## Items

- `ChangeEvent` — Single ordered change event emitted by connectors or ingestion jobs.
- `ChangeKind` — Mutation type recorded in the durable change journal.

### enum dok3-domain::change::ChangeKind

```rust
pub enum ChangeKind {
    Upsert,
    Delete,
}
```

Mutation type recorded in the durable change journal.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`

### struct dok3-domain::change::ChangeEvent

```rust
pub struct ChangeEvent {
    pub sequence: Sequence,
    pub document_id: DocumentId,
    pub revision_id: RevisionId,
    pub kind: ChangeKind,
}
```

Single ordered change event emitted by connectors or ingestion jobs.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`
