# Exercise: Generic `min`

In this short exercise, you will implement a generic `min` function that determines the minimum of two values, using the [`Ord`](https://doc.rust-lang.org/stable/std/cmp/trait.Ord.html) trait.

컴파일 오류 예제

``` rust
use std::cmp::Ordering;

// TODO: `main`에 사용되는 `min` 함수를 구현합니다.

fn main() {
    assert_eq!(min(0, 10), 0);
    assert_eq!(min(500, 123), 123);

    assert_eq!(min('a', 'z'), 'a');
    assert_eq!(min('7', '1'), '1');

    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");
}
```

> **발표자 노트**
>
> This slide and its sub-slides should take about 10 minutes.
>
> - Show students the [`Ord`](https://doc.rust-lang.org/stable/std/cmp/trait.Ord.html) trait and [`Ordering`](https://doc.rust-lang.org/stable/std/cmp/enum.Ordering.html) enum.
