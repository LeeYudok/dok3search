# Module tower::util::error::optional

## Overview

Error types for `Optional`.

## Items

- `None` — Error returned if the inner Service has not been set.

### struct tower::util::error::optional::None

```rust
pub struct None(/* private fields */);
```

Error returned if the inner `Service` has not been set.

**Methods**

- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, fmt: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
