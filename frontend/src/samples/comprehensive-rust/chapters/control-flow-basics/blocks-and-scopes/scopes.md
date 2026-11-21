# 범위(Scopes)와 쉐도잉(Shadowing)

변수의 범위는 자신을 포함하는 블록으로 제한됩니다.

현재 범위에 있는 변수와, 바깥 범위에 있는 변수 모두 가릴(쉐도잉)수 있습니다:

``` rust
fn main() {
    let a = 10;
    println!("이전: {a}");
    {
        let a = "hello";
        println!("내부 범위: {a}");

        let a = true;
        println!("내부 범위 섀도 처리됨: {a}");
    }

    println!("이후: {a}");
}
```

> **발표자 노트**
>
> - Show that a variable’s scope is limited by adding a `b` in the inner block in the last example, and then trying to access it outside that block.
> - Shadowing is different from mutation, because after shadowing both variable’s memory locations exist at the same time. Both are available under the same name, depending where you use it in the code.
> - A shadowing variable can have a different type.
> - 처음에 쉐도잉을 보면 코드를 더 모호하게 만든다고 생각할 수 도 있습니다. 그러나 실제로 쉐도잉을 이용하면, 어떤 변수에서 `.unwrap()` 된 값을 새로운 변수에 담을 경우 새로운 이름을 지을 필요 없이 기존 이름을 유지할 수 있어서 편리합니다.
