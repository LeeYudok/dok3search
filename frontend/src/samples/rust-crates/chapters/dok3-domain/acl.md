# Module dok3-domain::acl

## Overview

Access-control scopes and principal derivation (default-deny).

## Items

- `Principal` — Authenticated caller identity and server-derived allowed scopes.
- `AclScope` — Security scope attached to indexed documents and original blobs.

### enum dok3-domain::acl::AclScope

```rust
pub enum AclScope {
    Public,
    Internal(String),
}
```

Security scope attached to indexed documents and original blobs.

**Methods**

- `pub fn internal(name: impl Into<String>) -> Self`
- `pub fn try_internal(name: impl Into<String>) -> Result<Self, Dok3Error>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn from_str(raw: &str) -> Result<Self, Self::Err>`
- `fn hash<__H: Hasher>(&self, state: &mut __H)`
- `1.3.0 · fn hash_slice<H>(data: &[Self], state: &mut H)where H: Hasher, Self: Sized,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>`

### struct dok3-domain::acl::Principal

```rust
pub struct Principal {
    pub subject: String,
    pub allowed_scopes: Vec<AclScope>,
    pub is_admin: bool,
}
```

Authenticated caller identity and server-derived allowed scopes.

**Methods**

- `pub fn anonymous_public() -> Self`
- `pub fn local_admin() -> Self`
- `pub fn allows(&self, scope: &AclScope) -> bool`
- `pub fn effective_scopes(&self, requested: Option<&AclScope>) -> Vec<AclScope>`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`
