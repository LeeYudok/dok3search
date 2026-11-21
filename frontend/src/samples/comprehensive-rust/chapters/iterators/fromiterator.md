# FromIterator

어떤 컬렉션이 [`FromIterator`](https://doc.rust-lang.org/std/iter/trait.FromIterator.html)를 구현하고 있다면 [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html)로부터 그 컬렉션을 만들 수 있습니다.

``` rust
fn main() {
    let primes = vec![2, 3, 5, 7];
    let prime_squares = primes.into_iter().map(|p| p * p).collect::<Vec<_>>();
    println!("prime_squares: {prime_squares:?}");
}
```

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> `Iterator` implements
>
> ``` rust
> fn collect<B>(self) -> B
> where
>     B: FromIterator<Self::Item>,
>     Self: Sized
> ```
>
> 이 메서드에 `B`를 지정하는 방법에는 두 가지가 있습니다.
>
> - With the “turbofish”: `some_iterator.collect::<COLLECTION_TYPE>()`, as shown. The `_` shorthand used here lets Rust infer the type of the `Vec` elements.
> - 타입 추론 사용: `let prime_squares: Vec<_> = some_iterator.collect()`를 사용합니다. 이 방법으로 예제를 다시 작성해 보세요.
>
> There are basic implementations of `FromIterator` for `Vec`, `HashMap`, etc. There are also more specialized implementations which let you do cool things like convert an `Iterator<Item = Result<V, E>>` into a `Result<Vec<V>, E>`.
