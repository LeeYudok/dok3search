# Move 문법

(변수의) 할당은 \_소유권_을 변수 간에 이동시킵니다:

``` rust
fn main() {
    let s1: String = String::from("Hello!");
    let s2: String = s1;
    println!("s2: {s2}");
    // println!("s1: {s1}");
}
```

- `s1`을 `s2`에 할당하여 소유권을 이전시킵니다.
- `s1`의 스코프가 종료되면 아무 일도 없습니다: 왜냐하면 `s1`은 아무런 소유권이 없기 때문입니다.
- `s2`의 스코프가 종료되면 문자열 데이터는 해제됩니다.

`s2`로 이동 전 메모리:

![Stack · Heap · s1 · ptr · H · e · l · o · ! · len · 4 · capacity](../../assets/memory-management/move-diagram-1.svg)

`s2`로 이동 후 메모리:

![Stack · Heap · s1 · ptr · H · e · l · o · ! · len · 4 · capacity · s2 · (inaccessible)](../../assets/memory-management/move-diagram-2.svg)

값을 함수에 전달할때, 그 값은 매개변수에 할당됩니다. 이때 소유권의 이동이 일어납니다:

``` rust
fn say_hello(name: String) {
    println!("안녕하세요 {name}")
}

fn main() {
    let name = String::from("Alice");
    say_hello(name);
    // say_hello(name);
}
```

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> - 이는 C++과 정반대 임을 설명하세요. C++에서는 복사가 기본이고, `std::move` 를 이용해야만 (그리고 이동 생성자가 정의되어 있어야만!) 소유권 이전이 됩니다.
>
> - 실제로 이동되는 것은 소유권일 뿐입니다. 머신 코드 레벨에서 데이터 복사가 일어날 지 말 지에 대한 것은 컴파일러 내부에서 일어나는 최적화 문제입니다. 이런 복사는 최적화 과정에서 제거가 됩니다.
>
> - 정수와 같은 간단한 값들은 `Copy` (뒤에 설명합니다)로 마킹될 수 있습니다.
>
> - 러스트에서는 복사할때에는 명시적으로 `clone`을 사용합니다.
>
> `say_hello` 예:
>
> - `say_hello`함수의 첫번째 호출시 `main`함수는 자신이 가진 `name`에 대한 소유권을 포기하므로, 이후 `main`함수에서는 `name`을 사용할 수 없습니다.
> - `name`에 할당되있는 힙 메모리는 `say_hello`함수의 끝에서 해제됩니다.
> - `main`함수에서 `name`을 참조로 전달(빌림)하고(`&name`), `say_hello`에서 매개변수를 참조형으로 수정한다면 `main`함수는 `name`의 소유권을 유지할 수 있습니다.
> - 또는 첫번째 호출 시 `main`함수에서 `name`을 복제하여 전달할 수도 있습니다.(`name.clone()`)
> - 러스트는 이동을 기본으로 하고 복제를 명시적으로 선언하도록 만듬으로, 의도치 않게 복사본을 만드는 것이 C++에서보다 어렵습니다.
>
> ## 더 살펴보기
>
> ## Defensive Copies in Modern C++
>
> Modern C++은 이 문제를 다르게 해결합니다:
>
> ``` c++
> std::string s1 = "Cpp";
> std::string s2 = s1;  // s1의 데이터를 복제합니다.
> ```
>
> - `s1`의 힙 데이터는 복제되고, `s2`는 독립적인 복사본을 얻습니다.
> - `s1` 와 `s2`의 스코프가 종료되면 각각의 메모리가 해제됩니다.
>
> 복사 전:
>
> ![Stack · Heap · s1 · ptr · C · p · len · 3 · capacity](../../assets/memory-management/move-diagram-3.svg)
>
> 복사 후:
>
> ![Stack · Heap · s1 · ptr · C · p · len · 3 · capacity · s2](../../assets/memory-management/move-diagram-4.svg)
>
> 키 포인트:
>
> - C++는 Rust와 약간 다른 선택을 했습니다. `=`는 데이터를 복사하므로 문자열 데이터가 클론되어야 합니다. 그렇지 않으면 문자열 중 하나가 범위를 벗어날 때 double-free가 발생합니다.
>
> - C++에는 값을 이동할 수 있는 시점을 나타내는 데 사용되는 [`std::move`](https://en.cppreference.com/w/cpp/utility/move)도 있습니다. 예가 `s2 = std::move(s1)`이었다면 힙 할당이 발생하지 않습니다. 이동 후에는 `s1`이 유효하지만 지정되지 않은 상태가 됩니다. Rust와 달리 프로그래머는 `s1`을 계속 사용할 수 있습니다.
>
> - Rust와 달리, C++의 `=`는 복사되거나 이동되는 타입에 따라 결정된 임의의 코드를 실행할 수 있습니다.
