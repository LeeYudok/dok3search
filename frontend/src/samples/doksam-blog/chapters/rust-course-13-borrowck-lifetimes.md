# Rust 강의 13편 — 빌림 검사기와 수명 깊게

> 빌림 검사기는 정확히 어디까지 보고, 어디서부터 사람이 알려 줘야 할까?

답부터 적습니다. 빌림 검사기는 함수 하나 안의 흐름을 봅니다. 그 안에서는 필드 단위로 나눠 빌리는 것까지 알아봅니다. 함수 경계에서는 시그니처만 봅니다. `&mut self` 를 받는 메서드는 "구조체 전체를 빌린다"는 약속이라, 필드 둘을 메서드 두 개로 빌리면 막힙니다. 시그니처의 수명은 대부분 생략 규칙이 채우고, 규칙이 못 정하는 관계만 사람이 `'a` 로 적습니다. `'static` 은 두 가지 뜻으로 씁니다. `&'static T` 는 프로그램 끝까지 사는 참조이고, `T: 'static` 은 빌린 것을 품지 않은 타입입니다.

> [그림] 왼쪽 서랍장의 head 와 body 서랍에 &mut 대여증이 따로 붙어 통과한다. 오른쪽에는 같은 서랍장이 head_mut 과 body_mut 문 뒤에 있고 둘 다 &mut self 라 두 번째 문에 E0499 도장이 찍혀 있다. 가운데 검수 로봇이 문의 이름표를 본다. 아래에 'static 의 두 뜻이 적혀 있다

그림의 두 서랍장은 같은 구조체입니다. 차이는 필드를 직접 빌렸느냐, 메서드라는 문을 거쳤느냐뿐입니다.

Rust 강의 13편이고 심화 단계의 첫 편입니다. [3편](https://blog.doksam.com/blog/rust-course-03-borrowing/)에서 빌림 규칙, NLL, 생략 규칙, 두 단계 빌림을 맛봤습니다. 이번 편은 그 규칙이 어디서 끝나는지 봅니다. 코드는 2026-11-18 에 rustc 1.100.0 으로 돌려 확인했습니다.

## 함수 안에서는 필드 단위로 나눠 빌립니다

```rust
struct Doc {
    title: String,
    head: Vec<String>,
    body: Vec<String>,
    text: String,
}
```

필드를 직접 빌리면 두 `&mut` 이 공존합니다. 빌림 검사기가 `doc.head` 와 `doc.body` 가 겹치지 않는 메모리라는 것을 알기 때문입니다.

```rust
    let h = &mut doc.head;
    let b = &mut doc.body;
    h.push("제목".into());
    b.push("본문".into());
```

같은 일을 메서드로 하면 막힙니다.

```console
error[E0499]: cannot borrow `doc` as mutable more than once at a time
  --> src/main.rs:24:13
   |
23 |     let h = doc.head_mut();
   |             --- first mutable borrow occurs here
24 |     let b = doc.body_mut();
   |             ^^^ second mutable borrow occurs here
25 |     h.push("부제".into());
   |     - first borrow later used here
```

`fn head_mut(&mut self) -> &mut Vec<String>` 의 시그니처는 "`self` 전체를 고치는 빌림을 받고, 그 수명만큼 사는 참조를 준다"입니다. 몸통이 `head` 만 건드린다는 사실은 시그니처에 없습니다. 빌림 검사기는 호출하는 쪽을 볼 때 몸통을 열어 보지 않습니다. 그래서 두 호출이 모두 `doc` 전체를 빌린 것으로 봅니다.

고치는 길은 시그니처에 "둘로 나눠 준다"를 적는 것입니다.

```rust
    fn parts_mut(&mut self) -> (&mut Vec<String>, &mut Vec<String>) {
        (&mut self.head, &mut self.body)
    }
```

몸통 안에서는 필드 단위 분리 빌림이 되니, 메서드 하나가 둘을 한꺼번에 돌려줍니다. 표준 라이브러리의 `split_at_mut` 도 같은 생각입니다. `&mut sizes[0]` 과 `&mut sizes[1]` 은 인덱스가 다른데도 E0499 입니다. 인덱싱은 함수 호출(`IndexMut`)이라 벡터 전체를 빌리기 때문입니다. 컴파일러가 직접 권합니다.

```console
  = help: use `.split_at_mut(position)` to obtain two mutable non-overlapping sub-slices
```

`let (left, right) = sizes.split_at_mut(1);` 로 고치면 `[11, 21, 30]` 이 됐습니다.

## 빌린 값에서는 꺼낼 수 없고, 바꿔 넣을 수 있습니다

`&mut Doc` 을 받은 함수가 제목을 가져가려 하면 막힙니다.

```console
error[E0507]: cannot move out of `doc.title` which is behind a mutable reference
 --> src/main.rs:6:5
  |
6 |     doc.title
  |     ^^^^^^^^^ move occurs because `doc.title` has type `String`, which does not implement the `Copy` trait
```

꺼내 가면 빌려 준 쪽에 빈 자리가 남습니다. 빌림이 끝나면 주인이 그 필드를 쓸 텐데 값이 없습니다. 컴파일러는 `clone()` 을 권하지만, 복사 없이 꺼내는 길이 있습니다.

```rust
fn take_title(doc: &mut Doc) -> String {
    std::mem::take(&mut doc.title)
}
```

`mem::take` 는 기본값(빈 `String`)을 넣고 원래 값을 돌려줍니다. 실행하면 꺼낸 제목은 `"분기 보고서"`, 남은 제목은 `""` 이었습니다. 기본값이 없는 타입은 `mem::replace(&mut x, 새 값)` 을, `Option` 필드는 `.take()` 를 씁니다. 빈자리를 만들지 않는다는 규칙은 같습니다.

## 생략 규칙이 틀린 관계를 정하면 컴파일러가 묻습니다

3편의 생략 규칙 3은 "`&self` 가 있으면 결과는 `self` 의 수명을 받는다"입니다. 결과가 다른 인자에서 올 수도 있는 메서드에서는 이 규칙이 틀립니다.

```rust
    fn first_line(&self, fallback: &str) -> &str {
        match self.text.lines().next() {
            Some(line) => line,
            None => fallback,
        }
    }
```

```console
error: lifetime may not live long enough
...
   | |_________^ method was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
   |
help: consider introducing a named lifetime parameter and update trait if needed
   |
 7 |     fn first_line<'a>(&self, fallback: &'a str) -> &'a str {
```

이 오류에는 E 코드가 없습니다. 컴파일러가 수명마다 `'1`, `'2` 라는 임시 이름을 붙여 설명합니다. 그런데 권하는 대로 `fallback` 에만 `'a` 를 붙이면 이번에는 `self.text` 쪽 갈래가 같은 오류를 냅니다. 결과는 두 곳 어디서든 올 수 있으니 둘 다 묶어야 합니다.

```rust
    fn first_line<'a>(&'a self, fallback: &'a str) -> &'a str {
        self.text.lines().next().unwrap_or(fallback)
    }
```

3편 연습 2에서는 컴파일러의 제안이 실제보다 넓었습니다. 이번에는 좁았습니다. 해석하면 수명 제안은 출발점일 뿐이고, 결과가 어디서 오는지 따지는 일은 사람이 해야 합니다.

## 'static 은 두 가지로 읽습니다

| 표기 | 뜻 | 예 |
|---|---|---|
| `&'static T` | 프로그램 끝까지 사는 참조 | 문자열 리터럴, `Box::leak` 의 결과 |
| `T: 'static` | `T` 안에 `'static` 보다 짧은 빌림이 없다 | `String`, `Vec<u8>`, `u32`. 소유한 값은 모두 해당 |

둘째 뜻을 헷갈리면 "`'static` 이면 영원히 사는 값"이라고 오해합니다. `T: 'static` 인 `String` 도 drop 되면 해제됩니다. 조건은 "빌린 것이 없다"입니다. 그래서 소유한 값은 넘어가고 지역 변수의 참조는 막힙니다.

```rust
fn keep_forever<T: Debug + 'static>(t: T) -> Box<dyn Debug> {
    Box::new(t)
}
```

```console
error[E0597]: `local` does not live long enough
...
   |                      argument requires that `local` is borrowed for `'static`
...
note: requirement that the value outlives `'static` introduced here
  --> src/main.rs:3:28
   |
 3 | fn keep_forever<T: Debug + 'static>(t: T) -> Box<dyn Debug> {
```

`keep_forever(owned)` 는 통과하고 `keep_forever(&local)` 만 막혔습니다. `thread::spawn` 이 바로 이 조건을 겁니다. 빌린 `&str` 을 스레드로 넘기면 E0521 입니다.

```console
error[E0521]: borrowed data escapes outside of function
...
  |             `name` escapes the function body here
  |             argument requires that `'1` must outlive `'static`
```

`Box::leak` 은 첫째 뜻의 값을 일부러 만듭니다. 힙에 둔 값을 해제하지 않기로 하고 `&'static` 을 받습니다. 프로그램이 끝날 때까지 한 번만 만드는 설정 값 같은 곳에 씁니다. 반복해서 부르면 그만큼 메모리가 샙니다.

## NLL 도 아직 못 푸는 모양이 있습니다

조건에 따라 빌린 값을 돌려주거나, 아니면 고친 뒤 다시 빌려 주는 함수입니다.

```rust
fn get_or_insert(map: &mut HashMap<u32, String>) -> &String {
    if let Some(v) = map.get(&1) {
        return v;
    }
    map.insert(1, String::from("기본값"));
    map.get(&1).unwrap()
}
```

```console
error[E0502]: cannot borrow `*map` as mutable because it is also borrowed as immutable
...
4 |     if let Some(v) = map.get(&1) {
  |                      --- immutable borrow occurs here
5 |         return v;
  |                - returning this value requires that `*map` is borrowed for `'1`
...
7 |     map.insert(1, String::from("기본값"));
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
```

사람이 보면 `return v` 로 나간 경우에만 빌림이 이어지니 7행에서는 빌림이 없습니다. 지금의 빌림 검사기는 돌려준 빌림의 수명을 함수 전체에 걸린 `'1` 로 보고, 그 갈래를 나눠 따지지 못합니다. rustc 1.100.0 에서도 그대로 오류였습니다. Rust 프로젝트는 이 모양을 "NLL problem case #3"이라 부르고, 차세대 빌림 검사기(Polonius)의 목표로 둡니다. 지금은 모양을 바꿔 피합니다.

```rust
fn get_or_insert(map: &mut HashMap<u32, String>) -> &String {
    map.entry(1).or_insert_with(|| String::from("기본값"))
}
```

`entry` 는 "찾고 없으면 넣기"를 빌림 하나로 끝냅니다(8편).

## 면접에서 이렇게 묻는다

**Q1. 구조체의 두 필드를 `&mut` 로 동시에 빌릴 수 있나요?**
필드를 직접 빌리면 됩니다. 빌림 검사기가 필드 단위로 겹치지 않음을 압니다. `&mut self` 메서드 두 개로 빌리면 각각이 구조체 전체를 빌려 E0499 입니다. 둘을 한꺼번에 돌려주는 메서드를 만들거나 `split_at_mut` 같은 API 를 씁니다.

**Q2. `&mut` 로 받은 값의 필드를 가져가려면?**
그냥 옮기면 E0507 입니다. 빈자리를 남기지 않도록 `mem::take`, `mem::replace`, `Option::take` 로 바꿔 넣으며 꺼냅니다.

**Q3. 수명 생략 규칙이 틀린 결과를 낼 때는?**
결과가 `self` 가 아닌 인자에서도 올 수 있는 메서드가 대표적입니다. 결과가 올 수 있는 모든 입력을 같은 `'a` 로 묶습니다. 컴파일러 제안이 좁거나 넓을 수 있어 결과의 출처를 직접 따집니다.

**Q4. `'static` 은 무슨 뜻인가요?**
`&'static T` 는 프로그램 끝까지 사는 참조입니다. `T: 'static` 은 `T` 에 짧은 빌림이 없다는 조건이라 `String` 같은 소유 값도 만족합니다. `thread::spawn` 이 두 번째 조건을 요구합니다.

**Q5. NLL 이 못 푸는 경우를 아나요?**
조건에 따라 빌린 값을 돌려주고, 아니면 고친 뒤 다시 빌려 주는 모양입니다. 돌려준 빌림이 함수 전체에 걸려 E0502 가 납니다. `entry` 같은 API 로 모양을 바꿔 피합니다.

## 연습

**연습 1.** `Doc` 에 `title` 과 `body` 를 동시에 고칠 수 있게 하는 메서드를 쓰세요.

<details>
<summary>풀이 보기</summary>

```rust
    fn title_and_body_mut(&mut self) -> (&mut String, &mut Vec<String>) {
        (&mut self.title, &mut self.body)
    }
```

본문의 `parts_mut` 와 같은 모양입니다. 메서드 몸통 안에서는 필드 단위로 나눠 빌릴 수 있으니, 그 결과를 튜플로 묶어 시그니처에 드러냅니다.

</details>

**연습 2.** `Option<String>` 필드를 `&mut self` 메서드에서 꺼내 돌려주는 `take_note` 를 쓰세요.

<details>
<summary>풀이 보기</summary>

```rust
    fn take_note(&mut self) -> Option<String> {
        self.note.take()
    }
```

`Option::take` 는 `None` 을 넣고 원래 값을 돌려줍니다. `mem::take(&mut self.note)` 와 같습니다. 두 번째 호출은 `None` 을 돌려줍니다.

</details>

**연습 3.** 다음 함수는 왜 컴파일되지 않을까요? `fn longest<'a>(items: &'a [String]) -> &'a str { let joined = items.join(","); &joined }`

<details>
<summary>풀이 보기</summary>

E0515(`cannot return reference to local variable joined`)입니다. `joined` 는 함수 안에서 만든 `String` 이라 함수가 끝나면 해제됩니다. 시그니처는 결과가 `items` 만큼 산다고 약속했지만 실제로는 지역 값을 빌립니다. `'a` 를 붙여도 값의 수명은 늘지 않습니다(3편). 새로 만든 값은 `String` 으로 돌려줍니다.

</details>

## 한 단계 더 — 함수 단위로 보는 것은 설계입니다

빌림 검사기가 몸통을 열어 보지 않는 데는 이유가 있습니다. 시그니처만 보면 함수 몸통을 고쳐도 호출하는 쪽의 검사 결과가 바뀌지 않습니다. 라이브러리가 내부를 바꿔도 사용자 코드가 갑자기 깨지지 않습니다. 컴파일도 함수 단위로 나눠 할 수 있습니다. 대가는 이 편에서 본 분리 빌림의 손실입니다. 시그니처가 실제보다 넓게 빌리면 호출하는 쪽이 그만큼 묶입니다.

그래서 수명과 `&mut` 는 API 설계 문제이기도 합니다. `head_mut` 과 `body_mut` 을 따로 두면 각각은 단순하지만 함께 쓸 수 없습니다. `parts_mut` 은 함께 쓸 수 있지만 모양이 덜 자연스럽습니다. 표준 라이브러리가 `split_at_mut`, `iter_mut`, `HashMap::get_disjoint_mut` 같은 "나눠 주는" API 를 따로 두는 이유입니다. 이 해석은 rustc 동작과 표준 라이브러리 API 모양에서 끌어낸 것이고, 설계 의도를 밝힌 공식 문서를 근거로 한 것은 아닙니다.

## 정리

빌림 검사기는 함수 안에서는 필드 단위까지 보고, 경계에서는 시그니처만 봅니다. 그래서 무엇을 얼마나 빌리는지를 시그니처에 정확히 적는 것이 사람의 몫입니다. `'static` 은 "끝까지 사는 참조"와 "빌린 것 없는 타입" 두 뜻으로 나눠 읽습니다.

14편에서는 스마트 포인터를 다룹니다. `Box`, `Rc`, `Weak` 로 소유자가 여럿인 데이터와 참조 순환을 다루고 `Drop` 의 순서를 봅니다.
