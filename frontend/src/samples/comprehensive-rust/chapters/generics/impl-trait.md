# 트레잇 구현하기(`impl Trait`)

트레잇 바운드와 유사하게 `impl Trait` 문법은 함수의 인자와 반환값에도 적용 가능합니다:

``` rust
// 다음과 동일합니다:
//   fn add_42_millions<T: Into<i32>>(x: T) -> i32 {
fn add_42_millions(x: impl Into<i32>) -> i32 {
    x.into() + 42_000_000
}

fn pair_of(x: u32) -> impl std::fmt::Debug {
    (x + 1, x - 1)
}

fn main() {
    let many = add_42_millions(42_i8);
    println!("{many}");
    let many_more = add_42_millions(10_000_000);
    println!("{many_more}");
    let debuggable = pair_of(27);
    println!("디버그 가능: {debuggable:?}");
}
```

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> `impl Trait` allows you to work with types which you cannot name. The meaning of `impl Trait` is a bit different in the different positions.
>
> - 함수 인자의 타입으로 사용되었을 경우에는 `impl Trait`는 트레잇 경계가 있는 익명의 제네릭 타입을 의미합니다.
>
> - 리턴 타입으로 사용되었을 경우에는, 그 트레잇을 구현하는 구체적인 타입인데, 타입 이름을 프로그래머가 짓지 않았다는 것을 의미합니다. 이는 그 구체적인 타입 이름을 API로 공개하고 싶지 않은 경우에 유용합니다.
>
>   함수가 리턴되는 곳에서의 타입 추론은 어렵습니다. 어떤 함수의 리턴 타입이 `impl Foo`로 선언되어 있을 경우, 그 함수가 실제로 리턴하는 타입은 소스 코드 상 어디에도 나타나 있지 않습니다. `collect<B<() -> B`와 같이 제너릭 타입을 리턴하는 함수는 `B`를 만족하는 어떤 타입도 리턴할 수 있습니다. 이 경우, 호출하는 측에서는 `let x: Vec<_> = foo.collect()`나 터보피시 문법을 써서 `foo.collect::<Vec<_>>()`와 같이 리턴 타입을 명시적으로 써 주어야 할 수도 있습니다.
>
> `debuggable` 타입은 무엇인가요? `let debuggable: () = ..`을 시도하여 오류 메시지가 어떻게 표시되는지 확인합니다.
