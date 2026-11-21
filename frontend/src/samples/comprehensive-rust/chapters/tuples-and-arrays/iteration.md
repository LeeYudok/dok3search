# 배열 순회

`for` 문은 배열 반복을 지원하지만 튜플은 지원하지 않습니다.

``` rust
fn main() {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes {
        for i in 2..prime {
            assert_ne!(prime % i, 0);
        }
    }
}
```

> **발표자 노트**
>
> This slide should take about 3 minutes.
>
> 이 기능은 `IntoIterator` 트레잇을 사용하지만 여기서는 아직 다루지 않았습니다.
>
> `assert_ne!` 매크로가 여기에 새로 추가되었습니다. `assert_eq!` 및 `assert!` 매크로도 있습니다. 이 매크로들은 항상 값을 확인하지만, `debug_assert!`와 같은 디버그 전용 매크로는 출시 빌드에서 컴파일되지 않고 사라집니다.
