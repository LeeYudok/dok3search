# Module scraper::selectable

## Overview

Provides the `Selectable` to abstract over collections of elements

## Items

- `Selectable` — Trait to abstract over collections of elements to which a CSS selector can be applied

### trait scraper::selectable::Selectable

```rust
pub trait Selectable<'a> {
    type Select<'b>: Iterator<Item = ElementRef<'a>>;

    // Required method
    fn select(self, selector: &Selector) -> Self::Select<'_>;
}
```

Trait to abstract over collections of elements to which a CSS selector can be applied

The mainly enables writing helper functions which are generic over `Html` and `ElementRef`, e.g.

```rust
use scraper::{selectable::Selectable, selector::Selector};

fn text_of_first_match<'a, S>(selectable: S, selector: &Selector) -> Option<String>
where
    S: Selectable<'a>,
{
    selectable.select(selector).next().map(|element| element.text().collect())
}
```

**Methods**

- `type Select<'b>: Iterator<Item = ElementRef<'a>>`
- `fn select(self, selector: &Selector) -> Self::Select<'_>`
