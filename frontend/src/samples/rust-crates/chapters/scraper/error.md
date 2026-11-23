# Module scraper::error

## Overview

Custom error types for diagnostics
Includes re-exported error types from dependencies

## Items

- `SelectorErrorKind` — Error type that is returned when calling Selector::parse

### enum scraper::error::SelectorErrorKind

```rust
pub enum SelectorErrorKind {
    UnexpectedToken,
    EndOfLine,
    InvalidAtRule,
    InvalidAtRuleBody,
    QualRuleInvalid,
    TooManyNestedBlocks,
    ExpectedColonOnPseudoElement,
    ExpectedIdentityOnPseudoElement,
    UnexpectedSelectorParseError(SelectorParseErrorKind),
}
```

Error type that is returned when calling `Selector::parse`

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn description(&self) -> &str`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(err: BasicParseErrorKind) -> Self`
- `fn from(original: ParseError<SelectorParseErrorKind>) -> Self`
- `fn from(err: SelectorParseErrorKind) -> Self`
