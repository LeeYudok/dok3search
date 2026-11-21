# 연습문제: 콜라츠 수열

The [Collatz Sequence](https://en.wikipedia.org/wiki/Collatz_conjecture) is defined as follows, for an arbitrary n₁ greater than zero:

- If *n_(i)* is 1, then the sequence terminates at *n_(i)*.
- If *n_(i)* is even, then *n_(i+1) = n_(i) / 2*.
- If *n_(i)* is odd, then *n_(i+1) = 3 \* n_(i) + 1*.

For example, beginning with *n₁* = 3:

- 3 is odd, so *n₂* = 3 \* 3 + 1 = 10;
- 10 is even, so *n₃* = 10 / 2 = 5;
- 5 is odd, so *n₄* = 3 \* 5 + 1 = 16;
- 16 is even, so *n₅* = 16 / 2 = 8;
- 8 is even, so *n₆* = 8 / 2 = 4;
- 4 is even, so *n₇* = 4 / 2 = 2;
- 2 is even, so *n₈* = 1; and
- 수열이 종료됩니다.

주어진 첫 번째 `n`에 대해 콜라츠 수열의 길이를 계산하는 함수를 작성합니다.

패닉 발생 예제

``` rust
/// `n`에서 시작하는 콜라츠 수열의 길이를 결정합니다.
fn collatz_length(mut n: i32) -> u32 {
  todo!("Implement this")
}

fn main() {
  todo!("Implement this")
}
```
