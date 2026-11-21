# 연습문제: 피보나치

The first and second Fibonacci numbers are both `1`. For n\>2, the n’th Fibonacci number is calculated recursively as the sum of the n-1’th and n-2’th Fibonacci numbers.

Write a function `fib(n)` that calculates the n’th Fibonacci number. When will this function panic?

패닉 발생 예제

``` rust
fn fib(n: u32) -> u32 {
    if n <= 2 {
        // 기본 사례입니다.
        todo!("Implement this")
    } else {
        // 재귀 사례입니다.
        todo!("Implement this")
    }
}

fn main() {
    let n = 20;
    println!("fib({n}) = {}", fib(n));
}
```
