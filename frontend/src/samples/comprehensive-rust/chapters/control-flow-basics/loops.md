# 배열과 for 반복문

Rust에는 `while`, `loop`, `for`라는 세 가지 반복 키워드가 있습니다.

## `while`

The [`while` keyword](https://doc.rust-lang.org/reference/expressions/loop-expr.html#predicate-loops) works much like in other languages, executing the loop body as long as the condition is true.

``` rust
fn main() {
    let mut x = 200;
    while x >= 10 {
        x = x / 2;
    }
    println!("최종 x: {x}");
}
```
