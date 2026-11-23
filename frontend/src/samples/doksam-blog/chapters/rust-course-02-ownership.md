# Rust 강의 2편 — 소유권, 값의 주인은 하나다

> Python 에서는 `b = a` 뒤에도 `a` 를 쓸 수 있습니다. Rust 는 왜 막을까?

답부터 적습니다. Rust 는 가비지 컬렉터 없이 메모리를 해제하려고 값마다 주인을 하나로 정합니다. `let b = a;` 는 주인을 바꾸는 이동(move)입니다. 옮긴 뒤 `a` 는 빈 이름이라 쓰면 컴파일 오류가 납니다. 이동은 힙을 복사하지 않아서 쌉니다. 정말 복사가 필요하면 `clone()` 으로 드러내 씁니다.

> [그림] 검수 로봇이 돋보기로 점선만 남은 빈 상자 a 를 들여다보고, 그 위에 E0382 도장이 찍혀 있다. String 상자는 let b = a 화살표를 따라 초록 로봇 b 의 손에 있다. 오른쪽 clone 기계는 String 상자를 두 개로 만든다

그림의 숫자는 100MB 버퍼로 잰 값입니다. 이동은 겉의 24바이트만 옮겨 42나노초, `clone()` 은 힙까지 복사해 6.3밀리초가 걸렸습니다.

Rust 강의 2편입니다. [1편](https://blog.doksam.com/blog/rust-course-01-getting-started/)에서 만든 도구의 `&[u8]` 이 무엇인지 풀려면 먼저 소유권을 알아야 합니다. 빌림(`&`)은 3편에서 다룹니다. 코드는 2026-11-18 에 rustc 1.100.0 으로 돌려 확인했습니다.

## 소유권 규칙은 세 줄입니다

The Rust Programming Language(이하 The Book) 4장은 규칙을 세 줄로 적습니다.

1. 값마다 주인(owner)이 있다.
2. 주인은 한 번에 하나다.
3. 주인이 범위(scope)를 벗어나면 값은 버려진다(drop).

Python·JavaScript·Go 는 가비지 컬렉터가 쓰지 않는 값을 찾습니다. Rust 는 주인이 사라지는 지점을 컴파일할 때 알아냅니다. 그 자리에 해제 코드를 넣으므로 실행 중에 쓰레기를 찾을 필요가 없습니다. 대신 주인이 둘이 되는 코드를 컴파일러가 막습니다. 둘 다 해제하려 들면 같은 메모리를 두 번 해제하기 때문입니다.

## 대입은 복사가 아니라 이동입니다

```rust
fn main() {
    let title = String::from("분기 보고서");
    let copy = title;
    println!("{title} / {copy}");
}
```

```console
error[E0382]: borrow of moved value: `title`
 --> src/main.rs:4:16
  |
2 |     let title = String::from("분기 보고서");
  |         ----- move occurs because `title` has type `String`, which does not implement the `Copy` trait
3 |     let copy = title;
  |                ----- value moved here
4 |     println!("{title} / {copy}");
  |                ^^^^^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let copy = title.clone();
  |                     ++++++++
```

이 오류는 밑줄 세 개로 읽습니다. 2행은 왜 이동인지(`String` 은 `Copy` 가 아님), 3행은 어디서 옮겼는지, 4행은 어디서 옮긴 값을 다시 썼는지입니다. `help` 는 `clone()` 을 권하지만 "성능 비용을 받아들일 수 있다면"이라는 조건을 붙입니다. 오류 코드의 긴 설명은 `rustc --explain E0382` 로 봅니다.

함수에 값을 넘기는 것도 이동입니다. `fn count_lines(text: String)` 에 `body` 를 넘긴 뒤 `body` 를 출력하면 같은 E0382 가 나옵니다. 이번에는 컴파일러가 한 가지를 더 알려 줍니다.

```console
note: consider changing this parameter type in function `count_lines` to borrow instead if owning the value isn't necessary
 --> src/main.rs:1:22
  |
1 | fn count_lines(text: String) -> usize {
  |    -----------       ^^^^^^ this parameter takes ownership of the value
```

소유할 필요가 없으면 빌려 받으라는 뜻입니다. 고치는 방법은 연습 1에서 세 가지로 풉니다.

## 이동은 겉 24바이트만 옮깁니다

이동할 때 무엇이 옮겨지는지 크기부터 쟀습니다.

| 타입 | `size_of` (바이트) | 들어 있는 것 |
|---|---:|---|
| `String` | 24 | 힙 주소, 용량, 길이 |
| `Vec<u8>` | 24 | 힙 주소, 용량, 길이 |
| `&str` | 16 | 주소, 길이 |
| `&[u8]` | 16 | 주소, 길이 |
| `Box<[u8]>` | 16 | 힙 주소, 길이 |

`String` 의 글자는 힙에 있고, 변수에는 24바이트 머리만 있습니다. 이동 뒤에도 힙 주소가 그대로인지 확인했습니다.

```rust
    let a = String::from("분기 보고서");
    let before = a.as_ptr() as usize;
    let b = a; // 이동
    println!("  이동 후 같은 버퍼: {}", b.as_ptr() as usize == before);
    let c = b.clone();
    println!("  clone 후 같은 버퍼: {}", c.as_ptr() as usize == before);
```

```console
  이동 후 같은 버퍼: true
  clone 후 같은 버퍼: false
```

100MB `Vec<u8>` 로 시간도 쟀습니다(릴리스 빌드, 세 번). 이동은 41~167나노초로 타이머 오차 수준이었습니다. `clone()` 은 6.0~6.3밀리초였습니다. 해석하면 이렇습니다. 이동은 크기와 상관없이 머리만 옮기고, `clone()` 은 데이터 크기에 비례합니다. 그래서 E0382 를 `.clone()` 으로 덮는 습관은 리뷰에서 꼭 봅니다. 자세한 리뷰 기준은 12편에서 다룹니다.

## 빌려 주면 복사하고, 넘겨 주면 그대로 씁니다

문서 변환 엔진의 입력 타입은 파일 전체를 `Vec<u8>` 으로 소유합니다. 만드는 함수가 두 개입니다.

```rust
/// 문서 변환 엔진의 입력 타입을 줄인 것. 파일 전체를 소유한다.
struct Input {
    data: Vec<u8>,
}

impl Input {
    /// 빌린 바이트를 받는다. 소유하려면 복사해야 한다.
    fn from_bytes(data: &[u8]) -> Self {
        Self { data: data.to_vec() }
    }
    /// 소유권째 받는다. 복사하지 않는다.
    fn from_vec(data: Vec<u8>) -> Self {
        Self { data }
    }
}
```

같은 1KB 버퍼로 둘을 만들어 힙 주소를 비교하면 `from_bytes` 는 `false`, `from_vec` 은 `true` 가 나옵니다. 함수 모양이 비용을 드러냅니다. 인자가 `&[u8]` 이면 호출자가 계속 쓰고, 함수가 소유하려면 복사합니다. 인자가 `Vec<u8>` 이면 호출자가 넘겨주고 복사는 없습니다. 이미 버퍼를 가진 호출자는 `from_vec` 으로 복사를 피합니다.

## 정수와 bool 은 이동하지 않고 복사됩니다

`let y = x;` 뒤에도 `x` 를 쓸 수 있는 타입이 있습니다. `Copy` 트레이트가 붙은 타입입니다. 정수, 부동소수점, `bool`, `char`, 공유 참조 `&T`, 그리고 원소가 모두 `Copy` 인 튜플과 배열입니다. 비트를 그대로 복사해도 문제가 없는 타입들입니다.

구조체에도 `#[derive(Clone, Copy)]` 를 붙일 수 있습니다. 다만 필드가 모두 `Copy` 여야 합니다.

```rust
#[derive(Clone, Copy)]
struct Record {
    tag: u16,
    name: String,
}
```

```console
error[E0204]: the trait `Copy` cannot be implemented for this type
 --> src/main.rs:2:8
  |
1 | #[derive(Clone, Copy)]
  |                 ---- in this derive macro expansion
2 | struct Record {
  |        ^^^^^^
3 |     tag: u16,
4 |     name: String,
  |     ------------ this field does not implement `Copy`
```

`Clone` 과 `Copy` 는 다릅니다. `Clone` 은 `clone()` 을 불러야 복제하고, 힙까지 깊게 복사할 수 있습니다. `Copy` 는 대입만으로 조용히 복사되므로 싸고 단순한 타입에만 허락됩니다.

## 주인이 범위를 벗어나면 그 자리에서 해제됩니다

해제 시점을 보려고 버려질 때 이름을 찍는 타입을 만들었습니다. `Drop` 트레이트는 다른 언어의 소멸자에 해당합니다.

```rust
struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  drop {}", self.0);
    }
}

fn consume(n: Noisy) {
    println!("  consume({}) 끝", n.0);
}
```

```rust
    {
        let _first = Noisy("first");
        let _second = Noisy("second");
        let third = Noisy("third");
        consume(third);
        println!("  블록 끝");
    }
```

```console
  consume(third) 끝
  drop third
  블록 끝
  drop second
  drop first
```

`third` 는 `consume` 으로 이동했으므로 그 함수가 끝날 때 버려집니다. 블록에 남은 두 값은 선언의 역순으로 버려집니다. 나중에 만든 값이 먼저 만든 값을 참조할 수 있으니 역순이 안전합니다.

문서 변환 엔진에서 이 규칙이 하는 일은 단순합니다. 변환 함수가 끝나면 파일 전체를 담은 `Input` 이 그 자리에서 해제됩니다. 수백 개 파일을 병렬로 돌려도 메모리는 처리 중인 파일만큼만 잡힙니다. 정리할 시점을 정하는 가비지 컬렉터가 없기 때문입니다.

## String 은 소유하고 &str 은 빌려 봅니다

| 비교 | `String` | `&str` |
|---|---|---|
| 소유 | 힙 버퍼를 소유한다 | 남의 글자를 빌려 본다 |
| 크기 | 24바이트 | 16바이트 |
| 늘리기 | `push_str` 으로 늘린다 | 못 한다 |
| 흔한 출처 | `String::from`, `format!`, 파일 읽기 | 리터럴 `"..."`, `&s[..]`, `s.as_str()` |

함수 인자는 대개 `&str` 로 받습니다. `&String` 을 넘기면 컴파일러가 `&str` 로 바꿔 줍니다(역참조 강제 변환, deref coercion). 그래서 한 함수로 두 경우를 다 받습니다.

```rust
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}
```

`count_words(&owned_text)` 와 `count_words("세 낱말 문장")` 이 모두 3을 돌려줍니다.

한국어를 다룰 때 꼭 알아야 할 점이 있습니다. `String` 은 UTF-8 바이트이고, 한글 한 글자는 3바이트입니다. `"한글 문서 변환"` 은 바이트 20개, 글자 8개입니다. 바이트 위치로 자르다가 글자 가운데를 자르면 실행 중에 패닉합니다.

```rust
fn main() {
    let title = String::from("한글 문서");
    let head = &title[0..1];
    println!("{head}");
}
```

```console
thread 'main' (30115956) panicked at src/bin/slice_panic.rs:3:22:
end byte index 1 is not a char boundary; it is inside '한' (bytes 0..3 of string)
```

컴파일러가 잡지 못하는 실수입니다. 경계가 확실하지 않으면 `get(0..1)` 을 씁니다. 잘못된 경계에서 패닉하지 않고 `None` 을 돌려줍니다. 실제로 `get(0..1)` 은 `None`, `get(0..3)` 은 `Some("한")` 이었습니다.

## 면접에서 이렇게 묻는다

**Q1. 이동과 복사는 어떻게 다른가요?**
둘 다 비트를 복사합니다. 이동은 원래 이름을 더는 못 쓰게 만들고, 복사는 둘 다 쓰게 둡니다. 비트 복사만으로 안전한 타입만 `Copy` 를 구현할 수 있고, 필드가 모두 `Copy` 여야 합니다.

**Q2. `String` 을 이동하면 문자열 데이터가 복사되나요?**
아닙니다. 24바이트 머리(주소·용량·길이)만 복사되고 힙 버퍼는 그대로입니다. 이동 전후 `as_ptr()` 이 같고, 100MB 에서도 이동은 나노초 단위입니다. 데이터 복사는 `clone()` 이 합니다.

**Q3. 함수 인자로 `String` 과 `&str` 중 무엇을 받나요?**
읽기만 하면 `&str` 입니다. `&String` 과 리터럴을 모두 받습니다. 함수가 값을 저장하거나 돌려줄 때처럼 소유가 필요할 때만 `String` 을 받습니다. 그러면 호출자가 복사할지 넘길지 고릅니다.

**Q4. `drop` 은 언제, 어떤 순서로 불리나요?**
주인이 범위를 벗어날 때 불립니다. 같은 범위의 지역 변수는 선언의 역순입니다. 이동된 값은 새 주인에서 버려집니다. `std::mem::drop(x)` 로 일찍 버릴 수도 있습니다.

**Q5. `Copy` 와 `Drop` 을 함께 구현할 수 있나요?**
없습니다. E0184 입니다. 해제할 자원이 있는 값을 조용히 복사하면 같은 자원을 두 번 해제하게 됩니다.

## 연습

**연습 1.** 아래 코드의 E0382 를 세 가지 방법으로 고치고, 각 방법의 비용을 한 줄씩 적으세요.

```rust
fn count_lines(text: String) -> usize {
    text.lines().count()
}

fn main() {
    let body = String::from("첫 줄\n둘째 줄");
    let n = count_lines(body);
    println!("{n}줄: {body}");
}
```

<details>
<summary>풀이 보기</summary>

```rust
fn count_lines_borrow(text: &str) -> usize {
    text.lines().count()
}

fn count_lines_return(text: String) -> (usize, String) {
    (text.lines().count(), text)
}
```

1. 빌려 받는다. `count_lines_borrow(&body)`. 복사가 없고 가장 흔한 답입니다.
2. 복제해서 넘긴다. `count_lines(body.clone())`. 힙 버퍼를 하나 더 만듭니다.
3. 소유권을 돌려받는다. `let (n, body) = count_lines_return(body);`. 복사는 없지만 모양이 어색해서 잘 쓰지 않습니다.

세 방법 모두 `2줄: 첫 줄` 과 `둘째 줄` 을 찍었습니다.

</details>

**연습 2.** 앞에서부터 글자 n 개를 빌려 주는 `fn prefix_chars(s: &str, n: usize) -> &str` 을 쓰세요. 한글 중간에서 패닉하면 안 되고, n 이 글자 수보다 크면 전체를 돌려줍니다.

<details>
<summary>풀이 보기</summary>

```rust
pub fn prefix_chars(s: &str, n: usize) -> &str {
    match s.char_indices().nth(n) {
        Some((idx, _)) => &s[..idx],
        None => s,
    }
}

#[test]
fn cuts_on_char_boundary() {
    assert_eq!(prefix_chars("한글 문서", 2), "한글");
    assert_eq!(prefix_chars("한글 문서", 0), "");
    assert_eq!(prefix_chars("한글", 10), "한글");
    assert_eq!(prefix_chars("abc", 2), "ab");
}
```

`char_indices()` 는 글자마다 (바이트 위치, 글자)를 줍니다. n 번째 글자의 바이트 위치는 늘 글자 경계라 자르기가 안전합니다. 돌려주는 `&str` 은 새 문자열이 아니라 `s` 의 일부를 빌린 것입니다.

</details>

**연습 3.** 무엇이 어떤 순서로 찍힐지 먼저 적고 돌려 보세요.

```rust
    let a = Noisy("a");
    let b = Noisy("b");
    let _c = a; // 이동
    drop(b);
    println!("끝");
```

<details>
<summary>풀이 보기</summary>

```console
drop b
끝
drop a
```

`b` 는 `drop(b)` 에서 이동되어 바로 버려집니다. `a` 의 값은 `_c` 로 옮겨졌으니 함수 끝에서 `_c` 가 버릴 때 `drop a` 가 찍힙니다. 이름 `a` 는 이미 비어 있어 따로 버리지 않습니다.

</details>

## 한 단계 더 — 조건에 따라 이동하면 누가 해제할까

이동이 조건문 안에만 있으면 컴파일러는 실행 전에 해제 여부를 알 수 없습니다.

```rust
    for give_away in [true, false] {
        let n = Noisy(if give_away {
            "넘긴 값"
        } else {
            "남은 값"
        });
        if give_away {
            consume(n);
        }
        println!("  반복 끝 (give_away={give_away})");
    }
```

```console
  consume(넘긴 값) 끝
  drop 넘긴 값
  반복 끝 (give_away=true)
  반복 끝 (give_away=false)
  drop 남은 값
```

넘긴 값은 `consume` 이 끝날 때 한 번만 버려졌습니다. 남은 값은 반복이 끝날 때 버려졌습니다. The Rustonomicon 의 "Drop Flags" 절에 따르면 이럴 때 컴파일러는 스택에 숨은 플래그를 두고 실행 중에 확인합니다. 이동이 조건 없이 일어나면 플래그 없이 정적으로 처리합니다. 해석하면 소유권은 대부분 컴파일할 때 끝나는 규칙이지만, 조건부 이동에는 작은 실행 비용이 붙습니다. 플래그의 실제 모양은 기계어로 확인하지 않았고, 이 설명은 문서에 근거합니다.

## 정리

Rust 가 `b = a` 뒤의 `a` 를 막는 이유는 주인을 하나로 두어 해제 시점을 컴파일할 때 정하기 위해서입니다. 이동은 머리 24바이트만 옮기므로 싸고, 비싼 것은 `clone()` 입니다. 다음에 E0382 를 보면 `.clone()` 을 붙이기 전에 빌려 줄 수 있는지 먼저 봅니다.

3편에서는 그 빌림을 다룹니다. 여럿이 읽는 `&` 와 혼자 고치는 `&mut` 이 왜 동시에 있을 수 없는지, 수명 표기가 무엇을 약속하는지 컴파일 오류로 봅니다.
