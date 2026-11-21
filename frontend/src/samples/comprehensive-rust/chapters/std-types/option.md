# Option

`Option<T>`의 일부 사용법은 이미 살펴봤습니다. ‘T’ 타입의 값을 저장하거나 아무것도 저장하지 않습니다. 예를 들어 [`String::find`](https://doc.rust-lang.org/stable/std/string/struct.String.html#method.find)는 `Option<usize>`를 반환합니다.

패닉 발생 예제

``` rust
fn main() {
    let name = "Löwe 老虎 Léopard Gepardi";
    let mut position: Option<usize> = name.find('é');
    println!("find의 반환값 {position:?}");
    assert_eq!(position.unwrap(), 14);
    position = name.find('Z');
    println!("find의 반환값 {position:?}");
    assert_eq!(position.expect("문자를 찾을 수 없음"), 0);
}
```

> **발표자 노트**
>
> This slide should take about 10 minutes.
>
> - `Option` is widely used, not just in the standard library.
> - `unwrap`은 `Option`의 값을 반환하거나 패닉을 반환합니다. `expect`도 비슷하지만 오류 메시지가 표시됩니다.
>   - None 발생 시 패닉 상태가 될 수 있지만 ’실수’로 None을 체크하는 것을 잊을 수는 없습니다.
>   - 무언가를 함께 해킹할 때 모든 곳에서 `unwrap`/`expect`를 실행하는 것이 일반적이지만 프로덕션 코드는 일반적으로 더 나은 방식으로 `None`을 처리합니다.
> - 틈새 최적화란 `Option<T>`가 메모리에서 `T`와 크기가 같은 경우가 많다는 것을 의미합니다.
