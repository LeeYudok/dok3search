# 공유 타입

``` rust
#[cxx::bridge]
mod ffi {
    #[derive(Clone, Debug, Hash)]
    struct PlayingCard {
        suit: Suit,
        value: u8,  // A=1, J=11, Q=12, K=13
    }

    enum Suit {
        Clubs,
        Diamonds,
        Hearts,
        Spades,
    }
}
```

> **발표자 노트**
>
> - C와 같은 (단위) enum만 지원됩니다.
> - 공유 타입의 `#[derive()]`에는 제한된 수의 트레잇이 지원됩니다. C++ 코드에 대해서도 상응하는 기능이 생성됩니다. 예를 들어 `Hash`를 파생하면 해당 C++ 타입에 대한 `std::hash` 구현도 생성됩니다.
