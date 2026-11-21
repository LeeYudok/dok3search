# 비교

이러한 트레잇은 값 간의 비교를 지원합니다. 모든 트레잇은 이러한 트레잇을 구현하는 필드를 포함하는 타입에 대해 상속될 수 있습니다.

## `PartialEq` and `Eq`

`PartialEq`는 필수 메서드인 `eq` 및 제공된 `ne` 메서드를 사용하는 부분 등가 관계입니다. `==` 및 `!=` 연산자는 이러한 메서드를 호출합니다.

``` rust
struct Key {
    id: u32,
    metadata: Option<String>,
}
impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
```

`Eq` is a full equivalence relation (reflexive, symmetric, and transitive) and implies `PartialEq`. Functions that require full equivalence will use `Eq` as a trait bound.

## `PartialOrd` 및 `Ord`

`PartialOrd`는 `partial_cmp` 메서드를 사용하여 부분 순서를 정의합니다. `<`, `<=`, `>=`, `>` 연산자를 구현하는 데 사용됩니다.

``` rust
use std::cmp::Ordering;
#[derive(Eq, PartialEq)]
struct Citation {
    author: String,
    year: u32,
}
impl PartialOrd for Citation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.author.partial_cmp(&other.author) {
            Some(Ordering::Equal) => self.year.partial_cmp(&other.year),
            author_ord => author_ord,
        }
    }
}
```

`Ord`는 전체 순서 지정이며 `cmp`는 `Ordering`을 반환합니다.

> **발표자 노트**
>
> This slide should take about 10 minutes.
>
> `PartialEq`는 서로 다른 타입 간에 구현될 수 있지만 `Eq`는 구현될 수 없습니다. 반사적이기 때문입니다.
>
> ``` rust
> struct Key {
>     id: u32,
>     metadata: Option<String>,
> }
> impl PartialEq<u32> for Key {
>     fn eq(&self, other: &u32) -> bool {
>         self.id == *other
>     }
> }
> ```
>
> 실제로 이러한 트레잇을 상속하는 것은 일반적이지만 구현하는 것은 드문 일입니다.
