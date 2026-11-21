# 연습문제: ROT13

이 예에서는 기존의 [“ROT13” 암호화](https://en.wikipedia.org/wiki/ROT13)를 구현합니다. 이 코드를 플레이그라운드에 복사하고 누락된 비트를 구현합니다. 결과가 여전히 유효한 UTF-8인지 확인하려면 ASCII 영문자만 회전하세요.

컴파일 오류 예제

``` rust
use std::io::Read;

struct RotDecoder<R: Read> {
    input: R,
    rot: u8,
}

// `RotDecoder`의 `Read` 트레잇을 구현합니다.

fn main() {
    let mut rot =
        RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
    let mut result = String::new();
    rot.read_to_string(&mut result).unwrap();
    println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn joke() {
        let mut rot =
            RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
        let mut result = String::new();
        rot.read_to_string(&mut result).unwrap();
        assert_eq!(&result, "To get to the other side!");
    }

    #[test]
    fn binary() {
        let input: Vec<u8> = (0..=255u8).collect();
        let mut rot = RotDecoder::<&[u8]> { input: input.as_ref(), rot: 13 };
        let mut buf = [0u8; 256];
        assert_eq!(rot.read(&mut buf).unwrap(), 256);
        for i in 0..=255 {
            if input[i] != buf[i] {
                assert!(input[i].is_ascii_alphabetic());
                assert!(buf[i].is_ascii_alphabetic());
            }
        }
    }
}
```

각각 13자씩 회전하는 두 개의 `RotDecoder` 인스턴스를 함께 체이닝하면 어떻게 될까요?
