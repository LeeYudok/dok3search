# 함수 호출에서의 수명

Lifetimes for function arguments and return values must be fully specified, but Rust allows lifetimes to be elided in most cases with [a few simple rules](https://doc.rust-lang.org/nomicon/lifetime-elision.html). This is not inference – it is just a syntactic shorthand.

- 수명 주석이 없는 각 인수에 하나씩 제공됩니다.
- 인수 수명이 하나만 있는 경우 주석 처리되지 않은 모든 반환 값에 제공됩니다.
- 인수 수명이 여러 개 있지만 첫 번째가 ’self’의 수명이면 해당 전체 기간은 주석 처리되지 않은 모든 반환 값에 제공됩니다.

``` rust
#[derive(Debug)]
struct Point(i32, i32);

fn cab_distance(p1: &Point, p2: &Point) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn nearest<'a>(points: &'a [Point], query: &Point) -> Option<&'a Point> {
    let mut nearest = None;
    for p in points {
        if let Some((_, nearest_dist)) = nearest {
            let dist = cab_distance(p, query);
            if dist < nearest_dist {
                nearest = Some((p, dist));
            }
        } else {
            nearest = Some((p, cab_distance(p, query)));
        };
    }
    nearest.map(|(p, _)| p)
}

fn main() {
    println!(
        "{:?}",
        nearest(
            &[Point(1, 0), Point(1, 0), Point(-1, 0), Point(0, -1),],
            &Point(0, 2)
        )
    );
}
```

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> 이 예에서 `cab_distance`는 간단히 생략됩니다.
>
> `nearest` 함수는 인수에 여러 참조가 포함되어 명시적 주석이 필요한 함수의 또 다른 예를 제공합니다.
>
> 반환된 수명에 관해 ’거짓말’하도록 서명을 조정해 보세요.
>
> ``` rust
> fn nearest<'a, 'q>(points: &'a [Point], query: &'q Point) -> Option<&'q Point> {
> ```
>
> This won’t compile, demonstrating that the annotations are checked for validity by the compiler. Note that this is not the case for raw pointers (unsafe), and this is a common source of errors with unsafe Rust.
>
> Students may ask when to use lifetimes. Rust borrows *always* have lifetimes. Most of the time, elision and type inference mean these don’t need to be written out. In more complicated cases, lifetime annotations can help resolve ambiguity. Often, especially when prototyping, it’s easier to just work with owned data by cloning values where necessary.
