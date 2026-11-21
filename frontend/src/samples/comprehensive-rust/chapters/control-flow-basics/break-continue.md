# `break`와 `continue`

다음 반복을 즉시 시작하려면 [`continue`](https://doc.rust-lang.org/reference/expressions/loop-expr.html#continue-expressions)를 사용합니다.

If you want to exit any kind of loop early, use [`break`](https://doc.rust-lang.org/reference/expressions/loop-expr.html#break-expressions). For `loop`, this can take an optional expression that becomes the value of the `loop` expression.

``` rust
fn main() {
    let mut i = 0;
    loop {
        i += 1;
        if i > 5 {
            break;
        }
        if i % 2 == 0 {
            continue;
        }
        println!("{}", i);
    }
}
```
