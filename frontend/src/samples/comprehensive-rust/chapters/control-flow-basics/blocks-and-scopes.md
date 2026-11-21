# 블록 및 범위

## 블록

A block in Rust contains a sequence of expressions, enclosed by braces `{}`. Each block has a value and a type, which are those of the last expression of the block:

``` rust
fn main() {
    let z = 13;
    let x = {
        let y = 10;
        println!("y: {y}");
        z - y
    };
    println!("x: {x}");
}
```

위의 `main` 함수는 마지막 표현식이 `;`로 끝나기 때문에 반환되는 값과 타입이 `()`입니다.

> **발표자 노트**
>
> This slide and its sub-slides should take about 5 minutes.
>
> - 블록 마지막 줄을 수정하면서 블록의 값이 어떻게 바뀌는지 보여주세요. 예를 들어, 세미콜론을 넣거나 뺀다든지, 아니면 `return`을 사용해 보세요.
