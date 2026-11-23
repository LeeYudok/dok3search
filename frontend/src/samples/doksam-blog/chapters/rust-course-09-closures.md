# Rust 강의 9편 — 클로저: Fn, FnMut, FnOnce 와 move

> 클로저를 받는 함수는 `Fn`, `FnMut`, `FnOnce` 가운데 무엇을 받아야 할까?

답부터 적습니다. 클로저의 종류는 캡처한 값을 어떻게 쓰느냐로 정해집니다. 읽기만 하면 `Fn`, 고치면 `FnMut`, 꺼내서 내주면 `FnOnce` 입니다. 받는 쪽은 실제로 필요한 만큼만 요구합니다. 한 번만 부르면 `FnOnce`, 여러 번 차례로 부르면 `FnMut`, 여러 스레드가 동시에 부르면 `Fn` 입니다. `move` 는 캡처를 빌릴지 가져갈지만 바꾸고, 세 트레이트 가운데 무엇이 될지는 바꾸지 않습니다.

> [그림] 로봇 셋이 가방을 들고 있다. 파란 로봇의 & 가방은 Fn, 읽기만 몇 번이든. 초록 로봇의 &mut 가방은 FnMut, 고친다 한 번에 하나씩. 빨간 로봇은 가방에서 String 을 꺼내 내주고 FnOnce, 내주면 끝 한 번만. 오른쪽 검수 로봇이 그 String 을 본다. 아래에 1억 번 호출 시간이 적혀 있다

가방은 클로저가 캡처한 값입니다. 가방을 들여다보기만 하는지, 안의 것을 고치는지, 꺼내 내주는지가 클로저의 종류를 정합니다.

Rust 강의 9편입니다. [5편](https://blog.doksam.com/blog/rust-course-05-parser-cli/)에서 rayon 이 바깥 변수를 고치는 클로저를 E0594 로 거절했고, [8편](https://blog.doksam.com/blog/rust-course-08-collections-iterators/) 연습 3의 `filter` 클로저는 `HashSet` 을 고쳤습니다. 둘의 차이를 이번 편에서 풉니다. 코드는 2026-11-18 에 rustc 1.100.0 으로 돌려 확인했습니다.

## 클로저는 캡처한 것을 담은 구조체입니다

클로저는 이름 없는 구조체와 같습니다. 필드는 캡처한 값이고, 호출하면 그 구조체의 메서드가 불립니다. 크기를 재면 바로 보입니다.

```rust
    let title = String::from("분기 보고서");
    let level = 3u64;
    let no_capture = |x: u64| x + 1;
    let by_ref = |x: u64| x + level;
    let by_move = move |x: u64| x + title.len() as u64;
```

`size_of_val` 로 잰 크기는 캡처 없음 0바이트, `level` 을 빌린 클로저 8바이트(참조 하나), `title` 을 가져간 클로저 24바이트(`String` 하나)였습니다. 캡처한 것이 곧 크기입니다.

구조체이므로 클로저마다 타입이 따로 있습니다. 모양이 비슷한 두 클로저를 `if` 와 `else` 에서 고르면 막힙니다.

```console
error[E0308]: `if` and `else` have incompatible types
...
  = note: expected closure `{closure@src/main.rs:5:9: 5:18}`
             found closure `{closure@src/main.rs:7:9: 7:18}`
  = note: no two closures, even if identical, have the same type
  = help: consider boxing your closure and/or using it as a trait object
```

권하는 대로 트레이트 객체로 감싸면 됩니다.

```rust
pub fn make_filter(min_len: usize, korean_only: bool) -> Box<dyn Fn(&str) -> bool> {
    if korean_only {
        Box::new(move |s| {
            s.chars().count() >= min_len && s.chars().all(|c| ('가'..='힣').contains(&c))
        })
    } else {
        Box::new(move |s| s.chars().count() >= min_len)
    }
}
```

두 클로저의 타입은 다르지만 둘 다 `Fn(&str) -> bool` 을 구현합니다. `Box<dyn Fn>` 은 그 공통점만 남깁니다(4편의 `Box<dyn Parser>` 와 같은 구조).

## 캡처를 쓰는 방식이 트레이트를 정합니다

```rust
    let names = vec!["표", "문단"];
    let print_all = || println!("  Fn: {names:?}"); // 읽기만
    print_all();
    print_all();
    let mut seen = 0;
    let mut count = || seen += 1; // 고친다
    count();
    count();
    let report = String::from("보고서");
    let consume = move || report; // 캡처한 값을 내보낸다
    let taken = consume();
```

| 트레이트 | 캡처를 | 부를 수 있는 횟수 | 표준 라이브러리 예 |
|---|---|---|---|
| `Fn` | 읽기만 한다 | 몇 번이든, 동시에도 | rayon `for_each` |
| `FnMut` | 고친다 | 몇 번이든, 한 번에 하나씩 | `Iterator::map`, `sort_by` |
| `FnOnce` | 꺼내 소비한다 | 한 번 | `Option::map`, `thread::spawn` |

세 트레이트는 포함 관계입니다. std 문서에 따르면 모든 `Fn` 은 `FnMut` 이고 모든 `FnMut` 는 `FnOnce` 입니다. 그래서 `FnOnce` 를 받는 함수에는 어떤 클로저든 넘길 수 있고, `Fn` 을 받는 함수에는 읽기만 하는 클로저만 넘어갑니다.

`FnOnce` 클로저를 두 번 부르면 이렇게 막힙니다.

```console
error[E0382]: use of moved value: `send`
 --> src/main.rs:5:13
  |
4 |     let a = send();
  |             ------ `send` moved due to this call
5 |     let b = send();
  |             ^^^^ value used here after move
  |
note: closure cannot be invoked more than once because it moves the variable `report` out of its environment
```

캡처한 `report` 를 첫 호출에서 내보냈으니 두 번째 호출에서 내줄 것이 없습니다. 2편의 이동 규칙이 클로저 자신에게 적용된 것입니다.

## 받는 쪽은 필요한 만큼만 요구합니다

두 번 부르는 함수가 `Fn` 을 요구하면, 고치는 클로저는 넘어가지 못합니다.

```rust
fn call_twice<F: Fn()>(f: F) {
    f();
    f();
}
```

```console
error[E0594]: cannot assign to `calls`, as it is a captured variable in a `Fn` closure
 --> src/main.rs:8:19
  |
1 | fn call_twice<F: Fn()>(f: F) {
  |                           - change this to accept `FnMut` instead of `Fn`
...
8 |     call_twice(|| calls += 1);
  |     ---------- -- ^^^^^^^^^^ cannot assign
```

컴파일러가 받는 쪽을 `FnMut` 로 바꾸라고 알려 줍니다. 차례로 부르기만 한다면 `FnMut` 로 충분하기 때문입니다. 5편의 rayon 은 사정이 다릅니다. 여러 스레드가 클로저를 동시에 부르므로 고치는 클로저를 받으면 데이터 경쟁이 됩니다. 그래서 `Fn` 을 요구했고, 같은 E0594 로 막혔습니다.

기준을 정리하면 이렇습니다. 받는 쪽이 클로저를 한 번만 부르면 `FnOnce` 를 받습니다. 7편의 `with_context(|| format!(...))` 가 그렇습니다. 실패했을 때 한 번만 부르니 무엇이든 받습니다. 여러 번 부르되 동시에 부르지 않으면 `FnMut`, 동시에 부를 수 있으면 `Fn` 입니다. 요구를 좁힐수록 호출하는 쪽의 선택지가 넓어집니다.

## move 는 빌릴지 가져갈지만 바꿉니다

스레드에 넘기는 클로저는 지역 변수를 빌릴 수 없습니다. 스레드가 함수보다 오래 살 수 있기 때문입니다.

```console
error[E0373]: closure may outlive the current function, but it borrows `name`, which is owned by the current function
 --> src/main.rs:5:32
  |
5 |     let handle = thread::spawn(|| {
  |                                ^^ may outlive borrowed value `name`
6 |         println!("처리: {name}");
  |                          ---- `name` is borrowed here
...
help: to force the closure to take ownership of `name` (and any other referenced variables), use the `move` keyword
```

`move ||` 로 바꾸면 클로저가 `name` 을 가져가고 통과합니다. 기억할 점은 `move` 가 트레이트를 바꾸지 않는다는 것입니다. 위 `make_filter` 의 두 클로저는 `move` 로 `min_len` 을 가져갔지만 읽기만 하므로 `Fn` 입니다. 테스트에서 같은 필터를 여러 번 불렀습니다. 반대로 `move` 없이도 캡처를 꺼내 내주면 `FnOnce` 가 됩니다.

클로저를 돌려줄 때는 `impl Trait` 을 씁니다. 상태를 들고 다니는 카운터가 대표적입니다.

```rust
pub fn make_counter() -> impl FnMut() -> u32 {
    let mut n = 0;
    move || {
        n += 1;
        n
    }
}
```

`n` 은 함수가 끝나면 사라질 지역 변수라 `move` 로 클로저 안에 옮겨 둡니다. 부를 때마다 그 `n` 을 고치므로 `FnMut` 입니다. `next()` 를 세 번 부르면 1, 2, 3 이 나왔습니다.

## 면접에서 이렇게 묻는다

**Q1. `Fn`, `FnMut`, `FnOnce` 는 어떻게 다른가요?**
캡처를 읽기만 하면 `Fn`, 고치면 `FnMut`, 소비하면 `FnOnce` 입니다. 모든 `Fn` 은 `FnMut` 이고 모든 `FnMut` 는 `FnOnce` 입니다. 받는 쪽은 부르는 횟수와 동시성에 맞춰 가장 약한 것을 요구합니다.

**Q2. `move` 를 붙이면 `FnOnce` 가 되나요?**
아닙니다. `move` 는 캡처를 빌림 대신 소유로 바꿀 뿐입니다. 소유한 값을 읽기만 하면 여전히 `Fn` 입니다. 스레드나 반환처럼 클로저가 지역 변수보다 오래 살 때 씁니다.

**Q3. 두 클로저를 한 변수에 담을 수 없는 이유는?**
클로저마다 컴파일러가 만든 고유 타입이 있기 때문입니다. E0308 이 "no two closures, even if identical, have the same type" 이라고 알려 줍니다. `Box<dyn Fn>` 으로 감싸거나, 캡처가 없으면 `fn` 포인터로 바꿉니다.

**Q4. `thread::spawn` 은 왜 `move` 를 요구하나요?**
`spawn` 은 `FnOnce + Send + 'static` 을 요구합니다. 스레드가 현재 함수보다 오래 살 수 있으니 지역 변수의 빌림은 `'static` 이 아닙니다. 그래서 E0373 이 나고 `move` 로 소유권을 넘겨야 합니다.

**Q5. 클로저를 인자로 받을 때 제네릭과 `&dyn Fn` 은 성능이 다른가요?**
제네릭은 클로저마다 함수가 따로 만들어져 인라인됩니다. `&dyn Fn` 과 `fn` 포인터는 대상을 모르면 간접 호출입니다. 실측에서 1억 번 호출이 28ms 대 67ms 였습니다.

## 연습

**연습 1.** 실패하면 n 번까지 다시 부르는 `retry` 를 쓰세요. 클로저 트레이트로 무엇을 요구해야 할까요?

<details>
<summary>풀이 보기</summary>

```rust
pub fn retry<T, E>(n: usize, mut f: impl FnMut() -> Result<T, E>) -> Result<T, E> {
    let mut last = f();
    for _ in 1..n {
        if last.is_ok() {
            break;
        }
        last = f();
    }
    last
}
```

여러 번 부르지만 동시에 부르지는 않으므로 `FnMut` 입니다. 시도 횟수를 세는 클로저처럼 상태를 고치는 호출자도 받을 수 있습니다. 두 번 실패하고 세 번째에 성공하는 클로저로 `Ok(3)` 과 호출 3회를 확인했습니다.

</details>

**연습 2.** 위 `make_counter` 를 `impl Fn() -> u32` 로 바꾸면 어떻게 될까요? 이유를 적으세요.

<details>
<summary>풀이 보기</summary>

컴파일되지 않습니다. `n += 1` 에서 E0594 가 나고, 컴파일러는 `change this to return FnMut instead of Fn` 이라고 반환 타입을 짚습니다. 클로저가 캡처한 `n` 을 고치므로 `Fn` 을 약속할 수 없습니다. 반환 타입은 클로저가 실제로 구현하는 것 가운데서 고릅니다. `FnMut` 이나 그보다 약한 `FnOnce` 는 됩니다. 다만 `FnOnce` 로 돌려주면 받는 쪽이 한 번밖에 못 부릅니다.

</details>

**연습 3.** E0373 이 난 스레드 코드를 고치고, 고친 뒤에도 메인 스레드가 `name` 을 쓰려면 어떻게 할지 적으세요.

<details>
<summary>풀이 보기</summary>

`thread::spawn(move || ...)` 로 고칩니다. 그러면 `name` 이 스레드로 옮겨 가 메인에서는 못 씁니다. 메인에서도 쓰려면 넘기기 전에 `let name2 = name.clone();` 으로 복제해 하나를 넘기거나, 15편의 `Arc` 로 공유합니다. 17편의 범위 스레드(`thread::scope`)를 쓰면 `move` 없이 빌려 줄 수도 있습니다.

</details>

## 한 단계 더 — 클로저 호출의 비용은 대상을 아느냐로 갈립니다

같은 곱셈을 세 방식으로 1억 번 불렀습니다(릴리스 빌드, 아홉 번 중앙값).

| 받는 방식 | 시간 |
|---|---:|
| 제네릭 `F: Fn(u64) -> u64` | 28.2~28.9ms |
| `fn` 포인터 `fn(u64) -> u64` | 66.7ms |
| 트레이트 객체 `&dyn Fn(u64) -> u64` | 66.7~66.8ms |

제네릭은 클로저 타입마다 함수가 따로 만들어지고 클로저 몸통이 반복문 안에 인라인됩니다. `fn` 포인터와 `&dyn Fn` 은 호출할 때 주소를 따라가는 간접 호출입니다.

재다가 함정도 하나 만났습니다. 처음에는 세 방식이 26.4~26.8ms 로 같았습니다. 컴파일러가 넘긴 함수와 클로저를 알아보고 간접 호출을 직접 호출로 바꿨기(devirtualization) 때문으로 보입니다. 함수에 `#[inline(never)]` 를 붙이고 넘기는 함수를 `black_box` 로 가리자 차이가 나왔습니다. 해석하면 `dyn` 의 비용은 "늘 든다"가 아니라 "컴파일러가 대상을 모를 때 든다"입니다. 실제 프로그램에서 실행 중에 고른 콜백이라면 위 표의 오른쪽에 가깝습니다. 몸통이 무거운 콜백이라면 호출 비용은 묻힙니다. 이 구조는 16편에서 vtable 과 함께 다시 봅니다.

## 정리

클로저의 종류는 캡처를 읽느냐, 고치느냐, 소비하느냐로 정해집니다. 받는 쪽은 부르는 횟수와 동시성에 맞춰 가장 약한 약속을 요구합니다. 다음에 E0594 나 E0373 을 보면 클로저를 고치기 전에 받는 쪽의 요구가 맞는지 먼저 봅니다.

10편에서는 테스트를 설계합니다. 단위·통합·문서 테스트와 프로퍼티 테스트를 나눠 쓰고, 실패를 재현할 수 있게 만드는 법을 다룹니다.
