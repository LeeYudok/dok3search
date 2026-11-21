# Labels

사용합니다. 중첩 루프에서는 레이블과 함께 사용할 수 있습니다:

``` rust
fn main() {
    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let mut elements_searched = 0;
    let target_value = 10;
    'outer: for i in 0..=2 {
        for j in 0..=2 {
            elements_searched += 1;
            if s[i][j] == target_value {
                break 'outer;
            }
        }
    }
    print!("elements searched: {elements_searched}");
}
```

> **발표자 노트**
>
> - `loop`는 non-trivial 값을 반환하는 유일한 반복문입니다. 이는 `while` 및 `for` 반복문과 달리 최소한 한 번은 루프문을 수행하는 것이 보장되기 때문입니다.
