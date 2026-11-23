# Crate scraper

## Overview

**scraper**

HTML parsing and querying with CSS selectors.

`scraper` is on crates.io and GitHub.

Scraper provides an interface to Servo’s `html5ever` and `selectors` crates, for
browser-grade parsing and querying.

**Examples**

**Parsing a document**

```rust
use scraper::Html;

let html = r#"
    <!DOCTYPE html>
    <meta charset="utf-8">
    <title>Hello, world!</title>
    <h1 class="foo">Hello, <i>world!</i></h1>
"#;

let document = Html::parse_document(html);
```

**Parsing a fragment**

```rust
use scraper::Html;
let fragment = Html::parse_fragment("<h1>Hello, <i>world!</i></h1>");
```

**Parsing a selector**

```rust
use scraper::Selector;
let selector = Selector::parse("h1.foo").unwrap();
```

**Selecting elements**

```rust
use scraper::{Html, Selector};

let html = r#"
    <ul>
        <li>Foo</li>
        <li>Bar</li>
        <li>Baz</li>
    </ul>
"#;

let fragment = Html::parse_fragment(html);
let selector = Selector::parse("li").unwrap();

for element in fragment.select(&selector) {
    assert_eq!("li", element.value().name());
}
```

**Selecting descendent elements**

```rust
use scraper::{Html, Selector};

let html = r#"
    <ul>
        <li>Foo</li>
        <li>Bar</li>
        <li>Baz</li>
    </ul>
"#;

let fragment = Html::parse_fragment(html);
let ul_selector = Selector::parse("ul").unwrap();
let li_selector = Selector::parse("li").unwrap();

let ul = fragment.select(&ul_selector).next().unwrap();
for element in ul.select(&li_selector) {
    assert_eq!("li", element.value().name());
}
```

**Accessing element attributes**

```rust
use scraper::{Html, Selector};

let fragment = Html::parse_fragment(r#"<input name="foo" value="bar">"#);
let selector = Selector::parse(r#"input[name="foo"]"#).unwrap();

let input = fragment.select(&selector).next().unwrap();
assert_eq!(Some("bar"), input.value().attr("value"));
```

**Serializing HTML and inner HTML**

```rust
use scraper::{Html, Selector};

let fragment = Html::parse_fragment("<h1>Hello, <i>world!</i></h1>");
let selector = Selector::parse("h1").unwrap();

let h1 = fragment.select(&selector).next().unwrap();

assert_eq!("<h1>Hello, <i>world!</i></h1>", h1.html());
assert_eq!("Hello, <i>world!</i>", h1.inner_html());
```

**Accessing descendent text**

...

## Items

- `element_ref` — Element references.
- `error` — Custom error types for diagnostics Includes re-exported error types from dependencies
- `html` — HTML documents and fragments.
- `node` — HTML nodes.
- `selectable` — Provides the Selectable to abstract over collections of elements
- `selector` — CSS selectors.
- `StrTendril` — Primary string tendril type.

### enum scraper::CaseSensitivity

```rust
pub enum CaseSensitivity {
    CaseSensitive,
    AsciiCaseInsensitive,
}
```

**Methods**

- `pub fn eq(self, a: &[u8], b: &[u8]) -> bool`
- `pub fn contains(self, haystack: &str, needle: &str) -> bool`
- `fn clone(&self) -> CaseSensitivity`
- `1.0.0 (const: unstable) · fn clone_from(&mut self, source: &Self)`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn eq(&self, other: &CaseSensitivity) -> bool`
- `1.0.0 (const: unstable) · fn ne(&self, other: &Rhs) -> bool`

### trait scraper::Element

```rust
pub trait Element:
    Sized
    + Clone
    + Debug {
    type Impl: SelectorImpl;

    // Required methods
    fn opaque(&self) -> OpaqueElement;
    fn parent_element(&self) -> Option<Self>;
    fn parent_node_is_shadow_root(&self) -> bool;
    fn containing_shadow_host(&self) -> Option<Self>;
    fn is_pseudo_element(&self) -> bool;
    fn prev_sibling_element(&self) -> Option<Self>;
    fn next_sibling_element(&self) -> Option<Self>;
    fn first_element_child(&self) -> Option<Self>;
    fn is_html_element_in_html_document(&self) -> bool;
    fn has_local_name(
        &self,
        local_name: &<Self::Impl as SelectorImpl>::BorrowedLocalName,
    ) -> bool;
    fn has_namespace(
        &self,
        ns: &<Self::Impl as SelectorImpl>::BorrowedNamespaceUrl,
    ) -> bool;
    fn is_same_type(&self, other: &Self) -> bool;
    fn attr_matches(
        &self,
        ns: &NamespaceConstraint<&<Self::Impl as SelectorImpl>::NamespaceUrl>,
        local_name: &<Self::Impl as SelectorImpl>::LocalName,
        operation: &AttrSelectorOperation<&<Self::Impl as SelectorImpl>::AttrValue>,
    ) -> bool;
    fn match_non_ts_pseudo_class(
        &self,
        pc: &<Self::Impl as SelectorImpl>::NonTSPseudoClass,
        context: &mut MatchingContext<'_, Self::Impl>,
    ) -> bool;
    fn match_pseudo_element(
        &self,
        pe: &<Self::Impl as SelectorImpl>::PseudoElement,
        context: &mut MatchingContext<'_, Self::Impl>,
    ) -> bool;
    fn apply_selector_flags(&self, flags: ElementSelectorFlags);
    fn is_link(&self) -> bool;
    fn is_html_slot_element(&self) -> bool;
    fn has_id(
        &self,
        id: &<Self::Impl as SelectorImpl>::Identifier,
        case_sensitivity: CaseSensitivity,
    ) -> bool;
    fn has_class(
        &self,
        name: &<Self::Impl as SelectorImpl>::Identifier,
        case_sensitivity: CaseSensitivity,
    ) -> bool;
    fn has_custom_state(
        &self,
        name: &<Self::Impl as SelectorImpl>::Identifier,
    ) -> bool;
    fn imported_part(
        &self,
        name: &<Self::Impl as SelectorImpl>::Identifier,
    ) -> Option<<Self::Impl as SelectorImpl>::Identifier>;
    fn is_part(&self, name: &<Self::Impl as SelectorImpl>::Identifier) -> bool;
    fn is_empty(&self) -> bool;
    fn is_root(&self) -> bool;
    fn add_element_unique_hashes(
        &self,
        filter: &mut CountingBloomFilter<BloomStorageU8>,
    ) -> bool;

    // Provided methods
    fn pseudo_element_originating_element(&self) -> Option<Self> { ... }
    fn has_attr_in_no_namespace(
        &self,
        local_name: &<Self::Impl as SelectorImpl>::LocalName,
    ) -> bool { ... }
    fn assigned_slot(&self) -> Option<Self> { ... }
    fn ignores_nth_child_selectors(&self) -> bool { ... }
}
```

**Methods**

- `type Impl: SelectorImpl`
- `fn opaque(&self) -> OpaqueElement`
- `fn parent_element(&self) -> Option<Self>`
- `fn parent_node_is_shadow_root(&self) -> bool`
- `fn containing_shadow_host(&self) -> Option<Self>`
- `fn is_pseudo_element(&self) -> bool`
- `fn prev_sibling_element(&self) -> Option<Self>`
- `fn next_sibling_element(&self) -> Option<Self>`
- `fn first_element_child(&self) -> Option<Self>`
- `fn is_html_element_in_html_document(&self) -> bool`
- `fn has_local_name( &self, local_name: &<Self::Impl as SelectorImpl>::BorrowedLocalName, ) -> bool`
- `fn has_namespace( &self, ns: &<Self::Impl as SelectorImpl>::BorrowedNamespaceUrl, ) -> bool`
- `fn is_same_type(&self, other: &Self) -> bool`
- `fn attr_matches( &self, ns: &NamespaceConstraint<&<Self::Impl as SelectorImpl>::NamespaceUrl>, local_name: &<Self::Impl as SelectorImpl>::LocalName, operation: &AttrSelectorOperation<&<Self::Impl as SelectorImpl>::AttrValue>, ) -> bool`
- `fn match_non_ts_pseudo_class( &self, pc: &<Self::Impl as SelectorImpl>::NonTSPseudoClass, context: &mut MatchingContext<'_, Self::Impl>, ) -> bool`
- `fn match_pseudo_element( &self, pe: &<Self::Impl as SelectorImpl>::PseudoElement, context: &mut MatchingContext<'_, Self::Impl>, ) -> bool`
- `fn apply_selector_flags(&self, flags: ElementSelectorFlags)`
- `fn is_link(&self) -> bool`
- `fn is_html_slot_element(&self) -> bool`
- `fn has_id( &self, id: &<Self::Impl as SelectorImpl>::Identifier, case_sensitivity: CaseSensitivity, ) -> bool`
- `fn has_class( &self, name: &<Self::Impl as SelectorImpl>::Identifier, case_sensitivity: CaseSensitivity, ) -> bool`
- `fn has_custom_state( &self, name: &<Self::Impl as SelectorImpl>::Identifier, ) -> bool`
- `fn imported_part( &self, name: &<Self::Impl as SelectorImpl>::Identifier, ) -> Option<<Self::Impl as SelectorImpl>::Identifier>`
- `fn is_part(&self, name: &<Self::Impl as SelectorImpl>::Identifier) -> bool`
- `fn is_empty(&self) -> bool`
- `fn is_root(&self) -> bool`
- `fn add_element_unique_hashes( &self, filter: &mut CountingBloomFilter<BloomStorageU8>, ) -> bool`
- `fn pseudo_element_originating_element(&self) -> Option<Self>`
- `fn has_attr_in_no_namespace( &self, local_name: &<Self::Impl as SelectorImpl>::LocalName, ) -> bool`
- `fn assigned_slot(&self) -> Option<Self>`
- `fn ignores_nth_child_selectors(&self) -> bool`

### type scraper::StrTendril

```rust
pub type StrTendril = StrTendril;
```

Primary string tendril type.
