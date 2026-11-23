# Rust 강의 10편 — 테스트 설계: 단위·통합·문서·프로퍼티

> 테스트는 어디에 두고, 손으로 고른 예시가 놓치는 버그는 어떻게 찾을까?

답부터 적습니다. 테스트를 두는 자리가 볼 수 있는 범위를 정합니다. 같은 파일의 단위 테스트는 비공개 함수까지, `tests/` 의 통합 테스트는 공개 API 만, `///` 문서 테스트는 사용 예시가 실제로 돌아가는지를 봅니다. 손으로 고른 예시가 놓치는 경계는 프로퍼티 테스트로 찾습니다. "만들고 읽으면 원래대로"처럼 어떤 입력에도 성립해야 하는 성질을 적고 무작위 입력을 던집니다. 다만 무작위만으로는 부족했습니다. 경계 값을 섞어야 버그가 나왔습니다.

> [그림] 왼쪽에 단위·통합·문서·프로퍼티 테스트 카드가 쌓여 있고, 오른쪽 proptest 주사위 기계가 던진 입력 대부분은 통과 표시를 받지만 빨간 4,095바이트 상자 하나를 검수 로봇이 돋보기로 잡는다. 아래에 무작위 길이만으로는 네 번 모두 통과, 경계 길이를 섞자 바로 반례라고 적혀 있다

빨간 상자는 이 글에서 proptest 가 실제로 찾아 준 반례입니다. 본문 길이가 정확히 0xFFF 일 때만 나는 버그였습니다.

Rust 강의 10편입니다. [1편](https://blog.doksam.com/blog/rust-course-01-getting-started/)과 [5편](https://blog.doksam.com/blog/rust-course-05-parser-cli/)에서 단위 테스트와 실행 파일 통합 테스트를 맛봤습니다. 이번에는 네 종류를 나눠 쓰는 기준을 정합니다. 외부 크레이트는 개발 의존성 `proptest` 1.12.0 하나입니다. 코드는 2026-11-18 에 rustc 1.100.0 으로 돌려 확인했습니다.

## 시험 대상: 5편 레코드 형식을 줄인 인코더와 디코더

`encode(tag, body)` 는 머리 4바이트 뒤에 본문을 붙입니다. 본문이 0xFFF(4,095) 바이트 이상이면 크기 칸에 0xFFF 를 적고 실제 크기를 뒤 4바이트에 따로 적습니다. `decode_all` 은 바이트를 끝까지 읽어 `(태그, 본문)` 목록을 돌려주고, 하나라도 깨졌으면 `None` 입니다.

```rust
fn encode_with_limit(tag: u16, body: &[u8], limit: usize) -> Vec<u8> {
    let small = body.len() < limit;
    let size_field = if small { body.len() as u32 } else { 0xFFF };
    let header = (tag as u32 & 0x3FF) | ((size_field & 0xFFF) << 20);
    let mut out = header.to_le_bytes().to_vec();
    if !small {
        out.extend_from_slice(&(body.len() as u32).to_le_bytes());
    }
    out.extend_from_slice(body);
    out
}
```

공개 함수 `encode` 는 `limit` 에 0xFFF 를 넘깁니다. 시험용으로 경계를 일부러 틀린 `encode_buggy` 도 하나 둡니다. 이 함수는 0x1000 을 넘겨, 길이가 정확히 0xFFF 인 본문을 "작다"고 잘못 봅니다.

## 단위 테스트는 같은 파일에서 비공개 함수까지 봅니다

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_helper_is_reachable_here() {
        // 단위 테스트는 같은 모듈 안이라 비공개 함수도 부른다
        assert_eq!(split_u32(&[1, 0, 0, 0, 9]), Some((1, &[9u8][..])));
    }

    #[test]
    #[should_panic(expected = "range end index")]
    fn indexing_panics_on_short_input() {
        fn first4(d: &[u8]) -> &[u8] {
            &d[..4]
        }
        first4(&[1, 2, 3]);
    }
}
```

`split_u32` 는 `pub` 이 아닌 도우미입니다. 테스트 모듈이 같은 파일의 자식 모듈이라 부를 수 있습니다. `#[should_panic(expected = ...)]` 은 패닉을 기대하는 테스트이고, 메시지의 일부까지 맞춰 엉뚱한 패닉을 통과시키지 않습니다.

처음에는 `let data = [1u8, 2, 3]; &data[..4]` 로 썼습니다. 그러자 clippy 가 `out_of_bounds_indexing` 을 오류(deny)로 막았습니다. 길이가 고정된 배열이라 실행 전에 넘침을 안다는 뜻입니다. 그래서 슬라이스를 받는 함수로 감쌌습니다.

## 통합 테스트는 바깥 사용자처럼 공개 API 만 씁니다

`tests/` 폴더의 파일은 각자 별도 크레이트로 빌드됩니다. 라이브러리를 바깥에서 쓰는 사용자와 같은 자리입니다.

```rust
use tlvtest::{decode_all, encode};

#[test]
fn two_records_roundtrip() -> Result<(), String> {
    let mut bytes = encode(66, b"head");
    bytes.extend(encode(67, &[0x41, 0x00]));
    let recs = decode_all(&bytes).ok_or("decode failed")?;
    assert_eq!(recs, vec![(66, b"head".to_vec()), (67, vec![0x41, 0x00])]);
    Ok(())
}
```

테스트 함수는 `Result` 를 돌려줄 수 있어 `?` 를 씁니다. `Err` 를 돌려주면 실패입니다. 바깥에서 비공개 함수를 부르면 E0603 으로 막힙니다. 같은 일을 모듈 하나로 줄여 돌린 결과입니다.

```console
error[E0603]: function `split_u32` is private
  --> src/main.rs:14:27
   |
14 |     println!("{:?}", tlv::split_u32(&[1, 0, 0, 0]));
   |                           ^^^^^^^^^ private function
```

이 제약이 통합 테스트의 장점입니다. 내부를 바꿔도 공개 API 가 같으면 통합 테스트는 그대로 통과해야 합니다. 통합 테스트가 내부 구현에 기대고 있다면 리팩터링 때마다 깨집니다.

## 문서 테스트는 예시가 낡지 않게 합니다

`///` 주석 안의 코드 블록은 `cargo test` 가 컴파일하고 돌립니다.

```rust
/// ```
/// let bytes = tlvtest::encode(67, b"hi");
/// assert_eq!(bytes.len(), 4 + 2);
/// assert_eq!(tlvtest::decode_all(&bytes), Some(vec![(67, b"hi".to_vec())]));
/// ```
pub fn encode(tag: u16, body: &[u8]) -> Vec<u8> {
```

예시가 틀리면 테스트가 실패합니다. 머리 4바이트를 잊고 `bytes.len()` 을 2로 적은 예시를 돌렸습니다.

```console
src/lib.rs - encode (line 3) --- FAILED
...
assertion `left == right` failed
  left: 6
 right: 2
...
all doctests ran in 0.46s; merged doctests compilation took 0.12s
```

README 의 예시는 코드가 바뀌면 조용히 낡습니다. 문서 테스트는 낡는 순간 빨간불이 됩니다. 마지막 줄의 "merged doctests"는 에디션 2024 의 동작입니다. 문서 테스트들을 한 실행 파일로 묶어 컴파일하므로 예시가 많아도 빨리 돕니다.

## 프로퍼티 테스트는 성질을 적고 입력을 던집니다

손으로 고른 예시는 쓴 사람이 떠올린 경우만 봅니다. 프로퍼티 테스트는 성질을 적습니다.

```rust
use proptest::prelude::*;

proptest! {
    /// 만들고 읽으면 원래대로 돌아온다
    #[test]
    fn roundtrip(tag in 0u16..1024, body in prop::collection::vec(any::<u8>(), 0..6000)) {
        let bytes = encode(tag, &body);
        prop_assert_eq!(decode_all(&bytes), Some(vec![(tag, body)]));
    }

    /// 어떤 바이트를 넣어도 패닉하지 않는다
    #[test]
    fn never_panics(data in prop::collection::vec(any::<u8>(), 0..64)) {
        let _ = decode_all(&data);
    }
}
```

proptest 는 기본으로 성질마다 입력 256개를 만듭니다(소스의 `cases: 256`). 두 성질이 0.17초에 통과했습니다. 두 번째 성질은 퍼징(fuzzing, 무작위 입력으로 망가뜨려 보기)의 가벼운 판입니다. 5편에서 말한 "망가진 입력에도 패닉하지 않는다"를 테스트로 박아 둔 것입니다.

## 무작위만으로는 경계를 놓쳤습니다

같은 `roundtrip` 성질을 경계를 틀린 `encode_buggy` 에 걸었습니다. 네 번 돌렸는데 네 번 모두 통과했습니다. 버그는 본문 길이가 정확히 4,095 일 때만 나는데, 0~5,999 에서 고른 길이 256개가 그 값을 밟을 확률은 4% 남짓이기 때문입니다.

경계 근처 길이를 섞은 전략으로 바꿨습니다.

```rust
/// 경계(0xFFF 근처) 길이를 섞은 전략
fn boundary_body() -> impl Strategy<Value = Vec<u8>> {
    let len = prop_oneof![0usize..8, 0xFF8usize..0x1008, 0usize..6000];
    len.prop_flat_map(|n| prop::collection::vec(any::<u8>(), n))
}
```

첫 실행에서 바로 실패했습니다.

```console
test buggy_with_boundaries ... FAILED
...
Test failed: assertion failed: `(left == right)`
  left: `None`,
...
minimal failing input: tag = 0, body = [
    0,
    0,
...
proptest: Saving this and future failures in .../tests/prop_buggy.proptest-regressions
```

proptest 는 실패한 입력을 줄여(shrinking) 보여 줍니다. 태그는 0 이 됐고 본문 길이는 정확히 4,095 로 남았습니다. 그보다 줄이면 통과하기 때문입니다. 바이트 값은 앞쪽만 0 이 됐습니다. 출력에 따르면 1,024번 줄이기 시도에서 멈췄습니다. 실패한 입력의 씨앗은 `.proptest-regressions` 파일에 저장되고, 다음 실행 때 먼저 다시 돌립니다. 이 파일을 커밋하면 동료와 CI 도 같은 반례를 재현합니다.

해석은 이렇습니다. 프로퍼티 테스트의 힘은 무작위가 아니라 성질과 줄이기에 있습니다. 경계가 있는 형식이라면 그 경계를 전략에 넣어야 합니다. 경계를 모르는 코드라면 전략을 넓게 두고 횟수를 늘립니다.

## 면접에서 이렇게 묻는다

**Q1. 단위 테스트와 통합 테스트는 Rust 에서 어떻게 나누나요?**
단위 테스트는 같은 파일의 `#[cfg(test)]` 모듈에 두어 비공개 함수까지 봅니다. 통합 테스트는 `tests/` 에 두고 별도 크레이트로 빌드되어 공개 API 만 씁니다. 바깥에서 비공개 함수를 부르면 E0603 입니다.

**Q2. 비공개 함수는 테스트해야 하나요?**
같은 모듈의 단위 테스트로 할 수 있습니다. 다만 공개 API 로 시험할 수 있으면 그쪽이 리팩터링에 강합니다. 비공개 함수는 경계 계산처럼 따로 볼 가치가 있을 때 시험합니다.

**Q3. 문서 테스트는 왜 쓰나요?**
사용 예시가 실제로 컴파일되고 돌아가는지 확인해 문서가 낡지 않게 합니다. 예시가 틀리면 `cargo test` 가 실패합니다.

**Q4. 프로퍼티 테스트는 예시 테스트와 무엇이 다른가요?**
입력 하나의 기댓값 대신, 모든 입력에 성립할 성질을 적습니다. 실패하면 가장 작은 반례로 줄여 줍니다. 경계 값은 전략에 직접 넣어야 잘 찾습니다. 실험에서 무작위만으로는 4번 모두 놓쳤고 경계를 섞자 찾았습니다.

**Q5. 느린 테스트는 어떻게 다루나요?**
`#[ignore]` 를 붙여 평소에는 건너뛰고 `cargo test -- --ignored` 로 따로 돌립니다. 문서 변환 엔진도 1,000개 파일 벤치마크 테스트를 이렇게 둡니다.

## 연습

**연습 1.** `never_panics` 성질이 64바이트 이하만 던지는 이유를 생각하고, 확장 크기(0xFFF) 경로까지 두드리도록 고치세요.

<details>
<summary>풀이 보기</summary>

64바이트 안에서도 크기 칸이 0xFFF 인 머리는 무작위로 나올 수 있지만 드뭅니다. 머리를 직접 만들어 섞습니다.

```rust
    #[test]
    fn never_panics_extended(tag in 0u16..1024, ext in any::<u32>(), tail in prop::collection::vec(any::<u8>(), 0..64)) {
        let header = (tag as u32) | (0xFFF << 20);
        let mut data = header.to_le_bytes().to_vec();
        data.extend_from_slice(&ext.to_le_bytes());
        data.extend(tail);
        let _ = decode_all(&data);
    }
```

`ext` 가 `u32::MAX` 처럼 커도 `rest.get(..size)?` 가 `None` 을 돌려주므로 패닉하지 않아야 합니다. 이 성질도 통과했습니다.

</details>

**연습 2.** `encode` 의 문서 예시에 본문이 4,095바이트일 때 길이가 `4 + 4 + 4095` 인지 확인하는 줄을 더하세요.

<details>
<summary>풀이 보기</summary>

```rust
/// let big = tlvtest::encode(66, &vec![0u8; 0xFFF]);
/// assert_eq!(big.len(), 4 + 4 + 0xFFF);
```

경계 사례를 문서 예시에 두면 읽는 사람도 규칙을 바로 압니다. 같은 확인을 단위 테스트 `extended_size_boundary` 로도 두었습니다.

</details>

**연습 3.** 경계를 틀린 `encode_buggy` 를 찾지 못한 전략이 있다면, 실행 횟수를 얼마로 늘려야 길이 4,095 를 99% 확률로 한 번 이상 밟을까요?

<details>
<summary>풀이 보기</summary>

길이는 0~5,999 에서 고르게 뽑히니 한 번에 밟을 확률은 1/6,000 입니다. 한 번도 못 밟을 확률 (1 − 1/6,000)ⁿ 이 0.01 이하가 되려면 n ≈ 6,000 × ln 100 ≈ 27,600 번입니다. 기본 256번의 100배가 넘습니다. 경계를 전략에 넣는 편이 훨씬 쌉니다.

</details>

## 한 단계 더 — 테스트 비용과 결정성

테스트는 에이전트가 고칠 때마다 돌리므로 비용이 중요합니다(6편). 이 크레이트의 단위 3개, 통합 1개, 프로퍼티 3개(연습 1 포함, 입력 768개), 문서 2개는 모두 1초 안에 끝났습니다. 프로퍼티 테스트는 횟수를 `ProptestConfig::with_cases(n)` 으로 조절합니다. 평소에는 기본값으로 두고, 밤 사이 CI 에서만 늘리는 식으로 나눌 수 있습니다.

무작위 테스트가 실행마다 다른 입력을 쓰면 실패를 재현하기 어렵다는 걱정도 있습니다. proptest 는 실패한 씨앗을 파일에 남기고 다음 실행에서 먼저 돌려서 이 문제를 줄입니다. 남은 위험은 "통과했으니 맞다"는 착각입니다. 위 실험처럼 통과는 버그가 없다는 증거가 아닙니다. 문서 변환 엔진이 단위 테스트에 더해 실제 문서 묶음의 출력을 기준 파일과 비교하는 것도 같은 이유입니다. 기준 파일을 바꿀 때는 원본 문서와 대조한 뒤 어떤 변경 때문인지 커밋에 적습니다.

## 정리

테스트는 자리가 범위를 정합니다. 비공개 함수는 단위 테스트, 공개 약속은 통합 테스트, 사용 예시는 문서 테스트로 봅니다. 손이 떠올리지 못한 입력은 프로퍼티 테스트로 던지되, 경계는 전략에 직접 넣습니다.

11편에서는 크레이트를 나눕니다. 워크스페이스와 feature 로 라이브러리 사용자가 쓰지 않는 의존성을 빌드하지 않게 만드는 법을 다룹니다.
