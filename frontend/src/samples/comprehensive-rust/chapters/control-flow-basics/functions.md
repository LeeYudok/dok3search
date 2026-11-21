# 함수

``` rust
fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 {
        gcd(b, a % b)
    } else {
        a
    }
}

fn main() {
    println!("gcd: {}", gcd(143, 52));
}
```

> **발표자 노트**
>
> This slide should take about 3 minutes.
>
> - 매개변수를 선언할 때에는 이름을 먼저 쓰고, 타입을 나중에 씁니다. 이름과 타입은 `:` 로 구분합니다. 이는 일부 언어(예를 들어 C)와 반대임에 유의하시기 바랍니다. 마찬가지로, 리턴 타입도 함수의 시작이 아닌 가장 뒷부분에 선언합니다.
> - The last expression in a function body (or any block) becomes the return value. Simply omit the `;` at the end of the expression. The `return` keyword can be used for early return, but the “bare value” form is idiomatic at the end of a function (refactor `gcd` to use a `return`).
> - 반환값이 없는 함수의 경우, 유닛 타입 `()`을 반환합니다. `-> ()`가 생략된 경우 컴파일러는 이를 추론합니다.
> - Overloading is not supported – each function has a single implementation.
>   - 항상 고정된 개수의 매개변수를 사용합니다. 기본 인수는 지원되지 않습니다. 매크로는 가변 함수를 지원하는 데 사용할 수 있습니다.
>   - Always takes a single set of parameter types. These types can be generic, which will be covered later.
