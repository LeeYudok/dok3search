# Rust 강의 3편 — 빌림과 수명 첫걸음

> 값을 옮기지 않고 함수에 보여 주려면 어떻게 할까? 그때 컴파일러는 무엇을 검사할까?

답부터 적습니다. `&` 로 빌려 줍니다. 컴파일러는 두 가지를 봅니다. 하나, 같은 때에 읽는 빌림 여럿이나 고치는 빌림 하나만 허락합니다. 둘, 빌림은 주인보다 오래 살 수 없습니다. 함수에 붙는 수명 표기 `'a` 는 두 번째 규칙을 함수 경계 너머로 전하는 약속입니다. 값의 수명을 늘리지는 않습니다.

> [그림] 가운데 검수 로봇이 문서를 들고 있다. 왼쪽 파란 로봇 둘은 & 보기 대여증을 들고 문서를 함께 본다. 오른쪽 빨간 로봇은 &mut 고치기 대여증을 들고 보는 중엔 대기, E0502 표지판 앞에서 기다린다

그림처럼 문서의 주인은 하나이고 대여증은 두 종류입니다. 보기 대여증은 여러 장 나가지만, 고치기 대여증은 보기 대여증이 모두 돌아와야 나갑니다.

Rust 강의 3편입니다. [2편](https://blog.doksam.com/blog/rust-course-02-ownership/)에서 값을 옮기는 이동을 봤습니다. 이번 편은 옮기지 않고 빌리는 법입니다. 수명의 깊은 규칙은 13편에서 다시 다룹니다. 코드는 2026-11-18 에 rustc 1.100.0 으로 돌려 확인했습니다.

## 빌림은 & 로, 고치는 빌림은 &mut 로 만듭니다

`&x` 는 `x` 를 빌린 참조(reference)를 만듭니다. 고쳐야 하면 `&mut x` 를 넘깁니다. 이때 변수도 `let mut` 으로 선언해야 합니다.

```rust
fn append_footer(doc: &mut String) {
    doc.push_str("\n-- 끝 --");
}

fn main() {
    let doc = String::from("본문");
    append_footer(&mut doc);
    println!("{doc}");
}
```

```console
error[E0596]: cannot borrow `doc` as mutable, as it is not declared as mutable
 --> src/main.rs:7:19
  |
7 |     append_footer(&mut doc);
  |                   ^^^^^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
6 |     let mut doc = String::from("본문");
  |         +++
```

`let mut doc` 으로 고치면 `본문` 다음 줄에 `-- 끝 --` 가 찍힙니다. 호출하는 쪽에도 `&mut` 를 적게 한 것은 일부러입니다. 코드를 읽는 사람이 이 줄에서 값이 바뀐다는 것을 바로 압니다.

## 읽는 빌림과 고치는 빌림은 겹칠 수 없습니다

The Book 4.2절은 규칙을 이렇게 적습니다. 어느 때든 `&mut` 하나, 또는 `&` 여럿 가운데 하나만 가질 수 있습니다. 가장 흔히 걸리는 곳은 목록을 돌면서 그 목록을 고칠 때입니다.

```rust
fn main() {
    let mut tags: Vec<u16> = vec![66, 67, 68];
    for t in &tags {
        if *t == 67 {
            tags.push(80); // 표 머리 뒤에 표 본문 태그를 더하려 한다
        }
    }
    println!("{tags:?}");
}
```

```console
error[E0502]: cannot borrow `tags` as mutable because it is also borrowed as immutable
 --> src/main.rs:5:13
  |
3 |       for t in &tags {
  |       -        -----
  |       |        |
  |       |        immutable borrow occurs here
  |  _____|        immutable borrow later used here
  | |
4 | |         if *t == 67 {
5 | |             tags.push(80); // 표 머리 뒤에 표 본문 태그를 더하려 한다
  | |             ^^^^^^^^^^^^^ mutable borrow occurs here
6 | |         }
7 | |     }
  | |_____- this for loop borrows `tags` immutably, preventing mutation within its body
  |
  = help: consider using an index-based loop instead, or collecting modifications into a separate collection
```

읽는 순서는 이렇습니다. 3행에서 `&tags` 로 읽는 빌림이 생기고, 반복 내내 쓰입니다. 그 사이 5행이 고치는 빌림을 만들려 해서 막혔습니다. `help` 는 바꿀 것을 따로 모으라고 권합니다. 연습 1에서 그렇게 고칩니다.

`&mut` 두 개도 같은 이유로 막힙니다. `let a = &mut out; let b = &mut out;` 뒤에 `a` 를 쓰면 E0499(`cannot borrow out as mutable more than once at a time`)가 납니다.

## 막는 이유는 push 가 버퍼를 옮기기 때문입니다

규칙이 지나쳐 보이면 버퍼에 무슨 일이 생기는지 봅니다. 용량 4인 `Vec<u64>` 를 채운 뒤 하나를 더 넣고 힙 주소를 비교했습니다.

```rust
    let mut w: Vec<u64> = Vec::with_capacity(4);
    w.extend([1, 2, 3, 4]);
    let before = (w.as_ptr() as usize, w.capacity());
    w.push(5);
    let after = (w.as_ptr() as usize, w.capacity());
```

```console
용량 4 → 8, 버퍼 주소 같음: false
```

용량이 모자라면 `push` 는 더 큰 버퍼를 새로 잡고 원소를 옮긴 뒤 옛 버퍼를 해제합니다. 이번 실행에서는 주소가 바뀌었습니다. 할당기가 제자리에서 늘려 주면 같을 수도 있습니다. 어느 쪽이든 반복자는 옛 버퍼를 가리키고 있을 수 있습니다. 해제된 메모리를 읽는 버그를 컴파일러가 빌림 규칙 하나로 막는 셈입니다. C++ 에서는 같은 코드가 컴파일되고 반복자 무효화(iterator invalidation)라는 이름의 버그가 됩니다.

## 빌림은 마지막으로 쓴 곳까지만 이어집니다

빌림이 언제 끝나는지도 알아야 합니다. 아래 코드는 컴파일됩니다.

```rust
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    println!("첫 값 {first}");
    v.push(4); // first 는 위에서 마지막으로 쓰였다
```

`first` 의 빌림은 블록 끝이 아니라 마지막으로 쓴 `println!` 에서 끝납니다. 2018 에디션부터 들어온 NLL(non-lexical lifetimes, 어휘 범위에 묶이지 않는 수명)입니다. 순서를 바꿔 `push` 뒤에 `first` 를 쓰면 다시 E0502 가 납니다. 오류는 그 사용 위치를 `immutable borrow later used here` 로 짚습니다. 그래서 빌림 오류를 고칠 때는 "이 빌림을 마지막으로 쓰는 곳을 앞당길 수 있나"를 먼저 봅니다.

## 빌림은 주인보다 오래 살 수 없습니다

```rust
fn main() {
    let r;
    {
        let s = String::from("잠깐 사는 값");
        r = &s;
    }
    println!("{r}");
}
```

```console
error[E0597]: `s` does not live long enough
 --> src/main.rs:5:13
  |
4 |         let s = String::from("잠깐 사는 값");
  |             - binding `s` declared here
5 |         r = &s;
  |             ^^ borrowed value does not live long enough
6 |     }
  |     - `s` dropped here while still borrowed
7 |     println!("{r}");
  |                - borrow later used here
```

네 줄의 밑줄이 이야기 하나를 이룹니다. 선언, 빌림, 해제, 해제 뒤 사용입니다. 함수 안에서 만든 `String` 의 참조를 돌려주려 해도 같은 문제입니다. `fn make_title() -> &str` 을 쓰면 E0106 이 나고, 컴파일러는 `'static` 보다 `String` 을 돌려주라고 권합니다. 함수 안에서 만든 값은 소유권째 돌려줍니다.

## 수명 표기는 관계를 적을 뿐 수명을 늘리지 않습니다

둘 가운데 긴 문자열을 돌려주는 함수를 표기 없이 쓰면 컴파일러가 묻습니다.

```console
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:32
  |
1 | fn longer(a: &str, b: &str) -> &str {
  |              ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `a` or `b`
help: consider introducing a named lifetime parameter
  |
1 | fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
  |          ++++     ++          ++          ++
```

결과가 `a` 에서 왔는지 `b` 에서 왔는지 시그니처만으로는 알 수 없다는 뜻입니다. 권하는 대로 `'a` 를 붙이면 "결과는 `a` 와 `b` 가 둘 다 살아 있는 동안만 쓴다"는 약속이 됩니다. 호출하는 쪽이 이 약속을 어기면 그쪽에서 E0597 이 납니다. `b` 로 넘긴 `sub` 가 안쪽 블록에서 사라졌는데 결과를 블록 밖에서 쓰면 `` `sub` does not live long enough `` 입니다.

모든 함수에 표기를 붙이지는 않습니다. 컴파일러가 채워 넣는 수명 생략(lifetime elision) 규칙이 세 가지 있습니다(The Book 10.3절).

1. 참조 인자마다 각자의 수명을 받는다.
2. 참조 인자가 하나뿐이면 결과는 그 수명을 받는다.
3. 메서드에 `&self` 나 `&mut self` 가 있으면 결과는 `self` 의 수명을 받는다.

`longer` 는 참조 인자가 둘이고 `self` 도 없어서 세 규칙으로 정해지지 않았습니다. 그래서 물어본 것입니다.

## 구조체도 빌린 것을 품을 수 있습니다

문서 파서는 입력 버퍼에서 레코드를 잘라 냅니다. 본문을 복사하지 않고 버퍼를 빌리면 구조체에 수명 인자가 붙습니다.

```rust
/// 레코드 하나. 본문을 복사하지 않고 입력 버퍼를 빌린다.
struct Record<'a> {
    tag: u16,
    body: &'a [u8],
}

fn first_record(buf: &[u8]) -> Record<'_> {
    Record {
        tag: u16::from_le_bytes([buf[0], buf[1]]),
        body: &buf[2..],
    }
}
```

`Record<'_>` 의 `'_` 는 생략 규칙 2로 `buf` 의 수명을 받습니다. 5바이트 버퍼로 돌리면 `tag=66 본문 3바이트, 버퍼 안을 가리킴: true` 가 나옵니다. 복사가 없다는 뜻입니다. 대신 버퍼가 먼저 사라지면 `` `buf` does not live long enough `` 로 막힙니다.

문서 변환 엔진은 이 자리에서 반대를 골랐습니다. 레코드를 만들 때 `data[pos..pos + size].to_vec()` 로 본문을 복사합니다. 레코드가 버퍼와 따로 살 수 있어 수명 인자가 구조체 전체로 번지지 않습니다. 복사 비용과 코드 단순함을 맞바꾼 선택입니다. 그 비용은 5편에서 잽니다.

## 오류 코드 지도

| 코드 | 뜻 | 먼저 볼 것 |
|---|---|---|
| E0382 | 옮긴 값을 다시 썼다 | 빌려 줄 수 있나, `clone` 이 꼭 필요한가 |
| E0596 | `mut` 아닌 변수를 `&mut` 로 빌렸다 | `let mut` |
| E0499 | `&mut` 가 둘 겹쳤다 | 하나를 먼저 끝낼 수 있나 |
| E0502 | `&` 와 `&mut` 가 겹쳤다 | 마지막 사용을 앞당기거나 바꿀 것을 따로 모은다 |
| E0597 | 빌린 값이 먼저 사라졌다 | 주인을 더 바깥에 두거나 소유한 값으로 바꾼다 |
| E0106 | 결과 참조의 출처를 모른다 | 어느 인자에서 빌리는지 `'a` 로 적는다 |

## 면접에서 이렇게 묻는다

**Q1. 빌림 검사기의 규칙을 설명해 주세요.**
어느 때든 `&mut` 하나 또는 `&` 여럿만 있을 수 있습니다. 그리고 참조는 가리키는 값보다 오래 살 수 없습니다. 첫째는 데이터 경쟁과 반복자 무효화를, 둘째는 댕글링 참조를 막습니다.

**Q2. 벡터를 돌면서 `push` 하면 왜 막히나요?**
`push` 가 용량을 넘으면 버퍼를 새로 잡고 옛 버퍼를 해제합니다. 반복자는 옛 버퍼를 가리키므로 해제된 메모리를 읽게 됩니다. 실험에서 용량이 4에서 8로 늘며 주소가 바뀌었습니다.

**Q3. NLL 은 무엇인가요?**
빌림이 변수의 범위 끝이 아니라 마지막 사용 지점에서 끝나게 하는 분석입니다. 2018 에디션부터 들어왔습니다. 그 전에는 같은 코드가 블록 끝까지 빌림이 이어져 오류였습니다.

**Q4. 수명 표기 `'a` 는 값을 더 오래 살게 하나요?**
아닙니다. 값의 수명은 주인이 정합니다. `'a` 는 입력과 출력 참조 사이의 관계를 적어, 호출하는 쪽에서 컴파일러가 검사할 수 있게 합니다.

**Q5. 수명 생략 규칙을 말해 보세요.**
참조 인자마다 수명을 하나씩 받습니다. 참조 인자가 하나면 출력이 그 수명을 받습니다. `&self` 가 있으면 출력이 `self` 의 수명을 받습니다. 셋으로 정해지지 않으면 직접 적어야 합니다.

## 연습

**연습 1.** 위 E0502 코드를 고쳐 `[66, 67, 68, 80]` 을 찍게 하세요. 바꿀 것을 따로 모으는 방법을 씁니다.

<details>
<summary>풀이 보기</summary>

```rust
    let mut tags: Vec<u16> = vec![66, 67, 68];
    let extra: Vec<u16> = tags.iter().filter(|&&t| t == 67).map(|_| 80).collect();
    tags.extend(extra);
    println!("{tags:?}");
```

읽는 빌림(`tags.iter()`)은 `collect()` 에서 끝납니다. 그 뒤의 `extend` 는 고치는 빌림만 씁니다. 두 빌림이 겹치지 않습니다.

</details>

**연습 2.** 컴파일러는 아래 함수에 `'a` 를 두 인자 모두에 붙이라고 권합니다. 결과는 `line` 에서만 빌리므로 더 정확한 표기가 있습니다. 고친 뒤 `sep` 이 먼저 사라지는 호출이 컴파일되는지 확인하세요.

```rust
fn first_field(line: &str, sep: &str) -> &str {
    line.split(sep).next().unwrap_or("")
}
```

<details>
<summary>풀이 보기</summary>

```rust
fn first_field<'a>(line: &'a str, sep: &str) -> &'a str {
    line.split(sep).next().unwrap_or("")
}

    let line = String::from("이름,값");
    let field;
    {
        let sep = String::from(",");
        field = first_field(&line, &sep);
    }
    println!("{field}");
```

`이름` 이 찍힙니다. 컴파일러가 권한 대로 `sep` 에도 `'a` 를 붙이면 같은 호출이 `` `sep` does not live long enough `` 로 막힙니다. 표기가 실제 관계보다 넓으면 호출하는 쪽이 불필요하게 묶입니다.

</details>

**연습 3.** 셋 중 컴파일되는 것을 고르세요.

```rust
// (가)
let first = &v[0];
println!("{first}");
v.push(4);

// (나)
let first = &v[0];
v.push(4);
println!("{first}");

// (다)
lens.push(lens.len());
```

<details>
<summary>풀이 보기</summary>

(가)와 (다)가 컴파일됩니다. (가)는 NLL 덕분에 빌림이 `println!` 에서 끝납니다. (나)는 E0502 입니다. (다)는 `&mut` 와 `&` 가 겹쳐 보이지만 컴파일되고 `[10, 20, 2]` 를 찍습니다. 이유는 아래 "한 단계 더"에 있습니다.

</details>

## 한 단계 더 — `v.push(v.len())` 는 왜 될까

`lens.push(lens.len())` 은 `Vec::push(&mut lens, Vec::len(&lens))` 와 같습니다. 고치는 빌림을 먼저 만들고, 인자를 계산하며 읽는 빌림을 또 만듭니다. 규칙대로라면 막혀야 합니다. 그런데 컴파일됩니다.

rustc 개발 안내서(rustc dev guide)의 "Two-phase borrows" 절이 이유를 설명합니다. 메서드 호출의 자동 `&mut` 빌림은 두 단계로 나뉩니다. 먼저 예약만 해 두고, 인자를 다 계산한 뒤 실제 호출 때 활성화합니다. 예약 단계에서는 읽는 빌림을 허락합니다. 그래서 같은 일을 명시적으로 쓰면 막힙니다.

```rust
    let r = &mut lens;
    r.push(lens.len());
```

```console
error[E0502]: cannot borrow `lens` as immutable because it is also borrowed as mutable
 --> src/main.rs:4:12
  |
3 |     let r = &mut lens;
  |             --------- mutable borrow occurs here
4 |     r.push(lens.len());
  |       ---- ^^^^ immutable borrow occurs here
  |       |
  |       mutable borrow later used by call
```

해석하면 빌림 검사기는 규칙의 정신(겹친 동안 실제로 고치지 않는다)을 지키는 선에서 흔한 모양을 허락합니다. 그 범위는 문서에 정해진 경우로 좁습니다. 규칙을 외우기보다 "고치는 순간에 다른 읽기가 살아 있는가"로 판단하면 대부분 맞습니다.

## 정리

값을 옮기지 않고 보여 주려면 `&` 로 빌려 줍니다. 컴파일러는 읽기와 고치기가 겹치지 않는지, 빌림이 주인보다 오래 살지 않는지를 봅니다. 빌림 오류를 만나면 오류 코드 지도에서 먼저 볼 것을 찾고, 마지막 사용을 앞당길 수 있는지부터 봅니다.

4편에서는 타입으로 설계합니다. 구조체와 `enum`, `Option` 과 `Result`, `?` 와 에러 타입, 트레이트로 문서 형식과 실패를 타입에 담습니다.
