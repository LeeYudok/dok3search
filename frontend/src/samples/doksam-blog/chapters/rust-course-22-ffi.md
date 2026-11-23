# Rust 강의 22편 — FFI 와 바인딩: C ABI 경계를 넘는 법

> Rust 로 만든 코어를 다른 언어에서 부르려면 경계에서 무엇을 지켜야 할까?

답부터 적습니다. 경계에서는 C 가 아는 것만 주고받습니다. 포인터와 길이, 정수, NUL 로 끝나는 문자열입니다. 메모리는 만든 쪽이 해제합니다. Rust 가 만든 문자열은 Rust 의 해제 함수로 돌려받습니다. 패닉은 경계를 넘길 수 없습니다. rustc 1.100.0 에서 `extern "C"` 함수가 패닉하자 `panic in a function that cannot unwind` 를 찍고 프로세스가 종료 코드 134 로 끝났습니다. 그래서 경계 함수는 `catch_unwind` 로 패닉을 잡아 오류 코드로 바꿉니다. pyo3·napi 는 이 일을 대신 만들어 주는 도구입니다.

> [그림] 가운데 C ABI 벽을 사이에 두고 C 의 main.c 가 ptr 과 len 을 넘기면 Rust 의 rc_count 가 2, -1, -2 를 돌려준다. rc_describe 의 문자열은 C 가 쓰고 rc_free 로 돌려준다. 아래에서 panic 이 벽을 넘으려다 abort, 종료 코드 134 가 되고 catch_unwind 로 잡으면 -99 다. 검수 로봇이 벽을 본다

벽을 넘는 화살표는 모두 이 글의 C 프로그램이 실제로 주고받은 값입니다.

Rust 강의 22편입니다. [6편](https://blog.doksam.com/blog/rust-course-06-shipping/)에서 `cdylib` 을 Python `ctypes` 로 불렀습니다. 이번에는 C 프로그램에서 부르고, 반대로 Rust 에서 C 를 부르고, 문자열과 패닉을 다룹니다. Rust 는 rustc 1.100.0(에디션 2024), C 는 맥의 기본 `cc` 로 2026-11-18 에 빌드했습니다.

## C 에서 Rust 를 부르려면 C 의 타입으로 맞춥니다

```rust
/// 레코드 수를 센다. 깨졌으면 -1, 포인터가 null 이면 -2.
///
/// # Safety
/// `ptr` 은 `len` 바이트를 읽을 수 있어야 한다.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rc_count(ptr: *const u8, len: usize) -> i64 {
    if ptr.is_null() {
        return -2;
    }
    // SAFETY: 호출자가 ptr..ptr+len 이 유효하다고 약속했다.
    let data = unsafe { std::slice::from_raw_parts(ptr, len) };
    // ... 5편과 같이 머리를 따라 세고, 모자라면 -1
}
```

`extern "C"` 는 C 의 호출 규약을 따르라는 표시, `#[unsafe(no_mangle)]` 은 함수 이름을 `rc_count` 그대로 내보내라는 표시입니다. `&[u8]` 이나 `Result` 는 C 가 모르니 `*const u8` 과 길이, 정수 오류 코드로 바꿉니다. 경계 안쪽에서 슬라이스를 만든 뒤에는 평범한 안전한 Rust 입니다(19편의 안전한 추상화).

C 쪽에는 머리 파일을 손으로 썼습니다.

```c
int64_t rc_count(const uint8_t *ptr, size_t len);
char *rc_describe(int64_t records);
void rc_free(char *s);
```

`cbindgen` 은 Rust 소스에서 이 파일을 만들어 주는 도구입니다. 이 맥에 깔려 있지 않아 쓰지 않았습니다. `cc c/main.c -Ltarget/release -lffidemo` 로 빌드해 돌린 결과입니다.

```console
C: rc_count = 2, rc_describe = 레코드 2개
C: 잘린 입력 = -1, null = -2
```

## 문자열은 만든 쪽이 해제합니다

Rust 의 `String` 은 C 가 모르는 타입이고 NUL 로 끝나지도 않습니다. 경계에서는 `CString` 으로 바꿔 포인터만 넘깁니다.

```rust
#[unsafe(no_mangle)]
pub extern "C" fn rc_describe(records: i64) -> *mut c_char {
    let s = CString::new(format!("레코드 {records}개")).expect("NUL 없음");
    s.into_raw()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rc_free(s: *mut c_char) {
    if !s.is_null() {
        // SAFETY: CString::into_raw 로 만든 포인터를 되돌린다.
        drop(unsafe { CString::from_raw(s) });
    }
}
```

`into_raw` 는 소유권을 C 에 넘기고 Rust 의 `drop` 을 멈춥니다. 이 메모리는 Rust 의 할당기가 만들었으니 C 의 `free()` 로 해제하면 안 됩니다. 그래서 짝이 되는 `rc_free` 를 함께 내보내고, C 는 쓰고 난 뒤 그 함수로 돌려줍니다. 2편의 "주인은 하나" 규칙이 언어 경계를 넘어서도 이어지는 셈입니다. 컴파일러가 C 쪽을 검사해 주지 않으니 이 약속은 머리 파일과 문서로 지킵니다.

## Rust 에서 C 를 부를 때는 unsafe extern 으로 선언합니다

반대 방향입니다. C 표준 라이브러리의 `strlen` 을 불렀습니다.

```rust
// Rust 에서 C 표준 라이브러리 함수를 부른다(에디션 2024 는 unsafe extern).
unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
}

pub fn c_strlen(s: &CStr) -> usize {
    // SAFETY: CStr 은 NUL 로 끝나는 유효한 문자열이다.
    unsafe { strlen(s.as_ptr()) }
}
```

`strlen("문서")` 는 6을 돌려줬습니다. 바이트 수라서 한글 두 글자가 6입니다(2편). 에디션 2024 에서 `unsafe` 를 빼고 `extern "C" {` 로 쓰면 막힙니다.

```console
error: extern blocks must be unsafe
 --> src/main.rs:3:1
  |
3 |   extern "C" {
  |   ^
  |   |
  |  _help: needs `unsafe` before the extern keyword: `unsafe`
```

C 함수 선언이 맞는지 컴파일러가 확인할 수 없으니, 선언하는 자리부터 `unsafe` 로 표시하게 바뀐 것입니다. `strlen` 을 부를 때도 다시 `unsafe` 블록이 필요합니다. 바깥에는 `&CStr` 을 받는 안전한 `c_strlen` 만 내보냅니다.

## 패닉이 extern "C" 경계에 닿으면 프로세스가 끝납니다

C 에서 패닉하는 Rust 함수를 불렀습니다.

```rust
#[unsafe(no_mangle)]
pub extern "C" fn rc_panics(flag: i32) -> i32 {
    if flag != 0 {
        panic!("망가진 입력");
    }
    0
}
```

```console
C: rc_panics(1) 를 부른다
thread '<unnamed>' (30908853) panicked at src/lib.rs:56:9:
망가진 입력
...
panic in a function that cannot unwind
...
thread caused non-unwinding panic. aborting.
$ echo $?
134
```

C 는 Rust 의 패닉 풀기(unwinding)를 모릅니다. 그래서 rustc 는 `extern "C"` 경계에 닿은 패닉을 풀지 않고 프로세스를 중단(abort)합니다. 종료 코드 134 는 SIGABRT 입니다. C 쪽 `C: 돌아왔다` 는 끝내 찍히지 않았습니다. 이 동작은 Rust 1.81 부터 정해진 것으로 릴리스 문서에 적혀 있습니다. 중단이 정의되지 않은 동작보다는 낫지만, 라이브러리 하나의 패닉이 그 프로세스 전체를 끝냅니다. Python 이나 Node 프로세스 안이라면 서비스 전체가 죽습니다.

그래서 경계 함수는 패닉을 안에서 잡습니다.

```rust
#[unsafe(no_mangle)]
pub extern "C" fn rc_panics_caught(flag: i32) -> i32 {
    catch_unwind(|| {
        if flag != 0 {
            panic!("망가진 입력");
        }
        0
    })
    .unwrap_or(-99)
}
```

C 에서 부르자 `-99` 가 돌아왔고 프로그램은 이어서 돌았습니다. 기본 패닉 훅 때문에 메시지는 표준 에러에 한 번 찍혔습니다. 6편에서 본 `panic = "abort"` 프로필로 빌드하면 `catch_unwind` 도 잡지 못하니, 바인딩 라이브러리는 그 설정을 쓰지 않습니다.

## pyo3 와 napi 는 이 경계 코드를 만들어 줍니다

| 할 일 | 손으로 쓴 C ABI(이 편) | pyo3 · napi |
|---|---|---|
| 타입 변환 | 포인터·길이·정수 코드 | `&str`, `Vec`, 딕셔너리를 바로 받는다 |
| 문자열 소유권 | `into_raw` + 짝 해제 함수 | 그 언어의 문자열 객체로 복사해 준다 |
| 에러 | 정수 코드 | `Result` 를 예외로(`PyErr`, `napi::Error`) |
| 패닉 | `catch_unwind` 를 직접 | pyo3 는 `PanicException` 으로 바꾼다(문서). napi 는 확인하지 못했다 |

pyo3 와 napi 는 이번에도 빌드하지 않았습니다. 위 표의 오른쪽 열은 각 도구의 문서와 문서 변환 엔진의 바인딩 코드 구조(6편)에 근거합니다. Python 에는 전역 인터프리터 잠금(GIL)이 있어, pyo3 함수가 오래 걸리는 Rust 코드를 돌리는 동안 다른 Python 스레드가 멈출 수 있습니다. pyo3 문서는 이때 잠금을 잠시 풀고 Rust 코드를 돌리는 방법을 안내합니다. 이 부분도 문서 근거이고 직접 재지 않았습니다.

## 면접에서 이렇게 묻는다

**Q1. Rust 함수를 C 에서 부르려면 무엇이 필요한가요?**
`extern "C"` 호출 규약, `#[unsafe(no_mangle)]` 으로 이름 고정, C 가 아는 타입(포인터·길이·정수·`repr(C)` 구조체), `crate-type = ["cdylib"]` 또는 `staticlib` 입니다. 경계 안에서 안전한 타입으로 바꿔 씁니다.

**Q2. 경계를 넘긴 문자열은 누가 해제하나요?**
만든 쪽입니다. Rust 가 `CString::into_raw` 로 넘겼으면 Rust 가 내보낸 해제 함수(`CString::from_raw`)로 돌려받습니다. C 의 `free()` 를 쓰면 할당기가 달라 정의되지 않은 동작이 됩니다.

**Q3. 패닉이 FFI 경계를 넘으면?**
`extern "C"` 함수에서는 풀지 않고 프로세스를 중단합니다. 실험에서 `panic in a function that cannot unwind` 와 종료 코드 134 가 나왔습니다. 경계에서 `catch_unwind` 로 잡아 오류 코드로 바꿉니다.

**Q4. 에디션 2024 에서 FFI 선언은 무엇이 바뀌었나요?**
`extern` 블록은 `unsafe extern` 으로, `no_mangle` 은 `#[unsafe(no_mangle)]` 로 적습니다. 빼면 각각 "extern blocks must be unsafe", "unsafe attribute used without unsafe" 오류입니다.

**Q5. pyo3 를 쓸 때 주의할 점은?**
`Result` 를 Python 예외로 바꾸고, 긴 계산 동안 GIL 을 놓아 다른 Python 스레드를 막지 않게 하고, 경계의 패닉을 예외로 바꾸는지 확인합니다. 데이터를 넘길 때 복사가 일어나는지도 봅니다.

## 연습

**연습 1.** C 에서 `rc_describe` 의 결과를 `free()` 로 해제하면 어떤 문제가 생길 수 있나요?

<details>
<summary>풀이 보기</summary>

Rust 의 할당기와 C 의 할당기가 같다는 보장이 없어 정의되지 않은 동작입니다. 둘이 같은 시스템 할당기를 쓰는 환경에서는 우연히 동작할 수도 있어서 더 위험합니다. 이 연재에서는 일부러 돌려 보지 않았습니다. 규칙은 하나입니다. 해제는 만든 쪽의 함수로 합니다.

</details>

**연습 2.** `rc_count` 의 `-1`, `-2` 같은 오류 코드 대신 C 쪽에 더 많은 정보를 주려면 어떻게 할까요?

<details>
<summary>풀이 보기</summary>

오류 위치를 담을 출력 인자를 받습니다. 예를 들어 `int64_t rc_count2(const uint8_t *ptr, size_t len, size_t *err_offset)` 로 바꾸고, 깨졌을 때 `*err_offset` 에 5편의 `Truncated { offset }` 값을 씁니다. 메시지가 필요하면 `rc_describe` 처럼 문자열을 돌려주고 짝 해제 함수를 둡니다. 어느 쪽이든 경계에는 C 타입만 둡니다. 이 풀이는 설계 예시이고 구현해 돌리지는 않았습니다.

</details>

## 한 단계 더 — 경계 함수는 얇게, 테스트는 안쪽에

이 편의 경계 함수는 모두 "C 타입을 Rust 타입으로 바꾸고, 안쪽 함수를 부르고, 결과를 C 타입으로 바꾸는" 세 줄짜리입니다. 로직을 경계 함수에 두지 않으면 테스트를 안전한 Rust 로 할 수 있고, `unsafe` 줄은 변환 한두 곳에만 남습니다. 문서 변환 엔진의 Python·Node 바인딩도 함수마다 엔진의 공개 함수를 부르고 에러를 그 언어의 예외로 바꾸는 몇 줄뿐입니다(6편). 해석하면 바인딩을 늘릴수록 코어는 그대로 두고 얇은 겉만 늘어나는 구조가 됩니다. 반대로 경계 함수가 두꺼워지면 언어마다 같은 로직을 다시 시험해야 합니다.

## 정리

경계에서는 C 의 말로만 주고받고, 메모리는 만든 쪽이 해제하고, 패닉은 경계 안에서 잡습니다. 이 셋을 지키는 얇은 겉을 손으로 쓰거나 pyo3·napi 에 맡깁니다.

23편에서는 이 연재의 도구들을 모아 시스템 과제를 풉니다. 레이트 리미터, LRU 캐시, 로그 집계기를 설계하고 타입과 동시성 선택을 설명합니다.
