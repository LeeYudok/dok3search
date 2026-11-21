# 공유 Enum

``` rust
#[cxx::bridge]
mod ffi {
    enum Suit {
        Clubs,
        Diamonds,
        Hearts,
        Spades,
    }
}
```

생성된 러스트:

``` rust
#![allow(unused)]
fn main() {
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Suit {
    pub repr: u8,
}

#[allow(non_upper_case_globals)]
impl Suit {
    pub const Clubs: Self = Suit { repr: 0 };
    pub const Diamonds: Self = Suit { repr: 1 };
    pub const Hearts: Self = Suit { repr: 2 };
    pub const Spades: Self = Suit { repr: 3 };
}
}
```

Generated C++:

``` c++
enum class Suit : uint8_t {
  Clubs = 0,
  Diamonds = 1,
  Hearts = 2,
  Spades = 3,
};
```

> **발표자 노트**
>
> - Rust 측에서, 공유된 enum에 관해 생성된 코드는 실제로 숫자 값을 래핑하는 구조체입니다. 이는 enum 클래스가 나열된 모든 변형과 다른 값을 보유하는 것이 C++에서 UB가 아니고 Rust 표현도 동일한 동작을 가져야 하기 때문입니다.
