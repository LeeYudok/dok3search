# Module dok3-core::config

## Overview

Runtime configuration loaded from environment variables.

## Items

- `ServerConfig` — Configuration for the dok3search HTTP and static asset server.

### struct dok3-core::config::ServerConfig

```rust
pub struct ServerConfig {
    pub bind: String,
    pub port: u16,
    pub static_dir: Option<PathBuf>,
    pub design_docs_dir: Option<PathBuf>,
    pub source_dir: Option<PathBuf>,
    pub data_dir: Option<PathBuf>,
    pub refresh_interval_secs: Option<u64>,
    pub admin_enabled: bool,
    pub allow_non_loopback_bind: bool,
}
```

Configuration for the dok3search HTTP and static asset server.

**Methods**

- `pub fn from_env() -> Result<Self, Dok3Error>`
- `pub fn from_lookup<F>(lookup: F) -> Result<Self, Dok3Error>where F: FnMut(&str) -> Option<String>,`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>where __D: Deserializer<'de>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>where __S: Serializer,`
