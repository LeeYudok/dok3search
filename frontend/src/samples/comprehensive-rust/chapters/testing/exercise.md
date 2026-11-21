# 룬 알고리즘

## 룬 알고리즘

[룬(Luhn) 알고리즘](https://ko.wikipedia.org/wiki/%EB%A3%AC_%EC%95%8C%EA%B3%A0%EB%A6%AC%EC%A6%98)은 신용카드 번호 검증에 사용되는 알고리즘 입니다. 이 알고리즘은 신용카드 번호를 `문자열`로 입력받고, 아래의 순서에 따라 신용카드 번호의 유효성을 확인합니다:

- Ignore all spaces. Reject number with fewer than two digits.

- **오른쪽에서 왼쪽으로** 이동하며 2번째 자리마다 숫자를 2배 증가시킵니다. 예를 들어 `1234`에서 `3`과 `1`에 각각 2를 곱합니다.

- After doubling a digit, sum the digits if the result is greater than 9. So doubling `7` becomes `14` which becomes `1 + 4 = 5`.

- 모든 자리의 숫자를 더합니다.

- 합계의 끝자리가 `0`인 경우 유효한 신용카드 번호입니다.

제공된 코드는 대부분의 알고리즘이 올바르게 구현되었는지 확인하는 두 가지 기본 단위 테스트와 함께 luhn 알고리즘의 버그가 있는 구현을 제공합니다.

Copy the code below to <https://play.rust-lang.org/> and write additional tests to uncover bugs in the provided implementation, fixing any bugs you find.

``` rust
#![allow(unused)]
fn main() {
pub fn luhn(cc_number: &str) -> bool {
    let mut sum = 0;
    let mut double = false;

    for c in cc_number.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            if double {
                let double_digit = digit * 2;
                sum +=
                    if double_digit > 9 { double_digit - 9 } else { double_digit };
            } else {
                sum += digit;
            }
            double = !double;
        } else {
            continue;
        }
    }

    sum % 10 == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_cc_number() {
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4539 3195 0343 6467"));
        assert!(luhn("7992 7398 713"));
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!luhn("4223 9826 4026 9299"));
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0569"));
    }
}
}
```
