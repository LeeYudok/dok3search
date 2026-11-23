# Rust 강의 24편 — 면접 총정리: 질문 지도와 복습 순서

> 면접을 앞두고 이 연재를 어떻게 복습하면 좋을까?

답부터 적습니다. 질문, 30초 답, 근거 실험의 순서로 복습합니다. 답을 외우기보다 그 답을 받치는 오류 메시지와 숫자를 직접 다시 보는 쪽이 오래 남습니다. 이 편은 그 지도입니다. 23편에서 나온 질문을 주제별로 묶고, 질문마다 어느 편의 어떤 실험으로 돌아가면 되는지 적었습니다. 이 연재만으로 입사가 보장되지는 않습니다. 면접은 언어 말고도 자료 구조, 알고리즘, 협업 경험을 봅니다. 이 연재가 돕는 범위는 "Rust 를 근거를 들어 설명하는 힘"입니다.

> [그림] 입문(1~6편), 실무(7~12편), 심화(13~22편), 면접·설계(23~24편) 네 노선의 노선도이고 24편 역에 지금 여기 표시가 있다. 검수 로봇이 노선도를 보고, 아래에 질문, 30초 답, 근거 실험 순서로 복습한다고 적혀 있다

노선은 [로드맵](https://blog.doksam.com/blog/series/rust-course/)의 네 단계입니다. 막힌 질문이 있으면 그 역으로 돌아가 예제를 다시 돌립니다.

Rust 강의 24편이고 연재의 마지막 편입니다. 이 편의 새 코드는 라이브 코딩 두 문제뿐이고, 2026-11-18 에 rustc 1.100.0 과 Miri 로 돌려 확인했습니다.

## 소유권과 빌림

| 질문 | 30초 답의 뼈대 | 돌아갈 곳 |
|---|---|---|
| 이동과 복사는 무엇이 다른가 | 둘 다 비트 복사, 이동은 원래 이름을 무효로. `Copy` 는 필드가 모두 `Copy` 일 때만 | [2편](https://blog.doksam.com/blog/rust-course-02-ownership/): 100MB 이동 42ns, `clone` 6.3ms, E0382·E0204 |
| `String` 과 `&str` | 소유한 힙 버퍼 24B 대 빌린 뷰 16B, 인자는 `&str` | 2편: 한글 바이트 경계 패닉 |
| 빌림 규칙과 이유 | `&mut` 하나 또는 `&` 여럿, 빌림은 주인보다 짧게 | [3편](https://blog.doksam.com/blog/rust-course-03-borrowing/): `push` 가 용량 4→8 로 버퍼를 옮김, E0502 |
| NLL 과 그 한계 | 빌림은 마지막 사용까지. 조건부로 빌림을 돌려주는 모양은 아직 못 푼다 | 3편, [13편](https://blog.doksam.com/blog/rust-course-13-borrowck-lifetimes/): problem case #3 의 E0502, `entry` 로 우회 |
| 수명 표기의 뜻 | 수명을 늘리지 않고 입출력 참조의 관계를 적는다. 생략 규칙 세 가지 | 3편 연습 2, 13편: 컴파일러 제안이 넓거나 좁은 경우 |
| `'static` 의 두 뜻 | `&'static T` 는 끝까지 사는 참조, `T: 'static` 은 빌린 것이 없는 타입 | 13편: `String` 은 통과, `&local` 은 E0597 |

## 타입, 트레이트, 에러

| 질문 | 30초 답의 뼈대 | 돌아갈 곳 |
|---|---|---|
| `match` 완전성 | 변형을 더하면 처리 안 한 곳이 모두 E0004. 와일드카드는 검사를 끈다 | [4편](https://blog.doksam.com/blog/rust-course-04-types/): 엔진의 54갈래 `match` |
| `?` 가 하는 일 | `Err` 를 `From::from` 으로 바꿔 바로 반환. 변환이 없으면 E0277 | 4편 |
| `thiserror` 와 `anyhow` | 코드가 가르면 `enum`, 사람이 읽으면 `anyhow` 와 `context` | [7편](https://blog.doksam.com/blog/rust-course-07-error-design/): 문자열로 뭉개면 원인이 사라짐 |
| 정적·동적 디스패치 | 제네릭은 타입마다 복사본·인라인, `dyn` 은 vtable | [16편](https://blog.doksam.com/blog/rust-course-16-traits-deep/): 심볼 32 대 1, 392,608B 대 359,648B. [9편](https://blog.doksam.com/blog/rust-course-09-closures/): 28ms 대 67ms |
| 연관 타입과 제네릭 인자 | 결과가 하나면 연관 타입, 여럿이면 제네릭 인자 | 16편: E0119, E0283 |
| dyn 호환성, 고아 규칙 | `Self` 반환·제네릭 메서드는 E0038. 남의 타입에 남의 트레이트는 E0117 | 16편 |

## 컬렉션, 클로저, 매크로

| 질문 | 30초 답의 뼈대 | 돌아갈 곳 |
|---|---|---|
| 제로 코스트 추상화 | 이터레이터 체인이 반복문과 같은 기계어로 펼쳐진다 | [8편](https://blog.doksam.com/blog/rust-course-08-collections-iterators/): 셋 다 15.2ms, 범위 검사 사라짐 |
| `HashMap` 과 `BTreeMap` | 찾기만 빠르면 `HashMap`, 순서가 필요하면 `BTreeMap` | 8편: 키 100만 개 14ms 대 77ms |
| `Fn`·`FnMut`·`FnOnce` | 읽기·고치기·소비, 받는 쪽은 가장 약한 약속을 요구 | 9편: E0594, E0382, E0373 |
| 매크로를 언제 쓰나 | 함수·제네릭으로 안 되는 반복만. 펼친 결과로 리뷰 | [20편](https://blog.doksam.com/blog/rust-course-20-macros/): `-Zunpretty=expanded`, 빈 빌드 1.7초 |

## 메모리, 공유, 동시성, async

| 질문 | 30초 답의 뼈대 | 돌아갈 곳 |
|---|---|---|
| `Box`·`Rc`·`Weak` | 단독 힙, 공동 소유, 순환 끊기. 강한 순환은 누수 | [14편](https://blog.doksam.com/blog/rust-course-14-smart-pointers/): drop 이 끝내 안 불림 |
| 내부 가변성 | 검사를 실행 시점으로 옮긴다. `RefCell` 은 패닉, `Mutex` 는 중독 | [15편](https://blog.doksam.com/blog/rust-course-15-interior-mutability/): 다툼 1초 안팎, 각자 세기 5ms |
| `Send`·`Sync` | 넘겨도 되는 것, 함께 봐도 되는 것. 교착은 못 막는다 | [17편](https://blog.doksam.com/blog/rust-course-17-concurrency/): 엇갈린 잠금 1초 Timeout |
| async 와 스레드 | 기다림은 async, 계산은 스레드. 블로킹 호출은 실행기를 멈춘다 | [18편](https://blog.doksam.com/blog/rust-course-18-async/): 614ms 대 `spawn_blocking` 207ms |
| `Pin` 이 필요한 이유 | `.await` 를 건너는 값이 상태 기계의 필드가 되어 옮기면 안 된다 | 18편: 4KB 를 쥔 블록 4,240B |
| `repr(C)` 와 니치 | C 와 주고받을 때만 순서 고정. `Option<&T>` 는 공짜 | [19편](https://blog.doksam.com/blog/rust-course-19-layout-unsafe/): 8B 대 12B |
| `unsafe` 의 범위 | 다섯 가지만 허락, 증명은 사람이. 안전한 API 뒤에 가두고 Miri 로 시험 | 19편: 일반 실행은 조용, Miri 는 넷 다 잡음 |
| FFI 경계의 패닉 | `extern "C"` 에서 abort. `catch_unwind` 로 오류 코드로 | [22편](https://blog.doksam.com/blog/rust-course-22-ffi/): 종료 코드 134 |

## 도구, 성능, 운영

| 질문 | 30초 답의 뼈대 | 돌아갈 곳 |
|---|---|---|
| 디버그와 릴리스 | 최적화, 디버그 정보, 정수 오버플로 검사가 다르다 | [1편](https://blog.doksam.com/blog/rust-course-01-getting-started/): 디버그는 패닉, 릴리스는 감아 돌림 |
| LTO 와 바이너리 크기 | 크기는 줄지만 재빌드가 길어지고, 속도는 재 봐야 안다 | [6편](https://blog.doksam.com/blog/rust-course-06-shipping/): 0.33초 → 3.37초, 이 도구에서는 느려짐 |
| 빌드 시간 줄이기 | `check`, 대상까지 좁힌 테스트, 워크스페이스 분리 | 6편: `--lib` 로 5.8배. [11편](https://blog.doksam.com/blog/rust-course-11-workspace-features/): 27개 → 8개 |
| 테스트 설계 | 자리가 범위를 정한다. 경계는 프로퍼티 전략에 직접 넣는다 | [10편](https://blog.doksam.com/blog/rust-course-10-testing/): 무작위만으로는 4번 다 놓침 |
| clippy 와 리뷰 | 줄 단위 관용구는 clippy, 약속과 구조는 사람 | [12편](https://blog.doksam.com/blog/rust-course-12-clippy-review/): 경고 9개 중 6개 자동 수정 |
| 벤치마크 함정 | 입력을 `black_box` 로 가리지 않으면 일이 지워진다 | [21편](https://blog.doksam.com/blog/rust-course-21-performance/): `black_box` 없이 0.22ns(일을 안 함), 있으면 566ns |
| 시스템 과제 | 요구 질문, 구조 선택의 교환, 시간 주입 테스트 | [23편](https://blog.doksam.com/blog/rust-course-23-system-design/): LRU 를 O(log n) 으로 안전하게 |

## 라이브 코딩 두 문제

화면 공유로 짧은 함수를 쓰게 하는 면접이 많습니다. 이 연재의 주제와 이어진 두 문제를 풀었습니다.

**문제 1.** 문자열을 `max` 바이트 이하로 자르되 글자 중간에서 자르지 마세요.

```rust
pub fn truncate_utf8(s: &str, max: usize) -> &str {
    if s.len() <= max {
        return s;
    }
    let mut end = max;
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}
```

`truncate_utf8("한글 문서", 4)` 는 `"한"`, `6` 은 `"한글"`, `truncate_utf8("한", 2)` 는 빈 문자열이었습니다. 2편의 패닉을 피하는 문제입니다. 말로 설명할 점은 셋입니다. 결과가 입력을 빌리므로 복사가 없다는 것, 생략 규칙 2가 수명을 채운다는 것, `end` 가 0 이면 늘 경계라 반복이 끝난다는 것입니다.

**문제 2.** `()[]{}` 괄호의 짝이 맞는지 보세요.

```rust
pub fn balanced(s: &str) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => {
                let open = match c {
                    ')' => '(',
                    ']' => '[',
                    _ => '{',
                };
                if stack.pop() != Some(open) {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}
```

`"fn f() { v[0] }"` 은 참, `"(]"` 과 `"(("` 는 거짓이었습니다. 빈 스택에서 `pop()` 이 `None` 을 주므로 닫는 괄호가 먼저 와도 패닉하지 않습니다. 4편의 `Option` 이 경계 처리를 대신합니다. 두 문제의 테스트는 Miri 에서도 통과했습니다.

## 코드 리뷰 과제는 12편의 파일로 연습합니다

"이 코드를 리뷰해 보세요"라는 과제도 흔합니다. [12편](https://blog.doksam.com/blog/rust-course-12-clippy-review/)의 `before.rs` 를 열고 clippy 를 돌리기 전에 직접 지적을 적어 봅니다. 그다음 clippy 결과와 맞춰 봅니다. 사람이 먼저 봐야 하는 것은 clippy 가 못 보는 쪽입니다. 라이브러리 공개 함수의 `unwrap`, 원인을 버리는 에러 변환, 필요 이상으로 넓은 `pub` 과 `Fn` 요구입니다. 지적마다 "왜"와 "대안"을 한 문장씩 붙이는 연습을 합니다.

## 이 연재가 다루지 않은 것

정직하게 적습니다. 임베디드(`no_std`), WebAssembly, GUI, 게임 엔진은 다루지 않았습니다. 웹 프레임워크(axum 등)는 18편의 tokio 기초까지만 다뤘습니다. 자료 구조와 알고리즘 문제 풀이, 여러 서버에 걸친 분산 설계도 범위 밖입니다. 면접 준비에는 이 부분을 따로 채워야 합니다.

## 연습

**연습 1.** 지도에서 30초 안에 답하지 못한 질문 셋을 고르고, 해당 편의 예제를 다시 돌려 오류 메시지나 숫자를 한 줄씩 적으세요.

<details>
<summary>풀이 보기</summary>

정답은 사람마다 다릅니다. 예제는 각 편의 `examples/` 크레이트에 있고 `cargo test` 와 `cargo run` 으로 다시 돕니다. 다시 돌렸을 때 숫자가 글과 다르면 정상입니다. 장비와 툴체인이 다르기 때문입니다(21편). 방향(어느 쪽이 빠른가, 어떤 오류인가)이 같은지를 봅니다.

</details>

**연습 2.** `truncate_utf8` 을 바이트 대신 글자 수로 자르는 판으로 바꾸고 차이를 설명하세요.

<details>
<summary>풀이 보기</summary>

2편 연습 2의 `prefix_chars` 가 그 판입니다. `char_indices().nth(n)` 으로 n 번째 글자의 바이트 위치를 찾습니다. 바이트 판은 저장·전송 한도(예: 255바이트 필드)에 맞추고, 글자 판은 화면 표시(예: 20자 요약)에 맞춥니다. 요구를 먼저 묻는 이유가 여기에도 있습니다(23편).

</details>

## 한 단계 더 — 지식은 툴체인과 함께 낡습니다

이 연재의 숫자와 오류 문구는 rustc 1.100.0, 에디션 2024, 2026-11-18 기준입니다. 연재를 쓰는 동안에도 판이 바뀌며 달라진 것을 여럿 만났습니다. clippy 가 새 API(`as_chunks`, `is_multiple_of`)를 권했고(5·8편), 오류 문구가 "object safe"에서 "dyn compatible"로 바뀌어 있었습니다(16편). 같은 `if let` 이 에디션에 따라 패닉하기도 했습니다(15편). 면접에서 "최신 Rust 에서는 어떻게 되나"를 물으면, 외운 답보다 "그 판에서 돌려 보면 이렇게 나온다"가 강합니다. 그래서 복습은 늘 예제를 지금의 툴체인으로 다시 돌리는 것으로 끝냅니다.

## 정리

복습은 질문, 30초 답, 근거 실험의 순서입니다. 답이 막히면 지도의 편으로 돌아가 예제를 다시 돌립니다. 이 연재가 돕는 것은 Rust 를 근거로 설명하는 힘이고, 그 밖의 준비는 따로 채웁니다. 연재를 끝까지 따라와 주셔서 고맙습니다.
