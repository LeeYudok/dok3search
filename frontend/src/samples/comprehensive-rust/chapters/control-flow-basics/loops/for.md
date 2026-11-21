# `for`

The [`for` loop](https://doc.rust-lang.org/std/keyword.for.html) iterates over ranges of values or the items in a collection:

``` rust
fn main() {
    for x in 1..5 {
        println!("x: {x}");
    }

    for elem in [1, 2, 3, 4, 5] {
        println!("elem: {elem}");
    }
}
```

> **발표자 노트**
>
> - Under the hood `for` loops use a concept called “iterators” to handle iterating over different kinds of ranges/collections. Iterators will be discussed in more detail later.
> - `for` 반복문은 `4`까지만 실행됩니다. 마지막 값을 포함시키는 방법으로 `1..=5`와 같은 문법을 보여주세요.
