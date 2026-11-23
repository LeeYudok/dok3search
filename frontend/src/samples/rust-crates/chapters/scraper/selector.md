# Module scraper::selector

## Overview

CSS selectors.

## Items

- `CssLocalName` — Wraps [LocalName] so that it can be used with [selectors]
- `CssString` — Wraps String so that it can be used with [selectors]
- `Parser` — An implementation of Parser for selectors
- `Selector` — Wrapper around CSS selectors.
- `Simple` — A simple implementation of SelectorImpl with no pseudo-classes or pseudo-elements.
- `NonTSPseudoClass` — Non Tree-Structural Pseudo-Class.
- `PseudoElement` — CSS Pseudo-Element
- `ToCss` — Trait for things the can serialize themselves in CSS syntax.

### enum scraper::selector::NonTSPseudoClass

```rust
pub enum NonTSPseudoClass {}
```

Non Tree-Structural Pseudo-Class.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn is_active_or_hover(&self) -> bool`
- `fn is_user_action_state(&self) -> bool`
- `fn visit<V>(&self, _visitor: &mut V) -> boolwhere V: SelectorVisitor, <V as SelectorVisitor>::Impl: SelectorImpl<NonTSPseudoClass = Self>,`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn to_css<W>(&self, dest: &mut W) -> Resultwhere W: Write,`
- `fn to_css_string(&self) -> String`

### enum scraper::selector::PseudoElement

```rust
pub enum PseudoElement {}
```

CSS Pseudo-Element

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn accepts_state_pseudo_classes(&self) -> bool`
- `fn valid_after_slotted(&self) -> bool`
- `fn valid_after_before_or_after(&self) -> bool`
- `fn parses_as_element_backed(&self) -> bool`
- `fn is_before_or_after(&self) -> bool`
- `fn specificity_count(&self) -> u32`
- `fn is_in_pseudo_element_tree(&self) -> bool`
- `fn to_css<W>(&self, dest: &mut W) -> Resultwhere W: Write,`
- `fn to_css_string(&self) -> String`

### struct scraper::selector::CssLocalName

```rust
pub struct CssLocalName(pub LocalName);
```

Wraps [`LocalName`] so that it can be used with [`selectors`]

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn from(val: &'a str) -> Self`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn precomputed_hash(&self) -> u32`
- `fn to_css<W>(&self, dest: &mut W) -> Resultwhere W: Write,`
- `fn to_css_string(&self) -> String`

### struct scraper::selector::CssString

```rust
pub struct CssString(pub String);
```

Wraps `String` so that it can be used with [`selectors`]

**Methods**

- `fn as_ref(&self) -> &str`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn from(val: &'a str) -> Self`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn to_css<W>(&self, dest: &mut W) -> Resultwhere W: Write,`
- `fn to_css_string(&self) -> String`

### struct scraper::selector::Parser

```rust
pub struct Parser;
```

An implementation of `Parser` for `selectors`

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn parse_is_and_where(&self) -> bool`
- `fn parse_has(&self) -> bool`
- `fn parse_slotted(&self) -> bool`
- `fn parse_part(&self) -> bool`
- `fn parse_nth_child_of(&self) -> bool`
- `fn parse_parent_selector(&self) -> bool`
- `fn is_is_alias(&self, _name: &str) -> bool`
- `fn parse_host(&self) -> bool`
- `fn allow_forgiving_selectors(&self) -> bool`
- `fn parse_non_ts_pseudo_class( &self, _name: CowRcStr<'i>, ) -> Result<<Self::Impl as SelectorImpl>::NonTSPseudoClass, ParseError<Self::Error>>`
- `fn parse_non_ts_functional_pseudo_class( &self, _name: CowRcStr<'i>, _parser: &mut Parser<'i>, _after_part: bool, ) -> Result<<Self::Impl as SelectorImpl>::NonTSPseudoClass, ParseError<Self::Error>>`
- `fn parse_pseudo_element( &self, _name: CowRcStr<'i>, ) -> Result<<Self::Impl as SelectorImpl>::PseudoElement, ParseError<Self::Error>>`
- `fn parse_functional_pseudo_element( &self, _name: CowRcStr<'i>, _arguments: &mut Parser<'i>, ) -> Result<<Self::Impl as SelectorImpl>::PseudoElement, ParseError<Self::Error>>`
- `fn default_namespace( &self, ) -> Option<<Self::Impl as SelectorImpl>::NamespaceUrl>`
- `fn namespace_for_prefix( &self, _prefix: &<Self::Impl as SelectorImpl>::NamespacePrefix, ) -> Option<<Self::Impl as SelectorImpl>::NamespaceUrl>`

### struct scraper::selector::Selector

```rust
pub struct Selector { /* private fields */ }
```

Wrapper around CSS selectors.

Represents a “selector group”, i.e. a comma-separated list of selectors.

**Methods**

- `pub fn parse(selectors: &str) -> Result<Self, SelectorErrorKind>`
- `pub fn matches(&self, element: &ElementRef<'_>) -> bool`
- `pub fn matches_with_scope( &self, element: &ElementRef<'_>, scope: Option<ElementRef<'_>>, ) -> bool`
- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn from_str(s: &str) -> Result<Self, Self::Err>`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn to_css<W>(&self, dest: &mut W) -> Resultwhere W: Write,`
- `fn to_css_string(&self) -> String`
- `fn try_from(s: &str) -> Result<Self, Self::Error>`

### struct scraper::selector::Simple

```rust
pub struct Simple;
```

A simple implementation of `SelectorImpl` with no pseudo-classes or pseudo-elements.

**Methods**

- `fn clone(&self) -> Self`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn eq(&self, other: &Self) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`
- `fn should_collect_attr_hash(_name: &Self::LocalName) -> bool`

### trait scraper::selector::ToCss

```rust
pub trait ToCss {
    // Required method
    fn to_css<W>(&self, dest: &mut W) -> Result<(), Error>
       where W: Write;

    // Provided method
    fn to_css_string(&self) -> String { ... }
}
```

Trait for things the can serialize themselves in CSS syntax.

**Methods**

- `fn to_css<W>(&self, dest: &mut W) -> Result<(), Error>where W: Write,`
- `fn to_css_string(&self) -> String`
- `fn to_css<W>(&self, dest: &mut W) -> Result<(), Error>where W: Write,`
- `fn to_css<W>(&self, dest: &mut W) -> Result<(), Error>where W: Write,`
- `fn to_css<W>(&self, dest: &mut W) -> Result<(), Error>where W: Write,`
- `fn to_css<W>(&self, dest: &mut W) -> Result<(), Error>where W: Write,`
- `fn to_css<W>(&self, dest: &mut W) -> Result<(), Error>where W: Write,`
- `fn to_css<W>(&self, dest: &mut W) -> Result<(), Error>where W: Write,`
- `fn to_css<W>(&self, dest: &mut W) -> Result<(), Error>where W: Write,`
- `fn to_css<W>(&self, dest: &mut W) -> Result<(), Error>where W: Write,`
- `fn to_css<W>(&self, dest: &mut W) -> Result<(), Error>where W: Write,`
- `fn to_css<W>(&self, dest: &mut W) -> Result<(), Error>where W: Write,`
- `fn to_css<W>(&self, dest: &mut W) -> Result<(), Error>where W: Write,`
- `fn to_css<W>(&self, dest: &mut W) -> Result<(), Error>where W: Write,`
- `fn to_css<W>(&self, dest: &mut W) -> Result<(), Error>where W: Write,`
- `fn to_css<W>(&self, dest: &mut W) -> Result<(), Error>where W: Write,`
- `fn to_css<W>(&self, dest: &mut W) -> Result<(), Error>where W: Write,`
- `fn to_css<W>(&self, dest: &mut W) -> Result<(), Error>where W: Write,`
- `fn to_css<W>(&self, dest: &mut W) -> Result<(), Error>where W: Write,`
- `fn to_css<W>(&self, dest: &mut W) -> Result<(), Error>where W: Write,`
