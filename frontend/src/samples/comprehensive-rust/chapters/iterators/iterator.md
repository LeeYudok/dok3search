# `Iterator`

컬랙션에 있는 값들을 접근하기 위해서는 [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) 트레잇을 사용합니다. 이 트레잇은 `next` 메서드를 비롯한 많은 메서드를 제공합니다. 많은 표준 라이브러리 타입이 `Iterator`를 구현하고 있으며, 여러분도 여러분의 타입이 이 트레잇을 직접 구현하도록 할 수 있습니다.

``` rust
struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn main() {
    let fib = Fibonacci { curr: 0, next: 1 };
    for (i, n) in fib.enumerate().take(5) {
        println!("fib({i}): {n}");
    }
}
```

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> - The `Iterator` trait implements many common functional programming operations over collections (e.g. `map`, `filter`, `reduce`, etc). This is the trait where you can find all the documentation about them. In Rust these functions should produce the code as efficient as equivalent imperative implementations.
>
> - `IntoIterator`는 루프를 작동하게 만드는 트레잇입니다. 이는 `Vec<T>`와 같은 컬렉션 타입과 `&Vec<T>` 및 `&[T]`와 같은 이에 대한 참조에 의해 구현됩니다. 범위도 이를 구현합니다. 이런 이유로 `for i in some_vec { .. }`를 사용하여 벡터를 반복할 수 있지만 `some_vec.next()`는 존재하지 않습니다.
