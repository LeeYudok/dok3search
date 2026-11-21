# 함수 호출에서의 수명

A reference has a *lifetime*, which must not “outlive” the value it refers to. This is verified by the borrow checker.

The lifetime can be implicit - this is what we have seen so far. Lifetimes can also be explicit: `&'a Point`, `&'document str`. Lifetimes start with `'` and `'a` is a typical default name. Read `&'a Point` as “a borrowed `Point` which is valid for at least the lifetime `a`”.

Lifetimes are always inferred by the compiler: you cannot assign a lifetime yourself. Explicit lifetime annotations create constraints where there is ambiguity; the compiler verifies that there is a valid solution.

수명은 함수에 값을 전달하고 함수에서 값을 반환하는 경우를 고려할 때 더 복잡해집니다.

컴파일 오류 예제

``` rust
#[derive(Debug)]
struct Point(i32, i32);

fn left_most(p1: &Point, p2: &Point) -> &Point {
    if p1.0 < p2.0 {
        p1
    } else {
        p2
    }
}

fn main() {
    let p1: Point = Point(10, 10);
    let p2: Point = Point(20, 20);
    let p3 = left_most(&p1, &p2); // p3의 수명은 어떻게 되나요?
    println!("p3: {p3:?}");
}
```

> **발표자 노트**
>
> This slide should take about 10 minutes.
>
> In this example, the compiler does not know what lifetime to infer for `p3`. Looking inside the function body shows that it can only safely assume that `p3`’s lifetime is the shorter of `p1` and `p2`. But just like types, Rust requires explicit annotations of lifetimes on function arguments and return values.
>
> `'a`를 `left_most`에 적절하게 추가합니다.
>
> ``` rust
> fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
> ```
>
> 즉, “`a`보다 오래 지속되는 p1과 p2가 있으면 반환 값은 최소한 `'a` 동안 유지됩니다.
>
> 일반적으로 수명은 다음 슬라이드에 설명된 대로 생략될 수 있습니다.
