# 클로저(Closure)

클로저 혹은 람다표현식은 익명타입입니다. 이들은 [`Fn`](https://doc.rust-lang.org/std/ops/trait.Fn.html),[`FnMut`](https://doc.rust-lang.org/std/ops/trait.FnMut.html), [`FnOnce`](https://doc.rust-lang.org/std/ops/trait.FnOnce.html) 라는 특별한 트레잇을 구현합니다:

``` rust
fn apply_with_log(func: impl FnOnce(i32) -> i32, input: i32) -> i32 {
    println!("Calling function on {input}");
    func(input)
}

fn main() {
    let add_3 = |x| x + 3;
    println!("add_3: {}", apply_with_log(add_3, 10));
    println!("add_3: {}", apply_with_log(add_3, 20));

    let mut v = Vec::new();
    let mut accumulate = |x: i32| {
        v.push(x);
        v.iter().sum::<i32>()
    };
    println!("accumulate: {}", apply_with_log(&mut accumulate, 4));
    println!("accumulate: {}", apply_with_log(&mut accumulate, 5));

    let multiply_sum = |x| x * v.into_iter().sum::<i32>();
    println!("multiply_sum: {}", apply_with_log(multiply_sum, 3));
}
```

> **발표자 노트**
>
> This slide should take about 20 minutes.
>
> `Fn`(예를 들어 `add_3`)은 캡처된 값을 소모도 변경도 하지 않고, 혹은 어떤 것도 캡쳐하지 않았을 수도 있기 때문에 동시에 여러번 호출할 수 있습니다.
>
> `FnMut`(예를 들어 `accumulate`)는 캡처된 값을 변경할 수 있으므로 여러 번 호출은 가능하지만 동시에 호출 할 수는 없습니다.
>
> `FnOnce` (예를 들어 `multiply_sum`)는 한번만 호출되며 캡처된 값을 소모합니다.
>
> `FnMut` 는 `FnOnce`의 하위타입입니다. `Fn`은 `FnMut`과 `FnOnce`의 하위 타입입니다. 즉, `FnMut`는 `FnOnce`가 호출되는 곳이면 어디서나 사용 할 수 있고 `Fn`은 `FnMut`와 `FnOnce`가 호출되는 곳이면 어디든 사용할 수 있습니다.
>
> When you define a function that takes a closure, you should take `FnOnce` if you can (i.e. you call it once), or `FnMut` else, and last `Fn`. This allows the most flexibility for the caller.
>
> In contrast, when you have a closure, the most flexible you can have is `Fn` (it can be passed everywhere), then `FnMut`, and lastly `FnOnce`.
>
> 컴파일러는 클로저가 무엇을 캡쳐하는지에 따라 `Copy`(예를 들어 `add_3`)과 `Clone`(예를 들어 `multiply_sum`)을 알아서 추론합니다.
>
> 기본적으로 클로져는, 가능하다면, 참조를 사용하여 캡쳐를 합니다. `move` 키워드를 쓰면 값으로 캡쳐가 됩니다.
>
> ``` rust
> fn make_greeter(prefix: String) -> impl Fn(&str) {
>     return move |name| println!("{} {}", prefix, name);
> }
>
> fn main() {
>     let hi = make_greeter("Hi".to_string());
>     hi("Greg");
> }
> ```
