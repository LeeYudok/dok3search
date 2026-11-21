# 튜플

각 필드 이름이 중요하지 않다면 튜플 구조체를 사용할 수 있습니다:

``` rust
struct Point(i32, i32);

fn main() {
    let p = Point(17, 23);
    println!("({}, {})", p.0, p.1);
}
```

튜플 구조체는 종종 단일 필드의 래퍼(wrapper, 러스트에서 뉴타입(newtype)이라고 부름)로 사용됩니다:

컴파일 오류 예제

``` rust
struct PoundsOfForce(f64);
struct Newtons(f64);

fn compute_thruster_force() -> PoundsOfForce {
    todo!("NASA 로켓 과학자에게 물어보세요")
}

fn set_thruster_force(force: Newtons) {
    // ...
}

fn main() {
    let force = compute_thruster_force();
    set_thruster_force(force);
}
```

> **발표자 노트**
>
> This slide should take about 10 minutes.
>
> - 뉴타입은 기본 타입에 부가적인 의미를 더하는 좋은 방법입니다. 예를 들어:
>   - 숫자값에 단위를 표시할 수 있음: 위에서 `Newtons`이 그 예입니다.
>   - The value passed some validation when it was created, so you no longer have to validate it again at every use: `PhoneNumber(String)` or `OddNumber(u32)`.
> - `Newtons` 타입의 값에 `f64` 값을 더하는 방법을 보여주세요.
>   - 러스트는 분명하지 않은 것을 싫어합니다. 예를 들면 자동으로 unwrap하거나 불리언 값을 정수 값으로 사용하는 것들이 그렇습니다.
>   - 연산자 재정의는 3일차 제네릭 부분에서 다룹니다.
> - 이는 [화성 기후 궤도선 (Mars Climate Orbiter)](https://en.wikipedia.org/wiki/Mars_Climate_Orbiter)의 실패 원인으로 지목된 도량형 입력 오류를 보여줍니다.
