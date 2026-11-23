# Rust 강의 19편 — 메모리 배치와 unsafe

> 값은 메모리에 어떻게 놓이고, `unsafe` 는 그 위에서 무엇을 허락할까?

답부터 적습니다. Rust 의 기본 배치(`repr(Rust)`)는 컴파일러가 필드 순서를 바꿔 빈칸을 줄입니다. C 와 주고받을 구조체는 `repr(C)` 로 선언 순서를 고정합니다. 같은 필드가 기본 배치에서 8바이트, `repr(C)` 에서 12바이트였습니다. `unsafe` 는 컴파일러가 확인하지 못하는 일 몇 가지를 허락하고, 그 일이 안전하다는 증명을 사람에게 넘깁니다. 증명이 틀리면 정의되지 않은 동작(UB)이 됩니다. 실험에서 UB 네 가지는 일반 실행에서 아무 오류 없이 값을 찍었고, Miri 는 넷 모두를 잡았습니다.

> [그림] 위쪽에 기본 repr 8B 와 repr(C) 12B 의 바이트 배치가 있다. 오른쪽 Miri 화면에 정렬 위반, 범위 밖 읽기, 빌림 규칙 위반, 해제 뒤 사용이 빨갛게 떠 있고 검수 로봇이 그 화면을 본다. 아래 점선 unsafe 울타리 안에 SAFETY 주석과 read_unaligned 가 있고, 밖으로 안전한 함수 read_u32_le_at 이 Option 을 돌려준다

그림의 두 배치와 Miri 네 줄은 이 글의 실행 결과입니다.

Rust 강의 19편입니다. [6편](https://blog.doksam.com/blog/rust-course-06-shipping/)에서 C ABI 경계의 한 줄 `unsafe` 를 봤고, [17편](https://blog.doksam.com/blog/rust-course-17-concurrency/)은 `unsafe impl Send` 를 예고했습니다. 이번 편은 배치와 `unsafe` 를 직접 잽니다. stable 코드는 rustc 1.100.0, Miri 는 nightly 1.102.0(miri 0.1.0, 2026-11-16 판)으로 2026-11-18 에 돌렸습니다.

## 기본 배치는 필드 순서를 바꿉니다

```rust
struct Header {
    kind: u8,
    size: u32,
    level: u16,
}

#[repr(C)]
struct HeaderC {
    kind: u8,
    size: u32,
    level: u16,
}
```

`size_of`, `align_of`, `offset_of!` 로 잰 결과입니다.

| 배치 | 크기 | 정렬 | `kind` | `size` | `level` |
|---|---:|---:|---:|---:|---:|
| 기본 `repr(Rust)` | 8 | 4 | 6 | 0 | 4 |
| `repr(C)` | 12 | 4 | 0 | 4 | 8 |

`u32` 는 4의 배수 주소에 놓여야 합니다(정렬 4). `repr(C)` 는 선언 순서대로 놓아야 해서 `kind` 뒤에 빈칸 3바이트, 끝에 2바이트를 채웁니다. 기본 배치는 큰 필드부터 놓아 빈칸을 1바이트로 줄였습니다. The Rust Reference 는 기본 배치의 필드 순서를 보장하지 않는다고 적습니다. 그래서 바이트를 직접 해석하거나 C 와 주고받는 구조체는 `repr(C)` 로 고정합니다(22편).

## Option 은 빈 비트 패턴이 있으면 공짜입니다

| 타입 | 크기 | `Option<타입>` |
|---|---:|---:|
| `&u8` | 8 | 8 |
| `Box<u8>` | 8 | 8 |
| `NonZeroU32` | 4 | 4 |
| `u32` | 4 | 8 |
| `bool` | 1 | 1 |
| `String` | 24 | 24 |

참조와 `Box` 는 null 이 될 수 없으니 null 비트 패턴을 `None` 으로 씁니다. `bool` 은 0 과 1 만 쓰니 2 를 `None` 으로 씁니다. 이렇게 "그 타입이 절대 쓰지 않는 값"에 `None` 을 넣는 것을 니치(niche) 최적화라고 합니다. `u32` 는 모든 비트 패턴이 유효한 값이라 빈칸이 없어 꼬리표 자리가 따로 듭니다. 그래서 `0` 이 올 수 없는 크기나 번호는 `NonZeroU32` 로 두면 `Option` 이 공짜가 됩니다.

## unsafe 는 다섯 가지를 허락할 뿐입니다

The Book 20.1절은 `unsafe` 가 허락하는 일을 다섯으로 적습니다.

1. 날 포인터(`*const T`, `*mut T`)를 역참조한다
2. `unsafe` 함수나 메서드를 부른다
3. 바꿀 수 있는 `static` 변수를 읽고 쓴다
4. `unsafe` 트레이트를 구현한다(17편의 `Send`·`Sync`)
5. `union` 의 필드를 읽는다

빌림 검사기가 꺼지는 것이 아닙니다. `unsafe` 블록 안에서도 참조의 규칙은 그대로이고, 사람이 그 규칙을 지켰다고 약속할 뿐입니다. 에디션 2024 에서는 6편에서 본 `#[unsafe(no_mangle)]` 처럼 위험한 속성에도 `unsafe` 를 적어야 합니다.

## 정의되지 않은 동작은 조용히 지나갑니다

일부러 틀린 `unsafe` 네 개를 만들었습니다.

```rust
// 정렬되지 않은 주소에서 u32 를 읽는다
let d = [0xFFu8, 0x78, 0x56, 0x34, 0x12];
let v = unsafe { *(d.as_ptr().add(1) as *const u32) };

// 범위를 넘어 읽는다
let d = [1u8, 2, 3];
let v = unsafe { *d.as_ptr().add(3) };

// 같은 값에 &mut 두 개
let p = &mut n as *mut u32;
let a = unsafe { &mut *p };
let b = unsafe { &mut *p };
*a += 1; *b += 1; *a += 1;

// 해제된 메모리를 읽는다
let p = { let v = vec![7u8; 4]; v.as_ptr() };
let x = unsafe { *p };
```

릴리스 빌드로 그냥 돌린 결과는 `0x12345678`, `0`, `3`, `0` 이었습니다. 하나도 죽지 않았습니다. 마지막은 7 을 넣은 메모리를 읽었는데 0 이 나왔습니다. UB 는 "운이 좋으면 맞는 값, 나쁘면 아무 값"이고, 최적화 수준이나 컴파일러 판이 바뀌면 결과도 바뀔 수 있습니다. 테스트로는 잡히지 않는다는 뜻입니다.

## Miri 는 네 가지를 모두 잡았습니다

Miri 는 Rust 의 중간 표현을 한 줄씩 해석하며 메모리 규칙을 검사하는 도구입니다. nightly 툴체인에 `rustup component add miri` 로 깔고 `cargo +nightly miri run` 으로 돌립니다.

```console
error: Undefined Behavior: accessing memory based on pointer with alignment 1, but alignment 4 is required
 --> src/bin/ub_unaligned.rs:4:22
  |
4 |     let v = unsafe { *(d.as_ptr().add(1) as *const u32) };
  |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Undefined Behavior occurred here
```

```console
error: Undefined Behavior: memory access failed: attempting to access 1 byte, but got alloc205+0x3 which is at or beyond the end of the allocation of size 3 bytes
```

```console
error: Undefined Behavior: attempting a read access using <553> at alloc205[0x0], but that tag does not exist in the borrow stack for this location
 --> src/bin/ub_alias.rs:7:5
  |
7 |     *a += 1;
  |     ^^^^^^^ this error occurs as part of an access at alloc205[0x0..0x4]
  |
  = help: this indicates a potential bug in the program: it performed an invalid operation, but the Stacked Borrows rules it violated are still experimental
```

```console
error: Undefined Behavior: memory access failed: alloc305 has been freed, so this pointer is dangling
```

셋째 오류는 `b` 를 만든 순간 `a` 가 무효가 됐는데 `a` 를 다시 썼다는 뜻입니다. 3편의 "`&mut` 은 하나" 규칙이 `unsafe` 안에서도 살아 있다는 증거입니다. Miri 도 이 규칙(Stacked Borrows)은 아직 실험 단계라고 밝힙니다. Miri 는 코드를 한 단계씩 해석하므로 일반 실행보다 느리고, 실행된 경로만 검사합니다. 그래서 `unsafe` 가 있는 모듈의 테스트를 Miri 로 돌리는 방식으로 씁니다.

## unsafe 는 안전한 API 뒤에 가둡니다

정렬되지 않은 바이트에서 `u32` 를 읽는 일을 바르게 쓰면 이렇습니다.

```rust
pub fn read_u32_le_at(data: &[u8], pos: usize) -> Option<u32> {
    let end = pos.checked_add(4)?;
    if end > data.len() {
        return None;
    }
    // SAFETY: 위에서 pos..pos+4 가 data 안에 있음을 확인했다.
    // 바이트 버퍼는 4바이트 정렬을 보장하지 않으므로 read_unaligned 로 읽는다.
    let raw = unsafe { std::ptr::read_unaligned(data.as_ptr().add(pos) as *const u32) };
    Some(u32::from_le(raw))
}
```

범위 확인은 안전한 코드에서 하고, `unsafe` 는 그 확인에 기대는 한 줄만 둡니다. `SAFETY` 주석에 그 한 줄이 왜 안전한지 적습니다. 바깥에서는 `Option<u32>` 를 돌려주는 평범한 함수입니다. 이 함수의 테스트를 `cargo +nightly miri test` 로 돌리자 UB 없이 통과했습니다.

그런데 이 함수는 사실 `unsafe` 가 필요 없습니다. 5편처럼 `data.get(pos..end)?` 와 `u32::from_le_bytes` 로 같은 일을 합니다. 문서 변환 엔진의 소스 5만 줄에 `unsafe` 가 한 줄도 없는 것도 이 때문입니다. 바이트 파서에 필요한 일은 안전한 API 로 다 됩니다. `unsafe` 는 C 함수를 부르거나(22편) 안전한 API 로는 표현할 수 없는 자료 구조를 만들 때 남겨 둡니다.

## 면접에서 이렇게 묻는다

**Q1. `repr(C)` 는 언제 쓰나요?**
C 와 구조체를 주고받거나 바이트를 그 구조체로 직접 해석할 때 씁니다. 필드 순서와 채움이 C 규칙을 따릅니다. 기본 배치는 순서를 보장하지 않습니다. 실측에서 같은 필드가 8바이트 대 12바이트였습니다.

**Q2. `Option<&T>` 는 왜 `&T` 와 크기가 같나요?**
참조는 null 이 될 수 없어 null 을 `None` 으로 쓰는 니치 최적화 때문입니다. `Option<Box<T>>`, `Option<NonZeroU32>` 도 같습니다. `Option<u32>` 는 빈 비트 패턴이 없어 8바이트입니다.

**Q3. `unsafe` 는 무엇을 허락하나요?**
날 포인터 역참조, `unsafe` 함수 호출, 바꿀 수 있는 `static` 접근, `unsafe` 트레이트 구현, `union` 필드 읽기입니다. 빌림 검사를 끄지는 않습니다. 사람이 안전 조건을 증명할 책임을 집니다.

**Q4. 정의되지 않은 동작은 어떻게 찾나요?**
Miri 로 `unsafe` 가 있는 코드의 테스트를 돌립니다. 실험에서 일반 실행은 UB 네 개를 조용히 넘겼고 Miri 는 모두 잡았습니다. 실행된 경로만 검사하니 테스트가 경계 사례를 지나가야 합니다.

**Q5. 안전한 추상화란 무엇인가요?**
`unsafe` 를 함수 안에 가두고, 바깥 API 는 어떤 입력에도 UB 가 나지 않게 만드는 것입니다. 안전 조건을 안전한 코드로 먼저 확인하고 `SAFETY` 주석으로 이유를 남깁니다. `Vec`, `split_at_mut` 이 이렇게 만들어져 있습니다.

## 연습

**연습 1.** 길이가 0 이 될 수 없는 페이지 번호를 담는 구조체 필드를 `Option<u32>` 대신 무엇으로 두면 `Option` 비용이 사라질까요?

<details>
<summary>풀이 보기</summary>

`Option<NonZeroU32>` 입니다. 실측에서 4바이트로 `NonZeroU32` 와 같았고, `Option<u32>` 는 8바이트였습니다. `NonZeroU32::new(n)` 이 `Option` 을 돌려주므로 0 이 들어오는 경우도 안전하게 막힙니다.

</details>

**연습 2.** `read_u32_le_at` 을 `unsafe` 없이 다시 쓰세요.

<details>
<summary>풀이 보기</summary>

```rust
pub fn read_u32_le_at_safe(data: &[u8], pos: usize) -> Option<u32> {
    let bytes = data.get(pos..pos.checked_add(4)?)?;
    Some(u32::from_le_bytes(bytes.try_into().ok()?))
}
```

4편의 `read_u32_le` 와 같습니다. 같은 테스트 값(`0x12345678`, 범위 밖 `None`)을 냅니다. 성능이 문제라면 바꾸기 전에 21편의 방법으로 재 봅니다.

</details>

**연습 3.** `ub_alias` 를 Miri 가 통과하도록 고치세요.

<details>
<summary>풀이 보기</summary>

`&mut` 을 한 번에 하나만 쓰면 됩니다. `let a = unsafe { &mut *p }; *a += 1;` 을 끝낸 뒤 `let b = unsafe { &mut *p }; *b += 1;` 처럼, 새 `&mut` 을 만든 뒤에는 옛 `&mut` 을 다시 쓰지 않습니다. 애초에 날 포인터가 필요 없는 코드라면 `n += 1` 로 씁니다.

</details>

## 한 단계 더 — 에디션 2024 의 unsafe 규칙

에디션 2024 는 `unsafe` 를 더 좁게 적게 바꿨습니다. `unsafe fn` 안에서도 `unsafe` 연산마다 `unsafe {}` 블록을 다시 쓰라는 경고(`unsafe_op_in_unsafe_fn`)가 기본으로 켜졌습니다. 함수 전체가 아니라 실제로 위험한 줄에 표시를 남기라는 뜻입니다. `extern` 블록도 `unsafe extern` 으로 적습니다. 이 두 변경은 Rust Edition Guide 의 2024 장에 근거했고, 이번 편에서 별도 예제로 돌리지는 않았습니다. 6편의 `#[unsafe(no_mangle)]` 오류는 같은 흐름의 변경을 실제로 본 사례입니다.

## 정리

배치는 `size_of` 와 `offset_of!` 로 재고, C 와 주고받을 때만 `repr(C)` 로 고정합니다. `unsafe` 는 다섯 가지만 허락하고 규칙을 지킬 책임을 사람에게 넘깁니다. 그 책임은 작은 함수 안에 가두고, Miri 로 테스트합니다.

20편에서는 매크로를 다룹니다. 이 연재에서 이미 쓴 `vec!`, `#[derive]`, `thiserror` 가 무엇을 만들어 주는지, 직접 `macro_rules!` 와 derive 매크로를 만들어 봅니다.
