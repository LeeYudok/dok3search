# 슬라이스

슬라이스는 큰 컬랙션의 일부(혹은 전체)를 보여주는 뷰(view)입니다:

``` rust
fn main() {
    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];

    println!("s: {s:?}");
}
```

- 슬라이스는 다른(슬라이스 된) 타입으로부터 데이터를 ’빌려’옵니다.
- 질문: `s`를 출력하기 전에 `a[3]`을 수정하면 무슨 일이 있어날까요?

> **발표자 노트**
>
> This slide should take about 10 minutes.
>
> - 슬라이스는 우선 `a`를 빌린다음, 시작과 끝 인덱스를 브래킷(`[]`)안에 지정해서 만듭니다.
>
> - 슬라이스가 인덱스 0부터 시작한다면 시작 인덱스는 생략 가능합니다. 즉 `&a[0..a.len()]`와 `&a[..a.len()]` 는 동일합니다.
>
> - 마지막 인덱스도 생략 가능합니다. 그래서 `&a[2..a.len()]` 와 `&a[2..]`는 동일합니다.
>
> - 따라서 전체 배열에 대한 슬라이스는 `&a[..]`가 됩니다.
>
> - `s`는 `i32`들로 이루어진 슬라이스에 대한 참조입니다. `s`의 타입(`&[i32]`)에 배열의 크기가 빠져있음에 주목하시기 바랍니다. 즉, 슬라이스를 이용하면 다양한 길이의 데이터를 다룰 수 있습니다.
>
> - Slices always borrow from another object. In this example, `a` has to remain ‘alive’ (in scope) for at least as long as our slice.
>
> - The question about modifying `a[3]` can spark an interesting discussion, but the answer is that for memory safety reasons you cannot do it through `a` at this point in the execution, but you can read the data from both `a` and `s` safely. It works before you created the slice, and again after the `println`, when the slice is no longer used.
