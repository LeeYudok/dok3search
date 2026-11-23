# Module dok3-domain::ids

## Overview

Strongly typed domain identifiers and sequence numbers.

## Items

- `DocumentId` — Logical document identifier.
- `GenerationId` — Identifier for a Blue-Green index generation.
- `RevisionId` — Immutable revision identifier (e.g. content SHA-256).
- `SectionId` — Identifier for a logical section within a document.
- `Sequence` — Monotonic sequence number in the durable change journal.
- `SourceId` — Identifier for an ingested knowledge or data source.

### struct dok3-domain::ids::DocumentId

```rust
pub struct DocumentId(pub String);
```

Logical document identifier.

**Methods**

- `pub fn new(value: impl Into<String>) -> Self`
- `pub fn as_str(&self) -> &str`
- `fn as_ref(&self) -> &str`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn from(value: &str) -> Self`
- `fn from(value: String) -> Self`
- `fn hash<__H: Hasher>(&self, state: &mut __H)`
- `1.3.0 · fn hash_slice<H>(data: &[Self], state: &mut H)where H: Hasher, Self: Sized,`
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

### struct dok3-domain::ids::GenerationId

```rust
pub struct GenerationId(pub String);
```

Identifier for a Blue-Green index generation.

**Methods**

- `pub fn new(value: impl Into<String>) -> Self`
- `pub fn as_str(&self) -> &str`
- `fn as_ref(&self) -> &str`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn from(value: &str) -> Self`
- `fn from(value: String) -> Self`
- `fn hash<__H: Hasher>(&self, state: &mut __H)`
- `1.3.0 · fn hash_slice<H>(data: &[Self], state: &mut H)where H: Hasher, Self: Sized,`
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

### struct dok3-domain::ids::RevisionId

```rust
pub struct RevisionId(pub String);
```

Immutable revision identifier (e.g. content SHA-256).

**Methods**

- `pub fn new(value: impl Into<String>) -> Self`
- `pub fn as_str(&self) -> &str`
- `fn as_ref(&self) -> &str`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn from(value: &str) -> Self`
- `fn from(value: String) -> Self`
- `fn hash<__H: Hasher>(&self, state: &mut __H)`
- `1.3.0 · fn hash_slice<H>(data: &[Self], state: &mut H)where H: Hasher, Self: Sized,`
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

### struct dok3-domain::ids::SectionId

```rust
pub struct SectionId(pub String);
```

Identifier for a logical section within a document.

**Methods**

- `pub fn new(value: impl Into<String>) -> Self`
- `pub fn as_str(&self) -> &str`
- `fn as_ref(&self) -> &str`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn from(value: &str) -> Self`
- `fn from(value: String) -> Self`
- `fn hash<__H: Hasher>(&self, state: &mut __H)`
- `1.3.0 · fn hash_slice<H>(data: &[Self], state: &mut H)where H: Hasher, Self: Sized,`
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

### struct dok3-domain::ids::Sequence

```rust
pub struct Sequence(pub u64);
```

Monotonic sequence number in the durable change journal.

**Methods**

- `pub const fn new(value: u64) -> Self`
- `pub const fn get(self) -> u64`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn from(value: u64) -> Self`
- `fn hash<__H: Hasher>(&self, state: &mut __H)`
- `1.3.0 · fn hash_slice<H>(data: &[Self], state: &mut H)where H: Hasher, Self: Sized,`
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

### struct dok3-domain::ids::SourceId

```rust
pub struct SourceId(pub String);
```

Identifier for an ingested knowledge or data source.

**Methods**

- `pub fn new(value: impl Into<String>) -> Self`
- `pub fn as_str(&self) -> &str`
- `fn as_ref(&self) -> &str`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn from(value: &str) -> Self`
- `fn from(value: String) -> Self`
- `fn hash<__H: Hasher>(&self, state: &mut __H)`
- `1.3.0 · fn hash_slice<H>(data: &[Self], state: &mut H)where H: Hasher, Self: Sized,`
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
